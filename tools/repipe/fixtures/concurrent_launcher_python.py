#!/usr/bin/env python3
"""Minimal KUNA_PY stand-in for run.sh lifetime-lock smoke tests."""
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
    raise SystemExit(0)

if "--tick" in args:
    (state_dir / "tick-waiting").touch()
    deadline = time.time() + 10
    while not (state_dir / "release-supervisor").exists() and time.time() < deadline:
        time.sleep(0.01)
    if not (state_dir / "release-supervisor").exists():
        raise SystemExit(2)
    (state_dir / "fake-state").write_text("STOPPED\n")
    raise SystemExit(0)

if "--status" in args:
    state = (state_dir / "fake-state").read_text().strip()
    print(json.dumps({"states": {"supervisor": state}, "round": 1}))
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
