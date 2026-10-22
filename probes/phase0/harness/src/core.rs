use crate::native::Pty;
use crate::sftp::Sftp;
use crate::{json, random_id, sha, Fixture, Result, Value};
use serde::Serialize;
use sshx_phase0_protocol::{read_frame, write_frame, RequestCounter};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Domain<T> = std::result::Result<T, &'static str>;
#[derive(Clone, Serialize)]
struct Binding {
    core_instance_id: String,
    operation_id: String,
    session_id: String,
    generation: u64,
    client_id: String,
    attachment_id: String,
    writer_epoch: u64,
    profile_id: String,
    config_generation: u64,
}
struct Session {
    pty: Pty,
    profile: Profile,
    generation: u64,
    writer: Option<String>,
    attachment: String,
    epoch: u64,
}
#[derive(Clone)]
struct Profile {
    id: String,
    fixture: Fixture,
    source_config: String,
    config_digest: String,
}
#[derive(Clone)]
struct Operation {
    binding: Binding,
    nonce: u64,
    digest: String,
    cancel: bool,
    revoked: bool,
    expired: bool,
    upload: String,
    insert: String,
    path: Option<String>,
    upload_key: Option<String>,
    upload_digest: Option<String>,
    insert_key: Option<String>,
    insert_digest: Option<String>,
    bytes: usize,
    barrier: String,
    grant_id: Option<String>,
    external_steps: usize,
    last_step: String,
    cleanup_pending: bool,
    pause_step: Option<String>,
    capture_started: bool,
    commit_confirmed: bool,
    content_verified: bool,
    deadline: std::time::Instant,
}
struct Client {
    highwater: u64,
    live: bool,
}
struct Grant {
    operation_id: String,
    active: bool,
    deadline: std::time::Instant,
}
struct Ticket {
    client_id: String,
    session_id: String,
    epoch: u64,
    purpose: String,
    deadline: std::time::Instant,
}
struct State {
    boot: String,
    profiles: BTreeMap<String, Profile>,
    clients: BTreeMap<String, Client>,
    sessions: BTreeMap<String, Session>,
    operations: BTreeMap<String, Operation>,
    plugin_enabled: bool,
    stop: bool,
    grants: BTreeMap<String, Grant>,
    tickets: BTreeMap<String, Ticket>,
    retired_sessions: BTreeMap<String, Value>,
}
impl State {
    fn detach_client_writer(&mut self, cid: &str) {
        let mut detached = Vec::new();
        for (sid, s) in &mut self.sessions {
            if s.writer.as_deref() == Some(cid) {
                s.writer = None;
                detached.push(sid.clone());
            }
        }
        self.tickets.retain(|_, t| t.client_id != cid);
        for o in self.operations.values_mut() {
            if o.binding.client_id == cid && detached.contains(&o.binding.session_id) {
                o.cancel = true;
            }
        }
    }
    fn metadata_bytes(&self) -> usize {
        // Per-profile/session charges cover clones of the <=16KiB fixture manifest,
        // path strings, map nodes and handles. PTY rings have a separate 1MiB cap.
        65_536
            + self.profiles.len() * 131_072
            + self.clients.len() * 512
            + self.sessions.len() * 131_072
            + self.retired_sessions.len() * 2048
            + self.operations.len() * 8192
            + self.grants.len() * 2048
            + self.tickets.len() * 1024
    }
    fn check_writer(&self, client: &str, sid: &str, epoch: u64) -> Domain<()> {
        let s = self.sessions.get(sid).ok_or("NOT_FOUND")?;
        if s.pty
            .display_incomplete
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            return Err("DISPLAY_INCOMPLETE");
        }
        if s.writer.as_deref() != Some(client) || s.epoch != epoch {
            return Err("STALE_BINDING");
        }
        Ok(())
    }
    fn check_effect(&self, o: &Operation) -> Domain<()> {
        if o.cancel {
            return Err("CANCELLED");
        }
        let gid = o.grant_id.as_ref().ok_or("CAPABILITY_DENIED")?;
        if !self.grants.get(gid).is_some_and(|g| {
            g.active
                && g.operation_id == o.binding.operation_id
                && std::time::Instant::now() < g.deadline
        }) {
            return Err("CAPABILITY_DENIED");
        }
        if o.expired || std::time::Instant::now() >= o.deadline {
            return Err("EXPIRED");
        }
        if o.revoked || !self.plugin_enabled {
            return Err("CAPABILITY_DENIED");
        }
        let s = self
            .sessions
            .get(&o.binding.session_id)
            .ok_or("STALE_BINDING")?;
        if s.pty.ended.load(std::sync::atomic::Ordering::SeqCst) {
            return Err("STALE_BINDING");
        }
        if s.generation != o.binding.generation
            || s.attachment != o.binding.attachment_id
            || s.epoch != o.binding.writer_epoch
            || s.writer.as_deref() != Some(&o.binding.client_id)
            || s.profile.id != o.binding.profile_id
        {
            return Err("STALE_BINDING");
        }
        let current = bounded_file(Path::new(&s.profile.source_config), 65_536)
            .map_err(|_| "CONFIG_CHANGED")?;
        if sha(&current) != s.profile.config_digest {
            return Err("CONFIG_CHANGED");
        }
        Ok(())
    }
    fn projection(o: &Operation) -> Value {
        json!({"binding":o.binding,"upload_state":o.upload,"insert_state":o.insert,"cancel_requested":o.cancel,"bytes_written":o.bytes,"barrier":o.barrier,"external_steps_admitted":o.external_steps,"last_step":o.last_step,"cleanup_pending":o.cleanup_pending,"result":o.path.as_ref().map(|p|json!({"transfer_path":p,"shell_path":null,"mapping_id":null,"final_commit_confirmed":o.commit_confirmed,"readback_verified":o.content_verified}))})
    }
}
fn string<'a>(p: &'a Value, k: &str) -> Domain<&'a str> {
    p.get(k).and_then(Value::as_str).ok_or("INVALID_PARAMS")
}
fn number(p: &Value, k: &str) -> Domain<u64> {
    p.get(k).and_then(Value::as_u64).ok_or("INVALID_PARAMS")
}
fn response(id: &Value, result: Domain<Value>) -> Value {
    match result {
        Ok(v) => json!({"jsonrpc":"2.0","id":id,"result":v}),
        Err(name) => {
            json!({"jsonrpc":"2.0","id":id,"error":{"code":if name=="METHOD_NOT_FOUND"{-32601}else if name=="INVALID_PARAMS"{-32602}else{-32000},"message":name,"data":{"name":name,"retryable":false,"operation_id":null,"phase":null}}})
        }
    }
}
fn allowed_fields(method: &str) -> &'static [&'static str] {
    match method {
        "client.hello" => &["api_versions", "client_version"],
        "session.start" => &["profile_id", "receiver"],
        "session.attach" => &["session_id", "generation"],
        "session.reconnect" => &["session_id", "writer_epoch"],
        "session.detach" | "session.close" => &["session_id"],
        "session.resize" => &["session_id", "writer_epoch", "cols", "rows"],
        "probe.input" => &["session_id", "writer_epoch", "data"],
        "probe.output" | "probe.pty.stats" => &["session_id"],
        "action.invoke" => &["session_id", "writer_epoch", "trigger_nonce", "action_id"],
        "operation.get"
        | "operation.cancel"
        | "grant.revoke"
        | "probe.expire"
        | "probe.forget"
        | "probe.capture.begin" => &["operation_id"],
        "probe.upload" => &[
            "operation_id",
            "upload_key",
            "basename",
            "data",
            "hold_after_commit",
            "hold_mid_stream",
            "hold_step",
        ],
        "probe.release" => &["operation_id"],
        "grant.decide" => &["operation_id", "decision"],
        "probe.ticket.issue" => &["session_id", "writer_epoch", "purpose"],
        "probe.ticket.consume" => &["ticket"],
        "probe.insert" => &[
            "operation_id",
            "insert_key",
            "text",
            "context_revision",
            "mapping",
            "partial",
            "lose_ack",
            "drop_response",
        ],
        "plugin.manage" => &["enabled"],
        _ => &[],
    }
}

