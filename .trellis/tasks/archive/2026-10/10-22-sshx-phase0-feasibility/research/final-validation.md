# Final project-data validation

Date: 2026-10-22. **Final project-data checks pass; subsequent human review and local Git/archive authority were supplied by the user’s “接受”.** The observations below describe the pre-staging validation snapshot; subsequent authorized bookkeeping is recorded separately. This record covers document/data integrity after the independent native run and exact cleanup. The [formal check report](../check-report.md) remains authoritative for executed code checks, fixed findings and limitations. No native fixture was restarted for these document-only updates.

## Final observations

- Candidate inventory: **80** individually classified paths, comprising **14** authorized existing project documents and **66** new original source/tooling/task files. The [concrete batch](commit-plan.md) lists each work path, each task move and affected external reference. Unrecognized dirty files: none.
- All **102** pre-existing tracked files remain present; exactly the 14 authorized result/spec/mainline documents differ. The other **88**, including runtime/templates, generic scaffolds, original README, notice inventory and retained license, match the planning fingerprints.
- Canonical design is unchanged: **30,205 bytes**, SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`. Retained Trellis license matches SHA-256 `d8a6cc31abc16b6748c7a21f21611f5a1ec33f67d22ca23d7da1c19b95496bee`.
- All **17** recorded final execution source fingerprints still match. The canonical 44-case report matches SHA-256 `c74bb4208be5f101b2a9cf77eb5c83cf1783326044f78bad419c3c891c23678f`, records `controlled_linux_cases_passed: true` and retains **`phase0_ready: false`**. Final cleanup records both exact containers, credentials/pointers and port bindings removed, with no owned task process/container remaining.
- Structured data parse: **13 JSON files**, **2 JSONL files / 46 entries**, **5 TOML/lock files**. Supported task validation passes with **23 entries per role**. Its known 46,819-byte protocol injection warning remains explicit; complete direct reads were recorded, and the loader/config was preserved.
- Every candidate is a regular UTF-8 file with a final newline and no trailing whitespace. Candidate scan finds no private-key PEM. Root wrappers and SSH fixture entrypoints retain executable bits. Markdown local destinations and heading fragments resolve; code blocks, inline code and external URLs are excluded from this local-link check.
- `git diff --check` passes. The index is empty. Branch remains `phase0-feasibility`; HEAD and `main` remain `ec1e1470381b67a07cef4885ea057f843e5f7578`. No staging, commit, merge, task completion or archive has occurred.

## Phase 3.3 spec-sync judgment

This task produced non-obvious reusable knowledge, so a “nothing to update” disposition would be incorrect. Project-owned development/contract/validation/index documents now record the actual pinned Docker/native command profile, independent plugin/profile limits, strict PNG framing and exact streaming decompression requirement, source preservation and measured versus unavailable validation. The PNG regression demonstrates why decoder success/best-effort limits alone do not establish exact scanline length or hard allocation bounds.

ADRs, runbook, architecture and platform records now preserve the general mutable-parent/root SFTP no-go, cooperative-only route, restricted redraw choice and tested Linux scope. Existing protocol authorization/counter/writer/cancellation contracts already specified the boundaries exposed by the fixed code defects; lasting unit/native effect regressions verify them without weakening those contracts. Queue accounting is explicitly a finite wire-byte/frame bound, not complete allocation/RSS accounting. No Trellis runtime, generic scaffold or canonical source was changed.

## Commands and evidence boundary

Final main-owned checks used read-only Git state/diff inspection, supported `task.py validate`, and `/tmp/sshx-phase0-final-integrity.py` for baseline/source/report hashes, candidate classification, JSON/JSONL/TOML parsing, local destinations/fragments, formatting, executable bits and private-key PEM checks. The temporary validator is an execution aid, not shipped probe code. Rechecked after writing this record and the plan; all assertions pass. Existing independent fmt/strict Clippy/Rust check/build, 14 unit tests, 44 native cases and supplements were reused because executable source did not change.

These observations establish a reviewable controlled deliverable. They do not resolve the general-root guarantee, prove full product/platform readiness or supply the required user disposition.

## Authorized archive validation

After the user’s explicit acceptance, work `620ca22a49611e319daf6df9f9aa22113d428133` was fast-forward merged to local main and the supported no-auto-commit helper archived the task. Status `completed`, completion date 2026-10-22 and cleared active pointer are verified. All 35 archived task files and 10 external project-data references are individually classified; the resulting 80 old/new/external staging paths contain no unrelated changes. Archived contexts validate with 23 entries each; all 170 local links and 13 heading fragments resolve. All 17 source fingerprints and source-bound JSON reports remain unchanged; code, canonical design and retained license are preserved. Formatting/data checks pass. No native fixture was restarted. Archive/journal commit OIDs are recorded in mainline after creation.
