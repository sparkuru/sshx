use crate::native::{OscGate, Prefix, Pty};
use crate::sftp::Sftp;
use crate::{json, random_id, sha, Fixture, Result, Value};
use serde::Serialize;
use sshx_phase0_protocol::{read_frame, write_frame};
use std::fs::{self, OpenOptions};
use std::io::{BufReader, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct Case {
    experiment: String,
    case: String,
    status: String,
    expected: String,
    observations: Value,
    limitations: Vec<String>,
    cleanup_status: String,
}
struct Report {
    cases: Vec<Case>,
}
impl Report {
    fn case(&mut self, e: &str, id: &str, expected: &str, run: impl FnOnce() -> Result<Value>) {
        let r = run();
        let (status, obs) = match r {
            Ok(v) => (
                if v.get("status") == Some(&json!("fail")) {
                    "fail"
                } else {
                    "pass"
                },
                v,
            ),
            Err(err) => ("fail", json!({"error":err.to_string()})),
        };
        eprintln!("{e} {id}: {status}");
        self.cases.push(Case {
            experiment: e.into(),
            case: id.into(),
            status: status.into(),
            expected: expected.into(),
            observations: obs,
            limitations: vec![],
            cleanup_status: "owned handles closed by RAII".into(),
        });
    }
    fn gap(&mut self, e: &str, id: &str, reason: &str) {
        self.cases.push(Case {
            experiment: e.into(),
            case: id.into(),
            status: "not-run".into(),
            expected: reason.into(),
            observations: Value::Null,
            limitations: vec![reason.into()],
            cleanup_status: "no resources created".into(),
        });
    }
}
fn ensure(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_string().into())
    }
}
struct OwnedChild(Child);
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
struct Client {
    child: OwnedChild,
    input: Option<ChildStdin>,
    output: BufReader<ChildStdout>,
    counter: u64,
    pub id: String,
}
impl Client {
    fn connect(socket: &Path) -> Result<Self> {
        let mut child = Command::new(std::env::current_exe()?)
            .arg("client")
            .arg("--socket")
            .arg(socket)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let input = child.stdin.take().ok_or("stdin")?;
        let output = BufReader::new(child.stdout.take().ok_or("stdout")?);
        let mut c = Self {
            child: OwnedChild(child),
            input: Some(input),
            output,
            counter: 0,
            id: String::new(),
        };
        let hello = c.ok(
            "client.hello",
            json!({"api_versions":["1.0"],"client_version":"phase0"}),
        )?;
        c.id = hello["client_id"].as_str().ok_or("client id")?.into();
        Ok(c)
    }
    fn call(&mut self, method: &str, p: Value) -> Result<Value> {
        self.counter += 1;
        let req = json!({"jsonrpc":"2.0","id":self.counter.to_string(),"method":method,"params":p});
        write_frame(self.input.as_mut().ok_or("closed client")?, &req)?;
        read_frame(&mut self.output)?.ok_or_else(|| "Client EOF".into())
    }
    fn ok(&mut self, m: &str, p: Value) -> Result<Value> {
        let r = self.call(m, p)?;
        if let Some(e) = r.get("error") {
            Err(e["message"]
                .as_str()
                .unwrap_or("RPC error")
                .to_string()
                .into())
        } else {
            Ok(r["result"].clone())
        }
    }
    fn denied(&mut self, m: &str, p: Value, name: &str) -> Result<Value> {
        let r = self.call(m, p)?;
        ensure(
            r["error"]["data"]["name"] == name,
            &format!("expected {name}: {r}"),
        )?;
        Ok(r)
    }
    fn output(&mut self, sid: &str) -> Result<Vec<u8>> {
        let r = self.ok("probe.output", json!({"session_id":sid}))?;
        serde_json::from_value(r["data"].clone()).map_err(Into::into)
    }
    fn wait_output(&mut self, sid: &str, needle: &[u8]) -> Result<Vec<u8>> {
        let until = Instant::now() + Duration::from_secs(7);
        while Instant::now() < until {
            let out = self.output(sid)?;
            if out.windows(needle.len()).any(|v| v == needle) {
                return Ok(out);
            }
            thread::sleep(Duration::from_millis(10));
        }
        Err("PTY marker timeout".into())
    }
    fn wait_state(&mut self, id: &str, key: &str, value: &str) -> Result<Value> {
        let until = Instant::now() + Duration::from_secs(12);
        while Instant::now() < until {
            let r = self.ok("operation.get", json!({"operation_id":id}))?;
            if r[key] == value {
                return Ok(r);
            }
            thread::sleep(Duration::from_millis(10));
        }
        Err(format!("operation timeout {key}={value}").into())
    }
    fn close(mut self) {
        self.input.take();
        let _ = self.child.0.wait();
    }
}
fn op(c: &mut Client, sid: &str, epoch: u64, nonce: u64) -> Result<String> {
    let o=c.ok("action.invoke",json!({"session_id":sid,"writer_epoch":epoch,"trigger_nonce":nonce.to_string(),"action_id":"image"}))?;
    let id = o["binding"]["operation_id"]
        .as_str()
        .ok_or("operation id")?
        .to_string();
    c.ok(
        "grant.decide",
        json!({"operation_id":id,"decision":"allow"}),
    )?;
    Ok(id)
}
fn upload(c: &mut Client, id: &str, name: &str, bytes: &[u8]) -> Result<Value> {
    c.ok(
        "probe.upload",
        json!({"operation_id":id,"upload_key":"upload-1","basename":name,"data":bytes}),
    )?;
    c.wait_state(id, "barrier", "before_open")?;
    c.ok("probe.release", json!({"operation_id":id}))?;
    c.wait_state(id, "barrier", "before_commit")?;
    c.ok("probe.release", json!({"operation_id":id}))?;
    c.wait_state(id, "upload_state", "committed")
}