fn handle(
    state: &Arc<Mutex<State>>,
    client: &mut Option<String>,
    method: &str,
    p: &Value,
) -> Domain<Value> {
    if !p.is_object()
        || p.as_object().is_some_and(|v| {
            v.keys()
                .any(|k| !allowed_fields(method).contains(&k.as_str()))
        })
    {
        return Err("INVALID_PARAMS");
    }
    let booleans: &[&str] = match method {
        "probe.upload" => &["hold_after_commit", "hold_mid_stream"],
        "probe.insert" => &["mapping", "partial", "lose_ack", "drop_response"],
        "plugin.manage" => &["enabled"],
        _ => &[],
    };
    if booleans
        .iter()
        .any(|key| p.get(*key).is_some_and(|v| !v.is_boolean()))
    {
        return Err("INVALID_PARAMS");
    }
    let mut st = state.lock().map_err(|_| "INTERNAL_ERROR")?;
    for s in st.sessions.values_mut() {
        if s.pty
            .display_incomplete
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            s.writer = None;
        }
    }
    if st.metadata_bytes() + 8192 > 16 * 1024 * 1024 {
        return Err("BUSY");
    }
    if method == "client.hello" {
        if client.is_some() {
            return Err("ROLE_DENIED");
        }
        if !p["api_versions"]
            .as_array()
            .is_some_and(|v| v.contains(&json!("1.0")))
        {
            return Err("VERSION_MISMATCH");
        }
        if st.clients.len() >= 16 {
            return Err("BUSY");
        }
        let cid = random_id().map_err(|_| "IO_ERROR")?;
        st.clients.insert(
            cid.clone(),
            Client {
                highwater: 0,
                live: true,
            },
        );
        *client = Some(cid.clone());
        return Ok(
            json!({"api_version":"1.0","core_instance_id":st.boot,"client_id":cid,"limits_profile":"proposed-v1"}),
        );
    }
    let cid = client.as_ref().ok_or("ROLE_DENIED")?.clone();
    match method {
        "session.start" => {
            if st.sessions.len() + st.retired_sessions.len() >= 32 {
                return Err("BUSY");
            }
            let profile_id = match p.get("profile_id") {
                Some(value) => value.as_str().ok_or("INVALID_PARAMS")?,
                None => "primary",
            };
            let profile = st
                .profiles
                .get(profile_id)
                .ok_or("TARGET_UNVERIFIED")?
                .clone();
            let receiver = match p.get("receiver") {
                Some(value) => value.as_str().ok_or("INVALID_PARAMS")?,
                None => "cat",
            };
            let pty = spawn_pty(&profile.fixture, receiver)?;
            let pid = pty.child.id();
            let sid = random_id().map_err(|_| "IO_ERROR")?;
            let attachment = random_id().map_err(|_| "IO_ERROR")?;
            st.detach_client_writer(&cid);
            st.sessions.insert(
                sid.clone(),
                Session {
                    pty,
                    profile: profile.clone(),
                    generation: 1,
                    writer: Some(cid),
                    attachment: attachment.clone(),
                    epoch: 1,
                },
            );
            Ok(
                json!({"session_id":sid,"generation":1,"profile_id":profile.id,"attachment_id":attachment,"writer_epoch":1,"child_pid":pid,"authentication":"unknown","shell_readiness":"unknown"}),
            )
        }
        "session.list" => Ok(
            json!({"sessions":st.sessions.iter().map(|(id,s)|json!({"session_id":id,"generation":s.generation,"child_pid":s.pty.child.id(),"writer_epoch":s.epoch,"has_writer":s.writer.is_some()})).collect::<Vec<_>>()}),
        ),
        "session.reconnect" => {
            let sid = string(p, "session_id")?;
            st.check_writer(&cid, sid, number(p, "writer_epoch")?)?;
            let profile = st.sessions.get(sid).ok_or("NOT_FOUND")?.profile.clone();
            let replacement = spawn_pty(&profile.fixture, "cat")?;
            let s = st.sessions.get_mut(sid).ok_or("NOT_FOUND")?;
            let old = std::mem::replace(&mut s.pty, replacement);
            s.generation += 1;
            s.epoch += 1;
            s.attachment = random_id().map_err(|_| "IO_ERROR")?;
            let result = json!({"generation":s.generation,"writer_epoch":s.epoch,"attachment_id":s.attachment,"child_pid":s.pty.child.id()});
            st.tickets.retain(|_, t| t.session_id != sid);
            for o in st.operations.values_mut() {
                if o.binding.session_id == sid {
                    o.cancel = true;
                }
            }
            drop(st);
            drop(old);
            Ok(result)
        }
        "session.attach" => {
            let sid = string(p, "session_id")?;
            let generation = number(p, "generation")?;
            let s = st.sessions.get_mut(sid).ok_or("NOT_FOUND")?;
            if s.generation != generation {
                return Err("STALE_BINDING");
            }
            if s.pty
                .display_incomplete
                .load(std::sync::atomic::Ordering::SeqCst)
            {
                return Err("DISPLAY_INCOMPLETE");
            }
            if s.writer.is_some() {
                return Err("WRITER_BUSY");
            }
            st.detach_client_writer(&cid);
            let s = st.sessions.get_mut(sid).ok_or("NOT_FOUND")?;
            s.writer = Some(cid);
            s.epoch += 1;
            s.attachment = random_id().map_err(|_| "IO_ERROR")?;
            Ok(json!({"writer_epoch":s.epoch,"attachment_id":s.attachment}))
        }
        "session.detach" => {
            let sid = string(p, "session_id")?;
            let s = st.sessions.get_mut(sid).ok_or("NOT_FOUND")?;
            if s.writer.as_deref() != Some(&cid) {
                return Err("STALE_BINDING");
            }
            s.writer = None;
            st.tickets.retain(|_, t| t.session_id != sid);
            for o in st.operations.values_mut() {
                if o.binding.session_id == sid {
                    o.cancel = true;
                }
            }
            Ok(json!({"detached":true}))
        }
        "session.close" => {
            let sid = string(p, "session_id")?;
            let Some(session) = st.sessions.remove(sid) else {
                return Err("NOT_FOUND");
            };
            st.retired_sessions.insert(
                sid.into(),
                json!({"session_id":sid,"generation":session.generation,"process_state":"exited"}),
            );
            st.tickets.retain(|_, t| t.session_id != sid);
            for o in st.operations.values_mut() {
                if o.binding.session_id == sid {
                    o.cancel = true;
                }
            }
            drop(st);
            drop(session);
            Ok(json!({"closing":true}))
        }
        "session.resize" => {
            let sid = string(p, "session_id")?;
            st.check_writer(&cid, sid, number(p, "writer_epoch")?)?;
            let cols = u16::try_from(number(p, "cols")?).map_err(|_| "LIMIT_EXCEEDED")?;
            let rows = u16::try_from(number(p, "rows")?).map_err(|_| "LIMIT_EXCEEDED")?;
            st.sessions[sid]
                .pty
                .resize(cols, rows)
                .map_err(|_| "LIMIT_EXCEEDED")?;
            Ok(json!({"applied":true}))
        }
        "probe.input" => {
            let sid = string(p, "session_id")?;
            st.check_writer(&cid, sid, number(p, "writer_epoch")?)?;
            let data = bytes(p, "data", 65_536)?;
            let written = st
                .sessions
                .get_mut(sid)
                .ok_or("NOT_FOUND")?
                .pty
                .write(&data)
                .map_err(|e| {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        "BUSY"
                    } else {
                        "IO_ERROR"
                    }
                })?;
            Ok(json!({"written":written}))
        }
        "probe.pty.stats" => {
            let s = st
                .sessions
                .get(string(p, "session_id")?)
                .ok_or("NOT_FOUND")?;
            Ok(
                json!({"bytes_read":s.pty.bytes_read.load(std::sync::atomic::Ordering::SeqCst),"retained_bytes":s.pty.retained_len(),"ring_capacity":1_048_576,"display_incomplete":s.pty.display_incomplete.load(std::sync::atomic::Ordering::SeqCst),"has_writer":s.writer.is_some()}),
            )
        }
        "probe.output" => {
            let s = st
                .sessions
                .get_mut(string(p, "session_id")?)
                .ok_or("NOT_FOUND")?;
            if s.pty
                .display_incomplete
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                s.writer = None;
                return Err("DISPLAY_INCOMPLETE");
            }
            Ok(
                json!({"data":s.pty.output().into_iter().take(32_768).collect::<Vec<_>>(),"bytes_read":s.pty.bytes_read.load(std::sync::atomic::Ordering::Relaxed)}),
            )
        }
        "action.invoke" => {
            if !st.plugin_enabled {
                return Err("UNAVAILABLE");
            }
            let sid = string(p, "session_id")?;
            let epoch = number(p, "writer_epoch")?;
            st.check_writer(&cid, sid, epoch)?;
            let nonce_text = string(p, "trigger_nonce")?;
            if nonce_text.is_empty()
                || !nonce_text.bytes().all(|b| b.is_ascii_digit())
                || nonce_text.starts_with('0')
                || nonce_text.len() > 16
            {
                return Err("INVALID_PARAMS");
            }
            let nonce = nonce_text.parse::<u64>().map_err(|_| "INVALID_PARAMS")?;
            if nonce > sshx_phase0_protocol::INTEGER_MAX {
                return Err("INVALID_PARAMS");
            }
            let digest = sha(serde_json::to_string(p)
                .map_err(|_| "INVALID_PARAMS")?
                .as_bytes());
            if let Some(o) = st
                .operations
                .values()
                .find(|o| o.binding.client_id == cid && o.nonce == nonce)
            {
                return if o.digest == digest {
                    Ok(State::projection(o))
                } else {
                    Err("DEDUP_CONFLICT")
                };
            }
            let high = st.clients.get(&cid).ok_or("ROLE_DENIED")?.highwater;
            if nonce <= high {
                return Err("EXPIRED");
            }
            if nonce != high + 1 {
                return Err("INVALID_PARAMS");
            }
            if st.operations.len() >= 64
                || st
                    .operations
                    .values()
                    .filter(|o| {
                        !o.cancel
                            && !o.revoked
                            && !o.expired
                            && o.upload != "committed"
                            && o.upload != "failed"
                    })
                    .count()
                    >= 8
            {
                return Err("BUSY");
            }
            let s = &st.sessions[sid];
            let oid = random_id().map_err(|_| "IO_ERROR")?;
            let o = Operation {
                binding: Binding {
                    core_instance_id: st.boot.clone(),
                    operation_id: oid.clone(),
                    session_id: sid.into(),
                    generation: s.generation,
                    client_id: cid.clone(),
                    attachment_id: s.attachment.clone(),
                    writer_epoch: epoch,
                    profile_id: s.profile.id.clone(),
                    config_generation: 1,
                },
                nonce,
                digest,
                cancel: false,
                revoked: false,
                expired: false,
                upload: "none".into(),
                insert: "none".into(),
                path: None,
                upload_key: None,
                upload_digest: None,
                insert_key: None,
                insert_digest: None,
                bytes: 0,
                barrier: "none".into(),
                grant_id: None,
                external_steps: 0,
                last_step: "none".into(),
                cleanup_pending: false,
                pause_step: None,
                capture_started: false,
                commit_confirmed: false,
                content_verified: false,
                deadline: std::time::Instant::now() + Duration::from_secs(300),
            };
            st.clients.get_mut(&cid).ok_or("ROLE_DENIED")?.highwater = nonce;
            let result = State::projection(&o);
            st.operations.insert(oid, o);
            Ok(result)
        }
        "operation.get" => {
            let o = st
                .operations
                .get(string(p, "operation_id")?)
                .ok_or("NOT_FOUND")?;
            Ok(State::projection(o))
        }
        "probe.capture.begin" => {
            let o = owned(&st, &cid, p)?.clone();
            st.check_effect(&o)?;
            if o.capture_started {
                return Err("DEDUP_CONFLICT");
            }
            st.operations
                .get_mut(&o.binding.operation_id)
                .ok_or("NOT_FOUND")?
                .capture_started = true;
            Ok(json!({"binding":o.binding,"one_shot":true}))
        }
        "operation.cancel" | "grant.revoke" | "probe.expire" | "probe.forget" => {
            let id = owned(&st, &cid, p)?.binding.operation_id.clone();
            if method == "probe.forget" {
                if let Some(o) = st.operations.remove(&id) {
                    if let Some(gid) = o.grant_id {
                        st.grants.remove(&gid);
                    }
                }
                return Ok(json!({"forgotten":true}));
            }
            let o = st.operations.get_mut(&id).ok_or("NOT_FOUND")?;
            match method {
                "operation.cancel" => o.cancel = true,
                "grant.revoke" => o.revoked = true,
                _ => o.expired = true,
            }
            Ok(State::projection(o))
        }
        "grant.decide" => {
            let o = owned(&st, &cid, p)?.clone();
            if string(p, "decision")? != "allow" {
                return Err("CAPABILITY_DENIED");
            }
            if st.grants.len() >= 256 && o.grant_id.is_none() {
                return Err("BUSY");
            }
            let gid = match o.grant_id {
                Some(id) => id,
                None => random_id().map_err(|_| "IO_ERROR")?,
            };
            st.grants.insert(
                gid.clone(),
                Grant {
                    operation_id: o.binding.operation_id.clone(),
                    active: true,
                    deadline: o.deadline,
                },
            );
            st.operations
                .get_mut(&o.binding.operation_id)
                .ok_or("NOT_FOUND")?
                .grant_id = Some(gid.clone());
            Ok(
                json!({"grants":[gid],"composition":["clipboard.read_image","blob.read","blob.write","transfer.upload","terminal.insert"],"scope":"one synthetic operation/target/receiver only"}),
            )
        }
        "probe.ticket.issue" => {
            let sid = string(p, "session_id")?;
            let epoch = number(p, "writer_epoch")?;
            st.check_writer(&cid, sid, epoch)?;
            let purpose = string(p, "purpose")?;
            if !["input", "output"].contains(&purpose) {
                return Err("INVALID_PARAMS");
            }
            st.tickets.retain(|_, t| {
                !(t.client_id == cid && t.session_id == sid && t.purpose == purpose)
            });
            if st.tickets.len() >= 128 {
                return Err("BUSY");
            }
            let ticket = random_id().map_err(|_| "IO_ERROR")?;
            st.tickets.insert(
                ticket.clone(),
                Ticket {
                    client_id: cid,
                    session_id: sid.into(),
                    epoch,
                    purpose: purpose.into(),
                    deadline: std::time::Instant::now() + Duration::from_secs(5),
                },
            );
            Ok(json!({"ticket":ticket}))
        }
        "probe.ticket.consume" => {
            let key = string(p, "ticket")?;
            let t = st.tickets.get(key).ok_or("NOT_FOUND")?;
            if std::time::Instant::now() >= t.deadline {
                return Err("EXPIRED");
            }
            if t.client_id != cid {
                return Err("CAPABILITY_DENIED");
            }
            st.check_writer(&cid, &t.session_id, t.epoch)?;
            st.tickets.remove(key);
            Ok(json!({"consumed":true}))
        }
        "probe.metadata" => Ok(
            json!({"clients":st.clients.len(),"sessions":st.sessions.len()+st.retired_sessions.len(),"operations":st.operations.len(),"grants":st.grants.len(),"tickets":st.tickets.len(),"metadata_accounted_upper_bound":st.metadata_bytes(),"metadata_budget":16*1024*1024}),
        ),
        "plugin.manage" => {
            st.plugin_enabled = p["enabled"].as_bool().ok_or("INVALID_PARAMS")?;
            if !st.plugin_enabled {
                for o in st.operations.values_mut() {
                    o.revoked = true;
                }
            }
            Ok(json!({"enabled":st.plugin_enabled}))
        }
        "probe.upload" => {
            let o = owned(&st, &cid, p)?.clone();
            let id = o.binding.operation_id.clone();
            let pause_step = if let Some(value) = p.get("hold_step") {
                let name = value.as_str().ok_or("INVALID_PARAMS")?;
                if ![
                    "connect",
                    "root_lstat",
                    "mkdir",
                    "parent_lstat",
                    "open",
                    "write",
                    "close",
                    "commit_parent_lstat",
                    "rename",
                    "readback_open",
                    "readback",
                    "readback_close",
                ]
                .contains(&name)
                {
                    return Err("INVALID_PARAMS");
                }
                Some(name.to_string())
            } else {
                None
            };
            let key = string(p, "upload_key")?;
            if key.is_empty() || key.len() > 64 {
                return Err("INVALID_PARAMS");
            }
            let basename = string(p, "basename")?;
            if basename.is_empty()
                || basename.len() > 128
                || basename.contains('/')
                || basename == "."
                || basename == ".."
                || basename.chars().any(char::is_control)
            {
                return Err("PATH_DENIED");
            }
            let data = bytes(p, "data", 65_536)?;
            if data.starts_with(b"\x89PNG") {
                crate::seal::validate(&data).map_err(|_| "CORRUPT_IMAGE")?;
            }
            let intent =
                sha(format!("{}:{basename}:{}", o.binding.operation_id, sha(&data)).as_bytes());
            if let Some(old) = &o.upload_key {
                return if old == key && o.upload_digest.as_deref() == Some(&intent) {
                    Ok(State::projection(&o))
                } else {
                    Err("DEDUP_CONFLICT")
                };
            }
            st.check_effect(&o)?;
            let f = st
                .sessions
                .get(&o.binding.session_id)
                .ok_or("STALE_BINDING")?
                .profile
                .fixture
                .clone();
            let o = st.operations.get_mut(&id).ok_or("NOT_FOUND")?;
            o.upload = "in_progress".into();
            o.upload_key = Some(key.into());
            o.upload_digest = Some(intent);
            o.barrier = "before_open".into();
            o.pause_step = pause_step;
            let hold = p["hold_after_commit"].as_bool().unwrap_or(false);
            let mid = p["hold_mid_stream"].as_bool().unwrap_or(false);
            let shared = state.clone();
            let name = basename.to_string();
            thread::spawn(move || upload(shared, id, f, name, data, hold, mid));
            Ok(json!({"started":true}))
        }
        "probe.release" => {
            let id = owned(&st, &cid, p)?.binding.operation_id.clone();
            st.operations.get_mut(&id).ok_or("NOT_FOUND")?.barrier = "released".into();
            Ok(json!({"released":true}))
        }
        "probe.insert" => {
            let o = owned(&st, &cid, p)?.clone();
            let id = o.binding.operation_id.clone();
            let key = string(p, "insert_key")?;
            if key.is_empty() || key.len() > 64 {
                return Err("INVALID_PARAMS");
            }
            let text = string(p, "text")?;
            if text.len() > 4096
                || text
                    .chars()
                    .any(|c| c.is_control() || (0x7f..=0x9f).contains(&(c as u32)))
            {
                return Err("PATH_DENIED");
            }
            let digest = sha(format!(
                "{}:{text}:{}",
                o.path.clone().unwrap_or_default(),
                p["mapping"]
            )
            .as_bytes());
            if o.insert_key.is_some() {
                if o.insert_key.as_deref() != Some(key)
                    || o.insert_digest.as_deref() != Some(&digest)
                {
                    return Err("DEDUP_CONFLICT");
                }
                if o.insert != "refused" {
                    return Ok(State::projection(&o));
                }
            }
            st.check_effect(&o)?;
            if o.upload != "committed" || !o.content_verified {
                return Err("UNKNOWN_DELIVERY");
            }
            let entry = st.operations.get_mut(&id).ok_or("NOT_FOUND")?;
            entry.insert_key = Some(key.into());
            entry.insert_digest = Some(digest);
            if !p["mapping"].as_bool().unwrap_or(false) {
                entry.insert = "refused".into();
                return Err("MAPPING_UNKNOWN");
            }
            if number(p, "context_revision")? != 1 {
                entry.insert = "refused".into();
                return Err("CONTEXT_UNKNOWN");
            }
            if o.path.as_deref() != Some(text) {
                entry.insert = "refused".into();
                return Err("CAPABILITY_DENIED");
            }
            entry.insert = "pending".into();
            let requested = if p["partial"].as_bool() == Some(true) {
                text.len().min(3)
            } else {
                text.len()
            };
            let result = st
                .sessions
                .get_mut(&o.binding.session_id)
                .ok_or("STALE_BINDING")?
                .pty
                .write(&text.as_bytes()[..requested]);
            let entry = st.operations.get_mut(&id).ok_or("NOT_FOUND")?;
            let written = match result {
                Ok(n) => n,
                Err(e) => {
                    entry.insert = "refused".into();
                    return Err(if e.kind() == io::ErrorKind::WouldBlock {
                        "BUSY"
                    } else {
                        "IO_ERROR"
                    });
                }
            };
            entry.bytes = written;
            entry.insert = if written < text.len() {
                "partial"
            } else if p["lose_ack"].as_bool() == Some(true) {
                "indeterminate"
            } else {
                "committed"
            }
            .into();
            Ok(State::projection(entry))
        }
        "core.stop" => {
            if !st.sessions.is_empty() || st.operations.values().any(|o| o.upload == "in_progress")
            {
                return Err("BUSY");
            }
            st.stop = true;
            Ok(json!({"stopping":true}))
        }
        _ => Err("METHOD_NOT_FOUND"),
    }
}
fn bytes(p: &Value, k: &str, max: usize) -> Domain<Vec<u8>> {
    let a = p[k].as_array().ok_or("INVALID_PARAMS")?;
    if a.len() > max {
        return Err("LIMIT_EXCEEDED");
    }
    a.iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| u8::try_from(n).ok())
                .ok_or("INVALID_PARAMS")
        })
        .collect()
}
fn owned<'a>(st: &'a State, cid: &str, p: &Value) -> Domain<&'a Operation> {
    let o = st
        .operations
        .get(string(p, "operation_id")?)
        .ok_or("NOT_FOUND")?;
    if o.binding.client_id != cid {
        return Err("CAPABILITY_DENIED");
    }
    Ok(o)
}
fn barrier(state: &Arc<Mutex<State>>, id: &str, name: &str) -> Domain<()> {
    {
        let mut st = state.lock().map_err(|_| "INTERNAL_ERROR")?;
        let o = st.operations.get_mut(id).ok_or("NOT_FOUND")?;
        o.barrier = name.into();
    }
    let until = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let st = state.lock().map_err(|_| "INTERNAL_ERROR")?;
        let o = st.operations.get(id).ok_or("NOT_FOUND")?;
        st.check_effect(o)?;
        if o.barrier == "released" {
            return Ok(());
        }
        drop(st);
        if std::time::Instant::now() > until {
            return Err("TIMEOUT");
        }
        thread::sleep(Duration::from_millis(5));
    }
}
fn upload(
    state: Arc<Mutex<State>>,
    id: String,
    f: Fixture,
    basename: String,
    data: Vec<u8>,
    hold: bool,
    mid: bool,
) {
    let result = (|| -> Result<String> {
        barrier(&state, &id, "before_open").map_err(|e| e.to_string())?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let s = step(&state, &id, "connect", Sftp::connect(&f, None)).await?;
            let dir = format!("{}/{}", f.remote_root, id);
            let temp = format!("{dir}/.partial");
            let final_path = format!("{dir}/{basename}");
            let mut owns_dir = false;
            let mut owns_temp = false;
            let mut committed = false;
            let result = async {
                let root = step(&state, &id, "root_lstat", s.raw.lstat(&f.remote_root)).await?;
                if !root.attrs.is_dir() {
                    return Err("PATH_DENIED".into());
                }
                step(
                    &state,
                    &id,
                    "mkdir",
                    s.raw.mkdir(
                        dir.clone(),
                        russh_sftp::protocol::FileAttributes {
                            permissions: Some(0o700),
                            ..Default::default()
                        },
                    ),
                )
                .await?;
                owns_dir = true;
                let parent = step(&state, &id, "parent_lstat", s.raw.lstat(&dir)).await?;
                if !parent.attrs.is_dir() {
                    return Err("PATH_DENIED".into());
                }
                // An open failure does not establish ownership: never remove an unknown temp.
                use russh_sftp::protocol::{FileAttributes, OpenFlags};
                let handle = step(
                    &state,
                    &id,
                    "open",
                    s.raw.open(
                        &temp,
                        OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::EXCLUDE,
                        FileAttributes::default(),
                    ),
                )
                .await?;
                owns_temp = true;
                let streamed: Result<()> = async {
                    let split = if mid { data.len().min(1) } else { 0 };
                    if mid {
                        step(
                            &state,
                            &id,
                            "write",
                            s.raw
                                .write(handle.handle.clone(), 0, data[..split].to_vec()),
                        )
                        .await?;
                        barrier(&state, &id, "during_stream").map_err(|e| e.to_string())?;
                    }
                    for (n, chunk) in data[split..].chunks(32_768).enumerate() {
                        step(
                            &state,
                            &id,
                            "write",
                            s.raw.write(
                                handle.handle.clone(),
                                (split + n * 32_768) as u64,
                                chunk.to_vec(),
                            ),
                        )
                        .await?;
                    }
                    step(&state, &id, "close", s.raw.close(handle.handle.clone())).await?;
                    Ok(())
                }
                .await;
                if let Err(e) = streamed {
                    // Cleanup of an already owned handle is permitted after cancellation.
                    let _ = s.raw.close(handle.handle).await;
                    return Err(e);
                }
                barrier(&state, &id, "before_commit").map_err(|e| e.to_string())?;
                let parent = step(&state, &id, "commit_parent_lstat", s.raw.lstat(&dir)).await?;
                if !parent.attrs.is_dir() {
                    return Err("PATH_DENIED".into());
                }
                step(
                    &state,
                    &id,
                    "rename",
                    s.rename_no_replace(&temp, &final_path),
                )
                .await?;
                committed = true;
                owns_temp = false;
                {
                    let mut st = state.lock().map_err(|_| "state poisoned")?;
                    let o = st.operations.get_mut(&id).ok_or("missing operation")?;
                    o.path = Some(final_path.clone());
                    o.commit_confirmed = true;
                }
                let read_handle = step(
                    &state,
                    &id,
                    "readback_open",
                    s.raw
                        .open(&final_path, OpenFlags::READ, FileAttributes::default()),
                )
                .await?;
                let read = step(
                    &state,
                    &id,
                    "readback",
                    s.raw
                        .read(read_handle.handle.clone(), 0, (data.len() + 1) as u32),
                )
                .await?
                .data;
                step(
                    &state,
                    &id,
                    "readback_close",
                    s.raw.close(read_handle.handle),
                )
                .await?;
                if read != data {
                    return Err("readback mismatch".into());
                }
                state
                    .lock()
                    .map_err(|_| "state")?
                    .operations
                    .get_mut(&id)
                    .ok_or("operation")?
                    .content_verified = true;
                if hold {
                    {
                        let mut st = state.lock().map_err(|_| "state")?;
                        let o = st.operations.get_mut(&id).ok_or("operation")?;
                        o.path = Some(final_path.clone());
                        o.upload = "committed".into();
                        o.barrier = "after_commit".into();
                    }
                    let until = std::time::Instant::now() + Duration::from_secs(10);
                    loop {
                        let st = state.lock().map_err(|_| "state")?;
                        if st
                            .operations
                            .get(&id)
                            .is_some_and(|o| o.barrier == "released")
                        {
                            break;
                        }
                        drop(st);
                        if std::time::Instant::now() > until {
                            break;
                        }
                        thread::sleep(Duration::from_millis(5));
                    }
                }
                Ok(final_path.clone())
            }
            .await;
            if result.is_err() && !committed {
                let uncertain = state
                    .lock()
                    .map_err(|_| "state")?
                    .operations
                    .get(&id)
                    .is_some_and(|o| o.barrier == "commit_dispatched");
                if !uncertain {
                    let temp_removed = !owns_temp || s.raw.remove(&temp).await.is_ok();
                    let dir_removed = !owns_dir || s.raw.rmdir(&dir).await.is_ok();
                    if let Some(o) = state.lock().map_err(|_| "state")?.operations.get_mut(&id) {
                        o.cleanup_pending = !(temp_removed && dir_removed);
                    }
                }
            }
            let _ = s.close().await;
            result
        })
    })();
    if let Ok(mut st) = state.lock() {
        if let Some(o) = st.operations.get_mut(&id) {
            match result {
                Ok(path) => {
                    o.path = Some(path);
                    o.upload = "committed".into();
                    o.barrier = "completed".into();
                }
                Err(_) => {
                    o.upload = if o.commit_confirmed {
                        "committed"
                    } else if o.barrier == "commit_dispatched" {
                        "indeterminate"
                    } else {
                        "failed"
                    }
                    .into();
                    o.barrier = "completed".into();
                }
            }
        }
    }
}

