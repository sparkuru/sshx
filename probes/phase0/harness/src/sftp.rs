use crate::{Fixture, Result};
use russh_sftp::client::rawsession::RawSftpSession;
use russh_sftp::protocol::{FileAttributes, OpenFlags};
use std::collections::{BTreeMap, VecDeque};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

struct Pipes {
    read: ChildStdout,
    write: ChildStdin,
    matcher: Option<Arc<Mutex<ReplyMatcher>>>,
    incoming: Frames,
    ready: VecDeque<u8>,
    ack_dropped: Arc<AtomicBool>,
}
impl AsyncRead for Pipes {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if self.matcher.is_none() {
            return Pin::new(&mut self.read).poll_read(cx, buf);
        }
        loop {
            if !self.ready.is_empty() {
                let n = self.ready.len().min(buf.remaining());
                for _ in 0..n {
                    buf.put_slice(&[self.ready.pop_front().unwrap()]);
                }
                return Poll::Ready(Ok(()));
            }
            if self.ack_dropped.load(Ordering::SeqCst) {
                return Poll::Ready(Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "injected matched SFTP reply loss",
                )));
            }
            let mut bytes = [0; 8192];
            let mut input = ReadBuf::new(&mut bytes);
            match Pin::new(&mut self.read).poll_read(cx, &mut input) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                Poll::Ready(Ok(())) if input.filled().is_empty() => return Poll::Ready(Ok(())),
                Poll::Ready(Ok(())) => {}
            }
            let packets = self.incoming.feed(input.filled())?;
            for packet in packets {
                let matches = self
                    .matcher
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .matches(&packet);
                if matches {
                    self.ack_dropped.store(true, Ordering::SeqCst);
                } else {
                    self.ready.extend(packet);
                }
            }
        }
    }
}
impl AsyncWrite for Pipes {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let result = Pin::new(&mut self.write).poll_write(cx, buf);
        if let (Some(matcher), Poll::Ready(Ok(n))) = (&self.matcher, &result) {
            matcher.lock().unwrap().sent(&buf[..*n])?;
        }
        result
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.write).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.write).poll_shutdown(cx)
    }
}

#[derive(Default)]
struct Frames(Vec<u8>);
impl Frames {
    fn feed(&mut self, bytes: &[u8]) -> std::io::Result<Vec<Vec<u8>>> {
        self.0.extend_from_slice(bytes);
        let mut out = Vec::new();
        while self.0.len() >= 4 {
            let size = u32::from_be_bytes(self.0[..4].try_into().unwrap()) as usize;
            if size == 0 || size > 262_144 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "SFTP frame bound",
                ));
            }
            if self.0.len() < size + 4 {
                break;
            }
            out.push(self.0.drain(..size + 4).collect());
        }
        Ok(out)
    }
}
struct ReplyMatcher {
    outgoing: Frames,
    hardlink: bool,
    id: Option<[u8; 4]>,
}
impl ReplyMatcher {
    fn sent(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        for p in self.outgoing.feed(bytes)? {
            let selected = if self.hardlink {
                p.len() >= 13
                    && p[4] == 200
                    && u32::from_be_bytes(p[9..13].try_into().unwrap()) == 20
                    && p.get(13..33) == Some(b"hardlink@openssh.com".as_slice())
            } else {
                p.len() >= 9 && p[4] == 18
            };
            if selected {
                self.id = Some(p[5..9].try_into().unwrap());
            }
        }
        Ok(())
    }
    fn matches(&self, p: &[u8]) -> bool {
        p.len() >= 13 && p[4] == 101 && self.id.is_some_and(|id| p[5..9] == id)
    }
}

