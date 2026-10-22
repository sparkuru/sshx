use crate::{Fixture, Result};
use russh_sftp::client::rawsession::RawSftpSession;
use russh_sftp::protocol::{FileAttributes, OpenFlags};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

struct Pipes {
    read: ChildStdout,
    write: ChildStdin,
    drop_rename_ack: bool,
    rename_sent: Arc<AtomicBool>,
    ack_dropped: Arc<AtomicBool>,
}
impl AsyncRead for Pipes {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let start = buf.filled().len();
        let result = Pin::new(&mut self.read).poll_read(cx, buf);
        if self.drop_rename_ack
            && self.rename_sent.load(Ordering::SeqCst)
            && matches!(result, Poll::Ready(Ok(())))
            && buf.filled().len() > start
        {
            buf.set_filled(start);
            self.ack_dropped.store(true, Ordering::SeqCst);
            return Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "injected owned SFTP reply loss",
            )));
        }
        result
    }
}
impl AsyncWrite for Pipes {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let result = Pin::new(&mut self.write).poll_write(cx, buf);
        if self.drop_rename_ack
            && buf.len() >= 5
            && buf[4] == 18
            && matches!(result,Poll::Ready(Ok(n))if n==buf.len())
        {
            self.rename_sent.store(true, Ordering::SeqCst);
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

pub struct Sftp {
    pub raw: RawSftpSession,
    child: Child,
    pub extensions: Vec<String>,
    pub ack_dropped: Arc<AtomicBool>,
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
            drop_rename_ack,
            rename_sent: Arc::new(AtomicBool::new(false)),
            ack_dropped: ack_dropped.clone(),
            read: child.stdout.take().ok_or("missing stdout")?,
            write: child.stdin.take().ok_or("missing stdin")?,
        };
        let raw = RawSftpSession::new(pipes);
        raw.set_timeout(5);
        let version = raw.init().await?;
        let extensions = version.extensions.keys().cloned().collect();
        Ok(Self {
            raw,
            child,
            extensions,
            ack_dropped,
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
    pub async fn close(mut self) -> Result<()> {
        self.raw.close_session()?;
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
        Ok(())
    }
}
