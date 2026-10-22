# E6 independent plugin and package evidence

Observed 2026-10-22. The native standalone command returned status 0 and [machine results](plugin-results.json) contain **31 passing top-level cases, zero failures**, including 14 real package archives in the package aggregate. This is E6 feasibility evidence for the explicitly limited probe; it is not a Phase 0 PASS, product installer or malicious-plugin sandbox claim.

## Actual boundary and method

`harness/src/plugin.rs` launches an independent `/usr/bin/python3 -I -S fixtures/plugin.py MODE` executable process, with `env_clear` and only `PATH=/usr/bin:/bin`, `LANG=C.UTF-8`. Python isolated/no-site flags prevent personal `sitecustomize`/user-site influence even when no HOME is passed. The fixture imports stdlib only, no Core-private modules or Rust code. Framed stdout has its own bounded reader; stderr has an independent bounded drain. No credential, SSH agent, desktop or raw PTY descriptor/body is passed.

The experimental broker uses the shared public Rust framing crate and actual child/channel-derived instance identity, version negotiation, monotone per-direction request counters, action registration/invocation/stop, scoped descriptions/refusals and bounded event/log/restart handling. It implements a denial-only transfer/desktop/input surface for these protocol tests; **actual Core upload/insertion and its authorization are separate E7/Core tests**, not established by a synthetic RPC error here. Fixture binding IDs are synthetic constants for this isolated broker; they are not product model allocation evidence. The Core worker owns retained Core model/grant/ticket churn and actual IPC effects.

A separately owned **native** PTY/OpenSSH child continuously ran a synthetic Python counter on the generated loopback fixture. Its byte drain is independent of plugin control/log channels. Every wire/failure case recorded actual `bytes_read` before and after and asserted an increase; the final run read 31,944 bytes, without exposing terminal bodies to any plugin/report. Fast failures were observed for at least 120 ms; blocked cases exercised actual 5-second handshake/frame and 10-second RPC deadlines. This is a live-process/output oracle, not a state-only counter.

Actual command:

~~~text
.devhome/target/debug/phase0-probe plugin --fixture OWNED_GENERATED_STATE --output /tmp/sshx-phase0-plugin-final31.json
~~~

It ran through the scoped native-execution approval path because the ordinary sandbox disallows required sockets. No policy/allowlist was changed. Case results were copied without credentials or payload bodies into `plugin-results.json`. Source hashes there describe evidence-capture sources; a help-only argparse/stderr styling addition followed the 31-case run, then isolated package-cases, help stdout exclusion and py_compile passed. The main full-scope review owns the final integrated rerun.

## Executed assertions

| Cases | Observed assertion |
| --- | --- |
| Unicode/split framing | Chinese UTF-8 body lengths and single-byte writes negotiated and registered correctly |
| Response order | Two actual pending Core requests, IDs 2 and 3, resolved in order 3 then 2; opposite-direction request IDs remained independent |
| Wrong role/identity/operation | Client upgrade refused `ROLE_DENIED`; self-assigned instance field refused `INVALID_PARAMS`; foreign operation refused `CAPABILITY_DENIED` |
| Revoked/composed grants | An actual owned image grant was active, then revoked deterministically after action acknowledgement and before handling the read; the known grant was refused. A read-only grant could not authorize upload composition |
| Unknown executor | Returned `METHOD_NOT_FOUND`; no executor or raw terminal method exists |
| Malformed framing | stdout log, duplicate header/key, bare LF, oversized declaration, batch, depth, trailing JSON, invalid UTF-8, BOM and nonfinite JSON were rejected; partial frame hit its deadline |
| Counters | Actual external repeated and skipped request IDs closed the offending channel; storage uses one high-water counter and a 16-frame bounded receive channel |
| Slow/crashed/incompatible peer | Handshake and blocked stop deadlines, exit 42 and wrong version remained isolated while the real SSH output drain advanced |
| Logs | 524,288-byte multi-line stderr flood retained 262,144 bytes and dropped 262,144; a 524,288-byte single line retained 4,096 bytes and dropped 520,192. Bodies were not recorded |
| Events | Producer generated 300 progress records while the external consumer paused; queue peak 128 records, compact queued-event byte projection peak 9,281, 256 discarded progress records; actual gap delivery led to `operation.get` requery. Envelope/heap accounting and Core-wide metadata are separate Core concerns |
| Restart ceiling | A bounded admission object allowed three actual crash children total (initial plus two automatic restarts), with observed 1/4-second backoffs; denied the fourth admission within 60 seconds; each child got a fresh random instance and no action replay |

An early baseline passed before self-review found the grant fixture merely used an unknown grant and the restart test only bounded its loop. Both were strengthened to actual grant revocation/admission enforcement before the final report above. Those early passes are not substituted for final evidence.

## Real package archives and limits

Python's stdlib `tarfile` creates real USTAR+gzip archives independently of the tested parser. Before reading any tar metadata, a streaming gzip pass rejects expansion above 32 MiB, with reads capped at 64 KiB, into a task-owned temporary file. A deliberately small header validator then checks checksums, canonical relative paths, duplicates, ordinary file/directory types, file count/size, a bounded manifest and executable entrypoint; GNU/PAX extended metadata is unsupported and rejected. No archive member is extracted and no install hook is executed. The reported `max_decompression_chunk=65536` is the configured read cap, not an allocator/RSS measurement.

Executed archive cases: valid; traversal; absolute path; duplicate name; symlink; hardlink; character device; escaped entrypoint; API mismatch; wrong platform; install-hook field; >64 KiB manifest; >256 members; and a 33 MiB gzip expansion. Only valid was accepted. Temporary archives/expanded files were removed by owned temporary contexts. This selects a bounded experimental USTAR+gzip validation subset, not universal archive compatibility or complete product manifest validation.

## Checks, cleanup and scope

`cargo check --locked -p sshx-phase0-probe` and builds passed in the derived Docker toolchain. Shell syntax/ShellCheck/shfmt, Python compilation, protocol-only stdout help and JSON/whitespace checks passed. Targeted Clippy initially surfaced three core-worker warnings in core/native/suite (reported to that owner); after their fixes, `cargo clippy --locked -p sshx-phase0-probe --all-targets -- -D warnings` passed (exit 0). No unrelated source was edited by this worker.

Every plugin child was killed/waited as necessary, with its stdout/logger threads joined. Normal stop got an actual acknowledgement; failures did not own or stop the SSH progress child. The owned progress PTY/OpenSSH child closed through its native RAII owner at run end. Package temporary contexts removed their files. The shared SSH fixture remains intentionally owned by integration and final teardown belongs to the main session; see [fixture evidence](fixture-evidence.md). [Cross-UID IPC](ipc-crossuid.md) separately tests actual native kernel DAC and SO_PEERCRED refusal.

Loaded context evidence is recorded in the fixture report, including saved hook truncation, all fourteen role inputs and complete segmented protocol direct reads. No lifecycle/spec/Git/next-phase action was performed. Remaining acceptance belongs to integrated Core/E7, native terminal/image/VT experiments, independent full-scope review and the actual human-review gate; none is inferred from these passing E6 cases.

## Main-session final cleanup

The live-fixture/retained-integration wording above records the earlier implementation checkpoint. Subsequent main-session [final cleanup](cleanup.md) passed: both exact generated services, credential directories and pointers are removed, both loopback ports rebound successfully, and no task-owned container/process remains. Build caches/images and explicitly documented synthetic review artifacts are intentionally retained. No further fixture use is pending.
