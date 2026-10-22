# Design: integrated Linux Phase 0 probes

Status: execution approved by the user on 2026-10-22. The product design remains in `docs/`; this task builds a bounded experimental harness, not a released application or an automatic whole-platform support claim.

## Change boundary and ownership

The gap is that the architecture has no executable evidence. The relevant behavior lives at native kernel IPC/PTY, OpenSSH/SFTP, display selection and independent-plugin effect boundaries, so the harness must exercise those boundaries directly.

| Planned files | Responsibility |
| --- | --- |
| `probes/phase0/Cargo.toml`, `Cargo.lock`, focused Rust crate directories | Real probe workspace, public protocol subset, native Core/Client harness and independent image plugin; meaningful tests |
| `probes/phase0/fixtures/` | Minimal SSH fixture build context/entrypoint, independent Python stdlib plugin, synthetic data generation and fixture configuration templates |
| `hako`, `dev.sh`, new project-owned root `.gitignore` | Container build-command wrapper, SSH fixture lifecycle and ignored local cache/artifacts; derive exact commands after real manifests/entrypoints exist |
| `docs/phase-0.md`, relevant ADR/platform entries | Actual command/results pointers and measured decisions, preserving proposed-versus-verified status |
| Current task `research/` and `check-report.md` | Environment/commands/cases/results/limits/cleanup evidence and review |
| Shared development/contracts/mainline | Main-session-only reusable command/profile and lifecycle updates |

Do not touch Trellis runtime, protected `.trellis/.gitignore`, managed helpers, source/notices or human-owned README. Preserve unrelated work. A small public-protocol crate plus harness and independently built plugin are justified by actual ownership; do not create a crate per model or product scaffolding.

## Build and native execution

Use the cached amd64 Rust image identified in `research/readiness.md`, pinned to its immutable repository digest and explicit `linux/amd64`. Confirm actual compiler/Cargo versions before dependency selection. Use the cached amd64 Debian Bookworm fixture base; the cached Debian 13 image is ARM64 and is excluded.

Daemon metadata does not prove that its host sees this workspace. First verify a task-owned synthetic mount/output sentinel and native artifact ABI; a remote/unshared daemon filesystem or missing runtime dependency is a reported prerequisite failure, not a fabricated successful wrapper.

After real probe manifests exist, apply `dev-it-in-docker` to create the root `hako`. Map the invoking UID/GID for build commands and use a project-owned ignored `.devhome` cache. Mount only the probe source subtree and task-owned cache/output paths; do not mount the whole repository, home, agent/socket, Docker socket or credentials into dependency builds. Preserve child stdout/stderr and exit status; wrapper progress belongs on stderr. No network or port publishing for ordinary build/test commands except explicitly required dependency fetch. Registry/package access is limited to the required Rust crates and fixture OS packages; record failures and selected versions. No blind install pipelines or host toolchain installation.

Inspect selected dependency interfaces/MSRV and build scripts/features before execution; pin the actual exercised lockfile. Candidate responsibilities are serde/JSON framing, native Unix APIs, an SFTP protocol library on system-ssh pipes, PNG, a bounded VT parser and an X11 image adapter. Add only what a real case requires. Search snippets and moving `latest` URLs do not select APIs. The current VT/PNG research identifies resize and allocation caveats to test explicitly.

Build/lint/unit checks may run in the container. Execute the reviewed Linux probe binaries on the host kernel for native IPC/PTY/X11 evidence, verifying architecture/runtime compatibility first. Container unit tests are labeled container results. No browser/mobile validation or Web preview applies.

## Native Core, Client and public plugin subset

Core is a separate process owning PTY/ssh and bounded model/effect state in a private user runtime directory. Two or more Client processes use authenticated local IPC with role/version negotiation and single-writer epochs. The probe CLI drives explicit cases and deterministic barriers; it is not a globally installed `sshx` command. Use argv throughout and preserve independent passthrough streams.

Implement only the public subset needed to exercise E1–E7: hello/session/writer operations, explicit action creation, blob/image, upload/proposed insertion, cancel/status/events and plugin handshake/lifecycle/grants. Other product methods remain unavailable. Keep JSON framing/limits/errors and counters aligned with `docs/protocol.md`; record deviations rather than silently editing the baseline. The independent image plugin depends only on public protocol, and a Python stdlib fixture proves no private Core dependency.