pub fn run(fixture_path: &Path, secondary_path: Option<&Path>, output: &Path) -> Result<()> {
    let fixture = Fixture::load(fixture_path)?;
    let secondary = secondary_path.map(Fixture::load).transpose()?;
    let runtime = std::env::temp_dir().join(format!("sshx-core-{}", random_id()?));
    fs::create_dir(&runtime)?;
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o700))?;
    let mut report = Report { cases: vec![] };
    let mut core_command = Command::new(std::env::current_exe()?);
    core_command.args(["core", "--fixture"]).arg(fixture_path);
    if let Some(path) = secondary_path {
        core_command.arg("--fixture-secondary").arg(path);
    }
    let mut core = OwnedChild(
        core_command
            .arg("--runtime")
            .arg(&runtime)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    );
    let socket = runtime.join("core.sock");
    let until = Instant::now() + Duration::from_secs(5);
    while !socket.exists() && Instant::now() < until {
        thread::sleep(Duration::from_millis(10));
    }
    let execution = (|| -> Result<()> {
        let mut a = Client::connect(&socket)?;
        let mut b = Client::connect(&socket)?;
        report.case("E1","two-native-client-processes","Two distinct native Clients negotiate one Core",||{ensure(a.child.0.id()!=b.child.0.id()&&a.id!=b.id,"client identity collision")?;Ok(json!({"core_pid":core.0.id(),"client_pids":[a.child.0.id(),b.child.0.id()],"client_id_distinct":true}))});
        report.case(
            "E1",
            "concurrent-core-start-lock",
            "A second Core cannot replace a running endpoint",
            || {
                let status = Command::new(std::env::current_exe()?)
                    .args(["core", "--fixture"])
                    .arg(fixture_path)
                    .arg("--runtime")
                    .arg(&runtime)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()?;
                ensure(!status.success(), "second Core started")?;
                Ok(json!({"second_start_exit":status.code()}))
            },
        );
        let sa = a.ok("session.start", json!({}))?;
        let sid = sa["session_id"].as_str().ok_or("sid")?.to_string();
        a.wait_output(&sid, b"PHASE0_READY")?;
        report.case(
            "E1",
            "writer-contention-and-resize",
            "Only initiating writer can attach/input/resize",
            || {
                b.denied(
                    "session.attach",
                    json!({"session_id":sid,"generation":1}),
                    "WRITER_BUSY",
                )?;
                b.denied(
                    "probe.input",
                    json!({"session_id":sid,"writer_epoch":1,"data":[88]}),
                    "STALE_BINDING",
                )?;
                b.denied(
                    "session.resize",
                    json!({"session_id":sid,"writer_epoch":1,"cols":90,"rows":30}),
                    "STALE_BINDING",
                )?;
                a.ok(
                    "session.resize",
                    json!({"session_id":sid,"writer_epoch":1,"cols":90,"rows":30}),
                )?;
                Ok(json!({"loser_bytes_written":0,"writer_resize_applied":true}))
            },
        );
        report.case("E2","native-pty-unicode-binary-prefix","Core-owned SSH PTY preserves byte patterns; prefix action writes no trigger bytes",||{let bytes="ASCII中文🦀".as_bytes();a.ok("probe.input",json!({"session_id":sid,"writer_epoch":1,"data":bytes}))?;a.wait_output(&sid,bytes)?;let mut p=Prefix::new();let mut forwarded=Vec::new();let mut actions=0;for chunk in [b"start".as_slice(),b"\x1d",b"p",b"\x1d\x1d",b"\x1dq",b"\x1d\x1b",b"end"]{let (x,n)=p.feed(chunk);forwarded.extend(x);actions+=n;}ensure(actions==1,"prefix action count")?;p.feed(&[29]);ensure(p.timeout()==[29],"prefix timeout literal")?;a.ok("probe.input",json!({"session_id":sid,"writer_epoch":1,"data":forwarded}))?;a.wait_output(&sid,b"end")?;Ok(json!({"unicode_length":bytes.len(),"prefix_actions":actions,"forwarded_sha256":sha(&forwarded),"oracle":"synthetic stty-raw cat, not Shell readiness"}))});
        report.case(
            "E1",
            "busy-core-stop",
            "Busy Core refuses normal stop",
            || {
                a.denied("core.stop", json!({}), "BUSY")?;
                Ok(json!({"ssh_preserved":true}))
            },
        );
        report.case("E1/E7","one-active-attachment-per-client","Starting another Session revokes the same Client's old writer/leases while retaining old SSH",||{
            let mut c=Client::connect(&socket)?;let first=c.ok("session.start",json!({}))?;let old=first["session_id"].as_str().ok_or("old session")?.to_string();c.wait_output(&old,b"PHASE0_READY")?;
            let before=c.ok("session.list",json!({}))?;
            c.denied("session.start",json!({"profile_id":4}),"INVALID_PARAMS")?;c.denied("session.start",json!({"receiver":4}),"INVALID_PARAMS")?;c.denied("session.start",json!({"profile_id":"unknown"}),"TARGET_UNVERIFIED")?;
            ensure(c.ok("session.list",json!({}))?==before,"invalid session target spawned a child or changed writer")?;
            c.ok("probe.input",json!({"session_id":old,"writer_epoch":1,"data":b"PRESERVED_WRITER"}))?;c.wait_output(&old,b"PRESERVED_WRITER")?;
            let oid=op(&mut c,&old,1,1)?;let second=c.ok("session.start",json!({}))?;let new=second["session_id"].as_str().ok_or("new session")?.to_string();c.wait_output(&new,b"PHASE0_READY")?;
            c.denied("probe.input",json!({"session_id":old,"writer_epoch":1,"data":[88]}),"STALE_BINDING")?;c.denied("session.resize",json!({"session_id":old,"writer_epoch":1,"cols":80,"rows":24}),"STALE_BINDING")?;
            c.denied("probe.upload",json!({"operation_id":oid,"upload_key":"u","basename":"stale.png","data":[1]}),"CANCELLED")?;
            ensure(unsafe{libc::kill(first["child_pid"].as_u64().ok_or("child pid")?as i32,0)}==0,"old SSH died on Client switch")?;
            c.ok("probe.input",json!({"session_id":new,"writer_epoch":1,"data":b"NEW_WRITER"}))?;c.wait_output(&new,b"NEW_WRITER")?;
            c.ok("session.detach",json!({"session_id":new}))?;let attached=c.ok("session.attach",json!({"session_id":old,"generation":1}))?;ensure(attached["writer_epoch"]==2,"reattach epoch")?;
            c.ok("session.attach",json!({"session_id":new,"generation":1}))?;c.denied("probe.input",json!({"session_id":old,"writer_epoch":2,"data":[88]}),"STALE_BINDING")?;
            c.ok("session.close",json!({"session_id":old}))?;c.ok("session.close",json!({"session_id":new}))?;
            Ok(json!({"start_and_attach_revoke_previous_writer":true,"old_operation_future_effects":0,"old_ssh_survived":true,"global_active_session":false,"invalid_target_and_receiver_types_denied_without_fallback":true}))
        });
        report.case(
            "E6",
            "wrong-role-identity-and-unknown-method",
            "Client cannot self-assign identity or obtain unavailable executor/history methods",
            || {
                a.denied(
                    "client.hello",
                    json!({"api_versions":["1.0"],"client_version":"x","client_id":"forged"}),
                    "INVALID_PARAMS",
                )?;
                a.denied("plugin.initialize", json!({}), "METHOD_NOT_FOUND")?;
                a.denied("executor.run", json!({}), "METHOD_NOT_FOUND")?;
                a.denied("history.query", json!({}), "METHOD_NOT_FOUND")?;
                Ok(json!({"forged_identity_denied":true,"reserved_methods_unavailable":true}))
            },
        );
        // A separate Core-owned remote PTY is the negative delivery oracle.
        let sb = b.ok("session.start", json!({}))?;
        let other = sb["session_id"].as_str().ok_or("sid")?.to_string();
        b.wait_output(&other, b"PHASE0_READY")?;
        let mut nonce = 0;
        report.case("E7","canonical-trigger-counter","Malformed counters fail without advancing admission high-water",||{
            for invalid in ["+1","01","","0","2","1.0","9007199254740992"] {
                a.denied("action.invoke",json!({"session_id":sid,"writer_epoch":1,"trigger_nonce":invalid,"action_id":"image"}),"INVALID_PARAMS")?;
            }
            Ok(json!({"malformed_or_skipped_counters":7,"next_admitted_counter_in_following_case":"1"}))
        });
        for transition in ["cancel", "revoke", "expire", "disable"] {
            nonce += 1;
            let id = op(&mut a, &sid, 1, nonce)?;
            report.case("E7",&format!("{transition}-before-open"),"Barrier prevents actual remote creation and PTY insertion",||{
                a.ok("probe.upload",json!({"operation_id":id,"upload_key":"upload-1","basename":"no-final.png","data":[1,2,3]}))?;a.wait_state(&id,"barrier","before_open")?;
                match transition{"cancel"=>{a.ok("operation.cancel",json!({"operation_id":id}))?;},"revoke"=>{a.ok("grant.revoke",json!({"operation_id":id}))?;},"expire"=>{a.ok("probe.expire",json!({"operation_id":id}))?;},_=>{a.ok("plugin.manage",json!({"enabled":false}))?;}}
                let r=a.wait_state(&id,"upload_state","failed")?;
                let rt=tokio::runtime::Runtime::new()?;let exists=rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;let exists=s.raw.lstat(format!("{}/{}",fixture.remote_root,id)).await.is_ok();s.close().await?;Ok::<_,Box<dyn std::error::Error+Send+Sync>>(exists)})?;ensure(!exists,"cancelled operation created remote directory")?;
                ensure(!b.output(&other)?.windows(12).any(|v|v==b"no-final.png"),"other PTY got stale path")?;if transition=="disable"{a.ok("plugin.manage",json!({"enabled":true}))?;}Ok(json!({"operation":r,"remote_path_absent":!exists,"other_pty_no_path":true}))
            });
        }
        nonce += 1;
        let mid = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","revoke-during-real-stream","Partial remote bytes exist at deterministic streaming barrier; revocation cleans only owned partial and never inserts",||{
            a.ok("probe.upload",json!({"operation_id":mid,"upload_key":"u","basename":"stream.png","data":[42,43,44],"hold_mid_stream":true}))?;a.wait_state(&mid,"barrier","before_open")?;a.ok("probe.release",json!({"operation_id":mid}))?;a.wait_state(&mid,"barrier","during_stream")?;
            let path=format!("{}/{}/.partial",fixture.remote_root,mid);let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.read_bounded(&path,3).await?==[42],"partial stream oracle")?;s.close().await})?;
            a.ok("grant.revoke",json!({"operation_id":mid}))?;a.wait_state(&mid,"upload_state","failed")?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.raw.lstat(format!("{}/{}",fixture.remote_root,mid)).await.is_err(),"owned partial cleanup")?;s.close().await})?;
            Ok(json!({"actual_remote_bytes_before_revoke":1,"temporary_removed":true,"final_created":false,"insert_bytes":0}))
        });
        for stage in ["mkdir", "open", "write", "close"] {
            nonce += 1;
            let stage_id = op(&mut a, &sid, 1, nonce)?;
            report.case("E7",&format!("cancel-before-sftp-{stage}"),"Cancellation wins before the next external request; only owned temporary resources are cleaned",||{
                if stage=="mkdir" {
                    for invalid in [json!("x".repeat(65_536)),json!("unknown"),json!(123)] {
                        a.denied("probe.upload",json!({"operation_id":stage_id,"upload_key":"u","basename":"step.png","data":[1,2,3],"hold_step":invalid}),"INVALID_PARAMS")?;
                    }
                    a.denied("probe.upload",json!({"operation_id":stage_id,"upload_key":"u","basename":"step.png","data":[1,2,3],"hold_mid_stream":"yes"}),"INVALID_PARAMS")?;
                    let unchanged=a.ok("operation.get",json!({"operation_id":stage_id}))?;ensure(unchanged["upload_state"]=="none"&&unchanged["external_steps_admitted"]==0,"invalid fault controls consumed upload admission")?;
                }
                a.ok("probe.upload",json!({"operation_id":stage_id,"upload_key":"u","basename":"step.png","data":[1,2,3],"hold_step":stage}))?;
                a.wait_state(&stage_id,"barrier","before_open")?;a.ok("probe.release",json!({"operation_id":stage_id}))?;
                let before=a.wait_state(&stage_id,"barrier",&format!("before_{stage}"))?;
                a.ok("operation.cancel",json!({"operation_id":stage_id}))?;
                let after=a.wait_state(&stage_id,"upload_state","failed")?;
                ensure(before["external_steps_admitted"]==after["external_steps_admitted"]&&!after["cleanup_pending"].as_bool().unwrap_or(true),"step admitted after cancellation or cleanup pending")?;
                let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.raw.lstat(format!("{}/{}",fixture.remote_root,stage_id)).await.is_err(),"cancelled step left remote resource")?;s.close().await})?;
                Ok(json!({"stage":stage,"steps_before_cancel":before["external_steps_admitted"],"steps_after_cancel":after["external_steps_admitted"],"owned_directory_absent":true,"insert_bytes":0,"invalid_fault_control_checks":if stage=="mkdir" {json!({"cases":4,"oversized_bytes":65536,"admission_consumed":false})}else{Value::Null}}))
            });
        }
        nonce += 1;
        let same_a = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","same-target-concurrent-uploads","Two Sessions sharing one target keep simultaneous upload ledgers, file hashes and PTY writes distinct",||{
            let same_b=op(&mut b,&other,1,1)?;let one=png_payload(11)?;let two=png_payload(22)?;
            for (client,id,data) in [(&mut a,&same_a,&one),(&mut b,&same_b,&two)] {
                client.ok("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"same-target.png","data":data,"hold_mid_stream":true}))?;client.wait_state(id,"barrier","before_open")?;client.ok("probe.release",json!({"operation_id":id}))?;client.wait_state(id,"barrier","during_stream")?;
            }
            let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;for id in [&same_a,&same_b]{ensure(s.read_bounded(&format!("{}/{}/.partial",fixture.remote_root,id),1).await?==[137],"simultaneous same-target partial missing")?;}s.close().await})?;
            let mut paths=Vec::new();
            for (client,id) in [(&mut a,&same_a),(&mut b,&same_b)] {
                client.ok("probe.release",json!({"operation_id":id}))?;client.wait_state(id,"barrier","before_commit")?;client.ok("probe.release",json!({"operation_id":id}))?;let record=client.wait_state(id,"upload_state","committed")?;paths.push(record["result"]["transfer_path"].as_str().ok_or("path")?.to_string());
            }
            rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.read_bounded(&paths[0],one.len()).await?==one&&s.read_bounded(&paths[1],two.len()).await?==two,"same-target hashes crossed")?;s.close().await})?;
            for (client,id,path,session) in [(&mut a,&same_a,&paths[0],&sid),(&mut b,&same_b,&paths[1],&other)] {
                client.ok("probe.insert",json!({"operation_id":id,"insert_key":"i","text":path,"context_revision":1,"mapping":true}))?;client.wait_output(session,path.as_bytes())?;
            }
            ensure(!a.output(&sid)?.windows(paths[1].len()).any(|v|v==paths[1].as_bytes())&&!b.output(&other)?.windows(paths[0].len()).any(|v|v==paths[0].as_bytes()),"same-target input crossed")?;
            for path in &paths{clean_remote(&fixture,path)?;}
            Ok(json!({"same_profile":"primary","simultaneous_partial_files":2,"distinct_session_ids":[sid,other],"sha256":[sha(&one),sha(&two)],"cross_session_input_bytes":0}))
        });
        nonce += 1;
        let churn = op(&mut a, &sid, 1, nonce)?;
        report.case("E6","bounded-grant-ticket-churn","Renewed grants/tickets replace bounded owned records and consumed tickets cannot replay",||{
            let first=a.ok("grant.decide",json!({"operation_id":churn,"decision":"allow"}))?;
            let mut old=String::new();for _ in 0..300{
                ensure(a.ok("grant.decide",json!({"operation_id":churn,"decision":"allow"}))?["grants"]==first["grants"],"grant renewal appended history")?;
                let issued=a.ok("probe.ticket.issue",json!({"session_id":sid,"writer_epoch":1,"purpose":"input"}))?;
                let ticket=issued["ticket"].as_str().ok_or("ticket")?.to_string();
                if !old.is_empty(){a.denied("probe.ticket.consume",json!({"ticket":old}),"NOT_FOUND")?;}old=ticket;
            }
            b.denied("probe.ticket.consume",json!({"ticket":old}),"CAPABILITY_DENIED")?;
            a.ok("probe.ticket.consume",json!({"ticket":old}))?;a.denied("probe.ticket.consume",json!({"ticket":old}),"NOT_FOUND")?;
            let counts=a.ok("probe.metadata",json!({}))?;ensure(counts["tickets"]==0&&counts["metadata_accounted_upper_bound"].as_u64().unwrap_or(u64::MAX)<16*1024*1024,"metadata budget")?;
            Ok(json!({"iterations":300,"counts":counts,"stream_ticket_scope":"actual IPC admission/consume; data stream codec tested independently"}))
        });
        nonce += 1;
        let id = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","cancel-before-commit","Actual remote temp is removed and final absent after cancel barrier",||{a.ok("probe.upload",json!({"operation_id":id,"upload_key":"upload-1","basename":"cancel.png","data":[0,255,1,2]}))?;a.wait_state(&id,"barrier","before_open")?;a.ok("probe.release",json!({"operation_id":id}))?;a.wait_state(&id,"barrier","before_commit")?;a.ok("operation.cancel",json!({"operation_id":id}))?;let result=a.wait_state(&id,"upload_state","failed")?;let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.raw.lstat(format!("{}/{}",fixture.remote_root,id)).await.is_err(),"temporary directory remains")?;s.close().await})?;Ok(json!({"operation":result,"temp_and_final_absent":true}))});
        for transition in ["cancel", "revoke"] {
            nonce += 1;
            let post = op(&mut a, &sid, 1, nonce)?;
            report.case("E7",&format!("{transition}-after-confirmed-commit"),"Cancellation/revocation after actual commit retains verified final but blocks new insertion",||{
                let data=synthetic_payload()?;a.ok("probe.upload",json!({"operation_id":post,"upload_key":"upload-1","basename":"post.png","data":data,"hold_after_commit":true}))?;
                a.wait_state(&post,"barrier","before_open")?;a.ok("probe.release",json!({"operation_id":post}))?;a.wait_state(&post,"barrier","before_commit")?;a.ok("probe.release",json!({"operation_id":post}))?;let committed=a.wait_state(&post,"barrier","after_commit")?;
                let path=committed["result"]["transfer_path"].as_str().ok_or("path")?.to_string();
                let method=if transition=="cancel"{"operation.cancel"}else{"grant.revoke"};a.ok(method,json!({"operation_id":post}))?;
                a.denied("probe.insert",json!({"operation_id":post,"insert_key":"i","text":path,"context_revision":1,"mapping":true}),if transition=="cancel"{"CANCELLED"}else{"CAPABILITY_DENIED"})?;
                a.ok("probe.release",json!({"operation_id":post}))?;a.wait_state(&post,"barrier","completed")?;
                let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.read_bounded(&path,data.len()).await?==data,"postcommit final changed")?;s.close().await})?;clean_remote(&fixture,&path)?;
                Ok(json!({"file_commit_confirmed_before_transition":true,"new_insert_bytes":0,"automatic_rollback":false,"remote_sha256":sha(&data)}))
            });
        }
        nonce += 1;
        let id = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","upload-zero-byte-refresh-and-response-loss","Insertion retry after refused context reuses committed file; lost ack never writes twice",||{
            let current=a.ok("probe.capture.begin",json!({"operation_id":id}))?;let (data,image_evidence)=capture_after_trigger(&runtime,&current["binding"],None)?;let sealed=crate::seal::validate(&data)?;let up=upload(&mut a,&id,"sealed.png",&data)?;let path=up["result"]["transfer_path"].as_str().ok_or("path")?.to_string();
            a.ok("probe.upload",json!({"operation_id":id,"upload_key":"upload-1","basename":"sealed.png","data":data}))?;a.denied("probe.upload",json!({"operation_id":id,"upload_key":"upload-1","basename":"sealed.png","data":[9,9,9]}),"DEDUP_CONFLICT")?;
            a.denied("probe.insert",json!({"operation_id":id,"insert_key":"insert-1","text":path,"context_revision":0,"mapping":true}),"CONTEXT_UNKNOWN")?;
            let before=a.output(&sid)?.len();let first=a.ok("probe.insert",json!({"operation_id":id,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true,"lose_ack":true}))?;a.wait_output(&sid,path.as_bytes())?;let after=a.output(&sid)?;let replay=a.ok("probe.insert",json!({"operation_id":id,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true}))?;
            thread::sleep(Duration::from_millis(40));let final_out=a.output(&sid)?;ensure(first["insert_state"]=="indeterminate"&&replay["insert_state"]=="indeterminate","lost ack state")?;ensure(after==final_out,"replay wrote bytes")?;ensure(!b.output(&other)?.windows(path.len()).any(|v|v==path.as_bytes()),"cross-session insertion")?;
            clean_remote(&fixture,&path)?;Ok(json!({"upload_sha256":sha(&data),"sealed_sha256":sealed.2,"image_flow":image_evidence,"pty_length_before":before,"insert_bytes":first["bytes_written"],"repeat_wrote_bytes":false,"other_pty_path_absent":true,"mapping":"explicit synthetic receiver attestation only; no general Shell mapping"}))
        });
        nonce += 1;
        let partial_id = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","partial-pty-write-no-replay","Observed prefix written to actual SSH PTY cannot be replayed",||{let up=upload(&mut a,&partial_id,"partial.png",&synthetic_payload()?)?;let path=up["result"]["transfer_path"].as_str().ok_or("path")?.to_string();let before_write=a.output(&sid)?.len();let first=a.ok("probe.insert",json!({"operation_id":partial_id,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true,"partial":true}))?;ensure(first["insert_state"]=="partial"&&first["bytes_written"]==3,"partial write")?;let until=Instant::now()+Duration::from_secs(3);while a.output(&sid)?.len()<before_write+3&&Instant::now()<until{thread::sleep(Duration::from_millis(5));}let before=a.output(&sid)?;ensure(before.len()==before_write+3,"actual partial PTY output length")?;a.ok("probe.insert",json!({"operation_id":partial_id,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true}))?;thread::sleep(Duration::from_millis(40));ensure(a.output(&sid)?==before,"partial replay")?;clean_remote(&fixture,&path)?;Ok(json!({"actual_prefix_bytes":3,"automatic_replay":false,"fault_injection":"Core write slice cut at admission, actual syscall/remote PTY observed"}))});
        report.case("E7","network-child-loss-new-generation","Loss of owned SSH invalidates effects; explicit reconnect starts new PTY/generation and never receives old input",||{
            let mut c=Client::connect(&socket)?;let started=c.ok("session.start",json!({}))?;let csid=started["session_id"].as_str().ok_or("session")?.to_string();c.wait_output(&csid,b"PHASE0_READY")?;let old=op(&mut c,&csid,1,1)?;
            c.ok("probe.upload",json!({"operation_id":old,"upload_key":"u","basename":"generation.png","data":[1,2,3]}))?;c.wait_state(&old,"barrier","before_open")?;
            let pid=started["child_pid"].as_u64().ok_or("pid")?as i32;ensure(unsafe{libc::kill(pid,libc::SIGTERM)}==0,"owned SSH signal")?;
            c.wait_state(&old,"upload_state","failed")?;let next=c.ok("session.reconnect",json!({"session_id":csid,"writer_epoch":1}))?;
            ensure(next["generation"]==2&&next["writer_epoch"]==2,"new generation/epoch")?;c.wait_output(&csid,b"PHASE0_READY")?;
            c.denied("probe.insert",json!({"operation_id":old,"insert_key":"old","text":"old-path","context_revision":1,"mapping":true}),"CANCELLED")?;
            c.ok("probe.input",json!({"session_id":csid,"writer_epoch":2,"data":b"FRESH_GENERATION"}))?;let output=c.wait_output(&csid,b"FRESH_GENERATION")?;ensure(!output.windows(8).any(|x|x==b"old-path"),"stale insertion on reconnect")?;c.ok("session.close",json!({"session_id":csid}))?;
            Ok(json!({"owned_channel_failure":"SIGTERM only generated-fixture ssh child, no host network alteration","generation":2,"stale_input_bytes":0,"remote_job_recovery_claim":false}))
        });
        report.case("E7","actual-insert-ipc-response-loss","Actual initiating Client loses IPC reply after admitted PTY write; another user Client can query redacted retained status",||{
            let mut c=Client::connect(&socket)?;let started=c.ok("session.start",json!({}))?;let csid=started["session_id"].as_str().ok_or("session id")?.to_string();c.wait_output(&csid,b"PHASE0_READY")?;
            let oid=op(&mut c,&csid,1,1)?;let up=upload(&mut c,&oid,"reply-loss.png",&synthetic_payload()?)?;let path=up["result"]["transfer_path"].as_str().ok_or("path")?.to_string();
            ensure(c.call("probe.insert",json!({"operation_id":oid,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true,"drop_response":true})).is_err(),"IPC reply was delivered")?;
            let observed=b.wait_output(&csid,path.as_bytes())?;let status=b.ok("operation.get",json!({"operation_id":oid}))?;
            ensure(status["insert_state"]=="committed"&&status["bytes_written"].as_u64()==Some(path.len()as u64),"retained insertion outcome")?;
            ensure(observed.windows(path.len()).filter(|v|*v==path.as_bytes()).count()==1,"duplicate PTY text")?;
            b.denied("probe.insert",json!({"operation_id":oid,"insert_key":"insert-1","text":path,"context_revision":1,"mapping":true}),"CAPABILITY_DENIED")?;
            b.ok("session.close",json!({"session_id":csid}))?;clean_remote(&fixture,&path)?;
            Ok(json!({"ipc_reply_received":false,"retained_insert_state":status["insert_state"],"actual_pty_occurrences":1,"retarget_retry_denied":true,"fault_boundary":"Core closes real Client IPC after syscall, before framed response"}))
        });
        report.case("E7","cancel-and-switch-during-private-capture","An actual independent private-image process may finish after cancellation/switch; its original bound bytes cannot reach any SFTP target or PTY",||{
            let mut c=Client::connect(&socket)?;let started=c.ok("session.start",json!({}))?;let csid=started["session_id"].as_str().ok_or("capture session")?.to_string();c.wait_output(&csid,b"PHASE0_READY")?;
            let oid=op(&mut c,&csid,1,1)?;let admitted=c.ok("probe.capture.begin",json!({"operation_id":oid}))?;
            c.denied("probe.capture.begin",json!({"operation_id":oid}),"DEDUP_CONFLICT")?;
            let mut next_sid=String::new();
            let mut hook=||->Result<()> {
                c.ok("operation.cancel",json!({"operation_id":oid}))?;c.ok("session.detach",json!({"session_id":csid}))?;
                let started=c.ok("session.start",json!({}))?;next_sid=started["session_id"].as_str().ok_or("new capture target")?.to_string();c.wait_output(&next_sid,b"PHASE0_READY")?;Ok(())
            };
            let (data,image)=capture_after_trigger(&runtime,&admitted["binding"],Some(&mut hook))?;
            c.denied("probe.upload",json!({"operation_id":oid,"upload_key":"u","basename":"late.png","data":data}),"CANCELLED")?;
            c.denied("probe.capture.begin",json!({"operation_id":oid}),"CANCELLED")?;
            let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(&fixture,None).await?;ensure(s.raw.lstat(format!("{}/{}",fixture.remote_root,oid)).await.is_err(),"late capture created target file")?;s.close().await})?;
            ensure(!c.output(&next_sid)?.windows(8).any(|v|v==b"late.png"),"late capture input retargeted")?;
            c.ok("session.close",json!({"session_id":csid}))?;c.ok("session.close",json!({"session_id":next_sid}))?;
            Ok(json!({"actual_capture_pipeline":image,"cancel_barrier":"child process started and alive before private capture/encode completion","second_capture_admitted":false,"late_upload_admitted":false,"actual_remote_directory_absent":true,"new_pty_path_bytes":0}))
        });
        nonce += 1;
        let expired = op(&mut a, &sid, 1, nonce)?;
        report.case("E7","trigger-highwater-after-retention","Removed retained record cannot recreate an old nonce",||{a.ok("probe.forget",json!({"operation_id":expired}))?;a.denied("action.invoke",json!({"session_id":sid,"writer_epoch":1,"trigger_nonce":nonce.to_string(),"action_id":"image"}),"EXPIRED")?;Ok(json!({"retained_record_removed":true,"old_nonce_recreated":false,"clock":"explicit test retention-removal hook; not elapsed-wallclock evidence"}))});
        nonce += 1;
        let switched = op(&mut a, &sid, 1, nonce)?;
        if let Some(second) = secondary.as_ref() {
            report.case("E7","two-target-concurrent-uploads-and-midstream-switch","One Core keeps distinct prevalidated targets and immutable operation/PTY destinations",||{
                ensure(fixture.port!=second.port&&fixture.container_id!=second.container_id,"second fixture not distinct")?;
                let mut c=Client::connect(&socket)?;
                let start=c.ok("session.start",json!({"profile_id":"secondary"}))?;let ssid=start["session_id"].as_str().ok_or("session id")?.to_string();c.wait_output(&ssid,b"PHASE0_READY")?;
                let x=op(&mut c,&ssid,1,1)?;let y=op(&mut a,&sid,1,nonce+1)?;
                let one=png_payload(17)?;let two=png_payload(99)?;ensure(sha(&one)!=sha(&two),"payloads not distinct")?;
                for (client,id,data) in [(&mut a,&y,&one),(&mut c,&x,&two)] {
                    client.ok("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"target.png","data":data,"hold_mid_stream":true}))?;
                    client.wait_state(id,"barrier","before_open")?;client.ok("probe.release",json!({"operation_id":id}))?;client.wait_state(id,"barrier","during_stream")?;
                }
                let read_partial=|f:&Fixture,id:&str|->Result<()> {let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(f,None).await?;ensure(s.read_bounded(&format!("{}/{}/.partial",f.remote_root,id),1).await?==[137],"actual PNG partial missing")?;s.close().await})};
                read_partial(&fixture,&y)?;read_partial(second,&x)?;
                a.ok("probe.release",json!({"operation_id":y}))?;a.wait_state(&y,"barrier","before_commit")?;a.ok("probe.release",json!({"operation_id":y}))?;
                c.ok("probe.release",json!({"operation_id":x}))?;c.wait_state(&x,"barrier","before_commit")?;c.ok("probe.release",json!({"operation_id":x}))?;
                let py=a.wait_state(&y,"upload_state","committed")?;let px=c.wait_state(&x,"upload_state","committed")?;
                ensure(py["binding"]["profile_id"]=="primary"&&px["binding"]["profile_id"]=="secondary","operation target changed")?;
                let yp=py["result"]["transfer_path"].as_str().ok_or("primary path")?.to_string();let xp=px["result"]["transfer_path"].as_str().ok_or("secondary path")?.to_string();
                for (f,path,other_path,data) in [(&fixture,&yp,&xp,&one),(second,&xp,&yp,&two)] {
                    let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(f,None).await?;ensure(s.read_bounded(path,data.len()).await?==*data&&s.raw.lstat(other_path).await.is_err(),"cross-target file or wrong bytes")?;s.close().await})?;
                }
                for (client,id,path,target_sid) in [(&mut a,&y,&yp,&sid),(&mut c,&x,&xp,&ssid)] {
                    client.ok("probe.insert",json!({"operation_id":id,"insert_key":"i","text":path,"context_revision":1,"mapping":true}))?;client.wait_output(target_sid,path.as_bytes())?;
                }
                ensure(!a.output(&sid)?.windows(xp.len()).any(|v|v==xp.as_bytes())&&!c.output(&ssid)?.windows(yp.len()).any(|v|v==yp.as_bytes()),"cross-target PTY delivery")?;
                let cancel=op(&mut c,&ssid,1,2)?;c.ok("probe.upload",json!({"operation_id":cancel,"upload_key":"u","basename":"switched.png","data":two,"hold_mid_stream":true}))?;c.wait_state(&cancel,"barrier","before_open")?;c.ok("probe.release",json!({"operation_id":cancel}))?;c.wait_state(&cancel,"barrier","during_stream")?;
                c.ok("session.detach",json!({"session_id":ssid}))?;
                let next=c.ok("session.start",json!({"profile_id":"primary"}))?;let next_sid=next["session_id"].as_str().ok_or("switch target")?.to_string();c.wait_output(&next_sid,b"PHASE0_READY")?;
                c.wait_state(&cancel,"upload_state","failed")?;
                let rt=tokio::runtime::Runtime::new()?;rt.block_on(async{let s=Sftp::connect(second,None).await?;ensure(s.raw.lstat(format!("{}/{}",second.remote_root,cancel)).await.is_err(),"switched partial not cleaned")?;s.close().await})?;
                ensure(!c.output(&next_sid)?.windows(12).any(|v|v==b"switched.png"),"switch destination received stale input")?;
                c.ok("session.close",json!({"session_id":next_sid}))?;c.ok("session.close",json!({"session_id":ssid}))?;clean_remote(&fixture,&yp)?;clean_remote(second,&xp)?;nonce+=1;
                Ok(json!({"core_pid":core.0.id(),"profile_ids":["primary","secondary"],"fixture_ids":[fixture.run_id,second.run_id],"simultaneous_open_partial_files":2,"target_sha256":[sha(&one),sha(&two)],"cross_target_file_absent":true,"cross_target_pty_bytes":0,"switch_during_real_stream_cancelled":true,"scope":"two independent servers on one Docker host"}))
            });
        } else {
            report.gap("E7","two-target-concurrent-uploads","Supply --fixture-secondary with a second generated owned server for different-target routing evidence");
        }
        report.case("E7","detach-switch-before-commit","Detached initiating writer cannot deliver to another real Session",||{a.ok("probe.upload",json!({"operation_id":switched,"upload_key":"upload-1","basename":"switch.png","data":[1,2,3]}))?;a.wait_state(&switched,"barrier","before_open")?;a.ok("probe.release",json!({"operation_id":switched}))?;a.wait_state(&switched,"barrier","before_commit")?;a.ok("session.detach",json!({"session_id":sid}))?;a.wait_state(&switched,"upload_state","failed")?;b.denied("probe.insert",json!({"operation_id":switched,"insert_key":"x","text":"x","context_revision":1,"mapping":true}),"CAPABILITY_DENIED")?;Ok(json!({"other_pty_sha256":sha(&b.output(&other)?),"cross_client_operation_denied":true}))});
        report.case("E1","client-crash-preserves-ssh","Killing Client revokes writer while Core-owned SSH survives and fresh writer gets epoch",||{let old_pid=sa["child_pid"].as_u64().ok_or("pid")? as i32;a.child.0.kill()?;a.child.0.wait()?;let until=Instant::now()+Duration::from_secs(2);let mut attached=None;while Instant::now()<until{let r=b.call("session.attach",json!({"session_id":sid,"generation":1}))?;if r.get("result").is_some(){attached=Some(r["result"].clone());break;}thread::sleep(Duration::from_millis(10));}let attached=attached.ok_or("writer not revoked")?;ensure(unsafe{libc::kill(old_pid,0)}==0,"ssh died on Client loss")?;ensure(attached["writer_epoch"].as_u64().unwrap_or(0)>1,"epoch not advanced")?;Ok(json!({"ssh_pid_alive":old_pid,"new_writer_epoch":attached["writer_epoch"]}))});
        report.case("E1/E2/E5","bounded-output-and-blocked-input","A stopped receiver cannot block RPC; output overflow revokes writer and keeps draining with finite storage",||bounded_native_io(&socket));
        b.ok("session.close", json!({"session_id":sid}))?;
        b.ok("session.close", json!({"session_id":other}))?;
        report.case(
            "E6",
            "retained-session-client-cap",
            "Exited Session/disconnected Client churn hits finite admission caps",
            || {
                let existing = b.ok("probe.metadata", json!({}))?["sessions"]
                    .as_u64()
                    .ok_or("session count")?;
                for _ in existing..32 {
                    let s = b.ok("session.start", json!({}))?;
                    b.ok("session.close", json!({"session_id":s["session_id"]}))?;
                }
                b.denied("session.start", json!({}), "BUSY")?;
                let existing = b.ok("probe.metadata", json!({}))?["clients"]
                    .as_u64()
                    .ok_or("client count")?;
                for _ in existing..16 {
                    Client::connect(&socket)?.close();
                }
                ensure(
                    Client::connect(&socket).is_err(),
                    "retained Client cap escaped",
                )?;
                let counts = b.ok("probe.metadata", json!({}))?;
                ensure(
                    counts["sessions"] == 32 && counts["clients"] == 16,
                    "retained counts",
                )?;
                Ok(counts)
            },
        );
        b.ok("core.stop", json!({}))?;
        b.close();
        let _ = core.0.wait();
        Ok(())
    })();
    if let Err(e) = execution {
        report.cases.push(Case {
            experiment: "E1–E7".into(),
            case: "suite-prerequisite-or-flow".into(),
            status: "fail".into(),
            expected: "Shared Core flow completes".into(),
            observations: json!({"error":e.to_string()}),
            limitations: vec![],
            cleanup_status: "owned Core killed; remote known effects may require fixture teardown"
                .into(),
        });
    }
    report.case(
        "E1/E4",
        "endpoint-and-dynamic-config-refusal",
        "Owned symlink endpoint and executable dynamicconfig are refused without effect",
        || endpoint_and_config_refusal(fixture_path),
    );
    report.case(
        "E5",
        "actual-core-crash-new-boot",
        "Forced owned Core loss invalidates IPC/B; new Core starts empty and owned ssh is reaped",
        || core_crash(fixture_path),
    );
    report.case(
        "E1",
        "passthrough-byte-stdio-exit",
        "Native passthrough preserves independent binary stdout/stderr and exit status",
        || passthrough(&fixture),
    );
    report.case(
        "E2",
        "password-and-host-key",
        "OpenSSH owns password interaction and refuses mismatched expected host key",
        || password_and_hostkey(&fixture),
    );
    report.case(
        "E2",
        "remote-control-resize-exit",
        "Owned native terminal exercises remote control keys, job control and resize",
        || native_terminal(&fixture),
    );
    report.case(
        "E2",
        "owned-agent-authentication",
        "Auxiliary SSH authenticates with only a generated key in an owned agent",
        || owned_agent(&fixture),
    );
    report.case(
        "E2",
        "clipboard-control-gate",
        "Split/C1 OSC52/5522 denied; UTF8 remains intact and overflow stops display",
        osc,
    );
    report.case("E4","native-route-reuse-chroot","Native OpenSSH maps trusted jump/proxy/config and demonstrates real master reuse; chroot namespace stays distinct",||routes_and_reuse(&fixture));
    report.case(
        "E4",
        "sftp-backend-comparison",
        "Small A/B probes record no-clobber difference and readback content",
        || transfer_cases(&fixture),
    );
    report.case("E7","sftp-commit-reply-loss","Actual owned SFTP reply loss yields uncertain delivery and read-only known-path reconciliation",||lost_commit_ack(&fixture));
    report.case(
        "E6",
        "independent-python-plugin",
        "Non-Rust public stdio framing negotiates without Core-private code",
        || python_plugin(&fixture),
    );
    drop(core);
    let mut cleanup = Vec::new();
    let image_dir = runtime.join("image");
    if image_dir.is_dir() && !fs::symlink_metadata(&image_dir)?.file_type().is_symlink() {
        for name in ["sealed.png", "report.json", "binding.json"] {
            let path = image_dir.join(name);
            if let Ok(m) = fs::symlink_metadata(&path) {
                if m.is_file() {
                    fs::remove_file(path)?;
                }
            }
        }
        fs::remove_dir(image_dir)?;
    }
    for name in [
        "core.sock",
        "core.lock",
        "frozen_config",
        "frozen_known_hosts",
        "frozen_config-secondary",
        "frozen_known_hosts-secondary",
    ] {
        let path = runtime.join(name);
        if let Ok(m) = fs::symlink_metadata(&path) {
            if !m.file_type().is_symlink() {
                fs::remove_file(&path)?;
                cleanup.push(name);
            }
        }
    }
    fs::remove_dir(&runtime)?;
    let failed = report.cases.iter().any(|c| c.status == "fail");
    let gaps = report.cases.iter().any(|c| c.status == "not-run");
    let document = json!({"schema":1,"fixture_id":fixture.run_id,"secondary_fixture_id":secondary.as_ref().map(|f|&f.run_id),"environment":{"client_os":"Linux native","architecture":std::env::consts::ARCH,"terminal_host":"native Unix PTY / synthetic raw cat","local_shell":"no user shell used","display":"none except explicitly private-Xvfb child image suites","ssh_build":Command::new("ssh").arg("-V").output().ok().map(|x|String::from_utf8_lossy(&x.stderr).trim().to_string()),"backend":"system ssh pipes + russh-sftp3.0.1 probe","auth_mode":"generated key/password"},"cases":report.cases,"cleanup":{"runtime_removed":!runtime.exists(),"removed":cleanup},"controlled_linux_cases_passed":!failed&&!gaps,"phase0_ready":false,"supplemental_evidence":[{"fault_group":"E1 other-user IPC","report":"research/ipc-crossuid.json","status":"separately-executed","scope":"native kernel DAC and SO_PEERCRED refusal; not rerun inside this suite"},{"fault_group":"E6 packages/events","report":"research/check-plugin-results.json","status":"also-covered-by-independent-python-plugin-case","scope":"bounded test broker/USTAR gate, not production installer"}],"scope_limits":[{"surface":"physical IME/hardware key/MFA; Windows/macOS/Wayland/native non-Linux servers; reboot","status":"deferred-outside-approved-fixtures"},{"surface":"Core dynamic Include/Match/proxy config binding","status":"rejected-before-helper-execution; native trusted jump/proxy/reuse and chroot separately executed"},{"surface":"full Phase1 product API, installer, real-Shell context and every dispatch race permutation","status":"not-implemented; deterministic representative barriers exercise the shared admission checks"}],"limitations":["Probe-specific RPC methods differ from Phase1 public API and accept only synthetic owned fixtures","No all-platform or full-product support claim","Only explicitly generated controlled roots are advertised; lstat cannot defeat a malicious same-UID remote directory swap","Synthetic probe.capture.begin admits one pipeline; actual child may finish after cancellation, then Core rejects its late result"]});
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(output)?;
    serde_json::to_writer_pretty(&mut f, &document)?;
    f.write_all(b"\n")?;
    if failed {
        Err("native case assertions failed; see report".into())
    } else if gaps {
        Err("native cases completed with required unverified subset; see report".into())
    } else {
        Ok(())
    }
}
fn synthetic_payload() -> Result<Vec<u8>> {
    if let Ok(p) = std::env::var("SSHX_PHASE0_PNG") {
        let m = fs::metadata(&p)?;
        if m.len() > 65_536 {
            return Err("synthetic PNG probe byte limit".into());
        }
        Ok(fs::read(p)?)
    } else {
        Ok(b"synthetic sealed byte fixture; not an image claim".to_vec())
    }
}
fn png_payload(seed: u8) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()?
            .write_image_data(&[seed, 0, 255, 127])?;
    }
    Ok(bytes)
}
fn bounded_native_io(socket: &Path) -> Result<Value> {
    let mut c = Client::connect(socket)?;
    let start = c.ok("session.start", json!({"receiver":"blocked"}))?;
    let sid = start["session_id"].as_str().ok_or("session")?.to_string();
    c.wait_output(&sid, b"PHASE0_READY")?;
    let oid = op(&mut c, &sid, 1, 1)?;
    let data = vec![b'x'; 16_384];
    let until = Instant::now() + Duration::from_secs(10);
    let mut accepted = 0u64;
    let mut busy = false;
    for _ in 0..1024 {
        if Instant::now() >= until {
            break;
        }
        let r = c.call(
            "probe.input",
            json!({"session_id":sid,"writer_epoch":1,"data":data}),
        )?;
        if r["error"]["data"]["name"] == "BUSY" {
            busy = true;
            break;
        }
        accepted += r["result"]["written"].as_u64().ok_or("input result")?;
    }
    ensure(
        busy,
        "blocked receiver never reached bounded input admission",
    )?;
    let before = Instant::now();
    c.ok("operation.cancel", json!({"operation_id":oid}))?;
    c.ok("session.list", json!({}))?;
    let response_ms = before.elapsed().as_millis();
    ensure(response_ms < 1000, "blocked PTY stalled cancel/status")?;
    c.ok("session.close", json!({"session_id":sid}))?;
    let flood = c.ok("session.start", json!({"receiver":"flood"}))?;
    let fid = flood["session_id"]
        .as_str()
        .ok_or("flood session")?
        .to_string();
    let until = Instant::now() + Duration::from_secs(8);
    let mut stats = Value::Null;
    while Instant::now() < until {
        stats = c.ok("probe.pty.stats", json!({"session_id":fid}))?;
        if stats["bytes_read"].as_u64().unwrap_or(0) >= 4_194_304 {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    ensure(
        stats["display_incomplete"] == true
            && stats["has_writer"] == false
            && stats["retained_bytes"].as_u64().unwrap_or(u64::MAX) <= 1_048_576
            && stats["bytes_read"].as_u64().unwrap_or(0) >= 4_194_304,
        "output was not bounded/revoked/drained",
    )?;
    c.denied(
        "probe.input",
        json!({"session_id":fid,"writer_epoch":1,"data":[1]}),
        "DISPLAY_INCOMPLETE",
    )?;
    c.denied(
        "probe.output",
        json!({"session_id":fid}),
        "DISPLAY_INCOMPLETE",
    )?;
    c.ok("session.close", json!({"session_id":fid}))?;
    Ok(
        json!({"stopped_receiver_accepted_bytes_before_busy":accepted,"cancel_and_status_ms":response_ms,"nonblocking_syscall":true,"output":stats,"probe_output_chunk_limit":32768,"body_limit":262144,"separate_pty_drain_continued":true}),
    )
}
fn clean_remote(f: &Fixture, path: &str) -> Result<()> {
    ensure(
        path.starts_with(&format!("{}/", f.remote_root)),
        "cleanup scope",
    )?;
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let s = Sftp::connect(f, None).await?;
        s.raw.remove(path).await?;
        let parent = path.rsplit_once('/').ok_or("parent")?.0;
        s.raw.rmdir(parent).await?;
        s.close().await
    })
}
fn passthrough(f: &Fixture) -> Result<Value> {
    let args = f.ssh_args(None);
    let direct = Command::new("/usr/bin/ssh")
        .args(&args)
        .args([
            "fixture",
            "printf '\\000\\377\\200A'; printf 'ERR' >&2; exit 7",
        ])
        .output()?;
    let wrapped = Command::new(std::env::current_exe()?)
        .args(["passthrough", "--"])
        .args(&args)
        .args([
            "fixture",
            "printf '\\000\\377\\200A'; printf 'ERR' >&2; exit 7",
        ])
        .output()?;
    ensure(
        direct.stdout == wrapped.stdout
            && direct.stderr == wrapped.stderr
            && direct.status.code() == Some(7)
            && wrapped.status.code() == Some(7),
        "passthrough mismatch",
    )?;
    ensure(
        wrapped.stdout == [0, 255, 128, b'A'] && wrapped.stderr == b"ERR",
        "fixture stdout/stderr",
    )?;
    Ok(
        json!({"stdout_sha256":sha(&wrapped.stdout),"stderr_sha256":sha(&wrapped.stderr),"exit":7,"core_started":false}),
    )
}
fn password_and_hostkey(f: &Fixture) -> Result<Value> {
    let secret = fs::read(&f.password_file)?;
    ensure(secret.len() < 128, "password fixture size")?;
    let password = secret.split(|&x| x == b'\n').next().ok_or("password")?;
    let mut args = f.ssh_args(Some("probe_password"));
    args.retain(|x| x != "-oBatchMode=yes");
    args.push("-oBatchMode=no".into());
    args.push("-oPreferredAuthentications=password".into());
    args.push("-oPubkeyAuthentication=no".into());
    args.push("-oNumberOfPasswordPrompts=1".into());
    let mut c = Command::new("/usr/bin/ssh");
    c.args(args).args(["-tt", "fixture", "printf PASSWORD_OK"]);
    c.env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("LANG", "C.UTF-8");
    let mut pty = Pty::spawn(&mut c)?;
    ensure(
        pty.wait_contains(b"password:", Duration::from_secs(5)),
        "password prompt unavailable",
    )?;
    pty.write_all(password)?;
    pty.write_all(b"\n")?;
    ensure(
        pty.wait_contains(b"PASSWORD_OK", Duration::from_secs(5)),
        "password authentication failed",
    )?;
    // The synthetic password remains private and is not returned or recorded.
    let rt = std::env::temp_dir().join(format!("sshx-hostkey-{}", random_id()?));
    fs::create_dir(&rt)?;
    fs::set_permissions(&rt, fs::Permissions::from_mode(0o700))?;
    let key = rt.join("wrong");
    ensure(
        Command::new("ssh-keygen")
            .args(["-q", "-t", "ed25519", "-N", "", "-f"])
            .arg(&key)
            .status()?
            .success(),
        "hostkey fixture generation",
    )?;
    let pubkey = fs::read_to_string(key.with_extension("pub"))?;
    let parts: Vec<_> = pubkey.split_whitespace().collect();
    let known = rt.join("known");
    fs::write(
        &known,
        format!("[127.0.0.1]:{} {} {}\n", f.port, parts[0], parts[1]),
    )?;
    let out = Command::new("/usr/bin/ssh")
        .args([
            "-F",
            "/dev/null",
            "-oBatchMode=yes",
            "-oStrictHostKeyChecking=yes",
            "-oGlobalKnownHostsFile=/dev/null",
        ])
        .arg(format!("-oUserKnownHostsFile={}", known.display()))
        .args([
            "-p",
            &f.port.to_string(),
            "-i",
            &f.identity_file,
            "-l",
            "probe",
            "127.0.0.1",
            "true",
        ])
        .output()?;
    ensure(
        !out.status.success()
            && String::from_utf8_lossy(&out.stderr)
                .contains("REMOTE HOST IDENTIFICATION HAS CHANGED"),
        "changed key refusal not observed",
    )?;
    for p in [&key, &key.with_extension("pub"), &known] {
        fs::remove_file(p)?;
    }
    fs::remove_dir(&rt)?;
    Ok(
        json!({"password_authentication":"observed via owned PTY","hostkey_mismatch_exit":out.status.code(),"secret_logged":false}),
    )
}
fn osc() -> Result<Value> {
    let mut count = 0;
    for input in [
        b"A\x1b]52;c;evil\x07Z".as_slice(),
        b"A\x9d5522;evil\x9cZ",
        b"A\x1b]52;c;evil\x1b\\Z",
        b"A\x1b\x1b]52;c;evil\x07Z",
        b"A\x1b]052;c;evil\x07Z",
    ] {
        for split in 0..=input.len() {
            let mut g = OscGate::default();
            let mut out = g.feed(&input[..split]);
            out.extend(g.feed(&input[split..]));
            ensure(out == b"AZ", "OSC clipboard leaked")?;
            count += 1;
        }
    }
    let mut expired = OscGate::default();
    expired.feed(b"\x1b]52;c;");
    expired.expire();
    ensure(expired.incomplete, "unterminated OSC expiry")?;
    let mut g = OscGate::default();
    g.feed(b"\x1b]52;c;");
    g.feed(&vec![b'x'; 65_537]);
    ensure(g.incomplete && g.feed(b"tail").is_empty(), "OSC overflow")?;
    Ok(
        json!({"split_cases":count,"overflow_display_detached":true,"native_emulator_oracle":"not exercised here; byte gate only"}),
    )
}
fn transfer_cases(f: &Fixture) -> Result<Value> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async{
    let s=Sftp::connect(f,None).await?;let root=s.raw.realpath("/home/probe").await?;let dir=format!("{}/backend-{}",f.remote_root,random_id()?);s.raw.mkdir(&dir,russh_sftp::protocol::FileAttributes{permissions:Some(0o700),..Default::default()}).await?;
    let existing=format!("{dir}/existing");let temp=format!("{dir}/.partial");s.write_exclusive(&existing,b"ORIGINAL").await?;ensure(s.write_exclusive(&existing,b"REPLACED").await.is_err(),"exclusive create replaced file")?;s.write_exclusive(&temp,b"NEW").await?;ensure(s.rename_no_replace(&temp,&existing).await.is_err(),"baseline rename replaced existing")?;ensure(s.read_bounded(&existing,32).await?==b"ORIGINAL","collision changed bytes")?;
    let special=["space name.png","中文.png","quote'\".png","dollar$.png","tick`.png","semi;.png","-leading.png","newline\nname.png","C:\\drive.png"];
    for (i,name) in special.iter().enumerate(){let from=format!("{dir}/temp-{i}");let dest=format!("{dir}/{name}");let bytes=format!("synthetic-{i}").into_bytes();s.write_exclusive(&from,&bytes).await?;s.rename_no_replace(&from,&dest).await?;ensure(s.read_bounded(&dest,64).await?==bytes,"special path readback")?;s.raw.remove(dest).await?;}
    let link=format!("{dir}/link");s.raw.symlink(existing.clone(),link.clone()).await.map_err(|e|format!("symlink fixture: {e}"))?;ensure(s.write_exclusive(&link,b"BAD").await.is_err(),"exclusive create followed symlink")?;ensure(s.read_bounded(&existing,32).await?==b"ORIGINAL","symlink changed target")?;s.raw.remove(&link).await?;
    ensure(s.write_exclusive("/denied/no.png",b"X").await.is_err(),"denied root writable")?;
    // Bounded tmpfs quota fixture; at most 2 MiB generated data, never host disk filling.
    let quota=format!("/quota/full-{}",random_id()?);let full=s.write_exclusive(&quota,&vec![0u8;2*1024*1024]).await.is_err();ensure(full,"quota fixture did not report full")?;let _=s.raw.remove(quota).await;
    let local=std::env::temp_dir().join(format!("sshx-batch-{}",random_id()?));let mut file=OpenOptions::new().create_new(true).write(true).mode(0o600).open(&local)?;file.write_all(b"BATCH_OVERWRITE")?;drop(file);
    let mut child=Command::new("sftp").args(["-F",&f.ssh_config_file,"-oBatchMode=yes","-b","-","fixture"]).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;
    writeln!(child.stdin.take().ok_or("batch input")?,"put \"{}\" \"{}\"",local.display(),existing)?;let batch=child.wait()?;fs::remove_file(local)?;let overwritten=s.read_bounded(&existing,64).await?==b"BATCH_OVERWRITE";ensure(batch.success()&&overwritten,"batch comparison not observed")?;
    for p in [&temp,&existing]{s.raw.remove(p).await?;}s.raw.rmdir(&dir).await?;let extensions=s.extensions.clone();s.close().await?;
    ensure(Sftp::connect(f,Some("probe_nosftp")).await.is_err(),"noSFTP unexpectedly available")?;ensure(Sftp::connect(f,Some("probe_password")).await.is_err(),"additional authentication unexpectedly succeeded")?;
    Ok(json!({"backend_a":{"put_overwrites_existing":true,"selection":"rejected unsafe default put; batch surface has no demonstrated exclusive-open primitive"},"backend_b":{"library":"russh-sftp3.0.1 rawsession","baseline_rename_no_clobber":true,"exclusive_open":true,"symlink_collision_refused":true,"special_names":special.len(),"readback_sha256":true,"extensions":extensions},"canonical_home_reply_entries":root.files.len(),"no_sftp":"observed closed subsystem","extra_auth":"noninteractive refused; no main PTY prompt","disk_full":"1MiB owned tmpfs rejected 2MiB stream","selection":"B viable only for measured controlled fixture; general route/root guarantees unverified"}))
})
}
fn python_plugin(f: &Fixture) -> Result<Value> {
    crate::plugin::run(f)
}

