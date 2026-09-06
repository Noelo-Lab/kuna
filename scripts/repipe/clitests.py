"""Run the promoted regression probes in tests/cli/ as a gate.

Every need the loop closes leaves its acceptance probe here (`verify --promote`). Without
something that RUNS them they are inert: the loop would keep depositing evidence that a gap
was closed while nothing checked it stayed closed, which is precisely the rot the
`regressed` status exists to catch -- except a regression would then be found a round later
by a tester rather than a minute later by CI.

A probe only lands here if its target is `in-repo`, so this runs with no dataset and is safe
in CI. `make test-cli` is the entry point.
"""
from __future__ import annotations

import argparse
import json
import os
import sys

from . import config, probe, verify


def cases(directory=None):
    d = directory or config.cli_tests_dir()
    if not os.path.isdir(str(d)):
        return []
    return [os.path.join(str(d), f) for f in sorted(os.listdir(str(d))) if f.endswith(".json")]


def run_one(path, reps=None):
    """(name, ok, verdict_or_error). A probe here asserts the FIXED behaviour, so ok == passed."""
    name = os.path.basename(path)[:-5]
    try:
        with open(path) as fh:
            doc = json.load(fh)
    except (OSError, ValueError) as exc:
        return name, False, {"error": "unreadable: %s" % exc}

    # Refuse rather than skip: a dataset-backed probe here would pass on a developer's box
    # and fail in CI, which is worse than not having the test. The predicate is the one
    # `verify --promote` admitted the probe under, so the two can never disagree -- a probe
    # that needs no binary at all (`kuna decompile --help`) is runnable anywhere.
    ok, why = verify.vendorable(doc)
    if not ok:
        return name, False, {"error": "%s -- this corpus must run without the dataset" % why}
    rel = (doc.get("target") or {}).get("in_repo_path")
    ctx = {"work": str(config.repo_root())}
    if rel:
        ctx["bin"] = os.path.join(str(config.repo_root()), rel)
    if reps:
        doc = dict(doc)
        doc["repeat"] = max(int(doc.get("repeat") or 1), int(reps))
    try:
        v = probe.check(doc, ctx)
    except Exception as exc:
        return name, False, {"error": "%s: %s" % (type(exc).__name__, exc)}
    return name, bool(v.get("passed")) and not v.get("flaky"), v


def main(argv=None):
    ap = argparse.ArgumentParser(prog="python -m scripts.repipe.clitests")
    ap.add_argument("--dir", default=None)
    ap.add_argument("--reps", type=int, default=None)
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--name", action="append", default=[])
    args = ap.parse_args(argv)

    paths = cases(args.dir)
    if args.name:
        paths = [p for p in paths if os.path.basename(p)[:-5] in args.name]
    if not paths:
        # An empty corpus is a legitimate state (nothing promoted yet) and must not fail.
        print("tests/cli: no cases")
        return 0

    rows, failed = [], 0
    for p in paths:
        name, ok, v = run_one(p, args.reps)
        rows.append({"name": name, "ok": ok,
                     "error": v.get("error"), "flaky": v.get("flaky"),
                     "clauses": [c for c in (v.get("clauses") or []) if not c.get("ok")]})
        if not ok:
            failed += 1
        if not args.json:
            print("  %-6s %s%s" % ("ok" if ok else "FAIL", name,
                                   "" if ok else "  <- %s" % (v.get("error") or "clause failed")))
            if not ok:
                for c in rows[-1]["clauses"][:4]:
                    print("         %s expected %s actual %s"
                          % (c.get("clause"), c.get("expected"), c.get("actual")))

    if args.json:
        print(json.dumps({"total": len(rows), "failed": failed, "cases": rows}, indent=2,
                         default=str))
    else:
        print("tests/cli: %d/%d passed" % (len(rows) - failed, len(rows)))
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
