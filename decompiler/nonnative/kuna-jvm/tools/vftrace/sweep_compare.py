#!/usr/bin/env python3
"""Compare two corpus_trace.py output directories (A = Vineflower, B = kuna) case by case.

For every case of either report.json: every method of both traces is compared with compare.py's rules
(OK, first divergent step, or missing on one side); the produced .java files are compared with the oracle
(manifest cases) or with each other (--classes cases). Prints one line per diverging/missing method and
per output mismatch (a difference confined to the stack-trace lines of "Couldn't be decompiled" comments is
counted apart, as TEXT-EXC, and does not fail the run: kuna cannot print a Java stack trace), then the summary (methods OK / diverging / missing in A / missing in B, steps
compared) and the histogram of first-divergent steps. Exit status 1 when a method diverges or is missing on
one side (with --strict-text also on final-text differences).

usage: sweep_compare.py A_DIR B_DIR [--jobs N] [--json OUT] [--ignore REGEX]... [--max-diff N] [--quiet]
                        [--strict-text]
"""
import argparse
import collections
import concurrent.futures as cf
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import compare  # noqa: E402


def load_report(d):
    path = os.path.join(d, "report.json")
    if not os.path.isfile(path):
        return {}
    with open(path, encoding="utf-8") as f:
        return {r["id"]: r for r in json.load(f)["results"]}


def same_file(a, b):
    """True / False / None (a file is missing) / "exc" (equal once exception dumps are blanked)."""
    if a is None or b is None:
        return None
    with open(a, "rb") as f1, open(b, "rb") as f2:
        da, db = f1.read(), f2.read()
    if da == db:
        return True
    return "exc" if strip_exception_dumps(da) == strip_exception_dumps(db) else False


AT = re.compile(rb"^\s*//   at ")


def strip_exception_dumps(data):
    """Drops the stack-trace lines of "Couldn't be decompiled" comments and the exception line above them:
    kuna has no Java stack trace to print (dump-exception-on-error)."""
    lines = data.split(b"\n")
    drop = set()
    for i, line in enumerate(lines):
        if AT.match(line):
            drop.add(i)
            if i > 0 and not AT.match(lines[i - 1]):
                drop.add(i - 1)
    return [line for i, line in enumerate(lines) if i not in drop]


