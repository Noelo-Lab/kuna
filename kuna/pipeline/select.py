"""Pick the next opportunity for a worker to implement.

Reads the ranked backlog (docs/pipeline/opportunities.json), skips anything already
claimed or done (kuna.pipeline.state), and returns the highest-scoring remaining gap.
Emits shell-eval-able assignments so the driver loop can launch a worker without jq.

    python -m kuna.pipeline.select --shell        # KEY='val' lines; exit 1 if nothing left
    python -m kuna.pipeline.select --json
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys

from . import config, state


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


def load_backlog():
    path = config.pipeline_docs_dir() / "opportunities.json"
    if not path.exists():
        return []
    with open(path) as fh:
        return json.load(fh).get("ranked", [])


def pick(*, skip_custom=False, min_score=1):
    taken = state.claimed_or_done()
    for opp in load_backlog():  # already sorted by rank/score
        if opp.get("score", 0) < min_score:
            continue
        if not opp.get("comparable"):
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
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.select")
    p.add_argument("--shell", action="store_true", help="emit shell KEY='val' assignments")
    p.add_argument("--json", action="store_true")
    p.add_argument("--skip-custom", action="store_true")
    p.add_argument("--min-score", type=int, default=1)
    args = p.parse_args(argv)

    opp = pick(skip_custom=args.skip_custom, min_score=args.min_score)
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
