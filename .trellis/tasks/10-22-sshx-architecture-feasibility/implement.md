# Execution plan: SSHX architecture and feasibility package

## Phase gate

Final planning summary explicitly approved on 2026-10-22 ("ok；批准方案"). Architecture-documentation execution may begin after the installed activation gate. This approval does not authorize Git commits, archival or subsequent product phases.

## Ordered work

- [x] Read shared policies, mainline and source I–XIII; verify no application workspace or active product task exists.
- [x] Write and converge `prd.md`, `design.md` and this execution plan; curate both role manifests.
- [x] Obtain explicit approval of the final planning summary, then run the installed Phase 1.4 review/start gate.
- [x] Main session dispatches `trellis-implement` with the actual active-task path and exclusive ownership of the listed `docs/` outputs plus task `research/`. Require native context-injection evidence or complete child-side reads of both planning artifacts and every implement context entry. The worker is not alone in the repository and must preserve others' edits. It performs the role directly without recursively dispatching implement/check agents.
- [x] Verify current primary documentation for external technical claims, recording exact URLs, dates and limited findings in task research. No install, service, clipboard or remote actions. Missing external access is a reported limitation; do not invent APIs or cite search snippets as interface evidence.
- [x] Produce the architecture/requirement trace and platform matrix first, then the protocol/models, Image Paste sequence, six ADRs and Phase 0 runbook. Keep the artifacts mutually consistent and map R1–R7 and source XII groups to checks.
- [x] Main session reviews coverage, scope, trust boundaries and evidence. Update manifests only if real applicable research is added; workers do not modify lifecycle or mainline.
- [x] Dispatch `trellis-check` with ownership limited to task-owned documentation/research corrections and `check-report.md`. Require check context loading evidence and a full-scope review of AC1–AC8; record remaining material decisions instead of redefining acceptance to pass.
- [x] Resolve findings, rerun affected checks, and record the documentation-versus-runtime evidence boundary and submit-ready human review category.
- [x] Main session updates reusable specs only if a new established contract warrants it, and records verified results and the next gated candidate in mainline. No automatic Phase 0 task creation.
- [x] Prepare an explicit commit/archive proposal only after checks, if needed. Obtain the separately required authority before staging, committing, completing or archiving.

Delivery status: AC1–AC8 passed independent full-scope document review; main session classifies `human-not-needed` for this documentation-only scope. The user explicitly approved the separate three-commit batch in `research/commit-plan.md` on 2026-10-22. Execution is starting; no Git/lifecycle completion is implied before the actual commands succeed.

## Validation

From repository root, using the actual task path:

```sh
python3 .trellis/scripts/task.py validate .trellis/tasks/10-22-sshx-architecture-feasibility
python3 .trellis/scripts/task.py list-context .trellis/tasks/10-22-sshx-architecture-feasibility
python3 .trellis/scripts/get_context.py --mode packages
git diff --check
git status --short
```

Additionally inspect every new task/output file for trailing whitespace and final newline; parse task JSON and every JSONL record; resolve relative Markdown links (including fragment targets where present); validate actual protocol JSON examples; inspect Mermaid syntax against diagram definitions; and compare source/protected fingerprints. Baseline fingerprints of tracked input files were saved locally under `/tmp/sshx-architecture-planning-baseline.json` before project-data edits; use the actual baseline count from that file during verification. Preserve/update this evidence path if the session continues on another machine.

Review source I–XIII and all twelve XII groups against the trace table. Trace one successful Image Paste plus switch/detach/reconnect/cancel/revoke failures through each layer; check no retargeting, duplicate insertion or broad cleanup is possible under the written contract. Confirm each quota/timeout has one normative value and a defined failure behavior. Distinguish proposed protocol choices from measured platform behavior.

Cargo, native clipboard/terminal, SSH/SFTP integration, browser and mobile tests do not execute in this docs-only task. Native runtime results remain unverified; browser/mobile checks are inapplicable to the native design documentation. Exact product test commands belong to the later task once a real workspace/harness exists.

## Risk and rollback points

- Protected: `.trellis/workflow.md`, scripts/runtime/platform helpers, generic spec scaffolds, verbatim design and exact third-party notices. Compare against baseline; never repair them by bulk reset.
- Mainline: main-session-only narrow lifecycle/scope edits; preserve historical initialization authority and commit evidence.
- Before completing protocol/ADRs: stop on conflicts with fixed source requirements and resolve them before worker continuation.
- Before commit/archive: apply the project review and authorization gates; no automatic Git operations or next-phase execution.
- Roll back task-owned text with a reviewed patch only; preserve unrelated changes and do not remove unknown files or source data.
