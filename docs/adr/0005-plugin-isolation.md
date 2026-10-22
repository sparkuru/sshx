# ADR 0005: Independent trusted plugins with brokered capabilities

Date: 2026-10-22. Status: **decided boundary; sandbox deferred before untrusted execution**.

## Decision and threat boundary

All business plugins, including official Image Paste, are independent executables using identical public framed RPC and grants. Disabling all of them must leave basic login/session management usable. Official distribution grants no hidden Core traits, desktop permission or background history monitoring.

Independent processes isolate crashes/protocol failures, not same-user OS authority. MVP accepts explicitly trusted local packages only. A trusted but compromised/malicious plugin can attempt to use ordinary same-user filesystem/network APIs outside RPC. Broker grants cannot prevent that and must never be advertised as a malicious-plugin sandbox. Same-user malware and administrators are outside a strong isolation claim; other local users are separated by user-local IPC permissions/peer checks. Compromised remote output is untrusted data and cannot mint grants.

## Alternatives and enforcement

Built-in plugins/private traits are superseded by source I. A permission declaration alone is insufficient. Full per-platform sandboxing/WASM would add an unproven runtime and is deferred; before untrusted execution, actual filesystem, network, subprocess and resource confinement must be exercised including children and composed capabilities.

Core derives instance identity from the spawned child/channel, checks capabilities at each effect, binds operations/generation/Attachment, and revokes in-flight future steps on disable. Clipboard-read plus upload is an explicit exfiltration-capable composition requiring one-shot consent and target scope. Blob handles cannot become arbitrary local paths; notify/event APIs cannot smuggle raw executor or input access. No credentials, agent, full environment or raw terminal data are provided.

Package extraction rejects traversal/link/device/decompression abuse and runs no install scripts. Protocol startup/limits/restarts/errors are bounded in [protocol](../protocol.md). JSON-RPC supplies request/response structure, not transport, permissions or isolation. [Official JSON-RPC specification](https://www.jsonrpc.org/specification), reviewed 2026-10-22.

## Evidence and reopening gate

The original architecture task ran no plugin. Phase 0 now runs an independent Rust PNG plugin and an isolated stdlib Python fixture, with [image](../../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-evidence.md) and [E6 evidence](../../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/plugin-evidence.md). The latter exercises a focused denial-only test broker and bounded USTAR+gzip gate, not the full Phase 1 manager/installer. E6 verifies cross-language framing, role separation, malicious frames, slow consumers, identity spoofing, invalid package paths and independent crash/disable behavior; E7 tests composed grants and races. Before any untrusted-plugin claim, reopen with native sandbox design and adversarial child/alternate-channel tests. Reopen sooner if an official plugin needs privileged functionality unavailable through public scoped capabilities; do not add a private bypass.
