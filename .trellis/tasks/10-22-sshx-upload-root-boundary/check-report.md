# Independent cooperative upload-root document check

Date: 2026-10-22. **The approved document-only contract passes independent review after one local consistency correction.** All eight existing result/spec/mainline targets were reviewed against R1–R5 / AC1–AC5. No material unresolved product-contract finding remains. Main retains acceptance, review-gate classification, lifecycle and separate Git/archive authority.

## Findings (fixed)

- File: `docs/protocol.md`, Operation model and Images/result types.
  Issue: the pre-existing abbreviated Operation.result shape used `hash_evidence` while the detailed upload result defined `size`, `integrity` and `final_commit`. Two competing field lists would make AC3's result consistency ambiguous.
  Fix: reference the single detailed shape as the document-only alias `UploadResult`; explicitly preserve null before a result exists and the same shape for recorded indeterminate outcomes. This introduces no wire wrapper, method, field or implemented API. Main confirmed this mechanical consistency correction is within scope. No other result document was changed by this reviewer.

## Findings (not fixed)

None requiring a current document correction or a new product choice. The following are explicit, accepted scope limits, not passing runtime assertions:

- Cooperative root is a logical canonical SFTP namespace scope under trusted local policy and stable remote-account/server/ancestor/resource cooperation through cleanup. It does not enforce confinement against hostile same-account replacement or server behavior; metadata, 0700 and random names do not prove identity.
- Actual per-target policy enforcement, hardlink version/filesystem eligibility, collision/cross-device/lost-reply/cleanup behavior, generic Shell context and broader platform support remain unimplemented/unverified. A separately authorized runtime task must establish them. Historical baseline-rename probes, archived reports and `phase0_ready: false` are unchanged.
- No Cargo/native/Docker/SSH/SFTP/clipboard experiment or dependency retrieval ran. Mermaid was reviewed as text; no renderer/browser/UI result is claimed. Main must still record its acceptance/review disposition and obtain the separate concrete Git batch authority.

## Verification

- **Lint: pass for document/data integrity.** `git diff --check`, UTF-8/final-newline/trailing-whitespace checks include tracked and new task files. This is not an installed Markdown linter pass.
- **TypeCheck: not applicable.** No executable language/type changes. Five fenced JSON examples parse and were reviewed against their documented request/error/evidence field sets, including the 54-byte compact unknown-method example. The corrected result alias has one authoritative field shape.
- **Tests: document contract/integrity checks pass.** Both manifests validate with 21 unique existing inputs each; package discovery identifies the trellis-plus layer. Full paths, commands, counts and frozen eight-file fingerprints are in [review evidence](research/check-review.md) and [machine integrity evidence](research/check-integrity.json).
- **Source and scope preservation: pass.** All 168 baseline tracked files remain; exactly the eight authorized originals differ and the other 160 are byte-identical, including probes/lockfile/wrappers, archived evidence, source/notices, generic scaffolds and Trellis runtime. Canonical source is 30,205 bytes, SHA-256 `d115212b36001c4f0f83f128fb0af5f15623d20121bc0e5820d23da76893d4ff`; the retained Trellis notice hash also matches. Branch is `upload-root-boundary`, HEAD remains `a91878ecf9adc5a2b72f123e0bedc63e3e192428`, and the index is empty.
- **Context: complete with fallback.** The saved native payload was read, then every omitted/truncated registered input and PRD/design/implement was recovered directly. Full protocol segments were read despite the 32,768-byte injection cap; no loader/config change was made. Exact loading paths and known warning are recorded in review evidence.

R1/AC1 actor/root/threat scope, R2/AC2 explicit A/local policy, R2–R3/AC3 error/publication/cleanup/insertion rules, R4/AC4 candidate versus historical evidence, and R5/AC5 synchronized documents/data/preservation are satisfied for this document delivery. This does not establish implemented publication capability, Phase 0 PASS, MVP readiness or later-task authority. Reviewer edits are complete; the corrected documents are stable.
