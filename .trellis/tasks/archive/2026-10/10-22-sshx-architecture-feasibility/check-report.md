# Independent architecture check

Date: 2026-10-22. Role: dispatched `trellis-check`, full-scope Phase 2.2 review. Scope is documentation only. No recursive agent dispatch, staging, commit, archive, application implementation or Phase 0 execution occurred.

## Result

The architecture package is submit-ready within its approved documentation scope after the corrections below. No unresolved material source/contract inconsistency remains. This is independent document verification, not proof that a proposed API, platform adapter, transfer backend or recovery strategy works. Main session owns the final human-review classification and lifecycle/commit authority.

Recommended review category: `human-not-needed`, because the approved deliverable is a reviewable design with deterministic document checks and source coverage; required native experiments are explicitly future work, rather than skipped acceptance for an implemented product. Git commit/archive authority is separate and is not granted by this report.

## Context actually loaded

The native hook marker was present. I opened the saved output at `/tmp/hook_outputs/01a161c4-71e7-79e3-9dfa-5100af402d64/39d29eb5-b36d-43f0-a67c-97cf9144a83a.txt`; the initial tool display was itself truncated. The saved payload also explicitly omits implementation evidence and all three planning artifacts at its total-context limit. I therefore used complete direct child-side reads of the manifest, every referenced file and planning artifacts; hook success was not treated as complete context.

Exact thirteen check inputs read:

- `.trellis/spec/trellis-plus/index.md`
- `.trellis/spec/trellis-plus/development-principles.md`
- `.trellis/spec/trellis-plus/sshx-contracts.md`
- `.trellis/spec/trellis-plus/development.md`
- `.trellis/spec/trellis-plus/validation.md`
- `.trellis/spec/trellis-plus/commit-policy.md`
- `.trellis/spec/trellis-plus/continuity.md`
- `.trellis/spec/trellis-plus/sshx-design.txt`
- `.trellis/mainline.md`
- `.trellis/spec/guides/cross-layer-thinking-guide.md`
- `.trellis/tasks/10-22-sshx-architecture-feasibility/research/source-map.md`
- `.trellis/tasks/10-22-sshx-architecture-feasibility/research/primary-sources.md`
- `.trellis/tasks/10-22-sshx-architecture-feasibility/research/implementation-evidence.md`

Also read `check.jsonl`, `prd.md`, `design.md`, `implement.md`, task metadata and both context inventories; reviewed the entire changed/untracked inventory, all eleven `docs/` outputs and the three main-session shared changes. Current shared index includes the observed injection-limit/fallback correction. Current shared contracts include the main-authorized counter/metadata corrections below. Package discovery reports a single repository and backend/frontend/trellis-plus layers; shared policy supersedes the generic scaffolds. Common, Trellis start/check skills and the common engineering/security references were read; main-session dispatch/lifecycle instructions were not recursively followed.

## Findings fixed

1. `docs/protocol.md`: the binding introduction required `grant_ids` on every operation-scoped request, while the complete method tables omit them on several calls. Kept the tables authoritative: explicit grant IDs appear only where listed, and Core still resolves/checks the instance's active, unexpired, unrevoked grants for grant-requiring calls. Ownership-only methods and bootstrap/registration keep their documented checks. No omitted field bypasses authorization.
2. `docs/protocol.md`: the introduction required full Session identity in every asynchronous response, contradicting compact chunk/snapshot/insertion result shapes. Operation/event projections carry binding identity; compact results correlate through the authenticated channel's pending request and immutable B. This changes no request/result fields and never permits retargeting.
3. `docs/protocol.md`: clipboard error shorthand could be read as different names from the exhaustive error taxonomy. Expanded it to the exact `CLIPBOARD_UNAVAILABLE`/`CLIPBOARD_DENIED` names.
4. `docs/protocol.md`, `docs/image-paste.md`, `docs/phase-0.md`: an arbitrary random trigger nonce cannot be rejected after its dedup record expires without an unbounded used-nonce set. Reported this technical decision to the main session before changing it. Main session explicitly authorized a per-live-Client monotone decimal-string `trigger_nonce`, separate from RPC IDs, with one retained high-water counter. A retained identical intent queries its record; an expired old counter gets `EXPIRED`; failed admission does not consume a counter; skipped counters are invalid; reconnect never automatically replays prior actions. E7 now requires expiry/exhaustion/reconnect cases. Same-boot bounded dedup remains the scope; no cross-crash exactly-once claim was added.
5. The same files: live-resource limits plus TTL did not explicitly bound historical record churn. Main session authorized total Session/Client record caps, a derived Attachment tombstone bound, bounded grant refresh/ticket ownership and one proposed Core metadata budget. Added `CORE_METADATA_BUDGET_MAX` (16 MiB, proposed and unmeasured) covering models/profiles/grants/tickets/dedup/retained records. Admission fails before allocation; required live/unexpired records cannot be evicted. E6 now requires churn tests without advancing retention deadlines. Main session separately synchronized the shared-contract summary.
6. `task.json`: the first direct formatting pass found a missing final newline, although JSON parsing passed. Reported it to the main session because metadata is outside reviewer write ownership; main session appended the newline. Final verification includes this correction.

