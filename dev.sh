#!/usr/bin/env bash
# dev.sh - Manage only the generated Phase 0 SSH/SFTP fixture through hako.
# SSH_HOST_PORT=0 requests a free loopback port; SSH_CONTAINER_PORT defaults 2222.
# start prints the private state path; status prints redacted fixture metadata.
# down verifies exact container/run labels, removes that container and credentials.
set -Eeuo pipefail
REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
readonly REPO_ROOT
main() {
	command -v python3 >/dev/null 2>&1 || {
		printf 'Error: python3 is required\n' >&2
		return 127
	}
	exec python3 "$REPO_ROOT/probes/phase0/fixtures/lifecycle.py" "$@"
}
main "$@"
