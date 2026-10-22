# Planning evidence

Inspected 2026-10-22; no fixture startup, build, runtime experiment or implementation has occurred.

## Repository findings

- `probes/phase0/harness/src/sftp.rs:81`–103 drops extension values when constructing the wrapper. `:163`–166 calls raw.rename. Its Pipes fault assumes a rename packet at a write boundary; hardlink tests need precise framed extended-request/reply matching.
- `probes/phase0/harness/src/core.rs:48`–71 models operation outcomes with separate commit/content flags. `:920`–1136 owns real upload; its owns_dir/owns_temp cleanup lacks the newly documented explicit cooperative ledger continuity checks. `:1138` onward serializes admission before awaiting effects; preserve that useful cancellation boundary.
- `probes/phase0/harness/src/core.rs:700`–774 and suite callers use rename-specific hold steps. Change actual callers with the commit mechanism; do not claim unmodified historical tests exercise hardlink.
- The locked local russh-sftp 3.0.1 source exposes RawSftpSession::hardlink at `src/client/rawsession.rs:739` and version.extensions as a String-to-String map at `src/protocol/version.rs:9`. This was re-read locally, not assumed from a newer API. No new crate is needed for the selected request.
- Existing suite starts Core and Clients and accepts two fixture paths; main.rs already routes suite/core/plugin commands. `dev.sh` delegates to lifecycle.py, whose down verifies recorded container identity via hako. The archived dual-target report records its final cleanup; old services are not live prerequisites.
- Existing shared development profile records pinned container tooling and native probe ABI, but cache availability and current environment still need execution-time checks.

## Primary-source review

Reopened pinned [OpenSSH 9.2p1 extension protocol](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/PROTOCOL), section 4.5: hardlink is an extended request for regular files, advertised as version 1. Reopened [server implementation](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/sftp-server.c), process_extended_hardlink: it calls link and returns a portable status. Baseline process_rename has a filesystem-error fallback. These support the chosen experiment and conservative error handling, not runtime certification of a distribution build.

## Decisions and deferred technical evidence

Creation was approved with “创建”; the user then selected fresh local disposable Docker/Linux/OpenSSH fixtures with “采用”. Existing cooperative trust scope remains selected. Actual publication, collision, cross-device, denial and cleanup outcomes remain to be measured. Production capability discovery/persistence, general confinement, other OS/server combinations and full proposed-v1 API remain outside scope.

One task is appropriate because its policy, publication and cleanup assertions share Core state and one acceptance path. No planning sub-agent was needed; implementation and independent check will use the configured Trellis roles after final approval.
