"""Pick the next opportunity for a worker to implement.

Reads the ranked backlog (docs/improvement-pipeline/opportunities.json), skips anything already
claimed or done (scripts.pipeline.state), and returns the highest-scoring remaining gap.
Emits shell-eval-able assignments so the driver loop can launch a worker without jq.

    python -m scripts.pipeline.select --shell        # KEY='val' lines; exit 1 if nothing left
    python -m scripts.pipeline.select --json
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import sys

from . import config, state


class BacklogError(RuntimeError):
    """The ranked backlog is unreadable or cannot be used for selection."""


def _slug(opp):
    # base on the descriptive test name (the function is often the generic "main");
    # strip the boilerplate test prefix so the slug reads like the testcase it came from.
    t = opp.get("test_name", "") or opp.get("func_name") or "feat"
    t = re.sub(r"^test_", "", t)
    t = re.sub(r"^(decompiling|decompilation|decompile)_", "", t)
    base = re.sub(r"[^a-zA-Z0-9]+", "-", t.lower()).strip("-")[:28].strip("-") or "feat"
    h = hashlib.sha1((opp.get("test_name", "") + "::" + str(opp.get("selector"))).encode()).hexdigest()[:6]
    return "%s-%s" % (base, h)


def _opp_id(opp):
    return "%s::%s" % (opp.get("test_name", ""), opp.get("selector"))


def _validate_row(row, location):
    if not isinstance(row, dict):
        raise BacklogError(f"invalid backlog {location}: expected an object")
    score = row.get("score", 0)
    if type(score) not in (int, float) or (isinstance(score, float) and not math.isfinite(score)):
        raise BacklogError(f"invalid backlog {location}: score must be a finite number")
    if not isinstance(row.get("test_name", ""), str):
        raise BacklogError(f"invalid backlog {location}: test_name must be a string")
    if row.get("func_name") is not None and not isinstance(row["func_name"], str):
        raise BacklogError(f"invalid backlog {location}: func_name must be a string or null")
    for field in ("kinds", "reasons"):
        values = row.get(field, [])
        if not isinstance(values, list) or any(not isinstance(value, str) for value in values):
            raise BacklogError(f"invalid backlog {location}: {field} must be an array of strings")


def _read_ranked(name):
    path = config.pipeline_docs_dir() / name
    try:
        with open(path) as fh:
            document = json.load(fh)
    except FileNotFoundError:
        return []
    except (json.JSONDecodeError, UnicodeError, OSError) as exc:
        raise BacklogError(f"cannot read backlog {path}: {exc}") from exc
    if not isinstance(document, dict):
        raise BacklogError(f"invalid backlog {path}: expected an object")
    ranked = document.get("ranked")
    if not isinstance(ranked, list):
        raise BacklogError(f"invalid backlog {path}: ranked must be an array")
    for index, row in enumerate(ranked):
        _validate_row(row, f"{path}: ranked[{index}]")
    return ranked


def load_backlog():
    """The combined backlog: the structural matrix + the small-units worklist.

    units.json is kept separate from opportunities.json so a re-sweep (which regenerates
    opportunities.json) never wipes the small-units. Highest score first across both.
    """
    combined = _read_ranked("opportunities.json") + _read_ranked("units.json")
    combined.sort(key=lambda o: o.get("score", 0), reverse=True)
    return combined


def _matches_kind(opp, kind):
    if not kind:
        return True
    kinds = opp.get("kinds", []) or []
    if kind == "small-unit":
        return "small-unit" in kinds
    if kind == "structural":
        return "small-unit" not in kinds  # everything in the structural matrix
    return kind in kinds


def pick(*, skip_custom=False, min_score=1, kind=None):
    taken = state.taken()
    for opp in load_backlog():  # already sorted by rank/score
        if opp.get("score", 0) < min_score:
            continue
        if not opp.get("comparable"):
            continue
        if not _matches_kind(opp, kind):
            continue
        oid = _opp_id(opp)
        if oid in taken:
            continue
        opp = dict(opp)
        opp["opportunity_id"] = oid
        opp["slug"] = _slug(opp)
        return opp
    return None


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m scripts.pipeline.select")
    p.add_argument("--shell", action="store_true", help="emit shell KEY='val' assignments")
    p.add_argument("--json", action="store_true")
    p.add_argument("--skip-custom", action="store_true")
    p.add_argument("--min-score", type=int, default=1)
    p.add_argument("--kind", choices=["small-unit", "structural"], default=None,
                   help="restrict to small self-contained units or structural-matrix gaps")
    args = p.parse_args(argv)

    try:
        opp = pick(skip_custom=args.skip_custom, min_score=args.min_score, kind=args.kind)
    except (BacklogError, state.StateError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2
    if opp is None:
        if not (args.shell or args.json):
            print("no remaining opportunities", file=sys.stderr)
        return 1

    if args.json:
        print(json.dumps(opp, indent=2))
        return 0
    if args.shell:
        def sh(v):
            return "'" + str(v).replace("'", "'\\''") + "'"
        print("OPP_ID=%s" % sh(opp["opportunity_id"]))
        print("TEST_NAME=%s" % sh(opp.get("test_name", "")))
        print("BINARY=%s" % sh(opp.get("binary", "")))
        print("SELECTOR=%s" % sh(opp.get("selector", "")))
        print("ARCH=%s" % sh(opp.get("arch") or ""))
        print("SLUG=%s" % sh(opp["slug"]))
        print("SCORE=%s" % sh(opp.get("score", 0)))
        print("KINDS=%s" % sh(",".join(opp.get("kinds", []))))
        return 0
    # default human
    print("next: [score %d] %s :: %s  (slug %s)  kinds=%s"
          % (opp.get("score", 0), opp.get("test_name"), opp.get("selector"),
             opp["slug"], ",".join(opp.get("kinds", []))))
    print("  why: %s" % "; ".join(opp.get("reasons", [])))
    return 0


if __name__ == "__main__":
    sys.exit(main())
