# Initialization Review

Review scope: approved documentation/bootstrap delivery, complete retained design, seven shared policy Markdown files, mainline, notice inventory, task/context data and generic bootstrap disposition. No application implementation, install, service, staging, commit or archive was performed by this reviewer.

## Loading evidence

The native dispatch contained `<!-- trellis-hook-injected -->` and a saved full-output notice at `/tmp/hook_outputs/01a15dde-078c-7451-a1f4-bf2f74c817d9/3252dc97-cc12-40b2-8c76-58a6f90dca6d.txt`. The saved file was read, but its printed tool result was truncated. The reviewer therefore explicitly read the current `check.jsonl`, all 13 referenced documents below, `prd.md`, and `implement.md` before checking. The design source was read completely in two chunks. A separate inspection verified that the saved native output represents all 13 manifest paths; this is supplementary evidence, not a substitute for the explicit reads.

1. `.trellis/spec/guides/index.md`
2. `.trellis/tasks/10-21-trellis-plus-bootstrap/research/discovery.md`
3. `.trellis/tasks/10-21-trellis-plus-bootstrap/design.md`
4. `.trellis/spec/trellis-plus/sshx-design.txt`
5. `.trellis/tasks/10-21-trellis-plus-bootstrap/research/source-provenance.md`
6. `.trellis/spec/trellis-plus/index.md`
7. `.trellis/spec/trellis-plus/development-principles.md`
8. `.trellis/spec/trellis-plus/sshx-contracts.md`
9. `.trellis/spec/trellis-plus/development.md`
10. `.trellis/spec/trellis-plus/validation.md`
11. `.trellis/spec/trellis-plus/commit-policy.md`
12. `.trellis/spec/trellis-plus/continuity.md`
13. `.trellis/mainline.md`

Also inspected the current check report, both task records, original bootstrap PRD/disposition, third-party inventory, current skill-local checklist IDs 01–34 and default enhancement references. Current-role loading is demonstrated. This does not prove future-task automatic reads, archive loading, or any product behavior.

## Findings fixed

- `commit-policy.md`: added the verified 0.6.17 archive-path reconciliation requirement. `archive_task_complete` calls `archive_task_dir`/`shutil.move`; the archive route does not rewrite JSONL self-paths or mainline links. After a successful move, update affected project-data references and validate the actual archived task and links before its explicit commit. Preserve historical descriptions and never rerun the move or modify runtime to repair paths.

## Findings for main-session finalization

- The check report is explicitly provisional. Replace planning-only evidence/statuses with these results and independent implement-loading evidence; keep future automatic loading, actual update-cycle execution and archive/commit evidence separate.
- Main session corrected the execution plan's planning-era state and PRD's final approval sentence during review. Actual review/finalization and closure steps remain open until performed.
- Main session corrected the generic bootstrap disposition's policy count: six detail documents plus the index, seven Markdown files total.
- Automatic future-task loading remains a documented integration gap with a usable manual procedure. No protected loading file was changed.
- Both tasks remain `in_progress` and unarchived; no commit exists. Full initialization closure and once-only archive attribution are pending the concrete authorized commit/archive batch. These are pending lifecycle/authority outcomes, not failed document checks.

No other material design-import or shared-policy defect was found. Source decisions, Rust defaults, candidates and unknowns stay distinct; product work remains proposed. SSHX-specific authentication, data, local IPC and trust exceptions correctly constrain generic development convenience. Before-dev, native/UI/preview/environment applicability, bounded guided continuity and supported archive retry/attribution rules are actionable without a skill-installation path.

## Verification

- `python3 .trellis/scripts/get_context.py --mode packages`: pass; discovers `backend`, `frontend`, `trellis-plus`; discovery alone is not loading.
- `python3 .trellis/scripts/task.py validate .trellis/tasks/10-21-trellis-plus-bootstrap`: pass; both manifests have 13 entries. The recorded `main` warning reflects an unborn repository (symbolic HEAD is `main`, no HEAD commit), not evidence of a deleted/merged feature branch.
- `task.py list-context` for the current task: pass; explicit seven policy files, canonical source, mainline and supporting research/design entries appear in both roles.
- `task.py validate`/`list-context` for `00-bootstrap-guidelines`: command pass; manifests are absent and skipped, not evidence of loading for that original scaffold task.
- Temporary read-only checks: two task JSON files and both JSONL files parse; 13 unique existing paths per manifest; 34 local Markdown file links and three heading anchors resolve; task-owned whitespace/EOF checks pass. Verbatim design and license were excluded from normalization/style checks.
- All 107 protected/template/input SHA-256 values match `/tmp/sshx-trellis-bootstrap-gp6dijdw/protected-baseline.json`. All 87 installed template-metadata entries still match. All 28 repository Python scripts match the installed 0.6.17 distribution templates byte for byte.
- Source is exactly 30,205 bytes and matches `/tmp/tmp/design.txt`: SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`.
- Retained LICENSE matches the installed 0.6.17 package: SHA-256 `d8a6cc31abc16b6748c7a21f21611f5a1ec33f67d22ca23d7da1c19b95496bee`. Its package root contains only `LICENSE`, with no separate `NOTICE`/`COPYRIGHT`.
- `git diff --check`: pass; untracked content was separately checked as above. Git index remains empty; `.codex`/`.agents` remain ignored. No new root README, application, manifest, dependency, CI or service entry exists. Shared Markdown policy has no absolute machine/skill-installation dependency.
- Application lint: N/A. TypeCheck: N/A. Application tests/build/native terminal/clipboard/SSH/SFTP/browser/mobile: N/A to this docs-only change, with future product/platform behavior explicitly unverified. No test framework or permanent checker was added.

Human gate recommendation: `human-not-needed` after main-session evidence finalization, because this is documentation-only, source preservation and material document checks passed, and no unresolved product or human-only judgment remains. This recommendation grants no commit/archive permission. Main session owns the final gate, report, explicit path plan, lifecycle and any later verification after archival.
