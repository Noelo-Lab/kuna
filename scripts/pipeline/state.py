"""Shared, file-backed pipeline state: the worker inventory + opportunity claims.

Workers run as independent processes (headless `claude -p` sessions in worktrees) and the
driver loop is a separate process again, so live state lives in flock-guarded JSON under
``.kuna-pipeline/`` (gitignored), not in memory. This is the single place ``status.py``
reads to answer "how many workers are running and what is each doing", and the place the
driver reads to avoid two workers claiming the same angr testcase.

CLI (called by the driver shell and by workers to heartbeat):
    python -m scripts.pipeline.state register --worker ID --slug S --branch B --opportunity O
    python -m scripts.pipeline.state update   --worker ID --phase analyze [--status running] [--pr URL]
    python -m scripts.pipeline.state claim     --worker ID --opportunity O      # exit 0 claimed / 1 taken
    python -m scripts.pipeline.state done      --worker ID --opportunity O --pr URL
    python -m scripts.pipeline.state remove    --worker ID
    python -m scripts.pipeline.state list [--json] [--proposals]

Large-feature proposal flow (requirement #5: confirm big features before implementing):
    python -m scripts.pipeline.state proposal       --worker ID --opportunity O --pr URL --branch B
    python -m scripts.pipeline.state approve        --opportunity O [--by NAME]   # user said go
    python -m scripts.pipeline.state claim-approved --worker ID --opportunity O   # exit 0 (prints branch) / 1
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
# Feature workers use: analyze, design, code, build, test, docs, commit, pr.
# "proposal" = a large feature paused on a draft PR awaiting the user's go/no-go.
PHASES = ["queued", "setup", "analyze", "design", "code", "build",
          "test", "docs", "commit", "pr", "proposal", "done", "failed"]


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


def _empty():
    # proposals: opp_id -> {worker, slug, branch, pr_url, ts} (large feature awaiting go/no-go)
    # approved:  opp_id -> {branch, pr_url, approved_by, ts}   (user said go; ready to implement)
    return {"workers": {}, "claims": {}, "done": {}, "proposals": {}, "approved": {}}


def _load():
    p = _inventory_path()
    if not os.path.exists(p):
        return _empty()
    try:
        with open(p) as fh:
            data = json.load(fh)
    except (json.JSONDecodeError, OSError):
        return _empty()
    for k, v in _empty().items():
        data.setdefault(k, v)
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


# --- large-feature proposal flow (requirement #5) ---------------------------

def mark_proposal(worker, opportunity, pr_url, branch, slug=None):
    """A large feature opened a draft PR and stopped. Park it for human go/no-go.

    The opportunity leaves ``claims`` and enters ``proposals`` so it is neither re-picked as a
    fresh gap nor counted as done. The worker is left as phase/status ``proposal``.
    """
    now = time.time()
    with _locked():
        data = _load()
        data["claims"].pop(opportunity, None)
        data["proposals"][opportunity] = {
            "worker": worker, "slug": slug, "branch": branch,
            "pr_url": pr_url, "ts": now,
        }
        w = data["workers"].get(worker)
        if w:
            w["phase"] = "proposal"
            w["status"] = "proposal"
            w["pr_url"] = pr_url
            w["updated_at"] = now
        _save(data)


def approve_proposal(opportunity, *, approved_by="user"):
    """User approved a parked proposal: move it proposals -> approved (ready to implement)."""
    now = time.time()
    with _locked():
        data = _load()
        prop = data["proposals"].pop(opportunity, None)
        if prop is None:
            return False
        data["approved"][opportunity] = {
            "branch": prop.get("branch"), "pr_url": prop.get("pr_url"),
            "slug": prop.get("slug"), "approved_by": approved_by, "ts": now,
        }
        _save(data)
        return True


def claim_approved(worker, opportunity):
    """Atomically claim an approved proposal for an implementation worker.

    Returns the proposal's branch (to resume) on success, or None if it isn't approved or is
    already claimed by another worker.
    """
    now = time.time()
    with _locked():
        data = _load()
        appr = data["approved"].get(opportunity)
        if appr is None:
            return None
        held = data["claims"].get(opportunity)
        if held and held.get("worker") != worker:
            return None
        data["claims"][opportunity] = {"worker": worker, "ts": now}
        _save(data)
        return appr.get("branch")


def snapshot():
    with _locked():
        return _load()


def claimed_or_done():
    data = snapshot()
    return set(data["claims"].keys()) | set(data["done"].keys())


def taken():
    """Everything a fresh-gap selector must skip: claimed, done, proposed, or approved.

    Proposed/approved opportunities are driven by the orchestrator (human-in-the-loop), never
    auto-picked as a fresh gap, so they belong in the skip set.
    """
    data = snapshot()
    return (set(data["claims"]) | set(data["done"])
            | set(data["proposals"]) | set(data["approved"]))


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.pipeline.state")
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

    sp = sub.add_parser("proposal")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--opportunity", required=True)
    sp.add_argument("--pr", required=True)
    sp.add_argument("--branch", required=True)
    sp.add_argument("--slug", default=None)

    sp = sub.add_parser("approve")
    sp.add_argument("--opportunity", required=True)
    sp.add_argument("--by", default="user")

    sp = sub.add_parser("claim-approved")
    sp.add_argument("--worker", required=True)
    sp.add_argument("--opportunity", required=True)

    sp = sub.add_parser("list")
    sp.add_argument("--json", action="store_true")
    sp.add_argument("--proposals", action="store_true",
                    help="show only parked proposals + approved (ready-to-implement)")

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
    elif args.cmd == "proposal":
        mark_proposal(args.worker, args.opportunity, args.pr, args.branch, slug=args.slug)
    elif args.cmd == "approve":
        ok = approve_proposal(args.opportunity, approved_by=args.by)
        if not ok:
            print("no parked proposal for %r" % args.opportunity, file=sys.stderr)
        return 0 if ok else 1
    elif args.cmd == "claim-approved":
        branch = claim_approved(args.worker, args.opportunity)
        if branch is None:
            return 1
        print(branch)
        return 0
    elif args.cmd == "list":
        data = snapshot()
        if args.json:
            print(json.dumps(data, indent=2))
        elif args.proposals:
            for oid, p_ in data["proposals"].items():
                print("proposal  %-40s [%s]  %s" % (oid[:40], p_.get("branch"), p_.get("pr_url")))
            for oid, a_ in data["approved"].items():
                print("approved  %-40s [%s]  %s" % (oid[:40], a_.get("branch"), a_.get("pr_url")))
        else:
            for w in data["workers"].values():
                print("%-16s %-10s %-8s %s" % (
                    w.get("worker"), w.get("phase"), w.get("status"), w.get("slug")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
