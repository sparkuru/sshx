# Mechanism research for the cooperative upload contract

Reviewed 2026-10-22. Documentary/source review only; no new runtime or filesystem experiment. The intended document changes were subsequently approved by the user’s “批准”; no runtime execution follows from this source review.

## Historical SFTP v3 surface

The historical draft distinguishes filename-based OPEN/REMOVE/RENAME from handle-based I/O, and specifies exclusive creation and destination-collision refusal. These messages do not supply a directory-relative root capability. Treat that last conclusion as a mechanism inference, consistent with the accepted Phase 0 finding, rather than a new runtime result. [OpenSSH-hosted draft, §§6.3–6.5](https://www.openssh.org/txt/draft-ietf-secsh-filexfer-02.txt). This is the 2001 draft used for this protocol version, not a new standard.

## Version-pinned implementation finding

OpenSSH 9.2p1's regular-file baseline rename first tries linking, but selected filesystem errors lead to a stat/rename fallback explicitly identified in its source as racy. Its hardlink extension directly calls link; removal separately calls unlink. Therefore the previous fixture's positive baseline rename result cannot establish the same property on every filesystem. This upstream source review is not a byte-for-byte audit of Debian’s packaged binary and does not invalidate the recorded fixture observation. [Version-pinned implementation, process_rename/process_extended_hardlink/process_remove](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/sftp-server.c).

## Selected future commit candidate

The version-pinned extension document specifies `hardlink@openssh.com` version 1 and distinguishes POSIX rename semantics. For future regular-PNG publication, select one explicit link-to-new-final-name commit followed by separate owned-temp removal, with a verified server/filesystem capability. Unsupported/cross-filesystem results must refuse rather than fall back to overwrite-capable rename. [OpenSSH 9.2p1 protocol, §§4.2–4.5](https://raw.githubusercontent.com/openssh/openssh-portable/V_9_2_P1/PROTOCOL). This selection changes the planned commit primitive within backend B, not the SSH/auth backend.

This is a design candidate requiring future actual collision, cross-filesystem, lost-reply and cleanup tests. Advertisement alone does not prove support. The selected cooperative assumption does not establish stronger root confinement or make an unsupported commit safe.

## Repository and locked-library corroboration

- Accepted report `check-core-results.json`, case `sftp-backend-comparison`, records `hardlink@openssh.com` among observed extension names, but records no extension version or dedicated hardlink collision/cleanup execution. Do not call those future cases passed.
- The already downloaded locked `russh-sftp 3.0.1` source exposes `RawSftpSession::hardlink<O,N>(&self, oldpath: O, newpath: N) -> SftpResult<Status>` at `src/client/rawsession.rs:739`–755. It uses the public extension request path; no second transport stack or new dependency is needed for a future experiment. This is library-interface inspection, not proof that the existing harness calls it.
- Existing `probes/phase0/harness/src/sftp.rs:168` still calls baseline `raw.rename`. This task will not change or rerun that historical probe. Future implementation must deliberately use and validate the selected publication candidate before advertising it.

## Option disposition

| Option | Disposition for this task |
| --- | --- |
| Cooperative account/stable upload hierarchy | User explicitly selected A; trust precondition must be named and bound to local policy |
| Enforced protection against same-account ancestor replacement | Deferred/unsupported; original no-go retained |
| Backend B, system OpenSSH plus existing SFTP library | Retained direction; future upload eligibility requires separate verified commit capability |
| Baseline rename assumed safe from its name or one fixture result | Rejected as a universal admission rule; preserve measured historical result |
| Explicit hardlink publication plus separate temp cleanup | Selected future regular-file candidate; not implemented or runtime-verified here |
| POSIX rename fallback, implicit scp/remote command, second SSH stack | Excluded |
| Remote Agent, forced server administration or broad isolation framework | Excluded from the selected no-Agent document task |
