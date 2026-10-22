//! Disposable public plugin boundary experiments, not a product plugin manager.
use crate::native::Pty;
use crate::{json, random_id, Fixture, Result, Value};
use sshx_phase0_protocol::{read_frame, write_frame, RequestCounter};
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const LOG_MAX: usize = 262_144;
const EVENT_MAX: usize = 128;
const CONTROL_BYTES_MAX: usize = 1_048_576;
const CONTROL_FRAMES_MAX: usize = 16;
const OPERATION: &str = "00000000000000000000000000000001";
type Frame = std::result::Result<Option<Value>, String>;
type Message = (Frame, usize);

struct CountedReader<R> {
    inner: R,
    bytes: usize,
}
impl<R: Read> Read for CountedReader<R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let count = self.inner.read(output)?;
        self.bytes += count;
        Ok(count)
    }
}
impl<R: BufRead> BufRead for CountedReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, count: usize) {
        self.bytes += count;
        self.inner.consume(count);
    }
}

#[derive(Default)]
struct QueueBudget {
    bytes: usize,
    frames: usize,
    peak_bytes: usize,
    peak_frames: usize,
    overflow: bool,
}
impl QueueBudget {
    fn admit(&mut self, bytes: usize) -> bool {
        if bytes > CONTROL_BYTES_MAX.saturating_sub(self.bytes) || self.frames >= CONTROL_FRAMES_MAX
        {
            self.overflow = true;
            return false;
        }
        self.bytes += bytes;
        self.frames += 1;
        self.peak_bytes = self.peak_bytes.max(self.bytes);
        self.peak_frames = self.peak_frames.max(self.frames);
        true
    }
    fn release(&mut self, bytes: usize) {
        self.bytes -= bytes;
        self.frames -= 1;
    }
}

fn ensure(value: bool, message: &str) -> Result<()> {
    if value {
        Ok(())
    } else {
        Err(message.to_string().into())
    }
}

#[derive(Default)]
struct Logs {
    ring: VecDeque<u8>,
    seen: usize,
    line_length: usize,
    line_dropped: usize,
}
impl Logs {
    fn retain(&mut self, bytes: &[u8]) {
        self.seen = self.seen.saturating_add(bytes.len());
        for byte in bytes {
            if *byte == b'\n' {
                let keep = self.line_length < 4096;
                self.line_length = 0;
                if !keep {
                    self.line_dropped = self.line_dropped.saturating_add(1);
                    continue;
                }
            } else if self.line_length >= 4096 {
                self.line_dropped = self.line_dropped.saturating_add(1);
                continue;
            } else {
                self.line_length += 1;
            }
            if self.ring.len() == LOG_MAX {
                self.ring.pop_front();
            }
            self.ring.push_back(*byte);
        }
    }
}

