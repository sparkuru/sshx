# Image Paste transaction

Status: proposed product transaction, 2026-10-22. Phase 0 now exercises synthetic private-X11 capture, independent PNG encoding and controlled upload/input probes; this does not implement the entire transaction or trusted real-Shell context. See [actual commands and result scope](phase-0.md) and the [image evidence](../.trellis/tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/visual-evidence.md). Normative fields, state names, finite limits and error names live in [protocol](protocol.md). This document traces [source V–VII, XII](../.trellis/spec/trellis-plus/sshx-design.txt), R5 and the compositional checks for R4.

## Trigger and consent

Client's configurable default prefix is Ctrl+] followed by p. Client intercepts only that prefix sequence: repeated prefix forwards a literal prefix, Esc cancels without remote bytes, timeout forwards the held literal prefix, and an unrecognized action forwards prefix plus following bytes in original order. Prefix can be disabled/rebound per Client; OpenSSH's own escape sequence remains untouched. A partial key/IME sequence must not be decoded per read chunk. E2 records actual delivery/conflicts for each terminal host. Ordinary text paste stays ordinary terminal input; native Ctrl+V/Command+V is not a promised image trigger.

The explicit command/action entry takes a Session/generation and initiating Attachment, resolves its live authenticated Client and requires its writer consent. A management Client without that desktop/writer cannot capture from another Client implicitly. Trigger captures B, outer target label and writer_epoch. Core creates/deduplicates one Operation from the Client's monotone trigger_nonce counter and routes action.run to the independent plugin. The Client retains that counter for retries of the same action; expiry never makes an old counter a fresh action, and reconnect never automatically replays it under a new Client identity. Consent displays the composed effects: read this one desktop image, upload it to this verified outer target/namespace/root under cooperative policy, and optionally propose bounded input. Existing trusted local policy/consent must explicitly authorize that actual scope; a trigger or installation alone does not. Being bundled/installed does not grant them. Basic SSH continues when this plugin is absent or disabled, or upload is unavailable.

