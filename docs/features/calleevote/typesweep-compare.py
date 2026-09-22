"""cmp.py A.json B.json [--worse N] [--opt O2]: pinned + #93-credited comparison of two finalsweep rows files."""
import json, sys, collections, re
PRIM = {"char", "signed char", "unsigned char", "short", "unsigned short", "int",
        "unsigned int", "long", "unsigned long", "long long", "unsigned long long",
        "float", "double", "long double", "void", "bool", "_Bool", "code", "size_t",
        "wchar_t", "undefined", "undefined1", "undefined2", "undefined4", "undefined8",
        "undefined16", "int1", "int2", "int4", "int8", "int16",
        "uint1", "uint2", "uint4", "uint8", "uint16"}
def is_struct_ptr(t):
    if not t: return False
    s = t.strip()
    if not s.endswith("*") or s.count("*") != 1: return False
    base = s[:-1].strip()
    if base.startswith("struct "): base = base[7:].strip()
    return bool(base) and base not in PRIM
def per_fn(rows, opts):
    out = {}
    for k, v in rows.items():
        if opts and k.split("::")[1] not in opts: continue
        vals = v.get("base", {}).get("values", {})
        ds = (v.get("decisions") or {}).get("base") or []
        by = collections.defaultdict(list)
        for d in ds: by[d["fn"]].append(d)
        for fn, val in vals.items():
            dd = by.get(fn, [])
            n = len(dd)
            tp = sum(1 for d in dd if d["ok"] is True)
            cr = sum(1 for d in dd if d["ok"] is False and d["cls"] == "ptr_struct" and is_struct_ptr(d["kuna_type"]))
            out[(k, fn)] = (val, n, tp, cr, dd)
    return out
def main():
    a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
    opts = [x.split("=")[1] for x in sys.argv if x.startswith("--opt=")]
    A = per_fn(a, opts); B = per_fn(b, opts)
    keys = sorted(set(A) & set(B))
    res = collections.Counter()
    worse, better = [], []
    cls = collections.Counter()
    for key in keys:
        va, na, tpa, cra, da = A[key]; vb, nb, tpb, crb, db = B[key]
        opt = key[0].split("::")[1]
        res["n"] += 1
        res["perf_a"] += va == 1.0; res["perf_b"] += vb == 1.0
        res["agg_a"] += va; res["agg_b"] += vb
        if na:
            ca = min(1.0, (tpa + cra) / na); cb = min(1.0, (tpb + crb) / nb) if nb else vb
            res["cperf_a"] += ca == 1.0; res["cperf_b"] += cb == 1.0
            res["cagg_a"] += ca; res["cagg_b"] += cb
            if cb > ca + 1e-9: res["c_up"] += 1
            if cb < ca - 1e-9: res["c_down"] += 1
        res[f"perf_a_{opt}"] += va == 1.0; res[f"perf_b_{opt}"] += vb == 1.0
        if vb > va + 1e-9: res["up"] += 1; better.append((key, va, vb))
        if vb < va - 1e-9: res["down"] += 1; worse.append((key, va, vb, da, db))
        for d in da: cls[(d["cls"], "a")] += d["ok"] is True
        for d in db: cls[(d["cls"], "b")] += d["ok"] is True
        # decomp var count (arity/phantom proxy)
        res["nvars_a"] += a[key[0]]["base"]["nvars"].get(key[1], 0)
        res["nvars_b"] += b[key[0]]["base"]["nvars"].get(key[1], 0)
    for k in ("n", "perf_a", "perf_b", "up", "down", "cperf_a", "cperf_b", "c_up", "c_down", "nvars_a", "nvars_b"):
        print(f"{k:10s} {res[k]}")
    print(f"agg        {res['agg_a']:.2f} -> {res['agg_b']:.2f}   credited {res['cagg_a']:.2f} -> {res['cagg_b']:.2f}")
    for o in ("O0", "O2", "O2-noinline"):
        if res[f"perf_a_{o}"] or res[f"perf_b_{o}"]:
            print(f"perfect {o}: {res[f'perf_a_{o}']} -> {res[f'perf_b_{o}']}")
    print("class TP a->b:", {c: (cls[(c, 'a')], cls[(c, 'b')]) for c in sorted({c for c, _ in cls}) if cls[(c, 'a')] != cls[(c, 'b')]})
    N = int(next((x.split("=")[1] for x in sys.argv if x.startswith("--worse=")), "0"))
    for key, va, vb, da, db in worse[:N]:
        ch = [(x["gt_name"], x["gt_type"], x["kuna_type"], y["kuna_type"]) for x, y in zip(da, db) if x["kuna_type"] != y["kuna_type"] or x["ok"] != y["ok"]]
        print("WORSE", key, round(va, 3), round(vb, 3), ch[:4])
main()
