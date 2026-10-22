# Phase 0 planning evidence

Date: 2026-10-22. Read-only inventory and primary documentation review only. No container/image was executed, image pulled/built, dependency installed, service started, desktop clipboard read or SSH target contacted during planning.

## Observed host and tooling

| Item | Actual result | Meaning / limitation |
| --- | --- | --- |
| Git/session | Clean `main`, previous three-commit batch present, no active task before creation | New task creation explicitly consented by the user; final-plan approval still pending |
| OS | Debian GNU/Linux 13, Linux 6.12.111+deb13-amd64, x86_64 | Primary planning target is this Linux environment; no Windows/macOS claim |
| Rust tools | `RUSTUP_AUTO_INSTALL=0 rustc --version` and equivalent Cargo call: exit 127, command not found | Absent from current PATH; no host installation attempted |
| OpenSSH | `ssh -V`: OpenSSH_10.0p2 Debian-7+deb13u4, OpenSSL 3.5.7 | Native client build; no connection/authentication result |
| SFTP | `sftp -h`: help/usage, exit 1; `/usr/lib/openssh/sftp-server` exists | Help is not a transfer; direct server process is not an SSH-authentication fixture |
| Docker | Ordinary sandbox socket access denied; approved read-only escalation reports client/server 26.1.5+dfsg1, linux/amd64 | Daemon available through the normal approval path; no policy/configuration weakened |
| Native helpers | C compiler/make, Python, Xvfb, xauth/xdpyinfo/xwd/xprop, script, tmux, xterm, Vim, ssh-agent/add/keygen, timeout, readelf/ldd, ShellCheck/shfmt present | Discovery, not execution/coverage; host sshd, xclip/xsel, vttest and Neovim absent |
| Session/display hints | Tool stdin/stdout/stderr are not TTYs; DISPLAY exists, WAYLAND_DISPLAY absent; local X11 socket 0 exists | Does not establish desktop permission, terminal fidelity or clipboard access; existing display is outside test scope |
| Application files | No Cargo manifest/lockfile, hako/dev.sh or root .gitignore found | Create actual probe commands before deriving a wrapper; `.trellis/.gitignore` remains protected |

Exact cached image metadata from read-only `docker image inspect`:

| Candidate | ID / repository digest | Observed platform / use |
| --- | --- | --- |
| rust:1.90-bookworm | `sha256:cc39c88add68c6e4ed377beccd6a659e59b286954d2c7e5c8e5b1dde2a82a8e8`; `rust@sha256:3914072ca0c3b8aad871db9169a651ccfce30cf58303e5d6f2db16d1d8a7e58f` | linux/amd64; proposed build image; Rust version inside has not run |
| debian:bookworm-slim | `sha256:d4c45f66efc59f89b0597d343a9bd72808493c0eacecde5285961743252cee23`; `debian@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587` | linux/amd64; proposed disposable SSH fixture base |
| debian:13-slim | `sha256:7e3898f7b011a107d0ef7393d5f604a6e0c0ff05ac4f2476630a8af21059ec9b`; `debian@sha256:d7e12182ce18b85b93007c1dedf31f2d29e01ccf3182cc4017c709b6259bc132` | **linux/arm64**, unsuitable as a native amd64 assumption; exclude from planned runs |
| python:3.13-slim | `sha256:9351a9a0a697a69e156f8bd067dd5ef3f9fa9b110abbdbbda1f343c622c7adc0`; `python@sha256:ffb752e139c0a19692a43af8d8523b274222dd68eebad5d583b45c2201c6e30a` | linux/amd64; available alternative; native Python suffices for the independent stdlib fixture |

Do not infer architecture or runtime/toolchain success from a tag. Record actual compiler/package versions, immutable image IDs, installed fixture package versions and lockfile after approved execution.

## Primary technical research

These findings guide experiments and are not runtime decisions or selected crate APIs.

- [vt100 Parser](https://docs.rs/vt100/latest/vt100/struct.Parser.html), opened 2026-10-22: displayed version 0.18.1; current constructor/resize use `NonZeroU16`. Its documented width-resize behavior truncates/extends rows and explicitly differs from many terminals. Test resize/reflow before any faithful attach claim; do not use an older search snippet's constructor signature.
- [png Decoder](https://docs.rs/png/latest/png/struct.Decoder.html), opened 2026-10-22: displayed version 0.18.1; custom decoder limits are described as best-effort and exclude caller-owned output allocations. Encoded byte/pixel/raw-size checks and actual allocation evidence remain mandatory; the method's existence cannot establish a hard memory guarantee.
- [russh-sftp candidate API](https://docs.rs/russh-sftp/latest/russh_sftp/client/struct.SftpSession.html): discovery found a stream-based SFTP session candidate. The version-specific page could not be opened through the web tool, so the exact selected interface/version must be reopened or inspected in its published source before implementation. No russh SSH stack is selected; only an SFTP protocol library on system-ssh pipes is in scope.
- [arboard candidate documentation](https://docs.rs/arboard/latest/arboard/): discovery exposes image/clipboard adapters, but version-specific page access failed and search snapshots disagreed on versions. Do not select an API from those snippets. Check the actual resolved version/source and its pre-allocation behavior before use; isolate experiments to a private synthetic Xvfb display.
- Existing architecture primary-source research remains applicable for OpenSSH option/authentication/SFTP and JSON-RPC boundaries, with exact deployed client/server behavior requiring the new probes.

Technical unknowns (dependency compatibility/network availability, clipboard allocation bounds, SFTP primitive safety and restoration fidelity) are experiment subjects. They are not grounds to install a host toolchain or waive an invariant. If no candidate satisfies an invariant, return an evidenced no-go/blocked decision rather than a success-looking fallback.

## Planning convergence / context readiness

The final PRD separates R1–R8, AC1–AC9, Linux/private-display scope and explicitly deferred native combinations. The design/execution artifacts describe actual planned file ownership, build/fixture/native boundaries, failure/cleanup and approval gates. This is one integrated shared-harness feasibility checkpoint, with no per-library task proliferation or automatic application continuation. No blocking user-owned decision remains beyond approval of that concrete execution proposal.

Both contexts contain fourteen real existing inputs and validate structurally. The validator explicitly warns that `docs/protocol.md` is 46,819 bytes, exceeding its 32,768-byte injection limit; the execution plan requires complete direct segmented reads plus PRD/design/implement and any other omissions. No runtime setting was changed. The baseline at `/tmp/sshx-phase0-planning-baseline.json` records 102 tracked files; only project-owned mainline is an intended tracked planning change. Task creation reported `main` as the checked-out/default fallback branch because no default remote branch could be resolved; it has not activated implementation or changed Git history.

Actual planning checks pass: eight planning/mainline files, five Markdown files, seventeen local link targets, valid task JSON and both unique existing-path JSONL manifests, eight requirements/nine acceptance criteria, well-formed recorded SHA-256 image IDs and text whitespace/newlines. `git diff --check` passes; status contains only mainline and this new task. Of 102 tracked baseline inputs, 101 remain unchanged and only the intended mainline differs; the canonical source is still 30,205 bytes with its original SHA-256. No staged paths exist and task status is `planning`. These are planning integrity checks, not passing E0–E7 or observed child-agent loading; actual execution remains behind final approval.