struct Peer {
    child: Child,
    input: Option<ChildStdin>,
    receiver: Option<mpsc::Receiver<Message>>,
    reader: Option<JoinHandle<()>>,
    logger: Option<JoinHandle<()>>,
    logs: Arc<Mutex<Logs>>,
    queue_budget: Arc<Mutex<QueueBudget>>,
    instance: String,
}
impl Peer {
    fn spawn(script: &Path, mode: &str) -> Result<Self> {
        let mut child = Command::new("/usr/bin/python3")
            .args(["-I", "-S"])
            .arg(script)
            .arg(mode)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("LANG", "C.UTF-8")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let input = child.stdin.take().ok_or("plugin stdin")?;
        let output = child.stdout.take().ok_or("plugin stdout")?;
        let mut stderr = child.stderr.take().ok_or("plugin stderr")?;
        let (sender, receiver) = mpsc::sync_channel(CONTROL_FRAMES_MAX);
        let queue_budget = Arc::new(Mutex::new(QueueBudget::default()));
        let reader_budget = queue_budget.clone();
        let reader = thread::spawn(move || {
            let mut output = CountedReader {
                inner: BufReader::new(output),
                bytes: 0,
            };
            loop {
                let before = output.bytes;
                let frame = read_frame(&mut output).map_err(|e| e.to_string());
                let bytes = output.bytes - before;
                let stop = !matches!(&frame, Ok(Some(_)));
                if !reader_budget
                    .lock()
                    .expect("plugin queue mutex")
                    .admit(bytes)
                {
                    break;
                }
                if sender.send((frame, bytes)).is_err() || stop {
                    break;
                }
            }
        });
        let logs = Arc::new(Mutex::new(Logs::default()));
        let log_copy = logs.clone();
        let logger = thread::spawn(move || {
            let mut buffer = [0; 8192];
            while let Ok(length) = stderr.read(&mut buffer) {
                if length == 0 {
                    break;
                }
                log_copy
                    .lock()
                    .expect("plugin log mutex")
                    .retain(&buffer[..length]);
            }
        });
        Ok(Self {
            child,
            input: Some(input),
            receiver: Some(receiver),
            reader: Some(reader),
            logger: Some(logger),
            logs,
            queue_budget,
            instance: random_id()?,
        })
    }
    fn send(&mut self, value: Value) -> Result<()> {
        write_frame(self.input.as_mut().ok_or("plugin stopped")?, &value)?;
        Ok(())
    }
    fn receive(&self, timeout: Duration) -> Frame {
        if self
            .queue_budget
            .lock()
            .expect("plugin queue mutex")
            .overflow
        {
            return Err("CONTROL_QUEUE_MAX".into());
        }
        match self
            .receiver
            .as_ref()
            .expect("live receiver")
            .recv_timeout(timeout)
        {
            Ok((value, bytes)) => {
                self.queue_budget
                    .lock()
                    .expect("plugin queue mutex")
                    .release(bytes);
                value
            }
            Err(mpsc::RecvTimeoutError::Timeout) => Err("TIMEOUT".into()),
            Err(mpsc::RecvTimeoutError::Disconnected) => Ok(None),
        }
    }
    fn next(&self) -> Result<Value> {
        self.receive(Duration::from_secs(5))
            .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.into() })?
            .ok_or_else(|| "plugin EOF".into())
    }
    fn stop(&mut self) {
        self.input.take();
        self.receiver.take();
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(logger) = self.logger.take() {
            let _ = logger.join();
        }
    }
    fn log_projection(&self) -> Value {
        let logs = self.logs.lock().expect("plugin log mutex");
        json!({"bytes_seen": logs.seen, "bytes_retained": logs.ring.len(), "bytes_dropped": logs.seen.saturating_sub(logs.ring.len()), "line_bytes_dropped":logs.line_dropped,"line_limit":4096,"body_recorded": false})
    }
    fn queue_projection(&self) -> Value {
        let budget = self.queue_budget.lock().expect("plugin queue mutex");
        json!({"wire_bytes_peak":budget.peak_bytes,"frames_peak":budget.peak_frames,"wire_bytes_limit":CONTROL_BYTES_MAX,"frames_limit":CONTROL_FRAMES_MAX,"overflow_refused":budget.overflow,"scope":"queued wire bytes including headers; JSON heap and one bounded in-flight parser frame are separate"})
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn request(id: u64, method: &str, params: Value) -> Value {
    json!({"jsonrpc":"2.0", "id":id.to_string(), "method":method, "params":params})
}
fn response(id: &Value, value: Value) -> Value {
    json!({"jsonrpc":"2.0", "id":id, "result":value})
}
fn refusal(id: &Value, name: &str, code: i32) -> Value {
    json!({"jsonrpc":"2.0", "id":id, "error":{"code":code,"message":"Synthetic broker refused request","data":{"name":name,"retryable":false,"operation_id":OPERATION,"phase":"retained"}}})
}

struct Broker {
    counter: RequestCounter,
    read_grant: String,
    image_grant: String,
    image_grant_active: bool,
    image_grant_revoked: bool,
    denied: Vec<String>,
    requery: bool,
    event_peak: usize,
    event_bytes_peak: usize,
    events_dropped: usize,
}

struct RestartBudget {
    starts: u8,
    window_started: Instant,
}
impl RestartBudget {
    fn new() -> Self {
        Self {
            starts: 0,
            window_started: Instant::now(),
        }
    }
    fn admit(&mut self, now: Instant) -> Option<Duration> {
        if now.duration_since(self.window_started) >= Duration::from_secs(60) {
            self.starts = 0;
            self.window_started = now;
        }
        let delay = match self.starts {
            0 => 0,
            1 => 1,
            2 => 4,
            _ => return None,
        };
        self.starts += 1;
        Some(Duration::from_secs(delay))
    }
}
impl Broker {
    fn new() -> Result<Self> {
        Ok(Self {
            counter: RequestCounter::default(),
            read_grant: random_id()?,
            image_grant: random_id()?,
            image_grant_active: false,
            image_grant_revoked: false,
            denied: Vec::new(),
            requery: false,
            event_peak: 0,
            event_bytes_peak: 0,
            events_dropped: 0,
        })
    }
    fn deny(&mut self, id: &Value, name: &str, code: i32) -> Value {
        self.denied.push(name.into());
        refusal(id, name, code)
    }
    fn handle(&mut self, peer: &mut Peer, value: Value) -> Result<bool> {
        ensure(value["jsonrpc"] == "2.0", "invalid RPC version")?;
        let id = value.get("id").ok_or("request id required")?;
        self.counter
            .admit(id.as_str().ok_or("request counter required")?)?;
        let method = value["method"].as_str().ok_or("method required")?;
        let params = value["params"]
            .as_object()
            .ok_or("params object required")?;
        let mut complete = false;
        let answer = match method {
            "action.register" => {
                ensure(
                    params.len() == 1 && params.contains_key("actions"),
                    "register field set",
                )?;
                response(id, json!({"registered":["wire"]}))
            }
            "client.hello" | "session.attach" | "session.start" => {
                self.deny(id, "ROLE_DENIED", -32000)
            }
            "session.describe" | "operation.get" => {
                if params.len() != 1 || !params.contains_key("operation_id") {
                    self.deny(id, "INVALID_PARAMS", -32602)
                } else if params["operation_id"] != OPERATION {
                    self.deny(id, "CAPABILITY_DENIED", -32000)
                } else if method == "operation.get" {
                    self.requery = true;
                    response(
                        id,
                        json!({"operation_id":OPERATION,"phase":"retained","upload_state":"none"}),
                    )
                } else {
                    response(
                        id,
                        json!({"session_id":"00000000000000000000000000000002","generation":1,"target":{"display_label":"generated-loopback","verified_target_id":null},"authentication":"unknown","shell_readiness":"unknown","remote_context":null,"capabilities":["session.read"]}),
                    )
                }
            }
            "clipboard.read_image" => {
                let grant_matches = params
                    .get("grant_ids")
                    .and_then(Value::as_array)
                    .is_some_and(|ids| {
                        ids.iter()
                            .any(|id| id.as_str() == Some(self.image_grant.as_str()))
                    });
                if params.get("operation_id").and_then(Value::as_str) == Some(OPERATION)
                    && grant_matches
                    && self.image_grant_active
                {
                    self.deny(id, "UNAVAILABLE", -32000)
                } else {
                    self.deny(id, "CAPABILITY_DENIED", -32000)
                }
            }
            "transfer.upload" | "terminal.propose_insert" => {
                // No desktop/transfer/input grant exists in this scope. Even the
                // declared read-only scope cannot authorize a composed exfiltration.
                self.deny(id, "CAPABILITY_DENIED", -32000)
            }
            "event.subscribe" => {
                ensure(params["operation_id"] == OPERATION, "event owner")?;
                response(id, json!({"subscription_id":"one","cursor":0}))
            }
            "ui.notify" => {
                ensure(
                    params["operation_id"] == OPERATION
                        && params["text"] == "synthetic-wire-checked",
                    "bounded notification",
                )?;
                complete = true;
                response(id, json!({"delivered":true}))
            }
            _ => self.deny(id, "METHOD_NOT_FOUND", -32601),
        };
        peer.send(answer)?;
        if method == "event.subscribe" {
            self.events(peer)?;
        }
        Ok(complete)
    }
    fn events(&mut self, peer: &mut Peer) -> Result<()> {
        let mut queue = VecDeque::new();
        for sequence in 0..300 {
            if queue.len() == EVENT_MAX {
                self.events_dropped += queue.len();
                queue.clear();
                queue.push_back(json!({"sequence":sequence,"kind":"event.gap","payload":{"last_delivered":null,"next_available":sequence+1,"requery":["session.describe","operation.get"]}}));
            } else {
                queue.push_back(json!({"sequence":sequence,"kind":"operation.progress","payload":{"completed":sequence}}));
            }
            self.event_peak = self.event_peak.max(queue.len());
            let bytes = queue
                .iter()
                .map(|item| serde_json::to_vec(item).map(|v| v.len()))
                .collect::<std::result::Result<Vec<_>, _>>()?
                .iter()
                .sum();
            self.event_bytes_peak = self.event_bytes_peak.max(bytes);
            ensure(bytes <= 1_048_576, "event aggregate byte budget")?;
        }
        queue.push_back(
            json!({"sequence":300,"kind":"operation.result","payload":{"operation_id":OPERATION}}),
        );
        ensure(queue.len() <= EVENT_MAX, "event queue reserved result slot")?;
        for event in queue {
            peer.send(json!({"jsonrpc":"2.0","method":"event","params":{"api_version":"1.0","event_id":format!("event-{}",event["sequence"]),"subscription_id":"one","session_id":"00000000000000000000000000000002","generation":1,"operation_id":OPERATION,"source":"core","sequence":event["sequence"],"kind":event["kind"],"payload":event["payload"]}}))?;
        }
        Ok(())
    }
}

fn initialize(peer: &mut Peer) -> Result<Value> {
    peer.send(request(1, "plugin.initialize", json!({"api_versions":["1.0"],"core_instance_id":"00000000000000000000000000000003","core_version":"phase0","manifest_digest":"synthetic-trusted-fixture","limits_profile":"proposed-v1"})))?;
    peer.next()
}
fn ready(peer: &mut Peer) -> Result<()> {
    let instance = peer.instance.clone();
    peer.send(json!({"jsonrpc":"2.0","method":"plugin.ready","params":{"api_version":"1.0","instance_id":instance,"available_methods":["action.register","session.describe","operation.get","event.subscribe","ui.notify"]}}))
}
fn action(peer: &mut Peer, broker: &Broker) -> Result<()> {
    peer.send(request(2, "action.run", json!({"operation_id":OPERATION,"binding":{"core_instance_id":"00000000000000000000000000000003","operation_id":OPERATION,"session_id":"00000000000000000000000000000002","generation":1,"client_id":"00000000000000000000000000000004","attachment_id":"00000000000000000000000000000005","writer_epoch":1,"profile_id":"00000000000000000000000000000006","config_generation":1},"target":{"display_label":"generated-loopback"},"action_id":"wire","grant_ids":[broker.read_grant,broker.image_grant]})))
}

fn one_case(script: &Path, mode: &str, progress: &Pty) -> Result<Value> {
    let before = progress.bytes_read.load(Ordering::Relaxed);
    let started = Instant::now();
    let mut peer = Peer::spawn(script, mode)?;
    let pid = peer.child.id();
    let mut broker = Broker::new()?;
    let mut observed = String::from("negotiated");
    if mode == "handshake-timeout" {
        peer.send(request(
            1,
            "plugin.initialize",
            json!({"api_versions":["1.0"]}),
        ))?;
        ensure(
            matches!(peer.receive(Duration::from_secs(5)), Err(ref e) if e == "TIMEOUT"),
            "handshake timeout not enforced",
        )?;
        observed = "TIMEOUT".into();
    } else {
        let hello = initialize(&mut peer)?;
        ensure(hello["id"] == "1", "initialize correlation")?;
        if mode == "wrong-version" {
            ensure(
                hello["result"]["api_version"] != "1.0",
                "incompatible fixture version",
            )?;
            observed = "VERSION_MISMATCH".into();
        } else {
            ensure(hello["result"]["api_version"] == "1.0", "API version")?;
            ready(&mut peer)?;
            if mode == "crash" {
                ensure(
                    peer.receive(Duration::from_secs(5)) == Ok(None),
                    "plugin crash did not close channel",
                )?;
                ensure(
                    peer.child.wait()?.code() == Some(42),
                    "synthetic crash status",
                )?;
                observed = "isolated exit 42".into();
            } else {
                let registration = peer.next()?;
                ensure(
                    registration["method"] == "action.register",
                    "register required",
                )?;
                broker.handle(&mut peer, registration)?;
                broker.image_grant_active = mode == "revoked-grant";
                action(&mut peer, &broker)?;
                if mode == "response-order" {
                    peer.send(request(
                        3,
                        "plugin.stop",
                        json!({"reason":"synthetic reverse response ordering"}),
                    ))?;
                    let third = peer.next()?;
                    let second = peer.next()?;
                    ensure(
                        third["id"] == "3"
                            && third["result"]["stopping"] == true
                            && second["id"] == "2"
                            && second["result"]["accepted"] == true,
                        "out-of-order pending correlation",
                    )?;
                    observed = "pending ids 2/3 resolved in order 3/2".into();
                } else {
                    let accepted = peer.next()?;
                    ensure(
                        accepted["id"] == "2" && accepted["result"]["accepted"] == true,
                        "action acknowledgement",
                    )?;
                    if mode == "revoked-grant" {
                        ensure(
                            broker.image_grant_active,
                            "revocation fixture had no active grant",
                        )?;
                        broker.image_grant_active = false;
                        broker.image_grant_revoked = true;
                    }
                    let fault = [
                        "stdout-log",
                        "duplicate-header",
                        "bare-lf",
                        "oversize",
                        "partial-frame",
                        "duplicate-key",
                        "batch",
                        "depth",
                        "trailing-json",
                        "invalid-utf8",
                        "bom",
                        "nonfinite",
                        "counter-replay",
                        "counter-skipped",
                    ]
                    .contains(&mode);
                    if mode == "queue-byte-flood" {
                        let until = Instant::now() + Duration::from_secs(5);
                        while !peer.queue_budget.lock().expect("queue budget").overflow
                            && Instant::now() < until
                        {
                            thread::sleep(Duration::from_millis(5));
                        }
                        ensure(
                            matches!(peer.receive(Duration::from_millis(1)), Err(ref name) if name == "CONTROL_QUEUE_MAX"),
                            "queued wire-byte limit not enforced",
                        )?;
                        let budget = peer.queue_budget.lock().expect("queue budget");
                        ensure(budget.peak_bytes <= CONTROL_BYTES_MAX && budget.peak_frames == 4, "queue admission exceeded byte budget or did not exercise byte-first exhaustion")?;
                        observed = "CONTROL_QUEUE_MAX at fifth valid large frame".into();
                    } else if fault {
                        let received = peer.receive(Duration::from_secs(5));
                        if mode.starts_with("counter-") {
                            let value = received
                                .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                                    e.into()
                                })?
                                .ok_or("counter fixture EOF")?;
                            ensure(
                                broker.handle(&mut peer, value).is_err(),
                                "counter reuse/skipped admitted",
                            )?;
                            observed = "request counter closed".into();
                        } else {
                            ensure(received.is_err(), "invalid frame admitted")?;
                            observed = received.err().unwrap_or_default();
                        }
                    } else {
                        loop {
                            let value = peer.next()?;
                            if broker.handle(&mut peer, value)? {
                                break;
                            }
                        }
                        peer.send(request(
                            3,
                            "plugin.stop",
                            json!({"reason":"probe complete"}),
                        ))?;
                        if mode == "blocked-rpc" {
                            ensure(
                                matches!(peer.receive(Duration::from_secs(10)),Err(ref e) if e == "TIMEOUT"),
                                "RPC timeout not enforced",
                            )?;
                            observed = "TIMEOUT at plugin.stop".into();
                        } else {
                            let stopped = peer.next()?;
                            ensure(
                                stopped["id"] == "3" && stopped["result"]["stopping"] == true,
                                "stop acknowledgement",
                            )?;
                            ensure(peer.child.wait()?.success(), "normal fixture exit")?;
                        }
                    }
                }
            }
        }
    }
    let minimum = started + Duration::from_millis(120);
    while Instant::now() < minimum {
        thread::sleep(Duration::from_millis(5));
    }
    let after = progress.bytes_read.load(Ordering::Relaxed);
    ensure(
        after > before,
        "real SSH PTY stopped progressing during plugin case",
    )?;
    ensure(
        progress.child.id() != pid,
        "plugin and SSH child ownership mixed",
    )?;
    peer.stop();
    if mode == "stderr-flood" || mode == "stderr-long-line" {
        let logs = peer.logs.lock().expect("logs");
        ensure(
            logs.seen == 524_288
                && logs.ring.len()
                    == if mode == "stderr-flood" {
                        LOG_MAX
                    } else {
                        4096
                    },
            "stderr drain/ring bound",
        )?;
    }
    if mode == "events" {
        ensure(
            broker.requery && broker.events_dropped > 0 && broker.event_peak <= EVENT_MAX,
            "event gap/requery/bound",
        )?;
    }
    let expected_denial = match mode {
        "role-upgrade" => Some("ROLE_DENIED"),
        "identity-spoof" => Some("INVALID_PARAMS"),
        "cross-operation" | "revoked-grant" | "composed-grant" => Some("CAPABILITY_DENIED"),
        "unknown-method" => Some("METHOD_NOT_FOUND"),
        _ => None,
    };
    if let Some(expected) = expected_denial {
        ensure(
            broker.denied.iter().any(|item| item == expected),
            "expected effect-site refusal missing",
        )?;
    }
    Ok(
        json!({"plugin_pid":pid,"instance_id":peer.instance,"observed":observed,"denied":broker.denied,"ssh_pty_bytes_before":before,"ssh_pty_bytes_after":after,"elapsed_ms":started.elapsed().as_millis(),"plugin_environment":"python3 -I -S; env_clear; PATH and LANG only","private_handles_passed":false,"ungranted_effects_dispatched":0,"image_grant_was_active_then_revoked":broker.image_grant_revoked,"grant_records":2,"request_counter_storage":"one high-water counter plus bounded pending channel","pending_channel_frames_max":CONTROL_FRAMES_MAX,"queue":peer.queue_projection(),"logs":peer.log_projection(),"event_queue_peak":broker.event_peak,"event_bytes_peak":broker.event_bytes_peak,"events_dropped":broker.events_dropped,"operation_requeried":broker.requery}),
    )
}

