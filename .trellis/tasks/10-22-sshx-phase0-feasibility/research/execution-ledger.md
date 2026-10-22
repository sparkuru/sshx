# Phase 0 execution ledger

## Authority and initial state

On 2026-10-22 the user replied “可以” to the concrete execution summary covering Docker builds and necessary dependencies, Linux-native probes, generated test identities/passwords, loopback-only SSH/SFTP fixtures and a new private Xvfb containing synthetic images. This supersedes the historical pending-approval wording in `readiness.md`. No personal SSH/display/clipboard access, host toolchain installation, Git commit/archive or automatic MVP continuation is authorized.

The main session activated the task through `task.py start`, created feature branch `phase0-feasibility` without a commit, and recorded it through `task.py set-branch`. Existing history and user identity are preserved. The pre-planning baseline is `/tmp/sshx-phase0-planning-baseline.json` (102 tracked paths); protected source/runtime/notice fingerprints will be compared after implementation.

## Ownership

| Agent | Owned files and responsibility |
| --- | --- |
| Main | Task lifecycle/mainline, shared project specs, architecture/result documents, context/evidence integration and final validation |
| phase0_fixture | Root hako/dev.sh/project .gitignore; probe fixture build/lifecycle files; fixture evidence. After lifecycle smoke, additionally owns focused harness `plugin.rs` and stdlib `fixtures/plugin.py` E6 probes |
| phase0_core | Probe workspace manifests/lockfile, public protocol and harness crates except the explicitly reassigned `plugin.rs`; core/SSH/SFTP/effect evidence |
| phase0_visual | Independent image-plugin/visual probe crate; private-X11/PNG/VT evidence |

Each worker was told it is already the implement role, must not recursively dispatch implement/check agents, must preserve other edits, and has no Git/archive/mainline ownership. Actual public API and fixture interfaces are coordinated directly between workers. Planned fixture state carries schema/run/loopback/port/user and generated credential *paths*, never credential contents in evidence.

## Context gate

Both contexts have fourteen real inputs. Known per-file truncation affects `docs/protocol.md` (46,819 bytes), and total injection may omit others. Dispatches explicitly require complete bounded direct reads of every omitted input plus PRD/design/implement, runbook/platform matrix and all six ADRs. Worker reports must establish actual loading; hook success alone is insufficient. No protected loader/config change is allowed.

## Evidence policy

This ledger records coordination, not passing experiments. Actual case outputs and commands will be recorded by the responsible workers and independently checked. Candidate rejection is separate from a failed required assertion. Missing required evidence prevents Phase 0 PASS/MVP readiness; native Windows/macOS/Wayland, physical IME/MFA and reboot remain outside this run. Browser/mobile UI checks are inapplicable to these native probe binaries.

## Infrastructure checkpoint

Fixture implementer reported strict-key normal/no-SFTP/chroot outcomes, noninteractive password refusal, bounded quota/denied roots and a complete down/down/status/port-release proof. Exact versions, image IDs and commands are in `fixture-evidence.md` and `fixture-results.json`. A fresh exact-owned integration instance remains live intentionally; final teardown is pending. Native reviewed ELF ABI smoke passed. This infrastructure evidence was registered as the fifteenth input in both contexts.

An interim baseline comparison found only the three intended existing-file edits: mainline, project-owned development profile and phase-0 runbook status. All other 99 baseline paths remain byte-identical; the 30,205-byte canonical design retains SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`. Context validation passes with the known protocol-truncation warning, requiring the documented direct-read fallback.

## Native package inventory

Main-session read-only `dpkg-query -W` records the host packages used by native probes: GCC `4:14.2.0-1`, glibc `2.41-12+deb13u4`, libX11 and development headers `2:1.8.12-1`, Vim `2:9.1.1230-2`, xterm `398-1`, Xvfb `2:21.1.16-1.3+deb13u4`. Duplicate glibc/libX11 output entries are installed architecture variants with the same version. This inventory does not substitute for the visual cases. No packages were installed on the host. Both native probe binaries’ published `--help` succeeded in the main session.

## Core continuation and independent review

The first Core worker produced 32 passing native cases plus explicit unresolved rows, then its implementation turn was interrupted twice by the tool content filter. This was an agent-turn interruption, not an automatic action-approval rejection; no policy or authorization was weakened. The worker stopped code execution and saved `core-handoff.md`. A fresh implement worker, `phase0_complete`, now owns the remaining Core files and explicit correctness gaps. Existing artifacts and both generated-data fixtures are preserved. Independent `phase0_check` owns stable-surface review/fixes and will run a final full-scope pass after Core freeze.

Independent native visual rerun agrees with 20 PASS/1 rejected candidate. E6 review added an actual 1 MiB queued-wire-byte cap alongside the stricter 16-frame cap; its new flood case rejects frame five at peak 960,376 bytes while real SSH output advances. The new independent E6 report has 32 passing top-level cases and is distinct from the original 31-case implementation snapshot. Individual JSON parsing heap and the in-progress frame are separate from this queued-wire-byte measure.

## Distinct-target fixture extension

A second generated-credential SSH service was created through the existing lifecycle functions under a separate private `.devhome/phase0/secondary` pointer. The normal primary pointer is unchanged. Distinct containers, loopback ports, generated host identities and filesystems are documented in `dual-target-fixture.md/json`; these are two test services on one Docker host, not two physical machines. Both are intentionally retained for the final same-Core multi-profile tests. Main must tear down the secondary through the recorded helper and the primary through `dev.sh down`, then verify both exact container IDs/ports/credentials are gone.

Both contexts now have twenty valid existing-path entries. The known 46,819-byte protocol injection warning remains; complete direct reading is still mandatory. Main normalized the project-owned task JSON final newline after supported task metadata writes. No Git staging/commit/archive has occurred.
