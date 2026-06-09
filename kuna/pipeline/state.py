"""Shared, file-backed pipeline state: the worker inventory + opportunity claims.

Workers run as independent processes (headless `claude -p` sessions in worktrees) and the
driver loop is a separate process again, so live state lives in flock-guarded JSON under
``.kuna-pipeline/`` (gitignored), not in memory. This is the single place ``status.py``
reads to answer "how many workers are running and what is each doing", and the place the
driver reads to avoid two workers claiming the same angr testcase.

CLI (called by the driver shell and by workers to heartbeat):
    python -m kuna.pipeline.state register --worker ID --slug S --branch B --opportunity O
    python -m kuna.pipeline.state update   --worker ID --phase analyze [--status running] [--pr URL]
    python -m kuna.pipeline.state claim     --worker ID --opportunity O      # exit 0 claimed / 1 taken
    python -m kuna.pipeline.state done      --worker ID --opportunity O --pr URL
    python -m kuna.pipeline.state remove    --worker ID
    python -m kuna.pipeline.state list [--json]
"""
from __future__ import annotations

import argparse
import contextlib
import fcntl
import json
import os
import sys
import time

from . import config

# Ordered worker phases (for display + progress sense).
PHASES = ["queued", "setup", "analyze", "design", "code", "build", "test", "docs",
          "commit", "pr", "done", "failed"]


def _state_dir():
    d = config.state_dir()
    os.makedirs(d, exist_ok=True)
    return d


def _inventory_path():
    return os.path.join(_state_dir(), "inventory.json")


def _lock_path():
    return os.path.join(_state_dir(), ".lock")


@contextlib.contextmanager
def _locked():
    """Exclusive lock around a read-modify-write of the inventory."""
    lp = _lock_path()
    fh = open(lp, "w")
    try:
        fcntl.flock(fh, fcntl.LOCK_EX)
        yield
    finally:
        fcntl.flock(fh, fcntl.LOCK_UN)
        fh.close()


def _load():
    p = _inventory_path()
    if not os.path.exists(p):
        return {"workers": {}, "claims": {}, "done": {}}
    try:
        with open(p) as fh:
            data = json.load(fh)
    except (json.JSONDecodeError, OSError):
        return {"workers": {}, "claims": {}, "done": {}}
    data.setdefault("workers", {})
    data.setdefault("claims", {})
    data.setdefault("done", {})
    return data


def _save(data):
    p = _inventory_path()
    tmp = p + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(data, fh, indent=2)
    os.replace(tmp, p)


# --- worker inventory -------------------------------------------------------

def register(worker, slug, branch, opportunity, pid=None):
    now = time.time()
    with _locked():
        data = _load()
        data["workers"][worker] = {
            "worker": worker, "slug": slug, "branch": branch,
            "opportunity": opportunity, "phase": "setup", "status": "running",
            "pr_url": None, "pid": pid or os.getpid(),
            "started_at": now, "updated_at": now,
        }
        _save(data)


def update(worker, *, phase=None, status=None, pr_url=None, note=None):
    with _locked():
        data = _load()
        w = data["workers"].get(worker)
        if w is None:
            w = {"worker": worker, "started_at": time.time()}
            data["workers"][worker] = w
        if phase is not None:
            w["phase"] = phase
        if status is not None:
            w["status"] = status
        if pr_url is not None:
            w["pr_url"] = pr_url
        if note is not None:
            w["note"] = note
        w["updated_at"] = time.time()
        _save(data)


def remove(worker):
    with _locked():
        data = _load()
        data["workers"].pop(worker, None)
        _save(data)


# --- opportunity claims (dedup across concurrent workers) -------------------

def claim(worker, opportunity):
    """Atomically claim an opportunity. Returns True if claimed, False if already taken."""
    now = time.time()
    with _locked():
        data = _load()
        if opportunity in data["done"]:
            return False
        held = data["claims"].get(opportunity)
        if held and held.get("worker") != worker:
            return False
        data["claims"][opportunity] = {"worker": worker, "ts": now}
        _save(data)
        return True


def release(opportunity):
    with _locked():
        data = _load()
        data["claims"].pop(opportunity, None)
        _save(data)


def mark_done(worker, opportunity, pr_url=None):
    now = time.time()
    with _locked():
        data = _load()
        data["done"][opportunity] = {"worker": worker, "pr_url": pr_url, "ts": now}
        data["claims"].pop(opportunity, None)
        w = data["workers"].get(worker)
        if w:
            w["phase"] = "done"
            w["status"] = "done"
            w["pr_url"] = pr_url
            w["updated_at"] = now
        _save(data)


def snapshot():
    with _locked():
        return _load()


def claimed_or_done():
    data = snapshot()
    return set(data["claims"].keys()) | set(data["done"].keys())


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.state")
    sub = p.add_subparsers(dest="cmd", required=True)

    sp = sub.add_parser("register")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--slug", required=True)
    sp.add_argument("--branch", required=True)
    sp.add_argument("--opportunity", required=True)

    sp = sub.add_parser("update")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--phase", default=None)
    sp.add_argument("--status", default=None)
    sp.add_argument("--pr", default=None)
    sp.add_argument("--note", default=None)

    sp = sub.add_parser("claim")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--opportunity", required=True)

    sp = sub.add_parser("done")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--opportunity", required=True)
    sp.add_argument("--pr", default=None)

    sp = sub.add_parser("remove")
    sp.add_argument("--worker", required=True)

    sp = sub.add_parser("list")
    sp.add_argument("--json", action="store_true")

    args = p.parse_args(argv)

    if args.cmd == "register":
        register(args.worker, args.slug, args.branch, args.opportunity)
    elif args.cmd == "update":
        update(args.worker, phase=args.phase, status=args.status,
               pr_url=args.pr, note=args.note)
    elif args.cmd == "claim":
        ok = claim(args.worker, args.opportunity)
        return 0 if ok else 1
    elif args.cmd == "done":
        mark_done(args.worker, args.opportunity, args.pr)
    elif args.cmd == "remove":
        remove(args.worker)
    elif args.cmd == "list":
        data = snapshot()
        if args.json:
            print(json.dumps(data, indent=2))
        else:
            for w in data["workers"].values():
                print("%-16s %-10s %-8s %s" % (
                    w.get("worker"), w.get("phase"), w.get("status"), w.get("slug")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
