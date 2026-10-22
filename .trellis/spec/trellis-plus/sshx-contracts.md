# SSHX Design Contracts

These are design obligations derived from [the retained source](sshx-design.txt), sections I–XIII. They remain proposed product obligations; the bounded Phase 0 subset has measured Linux evidence linked from the runbook, without implementing the complete API. Read the original sections for complete requirements. [Mainline](../../mainline.md) controls authorized work; importing the design does not start the product roadmap.

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

Phase 0 compared system `sftp` with system `ssh` opening SFTP through a mature protocol library. Backend B remains the cooperative experiment route; the selected product contract and bounded hardlink probe follow-up are documented in [ADR 3](../../../docs/adr/0003-transfer-backend.md). The current probe implements direct hardlink publication; its task evidence determines the measured fixture scope. No general mutable-root backend or released upload capability is established. Do not implement a second SSH stack or two complete transfer backends without evidence. Batch `sftp` cannot borrow the main Shell terminal for hidden prompts. MVP transfer may require agent/key or a verified reusable connection; return `AUTH_REQUIRED` for unsupported additional interactive authentication. Do not cache/replay passwords or pass credentials to plugins.

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

## PNG validation implementation evidence

The Phase 0 finite RGBA8/noninterlaced profile requires encoded-size/IHDR/raw-layout preflight before output allocation, bounded chunk/CRC/order validation, and an independent bounded streaming zlib check for exactly `height * (width * 4 + 1)` filtered bytes with no unused compressed bytes. A measured png 0.18.1 decoder path accepted surplus decompressed pixels after an IHDR dimension reduction; decoder success and its best-effort limits alone do not establish exact layout or a hard allocation bound. Core verifies the original sealed bytes and hash; it does not silently repair/re-encode plugin output. See the [measured regression and allocation evidence](../../tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-evidence.md). This is a bounded probe implementation rule, not proof for every PNG/native clipboard adapter.

## Image Paste transaction (VII)

1. Explicit action captures Client, Attachment, Session/generation, writer and target. Test prefix conflicts, timeout, cancellation and literal forwarding; ordinary text paste remains normal terminal behavior. Native paste shortcuts may be consumed by the host; OSC 52 is not a generic image channel.
2. Read one snapshot from the triggering Client's desktop. Return explicit empty/unavailable/permission results, without scanning files, sending text or continuously monitoring the clipboard.
3. The independent plugin normalizes to PNG with transparency, bounded dimensions/decoded memory/encoded size and only the authorized image handle. File naming/output format are plugin policy; use uniqueness and no silent overwrite.
4. Core verifies target, explicit cooperative root policy and transfer grant. Upload to an operation-owned unique regular temp file; the intended product publication uses verified no-clobber hardlink to a new final basename, followed by separate owned-temp cleanup. Missing capability refuses publication; there is no implicit baseline/POSIX rename fallback. A confirmed final file survives temp-cleanup failure with `cleanup_pending: true`; equal size does not prove a content hash.
5. Propose insertion only after successful transfer and a usable mapped path. Upload success and insertion success are distinct states. Retrying insertion does not repeat upload; duplicate operation requests need deduplication semantics.
6. Before insertion, revalidate generation, Attachment, writer, interaction context and grants. Switching, disconnect, detach, Session close, plugin disable, expiry or cancellation prevents stale delivery. Clean only resources created by this operation; failure/cancel never inserts a partial result.

Insert text only: no Enter, CR/LF, NUL, ESC or unsafe control sequences. No-Enter and bracketed paste are not safety sandboxes. Supported Shell/editor integration with valid context can allow insertion; otherwise use an explicit cancellable paste interaction with temporary input ownership. If reliable context cannot be established, retain the result and offer confirmed insertion or path copy.

Separate Shell argument, plain path and Markdown output. Test POSIX Shell, PowerShell and cmd encoding independently; unknown Shells do not inherit POSIX quoting. Do not insert a complete quoted argument blindly inside existing quotes. Use preview/copy if edit context is unknown. Relative paths require a known reference directory; default to verified absolute Shell paths when available. Outer Session identity does not track nested ssh/sudo/container/tmux destinations: show the actual outer upload target and preserve ambiguity.

MVP has no continuous clipboard watch, long-lived image cache or implicit overwrite. Failure coverage includes empty/corrupt/oversized image, missing desktop/SFTP, extra auth, special paths, duplicate names, permission/disk-full/partial-transfer errors, cancellation and all stale-target races.

