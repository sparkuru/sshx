# Trellis Plus Project Policy

- ownership: project-shared
- source: project-authored; the separately retained design source has its own provenance below
- tracking: track this index and its project-owned details through explicit-path review; local agent settings and secrets remain untracked
- baseline: Trellis 0.6.17; SSHX has design documents and initialization data, with no application workspace, CI or runnable product yet

This directory is the single shared policy entry for SSHX. It supplements the installed Trellis workflow without modifying its runtime. The existing backend/frontend guideline files remain generic scaffolds: these project rules supersede their generic assumptions for this design-stage CLI/TUI project. They do not claim that scaffold examples or application conventions have been implemented.

## Read by responsibility

| Document | Load when |
| --- | --- |
| [Development principles](development-principles.md) | Starting, resuming, implementing or reviewing any project task |
| [SSHX contracts](sshx-contracts.md) | Planning or changing CLI, Core, sessions, transport, plugins, clipboard or permissions |
| [Development environment](development.md) | Before application development; whenever toolchain, fixtures or configuration changes |
| [Validation and review](validation.md) | Planning acceptance, checking changes, and before commit/completion/archive |
| [Commit and archive policy](commit-policy.md) | Preparing staging, commit, initialization closure or task archival |
| [Continuity](continuity.md) | Task start/resume, approved scope changes, check results, archive or relevant no-task requests |
| [Design source](sshx-design.txt) | Requirements planning and any change affecting the design contracts; cite sections I–XIII |

Also read [mainline](../../mainline.md) for current scope, authorization and ordered work. Exact retained third-party notices and provenance belong in the [notice inventory](../../../third_party/index.md); unknown provenance must remain `license-notice-needed`.

The user authorized importing the current design into spec and mainline. `sshx-design.txt` preserves that supplied source verbatim; it is requirements input, not Trellis source. Its import and this initialization do not authorize product implementation or serial continuation. Fixed source decisions, default choices and unverified candidates remain distinct in the contracts and mainline. The external links within the source have not established current API/platform behavior.

## Actual loading path

`python3 .trellis/scripts/get_context.py --mode packages` discovers the `trellis-plus` layer. Discovery does not establish automatic reading of this index or its Markdown links. No protected loader has been changed. Future-task automatic policy loading remains an integration gap; use the following manual procedure until an existing supported loading mechanism is demonstrated.

1. At task start/resume, before commit/archive and before choosing later work, the main session reads this index, mainline and the applicable details.
2. Set `TASK_DIR` to the current task's actual repository-relative path. Inspect its existing manifests, preserve entries and add each applicable document separately to **both** contexts:

   ```sh
   python3 .trellis/scripts/task.py add-context "$TASK_DIR" implement .trellis/spec/trellis-plus/index.md "Shared SSHX policy entry"
   python3 .trellis/scripts/task.py add-context "$TASK_DIR" check .trellis/spec/trellis-plus/index.md "Check shared SSHX policy"
   ```

   Repeat for required detail/source paths and mainline; the loader does not follow this table's links. Do not duplicate existing entries. Check with `task.py validate` and `task.py list-context` as described in the validation profile.
3. Dispatch with `Active task: <actual task path>`. Prefer complete native hook injection. If the hook gives a saved-output path, read that full output. If injection is absent or unusable, read the role JSONL, every referenced file, `prd.md`, and any `design.md`/`implement.md` before work.
4. Workers report the exact paths loaded and injection/fallback evidence. Current-task context registration and observed loading belong in task evidence; file existence alone cannot pass loading checks. The main session retains lifecycle and mainline ownership.

## Write ownership and updates

Original shared policy, mainline and normal task records are project data. Treat `.trellis/workflow.md`, `scripts/**`, `agents/**`, `config.yaml`, `.gitignore`, version/template hashes/backups, the managed AGENTS block and generated platform helpers as read-only Trellis material. Existing generic spec scaffolds are provenance-sensitive and remain unchanged. Keep platform settings such as `.codex/` and `.agents/`, runtime pointers and local secrets local; preserve current ignore behavior.

Before staging, apply the explicit-path ownership and notice checks in the commit policy. Do not copy upstream runtime/templates into project-authored files or replace the root license. Third-party notices supplement existing headers.

After an authorized `trellis update`, inspect the actual version, template hashes and relevant backup read-only. Recheck this directory, mainline, both task manifests, tool/profile paths and actual agent loading; merge only project-owned changes. Never restore policy into upstream workflow/runtime or add protected paths to `update.skip`. The initialization baseline is 0.6.17; no update, backup recovery or post-update execution is implied by writing this policy.
