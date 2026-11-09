#!/usr/bin/env python3
"""Compare two pass traces (vftrace format, see README.md) method by method.

For every method prints OK, or the first divergent step "<n> <StepName>" followed by a unified diff of
that step's dump, then a histogram of first-divergence steps across all methods.

usage: compare.py A.vft B.vft [--names A B] [--filter S] [--context N] [--max-lines N] [--quiet]
                  [--prefix] [--ignore REGEX]... [--json OUT]
  --prefix        a trace that stops early but agrees on every step it has is OK ("OK-PREFIX"):
                  for comparing a partially ported pipeline, or kuna's --initial-only mode
  --ignore REGEX  delete every match of REGEX from each dump line before comparing (repeatable)
  --quiet         no per-method diffs, only the OK/DIFF lines and the summary
  --json OUT      machine-readable result: {method: {"status", "step", "name_a", "name_b", "diff"}}
"""
import argparse
import collections
import difflib
import json
import re
import sys


def parse(path):
    """{method: [(step_name, [dump lines]), ...]} with "= n" back-references expanded."""
    methods = {}
    header = None
    cur = None
    step = None
    with open(path, encoding="utf-8") as f:
        for raw in f:
            line = raw.rstrip("\n")
            if line.startswith("#"):
                if header is None:
                    header = line
                continue
            if line.startswith("=M "):
                cur = methods.setdefault(line[3:], [])
                step = None
                continue
            if line.startswith("@ "):
                parts = line.split(" ", 2)
                name = parts[2] if len(parts) > 2 else ""
                step = (name, [])
                cur.append(step)
                continue
            if step is None:
                continue
            if line.startswith("= ") and not step[1]:
                ref = int(line[2:])
                step[1].extend(cur[ref][1])
                continue
            step[1].append(line)
    return header, methods


def base_name(name):
    name = name.split(" #")[0]
    name = re.sub(r"^(Loop_\d+_)?(Merge_\d+_)?", "", name)
    return re.sub(r"_\d+$", "", name)


def clean(lines, ignores):
    if not ignores:
        return lines
    out = []
    for line in lines:
        for rx in ignores:
            line = rx.sub("", line)
        out.append(line)
    return out


def compare_method(a_steps, b_steps, ignores, prefix=False):
    """None when equal, else (index, name_a, name_b, lines_a, lines_b)."""
    n = min(len(a_steps), len(b_steps)) if prefix else max(len(a_steps), len(b_steps))
    for i in range(n):
        a = a_steps[i] if i < len(a_steps) else None
        b = b_steps[i] if i < len(b_steps) else None
        if a is None or b is None:
            return (i, a[0] if a else None, b[0] if b else None, a[1] if a else [], b[1] if b else [])
        la, lb = clean(a[1], ignores), clean(b[1], ignores)
        if a[0] != b[0] or la != lb:
            return (i, a[0], b[0], la, lb)
    return None


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("a")
    ap.add_argument("b")
    ap.add_argument("--names", nargs=2, default=None)
    ap.add_argument("--filter", default=None)
    ap.add_argument("--context", type=int, default=3)
    ap.add_argument("--max-lines", type=int, default=80)
    ap.add_argument("--quiet", action="store_true")
    ap.add_argument("--prefix", action="store_true")
    ap.add_argument("--ignore", action="append", default=[])
    ap.add_argument("--json", default=None)
    args = ap.parse_args()
    na, nb = args.names or (args.a, args.b)
    ignores = [re.compile(r) for r in args.ignore]

    ha, ma = parse(args.a)
    hb, mb = parse(args.b)
    keys = sorted(set(ma) | set(mb))
    if args.filter:
        keys = [k for k in keys if args.filter in k]

    histogram = collections.Counter()
    counts = collections.Counter()
    result = {}
    out = sys.stdout
    for key in keys:
        if key not in ma or key not in mb:
            side = na if key not in ma else nb
            counts["missing"] += 1
            histogram["(method missing in %s)" % ("A" if key not in ma else "B")] += 1
            out.write(f"MISSING {key}: not traced by {side}\n")
            result[key] = {"status": "missing", "missing_in": side}
            continue
        d = compare_method(ma[key], mb[key], ignores, args.prefix)
        if d is None:
            counts["ok"] += 1
            common = min(len(ma[key]), len(mb[key]))
            if len(ma[key]) == len(mb[key]):
                out.write(f"OK {key}\n")
            else:
                out.write(f"OK-PREFIX {key}: {common} common steps agree\n")
            result[key] = {"status": "ok", "steps": common}
            continue
        i, sa, sb, la, lb = d
        counts["diff"] += 1
        if sa is None or sb is None:
            label = f"{i} {sa or sb} (only in {'A' if sb is None else 'B'})"
            histogram[f"(trace ends early in {'B' if sb is None else 'A'})"] += 1
        elif sa != sb:
            label = f"{i} {sa} | {sb} (step names differ)"
            histogram[f"{base_name(sa)}|{base_name(sb)}"] += 1
        else:
            label = f"{i} {sa}"
            histogram[base_name(sa)] += 1
        out.write(f"DIFF {key}: first divergence at {label}\n")
        diff = list(difflib.unified_diff(la, lb, f"{na} @{i} {sa}", f"{nb} @{i} {sb}", n=args.context,
                                         lineterm=""))
        if not args.quiet:
            shown = diff if args.max_lines <= 0 else diff[:args.max_lines]
            for line in shown:
                out.write("    " + line + "\n")
            if len(shown) < len(diff):
                out.write(f"    ... {len(diff) - len(shown)} more diff lines\n")
        result[key] = {"status": "diff", "step": i, "name_a": sa, "name_b": sb, "diff": diff}

    out.write(f"\n{counts['ok']} OK, {counts['diff']} diverge, {counts['missing']} missing "
              f"(of {len(keys)} methods)\n")
    if histogram:
        out.write("first divergence at: " +
                  ", ".join(f"{k} ×{v}" for k, v in histogram.most_common()) + "\n")
    if args.json:
        with open(args.json, "w", encoding="utf-8") as f:
            json.dump({"a": args.a, "b": args.b, "header_a": ha, "header_b": hb, "methods": result}, f, indent=1)
    return 0 if counts["diff"] == 0 and counts["missing"] == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
