"""Per-GT-variable diff of two sweep arms for named functions."""
import json, sys
from pathlib import Path
A = json.loads(Path(sys.argv[1]).read_text())
B = json.loads(Path(sys.argv[2]).read_text())
want = [tuple(x.split("|")) for x in sys.argv[3:]]
def decs(rows, slc, fn):
    out = {}
    for d in (rows.get(slc, {}).get("decisions") or {}).get("base") or []:
        if d["fn"] == fn:
            out[(d.get("gt_name"), d.get("gt_type"), d.get("cls"))] = (d["ok"], d.get("kuna_type"))
    return out
for slc, fn in want:
    a, b = decs(A, slc, fn), decs(B, slc, fn)
    print(f"== {slc} {fn}")
    for k in sorted(set(a) | set(b), key=str):
        x, y = a.get(k), b.get(k)
        if x != y:
            print(f"   GT {k[0]!r} : {k[1]!r} [{k[2]}]   A {x}   ->  B {y}")
