"""Pick the next need a builder should take, and refuse the ones that would collide.

Three layers of collision avoidance, cheapest first (docs/re-pipeline.md):

1. **Track separation is the structural win.** Track `tooling` touches kuna-cli /
   kuna-console / kuna-analysis / tests/cli / docs/cli.md. Track `quality` touches
   kuna-decomp / phases.toml / options.rs / the four hard-coded counters / docs/options.md /
   docs/history.md / tests/stages. Those sets are DISJOINT, so three tooling builders run in
   parallel with no shared mutable state at all.
2. **Named resource leases.** A need's track and `touches:` map to a resource set; a need
   whose set intersects a held lease is not dispatched. Because every quality need needs
   counter:catalog + counter:stages-corpus + counter:div + file:phases.toml, this yields
   "at most one option-adding builder at a time" without special-casing it.
3. **Contracts.** What each live builder is doing is written to contracts.json and rendered
   into every sibling's prompt, so a builder that needs someone else's file stops and says
   so rather than racing for it.

The ranked backlog is emitted in the exact `{"ranked": [...]}` shape that
docs/improvement-pipeline/opportunities.json uses, so scripts/pipeline/select.py can consume
it unchanged; this module adds the lease-feasibility filter that the angr selector has no
concept of.
"""
from __future__ import annotations

import argparse
import json
import os
import sys

from . import config
from ..pipeline import state as pstate

# Resources a track always contends for, regardless of what the need says it touches.
# Every quality need needs the whole counter set, so "at most one option-adding builder in
# flight" falls out of the lease algebra rather than being special-cased.
TRACK_RESOURCES = {
    "quality": ["counter:catalog", "counter:stages-corpus", "counter:div",
                "file:phases.toml", "file:docs/options.md"],
    "loader": [],
    "tooling": [],
    "perf": [],
}

# A path prefix -> the lease that owns it. Longest match wins.
# Only paths where a SILENT wrong merge is possible get a lease. Three tooling builders all
# adding a line to kuna-cli's dispatch table is a trivial rebase, and leasing the crate for
# that would throttle the pipeline to one builder -- which is exactly what the three-slot
# default exists to avoid. The counters, phases.toml and the baselines are different: an
# identical `85 -> 86` edit on two branches merges CLEANLY to the wrong number.
PATH_RESOURCES = [
    ("decompiler/crates/kuna-decomp/phases.toml", "file:phases.toml"),
    ("decompiler/crates/kuna-decomp/src/p0_knowledge/options.rs", "counter:catalog"),
    ("decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_phases", "counter:catalog"),
    ("decompiler/crates/kuna-decomp/tests/catalog_bytecompat.rs", "counter:catalog"),
    ("decompiler/crates/kuna-base/src/xml.rs", "counter:stages-corpus"),
    ("docs/options.md", "file:docs/options.md"),
    ("docs/history.md", "counter:div"),
    ("docs/baseline-stages.json", "counter:stages-corpus"),
    ("tests/stages", "counter:stages-corpus"),
]


def resources_for(need):
    """Every lease a builder on this need would have to hold."""
    track = getattr(need, "track", None) or need.get("track", "tooling")
    touches = getattr(need, "touches", None)
    if touches is None:
        touches = need.get("touches", []) if isinstance(need, dict) else []
    res = set(TRACK_RESOURCES.get(track, []))
    for t in touches or []:
        best = None
        for prefix, name in PATH_RESOURCES:
            if t.startswith(prefix) and (best is None or len(prefix) > len(best[0])):
                best = (prefix, name)
        if best:
            res.add(best[1])
    nid = getattr(need, "need_id", None) or (need.get("need_id") if isinstance(need, dict) else None)
    if nid:
        res.add("cluster:%s" % nid)
    return sorted(res)


def held_resources(snapshot=None):
    data = snapshot if snapshot is not None else pstate.snapshot()
    return set(data.get("leases", {}).keys())


def feasible(need, held=None):
    """(bool, blocking_resource_or_None) — can a builder start on this need right now?"""
    held = held if held is not None else held_resources()
    for r in resources_for(need):
        if r in held:
            return False, r
    return True, None


DISPATCHABLE = ("open", "regressed")


def candidates(needs_list=None):
    from . import needs as needs_mod
    all_needs = needs_list if needs_list is not None else needs_mod.load_all()
    out = [n for n in all_needs if getattr(n, "status", "open") in DISPATCHABLE]
    out.sort(key=needs_mod.rank_score, reverse=True)
    return out


