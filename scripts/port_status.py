#!/usr/bin/env python3
"""angr -> kuna PORT STATUS: how many angr features / testcases we've ported.

Two views:
  1. PORTED FEATURES  -- the angr passes/features implemented as kuna `--option`s.
     AUTO-DETECTED from `stages.toml` provenance (every shipped feature records
     `source_decompiler = "angr"` + `inspiration` = the angr test/pass it came from,
     per the pipeline's standing rules). So the "which have we completed" list
     maintains ITSELF -- no hand-edited manifest to keep in sync. The only thing
     stored here is the OUTCOMES table below: the human assessment the catalog
     can't know (real win vs a faithful no-op that just matches Ghidra vs infra).
  2. TESTCASE COVERAGE -- how many angr testcases kuna now MATCHES (no structural
     gap), from the latest angr-vs-kuna sweep (docs/pipeline/opportunities.json),
     plus the top remaining gaps so you can see what's left.

Usage:
  python3 scripts/port_status.py                # human report
  python3 scripts/port_status.py --json         # machine-readable
  python3 scripts/port_status.py --remaining 25 # show more remaining gaps
"""
import argparse
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STAGES = os.path.join(ROOT, "decompiler", "crates", "kuna-decomp", "stages.toml")
OPPS = os.path.join(ROOT, "docs", "pipeline", "opportunities.json")
MATRIX = os.path.join(ROOT, "docs", "pipeline", "matrix.md")

# --- the one thing not in the catalog: the OUTCOME of each port ---------------
# kind buckets:
#   win    = a real structural/correctness/goto-reduction win over stock Ghidra
#   noop   = faithful angr port that turns out to MATCH Ghidra (no in-tree delta)
#   infra  = platform / scaffold (enables future wins; neutral output today)
#   loader = input-format support (unblocks a class of binaries)
#   polish = presentation / naming / cosmetic cleanup
# pr = the PR that landed it (None for pre-this-effort ports).
# Edit this when you land a new angr port (the rest is auto from stages.toml).
OUTCOMES = {
    "relocobjects":         ("loader", 37,   "ET_REL .o loader (CLE) — unblocked the .o corpus"),
    "i386_pie_plt":         ("win",    51,   "i386 PIE PLT import names"),
    "noreturn_propagate":   ("win",    52,   "structural no-return propagation"),
    "loopbreak_recovery":   ("win",    55,   "loop-exit gotos -> break/continue"),
    "gotoreduce":           ("win",    57,   "return-tail duplication (SAILR)"),
    "switchguardbound":     ("win",    60,   "guard-bounded GCC PIC jump tables (slow -> opt-in)"),
    "tailcalljump":         ("win",    59,   "-O2 tail jumps -> tail calls"),
    "branchflip":           ("polish", 69,   "if(a0) vs if(a0==0) polarity"),
    "foldcallret":          ("polish", 61,   "fold single-use call returns into the use site"),
    "dedupvardecls":        ("polish", 42,   "collapse duplicate scalar local declarations"),
    "noreturn_extern":      ("win",    90,   "name-based no-return for undefined externs"),
    "noreturn_externmatch": ("win",    91,   "name-match known-no-return libc"),
    "crossjumprevert":      ("win",    92,   "SAILR CrossJumpReverter (cross-jump tail duplication)"),
    "switchsharedcase":     ("win",    93,   "b2sum loop-carried-guard getopt jump table"),
    "regionstructure":      ("infra",  85,   "Phoenix/SAILR region structurer (Inc 0-6) — matches Ghidra"),
    "regionlooprefine":     ("infra",  95,   "loop-successor refinement infra (goto-win = Inc-6 follow-up)"),
    "ifelseflatten":        ("noop",   96,   "angr IfElseFlattener — Ghidra already flattens (faithful no-op)"),
    "taildup":              ("win",    103,  "duplicate a return-tail WITH a call (gap gotoreduce/crossjumprevert reject)"),
    "switchmultipred":      ("win",    102,  "multi-predecessor image-base-relative unrolled-guard jump tables"),
    "dedupitetail":         ("win",    104,  "merge a duplicated if/else tail (inverse of the duplication passes)"),
    "unrolledguard":        ("win",    105,  "recover MSVC interleaved Duff's-device jump tables"),
    # pre-this-effort ports (the original loweredswitch/namestyle/stackguard work)
    "loweredswitch":        ("win",    None, "angr LoweredSwitchSimplifier"),
    "namestyle":            ("polish", None, "angr default naming scheme (vN/aN/sub_/...)"),
    "stackguard":           ("win",    None, "angr StackCanarySimplifier"),
}

KIND_LABEL = {
    "win":    "WIN    ",
    "noop":   "NO-OP  ",   # faithful port; Ghidra already matched
    "infra":  "INFRA  ",
    "loader": "LOADER ",
    "polish": "POLISH ",
    "?":      "???    ",
}
KIND_ORDER = ["win", "loader", "infra", "polish", "noop", "?"]


