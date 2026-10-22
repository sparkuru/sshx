# Independent review focus

Main-session checkpoints prepared before implementation, 2026-10-22. These are review assertions, not results.

- Trace explicit fixture-owner policy into Core; an action/plugin request must not mint cooperative scope or trusted qualification. Verify capability evidence includes exact target/config/namespace/root and is invalidated, not merely a boolean supplied by the test RPC.
- Keep fixture qualification effects separate from user upload effects. A policy-denial test cannot silently create a user operation directory while qualifying capability. Qualification evidence and privileged Docker observations may establish fixture expectations, not plugin authority.
- Trace normal upload through direct hardlink. Search all actual Core callers for rename/remote-command/scp fallback; historical backend comparisons must be unmistakably separate.
- For acknowledgement loss, verify request/reply IDs and that the intended server reply is actually withheld. Mark dispatch even if the future later errors. A final hash observed by a second connection is evidence of remote state, not permission to upgrade the first operation or clean its uncertain paths.
- Inspect cleanup after every early exit (mkdir/open/write/close/link/readback), cancellation and transport error. An acknowledged link must remain committed even if readback fails. A failed/unknown mkdir or open cannot establish cleanup ownership. Final files/directories and replacement paths cannot be removed as compensation.
- Every path unlink/rmdir needs its own fresh continuity/admission check; cancelled plugin authority must not disable safe narrow cleanup or authorize publication. Same-looking metadata cannot reestablish lost authority, and negative tests must not claim detection of unobservable ABA.
- Verify generic SFTP FAILURE retains uncertainty/generic error unless independent runtime evidence justifies a specific classification. A test fixture knowing EXDEV is not proof the client received EXDEV.
- For missing policy, wrong grant, stale binding and capability failure, assert actual filesystem and PTY effects, not only error strings. For retries, assert request/publication counts and stable assigned paths, not just repeated JSON.
- Re-run integrated Core cases with fresh output, inspect any historical assertion changes, and confirm image/protocol consumers remain coherent without invoking an unauthorized display/clipboard. Full product v1 conformance remains outside this probe task.
- Verify original design, notices, lockfile/dependencies, archived reports and protected Trellis files against research/baseline.json. Record all changed/new paths and any exceptions with evidence.
- Teardown must identify exact repo/run/container/scope and actual recorded credentials/processes. hako's static historical task label alone is insufficient current-task ownership; use the fresh run ledger. Destruction of the disposable fixture after observations is distinct from successful automatic Core cleanup.

The user approved the final implementation/runtime plan. Security enforcement and path deletion changes still require the project human-review disposition before commit/completion/archive. Prepare a concrete result and scoped Git plan after runnable checks; do not request generic review in place of performing them.
