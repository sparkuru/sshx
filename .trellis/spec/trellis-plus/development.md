# SSHX Development Environment

Read before application work or configuration/toolchain changes. The initialization baseline has no `Cargo.toml`, app source, lockfile, app tests, CI, `hako`, lifecycle script, environment example or preview service. There is no current app build/start command. The [design source](sshx-design.txt), X–XII, proposes Rust/system OpenSSH and native CLI/TUI; listed crates and platforms remain unverified.

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
