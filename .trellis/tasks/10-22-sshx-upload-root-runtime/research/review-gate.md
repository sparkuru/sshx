# Submit-ready human review gate

Evaluated 2026-10-23 by main. Category: **human-required** under `.trellis/spec/trellis-plus/validation.md`, because this task changes effect-site permissions and path deletion/cleanup behavior. Available automated work has been completed; this gate asks for disposition of the concrete bounded result, not for the user to rerun tests.

## Concrete result

- Direct hardlink publication in the existing Core/SFTP path, explicit fixture-owner qualification and default upload refusal without policy.
- Fresh policy/scope checks at upload and insertion; per-effect cancellation/revocation and conservative exact-resource cleanup.
- Confirmed final commit independent of cleanup; lost publication acknowledgement remains indeterminate, without replay or insertion.
- Source and real effect verification: independent fmt/strict Clippy/check/build and 16 unit tests, 13 focused groups and 43 headless Core cases pass; one desktop case is explicitly excluded.
- Exact two-container/generated-credential teardown and port/process checks pass. Canonical source, archived evidence, dependencies/notices and protected Trellis files are preserved.

## Residual scope and evidence limits

Only the generated Linux/OpenSSH cooperative fixture set is measured. No hostile same-account ancestor confinement, arbitrary real-server/filesystem guarantee, desktop coverage, full production plugin/v1 API or broad MVP readiness is claimed. Cross-device primitive evidence is separate from Core same-directory behavior. Status/evidence injections are separately labeled from real server observations.

Earlier fixture-setup and concurrent-wait failures remain recorded. The concrete double-published barrier race and premature completion observation were fixed; strengthened and independent regressions pass. The old timeout lacked last-state detail, so its exact causal interleaving is not asserted. Historical phase0_ready:false remains unchanged.

## Accepted user disposition

On 2026-10-23 the user replied “接受” to the bounded technical result and the concrete three-commit/local-fast-forward/archive/journal batch in commit-plan.md. The human-required decision is satisfied for that scope. This records authority; each Git/closure outcome is recorded only after execution. No remote push or automatic next phase is authorized.
