"""The ptr_char gap census behind next-levers.md.

Joins kuna's per-ground-truth-variable decisions (a typesweep `--dump-decisions`
rows.json) against binja's, ida's and ghidra's for the same variables, and splits
the ptr_char class by storage (argument / stack / register-only), by what kuna
spells instead, and by optimisation level.

    python3 gapcensus.py <rows.json> [arm] [out.json]

RIVALS is the round-A rival replay (finalsweep rivals); CLSREF is any finalsweep
kuna rows.json, used only for its per-variable `cls` (the ground-truth class),
which typesweep rows do not carry.
"""
import collections, json, os, re, sys

KROWS = sys.argv[1]
ARM = sys.argv[2] if len(sys.argv) > 2 else "base"
RIV = os.environ.get("RIVALS", "/home/mahaloz/kwt/_final/rivals/rivals.json")
CLSREF = os.environ.get("CLSREF", "/home/mahaloz/kwt/_final-d/sweep-d/rows.json")

def key(sk, d):
    return (sk, d["fn"], d.get("gt_name"), d.get("gt_is_arg"), d.get("gt_has_off"), d.get("gt_type"))

def keyed(items):
    out, seen = {}, collections.Counter()
    for sk, d in items:
        k = key(sk, d); i = seen[k]; seen[k] += 1
        out[k + (i,)] = d
    return out

def storage(d):
    return "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")

INT = re.compile(r"^(?:(?:unsigned|signed)\s+)?(?:long long|long|int|short|char|u?int\d+(?:_t)?|size_t|ssize_t|bool|_Bool|uint|ulong|ushort)$")
def spell(t):
    if t is None: return "none"
    t = t.strip()
    if re.match(r"^undefined\d*$", t): return "undefined(width only)"
    if t.endswith("*"):
        base = re.sub(r"\s*\*$", "", t).replace("const ", "").strip()
        if re.sub(r"^(unsigned|signed)\s+", "", base) in ("char", "uint1", "int1", "uint8_t", "int8_t", "uchar"): return "char * (ok)"
        if base == "void": return "void *"
        if re.match(r"^struct_\d+$", base) or base.startswith("struct "): return "struct_N *"
        if re.match(r"^undefined\d*$", base): return "undefinedN *"
        if base.endswith("*"): return "T ** (pointer to pointer)"
        if INT.match(base): return "int *"
        return "named T * (libc record etc.)"
    if INT.match(t): return "integer"
    if "[" in t: return "array"
    if t in ("float", "double", "long double"): return "float"
    return "other:" + t

kr = json.load(open(KROWS))
riv = json.load(open(RIV))
clsref = json.load(open(CLSREF))
cls_by_key = keyed((sk, d) for sk, v in clsref.items() for d in (v.get("decisions") or {}).get("base") or [])
kun = keyed((sk, d) for sk, v in kr.items() for d in ((v.get("decisions") or {}).get(ARM) or []))
rv = {r: keyed((sk, d) for sk, v in riv.items() for d in (v.get(r) or {}).get("decisions") or []) for r in ("binja", "ida", "ghidra")}

UNJOINED = collections.Counter()
def is_pc(k, d):
    c = d.get("cls")
    if c is None:
        j = cls_by_key.get(k)
        if j is None:
            UNJOINED[d.get("gt_type") in ("char*", "Char*", "UChar*", "cc_t*")] += 1
            return d.get("gt_type") in ("char*", "Char*", "UChar*", "cc_t*")
        c = j.get("cls")
    return c == "ptr_char"

res = {}
pcs = {k: d for k, d in kun.items() if is_pc(k, d)}
res["n_ptr_char_rows_kuna"] = len(pcs)
res["unjoined_rows(charlike?)"] = {str(a): n for a, n in UNJOINED.items()}
res["kuna_tp"] = sum(d["ok"] is True for d in pcs.values())
gtt = collections.Counter(d["gt_type"] for d in pcs.values())
res["gt_type"] = dict(gtt)
# rival own totals on ptr_char
for r in rv:
    rows = [d for d in rv[r].values() if d.get("cls") == "ptr_char"]
    res[f"{r}_own"] = {"n": len(rows), "tp": sum(d["ok"] is True for d in rows)}
# joint table vs binja on common keys
B = rv["binja"]
com = [k for k in pcs if k in B]
res["common_with_binja"] = len(com)
joint = collections.Counter()
for k in com:
    joint[(pcs[k]["ok"] is True, B[k]["ok"] is True)] += 1
res["joint"] = {f"kuna={'T' if a else 'F'},binja={'T' if b else 'F'}": n for (a, b), n in joint.items()}
# storage x outcome
st = collections.defaultdict(collections.Counter)
for k in com:
    s = storage(pcs[k]); a = pcs[k]["ok"] is True; b = B[k]["ok"] is True
    st[s]["n"] += 1; st[s]["kuna"] += a; st[s]["binja"] += b
    st[s]["binja_only"] += (b and not a); st[s]["kuna_only"] += (a and not b)
    st[s]["gt_void"] += pcs[k]["gt_type"] == "void"
res["storage"] = {s: dict(c) for s, c in st.items()}
# kuna misses: spelling bucket x storage, split by binja outcome
miss = collections.defaultdict(collections.Counter)
gap = collections.defaultdict(collections.Counter)
gapb = collections.defaultdict(collections.Counter)
for k in com:
    d = pcs[k]
    if d["ok"] is True or d["gt_type"] == "void":
        continue
    s = storage(d); sp = spell(d.get("kuna_type"))
    miss[s][sp] += 1
    if B[k]["ok"] is True:
        gap[s][sp] += 1
        gapb[s][B[k].get("kuna_type")] += 1
res["kuna_miss_spelling_by_storage(matchable GT)"] = {s: dict(c.most_common()) for s, c in miss.items()}
res["binja_TP_kuna_miss_spelling_by_storage"] = {s: dict(c.most_common()) for s, c in gap.items()}
res["binja_spelling_on_gap"] = {s: dict(c.most_common(8)) for s, c in gapb.items()}
# kuna TP binja miss: binja spelling
kb = collections.Counter(spell(B[k].get("kuna_type")) for k in com if pcs[k]["ok"] is True and B[k]["ok"] is not True)
res["kuna_TP_binja_miss_binja_spelling"] = dict(kb.most_common())
# by opt level
byopt = collections.defaultdict(collections.Counter)
for k in com:
    o = k[0].split("::")[1]; a = pcs[k]["ok"] is True; b = B[k]["ok"] is True
    byopt[o]["n"] += 1; byopt[o]["kuna"] += a; byopt[o]["binja"] += b; byopt[o]["binja_only"] += b and not a; byopt[o]["kuna_only"] += a and not b
res["by_opt"] = {o: dict(c) for o, c in byopt.items()}
# pass (how kuna matched it)
ps = collections.Counter((storage(pcs[k]), pcs[k].get("pass")) for k in com if pcs[k]["ok"] is not True and B[k]["ok"] is True)
res["gap_by_match_pass"] = {f"{a}/{b}": n for (a, b), n in ps.most_common()}
json.dump(res, open(sys.argv[3] if len(sys.argv) > 3 else "/dev/stdout", "w"), indent=1)
