# Bootstrap SSHX Trellis Plus

## Goal

Complete the repository's Trellis initialization with project-owned development policies and requirement continuity derived from `design.txt`, so later agents can plan and validate SSHX without inventing implementation evidence.

## Background

- The user requested reading `design.txt` and completing initialization with Trellis Plus, then explicitly consented to this task and initialization on 2026-10-21.
- Trellis 0.6.17, developer `wkyuu`, and the generated `00-bootstrap-guidelines` task already exist. There is no application manifest, source, test suite, CI, service, or Git commit.
- `design.txt` sections I–XIII describe a CLI/TUI SSH framework with a local Client/Core service, system OpenSSH, independent plugins, and an Image Paste vertical slice. Rust is a default to investigate; listed crates are candidates. This initialization does not authorize product implementation.
- Existing backend/frontend specs are generic scaffolds. Their contents and the installed runtime must be preserved.
- After planning approval, the user supplied the moved source at `/tmp/tmp/design.txt` and explicitly requested its inclusion in spec and mainline. The canonical imported source is `.trellis/spec/trellis-plus/sshx-design.txt`; references to root `design.txt` above describe the original baseline.

## Requirements

- R1: Establish one portable project policy under `.trellis/spec/trellis-plus/`, including development principles, ownership, validation, human review, archive attribution, policy loading, and update revalidation.
- R2: Preserve the full supplied design verbatim in `.trellis/spec/trellis-plus/sshx-design.txt` and import its direction into `.trellis/mainline.md` with source sections, explicit decisions versus defaults and technical unknowns, non-goals, ordered proposed phases, and guided continuation. Do not promote imported direction to implementation authorization.
- R3: Document SSHX-specific component boundaries, session/generation/attachment ownership, protocol and trust boundaries, credential isolation, remote paths, and risk-driven platform validation. Label these as design contracts rather than implemented patterns. Do not fabricate code examples.
- R4: Prove implementation/check agents can load the applicable policy through existing context manifests. Distinguish current task evidence from future-task discovery gaps; do not patch protected loaders.
- R5: Record a concrete before-development checkpoint for the future Rust/OpenSSH toolchain and Docker test fixtures. Do not introduce an unused environment loader, Web preview, browser-test stack, or unvalidated dependency versions.
- R6: Identify exact retained Trellis source/version and preserve obtainable matching license/notice files under `third_party/`; explicitly report unresolved provenance.
- R7: Check every current Trellis Plus checklist ID with evidence and applicability, using PASS/GAP/UNKNOWN/N/A. Separate configured policy from actual execution and missing platform environments.
- R8: Resolve the generic bootstrap task by demonstrable supersession where appropriate, preserve unfulfilled generic checklist items, and close through supported task/archive interfaces after checks and required authorization. An archive requiring a Git commit must not run its automatic commit without commit authorization.

## Acceptance Criteria

- [x] AC1 (R1, R3): A concise shared index links original, project-specific rules without changing protected Trellis runtime, generated platform files, generic specs, README, or user inputs.
- [x] AC2 (R2): The source copy matches the supplied file exactly; mainline maps it to proposed ordered work and testable outcomes; no application feature is described as implemented or verified.
- [x] AC3 (R4): Both curated manifests validate, reference required detail files explicitly, and have observed loading evidence; future-task manual steps and automation gaps are recorded accurately.
- [x] AC4 (R5): Validation/development profiles distinguish runnable initialization checks, future application checks, native terminal/platform checks, and browser/preview non-applicability.
- [x] AC5 (R6): Third-party notices match the identified installed distribution exactly, or the precise unresolved material is reported as `license-notice-needed`.
- [x] AC6 (R7): A report covers IDs 01–34 with source, pre-application situation, final evidence, target, and unresolved actions. Protected/input file content is unchanged and whitespace/context/link checks pass.
- [ ] AC7 (R8): Bootstrap supersession and closure evidence distinguish policy completion, on-disk archival, and archive commit completion; no unauthorized staging/commit or repeated archive occurs.

## Out of Scope

- SSHX source implementation, Phase 0 experiments, dependency installation, remote connections, plugin execution, a Rust workspace, or a new daemon.
- UUPM installation/design generation, Playwright, a Web UI, Docker wrapper generation without application commands, or a preview server.
- Editing protected runtime/templates, personal integration files, README, or the project design; broad staging, changing Git identity, or history rewriting.
- Approving the product roadmap or starting subsequent product tasks automatically.

## Deferred Decisions

- Product implementation and Phase 0 platform/terminal environments belong to later authorized tasks.
- Git commit authorization will be checked against explicit session instructions at submit-ready time; initialization consent alone is insufficient.
- The user approved the final planning summary and implementation; the task is in progress. No product decision blocks the approved documentation/configuration work. Final verification and commit/archive authorization remain distinct closure steps.
