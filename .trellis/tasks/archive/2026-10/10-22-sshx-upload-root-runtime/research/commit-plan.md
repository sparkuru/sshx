# Proposed local commit, merge and archive batch

Prepared 2026-10-23. **Authorized on 2026-10-23 by the user’s “接受”.** Technical acceptance, exact teardown and the [human-required disposition](review-gate.md) are accepted. Execution outcomes are recorded after they occur.

## 1. Work commit

Message: `feat: verify cooperative upload-root runtime`

Include exactly these 43 individual paths, after final status/diff/source and credential checks. All are task-owned; there are no unrecognized dirty paths in the inspected inventory.

- `.trellis/mainline.md`
- `.trellis/spec/trellis-plus/development.md`
- `.trellis/spec/trellis-plus/sshx-contracts.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/check-report.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/check.jsonl`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/design.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/implement.jsonl`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/implement.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/prd.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/baseline.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/check-core.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/check-evidence.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/check-upload.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/cleanup.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/cleanup.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/commit-plan.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/core-handoff.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/core-run-1.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/core-verified.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/execution-ledger.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/final-validation.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/fixtures.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/implementation-evidence.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/planning-evidence.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/review-focus.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/review-gate.md`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/upload-final.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/upload-handoff.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/upload-run-1.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/research/upload-verified.json`
- `.trellis/tasks/10-22-sshx-upload-root-runtime/task.json`
- `docs/adr/0003-transfer-backend.md`
- `docs/architecture.md`
- `docs/image-paste.md`
- `docs/phase-0.md`
- `docs/platform-matrix.md`
- `docs/protocol.md`
- `probes/phase0/harness/src/core.rs`
- `probes/phase0/harness/src/main.rs`
- `probes/phase0/harness/src/sftp.rs`
- `probes/phase0/harness/src/suite.rs`
- `probes/phase0/harness/src/suite/upload_cases.rs`
- `probes/phase0/harness/src/upload.rs`

This includes preserved failed/intermediate reports with explicit supersession, final independent evidence and bounded source/spec changes. Do not include .devhome, /tmp logs, ignored platform state or generated credentials.

## 2. Local integration and archive commit

After the work commit succeeds, switch to local main and fast-forward only from upload-root-runtime. Do not push, rebase, amend or force. If main has diverged, stop to assess rather than widening this batch.

Use the supported single-task `task.py archive sshx-upload-root-runtime --no-commit` after merge verification. Expected destination: `.trellis/tasks/archive/2026-10/10-22-sshx-upload-root-runtime/`. Do not bypass feature-branch validation. Preserve source-bound historical machine report bytes.

The archive batch removes each of the 28 task paths listed above from the active directory and adds its corresponding path under the archive destination. Update only task metadata/manifests and local document references required by the move. The exact old/new path mapping is the literal prefix replacement from `.trellis/tasks/10-22-sshx-upload-root-runtime/` to `.trellis/tasks/archive/2026-10/10-22-sshx-upload-root-runtime/`; stage each file individually, never the archive subtree.

External paths eligible for reference/status reconciliation (only when actually affected):

- `.trellis/mainline.md`
- `.trellis/spec/trellis-plus/development.md`
- `.trellis/spec/trellis-plus/sshx-contracts.md`
- `docs/adr/0003-transfer-backend.md`
- `docs/architecture.md`
- `docs/image-paste.md`
- `docs/phase-0.md`
- `docs/platform-matrix.md`
- `docs/protocol.md`

Archive message:

~~~text
chore(task): archive cooperative upload-root runtime verification

Archive sshx-upload-root-runtime after bounded independent verification:
16 unit tests, 13 focused groups and 43 headless Core cases passed;
one desktop case excluded and exact fixtures cleaned.
Preserve cooperative-only scope, failed checkpoints and general-root no-go.

Co-authored-by: OpenAI Codex <codex@openai.com>
~~~

Verify completed metadata/date, cleared current pointer, actual archived contexts/links, merged work ancestry and exactly one matching trailer. No other task is archived.

## 3. Session evidence commit

Message: `chore: record cooperative upload-root runtime session`

Use supported `add_session.py --no-commit` with the actual work OID only in its commit table. Record the actual archive OID and observed cleanup/limitations in the session prose/mainline. Exact eligible paths:

- `.trellis/workspace/wkyuu/journal-1.md`
- `.trellis/workspace/wkyuu/index.md`
- `.trellis/mainline.md`

No second task attribution trailer on the work or journal commit. Keep any unexpected path outside this batch and report it.

## Approval requested

One response may accept the bounded security/deletion result and authorize these three local commits, fast-forward merge, single-task archive and session record. No remote push, further experiment, wider MVP activation or automatic follow-up task is included.
