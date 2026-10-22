use super::*;

struct CoreRun {
    child: OwnedChild,
    runtime: std::path::PathBuf,
}
impl CoreRun {
    fn start(primary: &Path, secondary: &Path, qualified: bool) -> Result<Self> {
        let runtime = std::env::temp_dir().join(format!("sshx-upload-{}", random_id()?));
        fs::create_dir(&runtime)?;
        fs::set_permissions(&runtime, fs::Permissions::from_mode(0o700))?;
        let mut command = Command::new(std::env::current_exe()?);
        command
            .arg("core")
            .arg("--fixture")
            .arg(primary)
            .arg("--fixture-secondary")
            .arg(secondary)
            .arg("--runtime")
            .arg(&runtime);
        if qualified {
            command.arg("--qualify-upload");
        }
        let child = OwnedChild(
            command
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let until = Instant::now() + Duration::from_secs(15);
        while !runtime.join("core.sock").exists() && Instant::now() < until {
            thread::sleep(Duration::from_millis(10));
        }
        Ok(Self { child, runtime })
    }
    fn remove(mut self) -> Result<()> {
        self.child.0.kill()?;
        self.child.0.wait()?;
        for name in [
            "core.sock",
            "core.lock",
            "qualification.json",
            "frozen_config",
            "frozen_known_hosts",
            "frozen_config-secondary",
            "frozen_known_hosts-secondary",
        ] {
            let path = self.runtime.join(name);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        fs::remove_dir(&self.runtime)?;
        Ok(())
    }
}
struct Rig {
    client: Client,
    sid: String,
    nonce: u64,
    fixture: Fixture,
    initial_output: Vec<u8>,
}
impl Rig {
    fn start(core: &CoreRun, fixture: &Fixture) -> Result<Self> {
        let client = Client::connect(&core.runtime.join("core.sock"))?;
        let mut rig = Self {
            client,
            sid: String::new(),
            nonce: 0,
            fixture: fixture.clone(),
            initial_output: Vec::new(),
        };
        rig.session("primary")?;
        Ok(rig)
    }
    fn session(&mut self, profile: &str) -> Result<()> {
        if !self.sid.is_empty() {
            self.client
                .ok("session.close", json!({"session_id":self.sid}))?;
        }
        let s = self
            .client
            .ok("session.start", json!({"profile_id":profile}))?;
        self.sid = s["session_id"].as_str().ok_or("sid")?.into();
        self.initial_output = self.client.wait_output(&self.sid, b"PHASE0_READY")?;
        Ok(())
    }
    fn op(&mut self) -> Result<String> {
        self.nonce += 1;
        op(&mut self.client, &self.sid, 1, self.nonce)
    }
    fn start_upload(&mut self, id: &str, extras: Value) -> Result<Value> {
        let mut p =
            json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1,2,3,4]});
        for (k, v) in extras.as_object().ok_or("extras")? {
            p[k] = v.clone();
        }
        self.client.ok("probe.upload", p)
    }
    fn release(&mut self, id: &str) -> Result<()> {
        self.client
            .ok("probe.release", json!({"operation_id":id}))?;
        Ok(())
    }
    fn before_commit(&mut self, id: &str) -> Result<()> {
        self.client.wait_state(id, "barrier", "before_open")?;
        self.release(id)?;
        self.client.wait_state(id, "barrier", "before_commit")?;
        Ok(())
    }
    fn done(&mut self, id: &str) -> Result<Value> {
        self.client.wait_state(id, "barrier", "completed")
    }
    fn paths(&self, id: &str) -> (String, String, String) {
        let d = format!("{}/{}", self.fixture.remote_root, id);
        (d.clone(), format!("{d}/.partial"), format!("{d}/final.png"))
    }
    fn no_input(&mut self) -> Result<()> {
        let bytes = self.client.output(&self.sid)?;
        ensure(bytes == self.initial_output, "unexpected PTY insertion")
    }
}
fn remote<T>(f: &Fixture, run: impl AsyncFnOnce(&Sftp) -> Result<T>) -> Result<T> {
    tokio::runtime::Runtime::new()?.block_on(async {
        let s = Sftp::connect(f, None).await?;
        let result = run(&s).await;
        let _ = s.close().await;
        result
    })
}
fn observe(f: &Fixture, paths: &(String, String, String)) -> Result<Value> {
    remote(f, async |s| {
        let mut values = Vec::new();
        for p in [&paths.0, &paths.1, &paths.2] {
            let value = match s.raw.lstat(p).await {
                Ok(m) => {
                    json!({"path":p,"exists":true,"permissions":m.attrs.permissions,"size":m.attrs.size,"symlink":m.attrs.is_symlink(),"sha256":if m.attrs.is_regular()&&!m.attrs.is_symlink(){Some(sha(&s.read_bounded(p,65536).await?))}else{None}})
                }
                Err(russh_sftp::client::error::Error::Status(e))
                    if e.status_code == russh_sftp::protocol::StatusCode::NoSuchFile =>
                {
                    json!({"path":p,"exists":false})
                }
                Err(e) => return Err(e.into()),
            };
            values.push(value);
        }
        Ok(json!(values))
    })
}
fn no_directory(f: &Fixture, paths: &(String, String, String)) -> Result<Value> {
    let v = observe(f, paths)?;
    ensure(v[0]["exists"] == false, "unexpected operation directory")?;
    Ok(v)
}
async fn missing(s: &Sftp, path: &str) -> Result<bool> {
    match s.raw.lstat(path).await {
        Ok(_) => Ok(false),
        Err(russh_sftp::client::error::Error::Status(e))
            if e.status_code == russh_sftp::protocol::StatusCode::NoSuchFile =>
        {
            Ok(true)
        }
        Err(e) => Err(e.into()),
    }
}