Each effect resolves immutable B from Core state. Admission/commit points share a bounded serialization domain; expensive streaming never blocks terminal drains. Retained operation records and trigger high-water marks survive result expiry correctly within the Client identity lifetime. Partial/unknown input and remote commit acknowledgements require reconciliation, not automatic replay. Test actual file bytes and actual PTY destinations at deterministic pre/post-dispatch barriers. A controlled fixture marker is an observation oracle, never generic authentication/Shell-readiness authority inferred from terminal text.

## Controlled SSH/SFTP fixtures

The lifecycle entry is `./dev.sh start|status|down` once implemented; it manages remote SSH fixtures only, not a TCP Core or browser service. Centralize Docker parameters/labels with `hako` support instead of duplicating an alternate wrapper. Fixture sshd may use container-internal root solely for its disposable test users/privilege separation; build commands still map host UID. No privileged mode or host namespaces. Any capability exception must be necessary, explicit and tested.

- Build the fixture from the pinned amd64 Debian base with only needed OpenSSH/test packages. Record installed package versions and final image ID; this is a test server image, not a product deployment image.
- Publish only `127.0.0.1` with an automatically assigned host port by default (configurable loopback host/port variables). Never expose Core management. Collect actual mappings from Docker, not guessed ports/container IPs.
- Generate disposable client and host keys, isolated config and expected known-host entries under a task-private mode-0700 runtime directory, with private files mode 0600. Derive expected fingerprints from the generated keys before connection; never accept an unknown key as a workaround.
- An owned ssh-agent, if used, holds only generated keys. No personal agent, identities, config or passwords enter the fixture/build/plugin. Synthetic password interaction must stay in ephemeral fixture memory/private files and never command arguments/logs; hardware/MFA remains a separate native gap.
- Cases include working/no-SFTP, changed host key, additional unsupported auth, jump/proxy/config changes, owned denied/quota-limited roots, collision/symlink swaps and controlled channel failure. Never fill a host disk, alter global networking or modify a production target.
- Teardown uses exact recorded container IDs plus repo/task/run labels; host helper processes use owned process handles/start identity. No name/port-wide kill or global prune. Retain only documented build cache/evidence, not credentials or running services.

Compare small backend A/B probes. A is system sftp command-language behavior; B is system ssh opening SFTP with a mature protocol library, not another SSH stack. Both use the same isolated fixture assumptions. Only one may become the viable route. Verify exclusive temporary creation, actual no-clobber commit, target/config immutability, namespace mapping and cancel/unknown outcomes. If a backend cannot constrain a race or classify an authentication failure, report that limitation/refusal; do not manufacture success/error authority from untrusted terminal text.

## Private display and restoration experiments

Launch a new isolated Xvfb with no TCP display listener and a private authority cookie; never use the existing DISPLAY or read its selection. Synthetic image producers/receivers remain alive as required by selection ownership. Test image dimensions/encoded bytes/decoded allocations before granting support. In particular, PNG best-effort limits and an adapter that allocates before exposing dimensions cannot establish the required hard bound. Reject that candidate/combination if the experiment cannot prove it.

Use controlled native PTYs and, where useful, xterm/Vim on the private display. Compare bounded VT state with restricted redraw for normal/alternate screen, cursor/modes, resize and detach, then separately fail the owned network/Core. Emit only selected safe visual state for restoration; never replay raw OSC effects. A parser comparing its own output is insufficient as the only oracle: use expected synthetic screens and independent observations for the tested subset. Record unavailable physical IME, actual user desktop and reboot/MFA cases as gaps.

## Evidence, failure and rollback

Each case emits versioned runbook-compatible records with environment, command argv redacted, fixture ID, expected/observed assertions, result and exact owned cleanup. Reports may say pass, rejected candidate, blocked or not-run; only executed assertions support a pass. Produce an E0–E7/XII.1–12 matrix and separate backend/restoration decisions. Evidence of a no-go never turns a failed required test into success or grants MVP readiness.

All runtime secrets/resources live in ignored task-owned paths or private `/tmp` directories; tracked records contain only necessary redacted metadata and synthetic hashes. Before/after each native case, inspect owned child/port/display/container state and cleanup. On failure, stop dependent tests and preserve redacted diagnostics; remove only demonstrably task-created temporary resources. Rollback is a scoped patch/removal of task-owned source plus exact fixture teardown, never reset/reconfigure the host or unrelated work.

Final-plan approval authorizes the concrete bounded fixture/build/native execution above. It does not pre-authorize a Git batch, policy weakening, a new platform/remote target or serial MVP continuation. A genuine scope/authority change returns to planning; routine technical corrections within these contracts remain implementable.
