# Design: cooperative remote upload-root contract

Status: approved document/specification execution plan, 2026-10-22. User explicitly selected A, then replied “批准” to the completed final summary. This task delivers synchronized design/specification documents, not product code or new runtime experiments. [PRD](prd.md) owns requirements; [mechanism research](research/primary-sources.md) distinguishes source inspection from measured evidence.

## Boundary and trust model (R1–R2)

The upload root is a canonical path in the verified target's SFTP namespace, under a locally authorized dedicated upload directory. It is a logical directory scope under a stable cooperative hierarchy, not an inode capability or an OS-enforced jail. The remote account/server is expected to honor SFTP operations and the account's other programs must not rename, replace, symlink or independently alter operation-owned directories/files during their live operation and cleanup. Distinct SSHX operations use separate owned names. Other writers may cause final-name collisions; no-overwrite remains mandatory.

The user selected this environment assumption explicitly with A. Future users must authorize cooperative scope for their actual target/root through existing trusted local policy/consent; a plugin, manifest or server report cannot create that authorization. No new persistence subsystem or user-interface implementation is part of this task.

| Principal / condition | Protected property or limit |
| --- | --- |
| Trusted local Client/user policy | Select verified target, SFTP namespace/root and cooperative policy; never infer it from installation or clipboard action alone |
| Official/community plugin | Same narrow broker interface and B/grants; cannot select another root, upgrade policy, get credentials or bypass no-clobber/cancel/input checks |
| Other local users | Existing IPC/peer and local resource protections remain unchanged |
| Cooperative remote server/account | SFTP result and stable hierarchy are operating assumptions; local validation still rejects malformed/inconsistent observations |
| Another SSHX upload / final-name collision | Separate operation ownership; a pre-existing final file is never replaced, even when content matches |
| Same-account process replacing ancestors/temp or hostile server | Outside this mode's enforced guarantee; local metadata and random names cannot make this safe. Observed violations stop effects; undetected interference is not claimed prevented |
| Unknown target/config, namespace, mapping or context | Existing independent refusals remain; A does not supply target truth or Shell readiness |

Scope applies through cleanup, not only until the first OPEN. A caller requiring protection despite untrusted same-account replacement has no supported mode here. Basic login remains independent of upload availability.

## Proposed contract changes (R2–R3)

Use the existing `CapabilityGrant.path_scope` with `policy: "cooperative"` for the first documented upload mode. This is a proposed product contract; the Phase 0 harness has no full v1 compatibility promise. Null path scope cannot authorize path effects. A valid operation without an authorized cooperative policy, or requiring a stronger guarantee, is refused as `PATH_DENIED`; never coerce it into cooperative mode. Structurally malformed request fields retain the existing `-32602` validation behavior before authorization; this is distinct from a valid request whose local policy grants no upload scope. No dormant enforced-mode implementation or provider interface is introduced.

Core's existing `issuer: local_user_policy` authority binds verified target/profile/config generation, namespace, canonical root and policy into grant scope and `scope_digest`. Consent/policy changes, target/config or namespace/root changes invalidate affected grants/operations; a plugin cannot refresh or broaden them. Existing immutable B, deadlines, quotas, per-step effect admission and revocation still apply. An operator statement is a trust decision, not evidence of atomic filesystem confinement.

Server-confirmed path normalization and component-aware containment checks use the remote namespace's semantics, never local `Path` APIs or a raw string prefix. Filename policy allows a validated basename, not caller-supplied traversal or nested path construction. Check available ancestor/type/symlink/access observations before creation, final publication and cleanup. These observations detect known inconsistency; they do not establish unique directory identity or exclude replacement/ABA between requests. Reject unsupported filename/namespace semantics instead of guessing.

## Upload and cleanup flow (R3–R4)

