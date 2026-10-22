#!/usr/bin/env python3
"""Independent stdlib-only public-protocol and synthetic package fixture.

This trusted test executable receives no credentials, desktop or PTY handles.
Fault modes deliberately violate the wire protocol; normal stdout is framed JSON.
"""

import argparse
import gzip
import io
import json
import os
from pathlib import Path, PurePosixPath
import sys
import tarfile
import tempfile
import time
from typing import Any, BinaryIO


class CLIStyle:
    """Style human help/errors on stderr without touching framed stdout."""

    COLORS = {"TITLE": 36, "CONTENT": 37, "EXAMPLE": 32, "ERROR": 31}

    @staticmethod
    def color(text: str, role: str = "CONTENT") -> str:
        """Respect NO_COLOR and redirected diagnostics."""
        if not sys.stderr.isatty() or "NO_COLOR" in os.environ:
            return text
        return f"\033[1;{CLIStyle.COLORS[role]}m{text}\033[0m"


class ColoredArgumentParser(argparse.ArgumentParser):
    """Provide human help separately from the plugin protocol stream."""

    def format_help(self) -> str:
        """Color help only on an interactive diagnostic terminal."""
        return CLIStyle.color(super().format_help(), "TITLE")

    def print_help(self, file: Any = None) -> None:
        """Keep even explicitly requested help off the protocol stdout."""
        super().print_help(file=sys.stderr if file is None else file)


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    """Reject duplicate keys independently of the Rust framing implementation."""
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate JSON key")
        result[key] = value
    return result


def read_frame() -> dict[str, Any]:
    """Read one bounded Content-Length object without Core-private imports."""
    header = bytearray()
    while not header.endswith(b"\r\n\r\n"):
        byte = sys.stdin.buffer.read(1)
        if not byte:
            raise EOFError("broker EOF")
        header.extend(byte)
        if len(header) > 1024:
            raise ValueError("header limit")
    name, separator, size = header[:-4].partition(b": ")
    if not separator or name.lower() != b"content-length" or not size.isdigit():
        raise ValueError("invalid header")
    length = int(size)
    if not 0 < length <= 262144:
        raise ValueError("body limit")
    body = sys.stdin.buffer.read(length)
    if len(body) != length or body.startswith(b"\xef\xbb\xbf"):
        raise ValueError("invalid body")
    value = json.loads(body.decode("utf-8"), object_pairs_hook=unique_object)
    if not isinstance(value, dict):
        raise ValueError("single object required")
    return value


def frame(value: dict[str, Any], *, split: bool = False) -> None:
    """Write UTF-8 byte lengths, optionally across independent single-byte writes."""
    body = json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    data = f"Content-Length: {len(body)}\r\n\r\n".encode("ascii") + body
    if split:
        for byte in data:
            os.write(sys.stdout.fileno(), bytes((byte,)))
    else:
        sys.stdout.buffer.write(data)
        sys.stdout.buffer.flush()


def request(counter: int, method: str, params: dict[str, Any], *, split: bool = False) -> dict[str, Any]:
    """Use the public RPC channel and verify response correlation."""
    frame({"jsonrpc": "2.0", "id": str(counter), "method": method, "params": params}, split=split)
    response = read_frame()
    if response.get("id") != str(counter):
        raise ValueError("response correlation")
    return response


def malformed(mode: str) -> None:
    """Send a named protocol fault, then wait for the broker to close us."""
    bodies = {
        "duplicate-key": b'{"jsonrpc":"2.0","id":"2","id":"3"}',
        "batch": b"[]",
        "depth": b'{"x":' + b"[" * 34 + b"0" + b"]" * 34 + b"}",
        "trailing-json": b"{} {}",
        "invalid-utf8": b'{"x":"\xff"}',
        "bom": b"\xef\xbb\xbf{}",
        "nonfinite": b'{"x":NaN}',
    }
    raw = {
        "stdout-log": b"unframed synthetic log\n",
        "duplicate-header": b"Content-Length: 2\r\nContent-Length: 2\r\n\r\n{}",
        "bare-lf": b"Content-Length: 2\n\n{}",
        "oversize": b"Content-Length: 262145\r\n\r\n",
        "partial-frame": b"Content-Length: 100\r\n\r\n{",
    }
    if mode in bodies:
        body = bodies[mode]
        data = f"Content-Length: {len(body)}\r\n\r\n".encode("ascii") + body
    else:
        data = raw[mode]
    os.write(sys.stdout.fileno(), data)
    time.sleep(20)


