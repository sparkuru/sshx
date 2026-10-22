# Trellis Mainline

## Initiative

- title: SSHX — cross-platform SSH sessions and independent plugins
- parent task: none; the completed initialization task is not a product epic
- current approved objective: Phase 0 controlled feasibility deliverable accepted, committed, fast-forward merged and archived; finish the authorized archive/journal evidence while retaining the general-SFTP-root no-go
- owner decision: on 2026-10-21 the user approved initialization/source/closure. On 2026-10-22 the user approved the architecture plan and concrete work/archive/journal batch; those commits are verified below. The user then selected Phase 0 ("开始") and consented to its task creation/planning ("可以"); the user then approved its concrete Docker/native/loopback-SSH/private-Xvfb execution scope with “可以”
- current task: none; [SSHX Phase 0 feasibility](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/prd.md) completed/archived on 2026-10-22 after explicit acceptance of its controlled negative feasibility outcome

## Requirements and sources

The canonical repository-local design is [SSHX design](spec/trellis-plus/sshx-design.txt). It was imported byte for byte from the user-supplied file; see [provenance](tasks/archive/2026-10/10-21-trellis-plus-bootstrap/research/source-provenance.md). Import is approved. It establishes the requirements for the completed architecture package and future feasibility work. The approved Phase 0 task now adds experimental sources and controlled Linux runtime results; the released application and later phases remain unimplemented.

### Source decisions to retain

| Requirement | Source sections | Observable acceptance for future work |
| --- | --- | --- |
| Independent `sshx`; preserve system `ssh` | I–II | Installation/removal does not change `ssh`, aliases or existing processes; arguments are preserved or explicitly rejected |
| Local Client/Core architecture | III–IV | Core owns managed SSH processes and PTYs; multiple Clients use authenticated local IPC; no global active session |
| Stable asynchronous ownership | IV, VII, XII | Every operation retains Session/generation/Attachment ownership; switching, revocation, disconnect and cancellation cannot deliver stale input or files |
| OpenSSH authentication and explicit transport boundaries | II, V | Host checks stay with OpenSSH; login supports its interactive authentication; unsupported auxiliary authentication returns `AUTH_REQUIRED` |
| Independent official and community plugins | I, VI | Plugins build/run/release separately, depend on public protocol only, and do not affect basic sessions when disabled or crashed |
| Framed, bounded plugin protocol and effect-site authorization | VI, IX | Versioned Content-Length UTF-8 JSON-RPC, finite queues/blob resources, instance-bound identity and authorization; no raw credential/executor leaks |
| Image Paste as first vertical slice | VII | One clipboard snapshot from the triggering Client, bounded PNG handling, completed upload before insertion proposal, safe context validation and retained-result fallback |
| Remote namespaces and safe insertion | V, VII, XII | SFTP/Shell paths remain distinct; encoding is tested by target Shell; unknown mapping/context does not produce a guessed usable path or blind input |
| Native platform and terminal fidelity | II, IV, XI–XII | Distinguish OS, terminal, Shell and display system; actual Windows/macOS/Linux tests are recorded separately; terminal bytes and passthrough streams retain their semantics |
| Later plugin and integration scope | VIII, XI | History remains opt-in structured Shell data; Core owns forwarding mechanisms; optional remote hooks do not silently install permanent changes |

### Defaults, unknowns and superseded alternatives

- Rust plus system OpenSSH is the source's default technical baseline, not a claim that a dependency/toolchain has been chosen or tested. Tokio, portable-pty, Serde/TOML, Ratatui/Crossterm, arboard and SQLite remain candidates (X).
- Phase 0 must select one SFTP backend using evidence, verify reuse per client/platform/version, compare bounded VT state with restricted redraw, test prefix keys and clipboard access, and document unavailable environments (V, XI).
- The [architecture package](../docs/architecture.md) now defines proposed schemas, limits, errors, lifecycle policies and six ADRs. These are detailed design targets, not existing APIs, selected dependencies or runtime evidence; evidence-dependent backend/restoration choices remain deferred (VI, XIII).
- The source supersedes built-in-first plugins and official-plugin access to private Core traits (I). It does not authorize renaming the SSHX codename now.
- Initialization policies supersede generic Web/frontend and backend scaffolds for this repository. Generic template checkboxes stay unchecked; application code examples cannot be supplied before code exists.

### Constraints and non-goals