## Later features and delivery evidence (VIII, XI–XIII)

History is opt-in structured supported-Shell data, with pre-persistence filters, exclusions, pause, retention and deletion; never infer commands from keystrokes or promise perfect redaction/all-process coverage. Results only insert, never execute. Plugin data remains private unless separately authorized. Command examples require source/license evidence and no first-release AI service.

Core owns forwarding resources while plugins own policy/UI. Bind loopback by default; remote text cannot authorize ports or clipboard reads. Distinguish process alive, listener established and target reachable. Later hooks/Agents use minimal optional SSH-protected channels and explicit setup, no silent permanent Shell edits/public listeners. Treat remote events as untrusted, sourced and fresh; do not inject discovery commands into an active interactive Shell.

Before Phase 0, derive requirements/platform matrix, ownership/data flows, models/state machines/protocol limits, Image Paste failure sequence and ADRs. Later phases follow source XI; they remain proposed work until authorized. Use the [validation profile](validation.md) to map source XII failure cases to evidence. No architecture text, cross-compilation or container fixture establishes native three-platform support.

## Proposed v1 implementation handoff

### 1. Scope / trigger

When planning Phase 0 or implementing the first Client/Core/plugin slice, read the [architecture package](../../../docs/architecture.md), [protocol](../../../docs/protocol.md) and [Image Paste transaction](../../../docs/image-paste.md). These remain proposed product targets, supplemented by the user-approved cooperative-root contract and bounded historical Phase 0 evidence; they are not a released API or proof of broader platform/runtime support. The retained source remains requirements input; resolve any conflict before implementation. Register the applicable documents explicitly in both role contexts rather than assuming index links are injected.

### 2. Signatures

Proposed control requests include `action.invoke({attachment_id, writer_epoch, action_id, trigger_nonce}) -> {operation}` and `terminal.propose_insert({operation_id, insert_key, output_mode, text, mapping_id, context_revision, grant_ids}) -> {insert_state, reason}`. The upload request is `transfer.upload({operation_id,upload_key,blob_id,directory:{namespace_id,path},basename,grant_ids}) -> {operation}`. Core-issued `CapabilityGrant.path_scope` for this proposed mode is `{namespace_id,root,policy:"cooperative"}`; the plugin supplies no authorization policy. Existing `scope_digest`, immutable B and result `{final_commit,cleanup_pending}` carry the scope/outcome; no new provider, storage subsystem or error family is introduced. These excerpts locate the cross-layer boundary; the protocol's model/method tables own full field/nullable/error definitions. A plugin request is not authentication or a grant.

### 3. Contracts

Core captures immutable binding B, including its boot instance, Operation, Session/generation, Client/Attachment/writer epoch and profile/config generation. Effects resolve B from Core state; they never resolve a new active Session. Input text must match validated formatting of that Operation's confirmed mapped upload result. Upload commit and input commit are independent; a refused zero-byte insert may refresh context evidence without another upload, while partial/unknown writes cannot be automatically repeated. Exact framing, quotas and deadlines have one home in the protocol.

The cooperative root is a logical canonical path in the verified target's SFTP namespace. Existing trusted local policy explicitly authorizes that mode for the target/profile/config generation, namespace and root; bind those values plus policy into `scope_digest`. Changes invalidate affected grants/operations. Trust the remote account/server to honor operations and keep ancestors/operation-owned resources stable through cleanup; do not promise an inode capability, OS jail or resistance to hostile same-account replacement. Metadata, random names and 0700 are not proof of that stronger property. Plugins retain the same broker limits, and untrusted remote data never gains local authority. Component-aware remote path validation is still required; known symlink/namespace/type/access inconsistency stops new effects. No local Path interpretation or raw prefix containment.

Future regular-file publication requires verified `hardlink@openssh.com` version 1 behavior for the actual server/filesystem and same-directory regular temp/final pair. Advertisement alone is insufficient; the historical baseline rename observation does not establish it. Require successful stream/close before link, and keep the final path in that operation directory. A confirmed link commits user data; later temp-unlink failure sets cleanup pending and cannot roll back/delete the final name. Lost publication acknowledgement is indeterminate: no replay, success-looking result or input. Unknown server/filesystem failure is not automatically a collision or unsupported-capability classification.

