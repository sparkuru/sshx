#!/bin/sh
set -eu
main() {
	if [ "$(id -un)" = probe_nosftp ]; then
		printf 'SFTP unavailable in this controlled fixture\n' >&2
		return 127
	fi
	exec /usr/lib/openssh/sftp-server
}
main "$@"
