# SSHX Phase 0 feasibility

## Goal

Establish measured Linux feasibility evidence for the approved SSHX architecture before the application MVP: build reproducible probes, exercise controlled native/SSH/SFTP cases, and record viable choices, failures and precise unverified combinations.

## Background and authority

- Architecture and runbook: `docs/architecture.md`, `docs/protocol.md`, `docs/image-paste.md`, six ADRs and `docs/phase-0.md` (E0–E7).
- On 2026-10-22 the user selected Phase 0 continuation and explicitly consented to creating this task and entering planning. The user subsequently approved the concrete final execution plan with “可以”: bounded Docker builds/dependency retrieval, native Linux probes, generated-credential loopback SSH/SFTP and private synthetic Xvfb experiments are authorized. Git commit/archive and later product phases remain outside this approval.
- The current working tree was clean, with no active task or application workspace before creation. The previous architecture task is completed/archived.
- Read-only inventory: Debian 13 Linux x86_64; system OpenSSH 10.0p2 and SFTP available; `rustc`/`cargo` absent from PATH; C compiler, Python, Xvfb and X11 query tools available; system sshd absent but sftp-server present. Actual desktop/terminal coverage is not established by tool or environment-variable presence.
- Docker client/server 26.1.5+dfsg1 are available through the normal scoped approval path; ordinary sandbox socket access was denied. Read-only metadata confirms amd64 Rust/Bookworm candidates, while the cached Debian 13 image is ARM64 and excluded. No container, service, image pull, installation, clipboard read or remote connection has occurred.

## Requirements

| ID | User-visible outcome | Runbook |
| --- | --- | --- |
| R1 | A reproducible, narrowly scoped Rust probe workspace and development-command entrypoint run without installing a host Rust toolchain. Record actual compiler/dependency/image versions and preserve reviewed lockfiles. | E0 |
| R2 | Native Linux probes exercise user-local IPC, concurrent startup/Client/writer ownership, Core-owned PTY/OpenSSH lifetime, binary passthrough, terminal cleanup, prefix handling and synthetic remote clipboard-control denial. | E1–E2 |
| R3 | A task-private Xvfb/X11 selection containing synthetic images exercises one-shot capture, immutable raw/PNG blobs, alpha, encoded/pixel/allocation bounds, corrupt/oversized input and explicit missing-desktop outcomes. | E3 |
| R4 | Controlled SSH/SFTP fixtures supply isolated identities, expected host keys and owned paths. Compare two small source-defined backend probes, then select at most one viable route against target/config, authentication, no-overwrite, path, cancellation and integrity assertions. | E4 |
| R5 | Compare bounded VT-state restoration with restricted redraw using normal/alternate screen, cursor/mode, resize and a controlled editor. Report process survival, visual restoration and network/Core failure recovery separately. | E5 |
| R6 | A genuinely independent plugin and a non-Rust stdlib fixture exercise framed public RPC, version/role/grant enforcement, slow/crashed/malformed peers, bounded historical state and event-loss recovery without stalling SSH. | E6 |
| R7 | Deterministic barriers trace real upload/input effects across switch/detach/reconnect/revoke/disable/cancel/expiry and response-loss races. Prove bounded trigger/effect dedup, refreshed zero-byte insertion and no replay of partial/unknown writes. | E7 |
| R8 | A requirement-to-case report accounts for XII.1–12, records exact commands/environment/results/cleanup, and distinguishes measured viable choices, rejected candidates, blocked checks and unverified platforms. | E0–E7 |

## Scope and execution boundary

- Primary target: the current native Debian Linux x86_64 kernel, system OpenSSH and controlled PTY/X11 fixtures. An isolated Xvfb/xterm is a distinct tested display/terminal combination; it does not prove a user's desktop, physical IME or every emulator.
- The final execution proposal includes isolated Docker builds, narrowly scoped dependency/package retrieval, disposable SSH fixtures published only on dynamically assigned `127.0.0.1` ports, task-private Xvfb/agent processes, generated test credentials and native execution of reviewed probe binaries. None has run during planning.
- Fixtures use only generated identities/config/known-host data and synthetic files/images. Preserve personal SSH profiles/keys/agents, the existing display/clipboard, unrelated Docker resources and host packages/security policy. No privileged container, Docker socket/home/credential mount or Core TCP listener.
- This is one integrated feasibility harness and decision report: shared fixture/resource ownership and cross-layer E7 acceptance make the experiments parts of one checkpoint. No child task is created solely per experiment/library. If implementation reveals a genuinely separate deliverable, return to planning rather than silently widening the task.
- Out of scope: a released `sshx` application/full v1 API, system command/alias installation, full TUI, later plugins/history/forwarding, remote Agent, Windows/macOS/Wayland execution, physical input/MFA/hardware-key claims, host toolchain installation, production targets, deployment, and Git commit/archive without a later concrete batch approval.

