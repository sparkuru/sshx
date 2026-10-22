# SSHX architecture

Status: proposed product contracts, 2026-10-22. The original architecture task delivered this documentation package. The subsequent approved Phase 0 task now implements disposable Linux probe executables and records [native evidence and limits](phase-0.md); it does not implement the complete product API. Requirements come from the retained [source I–XIII](../.trellis/spec/trellis-plus/sshx-design.txt); task acceptance is [R1–R7](../.trellis/tasks/archive/2026-10/10-22-sshx-architecture-feasibility/prd.md). Words such as “must” define future acceptance, not implemented behavior.

## Requirement trace and decision classes

F = fixed source requirement; D = source default; P = concrete proposal in this package; U = requires evidence; S = superseded by source. The method names and numeric limits in the protocol are P. Broad runtime support remains U; only the exact measured [platform combinations](platform-matrix.md#measured-phase-0-combinations) have experimental evidence.

| ID | Source | Class and retained obligation | Owning artifact | Verification |
| --- | --- | --- | --- | --- |
| R1.1 | I | F: independent command and independent official/community plugins; S: built-in-first plugins/private Core traits, tray inference of active SSH target | This document; ADR 5 | Phase 0 E1/E6; Phase 1 disable all plugins |
| R1.2 | I, X | D: Rust/system OpenSSH; U: crates, packaging dependencies; defer release name collision review | [ADR 1](adr/0001-language.md) | E0; Phase 4 naming/packaging |
| R2 | II, IV, V, X–XII | F: distinguish client/terminal/Shell/display/remote; U: actual coverage | [Matrix](platform-matrix.md) | E0–E5 environment records |
| R3.1 | II | F: argv, explicit connect, preserve ssh, per-mode byte semantics | Invocation table below | E1/E2; XII.1–3 |
| R3.2 | III, IV, IX | F: user-local Core, per-Client activity, Core-owned PTY/children, bounded separated streams | Process/lifecycle below; [protocol](protocol.md) | E1/E2/E6/E7 |
| R4.1 | IV–VI | F: seven identities, single writer, immutable async target/generation, unknown readiness | Protocol models and effect ordering | E6/E7; XII.4–5,8 |
| R4.2 | VI, IX | F: public framed plugin RPC, no credentials/raw executor, roles, effect-site grants, events | Protocol transport/method tables; [ADR 5](adr/0005-plugin-isolation.md) | E6/E7; XII.9–10 |
| R5 | V, VII | F: one snapshot, PNG, owned non-overwriting upload, conditional insertion, retained result | [Image Paste](image-paste.md) | E3/E4/E7; XII.4–8 |
| R6.1 | IV, X | D: Rust and PTY/ConPTY; U: wrapper library and native behavior | [ADR 1](adr/0001-language.md), [ADR 2](adr/0002-pty-conpty.md) | E0/E2 |
| R6.2 | V, IV | U: one SFTP backend and bounded VT versus restricted redraw | [ADR 3](adr/0003-transfer-backend.md), [ADR 4](adr/0004-screen-restoration.md) | E4/E5 selection gates |
| R6.3 | VI, VIII, IX | F: trusted local plugins initially, no false sandbox claim; optional minimal remote integration | [ADR 5](adr/0005-plugin-isolation.md), [ADR 6](adr/0006-remote-integration.md) | E6; later remote integration tests |
| R7.1 | XI–XII | F: phased exits, all twelve fault groups, native evidence not cross-compilation | [Runbook](phase-0.md) | E0–E7; XII coverage table |
| R7.2 | VIII | F: later File Transfer, opt-in structured History, offline Command Reference, brokered loopback forwarding, separately authorized Clipboard Bridge | Runbook phase map; protocol reserved methods | Phase 2–4; XII.11 |
| R1.3 | XIII | F: coherent design before experiments; no runtime claims from documents | Whole package, dated evidence | Document checks and independent task check |

No section is silently dropped: I classification; II invocation/matrix; III processes; IV models/terminal/recovery; V transfer/namespaces; VI protocol; VII transaction; VIII reserved scope; IX threats; X baseline; XI phases; XII fault map; XIII this package. Exact dependency/API choices and primary native platform selection remain downstream gates. Full TUI, remote Agent, marketplace, GUI, cloud and AI are outside this task.

## Processes, dependency direction and streams

~~~mermaid
flowchart LR
    T[Terminal user] <--> C[Client and desktop adapter]
    D[Triggering desktop] --> C
    C <-->|Authenticated control IPC| K[User Core]
    C <-->|Separate terminal byte stream| K
    C <-->|Operation blob stream| K
    K <-->|Public framed stdio RPC| P[Independent plugin]
    K <-->|PTY or ConPTY bytes| S[System ssh child]
    K <-->|Selected SFTP transport| X[Auxiliary OpenSSH child]
    S <--> R[Remote SSH server]
    X <--> R
~~~

Core owns Session Manager, PTY, ssh, Transport Broker, grants, plugin lifecycle and effect serialization. Client owns input/display, local menus/progress, per-Client attachment and desktop access. Plugins own business configuration, image encoding/naming and presentation policy. Plugins never obtain PTY descriptors, arbitrary process execution, agent handles or an internal service registry. Basic login and management work with no plugin installed.

Future source layout follows real boundaries: crates/cli, crates/core, crates/protocol, plugins/image-paste, tests and later integrations. Core depends on public protocol and mechanisms, never a concrete plugin. Plugins depend only on protocol/SDK; a non-Rust fixture must prove interoperability without Core libraries. This is a layout recommendation, not permission to generate empty crates.

| Boundary | Representation and ownership | Congestion/failure |
| --- | --- | --- |
| User ↔ Client | Terminal bytes and platform input events; Client restores its saved terminal state | Local notifications never become remote keystrokes |
| Client ↔ Core control | Framed JSON-RPC; kernel-authenticated Client role | Bounded queue; fail/disconnect slow peer |
| Client ↔ Core terminal | Binary stream bound to Attachment; no JSON decoding of bytes | Bounded queue, overflow revokes attachment and marks display incomplete; PTY drain continues |
| Client ↔ Core image/blob | Dedicated binary operation channel, exact offsets/size | Reject quota/late data; no stdout contamination |
| Core ↔ PTY/ConPTY ↔ ssh | Bytes across arbitrary read boundaries; one input writer | Independent drain tasks; detached output is consumed, never unbounded spool |
| Core ↔ plugin | JSON-RPC stdio with bounded chunk RPC for blobs; stderr is diagnostic only | Slow subscriber/crash cannot stall PTY; terminate violating plugin |
| Core ↔ SFTP | File protocol bytes or selected sftp subprocess interface | Isolated transport workers; cancel kills only owned transport |
| ssh ↔ user authentication | Main ssh on PTY; credentials handled by OpenSSH | No prompt scraping or password storage |

All resource constants live in [protocol limits](protocol.md#limits). Live terminal passthrough is not a sanitization guarantee. MVP intercepts local SSHX prefix and blocks remote OSC 52/5522 clipboard operations in the managed output path; it grants no desktop access from terminal output. Other terminal sequences pass through under host permissions and are not called safe. A later Clipboard Bridge needs explicit permissions and tested combinations. Reattach must render selected safe state, never replay side-effect sequences. Prefix/control-sequence parsing must carry state across reads and stay bounded.

## Cooperative upload boundary

On 2026-10-22 the user chose A for the future MVP upload contract: trust a cooperative remote server/account and stable upload hierarchy through cleanup. A root is a canonical, locally authorized scope in the verified target's SFTP namespace, not an OS jail or durable inode capability. This narrows the planned environment without overturning the accepted Phase 0 no-go for arbitrary same-account ancestor replacement. The [protocol](protocol.md#cooperative-upload-scope) owns the existing grant policy/digest and effect-site rules; [ADR 3](adr/0003-transfer-backend.md) records the decision and source evidence.

| Principal / asset boundary | Proposed obligation or limit |
| --- | --- |
| Local user/Client policy | Explicitly authorize actual target/namespace/root and `path_scope.policy: "cooperative"`; Core binds `scope_digest` and B. A clipboard action or installation alone is insufficient |
| Official/community plugin | Same method grants, path scope, quotas, cancellation and input-context checks; no self-grants, credentials, raw executor or target substitution |
| Other local users | Existing authenticated IPC and protected local resources remain required; this is separate from remote cooperation |
| Cooperative server/account programs | Honor SFTP results and leave ancestors/operation-owned resources stable throughout live work and cleanup; distinct SSHX operations use separate resources |
| Final user files / competing final names | Verified no-overwrite publication; confirmed final data survives cancellation, insertion failure and temp-cleanup failure |
| Hostile server / same-account resource replacement | Outside enforced guarantees; metadata, random names and 0700 do not establish identity or atomic confinement. Observed inconsistency stops effects; undetected interference is not claimed prevented |
| Input receiver / namespaces | Confirmed upload does not supply a Shell mapping or fresh edit context; insertion is a separately authorized effect |

The selected future regular-PNG publication in existing backend B uses capability/version-verified `hardlink@openssh.com` v1 from a closed temp to a new final basename in the same operation directory/filesystem, then separate owned-temp cleanup. It is not implemented or newly tested here. Unsupported publication remains unavailable, with no overwrite-capable rename/remote-command fallback. Confirmed final plus failed temp unlink stays committed with `cleanup_pending`; lost publication reply is indeterminate and cannot trigger replay/input. Core's narrow ledger authority may clean known temps after revocation only while ownership and verified cooperative continuity remain adequate; loss of evidence leaves cleanup pending, never a guessed deletion or recursive sweep. [Image Paste](image-paste.md) composes those rules.

This document decision supplies a boundary for later MVP planning, not runtime/release readiness or automatic next-phase authority. Dedicated hardlink/policy/cleanup tests, target/auth binding, real Shell mapping/context and native-platform evidence remain gates. Historical probes and `phase0_ready: false` are unchanged.

## Invocation contract

The independent executable must not install an ssh alias, replace ssh or adopt an existing ssh process. Proposed grammar reserves top-level connect, session, plugin and core; use “sshx connect -- session” for a host named session. After connect's required separator, all tokens belong to OpenSSH. In shorthand, the original option/destination vector is preserved. An unrecognized SSH option is forwarded using native passthrough, never guessed or dropped. Unknown management options fail locally. CLI parse failures use the protocol's local exit status; native passthrough preserves the child's status and platform signal semantics.

The mode classifier is conservative. Enhanced mode requires terminal input/output/error handles, a login invocation, supported options and a validated effective configuration without RemoteCommand, forced no-TTY, subsystem or background behavior. OpenSSH config can affect mode: when classification cannot be established, native passthrough is the fallback. Do not infer interactive mode merely from the absence of trailing command tokens. Passthrough adds no SSHX bytes to stdout or stderr; an explanation requires an explicitly opted-in separate diagnostic destination or managed-mode UI. Configuration evaluation is subject to the protocol's helper/side-effect constraints.

| Invocation class | Proposed behavior | Managed Session / streams |
| --- | --- | --- |
| ssh target | System command unchanged | Outside SSHX |
| sshx target; connect -- target | Enhanced if classifier accepts; otherwise native passthrough | Core creates PTY only in enhanced mode |
| -p, -i, -J, -F, supported -o options | Preserve argv and semantics; classify resolved mode | Config/agent context is per-session |
| Pipe into stdin; stdout or stderr redirected | Native passthrough, including binary data | No managed Session; inherit all three handles separately |
| destination plus remote command/arguments | Native passthrough | No local Shell assembly; remote semantics stay OpenSSH's |
| -T; -N; -W host:port | Native passthrough individually | No forced PTY, no plugin attachment |
| -s subsystem | Native passthrough | Binary subsystem output preserved |
| -f, -n or config ForkAfterAuthentication/StdinNull | Native passthrough | Background process ownership remains native |
| -G, -Q, -V, -O control command | Native passthrough | No daemon needed |
| -t/-tt login | Enhanced only on a validated terminal/config combination | Otherwise passthrough; no pretend ConPTY support |
| Unknown SSH flag/version-specific behavior | Native passthrough original vector | OpenSSH decides validity |
| Management conflict/invalid session ID | Explicit local failure | Never treats a failed management command as a host |

Native passthrough launches the selected system ssh directly, preserves stdio/argv and exit status, and provides no plugins, retention or attachment. It must not start Core unnecessarily. OpenSSH sends remote command arguments with its own remote parsing semantics; local argv construction does not remove those semantics. See the official [ssh synopsis/description](https://man.openbsd.org/ssh.1), checked 2026-10-22.

## User-local Core lifecycle

Client takes a user-scoped startup lock, verifies the protected runtime directory/endpoint ownership and connects to an authenticated ready Core. If absent, it starts a nonprivileged Core with a minimal environment, then waits for negotiated readiness. A process PID or socket file alone is insufficient. Under the lock, stale cleanup requires failed peer connection plus positively identified owned endpoint/instance metadata; never unlink unknown files/symlinks. Hold parent directory handles and validate endpoint type/owner to avoid replacement races. Unix uses pathname sockets and native peer checks; Windows uses current-user ACL, logon/peer checks and local-only pipe access. The Windows per-logon Core scope may yield separate Core instances for concurrent logon sessions, never desktop permission borrowing. See [protocol roles](protocol.md#roles-and-local-ipc).

Concurrent starters converge on one instance in that scope. Incompatible Clients receive VERSION_MISMATCH with the running instance version; no automatic kill/restart. Updates stage a separate executable, wait for zero live sessions/operations/clients, then shut down and restart explicitly; active sessions block replacement. An explicit force-close action lists affected sessions and requires its own user confirmation. Normal stop with live resources returns BUSY. Idle exit occurs only after the protocol idle interval and no live resources.

Detach, terminal-window close and Client crash revoke the attachment/writer and all operation delivery leases; Core, PTY and ssh survive while Core is healthy. Remote Shell exit drains output, records status, revokes session resources and closes PTY/child handles. Session close cancels operations, stops owned transport, requests child termination, escalates only that child after the shutdown grace and marks final resources; it never kills arbitrary ssh processes. Core crash invalidates all boot-scoped handles and does not promise PTY adoption. Persisted metadata is historical, not a living Session. Network reconnect is a new generation and fresh grants; remote work recovery is a separate optional capability.

Each Session start captures local cwd, terminal type/locale, chosen ssh executable, original argv and explicit environment allowlist. Proposed allowlist: TERM, COLORTERM, LANG, LC_ALL, LC_CTYPE and the initiating Client's SSH_AUTH_SOCK where supported. HOME/USERPROFILE and platform system variables come from a validated user profile; executable lookup uses trusted resolved paths, not a plugin's PATH. User-configured OpenSSH helper/agent/askpass requirements outside the list must be explicitly added to the session profile and audited before spawn, never copied from the first daemon's environment. Client desktop variables/permissions stay in Client and are not exported to plugins. No environment values or identity contents appear in session.describe.

Process-started, authenticated and Shell-ready are separate fields. Process existence supplies only the first. Trustworthy authenticated/target evidence may come from a tested broker/master interface; otherwise unknown. Prompt text/title is never evidence. A user can interact with ssh while readiness remains unknown; plugin automatic insertion cannot use that uncertainty as permission.

## Storage and evidence

Use platform-standard user config/data/cache/runtime locations selected during implementation; no universal Unix path. Core stores minimal historical metadata only when enabled; no terminal recording or keystrokes by default. Blob/temp resources are operation-scoped, plugin data uses a separate namespace, and logs exclude credentials, clipboard, commands and terminal bodies. Unsandboxed trusted plugins retain same-user OS access outside RPC; permissions are broker enforcement, not malicious-code confinement.

Continue with [platform conditions](platform-matrix.md), [public protocol](protocol.md), [Image Paste](image-paste.md), the six linked ADRs and [Phase 0](phase-0.md). [Primary-source research](../.trellis/tasks/archive/2026-10/10-22-sshx-architecture-feasibility/research/primary-sources.md) records documentary evidence and remaining uncertainties.
