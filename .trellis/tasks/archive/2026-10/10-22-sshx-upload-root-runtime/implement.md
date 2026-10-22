# Execution and validation plan

## Gates and ownership

- [x] Task creation and disposable Docker/Linux/OpenSSH scope approved; repository and pinned-library interfaces inspected.
- [x] PRD converged; design and acceptance matrix written; no blocking product decision remains.
- [x] Final planning summary approved with “批准” on 2026-10-22; task.py start completed and branch set to upload-root-runtime.
- [x] Main reads phase 2.1 and applicable before-development/security/language guidance, records baseline fingerprints and activates a task branch using the supported workflow.
- [x] Dispatch trellis-implement for the bounded implementation scope, then trellis-check for independent full-scope verification. Every dispatch starts with the current Active task path. Prefer native injection; directly read omitted files and report loading evidence. Workers are not alone in the workspace and must preserve others' edits. Main retains lifecycle, mainline, scope/spec and Git ownership.

Known context limit: task validation reports docs/protocol.md is 54,261 bytes, above the 32,768-byte per-file injection limit. Both roles must directly read its omitted remainder (or the entire file in bounded chunks), and any omitted PRD/design/implement or manifest input. A successful hook marker does not establish complete loading. Read architecture context directly when a changed field requires it; do not rely on index links being followed.

Execution checkpoint 2026-10-23: code, independent checks, shared-spec updates and exact fixture cleanup are complete for bounded technical acceptance. Current protocol/runbook both exceed the injection limit; both roles reported direct-read recovery. The final report and review-gate own actual evidence, failures and limitations. Human-required disposition and the concrete Git/archive/journal batch remain pending.

## Ordered work

1. Read existing wrapper/fixture code and current runtime state. Check pinned cached resources without starting unknown services. Preserve any unrelated existing fixture; use an isolated owned pointer when needed. Record host/SSH/image/library/filesystem identities and exact teardown authority. Do not install a host toolchain or update dependencies to work around missing resources.
2. Add extension-version retention and direct hardlink support, qualification evidence and deterministic bounded fault support. Keep baseline rename only in explicitly labeled historical comparison. Test fragmented/coalesced reply matching and generic-error classification meaningfully.
3. Integrate Core policy/scope binding, per-effect checks, acknowledgement/dispatch outcomes and ledger-based cleanup. Update actual probe projections and callers together. No synthetic shortcut may replace real upload admission.
4. Add the cases below to the existing suite or a focused upload subset sharing its Core path. A focused command may be added if needed; record the actual supported command in task evidence. Extend only necessary generated fixture setup for denied operations or separate-filesystem cases.
5. Build, run fresh primary/secondary loopback fixtures through existing lifecycle, execute focused cases and affected full Core suite, then independent check with new outputs. Capture negative evidence without overwriting historical results. Main coordinates teardown after independent review and records exact container/process/port/credential cleanup.
6. Synchronize only verified current documentation, source/case mappings and limitations. Main updates reusable specs/mainline after check, records the human review category, then presents the concrete result and Git/archive plan. Runtime permissions/deletion changes require human review before closure under validation.md. No Git commit/archive or automatic next phase is included in planning approval.

## Mandatory acceptance matrix

| Cases | Requirement | Method and expected evidence |
| --- | --- | --- |
| C01 | R1/R2 | Real qualification records version 1, successful link and existing-name preservation on the actual root/filesystem; missing qualification denies upload creation |
| C02 | R1 | Core upload: closed temp, actual acknowledged hardlink, exact final hash, owned-temp removal, retained final directory |
| C03 | R1/R3 | Existing final, symlink final, and creator between precheck and dispatch: preserved existing data, no fallback, no automatic new name; distinguish confirmed collision from generic failure |
| C04 | R2 | Missing/unsupported extension version and explicit unsupported response via labeled injection; actual cross-device link rejection on separate fixture mounts; no upload success/fallback, unknown remote status stays generic |
| C05 | R2 | Absent cooperative policy, stronger requirement, root/namespace/policy/config change, wrong instance/grant and stale B: correct denial ordering and no new effects; malformed input does not consume authority |
| C06 | R2/R4 | Known traversal, prefix-confusable root, symlink/ancestor/type inconsistency before creation/publication/cleanup: refuse effects; no stronger ABA guarantee claimed |
| C07 | R2/R4 | Cancel and revoke before mkdir/open/write/close/link, plus during stream; disable/expiry/stale generation regression; actual effect and filesystem evidence distinguishes already-admitted work from prohibited new work |
| C08 | R3/R4 | Link acknowledged then temp unlink denied by real fixture permissions where feasible: confirmed final/hash preserved, cleanup pending, no compensating final removal; separately label any injection variant |
| C09 | R3/R4 | Drop only matching hardlink reply, including fragmented/coalesced transport tests: indeterminate result, no replay/input/deletion; independent read-only observation does not promote operation result |
| C10 | R4 | Missing ledger, observed replaced temp/parent, lost transport and boot/generation change: no automatic deletion/adoption; cancellation with intact continuity allows only exact owned-temp cleanup |
| C11 | R2/R3 | Same key retries after success/collision/unknown outcome and changed intent: no duplicate publication; mapping/context refusal keeps final result without input |
| C12 | R2/R5 | Two generated targets and session switching: no cross-target files or PTY input; unavailable upload does not break basic login; current historical regression assertions remain meaningful |
| C13 | R5 | Independent results, source/environment linkage, preserved historical fingerprints, documentation consistency and exact fixture teardown |

Failures cannot be hidden by removing a case. Missing mandatory evidence blocks positive promotion; an honest bounded negative report may be delivered for user disposition. Tests with injected capability observations do not establish a real unsupported server's behavior.

## Validation commands

After activation, run through existing pinned tooling (not executed during planning):

```sh
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --offline --workspace --all-targets -- -D warnings
./hako cargo check --locked --offline --workspace --all-targets
./hako cargo test --locked --offline --workspace --all-targets
./hako cargo build --locked --offline --workspace
```

Use `./dev.sh start/status/down` only for the verified owned primary pointer. For a secondary pointer reuse the archived dual-target lifecycle procedure with fresh run identities. Do not reuse old IDs/ports/credentials. Existing integration command shape (replace placeholders with fresh actual paths):

```sh
.devhome/target/debug/phase0-probe suite --fixture PRIMARY_STATE --fixture-secondary SECOND_STATE --output NEW_JSON
```

Run shell syntax/ShellCheck/shfmt only when shell changes, and Python syntax/annotation/style checks if Python changes, following language skills. Full image desktop/visual runs are excluded; compile and adapt image callers and cover their changed transfer projection in focused synthetic tests if affected. No clipboard/display execution is needed for this upload task.

Validate task contexts, JSON/JSONL, exact new text formatting and local links; run git diff --check and inspect final path inventory. Preserve canonical source/notices/protected Trellis files and archived machine reports. Record all actual commands, failures and unavailable checks in current-task research. No current planning check is a runtime PASS.
