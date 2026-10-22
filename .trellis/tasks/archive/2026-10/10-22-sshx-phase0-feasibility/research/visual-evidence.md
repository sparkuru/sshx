# Native image and visual feasibility evidence

Recorded 2026-10-22. This worker implemented only `probes/phase0/image-probe/` and these `visual-*` task records. Workspace/protocol/harness, wrappers, shared specs and lifecycle belong to other workers/main; no commits, protected runtime edits, host installs or personal display/credential access occurred.

## Results and commands

The final native run exited **0**: **20 passing cases and 1 rejected candidate**, retained in [visual-results.json](visual-results.json). The rejection is real: rendering the visible alternate screen does not reconstruct its hidden normal screen; returning to normal produces different independent xterm screens. This renderer is unsuitable as a complete attach implementation. It is not a failed check relabeled as full restoration success.

Actual focused commands, run from the repository root:

~~~text
./hako cargo fmt --manifest-path image-probe/Cargo.toml -- --check
./hako cargo clippy --locked --offline -p sshx-phase0-image --all-targets -- -D warnings
./hako cargo test --offline -p sshx-phase0-image
./hako cargo build --locked --offline -p sshx-phase0-image
readelf -h .devhome/target/debug/phase0-image
.devhome/target/debug/phase0-image --help
.devhome/target/debug/phase0-image suite --output /tmp/sshx-phase0-visual-final.json --artifact-dir /tmp/sshx-phase0-image-artifacts-final
.devhome/target/debug/phase0-image suite --output /tmp/sshx-phase0-visual-final-bound.json --binding-file /tmp/sshx-phase0-visual-binding.json
~~~

Formatting, strict Clippy and all **5 meaningful unit tests** pass. Native ELF is x86-64 and executes on Debian 13. Embedded selection helper is compiled on the host with `cc -std=c11 -O2 -Wall -Wextra -Werror ... -lX11`; this genuine compile and actual execution pass. Host Rust remains absent. Required Rust crates were fetched with `HAKO_NETWORK=bridge ./hako cargo fetch`; builds/tests subsequently use network none. An initial default-network-none fetch failed DNS explicitly. Original Rust image lacked rustfmt; the fixture worker's approved derived image resolved it, with final image ID `sha256:1b83a6dc7f4132d5cc36f44df414ce003935e0fb63508d806a988f0bd6aa6ef1`, Rust/Cargo 1.90.0, rustfmt 1.8.0 and Clippy 0.1.90.

Native package versions: xterm 398-1; Xvfb `2:21.1.16-1.3+deb13u4`; Vim `2:9.1.1230-2`; Xlib/runtime and headers `2:1.8.12-1`; GCC package `4:14.2.0-1`. `Xvfb -version` is unsupported; its installed package version is recorded rather than interpreting usage as a version result.

## Image boundary and measured allocations

Two new Xvfb displays use `-nolisten tcp`, independent private Xauthority cookies and mode-0700 runtime directories. Synthetic Client A and B have different image selections. Only A is captured for the operation. Changing A's selection afterward leaves the authorized raw bytes and independent plugin output unchanged. Alpha values 0, 127, 255 and 9 survive exactly. Empty, text-only, corrupt, owner-exit, absent/denied display, oversized property/dimensions and INCR cases run against these private displays only.

The PNG-only Xlib receiver first requests at most **36 property bytes**. It checks PNG signature/IHDR, dimensions, checked raw size and complete encoded property length before requesting the bounded payload. A 16,777,217-byte property and a forged `u32::MAX` width are refused after that 36-byte request, before any raw allocation. INCR is explicitly unsupported instead of accumulating it. Xlib cannot reliably distinguish authentication denial from missing/unreachable display: both are `CLIPBOARD_UNAVAILABLE`; a distinct `CLIPBOARD_DENIED` classification is **unsupported**, not claimed. No ordinary-user desktop, Wayland, Windows or macOS support follows.

