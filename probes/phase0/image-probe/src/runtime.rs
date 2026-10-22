use crate::image::{fail, Result};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

pub struct Runtime {
    pub path: PathBuf,
}
impl Runtime {
    pub fn new() -> Result<Self> {
        let mut random = [0u8; 16];
        File::open("/dev/urandom")?.read_exact(&mut random)?;
        let suffix: String = random.iter().map(|byte| format!("{byte:02x}")).collect();
        let path = PathBuf::from("/tmp").join(format!("sshx-phase0-visual-{suffix}"));
        DirBuilder::new().mode(0o700).create(&path)?;
        Ok(Self { path })
    }
    pub fn write(&self, name: &str, bytes: &[u8]) -> Result<PathBuf> {
        let path = self.path.join(name);
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)?;
        file.write_all(bytes)?;
        Ok(path)
    }
    pub fn command(&self, executable: impl AsRef<std::ffi::OsStr>) -> Command {
        let mut command = Command::new(executable);
        command
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", &self.path)
            .env("LANG", "C.UTF-8")
            .current_dir(&self.path)
            .process_group(0);
        command
    }
}
impl Drop for Runtime {
    fn drop(&mut self) {
        // Only the newly and exclusively created task directory is removed.
        if let Err(error) = fs::remove_dir_all(&self.path) {
            eprintln!("owned visual directory cleanup failed: {error}");
        }
    }
}

pub struct OwnedChild {
    child: Child,
}
impl OwnedChild {
    pub fn spawn(command: &mut Command) -> Result<Self> {
        Ok(Self {
            child: command.spawn()?,
        })
    }
    pub fn child(&mut self) -> &mut Child {
        &mut self.child
    }
    pub fn stop(&mut self) -> Result<()> {
        if self.child.try_wait()?.is_none() {
            // The un-reaped child reserves its PID/process group against reuse.
            unsafe {
                libc::kill(-(self.child.id() as i32), libc::SIGTERM);
            }
            let deadline = Instant::now() + Duration::from_secs(2);
            while Instant::now() < deadline {
                if self.child.try_wait()?.is_some() {
                    return Ok(());
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            unsafe {
                libc::kill(-(self.child.id() as i32), libc::SIGKILL);
            }
            self.child.wait()?;
        }
        Ok(())
    }
    pub fn wait(&mut self, timeout: Duration) -> Result<std::process::ExitStatus> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if let Some(status) = self.child.try_wait()? {
                return Ok(status);
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        self.stop()?;
        Err(fail("owned child timeout"))
    }
}
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

pub struct Display {
    pub name: String,
    pub authority: PathBuf,
    child: OwnedChild,
}
impl Display {
    pub fn start(runtime: &Runtime, label: &str) -> Result<Self> {
        let mut cookie = [0u8; 16];
        File::open("/dev/urandom")?.read_exact(&mut cookie)?;
        let authority = runtime.write(
            &format!("Xauthority-{label}"),
            &authority_record("", &cookie),
        )?;
        let diagnostic_path = runtime.path.join(format!("xvfb-{label}.stderr"));
        let stderr = File::create(&diagnostic_path)?;
        let mut command = runtime.command("/usr/bin/Xvfb");
        command
            .args([
                "-displayfd",
                "1",
                "-screen",
                "0",
                "800x600x24",
                "-nolisten",
                "tcp",
                "-auth",
            ])
            .arg(&authority)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(stderr);
        let mut child = OwnedChild::spawn(&mut command)?;
        let stdout = child
            .child()
            .stdout
            .take()
            .ok_or_else(|| fail("missing display fd"))?;
        let fd = std::os::fd::AsRawFd::as_raw_fd(&stdout);
        let mut poll = libc::pollfd {
            fd,
            events: libc::POLLIN,
            revents: 0,
        };
        if unsafe { libc::poll(&mut poll, 1, 3000) } != 1 {
            return Err(fail("Xvfb startup timeout"));
        }
        let mut line = String::new();
        BufReader::new(stdout).read_line(&mut line)?;
        if line.trim().is_empty() {
            let mut diagnostic = String::new();
            File::open(&diagnostic_path)?
                .take(4096)
                .read_to_string(&mut diagnostic)?;
            return Err(fail(&format!(
                "Xvfb failed before display allocation: {diagnostic}"
            )));
        }
        let number: u16 = line.trim().parse()?;
        fs::write(&authority, authority_record(&number.to_string(), &cookie))?;
        Ok(Self {
            name: format!(":{number}"),
            authority,
            child,
        })
    }
    pub fn command(&self, runtime: &Runtime, executable: impl AsRef<std::ffi::OsStr>) -> Command {
        let mut command = runtime.command(executable);
        command
            .env("DISPLAY", &self.name)
            .env("XAUTHORITY", &self.authority);
        command
    }
    pub fn stop(&mut self) -> Result<()> {
        self.child.stop()
    }
}
fn authority_record(display: &str, cookie: &[u8]) -> Vec<u8> {
    let mut bytes = 65535u16.to_be_bytes().to_vec();
    for part in [
        b"".as_slice(),
        display.as_bytes(),
        b"MIT-MAGIC-COOKIE-1".as_slice(),
        cookie,
    ] {
        bytes.extend_from_slice(&(part.len() as u16).to_be_bytes());
        bytes.extend_from_slice(part);
    }
    bytes
}

pub fn compile_selection(runtime: &Runtime) -> Result<PathBuf> {
    let source = runtime.write("xselection.c", include_bytes!("../native/xselection.c"))?;
    let helper = runtime.path.join("xselection");
    let output = runtime
        .command("/usr/bin/cc")
        .args(["-std=c11", "-O2", "-Wall", "-Wextra", "-Werror"])
        .arg(&source)
        .arg("-lX11")
        .arg("-o")
        .arg(&helper)
        .output()?;
    if !output.status.success() {
        return Err(fail(&format!(
            "selection compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(helper)
}

pub fn write_report(path: &Path, value: &serde_json::Value) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    Ok(())
}
