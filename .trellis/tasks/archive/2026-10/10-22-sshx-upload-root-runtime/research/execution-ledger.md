# Execution ledger

## Approval and branch

On 2026-10-22 the user approved the final runtime-verification plan with “批准”. Supported task activation changed planning to in_progress. Feature branch `upload-root-runtime` was created from `21e62e5` after the initial sandbox read-only .git failure; a scoped elevated git switch succeeded. Task metadata matches the actual branch. No commit, merge or archive has occurred.

Before worker edits, main recorded 186 tracked-file SHA-256 values in baseline.json. Existing task-owned planning/mainline edits are preserved. Native trellis-implement was dispatched with explicit ownership, complete-context fallback and the approved disposable fixture boundary; independent check remains pending.

## Main preflight

Read-only `docker image inspect --format '{{.Id}} {{.Os}}/{{.Architecture}}' sshx-phase0-build:rust1.90 sshx-phase0-fixture:local` failed in the ordinary sandbox because Docker socket access was denied. The same exact inspection succeeded with scoped escalation:

- build: `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1`, linux/amd64;
- fixture: `sha256:cb3eb061776d0835b474754593e692492c473a811607dbab68a815b02cedc70c`, linux/amd64.

These match the historical image identities. This is availability evidence only, not a new runtime PASS. Main started no fixture during this preflight. Exact fresh run/container identities and teardown must be recorded by the execution owner before closure.

## Fresh fixture ownership

Main subsequently provisioned both fixtures at the implement worker's request. `./dev.sh start` used the absent primary pointer; an isolated `python3 -I -S` invocation of the reviewed lifecycle module created the secondary under its own private pointer after asserting it absent. Both image builds were cached, both strict generated-host-key authenticated readiness checks passed, and no existing service was replaced.

`fixtures.json` records exact state paths, current run/container identities, host-key fingerprints, image IDs, labels, filesystem/device metadata and mode 0600 state files. Primary is loopback port 32781, secondary 32782. Both use OpenSSH package 1:9.2p1-2+deb12u10; operation roots are overlayfs and the existing /quota mount is a distinct tmpfs. These fixture metadata reads use exact-container Docker inspect/exec only. No password or key body is recorded.

The wrapper's static sshx.task label still names the original Phase 0 infrastructure task. Current ownership is established by the fresh run/container/pointer ledger plus matching repository/run/scope labels, not the historical task label alone. Main owns final teardown, after worker and independent check no longer need the services. They are currently retained for testing; cleanup is pending.

Main also ran `./hako rustc --version` using the existing no-network, UID-mapped build wrapper: Rust 1.90.0 (1159e78c4 2025-09-14). No host installation or dependency update was performed.

## Review corrections before independent verification

Read-only checker preparation loaded all 19 registered check inputs plus task plans, recovering truncated/omitted native injection content by direct reads. Provisional findings were verified by main and sent to implementation: retain hardlink dispatch history separately from definitive rejection; test real ledger/boot/transport facts rather than fault-name refusal shortcuts; cover both pre-unlink refusal and the actual remove-error branch; distinguish current headless evidence from historical desktop/supplemental observations.

The first focused report and first headless Core report are retained checkpoints, not final acceptance. The later `upload-final.json` filename also does not imply success: its C06 fixture setup failed when renaming an overlayfs image-created root; the failure is preserved and must be superseded by a separately named corrected run. Main has not changed any machine report to remove failures.

## Final native verification and teardown

On 2026-10-23 implementation handoff and independent runs both passed: 13 focused groups and 43 Core cases with one desktop case explicitly excluded. The independent worker also reran all five Cargo checks and 16 unit tests. See check-report.md, research/check-upload.json and research/check-core.json for actual source-bound evidence and limits.

After both workers released the services, main executed exact lifecycle teardown and verified absence, generated credential deletion and both loopback port rebinds. [Cleanup](cleanup.md) and [machine verification](cleanup.json) supersede the earlier retained-fixture checkpoint. No task-owned service remains pending. Git/human-review closure is separate and has not yet been authorized or executed.