The user selected cooperative mode A for future MVP planning: the remote server/account honors SFTP and its programs keep upload ancestors and operation-owned resources stable through cleanup. This is a trust precondition, not a claim to prevent hostile same-account replacement. The logical canonical SFTP root is not an OS jail or inode capability. Core binds the existing `path_scope.policy: "cooperative"` and actual target/namespace/root into `scope_digest` and B; plugins cannot grant or broaden it. Common role/method-grant errors remain separate; a valid upload lacking that path-policy authorization or requiring stronger confinement returns `PATH_DENIED`, while structurally malformed fields retain `-32602`. The [protocol](protocol.md#cooperative-upload-scope) owns the full checks and trust limits.

## Success sequence

~~~mermaid
sequenceDiagram
    participant U as User
    participant C as Triggering Client
    participant K as Core
    participant P as Image Paste process
    participant S as SFTP broker
    U->>C: Explicit action on Attachment A
    C->>K: action.invoke (epoch, trigger_nonce)
    K->>K: Capture B and locally authorized cooperative scope
    K->>P: action.run (operation, target)
    P->>K: clipboard.read_image
    K->>C: clipboard.snapshot (one-shot ticket)
    C->>K: Bounded raw RGBA snapshot stream
    K->>K: Validate layout, dimensions, actual length, quota
    K-->>P: Sealed raw blob handle
    P->>K: blob.read chunks
    P->>P: Encode static PNG preserving alpha
    P->>K: blob.create, write, seal
    K->>K: Bounded PNG validation and local digest
    P->>K: transfer.upload (upload_key, path, PNG handle)
    K->>S: Verify B, target/config, scope, publication capability
    S->>S: Exclusive operation dir/temp, stream, close, recheck
    S->>S: Verified hardlink v1 to new final basename
    S-->>K: Confirmed transfer result and mapping evidence
    K->>S: Separate owned-temp cleanup, or record pending
    K-->>P: operation.result event (upload committed)
    P->>K: terminal.propose_insert (insert_key, format, context)
    K->>C: Preview/confirmation if integration unavailable
    C-->>K: Fresh lease or cancel
    K->>K: Serialize writer/context/grant check and insert admission
    K->>K: Write bounded text once, record outcome
    K-->>C: Upload result and independent insert outcome
    P->>K: Release image/PNG handles
~~~

This is the proposed future success flow, not a hardlink execution report. Acknowledged final publication is committed independently of temp cleanup; cleanup failure retains that result with `cleanup_pending: true`. No progress, filename or error message is written into the remote Shell. Terminal input remains user-owned except a clearly indicated cancellable local insertion lease. Any input that would invalidate a preview cancels the lease before being forwarded; input is not silently lost. The Core must continue draining terminal output while encoding/uploading.

## Snapshot and PNG

Only the triggering desktop adapter reads once. It returns empty/unavailable/denied separately. It must not fall back to clipboard text, scan screenshot directories, monitor clipboard changes or query another desktop. Linux fallback is a separately selected local file action, with new consent and explicit path validation; it is not Image Paste success.

The raw RGBA layout and pre-allocation limits in [Images and result types](protocol.md#images-and-result-types) govern the Client, Core and plugin. Core validates received byte length against checked width/height/stride before exposing a sealed handle. Unsupported native clipboard APIs that allocate unboundedly before validation cannot become a supported combination merely by adding a post-read check. The plugin sees this immutable image only, not arbitrary file paths or desktop handles. It encodes a static PNG with alpha; animation/unsupported formats return an explicit error. Core independently validates the sealed PNG profile and budget before allowing upload. No long-lived image cache is enabled by default.

Naming is plugin policy: a timestamp label plus random operation-unique suffix and a .png extension, validated as one basename by Core. Configured templates cannot add separators, traversal or controls. A same-key retry reuses the same assigned name and resource ledger; NAME_EXISTS does not silently choose a second name and upload again. A new naming choice needs an explicit reconciled user action after proving the prior effect did not commit.

## Transfer and namespaces

Before any auxiliary connection/file effect, the broker checks B, actual plugin instance, grants, immutable config generation and verified outer target. It distinguishes configuration reuse, identity reuse and actual network reuse. Additional interactive authentication in MVP returns AUTH_REQUIRED; the broker does not borrow the live Shell terminal, scrape passwords or let the plugin prompt for credentials.

Use a canonical SFTP-confirmed user-root subdirectory, or an explicitly configured and validated namespace/root grant. Never treat '~', the initial SFTP directory or a local Path join as Shell cwd. Proposed upload stages follow the authoritative [publication contract](protocol.md#future-regular-png-publication):

1. Require applicable cooperative policy, verified target/config/namespace/canonical containment and B/grants, plus actual server/filesystem evidence for the selected regular-file no-overwrite publication. Advertisement alone cannot admit data creation; absent capability gives `COMMIT_UNSUPPORTED`.
2. Exclusively create a private operation directory and regular temp within it under the granted root. Record exact paths/namespace and acknowledged creation in Core's resource ledger; verify type/access observations. Do not adopt existing/symlinked resources. Use remote namespace component semantics and backend-verified filename encoding, not local Path joins or raw prefixes.
3. Stream the sealed PNG through the file handle, checking grants/quota/deadlines/cancel at every external step. Close successfully, then revalidate scope/observations and serialized publication admission. Record local SHA-256 and actual remote verification level; equal size means size-only evidence.
4. The future candidate in existing backend B is verified `hardlink@openssh.com` version 1: link the closed regular temp to a new final basename in the same operation directory/filesystem. The final path includes that directory; no directory rename is needed. Confirmed collision gives `NAME_EXISTS` without changing existing bytes. Unsupported version/filesystem/cross-device capability gives `COMMIT_UNSUPPORTED`; no baseline/POSIX rename, scp or remote-command fallback. Other generic SFTP failures retain observed I/O/indeterminate outcomes, not invented collision/unsupported classifications.
5. An acknowledged link commits the final user file. Separately unlink only the owned temp under the cleanup rules; failure leaves the upload committed with `cleanup_pending: true`, never deletes the final name to compensate. A lost publication reply instead leaves `upload_state: indeterminate`/`final_commit: "indeterminate"`, no insertion or automatic replay. Reconciliation is read-only until explicit disposition; no temp path is a completed result.
6. Retain confirmed result metadata independently of the plugin/Attachment. Verify an explicit versioned mapping before returning a Shell path. Unknown mapping still permits a transfer-path-only user result, never a guessed Shell path. Cleanup pending alone does not invalidate a confirmed final path; fresh insertion gates remain mandatory.

Check known ancestor/type/symlink/access/containment observations before creation, publication and cleanup; observed inconsistency stops new effects. Portable attributes, private 0700 directories, exclusive leaf creation and random names do not prove durable identity or prevent replacement/ABA between requests. The selected cooperative assumption is required through cleanup; hostile servers and competing same-account replacement remain outside its enforced guarantee. Atomicity, no-overwrite, durability and content verification are distinct. Phase 0's positive baseline-rename observations and general-root no-go remain historical; the current Core probe adds separately authorized [bounded hardlink evidence](phase-0.md#cooperative-runtime-checkpoint), without implementing the full transaction.

## Insertion and formatting

Upload commit and insertion commit are separate effects. terminal.propose_insert uses the existing Operation/result; it never re-uploads. It requires confirmed final upload, verified Shell namespace mapping, the same generation/Attachment/writer_epoch and fresh scope/grants. A-to-B switching does not move a pending proposal to B, even when the hostname matches. The UI shows the actual outer upload target; nested ssh/sudo/container/tmux does not inherit outer namespace knowledge.

| Requested output | Required context / proposed encoding | Unknown or unsafe context |
| --- | --- | --- |
| Shell argument, POSIX family | Tested exact Shell/encoding; complete argument boundary, no open quote; proposed single-quote encoding with embedded apostrophes closed/escaped/reopened | No automatic insertion; preview/copy |
| Shell argument, PowerShell | Tested PowerShell version/edit position; proposed literal single-quoted argument with apostrophes doubled, including Unicode/path tests | Do not reuse POSIX encoding; preview/copy |
| Shell argument, cmd | Dedicated tested cmd quoting/expansion rules including percent, exclamation and metacharacters; no universal encoder selected | MVP argument insertion unavailable until evidence; plain-path preview/copy |
| Plain path | Verified mapped absolute path; supported plain-text receiver or explicit user-attested local paste context and temporary lease | No receiver/context → copy/manual use only |
| Markdown | Tested escaping/URI representation in a known editor/document reference; relative links only with known document base | Show absolute mapped result/preview; Shell cwd is not document directory |

These are candidate encoder contracts, not tested library calls. POSIX/PowerShell quoting must be tested with exact fixtures before activation; an already-open quote invalidates the complete-argument contract. Newline/NUL/ESC/C0/DEL/C1 path text is never insertable in any mode, even if transfer succeeded. No Enter is added; bracketed paste is used only for a verified receiver and grants no safety guarantee. Unknown Shells never inherit POSIX quoting. A full-screen receiver may interpret ordinary text as commands, so “no Enter” alone cannot authorize insertion.

Supported integration may authorize automatic insertion only with fresh validated edit revision and mapping. Without it, Core presents the exact text/target locally and accepts a one-use, expiring user confirmation for the selected receiver; confirmation attests the user's chosen context, not an automatic discovery of Shell readiness. If Core cannot establish that lease and exclude intervening input/context changes, it retains the result and offers copy/manual use. No repeated remote prompt probes are sent through the live Shell.

## Failure and race outcomes

The serialization/point-of-no-rollback rules in [Effect ordering](protocol.md#effect-ordering-deduplication-and-cancellation) apply to every row. Revocation/cancel forbids effects not yet admitted; already dispatched remote commit/PTy writes can finish and must be reported honestly.

| Trigger or race | Upload outcome | Insertion/result behavior |
| --- | --- | --- |
| Empty/denied/unavailable clipboard | None; explicit snapshot error | No text fallback; new action needed for new snapshot |
| Corrupt/oversized raw image or PNG | None; reject before unsafe allocation/transfer | Release owned blobs and report correct validation error |
| No SFTP / extra auth | NO_SFTP / AUTH_REQUIRED | Main login unaffected; no hidden alternative backend/credential replay |
| No cooperative path authorization / stronger confinement required | PATH_DENIED after common role/method-grant checks; no new path effects | Login remains independent; no silent downgrade or self-grant |
| Denied directory, observed symlink/root change, disk full | Failed; stop new effects; cleanup only if ownership/continuity remain adequate | Never insert partial path; uncertain resources stay pending |
| Name collision | NAME_EXISTS; existing destination untouched | Same effect key cannot create another file automatically |
| Verified hardlink publication unavailable | COMMIT_UNSUPPORTED; no overwrite-capable fallback | No success or insertion from temp path; clean only demonstrably owned resources |
| Final link acknowledged; temp unlink fails | Committed final result, cleanup_pending true | Final file remains user data; independent mapping/context gates still apply |
| Publication reply lost/ambiguous | Indeterminate; no automatic link replay | Read-only reconciliation; no input or guessed final deletion |
| Cleanup ownership/target/namespace/continuity uncertain | Pending cleanup; stop automatic path deletion | No replacement-path removal, reconnect adoption or recursive sweep |
| A switches to B during encode/upload | Cancel further A effects; already committed A file remains | Revoke A delivery; never insert into B; retained A result |
| Detach, Client crash/window close | Cancel further effects; in-flight effect may complete | No stale delivery; user can inspect retained metadata later |
| Network loss / Session close / new generation | Stop old transport; reconcile uncertain commit | Old B/grants invalid; never deliver on new generation |
| Grant expires/revokes; plugin disables/crashes/restarts | Stop further steps, close owned auxiliary work | No pending insert; retained user result survives plugin lifetime |
| Cancel before remote commit dispatch | Stop/clean owned temporary resources | No final success and no insertion |
| Cancel/revoke after commit dispatch | Confirmed or indeterminate final effect; no rollback promise | Retain metadata; prohibit later insertion; do not delete final file |
| Upload succeeds but mapping/context unavailable | Confirmed transfer result | retained, MAPPING_UNKNOWN/CONTEXT_UNKNOWN; offer copy/manual use |
| User input/resize/output changes preview context | Upload stays committed | CONTEXT_CHANGED; no bytes sent; fresh explicit revision can retry insertion on same Operation |
| Insert completes but RPC response is lost | Upload stays committed | Query ledger; repeated insert_key returns recorded status without new bytes |
| Partial PTY write / completion unknown / Core crash | Upload stays committed or uncertain | partial/indeterminate insertion, never automatically repeat; user reconciles visible text |
| Duplicate trigger/upload/insert request | Same digest returns same record | Conflicting digest fails DEDUP_CONFLICT; no second file/input |

## Resource ownership and retained results

| Resource | Owner / release |
| --- | --- |
| Clipboard read permission | Core one-shot grant bound to original Client/action; consumed once, revoked on cancel/switch/expiry |
| Raw and encoded blobs | Core reserved bounded memory/temp storage, instance/operation-bound opaque handles; release after in-flight pins, deadline, cancel or plugin stop |
| Encoder memory | Trusted plugin's finite image contract; OS sandbox/resource confinement is not yet present |
| Auxiliary transport/child | Core broker; close owned resource only; never close the interactive ssh/master merely to cancel a file |
| Remote temporary files/directory | Core operation ledger; exact acknowledged resources and live verified continuity only; after cancel/revoke narrow Core cleanup authority never renews plugin upload. A directory containing a final file is retained |
| Confirmed remote final file | User data after commit; never auto-delete on cancel/insertion failure or dedup expiry |
| Result/dedup ledger | Core bounded same-boot metadata under RESULT_TTL; owning Client may inspect, plugins only their own grants; no image/terminal body persistence |
| Input lease | Core/triggering Client; expires or revokes on any conflicting change; no cross-Attachment transfer |

For late results, Core may show operation status in user-local management without delivering input or clipboard effects. Another Attachment may manually copy/use a path through a new explicit user interaction, not inherit an old pending insert key. Releasing blobs does not delete final files. Every cleanup removal rechecks the exact ledger entry, target/config/namespace and cooperative continuity; cancellation/revocation permits only Core's narrow pre-existing cleanup authority, no new publication. Lost/missing ownership evidence, transport loss or a new Core/generation leaves cleanup pending: do not adopt names after reconnect, guess deletion from matching attributes, follow replacement symlinks or sweep recursively. The [cleanup contract](protocol.md#owned-cleanup-authority) owns these rules. Existing bounded Phase 0 results are historical. Current [hardlink/collision/lost-reply/cleanup and policy evidence](phase-0.md#cooperative-runtime-checkpoint) uses synthetic bytes and headless Core tests; it does not rerun or promote desktop capture or real Shell insertion.
