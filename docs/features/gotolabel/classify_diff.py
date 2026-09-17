#!/usr/bin/env python3
"""Per-function before/after classification of two decompile-all output trees.

Usage: classify_diff.py <before_dir> <after_dir>
Reports every function whose text changed, what changed (added label lines vs
other), and whether that function carried an orphan goto in the BEFORE text.
"""
import sys, os, glob, difflib, re

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import importlib.util
spec = importlib.util.spec_from_file_location("orphan_scan", os.path.join(os.path.dirname(os.path.abspath(__file__)), "orphan_scan.py"))
osc = importlib.util.module_from_spec(spec); spec.loader.exec_module(osc)

LABEL_LINE = re.compile(r'^\s*[A-Za-z_]\w*\s*:(?!:)\s*$')

def load(path):
    text = open(path, errors='replace').read()
    return {f"{n}@{a}": b for n, a, b in osc.functions(text)}

def orphans_of(body):
    labels = {m.group(1) for ln in body for m in [osc.LABEL.match(ln)] if m}
    gotos = {g.group(1) for ln in body for g in osc.GOTO.finditer(ln)}
    return gotos - labels - {'break', 'continue'}

def main():
    bdir, adir = sys.argv[1], sys.argv[2]
    tot_funcs = tot_changed = tot_label_only = tot_other = 0
    other_rows = []
    for bf in sorted(glob.glob(os.path.join(bdir, '*.c'))):
        af = os.path.join(adir, os.path.basename(bf))
        if not os.path.exists(af):
            print("MISSING", af); continue
        B, A = load(bf), load(af)
        assert set(B) == set(A), f"function sets differ in {bf}: {set(B) ^ set(A)}"
        for key in B:
            tot_funcs += 1
            if B[key] == A[key]:
                continue
            tot_changed += 1
            had_orphan = bool(orphans_of(B[key]))
            diff = list(difflib.unified_diff(B[key], A[key], lineterm='', n=0))
            added = [l[1:] for l in diff if l.startswith('+') and not l.startswith('+++')]
            removed = [l[1:] for l in diff if l.startswith('-') and not l.startswith('---')]
            label_only = (not removed) and added and all(LABEL_LINE.match(l) for l in added)
            if label_only and had_orphan:
                tot_label_only += 1
            else:
                tot_other += 1
                other_rows.append((os.path.basename(bf), key, had_orphan, len(added), len(removed), added[:4], removed[:4]))
    print(f"functions={tot_funcs} changed={tot_changed} "
          f"added-label-only-in-orphan-function={tot_label_only} other={tot_other}")
    for r in other_rows:
        print("OTHER", r[0], r[1], "had_orphan=" + str(r[2]), f"+{r[3]}/-{r[4]}")
        for l in r[5]:
            print("   +", l)
        for l in r[6]:
            print("   -", l)

if __name__ == '__main__':
    main()
