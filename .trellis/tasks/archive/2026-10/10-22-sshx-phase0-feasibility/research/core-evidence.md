# Native Core, SSH, SFTP and effect evidence

Recorded 2026-10-22. The independent checker's final frozen-source native run exited **0** in **66.513 seconds**, with **44 passing top-level cases and zero failures**. It includes 32 independent Python plugin cases and two actual private-image suites (each 20 passes and the explicit hidden-normal-buffer candidate rejection). The complete canonical machine report is [check-core-results.json](check-core-results.json); the compact [core-results.json](core-results.json) indexes it and the supplemental evidence without duplicating the large case body. Report SHA-256 is `c74bb4208be5f101b2a9cf77eb5c83cf1783326044f78bad419c3c891c23678f`.

This is a disposable Linux feasibility harness. It does not implement the complete Phase 1 public API or promote the product automatically: `phase0_ready` remains false, independently of `controlled_linux_cases_passed`.

## Commands and environment

Executed from the repository root, using the existing approved Docker wrapper for Rust and scoped native execution for generated loopback/private-display tests:

~~~text
./hako cargo fmt --all -- --check
./hako cargo clippy --locked --workspace --all-targets -- -D warnings
./hako cargo test --locked --workspace --all-targets
./hako cargo build --locked --workspace
.devhome/target/debug/phase0-probe --help
.devhome/target/debug/phase0-probe suite --fixture .devhome/phase0/09422203e8e5490890c2ddf05c2ff249/state.json --fixture-secondary .devhome/phase0/secondary/8a26f072521876cee2e63876fc9bd093/state.json --output /tmp/sshx-phase0-core-native5.json
.devhome/target/debug/phase0-probe suite --fixture .devhome/phase0/09422203e8e5490890c2ddf05c2ff249/state.json --fixture-secondary .devhome/phase0/secondary/8a26f072521876cee2e63876fc9bd093/state.json --output /tmp/sshx-phase0-check-core-final.json
git diff --check
~~~

Formatting, strict Clippy, locked build and **14 workspace unit tests** passed (5 image/visual, 4 harness prefix/OSC/seal, 5 public protocol), including independent final checks after strict parameter corrections. Source hashes remained unchanged during the independent native run and subsequent insertion supplement. Host Rust remains absent. No host install, personal credential/display access, outside target, Git staging/commit or task lifecycle action was performed by this worker.

Build image: `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1`, Rust/Cargo 1.90.0, rustfmt 1.8.0, Clippy 0.1.90, Linux amd64. Native execution is Debian 13 x86_64/Linux 6.12.111 with OpenSSH_10.0p2 Debian-7+deb13u4/OpenSSL 3.5.7. Controlled server image is `sha256:cb3eb061776d0835b474754593e692492c473a811607dbab68a815b02cedc70c`, Debian 12 OpenSSH `1:9.2p1-2+deb12u10`. Exact infrastructure provenance/ownership is in [fixture evidence](fixture-evidence.md).

Locked exercised dependencies: russh-sftp 3.0.1 (SFTP protocol only, on system-ssh pipes), Tokio 1.54.1, libc 0.2.191, Serde 1.0.229, serde_json 1.0.154, SHA-2 0.10.9, png 0.18.1, crc32fast 1.5.0 and flate2 1.1.10. The selected downloaded raw-session source exposes actual asynchronous open/write/lstat/mkdir/close/rename methods. No second SSH implementation was added. [Visual evidence](visual-evidence.md) records the exact PNG/VT/X11 API and allocation investigation.

## Observed Core and native effects

