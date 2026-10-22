# Platform and capability matrix

Status: design targets plus bounded Linux Phase 0 evidence, 2026-10-22. The broad target rows below remain investigation scope, not blanket support. Only the explicitly measured combinations in the next table have runtime evidence. A build, container or library support statement does not establish another native combination. Record actual combinations using [the evidence template](phase-0.md#evidence-record).

## Measured Phase 0 combinations

| Actual combination | Evidence and observed boundary | Status |
| --- | --- | --- |
| Debian 13 native Core/Client/PTY; system OpenSSH 10.0p2; two generated Debian 12 OpenSSH 9.2 services | [Independent final Core/effect report](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check-report.md): 44 top-level cases, real per-Session target/PTY isolation, key/agent/password, lifecycle/cancellation/backpressure/reply-loss assertions; two distinct containers on one Docker host | Controlled subset passes; general mutable-root binding remains no-go; full API/real Shell context unimplemented |
| Debian 13 Linux x86_64; private Xvfb `2:21.1.16-1.3+deb13u4`, Xlib `2:1.8.12-1`; generated PNG selections | [Native image/visual evidence](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-evidence.md): one-shot Client-bound capture, alpha, immutable sealed bytes, strict PNG profile and preallocation checks. INCR is refused; missing and denied Xlib access cannot be classified separately | Measured finite X11 profile; existing user desktop/Wayland unverified |
| Same host, private xterm `398-1`, Vim `2:9.1.1230-2`, native Unix PTY | Independent screen/cursor/mode oracle; 20 total E3/E5 passing cases, one hidden-normal-buffer restoration candidate rejected. Restricted cooperating redraw works for the tested receiver; attributes/wrap/scroll-region fidelity unavailable | No generic full attach claim; Phase 2 remains gated |
| Native Linux Core UID 1000; isolated same-kernel container-origin UIDs 65534 and 0 | [Cross-UID evidence](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/ipc-crossuid.md): actual 0700 runtime/0600 socket DAC refusal, then SO_PEERCRED refusal before handshake with unchanged metadata. Read-only synthetic socket mount only; no host account changes | Two native-kernel IPC negative cases pass; no Windows/macOS peer claim |
| Pinned amd64 Rust build container; native Debian 13 ELF execution; generated-key loopback Debian 12 SSH fixture | [Fixture evidence](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-evidence.md): Rust/Cargo 1.90.0, strict-key fixture login, normal/no-SFTP/chroot, bounded quota and exact teardown; OpenSSH server `1:9.2p1-2+deb12u10` | Infrastructure measured; substantive Core/effect results recorded separately |

No host packages were installed. Build-only results are explicitly container results; the image/PTY cases execute on the host kernel using task-private displays and generated data. Physical IME, hardware-key/MFA, Windows/macOS/Wayland, native non-Linux remote servers and reboot are unverified.

## Client dimensions

Architectures listed are investigation targets, not promised release artifacts. Terminal host and Shell must be recorded independently. POSIX Shell means a separately identified implementation/version, not every Unix Shell.

| Native client / architecture target | Terminal host / local Shell examples | Required evidence/prerequisites | Planned fallback | Status / experiment |
| --- | --- | --- | --- | --- |
| Windows x86_64, ARM64 | Windows Terminal and console host separately; PowerShell and cmd separately | System OpenSSH build; ConPTY availability, native pipe ACL/peer checks, Unicode/control/resize/close, desktop session image access | Native ssh passthrough; no image action if desktop unavailable; retain uploaded path if no Shell mapping | Unverified; E1–E7; ARM64 separate from x86_64 |
| macOS ARM64, x86_64 | Terminal.app and another host separately; zsh/bash separately | System OpenSSH build, PTY/job control, user socket peer checks, current-login clipboard permission/alpha | Native ssh; explicit file selection when desktop image unavailable; preview/copy | Unverified; E1–E7 |
| Linux x86_64, ARM64, X11 | Selected emulator; bash/zsh/fish separately | Actual X server/selection owner, adapter build features/system dependencies, native Unix peer credentials | Native ssh; explicit local file selection, never file scanning | Unverified; E1–E7 |
| Linux x86_64, ARM64, Wayland | Selected emulator/compositor; local Shell separately | Compositor/protocol versions, adapter features, actual image owner, XWayland if used; native clipboard restrictions | Label XWayland result as such; unsupported pure Wayland returns CLIPBOARD_UNAVAILABLE, offer explicit file selection | Unverified; E1–E7 |
| Linux headless/TTY/SSH login | Real console or nested terminal; no desktop assumed | PTY and prefix delivery, no inherited stale DISPLAY/Wayland assumptions | Basic managed session where verified; Image Paste unavailable; explicit file action only | Unverified; E1/E2/E3/E6 |

The current arboard project documents Linux X11 and feature/compositor-dependent Wayland paths. That is evidence to test build features and desktop conditions, not selection of a crate/version or proof of SSHX support. [arboard backend documentation](https://github.com/1Password/arboard), reviewed 2026-10-22.

ConPTY setup/teardown requires native Windows evidence. Microsoft describes pseudoconsole channels and a hosted child; no Linux run can establish its close ordering. [Microsoft pseudoconsole session](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session), reviewed 2026-10-22.

## Remote and authentication dimensions

Every client/remote pair additionally records target user, port, key/agent/password/MFA, jump/proxy, explicit config source and host-key expectation. Native Windows/macOS remote services need actual native fixtures when claimed; a Linux container named “Windows” is not one.

| Remote target | Login condition | Transfer and namespace condition | Fallback / unknown | Evidence |
| --- | --- | --- | --- | --- |
| Linux standard SSH service | System ssh handles available interactive authentication and host checks | Actually enabled SFTP; verified canonical user root and dedicated upload directory; tested POSIX Shell mapping | No SFTP leaves login usable; unknown cwd/mapping means transfer-path-only | Unverified; E2/E4 |
| macOS standard SSH service | Record service/config/auth and remote Shell | Confirm SFTP configuration and user-root mapping; do not infer from local macOS client test | Same transfer-only result when mapping unknown | Unverified; E2/E4 |
| Windows SSH service | Record server/client builds, auth, configured default Shell | Verify actual SFTP namespace, drive/user-root behavior and explicit mapping to PowerShell or cmd paths | Never treat /C:/, C:/ and C:\\ as equivalent without fixture; plain transfer path only when unknown | Unverified; E2/E4/E7 |
| SFTP chroot / container / nested ssh/sudo/tmux | Outer Session remains actual transfer target | Explicit versioned mapping identifies namespace root and Shell destination | Unknown nested destination disables automatic insertion; show outer target; no probing through live Shell | Unverified; E4/E7 |
| SSH service without SFTP | Login may work | NO_SFTP for transfer | No hidden scp or remote command fallback | Unverified negative fixture; E4 |

| Reuse/auth capability | Planned contract | Required evidence |
| --- | --- | --- |
| Config reuse | Preserve applicable target/options; config-generation binding and fail closed on unbound changes | E4 Include/Match/config mutation fixtures |
| Identity reuse | OpenSSH consumes isolated identity/agent, not plugin credentials | E2/E4 key, agent, hardware and additional-auth outcomes |
| Network connection reuse | Use only a validated master/control route for exact client/platform/version | E4 observes actual connection/auth reuse, not just equal hostname |
| Auxiliary authentication | MVP supports noninteractive key/agent or validated master; extra interactive auth returns AUTH_REQUIRED | E4 proves no prompt steals main terminal; dedicated auth UX deferred |

ControlMaster is a documented OpenSSH configuration facility, with behavior contingent on the actual deployed client. No universal Windows or Unix capability claim follows from the documentation. [OpenSSH configuration manual](https://man.openbsd.org/ssh_config.5), reviewed 2026-10-22.

## Independent capability results

| Capability | Acceptance boundary | Fallback / release statement |
| --- | --- | --- |
| Managed interactive login | E1/E2 native terminal/auth/IPC tests pass for recorded combination | Native passthrough for unclassified invocations |
| Prefix image action | Host really delivers configured prefix/action bytes; escape/timeout/literal rules pass | Disable/rebind prefix; use explicit Session action |
| Image snapshot | Triggering Client gets a single supported raw image; limits/alpha verified | Empty, denied and unavailable are different errors; separate file chooser requires fresh user action |
| Safe upload | One backend passes target, auth, non-overwrite, cancellation and namespace tests | Missing safe-commit capability is unsupported, never silent overwrite |
| Path insertion | Fresh known mapping and verified edit context or explicit local preview/lease | Retained result/path copy, no blind keystrokes |
| Detach survival | E5 proves Core/PTY/ssh stay alive after Client loss | Cannot claim screen restoration from process survival |
| Screen restoration | E5 selects strategy and publishes supported modes/limitations | Restricted redraw explicitly disclaims faithful full-screen restoration |
| Network/Core/reboot recovery | Separate E5 failures; new generation always | No default remote-work recovery or PTY adoption |

Native paste shortcuts are host-controlled inputs and cannot be a universal image trigger. Kitty documents its own MIME clipboard protocol; enabling such an extension requires a separately tested terminal/multiplexer combination and user permission, and is outside the generic MVP. [kitty clipboard protocol](https://sw.kovidgoyal.net/kitty/clipboard/), reviewed 2026-10-22.

The measured primary probe environment is Linux x86_64 with private X11. Other dependency/backend/restoration choices require their exact case evidence and advertised scope; the broad target rows do not select them. [Phase 0](phase-0.md) records those selection gates and honest gaps before a Phase 1 support claim.