Core may perform only narrow ledger-owned temp cleanup after cancellation/revocation, with fresh target/namespace/cooperative continuity and ownership checks. This is not a refreshed plugin grant or permission for new upload/publication. Changed/uncertain ownership, transport loss, new boot/generation or replacement observations stop automatic path deletion and retain cleanup pending. Never adopt paths after reconnect, recursively sweep a root, follow replacement symlinks, remove a directory retaining a final file, or remove final user data. Mapping, receiver context and insertion authorization remain independent of cooperative upload admission.


For proposed `action.invoke`, `trigger_nonce` is a per-live-Client monotone decimal-string counter, separate from RPC correlation IDs. Keep its high-water mark for the Client identity's lifetime: an old counter whose result record expired cannot become a fresh action. Reconnection gets a new Client identity and never automatically replays prior actions. Count retained metadata as well as live resources against the protocol caps/budget; TTL alone cannot bound record churn.

### 4. Validation and error matrix

| Condition | Proposed outcome |
| --- | --- |
| Structurally malformed request | Existing `-32602`; no authorization/state consumption |
| Wrong role or missing/revoked required method capability | Existing `ROLE_DENIED` / `CAPABILITY_DENIED`; preserve common authorization checks |
| Valid request without applicable cooperative path-policy authorization, stronger root requirement or observed scope/type/symlink inconsistency | `PATH_DENIED`; no implicit cooperative fallback |
| Stale initiating generation/Attachment/writer | `STALE_BINDING`; no retargeted effect |
| Unbound or changed auxiliary target/config | `TARGET_UNVERIFIED` / `CONFIG_CHANGED`; basic OpenSSH login remains separate |
| Upload committed but edit context/mapping unavailable | `CONTEXT_UNKNOWN` / `MAPPING_UNKNOWN`; retain result, no blind insertion |
| Missing/unverified hardlink commit capability or definite unsupported version/filesystem | `COMMIT_UNSUPPORTED`; no final-upload success or temp-path insertion, no rename fallback |
| Confirmed final collision | `NAME_EXISTS`; existing file unchanged; same effect key does not choose another name |
| Confirmed final link followed by failed temp cleanup | Upload committed, `cleanup_pending: true`; final user data retained |
| Lost/ambiguous publication reply or uncertain cleanup ownership | Indeterminate publication or pending cleanup as appropriate; no guessed replay/deletion/insertion |
| Remote commit/write outcome uncertain | Record indeterminate/partial result; query/reconcile without blind effect replay |

### 5. Good / base / bad cases

Good: A's upload finishes after the user switches to B; A's user-visible metadata can be retained, and B receives no input. Base: no desktop image returns the explicit clipboard result without scanning files or sending text. Bad: a callback queries the current Attachment and inserts an old upload path, even if both Sessions use the same hostname. For cooperative publication, good is a confirmed final link with failed temp unlink retaining committed user data and pending cleanup. Base is an unavailable commit capability refusing upload while login continues. Bad is treating 0700/lstat as same-account confinement, or deleting a final file to compensate for temp-cleanup failure.

### 6. Tests required

The full product must validate framing/version/role refusal, bounded queues, same-key dedup and refreshed-context insertion, switch/detach/reconnect/revoke/cancel ordering, lost publication acknowledgements and partial PTY writes. Bounded historical Phase 0 examples do not implement the complete product contract.

Required cooperative-mode assertions are: absent policy/cross-plugin/changed target-root grants admit no effects; malformed input preserves common validation; observed ancestor/symlink changes stop effects; actual hardlink collisions preserve existing bytes; unsupported capability and cross-device results have no fallback; lost link replies do not replay or insert; confirmed final plus failed temp unlink retains final data and cleanup pending; uncertain/replaced cleanup paths remain untouched; per-step cancel/revoke and independent mapping/context refusal produce the promised remote-file/PTY outcomes. The [bounded runtime task](../../tasks/archive/2026-10/10-22-sshx-upload-root-runtime/prd.md) exercises a probe subset, with actual cases and limitations in its evidence. Record server, filesystem and extension version, not only extension names. Advertisement and library inspection are not passing effect tests, and probe Client ownership is not full production cross-plugin authorization coverage.

### 7. Wrong versus correct

Wrong: treat cancellation as remote rollback, or resend a partial insertion because its RPC response was lost. Correct: serialize effect admission against B/grants/cancel, record known and uncertain external outcomes, and retain the result for explicit reconciliation. Core crash invalidates boot-scoped authority; metadata does not restore a live Session or exactly-once guarantee.

