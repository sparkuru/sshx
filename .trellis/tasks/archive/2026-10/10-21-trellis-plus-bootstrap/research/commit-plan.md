# Proposed Initialization Commit and Closure Batch

Status: **authorized on 2026-10-21** by the user's approval of the presented four-commit batch and two task archives. Checks and the human gate passed. Execution begins with the explicit work-file list below; later archive/journal evidence records actual outcomes. Do not push or amend.

## 1. Work commit

Subject: `docs: initialize SSHX project policies and design mainline`

Body: retain the supplied SSHX design, establish original project policies and guided mainline, collect matching Trellis 0.6.17 license, document bootstrap supersession, and record context/integrity checks. Product implementation and native runtime tests remain outside this initialization. No task attribution trailer on the work commit.

Explicit candidate files, each classified as shared project data or identified exact third-party notice:

```text
.trellis/mainline.md
.trellis/spec/trellis-plus/index.md
.trellis/spec/trellis-plus/development-principles.md
.trellis/spec/trellis-plus/sshx-contracts.md
.trellis/spec/trellis-plus/development.md
.trellis/spec/trellis-plus/validation.md
.trellis/spec/trellis-plus/commit-policy.md
.trellis/spec/trellis-plus/continuity.md
.trellis/spec/trellis-plus/sshx-design.txt
third_party/index.md
third_party/trellis-0.6.17-LICENSE
.trellis/tasks/00-bootstrap-guidelines/prd.md
.trellis/tasks/00-bootstrap-guidelines/task.json
.trellis/tasks/10-21-trellis-plus-bootstrap/task.json
.trellis/tasks/10-21-trellis-plus-bootstrap/prd.md
.trellis/tasks/10-21-trellis-plus-bootstrap/design.md
.trellis/tasks/10-21-trellis-plus-bootstrap/implement.md
.trellis/tasks/10-21-trellis-plus-bootstrap/implement.jsonl
.trellis/tasks/10-21-trellis-plus-bootstrap/check.jsonl
.trellis/tasks/10-21-trellis-plus-bootstrap/check-report.md
.trellis/tasks/10-21-trellis-plus-bootstrap/research/discovery.md
.trellis/tasks/10-21-trellis-plus-bootstrap/research/source-provenance.md
.trellis/tasks/10-21-trellis-plus-bootstrap/research/check-review.md
.trellis/tasks/10-21-trellis-plus-bootstrap/research/commit-plan.md
```

Only existing reviewed files are eligible; if the check worker does not create `check-review.md`, omit it rather than manufacture review evidence. Reconcile actual filenames before staging. The original generic bootstrap PRD text is preserved; new disposition is task data and existing upstream material is covered by the retained notice, not relabeled project source.

## 2. Generic bootstrap archive commit

After the work commit and required review/authorization, inspect active/archive/history state and use:

```sh
python3 .trellis/scripts/task.py archive 00-bootstrap-guidelines --no-commit --skip-branch-validation
```

This setup task was never PR-backed; the supported branch-validation exception is justified by that fact. Do not run the command as an authorization probe. Confirm successful move/status/date and no prior attributed archive before creating this commit.

Subject: `chore(task): archive 00-bootstrap-guidelines`

Body: close the generic setup by verified SSHX policy supersession, preserve unchecked example/template history, and cite work-commit/check evidence and absent application runtime. Append exactly once:

```text
Co-authored-by: OpenAI Codex <codex@openai.com>
```

Explicit candidates after a successful move:

```text
.trellis/tasks/00-bootstrap-guidelines/prd.md                 (tracked source deletion)
.trellis/tasks/00-bootstrap-guidelines/task.json              (tracked source deletion)
.trellis/tasks/archive/2026-10/00-bootstrap-guidelines/prd.md
.trellis/tasks/archive/2026-10/00-bootstrap-guidelines/task.json
.trellis/mainline.md
.trellis/tasks/10-21-trellis-plus-bootstrap/task.json
.trellis/tasks/10-21-trellis-plus-bootstrap/check-report.md
.trellis/tasks/10-21-trellis-plus-bootstrap/implement.md
```

