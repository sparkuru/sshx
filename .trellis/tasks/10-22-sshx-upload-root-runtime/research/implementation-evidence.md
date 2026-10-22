# Implementation and runtime evidence

Observed 2026-10-22–23 (Asia/Singapore); final handoff runs completed October 23. This is probe evidence, not full v1 or release qualification. Main owns task lifecycle, final fixture teardown, independent review and human disposition.

## Loading and scope

The implement worker received the native `trellis-hook-injected` marker plus saved output `/tmp/hook_outputs/01a16436-bd24-7580-a842-96e32fd2cefa/ccb533d5-f127-4d3b-8429-76021edb17ac.txt`. That output retained omission markers and the tool rendering also truncated the large combined read. The worker recovered inputs with direct reads rather than treating the hook marker as complete context: implement.jsonl, prd.md, design.md, implement.md; all eight trellis-plus policy/source files (index, development-principles, sshx-contracts, development, validation, continuity, commit-policy, sshx-design.txt); both shared thinking guides; mainline; protocol (entire original 236 lines in two bounded chunks), ADR 3, Image Paste, Phase 0, platform matrix, planning-evidence and archived dual-target-fixture. Architecture was read before its edits. Main's review-focus was also read. This is observed task-specific loading, not automatic future-task discovery.

Applied common and trellis-before-dev, including package discovery and guides index. Read common's workspace/engineering/execution/agent-security references. code-python was read for inline source-edit scripts; no Python or shell source file was changed. The worker did not spawn agents, install dependencies, modify runtime/templates/lockfile/notices, commit or archive.

The smallest behavior gap was existing Core rename publication and boolean cleanup ownership. Changes stay in six Rust probe files: core.rs, sftp.rs, main.rs, suite.rs, new upload.rs and suite/upload_cases.rs; plus six current design documents. Core continues to own effects and grants; independent plugin/image crates remain separate and compile unchanged. Existing historical backend-comparison and rename-reply-loss cases retain rename and remain explicitly historical comparison evidence.

## Implemented boundary

- Explicit fixture-owner `core --qualify-upload` runs synthetic preparation before accepting requests. It verifies actual extension value 1, link/readback and occupied-name preservation, then enables process-local policy for the actual fixture/config/namespace/root. Ordinary startup has no policy and does no qualification. Action invocation cannot qualify or self-grant. Policy invalidation hooks can only disable/invalidate, not manufacture a qualified scope.
- Operation scope digest is captured with B and grants refer to that operation. Common authority precedes policy admission; upload and fresh/refused insertion recheck current scope. Request key/digest prevents republishing after success, collision and unknown outcome.
- Closed temp is published through raw hardlink, with no rename/scp/remote-command fallback. Dispatch and definitive rejection remain separate facts. Generic SFTP FAILURE stays IO_ERROR; only a directly observed occupied name is NAME_EXISTS and explicit unsupported is COMMIT_UNSUPPORTED.
- Acknowledged publication immediately records confirmed final, independently of readback and temp cleanup. Lost matching STATUS response records indeterminate, no replay/input/cleanup. The stream matcher reconstructs fragmented/coalesced packets and matches extension plus request ID, rather than assuming write boundaries are packets.
- Cleanup uses acknowledged creation, captured access/type/size observations, boot/namespace/operation binding and original live transport. Each removal has fresh binding checks. Cancellation/revocation permit narrow cleanup, while generation/config/scope/transport/ownership uncertainty blocks it. Final names are never cleanup targets. Metadata is only an observation under cooperation, not inode/ABA authority.

## Environment and commands

[Fixture identity ledger](fixtures.json) owns exact fresh container/run/port/source identity. Primary state `.devhome/phase0/b590f2c6767da5384787f0c063feaa79/state.json`; secondary `.devhome/phase0/secondary/93f2db10abe08010513e2956a7ac4369/state.json`. Both are generated loopback-only OpenSSH `1:9.2p1-2+deb12u10`, separate overlayfs roots and distinct `/quota` tmpfs devices. hako's historical task label is not sole ownership; current run IDs and main's ledger establish ownership. Main retains services for independent check and exact teardown. Pending resources/final data inside those disposable fixtures are intentional evidence, not a claim Core cleaned them.

Commands actually executed from repository root, with scoped sandbox escalation for Docker/native loopback execution:

~~~sh
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --offline --workspace --all-targets -- -D warnings
./hako cargo check --locked --offline --workspace --all-targets
./hako cargo test --locked --offline --workspace --all-targets
./hako cargo build --locked --offline --workspace
.devhome/target/debug/phase0-probe upload-suite --fixture .devhome/phase0/b590f2c6767da5384787f0c063feaa79/state.json --fixture-secondary .devhome/phase0/secondary/93f2db10abe08010513e2956a7ac4369/state.json --output NEW_JSON
.devhome/target/debug/phase0-probe suite --headless --fixture .devhome/phase0/b590f2c6767da5384787f0c063feaa79/state.json --fixture-secondary .devhome/phase0/secondary/93f2db10abe08010513e2956a7ac4369/state.json --output NEW_JSON
~~~

Pinned network-none build image is `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1`, locked russh-sftp 3.0.1; no host toolchain or new crate. Sixteen unit tests pass (image 5, harness 6, protocol 5); added tests exercise component containment and every split position/coalescing of hardlink request/reply framing. No display/clipboard access occurred. Headless mode uses a generated PNG in the transfer/insert case and labels the desktop-capture/switch case excluded, never passed.

## Iteration records and failures

