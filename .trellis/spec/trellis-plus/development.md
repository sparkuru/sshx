# SSHX Development Environment

Read before application work or configuration/toolchain changes. The initialization baseline had no runnable workspace. The approved Phase 0 task now adds a disposable Rust probe workspace under `probes/phase0`, root `hako` and `dev.sh` for isolated experiments. These are feasibility tools, not a released application or product service. The [design source](sshx-design.txt), X–XII, retains Rust/system OpenSSH and native CLI/TUI; only explicitly recorded experiment combinations are verified.

## Before-development checkpoint

The first authorized application task must finish this checkpoint before installing dependencies or generating a development wrapper:

1. Inspect actual manifests, lockfiles, toolchain files, CI, task acceptance and any newly supplied code. Record target host OS/architecture, terminal host, Shell, display system and remote fixture needs. Reuse existing commands rather than inventing a second runtime.
2. Discover the available toolchain read-only using `rustc --version`, `cargo --version` and `ssh -V`, plus `sftp -h` when selecting the SFTP backend. Record missing tools and their actual output/exit semantics; help is not a transfer test. Pin only versions selected and exercised by that task, with official documentation for those versions. Do not install a host toolchain merely because it is absent.
3. Decide host-native versus container execution per check. Build/lint/unit/protocol checks may use a container. Native PTY/ConPTY, desktop clipboard, real terminal and platform ACL behavior require native environments. Record actual permissions/network/offline constraints; this policy does not grant Docker, install or remote authority.
4. If no equivalent wrapper exists and the verified workspace needs one, use the available `dev-it-in-docker` procedure for a project-owned `hako` with pinned toolchain/image, task-relevant mounts, user/cache handling and exact argument syntax. If that procedure is unavailable, record the missing prerequisite rather than invent its implementation. Do not generate a wrapper around nonexistent commands or mount SSH keys/agent/whole home into untrusted test environments.
5. Once a real Cargo workspace exists, verify and record its exact package/features and commands. Candidate checks are `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace`; adapt to actual supported targets/features. These are **conditional command shapes, not currently runnable or validated project checks**. Document locked dependency fetching when an actual lockfile exists; follow explicit no-install/offline restrictions.
6. For SSH/SFTP fixtures, define isolated test identities, known-host expectations, remote roots, supported auth modes, data ownership and teardown before use. Docker may provide disposable remote services with loopback/private test access. Start/stop only owned resources and preserve unknown data; no production targets or credentials. Record what fixture results do not prove about real Windows/macOS/desktop behavior.
7. Update this profile with exact wrapper/build/test/start/stop/readiness commands and local prerequisites actually verified; register it in both task contexts and run the focused acceptance checks. Unavailable platform environments remain explicit gaps.

No initialization wrapper or container run has been performed merely by installing this checkpoint.

## Verified Phase 0 build and fixture infrastructure

The [fixture evidence](../../tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-evidence.md) records actual tool/image/package versions and smoke/teardown results. Build Rust is 1.90.0, Cargo 1.90.0 on the pinned linux/amd64 Rust image. Host-native Rust tools remain absent; no host toolchain was installed. A reviewed container-built ELF ran on the Debian 13 x86_64 host. This ABI smoke is separate from substantive probe cases.

- `./hako rustc --version` executes in `/work` mapped from `probes/phase0`. Only source and separate `.devhome/{cargo,target,home}` caches are mounted. Fixture credentials in `.devhome/phase0`, personal SSH/agent/display and repository metadata are excluded. UID/GID map to the caller. Ordinary commands use network none; authorized dependency retrieval explicitly sets `HAKO_NETWORK=bridge`.
- `./dev.sh start` builds/starts a generated-key/password SSH/SFTP service and prints its private state path. `./dev.sh status` prints redacted metadata; stopped status exits 1. `./dev.sh down` verifies exact recorded container labels/ID, removes that service and its generated credentials; repeated down is safe. Coordinate with active cases before restarting/downing it.
- Fixture publication is dynamically assigned IPv4 loopback only. Server accounts cover normal, password-only, no-SFTP and chroot behavior; denied paths and a bounded 1 MiB quota tmpfs support negative cases. Known-host expectations derive from generated host keys before first connection. The server is a test fixture, not an application preview.
- Native probes are built to `.devhome/target/debug/`. Final Cargo checks are recorded below; infrastructure smoke alone does not pass E1–E7. Generated runtime state remains mode 0700/0600 and ignored. No browser/mobile preview applies.

## Phase 0 commands and selected probe dependencies

The independent whole-workspace run passed all of these commands in the scoped Docker image, with network none and the reviewed lockfile:

~~~sh
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --offline --workspace --all-targets -- -D warnings
./hako cargo check --locked --offline --workspace --all-targets
./hako cargo test --locked --offline --workspace --all-targets
./hako cargo build --locked --offline --workspace
~~~

The original Phase 0 run had 14 unit tests: image 5, harness 4, public protocol 5. The cooperative upload follow-up adds containment and framed reply-matching coverage; use its current check report for actual test counts. The derived image ID is `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1`; rustfmt 1.8.0 and Clippy 0.1.90 were exercised. `./hako --build-image` explicitly builds this toolchain layer from its pinned Rust base. Required crate retrieval uses `HAKO_NETWORK=bridge ./hako cargo fetch --locked`; ordinary commands never implicitly pull an unavailable local development tag.