def plugin(mode: str) -> int:
    """Exercise a real independently launched plugin lifecycle and public calls."""
    initialize = read_frame()
    if initialize.get("method") != "plugin.initialize":
        raise ValueError("initialize required")
    if mode == "handshake-timeout":
        time.sleep(20)
        return 0
    version = "99.0" if mode == "wrong-version" else "1.0"
    frame({"jsonrpc": "2.0", "id": initialize["id"], "result": {"api_version": version, "plugin_version": "0.1.0", "features": []}}, split=mode == "unicode-split")
    if mode == "wrong-version":
        time.sleep(20)
        return 0
    ready = read_frame()
    if ready.get("method") != "plugin.ready":
        raise ValueError("ready required")
    if mode == "crash":
        return 42
    if mode in ("stderr-flood", "stderr-long-line"):
        for _ in range(128):
            os.write(sys.stderr.fileno(), (b"x" * 4095 + b"\n") if mode == "stderr-flood" else b"x" * 4096)
    registered = request(1, "action.register", {"actions": [{"id": "wire", "title": "Chinese \u4e2d\u6587 wire fixture"}]}, split=mode == "unicode-split")
    if "error" in registered:
        raise ValueError("registration refused")
    action = read_frame()
    if action.get("method") != "action.run":
        raise ValueError("action required")
    if mode == "response-order":
        stop = read_frame()
        if stop.get("method") != "plugin.stop":
            raise ValueError("stop required")
        frame({"jsonrpc": "2.0", "id": stop["id"], "result": {"stopping": True}})
        frame({"jsonrpc": "2.0", "id": action["id"], "result": {"accepted": True}})
        return 0
    frame({"jsonrpc": "2.0", "id": action["id"], "result": {"accepted": True}})
    if mode == "queue-byte-flood":
        for counter in range(2, 7):
            frame({"jsonrpc": "2.0", "id": str(counter), "method": "ui.notify", "params": {"text": "x" * 240000}})
        time.sleep(20)
        return 0
    operation = action["params"]["operation_id"]
    grants = action["params"]["grant_ids"]
    faults = {"stdout-log", "duplicate-header", "bare-lf", "oversize", "partial-frame", "duplicate-key", "batch", "depth", "trailing-json", "invalid-utf8", "bom", "nonfinite"}
    if mode in faults:
        malformed(mode)
        return 0
    if mode in ("counter-replay", "counter-skipped"):
        frame({"jsonrpc": "2.0", "id": "1" if mode == "counter-replay" else "3", "method": "session.describe", "params": {"operation_id": operation}})
        time.sleep(20)
        return 0
    if mode == "role-upgrade":
        response = request(2, "client.hello", {"api_versions": ["1.0"], "client_version": "spoof"})
        expected = "ROLE_DENIED"
    elif mode == "identity-spoof":
        response = request(2, "session.describe", {"operation_id": operation, "plugin_instance_id": "forged"})
        expected = "INVALID_PARAMS"
    elif mode == "cross-operation":
        response = request(2, "session.describe", {"operation_id": "f" * 32})
        expected = "CAPABILITY_DENIED"
    elif mode == "revoked-grant":
        response = request(2, "clipboard.read_image", {"operation_id": operation, "grant_ids": [grants[1]]})
        expected = "CAPABILITY_DENIED"
    elif mode == "composed-grant":
        response = request(2, "transfer.upload", {"operation_id": operation, "upload_key": "one", "blob_id": "forged", "directory": {"namespace_id": "test", "path": "/denied"}, "basename": "no-file", "grant_ids": [grants[0]]})
        expected = "CAPABILITY_DENIED"
    elif mode == "unknown-method":
        response = request(2, "executor.run", {})
        expected = "METHOD_NOT_FOUND"
    else:
        response = request(2, "session.describe", {"operation_id": operation}, split=mode == "unicode-split")
        expected = "result"
    if expected == "result":
        if "result" not in response:
            raise ValueError("describe refused")
    elif response.get("error", {}).get("data", {}).get("name") != expected:
        raise ValueError("wrong refusal result")
    forbidden = ("SSH_AUTH_SOCK", "DISPLAY", "WAYLAND_DISPLAY", "HOME", "SSH_ASKPASS")
    if any(name in os.environ for name in forbidden):
        raise ValueError("private environment inherited")
    counter = 3
    if mode == "events":
        subscribed = request(counter, "event.subscribe", {"operation_id": operation, "kinds": ["operation.progress", "operation.result"]})
        if "result" not in subscribed:
            raise ValueError("subscribe refused")
        time.sleep(0.3)
        saw_gap = False
        last = -1
        for _ in range(129):
            event = read_frame()
            if event.get("method") != "event":
                raise ValueError("event expected")
            params = event["params"]
            if params["sequence"] <= last:
                raise ValueError("event ordering")
            last = params["sequence"]
            saw_gap |= params["kind"] == "event.gap"
            if params["kind"] == "operation.result":
                break
        if not saw_gap:
            raise ValueError("event overflow lacked gap")
        counter += 1
        if request(counter, "operation.get", {"operation_id": operation}).get("result", {}).get("phase") != "retained":
            raise ValueError("event requery failed")
        counter += 1
    result = request(counter, "ui.notify", {"operation_id": operation, "level": "info", "text": "synthetic-wire-checked"})
    if "result" not in result:
        raise ValueError("notify refused")
    stop = read_frame()
    if stop.get("method") != "plugin.stop":
        raise ValueError("stop required")
    if mode == "blocked-rpc":
        time.sleep(20)
        return 0
    frame({"jsonrpc": "2.0", "id": stop["id"], "result": {"stopping": True}})
    return 0


