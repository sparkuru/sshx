# SSHX Development Principles

Read for every project task together with the [shared index](index.md), [mainline](../../mainline.md) and task acceptance criteria. SSHX is at design/bootstrap stage; no released SSHX compatibility promise or disposable application database has been established.

## Scope and reuse

- Implement only the authorized task and evidenced near-term needs. Search with `rg` before adding or changing a helper, model, field, constant, command or configuration consumer. Reuse the real execution path; avoid parallel wrappers and speculative adapters, caches, provider systems or extra crates.
- The independent-plugin and local Client/Core boundaries are explicit product requirements, not optional abstractions to remove for convenience. Within those boundaries, choose the smallest testable implementation; follow [SSHX contracts](sshx-contracts.md).
- For unreleased internal prototypes with no external commitment, update schema, callers and tests together and remove superseded names. Do not invent legacy aliases, dual interfaces or migration chains. Verify any actual external consumer/release before deciding compatibility is unnecessary.
- Add dependencies only for a concrete requirement that existing facilities cannot reasonably meet. Validate the exact selected version's official interface before using it. Source candidate libraries are not installation instructions.
- Keep unrelated refactoring, mass formatting, renaming, dependency upgrades and repository reorganization out of the change. Include a discovered issue only if it blocks the agreed outcome or the user expands scope.
- Expected failure is an explicit result with correct exit/error semantics. Do not hide missing resources behind success-looking defaults or silently drop SSH arguments.

## Real data and authentication exceptions

Development status does not make SSH configuration, `known_hosts`, keys, agent access, desktop clipboard, terminal buffers, local user files, plugin data or remote files disposable. Preserve them unless the requested operation explicitly authorizes replacement of identified disposable data. Never reset a user profile, database or remote directory as automatic recovery.

Authentication and permissions are central behavior under test. Do not bypass OpenSSH host-key checks, password/MFA interaction, IPC peer verification, attachment ownership or plugin capability checks to simplify development. Use isolated test identities, fixtures and separate credentials; keep secrets out of source, messages, fixtures, logs and plugins. Production credentials are not test fixtures.

Core is a local user service. Generic trusted-LAN preview convenience does not apply: do not add a TCP management listener, bind Core on `0.0.0.0`, publish it from Docker or weaken local IPC authorization. Future Port Forward capabilities default to loopback and require explicit choice for broader exposure. Any later unrelated Web preview must establish its own documented access boundary before starting.

No convenience login, CAPTCHA or session-expiry workaround currently applies because no application exists. If a later isolated development interface has unnecessary friction, use an explicit separate development configuration without disabling the authentication or permission behavior its acceptance tests need to exercise.

## Diagnose and validate

Before a bug fix, establish observed behavior and the execution path, reproduce when possible, and distinguish confirmed cause from hypothesis. Missing reproduction is a recorded limitation. Do not rewrite contracts and tests around a guess and cite them as proof of the original cause.

Check observable requirements and failure cases, not implementation shape alone. Preserve meaningful assertions and error propagation; do not mask the behavior under test with mocks, unconditional fallbacks, broad selectors or weakened checks. Controlled fixtures must identify what they do and do not verify.

Run the smallest relevant existing checks, adding lasting coverage where it protects a meaningful contract. Builds and unit tests do not prove real terminal, clipboard, ConPTY, authentication or remote path behavior. Record actual commands, environment, outcomes, failures and unverified surfaces using the [validation profile](validation.md). Stop and expose material failures before completion rather than expanding tests indefinitely or reporting unexecuted checks as passed.

## Workspace and documentation

Inspect status and affected files before writing; pre-existing work belongs to the user. Preserve edits from other workers, unknown files, Git identity and history. No destructive reset, broad cleanup or unrelated staging. Remove only known task-created temporary artifacts; use `/tmp` for scratch outputs and normal Trellis task records for durable evidence.

README is human-owned presentation: create or change it only on explicit request. Agent conventions belong here, task-specific design/results in the active task, and project direction in mainline. Do not manufacture examples from nonexistent application code or add redundant guides to make initialization look implemented.

At review, compare the actual change with acceptance, check evidence, data ownership and scope. Record only reusable decisions in shared specs; task failures and diagnostics stay with their task. Apply the review and commit/archive gates before declaring completion.