Resolved direct probe versions include serde 1.0.229, serde_json 1.0.154, Tokio 1.54.1, libc 0.2.191, russh-sftp 3.0.1, png 0.18.1, vt100 0.18.1 with default features off, flate2 1.1.10, crc32fast 1.5.0, base64 0.22.1 and sha2 0.10.9. Cargo.lock is authoritative for all resolutions. This selects experimental dependencies only, not every proposed product crate/platform.

Native interfaces are `.devhome/target/debug/phase0-probe --help` and `phase0-image --help`. The full two-target Core run adds `--fixture-secondary SECOND_STATE` to `suite --fixture PRIMARY_STATE --output NEW_JSON`; [second-fixture setup/teardown](../../tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/dual-target-fixture.md) reuses the existing lifecycle functions under a separate private pointer. Never call normal `dev.sh start` to create a second instance while the first is in use: start replaces its own recorded service. Core accepts only prevalidated generated profiles, and every Session keeps its own frozen profile.

Native image/visual tests additionally need the existing host C compiler, Xlib headers/runtime, Xvfb, xterm and Vim; no host package installation occurred. Run only with the task's fixture authority and fresh evidence paths. Private Xvfb cookies and generated SSH credentials are removed with their exact owners; caches/build images may remain intentionally. General mutable-parent SFTP capability, complete v1 API, real Shell edit-context integration, full attach and other native platforms remain unsupported/unverified regardless of passing finite probe cases.

The cooperative upload follow-up supports `phase0-probe upload-suite --fixture PRIMARY_STATE --fixture-secondary SECOND_STATE --output NEW_JSON` and `phase0-probe suite --headless --fixture PRIMARY_STATE --fixture-secondary SECOND_STATE --output NEW_JSON`. Use headless mode for upload-only work without desktop authority; it records the excluded capture case and uses synthetic PNG for transfer/input assertions. Standalone Core needs explicit `--qualify-upload` for fixture-owner synthetic qualification; no flag means upload unavailable. Read the [bounded probe contract](sshx-contracts.md#bounded-cooperative-upload-probe-contract) before changing those interfaces. Reports never replace an existing output file.

When reusing the fixture lifecycle, record fresh run/container/pointer ownership and exact repo/run/scope labels. The existing wrapper's static historical task label alone does not establish current-task ownership. Preserve unknown services; remove only exact task-created resources after evidence capture. Disposable container destruction after a test is not evidence that Core removed uncertain remote paths.

## Service and preview applicability

`preview: not applicable` at initialization. The proposed Core is an authenticated local IPC service attached to native CLI/TUI clients, not a browser application. No HTTP URL, `.env` preview port, `preview.sh`, LAN publishing or browser startup is useful for the current documentation task. No service is started and no listener/address is promised.

When an actual Core lifecycle is implemented, document endpoint ownership, authenticated readiness, concurrent startup, stale-endpoint handling, live-session retention, normal stop and crash cleanup. A running process alone is not authenticated readiness. Do not expose a TCP management port to meet a generic preview convention. Distinguish native Core lifecycle from optional remote test fixtures.

If a later authorized task adds a browser-previewable service, reclassify before writing scripts. Reuse one service lifecycle behind a thin preview entry; establish configuration, readiness, preparation, owned teardown, redacted failure diagnostics and full effective listener/publishing/address reporting. The applicable preview procedure must be loaded then; do not copy a generic renderer now or claim its output contract was exercised.

## Configuration when real consumers exist

There is currently no `.env` consumer or required user value. Do not create an unused environment file/loader or ask the user for secrets. Native product configuration should follow the source's platform-standard user directories; `.env` is not automatically the application config format.

For a future task that adds configuration:

- Identify the actual loader/file, schema, consumers, precedence, required keys, safe defaults and restart/rebuild action. Update example and consumer together. Never source dotenv as Shell code.
- Keep examples secret-free and local values ignored/untracked. If initial setup needs a local copy, copy the example only when the local file is absent and authorized; list keys needing user input, purpose/source and exact next command. Do not overwrite an existing file or create a partial one with only new keys.
- On incremental changes, test key presence with the actual format. Append only missing keys, preserving custom values, existing empty values, comments, quoting and line endings; separate new content correctly and make repeated application deduplicate. Report empty/duplicate keys needing attention without replacing them silently.
- Renamed/removed keys require consumer/example updates and explicit local migration instructions; do not delete user values or invent compatibility aliases. Report tracked secrets without printing them or rewriting Git history.
- Validate changed synchronization with temporary missing-file, custom-value, empty-value, new-key and repeat-run fixtures. Report only key names and required actions, never local secret diffs.

## UI tooling boundary

The design includes a future native TUI, but initialization is documentation-only and creates no screen, interaction or visual assets. It has no browser surface. No UUPM initialization or Playwright installation runs for this task.

On a later actual UI task, reassess native/browser/mobile applicability from its PRD and code. If UUPM is applicable, check a complete active-platform local installation first (for Codex, `.codex/skills/ui-ux-pro-max/SKILL.md` and its required scripts/data). When absent, ask whether to initialize for that platform before UUPM commands or derived artifacts; explain that the generated files stay local. A global install is insufficient, and decline does not remove normal UI verification.

For an accepted applicable UUPM flow: Plan saves research under the task and selected decisions in approved `design.md`; Implement reads both; Check compares behavior/states/keyboard/accessibility and applicable sizing against that design; Update Spec promotes only reusable original decisions with license-safe source handling. Register research/design and applicable UI rules in both task manifests. Never overwrite existing platform helpers or treat missing tooling as proof that UI work is inapplicable.
