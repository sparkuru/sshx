# Two independent loopback SSH server fixtures

Observed 2026-10-22. A second generated-credential server was started through the **existing** reviewed lifecycle functions, with a separate `.devhome/phase0/secondary` runtime. The primary current pointer/container was preserved. No application, wrapper or lifecycle source changed for this follow-up. [Machine evidence](dual-target-fixture.json) records both exact identities and checks.

| Property | Primary | Secondary |
| --- | --- | --- |
| Loopback endpoint | `127.0.0.1:32770` | `127.0.0.1:32771` |
| Run ID | `09422203e8e5490890c2ddf05c2ff249` | `8a26f072521876cee2e63876fc9bd093` |
| Container ID | `1d11fd8f596079ec7873f3b488974d53630e37560494bb1935960b6755327be3` | `b0ea52e13e90b7a7272b5d9ed8056bf0b0f96ebe1be4f43c1022adb83e014939` |
| Expected generated ED25519 fingerprint | `SHA256:kSusXBuqKc9/pTYSjk7GZC/Net/Md/O0MSPG5H85pVY` | `SHA256:ptnZAsRZZ+qt4Q2i1CptzY9u1K6Wtgm5kcWJMB0Bw1g` |

Both use the exercised image `sha256:cb3eb061776d0835b474754593e692492c473a811607dbab68a815b02cedc70c`, distinct generated client/host keys, isolated config/known_hosts/password files and independent server filesystems. Actual Docker publication for each was exactly one IPv4 loopback mapping. Both containers were running; secondary strict-host-key authenticated readiness passed. Both run/server directories were mode 0700 and credential/config/known_hosts files mode 0600. No secret body was printed or copied into evidence, and the build mounts still exclude both runtime trees.

The secondary state is:

~~~text
/home/wkyuu/cargo/repo/29-sshx/.devhome/phase0/secondary/8a26f072521876cee2e63876fc9bd093/state.json
~~~

The isolated `python3 -I -S` driver used `importlib.util.spec_from_file_location` with the explicit repository `probes/phase0/fixtures/lifecycle.py` path, applied umask 077, called `private_directory(secondary)`, asserted no existing secondary pointer, then `start(repo, secondary)`. It did **not** call normal `dev.sh start`, which would replace the primary fixture. Building reused the cached image. The primary pointer was read before and compared after; exact running state and actual port bindings were independently inspected for both containers.

These are two distinct SSH server identities/endpoints on the same Docker host. They provide separate targets/namespaces for per-Session routing tests in **one Core**; this infrastructure result alone does not prove those integrated operations or two physical machines. The core worker received the secondary state path immediately after readiness and owns that integration evidence.

Both fixtures remain intentionally running for integration. Main-session final teardown should load the same reviewed local module explicitly under `python3 -I -S`, then use the separate runtime:

~~~python
from pathlib import Path
import importlib.util

repo = Path.cwd().resolve()
spec = importlib.util.spec_from_file_location(
    "sshx_owned_fixture_lifecycle", repo / "probes/phase0/fixtures/lifecycle.py"
)
assert spec is not None and spec.loader is not None
lifecycle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lifecycle)
secondary = repo / ".devhome/phase0/secondary"
lifecycle.down(repo, secondary)
secondary.rmdir()  # Only succeeds after that exact owned runtime is empty.
~~~

Then `./dev.sh down` tears down the primary through its own pointer. Each `down` verifies exact repo/task/run/scope labels and recorded container ID before removal, then removes only that run's generated credentials. Main should verify both containers absent, both run directories/current pointers removed and both recorded loopback ports released. This report records pending final cleanup, not a clean final state. No service will be stopped by this worker while integration runs.

## Main-session final cleanup

The live-fixture/retained-integration wording above records the earlier implementation checkpoint. Subsequent main-session [final cleanup](cleanup.md) passed: both exact generated services, credential directories and pointers are removed, both loopback ports rebound successfully, and no task-owned container/process remains. Build caches/images and explicitly documented synthetic review artifacts are intentionally retained. No further fixture use is pending.
