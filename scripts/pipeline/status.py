"""Observe the running pipeline: how many workers, what each is doing, which PR.

Aggregates three sources into one live view:
  - the flock-guarded worker inventory (scripts.pipeline.state);
  - `git worktree list` (the actual isolated workspaces on disk);
  - open PRs via `gh pr list` (best-effort; skipped if gh is absent).

    python -m scripts.pipeline.status            # one-shot table + "N workers active"
    python -m scripts.pipeline.status --watch    # refresh every 2s
    python -m scripts.pipeline.status --json      # machine-readable
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time

from . import config, state


def _git_worktrees():
    try:
        out = subprocess.run(
            ["git", "-C", str(config.repo_root()), "worktree", "list", "--porcelain"],
            capture_output=True, text=True, timeout=20,
        ).stdout
    except Exception:
        return []
    trees = []
    cur = {}
    for line in out.splitlines():
        if line.startswith("worktree "):
            if cur:
                trees.append(cur)
            cur = {"path": line.split(" ", 1)[1]}
        elif line.startswith("branch "):
            cur["branch"] = line.split(" ", 1)[1].replace("refs/heads/", "")
    if cur:
        trees.append(cur)
    # only the pipeline's worktrees
    return [t for t in trees if ".kuna-pipeline" in t["path"] or "/kuna-wt-" in t["path"]
            or (t.get("branch", "").startswith("feat/angr-"))]


def _open_prs():
    # Use the REST API (gh api), not `gh pr list` (GraphQL): GitHub's classic-Projects
    # sunset makes gh's GraphQL PR commands fail/misreport on this repo.
    try:
        out = subprocess.run(
            ["gh", "api", "repos/Noelo-Lab/kuna/pulls?state=open&per_page=50",
             "--jq", "[.[] | {number, title, headRefName: .head.ref, url: .html_url}]"],
            capture_output=True, text=True, timeout=30,
        )
        if out.returncode != 0:
            return None
        return json.loads(out.stdout or "[]")
    except Exception:
        return None


def collect():
    data = state.snapshot()
    workers = list(data["workers"].values())
    now = time.time()
    for w in workers:
        w["elapsed_s"] = int(now - w.get("started_at", now))
        w["stale_s"] = int(now - w.get("updated_at", now))
    workers.sort(key=lambda w: w.get("started_at", 0))
    active = [w for w in workers if w.get("status") == "running"]
    return {
        "workers": workers,
        "active": len(active),
        "claims": data["claims"],
        "done": data["done"],
        "proposals": data.get("proposals", {}),
        "approved": data.get("approved", {}),
        "worktrees": _git_worktrees(),
        "prs": _open_prs(),
        "ts": now,
    }


def _fmt_elapsed(s):
    s = int(s)
    if s < 60:
        return "%ds" % s
    if s < 3600:
        return "%dm%02ds" % (s // 60, s % 60)
    return "%dh%02dm" % (s // 3600, (s % 3600) // 60)


def render(snap):
    lines = []
    lines.append("kuna pipeline — %d worker(s) active, %d done, %d claimed"
                 % (snap["active"], len(snap["done"]), len(snap["claims"])))
    lines.append("-" * 92)
    if not snap["workers"]:
        lines.append("  (no workers registered)")
    else:
        lines.append("  %-14s %-9s %-8s %-8s %-7s %-22s %s"
                     % ("WORKER", "PHASE", "STATUS", "ELAPSED", "STALE", "SLUG", "PR"))
        for w in snap["workers"]:
            stale = w["stale_s"]
            stale_mark = ("%ds" % stale) if stale < 120 else ("%ds!" % stale)
            lines.append("  %-14s %-9s %-8s %-8s %-7s %-22s %s" % (
                (w.get("worker") or "?")[:14],
                (w.get("phase") or "?")[:9],
                (w.get("status") or "?")[:8],
                _fmt_elapsed(w["elapsed_s"]),
                stale_mark,
                (w.get("slug") or "?")[:22],
                w.get("pr_url") or "-",
            ))
    wts = snap["worktrees"]
    lines.append("-" * 92)
    lines.append("  worktrees: %d" % len(wts))
    for t in wts:
        lines.append("    %s  [%s]" % (t["path"], t.get("branch", "?")))
    props, appr = snap.get("proposals", {}), snap.get("approved", {})
    if props or appr:
        lines.append("-" * 92)
        if props:
            lines.append("  proposals awaiting confirmation: %d" % len(props))
            for oid, p_ in props.items():
                lines.append("    %-38s [%s]  %s" % (oid[:38], p_.get("branch", "?"), p_.get("pr_url") or "-"))
        if appr:
            lines.append("  approved (ready to implement): %d" % len(appr))
            for oid, a_ in appr.items():
                lines.append("    %-38s [%s]  %s" % (oid[:38], a_.get("branch", "?"), a_.get("pr_url") or "-"))
    if snap["prs"] is not None:
        lines.append("  open PRs: %d" % len(snap["prs"]))
        for pr in snap["prs"]:
            lines.append("    #%s %s  (%s)" % (pr["number"], pr["title"][:50], pr["headRefName"]))
    else:
        lines.append("  open PRs: (gh unavailable)")
    return "\n".join(lines)


def render_proposals(snap):
    """Just the human-in-the-loop section (for `--proposals`)."""
    lines = []
    props, appr = snap.get("proposals", {}), snap.get("approved", {})
    lines.append("proposals awaiting confirmation: %d" % len(props))
    for oid, p_ in props.items():
        lines.append("  %-40s [%s]  %s" % (oid[:40], p_.get("branch", "?"), p_.get("pr_url") or "-"))
    lines.append("approved (ready to implement): %d" % len(appr))
    for oid, a_ in appr.items():
        lines.append("  %-40s [%s]  %s" % (oid[:40], a_.get("branch", "?"), a_.get("pr_url") or "-"))
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.pipeline.status")
    p.add_argument("--watch", action="store_true", help="refresh continuously")
    p.add_argument("--interval", type=float, default=2.0)
    p.add_argument("--json", action="store_true")
    p.add_argument("--proposals", action="store_true",
                   help="show only the proposals/approved (human-in-the-loop) section")
    args = p.parse_args(argv)

    if args.json and not args.watch:
        print(json.dumps(collect(), indent=2, default=str))
        return 0

    if args.proposals and not args.watch:
        print(render_proposals(collect()))
        return 0

    if not args.watch:
        print(render(collect()))
        return 0

    try:
        while True:
            snap = collect()
            sys.stdout.write("\x1b[2J\x1b[H")  # clear + home
            sys.stdout.write(render(snap) + "\n")
            sys.stdout.flush()
            time.sleep(args.interval)
    except KeyboardInterrupt:
        return 0


if __name__ == "__main__":
    sys.exit(main())