fn script_path() -> Result<PathBuf> {
    for ancestor in std::env::current_exe()?.ancestors().skip(1) {
        let script = ancestor.join("probes/phase0/fixtures/plugin.py");
        if script.is_file() {
            return Ok(script);
        }
    }
    Err("cannot find reviewed repository plugin.py relative to native probe binary".into())
}

pub fn run(fixture: &Fixture) -> Result<Value> {
    let script = script_path()?;
    let mut ssh = Command::new("/usr/bin/ssh");
    ssh.args(fixture.ssh_args(None)).args(["-T","fixture","python3 -u -c 'import time; [(print(\"SSHX_PLUGIN_PROGRESS\",flush=True),time.sleep(0.02)) for _ in range(15000)]'"])
        .env_clear().env("PATH","/usr/bin:/bin").env("LANG","C.UTF-8");
    let progress = Pty::spawn(&mut ssh)?;
    ensure(
        progress.wait_contains(b"SSHX_PLUGIN_PROGRESS", Duration::from_secs(7)),
        "real SSH progress oracle unavailable",
    )?;
    let mut cases = Vec::new();
    for mode in [
        "unicode-split",
        "response-order",
        "role-upgrade",
        "identity-spoof",
        "cross-operation",
        "revoked-grant",
        "composed-grant",
        "unknown-method",
        "stderr-flood",
        "stderr-long-line",
        "events",
        "crash",
        "wrong-version",
        "stdout-log",
        "duplicate-header",
        "bare-lf",
        "oversize",
        "duplicate-key",
        "batch",
        "depth",
        "trailing-json",
        "invalid-utf8",
        "bom",
        "nonfinite",
        "counter-replay",
        "counter-skipped",
        "partial-frame",
        "handshake-timeout",
        "blocked-rpc",
        "queue-byte-flood",
    ] {
        let outcome = one_case(&script, mode, &progress);
        let (status, observations) = match outcome {
            Ok(value) => ("pass", value),
            Err(error) => ("fail", json!({"error":error.to_string()})),
        };
        eprintln!("E6 plugin-{mode}: {status}");
        cases.push(json!({"case":format!("plugin-{mode}"),"status":status,"expected":"Public framed peer fault remains isolated while actual SSH PTY bytes progress; expected refusal/correlation/budget applies","observations":observations,"cleanup_status":"exact plugin child killed/waited; reader/logger threads joined"}));
    }
    let restart = (|| -> Result<Value> {
        let mut instances = Vec::new();
        let mut budget = RestartBudget::new();
        let mut backoff = Vec::new();
        while let Some(delay) = budget.admit(Instant::now()) {
            if !delay.is_zero() {
                backoff.push(delay.as_secs());
                thread::sleep(delay);
            }
            let result = one_case(&script, "crash", &progress)?;
            instances.push(result["instance_id"].clone());
        }
        ensure(
            instances.len() == 3 && budget.admit(Instant::now()).is_none(),
            "restart ceiling not enforced",
        )?;
        ensure(
            instances[0] != instances[1] && instances[1] != instances[2],
            "restart reused instance",
        )?;
        Ok(
            json!({"actual_starts":instances.len(),"automatic_restarts":instances.len()-1,"fourth_start_refused":true,"backoff_seconds":backoff,"restart_window_seconds":60,"instance_ids":instances,"action_replays":0}),
        )
    })();
    let (status, observations) = match restart {
        Ok(value) => ("pass", value),
        Err(error) => ("fail", json!({"error":error.to_string()})),
    };
    cases.push(json!({"case":"plugin-restart-ceiling","status":status,"expected":"Initial start plus at most two automatic restarts; new instance/grants and no action replay","observations":observations,"cleanup_status":"all three exact plugin children waited"}));
    let packages = (|| -> Result<Value> {
        let mut peer = Peer::spawn(&script, "package-cases")?;
        let result = peer.next()?;
        ensure(
            peer.child.wait()?.success(),
            "package fixture assertions failed",
        )?;
        let value = result["result"].clone();
        ensure(
            value["cases"].as_array().is_some_and(|items| {
                items.len() == 14 && items.iter().all(|item| item["status"] == "pass")
            }),
            "package fixture coverage",
        )?;
        peer.stop();
        Ok(value)
    })();
    let (status, observations) = match packages {
        Ok(value) => ("pass", value),
        Err(error) => ("fail", json!({"error":error.to_string()})),
    };
    cases.push(json!({"case":"bounded-package-archive-gate","status":status,"expected":"Valid synthetic USTAR+gzip accepted; traversal/absolute/duplicate/link/device/API/platform/entrypoint/install-hook/manifest/file-count/decompression attacks rejected without extraction","observations":observations,"cleanup_status":"Python TemporaryDirectory removed archives; exact child waited"}));
    let failed = cases.iter().filter(|case| case["status"] == "fail").count();
    Ok(
        json!({"status":if failed==0{"pass"}else{"fail"},"cases":cases,"failed":failed,"ssh_progress_oracle":{"owner":"native harness PTY/OpenSSH child","bytes_read":progress.bytes_read.load(Ordering::Relaxed),"terminal_bodies_sent_to_plugin":false},"limitations":["Focused trusted-plugin broker and bounded USTAR+gzip package profile are feasibility probes, not the product plugin manager/installer","Request, event, log and restart budgets exercised here; full Core grant/ticket/metadata admission uses separate Core probes","Independent process isolation is not a same-user malicious-code sandbox"]}),
    )
}
