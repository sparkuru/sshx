# Core implementation handoff

2026-10-22. Main requested this handoff and stopped further implementation/execution in this turn. No Git changes were staged or committed. Preserve the checker's and other workers' edits.

## Owned changes and current stage

- `probes/phase0/Cargo.toml`, `Cargo.lock`: real workspace `protocol`, `harness`, `image-probe`; Rust 1.90; locked dependencies. Harness uses system OpenSSH, `russh-sftp =3.0.1` raw session, Tokio, libc, serde/JSON, SHA-256, PNG 0.18.1, crc32fast and flate2.
- `protocol/`: strict bounded Content-Length/JSON decoder (duplicates/depth/UTF-8/header limits), monotone request counter, binary record codec, four passing unit tests.
- `harness/src/{main,core,native,sftp,seal,suite}.rs`: actual separate Core and child Clients over SO_PEERCRED-checked private Unix IPC; Core-owned OpenSSH/PTys; writer epochs, immutable operation binding, explicit one-operation composed grant, bounded retained records/tickets; actual upload and input effects; deterministic open/commit/mid-stream hooks; independent Core PNG seal validation including streaming deflate-surplus rejection; standalone focused case/plugin commands.
- `harness/src/plugin.rs` belongs to fixture worker/checker, not this worker. Visual crate/fixtures/wrappers also have separate owners. Do not overwrite them.
- `research/core-context.md` records full context fallback reads. `core-results.json` and `core-evidence.md` have NOT yet been finalized.

## Last actual verification

All commands below ran from repository root through pinned isolated `hako`; native suites needed scoped escalation because the ordinary sandbox denied Unix socket creation/loopback access.

- `./hako cargo clippy --locked -p sshx-phase0-probe -p sshx-phase0-protocol --all-targets -- -D warnings`: PASS after native generation/post-commit/Core-crash additions, before the latest uncompiled edits listed below.
- `./hako cargo test --locked -p sshx-phase0-probe -p sshx-phase0-protocol --all-targets`: PASS six tests (two native prefix/OSC + four framing/record tests), before latest edits/new seal regression.
- Scoped `rustfmt --edition 2021 --config skip_children=true ...`: PASS on worker-owned files before the latest small patches; final formatting is still needed.
- `./hako cargo build --locked -p sshx-phase0-probe`: last PASS after adding `routes_and_reuse`; no compilation has run after the deadline/mid-stream/repeated-ESC patches.
- Native suite report `/tmp/sshx-phase0-core-native3.json`: 32 executed cases PASS, zero assertion failures, five stale/unresolved gap rows. Exit 1 explicitly reports required unverified subset; NOT Phase 0 PASS. Earlier `/tmp/sshx-phase0-core-native1.json` records two fixed failures; native2 records 27 PASS, zero failures. `/tmp/sshx-phase0-core-first.json` is the ordinary-sandbox failure snapshot, not native success.
- Focused SFTP `/tmp/sshx-phase0-sftp-focus2.json`: PASS (exclusive open, baseline no-replace rename, nine special names, symlink collision, denied root, bounded tmpfs disk full, no-SFTP/additional-auth refusal, system-sftp overwrite comparison).
- Focused `/tmp/sshx-phase0-ack-focus1.json`: PASS. Actual owned SFTP pipe adapter discards the received rename reply then aborts channel; fresh read-only known-path reconciliation observes final hash and absent temp, with no reupload. This is explicit pipe fault injection, not host network disruption.
- Focused `/tmp/sshx-phase0-routes-focus1.json`: PASS. Native ProxyJump and ProxyCommand on owned generated config, real ControlMaster SSH and SFTP reuse with fresh authentication disabled, chroot namespace/write/readback separation. Core still refuses dynamic helper profiles; native route success is not Core binding support.
- Image flow in native3: actual Core B created before fresh private-display capture; `phase0-image suite --binding-file ...` passes that exact B into the independent plugin; Core compares artifact binding and independently validates exact sealed PNG; SFTP readback and initiating PTY insertion assertions pass. `capture_after_trigger` still needs explicit child-status success assertion (checker finding).

## Uncompiled latest edits

