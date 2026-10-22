# Phase 0 build and SSH fixture evidence

Observed 2026-10-22 by the fixture implementer. These results validate the build/fixture infrastructure only; they are not an E1–E7 suite PASS or MVP readiness claim. The core/visual workers own the substantive cases and the main session owns final cleanup/review.

## Context and ownership

Native hook output was truncated and saved at `/tmp/hook_outputs/01a1635e-d0b8-7512-8501-2b946b1ebb5e/a50d3790-78a8-4d3b-b0d8-fd35e383d997.txt`. Reading it confirmed omission markers rather than recovered bodies. Direct fallback loaded all fourteen `implement.jsonl` inputs: shared index, development-principles, sshx-contracts, development, validation, commit-policy, continuity, verbatim sshx-design, mainline, cross-layer-thinking-guide, architecture, protocol, image-paste and task readiness. The full 211-line protocol was read in lines 1–70, 71–140 and 141–end. PRD/design/implement, phase-0, platform-matrix and all six ADRs were read directly; E0 was re-read separately after an aggregated read truncated. `dev-it-in-docker`, `code-shellscript` and `code-python` skills were read completely before their respective edits.

Owned infrastructure files: root `hako`, `dev.sh`, new root `.gitignore`; `probes/phase0/fixtures/lifecycle.py`, `fixtures/build/{Dockerfile,.dockerignore}` and `fixtures/sshd/{Dockerfile,.dockerignore,entrypoint.sh,sftp-gate.sh}`; these fixture evidence files. Main later assigned `harness/src/plugin.rs`, `fixtures/plugin.py` and plugin/cross-UID evidence separately. No task lifecycle/spec/Git history operations or protected Trellis edits were performed. The workspace manifests came from the core/visual workers before deriving wrapper commands.

## Build boundary and actual versions

- Docker client/server: 26.1.5+dfsg1, Linux amd64. Docker commands required the normal scoped escalation because the ordinary sandbox denies the daemon socket. No approval, sandbox or agent configuration was weakened.
- Rust base: `rust@sha256:3914072ca0c3b8aad871db9169a651ccfce30cf58303e5d6f2db16d1d8a7e58f`; observed Rust 1.90.0 (`1159e78c4 2025-09-14`), Cargo 1.90.0 (`840b83a10 2025-07-30`), x86_64.
- This base lacked formatter/linter components. The approved Docker-only correction added `fixtures/build/Dockerfile` with `rustup component add --toolchain 1.90.0 rustfmt clippy` after observing active toolchain `1.90.0-x86_64-unknown-linux-gnu`. `./hako --build-image` produced derived ID `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1` (tag `sshx-phase0-build:rust1.90`). Observed versions: rustfmt 1.8.0-stable and Clippy 0.1.90, both commit `1159e78c47 2025-09-14`. A transient Clippy TLS EOF was retried by rustup and installation succeeded. No host component was installed.
- Normal `hako` now requires that prepared local image, resolves its immutable ID before `docker run`, and uses `--pull never`; absent image gives an explicit `--build-image` instruction. This avoids an implicit pull of an unprepared local tag. Base image retrieval remains explicit inside the scoped build. During the brief tag transition, a concurrent visual worker encountered a missing-tag failure; the local guard/pull-never behavior corrected it before final checks.
- Before the wrapper ran, a synthetic `/tmp/sshx-phase0-sentinel-9e8aa298370c` mount proved the daemon sees host files and writes with UID/GID 1000. The exact sentinel inputs/output/directory were removed after verification.
- `hako` mounts only `probes/phase0` at `/work` and three separate `.devhome/{cargo,target,home}` directories at `/cache/{cargo,target,home}`. It does **not** mount `.devhome/phase0`, repository root, home, personal credentials, Docker/SSH-agent sockets or existing display. Cache directories reject links/unexpected ownership/types; mode is 0700. Build commands drop all capabilities and set no-new-privileges.
- Ordinary commands default to `--network none`, publish no ports, use the invoking UID/GID and preserve stdout/stderr/status. Required dependency retrieval uses the explicit `HAKO_NETWORK=bridge` option. No host toolchain was installed. No agent allowlist/hook/settings was written.
- Actual commands `./hako rustc --version` and a network-none `./hako sh -c ... rustc ...` succeeded. The latter asserted that `/cache/phase0` and inherited `SSH_AUTH_SOCK` were absent. A tiny reviewed Rust executable was ELF64 x86-64, used `/lib64/ld-linux-x86-64.so.2`, and printed `phase0-native-abi-ok` on the native host; its source and binary were then removed. This establishes this image/host ABI, not final probe behavior.

## Remote fixture