## Findings not fixed / downstream limits

No unresolved material finding remains in this documentation task. The following are explicit downstream entry/selection gates, not passing runtime evidence: primary native environment/fixture selection, exact dependencies, executable probe harness, target/config freezing or verified master route, a safe no-overwrite backend on the advertised server set, clipboard allocation bounds, native ACL/PTY/ConPTY behavior, supported Shell encoders, and visual restoration strategy. If Phase 0 cannot demonstrate these invariants, it must report a blocker or restrict supported combinations; the documents do not waive them.

No application workspace, product lint/type/test command or Mermaid renderer exists in the reviewed inventory. `command -v mmdc` returned exit 1. Mermaid fences and process/sequence node/participant references were manually inspected; no render test is claimed. Browser/mobile checks are inapplicable to these native-design documents. No dependency was installed to manufacture a renderer or an application check.

## Acceptance and flow review

| Criterion | Independent evidence / conclusion |
| --- | --- |
| AC1 | Architecture trace covers I–XIII/R1–R7, fixed/default/proposed/unknown/superseded classes and owning artifact/verification; original source preserved. |
| AC2 | Matrix separates Windows/macOS/Linux X11/Wayland/headless, architectures, host/Shell/display and Linux/macOS/Windows/SFTP namespaces; all runtime cells unverified. |
| AC3 | Three process/sequence diagrams plus stream/invocation/lifecycle tables assign Core-owned PTY/children, per-Client writer, protected IPC and independent binary stdio passthrough; requested invocation classes individually covered. |
| AC4 | All seven models reviewed with ownership/transitions/cleanup; complete Client/plugin tables, version/framing/IDs/errors/events/limits and reserved later methods agree after corrections. Bootstrap roles cannot mint grants. |
| AC5 | Traced capture→sealed PNG→target-bound exclusive temp→confirmed no-clobber commit→mapped-path formatting→conditional insertion. Switch/detach/generation/revoke/disable/cancel never change B. Upload and input outcomes remain independent; no automatic repeat after partial/uncertain write. |
| AC6 | Six ADRs have status, alternatives, consequences, evidence, selection/reopening gates; Rust/system OpenSSH retained, no crate/backend/visual winner invented. |
| AC7 | E0–E7 identify fixtures, actions/command shapes, future harness I/O/barriers, pass/fail, teardown and evidence. Source XII.1–12 all mapped; History/complete attach/later integration cases explicitly deferred. |
| AC8 | Independent full-scope review and direct untracked checks passed after local/main-owned corrections; exact source/protected fingerprints checked. |

Specific cross-layer cases reviewed: target/helper/config drift rejects auxiliary effects rather than treating `ssh -G` as target proof; peer credentials separate other users without claiming same-user sandboxing; clipboard-read→upload→insert requires composed narrow grants; budgets precede image/native/blob allocation, Core hashes the exact sealed PNG and never rewrites it; transfer namespace and Shell mapping are distinct; a prior stat plus overwriting rename cannot satisfy commit. Session serialization orders cancel/revoke against effect admission; an already dispatched rename/write may commit. Lost event/ack delivery leads to bounded query/reconciliation, not new effects. A zero-byte refused insert may refresh authorization/context on the same stable intent/key; committed/partial/indeterminate input cannot repeat. Insert text must match validated formatting of the recorded mapped path, not arbitrary safe-looking plugin text. Result metadata is still available to the owning authenticated user after plugin/Attachment loss, under retention bounds.

I reopened all thirteen primary URLs from the research table and checked key request/error, helper, SFTP rename, native IPC, PNG/clipboard and Shell-marker claims against visible primary material. Microsoft pages again displayed authorization banners alongside article text; no sign-in/bypass occurred. Sources support the narrow documentary claims, not selected installed APIs or runtime capability. No upstream source was copied into the product or notices changed.

## Commands and verification

Commands run from repository root:

~~~text
python3 .trellis/scripts/get_context.py --mode packages
python3 .trellis/scripts/task.py validate .trellis/tasks/10-22-sshx-architecture-feasibility
python3 .trellis/scripts/task.py list-context .trellis/tasks/10-22-sshx-architecture-feasibility
git status --short --untracked-files=all
git diff -- .trellis/mainline.md .trellis/spec/trellis-plus/index.md .trellis/spec/trellis-plus/sshx-contracts.md
git diff --check
command -v mmdc
~~~