1. Added `Operation.deadline`, `Grant.deadline`, `Ticket.deadline` and effect/consume monotonic checks in `core.rs`. Verify every initializer and cleanup path compiles. Existing `probe.expire` and `probe.forget` are explicit test-clock/retention hooks, not elapsed-hour evidence.
2. Added `hold_mid_stream` upload flag/function argument. Core upload opens exclusive temp, writes one actual byte, waits `during_stream`, then continues or cleans known owned partial on revocation. Added `revoke-during-real-stream` suite case and same-key upload/conflicting-payload checks. This has NOT compiled or run yet.
3. Added Core seal regression `rejects_deflate_surplus_before_raw_output` in `seal.rs`; NOT run yet.
4. Fixed repeated ESC handling in `OscGate` (retain escape state, drop repeated ESC rather than leaking it) and added repeated-ESC/leading-zero OSC cases; NOT compiled/run yet. Verify the replacement landed in the intended branch.

## Required next correctness work

1. Compile latest saved edits first, fix errors, then run focused tests before further changes.
2. Implement the main's specified minimal two-profile support: startup accepts optional trusted secondary manifest; each Session owns its frozen Fixture/config digest/profile id; operation B and upload resolve that original Session's profile. Current Core has ONE global fixture/config digest. No part of the two-profile refactor has been started: it exists only as a plan, so there is no half-migrated schema to preserve.
3. Checker finding: `upload()` currently checks before connection/at barriers and before rename, but not before every new mkdir/open/write/close step. Add serialized per-step admission, keep streaming outside State mutex, and test actual files at deterministic boundaries. Never delete temp if exclusive open did not establish ownership. Existing private-opdir assumptions do not establish adversarial same-UID remote root-swap safety; scope/refuse unsupported roots explicitly.
4. Checker finding: actual bounded output/backpressure is not yet demonstrated. PTY ring sets `display_incomplete` and keeps draining, but Core only revokes writer on output query. Large `probe.output` JSON arrays can exceed BODY_MAX before ring capacity. Add bounded metadata/progress observations and a real overflow test; do not claim a production renderer from this test-only output API.
5. Checker finding: State mutex spans blocking PTY write/write_all, child spawn/drop/wait. Raw-cat oracle drains promptly, but blocked receiver could stall control/cancel/status. Use bounded dispatch/nonblocking writes or preserve an explicit unresolved limitation; independent output drain alone is not sufficient.
6. Add `ensure(status.success())` in `capture_after_trigger()` before accepting an artifact. Latest visual suite is now clean, but a previous failed visual suite still yielded an artifact; do not let that become a success-looking fallback.
7. Remove ONLY superseded gap rows with exact supplemental evidence: real cross-UID results are `research/ipc-crossuid.json`/`.md` (UID65534 DAC denial, UID0 DAC-only bypass reaches native SO_PEERCRED rejection; metadata unchanged); package/events/capability E6 is separately exercised in plugin report; chroot/routes now focused PASS. Physical IME/MFA/other native platforms are explicitly deferred. Keep any genuinely unresolved required case non-PASS.
8. Run final two-fixture native suite, scoped fmt, locked clippy/tests/build; copy sanitized actual reports into `research/core-results.json`, write `core-evidence.md` with exact commands, failures/limits and cleanup. Tell main/checker when source is frozen. Main owns overall readiness/docs/lifecycle/Git and final services cleanup.

## Runtime resources and coordination

- Primary manifest: `/home/wkyuu/cargo/repo/29-sshx/.devhome/phase0/09422203e8e5490890c2ddf05c2ff249/state.json`, loopback port 32770.
- Secondary manifest: `/home/wkyuu/cargo/repo/29-sshx/.devhome/phase0/secondary/8a26f072521876cee2e63876fc9bd093/state.json`, port 32771. Separate container/host key/filesystem, same Docker host (not two physical machines). Evidence `research/dual-target-fixture.json`.
- Main/fixture worker explicitly retained both services for this continuation. Do not run normal `dev.sh start` or broad cleanup; main owns exact final teardown for both pointers.
- Native3 report confirms its private Core runtime removed. Per-case owned Core/client/agent/PTy/display handles were stopped; forced Core crash case uses process-local child-subreaper and reaps exact orphan SSH PID. Earlier failed cases may leave synthetic known files only inside the disposable fixture; main's exact container teardown owns their cleanup.
- No native suite/build process is intentionally left running by this worker; latest tool was only a source patch. Fixture and checker may have their own concurrent commands; coordinate rather than killing them.
- `phase0_check` is reviewing stable surfaces and has not edited evolving Core files; send it the freeze signal after the remaining work.
