#!/usr/bin/env python3
"""Manage an isolated generated-credential SSH fixture, never personal SSH state."""

import argparse
import fcntl
import json
import os
from pathlib import Path
import secrets
import shutil
import socket
import stat
import subprocess
import sys
import time
from typing import Any


class CLIStyle:
    """Apply semantic colors only to interactive help and errors."""

    COLORS = {"TITLE": 36, "CONTENT": 37, "EXAMPLE": 32, "ERROR": 31}

    @staticmethod
    def color(text: str, role: str = "CONTENT") -> str:
        """Keep protocol and redirected output free of ANSI escapes."""
        if not sys.stderr.isatty() or "NO_COLOR" in os.environ:
            return text
        return f"\033[1;{CLIStyle.COLORS[role]}m{text}\033[0m"


class ColoredArgumentParser(argparse.ArgumentParser):
    """Preserve a styled help pipeline without affecting JSON output."""

    def format_help(self) -> str:
        """Style the command help when the terminal supports it."""
        return CLIStyle.color(super().format_help(), "TITLE")


def command(args: list[str], *, timeout: float = 120.0) -> str:
    """Run explicit argv; subprocess stdout is data and stderr stays diagnostic."""
    result = subprocess.run(args, check=True, stdout=subprocess.PIPE, text=True, timeout=timeout)
    return result.stdout.strip()


def private_directory(path: Path) -> None:
    """Reject linked or foreign-owned runtime directories before reuse."""
    if path.is_symlink():
        raise ValueError("runtime directory must not be a symlink")
    path.mkdir(mode=0o700, parents=False, exist_ok=True)
    metadata = path.stat()
    if metadata.st_uid != os.getuid() or stat.S_IMODE(metadata.st_mode) != 0o700:
        raise ValueError("runtime directory must be owned by the current user with mode 0700")


def write_private(path: Path, value: str) -> None:
    """Create a new owner-only file without following or replacing links."""
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
        stream.write(value)


def read_current(runtime: Path) -> dict[str, Any] | None:
    """Read only the exact lifecycle-owned pointer."""
    path = runtime / "current.json"
    if not path.exists() and not path.is_symlink():
        return None
    if path.is_symlink() or path.stat().st_uid != os.getuid() or path.stat().st_size > 4096:
        raise ValueError("invalid current fixture pointer")
    current = json.loads(path.read_text(encoding="utf-8"))
    run_id = current.get("run_id", "")
    if len(run_id) != 32 or any(character not in "0123456789abcdef" for character in run_id):
        raise ValueError("invalid run ID in fixture pointer")
    if current.get("state_file") != str(runtime / run_id / "state.json"):
        raise ValueError("fixture state path does not match the owned runtime")
    return current


def down(repo: Path, runtime: Path) -> None:
    """Remove only a recorded exact container with verified ownership labels."""
    current = read_current(runtime)
    if current is None:
        return
    run_dir = runtime / current["run_id"]
    if run_dir.is_symlink():
        raise ValueError("refusing linked fixture run directory")
    command([str(repo / "hako"), "--fixture", "remove", current["container_id"], current["run_id"]])
    shutil.rmtree(run_dir)
    (runtime / "current.json").unlink()


def ssh_config(state: dict[str, Any]) -> str:
    """Generate only synthetic trusted configuration with strict host checks."""
    def quoted(value: str) -> str:
        """Encode a path as one OpenSSH configuration token."""
        return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'

    return f"""Host phase0-password
    User probe_password
Host phase0-nosftp
    User probe_nosftp
Host phase0-chroot
    User probe_chroot
Host fixture phase0 phase0-password phase0-nosftp phase0-chroot
    HostName 127.0.0.1
    Port {state['port']}
    User probe
    IdentityFile {quoted(state['identity_file'])}
    IdentityAgent none
    IdentitiesOnly yes
    UserKnownHostsFile {quoted(state['known_hosts_file'])}
    GlobalKnownHostsFile /dev/null
    StrictHostKeyChecking yes
    HostKeyAlgorithms ssh-ed25519
    BatchMode yes
    ConnectTimeout 5
    ConnectionAttempts 1
    ControlMaster no
    LogLevel ERROR
"""


def ready(state: dict[str, Any]) -> None:
    """Wait for strict-key authenticated readiness rather than socket existence."""
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        try:
            with socket.create_connection((state["host"], state["port"]), timeout=0.5):
                result = subprocess.run(
                    ["ssh", "-F", state["ssh_config_file"], "phase0", "true"],
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=3, check=False,
                )
                if result.returncode == 0:
                    return
        except OSError:
            pass
        time.sleep(0.1)
    raise TimeoutError("strict-host-key authenticated fixture readiness timed out")


