# Exact fixture teardown

Completed 2026-10-23 after implementation and independent native checks exited successfully and both agents released the fixtures. [Machine verification](cleanup.json) records exact identities and observed absence.

Main loaded the reviewed lifecycle module under isolated Python, locked the existing primary lifecycle lock, and compared both current pointers against fixtures.json before removal. Each lifecycle down verified exact repository/run/container/scope labels through hako. The secondary was removed first, then the primary; the now-empty secondary runtime directory was removed. Both generated credential directories and pointers are gone. No unknown service was selected by a broad label or name search.

Both exact container inspections return absent, loopback ports 32781 and 32782 were successfully rebound, and no matching native probe or fixture-ID SSH process remains. The pre-existing primary lifecycle.lock and pinned images/development caches remain. Task reports and known scratch validation logs are retained; no generated key/password body appeared in the candidate-file scan before credential deletion.

Some negative cases intentionally left final or uncertain files inside the disposable servers for observation. Destroying those exact containers is authorized fixture-owner teardown; it does not claim Core automatically deleted uncertain remote paths or final user data.