- Preserve OpenSSH host verification and credential ownership. Do not cache/replay passwords or expose agent access, full environment, terminal content or clipboard data to unapproved consumers.
- Core uses authenticated user-local IPC, not a TCP management port. IPC randomness is not authorization. Independent processes are failure isolation, not an OS sandbox; initial plugins must be explicitly trusted local packages.
- No basic-login requirement for SFTP or remote Agent. No implicit file overwrite, continuous clipboard monitoring, terminal recording or history persistence.
- No full GUI/remote IDE, cloud accounts, AI service, plugin marketplace or WASM requirement. Container tests and cross-compilation do not substitute for native clipboard, ConPTY or terminal evidence.
- Detach process survival, screen restoration and recovery after network/Core/machine failure have separate acceptance criteria. The complete product contracts are not implemented; Phase 0 probes separately measure bounded examples.

## Continuation

- mode: guided
- serial authorization: none
- next pulse: after the current task closes, or on a relevant user request with no active task
- permitted next action: execute the user-approved three local commits, fast-forward merge and supported task archive; report the retained general-root limitation without starting Phase 1
- product implementation: this task proposes Phase 0 probes only; the application MVP and other product phases remain outside its scope and serial authority

## Ordered work

These source-ordered rows distinguish completed design work from later proposals; they are not a scheduler. The architecture/design deliverable precedes Phase 0 as required by source XIII. A ready recommendation is not authority to execute it.

| order | task / proposed work | state | readiness and dependency evidence |
| --- | --- | --- | --- |
| 0 | [Trellis Plus bootstrap](tasks/archive/2026-10/10-21-trellis-plus-bootstrap/prd.md) | complete | Policy/source/checks delivered, both tasks archived with verified attributed commits, archived contexts valid and session recorded |
| 1 | [Architecture and feasibility plan (XIII)](tasks/archive/2026-10/10-22-sshx-architecture-feasibility/prd.md) | complete | Eleven documents and all AC1–AC8 delivered; review `human-not-needed`; work `9b483af`, archive `04dd6b6`. Completed date, archived references and unique attribution verified; Session 2 records checks and limits |
| 2 | [Phase 0 feasibility (XI)](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/prd.md) | complete | User accepted the controlled negative deliverable; 14 unit/44 native checks and exact cleanup pass. Work `620ca22` fast-forward merged to main; supported archive completed. General-root guarantee remains no-go; archive commit/journal evidence pending |
| 3 | Phase 1 C/S and independent-plugin MVP (XI) | proposed / blocked | First resolve the explicit remote-root capability/environment scope decision; Phase 0 acceptance does not establish a general safe-root backend. Then plan the primary-platform slice, plugin failure isolation and honest platform gaps |
| 4 | Phase 2 TUI and common plugins (XI) | proposed | Depends on MVP; switching/attach/screen/input handoff and File Transfer/Port Forward/Command Reference |
| 5 | Phase 3 structured remote integration (XI) | proposed | Depends on supported Shell contracts and authorization; History/Clipboard Bridge and auxiliary interactive authentication |
| 6 | Phase 4 releases and ecosystem (XI) | proposed | Depends on supported platform evidence, packaging/update/trust decisions; untrusted execution requires verified OS sandboxing first |

## Evidence and decisions