| Case group | Actual observed assertion |
| --- | --- |
| Native IPC and lifetime | Two different Client processes negotiate one private Core; a second Core start refuses to replace it. Writer contention, loser input/resize and busy stop fail explicitly. Client kill leaves its Core-owned SSH alive; a new writer obtains a new epoch. |
| Per-Client switching | Starting or attaching another Session atomically revokes that Client's prior writer, tickets and future operation effects. Old input/resize/upload fail while the old SSH remains alive. Invalid target/receiver types are refused before fallback, spawn or writer revocation. |
| Retained identity and caps | Seven malformed/skipped trigger counters are refused before counter advancement; a legitimate next counter works. Retention-removal leaves the Client high-water counter intact and old intent cannot recreate an operation. Three hundred grant/ticket renewals replace owned records, consumed tickets cannot replay, and retained Session/Client churn reaches 32/16 admission caps. |
| Byte and terminal behavior | Actual raw remote cat returns ASCII/Chinese/emoji bytes. Prefix split/action/cancel/timeout/literal behavior and repeated-ESC/leading-zero OSC regressions pass. A native controlled bash exercises resize 101×37, foreground Ctrl+C, Ctrl+Z/job cleanup and exit 23. Direct/passthrough binary stdout, separate stderr and exit 7 agree. |
| Authentication and routes | Generated-key and owned-agent authentication work; a synthetic password passes through the owned OpenSSH PTY without logging it. A wrong independently generated expected host key is refused. Trusted synthetic ProxyJump/ProxyCommand routes and a live master with fresh authentication disabled support actual SSH/SFTP reuse. Chroot readback uses its distinct namespace. |
| Local trust negatives | Endpoint symlink remains untouched. Unsupported Match/Include/helper profiles are refused before helper execution. The separate [cross-UID report](ipc-crossuid.md) observes actual kernel DAC and SO_PEERCRED refusals; this suite does not pretend to rerun those peers. |
| Cancellation admission | Every new SFTP connection/lstat/mkdir/open/chunk/close/rename/readback step first enters the same Core serialization domain as cancel/revoke/switch. Barriers before mkdir/open/write/close observe unchanged admission counts after cancel and actual remote directory absence. Revocation after one real written byte observes that byte first, then owned partial cleanup and zero insertion. |
| Fault-control bounds | Oversized (65,536-byte), unknown and non-string pause labels, plus a wrongly typed stream flag, fail before retaining metadata or consuming the upload key. The legitimate same-operation/key request then runs. These are test controls, never arbitrary config/path authority. |
| Same/different target concurrency | Two Sessions on one server hold distinct actual partial PNG files concurrently, then distinct final hashes and initiating PTY insertions. A single Core also holds primary and secondary server partial files concurrently, verifies distinct hashes on their original servers and absence on the opposite server, and writes each path only to its initiating PTY. Detach/switch during a secondary partial stream removes only its partial; the new primary Session receives no stale path. |
| Actual image pipeline | Core B exists before capture admission. The independent image executable creates private synthetic Xvfb selections, passes that exact B through public plugin framing, emits an exact PNG artifact, and must exit successfully before Core accepts it. Core independently validates the sealed bytes; actual SFTP readback and actual initiating PTY output agree. A second capture admission is refused. |
| In-flight capture cancellation | Cancellation/switch occurs after the real image subprocess starts and before completion. Its bounded capture/encode may finish; Core rejects the late upload and a new capture. Actual remote directory remains absent and the new PTY receives no late path. This is a process-stage barrier, not an instrumented native selection-read microstep. |
| Independent effect results | Zero-byte context refusal can refresh on the same operation/key without another upload. Arbitrary text fails validation. Partial actual PTY writes stay partial and never replay. A lost/unknown insert reply returns the ledger outcome; an actual closed IPC reply still yields exactly one observed path occurrence and retained status accessible to the user. |
| Commit and cancellation | Before-commit cancellation removes only owned partials. Cancellation/revocation after confirmed commit retains exact final bytes and prohibits insertion. Commit acknowledgement is distinct from verified readback; only a readback-verified completed result permits insertion. A received SFTP rename reply deliberately discarded by the pipe adapter produces uncertainty, then a fresh read-only known-path reconciliation observes final hash and absent temp without reupload. |
| Failure recovery | Terminating only the owned SSH channel fails old operations; explicit reconnect starts generation 2 and old results cannot insert there. A forced owned Core crash loses IPC/B; a fresh Core starts empty with a different boot ID. A process-local subreaper reaps only the exact orphan SSH child. No PTY adoption or remote-work recovery is claimed. |

The blocked receiver accepted **2,124,800 bytes** before a nonblocking input returned `BUSY` in the independent final run. Cancel plus status then completed in **0 ms** at millisecond resolution. Output overflow drained **4,194,316 bytes**, retained **0 bytes** after marking incomplete, and had already revoked the writer before querying output. The ring capacity is 1,048,576 bytes; `probe.output` emits at most 32,768 bytes, keeping its JSON below BODY_MAX. These are measured syscall/drain assertions, not a physical-terminal throughput guarantee.

