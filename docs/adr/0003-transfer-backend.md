# ADR 0003: Select one OpenSSH-backed SFTP route after Phase 0

Date: 2026-10-22. Status: **deferred by evidence gate**; no backend selected or implemented.

## Context and options

Basic login must not depend on SFTP. For transfers, source V offers two candidates while keeping system OpenSSH authentication/configuration:

| Candidate | Benefit to investigate | Required risk evidence |
| --- | --- | --- |
| A: system sftp subprocess | Small executable-driven feasibility probe | Batch command-language quoting, machine-readable error/progress limits, cancellation, exclusive creation and no-overwrite commit |
| B: system ssh opens SFTP subsystem; mature protocol library on pipes | Structured operations/handles without replacing SSH auth stack | Actual library framing/lifecycle/version support, OpenSSH subprocess I/O, server capability behavior and cancellation |

Do not build two complete backends or a provider framework. A small disposable probe of each risk is sufficient to choose one. A second SSH protocol stack is outside this comparison.

## Decision constraints

Official sftp documentation says batch use needs noninteractive authentication and exposes a separate batch command language. That supports an explicit AUTH_REQUIRED MVP result for additional interactive authentication. [sftp manual](https://man.openbsd.org/sftp.1), reviewed 2026-10-22. Main Shell credentials are never cached, replayed or read by plugins.

Configuration, identity and network connection reuse must be measured separately. -F, ssh -p versus sftp -P, -i, -J, ProxyCommand and applicable -o options require fixtures; RemoteCommand, forced RequestTTY and original forwarding must not be blindly duplicated. Unknown dynamic config/target binding is a refusal, not a guessed same-host auxiliary connection.

The selected route must satisfy [target/path authority](../protocol.md#targetconfig-and-path-authority), including a demonstrated no-clobber commit and safe owned temporary resources. POSIX-rename is a distinct OpenSSH extension; do not substitute it for a verified no-overwrite primitive. [OpenSSH protocol extensions](https://raw.githubusercontent.com/openssh/openssh-portable/master/PROTOCOL), reviewed 2026-10-22. No claim of universal rename atomicity/durability or remote hash verification follows.

## Selection and reopening gate

E4 compares candidate probes on the chosen fixture/server set. Pass requires verified target/config binding; key/agent/master and extra-auth results; special paths/namespaces; collision/symlink/full-disk/partial/cancel outcomes; bounded errors and no Shell prompt theft. Select one candidate only if all mandatory invariants pass; otherwise report a blocker and restrict the advertised fixture set or revisit the route explicitly. Record dependency/version/interface evidence only for the chosen exercised implementation. Reopen on a server/platform requirement that fails a load-bearing invariant, with a scoped comparison rather than silently shipping both backends.