Additional one-shot Python standard-library checks read every task/output Markdown and all three changed shared files directly: parse task JSON and each JSONL line, validate input existence/uniqueness, parse JSON code examples, compute the UTF-8 frame length, resolve local Markdown paths/fragments, count table cells/fences, inspect new-file whitespace/final newlines, enumerate limit/error names and compare all 80 baseline SHA-256 values. The initial metadata newline failure was corrected and rerun; untracked checks do not rely on `git diff --check`.

Final measured counts/results are recorded after the final rerun below. Baseline comparison permits exactly `.trellis/mainline.md`, `.trellis/spec/trellis-plus/index.md` and `.trellis/spec/trellis-plus/sshx-contracts.md`; all other 77 tracked originals, including Trellis runtime, generic scaffolds, original notices and retained source, remain unchanged. Source remains 30,205 bytes, SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`.

Final rerun: PASS for 21 Markdown files (11 outputs, 7 task/report/research files and 3 changed shared files), 29 tables, 14 balanced fenced blocks, 5 parsed JSON examples and 80 local links/fragments. All 63 referenced limit symbols have normative definitions; all 30 domain error names are defined in the error-table name column and example errors use them. Seven models, all 12 source XII groups and the exact 54-byte compact frame body pass. Both manifests validate/list successfully with 12 implement / 13 check entries and no duplicate/missing inputs; task JSON parses and ends with a newline. All 80 baseline files exist, with exactly 77 unchanged and the three permitted project-data changes. `git diff --check` exits 0. Three Mermaid diagrams were inspected manually, without renderer execution. Final worktree inventory has only the three tracked changes and 21 expected untracked task/output files, no staged paths; task remains `in_progress`. One-shot measured details are also saved locally at `/tmp/sshx-architecture-check-summary.json`.

Lint: document formatting/link/schema checks pass; no product lint exists. TypeCheck: not applicable, no application code/workspace. Tests: document/contract consistency checks pass; application/native/SSH/SFTP/clipboard/browser/benchmark tests were not run and are not reported as passes.

## Main-session review gate

The main session read this full report, inspected the worker corrections and all eleven design outputs, verified the measured summary and both saved native-hook omission reports, and accepts AC1–AC8 for the approved documentation deliverable. Category: **human-not-needed**. This changes future design descriptions, not implemented authorization/security behavior; no required documentation check is blocked, and native experiments are a separately scoped future deliverable. Mermaid syntax/node/participant review was manual; no rendered-output or native-support claim is made.

Shared spec synchronization is complete: index points to the design package and records the observed injection-limit fallback; the contracts handoff includes effect binding, expired-trigger refusal and bounded retained metadata without duplicating normative constants. Final acceptance/status bookkeeping and the concrete commit plan are main-session-owned follow-ups to this report and receive focused integrity checks. The user subsequently granted permission for the concrete work/archive/journal batch on 2026-10-22; the review category itself supplies no such authority.

After preparing the concrete commit/archive proposal and checking AC1–AC8, the main session reran focused integrity checks: PASS for 22 Markdown files, 82 local links/fragments, 20 balanced fenced blocks, five parsed JSON examples, JSON/JSONL input paths, all twelve fault groups and exact equality between the 25 proposed work-commit files and the actual dirty inventory. Source size/hash and 77 unchanged baseline files passed; only the same three allowed shared files differ. No staged changes exist; status is still `in_progress` with no completed date. `task.py validate` passes at 12 implement / 13 check references and `git diff --check` exits 0. This final bookkeeping pass does not replace or broaden the independent design check above.

## Authorized archive verification

Work commit `9b483af56c319ef0aff21ac37a4536d17dcb46e6` succeeded with exactly the 25 approved files and no task trailer. The installed `archive --no-commit --skip-branch-validation` route then completed this documentation-only, never-PR-backed task on 2026-10-22; active task lookup now returns none (the tool's normal exit 1 for no selection). No active archive hook was configured. The eleven task files moved once to the actual archive directory; live JSONL paths and actual relative Markdown links were reconciled while historical commands/loading paths were retained.

Post-move checks pass at the actual archived path: completed status/date, 12 implement / 13 check references, 22 Markdown files and 81 local links/fragments, JSON/JSONL/formatting and `git diff --check`. Ten design outputs are byte-identical to the work commit; only architecture entry links changed. All 77 non-policy originals and the exact 30,205-byte source/hash remain unchanged. The archive attribution commit and session journal are separate subsequent actions, with their actual resulting hashes recorded in mainline after success.