## Supplemental current-writer insertion negatives

[core-insert-negative.json](core-insert-negative.json) records **3 passing focused cases** against a new native Core and the same explicit generated primary fixture. The temporary stdlib-only driver was `/tmp/sshx-core-insert-negatives.py`, SHA-256 `c6b2919515413f806e86f0a1997f35d6053c491140a56a58023de3c958a8d45a`; actual commands were `python3 -m py_compile /tmp/sshx-core-insert-negatives.py` and scoped native `python3 -I -S /tmp/sshx-core-insert-negatives.py`. The Python skill was read before writing this temporary driver; no project Python source/dependency was added.

Its method is explicit: create a mode-0700 runtime, launch the frozen native Core with only the approved fixture, negotiate real framed Unix IPC, start the Core-owned raw SSH receiver, generate a 1×1 static RGBA PNG with stdlib zlib/CRC, and upload via the actual Core/SFTP barriers. For each test the confirmed result requires readback verification. Safe but arbitrary text is refused `CAPABILITY_DENIED`, unknown mapping is refused `MAPPING_UNKNOWN`, and NUL/ESC/LF/C1 suffixes are refused `PATH_DENIED`; all leave actual PTY output unchanged. After the four control refusals, the valid same-key path writes exactly **68 bytes once**, and repeating it adds no bytes. No terminal/clipboard body or credential is recorded. The driver closes its Session/Core, removes exactly its three known canonical final files/directories through the generated fixture's SFTP profile, and removes its private runtime; all cleanup assertions pass.

The first isolated driver attempt returned `TimeoutError` after one verified upload, before per-method diagnostics existed. [The initial diagnostic](core-insert-negative-initial.json) remains a failure record with successful exact cleanup. Adding redacted last-method/operation-stage diagnostics to the temporary driver and rerunning the same focused cases yielded the three passes above, without any harness source change. The initial timeout was not reproduced and its cause is **unestablished**; this report does not claim an identified bug was fixed or relabel the failed attempt as a pass.

## Transfer and restoration decisions

The small system-sftp A probe actually overwrites an existing destination via ordinary `put`; no exclusive-create contract was established for that command surface, so A is rejected for this transaction. B uses system ssh plus russh-sftp's raw interface: exclusive open and baseline rename refuse an existing file; existing bytes remain unchanged. Nine special names (space, Chinese, quotes, dollar, backtick, semicolon, leading hyphen, newline and Windows-like drive spelling) survive actual readback. Newline/unsafe controls remain uninsertable. Symlink collision cannot overwrite its target. Denied-root and 1 MiB tmpfs/full-stream negatives run on disposable server storage. No-SFTP and extra noninteractive-auth attempts fail without stealing the main terminal; these underlying outcomes are not a released typed AUTH_REQUIRED/NO_SFTP API.

**B is viable only for the measured generated controlled roots/server set.** Lstat and an exclusive private directory do not defeat a malicious same-UID remote ancestor swap. No general hostile-root, dynamic helper closure, Windows namespace, durability or atomic-persistence claim is established. Unsupported source configuration is rejected rather than silently following a changed target. A broader product backend selection must retain these constraints or remain blocked.

Visual restoration remains the separately measured [restricted cooperating redraw decision](visual-evidence.md). A full alternate/normal-buffer reconstruction candidate was actually rejected. Core survival, visual fidelity and remote recovery have separate evidence; a working PTY or same-host reconnect does not establish full attach or reboot recovery.

## Implementation and evidence limits