Include only actual bookkeeping differences; use explicit individual paths for source deletions and destination files. No entire archive-directory staging. Record generic closure evidence without falsely closing this task first.

## 3. Trellis Plus bootstrap archive commit

Inspect the actual state, finish acceptance, update source references and use:

```sh
python3 .trellis/scripts/task.py archive 10-21-trellis-plus-bootstrap --no-commit --skip-branch-validation
```

This is also a docs-only non-PR task; do not invent feature-branch metadata. Installed archive performs a move, not a JSONL/link rewrite. Reconcile project-owned self-references from the old prefix to the actual returned archive path, update mainline and bootstrap metadata links, then validate the archived context and links before staging.

Subject: `chore(task): archive 10-21-trellis-plus-bootstrap`

Body: completed design retention, shared policy/mainline/notices, both 13-entry role contexts with observed loading, source/notice/protected-file checks and generic bootstrap closure. Retain automatic future-task loading and unexecuted update/native-runtime limitations. Cite work-commit evidence and append exactly one Codex trailer as in step 2.

For each individual current-task file listed in step 1, stage its tracked source deletion and the corresponding individual file at `.trellis/tasks/archive/2026-10/10-21-trellis-plus-bootstrap/`. Additional eligible bookkeeping paths are only:

```text
.trellis/mainline.md
.trellis/tasks/archive/2026-10/00-bootstrap-guidelines/task.json
.trellis/tasks/archive/2026-10/00-bootstrap-guidelines/prd.md
```

The displayed archive month is the current expected value; verify the actual command result/date instead of hard-coding an assumed destination if execution occurs later. Verify active task list, completed status/date, move, changed-path inventory and resulting exact-once trailer. A failed move gets no archive commit. If the move succeeds but the commit fails, inspect Git evidence and resume only the pending commit.

## 4. Separate journal commit

After work and both successful archive commits, use `add_session.py --no-commit` with the work-commit reference, concise actual changes/tests/limits and an idempotency key. Inspect its actual output before staging. Subject: `chore: record initialization journal`. No task attribution trailer.

Expected explicit candidates:

```text
.trellis/workspace/wkyuu/journal-1.md
.trellis/workspace/wkyuu/index.md
```

Preserve the existing seven-line journal baseline and any new user entries; if rotation or parallel work changes the candidate set, inspect and reconcile rather than collect the whole workspace. Avoid journal-triggered auto-staging of current task data.

## Excluded original/local material

Existing `.gitattributes`, `AGENTS.md`, `.trellis/workflow.md`, `.trellis/scripts/**`, `.trellis/agents/**`, config/version/hash metadata, generic backend/frontend/guides scaffolds, workspace root index, all personal `.codex/`/`.agents/` files and runtime pointers are excluded. They were not created by this task. The original moved temporary design is neither changed nor staged; the exact requested repository-local copy is included.

The protected Trellis installation remains untracked as before. This batch does not collect it into an initial runtime distribution; any separate decision to track protected material requires its own source/license-aware path review. That boundary does not authorize a broad add.

Before each commit, inspect complete actual candidates and staged diff; run `git diff --check` and `git diff --cached --check`, use explicit paths only and preserve unrelated staged work. Use temporary message files and `git commit -F`; archive messages receive the exact task trailer once, ordinary/journal messages do not. Keep Git identity unchanged.

## Execution bookkeeping clarification

The approved four-commit order and file ownership boundary are unchanged. The final journal commit also records the actual current-task archive hash and post-commit verification in `.trellis/mainline.md` and this archived task's `check-report.md`, `prd.md`, `implement.md` and `task.json`. These files were already in the approved work/archive path set. Recording that evidence after the archive commit avoids a self-referential hash or an extra fifth commit. No new unapproved file or effect is collected.