1. Require explicit cooperative policy, verified target/config/namespace/root, current B/grants and a suitable regular-file commit capability before admitting data creation. Version advertisement alone is insufficient. Unsupported capability returns `COMMIT_UNSUPPORTED`; main login is unaffected.
2. Under the dedicated root, exclusively create an operation-owned private directory/temp regular file, record exact resource paths and acknowledged creation in the Core-owned ledger, and verify the expected type/access result. A unique name or 0700 is collision/cross-user protection, not protection against the same account. Existing/symlinked resources are not adopted.
3. Stream through the returned file handle with bounded chunks, actual byte counts, grant/quota/deadline/cancel checks, then require successful close. Before final publication revalidate the cooperative observations and serialized effect admission. Do not infer a durable object identity from portable SFTP attributes.
4. Intended future publication uses verified `hardlink@openssh.com` version 1 from the completed regular temp to a new final basename in the same operation directory/filesystem. The final result path includes that operation directory; no later directory rename is required. Do not use baseline/POSIX rename as an implicit fallback. Actual eligibility remains unimplemented/unverified in this task; filesystem failure refuses the commit.
5. A confirmed successful link commits the final user file. Separate removal of the known temp link is cleanup, not a rollback prerequisite. Cleanup failure leaves committed upload plus `cleanup_pending: true`; never remove the final name to compensate. A lost/ambiguous publication response leaves `upload_state: indeterminate` and `final_commit: "indeterminate"`, with no insertion or automatic re-publication.
6. Cleanup may remove only exact acknowledged operation-owned temporary resources while live target/namespace/cooperative continuity and ownership evidence remain adequate. Core alone may perform narrowly bounded cleanup after cancellation/revocation using the pre-existing owned-resource ledger; this is no renewed plugin grant and permits no new upload/publication. Before each cleanup path effect recheck the exact ledger resource, verified target/namespace and cooperative continuity. Changed/missing/inconsistent evidence, unknown creation/commit outcome, transport loss or new Core/generation stops automatic path-based deletion and records pending cleanup. Do not recursively sweep, follow replacement symlinks, adopt names after reconnect, or remove final user data. A retained directory containing a final file is not disposable temp.
7. Result delivery/insertion keeps existing independent rules: confirmed final result, verified Shell mapping and fresh receiver context/lease are required. Cooperative policy does not permit a guessed Shell path, temp path insertion, stale delivery or Enter.

The selected publication candidate is an explicit implementation requirement for later MVP work, not a claim about unchanged Phase 0 baseline-rename code. Root trust choice and verified no-clobber publication are separate gates.

## Result/error matrix

| Condition | Contract |
| --- | --- |
| Structurally malformed request | Existing `-32602`; no admission/state consumption |
| No authorized cooperative path policy; stronger guarantee requested; observed symlink/traversal/root inconsistency | `PATH_DENIED`; no newly admitted transfer/input effect |
| Unverified/changed auxiliary target or configuration | Existing `TARGET_UNVERIFIED` / `CONFIG_CHANGED`; no retarget |
| No SFTP / extra interactive auth | Existing `NO_SFTP` / `AUTH_REQUIRED`; basic login unchanged |
| Missing/unverified commit primitive, unsupported hardlink version/filesystem or explicit capability failure | `COMMIT_UNSUPPORTED`; no final success, cleanup only demonstrably owned temp |
| Confirmed final destination collision | `NAME_EXISTS`; existing file unchanged; same key does not pick another name |
| Ambiguous remote failure or lost commit response | Indeterminate result; do not label every generic SFTP failure a collision/unsupported capability; no automatic replay or insertion |
| Commit confirmed; temp cleanup fails | Upload committed, final path retained, `cleanup_pending: true`; final user file is preserved |
| Ownership/continuity lost during cleanup | Preserve uncertain remote resources; report pending cleanup rather than guessing/removing another object |
| Cancel/revoke before an effect is dispatched | No newly admitted effects; apply existing owned cleanup exception only to demonstrably owned resources |
| Cancel/revoke after publication was dispatched | Record confirmed or indeterminate external outcome; cancellation is not rollback and never deletes a final file |

## Compatibility, scope and traceability (R4–R5)

No released v1 API consumer exists; only proposed documents change. Keep the existing library/lockfile and one backend B. Do not rewrite archived reports, promote historical `phase0_ready: false`, or call unexecuted hardlink tests passed. Existing dependencies/platform/readiness gaps remain. Phase 1 planning may use the accepted cooperative contract once this task is delivered, but runtime/release support still requires actual implementation and evidence.

One coherent task is appropriate: root meaning, grant policy, publication/cleanup and documented promises share the same acceptance. A parent/child split would leave the contract inconsistent. Future runtime implementation is outside this deliverable and can become its own task only when selected.

Affected existing project files after execution approval are exactly:

- `docs/protocol.md`: authoritative root/policy/publication/cleanup contracts and error semantics.
- `docs/adr/0003-transfer-backend.md`: user decision A, pinned rename caveat, future publication selection and preserved general-root no-go.
- `docs/image-paste.md`: align sequence, consent, admission, final path and independent cleanup/insertion outcomes.
- `docs/architecture.md`: concise trust-scope link and proposed-versus-measured status.
- `docs/platform-matrix.md`: separate cooperative policy, commit capability and unverified server/filesystem support from the historical measured table.
- `docs/phase-0.md`: preserve historical results; state current scoped product decision and remaining runtime gates.
- `.trellis/spec/trellis-plus/sshx-contracts.md`: reusable seven-part executable handoff for proposed policy, checks, error/case matrix and future tests, pointing to authoritative protocol details.
- `.trellis/mainline.md`: scope authorization, actual acceptance/check state and Phase 1 planning versus runtime-readiness dependency.

Task planning/research/context/check records are also main-owned project data. Preserve canonical source, archived reports, probes, lockfile, wrappers, README, notices, generic specs and Trellis runtime/templates. Rollback of document edits uses individually reviewed diffs; no history reset, broad cleanup or automatic Git operation.