For cooperative upload, wrong is interpreting a configured root or metadata snapshot as enforced confinement, or mapping every generic remote failure to `NAME_EXISTS`. Correct is explicit narrow local policy, verified publication capability, honest confirmed/indeterminate outcomes and conservative ledger-only cleanup. The general-root no-go and unchanged Phase 0 reports remain historical evidence.

## Bounded cooperative upload probe contract

### 1. Scope / trigger

Apply when modifying `probes/phase0/harness` upload, policy, SFTP faults or its tests. This is an experimental Linux/generated-fixture contract, not the proposed production v1 API. Keep real-server observations, injected failures and historical evidence separate. See the current task's design and independent review before claiming a capability passed.

### 2. Signatures

`phase0-probe core [--qualify-upload] --fixture STATE [--fixture-secondary STATE] --runtime DIR` defaults to no upload policy. The explicit fixture-owner flag performs synthetic qualification and writes private runtime evidence; it is not plugin-granted authority. `upload-suite --fixture STATE --fixture-secondary STATE --output NEW_JSON` exercises the focused matrix. `suite --headless ...` uses synthetic PNG data and explicitly excludes desktop capture; ordinary desktop mode needs its own authority.

### 3. Contracts

Core binds cooperative mode, canonical root, namespace, frozen config and actual fixture target/run into scope_digest. Preserve that authority at every effect. Retain extension values and require version 1 plus run-scoped observed qualification; advertisement alone is insufficient. Dispatch, definitive rejection, confirmed final commit, content verification and cleanup are independent facts. Probe results expose final_commit and cleanup_pending; a lost reply cannot be promoted by matching bytes observed on another connection. Cleanup uses narrow original ledger authority, not a revived plugin grant.

Upload success does not freeze authorization for later input. Fresh or previously refused `probe.insert` must recheck both common B/grants and current scope before writing PTY bytes. A policy/root/namespace change after publication must deny new insertion while retaining final user data; read-only replay of an already completed result is a separate dedup path.

Deterministic probe barriers have one publisher: the worker that is actually waiting. Request admission reports `starting`, never the worker's `before_open` barrier before it runs; otherwise a Client release can be overwritten by worker startup. Tests wait for the appropriate barrier and for `completed` when they inspect readback/cleanup, since committed publication does not mean cleanup has finished.

### 4. Validation and error matrix

| Observed condition | Probe rule |
| --- | --- |
| No applicable policy / changed scope | PATH_DENIED after common identity/grant checks; no upload directory |
| Unqualified or unsupported extension | COMMIT_UNSUPPORTED; no rename fallback |
| Final name known occupied before dispatch | NAME_EXISTS; preserve its data |
| Generic SFTP failure after concurrent creation/cross-device attempt | Preserve IO_ERROR/generic status; fixture knowledge does not become runtime EEXIST/EXDEV |
| Acknowledged link then readback/cleanup error | Keep confirmed commit; do not delete final file |
| Lost link reply | Indeterminate, no replay/input; retain cleanup uncertainty |
| Lost ledger/transport/boot/generation continuity | Pending cleanup, no path adoption or guessed removal |

Probe INVALID_PARAMS and Client ownership checks are not a claim of complete v1 malformed-input or plugin-role conformance.

### 5. Good / base / bad cases

Good: confirmed final data remains while temp cleanup is pending. Base: Core without explicit fixture qualification still supports login but refuses upload. Bad: clearing dispatch history after a server rejection, or deleting a final name to compensate for a temp-unlink failure.

### 6. Tests required

Assert remote presence/hashes and PTY effects, not only error enums. Test ordinary and concurrent collisions, matching framed lost replies including split/coalesced I/O, per-step revoke/cancel, scope drift both before upload and after commit before insertion, real generation change, replaced resources and both pre-unlink refusal and remove-error propagation. A fault must alter the fact consumed by a normal guard or the actual response boundary; a fault-name shortcut returning the desired error does not prove that guard. Headless exclusions and source-bound failed checkpoints must remain visible. Fixture teardown is independent evidence, not successful Core cleanup.

### 7. Wrong versus correct

Wrong: label a passing C13 bookkeeping record as completed independent review/teardown, or cite a historical desktop report as a current run. Correct: retain fresh output/source identities, explicit excluded/pending states, separate independent verification and exact owned-container cleanup. No test count changes the general-root no-go or historical phase0_ready:false.
