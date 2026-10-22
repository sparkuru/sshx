# Cooperative upload-root runtime verification

## Goal

Verify cooperative upload-root hardlink publication, policy enforcement, no-clobber behavior and conservative cleanup in controlled fixtures.

## Background and authority

The user replied “创建” on 2026-10-22, approving task creation and planning only. The prior archived `10-22-sshx-upload-root-boundary` PRD records the already-selected cooperative mode A. `docs/adr/0003-transfer-backend.md` retains B and identifies hardlink as a future candidate; existing probes exercise baseline rename. `docs/image-paste.md:67`–74 and `:133` define publication and cleanup acceptance. The prior Phase 0 cleanup evidence confirms old fixtures/credentials were removed; they are historical, not live resources.

## Requirements

- R1: Exercise backend B hardlink v1 publication with actual content readback and no overwrite-capable fallback.
- R2: Verify cooperative policy, target/scope binding, grants and capability admission at the relevant effects.
- R3: Preserve independent final-commit and cleanup outcomes, including lost replies and cancellation.
- R4: Delete only proven operation-owned temporary resources with verified continuity; uncertainty leaves cleanup pending.
- R5: Produce independently reviewed, environment-scoped evidence and exact fixture teardown records.

## Acceptance Criteria

- [x] AC1 (R1; C01–C04): Acknowledged hardlink publication preserves uploaded bytes; existing/concurrently created destination data is unchanged.
- [x] AC2 (R2; C01/C04–C07/C12): Missing policy, changed binding/grants and unavailable publication capability stop the relevant effects; actual filesystem effects are observed separately from error enums.
- [x] AC3 (R3; C08/C09/C11): Acknowledged final publication survives temp-unlink failure with cleanup pending; lost publication reply stays indeterminate without insertion or replay.
- [x] AC4 (R4; C06–C10): Cancellation/revocation do not authorize new publication; uncertain ownership/replacement/continuity never causes deletion of replacement or final data.
- [x] AC5 (R5; C12/C13): Real server/filesystem observations are distinguished from fault injection and unverified cases; exact disposable resources are cleaned and results independently reviewed.
- [x] AC6 (R5; C13): Historical Phase 0 reports and their general-root no-go remain unchanged. A negative experiment is reported honestly and does not pass a capability gate.

## Out of scope

Full MVP, a second SSH stack, remote Agent, hostile same-account confinement, production targets, personal SSH configuration/credentials, clipboard/display access, cross-platform release claims and automatic next-task execution. Historical `phase0_ready: false` is not rewritten by a new experiment.

## Approved environment and scope

The user replied “采用” on 2026-10-22: use fresh disposable Linux/OpenSSH Docker fixtures, loopback-only access, generated credentials and synthetic data, reusing existing lifecycle/build tooling. Conclusions cover only the observed server/filesystem set. Use up to two independent generated SSH fixtures to verify target isolation. Build through the existing pinned Docker toolchain; execute the probe locally against those fixtures. No personal desktop or clipboard is required.

Changes are bounded to the existing Phase 0 Core/SFTP probe path, its callers/tests, essential fixture support and evidence-aligned documentation. This is one integrated deliverable: policy, publication and cleanup must be verified in the same operation flow, so separate child tasks would not have independent acceptance. Source design, archived evidence, unrelated runtime/templates, dependencies and release scope remain unchanged.

## Planning status

The user replied “批准” to the final planning summary on 2026-10-22, authorizing the reviewed bounded implementation and disposable-fixture execution. The task is now in_progress on upload-root-runtime. No blocking user-owned decision remains. Technical unknowns are experiment inputs, not assumed passes; missing mandatory evidence blocks positive promotion. See design.md, implement.md and research/planning-evidence.md for scope and case mapping. Git commit/archive and broader MVP remain separate.

## Technical acceptance checkpoint

On 2026-10-23 independent review verified AC1–AC6 within the approved probe scope: 16 unit tests, 13 focused groups, 43 headless Core cases passed and one desktop case was explicitly excluded. Exact fixture teardown passed. See [check report](check-report.md) and [cleanup](research/cleanup.md); source-bound machine reports retain their run-time pending flags and failed iterations remain preserved. The [human-required gate](research/review-gate.md) and local Git/archive disposition are pending; task status remains in_progress. No general-root, production API or platform promotion follows.
