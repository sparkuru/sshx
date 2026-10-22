# ADR 0001: Rust Core and system OpenSSH

Date: 2026-10-22. Status: **source default retained and exercised by bounded Linux Phase 0 probes**; product/platform promotion remains evidence-gated.

## Decision and context

[Source I, X](../../.trellis/spec/trellis-plus/sshx-design.txt) defaults to Rust for CLI/Core and public types. Preserve that baseline rather than presenting it as a separately mandated user language. The user approved the architecture package after its language explanation. System OpenSSH owns SSH configuration, authentication and host verification. All business plugins are independent executables speaking public RPC; their language is not prescribed. Core must not link to a concrete plugin.

Use the small source-proposed boundaries in [architecture](../architecture.md), not a crate per concept. Tokio, portable-pty, Serde/TOML, Ratatui/Crossterm, arboard and SQLite remain candidates. The original architecture task selected no dependencies. Phase 0 now has a separate experimental [workspace and lockfile](../../probes/phase0/Cargo.toml); remaining product dependencies are not implied by those probes.

## Alternatives and consequences

Go or another systems language can be reconsidered if measured primary-platform blockers outweigh the Rust baseline. Python/other languages are valid independent plugin choices. A single-process script would reduce initial work but fails the required Client/Core ownership and independent-plugin boundary. A second SSH implementation would duplicate authentication/config semantics and is outside the proposed MVP.

System OpenSSH adds an explicit deployed-system dependency and version/platform behavior to record. Local argv preserves local argument boundaries but remote commands retain OpenSSH semantics. [Official ssh manual](https://man.openbsd.org/ssh.1), reviewed 2026-10-22. This source review does not exercise Rust integration or prove compatibility with any installed ssh build.

## Selection and reopening gate

Phase 0 E0 records actual toolchains/licenses, and E1/E2/E6 verifies C/S, PTY and a non-Rust public-protocol fixture. Exact dependencies require their selected version's primary documentation plus exercised evidence; pin only then. Reopen language choice only on a reproduced platform/distribution/maintenance blocker with a concrete alternative, preserving the fixed architecture. No blocker is established today.

## Phase 0 implementation evidence

The probe workspace uses Rust/Cargo 1.90.0, a public protocol crate, Core/Client harness and independently built image plugin. System OpenSSH remains the connection/authentication implementation; russh-sftp supplies only the SFTP protocol on its pipes. The [fixture report](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/fixture-evidence.md) records pinned base/derived images, actual components and native ELF compatibility; [image](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/visual-evidence.md) and [Python plugin evidence](../../.trellis/tasks/10-22-sshx-phase0-feasibility/research/plugin-evidence.md) exercise independent processes/public framing. Exact resolved versions are in Cargo.lock. No host Rust installation or second SSH stack was added. This establishes the bounded Linux prototype baseline, not a released application or other-platform feasibility.