## Acceptance Criteria

- [x] AC1 (R1): Actual manifests/lockfile, wrapper and probe help expose working build/fmt/lint/test/case/suite commands; commands and exact versions are exercised and documented. No host Rust installation or placeholder product tree is introduced.
- [x] AC2 (R2): At least two real Clients contend through actual user-local IPC and Core-owned native PTY/SSH processes. Loser resize/input fails; Client loss preserves the owned child; stale endpoint/version, bytes/stdio/exit, prefix/control and cleanup cases have observable assertions.
- [x] AC3 (R3): Synthetic private-X11 image capture/PNG tests show one snapshot, preserved alpha and exact sealed bytes. Pre-allocation/encoded/raw limits are measured; an adapter that cannot meet the bound is explicitly rejected/blocked rather than called supported.
- [x] AC4 (R4): Real isolated SSH/SFTP sessions demonstrate key/owned-agent and synthetic-password login, changed-host-key refusal, no-SFTP/additional-auth outcomes, actual config/reuse evidence, special paths and destination collision/symlink/partial/cancel cases. One backend is selected only if its advertised fixture set satisfies mandatory invariants; an evidenced no-go is distinct from success.
- [x] AC5 (R5): Native process-lifetime observations and separate screen/cursor/mode comparisons support a bounded restoration choice or precise no-go, including resize/alternate-screen/editor limitations. Raw terminal log replay is never accepted as restoration.
- [x] AC6 (R6): Independent-process and non-Rust protocol tests cover wrong roles/identity, version/framing/limits, capability composition/revocation, crashed/slow peers, event requery and retained-record churn while a real SSH process makes progress.
- [x] AC7 (R7): Barrier-driven integration traces assert actual remote files and actual initiating PTY writes for switch/revoke/cancel and lost/partial outcomes. No state-only mock substitutes for the real effect; old trigger counters expire without recreation and insertion retry creates no second upload.
- [x] AC8 (R8): Machine-readable and human-readable evidence maps every runbook/source fault group to a case/outcome/limitation, keeps secrets/bodies out of logs and verifies exact owned-resource cleanup. Failures/blocked required evidence prevent a Phase 0 PASS/MVP-ready claim; they are never relabeled as passing tests.
- [x] AC9 (R1–R8): Independent full-scope review, relevant Rust/Python/shell checks and meaningful unit/integration tests pass; protected/source integrity and document/context checks pass. Evaluate the actual human-review gate before commit/completion, especially if authorization/security behavior changed.

## Deferred evidence and promotion gates

Windows/macOS/Wayland, physical IME, real desktop permissions, hardware keys/MFA and machine-reboot recovery remain unverified unless separately authorized native fixtures are supplied. Dependency compatibility/fetching, strict clipboard allocation bounds, viable SFTP target/no-clobber primitives and restoration fidelity are experiment questions. A candidate's measured rejection is useful evidence, but a material unresolved required check still blocks completion/promotion until resolved or explicitly accepted by the user. No user-owned planning question remains beyond approving this concrete execution scope.

## Delivery and residual decision

Controlled implementation and independent checks are complete: 14 unit tests, 44 native top-level cases, 3 insertion supplements and exact owned-resource cleanup pass. [Acceptance/source map](research/acceptance-map.md) and [formal check](check-report.md) preserve the measured scope and negative candidates. On 2026-10-22 the user replied “接受” to the concrete conclusion and linked local batch: AC4 is accepted as an evidenced negative comparison outcome, and AC9's human-required disposition is satisfied for this controlled deliverable. The same response authorizes the listed three local commits, fast-forward merge and task archive. This acceptance does not establish general-root capability, Phase 0 PASS or MVP readiness, and authorizes no later product phase. Git/archive actions are recorded as completed only after they actually succeed.

The execution approval above reserved Git actions; the subsequent “接受” response supplies the separate, exact local-batch authority. All AC1–AC9 are now satisfied for the accepted controlled feasibility deliverable, including its explicit negative findings.
