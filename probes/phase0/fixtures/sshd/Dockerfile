FROM --platform=linux/amd64 debian@sha256:7c7b2c966bc9ee8cedfeef67e0e279108992c77681fa595db4a9d65c06ccc587
RUN apt-get update && apt-get install -y --no-install-recommends openssh-server python3 ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 -s /bin/bash probe \
    && useradd -m -u 1001 -s /bin/bash probe_password \
    && useradd -m -u 1002 -s /bin/bash probe_nosftp \
    && useradd -m -u 1003 -s /bin/bash probe_chroot \
    && mkdir -p /home/probe/probe-data /denied /chroot/upload \
    && chown probe:probe /home/probe/probe-data \
    && chown probe_chroot:probe_chroot /chroot/upload \
    && chmod 0555 /denied \
    && rm -f /etc/ssh/ssh_host_*
COPY entrypoint.sh sftp-gate.sh /usr/local/bin/
RUN chmod 0755 /usr/local/bin/entrypoint.sh /usr/local/bin/sftp-gate.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
