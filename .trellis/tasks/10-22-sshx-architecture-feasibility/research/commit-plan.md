# Proposed commit and archive batch

Status: prepared and **explicitly authorized on 2026-10-22**, execution starting. After the checked deliverable and this concrete three-commit batch were presented, the user replied "可以". The independent check passed AC1–AC8; the main session recorded `human-not-needed` before preparing the proposal. There is no application/native runtime evidence and no Phase 0 continuation in this batch.

## 1. Work commit

Message: `docs: define SSHX architecture and Phase 0 plan`

Body: document the Client/Core/OpenSSH and independent-plugin boundaries, proposed protocol/state/resource contracts, Image Paste failure flow, six ADRs and E0–E7 feasibility procedures. Include task acceptance/check evidence and reusable spec/mainline updates. Explicitly retain unverified platform/backend/restoration status. This work commit has no archive attribution trailer.

Exactly 25 individually reviewed project-owned files:

~~~text
.trellis/mainline.md
.trellis/spec/trellis-plus/index.md
.trellis/spec/trellis-plus/sshx-contracts.md
docs/architecture.md
docs/platform-matrix.md
docs/protocol.md
docs/image-paste.md
docs/phase-0.md
docs/adr/0001-language.md
docs/adr/0002-pty-conpty.md
docs/adr/0003-transfer-backend.md
docs/adr/0004-screen-restoration.md
docs/adr/0005-plugin-isolation.md
docs/adr/0006-remote-integration.md
.trellis/tasks/10-22-sshx-architecture-feasibility/task.json
.trellis/tasks/10-22-sshx-architecture-feasibility/prd.md
.trellis/tasks/10-22-sshx-architecture-feasibility/design.md
.trellis/tasks/10-22-sshx-architecture-feasibility/implement.md
.trellis/tasks/10-22-sshx-architecture-feasibility/implement.jsonl
.trellis/tasks/10-22-sshx-architecture-feasibility/check.jsonl
.trellis/tasks/10-22-sshx-architecture-feasibility/check-report.md
.trellis/tasks/10-22-sshx-architecture-feasibility/research/source-map.md
.trellis/tasks/10-22-sshx-architecture-feasibility/research/primary-sources.md
.trellis/tasks/10-22-sshx-architecture-feasibility/research/implementation-evidence.md
.trellis/tasks/10-22-sshx-architecture-feasibility/research/commit-plan.md
~~~

## 2. Single-task archive commit

After the authorized work commit succeeds, use the installed supported route:

~~~text
python3 .trellis/scripts/task.py archive 10-22-sshx-architecture-feasibility --no-commit --skip-branch-validation
~~~

The branch exception is limited to this documentation-only task, which was never PR-backed and records `main` as both branch/base. It does not waive missing merge evidence for a feature task. Installed help confirms both flags; `.trellis/config.yaml` currently has no active after-archive hook. Recheck state/hooks before executing. Do not let the archive command auto-stage or auto-commit.

Proposed message:

~~~text
chore(task): archive 10-22-sshx-architecture-feasibility

Complete the source-XIII architecture package: eleven documents, proposed
protocol contracts, six ADRs and the Phase 0 feasibility runbook.
Independent document/source/context checks passed; native product behavior
remains unverified. Record the completed task and reconcile archive links.

Task: 10-22-sshx-architecture-feasibility
Work: append the actual successful work-commit hash before commit

Co-authored-by: OpenAI Codex <codex@openai.com>
~~~

The Work line is deliberately a proposal placeholder, not an existing commit claim. Replace it with the actual hash only after step 1 succeeds. Exactly one matching Codex trailer belongs to this successful task archive, and none to the work/journal commits. Preserve Git identity; no amend or push.

The eleven task files above move from `.trellis/tasks/10-22-sshx-architecture-feasibility/` to `.trellis/tasks/archive/2026-10/10-22-sshx-architecture-feasibility/`. Review/stage each old removal and each new destination individually. The complete relative suffix list is:

~~~text
task.json
prd.md
design.md
implement.md
implement.jsonl
check.jsonl
check-report.md
research/source-map.md
research/primary-sources.md
research/implementation-evidence.md
research/commit-plan.md
~~~

Expected non-task files in this archive commit, individually reviewed:

~~~text
.trellis/mainline.md
.trellis/spec/trellis-plus/index.md
docs/architecture.md
~~~

These reconcile actual archive links/current-task evidence. Rewrite live JSONL self-references and relative Markdown links inside the moved task files; do not rewrite historical commands or original loading-path evidence as if they ran at the archive path. Validate both manifests and all local links at the actual archive destination. Verify completed status/date, cleared local task pointer, actual changed inventory and the single attribution trailer. If a move succeeds but commit fails, resume the pending commit after inspection, never repeat the move.

## 3. Session journal commit

Message: `chore: record SSHX architecture session`

After work and archive commits, record one session with `add_session.py --no-commit`, the actual work-commit hash, actual checks/gaps and no automatic next task. Installed help supports the no-commit option. Retain an idempotency key for retry safety. Stage only the two expected journal files individually:

~~~text
.trellis/workspace/wkyuu/journal-1.md
.trellis/workspace/wkyuu/index.md
~~~

The current journal has ample room; inspect again before writing, and report any unexpected rotation/path rather than silently broadening this list. This separate session commit receives no task attribution trailer. Reconcile the archived work/archive commit references in mainline during this final bookkeeping commit if those references are not yet retained; in that case the one additional eligible file is `.trellis/mainline.md`.

## Execution boundary

Use explicit individual paths, inspect staged diffs and `git diff --check` before every commit, and verify the resulting history/files/messages afterward. Never stage a directory subtree, local session pointers, platform settings, secrets, runtime/template files or unrelated work. No unrecognized dirty files are currently present. If unrelated changes appear before execution, preserve and report them rather than including/cleaning them.

No action in this batch creates/starts Phase 0, selects its primary native environment, installs dependencies, starts services, reads a clipboard, connects remotely or pushes Git history. The next product candidate remains separately gated in mainline. The user has approved this concrete batch; retain actual step outcomes and hashes rather than claiming completion before each command succeeds.
