# SSHX Mainline Continuity

[Mainline](../../mainline.md) preserves direction and evidence across normal Trellis tasks. Read it with the [shared index](index.md), task acceptance and applicable [source sections](sshx-design.txt) at start/resume, before commit/archive and before choosing later work. This is project data, not another task schema or scheduler.

## Source and approval boundaries

The retained design is current requirements input authorized for import. Keep explicit source decisions separate from Rust/default choices and technical unknowns; preserve superseded alternatives as superseded. This initialization authorizes policy, source import, continuity and checks only. Product execution and ordered future phases remain proposed work until explicitly authorized.

For later imports, inspect bounded current project sources, cite actual paths/sections and distinguish approved, proposed, conflicting and superseded content. Do not move/delete/rewrite source text, manufacture source dates, or promote a draft by filename, recency or implementation drift. Reuse already approved requirements; ask only for an actual conflicting/ambiguous decision affecting acceptance. Merge into the existing record without duplicating work rows or erasing prior user edits.

## Lifecycle updates

The main session maintains initiative, scope/non-goals, source/approval record, ordered tasks, dependencies/readiness, unresolved decisions and actual check/archive evidence in mainline.

- At task creation/start, map its acceptance to the authorized initiative and normal parent/child relationship; proposed order is not readiness or consent.
- After interruption, read current task/source/mainline instead of treating the latest question as a new scope. Resolve genuine conflicts before edits.
- On approved scope changes, update constraints, acceptance, affected dependencies and decision evidence together.
- After checks, record verified outcomes and material gaps; implemented-but-unverified behavior is not completed acceptance.
- At archive, replace active location with actual archive evidence, work/archive commit references when available, remaining scope and next decision. Follow [archive retry/attribution rules](commit-policy.md); do not claim a pending commit exists.

Keep detailed diagnostics in task records and session history in normal journals. Mainline is a concise control record; no daemon, global hook or new task-state parser is needed.

## Relevant no-task requests: read-only Project Pulse

For project status, next-step or continuation requests when no task is active, and immediately after archive, inspect mainline, active/archived task evidence, Git state and available check results. Report the current objective and completed evidence, precise blocker/dirty warning, one uniquely ready candidate if evidence determines it, and next permitted action. Unrelated conversation does not trigger a Pulse.

| Situation | Permitted action |
| --- | --- |
| No declared objective/source evidence | State what is missing and ask for the one priority decision |
| `guided` (default), one ready candidate | Recommend with evidence and wait for user selection before task creation/product edits |
| `guided`, competing/unclear candidates | Explain the choice and request ordering; do not choose a new roadmap as a worker |
| `paused` | Report current state only; do not create tasks or implement |
| Explicit `serial` authorization | Advance only the next named ready child in the authorized initiative/list/order after clean closure |
| Unrelated dirty work, unresolved material checks, incomplete/uncertain archive, ambiguity, new risk, scope change or unmet dependency | Stop dependent continuation; report the exact condition and resolve it or obtain direction |

Routine task-owned changes during an active authorized task do not by themselves forbid completing that task. Do not clean or absorb unrelated work to make continuation appear ready.

Serial authorization substitutes for repeated creation/start consent only for its explicit bounded list. It does not waive planning artifacts, human-review checks, commit authorization or closure evidence. Do not infer serial authority from the design's instruction to make progress. No active task can be selected after archive from a supposed `completed` resolver event; use the real no-task/post-archive state.

## Main session and workers

Main session owns phase/scope decisions, task lifecycle, context registration, mainline/Pulse, commit and archive. Workers receive the active task path, bounded responsibility and actual context; they preserve other edits, report files/checks/gaps, and return unresolved decisions. Workers do not choose the next initiative, perform project-wide cleanup or archive independently. A dispatched implementation/check role performs that role directly rather than recursively dispatching another implement/check agent.

Future-task automatic loading is not established by this file's existence. Use the [index's explicit loading procedure](index.md#actual-loading-path), verify both role contexts and retain any unsupported-integration gap in task results.
