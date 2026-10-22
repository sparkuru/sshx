# SSHX Validation and Submit-Ready Review

Read when defining acceptance, checking a change and before staging, commit, completion or archive. Evidence derives from actual repository/task files; record each command and result in the task, with unavailable checks distinguished from passed checks. Initialization baseline: original design/documentation only. Phase 0 now has a scoped Rust probe workspace and native fixture suites; no released application, CI or browser UI is established. Exact task evidence distinguishes container checks from host-native experiments.

## Initialization validation profile

Set `TASK_DIR` to the actual current task path and run from the repository root:

```sh
python3 .trellis/scripts/task.py validate "$TASK_DIR"
python3 .trellis/scripts/task.py list-context "$TASK_DIR"
python3 .trellis/scripts/get_context.py --mode packages
git diff --check
git status --short
```

These check context structure, explicit references, layer discovery, tracked whitespace and worktree inventory. They do not prove that an agent read the documents. Inspect resolved injection or worker loading reports for both implement/check; register all needed detail paths because the loader does not follow index links. See the [loading procedure](index.md#actual-loading-path).

For new/untracked files, `git diff --check` alone is insufficient. Also inspect trailing whitespace/end-of-file formatting for the exact task-owned text files, parse task JSON/JSONL, resolve local Markdown links relative to their containing files and confirm all referenced paths exist. Verify the exact imported source bytes/hash, compare retained notice text to its identified source, and compare protected/user-input fingerprints taken before writes. Do not normalize the verbatim design or notices to satisfy a style checker. If any target was already missing, record that fact rather than pretending it was preserved.

Inspect final ownership/path inventory, no secret/personal material, and preserved generic scaffolds. The initial repository has no commits and pre-existing untracked user work, so a clean-looking Git diff is not proof of unchanged content or task ownership. Record actual results in the task; this profile itself is not successful execution evidence.

The original documentation-only initialization could not pass application runtime checks. The approved Phase 0 task now exercises Cargo checks and controlled SSH/SFTP/plugin/terminal/image probes, with actual results recorded in task research and the [development profile](development.md). These results do not prove the complete product or untested native platforms. Browser/mobile tests remain inapplicable to these native probe binaries.

## Application risk profile (conditional)

Before application implementation, turn source XI–XII and the affected [contracts](sshx-contracts.md) into task-specific assertions and exact commands using actual workspace/fixture evidence. Keep a test/result mapping to acceptance. Reuse existing tests; do not install a framework solely to mirror implementation.

| Risk | Required evidence when affected |
| --- | --- |
| CLI compatibility | Native `ssh` untouched; argument/`--`/subcommand behavior, passthrough bytes/stdout/stderr/exit status; clear unsupported modes |
| Session/operation races | Multiple same/different-host sessions and uploads; A-to-B switching; detach, generation change, close, revoke and disable; no stale upload target/input |
| Protocol/trust | Framing byte limits, malformed/oversized JSON, handshake mismatch, spoofed identity, timeout/crash/restart limit, slow subscriber isolation and effect-site denial |
| IPC/auth | Unauthorized local peer refusal, endpoint races/ACLs, host-key change refusal, password/MFA and auxiliary `AUTH_REQUIRED`; credential exclusion |
| Image/transfer/path | Empty/corrupt/oversized image, missing desktop/SFTP, denied/full/partial/cancelled transfer, duplicates/symlinks, Shell quoting/control rejection, Windows/SFTP namespace differences, upload-versus-insert retry |
| Terminal/recovery | Unicode/IME/control keys/resize/full-screen programs/exit cleanup; writer ownership; detach survival, screen restoration, Core crash and network generation tested separately |
| Later persistence/forwarding | Opt-in structured history, unknown fields, pause/filter/retention/deletion, no automatic command execution; loopback defaults, port lifecycle and real listener/reachability states |

Use unit tests for state/encoding, contract tests for plugin/IPC protocols, controlled SSH/SFTP integration and actual native terminal runs. Containers can validate remote-service fixtures; cross-compilation and Docker cannot replace Windows ConPTY, macOS Terminal, desktop clipboard or ACL/runtime behavior. Record OS/architecture/version, terminal host, Shell, desktop/display (including X11/Wayland/headless), OpenSSH/backend/auth and remote OS per tested combination. Missing environments stay unverified rather than becoming universal support claims.

## Browser, native UI and mobile

Current initialization has no changed UI; proposed CLI/TUI is native. Browser validation and browser/mobile viewports are N/A now. A future native UI task uses real terminal/platform interaction and accessibility evidence; Web Playwright cannot substitute for it.

If an authorized browser surface is later introduced, classify before implementation: `playwright-required`, maintained `playwright-existing-equivalent`, `playwright-not-effective` for genuinely native/hardware/subjective limits, or `playwright-unavailable` with the exact blocker. Missing runtime/fixtures are not non-applicability. Automate changed interactions and final states with controlled fixtures and semantic locators before asking for human smoke tests.

That first browser task must add a single **Trellis Plus: Playwright Validation Profile** here, populated from real configuration: execution mode (`project-local`, `docker-wrapper` or `ci`), exact dependency/browser setup, startup/readiness/base URL, focused and full/CI commands, tests/config path, browser projects/viewports, fixtures/data boundary, mobile classification/device settings, accessibility/visual policy and failure artifact locations. Do not invent those values now. Future eligible tasks read the profile before rediscovering commands or consulting external docs.

For affected UI, classify `mobile-required`, `mobile-not-applicable` with scope evidence, or `mobile-unavailable` before implementation. General-purpose browser pages normally need desktop and narrow-mobile interaction/final-state checks; record them separately. Native-only SSHX has no present mobile requirement. Emulation does not establish physical-device behavior. Preserve failure reports, traces/screenshots, console/network logs and reviewed snapshot-change reasons. Use no production data or personal sessions; do not hide failures by changing baselines or assertions.

## Submit-ready human review gate

Evaluate and record exactly one category **before** a commit plan, staging, commit, task completion or archive. Installing this rule changes no task state. Complete available material checks first, compare the final change to acceptance and reuse prior explicit decisions/authorization.

- `human-required`: a material required check cannot run or is skipped; acceptance remains ambiguous; subjective product/visual judgment lacks an agreed baseline; private service/credentials, hardware/real device/assistive technology is necessary; or the change alters auth, permissions, security enforcement, billing, data migration/deletion, deployment/CI or irreversible behavior. This category blocks commit/completion/archive pending the specific missing evidence or decision. Documentation about future security contracts is not an implemented security change; any actual unresolved contract decision still blocks.
- `human-optional`: relevant checks passed and only low-risk subjective confidence remains. Say explicitly that optional feedback is nonblocking, and proceed only within existing commit/archive authority.
- `human-not-needed`: a documentation-only/mechanical change or focused acceptance coverage leaves no meaningful human-only judgment. State the concrete reason and actual checks. Documentation initialization can use this category only after its material checks pass; it does not grant commit permission.

When input is necessary, request only the residual signal: changed behavior, checks already run/results, exact manual scenario/environment, expected outcome and useful response format (pass/fail plus relevant reproduction/log/screenshot). Do not ask generic “please review” or delegate runnable browser/native automation to the user. A failed/unavailable material check is a limitation, not PASS. Resolve failures or obtain a scope/evidence decision before closing acceptance.

The main session evaluates this gate after the check agent reports, then applies the separate [commit/archive authorization and retry rules](commit-policy.md). Missing Git commit authorization is an authority boundary, not a failed product check or an extra optional-review gate.
