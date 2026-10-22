# Planning evidence and source map

Inspected on 2026-10-22. This record maps repository-local requirements to the proposed deliverable; it is not external technical verification or runtime evidence.

## Evidence

- `get_context.py` reported branch `main`, a clean worktree, no current/active task, developer `wkyuu` and the three backend/frontend/trellis-plus layers before task creation.
- The shared layer explicitly supersedes generic backend/frontend scaffolds for this CLI/TUI project. Its source and mainline establish architecture planning before Phase 0.
- A visible application-file inventory found only `AGENTS.md` and exact third-party notice inventory/license outside the managed/project-data directories. No Cargo workspace, application source, product tests or CI command was found.
- Current user authorization: request to begin implementation, followed by consent to creating the architecture-and-feasibility planning task. No final-plan approval, Git commit/archive permission or serial phase authorization has been received.
- The create command succeeded and set task status to `planning`. It warned that no repository default branch could be resolved and used the checked-out `main` as `base_branch`; it did not change Git branch or history.
- Full retained source I–XIII and shared policies were read. Prior initialization decisions are already present in these inputs; no past-session transcript search is needed.
- Before editing tracked project data, 80 tracked-file SHA-256 fingerprints were saved to `/tmp/sshx-architecture-planning-baseline.json`. Only the project-owned mainline is an intended tracked change at this planning stage.

## Source coverage

The canonical input is [sshx-design.txt](../../../spec/trellis-plus/sshx-design.txt). Anchors below are section and line references to that unchanged input, not a claim of product implementation.

| Source | Lines | Planning requirement | Required design coverage |
| --- | --- | --- | --- |
| I | 4–19 | R1, R6 | Intent, independent command/plugins, source default versus fixed decisions, superseded built-in approaches, deferred release naming |
| II | 21–47 | R2, R3 | Client/remote dimensions, argv/CLI conflict and invocation modes, OpenSSH semantics and credential ownership |
| III | 49–85 | R3, R4 | Process/component ownership, client-local clipboard, explicit session context, public capability boundary |
| IV | 87–113 | R2, R4, R6 | Seven models, single writer, byte streams, concurrency and lifecycle/recovery distinctions |
| V | 115–132 | R2, R4–R7 | Three kinds of reuse, two SFTP candidates with one eventual selection, auxiliary auth/options and namespace/target binding |
| VI | 134–162 | R4, R6 | Independent lifecycle/manifest, byte framing, protocol/version/error/limits, channel identity and effect-time authorization |
| VII | 164–190 | R5 | Explicit trigger/snapshot, bounded image/blob, transactional upload, conditional insertion, retries and cleanup |
| VIII | 192–207 | R1, R6, R7 | Later plugin/integration scope, structured history, loopback forwarding, trusted local actions and sourced remote events |
| IX | 209–217 | R3–R6 | Threat model, IPC roles/peer checks, credentials and data boundaries, effect-site constraints and side effects |
| X | 219–235 | R3, R6 | Default Rust/OpenSSH, unselected dependency candidates, actual-boundary layout and public-protocol-only dependencies |
| XI | 237–269 | R2, R7 | Phase 0 experiments and Phase 1–4 gates; honest platform gaps and independent-plugin MVP |
| XII | 271–287 | R5, R7 | All twelve fault groups mapped to future test methods and phase applicability |
| XIII | 289–304 | R1–R7 | Seven architecture/planning deliverables preceding execution; reports distinguish verified facts and untested hypotheses |

## Decisions carried forward versus deferred

Carry forward independent `sshx`, local Client/Core, Core-owned PTY/children, independent public-protocol plugins, one writer, operation/generation ownership, OpenSSH credentials, Content-Length UTF-8 JSON-RPC and conditional insertion. Retain Rust/system OpenSSH as the source default, rather than newly presenting it as a user-mandated technology.

Before executing Phase 0, choose the primary native environment and establish controlled fixtures. Use measured results to select a transfer backend and restoration strategy. Do not choose exact dependency versions or infer host desktop availability during this documentation task. The matrix must list file-selection fallback for inaccessible Linux clipboard as a separately explicit user action, never silent filesystem scanning or an Image Paste success claim.

No user-owned decision currently blocks defining the architecture-documentation task. Concrete protocol proposals are reviewed against the source during execution; a conflict with product requirements returns to planning instead of being silently resolved by the worker.

## Planning convergence and validation

The final PRD was read top to bottom after convergence. It has one requirement table, mapped observable acceptance criteria, explicit non-goals and downstream technical gates. Resolved decisions were folded into those sections; there is no blocking open question for the documentation scope. `design.md` and `implement.md` are present. Final planning-summary approval remains pending, and task status remains `planning`.

Actual planning checks on 2026-10-22:

- `task.py validate .trellis/tasks/10-22-sshx-architecture-feasibility`: PASS, 11 real entries in each role context.
- `task.py list-context` for that task: PASS, both contexts enumerate the intended existing inputs without duplicate paths.
- `get_context.py --mode packages`: single repository, backend/frontend/trellis-plus layers; shared policy takes precedence over generic scaffolds.
- Focused inline standard-library inspection: PASS for the eight planning/mainline files, JSON/JSONL parsing, context paths, local Markdown link targets, text whitespace and Markdown/JSONL final newlines.
- Baseline comparison: PASS, 79 of 80 tracked input files unchanged; only the intended project-owned mainline differs. The retained design is still 30,205 bytes with SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`.
- `git diff --check`: PASS. `git status --short` contains only mainline modification and this new task directory; no staging, commit, archive or branch change occurred.

These checks establish planning-file integrity only. No implement/check agent has yet been dispatched, so actual child-side context loading and independent full-scope design verification remain execution gates, not passed checks. No product/native/SSH/clipboard test ran. No submit-ready completion/archive classification is made at this planning stage.

## Execution authorization

After the final summary and language/architecture explanation, the user explicitly approved the plan on 2026-10-22 ("ok；批准方案"). `task.py start` changed status to `in_progress`; the task records `main` as its current/base branch. The tool warned that normal PR-based archival would refuse that branch relationship. This documentation task has no branch/PR yet; commit/archive routing remains a later main-session authorization decision, not an application-delivery blocker or permission to bypass checks.
