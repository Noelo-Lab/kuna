#!/usr/bin/env python3
"""Minimal KUNA_PY stand-in for the concurrent run.sh restart smoke."""
import fcntl
import json
import os
from pathlib import Path
import sys
import time


state_dir = Path(os.environ["KUNA_PIPELINE_STATE_DIR"])
args = sys.argv[1:]


def locked(name):
    lock = open(state_dir / name, "a+")
    fcntl.flock(lock, fcntl.LOCK_EX)
    return lock


if "--preflight" in args:
    raise SystemExit(0)

if "--restart-stopped" in args:
    with locked("fake-restart.lock"):
        state = (state_dir / "fake-state").read_text().strip()
        if state != "STOPPED":
            raise SystemExit(1)
        if "--consume-stop" in args:
            (state_dir / "STOP").unlink(missing_ok=True)
        (state_dir / "fake-state").write_text("RUNNING\n")
        with open(state_dir / "fake-restarts", "a") as out:
            out.write("restart\n")
        time.sleep(0.1)
    raise SystemExit(0)

if "--status" in args:
    with locked("fake-status.lock"):
        count_path = state_dir / "fake-status-count"
        count = int(count_path.read_text()) if count_path.exists() else 0
        count += 1
        count_path.write_text(str(count))
    if count <= 2:
        deadline = time.time() + 10
        while int(count_path.read_text()) < 2 and time.time() < deadline:
            time.sleep(0.01)
    print(json.dumps({"states": {"supervisor": "STOPPED"}, "round": 1}))
    raise SystemExit(0)

if "-c" in args:
    code = args[-1]
    if '["states"]["supervisor"]' in code:
        print(json.load(sys.stdin)["states"]["supervisor"])
    elif '["round"]' in code:
        print(json.load(sys.stdin)["round"])
    else:
        print(0)
    raise SystemExit(0)

raise SystemExit(0)
