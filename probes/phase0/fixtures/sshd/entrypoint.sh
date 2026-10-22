#!/bin/sh
set -eu
main() {
	mkdir -p /run/sshd /run/keys
	cp /fixture/host_key /run/keys/host_key
	chmod 0600 /run/keys/host_key
	for user in probe probe_password probe_nosftp probe_chroot; do
		printf '%s:' "$user"
		cat /fixture/password
	done | chpasswd
	cp /fixture/client_key.pub /run/keys/authorized_keys
	chmod 0644 /run/keys/authorized_keys
	cat >/run/sshd_config <<EOF
Port ${SSH_CONTAINER_PORT:-2222}
ListenAddress 0.0.0.0
HostKey /run/keys/host_key
PidFile /run/sshd.pid
AuthorizedKeysFile /run/keys/authorized_keys
StrictModes yes
PermitRootLogin no
PasswordAuthentication yes
KbdInteractiveAuthentication no
UsePAM no
PrintMotd no
PrintLastLog no
LogLevel ERROR
AllowUsers probe probe_password probe_nosftp probe_chroot
AllowTcpForwarding local
X11Forwarding no
PermitTunnel no
AcceptEnv LANG LC_*
Subsystem sftp /usr/local/bin/sftp-gate.sh
Match User probe_password
    PubkeyAuthentication no
Match User probe_chroot
    ChrootDirectory /chroot
    ForceCommand internal-sftp -d /upload
EOF
	/usr/sbin/sshd -t -f /run/sshd_config
	exec /usr/sbin/sshd -D -e -f /run/sshd_config
}
main "$@"