- Completed evidence: the supplied design is retained in spec, exactly 30,205 bytes with SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`; this is document fidelity, not product verification.
- Installation evidence: `.trellis/.version` and installed package metadata both identify 0.6.17; all 87 files listed by existing template hashes match, and all 28 installed Python scripts match distribution templates.
- Verification evidence: both role contexts have 13 valid references and observed complete agent reads; 107 original protected/input fingerprints, 87 template hashes, source/notice bytes, local links and document whitespace checks pass. See [full report](tasks/archive/2026-10/10-21-trellis-plus-bootstrap/check-report.md) and [review](tasks/archive/2026-10/10-21-trellis-plus-bootstrap/research/check-review.md).
- Current limitation: future tasks need explicit policy reads/context registration; a discovered spec index does not prove automatic policy loading. No actual Trellis update or native application runtime has been exercised. Final checklist: 21 PASS, 1 GAP (automatic future loading), 1 UNKNOWN (actual update cycle), 11 N/A.
- Version-control follow-up: on 2026-10-22 the user separately requested committing the remaining original initialization files. The 54-file runtime/template/AGENTS/attributes/workspace-index set was reviewed against Trellis 0.6.17 provenance and existing exact license before inclusion. This authorizes tracking those unchanged files, not editing upstream runtime; personal integrations and local state remain excluded.
- Closure: generic bootstrap supersession is recorded, its original unchecked checklist preserved, and documentation review is `human-not-needed`; the approved [batch](tasks/archive/2026-10/10-21-trellis-plus-bootstrap/research/commit-plan.md) produced completed archive status/dates and one exact Codex trailer per archive commit. No initialization task remains active.
- Initialization session evidence: [Session 1](workspace/wkyuu/journal-1.md) records the actual work/checks/limits and both bootstrap archives; its developer index reported one session at that closure. The separate initialization journal commit retains final post-archive evidence, without another task trailer.
- Current delivery evidence: [implementation report](tasks/archive/2026-10/10-22-sshx-architecture-feasibility/research/implementation-evidence.md) and [independent check](tasks/archive/2026-10/10-22-sshx-architecture-feasibility/check-report.md) establish eleven design documents, thirteen primary-source reviews, native injection plus direct-read fallback and AC1–AC8 verification. Independent snapshot: 21 Markdown files and 80 local links/fragments; pre-commit check: 22 Markdown files, 82 links/fragments and exactly 25 work-commit files; post-archive links: 81. Five JSON examples, all twelve fault groups and archived contexts 12 implement / 13 check pass. The 30,205-byte source/hash remains exact; final original-file changes are limited to mainline/shared index/contracts and the two authorized journal files. Metadata/trigger dedup gaps were corrected. Mermaid was manually inspected without rendering; no application/native evidence exists.
- Closure authorization: [explicit commit/archive plan](tasks/archive/2026-10/10-22-sshx-architecture-feasibility/research/commit-plan.md) was approved on 2026-10-22 and executed through supported no-auto-commit helpers and explicit paths. Work commit is `9b483af56c319ef0aff21ac37a4536d17dcb46e6`; archive commit is `04dd6b67b3119c2f3e9e6777f259d0752a51bb8a`, with completed date/status, cleared task pointer and exactly one Codex attribution trailer verified. `human-not-needed` applies to the checked documentation scope.
- Session 2 evidence: [journal](workspace/wkyuu/journal-1.md) records the actual work commit, document/archive checks, runtime limitations and guided next step; its developer index reports two sessions. The separate journal commit receives no task attribution trailer.
- Phase 0 planning evidence: [readiness](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/readiness.md) records Debian x86_64/OpenSSH, missing PATH Rust tools, scoped Docker read-only access, actual cached-image architectures and primary-document caveats. Proposed builds use an amd64 cached Rust image; test SSH and X11 are disposable loopback/private fixtures, not personal targets/clipboard. During planning no executable probe, fixture startup or dependency retrieval had occurred.
- Phase 0 execution evidence: [fixture report](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/fixture-evidence.md) verifies Rust/Cargo 1.90 in the pinned amd64 build image, host-visible UID-mapped mounts, native ELF ABI smoke, strict-key loopback SSH/SFTP and exact fixture teardown. Cargo builds exclude fixture credentials. At that infrastructure checkpoint Core/visual/plugin acceptance and final integration cleanup were still in progress; the final results and no-go disposition are recorded below. Feature branch is `phase0-feasibility`; the later work commit `620ca22a49611e319daf6df9f9aa22113d428133` is now fast-forward merged to local main.
- Phase 0 delivery checkpoint: [formal review](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/check-report.md), [AC/XII map](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/acceptance-map.md) and [cleanup](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/cleanup.md) record independent 14-unit/44-native success, supplemental 3-case success, fixed findings and no remaining owned services/credentials/processes. General SFTP ancestor binding is no-go; hidden screen/full API/real Shell/platform limitations remain explicit. `human-required` disposition was subsequently supplied by the user’s “接受”; AC4/AC9 are accepted for this controlled negative deliverable, with no MVP-ready claim. Work `620ca22` is committed and fast-forward merged; the supported task move is complete. Archive commit and journal are next in the approved batch.
- Current user decision: on 2026-10-22 the user replied “接受” to the precise Phase 0 negative feasibility conclusion and [concrete local batch](tasks/archive/2026-10/10-22-sshx-phase0-feasibility/research/commit-plan.md). The listed work/archive/journal commits and fast-forward merge are authorized; no automatic application/MVP continuation or broader Git batch is granted.

Initialization work commit: `7aa9291a0e49a522c962362af095586081531389`. The [generic bootstrap](tasks/archive/2026-10/00-bootstrap-guidelines/prd.md) archive commit is `619ecabcf7342b56803b7dfed08f24fdc8b83962`; the Trellis Plus task archive commit is `9d1a70ef7fca39db4950d3ca1abb2352d32b0fbf`. Both tasks completed on 2026-10-21, archived references validate, and initialization wrap-up is complete. Product/runtime claims remain proposed or unverified as above.

- Phase 0 archive checkpoint: the supported `archive --no-commit` route completed on 2026-10-22; status/date, actual destination, feature-branch ancestry and cleared current-task pointer are verified. Archived JSONL/Markdown and external project references were reconciled without changing runtime/templates or source-bound machine reports. Archive commit evidence is recorded after creation; Session 3 remains pending.
