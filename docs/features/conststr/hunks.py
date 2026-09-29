#!/usr/bin/env python3
"""Classify every line `conststr` changes between two castbench-format arms.

  hunks.py OFF_ARM ON_ARM [--json OUT]

Each changed line must be one of:
  subst   -- the old line with `(T *)0x<a>` replaced by `&dat_<a>` (or by the
             array name `dat_<a>` where the constant was indexed, `((T *)0x<a>)[i]`)
             and nothing else;
  literal -- the old line with a pointer to read-only bytes (`&dat_<a>` or
             `(T *)0x<a>`) replaced by a string literal;
anything else is OTHER and is printed (a bug by the option's documented effect).
"""
import collections, difflib, json, re, sys
from pathlib import Path

off, on = Path(sys.argv[1]), Path(sys.argv[2])
CAST = r"\((?:[\w ]+?)\s*\*+\)0x([0-9a-f]+)"
LIT = r'"(?:[^"\\]|\\.)*"(?:\s*"(?:[^"\\]|\\.)*")*'


def norm_subst(a):
    a = re.sub(r"\(" + CAST + r"\)\[", r"dat_\1[", a)
    a = re.sub(r"&\(" + CAST + r"\)\[", r"&dat_\1[", a)
    return re.sub(CAST, r"&dat_\1", a)


def is_subst(a, b):
    x = norm_subst(a)
    y = re.sub(r"(?<![&\w])dat_([0-9a-f]+)(?![\[\w])", r"&dat_\1", b)
    return x == b or x == y or norm_subst(a) == norm_subst(b)


def is_literal(a, b):
    pa = re.split(r"&dat_[0-9a-f]+|\((?:[\w ]+?)\s*\*+\)0x[0-9a-f]+", a)
    pb = re.split(LIT, b)
    return len(pa) == len(pb) and pa == pb


counts, other, per_bin = collections.Counter(), [], collections.Counter()
for f in sorted(off.rglob("*.c")):
    g = on / f.relative_to(off)
    A, B = f.read_text(errors="replace").splitlines(), g.read_text(errors="replace").splitlines()
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, A, B, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        if tag != "replace" or i2 - i1 != j2 - j1:
            other.append((str(f.relative_to(off)), tag, A[i1:i2], B[j1:j2]))
            counts["OTHER"] += max(i2 - i1, j2 - j1)
            continue
        for a, b in zip(A[i1:i2], B[j1:j2]):
            k = "subst" if is_subst(a, b) else "literal" if is_literal(a, b) else "OTHER"
            counts[k] += 1
            per_bin[str(f.relative_to(off))] += 1
            if k == "OTHER":
                other.append((str(f.relative_to(off)), "line", [a], [b]))
print(dict(counts), "binaries changed:", len(per_bin))
for o in other:
    print("OTHER", o)
if "--json" in sys.argv:
    json.dump({"counts": dict(counts), "binaries_changed": len(per_bin), "per_binary": dict(per_bin),
               "other": other}, open(sys.argv[sys.argv.index("--json") + 1], "w"), indent=1)
