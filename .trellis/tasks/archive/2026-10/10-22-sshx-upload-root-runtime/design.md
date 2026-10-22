# Cooperative upload runtime design

## Boundaries and reuse

Extend the existing experimental Core upload path, not a second backend or product application. Keep system OpenSSH, strict generated host-key verification and locked russh-sftp 3.0.1. The SFTP wrapper retains extension values as well as names and calls its existing library's hardlink method. The historical backend-comparison experiment may retain baseline rename as explicitly historical comparison; cooperative Core uploads never fall back to it.

Primary ownership: `harness/src/sftp.rs` owns transport/capability observations and bounded reply-loss injection; `harness/src/core.rs` owns policy, operation authority and effects; `harness/src/suite.rs` owns integrated assertions. Small cohesive helper modules within that crate are allowed if they simplify these responsibilities. `main.rs`, protocol projections, plugin/image callers change only as necessary to remain coherent with this probe contract. No complete proposed-v1 API implementation is implied.

Use existing `hako`, `dev.sh` and fixture lifecycle. Up to two isolated generated servers allow wrong-target checks. No service startup during planning. No dependencies, host packages, wrapper replacement, desktop service or production config are planned. Essential fixture changes must stay within existing generated accounts, containers and temporary mounts.

## Policy and capability admission

Core holds explicit local fixture policy for the actual target/profile/config generation, SFTP namespace and canonical root, including cooperative mode. Do not infer policy from a plugin request or automatically grant on action invocation. Bind its canonical serialized values into scope_digest and the operation/grant authority; policy/root/namespace/config changes invalidate affected effects. Test missing policy without creating a transfer directory. Preserve structural validation, role/instance/grant and stale-binding checks before path policy errors. Existing probe error aliases must be mapped explicitly in the report, not called full v1 conformance.

Capability qualification is a separately identified, authorized **fixture preparation experiment**, using only synthetic resources under the approved root. Record server/image/filesystem, extension value 1, actual successful link/readback and collision preservation. This avoids treating extension advertisement as verification or allowing an unqualified user upload to perform its own speculative write. Core accepts only local run-scoped qualification evidence bound to that fixture/config/namespace/root; a plugin cannot supply it. Recheck extension version and live binding on the operation transport. Changed run/target/root or uncertain continuity invalidates evidence. No persistent production capability cache is introduced. If evidence cannot be bound adequately, refuse admission and report the gap.

This finite qualification is evidence for the controlled cooperative fixture, not a universal guarantee or detection of hostile filesystem replacement. Test unsupported advertisement/version by an explicitly labeled bounded harness fault; test actual cross-device rejection using the fixture's separate tmpfs. A generic SFTP FAILURE cannot by itself prove EXDEV or EEXIST. Record server-side fixture facts separately; do not let privileged fixture observations become runtime authorization.

## Operation flow and outcomes

1. Validate request and immutable B/grants/policy/verified capability. Normalize SFTP paths with component-aware containment; reject known symlink/type/namespace inconsistency. Never use a host Path or string prefix as remote containment.
2. Exclusively create private operation directory and regular temp; record exact acknowledged creations in a bounded operation ledger. Do not adopt an existing path. Revalidate before creation, each write/close, publication and cleanup; streaming does not hold the Core state lock.
3. After successful close and fresh admission, issue one hardlink to a new basename in the same directory. Mark dispatch independently of acknowledgement; effects admitted before cancel may complete and must be reported honestly.
4. Acknowledgement establishes confirmed final commit immediately. Readback verification and temp cleanup are separate outcomes; readback or cleanup failure cannot undo the final file. Final path stays inside the operation directory. Preserve existing bounded content verification before any insertion eligibility.
5. Lost/ambiguous link acknowledgement records indeterminate final commit, no replay and no insertion. Use read-only observation for evidence; matching bytes alone do not authorize adoption, deletion or a success result. Same-key retries return retained state without renaming/reuploading.

Use the documented final_commit/cleanup_pending semantics in probe projections and update affected consumers together. Keep existing independent mapping/context/receiver checks; use synthetic PTY input observation for no-insertion assertions, without claiming real Shell editing or clipboard coverage.

## Cleanup authority

The ledger records namespace, exact paths, acknowledged creation, operation/boot/generation and original live transport continuity. Cooperative stability remains a prerequisite; lstat and private mode are observations, not inode authority. Each path removal rechecks ledger, target/config/namespace, containment and available type/ancestor observations. Missing evidence, transport loss, replacement observation or generation/boot change leaves pending cleanup. No reconnect adoption, recursive sweep or compensation by deleting final data.

Cancellation/revocation/disable deny new upload/publication but may permit narrow cleanup of already-owned temporary resources with otherwise intact continuity. Cleanup authorization must therefore be distinct from the cancelled plugin effect gate. Remove an owned empty directory only after confirmed temp removal and no final/uncertain content; a directory containing final user data remains. A lost creation/removal acknowledgement is uncertainty, never permission to guess.

Fixture teardown is separate: after recording asserted retained/pending resources, the harness owner destroys the exact disposable container and generated credentials. This does not demonstrate that Core automatically cleaned pending resources.

## Faults, budgets and observability

Reuse deterministic barriers around actual effects. Match a lost hardlink reply by framed request ID and extension, robust to fragmented/coalesced I/O; the existing rename byte-pattern trigger is insufficient for extended requests. Record that the intended response was intercepted and independently observe the remote file. Injected failures must carry evidence_kind=fault_injection and the precise injection site; real filesystem failures remain distinct.

Keep existing bounded frame/data/deadline limits. Each case has a finite timeout, known owned children and a fresh output path. Evidence records case ID, environment/source identity, authority result, admitted/dispatched effects, final/temp presence and hashes where relevant, insertion bytes, cleanup outcome and limitations. No credentials or real terminal/clipboard data enter reports. Qualifying and independent check runs produce separate reports; nested cases are not added into misleading totals.

## Compatibility, documentation and rollback

Probe-only changes have no released API compatibility requirement. Update all actual callers when changing projections or barrier names. Do not silently preserve a legacy cooperative rename mode. Preserve archived source-bound reports byte-for-byte and label new source/version evidence separately. Synchronize current tense in protocol, ADR 3, Image Paste, architecture, platform matrix, runbook and shared contracts only to the degree actually demonstrated; retain the general-root no-go and historical phase0_ready:false.

If mandatory invariants fail, leave that capability unavailable, preserve diagnostics and report a negative result. No fallback or expanded platform/root promise. Rollback is limited to reviewed task-owned changes, never a broad reset; main owns fixture cleanup and eventual Git lifecycle. A broader MVP and any new infrastructure beyond the approved boundary require a new decision.
