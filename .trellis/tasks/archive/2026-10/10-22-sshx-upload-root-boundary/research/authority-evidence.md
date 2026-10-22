# Upload-root planning evidence and decision inventory

Date: 2026-10-22. Read-only source/code review plus task/mainline planning edits only. No dependency retrieval, native execution, server configuration or new fixture. The clean starting Git HEAD is `a91878ecf9adc5a2b72f123e0bedc63e3e192428`; 168 tracked-file fingerprints were saved in `/tmp/sshx-upload-root-planning-baseline.json` before task creation.

## Confirmed facts

| Evidence | What it establishes | What it does not establish |
| --- | --- | --- |
| Canonical source V, lines 120–132 | One OpenSSH-backed SFTP route, independent basic login, explicit auxiliary-auth/target failures, SFTP/Shell namespaces distinguished and a dedicated upload subdirectory | Atomic ancestor capability or support against concurrent same-account path substitution |
| Canonical source IX, lines 210–216 | Threats and same-user boundaries must be stated; permissions, path/symlink behavior, quota and cleanup belong at the actual effect | A promise that filename metadata probes can confine a hostile remote account/server |
| `docs/protocol.md:68` | Current architecture requires rejection of unconstrained namespace/symlink races and demonstrably owned resources | Permission to change the contract without a user-owned scope decision |
| `docs/adr/0003-transfer-backend.md:28`–34 | A batch overwrite route was rejected; B remains only a cooperative Linux experiment; general MVP promotion is blocked | A passing general confinement result or permission to install a hidden remote helper |
| `probes/phase0/harness/src/sftp.rs:117`–171 | Actual exclusive leaf open, file-handle writes/close, bounded readback and path-based no-replace rename on SFTP pipes | Directory-handle-relative open/rename/removal rooted in a durable granted ancestor |
| Accepted Phase 0 report and source-bound evidence | Controlled no-clobber/target/cancel/reply-loss checks and precise no-go, with final exact cleanup | Universal server behavior, hostile-server truthfulness, other native platforms or released-product readiness |

Canonical source is immutable requirements input. The architecture protocol/ADRs are project-authored contracts that can be revised only through an explicit, documented product choice with corresponding observable promises. Accepting a negative Phase 0 deliverable did not itself make that choice.

## Distinctions to retain

- A trusted/cooperative remote-account assumption is an operating precondition. It is not proof from one consent prompt, 0700, exclusive leaf creation, random names, path normalization or pre/post lstat. A cooperative mode must name stable-ancestor/noninterference assumptions and refuse observed violations; it must not advertise enforced protection against competing same-account mutation.
- Trusted remote cooperation does not grant plugins raw execution, credentials, arbitrary target/path selection or arbitrary input. Existing local effect-site grants, binding, quotas, revocation, no-overwrite and input-context checks still apply. Remote reports/data stay untrusted at the local boundary.
- Stronger confinement is a different capability. Its root semantics, enforcing principal, namespace, evidence and limitations must be defined before claiming support. Server configuration, isolation or optional helper ideas require separate mechanism research; none is selected or presumed sufficient here.
- Root identity, namespace containment, final-leaf no-clobber, remote-data integrity and safe cleanup are distinct properties. File handles may bind writes while later path-based rename/cleanup still lacks ancestor authority. Losing ownership evidence must not authorize deletion of a replacement path.
- Basic OpenSSH login stays available independently when upload capability is refused. Returning an unsupported upload result is not a login/authentication failure.

## User-owned decision (resolved)

**Selected A:** on 2026-10-22 the user chose the explicit cooperative remote account/stable upload hierarchy for the first MVP upload contract. Enforced confinement against competing same-account path replacement is not promised.

Recommendation: the explicit cooperative no-Agent baseline, if its narrower promise is acceptable, fits the retained deployment goal. The alternative prioritizes stronger confinement and may restrict supported server environments or leave upload unavailable until an enforcing mechanism is justified. The user’s explicit A response selects the cooperative option; it is not an agent-selected default. Exact admission/error/cleanup mechanics follow in the design. The subsequent “批准” response now supplies final-plan document execution authority; no new runtime/Git authority follows.

## Current artifacts

PRD converged with R1–R5 and AC1–AC5; mainline points at this planning task. The deliverable is explicitly document/specification synchronization, with new runtime implementation deferred. Both role contexts register the relevant existing contracts/evidence rather than relying on recursive Markdown loading. The product boundary is chosen and `design.md` / `implement.md` are complete for final review. Both contexts now contain 18 inputs, including the pinned mechanism research and all affected result documents. The supported task start has since completed after explicit final-plan approval; document implementation is now in progress. Existing full-protocol injection limits still require complete bounded direct reads by a future dispatched role; no loader or protected configuration is modified.
