# 远端上传根目录安全边界

## Goal and deliverable

Deliver a consistent, testable **document/specification contract** for cooperative remote uploads so future MVP planning can use an explicit supported environment instead of the general-root guarantee that Phase 0 could not establish. Preserve independent OpenSSH login, effect-site checks, no implicit overwrite and safe owned-resource cleanup. This task does not implement or validate a new runtime capability.

## Background and authority

- On 2026-10-22 the user requested continuation, then replied “接受” to creating this task and entering planning. The user subsequently selected **A**: trust a cooperative remote account and its programs not to replace the upload hierarchy concurrently. Keep path checks, no-overwrite, revocation and cleanup; do not promise confinement against hostile same-account mutation. No Agent or forced server administration is introduced.
- A resolves the product boundary. The user subsequently replied “批准” on 2026-10-22 to the completed final PRD/design/implement summary, authorizing this exact document/specification execution scope. New runtime fixtures, Git/archive and automatic Phase 1 execution remain outside this approval.
- Phase 0 is accepted/archived: work `620ca22a49611e319daf6df9f9aa22113d428133`, archive `0e3dddf7f583c0041ef1d899f47ddf620090e111`, Session 3 commit `a91878ecf9adc5a2b72f123e0bedc63e3e192428`. Existing 14-unit/44-native checks and supplements are reused as historical evidence; exact cleanup completed and `phase0_ready: false` remains. The current task began from a clean working tree with 168 tracked-file fingerprints.
- Original source V/VII/IX/XIII in `.trellis/spec/trellis-plus/sshx-design.txt` requires one OpenSSH-backed transfer route, independent login, permissions at effects and owned cleanup. The canonical source remains unchanged. Its threat boundaries must be stated; the stronger project-authored architecture wording can be revised only explicitly.
- `docs/protocol.md:68` requires refusal of unconstrained path/namespace races; `docs/adr/0003-transfer-backend.md:32` records the general filename-based SFTP ancestor no-go. Exclusive leaf creation, 0700 and metadata observations do not establish atomic directory authority. The selected mode therefore states an operating precondition instead of claiming enforced confinement.
- [Version-pinned mechanism research](research/primary-sources.md) identifies an OpenSSH baseline rename filesystem fallback that prevents a universal no-clobber claim. The intended future regular-file commit candidate is verified hardlink publication plus separate temp removal within the existing backend B. Existing probes still use baseline rename; no new candidate tests are claimed.

## Requirements

| ID | Delivered document outcome |
| --- | --- |
| R1 | Define assets, root meaning and trusted principals; separate other local users, plugin broker authority, cooperative remote programs, hostile remote servers/accounts and accidental observable mutation. Logical SFTP root scope is not an OS jail or durable inode capability. |
| R2 | Record explicit A and its stable-ancestor/noninterference preconditions through cleanup. Use existing local policy/grant fields to authorize cooperative mode for actual target/namespace/root; operator consent or metadata is not an enforcement proof. Stronger confinement stays unsupported. |
| R3 | Specify valid-request admission/refusal separately from malformed input, independent login, verified target binding, no-overwrite commit, cancellation, indeterminate responses and safe cleanup. Preserve final user files; uncertain ownership never authorizes removing a replacement path or inserting an uncertain result. |
| R4 | Document the bounded option comparison and selected future publication candidate within B using primary and repository evidence. Require explicit capability verification/no implicit fallback, and give a finite future runtime test matrix without running or claiming it here. |
| R5 | Synchronize exactly the eight existing targets named in design plus task evidence/context/check records. Deliver a seven-part shared-spec handoff and independently checked documents, preserve source/probes/runtime/notices, and distinguish readiness to plan an MVP from implemented capability/release readiness. |

## Acceptance Criteria

- [x] AC1 (R1): The final authoritative documents and threat matrix consistently identify the trusted/cooperative root contract, protected effects, actors and excluded stronger guarantee, with source versus later decisions traceable.
- [x] AC2 (R2): The user explicitly selected A on 2026-10-22. Design records assumptions/limits, and no stronger confinement guarantee or implementation authority is inferred from that choice.
- [x] AC3 (R2–R3): Protocol, Image Paste and shared-spec fields/error/outcome/cleanup rules agree. Missing authorization refuses valid path effects; malformed request validation remains separate. Confirmed final publication survives temp-cleanup failure, and unknown ownership/commit outcome produces no guessed deletion/replay/insertion.
- [x] AC4 (R4): ADR, version-pinned research and prospective platform gates distinguish the selected future hardlink candidate from historical baseline-rename observations. Capability/collision/lost-reply/cleanup tests are concrete future requirements; no new native pass is fabricated.
- [x] AC5 (R5): Full-scope independent document review, task JSON/JSONL/context/link/format checks and baseline source/license/probe/runtime integrity pass. PRD/design/implement and real role contexts converge; no unrelated files, executable changes or false Phase 0/MVP-ready claim enter the result.

## Scope and decisions

Execution after final-plan approval changes only `docs/protocol.md`, ADR 3, `docs/image-paste.md`, `docs/architecture.md`, `docs/platform-matrix.md`, `docs/phase-0.md`, `.trellis/spec/trellis-plus/sshx-contracts.md`, `.trellis/mainline.md` and normal current-task artifacts. [Design](design.md) specifies the existing `path_scope.policy` / `scope_digest` / B / result fields, authority/error matrix and cleanup continuity; [implementation plan](implement.md) fixes file ownership, validation and role loading.

One coherent task is appropriate because these documents jointly define the same cross-layer acceptance. No parent/child split or provider framework is introduced. No blocking user-owned product decision remains; final-plan document execution is now explicitly approved.

## Out of scope and deferred evidence

Full MVP or probe code changes, Rust/Shell/Python changes, dependency/lockfile/toolchain changes, a new SSH stack, actual hardlink implementation/tests, remote Agent/Shell hooks, server isolation/administration, production or personal targets/credentials/displays, new Docker/native/SSH/SFTP/clipboard execution, Git staging/commit/archive and automatic later-phase tasks are excluded. Windows/macOS/Wayland and broader server/filesystem behavior remain unverified.

The future hardlink candidate requires real collision, unsupported version/filesystem/cross-device, lost-reply, temp-cleanup and effect-admission evidence before activation. Those technical unknowns are explicitly deferred to a separately authorized runtime task; the defined behavior remains unavailable/refused until verified. They do not block delivering honest documents and do not become passing results. General mutable-root confinement remains unsupported; original Phase 0 reports are immutable historical evidence.

## Delivered document acceptance

All AC1–AC5 pass for the approved document/specification scope. The [independent report](check-report.md) records complete loading, the single UploadResult consistency correction, five JSON examples, data/link/format checks and preserved baseline evidence. Main accepts that document result and records the review gate separately. Actual hardlink/runtime enforcement remains future/unavailable; no native pass, Phase 0 PASS or MVP release readiness is inferred. The subsequent “批准” response on 2026-10-22 authorizes the reviewed three local commits, fast-forward merge, archive and session record. The task stays `in_progress` until those actual lifecycle steps execute; no later runtime task is authorized.
