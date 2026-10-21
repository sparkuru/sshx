# SSHX Design Contracts

These are design obligations derived from [the retained source](sshx-design.txt), sections I–XIII. They describe no implemented API or tested platform. Read the original sections for complete requirements. [Mainline](../../mainline.md) controls authorized work; importing the design does not start the product roadmap.

## Decisions, defaults and unknowns

| Category | Contract and source |
| --- | --- |
| Fixed direction | Independent `sshx` command, CLI/TUI, local Client/Core, system OpenSSH authentication/configuration, independent plugin executables using the same public permission model (I–III, VI) |
| Default route | Rust with system OpenSSH; interactive enhanced mode first; Image Paste as the first vertical slice; candidate directory boundaries and staged rollout (II, VII, X–XIII) |
| Requires evidence | Specific crate versions/APIs, PTY/ConPTY behavior, actual OpenSSH multiplexing, SFTP backend, desktop clipboard combinations, screen restoration, shell/path mappings and platform runtime coverage (IV–V, X–XII) |

Tokio, portable-pty, Serde/TOML, Ratatui/Crossterm, arboard and SQLite are candidates. Do not install all of them, create one crate per concept or promise zero system dependencies. Record verified versions and the reason for each selection. Core must not depend on a concrete plugin; plugin clients depend only on public protocol/SDK. Later language-independence validation needs a minimal external client that does not use Core internals.

`sshx` is a design codename with a known name conflict mentioned by the source. Do not rename it during initialization; review command/package collisions before actual release.

## CLI and process ownership (II–IV)

Leave system `ssh`, its aliases and existing SSH processes untouched. Define argument parsing, `connect --`, management-command conflicts and unknown arguments before implementation. Pass local arguments as argv. This does not eliminate OpenSSH's remote Shell parsing semantics.

For pipes, redirection, remote commands, `-T`, `-N`, `-W`, subsystems and background mode, specify native passthrough or an explicit enhanced-mode rejection individually. Do not force a PTY onto every invocation, merge binary stdout/stderr or silently discard flags.

Client owns terminal display/input, local feedback, shortcuts and the triggering desktop's clipboard adapter. Core owns PTY/ConPTY, OpenSSH children, sessions, transfer/forwarding resources and public plugin capabilities from the first C/S slice. A background Core cannot assume desktop permission or a reusable display environment. Capture required session working directory and allowlisted environment explicitly instead of inheriting the first daemon launch's agent/desktop state for every session.

Core supplies mechanisms; plugins supply business policy such as image naming, history querying and command examples. No arbitrary process executor, bare PTY handle or internal service registry is a plugin capability. Separate startup locking, stale endpoints, protocol compatibility, idle exit and upgrades; upgrading must not silently interrupt live sessions.

## Session and operation identity (IV, VI–VII)

Keep separate owners for ConnectionProfile (target/options/config version), Session (`session_id`, `generation`, process state), Attachment (`client_id`, `attachment_id`, size/input ownership), Operation (`operation_id`, initiating plugin/session/attachment, cancellation/result), TransportHandle, CapabilityGrant and RemoteContext (namespace/source/freshness/trust). A hostname alone is not a connection identity.

Use one interactive writer per Session initially; return explicit contention errors. Attachment switching revokes prior input ownership; the writer controls resize. Activity belongs to the Client/Attachment, never a global `active_session`.

Bind asynchronous operations at creation to the plugin instance, Session/generation and initiating Attachment. At each effect, recheck operation ownership, target/config binding, scope, cancellation, expiry and revocation. Reconnection creates a new generation; old grants/results cannot be retargeted by querying whichever session is now active. Stale, detached, disabled or revoked work cannot inject input into another window.

Keep process-started, authenticated and Shell-ready states distinct. Shell, cwd, target and command events need trustworthy evidence and freshness; return `unknown` when absent. Never infer readiness or command completion from `$`, `#`, `>`, titles or arbitrary terminal text.

## Terminal streams and recovery (IV, IX)