fn capture_after_trigger(
    runtime: &Path,
    binding: &Value,
    hook: Option<&mut dyn FnMut() -> Result<()>>,
) -> Result<(Vec<u8>, Value)> {
    let dir = runtime.join("image");
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    let report = dir.join("report.json");
    let binding_file = dir.join("binding.json");
    let mut bound = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&binding_file)?;
    serde_json::to_writer(&mut bound, binding)?;
    drop(bound);
    let image = std::env::current_exe()?.with_file_name("phase0-image");
    let mut child = OwnedChild(
        Command::new(&image)
            .arg("suite")
            .arg("--output")
            .arg(&report)
            .arg("--binding-file")
            .arg(&binding_file)
            .arg("--artifact-dir")
            .arg(&dir)
            .env_remove("DISPLAY")
            .env_remove("XAUTHORITY")
            .spawn()?,
    );
    if let Some(hook) = hook {
        ensure(
            child.0.try_wait()?.is_none(),
            "capture process exited before in-flight barrier",
        )?;
        hook()?;
    }
    let status = child.0.wait()?;
    ensure(
        status.success(),
        "independent image suite failed; artifact cannot be accepted",
    )?;
    let metadata: Value = serde_json::from_slice(&fs::read(&report)?)?;
    let path = dir.join("sealed.png");
    let bytes = fs::read(&path)?;
    let sealed = crate::seal::validate(&bytes)?;
    ensure(
        metadata["artifact"]["sha256"] == sealed.2,
        "independent plugin sealed bytes differ",
    )?;
    ensure(
        metadata["artifact"]["binding"] == *binding,
        "image plugin operation binding differs",
    )?;
    let info = json!({"operation_id_created_before_capture":binding["operation_id"],"binding":binding,"visual_suite_exit":status.code(),"plugin_artifact":metadata["artifact"],"core_validation":{"width":sealed.0,"height":sealed.1,"sha256":sealed.2},"single_snapshot_source":"private Xvfb synthetic selection; separate plugin process"});
    for file in [&path, &report, &binding_file] {
        fs::remove_file(file)?;
    }
    fs::remove_dir(dir)?;
    Ok((bytes, info))
}
fn native_terminal(f: &Fixture) -> Result<Value> {
    let mut c = Command::new("ssh");
    c.args(f.ssh_args(None))
        .args(["-tt", "fixture", "bash --noprofile --norc"]);
    c.env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("TERM", "xterm-256color")
        .env("LANG", "C.UTF-8");
    let mut p = Pty::spawn(&mut c)?;
    p.write_all(b"printf '\\nSHELL_READY\\n'\r")?;
    ensure(
        p.wait_contains(b"\r\nSHELL_READY\r\n", Duration::from_secs(5)),
        "synthetic shell marker",
    )?;
    p.resize(101, 37)?;
    p.write_all(b"stty size; printf 'SIZE_DONE\\n'\r")?;
    ensure(
        p.wait_contains(b"37 101", Duration::from_secs(5)),
        "remote resize not observed",
    )?;
    p.write_all(b"sleep 30\r")?;
    thread::sleep(Duration::from_millis(100));
    p.write_all(&[3])?;
    p.write_all(b"printf '\\nINT_DONE\\n'\r")?;
    ensure(
        p.wait_contains(b"\r\nINT_DONE\r\n", Duration::from_secs(5)),
        "CtrlC shell recovery",
    )?;
    p.write_all(b"sleep 30\r")?;
    thread::sleep(Duration::from_millis(100));
    p.write_all(&[26])?;
    p.write_all(b"jobs -p; kill %1; printf '\\nJOB_DONE\\n'\r")?;
    ensure(
        p.wait_contains(b"\r\nJOB_DONE\r\n", Duration::from_secs(5)),
        "job control marker",
    )?;
    p.write_all("printf '\\nUNICODE_中文\\n'\r".as_bytes())?;
    p.write_all(b"exit 23\r")?;
    let until = Instant::now() + Duration::from_secs(5);
    let mut exit = None;
    while Instant::now() < until {
        if let Some(s) = p.child.try_wait()? {
            exit = s.code();
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    ensure(exit == Some(23), "PTY exit status")?;
    Ok(
        json!({"resize":[101,37],"ctrl_c":"remote foreground interrupted, shell survives","ctrl_z":"job stopped and explicitly killed","exit_status":exit,"terminal_scope":"owned synthetic PTY, no user's termios changed","physical_ime":false}),
    )
}
fn owned_agent(f: &Fixture) -> Result<Value> {
    let dir = std::env::temp_dir().join(format!("sshx-agent-{}", random_id()?));
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    let socket = dir.join("agent.sock");
    let mut agent = OwnedChild(
        Command::new("ssh-agent")
            .arg("-D")
            .arg("-a")
            .arg(&socket)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    );
    let until = Instant::now() + Duration::from_secs(3);
    while !socket.exists() && Instant::now() < until {
        thread::sleep(Duration::from_millis(10));
    }
    let added = Command::new("ssh-add")
        .arg(&f.identity_file)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("SSH_AUTH_SOCK", &socket)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    ensure(added.success(), "owned key agent add")?;
    let result = Command::new("ssh")
        .args([
            "-F",
            "/dev/null",
            "-oBatchMode=yes",
            "-oIdentityFile=none",
            "-oIdentitiesOnly=no",
            "-oStrictHostKeyChecking=yes",
            "-oGlobalKnownHostsFile=/dev/null",
        ])
        .arg(format!("-oUserKnownHostsFile={}", f.known_hosts_file))
        .arg(format!("-oIdentityAgent={}", socket.display()))
        .args([
            "-p",
            &f.port.to_string(),
            "-l",
            "probe",
            "127.0.0.1",
            "printf AGENT_OK",
        ])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .stdin(Stdio::null())
        .output()?;
    ensure(
        result.status.success() && result.stdout == b"AGENT_OK",
        "owned agent authentication",
    )?;
    agent.0.kill()?;
    agent.0.wait()?;
    if socket.exists() {
        fs::remove_file(&socket)?;
    }
    fs::remove_dir(dir)?;
    Ok(
        json!({"owned_agent_only":true,"identity_file":"none in ssh argv","authentication":"generated key in owned ssh-agent","personal_agent_inherited":false}),
    )
}
pub fn run_case(fixture_path: &Path, name: &str, output: &Path) -> Result<()> {
    let fixture = Fixture::load(fixture_path)?;
    let result = match name {
        "sftp" => transfer_cases(&fixture),
        "ack-loss" => lost_commit_ack(&fixture),
        "routes" => routes_and_reuse(&fixture),
        "password" => password_and_hostkey(&fixture),
        "terminal" => native_terminal(&fixture),
        "agent" => owned_agent(&fixture),
        "osc" => osc(),
        "passthrough" => passthrough(&fixture),
        _ => {
            return Err("unknown case; choose sftp/password/terminal/agent/osc/passthrough".into())
        }
    };
    let failed = result.is_err();
    let value = match result {
        Ok(v) => json!({"schema":1,"case":name,"status":"pass","observations":v}),
        Err(e) => json!({"schema":1,"case":name,"status":"fail","error":e.to_string()}),
    };
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &value)?;
    file.write_all(b"\n")?;
    if failed {
        Err("case failed; see evidence".into())
    } else {
        Ok(())
    }
}
fn lost_commit_ack(f: &Fixture) -> Result<Value> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async{
    let s=Sftp::connect_with_fault(f,None,true).await?;let dir=format!("{}/lost-ack-{}",f.remote_root,random_id()?);s.raw.mkdir(&dir,russh_sftp::protocol::FileAttributes{permissions:Some(0o700),..Default::default()}).await?;
    let temp=format!("{dir}/.partial");let final_path=format!("{dir}/final.png");let data=b"sealed synthetic reply-loss bytes";s.write_exclusive(&temp,data).await?;
    let result=s.rename_no_replace(&temp,&final_path).await;ensure(result.is_err()&&s.ack_dropped.load(std::sync::atomic::Ordering::SeqCst),"reply loss was not exercised")?;let _=s.close().await;
    // Reconcile only the known operation-owned paths, read-only before explicit cleanup.
    let fresh=Sftp::connect(f,None).await?;let observed=fresh.read_bounded(&final_path,data.len()).await?;ensure(observed==data&&fresh.raw.lstat(&temp).await.is_err(),"rename reconciliation")?;
    fresh.raw.remove(&final_path).await?;fresh.raw.rmdir(&dir).await?;fresh.close().await?;
    Ok(json!({"rename_client_result":"error/unknown delivery","owned_pipe_reply_discarded_after_request_write":true,"read_only_reconciliation":{"final_sha256":sha(&observed),"temp_absent":true},"reupload_attempts":0,"cleanup":"explicit known-final removal after reconciliation","fault_boundary":"actual SFTP subsystem pipe adapter consumes reply then aborts channel; no host/network disruption"}))
})
}
fn core_crash(fixture_path: &Path) -> Result<Value> {
    // Adopt only this probe's orphaned descendants so forced Core failure is fully reaped.
    ensure(
        unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) } == 0,
        "subreaper setup",
    )?;
    let mut roots = Vec::new();
    let mut boots = Vec::new();
    let mut child_pid = 0;
    for turn in 0..2 {
        let dir = std::env::temp_dir().join(format!("sshx-crash-{}", random_id()?));
        fs::create_dir(&dir)?;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
        let mut core = OwnedChild(
            Command::new(std::env::current_exe()?)
                .args(["core", "--fixture"])
                .arg(fixture_path)
                .arg("--runtime")
                .arg(&dir)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let socket = dir.join("core.sock");
        let until = Instant::now() + Duration::from_secs(5);
        while !socket.exists() && Instant::now() < until {
            thread::sleep(Duration::from_millis(5));
        }
        let mut client = Client::connect(&socket)?;
        // A second hello is role-denied; record the boot from an operation binding instead.
        if turn == 0 {
            let s = client.ok("session.start", json!({}))?;
            let sid = s["session_id"].as_str().ok_or("session")?;
            client.wait_output(sid, b"PHASE0_READY")?;
            child_pid = s["child_pid"].as_u64().ok_or("child")? as i32;
            let id = op(&mut client, sid, 1, 1)?;
            let record = client.ok("operation.get", json!({"operation_id":id}))?;
            boots.push(record["binding"]["core_instance_id"].clone());
            core.0.kill()?;
            core.0.wait()?;
            ensure(
                client.call("session.list", json!({})).is_err(),
                "dead Core still usable",
            )?;
            unsafe {
                libc::kill(child_pid, libc::SIGKILL);
            }
            let mut status = 0;
            let waited = unsafe { libc::waitpid(child_pid, &mut status, 0) };
            ensure(waited == child_pid, "owned orphan not reaped")?;
        } else {
            let sessions = client.ok("session.list", json!({}))?;
            ensure(
                sessions["sessions"].as_array().is_some_and(Vec::is_empty),
                "new Core resurrected metadata",
            )?;
            let s = client.ok("session.start", json!({}))?;
            let sid = s["session_id"].as_str().ok_or("session")?;
            client.wait_output(sid, b"PHASE0_READY")?;
            let id = op(&mut client, sid, 1, 1)?;
            let record = client.ok("operation.get", json!({"operation_id":id}))?;
            boots.push(record["binding"]["core_instance_id"].clone());
            client.ok("session.close", json!({"session_id":sid}))?;
            client.ok("core.stop", json!({}))?;
            client.close();
            core.0.wait()?;
        }
        for name in [
            "core.sock",
            "core.lock",
            "frozen_config",
            "frozen_known_hosts",
        ] {
            let p = dir.join(name);
            if p.exists() {
                fs::remove_file(p)?;
            }
        }
        fs::remove_dir(&dir)?;
        roots.push(dir);
    }
    ensure(
        boots.len() == 2 && boots[0] != boots[1],
        "Core boot identity reused",
    )?;
    Ok(
        json!({"core_sigkill":true,"orphan_ssh_pid_reaped":child_pid,"new_boot":true,"metadata_adopted":false,"remote_work_recovery":"not provided","owned_runtime_cleanup":roots.iter().all(|p|!p.exists())}),
    )
}
fn endpoint_and_config_refusal(fixture_path: &Path) -> Result<Value> {
    use std::os::unix::fs::symlink;
    let dir = std::env::temp_dir().join(format!("sshx-refusal-{}", random_id()?));
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    let sentinel = dir.join("sentinel");
    fs::write(&sentinel, b"UNCHANGED")?;
    let socket = dir.join("core.sock");
    symlink(&sentinel, &socket)?;
    let first = Command::new(std::env::current_exe()?)
        .args(["core", "--fixture"])
        .arg(fixture_path)
        .arg("--runtime")
        .arg(&dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;
    ensure(
        !first.status.success()
            && fs::symlink_metadata(&socket)?.file_type().is_symlink()
            && fs::read(&sentinel)? == b"UNCHANGED",
        "endpoint symlink replaced/deleted",
    )?;
    fs::remove_file(&socket)?;
    let marker = dir.join("helper-ran");
    let config = dir.join("config");
    let f = Fixture::load(fixture_path)?;
    fs::write(
        &config,
        format!(
            "{}\nMatch exec \"touch {}\"\n",
            fs::read_to_string(&f.ssh_config_file)?,
            marker.display()
        ),
    )?;
    fs::set_permissions(&config, fs::Permissions::from_mode(0o600))?;
    let mut variant = f.clone();
    variant.ssh_config_file = config.to_string_lossy().into();
    let manifest = dir.join("fixture.json");
    let mut out = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&manifest)?;
    serde_json::to_writer(&mut out, &variant)?;
    drop(out);
    let second = Command::new(std::env::current_exe()?)
        .args(["core", "--fixture"])
        .arg(&manifest)
        .arg("--runtime")
        .arg(&dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;
    ensure(
        !second.status.success()
            && String::from_utf8_lossy(&second.stderr).contains("TARGET_UNVERIFIED")
            && !marker.exists(),
        "dynamic config helper executed or accepted",
    )?;
    for name in [
        "core.sock",
        "core.lock",
        "sentinel",
        "config",
        "fixture.json",
    ] {
        let p = dir.join(name);
        if p.exists() {
            fs::remove_file(p)?;
        }
    }
    fs::remove_dir(dir)?;
    Ok(
        json!({"endpoint_symlink_refused_without_unlink":true,"dynamic_match_exec":"TARGET_UNVERIFIED before SSH/config evaluation","helper_executed":false,"auxiliary_profile":"explicit frozen numeric loopback route, public expected hostkey copied into Core-owned snapshot"}),
    )
}
fn routes_and_reuse(f: &Fixture) -> Result<Value> {
    let dir = std::env::temp_dir().join(format!("sshx-routes-{}", random_id()?));
    fs::create_dir(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    let known = dir.join("known_hosts");
    let public = fs::read_to_string(&f.known_hosts_file)?;
    let fields: Vec<_> = public
        .lines()
        .next()
        .ok_or("known host")?
        .split_whitespace()
        .collect();
    ensure(fields.len() >= 3, "hostkey fields")?;
    fs::write(
        &known,
        format!("{public}task-target {} {}\n", fields[1], fields[2]),
    )?;
    let config = dir.join("config");
    let base=format!("Host jump\n HostName 127.0.0.1\n Port {}\nHost route\n HostName 127.0.0.1\n Port 2222\n HostKeyAlias task-target\nHost *\n User probe\n IdentityFile \"{}\"\n IdentityAgent none\n IdentitiesOnly yes\n UserKnownHostsFile \"{}\"\n GlobalKnownHostsFile /dev/null\n StrictHostKeyChecking yes\n BatchMode yes\n ConnectTimeout 5\n LogLevel ERROR\n",f.port,f.identity_file,known.display());
    fs::write(&config, base)?;
    fs::set_permissions(&config, fs::Permissions::from_mode(0o600))?;
    let jumped = Command::new("ssh")
        .arg("-F")
        .arg(&config)
        .args(["-J", "jump", "route", "printf JUMP_OK"])
        .stdin(Stdio::null())
        .output()?;
    ensure(
        jumped.status.success() && jumped.stdout == b"JUMP_OK",
        "ProxyJump fixture route",
    )?;
    // Paths are task-generated ASCII with no spaces/metacharacters; this is an owned helper fixture.
    let proxy = format!("ssh -F {} -W %h:%p jump", config.display());
    let proxied = Command::new("ssh")
        .arg("-F")
        .arg(&config)
        .arg(format!("-oProxyCommand={proxy}"))
        .args(["route", "printf PROXY_OK"])
        .stdin(Stdio::null())
        .output()?;
    ensure(
        proxied.status.success() && proxied.stdout == b"PROXY_OK",
        "ProxyCommand fixture route",
    )?;
    let control = dir.join("master.sock");
    let mut master = OwnedChild(
        Command::new("ssh")
            .args(
                f.ssh_args(None)
                    .into_iter()
                    .filter(|x| x != "-oControlMaster=no" && x != "-oControlPath=none"),
            )
            .args(["-M", "-N", "-S"])
            .arg(&control)
            .arg("fixture")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    );
    let until = Instant::now() + Duration::from_secs(5);
    while !control.exists() && Instant::now() < until {
        thread::sleep(Duration::from_millis(5));
    }
    let reused = Command::new("ssh")
        .args(["-F", &f.ssh_config_file, "-S"])
        .arg(&control)
        .args([
            "-oPreferredAuthentications=none",
            "fixture",
            "printf REUSE_OK",
        ])
        .stdin(Stdio::null())
        .output()?;
    ensure(
        reused.status.success() && reused.stdout == b"REUSE_OK",
        "verified master reuse with fresh authentication disabled",
    )?;
    let mut batch = Command::new("sftp")
        .args(["-F", &f.ssh_config_file, "-oPreferredAuthentications=none"])
        .arg(format!("-oControlPath={}", control.display()))
        .args(["-b", "-", "fixture"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    batch.stdin.take().ok_or("batch")?.write_all(b"pwd\n")?;
    ensure(batch.wait()?.success(), "SFTP master reuse")?;
    let pid = master.0.id();
    let stopped = Command::new("ssh")
        .args(["-F", &f.ssh_config_file, "-S"])
        .arg(&control)
        .args(["-O", "exit", "fixture"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    ensure(stopped.success(), "owned master exit")?;
    master.0.wait()?;
    for p in [&known, &config, &control] {
        if p.exists() {
            fs::remove_file(p)?;
        }
    }
    fs::remove_dir(dir)?;
    let rt = tokio::runtime::Runtime::new()?;
    let chroot=rt.block_on(async{let s=Sftp::connect(f,Some("probe_chroot")).await?;let root=s.raw.realpath(".").await?;let temp=format!("/upload/chroot-{}",random_id()?);s.write_exclusive(&temp,b"CHROOT").await?;ensure(s.read_bounded(&temp,6).await?==b"CHROOT","chroot readback")?;ensure(s.raw.lstat(&f.remote_root).await.is_err(),"chroot shares normal root")?;s.raw.remove(&temp).await?;s.close().await?;Ok::<_,Box<dyn std::error::Error+Send+Sync>>(json!({"canonical_path":root.files.first().map(|v|v.filename.clone()),"normal_root_unavailable":true,"shell_mapping":null}))})?;
    Ok(
        json!({"proxy_jump":"observed two-hop generated fixture route","proxy_command":"observed owned helper route","config_identity_reuse":true,"network_reuse":{"owned_master_pid":pid,"ssh_command_with_fresh_auth_disabled":true,"sftp_with_fresh_auth_disabled":true},"core_policy":"dynamic proxy/helper routes remain rejected for auxiliary effects until their closure can be frozen/bound","chroot":chroot}),
    )
}
