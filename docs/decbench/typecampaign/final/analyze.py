"""Aggregate the Stage-3 re-measure: headline, per-slice, per-GT-class, rivals."""
import collections
import json
import sys
from pathlib import Path

F = Path("/home/mahaloz/kwt/_final")
CLASSES = ["ptr_struct", "ptr_char", "ptr_ptr", "ptr_void", "ptr_prim", "int_s4", "int_u4",
           "int_s8", "int_u8", "char", "bool", "enum", "array", "array_char", "struct_val"]


def load(p):
    return json.loads(Path(p).read_text())


def vals(rows, arm="base"):
    out = {}
    for k, v in rows.items():
        for fn, x in ((v.get(arm) or {}).get("values") or {}).items():
            out[(k, fn)] = x
    return out


def summarize(v):
    n = len(v)
    s = sum(v.values())
    p = sum(1 for x in v.values() if x == 1.0)
    return n, p, s


def slice_groups(key):
    proj, opt, _ = key[0].split("::")
    return ["ALL", f"opt:{opt}", f"proj:{proj}", f"proj:{proj}@{opt}"]


def table(vb, vf):
    g = collections.defaultdict(lambda: [0, 0, 0.0, 0, 0.0])
    for key in set(vb) | set(vf):
        for grp in slice_groups(key):
            r = g[grp]
            if key in vb:
                r[0] += 1
                r[1] += vb[key] == 1.0
                r[2] += vb[key]
            if key in vf:
                r[3] += vf[key] == 1.0
                r[4] += vf[key]
    return g


def classes(decs_iter, keep=None):
    n = collections.Counter()
    tp = collections.Counter()
    fp = collections.Counter()
    st = collections.Counter()
    stp = collections.Counter()
    for slice_key, d in decs_iter:
        if keep is not None and (slice_key, d["fn"]) not in keep:
            continue
        c = d.get("cls")
        n[c] += 1
        tp[c] += d["ok"] is True
        fp[c] += d["ok"] is False
        s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
        st[s] += 1
        stp[s] += d["ok"] is True
    return n, tp, fp, st, stp


def kuna_decs(rows):
    for k, v in rows.items():
        for d in (v.get("decisions") or {}).get("base") or []:
            yield k, d


def rival_decs(riv, name):
    for k, v in riv.items():
        for d in (v.get(name) or {}).get("decisions") or []:
            yield k, d


def main():
    base = load(F / "sweep-base/rows.json")
    fin = load(F / "sweep-final/rows.json")
    riv = load(F / "rivals/rivals.json")
    old = load("/home/mahaloz/kwt/_baseline/rows.json")
    vb, vf, vo = vals(base), vals(fin), vals(old)
    rep = {}
    # control: re-run of the pinned baseline binary vs the 2026-09-16 capture
    common = set(vb) & set(vo)
    rep["control_rerun_vs_0916"] = {
        "rerun_functions": len(vb), "0916_functions": len(vo), "common": len(common),
        "only_rerun": len(set(vb) - set(vo)), "only_0916": len(set(vo) - set(vb)),
        "value_differs_on_common": sum(1 for k in common if abs(vb[k] - vo[k]) > 1e-9),
        "0916": summarize(vo), "rerun": summarize(vb),
    }
    both = set(vb) & set(vf)
    onto = sum(1 for k in both if vf[k] == 1.0 and vb[k] != 1.0)
    offp = sum(1 for k in both if vb[k] == 1.0 and vf[k] != 1.0)
    imp = sum(1 for k in both if vf[k] > vb[k] + 1e-9)
    wor = sum(1 for k in both if vf[k] < vb[k] - 1e-9)
    rep["moves"] = {"functions_both": len(both), "improved_any": imp, "worse_any": wor,
                    "onto_perfect": onto, "off_perfect": offp,
                    "only_base": len(set(vb) - set(vf)), "only_final": len(set(vf) - set(vb))}
    t = table(vb, vf)
    rep["slices"] = {k: {"n": r[0], "perfect_base": r[1], "mean_base": round(r[2] / max(1, r[0]), 4),
                         "perfect_final": r[3], "mean_final": round(r[4] / max(1, r[0]), 4)}
                     for k, r in sorted(t.items())}
    # per-project improved/worse
    pw = collections.defaultdict(lambda: [0, 0])
    for k in both:
        proj = k[0].split("::")[0]
        pw[proj][0] += vf[k] > vb[k] + 1e-9
        pw[proj][1] += vf[k] < vb[k] - 1e-9
    rep["per_project_moves"] = {p: {"improved": a, "worse": b} for p, (a, b) in sorted(pw.items())}
    # worst regressions list
    worst = sorted(((vf[k] - vb[k], k) for k in both if vf[k] < vb[k] - 1e-9))[:40]
    rep["worst"] = [[round(d, 4), k[0], k[1], vb[k], vf[k]] for d, k in worst]
    # classes: kuna base/final over their own scored sets (identical fn sets expected)
    keep = both
    out = {}
    for name, it in (("kuna_base", kuna_decs(base)), ("kuna_final", kuna_decs(fin))):
        out[name] = classes(it, keep)
    kept_rival = {}
    for r in ("ida", "binja", "ghidra", "angr", "kuna"):
        rv = {}
        for k, v in riv.items():
            for fn, x in ((v.get(r) or {}).get("values") or {}).items():
                rv[(k, fn)] = x
        kept_rival[r] = rv
        out[f"{r}_tree_own"] = classes(rival_decs(riv, r))
        out[f"{r}_tree_common"] = classes(rival_decs(riv, r), keep & set(rv))
    rep["classes"] = {}
    for name, (n, tp, fp, st, stp) in out.items():
        rep["classes"][name] = {
            "n_total": sum(n.values()), "tp_total": sum(tp.values()),
            "per_class": {c: {"n": n[c], "tp": tp[c], "fp": fp[c],
                              "rate": round(100.0 * tp[c] / max(1, n[c]), 1)} for c in n},
            "storage": {s: {"n": st[s], "tp": stp[s], "rate": round(100.0 * stp[s] / max(1, st[s]), 1)}
                        for s in st},
        }
    # rival headline: own sets, and on the kuna-scored common set
    rep["rivals"] = {}
    for r, rv in kept_rival.items():
        com = set(rv) & both
        rep["rivals"][r] = {"own": summarize(rv), "common": summarize({k: rv[k] for k in com}),
                            "kuna_final_on_common": summarize({k: vf[k] for k in com}),
                            "kuna_base_on_common": summarize({k: vb[k] for k in com}),
                            "by_opt_own": {}}
        for opt in ("O0", "O2", "O2-noinline"):
            sub = {k: x for k, x in rv.items() if k[0].split("::")[1] == opt}
            subc = {k: x for k, x in sub.items() if k in both}
            rep["rivals"][r]["by_opt_own"][opt] = {"own": summarize(sub), "common": summarize(subc),
                                                   "kuna_final_common": summarize({k: vf[k] for k in subc})}
    (F / "analysis.json").write_text(json.dumps(rep, indent=1))
    print(json.dumps({k: rep[k] for k in ("control_rerun_vs_0916", "moves")}, indent=1))


if __name__ == "__main__":
    main()
