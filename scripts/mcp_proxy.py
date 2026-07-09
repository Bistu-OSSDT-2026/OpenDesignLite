"""Transparent stdio proxy: Cursor <-> odl mcp, with traffic logging."""
from __future__ import annotations

import os
import subprocess
import sys
import threading
from datetime import datetime
from pathlib import Path

ODL = Path(r"D:\projects2\opendesign\target\debug\odl.exe")
LOG = Path(os.environ.get("TEMP", ".")) / "odl-mcp-proxy.log"


def log(msg: str) -> None:
    with LOG.open("a", encoding="utf-8", errors="replace") as f:
        f.write(f"{datetime.now().isoformat()} {msg}\n")
        f.flush()


def pump(src, dst, label: str) -> None:
    try:
        while True:
            chunk = os.read(src.fileno(), 4096)
            if not chunk:
                log(f"{label}: EOF")
                break
            log(f"{label}: {chunk!r}")
            os.write(dst.fileno(), chunk)
    except Exception as exc:  # noqa: BLE001
        log(f"{label}: error {exc!r}")


def main() -> int:
    if LOG.exists():
        # keep history across reloads
        log("--- proxy start ---")
    else:
        log("--- proxy start (new log) ---")

    if not ODL.exists():
        log(f"missing {ODL}")
        return 1

    # Detach from console so Cursor doesn't treat child console noise as protocol.
    creationflags = 0
    if sys.platform == "win32":
        creationflags = subprocess.CREATE_NO_WINDOW  # type: ignore[attr-defined]

    child = subprocess.Popen(
        [str(ODL), "mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        bufsize=0,
        creationflags=creationflags,
    )
    assert child.stdin and child.stdout and child.stderr
    log(f"child pid={child.pid}")

    def stderr_to_log() -> None:
        assert child.stderr
        while True:
            chunk = child.stderr.read(4096)
            if not chunk:
                break
            log(f"child-stderr: {chunk!r}")

    threads = [
        threading.Thread(
            target=pump,
            args=(sys.stdin.buffer, child.stdin, "IN"),
            daemon=True,
        ),
        threading.Thread(
            target=pump,
            args=(child.stdout, sys.stdout.buffer, "OUT"),
            daemon=True,
        ),
        threading.Thread(target=stderr_to_log, daemon=True),
    ]
    for t in threads:
        t.start()

    code = child.wait()
    log(f"child exit {code}")
    # Give pumps a moment to flush remaining bytes
    for t in threads[:2]:
        t.join(timeout=0.5)
    return code or 0


if __name__ == "__main__":
    raise SystemExit(main())
