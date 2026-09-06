"""One-shot / --watch / --json view of the RE-friction loop, for a terminal.

The sibling of scripts/pipeline/status.py, and it reuses that module's collector rather than
re-deriving worker state: the two loops share one flock-guarded inventory, so the agents,
slots and leases come from there and only the round, backlog and budget are new here.

This is what the captain prompt tells a tick to read first, and what an operator runs when
they do not want the web dashboard. It is deliberately cheap -- everything expensive
(`git worktree list`, `gh api`) is behind the TTL cache scripts/pipeline/status.py owns.
"""
from __future__ import annotations

import argparse
import json
import os
import sys
import time

from . import config

# `scripts.pipeline.state` resolves its inventory from KUNA_PIPELINE_STATE_DIR at call time,
# and this module is normally run as a bare `python3 -m scripts.repipe.status` with no env
# set -- in which case every slot, lease and worker read here came from the OTHER loop's
# `.kuna-pipeline/`, which is empty. The symptom is a status line reading `testers 0/3` while
# three testers are live and holding all three slots. Pin it before importing the collector,
# exactly as webui.py does; an explicit override still wins.
os.environ.setdefault("KUNA_PIPELINE_STATE_DIR", str(config.state_dir()))

from ..pipeline import status as pstatus
from ..pipeline import state as pstate


def _round_doc(n=None):
    from . import captain
    try:
        return captain.load_round(n if n is not None else captain.current_round())
    except Exception:
        return {}


def _backlog():
    try:
        from . import needs as needs_mod
        rows = needs_mod.load_all()
        by_status = {}
        for n in rows:
            by_status[n.status] = by_status.get(n.status, 0) + 1
        top = sorted(rows, key=needs_mod.rank_score, reverse=True)[:8]
        return {"total": len(rows), "by_status": by_status,
                "top": [{"need_id": n.need_id, "track": n.track, "status": n.status,
                         "severity": n.severity, "instances": n.instances,
                         "score": round(needs_mod.rank_score(n), 2), "title": n.title}
                        for n in top]}
    except Exception as exc:
        return {"error": str(exc), "total": 0, "by_status": {}, "top": []}


def _disk():
    st = os.statvfs(str(config.repo_root()))
    return {"free_gb": int(st.f_bavail * st.f_frsize / (1024 ** 3)),
            "min_gb": config.MIN_FREE_GB, "halt_gb": config.HALT_FREE_GB}


def collect(round_n=None):
    base = pstatus.collect()
    doc = _round_doc(round_n)
    data = pstate.snapshot()
    return {
        "ts": time.time(),
        "round": doc.get("round"),
        "states": {k: doc.get(k) for k in ("supervisor", "test", "build") if k in doc},
        "split": config.agent_split(),
        "slots": data.get("slots", {}),
        "leases": data.get("leases", {}),
        "agents": base.get("workers", []),
        "worktrees": base.get("worktrees", []),
        "prs": base.get("prs"),
        "cache": base.get("cache", {}),
        "backlog": _backlog(),
        "disk": _disk(),
        "spend_usd": doc.get("spend_usd", 0.0),
        "flags": {f: (config.state_dir() / f).exists() for f in ("STOP", "PAUSE", "ABORT")},
    }


def render(s):
    L = []
    st = s.get("states") or {}
    L.append("kuna RE-friction loop — round %s · %s / %s / %s"
             % (s.get("round"), st.get("supervisor", "?"), st.get("test", "?"),
                st.get("build", "?")))
    sp = s["split"]
    held = lambda pool: len((s["slots"].get(pool) or {}).get("held") or {})
    L.append("  agents: captain %d/%d · testers %d/%d · builders %d/%d   disk %dG (halt <%dG)"
             % (held("captain"), sp["captain"], held("tester"), sp["testers"],
                held("builder"), sp["builders"], s["disk"]["free_gb"], s["disk"]["halt_gb"]))
    on = [f for f, v in s["flags"].items() if v]
    if on:
        L.append("  FLAGS: %s" % ", ".join(on))
    L.append("-" * 92)
    # Show THIS round plus anything still running, not the whole history. The inventory is
    # cumulative across rounds and worker ids carry the round (`t-r2-<hexid>`), so by round 3
    # the live agents are buried under two rounds of `done` rows with five-figure stale times
    # -- which is how a real round-2 run rendered: six round-1 rows above the working fleet.
    agents = s["agents"] or []
    rnd = s.get("round")
    tag = "-r%s-" % rnd if rnd is not None else None

    def _current(w):
        if str(w.get("status")) == "running":
            return True
        return bool(tag) and tag in str(w.get("worker") or "")

    shown = [w for w in agents if _current(w)]
    hidden = len(agents) - len(shown)
    # running first, then most recently active -- the ones an operator is waiting on
    shown.sort(key=lambda w: (str(w.get("status")) != "running", w.get("stale_s") or 0))

    if not shown:
        L.append("  (no agents registered)" if not agents
                 else "  (no agents this round; %d from earlier rounds)" % hidden)
    else:
        L.append("  %-18s %-9s %-8s %-8s %-7s %s" % ("AGENT", "PHASE", "STATUS", "ELAPSED",
                                                     "STALE", "SLUG / PR"))
        for w in shown:
            stale = int(w.get("stale_s") or 0)
            # A row saying `running` whose pid is gone is the single most misleading thing
            # this table can print, and it is what sent round 4's captain to `ps`. Say it.
            alive = w.get("alive")
            status_txt = str(w.get("status"))
            if status_txt == "running" and alive is False:
                status_txt = "DEAD"
            L.append("  %-18s %-9s %-8s %-8s %-7s %s"
                     % (str(w.get("worker"))[:18], str(w.get("phase"))[:9],
                        status_txt[:8], pstatus._fmt_elapsed(w.get("elapsed_s") or 0),
                        ("%ds" % stale) if stale < 120 else ("%ds!" % stale),
                        w.get("pr_url") or w.get("slug") or "-"))
        if hidden:
            L.append("  (+%d agent(s) from earlier rounds, not shown)" % hidden)
    leases = s.get("leases") or {}
    if leases:
        L.append("  leases: %s" % ", ".join("%s=%s" % (r, l.get("holder"))
                                            for r, l in sorted(leases.items())))
    b = s["backlog"]
    L.append("-" * 92)
    L.append("  backlog: %d needs — %s" % (b["total"], ", ".join(
        "%s %d" % (k, v) for k, v in sorted(b["by_status"].items())) or "empty"))
    for n in b["top"]:
        L.append("    %-26s %-8s %-10s %7.1f  %s"
                 % (n["need_id"][:26], n["track"], n["status"], n["score"], n["title"][:44]))
    prs = s.get("prs")
    L.append("  open PRs: %s" % (len(prs) if prs is not None else "(gh unavailable)"))
    return "\n".join(L)


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.status")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--watch", action="store_true")
    ap.add_argument("--interval", type=float, default=2.0)
    ap.add_argument("--round", type=int, default=None)
    args = ap.parse_args(argv)

    if args.json and not args.watch:
        print(json.dumps(collect(args.round), indent=2, default=str))
        return 0
    if not args.watch:
        print(render(collect(args.round)))
        return 0
    try:
        while True:
            sys.stdout.write("\x1b[2J\x1b[H" + render(collect(args.round)) + "\n")
            sys.stdout.flush()
            time.sleep(args.interval)
    except KeyboardInterrupt:
        return 0


if __name__ == "__main__":
    sys.exit(main())