The plugin process receives framed public RPC only, an empty owned HOME/cwd and minimal PATH/LANG. It receives no DISPLAY, Xauthority path, agent or SSH credentials. Its only project dependency is the public protocol crate. The tested pipeline is initialize/ready/register/action, clipboard handle, bounded raw chunks, independent PNG encode, bounded create/write/seal and metadata notice. Request counters are monotone and every fixture method checks the captured operation ID. Optional `--binding-file` supplies the exact Core B to `action.run` and boot negotiation; the machine evidence preserves B/digest. This public encoding fixture does not claim a complete Phase 1 broker or sandbox.

Strict preflight borrows encoded bytes, verifies CRC/order and permits only static noninterlaced RGBA8 IHDR, optional sRGB, IDAT and IEND. It refuses metadata/APNG, bad CRC/truncation/trailing bytes and mismatched dimensions. Encoded/output Vec reservations are checked before growth. Limits are 16 MiB encoded, 64 MiB raw, 8192 per axis and 16,777,216 pixels; lower individual limits do not establish an aggregate Core budget.

A real defect in the proposed validator path was reproduced: **png 0.18.1 `next_frame` plus `finish` accepted surplus decompressed pixels after shrinking IHDR to 1×1**. The fixed path independently streams the complete zlib stream with mature flate2 into an 8192-byte scratch buffer, requiring exactly `height * (width * 4 + 1)` filtered bytes and consumption of every compressed IDAT byte before allocating output. A lasting regression covers this case. Final measured largest allocation for an oversized dimension claim is **24 bytes**; the decompression-surplus rejection peaks at **43,365 extra Rust bytes**; a valid 2048×2048 decode allocates **16,777,216 output bytes**, with **17,058,936 extra Rust bytes** at peak. PNG's library limit is best effort; it alone is never cited as the hard-bound proof. Measurements cover this finite profile and fixtures, not all desktop decoders.

The independent plugin's exact sealed artifact is 80 bytes, SHA-256 `bb159bfa3401776c3357005c19fd7b42d62cb935bfa83461c2210d24ed4bd20c`; raw SHA-256 is `d9cd6270e5c6ecfed62f2bff9baabcd03ec7b3ca1b8618af3f7879e0f67e8d58`. Synthetic artifact `/tmp/sshx-phase0-image-artifacts-final/sealed.png` is intentionally retained for the Core's real SFTP/readback/input integration; Core evidence must independently establish remote hash equality and the actual initiating PTY. Image evidence alone does not establish that link.

## Visual comparison and separate recovery claims

Actual private xterm screen printing plus explicit cursor/mode queries provide the independent oracle; a parser comparing itself is not the sole evidence. Bounded vt100 state is rendered as selected safe cells/cursor/application-cursor/keypad/bracketed-paste/visibility state, never replaying OSC/history. Normal and visible alternate screen, cursor/modes, width shrink, explicit resize/redraw and suppressed replay of synthetic clipboard/title OSC pass for the recorded combination. The tested width shrink matches xterm 398; vt100 documents naive row truncation, and other emulators' reflow remains unverified.

A native task-owned PTY runs real Vim with user configuration, swap and viminfo disabled. A fixture file/explicit Vim command supplies readiness barriers. Resize and explicit redraw occur while the real process remains alive; captured original editor output is sent only to the independent test emulator as a baseline. Restoration uses selected safe state. Screen, cursor and queried modes agree with that independent xterm observation, Vim exits normally, and a separate cooperative redraw also reconstructs the tested screen. No raw-log replay is used as restoration.

At maximum proposed 500×300 dimensions, normal plus alternate state and **8,454,144 input bytes** use **9,619,200 extra Rust bytes**, below 16 MiB. Scrollback is zero; rendered state is 152,341 bytes. Oversized dimensions are refused before constructing the parser. vt100 uses default features off so vte's OSC storage is a bounded ArrayVec, not the std feature's growing Vec.

