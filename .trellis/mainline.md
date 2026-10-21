# Trellis Mainline

## Initiative

- title: SSHX — cross-platform SSH sessions and independent plugins
- parent task: none; the current initialization task is not a product epic
- current approved objective: establish project-owned Trellis policies and import the supplied SSHX design, without implementing application features
- owner decision: on 2026-10-21 the user requested Trellis Plus initialization, consented to the task, approved its final plan, and then explicitly requested importing the source into spec and mainline
- current task: [Trellis Plus bootstrap](tasks/10-21-trellis-plus-bootstrap/prd.md), `in_progress`

## Requirements and sources

The canonical repository-local design is [SSHX design](spec/trellis-plus/sshx-design.txt). It was imported byte for byte from the user-supplied file; see [provenance](tasks/10-21-trellis-plus-bootstrap/research/source-provenance.md). Import is approved. It establishes the direction and constraints for future planning; the approved execution scope in this session is initialization only. There are no application sources or runtime acceptance results yet.

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
- Precise schemas, limits, errors, lifecycle policies and ADR conclusions are design deliverables still to produce; proposed method/command names in the source are not existing APIs (VI, XIII).
- The source supersedes built-in-first plugins and official-plugin access to private Core traits (I). It does not authorize renaming the SSHX codename now.
- Initialization policies supersede generic Web/frontend and backend scaffolds for this repository. Generic template checkboxes stay unchecked; application code examples cannot be supplied before code exists.

### Constraints and non-goals

- Preserve OpenSSH host verification and credential ownership. Do not cache/replay passwords or expose agent access, full environment, terminal content or clipboard data to unapproved consumers.
- Core uses authenticated user-local IPC, not a TCP management port. IPC randomness is not authorization. Independent processes are failure isolation, not an OS sandbox; initial plugins must be explicitly trusted local packages.
- No basic-login requirement for SFTP or remote Agent. No implicit file overwrite, continuous clipboard monitoring, terminal recording or history persistence.
- No full GUI/remote IDE, cloud accounts, AI service, plugin marketplace or WASM requirement. Container tests and cross-compilation do not substitute for native clipboard, ConPTY or terminal evidence.
- Detach process survival, screen restoration and recovery after network/Core/machine failure have separate acceptance criteria. None is currently implemented.

## Continuation

- mode: guided
- serial authorization: none
- next pulse: after archive or a relevant user request with no active task
- permitted current action: execute the user-approved four local commits and two initialization archives; verify actual closure evidence and retain guided product continuation
- product implementation: proposed; each selected item requires the normal task-creation/planning/start gates

## Ordered work

These source-ordered product rows are proposals, not created child tasks or a scheduler. The architecture/design deliverable precedes Phase 0 as required by source XIII. A ready recommendation is not authority to execute it.

| order | task / proposed work | state | readiness and dependency evidence |
| --- | --- | --- | --- |
| 0 | [Trellis Plus bootstrap](tasks/10-21-trellis-plus-bootstrap/prd.md) | active | Policy, source import and documentation checks complete; user approved the concrete commit/archive batch, execution in progress |
| 1 | Architecture and feasibility plan (XIII) | proposed | Next candidate after initialization closure and user selection; deliver requirement classification, platform matrix, processes, data/state/protocol design, Image Paste failure sequence, ADRs and executable Phase 0 plan |
| 2 | Phase 0 feasibility (XI) | proposed | Depends on preceding plan, selected primary native environment and controlled SSH/SFTP fixtures; collect methods/results/limitations per platform |
| 3 | Phase 1 C/S and independent-plugin MVP (XI) | proposed | Depends on evidenced Phase 0 choices; primary-platform vertical slice, plugin failure isolation and honest other-platform gaps |
| 4 | Phase 2 TUI and common plugins (XI) | proposed | Depends on MVP; switching/attach/screen/input handoff and File Transfer/Port Forward/Command Reference |
| 5 | Phase 3 structured remote integration (XI) | proposed | Depends on supported Shell contracts and authorization; History/Clipboard Bridge and auxiliary interactive authentication |
| 6 | Phase 4 releases and ecosystem (XI) | proposed | Depends on supported platform evidence, packaging/update/trust decisions; untrusted execution requires verified OS sandboxing first |

## Evidence and decisions

- Completed evidence: the supplied design is retained in spec, exactly 30,205 bytes with SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`; this is document fidelity, not product verification.
- Installation evidence: `.trellis/.version` and installed package metadata both identify 0.6.17; all 87 files listed by existing template hashes match, and all 28 installed Python scripts match distribution templates.
- Verification evidence: both role contexts have 13 valid references and observed complete agent reads; 107 original protected/input fingerprints, 87 template hashes, source/notice bytes, local links and document whitespace checks pass. See [full report](tasks/10-21-trellis-plus-bootstrap/check-report.md) and [review](tasks/10-21-trellis-plus-bootstrap/research/check-review.md).
- Current limitation: future tasks need explicit policy reads/context registration; a discovered spec index does not prove automatic policy loading. No actual Trellis update, native application runtime or archive attribution commit has been exercised.
- Dirty-state warning: the repository has no commits; existing runtime/templates and user files were already untracked. Do not stage them as part of Trellis Plus.
- Closure: generic bootstrap supersession is recorded, its original unchecked checklist preserved, and documentation review is `human-not-needed`; the user approved the [concrete commit/archive batch](tasks/10-21-trellis-plus-bootstrap/research/commit-plan.md). Both tasks remain active until the authorized archive moves and commits succeed.
- Next user decision: none for initialization closure; product work remains guided and requires later selection.
