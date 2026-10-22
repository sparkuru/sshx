# Primary-source evidence

Reviewed online on 2026-10-22 using read-only web open/find. These are narrow documentary findings, **not native execution evidence**. No package source was copied into the deliverables, no dependency was installed/selected and no credential/clipboard/remote action occurred. Current online documentation may differ from a later selected installed build; verify that build/version again before implementation. Design choices such as quotas and role/effect semantics are SSHX proposals, not attributed upstream APIs.

| Source / exact URL | Reviewed section and limited finding | Consequence / unverified boundary |
| --- | --- | --- |
| [JSON-RPC 2.0](https://www.jsonrpc.org/specification) | Request/response, notifications, standard errors; transport agnostic | SSHX must define framing/identity/limits; no selected RPC library |
| [OpenSSH ssh](https://man.openbsd.org/ssh.1) | Synopsis, description and option semantics; remote command handling | Preserve native invocation and remote parsing; no installed-build test |
| [OpenSSH ssh_config](https://man.openbsd.org/ssh_config.5) | Source precedence, Match exec, Include and ControlMaster | Config evaluation may execute helpers; config/route/reuse still needs runtime proof |
| [OpenSSH sftp](https://man.openbsd.org/sftp.1) | Batch mode/noninteractive auth, options and commands | Candidate A has a distinct command language; auth/cancel/no-clobber not tested |
| [OpenSSH PROTOCOL](https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL) | SFTP version/extension announcement and section 4.3 POSIX-rename | Distinct rename semantics; no universal safe-commit claim; moving master, not a pinned implementation |
| [Referenced historical SFTP draft](https://www.openssh.org/txt/draft-ietf-secsh-filexfer-02.txt) | Sections 6.3, 6.5–6.6; exclusive-create flags, rename existing-destination failure and mkdir collision | Historical work-in-progress protocol used by OpenSSH reference; actual client/server race behavior still needs E4 |
| [Microsoft ConPTY session](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session) | Communication channel preparation, hosted process and lifecycle | Native Windows setup/drain/close ordering remains unverified |
| [Microsoft named-pipe security](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights) | Security descriptors, access checks and logon SID discussion | Set explicit ACL/peer/local-access policy; default ACL is not SSHX policy |
| [Linux unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html) | Socket credentials, SO_PEERCRED | Linux-specific identity facility; no macOS/Windows equivalence claim |
| [PNG specification](https://www.w3.org/TR/png-3/) | Format overview/chunks, truecolor/alpha; points to 2025 recommendation | SSHX narrows to static RGBA8 profile; encoder/validator implementation unselected |
| [arboard upstream](https://github.com/1Password/arboard) | Backend Support/Clipboard Ownership | Linux features/compositor/selection lifetime need actual tests; no crate/version/API selection |
| [kitty clipboard](https://sw.kovidgoyal.net/kitty/clipboard/) | OSC 5522 MIME reads/writes and capability detection | Optional terminal-specific extension, not a universal MVP image channel |
| [WezTerm Shell integration](https://wezterm.org/shell-integration.html) | OSC 7/133/1337 and explicitly configured Shell emission | Marker reference only; not complete/authenticated SSHX history/context |

Microsoft pages displayed an authorization banner but also returned the relevant article body; only visible technical text was used. No sign-in or bypass was attempted. All other listed pages returned their primary content. Browser source review does not settle library selection, desktop permission, server namespace, multiplexing, no-clobber behavior, target evidence or visual restoration.

The source's external project-name link is provenance for a deferred naming concern, not a researched release name decision. No naming research or dependency-license inventory is claimed for this architecture-only task. The retained source and exact existing notices remain unchanged.