The hidden-normal-buffer case is a measured rejection. Text attributes, wrap-pending flags and scroll regions are not faithfully reconstructed by this deliberately limited renderer. Restricted redraw is viable only with a cooperating receiver, as demonstrated with Vim; arbitrary fullscreen attach remains a Phase 2 gap. Core-owned SSH survival after Client loss, network/Core failure and generation handling are **separate core harness results**, not inferred from this editor process or screen result. Machine reboot, physical IME, user terminal hosts and other native OS/display combinations are unverified.

## Source/API review and cleanup

Exact downloaded primary crate source/API review covered: png 0.18.1 decoder/preallocation/unfiltering paths; vt100 0.18.1 NonZeroU16 constructor/resize/Cell/safe state APIs; vte 0.15.0 feature-dependent OSC storage; base64 0.22.1 bounded chunk API; crc32fast 1.5.0 build script (only queries rustc version); flate2 1.1.10 Rust-backend bounded streaming decoder. png/vt100/fdeflate/flate2 declare no build script; no convenience install script or C dependency installer ran. Primary PNG API page: [png 0.18.1](https://docs.rs/png/0.18.1/png/). Version-specific VT/X11 web opens failed; downloaded crate sources, installed Xlib headers and xterm manuals supplied actual interfaces. The separate XGetWindowProperty manual page was unavailable. Xlib read lengths and xterm MC/query semantics were checked against actual native results.

An initial sandboxed Xvfb launch could not bind even local/Unix listeners; the approved native run used scoped escalation. No security setting was weakened. Bounded error reporting replaced an initially excessive Xvfb startup diagnostic. Earlier fixture bugs (buffered stdin/JSON frames versus kernel polling, editor color replies preceding explicit CPR) were corrected, and final native assertions pass. Earlier failed `/tmp` reports remain diagnostics, not selected passing evidence.

Final suite reports both owned displays stopped and private runtime removed. `ps -C Xvfb,xterm,vim,phase0-image -o pid,ppid,comm` returned only the header after the final run. Cookies, selection producers, xterms/printer children, Vim, plugin processes and temporary native sources/executables are removed through exact owned child handles/process groups and runtime paths. Only documented synthetic artifact/report files remain under `/tmp`; no process-name kill, port-wide cleanup, existing clipboard inspection/restoration or unrelated Docker cleanup ran.

## Context-loading evidence

Native injection was truncated. The full hook payload at `/tmp/hook_outputs/01a1635f-205c-7833-868a-800bc035510d/875dcb15-216f-4d42-b3de-9598fe09f3e1.txt` was read; it retained omission markers rather than omitted file contents. Fallback read `implement.jsonl` and all fourteen named inputs: shared `index.md`, `development-principles.md`, `sshx-contracts.md`, `development.md`, `validation.md`, `commit-policy.md`, `continuity.md`, verbatim `sshx-design.txt`; `.trellis/mainline.md`; `cross-layer-thinking-guide.md`; `docs/architecture.md`, `docs/protocol.md`, `docs/image-paste.md`; task `research/readiness.md`. The complete 211-line/46,819-byte protocol was directly loaded in bounded ranges 1–70, 71–140 and 141–230. PRD/design/implement were directly read, as were all six ADRs, platform matrix, Phase 0 runbook, guides index, common references and trellis-before-dev. Package discovery identified trellis-plus and no application packages. No child agents were spawned; no lifecycle/mainline operation was performed by this worker.

## Main-session final cleanup

The live-fixture/retained-integration wording above records the earlier implementation checkpoint. Subsequent main-session [final cleanup](cleanup.md) passed: both exact generated services, credential directories and pointers are removed, both loopback ports rebound successfully, and no task-owned container/process remains. Build caches/images and explicitly documented synthetic review artifacts are intentionally retained. No further fixture use is pending.
