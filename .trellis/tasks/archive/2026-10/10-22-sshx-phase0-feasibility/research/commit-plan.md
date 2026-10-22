# Approved local Git/archive batch

Status: **approved on 2026-10-22 by the user’s “接受” response to this exact batch and the Phase 0 conclusion**. Execution is now authorized; completion evidence is recorded after each successful action. The proposal baseline and file inventories below are retained as the reviewed plan. Prepared after implementation, independent full-scope review, exact service/credential cleanup and spec synchronization. Read [the formal report](../check-report.md), [review disposition](review-gate.md) and [final integrity checks](final-validation.md).

## Decision requested

Accept the controlled Linux Phase 0 deliverable, including the **negative general SFTP mutable-parent/root feasibility finding**, then authorize the exact local batch below. Acceptance closes AC4 as a documented negative comparison outcome and the human-review portion of AC9. It preserves `phase0_ready: false`, the unsupported general-root guarantee, limited screen/API/platform coverage and the unestablished cause of the preserved initial supplementary timeout. This does not authorize general MVP promotion, real-user deployment or automatic later-phase work.

The execution approval explicitly reserved a later concrete Git/archive batch. [Validation policy](../../../../../spec/trellis-plus/validation.md#submit-ready-human-review-gate) requires human disposition for actual auth/permission enforcement and unresolved required contracts. [Workflow Phase 3.4](../../../../../workflow.md#34-commit-changes-required--once) separately requires one-shot approval of concrete commits. No approval is inferred from elapsed time.

## Sequence and exact messages

Baseline: current branch `phase0-feasibility`, base `main`, HEAD `ec1e1470381b67a07cef4885ea057f843e5f7578`. No Phase 0 commit exists, and the index is empty. Recent history uses English conventional prefixes. Reinspect actual state and each diff before staging; stop for unrelated changes or a non-fast-forward branch advance. Preserve Git identity. Use only explicit individual paths, with no push, amend, forced add, broad staging, history rewriting or protected-runtime edits.

1. On approval, record the actual user review decision in PRD/implementation/review/task/mainline evidence; mark AC4 as an accepted negative result and AC9 as human-reviewed without claiming MVP readiness. Commit the coherent controlled probe implementation and evidence on `phase0-feasibility`:

   ```text
   feat: add controlled Linux Phase 0 feasibility probes
   ```

   The complete individually classified inventory appears below. Review the staged path list and diff and run `git diff --check` before committing. No unrecognized files are included.

2. Capture the actual work commit OID. After verifying `main` has not advanced and the working tree is clean, switch to `main` and merge `phase0-feasibility` with `git merge --ff-only phase0-feasibility`. This supplies real feature-branch merge evidence; do not use the documentation-only branch-validation exception. Inspect task archive hooks/config and metadata again, then run the supported single-task command with auto-commit disabled:

   ```text
   python3 .trellis/scripts/task.py archive 10-22-sshx-phase0-feasibility --no-commit
   ```

   Confirm the exact destination, completed status/date and local pointer effects. Repair only affected project-data references: archived JSONL self-paths, task-owned relative links and the external references listed below. Preserve historical command/inventory descriptions. Validate the actual archived context and local links; stage each old/new path individually. Do not repeat a successful move on retry. Use this message, replacing `<ACTUAL_WORK_OID>` with the work commit captured above:

   ```text
   chore(task): archive SSHX Phase 0 feasibility

   Record the controlled Linux probes, 14 unit and 44 native checks,
   supplementary evidence, and exact fixture cleanup. Preserve the
   general SFTP parent/root no-go and limited platform/API coverage.
   User accepted the negative Phase 0 deliverable; no MVP promotion.

   Task: 10-22-sshx-phase0-feasibility
   Work: <ACTUAL_WORK_OID>

   Co-authored-by: OpenAI Codex <codex@openai.com>
   ```

   Exactly one matching Codex trailer belongs in this successfully archived task's commit. Inspect the resulting message/file list and record its actual OID.

3. After the work and archive commits, record the session using supported `add_session.py --no-commit` with the **actual work OID** as `--commit`; mention the archive OID in the journal content and mainline. Use the existing developer identity and a short entry so no journal rotation is needed. Verify the helper's actual file list. Commit only the following project-owned evidence files, without another attribution trailer:

   ```text
   chore: record Phase 0 feasibility session
   ```

   - `.trellis/workspace/wkyuu/journal-1.md`
   - `.trellis/workspace/wkyuu/index.md`
   - `.trellis/mainline.md`

All three commits and the local fast-forward merge are part of this one requested approval. The final report must distinguish committed work, completed archive, journal recording and the retained no-go finding. No future product task starts as part of this batch.

## Work commit: individual file classification

Every path below was created/edited for this task by the primary session or its scoped implementation/check workers. The 14 existing changed paths are authorized project-owned result/spec/mainline documents; all other pre-existing tracked files are preserved. Original source/tooling has no imported third-party code; Cargo.lock records dependencies, not vendored source. Existing notice inventory and retained notices are unchanged. Ignored caches, credentials, runtime pointers, personal agent settings and synthetic `/tmp` artifacts are excluded.

| Exact path | Class |
| --- | --- |
| `.gitignore` | New original probe source/tooling/configuration |
| `.trellis/mainline.md` | Existing project-owned spec/mainline/document |
| `.trellis/spec/trellis-plus/development.md` | Existing project-owned spec/mainline/document |
| `.trellis/spec/trellis-plus/index.md` | Existing project-owned spec/mainline/document |
| `.trellis/spec/trellis-plus/sshx-contracts.md` | Existing project-owned spec/mainline/document |
| `.trellis/spec/trellis-plus/validation.md` | Existing project-owned spec/mainline/document |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/check-report.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/check.jsonl` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/design.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/implement.jsonl` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/implement.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/prd.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/acceptance-map.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-core-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-execution.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-plugin-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-review.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/cleanup.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/cleanup.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/commit-plan.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-context.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-evidence.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-handoff.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-insert-negative-initial.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-insert-negative.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/dual-target-fixture.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/dual-target-fixture.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/execution-ledger.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/final-validation.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/fixture-evidence.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/fixture-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/ipc-crossuid.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/ipc-crossuid.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/plugin-evidence.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/plugin-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/readiness.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/review-gate.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-evidence.md` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-results.json` | New project-owned task/evidence data |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/task.json` | New project-owned task/evidence data |
| `dev.sh` | New original probe source/tooling/configuration |
| `docs/adr/0001-language.md` | Existing project-owned spec/mainline/document |
| `docs/adr/0002-pty-conpty.md` | Existing project-owned spec/mainline/document |
| `docs/adr/0003-transfer-backend.md` | Existing project-owned spec/mainline/document |
| `docs/adr/0004-screen-restoration.md` | Existing project-owned spec/mainline/document |
| `docs/adr/0005-plugin-isolation.md` | Existing project-owned spec/mainline/document |
| `docs/architecture.md` | Existing project-owned spec/mainline/document |
| `docs/image-paste.md` | Existing project-owned spec/mainline/document |
| `docs/phase-0.md` | Existing project-owned spec/mainline/document |
| `docs/platform-matrix.md` | Existing project-owned spec/mainline/document |
| `hako` | New original probe source/tooling/configuration |
| `probes/phase0/Cargo.lock` | New original probe source/tooling/configuration |
| `probes/phase0/Cargo.toml` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/build/.dockerignore` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/build/Dockerfile` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/lifecycle.py` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/plugin.py` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/sshd/.dockerignore` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/sshd/Dockerfile` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/sshd/entrypoint.sh` | New original probe source/tooling/configuration |
| `probes/phase0/fixtures/sshd/sftp-gate.sh` | New original probe source/tooling/configuration |
| `probes/phase0/harness/Cargo.toml` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/core.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/main.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/native.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/plugin.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/seal.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/sftp.rs` | New original probe source/tooling/configuration |
| `probes/phase0/harness/src/suite.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/Cargo.toml` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/native/xselection.c` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/image.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/main.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/plugin.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/runtime.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/suite.rs` | New original probe source/tooling/configuration |
| `probes/phase0/image-probe/src/visual.rs` | New original probe source/tooling/configuration |
| `probes/phase0/protocol/Cargo.toml` | New original probe source/tooling/configuration |
| `probes/phase0/protocol/src/lib.rs` | New original probe source/tooling/configuration |

Inventory: **80 individual work paths**. Unrecognized dirty files: **none**. This plan and the final validation record are included; neither records a planned action as completed.

## Archive commit: individual task moves

For each row, stage the old deletion/new addition individually after the supported move. The task metadata gets only the supported completion changes and actual work reference; no generic runtime/template files enter the batch.

| Old task path | Exact archive destination |
| --- | --- |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/check-report.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check-report.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/check.jsonl` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check.jsonl` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/design.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/design.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/implement.jsonl` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/implement.jsonl` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/implement.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/implement.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/prd.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/prd.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/acceptance-map.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/acceptance-map.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-core-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/check-core-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-execution.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/check-execution.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-plugin-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/check-plugin-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/check-review.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/check-review.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/cleanup.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/cleanup.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/cleanup.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/cleanup.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/commit-plan.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/commit-plan.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-context.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-context.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-evidence.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-evidence.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-handoff.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-handoff.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-insert-negative-initial.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-insert-negative-initial.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-insert-negative.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-insert-negative.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/core-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/core-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/dual-target-fixture.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/dual-target-fixture.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/dual-target-fixture.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/dual-target-fixture.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/execution-ledger.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/execution-ledger.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/final-validation.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/final-validation.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/fixture-evidence.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-evidence.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/fixture-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/ipc-crossuid.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/ipc-crossuid.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/ipc-crossuid.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/ipc-crossuid.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/plugin-evidence.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/plugin-evidence.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/plugin-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/plugin-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/readiness.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/readiness.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/review-gate.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/review-gate.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-evidence.md` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-evidence.md` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-results.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-results.json` |
| `.trellis/tasks/10-22-sshx-phase0-feasibility/task.json` | `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/task.json` |

Only the following external project-data files currently refer to the active task path and need link/mainline reconciliation in this archive commit:

- `.trellis/mainline.md`
- `.trellis/spec/trellis-plus/development.md`
- `.trellis/spec/trellis-plus/sshx-contracts.md`
- `docs/adr/0001-language.md`
- `docs/adr/0002-pty-conpty.md`
- `docs/adr/0004-screen-restoration.md`
- `docs/adr/0005-plugin-isolation.md`
- `docs/image-paste.md`
- `docs/phase-0.md`
- `docs/platform-matrix.md`

If the live move/helper produces any other tracked or unignored change, classify it explicitly against this scope before staging. Unrelated paths remain excluded. The archive evidence records completion after user acceptance of the negative deliverable; `phase0_ready: false` and the general-root no-go persist.

## Actual execution checkpoint

User approval “接受” supplied the review and exact local-batch authority. Work commit `620ca22a49611e319daf6df9f9aa22113d428133` includes exactly the 80 classified paths and was fast-forward merged to local `main`. The supported archive move completed on 2026-10-22, with task status/date and cleared pointer verified; archived links/context paths were reconciled. The archive and journal commit OIDs are recorded in mainline/journal after their creation. This inventory retains the original reviewed old/new path mapping and historical baseline. General-root no-go and `phase0_ready: false` persist.