- [upload-run-1.json](upload-run-1.json): first 13 top-level groups pass; later review strengthened cleanup guard evidence and added cases, so this is not final source evidence.
- [core-run-1.json](core-run-1.json): first headless Core run, 43 passed and 1 desktop case excluded. Inherited metadata was subsequently corrected: its desktop wording/supplemental relative paths are historical presentation defects, not evidence desktop ran.
- [upload-final.json](upload-final.json): 12 groups pass, C06 setup fails with generic `Failure: Failure` while attempting baseline rename of the image-created upload root. No EXDEV assertion is inferred. Root remained available and subsequent cases passed. The test was replaced with a newly created operation-directory symlink, avoiding an unsupported fixture setup assumption; no acceptance requirement was removed.
- [upload-verified.json](upload-verified.json): corrected C06 plus all 13 groups pass. Subsequent review added independent cleanup-parent observation and post-commit scope/insertion regression, so final evidence must use the later report.
- [core-verified.json](core-verified.json): 42 pass, 1 failure, 1 excluded. Same-target concurrent uploads timed out waiting at during_stream. Preserved for diagnosis; later report must establish disposition rather than hide this failed run.

## Final handoff and failure disposition

[upload-handoff.json](upload-handoff.json) completed exit 0 with all 13 groups passing. [core-handoff.json](core-handoff.json) completed exit 0 with 43 cases passing and one desktop case explicitly excluded. These final reports supersede earlier checkpoints for current-source claims. Both use the same fresh fixture pair; no source changes followed their build. C13's six Rust source hashes and Cargo.lock hash were independently compared with current files and all seven match.

The prior concurrent-upload timeout exposed a concrete barrier ownership race: the handler advertised before_open before worker entry, and the worker could overwrite a client's already-issued release with the same barrier label. The handler now advertises starting; only the worker publishes before_open. The old failed run had no last-state projection, so attributing that historical timeout specifically to this race remains an inference. Timeout diagnostics now include the last projection. Final focused evidence includes 12 fast initial-release iterations through normal cancellation/cleanup, and the final full suite passes same-target concurrent uploads. This is a synchronization correction, not a timeout increase.

All five listed Cargo commands were rerun after final source changes and exited 0; logs are `/tmp/sshx-upload-fmt-handoff.log`, `/tmp/sshx-upload-clippy-handoff.log`, `/tmp/sshx-upload-check-handoff.log`, `/tmp/sshx-upload-test-handoff.log` and `/tmp/sshx-upload-build-handoff.log`. These temporary logs are session-local; committed runtime reports and source fingerprints provide durable case evidence. Final insertion regressions cover all four postcommit scope changes (absent, root, namespace, policy), with exact unchanged native PTY bytes and independently retained final hashes. Cleanup-parent replacement occurs after confirmed publication and therefore exercises the independent cleanup guard.

The 186-entry preimplementation tracked baseline was checked again: differences are exactly six current documents, four existing Rust files, and main-owned mainline/development/sshx-contracts updates. The two new Rust files and task research are additions. Archived evidence, protected runtime/template files, dependency lockfile and notices retain baseline fingerprints. The implementation worker leaves both exact fixtures running for main's independent checks and teardown. Neither passing report changes phase0_ready or asserts general-root qualification.

## Acceptance map and evidence limits

| Cases | Evidence and boundary |
| --- | --- |
| C01 | Actual v1/link/hash/collision qualification on both targets; default absent-policy startup performs no preparation; absent qualification rejects before operation creation |
| C02 | Real Core hardlink, final exact hash, temp removed and final directory retained; no PTY input |
| C03 | Existing regular/symlink and creator at before-link barrier preserve bytes; generic concurrent FAILURE remains IO_ERROR; retries admit no new steps |
| C04 | Missing/version/unsupported-result injection explicitly labeled; separate direct-wrapper cross-device experiment observes real failure and absent destination. Core only publishes within one operation directory and does not claim runtime EXDEV classification |
| C05 | Absent/changed policy/root/namespace, stronger requirement, wrong Client instance, revoked grant and malformed request; isolated actual config copy changes and is rejected. Client fixture ownership is not complete product plugin conformance |
| C06 | Precreation operation-directory symlink remains untouched; publication parent replacement blocks link and leaves original temp/replacement. Component-prefix/traversal unit coverage; no hostile ABA guarantee |
| C07 | Cancel and revoke before mkdir/open/write/close/link and during stream; actual files plus admission counters and unchanged PTY. Existing full Core regressions cover expiry/disable/detach/generation and postcommit transitions |
| C08 | Real mode change plus independently denied REMOVE preserves final/temp; Core stops at observed mode change. Separate labeled REMOVE-result injection exercises its error propagation with unchanged observations; final remains confirmed/pending |
| C09 | Actual hardlink executes; only matching reply is intercepted. Independent read-only hashes do not promote indeterminate operation; same-key retry and attempted insertion cause no effects |
| C10 | Missing ledger removes actual evidence but retains acknowledged-created fact; stale boot mutates ledger identity; transport is actually closed. Real temp replacement, parent replacement at cleanup, and session generation change exercise normal guards. No name-based adoption |
| C11 | Same-key success/conflict and mapping refusal preserve final. Postcommit policy/root/namespace/scope changes deny insertion with exact unchanged native PTY output. Full Core covers refreshed context and partial/lost input replies |
| C12 | Two real generated targets, no cross-target files/input; basic login works with upload policy disabled. Full Core covers simultaneous targets and switching during real streams |
| C13 | Source hashes and exact fixture identity are recorded. Case success is bookkeeping only; independent check, preserved historical fingerprints and final teardown are separate acceptance evidence |

Probe errors INVALID_PARAMS and CAPABILITY_DENIED map to proposed -32602 and capability denial; experimental probe RPC/result shapes remain narrower than full v1. No API compatibility/release guarantee follows. C13 reports deliberately keep acceptance_complete=false while independent review/teardown are pending. Actual security and deletion changes require main's human-required disposition before commit/completion/archive.
