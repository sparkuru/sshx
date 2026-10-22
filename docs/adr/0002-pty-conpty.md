# ADR 0002: Core-owned PTY and ConPTY

Date: 2026-10-22. Status: **ownership decided; direct Linux PTY APIs exercised in Phase 0**; portable/ConPTY candidate unselected.

## Decision and context

The user-local Core owns system ssh children and Unix PTY/Windows ConPTY from the first C/S slice. Client owns display, input and desktop capability. This preserves live managed sessions when a Client leaves, without claiming remote-work recovery. Only classified interactive login uses this route; [native passthrough](../architecture.md#invocation-contract) keeps independent stdout/stderr for other modes.

A portable wrapper is a candidate, not evidence. Do not assert a portable-pty version/API or Windows runtime behavior from Unix code. Microsoft documents separate pseudoconsole channels and hosted-process setup. That motivates independent drains and close-order testing; it does not prove SSHX works. [Microsoft ConPTY session documentation](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session), reviewed 2026-10-22.

## Alternatives and consequences

Client-owned PTYs are rejected because Client loss would own the critical process lifetime. Forced PTY for every invocation is rejected because command/subsystem/binary/piped use has different semantics. Direct OS APIs versus a mature portable wrapper remain competing implementation choices; evaluate the smallest approach meeting E2 on the primary platform and recording other native gaps.

Terminal data stays bytes across reads. Ctrl+C received locally, byte 0x03 written to the PTY, local child signals and remote signals are separate operations. Preserve OpenSSH escapes and test the SSHX prefix/literal rules. Core/PTY survival, visual restoration and recovery after Core/network failure remain separate claims under [ADR 4](0004-screen-restoration.md).

## Selection and reopening gate

E1/E2 requires Unicode/IME, resize, Ctrl+C/Ctrl+D/job control, password/MFA, paste/full-screen behavior, exit status and terminal restoration. Native Windows must separately exercise process creation, I/O, resize and close ordering, including blocked output. Select a wrapper only after those results; unavailable platforms stay unverified. Reopen on a reproduced fidelity/deadlock/lifetime defect, not on cross-compilation alone.

## Phase 0 scope

The Linux harness uses direct `openpty`/controlling-terminal/resize APIs through libc, with close-on-exec descriptors and a Core-owned SSH child. Actual independent Client/IPC, terminal-control and failure cases are recorded by the Phase 0 harness; [cross-UID IPC](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/ipc-crossuid.md) separately verifies kernel DAC and peer identity. This selects the smallest experimental Linux adapter; it does not select a portable wrapper or establish ConPTY, physical keyboard/IME, user-terminal or hardware-MFA behavior. The private native-PTY Vim comparison in [visual evidence](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-evidence.md) is a separate visual-scope result.
