# ADR 0003: Select one OpenSSH-backed SFTP route after Phase 0

Date: 2026-10-22. Status: **B retained; cooperative mode A selected as the future upload contract, with runtime promotion still gated**. No backend is approved for mutable-parent/root capability guarantees. The scope decision and future hardlink candidate below do not change the historical Phase 0 result.

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

The selected route must satisfy [target/path authority](../protocol.md#targetconfig-and-path-authority), including an explicitly authorized cooperative scope, a demonstrated no-clobber publication and owned temporary resources. POSIX-rename is a distinct OpenSSH extension; do not substitute it for a verified no-overwrite primitive. [Pinned OpenSSH protocol extensions](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/PROTOCOL), reviewed 2026-10-22. No claim of universal rename atomicity/durability or remote hash verification follows.

## Selection and reopening gate

E4 compares candidate probes on the chosen fixture/server set. Pass requires verified target/config binding; key/agent/master and extra-auth results; special paths/namespaces; collision/symlink/full-disk/partial/cancel outcomes; bounded errors and no Shell prompt theft. Select one candidate only if all mandatory invariants pass; otherwise report a blocker and restrict the advertised fixture set or revisit the route explicitly. Record dependency/version/interface evidence only for the chosen exercised implementation. Reopen on a server/platform requirement that fails a load-bearing invariant, with a scoped comparison rather than silently shipping both backends.

## Phase 0 measured outcome and no-go boundary

This section preserves the accepted experiment-stage disposition. The subsequent product scope decision below narrows future planning; it does not turn these observations into general confinement or hardlink runtime evidence.

The Linux probes compare both routes against generated-key, strict-host-check OpenSSH fixtures. Candidate A's ordinary batch `put` overwrites an existing destination and exposes no demonstrated exclusive-create primitive for this contract; it is rejected as the implementation route. Candidate B uses system ssh subsystem pipes plus `russh-sftp 3.0.1` raw requests, with no second SSH stack. Actual exclusive leaf creation, baseline no-replace rename, special names, destination/symlink collisions, denied/quota writes, partial/cancelled streams, readback hashes and lost-reply read-only reconciliation are exercised in the [Phase 0 probes](../phase-0.md). The measured server is OpenSSH `1:9.2p1-2+deb12u10`; these results do not establish another server's rename/durability behavior.

Retain **B only as the cooperative-fixture experiment route**. No general MVP backend is promoted: SFTP v3 filename operations provide no directory-handle-relative parent capability. Exclusive leaf creation and no-clobber rename do not atomically bind every ancestor to the granted root; an operation directory owned by the same SSH UID can be renamed/replaced by another process with that UID. Pre/post lstat checks still leave a gap between requests. Arbitrary concurrently mutable parent/root support therefore remains a no-go requiring explicit capability refusal (`PATH_DENIED`/`COMMIT_UNSUPPORTED`) or a separately justified narrower environment contract. Do not label this invariant passed from the positive fixture cases, introduce a hidden overwrite/remote-command fallback, or solve it by silently installing a remote Agent.

The probe's frozen generated profiles and direct native jump/proxy/master/chroot observations have distinct scopes: success of a native helper route does not establish complete Core binding for arbitrary executable user configuration. Namespace mapping and real Shell edit context remain separately checked/refused; controlled raw-cat input is not a general Shell-insertion proof. Final promotion needs an explicit resolution of the unsupported root/route set and the independent task review. This negative feasibility result is useful evidence and is not a Phase 0 PASS or automatic Phase 1 approval.

## Subsequent decision: cooperative scope A

On 2026-10-22 the user selected A: trust the remote account/server to honor SFTP and keep the canonical upload hierarchy stable, with no same-account replacement or independent modification of operation-owned resources through cleanup. The root is a logical canonical SFTP namespace scope, not a jail or durable inode capability. A local user policy must explicitly authorize the actual target/namespace/root and existing `CapabilityGrant.path_scope.policy: "cooperative"`, bound by `scope_digest` and B. A plugin, manifest or remote report cannot grant this policy. Null/inapplicable path scope or a valid request requiring stronger confinement returns `PATH_DENIED` after common role/method-grant checks; malformed request fields remain `-32602`.

This resolves the product environment choice for future MVP planning only. The general same-account mutable-ancestor no-go remains unchanged. 0700, exclusive leaf creation, random names and pre/post metadata observations do not prove identity or close the race. Check containment/type/symlink/access and refuse observed inconsistency at each effect; undetected hostile replacement is outside the guarantee. Login, verified target/config binding, quotas/cancellation and Shell mapping/input context remain independent gates. No Agent, second backend/SSH stack, credential exposure or raw plugin executor is introduced.

## Future publication within B

Source review of OpenSSH 9.2p1 finds that baseline regular-file rename first tries linking but can use a racy stat/rename fallback for selected filesystem errors. The dedicated hardlink extension directly calls link, with removal a separate unlink. [Pinned implementation, process_rename/process_extended_hardlink/process_remove](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/sftp-server.c), reviewed 2026-10-22. This upstream review is not a byte-for-byte audit of the Debian binary and does not negate its recorded positive fixture result. It does rule out promoting that result to every filesystem.

Select verified `hardlink@openssh.com` version 1 as the **future regular-PNG publication candidate** in B: successfully close the exclusively created regular temp, link it to a new final basename in the same operation directory/filesystem, then separately unlink the owned temp. The final result retains that operation directory; no later directory rename is needed. Verify actual version and server/filesystem behavior before admission; advertisement alone is not proof. No baseline/POSIX-rename, scp or remote-command fallback is permitted. Unsupported capability/cross-filesystem failure refuses with `COMMIT_UNSUPPORTED`; a confirmed collision is `NAME_EXISTS`. Generic I/O or ambiguous responses keep their actual/indeterminate outcome instead of being universally mapped to either error.

An acknowledged final link is committed user data even if temp unlink fails (`cleanup_pending: true`). Never delete the final name to roll back. Lost publication acknowledgement is indeterminate, not insertable and not automatically replayed. After cancellation/revocation only Core's pre-existing exact owned-resource ledger can authorize narrow temp cleanup, never renewed plugin upload. Ownership/target/namespace/continuity loss or a new Core/generation leaves cleanup pending; no name adoption after reconnect, replacement-path deletion or recursive sweep. See the authoritative [publication](../protocol.md#future-regular-png-publication) and [cleanup](../protocol.md#owned-cleanup-authority) contracts.

## Remaining promotion gate

The historical probe still calls baseline `raw.rename`; its code, lockfile and source-bound reports remain unchanged, including `phase0_ready: false`. Recorded extension names do not prove version or dedicated hardlink execution. The locked `russh-sftp 3.0.1` public `RawSftpSession::hardlink` interface is available for later work; interface inspection is not an executed case.

A separately authorized runtime task must demonstrate cooperative-policy admission/denial and binding changes, actual hardlink no-clobber collisions, unsupported version/filesystem/cross-device refusal without fallback, lost-link reply without replay/input, confirmed-final plus failed-temp-cleanup, uncertain/replaced cleanup resources left untouched, and per-step cancellation/revocation with final files preserved. It must also satisfy the existing target/auth/namespace/mapping/context/platform gates on the advertised set. Until then the proposed publication is unavailable. This document delivery supplies input for a future scoped MVP plan; it grants no implementation/release or next-task authority and makes no Phase 0 PASS claim.