async fn step<T, E>(
    state: &Arc<Mutex<State>>,
    id: &str,
    name: &str,
    future: impl std::future::Future<Output = std::result::Result<T, E>>,
) -> Result<T>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let pause = state
        .lock()
        .map_err(|_| "state poisoned")?
        .operations
        .get(id)
        .is_some_and(|o| o.pause_step.as_deref() == Some(name));
    if pause {
        barrier(state, id, &format!("before_{name}")).map_err(|e| e.to_string())?;
        if let Some(o) = state
            .lock()
            .map_err(|_| "state poisoned")?
            .operations
            .get_mut(id)
        {
            o.pause_step = None;
        }
    }
    {
        let mut st = state.lock().map_err(|_| "state poisoned")?;
        let o = st.operations.get(id).ok_or("missing operation")?;
        st.check_effect(o).map_err(|e| e.to_string())?;
        let o = st.operations.get_mut(id).ok_or("missing operation")?;
        o.external_steps += 1;
        o.last_step = name.into();
        if name == "rename" {
            o.barrier = "commit_dispatched".into();
        }
    }
    // Admission above linearizes against cancellation; a step in flight may complete.
    future.await.map_err(Into::into)
}

fn peer(stream: &UnixStream) -> io::Result<()> {
    // SAFETY: getsockopt receives the correct writable buffer and length.
    let mut c: libc::ucred = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    if unsafe {
        libc::getsockopt(
            stream.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut c as *mut libc::ucred).cast(),
            &mut len,
        )
    } < 0
    {
        return Err(io::Error::last_os_error());
    }
    if c.uid != unsafe { libc::geteuid() } {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "peer UID"));
    }
    Ok(())
}
pub fn serve(fixture: Fixture, secondary: Option<Fixture>, runtime: &Path) -> Result<()> {
    let m = fs::symlink_metadata(runtime)?;
    if !m.is_dir() || m.mode() & 0o077 != 0 || m.uid() != unsafe { libc::geteuid() } {
        return Err("runtime directory ownership/mode".into());
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(runtime.join("core.lock"))?;
    if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err("Core already started".into());
    }
    let socket = runtime.join("core.sock");
    let listener = UnixListener::bind(&socket)?;
    let original = fs::symlink_metadata(&socket)?;
    fs::set_permissions(&socket, fs::Permissions::from_mode(0o600))?;
    listener.set_nonblocking(true)?;
    let mut profiles = BTreeMap::new();
    for (id, f) in std::iter::once(("primary", fixture)).chain(secondary.map(|f| ("secondary", f)))
    {
        let source_config = f.ssh_config_file.clone();
        let original_config = bounded_file(Path::new(&source_config), 65_536)?;
        let frozen = freeze_fixture(f, runtime, id)?;
        profiles.insert(
            id.into(),
            Profile {
                id: id.into(),
                fixture: frozen,
                source_config,
                config_digest: sha(&original_config),
            },
        );
    }
    let state = Arc::new(Mutex::new(State {
        boot: random_id()?,
        profiles,
        clients: BTreeMap::new(),
        sessions: BTreeMap::new(),
        operations: BTreeMap::new(),
        plugin_enabled: true,
        grants: BTreeMap::new(),
        tickets: BTreeMap::new(),
        retired_sessions: BTreeMap::new(),
        stop: false,
    }));
    let connections = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    loop {
        {
            let mut st = state.lock().map_err(|_| "state poisoned")?;
            for s in st.sessions.values_mut() {
                if s.pty
                    .display_incomplete
                    .load(std::sync::atomic::Ordering::SeqCst)
                {
                    s.writer = None;
                }
            }
            if st.stop {
                break;
            }
        }
        match listener.accept() {
            Ok((stream, _)) => {
                if peer(&stream).is_err() {
                    continue;
                }
                if connections.load(std::sync::atomic::Ordering::Relaxed) >= 16 {
                    continue;
                }
                connections.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let count = connections.clone();
                let shared = state.clone();
                thread::spawn(move || {
                    let _ = connection(stream, shared);
                    count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                });
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5))
            }
            Err(e) => return Err(e.into()),
        }
    }
    // The endpoint was created in a fresh private directory; never remove replacements.
    drop(listener);
    if let Ok(now) = fs::symlink_metadata(&socket) {
        if now.ino() == original.ino() && now.dev() == original.dev() {
            fs::remove_file(socket)?;
        }
    }
    for id in ["primary", "secondary"] {
        for stem in ["frozen_config", "frozen_known_hosts"] {
            let file = runtime.join(if id == "primary" {
                stem.into()
            } else {
                format!("{stem}-{id}")
            });
            if file.exists() {
                fs::remove_file(file)?;
            }
        }
    }
    drop(lock);
    Ok(())
}
fn connection(mut stream: UnixStream, state: Arc<Mutex<State>>) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut counter = RequestCounter::default();
    let mut client = None;
    let result = (|| -> Result<()> {
        loop {
            if client.is_some() {
                stream.set_read_timeout(None)?;
                if reader.fill_buf()?.is_empty() {
                    break;
                }
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;
            }
            let Some(request) = read_frame(&mut FrameReader {
                reader: &mut reader,
                deadline: std::time::Instant::now() + Duration::from_secs(5),
            })?
            else {
                break;
            };
            let id = request
                .get("id")
                .and_then(Value::as_str)
                .ok_or("request id")?;
            counter.admit(id)?;
            if request["jsonrpc"] != "2.0"
                || request.as_object().is_some_and(|m| {
                    m.keys()
                        .any(|k| !["jsonrpc", "id", "method", "params"].contains(&k.as_str()))
                })
            {
                return Err("invalid request".into());
            }
            let method = request["method"].as_str().ok_or("method")?;
            let result = handle(&state, &mut client, method, &request["params"]);
            if method == "probe.insert"
                && request["params"]["drop_response"] == true
                && result.is_ok()
            {
                break;
            }
            let close = result == Err("VERSION_MISMATCH");
            write_frame(&mut stream, &response(&request["id"], result))?;
            if close {
                break;
            }
        }
        Ok(())
    })();
    if let Some(cid) = client {
        if let Ok(mut st) = state.lock() {
            if let Some(c) = st.clients.get_mut(&cid) {
                c.live = false;
            }
            st.tickets.retain(|_, t| t.client_id != cid);
            for s in st.sessions.values_mut() {
                if s.writer.as_deref() == Some(&cid) {
                    s.writer = None;
                }
            }
            for o in st.operations.values_mut() {
                if o.binding.client_id == cid {
                    o.cancel = true;
                }
            }
        }
    }
    result
}
pub fn proxy(socket: &Path) -> Result<()> {
    let mut stream = UnixStream::connect(socket)?;
    peer(&stream)?;
    stream.set_read_timeout(Some(Duration::from_secs(15)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let input = io::stdin();
    let mut stdin = input.lock();
    let mut output = io::stdout().lock();
    while let Some(v) = read_frame(&mut stdin)? {
        write_frame(&mut stream, &v)?;
        let r = read_frame(&mut reader)?.ok_or("Core EOF")?;
        write_frame(&mut output, &r)?;
    }
    Ok(())
}

fn freeze_fixture(mut f: Fixture, runtime: &Path, id: &str) -> Result<Fixture> {
    let text = String::from_utf8(bounded_file(Path::new(&f.ssh_config_file), 65_536)?)?;
    let allowed = [
        "host",
        "hostname",
        "port",
        "user",
        "identityfile",
        "identityagent",
        "identitiesonly",
        "userknownhostsfile",
        "globalknownhostsfile",
        "stricthostkeychecking",
        "hostkeyalgorithms",
        "batchmode",
        "connecttimeout",
        "connectionattempts",
        "controlmaster",
        "loglevel",
    ];
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let directive = line
            .split_whitespace()
            .next()
            .ok_or("config line")?
            .to_ascii_lowercase();
        if !allowed.contains(&directive.as_str()) {
            return Err("TARGET_UNVERIFIED: unsupported dynamic config/helper directive".into());
        }
    }
    // Freeze the deliberately restricted fixture route, never re-evaluate source includes/helpers.
    let frozen_hosts = runtime.join(if id == "primary" {
        "frozen_known_hosts".into()
    } else {
        format!("frozen_known_hosts-{id}")
    });
    let mut hosts = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(&frozen_hosts)?;
    use std::io::Write;
    hosts.write_all(&bounded_file(Path::new(&f.known_hosts_file), 16_384)?)?;
    drop(hosts);
    f.known_hosts_file = frozen_hosts.to_string_lossy().into();
    let snapshot = runtime.join(if id == "primary" {
        "frozen_config".into()
    } else {
        format!("frozen_config-{id}")
    });
    let data=format!("Host fixture\n HostName 127.0.0.1\n Port {}\n User probe\n IdentityFile \"{}\"\n IdentityAgent none\n IdentitiesOnly yes\n UserKnownHostsFile \"{}\"\n GlobalKnownHostsFile /dev/null\n StrictHostKeyChecking yes\n HostKeyAlgorithms ssh-ed25519\n BatchMode yes\n ConnectTimeout 5\n LogLevel ERROR\n",f.port,f.identity_file,f.known_hosts_file);
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(&snapshot)?;
    out.write_all(data.as_bytes())?;
    f.ssh_config_file = snapshot.to_string_lossy().into();
    Ok(f)
}

fn bounded_file(path: &Path, limit: usize) -> Result<Vec<u8>> {
    use std::io::Read;
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take((limit + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err("fixture input size limit".into());
    }
    Ok(bytes)
}

struct FrameReader<'a> {
    reader: &'a mut BufReader<UnixStream>,
    deadline: std::time::Instant,
}
impl FrameReader<'_> {
    fn budget(&self) -> io::Result<()> {
        let remain = self
            .deadline
            .checked_duration_since(std::time::Instant::now())
            .filter(|n| !n.is_zero())
            .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "frame deadline"))?;
        self.reader.get_ref().set_read_timeout(Some(remain))
    }
}
impl std::io::Read for FrameReader<'_> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.budget()?;
        std::io::Read::read(self.reader, out)
    }
}
impl BufRead for FrameReader<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.budget()?;
        self.reader.fill_buf()
    }
    fn consume(&mut self, n: usize) {
        self.reader.consume(n)
    }
}

fn spawn_pty(f: &Fixture, receiver: &str) -> Domain<Pty> {
    let command=match receiver {
        "cat"=>"stty raw -echo; printf PHASE0_READY; exec cat",
        "blocked"=>"stty raw -echo; printf PHASE0_READY; exec sleep 60",
        "flood"=>"stty raw -echo; printf PHASE0_READY; python3 -c 'import os; [os.write(1,b\"x\"*8192) for _ in range(512)]'; exec cat",
        _=>return Err("INVALID_PARAMS"),
    };
    let mut c = Command::new("/usr/bin/ssh");
    c.args(f.ssh_args(None)).args(["-tt", "fixture", command]);
    c.env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("TERM", "xterm-256color")
        .env("LANG", "C.UTF-8");
    Pty::spawn(&mut c).map_err(|_| "IO_ERROR")
}