def compare_case(args):
    cid, a_dir, b_dir, ra, rb, ignores, max_diff = args
    ignores = [re.compile(r) for r in ignores]
    out = dict(id=cid, methods={}, steps=0)
    ta, tb = os.path.join(a_dir, cid + ".vft"), os.path.join(b_dir, cid + ".vft")
    ma = compare.parse(ta)[1] if os.path.isfile(ta) else {}
    mb = compare.parse(tb)[1] if os.path.isfile(tb) else {}
    for key in sorted(set(ma) | set(mb)):
        if key not in ma or key not in mb:
            out["methods"][key] = dict(status="missing", missing_in="A" if key not in ma else "B",
                                       steps=len(ma.get(key) or mb.get(key)))
            continue
        d = compare.compare_method(ma[key], mb[key], ignores)
        if d is None:
            out["steps"] += len(ma[key])
            out["methods"][key] = dict(status="ok", steps=len(ma[key]))
            continue
        i, sa, sb, la, lb = d
        out["steps"] += i + 1
        if sa is None or sb is None:
            hist = f"(trace ends early in {'B' if sb is None else 'A'})"
        elif sa != sb:
            hist = f"{compare.base_name(sa)}|{compare.base_name(sb)}"
        else:
            hist = compare.base_name(sa)
        diff = list(compare.difflib.unified_diff(la, lb, f"A @{i} {sa}", f"B @{i} {sb}", n=3, lineterm=""))
        out["methods"][key] = dict(status="diff", step=i, name_a=sa, name_b=sb, hist=hist,
                                   steps_a=len(ma[key]), steps_b=len(mb[key]), diff=diff[:max_diff])
    pa = os.path.join(a_dir, ra["produced"]) if ra and ra.get("produced") else None
    pb = os.path.join(b_dir, rb["produced"]) if rb and rb.get("produced") else None
    out["oracle_a"] = ra.get("output_match") if ra else None
    out["oracle_b"] = rb.get("output_match") if rb else None
    out["text_equal"] = same_file(pa, pb)
    out["rc_a"] = ra.get("rc") if ra else None
    out["rc_b"] = rb.get("rc") if rb else None
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--json", default=None)
    ap.add_argument("--ignore", action="append", default=[])
    ap.add_argument("--max-diff", type=int, default=120)
    ap.add_argument("--quiet", action="store_true", help="summary only")
    ap.add_argument("--strict-text", action="store_true",
                    help="also fail on final-text differences (default: reported only; the corpus runner owns text parity)")
    args = ap.parse_args()
    ra, rb = load_report(args.a), load_report(args.b)
    ids = sorted(set(ra) | set(rb))
    work = [(cid, args.a, args.b, ra.get(cid), rb.get(cid), args.ignore, args.max_diff) for cid in ids]
    with cf.ProcessPoolExecutor(max_workers=args.jobs) as ex:
        cases = list(ex.map(compare_case, work, chunksize=4))

    counts = collections.Counter()
    hist = collections.Counter()
    steps = 0
    lines = []
    for c in cases:
        steps += c["steps"]
        for key, m in c["methods"].items():
            st = m["status"]
            if st == "ok":
                counts["ok"] += 1
            elif st == "missing":
                counts["missing_" + m["missing_in"]] += 1
                hist[f"(method missing in {m['missing_in']})"] += 1
                lines.append(f"MISSING-{m['missing_in']} {c['id']} {key}")
            else:
                counts["diff"] += 1
                hist[m["hist"]] += 1
                lines.append(f"DIFF {c['id']} {key}: step {m['step']} {m['name_a']}"
                             + ("" if m["name_a"] == m["name_b"] else f" | {m['name_b']}"))
        if c["oracle_a"] is False or c["oracle_b"] is False:
            counts["oracle_mismatch_" + ("A" if c["oracle_a"] is False else "B")] += 1
            lines.append(f"ORACLE {c['id']}: A={c['oracle_a']} B={c['oracle_b']}")
        if c["text_equal"] == "exc":
            counts["text_exc_only"] += 1
            lines.append(f"TEXT-EXC {c['id']}: outputs differ only in exception dumps")
        elif c["text_equal"] is False:
            counts["text_differs"] += 1
            lines.append(f"TEXT {c['id']}: outputs differ")
        elif c["text_equal"] is None:
            counts["text_missing"] += 1
            lines.append(f"NOTEXT {c['id']}: output missing (rc A={c['rc_a']} B={c['rc_b']})")
    if not args.quiet:
        for line in lines:
            print(line)
    total = sum(counts[k] for k in ("ok", "diff", "missing_A", "missing_B"))
    summary = dict(cases=len(cases), methods=total, ok=counts["ok"], diverge=counts["diff"],
                   missing_in_a=counts["missing_A"], missing_in_b=counts["missing_B"], steps_compared=steps,
                   text_differs=counts["text_differs"], text_exc_only=counts["text_exc_only"],
                   text_missing=counts["text_missing"],
                   oracle_mismatch_a=counts["oracle_mismatch_A"], oracle_mismatch_b=counts["oracle_mismatch_B"])
    print(f"\n{summary['cases']} cases, {total} methods: {counts['ok']} OK, {counts['diff']} diverge, "
          f"{counts['missing_A']} missing in A, {counts['missing_B']} missing in B; {steps} steps compared; "
          f"final text: {counts['text_differs']} differ, {counts['text_exc_only']} differ only in exception dumps, "
          f"{counts['text_missing']} missing; "
          f"oracle mismatches A={counts['oracle_mismatch_A']} B={counts['oracle_mismatch_B']}")
    if hist:
        print("first divergence at: " + ", ".join(f"{k} ×{v}" for k, v in hist.most_common()))
    if args.json:
        with open(args.json, "w", encoding="utf-8") as f:
            json.dump(dict(summary=summary, histogram=dict(hist), cases=cases), f, indent=1)
    bad = counts["diff"] + counts["missing_A"] + counts["missing_B"]
    if args.strict_text:
        bad += counts["text_differs"] + counts["text_missing"] + counts["oracle_mismatch_B"]
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
