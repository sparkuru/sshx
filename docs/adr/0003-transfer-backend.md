# ADR 0003: Select one OpenSSH-backed SFTP route after Phase 0

Date: 2026-10-22. Status: **B retained for cooperative Linux experiments; general MVP promotion blocked**. No backend is approved for mutable-parent/root capability guarantees.

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

## Phase 0 measured outcome and no-go boundary

The Linux probes compare both routes against generated-key, strict-host-check OpenSSH fixtures. Candidate A's ordinary batch `put` overwrites an existing destination and exposes no demonstrated exclusive-create primitive for this contract; it is rejected as the implementation route. Candidate B uses system ssh subsystem pipes plus `russh-sftp 3.0.1` raw requests, with no second SSH stack. Actual exclusive leaf creation, baseline no-replace rename, special names, destination/symlink collisions, denied/quota writes, partial/cancelled streams, readback hashes and lost-reply read-only reconciliation are exercised in the [Phase 0 probes](../phase-0.md). The measured server is OpenSSH `1:9.2p1-2+deb12u10`; these results do not establish another server's rename/durability behavior.

Retain **B only as the cooperative-fixture experiment route**. No general MVP backend is promoted: SFTP v3 filename operations provide no directory-handle-relative parent capability. Exclusive leaf creation and no-clobber rename do not atomically bind every ancestor to the granted root; an operation directory owned by the same SSH UID can be renamed/replaced by another process with that UID. Pre/post lstat checks still leave a gap between requests. Arbitrary concurrently mutable parent/root support therefore remains a no-go requiring explicit capability refusal (`PATH_DENIED`/`COMMIT_UNSUPPORTED`) or a separately justified narrower environment contract. Do not label this invariant passed from the positive fixture cases, introduce a hidden overwrite/remote-command fallback, or solve it by silently installing a remote Agent.

The probe's frozen generated profiles and direct native jump/proxy/master/chroot observations have distinct scopes: success of a native helper route does not establish complete Core binding for arbitrary executable user configuration. Namespace mapping and real Shell edit context remain separately checked/refused; controlled raw-cat input is not a general Shell-insertion proof. Final promotion needs an explicit resolution of the unsupported root/route set and the independent task review. This negative feasibility result is useful evidence and is not a Phase 0 PASS or automatic Phase 1 approval.