fn config_change(core: &CoreRun, fixture: &Fixture, secondary: &Path) -> Result<Value> {
    let config = core.runtime.join("case-config");
    let manifest = core.runtime.join("case-fixture.json");
    let mut copy = fixture.clone();
    copy.ssh_config_file = config.to_string_lossy().into();
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&config)?
        .write_all(&fs::read(&fixture.ssh_config_file)?)?;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&manifest)?
        .write_all(&serde_json::to_vec(&copy)?)?;
    let isolated = CoreRun::start(&manifest, secondary, true)?;
    let mut rig = Rig::start(&isolated, &copy)?;
    let id = rig.op()?;
    OpenOptions::new()
        .append(true)
        .open(&config)?
        .write_all(b"\n# synthetic changed configuration generation\n")?;
    let denied = rig.client.denied(
        "probe.upload",
        json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),
        "CONFIG_CHANGED",
    )?;
    let files = no_directory(fixture, &rig.paths(&id))?;
    rig.client
        .ok("session.close", json!({"session_id":rig.sid}))?;
    rig.client.close();
    isolated.remove()?;
    fs::remove_file(config)?;
    fs::remove_file(manifest)?;
    Ok(
        json!({"evidence_kind":"real_local_config_change","denial":denied,"files":files,"original_fixture_config":"unchanged; test changed isolated copy"}),
    )
}

