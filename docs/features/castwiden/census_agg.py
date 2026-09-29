import sys, collections, glob, os
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB
cdir = Path(sys.argv[1])  # the census_run.py output directory; base = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("/home/mahaloz/kwt/castbench/main-0096e984d")
shared = {}
for opt, proj, b in CB.corpus("full"):
    k = CB.funcs(base / opt / proj / f"{b}.c"); i = CB.funcs(CB.RES / opt / proj / "decompiled" / f"ida_{b}.c")
    if k and i: shared[(opt, b)] = set(k) & set(i)
rows = []
for f in sorted(cdir.glob("*.tsv")):
    opt, proj, b = f.stem.split("__")
    s = shared.get((opt, b), set())
    for l in f.read_text().splitlines():
        if not l: continue
        p = l.split("\t")
        fa = int(p[1], 16)
        if fa not in s: continue
        rows.append((opt, b, p))
def ctxclass(p):
    r = p[9]
    if r == "TOP": return "TOP " + (p[10] if len(p) > 10 else "") + " " + (p[11] if len(p) > 11 else "")
    if r in ("INT_ADD","INT_SUB"): return "addsub"
    if r in ("INT_EQUAL","INT_NOTEQUAL","INT_LESS","INT_SLESS","INT_LESSEQUAL","INT_SLESSEQUAL"): return "compare"
    if r in ("INT_LEFT","INT_RIGHT","INT_SRIGHT"): return f"shift slot{p[10]}"
    if r in ("INT_AND","INT_OR","INT_XOR"): return "bitwise"
    if r in ("INT_DIV","INT_SDIV","INT_REM","INT_SREM"): return "divrem"
    return r
tot = collections.Counter(); byctx = collections.Counter(); byct = collections.Counter()
for opt, b, p in rows:
    kind, how = p[3], p[4]
    tot[(kind, how)] += 1
    if how == "PRINT":
        byctx[(kind, ctxclass(p))] += 1
print("rows (shared set):", len(rows))
for k, n in tot.most_common(): print(f"  {n:6}  {k}")
print("PRINTED by (kind, context):")
for k, n in byctx.most_common(60): print(f"  {n:6}  {k}")

def other_class(p):
    o = [x for x in p[10:] if x.startswith("other=")]
    if not o: return "-"
    o = o[0][6:]
    if o.startswith("const:"):
        sz, v = o.split(":")[1:3]
        return f"const{sz}"
    parts = o.split(":")
    return f"{parts[0]}:{parts[1]}:{':'.join(parts[2:])}"
det = collections.Counter()
for opt, b, p in rows:
    kind, how = p[3], p[4]
    if how != "PRINT" or kind not in ("SEXT", "ZEXT"): continue
    c = ctxclass(p)
    if c in ("addsub", "INT_MULT", "bitwise", "divrem", "compare") or c.startswith("shift"):
        det[(kind, c, p[5], p[7], other_class(p), [x for x in p[10:] if x.startswith("out=")][0])] += 1
    elif c == "CALL" or c == "STORE" or c.startswith("TOP") or c in ("COPY", "RETURN", "CAST", "INT_2COMP"):
        det[(kind, c, p[5], p[7], "|".join(p[10:]))] += 1
print("DETAIL (kind, ctx, target, srcCtype, other/rest):")
for k, n in det.most_common(90): print(f"  {n:5}  {k}")