def expand_package(source: Path, destination: BinaryIO) -> int:
    """Bound decompression before any archive metadata parser sees the bytes."""
    length = 0
    with gzip.open(source, "rb") as stream:
        while data := stream.read(65536):
            length += len(data)
            if length > 32 * 1024 * 1024:
                raise ValueError("PACKAGE_MAX")
            destination.write(data)
    destination.seek(0)
    return length


def package_gate(source: Path) -> dict[str, Any]:
    """Validate a deliberately small USTAR+gzip profile without extracting it.

    Reject GNU/PAX metadata rather than letting a general tar parser allocate
    attacker-declared metadata. This is a probe profile, not a product installer.
    """
    if source.stat().st_size > 32 * 1024 * 1024:
        raise ValueError("PACKAGE_MAX")
    names: set[str] = set()
    manifest: dict[str, Any] | None = None
    executables: set[str] = set()
    total = 0
    with tempfile.TemporaryFile() as expanded:
        length = expand_package(source, expanded)
        while True:
            header = expanded.read(512)
            if len(header) != 512:
                raise ValueError("truncated tar")
            if not any(header):
                if expanded.read(512) != bytes(512):
                    raise ValueError("missing tar end marker")
                while padding := expanded.read(65536):
                    if any(padding):
                        raise ValueError("trailing tar content")
                break
            expected = int(header[148:156].rstrip(b"\0 ") or b"0", 8)
            if sum(header[:148] + b" " * 8 + header[156:]) != expected:
                raise ValueError("tar checksum")
            if header[257:263] != b"ustar\0" or header[263:265] != b"00":
                raise ValueError("unsupported tar profile")
            name = header[:100].split(b"\0", 1)[0].decode("utf-8")
            prefix = header[345:500].split(b"\0", 1)[0].decode("utf-8")
            if prefix:
                name = prefix + "/" + name
            path = PurePosixPath(name)
            if not name or path.is_absolute() or ".." in path.parts or "\\" in name or str(path) != name.rstrip("/") or str(path) in names:
                raise ValueError("package path")
            names.add(str(path))
            if len(names) > 256:
                raise ValueError("PACKAGE_FILES_MAX")
            kind = header[156:157]
            if kind not in (b"0", b"\0", b"5"):
                raise ValueError("package link/device/extended metadata")
            size = int(header[124:136].rstrip(b"\0 ") or b"0", 8)
            total += size
            if size < 0 or total > 32 * 1024 * 1024 or expanded.tell() + ((size + 511) // 512) * 512 > length:
                raise ValueError("package size")
            mode = int(header[100:108].rstrip(b"\0 ") or b"0", 8)
            if mode & 0o111 and kind in (b"0", b"\0"):
                executables.add(name)
            if name == "manifest.json":
                if size > 65536:
                    raise ValueError("MANIFEST_MAX")
                manifest = json.loads(expanded.read(size), object_pairs_hook=unique_object)
                expanded.seek((-size) % 512, 1)
            else:
                expanded.seek(((size + 511) // 512) * 512, 1)
    if not isinstance(manifest, dict) or set(manifest) != {"id", "version", "api", "entrypoint", "args", "platforms", "config_schema", "actions", "capabilities"}:
        raise ValueError("manifest schema/install hooks")
    if manifest["api"] != {"min": "1.0", "max": "1.0"} or manifest["platforms"] != ["linux-amd64"]:
        raise ValueError("API/platform mismatch")
    entry = manifest["entrypoint"]
    if not isinstance(entry, str) or entry not in executables or PurePosixPath(entry).is_absolute() or ".." in PurePosixPath(entry).parts:
        raise ValueError("entrypoint")
    if not isinstance(manifest["args"], list) or not all(isinstance(arg, str) for arg in manifest["args"]):
        raise ValueError("manifest args")
    return {"files": len(names), "expanded_bytes": length, "max_decompression_chunk": 65536, "extracted_files": 0}


def package_cases() -> int:
    """Produce standard-library tar archives as an independent format oracle."""
    outcomes: list[dict[str, Any]] = []
    with tempfile.TemporaryDirectory(prefix="sshx-package-") as temporary:
        root = Path(temporary)
        base = {"id": "wire", "version": "0.1.0", "api": {"min": "1.0", "max": "1.0"}, "entrypoint": "plugin", "args": [], "platforms": ["linux-amd64"], "config_schema": {}, "actions": [], "capabilities": []}
        for fault in ("valid", "traversal", "absolute", "duplicate", "symlink", "hardlink", "device", "entrypoint", "api", "platform", "install-hook", "manifest-limit", "file-count", "decompression-limit"):
            path = root / (fault + ".tar.gz")
            if fault == "decompression-limit":
                with gzip.open(path, "wb") as compressed:
                    for _ in range(513):
                        compressed.write(bytes(65536))
            else:
                manifest = dict(base)
                if fault == "entrypoint":
                    manifest["entrypoint"] = "../escaped"
                elif fault == "api":
                    manifest["api"] = {"min": "99.0", "max": "99.0"}
                elif fault == "platform":
                    manifest["platforms"] = ["windows-amd64"]
                elif fault == "install-hook":
                    manifest["install"] = "touch hook-ran"
                data = json.dumps(manifest).encode("utf-8")
                if fault == "manifest-limit":
                    data += b" " * 65536
                with tarfile.open(path, "w:gz", format=tarfile.USTAR_FORMAT) as archive:
                    item = tarfile.TarInfo("manifest.json")
                    item.size = len(data)
                    archive.addfile(item, io.BytesIO(data))
                    count = 257 if fault == "file-count" else 1
                    for index in range(count):
                        name = f"plugin{index}" if count > 1 else "plugin"
                        if fault == "traversal":
                            name = "../escaped"
                        elif fault == "absolute":
                            name = "/escaped"
                        elif fault == "duplicate":
                            name = "manifest.json"
                        item = tarfile.TarInfo(name)
                        item.mode = 0o700
                        item.type = {"symlink": tarfile.SYMTYPE, "hardlink": tarfile.LNKTYPE, "device": tarfile.CHRTYPE}.get(fault, tarfile.REGTYPE)
                        item.linkname = "../escaped" if fault in ("symlink", "hardlink") else ""
                        item.size = 0
                        archive.addfile(item)
            try:
                details = package_gate(path)
                accepted = True
                reason = None
            except (ValueError, OSError, UnicodeError) as error:
                accepted = False
                details = {}
                reason = str(error)
            outcomes.append({"case": fault, "status": "pass" if accepted == (fault == "valid") else "fail", "accepted": accepted, "reason": reason, **details})
        if (root / "hook-ran").exists():
            raise ValueError("package validation caused an external effect")
    frame({"jsonrpc": "2.0", "id": "1", "result": {"cases": outcomes, "cleanup_status": "temporary archive directory removed", "profile": "bounded USTAR+gzip; no extraction/install execution"}})
    return 0 if all(item["status"] == "pass" for item in outcomes) else 1


def main() -> int:
    """Select one explicit test profile; no implicit installation or service."""
    parser = ColoredArgumentParser(description="Independent synthetic public-protocol fixture.", epilog=CLIStyle.color("Example: python3 -I -S plugin.py package-cases", "EXAMPLE"))
    parser.add_argument("mode", metavar=CLIStyle.color("MODE"), help=CLIStyle.color("Explicit wire fault profile, or package-cases"))
    args = parser.parse_args()
    if args.mode == "package-cases":
        return package_cases()
    return plugin(args.mode)


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, EOFError, KeyError) as error:
        sys.stderr.write(CLIStyle.color(f"Synthetic fixture error: {error}", "ERROR") + "\n")
        sys.exit(1)