Handle terminal data as bytes across read boundaries. Keep control traffic, terminal streams, plugin RPC and file bytes separate; bound queues/buffers and define overflow/backpressure. Slow plugins/subscribers must not stall SSH. Preserve OpenSSH escapes and provide SSHX prefix cancellation, timeout, disable and literal passthrough. Local key receipt, a control byte and local/remote signals are different actions.

Check real terminal behavior for Unicode/IME, ANSI/VT, resize, control keys, job control, password/MFA, paste modes, full-screen programs, exit status and terminal restoration. Windows ConPTY process/I/O/resize/close ordering needs Windows runtime evidence.

Test three recovery claims separately: Core/PTY/OpenSSH survival after Client detach; screen/cursor/mode restoration on attach; and remote-work recovery after network failure, Core crash or reboot. Bounded VT state versus limited redraw is a Phase 0 decision. Raw byte replay is not a safe screen snapshot and must not repeat clipboard/notification effects. Metadata alone does not identify a surviving session. Do not promise PTY adoption after Core crash or remote recovery without separately verified tmux/Agent support.

## SSH, SFTP and namespaces (V)

OpenSSH retains config, identities, agent, jump hosts, known-host verification and interactive authentication. Never silently accept a changed host key. Distinguish configuration reuse, authentication identity reuse and actual network connection reuse. Probe multiplexing for the exact platform/version; an auxiliary same-target transfer may need separate authentication.

Phase 0 selects **one** MVP backend: system `sftp`, or system `ssh` opening SFTP with a mature protocol library. Do not implement a second SSH stack or two complete transfer backends without evidence. Batch `sftp` cannot borrow the main Shell terminal for hidden prompts. MVP transfer may require agent/key or a verified reusable connection; return `AUTH_REQUIRED` for unsupported additional interactive authentication. Do not cache/replay passwords or pass credentials to plugins.

Auxiliary connection mappings must verify `-F`, `-p`/`-P`, `-i`, `-J`, `ProxyCommand` and `-o`, while avoiding duplicated RemoteCommand, forwarding and forced RequestTTY. Bind transfers to the original configuration generation and verified target; ambiguous/config-changed targets require rejection or reselection.

Model SFTP paths separately from Shell paths. Local Path APIs, `~`, SFTP initial directory and inferred prompts do not establish remote Shell cwd. Use an SFTP-confirmed user directory/subdirectory for default uploads. Verify explicit mappings for chroot, Windows and containers; if mapping is unknown, offer the transfer path without claiming it is Shell-usable.

## Plugin and IPC boundaries (VI, IX)

All plugins are independent executables from Image Paste onward, including official ones. Disabling/removing them all must leave login and session management functional. Bundled installation grants neither every capability nor background clipboard/history access. Validate local package archive paths, manifest/entrypoint/platform/API requirements before starting; do not run arbitrary install scripts.

Specify discovery, validation, handshake, ready, calls/subscriptions and stop. Bound startup/restart attempts, timeouts and logs; malformed or crashed plugins cannot take down the SSH session. API version is independent of plugin/Core version. Define incompatible-version refusal, optional/unknown methods/fields, request IDs, errors, concurrency and cancellation in protocol work.

The source selects JSON-RPC 2.0 over stdio with Content-Length headers and UTF-8 JSON bodies; lengths are bytes. Define limits and framing details before code. stdout is protocol-only; stderr is bounded diagnostics. Large image/file data uses short-lived blob/stream handles bound to instance, session and operation, with size/read/release checks. Arbitrary file paths are not authorization handles. Source method names are design proposals, not existing library calls.

Derive plugin identity from its actual process/channel, not self-reported `plugin_id`. Enforce capabilities at the effect site, including resolved paths, symlinks, ACLs, quota, expiry and cancellation. Validate composed capabilities and alternate paths. Revocation on disable covers in-flight work. Do not broadcast other sessions, private plugin state, clipboard or raw terminal streams. Raw stream access is highly sensitive; password-prompt detection cannot make it safe.

Independent processes isolate failure, **not** same-user filesystem/network authority. MVP runs explicitly trusted local plugins only. Do not advertise an OS sandbox or malicious-plugin safety. Untrusted execution requires real per-platform filesystem, network, subprocess and resource confinement first; official plugins have no bypass.

