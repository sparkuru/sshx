# Initialization Design

## Scope and ownership

This task adds project data to an existing Trellis 0.6.17 installation. It does not reinstall or update Trellis. New policy is original SSHX project text, informed by `design.txt`; it is not copied tool source or a modified upstream template.

| Target | Ownership | Treatment |
| --- | --- | --- |
| `.trellis/spec/trellis-plus/index.md` and linked new details | Project-shared | Canonical policy and loading instructions |
| `.trellis/mainline.md` | Project-shared | Source/approval boundaries and guided roadmap |
| This task's planning, manifests, research and check evidence | Project-shared | Existing task schema and context mechanism |
| `00-bootstrap-guidelines` task records | Task data | Append supersession/closure evidence, retain original history |
| `third_party/` | Exact third-party material plus original inventory | Preserve matching notices and provenance |
| Runtime, config, version/hashes, managed AGENTS block, generated skills/agents | Protected | Read-only; snapshot content before application |
| Existing backend/frontend spec scaffolds | Provenance-sensitive templates | Read-only; explain their replacement by project-owned guidance |
| `.codex/`, `.agents/`, secrets and runtime state | Personal/local | Preserve existing ignore rules; do not stage |
| `design.txt`, `.gitattributes`, `AGENTS.md` | Existing user work | Preserve; do not include automatically in the change set |

## Policy structure

Keep the shared index small. Link separate original detail files for development principles, SSHX architecture/contracts, development environment, validation/review, commit/archive policy, and continuity. Do not maintain parallel copies in generic backend/frontend indexes or personal settings.

The SSHX contract summary names source sections and differentiates fixed product boundaries (independent plugins, local C/S, OpenSSH credentials), default technology choices (Rust), and unknowns requiring Phase 0 (PTY/ConPTY, transport choice, desktop clipboard, screen restoration). It must not invent runtime API availability or examples from nonexistent application code.

## Loading and evidence flow

1. Main session reads the shared index and mainline at task start/resume, before archive, and before selecting work.
2. Existing `task.py add-context` registers the index plus every applicable detail in both `implement.jsonl` and `check.jsonl`; Markdown links alone are not a loading mechanism.
3. Dispatch prompts begin with the active task path. Prefer native context injection; agents explicitly pull manifest contents when injection is absent and report the paths loaded.
4. Check manifests with `task.py validate`, then inspect observed agent loading evidence. Do not modify protected loaders to claim future automation.
5. If no supported portable startup extension loads this namespace for future tasks, document the exact manual read/register procedure and retain the automatic-discovery gap in the report.

## Development and validation applicability

No Cargo manifest or app command exists. Establish a before-dev checkpoint rather than a nonfunctional wrapper: inspect the first workspace/toolchain decision, pin actually tested versions, select containerized checks where useful, and keep native terminal/clipboard/ConPTY validation separate. The future local IPC Core is not an HTTP preview service. Do not open network listeners for initialization.

The source proposes native CLI/TUI interactions, not browser pages. This documentation/bootstrap task does not change an interface. UUPM, desktop/mobile browser fixtures, Playwright profiles and preview console integration are not installed. Reclassify when an actual UI task is approved; a missing UI tool must never be the sole reason to declare non-applicability.

Initialization checks cover manifests, links, source/import fidelity, protected file fingerprints, exact notice comparison, scope and whitespace. Future cargo build/test commands remain explicitly conditional until a workspace exists. Platform behavior remains unverified.

## Archive and migration

The installed archive interface supports `--no-commit` and `--skip-branch-validation`. Its default auto-commit has no custom message input. The supported attribution route is on-disk archival without auto-commit, followed by an explicitly scoped commit with the exact Codex trailer after commit authorization. Inspect history/destination before retrying; archive commits and journal commits are separate.

Resolve `00-bootstrap-guidelines` by recording how the new SSHX rules supersede Web/backend scaffolds. Keep original unfulfilled checkboxes unchecked; no code examples exist to fabricate. Substantive unfinished acceptance blocks closure. Archive only after the final gate, with commit status stated separately. The current new task also follows normal task closure when ready.

## Risks and rollback

- The repository has no commits; all existing files are untracked user work. Use content fingerprints to detect accidental edits; do not infer ownership from `git diff` alone.
- Exact Trellis notice provenance and future-task automatic loading may remain gaps; report these rather than alter upstream runtime.
- A rerun merges into these same project paths and task records. Never duplicate initiative rows, context entries, or attribution.
- Rollback removes only files created by this task and restores only known task-data changes. Never delete or reset existing user files. Archive rollback is not automatic; inspect partial move/commit state first.