- Base: `debian@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587`, explicit linux/amd64. The cached ARM64 Debian 13 image was not executed.
- Tested built image: `sha256:cb3eb061776d0835b474754593e692492c473a811607dbab68a815b02cedc70c`, local tag `sshx-phase0-fixture:local`. The pinned base and exact exercised image are recorded; future apt repository availability and dependencies remain prerequisites, not a byte-identical rebuild promise.
- Native strict-key SSH query returned `openssh-server 1:9.2p1-2+deb12u10`, `openssh-sftp-server 1:9.2p1-2+deb12u10`, `python3 3.11.2-1+b1`, `libc6 2.36-9+deb12u14`.
- Server runs with normal bounded Docker isolation, init, 128 PIDs, 256 MiB memory and one CPU; no privileged mode, host namespace or Docker socket. `/run` and `/tmp` each have a 16 MiB tmpfs; `/quota` is a disposable 1 MiB tmpfs, never a host-disk fill experiment. Its remote probe root is `/home/probe/probe-data`; `/denied` is root-owned and not writable by `probe`.
- Container-internal root is used for generated-account password installation and sshd privilege separation. Only the run's **server-input subdirectory** is mounted read-only: generated server host key, client public key and synthetic password. The client private key/config/state stay outside that mount. All runtime directories are mode 0700; private keys/password/config/state/known_hosts are mode 0600. Passwords flow from private file to `chpasswd` stdin, never argv/environment/logs.
- Host/client keys are generated before startup. Known-host expectations derive directly from the generated host public key and actual loopback port before the first SSH connection. `StrictHostKeyChecking yes`, `IdentitiesOnly yes`, `IdentityAgent none`, isolated config and known_hosts remain active; no ssh-keyscan acceptance or host-check bypass.
- Accounts: `probe` normal SSH/SFTP, `probe_nosftp` working SSH but controlled SFTP refusal, `probe_password` password-only auth, `probe_chroot` internal-SFTP rooted at `/chroot`, initial `/upload`. `fixture` and `phase0` name the normal generated profile; password/no-SFTP/chroot aliases exist too.

## Exercised checks and cleanup

| Check | Observed result |
| --- | --- |
| Shell syntax, ShellCheck, shfmt | `bash -n hako dev.sh`, `sh -n fixtures/sshd/*.sh`, `shellcheck` and `shfmt -d` passed after splitting readonly declaration/assignment |
| Python/help/whitespace | `python3 -m py_compile fixtures/lifecycle.py`, both wrapper help commands and `git diff --check` passed |
| Nonloopback publication | `HAKO_BIND_HOST=0.0.0.0 ./hako --fixture start ...` refused before any Docker run |
| Initial startup failure | Socket readiness preceded sshd authentication readiness; failed container `3c1225b6fdce892faac03524ee2fb82f0465cb7c8ef75f1628e2f720a2492f42` was removed and its run directory deleted. Readiness was fixed to retry strict-key authenticated `true`, not accept a socket alone |
| Strict-key baseline | SSH `id -un` returned expected users for `phase0` and `phase0-nosftp` |
| Subsystems | `sftp -F CONFIG -b - ALIAS` with `pwd` succeeded for normal and chroot accounts, failed for no-SFTP account |
| Additional auth | Password-only account with noninteractive BatchMode returned 255; interactive synthetic-password acceptance belongs to the core suite |
| Paths | Normal root and bounded quota were writable, denied root was not, under the actual probe user |
| Teardown repeatability | `./dev.sh down` twice succeeded; `status` returned `{"status":"stopped"}` with status 1. Removed run `3dcccf330b54fdc98a30d28b6c3ad95c`, container `cc7e21b5cdb46b350b74e622c2392f5f4ba5505faa7486686b69932be5e86141`; credentials absent and old loopback port 32769 successfully rebound in an escalated native socket check |

After that lifecycle proof, a fresh authenticated fixture was handed to the core worker: run `09422203e8e5490890c2ddf05c2ff249`, container `1d11fd8f596079ec7873f3b488974d53630e37560494bb1935960b6755327be3`. The only live pointer is ignored `.devhome/phase0/current.json`, pointing to that run's private `state.json` (schema 1; host/port/user, generated credential-file paths, remote roots, exact container/image IDs). This fixture is intentionally retained for authorized integration and will not be restarted/downed mid-suite. **Final integration cleanup is pending main-session teardown**; no final clean claim is made here. `./dev.sh down` verifies exact repo/task/run/scope labels against that recorded container before removing it and its credentials. No global prune, wildcard cleanup or process/port-wide kill is used. The build image/cache are intentionally retained for reruns.

Current limitations: this is a Linux controlled server fixture. It does not prove physical terminal/IME/MFA, real desktop access, non-Linux client/server support or a safe SFTP backend. Generated fixture state and secret bodies remain ignored and absent from tracked evidence.

## Main-session final cleanup

The live-fixture/retained-integration wording above records the earlier implementation checkpoint. Subsequent main-session [final cleanup](cleanup.md) passed: both exact generated services, credential directories and pointers are removed, both loopback ports rebound successfully, and no task-owned container/process remains. Build caches/images and explicitly documented synthetic review artifacts are intentionally retained. No further fixture use is pending.