def pick(k=1, needs_list=None):
    """The top k dispatchable needs whose resource sets are free AND disjoint from each other.

    Disjointness among the picks matters as much as against held leases: handing two
    quality needs to two builders in the same tick would recreate exactly the shared-counter
    race the leases exist to prevent.
    """
    held = set(held_resources())
    picks = []
    for n in candidates(needs_list):
        res = set(resources_for(n))
        if res & held:
            continue
        picks.append({"need": n, "resources": sorted(res)})
        held |= res
        if len(picks) >= k:
            break
    return picks


def write_contracts(picks, round_n, path=None):
    """The block rendered into every builder prompt: what the siblings are doing.

    This is the captain 'making the developers aware of each other's goals'. It is prose on
    purpose — the leases are the enforcement, this is the explanation.
    """
    path = path or (config.state_dir() / "contracts.json")
    doc = {"round": round_n, "builders": [
        {"need": p["need"].need_id, "track": p["need"].track,
         "title": p["need"].title, "resources": p["resources"],
         "touches": list(getattr(p["need"], "touches", []) or [])}
        for p in picks]}
    os.makedirs(os.path.dirname(str(path)), exist_ok=True)
    tmp = str(path) + ".tmp"
    with open(tmp, "w") as fh:
        json.dump(doc, fh, indent=2)
    os.replace(tmp, path)
    return path


def contracts_markdown(exclude_need=None, path=None):
    path = path or (config.state_dir() / "contracts.json")
    if not os.path.exists(str(path)):
        return ""
    with open(str(path)) as fh:
        doc = json.load(fh)
    rows = [b for b in doc.get("builders", []) if b["need"] != exclude_need]
    if not rows:
        return ""
    lines = ["## Other builders running right now — do not touch their files", ""]
    for b in rows:
        lines.append("- `%s` (%s) — %s" % (b["need"], b["track"], b["title"]))
        lines.append("  leases: %s" % ", ".join(b["resources"]))
        if b.get("touches"):
            lines.append("  declared files: %s" % ", ".join(b["touches"]))
    lines += ["",
              "If you need a file another builder holds, STOP and report",
              "`state update --worker <id> --status blocked --note \"needs <resource> held by <worker>\"`.",
              "Do not race for it; the captain re-queues you next round."]
    return "\n".join(lines)


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.select")
    ap.add_argument("-k", "--count", type=int, default=1)
    ap.add_argument("--round", type=int, default=0)
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--shell", action="store_true",
                    help="eval-able assignments, in tools/pipeline/run.sh's idiom")
    ap.add_argument("--write-contracts", action="store_true")
    ap.add_argument("--resources-for", default=None, metavar="NEED_ID")
    args = ap.parse_args(argv)

    if args.resources_for:
        from . import needs as needs_mod
        n = needs_mod.load(args.resources_for)
        if n is None:
            print("no such need: %s" % args.resources_for, file=sys.stderr)
            return 1
        ok, blocker = feasible(n)
        out = {"need": n.need_id, "resources": resources_for(n),
               "feasible": ok, "blocked_by": blocker}
        print(json.dumps(out, indent=2) if args.json else
              "%s feasible=%s blocked_by=%s\n  %s" % (n.need_id, ok, blocker,
                                                     " ".join(resources_for(n))))
        return 0 if ok else 1

    picks = pick(args.count)
    if args.write_contracts:
        write_contracts(picks, args.round)
    if not picks:
        return 1
    if args.json:
        print(json.dumps([{"need_id": p["need"].need_id, "track": p["need"].track,
                           "title": p["need"].title, "resources": p["resources"]}
                          for p in picks], indent=2))
    elif args.shell:
        p = picks[0]["need"]
        print("OPP_ID=%s" % _q(p.need_id))
        print("TEST_NAME=%s" % _q(p.need_id))
        print("BINARY=%s" % _q(getattr(p, "binary", "") or ""))
        print("SELECTOR=%s" % _q(getattr(p, "selector", "") or ""))
        print("ARCH=")
        print("SLUG=%s" % _q(p.need_id))
        print("SCORE=%s" % _q("%.3f" % __import__("scripts.repipe.needs", fromlist=["x"]).rank_score(p)))
        print("KINDS=%s" % _q(p.track))
    else:
        for p in picks:
            print("%-28s %-8s %s" % (p["need"].need_id, p["need"].track, p["need"].title))
    return 0


def _q(v):
    return "'" + str(v).replace("'", "'\\''") + "'"


if __name__ == "__main__":
    sys.exit(main())
