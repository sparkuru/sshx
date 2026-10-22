# Phase 0 feasibility runbook and later gates

Status: **controlled Linux probes implemented and independently checked; general MVP promotion blocked**, 2026-10-22. The [formal report](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check-report.md) records 14 passing unit tests and 44 passing native cases, including independent plugins and bound-image effects. [Acceptance/source map](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/acceptance-map.md) accounts for XII.1–12. [Exact final cleanup](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/cleanup.md) passed. General SFTP parent/root atomic binding remains no-go; full hidden-screen restoration and product/platform scope remain limited. Passing controlled cases do not establish Phase 0 PASS or MVP readiness. The user subsequently accepted the precise negative feasibility deliverable and local Git/archive batch; the task is archived, while general-root capability remains unsupported.

Source basis: [XI–XIII](../.trellis/spec/trellis-plus/sshx-design.txt). Experiments E0–E7 jointly cover the high-risk questions. [Protocol](protocol.md) owns every proposed resource limit/error; record any experimental deviation rather than silently editing the baseline. [Matrix](platform-matrix.md) owns platform dimensions and [ADRs](adr/0001-language.md) own selection gates.

## Subsequent scope decision and future runtime gates

After accepting the Phase 0 negative deliverable, the user selected A on 2026-10-22: a cooperative remote server/account with stable upload ancestors and no independent modification/replacement of operation-owned resources through cleanup. The [protocol scope](protocol.md#cooperative-upload-scope) now makes this an explicit per-target local policy precondition using existing `path_scope.policy: "cooperative"`, `scope_digest` and B. A logical canonical SFTP root is not a jail or durable directory capability; 0700/random names/metadata do not establish one. Stronger same-account confinement remains unsupported, and observed path/ownership inconsistency stops new effects. Login and input-context authorization remain independent.

The future regular-PNG candidate in B uses verified `hardlink@openssh.com` v1 to publish a closed temp to a new final basename in the same operation directory/filesystem, then separate owned-temp cleanup. [ADR 3](adr/0003-transfer-backend.md) records version-pinned source evidence that baseline rename may fall back to a racy stat/rename on selected filesystem errors. This caveat does not rewrite the historical positive fixture result. The archived probe revision used `raw.rename`; the current Core path now uses hardlink under explicit fixture qualification. Historical extension names alone prove neither version nor dedicated hardlink execution. Source-bound reports, their checksums and `phase0_ready: false` are unchanged.

The original document-only scope decision supplied input to later MVP planning. The subsequently authorized runtime task now exercises the bounded probe path below. These product acceptance obligations remain applicable:

| Future assertion point | Required observable outcome |
| --- | --- |
| Common validation, then cooperative policy/scope admission | Malformed fields retain -32602; wrong roles/missing method grants retain their errors; valid absent/inapplicable cooperative policy or stronger confinement requirement gives PATH_DENIED with no path effect. Policy/target/root changes and cross-plugin use cannot retain authority |
| Creation/publication/cleanup observations | Stable cooperative fixture succeeds only under its stated assumptions; observed symlink/type/namespace/ownership inconsistency stops new effects. No claim to catch all replacement/ABA races |
| Actual hardlink extension version and filesystem | Verify v1 and execute regular-file publication/collision tests; existing final bytes unchanged, including a concurrent final-name creator. Unsupported version/filesystem/cross-device gives COMMIT_UNSUPPORTED without fallback; generic failure is not invented NAME_EXISTS |
| Link dispatched, acknowledgement lost | Record indeterminate final outcome; read-only reconciliation, no automatic re-publication or PTY input |
| Final link acknowledged, temp unlink fails | Upload remains committed with exact final path and cleanup_pending; never remove final to compensate |
| Cancel/revoke before/after each external step | No newly admitted upload/publication/input effects after revocation; in-flight effects retain actual/uncertain outcomes. Only Core's pre-existing narrow ledger authority may clean exact owned temps |
| Cleanup ownership/target/continuity lost | Pending cleanup; no replacement-path deletion, recursive sweep or adoption after reconnect/new Core/generation. Preserve committed final files and their containing operation directory |
| Confirmed final versus insertion | Mapping/context/writer failure retains final result with no guessed path/input; zero-byte insertion retry does not repeat publication |

Use actual remote bytes/path/PTY and effect counters, not merely expected state enums. Advertisement and source review alone are not passing runtime results; the current checkpoint below links executed observations. Existing target/config/auth, platform and full-product gaps remain; cooperative scope does not grant general MVP promotion or automatic Phase 1 execution.

## Cooperative runtime checkpoint

The current [runtime task](../.trellis/tasks/archive/2026-10/10-22-sshx-upload-root-runtime/prd.md) extends the existing Core path, not the archived report. Its [implementation evidence](../.trellis/tasks/archive/2026-10/10-22-sshx-upload-root-runtime/research/implementation-evidence.md) records source-linked commands and case mapping. Two fresh generated Debian OpenSSH 9.2p1 containers on one Docker host provide separate targets; their upload root is overlayfs and `/quota` is a distinct tmpfs. No other server/filesystem is certified.

The fixture-owner CLI flag `core --qualify-upload` explicitly authorizes synthetic preparation before serving requests: check extension value 1, link/read back a regular file, verify occupied-name preservation, then remove those acknowledged preparation resources. Core retains the resulting policy/qualification only for that process and actual fixture/config/namespace/root. Without the flag, login remains available but uploads have no policy. Action invocation or plugin requests cannot trigger qualification or create policy. Preparation failure aborts startup and never qualifies the fixture; uncertain preparation resources are left for exact fixture teardown.

`upload-suite` executes C01–C13 through the same Core upload path. C01–C12 cover qualification/policy refusal, publication and collisions, unavailable capability, path observations, cancel/revoke, final-versus-cleanup outcomes, matching reply loss, cleanup continuity, dedup/context and target isolation. C13 is source/environment bookkeeping; a passing case is **not** independent review or teardown completion. The report intentionally retains `acceptance_complete: false` until those separate records exist.

Real cases observe remote hashes/presence and native synthetic PTY bytes. Missing/unsupported advertisement, explicit unsupported response, lost reply and altered cleanup evidence are labeled fault injection. Cross-device rejection is a real SFTP generic failure; privileged device metadata is separate fixture evidence and never runtime EXDEV authority. A concurrent creator after precheck likewise retains generic IO_ERROR rather than inventing EEXIST. Actual permission denial is checked independently; Core conservatively stops at changed parent mode, with a separate injected REMOVE error covering result propagation. Unknown publication remains indeterminate even when an independent connection reads matching final bytes.

~~~sh
.devhome/target/debug/phase0-probe upload-suite --fixture PRIMARY_STATE --fixture-secondary SECOND_STATE --output NEW_UPLOAD_JSON
.devhome/target/debug/phase0-probe suite --headless --fixture PRIMARY_STATE --fixture-secondary SECOND_STATE --output NEW_CORE_JSON
~~~

`--headless` excludes the desktop capture/switch case and uses a generated PNG for the upload/insert regression; the excluded case is not counted as passed. The ordinary suite retains its explicitly authorized private-desktop mode. Current probe RPC projections use `final_commit: confirmed/indeterminate`, independent `cleanup_pending`, immutable dispatch plus definitive-rejection facts and cleanup counters. They remain an experimental subset: synthetic non-PNG bytes are also accepted by probe.upload, plugin/Client fixture identities are not a complete v1 implementation, and general Shell context, arbitrary configurations, hostile same-account confinement and platform promotion remain unsupported. Historical `phase0_ready: false` remains unchanged.

## Runnable Linux probe interfaces

The Phase 0 workspace is `probes/phase0`; `hako` builds it and `.devhome/target/debug/` holds host-native Linux binaries. Both binaries’ `--help` were exercised. These commands are now real interfaces, while per-case success still requires recorded results. Build dependencies stay inside the scoped Docker environment.

~~~sh
./hako --build-image
HAKO_NETWORK=bridge ./hako cargo fetch --locked
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --workspace --all-targets -- -D warnings
./hako cargo test --locked --workspace --all-targets
./hako cargo build --locked --workspace
phase0_state=$(./dev.sh start)
./dev.sh status
.devhome/target/debug/phase0-probe suite --fixture "$phase0_state" --output /tmp/sshx-core-new.json
.devhome/target/debug/phase0-image suite --output /tmp/sshx-visual-new.json
./dev.sh down
~~~

Run from the repository root, with Docker/native-fixture authority and the recorded prerequisites. Choose fresh output names: reports refuse replacement. `phase0-probe case --fixture STATE --case ID --output NEW_JSON` and `plugin --fixture STATE --output NEW_JSON` support focused reruns. For the complete two-target matrix add `--fixture-secondary SECOND_STATE`; create that separate generated fixture using the [recorded existing lifecycle helper](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/dual-target-fixture.md), preserving the primary pointer. Core/Client modes are disposable harness internals, with explicit `probe.*` fault controls; the released SSHX API does not exist. The image plugin builds independently against the public protocol crate. Its suite creates its own private display; it never uses the current DISPLAY.

The derived Rust image adds exact-toolchain rustfmt/Clippy to the pinned base; `--build-image` is explicit and ordinary commands default to network none. The SSH fixture binds only dynamically assigned IPv4 loopback and uses generated identities/passwords/known-host expectations. The [fixture report](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-evidence.md) records exact versions, mounts and lifecycle proof. These controlled Linux probes establish no Windows/macOS/Wayland, physical IME/MFA, generic live-Shell context or released-product support.

## Entry gate and controlled fixtures

Before executing anything, select the primary **native** OS/architecture, terminal host, local Shell/display and remote fixture set with the user. Confirm separate authority for necessary tool installation, remote connections, native clipboard access or disposable container/service startup. Use [the environment checkpoint](../.trellis/spec/trellis-plus/development.md), not a fabricated hako/Cargo command. Containers may host disposable remote SSH/SFTP services; they cannot prove client clipboard, Windows ConPTY, native macOS or Windows remote behavior.

Fixture owner supplies an isolated local test account/profile and a controlled SSH config, test key/agent, known_hosts with independently expected fingerprints, dedicated local evidence directory and dedicated remote root. No personal keys, home mounts, production endpoints or real clipboard contents. Use synthetic RGBA/color/alpha images and text. Password/MFA tests use disposable test secrets entered only into visible OpenSSH UI; redact credentials from evidence. Before mutation, record exact owned resource IDs/paths and teardown commands chosen for that fixture. Never wildcard-delete an unknown remote directory or shut down a shared service.

Required remote cases: working SFTP; SSH without SFTP; changed-host-key service; key/agent and additional password/MFA; restricted/no-write/full/quota-limited root; controlled disconnect; duplicate name and symlink/root-swap; chroot; a real Windows SFTP namespace for claims about Windows. Native macOS/Windows servers are separately identified gaps if absent. Network disruptions affect only owned test channels/fixture namespaces, never a host-wide firewall/network outage. Disk-full uses a limited disposable volume/quota, never filling a real system disk.

Each experiment has a standard end: stop its owned probe/client/plugin children, request normal close of owned Core/session/transport, wait under protocol grace, remove only verified temporary test artifacts, retain intended synthetic final files until collision/integrity assertions finish, then explicitly remove that fixture's known final files. Preserve evidence and report cleanup_pending on failure. Fixture owner releases services/accounts/volumes through their recorded exact-resource procedure; no cleanup of pre-existing user state.

## Evidence record

Create one record per case/combination in the future task's research directory. A missing fixture yields blocked/unverified, never pass. Native video/screenshots may supplement synthetic terminal cases but must not expose credentials or personal text.

~~~json
{"experiment":"E4","case":"no-clobber-commit","status":"not-run","date":null,"environment":{"client_os":null,"architecture":null,"terminal_host":null,"local_shell":null,"display":null,"ssh_build":null,"backend":null,"remote_os":null,"server_build":null,"auth_mode":null},"fixture_id":null,"source_revision":null,"commands":[],"expected":"existing destination bytes remain unchanged","observations":[],"selection_effect":null,"limitations":[],"owned_cleanup":[],"cleanup_status":"not-started"}
~~~

Record exact tool/library/features/versions, command argv (secrets excluded), fixture/config generation, timestamps, exit status, assertions, sanitized event sequence, hashes of synthetic artifacts, result and limitation. No terminal/clipboard body capture by default; synthetic byte fixtures may be compared by digest/length. Documentation review supports a contract proposal; only actual runs support platform claims.

## E0: environment and experiment readiness

**Inputs:** selected primary native environment, isolated fixture inventory and authorization. Read actual repository manifests if they exist by then. Read-only command shapes include:

~~~text
rustc --version
cargo --version
ssh -V
sftp -h
~~~

**Observe:** output and actual exit semantics; sftp help is not a transfer result. Record OS/kernel/architecture and terminal/Shell/display from their native version facilities without assuming environment-variable presence proves desktop access. Verify chosen dependencies' exact official interfaces before installing/selecting them. No selected toolchain means mark blocked, not install automatically.

**Pass:** every planned case has a native or controlled remote fixture owner, known data boundary, expected outcome and teardown; unavailable platforms are explicit matrix gaps. **Failure/cleanup:** no services need start for inventory; remove only task-created scratch metadata. E0 chooses what later probes can run, not a universal platform priority.

## Probe harness acceptance contract

E1–E7 use a small disposable experimental harness, separate from a released application. Proposed input is a case ID, explicit fixture manifest, protocol/limits revision, native adapter/backend candidate and deterministic interleaving schedule. Output is the evidence record above plus timestamped state/effect markers, resource ledger, synthetic byte hashes and final cleanup status. Fail with a nonzero status for assertion failure or unmet prerequisites; unsupported must be distinct from pass. It must expose barriers immediately before external open/publication/write and after dispatch/before acknowledgement so cancellation races are reproducible, rather than relying on sleeps.

The illustrative interface is “probe --experiment E7 --case switch-after-upload --fixture FIXTURE --schedule SCHEDULE --output EVIDENCE”. That illustrative shape is not the implemented CLI; use the runnable interfaces above and their exercised help. Case-specific contracts below are acceptance for that harness. Use standard-tool observations as baselines, not as evidence of an unbuilt Core.

## E1: Core, IPC, CLI and ownership

**Inputs/harness:** isolated native account/runtime directory, a second local account, synthetic endpoint/symlink fixtures, two Clients, managed ssh child fixture. Harness accepts invocation argv/stdio class and concurrent-start/attach schedule; emits actual child argv, handle ownership, peer check outcome, exit status and stream digests without secret environment values.

**Actions:** compare direct ssh with proposed passthrough for pipe/stdin/stdout/stderr redirection, remote command, -T, -N, -W, subsystem, -f/-n, -G/-Q/-V/-O and unknown options. Use binary fixtures containing non-UTF-8 and separate stderr. Test a host named session via connect --. For enhanced login, concurrently start Clients, connect to the authenticated ready Core, contend for one writer, resize from loser, switch and detach, kill only a Client, close remote Shell, stop busy Core and attempt incompatible Client/version upgrade. Race owned stale endpoint cleanup against replacement with a symlink/foreign endpoint; verify refusal, not deletion.

**Pass:** native ssh files/aliases are untouched; passthrough adds no SSHX stdout/stderr bytes, preserves argv/exit and never creates PTY/daemon; one Core in scope and one writer; wrong user/role denied; live sessions block silent upgrade/stop; Client loss leaves Core-owned ssh alive, revokes old input and requires fresh epoch. Core crash does not report metadata as live. **Failure/cleanup:** preserve redacted assertion trace; close only owned children/endpoints after ownership checks. Native ACL/peer tests unavailable on an OS remain unverified.

## E2: terminal, OpenSSH authentication and prefix

**Inputs/harness:** real native terminal/desktop and isolated known-host/key/password/MFA fixtures; Windows native ConPTY is a distinct case. Harness records saved/restored terminal mode, process/PTY handles and byte/resize/exit observations; no real password recording. Direct baseline command shape:

~~~text
ssh -F FIXTURE_CONFIG FIXTURE_ALIAS
~~~

**Actions:** log in with normal and changed host keys, key/agent and interactive auth; enter synthetic ASCII/Chinese/IME text; exercise Ctrl+C, Ctrl+D, Unix job control where applicable, resize, bracketed paste and a full-screen editor. Compare direct ssh control-key/escape behavior. Trigger prefix/action, cancel, timeout, literal prefix, unknown action, disable and rebinding across split reads and host key interception. Exercise normal exit, Client interruption and forced child failure. On Windows, run creation/read/write/resize/close under active output and blocked-reader cases.

Feed synthetic split OSC 52/5522, raw C1 forms and UTF-8 continuation bytes, malformed/oversized/unterminated strings into the managed display harness. **Pass:** no local clipboard read/write is granted by remote output, blocked sequences never leak a trailing control fragment, allowed unrelated output remains ordered, parser failure detaches display without killing ssh. Do not infer authenticated/Shell-ready from a prompt. All terminal fidelity outcomes and recovery instructions are per host; no universal sanitization claim.

**Pass:** native baseline/managed differences are explained, modes restore on normal/catchable exits, uncatchable kill has documented user recovery instructions, OpenSSH host checks are never bypassed and actual prefix delivery is proven. **Failure/cleanup:** restore saved terminal state through the native adapter; for a disposable Unix terminal only, document the applicable native reset/stty recovery after verifying the context. Close owned fixture sessions; preserve no keystroke log. This selects PTY adapter feasibility, not full screen restoration.

## E3: single image snapshot and bounded encoding

**Inputs/harness:** synthetic transparent/opaque/color/Unicode-named images, empty/text-only/corrupt/oversized selections, real Windows/macOS/X11/Wayland adapters separately. Harness accepts explicit one-shot capture and returns metadata, byte length/hash, alpha/color assertions, allocation/budget counters and owned blob lifetime; platform clipboard priming is a deliberate manual/fixture action, never an implicit read of real contents.

**Actions:** trigger on Client A's desktop while B exists; change clipboard after capture and show raw blob immutability. Test missing/denied desktop, X11 owner exit, Wayland protocol/feature combinations, headless login and encoding timeouts. Stream malformed layout/length/overflow metadata to Core before plugin sees it. Normalize to the PNG profile and test alpha, forbidden chunks, excessive decompression, mismatched dimensions, invalid CRC, declared-size mismatch and expiry. Try forged/cross-operation/released blob handles and chunk overlaps.

**Pass:** Core bounds/validates raw data before exposing a handle, plugin sees exactly one snapshot, encoded data obeys profile/budget, no implicit file/text fallback, final local digest matches exact sealed/uploaded bytes. If a native API allocates before dimensions can be checked, record that limitation and require a bounded adapter or mark unsupported. **Failure/cleanup:** release only owned blobs/temp files, clear the synthetic selection only if still owned by this fixture; do not restore/inspect a real prior clipboard. File-selection fallback gets its own explicit user action and test.

## E4: one SFTP backend, reuse, target binding and paths

**Inputs/harness:** candidates A/B from ADR 3; controlled configs with -F/-p/-P/-i/-J/ProxyCommand/-o, Include and Match variants; native client/server builds; all remote negative fixtures listed above. Configuration containing Match exec/helpers is executable behavior: use a trusted synthetic config whose helper effects are owned and expected. Do not run ssh -G against unknown user config or call it proof of target equality.

Standard command shapes (replace placeholders with owned fixture values; the batch is SFTP command language, not a Shell script):

~~~text
ssh -F FIXTURE_CONFIG -G FIXTURE_ALIAS
sftp -F FIXTURE_CONFIG -P FIXTURE_PORT -oBatchMode=yes -b FIXTURE_BATCH FIXTURE_ALIAS
ssh -F FIXTURE_CONFIG -T -s FIXTURE_ALIAS sftp
~~~

The subsystem command's stdin/stdout must be piped to the future protocol harness, never printed to an interactive terminal as binary output. A sample batch for an intentionally disposable directory uses pwd, put LOCAL_SYNTHETIC OWNED_TEMP, ls -l OWNED_TEMP and rm OWNED_TEMP; choose real quoted operands through the candidate's tested command-language encoder. This baseline is not proof of exclusive creation/no-clobber semantics. Do not use a production directory to find out.

**Actions:** separately measure config, identity and actual network/auth reuse; compare direct/multiplexed/auxiliary cases without claiming common hostname means common connection. Validate that auxiliary options do not replay RemoteCommand/forwarding/RequestTTY. Change included config/route/helper state between capture and spawn; test unsupported dynamic config refusal and verified live-master or frozen-config binding. Observe actual target/host/user evidence without bypassing host checks. Test missing SFTP and additional auth with no main-terminal prompt theft.

Probe special names: space, Chinese, quotes, dollar, backtick, semicolon, leading hyphen, newline and Windows drive forms. Transfer may support a path that insertion rejects; no case executes it as a Shell command. Confirm SFTP root versus Shell cwd, chroot mapping, Windows namespace and unknown mapping. Force duplicate-name collision and a concurrent destination creator immediately before commit; existing bytes must remain unchanged. Inspect exclusive temporary creation, symlink/root swap, denied ACL, quota/disk full, partial transfer, cancel before/after commit dispatch and lost acknowledgements. Use synthetic readback SHA-256 when claiming content verification; otherwise label size-only/none.

**Pass:** select **one** backend only after mandatory target binding/non-overwrite/limits/cancel invariants pass on its advertised fixture set. No safe commit gives COMMIT_UNSUPPORTED, not success. No reused/auxiliary auth gives AUTH_REQUIRED; no hidden second SSH stack, scp fallback or two complete backends. Record atomicity, durability and hash evidence separately. **Failure/cleanup:** cancel only owned auxiliary child/channel; retain indeterminate final status and reconcile known paths before user-authorized cleanup, never delete an uncertain unrelated final file. No candidate passes: record a Phase 1 blocker and the precise unsupported set/alternative, rather than arbitrarily selecting one.

## E5: process survival, screen and recovery separately

**Inputs/harness:** native terminal/PTY adapter and controlled remote job that emits synthetic output; bounded VT and restricted redraw probes, normal/alternate screens, editor, resize fixtures. Harness reports child liveness separately from screen/cursor/mode comparison and remote-job identity. No terminal byte-log replay as the solution.

**Actions:** explicit detach; close/kill Client; reattach after normal screen and after alternate-screen/Vim plus resize. Saturate output while detached/slow, hit state budget and verify safe incomplete display. Reattach across mode changes and ensure clipboard/notification side effects do not replay. Separately disconnect the owned network channel, crash only owned Core, and reboot only an authorized disposable test VM if such authority/environment exists; otherwise mark reboot not-run. Reconnection creates a new generation; metadata alone cannot resurrect a process.

**Pass:** each of three claims (Core/PTY/ssh survival, visual restoration, remote recovery) has independent observed outcome. Compare candidate scope/cost and select one restoration strategy with supported modes listed; restricted redraw must disclose full-screen gaps, which block full Phase 2 attach acceptance until resolved. Remote tmux/Agent recovery is optional and separate; do not pretend local recovery establishes it. **Failure/cleanup:** stop only known remote synthetic jobs and owned local handles, reset test terminal through native saved-state restoration, preserve result limitations.

## E6: public protocol, packages and trust

**Inputs/harness:** standalone non-Rust minimal plugin fixture using no Core-private code, faulty plugin variants, synthetic archives, bounded IPC peers, slow subscribers. Feed bytes/objects with explicit expected outcome and instrument terminal-progress independence plus queue/resource counters.

**Actions:** UTF-8 multi-byte frame length, split headers/body, duplicate headers/keys, bare LF, oversize/depth/batch/unknown method, monotone request counter replay/out-of-order and independent response order. Test handshake incompatible/timeout, logs on stdout, blocked stderr/RPC, crash/restart ceiling, package traversal/link/device/decompression and wrong platform/entrypoint. Try self-reported identity, Client role upgrade, other Session/operation/blob, expired/revoked/composed grants. Churn exited Sessions, disconnected Clients, detached Attachments, refreshed/revoked grants and reissued/consumed stream tickets without advancing their retention deadlines: total record/metadata caps must reject admission before allocation, preserve required dedup records and release unreferenced records. Flood subscriptions and verify gap plus requery; disable/uninstall all plugins and keep basic login/list/close usable. Remote synthetic OSC/hook/URL cannot grant clipboard, upload or forward capabilities.

**Pass:** exact documented errors/connection closure under finite budgets, no duplicate request-ID tracking growth, no credential/env/terminal payload leak, no plugin failure stalls SSH, untrusted packages do not execute install hooks. Claim failure isolation only, never OS sandboxing. **Failure/cleanup:** stop only owned fixture plugin/IPC children, release their grants/handles, preserve SSH and redacted assertion traces. Native other-user/pipe/logon tests need actual accounts/OS and stay blocked when absent.

## E7: deterministic Image Paste races and independent effects

**Inputs/harness:** E1/E3/E4 feasible probes, two same-host and two different-host Sessions/Clients, event/effect ledger, deterministic barriers from the harness contract. Input cases include (B, intent digest, context revision, schedule); output proves actual target/path/write and cleanup, not merely expected enum values.

**Actions:** run the full [transaction](image-paste.md); concurrently upload; switch A→B, detach, close, disconnect/reconnect, expire/revoke grants, disable/restart plugin and cancel at capture, stream/open, pre-commit, dispatched-commit, post-upload, pre-insert, dispatched-write and response-loss barriers. Assert both file bytes and the actual PTY target. Feed unknown/mismatched mapping, nested target and unsupported/open-quote Shell context; plain/Markdown/POSIX/PowerShell/cmd encoding are separate test tables, with unsupported encoders inactive.

Replay trigger_nonce/upload_key/insert_key with identical payload and with conflicting stable intent. Advance the retained-record clock beyond expiry on a still-live Client, resend an old trigger_nonce and require EXPIRED with no new Operation/file; test skipped counters, failed admission without counter advancement and exhaustion. Reconnect with a fresh client_id and verify the Client does not auto-resubmit an earlier action under its new counter. For a zero-byte refused insertion, refresh context/confirmation revision on the **same** key and upload; ensure no second file and at most one admitted insertion. Attempt arbitrary plugin text despite safe characters: it must fail because it is not Core-validated formatting of the recorded result. Partial/unknown write and Core crash prohibit automatic reinsertion; dispatched publication may commit after cancel and must yield confirmed/indeterminate retained status rather than a rollback claim.

**Pass:** no data/input retargeting, no overwrite, no hidden upload on insertion retry, no second snapshot, committed files survive failed insertion, exact same-operation ownership cleanup, and honest partial/indeterminate outcomes. Unknown context yields retained result/copy. **Failure/cleanup:** freeze evidence, stop affected fixture operations; reconcile only operation-owned paths. Do not “fix” the test by weakening generation/attachment checks or accepting duplicate input.

## Source XII fault coverage

| Group | Required fault | Named experiment / later completion |
| --- | --- | --- |
| XII.1 | Native ssh unaffected; all plugins removed | E1 passthrough/install boundary; E6 disable-all; Phase 1 actual packaging/login |
| XII.2 | User/port/key/jump/config/host-key checks | E2 auth/changed-key; E4 option/config binding |
| XII.3 | Bytes/Unicode/control/resize/full-screen/auth/exit | E1 binary stdio; E2 native terminal; E5 screen |
| XII.4 | Same/different host concurrent sessions/uploads; A→B | E1 writer; E7 switch/concurrent effect traces |
| XII.5 | Detach/network/close/generation/disable/revoke | E5 recovery; E7 barriers and no stale delivery |
| XII.6 | Empty/corrupt/large/denied/no-SFTP/collision/full/partial/cancel | E3 snapshot/PNG; E4 negative transfer; E7 cleanup |
| XII.7 | Special/control/Windows/namespace paths, injection | E4 path set; E7 Shell/Markdown encoders and arbitrary-text denial |
| XII.8 | Upload versus insert retry and dedup | E7 same-key/revision/partial/response-loss cases |
| XII.9 | Malformed/huge/mismatch/block/crash/spoof/slow plugin | E6 protocol/package/failure tests |
| XII.10 | Local-user takeover; remote cannot grant local effects | E1 native IPC; E2/E6 remote control input, scoped capabilities |
| XII.11 | Unknown history; disable/retention/delete | E6 reserved-method refusal and no fake history now; **Phase 3** structured-hook opt-in/filter/pause/delete acceptance |
| XII.12 | Detach behavior, screen, Core/network recovery distinct | E5 independent observations; E7 generation invalidation; **Phase 2** full switching/attach mode acceptance |

## Phase exits and follow-up dependencies

| Phase | Entry and independent exit evidence | Outside that phase |
| --- | --- | --- |
| 0 feasibility | Approved design; primary native/fixture authorization. Exit: per-experiment methods/results/gaps, selected single transfer route and visual strategy, viable Client/Core/PTY/trust/prefix/image slice or precise blocker | No all-platform support claim or full product skeleton |
| 1 C/S + independent Image Paste | Accepted cooperative-scope contract, actual runtime publication/target/auth gates above and normal task approval; historical Phase 0 choices alone are insufficient. Exit: managed multi-session login/list/close, authenticated IPC, public versioned plugin lifecycle/install/grants/events, PNG→verified cooperative upload→conditional insert/retained result, plugin failure independence on primary platform; other builds/runtime gaps recorded | Full TUI, all-platform completion and advanced plugins |
| 2 TUI/common plugins | Phase 1 stable mechanisms, selected restoration scope. Exit: switch/detach/attach/epoch/screen/full-screen tests, File Transfer traversal/symlink/download/overwrite/retry policy, Core-owned Port Forward lifecycle/loopback/listening/reachability and licensed offline Command Reference; representative native client/remote combinations | Persistent remote hooks/history by default |
| 3 structured remote integration | Exact supported Shell/editor contracts and installation authority. Exit: sourced fresh cwd/command events, default-off History with filter-before-persist/exclude/pause/retention/delete/unknown fields; queries never auto-execute; Clipboard Bridge separate read/write grants; dedicated user-visible auxiliary auth; nested mapping expansion | Guessing command text/exit from terminal bytes or silent permanent hooks |
| 4 releases/ecosystem | Supported runtime matrix and package provenance. Exit: independent plugin updates/rollback/integrity/publisher trust, protocol SDK compatibility, release naming review, platform packages/notices. Untrusted execution requires proven OS sandbox first | Marketplace/WASM/cloud/GUI/AI remain optional |

These are proposed dependent work items, not created child tasks or serial authorization. Documentation completion does not imply any experiment passed. A follow-up task must contain the actual selected environment, controlled-fixture authority, runnable probe interface and required evidence before it starts.