pub struct Sftp {
    pub raw: RawSftpSession,
    child: Child,
    pub extensions: BTreeMap<String, String>,
    pub ack_dropped: Arc<AtomicBool>,
    pub continuous: bool,
}
impl Sftp {
    pub async fn connect(f: &Fixture, user: Option<&str>) -> Result<Self> {
        Self::connect_with_fault(f, user, false).await
    }
    pub async fn connect_with_fault(
        f: &Fixture,
        user: Option<&str>,
        drop_rename_ack: bool,
    ) -> Result<Self> {
        Self::connect_fault(f, user, drop_rename_ack.then_some(false)).await
    }
    pub async fn connect_hardlink_fault(f: &Fixture, drop_ack: bool) -> Result<Self> {
        Self::connect_fault(f, None, drop_ack.then_some(true)).await
    }
    async fn connect_fault(f: &Fixture, user: Option<&str>, fault: Option<bool>) -> Result<Self> {
        let mut c = Command::new("/usr/bin/ssh");
        c.args(f.ssh_args(user))
            .args(["-T", "-s", "fixture", "sftp"]);
        c.env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("LANG", "C.UTF-8");
        c.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = c.spawn()?;
        let ack_dropped = Arc::new(AtomicBool::new(false));
        let pipes = Pipes {
            matcher: fault.map(|hardlink| {
                Arc::new(Mutex::new(ReplyMatcher {
                    outgoing: Frames::default(),
                    hardlink,
                    id: None,
                }))
            }),
            incoming: Frames::default(),
            ready: VecDeque::new(),
            ack_dropped: ack_dropped.clone(),
            read: child.stdout.take().ok_or("missing stdout")?,
            write: child.stdin.take().ok_or("missing stdin")?,
        };
        let raw = RawSftpSession::new(pipes);
        raw.set_timeout(5);
        let version = raw.init().await?;
        let extensions = version.extensions.into_iter().collect();
        Ok(Self {
            raw,
            child,
            extensions,
            ack_dropped,
            continuous: true,
        })
    }
    pub async fn write_exclusive(&self, path: &str, data: &[u8]) -> Result<()> {
        let h = self
            .raw
            .open(
                path,
                OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::EXCLUDE,
                FileAttributes::default(),
            )
            .await?;
        for (i, chunk) in data.chunks(32_768).enumerate() {
            self.raw
                .write(h.handle.clone(), (i * 32_768) as u64, chunk.to_vec())
                .await?;
        }
        self.raw.close(h.handle).await?;
        Ok(())
    }
    pub async fn read_bounded(&self, path: &str, limit: usize) -> Result<Vec<u8>> {
        let h = self
            .raw
            .open(path, OpenFlags::READ, FileAttributes::default())
            .await?;
        let mut out = Vec::new();
        loop {
            let n = (limit.saturating_sub(out.len()) + 1).min(32_768);
            match self
                .raw
                .read(h.handle.clone(), out.len() as u64, n as u32)
                .await
            {
                Ok(d) => {
                    if d.data.is_empty() {
                        break;
                    }
                    if out.len() + d.data.len() > limit {
                        return Err("readback limit".into());
                    }
                    out.extend_from_slice(&d.data);
                }
                Err(russh_sftp::client::error::Error::Status(s))
                    if s.status_code == russh_sftp::protocol::StatusCode::Eof =>
                {
                    break
                }
                Err(e) => return Err(e.into()),
            }
        }
        self.raw.close(h.handle).await?;
        Ok(out)
    }
    pub async fn rename_no_replace(&self, from: &str, to: &str) -> Result<()> {
        self.raw.rename(from, to).await?;
        Ok(())
    }
    pub fn hardlink_v1(&self) -> bool {
        self.extensions
            .get("hardlink@openssh.com")
            .map(String::as_str)
            == Some("1")
    }
    pub fn disconnect(&mut self) -> Result<()> {
        self.continuous = false;
        self.raw.close_session()?;
        self.child.start_kill()?;
        Ok(())
    }
    pub async fn close(mut self) -> Result<()> {
        self.raw.close_session()?;
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn packet(kind: u8, id: u32, tail: &[u8]) -> Vec<u8> {
        let mut p = ((5 + tail.len()) as u32).to_be_bytes().to_vec();
        p.push(kind);
        p.extend(id.to_be_bytes());
        p.extend(tail);
        p
    }
    #[test]
    fn hardlink_reply_matching_survives_all_fragment_boundaries() {
        let mut tail = 20u32.to_be_bytes().to_vec();
        tail.extend(b"hardlink@openssh.com");
        let target = packet(200, 42, &tail);
        let mut wire = packet(18, 41, b"irrelevant rename");
        wire.extend(&target);
        wire.extend(packet(200, 43, b"another extension"));
        for split in 0..=wire.len() {
            let mut m = ReplyMatcher {
                outgoing: Frames::default(),
                hardlink: true,
                id: None,
            };
            m.sent(&wire[..split]).unwrap();
            m.sent(&wire[split..]).unwrap();
            assert!(!m.matches(&packet(101, 41, &0u32.to_be_bytes())));
            assert!(!m.matches(&packet(101, 43, &0u32.to_be_bytes())));
            assert!(m.matches(&packet(101, 42, &0u32.to_be_bytes())));
            assert!(!m.matches(&packet(104, 42, &0u32.to_be_bytes())));
        }
        let mut replies = packet(101, 41, &0u32.to_be_bytes());
        replies.extend(packet(101, 42, &0u32.to_be_bytes()));
        replies.extend(packet(101, 43, &0u32.to_be_bytes()));
        for split in 0..=replies.len() {
            let mut frames = Frames::default();
            let mut out = frames.feed(&replies[..split]).unwrap();
            out.extend(frames.feed(&replies[split..]).unwrap());
            assert_eq!(out.len(), 3);
            assert_eq!(out.concat(), replies);
        }
        assert!(Frames::default().feed(&262_145u32.to_be_bytes()).is_err());
    }
}
