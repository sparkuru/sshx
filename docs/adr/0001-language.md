# ADR 0001: Rust Core and system OpenSSH

Date: 2026-10-22. Status: **source default retained**; dependency selection and runtime feasibility pending.

## Decision and context

[Source I, X](../../.trellis/spec/trellis-plus/sshx-design.txt) defaults to Rust for CLI/Core and public types. Preserve that baseline rather than presenting it as a separately mandated user language. The user approved the architecture package after its language explanation. System OpenSSH owns SSH configuration, authentication and host verification. All business plugins are independent executables speaking public RPC; their language is not prescribed. Core must not link to a concrete plugin.

Use the small source-proposed boundaries in [architecture](../architecture.md), not a crate per concept. Tokio, portable-pty, Serde/TOML, Ratatui/Crossterm, arboard and SQLite remain candidates. No dependency, version, feature or API is selected by this ADR; no Cargo workspace is created in this task.

## Alternatives and consequences

Go or another systems language can be reconsidered if measured primary-platform blockers outweigh the Rust baseline. Python/other languages are valid independent plugin choices. A single-process script would reduce initial work but fails the required Client/Core ownership and independent-plugin boundary. A second SSH implementation would duplicate authentication/config semantics and is outside the proposed MVP.

System OpenSSH adds an explicit deployed-system dependency and version/platform behavior to record. Local argv preserves local argument boundaries but remote commands retain OpenSSH semantics. [Official ssh manual](https://man.openbsd.org/ssh.1), reviewed 2026-10-22. This source review does not exercise Rust integration or prove compatibility with any installed ssh build.

## Selection and reopening gate

Phase 0 E0 records actual toolchains/licenses, and E1/E2/E6 verifies C/S, PTY and a non-Rust public-protocol fixture. Exact dependencies require their selected version's primary documentation plus exercised evidence; pin only then. Reopen language choice only on a reproduced platform/distribution/maintenance blocker with a concrete alternative, preserving the fixed architecture. No blocker is established today.
