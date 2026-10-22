# ADR 0004: Separate process survival, visual state and remote recovery

Date: 2026-10-22. Status: **recovery boundary decided; restricted cooperating redraw selected for the measured prototype scope**.

## Decision and alternatives

Core/PTY/ssh survival after detach does not prove a restored screen, cursor, modes or remote job. A Core crash does not promise PTY adoption; reboot/network recovery needs separate evidence and possibly remote tmux/Agent support. Persisted metadata is not a live session. Reconnect creates a fresh generation and invalidates old effects.

Phase 0 E5 compares:

| Strategy | Required acceptance / cost |
| --- | --- |
| Bounded VT state model | Correct normal/alternate screen, cursor, resize and negotiated modes for an explicitly listed subset; bounded parser/state/render budget and no side-effect replay |
| Restricted redraw | Explicitly limited restoration based on verified receiver redraw/reset; never claim full Vim/alternate-screen reconstruction; disclose unsupported transitions |

Raw byte-log replay is rejected as the restoration mechanism: it is not a reliable snapshot and may repeat terminal effects. Neither alternative can merely hide gaps behind a “reattach succeeded” message. [Protocol](../protocol.md#limits) bounds state/output; output loss marks display incomplete and prevents pretend fidelity.

## Evidence and consequences

This boundary follows [source IV, IX, XI–XII](../../.trellis/spec/trellis-plus/sshx-design.txt), not executed experiments. Official ConPTY documentation describes host channels/lifecycle but does not establish a complete restoration design. [Microsoft session documentation](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session), reviewed 2026-10-22. This ADR makes no external emulator capability claim.

A safe renderer must emit only selected visual state and omit clipboard, notifications and arbitrary past OSC side effects. Live output and reconstructed output have different trust handling. Document exactly which terminal modes are supported, what unknown sequences do, and how overflow recovers; unsupported restoration is explicit rather than unsafe replay.

## Selection and reopening gate

E5 records detach/window-close/Client-crash survival, normal and alternate screen/resize/Vim restoration, network loss, Core crash and reboot separately. Select the smallest strategy whose demonstrated scope meets the next phase; full switching/attach acceptance remains Phase 2. Reopen when a required mode fails or a bounded parser cannot represent the supported host. Missing native environments are gaps, not evidence against one option.

## Phase 0 measured decision

[Native image/visual evidence](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-evidence.md) compares both routes on Debian 13/private xterm 398 and native-PTY Vim. Selected safe visible-state rendering matches the tested normal/alternate screen, cursor/modes and Vim resize fixture; maximum VT state measured 9,619,200 bytes below 16 MiB. It rejects full attach: leaving a reconstructed alternate screen loses the hidden normal buffer, and attributes, wrap-pending flags and scroll regions are not faithfully reconstructed. A successful visible Vim comparison does not remove those limitations.

Use **restricted cooperating redraw** as the prototype strategy only where the receiver and redraw action are explicitly known, as in the controlled Vim fixture. Unknown receivers offer no automatic full-screen reconstruction. Keep the bounded VT renderer as an experiment, not the full attach implementation. Generic application switching/full attach stays a Phase 2 blocker until hidden state and required terminal modes have independent evidence. Renderer output excludes past OSC side effects. Core/SSH lifetime and network/Core recovery remain separate observations in the Core report; no reboot or other-platform recovery follows.
