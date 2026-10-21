# SSHX Commit and Archive Policy

Read with the [review gate](validation.md#submit-ready-human-review-gate), [mainline](../../mainline.md) and task evidence before staging, committing, completing or archiving. Main session owns these actions. Reuse authorization already given for the exact action; initialization approval alone is insufficient for Git commits. Do not change Git author/committer identity.

## Explicit staging boundary

Inspect `git status --short`, unstaged/staged path inventories and the complete proposed candidate list. Classify each individual file before adding it:

| Class | Treatment |
| --- | --- |
| Original project-shared spec/mainline/task/journal data | Eligible only for the authorized change and commit plan |
| Exact third-party notices/inventory | Review version, source and exact text before inclusion; preserve existing headers |
| Ordinary user project files | Include only when the requested work authorizes them; pre-existing untracked does not imply ownership |
| Trellis runtime/templates/metadata/managed platform files | Read-only in this workflow; remove from candidate set and report the boundary |
| Personal platform settings, runtime pointers, `.env`/secrets | Keep local/ignored; never stage through Trellis Plus |

Use `git add -- <explicit individual classified paths>` only after the plan/authority is established. No broad directory collection, `git add .`, `git add -A`, forced adds or equivalent broad operations. Preserve the initial user files, generic spec scaffolds and unrelated dirty work. Verify `git diff --check`, inspect `git diff --cached --name-only` and the staged diff before committing. An initial repository with no commits requires direct untracked-content/fingerprint checks too.

Unknown retained third-party provenance is `license-notice-needed`, not permission to replace notices/root license or stage unresolved material as complete. Consult the [notice inventory](../../../third_party/index.md). Platform integration remains personal/local even if generated under the repository.

## Installed archive route

Trellis 0.6.17 provides:

```sh
python3 .trellis/scripts/task.py archive <task-name> --no-commit
```

For a documentation/bootstrap task that never had its own branch/PR, inspect its metadata and use the supported `--skip-branch-validation` only when that exception is justified. Do not bypass an actual feature task's missing merge evidence.

The command defaults to automatic commit and has no custom commit-message input. Inspection of `common/task_store.py` and `common/safe_commit.py` shows its automatic archive staging can collect the **whole `.trellis/tasks/archive` subtree**, plus affected task relationships; do not assume it stages only the named task. Always disable automatic commit for this project's explicit attribution/staging route.

Archival updates task status/completion date, moves the directory into `.trellis/tasks/archive/YYYY-MM/<task-name>/`, clears relevant session pointers and can update child/parent task metadata and run configured `after_archive` hooks. The initialization configuration has no active archive hook; re-inspect before future use. Identify actual changed files after each move. Ignored session pointers stay local; stage only individually reviewed archive/task bookkeeping files, never the archive subtree.

The 0.6.17 archive move does not rewrite repository-relative self-paths in task JSONL or Markdown links to the former task directory. After a successful move, reconcile only affected project-data references to the actual archive path, including archived manifests, mainline links and related task records. Preserve historical command/source descriptions. Run `task.py validate <actual-archive-path>` and resolve local links again before the explicit commit; a successful move alone does not establish usable archived context. Do not patch the runtime or rerun the move to repair references.

Journal recording supports `python3 .trellis/scripts/add_session.py --no-commit` with its actual required session arguments. Its default auto-commit can stage the current developer's journal/index and current task; keep it disabled and inspect actual paths. A separate journal commit is separate from task archival and receives no task attribution trailer.

Recheck installed help/implementation after updates before reusing this route. If neither supported message input nor `--no-commit` is available, report `archive-attribution-blocked`; do not patch protected runtime, invent flags or install a Git hook.

## Once per successfully archived Codex task

1. Read acceptance, validation, review decision, work commits, current/archived state and recent `git log --format=%B -n 20`. No history at bootstrap means no established message style. Confirm existing archive/commit evidence before retrying.
2. Prepare one archive completion message per Codex-assisted task, proportionate to its scope: requested outcome, delivered result, actual checks/limitations, task identity and work references when available. Preserve other legitimate authorship without inventing historical Codex contribution.
3. Include exactly one matching trailer, separated from the body by a blank line:

   ```text
   Co-authored-by: OpenAI Codex <codex@openai.com>
   ```

4. Present the concrete message and explicit file list under normal commit/archive authorization. Honor the human-review gate before the move or completion. Execute the supported single-task `archive --no-commit` route only when permitted.
5. Verify destination, status/date, active task list and relationship/pointer effects. Only a successfully archived task receives its attributed archive commit. Stage individually reviewed files and use `git commit -F <temporary-message-file>` after authorization, with the message file outside tracked paths.
6. Inspect `git show --format=fuller --name-only <archive-commit>` and `git show -s --format=%B <archive-commit>`: correct task/files, successful archive evidence and exactly one trailer. Update mainline with archive location and commit reference; a later evidence/journal-only commit does not repeat the trailer.

Ordinary implementation, fix and checkpoint commits do not acquire this task trailer. Small tasks still receive attribution at successful archival. Tasks authored by someone else are not retroactively attributed merely because Codex inspects them.

If a move succeeded but commit failed or was not authorized, inspect destination, pending files, attempt evidence and Git history. Verify the commit does not already exist; resume only the pending explicit commit. Do not repeat the move. If the state is uncertain, stop and report it. If an existing archive commit lacks the trailer, report missing attribution without amending history or making an empty attribution commit. Repeated finish-work must not create another archive/attribution.

## Initialization task closure

After the authorized initialization/application checks pass, inspect `00-bootstrap-guidelines` and any existing archive attempts/commits before deciding its disposition. Record how this shared policy replaces the generic Web/backend scaffold assumptions for SSHX, while keeping unfulfilled original checkboxes unchecked. No code exists to provide genuine application examples; do not fabricate them.

Substantive unfinished acceptance blocks closure unless delivered, demonstrably superseded, or explicitly transferred to an authorized linked follow-up. Missing active task with no pending attempt may be skipped; do not recreate it. Existing successful archive/commit is not repeated. Close the initialization task itself through the same normal route when ready.

Report policy delivery, on-disk archival and archive commit as separate outcomes. A blocked closure is not completed initialization wrap-up. Missing commit authorization does not prevent independent document/check work; prepare the concrete plan first and ask only for the remaining authority. Mainline records the actual resulting state, never a planned move/commit as completed.
