# Independent review methods and evidence

Date: 2026-10-22. Dispatched `trellis-check` role, no recursive delegation. Explicit active task is `.trellis/tasks/10-22-sshx-upload-root-boundary`. Main owns lifecycle/task/spec/mainline/Git; this reviewer changed only the protocol's local result-shape reference and its own check artifacts. Other worker changes were preserved.

## Complete context loading

Read check.jsonl (21 entries), the saved native hook output `/tmp/hook_outputs/01a163f6-02b4-7752-849a-f9c782d7e88f/9e130225-faf9-4557-baf3-9befc25492ea.txt` in bounded ranges through its end, and all omitted entries directly. The native payload contained the injection marker but also explicit omission markers. Those markers were not treated as loaded content. A combined output limit also required direct recovery/re-reading; full loading, not only registration, is the evidence.

The complete registered input set reviewed was:

1. `.trellis/spec/trellis-plus/index.md`
2. `.trellis/spec/trellis-plus/development-principles.md`
3. `.trellis/spec/trellis-plus/sshx-contracts.md`
4. `.trellis/spec/trellis-plus/validation.md`
5. `.trellis/spec/trellis-plus/continuity.md`
6. `.trellis/spec/trellis-plus/commit-policy.md`
7. `.trellis/spec/trellis-plus/sshx-design.txt`
8. `.trellis/mainline.md`
9. `docs/protocol.md`
10. `docs/adr/0003-transfer-backend.md`
11. `docs/phase-0.md`
12. `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check-report.md`
13. `.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/acceptance-map.md`
14. `.trellis/tasks/10-22-sshx-upload-root-boundary/research/authority-evidence.md`
15. `docs/architecture.md`
16. `docs/image-paste.md`
17. `docs/platform-matrix.md`
18. `.trellis/tasks/10-22-sshx-upload-root-boundary/research/primary-sources.md`
19. `.trellis/tasks/10-22-sshx-upload-root-boundary/research/implementation-evidence.md`
20. `.trellis/tasks/10-22-sshx-upload-root-boundary/research/execution-ledger.md`
21. `.trellis/tasks/10-22-sshx-upload-root-boundary/research/acceptance-map.md`

Also read PRD, design and implement completely, task.json and planning-validation. The 54,122-byte pre-correction protocol was read using lines 1–180, an explicit 75–140 recovery, and 181–320 through the actual end (236 lines); its only later reviewer changes are the two fully inspected result-shape lines. Shared contracts were additionally read directly in full. Read `common`, its engineering and agent-system security references, and the local trellis-check instructions. No task pointer, loader, protected runtime or configuration was changed.

## Contract and source review

Reviewed full final documents and their actual diffs, including main's two owned targets, against R1–R5 / AC1–AC5. Traced local policy/role/grant validation to path admission, digest/B binding, streaming/close/publication, known final versus indeterminate outcome, cancellation, ledger-only cleanup and independent mapping/context/input admission. Confirmed there is no self-grant, raw executor/credential, fallback backend, implicit stronger-to-cooperative downgrade, replacement-path deletion or final-file rollback promise. Observed path inconsistency stops effects, while undetected same-account mutation remains an explicit excluded guarantee.

Independently opened the pinned [OpenSSH protocol](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/PROTOCOL) §4.5 and [sftp-server.c](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/sftp-server.c) process_rename/process_extended_hardlink: the extension is version 1, its handler calls link directly, and baseline regular-file rename has a selected-error stat/rename fallback. This corroborates the documentary candidate and caveat, not Debian-binary auditing or runtime behavior. Existing sftp.rs still calls raw.rename; the archived report's other `hardlink` case is a local package-archive rejection, not SFTP publication coverage.

The existing Operation.result shorthand conflicted with its detailed upload result. The reviewer mechanically made the former reference the named document alias `UploadResult`, preserving nullable/indeterminate semantics and all existing detailed fields. No new wire shape was introduced. Main was informed and confirmed scope before the correction.

## Checks actually executed

- Read and inspected `/tmp/sshx-upload-root-integrity.py` before execution. Its checks cover every exact dirty/new path, Markdown destinations and heading fragments, fenced JSON parsing, task JSON/JSONL targets and unique contexts, whitespace/UTF-8, source/notice hashes, branch/HEAD/index and all baseline fingerprints. Manual field/contract review complements the script; parsing alone is not schema verification.
- Initial independent checkpoint passed: 20 candidates (8 changed originals + 12 new task files), 17 Markdown files, 107 local links including 18 heading fragments, 1 JSON file, 42 JSONL records and 5 embedded JSON examples. All 168 baseline files remain; other 160 match. New report files were subsequently included in the final run indexed by [machine evidence](check-integrity.json).
- `python3 .trellis/scripts/task.py validate .trellis/tasks/10-22-sshx-upload-root-boundary`: pass, 21 implement and 21 check entries. Known per-file protocol injection warning persists and was handled by complete direct reads rather than a configuration change.
- `python3 .trellis/scripts/task.py list-context .trellis/tasks/10-22-sshx-upload-root-boundary`: exact role entries inspected; no unknown/missing/duplicate inputs.
- `python3 .trellis/scripts/get_context.py --mode packages`: single repository, backend/frontend/trellis-plus discovery; generic scaffolds preserved.
- `git diff --check`, exact eight-file diff/status inventory and empty staged inventory: pass. Frozen result fingerprints in machine evidence bind this report to corrected documents; all preserved baseline fingerprints were independently compared again after correction.
- JSON example/schema review: four protocol bodies are independent complete JSON-RPC examples with string IDs and coherent request/error shapes; unknown ping stays unknown and its compact body is 54 UTF-8 bytes. The runbook evidence example remains `not-run` with null environment/empty observations. No example creates a grant or asserts a new runtime pass.
- Mermaid sequence and process diagram were reviewed as text only. No application lint/typechecker, Cargo build/test, native probe, Docker/service, SSH/SFTP, clipboard, browser or mobile run was performed or claimed for this docs-only change.

The formal result is [check-report](../check-report.md). Main may now converge acceptance/checklist/mainline and classify the review gate, then prepare the separately authorized Git proposal. No additional reviewer edits remain.
