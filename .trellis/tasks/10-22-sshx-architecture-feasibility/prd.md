# SSHX architecture and feasibility plan

## Goal

Turn the retained SSHX requirements into a consistent architecture and executable Phase 0 investigation plan, so the first application slice can be built against explicit ownership, permission, protocol and failure contracts.

## Background and authority

- Canonical requirements: [SSHX design](../../spec/trellis-plus/sshx-design.txt), sections I–XIII; section XIII requires this deliverable before Phase 0.
- The repository contains project policies and initialization records, with no application workspace, runtime tests or platform support evidence.
- On 2026-10-22 the user requested implementation, consented to creating this architecture-and-feasibility task, and explicitly approved the final plan after the language/architecture explanation ("ok；批准方案"). Architecture-documentation execution is authorized.
- This is one integrated design deliverable. The eventual Phase 0 and application phases remain separate proposed work, without serial authorization.

## Requirements

| ID | Required outcome | Source |
| --- | --- | --- |
| R1 | A traceable requirements inventory distinguishes fixed requirements, source defaults, superseded approaches and technical unknowns. It accounts for all thirteen source sections and preserves the source's phased scope. | I–XIII |
| R2 | A capability/platform matrix distinguishes client OS, architecture, terminal host, Shell, display system, OpenSSH/authentication and remote system. Each row states prerequisites, fallback and evidence status; unavailable native environments stay unverified. | II, IV–V, VII, X–XII |
| R3 | Process/dependency and data-flow diagrams assign Client, Core, OpenSSH and independent-plugin ownership. Define CLI enhanced/passthrough/refusal behavior per invocation class, user-local service lifecycle and allowlisted per-session context. | II–IV, IX–X |
| R4 | Models and state transitions define ConnectionProfile, Session/generation, Attachment/writer, Operation, TransportHandle, CapabilityGrant and RemoteContext. Local IPC and public plugin contracts specify framing, versions, limits, errors, cancellation, identity, event loss and effect-site authorization. | III–VI, IX |
| R5 | An Image Paste transaction describes one triggering-Client snapshot, bounded PNG/blob processing, verified transfer target, namespace mapping, non-overwriting upload, conditional insertion, deduplication and resource ownership. Include failure/race sequences and retained-result behavior. | V–VII, XII |
| R6 | Six focused ADRs cover language, PTY/ConPTY, SFTP backend, screen restoration, plugin isolation and remote integration. Preserve Rust/system OpenSSH as the source default; exact dependencies and evidence-dependent choices stay provisional until actually tested. External technical claims need primary sources and verification dates. | IV–VI, IX–XIII |
| R7 | A Phase 0 runbook and phased follow-up map specify controlled-fixture prerequisites, experiment methods, observable pass/fail conditions, evidence templates and selection gates. Cover all twelve source XII fault groups, with later-phase cases explicitly deferred. | XI–XIII |

## Constraints and non-goals

- Preserve system `ssh`, OpenSSH credential/host verification ownership, authenticated user-local IPC and the independent public plugin boundary. No global active session or plugin access to raw executors, PTYs, credentials or the full environment.
- Only explicitly trusted local plugins are planned for the unsandboxed MVP. Distinguish failure isolation from operating-system sandboxing and identify the same-user-process boundary.
- No application code, Cargo workspace, dependency installation, service/container startup, clipboard read, remote connection, benchmark or native feasibility experiment in this task. Writing an executable experiment procedure does not authorize executing it.
- No full TUI, later plugins, permanent remote hooks, remote Agent, GUI, cloud, AI, marketplace or release/name change. Later-phase contracts may be identified without implementing them.
- No changes to the verbatim design, generic scaffolds, Trellis runtime/managed helpers, root notices or human-owned README. Git commits/archive require their separate concrete authorization.

## Acceptance Criteria

- [x] AC1 (R1): A reviewer can follow each requirement ID to a source section, its owning design artifact and its verification method, with no source requirement silently removed or described as implemented.
- [x] AC2 (R2): Windows, macOS and Linux X11/Wayland/headless client cases and Linux/macOS/Windows remote cases appear with explicit support conditions and evidence status. Native coverage is never inferred from containers or cross-compilation.
- [x] AC3 (R3): Diagrams and invocation tables unambiguously locate process/PTY ownership, per-Client activity, all stream boundaries, startup/stop/upgrade behavior and the behavior of pipes, redirection, remote commands, `-T`, `-N`, `-W`, subsystems and background mode.
- [x] AC4 (R4): All seven models have fields, ownership, transitions and cleanup rules. Every proposed method identifies caller/owner, authorization/effect site, limits, timeout/cancellation and error/result semantics; reserved later methods are labeled unavailable. Protocol examples agree with framing and schemas.
- [x] AC5 (R5): A reviewer can trace upload success separately from insertion success, including A-to-B switching, detach, reconnect, revoke/disable, expiry and cancel. No branch silently retargets work, repeats upload on insertion retry, overwrites an existing file or inserts with unknown context.
- [x] AC6 (R6): Six ADRs record decision status, alternatives, evidence, selection criteria and reopening triggers. No untested crate API/version, universal multiplexing support or platform capability is asserted as verified.
- [x] AC7 (R7): Each Phase 0 experiment has identified fixtures, concrete actions/commands or explicitly future harness requirements, observable expected outcomes, failure handling, teardown ownership and evidence fields. Missing primary native environment/fixtures block running the affected experiment, not completion of this documentation task.
- [x] AC8 (R1–R7): An independent Trellis check finds no unresolved material design inconsistency; JSON/JSONL, local references, new-file formatting and protected/source fingerprints pass. The check report distinguishes document verification from product/runtime evidence.

## Deferred decisions

Primary native environment and controlled SSH/SFTP fixtures must be selected before the Phase 0 execution task. SFTP backend and restoration strategy must be selected from that phase's measured results. Exact dependency versions are chosen only with matching official-interface and execution evidence. These are explicit downstream entry gates, not blockers for specifying this investigation.