def parse_angr_settables():
    """Auto-detect ported angr features from stages.toml provenance."""
    if not os.path.exists(STAGES):
        return []
    txt = open(STAGES).read()
    out = []
    for block in re.split(r"\n\[\[settable\]\]\n", txt):
        if re.search(r'source_decompiler\s*=\s*"angr"', block) is None:
            continue

        def g(key):
            m = re.search(key + r'\s*=\s*"((?:[^"\\]|\\.)*)"', block)
            return m.group(1) if m else ""

        out.append({
            "option": g("option"),
            "default": g("default"),
            "change_kind": g("change_kind"),
            "inspiration": g("inspiration"),
        })
    return out


def sweep_coverage():
    """Testcase coverage from the latest angr-vs-kuna sweep."""
    if not os.path.exists(OPPS):
        return None
    ranked = json.load(open(OPPS)).get("ranked", [])
    comparable = [o for o in ranked if o.get("comparable")]
    match = [o for o in comparable if o.get("score", 0) == 0]
    gap = sorted((o for o in comparable if o.get("score", 0) >= 1),
                 key=lambda o: -o.get("score", 0))
    return {"swept": len(ranked), "comparable": len(comparable),
            "match": len(match), "gap": gap}


def bar(done, total, width=42):
    if total <= 0:
        return "(no data)"
    filled = int(round(width * done / total))
    return "[%s%s] %d/%d (%d%%)" % ("#" * filled, "." * (width - filled),
                                    done, total, round(100 * done / total))


def main(argv=None):
    ap = argparse.ArgumentParser(description="angr -> kuna port status")
    ap.add_argument("--json", action="store_true", help="machine-readable output")
    ap.add_argument("--remaining", type=int, default=15, help="how many remaining gaps to list")
    args = ap.parse_args(argv)

    ported = parse_angr_settables()
    for p in ported:
        kind, pr, note = OUTCOMES.get(p["option"], ("?", None, ""))
        p["kind"], p["pr"], p["note"] = kind, pr, note
    by_kind = {}
    for p in ported:
        by_kind.setdefault(p["kind"], []).append(p)

    cov = sweep_coverage()

    if args.json:
        print(json.dumps({
            "ported_count": len(ported),
            "ported": ported,
            "by_kind": {k: len(v) for k, v in by_kind.items()},
            "coverage": ({"swept": cov["swept"], "comparable": cov["comparable"],
                          "match": cov["match"], "gap": len(cov["gap"])} if cov else None),
        }, indent=2))
        return 0

    print("=" * 74)
    print("  angr  ->  kuna     PORT STATUS")
    print("=" * 74)
    print()
    print("  PORTED ANGR FEATURES: %d   (auto-detected from stages.toml provenance)" % len(ported))
    print()
    for k in KIND_ORDER:
        items = sorted(by_kind.get(k, []), key=lambda x: x["option"])
        if not items:
            continue
        print("  %s(%d)" % (KIND_LABEL.get(k, k), len(items)))
        for p in items:
            pr = ("#%s" % p["pr"]) if p["pr"] else "  —"
            note = p["note"] or p["inspiration"][:50]
            print("      %-22s %-5s [%-5s] %s" % (p["option"], pr, p["default"], note))
        print()
    print("  ----")
    print("  %d real WINS  ·  %d loader  ·  %d infra  ·  %d polish  ·  %d no-op (Ghidra already matched)"
          % (len(by_kind.get("win", [])), len(by_kind.get("loader", [])),
             len(by_kind.get("infra", [])), len(by_kind.get("polish", [])),
             len(by_kind.get("noop", []))))
    print()

    if cov:
        print("-" * 74)
        print("  TESTCASE COVERAGE   (latest angr-vs-kuna sweep, docs/pipeline/opportunities.json)")
        print()
        print("    kuna matches angr   " + bar(cov["match"], cov["comparable"]))
        print("    %d of %d comparable angr testcases have NO structural gap;" % (cov["match"], cov["comparable"]))
        print("    %d still show an angr advantage (the work that remains)." % len(cov["gap"]))
        print()
        print("  TOP %d REMAINING GAPS (what's left to close):" % args.remaining)
        for o in cov["gap"][:args.remaining]:
            kinds = ",".join(o.get("kinds", [])[:3])
            label = (o.get("test_name", "") + " :: " + str(o.get("selector")))[:48]
            print("    [score %2d]  %-48s  %s" % (o.get("score", 0), label, kinds))
        print()
        print("  (refresh coverage with:  PYTHONPATH=. python -m scripts.pipeline.sweep --arch x86)")
    else:
        print("  (no sweep data at docs/pipeline/opportunities.json — run scripts.pipeline.sweep)")
    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
