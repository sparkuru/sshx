use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Pty {
    pub child: Child,
    master: File,
    output: Arc<Mutex<VecDeque<u8>>>,
    pub bytes_read: Arc<std::sync::atomic::AtomicU64>,
    pub display_incomplete: Arc<std::sync::atomic::AtomicBool>,
    pub ended: Arc<std::sync::atomic::AtomicBool>,
}
impl Pty {
    pub fn spawn(command: &mut Command) -> io::Result<Self> {
        let (mut m, mut s) = (-1, -1);
        let size = libc::winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: openpty receives valid out pointers and a fully initialized winsize.
        if unsafe {
            libc::openpty(
                &mut m,
                &mut s,
                std::ptr::null_mut(),
                std::ptr::null(),
                &size,
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: successful openpty transferred two unique open descriptors.
        let master = unsafe { File::from_raw_fd(m) };
        let slave = unsafe { File::from_raw_fd(s) };
        for fd in [m, s] {
            if unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        command
            .stdin(Stdio::from(slave.try_clone()?))
            .stdout(Stdio::from(slave.try_clone()?))
            .stderr(Stdio::from(slave));
        // SAFETY: only async-signal-safe libc calls execute between fork and exec.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() < 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::ioctl(0, libc::TIOCSCTTY, 0) < 0 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn()?;
        // The shared master is nonblocking so one full receiver cannot stall Core RPC.
        let flags = unsafe { libc::fcntl(m, libc::F_GETFL) };
        if flags < 0 || unsafe { libc::fcntl(m, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::last_os_error());
        }
        let output = Arc::new(Mutex::new(VecDeque::with_capacity(1_048_576)));
        let bytes_read = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let display_incomplete = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (ring, count, incomplete) = (
            output.clone(),
            bytes_read.clone(),
            display_incomplete.clone(),
        );
        let ended = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let read_ended = ended.clone();
        let mut reader = master.try_clone()?;
        thread::spawn(move || {
            let mut b = [0; 8192];
            loop {
                let n = match reader.read(&mut b) {
                    Ok(n) => n,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                        continue;
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                };
                if n == 0 {
                    break;
                }
                count.fetch_add(n as u64, std::sync::atomic::Ordering::Relaxed);
                let mut r = ring.lock().expect("PTY buffer poisoned");
                for &v in &b[..n] {
                    if incomplete.load(std::sync::atomic::Ordering::Relaxed) {
                        continue;
                    }
                    if r.len() == 1_048_576 {
                        r.clear();
                        incomplete.store(true, std::sync::atomic::Ordering::Relaxed);
                        continue;
                    }
                    r.push_back(v);
                }
            }
            read_ended.store(true, std::sync::atomic::Ordering::SeqCst);
        });
        Ok(Self {
            child,
            master,
            output,
            bytes_read,
            display_incomplete,
            ended,
        })
    }
    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.master.write(data)
    }
    pub fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut offset = 0;
        while offset < data.len() {
            match self.write(&data[offset..]) {
                Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
                Ok(n) => offset += n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(2));
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    pub fn resize(&self, cols: u16, rows: u16) -> io::Result<()> {
        if cols == 0 || rows == 0 || cols > 500 || rows > 300 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "size"));
        }
        let s = libc::winsize {
            ws_row: rows,
            ws_col: cols,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: descriptor remains live and ioctl reads the initialized winsize.
        if unsafe { libc::ioctl(self.master.as_raw_fd(), libc::TIOCSWINSZ, &s) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub fn output(&self) -> Vec<u8> {
        self.output
            .lock()
            .expect("PTY buffer poisoned")
            .iter()
            .copied()
            .collect()
    }
    pub fn retained_len(&self) -> usize {
        self.output.lock().expect("PTY buffer poisoned").len()
    }
    pub fn wait_contains(&self, needle: &[u8], timeout: Duration) -> bool {
        let until = Instant::now() + timeout;
        while Instant::now() < until {
            if self.output().windows(needle.len()).any(|v| v == needle) {
                return true;
            }
            thread::sleep(Duration::from_millis(10));
        }
        false
    }
}
impl Drop for Pty {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Default)]
pub struct Prefix {
    held: bool,
    pub enabled: bool,
}
impl Prefix {
    pub fn new() -> Self {
        Self {
            held: false,
            enabled: true,
        }
    }
    pub fn feed(&mut self, data: &[u8]) -> (Vec<u8>, usize) {
        let mut out = Vec::new();
        let mut actions = 0;
        for &b in data {
            if !self.enabled {
                out.push(b);
            } else if self.held {
                self.held = false;
                match b {
                    b'p' => actions += 1,
                    0x1b => {}
                    0x1d => out.push(0x1d),
                    _ => out.extend_from_slice(&[0x1d, b]),
                }
            } else if b == 0x1d {
                self.held = true;
            } else {
                out.push(b)
            }
        }
        (out, actions)
    }
    pub fn timeout(&mut self) -> Vec<u8> {
        if self.held {
            self.held = false;
            vec![0x1d]
        } else {
            vec![]
        }
    }
}

/// Narrow OSC clipboard gate; never interprets a remote string as authority.
#[derive(Default)]
pub struct OscGate {
    pending: Vec<u8>,
    escape: bool,
    utf8_left: u8,
    pub incomplete: bool,
}
impl OscGate {
    pub fn feed(&mut self, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        if self.incomplete {
            return out;
        }
        for &b in data {
            let continuation = self.utf8_left > 0 && b & 0xc0 == 0x80;
            if continuation {
                self.utf8_left -= 1;
            } else {
                self.utf8_left = match b {
                    0xc2..=0xdf => 1,
                    0xe0..=0xef => 2,
                    0xf0..=0xf4 => 3,
                    _ => 0,
                };
            }
            if !self.pending.is_empty() {
                self.pending.push(b);
                if self.pending.len() > 65_536 {
                    self.pending.clear();
                    self.incomplete = true;
                    break;
                }
                if b == 7 || (b == 0x9c && !continuation) || self.pending.ends_with(b"\x1b\\") {
                    let start = if self.pending[0] == 0x9d { 1 } else { 2 };
                    let tail = &self.pending[start..];
                    let command = tail.split(|v| *v == b';').next().unwrap_or_default();
                    let numeric = std::str::from_utf8(command)
                        .ok()
                        .and_then(|s| s.parse::<u32>().ok());
                    let nested = tail.windows(2).any(|s| s == b"\x1b]") || tail.contains(&0x9d);
                    if nested {
                        self.incomplete = true;
                    } else if !matches!(numeric, Some(52 | 5522)) {
                        out.extend_from_slice(&self.pending);
                    }
                    self.pending.clear();
                }
            } else if self.escape {
                self.escape = false;
                if b == 0x1b {
                    self.escape = true;
                } else if b == b']' {
                    self.pending.extend_from_slice(b"\x1b]");
                } else {
                    out.extend_from_slice(&[0x1b, b]);
                }
            } else if b == 0x1b {
                self.escape = true;
            } else if b == 0x9d && !continuation {
                self.pending.push(b);
            } else {
                out.push(b);
            }
        }
        out
    }
    pub fn expire(&mut self) {
        if !self.pending.is_empty() {
            self.pending.clear();
            self.incomplete = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_split() {
        let mut p = Prefix::new();
        assert_eq!(p.feed(b"x\x1d").0, b"x");
        assert_eq!(p.feed(b"p").1, 1);
        assert_eq!(p.feed(b"\x1d\x1d").0, [29]);
        assert_eq!(p.feed(b"\x1d\x1b").0, b"");
        assert_eq!(p.feed(b"\x1dx").0, [29, b'x']);
        p.feed(&[29]);
        assert_eq!(p.timeout(), [29]);
    }
    #[test]
    fn split_osc_and_utf8() {
        for input in [b"a\x1b]52;c;evil\x07z".as_slice(), b"a\x9d5522;evil\x9cz"] {
            let mut g = OscGate::default();
            let out: Vec<_> = input.iter().flat_map(|b| g.feed(&[*b])).collect();
            assert_eq!(out, b"az");
        }
        let mut g = OscGate::default();
        let text = "\u{45d}".as_bytes();
        assert_eq!(g.feed(text), text);
        g.feed(b"\x1b]52;c;");
        g.expire();
        assert!(g.incomplete);
        assert!(g.feed(b"tail").is_empty());
    }
    #[test]
    fn repeated_escape_never_opens_a_second_osc() {
        for input in [
            b"A\x1b\x1b]52;c;evil\x07Z".as_slice(),
            b"A\x1b\x1b\x1b]5522;evil\x1b\\Z",
            b"A\x1b]052;c;evil\x07Z",
        ] {
            for split in 0..=input.len() {
                let mut gate = OscGate::default();
                let mut out = gate.feed(&input[..split]);
                out.extend(gate.feed(&input[split..]));
                assert_eq!(out, b"AZ");
            }
        }
    }
}
