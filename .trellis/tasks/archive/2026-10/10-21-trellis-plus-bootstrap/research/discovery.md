# Initialization Discovery

## Observed baseline (2026-10-21)

- Git branch `main` has no commits. Initial status: `.gitattributes`, `.trellis/`, `AGENTS.md`, `design.txt` are untracked. Platform directories `.codex/` and `.agents/` are ignored by the user's existing global rules.
- `.trellis/.version` identifies 0.6.17. `get_context.py` reports developer `wkyuu`, no current task, and `00-bootstrap-guidelines` in progress.
- There are no app manifests, source, CI, app tests, Docker lifecycle, environment example, browser UI, or project-owned Trellis Plus policies.
- Backend/frontend index files explicitly say to fill generic guidelines; they are not evidence of a Web app or an existing database.
- The user consented to creating this initialization task. Product implementation and Git commits are not included automatically.

## Design source map

| Source sections in `design.txt` | Import into project guidance |
| --- | --- |
| I–II | SSHX codename; independent `sshx`, original `ssh` untouched; CLI/TUI; three client/remote platforms; explicit passthrough/rejection semantics |
| III–IV | Local Client/Core, desktop-scoped clipboard, PTY/ConPTY, single writer, attachment ownership, generation-bound operations, bounded byte streams; survival/screen/network restoration distinguished |
| V | System OpenSSH authentication/config, one evidence-selected SFTP backend, auxiliary authentication limits, target/config binding, distinct SFTP/Shell namespaces |
| VI | Independent plugin executables, versioned Content-Length JSON-RPC, bounded messages/logs, opaque blobs, capability enforcement at effect sites, trust versus OS sandbox distinction |
| VII | Snapshot clipboard once; PNG limits; unique temp upload; completed-transfer insert proposal; cancellation/stale-target rejection; safe text output and fallback |
| VIII–IX | Later plugin boundaries; opt-in structured history; loopback forwarding; authenticated local IPC; sensitive-data exclusion; no fabricated remote context |
| X | Rust/OpenSSH default, candidate libraries not validated APIs; proposed modules, no premature crates |
| XI–XIII | Architecture/ADR and platform matrix before Phase 0; later MVP and phases; risk-based acceptance and honest unverified-platform reporting |

The source's embedded links have not been used as external verification. Import its requirements without making unsupported claims about current third-party APIs.

## Skill/procedure discovery

Read the user-specified Trellis Plus `SKILL.md`, its current sibling `CHACKLIST.md` (IDs 01–34), and all Default Enhancement Set references. Conditional preview console/base Docker procedures are needed only if actually implementing preview/wrappers; this task plans a before-dev checkpoint instead.

Read installed `trellis-start` and `trellis-brainstorm`. The latter requires a final planning summary and a subsequent approving user response before `task.py start`.

## Installed interfaces

- `task.py create` seeds empty manifests, supports `--no-start`, and creates planning task data.
- `task.py add-context <dir> implement|check <path> <reason>` registers existing documents; `validate` and `list-context` are available.
- `task.py init-context` is removed; do not use it.
- `task.py archive <name> --no-commit [--skip-branch-validation]` supports avoiding auto-commit; no custom message flag exists.
- `common/task_store.py` `cmd_archive` marks completed, clears session pointers, moves the directory, updates relationships and runs configured archive hooks. Default auto-commit scopes source/destination and edited children.
- `add_session.py --no-commit` supports separate journal recording. Its default auto-commit is enabled by installed config; avoid accidental automatic commits.
- `git log --format=%B -n 20` fails because the repository has no commits. Do not invent a historical commit style.

## Expected remaining checks

- Exact license/notice provenance from the installed package at implementation time.
- Hash snapshot before applying policy; application does not update Trellis.
- Actual current-agent context loading after policy registration; future-task automatic loading is not established by existing spec discovery alone.
- Final complete checklist report, then review/commit/archive authority and closure evidence.

## Execution approval and source relocation

The user approved the final initialization plan and said to begin implementation. `task.py start` succeeded using the native session identity. The user subsequently identified the design's new temporary path and requested its retention in spec and mainline; see `research/source-provenance.md`. This adds exact source retention without expanding into product implementation.

Notice collection verified the installed Trellis npm package version is also 0.6.17; all 87 template hash entries and all 28 Python runtime templates match. The license is preserved in `third_party/`. No Trellis update, backup restore or upstream patch was run.

Archive implementation caveat: despite the narrower `task_store.py` description, `safe_archive_paths_to_add(task_name=...)` includes the entire `.trellis/tasks/archive` subtree plus affected children. This reinforces using `--no-commit` and explicit individual archived-task paths, rather than default auto-staging.
