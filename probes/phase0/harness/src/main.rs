mod core;
mod native;
mod plugin;
mod seal;
mod sftp;
mod suite;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fixture {
    pub schema: u64,
    pub run_id: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub identity_file: String,
    pub known_hosts_file: String,
    pub ssh_config_file: String,
    pub password_file: String,
    pub remote_root: String,
    pub container_id: String,
}
impl Fixture {
    pub fn load(path: &Path) -> Result<Self> {
        let m = fs::symlink_metadata(path)?;
        // SAFETY: geteuid has no preconditions.
        if !m.is_file()
            || m.uid() != unsafe { libc::geteuid() }
            || m.mode() & 0o077 != 0
            || m.len() > 16_384
        {
            return Err("fixture manifest permissions/size".into());
        }
        let f: Self = serde_json::from_slice(&fs::read(path)?)?;
        if f.schema != 1
            || f.host != "127.0.0.1"
            || f.port == 0
            || f.user != "probe"
            || f.remote_root != "/home/probe/probe-data"
        {
            return Err("unsupported fixture manifest".into());
        }
        for p in [
            &f.identity_file,
            &f.known_hosts_file,
            &f.ssh_config_file,
            &f.password_file,
        ] {
            let m = fs::symlink_metadata(p)?;
            if !m.is_file() || m.uid() != unsafe { libc::geteuid() } || m.mode() & 0o022 != 0 {
                return Err("unsafe fixture input".into());
            }
        }
        Ok(f)
    }
    pub fn ssh_args(&self, user: Option<&str>) -> Vec<String> {
        vec![
            "-F".into(),
            self.ssh_config_file.clone(),
            "-oBatchMode=yes".into(),
            "-oConnectTimeout=5".into(),
            "-oConnectionAttempts=1".into(),
            "-oControlMaster=no".into(),
            "-oControlPath=none".into(),
            "-oClearAllForwardings=yes".into(),
            "-oRemoteCommand=none".into(),
            "-oRequestTTY=no".into(),
            "-l".into(),
            user.unwrap_or(&self.user).into(),
        ]
    }
}
pub fn sha(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
pub fn random_id() -> Result<String> {
    use std::io::Read;
    let mut b = [0; 16];
    fs::File::open("/dev/urandom")?.read_exact(&mut b)?;
    Ok(b.iter().map(|x| format!("{x:02x}")).collect())
}
fn flag(args: &[String], name: &str) -> Result<PathBuf> {
    let n = args
        .iter()
        .position(|x| x == name)
        .ok_or_else(|| format!("missing {name}"))?;
    Ok(args.get(n + 1).ok_or("missing flag value")?.into())
}
fn optional_flag(args: &[String], name: &str) -> Result<Option<PathBuf>> {
    args.iter()
        .any(|a| a == name)
        .then(|| flag(args, name))
        .transpose()
}
fn main() {
    if let Err(e) = run() {
        eprintln!("phase0-probe: {e}");
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None | Some("--help") => {
            println!("SSHX Phase 0 Linux probes\n\nUsage:\n  phase0-probe suite --fixture STATE [--fixture-secondary STATE] --output FILE\n  phase0-probe case --fixture STATE --case ID --output FILE\n  phase0-probe plugin --fixture STATE --output FILE\n  phase0-probe core --fixture STATE [--fixture-secondary STATE] --runtime DIR\n  phase0-probe client --socket SOCKET\n  phase0-probe passthrough -- SSH_ARGS...\n\nOnly task-owned generated fixtures are supported. Probe RPC extensions are not a released SSHX API.");
            Ok(())
        }
        Some("case") => suite::run_case(
            &flag(&args, "--fixture")?,
            flag(&args, "--case")?.to_str().ok_or("case UTF8")?,
            &flag(&args, "--output")?,
        ),
        Some("plugin") => {
            let f = Fixture::load(&flag(&args, "--fixture")?)?;
            let out = flag(&args, "--output")?;
            let value = plugin::run(&f)?;
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(out)?;
            serde_json::to_writer_pretty(&mut file, &value)?;
            file.write_all(b"\n")?;
            if value["status"] != "pass" {
                return Err("plugin cases failed".into());
            }
            Ok(())
        }
        Some("suite") => suite::run(
            &flag(&args, "--fixture")?,
            optional_flag(&args, "--fixture-secondary")?.as_deref(),
            &flag(&args, "--output")?,
        ),
        Some("core") => core::serve(
            Fixture::load(&flag(&args, "--fixture")?)?,
            optional_flag(&args, "--fixture-secondary")?
                .map(|p| Fixture::load(&p))
                .transpose()?,
            &flag(&args, "--runtime")?,
        ),
        Some("client") => core::proxy(&flag(&args, "--socket")?),
        Some("passthrough") => {
            if args.get(1).map(String::as_str) != Some("--") {
                return Err("passthrough requires --".into());
            }
            use std::os::unix::process::CommandExt;
            let e = std::process::Command::new("/usr/bin/ssh")
                .args(&args[2..])
                .exec();
            Err(e.into())
        }
        _ => Err("unknown probe command; use --help".into()),
    }
}