- `probe.*` methods, explicit synthetic receiver/mapping attestations, test-clock expiry/removal and Client-driven byte uploads are scoped fault controls. They are not the entire v1 schema, a production clipboard broker, real Shell context, product installer or OS plugin sandbox. The independent Python broker exercises denial/queue/package boundaries; the actual Core effects have separate tests.
- `probe.upload` also accepts nonimage synthetic byte fixtures for fault cases; the real image path independently validates exact PNG bytes and B. This is not permission to expose a general plugin file uploader or bypass sealed-blob ownership in Phase 1.
- Per-record caps and conservative metadata charges bound this harness. Profile/Session charges include bounded manifest/path clones; config reads cap at 64 KiB and known-host snapshots at 16 KiB. PTY rings, frames and plugin queues have separate limits. This is not an allocator/RSS proof or complete accounting for every aggregate proposed v1 queue/blob allocation.
- Current native schema does not implement every management invocation classifier, terminal host, grant/event field or dispatched-race permutation. The passthrough observation compares a binary remote-command case; arbitrary argv forwarding is implemented, but the complete product CLI mode matrix is not claimed tested.
- Physical IME, real desktop permissions, hardware keys/MFA, Windows/macOS/Wayland, native non-Linux servers, reboot and generic nested-Shell mapping are explicitly unverified outside these approved fixtures. They are not passing cases or merely renamed required Linux failures.

## Cleanup and prior diagnostics

The independent final report confirms its Core socket/frozen configurations removed normally, exact lock removed, and private Core runtime removed. Per-case Client/Core/PTY/agent/plugin/image/editor children use owned handles; forced Core crash reaps the exact orphan PID. Known successful synthetic files are explicitly removed after assertions; cancellation removes only resources whose exclusive creation established ownership. The independent image suites stop their private displays and delete their own authority/runtime files. The insertion supplement also reports exact file/Core/runtime cleanup. This worker has signaled **no further fixture use**; **main owns final exact container/credential/loopback teardown**, recorded separately. No service-name/process-name kill, global prune or personal-state restoration occurs.

Historical `/tmp/sshx-phase0-core-first.json` is a sandbox-socket denial, not success. Earlier native1 failures were corrected; native2/native3 contain narrower passing subsets and stale gap rows, not the final report. Native4 passed 43 cases before the final same-target case and result bookkeeping; native5 passed 44 before the final strict schema checks. Review findings fixed repeated ESC leakage, per-step cancellation, blocking PTY I/O, image-child success admission, target/profile typing, canonical counters, conservative metadata charging and bounded fault controls. The independent final canonical report corresponds to the final reviewed source hashes below.

## Final source inventory

Continuation edits only `harness/src/{core,suite,native,main}.rs` (sftp/seal retain predecessor work), plus these `core-*` records. The prior implementer created workspace/protocol/SFTP/seal files; checker-owned protocol/plugin corrections and all fixture/visual/docs/spec/lifecycle work are preserved. No Cargo manifest/lock update was needed during continuation.

| Source | SHA-256 after final schema fix |
| --- | --- |
| `harness/src/core.rs` | `9b5d225cae6c36348d533de14120a66c420ce18110017c2e84b3c8e37c24c639` |
| `harness/src/suite.rs` | `004f8a2673b0acfccde670f94afa86d12c384a6346e1e4b2887dda9c69c09adc` |
| `harness/src/native.rs` | `ded4cfea10da2ad0429c7933e892fd003edc809acd2ffab54e3a457d4bb576ab` |
| `harness/src/sftp.rs` | `70f347d984dcadd174218607b115db9dc540527ce0e66caefe8ba4192f06e600` |
| `harness/src/seal.rs` | `79830aba3e857a6e961a1fc89765a40d94303383c376f88827b4e10f968f0c2c` |
| `harness/src/main.rs` | `b7d5621218ecb0d091d0fb1bd781fd5d175054b92b5abcf3c43b03e9ce16be9b` |
| `Cargo.toml` | `447b6f0f99b54c0f2ea4bab7c931555110d22b72456fc80e9f4f7cdaddd7ff32` |
| `Cargo.lock` | `0a274b8219488db7067d87b3a0d706f0925a6e4acb5784fb058d6537daf7ee45` |

Complete context loading evidence is in [core-context.md](core-context.md). Main owns the overall acceptance/source-XII matrix, review gate, readiness decision and next authorization; passing this bounded harness does not independently close those gates.

## Main-session final cleanup

The live-fixture/retained-integration wording above records the earlier implementation checkpoint. Subsequent main-session [final cleanup](cleanup.md) passed: both exact generated services, credential directories and pointers are removed, both loopback ports rebound successfully, and no task-owned container/process remains. Build caches/images and explicitly documented synthetic review artifacts are intentionally retained. No further fixture use is pending.
