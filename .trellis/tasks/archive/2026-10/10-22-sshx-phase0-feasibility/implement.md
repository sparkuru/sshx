# Execution plan: Phase 0 Linux feasibility

Status: execution approved on 2026-10-22 by the user’s “可以” in reply to the concrete final scope. Start the task and implement/run the bounded probes and generated-data fixtures. Existing user displays, credentials and unrelated resources remain excluded; Git commit/archive require a later concrete batch approval.

## Ordered steps

- [x] Read prior architecture/source contracts and inventory actual OS/tools/cached image metadata without execution. Capture findings and baseline fingerprints.
- [x] Converge PRD, design and execution scope; curate both role manifests. Treat this as one integrated cross-layer feasibility checkpoint.
- [x] Present final scope/fixture authority/acceptance summary and obtain explicit approval; activate only afterward.
- [x] Dispatch `trellis-implement` with current task path, exact file ownership and role self-exemption. Require all role JSONL inputs plus PRD/design/implement direct reads whenever native injection omits/truncates them. Worker preserves other edits, does no lifecycle/mainline/Git operations and reports observed context paths.
- [x] Establish actual Rust manifests/help and narrowly scoped Docker wrapper; verify image architecture/compiler/MSRV, dependency primary interfaces/features/build scripts and lockfile. Apply language skills before Shell/Python edits. Do not derive a dev command from an imaginary product service.
- [x] Establish exact fixture start/status/down, generated identities/known-host expectations, loopback mappings, private Xvfb/agent and owned-resource ledger. Smoke-test teardown/readiness before running dependent cases.
- [x] Implement and run E1/E2 native IPC/PTY/SSH key/owned-agent/synthetic-password/binary/prefix cases and E6 independent public-protocol/limit/failure cases first. Generated password material stays private/ephemeral and out of argv/logs. Shared state tests alone are insufficient.
- [x] Run E3 synthetic private-X11/raw/PNG bounds and E4 minimal backend comparison, including no-clobber/target/path/auth negative cases. Select at most one viable transfer route, with exact evidence or a precise blocker.
- [x] Run E5 restoration comparison and separate survival/Core/network cases. Keep unsupported reflow/terminal modes and physical platforms explicit.
- [x] Run E7 barrier-driven actual transfer/input races, including expired trigger counters, metadata churn, refreshed zero-byte insertion and partial/unknown effect cases. Produce a complete case/source matrix and cleanup report.
- [x] Main session inspects evidence/scope, registers real research and synchronizes actual environment/command/contract decisions into project-owned specs/mainline. Return product/authority conflicts to planning.
- [x] Dispatch `trellis-check` for full-scope independent code/spec/evidence review and fixes; require complete context loading and appropriate unit/integration/native checks, not implementation-shaped assertions.
- [x] Resolve findings and rerun affected checks, evaluate actual human-review gate and any required residual user signal, then prepare a concrete Git/archive proposal. No commit/archive or next phase without its authority.

Mandatory additional direct reads: `docs/phase-0.md`, `docs/platform-matrix.md` and all six `docs/adr/*.md` are existing result-target/decision files, so read them directly rather than registering outputs as injected inputs. Also load the actual Shell/Python skill before those languages are written. Initial role manifests had 14 inputs each; final evidence registration brings each to 23. `task.py validate` warns that `docs/protocol.md` is 46,819 bytes, over the 32,768-byte per-file injection limit. A saved hook payload cannot recover omitted bytes: explicitly read that whole protocol in bounded segments and every named omitted planning/input file before work, and report completion. Do not change protected injection settings to avoid the fallback.

## Intended command contract

These were the **planned interfaces at approval**. Implementation and smoke checks established the actual commands recorded in the [runbook](../../../../../docs/phase-0.md) and evidence; retain this block as the original execution plan.

~~~text
./hako rustc --version
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --workspace --all-targets -- -D warnings
./hako cargo test --locked --workspace --all-targets
./hako cargo build --locked --workspace
./dev.sh start
./dev.sh status
.devhome/target/debug/phase0-probe --help
.devhome/target/debug/phase0-probe suite --fixture OWNED_STATE --output OWNED_RESULTS
./dev.sh down
~~~

The wrapper's workdir is the actual `probes/phase0` workspace and its target/cache are explicitly mapped; ordinary commands publish no ports. Native suite commands use the recorded generated fixture state, never user SSH configuration/desktop. Check Rust `--locked` builds only after the actual lockfile exists. Confirm and record fmt/clippy components rather than installing a host toolchain.

For Shell/Python, run the relevant skill's syntax/style checks against actual changed files; ShellCheck/shfmt are present. Python fixture protocol uses stdlib where practical; add a dependency only for a concrete missing requirement, with explicit source/version evidence. Tests protect observable boundaries and failure cases, not helper implementation shape.

## Evidence and review gates

- Every case records runbook fields and exact environment/tool/library/image/server/auth/display combination. No fabricated pass or `unknown` replaced by prompt parsing.
- Check the actual native process/file/stream effects at barriers, resource allocation/retention budgets, expected host-key refusal, no-overwrite primitives and independent plugin failure behavior.
- Keep synthetic-only bodies out of ordinary logs; store only necessary hashes/metadata. No credentials, private keys, agent access or full environments in tracked records.
- Distinguish Linux host-native, Linux container, private-Xvfb and unavailable native platforms. No browser/mobile/physical-IME/reboot result from a unit or container run.
- A negative candidate result can be a valid comparison outcome; an unresolved required failure/blocked material check prevents promotion and must be exposed. Apply `human-required` when actual code changes auth/permission/security behavior or required external evidence is missing, per project validation policy.
- Independent review must inspect all tracked/untracked changes and full applicable specs, not just latest fixes. Recheck actual service/agent/display/port cleanup before completion.

## Project-data validation and rollback

~~~text
python3 .trellis/scripts/task.py validate .trellis/tasks/10-22-sshx-phase0-feasibility
python3 .trellis/scripts/task.py list-context .trellis/tasks/10-22-sshx-phase0-feasibility
python3 .trellis/scripts/get_context.py --mode packages
git diff --check
git status --short
~~~

Also parse task JSON/JSONL, case evidence and protocol fixtures; inspect new-file formatting and local links; compare all existing tracked fingerprints against `/tmp/sshx-phase0-planning-baseline.json`. The planning baseline has 102 tracked files; only task-owned mainline/spec changes may differ before implementation, and any later authorized docs/profile changes must be individually accounted for. Source/notices/runtime/generic scaffolds remain unchanged.

If cleanup fails, retain a redacted exact-resource recovery note and report failure instead of deleting unrelated data. Do not use broad Docker prune, process-name/port kills, global network changes, history reset or root/home mounts. Each independent stage can be rerun only against its owned fixture state and applicable dedup/reconciliation semantics. Preserve any pre-existing user work.

## Final execution checkpoint

The final independent run passes 44 native cases and all 14 workspace unit tests; full checks and exact teardown pass. Source remained unchanged during the final integration run. The user supplied the explicit [human-required disposition](research/review-gate.md) and local Git/archive authority with “接受” on 2026-10-22. All runnable findings were resolved or accepted as documented limitations; the approved batch may now execute, with general mutable-root SFTP support kept no-go. No further fixture is running or requested.