def start(repo: Path, runtime: Path) -> Path:
    """Build/start one test service, recording cleanup ownership before readiness."""
    down(repo, runtime)
    command([str(repo / "hako"), "--fixture", "build"], timeout=300)
    run_id = secrets.token_hex(16)
    run_dir = runtime / run_id
    run_dir.mkdir(mode=0o700)
    server = run_dir / "server"
    server.mkdir(mode=0o700)
    container_id: str | None = None
    try:
        command(["ssh-keygen", "-q", "-t", "ed25519", "-N", "", "-C", "phase0-client", "-f", str(run_dir / "client_key")])
        command(["ssh-keygen", "-q", "-t", "ed25519", "-N", "", "-C", "phase0-host", "-f", str(server / "host_key")])
        shutil.copyfile(run_dir / "client_key.pub", server / "client_key.pub")
        os.chmod(server / "client_key.pub", 0o600)
        write_private(server / "password", secrets.token_urlsafe(32) + "\n")
        container_id = command([str(repo / "hako"), "--fixture", "start", run_id, str(server)])
        pointer = {"run_id": run_id, "container_id": container_id, "state_file": str(run_dir / "state.json")}
        write_private(runtime / "current.json", json.dumps(pointer) + "\n")
        details = json.loads(command([str(repo / "hako"), "--fixture", "inspect", container_id]))[0]
        mapping = details["NetworkSettings"]["Ports"][os.environ.get("SSH_CONTAINER_PORT", "2222") + "/tcp"]
        if len(mapping) != 1 or mapping[0]["HostIp"] != "127.0.0.1":
            raise ValueError("fixture publication is not exactly IPv4 loopback")
        state = {
            "schema": 1, "run_id": run_id, "host": "127.0.0.1", "port": int(mapping[0]["HostPort"]),
            "user": "probe", "identity_file": str(run_dir / "client_key"),
            "known_hosts_file": str(run_dir / "known_hosts"), "ssh_config_file": str(run_dir / "ssh_config"),
            "password_file": str(server / "password"), "remote_root": "/home/probe/probe-data",
            "container_id": container_id, "no_sftp_user": "probe_nosftp", "password_user": "probe_password",
            "chroot_user": "probe_chroot", "quota_root": "/quota", "denied_root": "/denied",
            "image_id": details["Image"], "host_key_fingerprint": command(["ssh-keygen", "-l", "-f", str(server / "host_key.pub")]),
        }
        public_key = " ".join((server / "host_key.pub").read_text(encoding="ascii").split()[:2])
        write_private(run_dir / "known_hosts", f"[127.0.0.1]:{state['port']} {public_key}\n")
        write_private(run_dir / "ssh_config", ssh_config(state))
        write_private(run_dir / "state.json", json.dumps(state, indent=2) + "\n")
        ready(state)
        return run_dir / "state.json"
    except (OSError, ValueError, subprocess.SubprocessError, KeyError):
        if container_id is not None:
            command([str(repo / "hako"), "--fixture", "remove", container_id, run_id])
        shutil.rmtree(run_dir)
        (runtime / "current.json").unlink(missing_ok=True)
        raise


def main() -> int:
    """Serialize exact fixture start/status/down operations under an owned lock."""
    parser = ColoredArgumentParser(description="Manage the isolated Phase 0 loopback SSH fixture.", epilog="Example: ./dev.sh start; ./dev.sh status; ./dev.sh down")
    parser.add_argument("command", nargs="?", default="start", choices=("start", "status", "down", "build"), help=CLIStyle.color("Fixture lifecycle operation", "CONTENT"))
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[3]
    os.umask(0o077)
    for executable in ("docker", "ssh", "ssh-keygen"):
        if shutil.which(executable) is None:
            raise FileNotFoundError(f"required command not found: {executable}")
    private_directory(repo / ".devhome")
    runtime = repo / ".devhome" / "phase0"
    private_directory(runtime)
    descriptor = os.open(runtime / "lifecycle.lock", os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "r+") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if args.command == "start":
            sys.stdout.write(str(start(repo, runtime)) + "\n")
        elif args.command == "down":
            down(repo, runtime)
        elif args.command == "build":
            command([str(repo / "hako"), "--fixture", "build"], timeout=300)
        else:
            current = read_current(runtime)
            if current is None:
                sys.stdout.write('{"status":"stopped"}\n')
                return 1
            details = json.loads(command([str(repo / "hako"), "--fixture", "inspect", current["container_id"]]))[0]
            state = json.loads(Path(current["state_file"]).read_text(encoding="utf-8"))
            sys.stdout.write(json.dumps({"status": details["State"]["Status"], **state}) + "\n")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError, KeyError) as error:
        sys.stderr.write(CLIStyle.color(f"Error: {error}", "ERROR") + "\n")
        sys.exit(1)