pub fn run(primary: &Path, secondary: &Path, output: &Path) -> Result<()> {
    let fixture = Fixture::load(primary)?;
    let second = Fixture::load(secondary)?;
    let core = CoreRun::start(primary, secondary, true)?;
    let mut r = Rig::start(&core, &fixture)?;
    let qualification: Value =
        serde_json::from_slice(&fs::read(core.runtime.join("qualification.json"))?)?;
    let mut report = Report { cases: vec![] };
    report.case("upload","C01","Real qualification and default-unavailable Core; unqualified upload creates nothing",|| {
        ensure(qualification.as_array().is_some_and(|v|v.len()==2&&v.iter().all(|x|x["extension_version"]=="1"&&x["collision_preserved"]==true)),"qualification evidence")?;
        let unqualified=CoreRun::start(primary,secondary,false)?;
        let mut plain=Rig::start(&unqualified,&fixture)?; let id=plain.op()?;
        plain.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),"PATH_DENIED")?;
        let files=no_directory(&fixture,&plain.paths(&id))?; plain.no_input()?;
        ensure(!unqualified.runtime.join("qualification.json").exists(),"implicit preparation")?;
        plain.client.ok("session.close",json!({"session_id":plain.sid}))?; plain.client.close(); unqualified.remove()?;
        let id=r.op()?; r.client.ok("probe.policy",json!({"session_id":r.sid,"change":"unqualified"}))?;
        r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),"COMMIT_UNSUPPORTED")?;
        no_directory(&fixture,&r.paths(&id))?; r.session("primary")?;
        Ok(json!({"qualification":qualification,"without_policy":files,"without_qualification":"denied before connection","preparation_authority":"explicit --qualify-upload fixture-owner CLI; never action/plugin admission"}))
    });
    report.case("upload","C02","Acknowledged link, exact bytes, separate owned-temp cleanup and retained final directory",|| {
        let id=r.op()?; r.start_upload(&id,json!({}))?; r.before_commit(&id)?; r.release(&id)?; let result=r.done(&id)?;
        let files=observe(&fixture,&r.paths(&id))?;
        ensure(result["upload_state"]=="committed"&&result["result"]["final_commit"]=="confirmed"&&result["result"]["readback_verified"]==true&&result["cleanup_pending"]==false,"commit projection")?;
        ensure(files[0]["exists"]==true&&files[1]["exists"]==false&&files[2]["sha256"]==sha(&[1,2,3,4]),"publication files")?; r.no_input()?;
        Ok(json!({"evidence_kind":"real_server","operation":result,"files":files,"insertion_bytes":0}))
    });
    report.case("upload","C03","Existing regular/symlink and concurrent creator preserve destination; no retry/fallback",|| {
        let mut cases=Vec::new();
        for kind in ["existing","symlink","concurrent"] {
            let id=r.op()?; r.start_upload(&id,if kind=="concurrent"{json!({"hold_step":"link"})}else{json!({})})?; r.before_commit(&id)?;
            let paths=r.paths(&id);
            if kind=="concurrent" { r.release(&id)?; r.client.wait_state(&id,"barrier","before_link")?; }
            let sentinel=format!("{}/sentinel",paths.0);
            remote(&fixture,async |s| { if kind=="symlink" {s.write_exclusive(&sentinel,b"ORIGINAL").await?;s.raw.symlink(&sentinel,&paths.2).await?;}else{s.write_exclusive(&paths.2,b"ORIGINAL").await?;} Ok(()) })?;
            r.release(&id)?; let result=r.done(&id)?;
            ensure(result["upload_state"]=="failed"&&result["error"]==if kind=="concurrent"{"IO_ERROR"}else{"NAME_EXISTS"},"collision classification")?;
            let preserved=remote(&fixture,async |s|s.read_bounded(if kind=="symlink"{&sentinel}else{&paths.2},32).await)?; ensure(preserved==b"ORIGINAL","destination overwritten")?;
            let replay=r.start_upload(&id,json!({}))?; ensure(replay["external_steps_admitted"]==result["external_steps_admitted"],"collision replay effect")?;
            r.no_input()?; cases.push(json!({"kind":kind,"operation":result,"files":observe(&fixture,&paths)?,"preserved_sha256":sha(&preserved),"replay":replay}));
        }
        Ok(json!({"evidence_kind":"real_server","cases":cases}))
    });
    report.case("upload","C04","Unavailable version/explicit unsupported injection and real cross-device failure never fall back",|| {
        let mut cases=Vec::new();
        for fault in ["missing_extension","unsupported_version","unsupported_response"] {
            let id=r.op()?; r.start_upload(&id,json!({"fault":fault}))?;
            r.client.wait_state(&id,"barrier","before_open")?;r.release(&id)?;
            if fault=="unsupported_response" {r.client.wait_state(&id,"barrier","before_commit")?;r.release(&id)?;}
            let result=r.done(&id)?; ensure(result["error"]=="COMMIT_UNSUPPORTED"&&result["upload_state"]=="failed","unsupported accepted")?;
            let files=no_directory(&fixture,&r.paths(&id))?;
            cases.push(json!({"fault":fault,"evidence_kind":"fault_injection","operation":result,"files":files}));
        }
        let cross=remote(&fixture,async |s| {
            let from=format!("{}/cross-{}",fixture.remote_root,random_id()?);let to=format!("/quota/cross-{}",random_id()?);
            s.write_exclusive(&from,b"CROSS").await?;
            let error=s.raw.hardlink(&from,&to).await.err().ok_or("cross-device link succeeded")?;
            ensure(s.read_bounded(&from,16).await?==b"CROSS"&&missing(s,&to).await?,"cross-device effects")?;
            s.raw.remove(&from).await?;
            Ok(json!({"evidence_kind":"real_server","error":error.to_string(),"classification":"generic SFTP failure; distinct-device fixture facts are separate, not runtime EXDEV authority","destination_absent":true,"fallback":false}))
        })?; r.no_input()?; Ok(json!({"injected":cases,"cross_device":cross}))
    });
    report.case("upload","C05","Policy/scope changes, stronger request, wrong owner/grant and malformed request fail before effects",|| {
        let mut cases=Vec::new();
        for change in ["absent","root","namespace","policy"] {
            let id=r.op()?;r.client.ok("probe.policy",json!({"session_id":r.sid,"change":change}))?;
            let denial=r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),"PATH_DENIED")?;
            cases.push(json!({"change":change,"denial":denial,"files":no_directory(&fixture,&r.paths(&id))?}));r.session("primary")?;
        }
        let id=r.op()?;
        r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1],"required_policy":"enforced"}),"PATH_DENIED")?;
        r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1],"fault":42}),"INVALID_PARAMS")?;
        let unchanged=r.client.ok("operation.get",json!({"operation_id":id}))?;ensure(unchanged["external_steps_admitted"]==0&&unchanged["upload_state"]=="none","malformed consumed authority")?;
        let mut other=Client::connect(&core.runtime.join("core.sock"))?;
        other.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),"CAPABILITY_DENIED")?;other.close();
        r.client.ok("grant.revoke",json!({"operation_id":id}))?;r.client.ok("probe.policy",json!({"session_id":r.sid,"change":"absent"}))?;
        r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"final.png","data":[1]}),"CAPABILITY_DENIED")?;
        no_directory(&fixture,&r.paths(&id))?;r.session("primary")?;
        Ok(json!({"config_change":config_change(&core,&fixture,secondary)?,"cases":cases,"stronger_requirement":"PATH_DENIED","wrong_client_instance":"CAPABILITY_DENIED","revoked_before_policy":"CAPABILITY_DENIED","malformed":unchanged,"limitation":"probe Client ownership represents instance; production plugin admission remains separate"}))
    });
    report.case("upload","C06","Traversal and observed symlink/type replacement block publication and cleanup",|| {
        let id=r.op()?;
        for name in ["../escape","/absolute",".."] {r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":name,"data":[1]}),"PATH_DENIED")?;}
        no_directory(&fixture,&r.paths(&id))?;
        r.start_upload(&id,json!({}))?;r.client.wait_state(&id,"barrier","before_open")?;
        let before_paths=r.paths(&id);let preexisting=format!("{}/preexisting-{}",fixture.remote_root,random_id()?);
        remote(&fixture,async |s|{s.raw.mkdir(&preexisting,russh_sftp::protocol::FileAttributes {permissions:Some(0o700),..Default::default()}).await?;s.write_exclusive(&format!("{preexisting}/sentinel"),b"PREEXISTING").await?;s.raw.symlink(&preexisting,&before_paths.0).await?;Ok(())})?;
        r.release(&id)?;let precreation=r.done(&id)?;
        ensure(precreation["error"]=="PATH_DENIED"&&precreation["upload_state"]=="failed","symlink precreation continued")?;
        let precreation_files=remote(&fixture,async |s|{ensure(s.raw.lstat(&before_paths.0).await?.attrs.is_symlink(),"foreign symlink removed")?;ensure(missing(s,&before_paths.1).await?,"temp created through preexisting symlink")?;let bytes=s.read_bounded(&format!("{preexisting}/sentinel"),32).await?;ensure(bytes==b"PREEXISTING","foreign sentinel changed")?;Ok(json!({"preexisting_symlink_retained":true,"temp_absent":true,"sentinel_sha256":sha(&bytes)}))})?;
        let id=r.op()?;r.start_upload(&id,json!({}))?;r.before_commit(&id)?;let paths=r.paths(&id);let backup=format!("{}.original",paths.0);
        remote(&fixture,async |s|{s.raw.rename(&paths.0,&backup).await?;s.raw.symlink(&backup,&paths.0).await?;Ok(())})?;
        r.release(&id)?;let result=r.done(&id)?;ensure(result["upload_state"]=="failed"&&result["cleanup_pending"]==true,"parent swap continued")?;
        let retained=remote(&fixture,async |s|{ensure(s.raw.lstat(&paths.0).await?.attrs.is_symlink(),"replacement removed")?;ensure(s.read_bounded(&format!("{backup}/.partial"),8).await?==[1,2,3,4],"original removed")?;Ok(json!({"replacement_symlink":true,"original_temp_sha256":sha(&[1,2,3,4])}))})?;
        r.no_input()?;Ok(json!({"evidence_kind":"real_server","operation":result,"observations":retained,"precreation":{"operation":precreation,"files":precreation_files,"stage":"before mkdir/open; preexisting operation-directory symlink"},"containment_unit_test":"prefix-confusable/traversal","limitation":"observed replacement only; no ABA guarantee"}))
    });
    report.case("upload","C07","Cancel/revoke at each step prohibit new requests; owned cleanup separate",|| {
        let mut cases=Vec::new();
        for method in ["operation.cancel","grant.revoke"] {
            for stage in ["mkdir","open","write","close","link","stream"] {
                let id=r.op()?;r.start_upload(&id,if stage=="stream"{json!({"hold_mid_stream":true})}else{json!({"hold_step":stage})})?;
                r.client.wait_state(&id,"barrier","before_open")?;r.release(&id)?;
                if stage=="link" {r.client.wait_state(&id,"barrier","before_commit")?;r.release(&id)?;}
                let before=r.client.wait_state(&id,"barrier",if stage=="stream"{"during_stream"}else{match stage{"mkdir"=>"before_mkdir","open"=>"before_open","write"=>"before_write","close"=>"before_close",_=>"before_link"}})?;
                r.client.ok(method,json!({"operation_id":id}))?;let result=r.done(&id)?;
                ensure(result["upload_state"]=="failed"&&result["external_steps_admitted"]==before["external_steps_admitted"],"effect after revoke")?;
                let files=no_directory(&fixture,&r.paths(&id))?;
                cases.push(json!({"transition":method,"stage":stage,"before":before,"after":result,"files":files}));
            }
        }
        for iteration in 0..12 {
            let id=r.op()?;r.start_upload(&id,json!({"hold_mid_stream":true}))?;
            r.client.wait_state(&id,"barrier","before_open")?;r.release(&id)?;
            let during=r.client.wait_state(&id,"barrier","during_stream")?;
            r.client.ok("operation.cancel",json!({"operation_id":id}))?;let result=r.done(&id)?;no_directory(&fixture,&r.paths(&id))?;
            ensure(result["external_steps_admitted"]==during["external_steps_admitted"],"stress cancellation admitted new work")?;
            cases.push(json!({"kind":"fast_initial_release_stress","iteration":iteration,"worker_barrier_observed":true,"after":result}));
            r.client.ok("probe.forget",json!({"operation_id":id}))?;
        }
        r.no_input()?;Ok(json!({"evidence_kind":"real_server_deterministic_barriers","cases":cases,"input_bytes":0}))
    });
    report.case("upload","C08","Confirmed final survives real directory permission denial of temporary cleanup",|| {
        let id=r.op()?;r.start_upload(&id,json!({"hold_step":"cleanup"}))?;r.before_commit(&id)?;r.release(&id)?;r.client.wait_state(&id,"barrier","before_cleanup")?;
        let paths=r.paths(&id);
        remote(&fixture,async |s|{s.raw.setstat(&paths.0,russh_sftp::protocol::FileAttributes{permissions:Some(0o500),..Default::default()}).await?;ensure(s.raw.remove(&paths.1).await.is_err(),"fixture unlink permission not denied")?;Ok(())})?;
        r.release(&id)?;let result=r.done(&id)?;let files=observe(&fixture,&paths)?;
        ensure(result["upload_state"]=="committed"&&result["result"]["final_commit"]=="confirmed"&&result["cleanup_pending"]==true&&files[1]["exists"]==true&&files[2]["sha256"]==sha(&[1,2,3,4]),"cleanup revoked final")?;
        let injected_id=r.op()?;r.start_upload(&injected_id,json!({"fault":"unlink_denied"}))?;r.before_commit(&injected_id)?;r.release(&injected_id)?;let injected=r.done(&injected_id)?;
        let injected_files=observe(&fixture,&r.paths(&injected_id))?;
        ensure(injected["upload_state"]=="committed"&&injected["cleanup_pending"]==true&&injected["cleanup_steps"]==0&&injected_files[1]["exists"]==true&&injected_files[2]["sha256"]==sha(&[1,2,3,4]),"failed unlink response retracted final")?;
        r.no_input()?;Ok(json!({"evidence_kind":"real_server","operation":result,"files":files,"permission_oracle":"independent SFTP REMOVE denied; Core observes changed mode and conservatively refuses before unlink","rollback_final":false,"remove_error_branch":{"evidence_kind":"fault_injection","site":"REMOVE result after successful ownership observations, before successful-cleanup bookkeeping; no remove wire request","operation":injected,"files":injected_files}}))
    });
    report.case("upload","C09","Matching link reply loss is indeterminate despite read-only final observation; no replay/input/deletion",|| {
        let id=r.op()?;r.start_upload(&id,json!({"fault":"drop_link_ack"}))?;r.before_commit(&id)?;r.release(&id)?;let result=r.done(&id)?;
        ensure(result["upload_state"]=="indeterminate"&&result["result"]["final_commit"]=="indeterminate"&&result["ack_dropped"]==true&&result["cleanup_pending"]==true,"lost reply not indeterminate")?;
        let files=observe(&fixture,&r.paths(&id))?;ensure(files[1]["sha256"]==sha(&[1,2,3,4])&&files[2]["sha256"]==sha(&[1,2,3,4]),"actual link not observed")?;
        let replay=r.start_upload(&id,json!({}))?;ensure(replay["external_steps_admitted"]==result["external_steps_admitted"]&&replay["upload_state"]=="indeterminate","replay promoted outcome")?;
        r.client.denied("probe.insert",json!({"operation_id":id,"insert_key":"i","text":r.paths(&id).2,"context_revision":1,"mapping":true}),"UNKNOWN_DELIVERY")?;r.no_input()?;
        Ok(json!({"evidence_kind":"fault_injection","site":"framed SFTP STATUS matching actual hardlink request id, after real server execution","operation":result,"read_only_observation":files,"retry":replay,"input_bytes":0}))
    });
    report.case("upload","C10","Uncertain ledger/continuity and replaced temp are never deleted",|| {
        let mut cases=Vec::new();
        for fault in ["missing_ledger","lost_transport","boot_change","replaced_temp","replaced_parent"] {
            let id=r.op()?;r.start_upload(&id,if fault=="replaced_temp"||fault=="replaced_parent"{json!({"hold_step":"cleanup"})}else{json!({"hold_step":"cleanup","fault":fault})})?;r.before_commit(&id)?;r.release(&id)?;r.client.wait_state(&id,"barrier","before_cleanup")?;
            let paths=r.paths(&id);
            if fault=="replaced_temp" {remote(&fixture,async |s|{s.raw.remove(&paths.1).await?;s.write_exclusive(&paths.1,b"REPLACEMENT-LONGER").await?;Ok(())})?;}
            if fault=="replaced_parent" {remote(&fixture,async |s|{let original=format!("{}.original",paths.0);s.raw.rename(&paths.0,&original).await?;s.raw.symlink(&original,&paths.0).await?;Ok(())})?;}
            r.release(&id)?;let result=r.done(&id)?;let files=observe(&fixture,&paths)?;
            ensure(result["cleanup_pending"]==true&&result["cleanup_steps"]==0&&files[1]["exists"]==true&&files[2]["sha256"]==sha(&[1,2,3,4]),"uncertain cleanup deleted resource")?;
            if fault=="replaced_temp" {ensure(files[1]["sha256"]==sha(b"REPLACEMENT-LONGER"),"replacement bytes changed")?;}
            if fault=="replaced_parent" {ensure(files[0]["symlink"]==true&&files[1]["sha256"]==sha(&[1,2,3,4]),"cleanup parent replacement or original temp changed")?;}
            cases.push(json!({"kind":fault,"evidence_kind":if fault=="replaced_temp"||fault=="replaced_parent"{"real_server"}else{"fault_injection"},"site":match fault {"missing_ledger"=>"remove actual directory ledger observation while retaining acknowledged creation", "lost_transport"=>"close actual SFTP session and kill its owned ssh child", "boot_change"=>"substitute distinct boot identity into ledger", "replaced_parent"=>"actual parent symlink replacement after confirmed publication; normal cleanup ancestor check", _=>"actual remote temp replacement"},"operation":result,"files":files}));
        }
        let id=r.op()?;r.start_upload(&id,json!({"hold_step":"cleanup"}))?;r.before_commit(&id)?;r.release(&id)?;r.client.wait_state(&id,"barrier","before_cleanup")?;
        let next=r.client.ok("session.reconnect",json!({"session_id":r.sid,"writer_epoch":1}))?;
        let result=r.done(&id)?;let files=observe(&fixture,&r.paths(&id))?;
        ensure(next["generation"]==2&&result["cleanup_pending"]==true&&result["cleanup_steps"]==0&&files[1]["exists"]==true,"new generation adopted cleanup")?;
        cases.push(json!({"kind":"new_generation","evidence_kind":"real_core_transition","operation":result,"files":files}));r.session("primary")?;
        r.no_input()?;Ok(json!({"cases":cases,"limitation":"missing/stale ledger is injected evidence mutation; transport is actually closed; C09 is reply loss and full suite covers Core restart"}))
    });
    report.case("upload","C11","Same-key success and changed-intent retries cannot republish; context refusal retains final",|| {
        let id=r.op()?;r.start_upload(&id,json!({}))?;r.before_commit(&id)?;r.release(&id)?;let result=r.done(&id)?;let replay=r.start_upload(&id,json!({}))?;
        ensure(replay["external_steps_admitted"]==result["external_steps_admitted"],"success replay effect")?;
        r.client.denied("probe.upload",json!({"operation_id":id,"upload_key":"u","basename":"different.png","data":[1,2,3,4]}),"DEDUP_CONFLICT")?;
        r.client.denied("probe.insert",json!({"operation_id":id,"insert_key":"i","text":r.paths(&id).2,"context_revision":1,"mapping":false}),"MAPPING_UNKNOWN")?;
        let files=observe(&fixture,&r.paths(&id))?;ensure(files[2]["sha256"]==sha(&[1,2,3,4]),"mapping refusal removed final")?;r.no_input()?;
        let mut scope_changes=Vec::new();
        for change in ["absent","root","namespace","policy"] {
            let id=r.op()?;r.start_upload(&id,json!({}))?;r.before_commit(&id)?;r.release(&id)?;let committed=r.done(&id)?;
            ensure(committed["upload_state"]=="committed","scope regression needs real commit")?;
            r.client.ok("probe.policy",json!({"session_id":r.sid,"change":change}))?;
            let denial=r.client.denied("probe.insert",json!({"operation_id":id,"insert_key":"i","text":r.paths(&id).2,"context_revision":1,"mapping":true}),"PATH_DENIED")?;
            r.no_input()?;let retained=observe(&fixture,&r.paths(&id))?;ensure(retained[2]["sha256"]==sha(&[1,2,3,4]),"scope change removed committed final")?;
            scope_changes.push(json!({"change":change,"denial":denial,"files":retained,"actual_pty_bytes_since_ready":0}));r.session("primary")?;
        }
        Ok(json!({"scope_changes_after_commit":scope_changes,"operation":result,"retry":replay,"files":files,"context_regression":"headless full Core suite exercises zero-byte refreshed context on same committed result"}))
    });
    report.case("upload","C12","Two generated targets keep files and PTY independent; unavailable upload leaves login usable",|| {
        let id=r.op()?;r.start_upload(&id,json!({}))?;r.before_commit(&id)?;r.release(&id)?;r.done(&id)?;
        let paths=r.paths(&id);let other=no_directory(&second,&paths)?;
        r.client.ok("probe.policy",json!({"session_id":r.sid,"change":"absent"}))?;
        r.client.ok("probe.input",json!({"session_id":r.sid,"writer_epoch":1,"data":b"LOGIN_STILL_WORKS"}))?;r.client.wait_output(&r.sid,b"LOGIN_STILL_WORKS")?;
        r.session("secondary")?;r.no_input()?;
        Ok(json!({"run_ids":[fixture.run_id,second.run_id],"primary":observe(&fixture,&paths)?,"secondary":other,"login_echo":true,"switch_input_bytes":0,"full_core_suite":"simultaneous uploads and switching during stream tested separately"}))
    });
    report.case("upload","C13","Source and fixture linkage retained for independent check; no general-root promotion",|| {
        let mut sources=serde_json::Map::new();
        for path in ["probes/phase0/harness/src/main.rs","probes/phase0/harness/src/suite.rs","probes/phase0/harness/src/core.rs","probes/phase0/harness/src/sftp.rs","probes/phase0/harness/src/upload.rs","probes/phase0/harness/src/suite/upload_cases.rs","probes/phase0/Cargo.lock"] {sources.insert(path.into(),json!(sha(&fs::read(path)?)));}
        Ok(json!({"sources":sources,"fixture_runs":[fixture.run_id,second.run_id],"independent_review":"pending separate run","fixture_teardown":"owned by main after independent check","historical_phase0_ready":false,"general_root":"no-go"}))
    });
    r.client.ok("session.close", json!({"session_id":r.sid}))?;
    r.client.close();
    core.remove()?;
    let failed = report.cases.iter().any(|c| c.status != "pass");
    let document = json!({"schema":1,"suite":"cooperative-upload-runtime","cases":report.cases,"controlled_cases_passed":!failed,"acceptance_complete":false,"phase0_ready":false,"pending":["independent review and source fingerprint validation","exact fixture teardown by main"],"limitations":["cooperative stable ancestors only","finite Linux/OpenSSH fixture qualification; no universal support","probe RPC is not full v1","fault injections explicitly separated","no desktop accessed"],"runtime_cleanup":"known local files and owned Core process removed"});
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &document)?;
    file.write_all(b"\n")?;
    if failed {
        return Err("upload acceptance cases failed".into());
    }
    Ok(())
}