Local IPC must authenticate peers: protected user runtime directory/Unix socket on Unix; current-user ACL, peer validation and remote-access limits on Windows Named Pipes. Address randomization is insufficient. Handle endpoint/symlink/stale-file races. Client and Plugin channel roles cannot upgrade into each other. Core has no default TCP management endpoint.

Credentials, private keys/passwords, agent access and full environments never reach plugins, logs or telemetry. Minimize diagnostics; exclude terminal/clipboard/command bodies by default. Do not persist keystrokes or recordings by default. Keep bounded memory, separate plugin/Core storage and platform-standard config directories. Terminal passthrough is not proof of sanitizing every control sequence.

## Image Paste transaction (VII)

1. Explicit action captures Client, Attachment, Session/generation, writer and target. Test prefix conflicts, timeout, cancellation and literal forwarding; ordinary text paste remains normal terminal behavior. Native paste shortcuts may be consumed by the host; OSC 52 is not a generic image channel.
2. Read one snapshot from the triggering Client's desktop. Return explicit empty/unavailable/permission results, without scanning files, sending text or continuously monitoring the clipboard.
3. The independent plugin normalizes to PNG with transparency, bounded dimensions/decoded memory/encoded size and only the authorized image handle. File naming/output format are plugin policy; use uniqueness and no silent overwrite.
4. Core verifies target, directory and transfer grant. Upload to an operation-owned unique temporary name, then rename if supported. Document actual overwrite/atomicity behavior; equal size does not prove a content hash.
5. Propose insertion only after successful transfer and a usable mapped path. Upload success and insertion success are distinct states. Retrying insertion does not repeat upload; duplicate operation requests need deduplication semantics.
6. Before insertion, revalidate generation, Attachment, writer, interaction context and grants. Switching, disconnect, detach, Session close, plugin disable, expiry or cancellation prevents stale delivery. Clean only resources created by this operation; failure/cancel never inserts a partial result.

Insert text only: no Enter, CR/LF, NUL, ESC or unsafe control sequences. No-Enter and bracketed paste are not safety sandboxes. Supported Shell/editor integration with valid context can allow insertion; otherwise use an explicit cancellable paste interaction with temporary input ownership. If reliable context cannot be established, retain the result and offer confirmed insertion or path copy.

Separate Shell argument, plain path and Markdown output. Test POSIX Shell, PowerShell and cmd encoding independently; unknown Shells do not inherit POSIX quoting. Do not insert a complete quoted argument blindly inside existing quotes. Use preview/copy if edit context is unknown. Relative paths require a known reference directory; default to verified absolute Shell paths when available. Outer Session identity does not track nested ssh/sudo/container/tmux destinations: show the actual outer upload target and preserve ambiguity.

MVP has no continuous clipboard watch, long-lived image cache or implicit overwrite. Failure coverage includes empty/corrupt/oversized image, missing desktop/SFTP, extra auth, special paths, duplicate names, permission/disk-full/partial-transfer errors, cancellation and all stale-target races.

## Later features and delivery evidence (VIII, XI–XIII)

History is opt-in structured supported-Shell data, with pre-persistence filters, exclusions, pause, retention and deletion; never infer commands from keystrokes or promise perfect redaction/all-process coverage. Results only insert, never execute. Plugin data remains private unless separately authorized. Command examples require source/license evidence and no first-release AI service.

Core owns forwarding resources while plugins own policy/UI. Bind loopback by default; remote text cannot authorize ports or clipboard reads. Distinguish process alive, listener established and target reachable. Later hooks/Agents use minimal optional SSH-protected channels and explicit setup, no silent permanent Shell edits/public listeners. Treat remote events as untrusted, sourced and fresh; do not inject discovery commands into an active interactive Shell.

Before Phase 0, derive requirements/platform matrix, ownership/data flows, models/state machines/protocol limits, Image Paste failure sequence and ADRs. Later phases follow source XI; they remain proposed work until authorized. Use the [validation profile](validation.md) to map source XII failure cases to evidence. No architecture text, cross-compilation or container fixture establishes native three-platform support.
