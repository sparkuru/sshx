# Final owned-resource cleanup

Main-session cleanup completed on 2026-10-22 after the independent 44-case native suite and the final current-writer insertion supplement. Both implementation and checking agents ended their fixture use before teardown. [Machine observations](cleanup.json) record exact identities and actual UTC observation time.

The main session loaded the existing reviewed lifecycle helper through Python isolated/no-site mode. Before mutation it verified each current pointer against the previously recorded exact run/container ID, state path and IPv4 loopback publication. It called `down(repo, secondary_runtime)` for the separate secondary pointer, then the existing `./dev.sh down` for the primary. The helper verified repo/task/run/scope labels before deleting each exact container. No global prune, process-name kill, port kill, host account change or unrelated-resource cleanup ran.

| Resource | Observed final state |
| --- | --- |
| Primary generated SSH service, port 32770 | Exact container absent; generated credential directory/current pointer absent; loopback port successfully rebound |
| Secondary generated SSH service, port 32771 | Exact container absent; generated credential directory/current pointer absent; loopback port successfully rebound |
| Other containers with this exact task label | None remain |
| Task-namespaced Core/image/Xvfb/xterm/Vim/agent/selection processes | None remain after read-only namespace-filtered process inspection |
| Existing display, personal SSH/agent data, unrelated Docker/process resources | Outside cleanup scope; unchanged |
| Ignored Cargo/build caches and local development/test images | Intentionally retained for reproducible reruns |
| Documented synthetic reports/PNG artifacts under `/tmp` | Intentionally retained for review; no generated credentials remain in the removed service directories |

Both loopback rebind checks used short-lived task-owned sockets and closed them immediately. The final inspection does not assert that every host process is absent; it checks the task's exact containers and process namespace. Native case reports separately establish owned-child wait/display/runtime cleanup. Empty private runtime lock/cache directories may remain without credentials. Overall cleanup status is **pass**.
