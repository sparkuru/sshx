# Phase 0 acceptance and source-fault map

Status: controlled Linux implementation, independent verification and final cleanup are complete. The independent run passes 44 top-level native cases and 14 unit tests; the current-writer supplement passes 3 cases. The general SFTP parent/root capability remains no-go. On 2026-10-22 the user explicitly accepted this negative feasibility deliverable and the concrete local Git/archive batch. AC4/AC9 are accepted for this controlled scope; this is not Phase 0 PASS or MVP readiness. Actual task closure is recorded only after supported archival. See [formal check](../check-report.md), [canonical native report](check-core-results.json) and [cleanup](cleanup.md).

## Requirements and acceptance

| Requirement / AC | Observable evidence | Disposition |
| --- | --- | --- |
| R1 / AC1 | Real protocol/harness/image workspace and lockfile; pinned Docker toolchain, narrow source/cache mounts; native ELF ABI smoke; exercised binary help; fmt/Clippy/unit/build commands | Independent whole-workspace fmt/strict Clippy/typecheck/build and 14 unit tests pass |
| R2 / AC2 | Separate native Core and two Client processes; writer/resize contention, one active attachment per Client, passthrough binary stdio/exit, startup lock/endpoint refusal, Client-loss survival and bounded output/input | Controlled native assertions; [actual cross-UID DAC/peer refusal](ipc-crossuid.md) supplements the suite |
| R3 / AC3 | [Private-X11/PNG report](visual-evidence.md): one triggering-Client snapshot, immutable raw/sealed bytes, alpha, preallocation/property/decompression bounds, corrupt/oversize/empty/text/missing/owner-exit cases; independent image encoder | Finite PNG-only X11 profile; INCR refused, denied/unavailable classification indistinguishable; other desktops unverified |
| R4 / AC4 | Actual A/B SFTP comparison, strict generated key/agent/password/host checks, leaf collisions, exclusive open/rename, special names, denied/quota/cancel/partial/reply-loss/readback; direct route/master/chroot observations | **General parent/root capability is no-go.** Cooperative fixture leaf operations pass; [ADR 3](../../../../docs/adr/0003-transfer-backend.md) blocks general MVP backend promotion. Negative comparison outcome explicitly accepted by the user; no general backend promoted |
| R5 / AC5 | Independent private-xterm screen/cursor/mode oracle, real Vim/PTy resize, VT budget, cooperating redraw; Core/SSH lifetime and Core/child-loss observations separate | Restricted cooperating redraw selected only for demonstrated receivers; hidden-normal-buffer restoration rejected, attributes/wrap/scroll-region fidelity unavailable; reboot unverified |
| R6 / AC6 | [Independent plugin evidence](plugin-evidence.md), final [32-case queue-capped rerun](check-plugin-results.json), actual SSH progress, 14 real bounded archives; Core grant/ticket/retained-record tests and protocol regression tests | Focused trusted-plugin broker/USTAR subset. No full product manager/installer, same-user sandbox, complete v1 API or total-RSS claim |
| R7 / AC7 | Real bound-image bytes reach original target and PTY; separate target identities/filesystems; same/different-target concurrent operations, capture/stream/open/commit/insert barriers, cancel/revoke/disable/expiry, new generation, trigger high-water, partial writes and actual lost responses | Actual effect assertions within controlled profiles; fixture edit-context flags are explicit test controls, not real Shell trust. Generic mutable-root guarantees remain blocked |
| R8 / AC8 | Versioned per-case machine reports, commands/versions/hash/counter observations, original failure history and owned resource records; all XII groups below accounted for | Independent report/data/integrity checks and exact final cleanup pass; no secret or terminal bodies in ordinary logs |
| AC9 | Independent full-scope source/spec/check report, exact final Rust/Shell/Python/data checks, source/notice/runtime fingerprints and human-review classification | Independent code/check review passed for the controlled subset. [Required human review](review-gate.md) was explicitly supplied by the user for the controlled auth/permission probes and negative general-root finding; the listed local batch is approved |

## XII.1–12 coverage

| Source group | Executed evidence / explicit boundary |
| --- | --- |
| XII.1 native ssh / plugins removed | Native passthrough preserves separate binary streams/exit; no installation, alias or system ssh mutation. Plugin crashes/disabled states do not own SSH. Actual release packaging remains Phase 1 |
| XII.2 config/auth/target | Generated strict known-host/key/password/owned-agent; changed-key refusal; frozen literal profile plus helper/config refusal; actual native jump/proxy/master/chroot observations. Arbitrary executable-config binding and non-Linux targets unverified |
| XII.3 terminal | Native PTY Unicode/binary/prefix/control/resize/exit plus private Vim/xterm. Physical keyboard/IME, host shortcut interception and ConPTY unverified |
| XII.4 concurrency / switch | Actual same-target and two distinct generated server identities, per-Session frozen profiles and real remote hashes/PTY destinations. Two containers on one Docker host are not two physical machines |
| XII.5 lifetime / stale effects | Client loss, detach, new generation, Core crash/new boot, disable/revoke/expiry and effect barriers. Owned SSH child/channel loss is labeled precisely; no true partition/reboot recovery claim |
| XII.6 image / transfer failures | Empty/text/corrupt/large/missing/owner-exit X11; no SFTP/additional auth, collisions/symlink leaf/denied/quota/partial/cancel/reply loss. Concurrent parent/root replacement atomicity is the explicit no-go |
| XII.7 paths / injection | SFTP special-name readbacks and exact recorded-path text checks; controls/unknown mapping/context refused. Real Shell encoders/nested mappings/Windows path equivalence remain future scoped work |
| XII.8 independent retry | Same upload/insert keys, changed intent refusal, zero-byte refresh without second upload, retained counter after expiry, actual partial/lost delivery without replay |
| XII.9 faulty plugin / package | Framing/version/identity/role/counters/deadlines/crash/slow/log/queue/event/requery/restart tests plus 14 real USTAR+gzip archives; full installer/update product not implemented |
| XII.10 local / remote authority | Actual cross-UID kernel DAC and SO_PEERCRED; terminal OSC gate regressions; scoped public denial-only plugin broker plus Core effect guards. No same-user malicious-code confinement claim |
| XII.11 history | Reserved/unknown methods refused; no fake terminal-derived history. Opt-in structured hooks/filter/pause/retention/delete remain Phase 3 acceptance |
| XII.12 recovery | Process survival, selected visual reconstruction and owned Core/SSH failure are separate observations. Full attach/hidden screen/modes remain Phase 2; reboot and remote-work resurrection unverified |

## Interpretation

Passing case assertions do not convert a known unsupported invariant into support. In particular, a cooperative SFTP leaf collision/rename test is not an atomic ancestor capability proof. The user explicitly accepted Phase 0 as this precise negative feasibility deliverable and authorized its concrete local Git/archive batch. General MVP promotion still requires a separate capability decision and implementation; no later product phase is authorized.
