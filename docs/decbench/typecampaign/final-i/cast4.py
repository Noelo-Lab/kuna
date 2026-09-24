"""Cast table over several castbench arms on ONE shared set (every arm ∩ IDA), per opt level and pairwise moves.

  cast4.py LABEL=DIR ... [--pairs A:B,...] [--top N]
Reuses castbench.funcs/corpus (the census counter); never writes into the decbench tree.
"""
import collections, json, sys
from pathlib import Path
sys.path.insert(0, "/home/mahaloz/kwt/castbench")
import castbench as CB

args = [a for a in sys.argv[1:] if "=" in a and not a.startswith("--")]
pairs = next((a.split("=", 1)[1] for a in sys.argv[1:] if a.startswith("--pairs=")), "")
top = int(next((a.split("=", 1)[1] for a in sys.argv[1:] if a.startswith("--top=")), "25"))
ARMS = [a.split("=", 1) for a in args]
labs = [l for l, _ in ARMS]
tot = collections.defaultdict(lambda: [0, 0, 0])
per_opt = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0]))
per_fn = {}
shapes = collections.defaultdict(collections.Counter)
n = 0
nopt = collections.Counter()
for opt, proj, b in CB.corpus("full"):
    ida = CB.funcs(CB.RES / opt / proj / "decompiled" / f"ida_{b}.c")
    ks = {l: CB.funcs(Path(d) / opt / proj / f"{b}.c") for l, d in ARMS}
    if not ida or any(not v for v in ks.values()):
        print("SKIP", opt, proj, b, file=sys.stderr)
        continue
    common = set(ida)
    for v in ks.values():
        common &= set(v)
    for a in sorted(common):
        n += 1
        nopt[opt] += 1
        for tag, r in [("ida", ida[a])] + [(l, ks[l][a]) for l in labs]:
            for j in range(3):
                tot[tag][j] += r[j]
                per_opt[opt][tag][j] += r[j]
            if tag != "ida":
                shapes[tag].update(r[3])
        per_fn[(opt, b, a)] = {l: ks[l][a][2] for l in labs} | {"ida": ida[a][2]}


def line(tag, r, ref):
    c = r[2]
    return {"casts": c, "lines": r[0], "stmts": r[1], "per_kloc": round(1000 * c / max(r[0], 1), 1),
            "per_100stmt": round(100 * c / max(r[1], 1), 2), "vs_ida": round(c / max(ref[2], 1), 3)}


rep = {"shared_functions": n, "per_opt_functions": dict(nopt),
       "all": {t: line(t, tot[t], tot["ida"]) for t in ["ida"] + labs},
       "per_opt": {o: {t: line(t, per_opt[o][t], per_opt[o]["ida"]) for t in ["ida"] + labs} for o in sorted(per_opt)}}
# functions where kuna casts less than / equal to / more than IDA, per arm
rep["vs_ida_fn"] = {}
for l in labs:
    lt = sum(1 for v in per_fn.values() if v[l] < v["ida"])
    eq = sum(1 for v in per_fn.values() if v[l] == v["ida"])
    gt = sum(1 for v in per_fn.values() if v[l] > v["ida"])
    rep["vs_ida_fn"][l] = {"fewer_than_ida": lt, "equal": eq, "more_than_ida": gt}
rep["moves"] = {}
for p in [x for x in pairs.split(",") if x]:
    a, b = p.split(":")
    up = sorted(((v[b] - v[a], k) for k, v in per_fn.items() if v[b] > v[a]), reverse=True)
    dn = sorted(((v[b] - v[a], k) for k, v in per_fn.items() if v[b] < v[a]))
    sd = shapes[b].copy(); sd.subtract(shapes[a])
    rep["moves"][p] = {"fewer": len(dn), "fewer_casts": -sum(x[0] for x in dn), "more": len(up),
                       "more_casts": sum(x[0] for x in up),
                       "top_more": [[d, o, bb, hex(ad)] for d, (o, bb, ad) in up[:top]],
                       "top_fewer": [[d, o, bb, hex(ad)] for d, (o, bb, ad) in dn[:top]],
                       "shape_down": [[c, t, s] for (t, s), c in sorted(sd.items(), key=lambda x: x[1])[:15] if c < 0],
                       "shape_up": [[c, t, s] for (t, s), c in sorted(sd.items(), key=lambda x: -x[1])[:15] if c > 0]}
print(json.dumps(rep, indent=1))
