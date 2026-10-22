//! Fixture-only qualification and cooperative path observations, never inode authority.
use crate::{json, random_id, sftp::Sftp, sha, Fixture, Result, Value};
use russh_sftp::protocol::FileAttributes;

#[derive(Clone)]
pub struct Policy {
    pub scope_digest: String,
    pub root: String,
    pub namespace: String,
    pub enabled: bool,
    pub qualified: bool,
}
impl Policy {
    pub fn new(f: &Fixture, config_digest: &str) -> Self {
        let namespace = format!("fixture-sftp:{}", f.run_id);
        let scope_digest = sha(serde_json::to_vec(&json!({"mode":"cooperative","root":f.remote_root,"namespace":namespace,"config_generation":1,"config_digest":config_digest,"host":f.host,"port":f.port,"user":f.user,"container":f.container_id,"run":f.run_id})).unwrap().as_slice());
        Self {
            scope_digest,
            root: f.remote_root.clone(),
            namespace,
            enabled: true,
            qualified: false,
        }
    }
}

pub fn contained(root: &str, path: &str) -> bool {
    fn components(path: &str) -> Option<Vec<&str>> {
        if !path.starts_with('/') || path.chars().any(char::is_control) {
            return None;
        }
        let parts: Vec<_> = path[1..].split('/').collect();
        if parts
            .iter()
            .any(|c| c.is_empty() || *c == "." || *c == "..")
        {
            None
        } else {
            Some(parts)
        }
    }
    match (components(root), components(path)) {
        (Some(root), Some(path)) => path.starts_with(&root),
        _ => false,
    }
}

pub async fn directory(s: &Sftp, root: &str, path: &str) -> Result<FileAttributes> {
    if !contained(root, path) {
        return Err("PATH_DENIED".into());
    }
    let mut current = String::new();
    let mut last = FileAttributes::default();
    for component in path[1..].split('/') {
        current.push('/');
        current.push_str(component);
        last = s.raw.lstat(&current).await?.attrs;
        if !last.is_dir() || last.is_symlink() {
            return Err("PATH_DENIED".into());
        }
    }
    let normalized = s.raw.realpath(path).await?;
    if normalized.files.len() != 1 || normalized.files[0].filename != path {
        return Err("PATH_DENIED".into());
    }
    Ok(last)
}

pub fn same_access(a: &FileAttributes, b: &FileAttributes) -> bool {
    a.permissions.is_some()
        && a.uid.is_some()
        && a.gid.is_some()
        && a.permissions == b.permissions
        && a.uid == b.uid
        && a.gid == b.gid
}

pub async fn qualify(f: &Fixture, policy: &mut Policy) -> Result<Value> {
    let s = Sftp::connect(f, None).await?;
    if !s.hardlink_v1() {
        return Err("COMMIT_UNSUPPORTED".into());
    }
    directory(&s, &policy.root, &policy.root).await?;
    let dir = format!("{}/qualification-{}", policy.root, random_id()?);
    s.raw
        .mkdir(
            &dir,
            FileAttributes {
                permissions: Some(0o700),
                ..Default::default()
            },
        )
        .await?;
    let temp = format!("{dir}/source");
    let dest = format!("{dir}/final");
    let occupied = format!("{dir}/occupied");
    s.write_exclusive(&temp, b"QUALIFIED-LINK").await?;
    s.write_exclusive(&occupied, b"PRESERVE").await?;
    s.raw.hardlink(&temp, &dest).await?;
    let linked = s.read_bounded(&dest, 64).await?;
    let collision = s.raw.hardlink(&temp, &occupied).await;
    if linked != b"QUALIFIED-LINK"
        || collision.is_ok()
        || s.read_bounded(&occupied, 64).await? != b"PRESERVE"
    {
        return Err("qualification failed".into());
    }
    // Preparation owns these acknowledged resources on this still-live transport.
    for path in [&temp, &dest, &occupied] {
        s.raw.remove(path).await?;
    }
    s.raw.rmdir(&dir).await?;
    let evidence = json!({"evidence_kind":"real_server","run_id":f.run_id,"container_id":f.container_id,"namespace":policy.namespace,"root":policy.root,"scope_digest":policy.scope_digest,"extension_version":s.extensions.get("hardlink@openssh.com"),"linked_sha256":sha(&linked),"collision_preserved":true,"preparation_cleanup":"confirmed","limitation":"run-scoped cooperative fixture; no inode confinement or durability claim"});
    s.close().await?;
    policy.qualified = true;
    Ok(evidence)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn containment_is_components_not_prefix() {
        assert!(contained("/a/root", "/a/root/op/file"));
        for bad in [
            "/a/rooted/file",
            "/a/root/../other",
            "/a/root//x",
            "a/root/x",
            "/a/root/./x",
        ] {
            assert!(!contained("/a/root", bad));
        }
    }
}
