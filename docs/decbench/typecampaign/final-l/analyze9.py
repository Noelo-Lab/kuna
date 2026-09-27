"""Rounds J-L aggregate: baseline vs rounds B-I vs rounds J, K and L (final), per slice, per GT class, rivals.

analyze8.py with the round-J/K/L arms added and the round-I -> round-L comparisons in place of G -> I."""
import collections
import json
from pathlib import Path

OUT = Path("/home/mahaloz/kwt/_final-l")
I = Path("/home/mahaloz/kwt/_final-i")
G = Path("/home/mahaloz/kwt/_final-g")
F = Path("/home/mahaloz/kwt/_final-f")
E = Path("/home/mahaloz/kwt/_final-e")
D = Path("/home/mahaloz/kwt/_final-d")
C = Path("/home/mahaloz/kwt/_final-c")
B = Path("/home/mahaloz/kwt/_final")
SETS = {"base": B / "sweep-base/rows.json", "roundB": C / "sweep-bctl/rows.json",
        "roundC": C / "sweep-c/rows.json", "roundD": D / "sweep-d/rows.json",
        "roundE": E / "sweep-e/rows.json", "roundF": F / "sweep-f/rows.json",
        "roundG": G / "sweep-g/rows.json", "roundH": I / "sweep-h/rows.json",
        "roundI": I / "sweep-i/rows.json",
        "roundJ": OUT / "sweep-j/rows.json", "roundK": OUT / "sweep-k/rows.json",
        "roundL": OUT / "sweep-l/rows.json",
        "roundIctl": OUT / "sweep-ictl/rows.json",
        "L0": OUT / "sweep-l0/rows.json", "L1": OUT / "sweep-l1/rows.json", "L2": OUT / "sweep-l2/rows.json",
        "L3": OUT / "sweep-l3/rows.json", "L4": OUT / "sweep-l4/rows.json"}
ARMS = ["base", "roundB", "roundC", "roundD", "roundE", "roundF", "roundG", "roundH", "roundI", "roundJ", "roundK", "roundL"]


def vals(rows):
    out = {}
    for k, v in rows.items():
        for fn, x in ((v.get("base") or {}).get("values") or {}).items():
            out[(k, fn)] = x
    return out


def summarize(v):
    n = len(v)
    return {"n": n, "perfect": sum(1 for x in v.values() if x == 1.0),
            "perfect_pct": round(100.0 * sum(1 for x in v.values() if x == 1.0) / max(1, n), 2),
            "sum": round(sum(v.values()), 2),
            "mean": round(sum(v.values()) / max(1, n), 4)}


def groups(key):
    proj, opt, _ = key[0].split("::")
    return ["ALL", f"opt:{opt}", f"proj:{proj}", f"proj:{proj}@{opt}"]


def classes(decs, keep=None):
    n, tp, st, stp = (collections.Counter() for _ in range(4))
    for sk, d in decs:
        if keep is not None and (sk, d["fn"]) not in keep:
            continue
        n[d.get("cls")] += 1
        tp[d.get("cls")] += d["ok"] is True
        s = "arg" if d["gt_is_arg"] else ("stack" if d["gt_has_off"] else "reg")
        st[s] += 1
        stp[s] += d["ok"] is True
    return n, tp, st, stp


def kuna_decs(rows):
    for k, v in rows.items():
        for d in (v.get("decisions") or {}).get("base") or []:
            yield k, d


def rival_decs(riv, name):
    for k, v in riv.items():
        for d in (v.get(name) or {}).get("decisions") or []:
            yield k, d


def main():
    rows = {k: json.loads(p.read_text()) for k, p in SETS.items() if p.exists()}
    V = {k: vals(r) for k, r in rows.items()}
    rep = {"headline": {k: summarize(v) for k, v in V.items()}}
    for a, b in (("roundI", "roundIctl"), ("roundI", "roundJ"), ("roundJ", "roundK"), ("roundK", "L0"),
                 ("L0", "L1"), ("L1", "L2"), ("L2", "L3"), ("L3", "L4"), ("L4", "roundL"),
                 ("roundK", "roundL"), ("roundI", "roundL"), ("roundG", "roundL"), ("base", "roundL")):
        if a not in V or b not in V:
            continue
        s = set(V[a]) & set(V[b])
        rep[f"moves_{a}_to_{b}"] = {
            "functions": len(s),
            "improved": sum(1 for k in s if V[b][k] > V[a][k] + 1e-9),
            "worse": sum(1 for k in s if V[b][k] < V[a][k] - 1e-9),
            "onto_perfect": sum(1 for k in s if V[b][k] == 1.0 and V[a][k] != 1.0),
            "off_perfect": sum(1 for k in s if V[a][k] == 1.0 and V[b][k] != 1.0)}
    g = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0.0]))
    allk = set()
    for lab in ARMS:
        allk |= set(V[lab])
    for key in allk:
        for grp in groups(key):
            for lab in ARMS:
                if key in V[lab]:
                    r = g[grp][lab]
                    r[0] += 1
                    r[1] += V[lab][key] == 1.0
                    r[2] += V[lab][key]
    rep["slices"] = {grp: {lab: {"n": r[0], "perfect": r[1], "mean": round(r[2] / max(1, r[0]), 4)}
                           for lab, r in d.items()} for grp, d in sorted(g.items())}
    both = set(V["roundI"]) & set(V["roundL"])
    pw = collections.defaultdict(lambda: [0, 0])
    for k in both:
        p = k[0].split("::")[0]
        pw[p][0] += V["roundL"][k] > V["roundI"][k] + 1e-9
        pw[p][1] += V["roundL"][k] < V["roundI"][k] - 1e-9
    rep["per_project_moves_I_to_L"] = {p: {"improved": a, "worse": b} for p, (a, b) in sorted(pw.items())}
    rep["worse_I_to_L"] = [[round(V["roundL"][k] - V["roundI"][k], 4), k[0], k[1],
                            round(V["roundI"][k], 4), round(V["roundL"][k], 4)]
                           for k in sorted(both, key=lambda k: V["roundL"][k] - V["roundI"][k])
                           if V["roundL"][k] < V["roundI"][k] - 1e-9]
    basek = set(V["base"]) & set(V["roundL"])
    rep["worse_base_to_L"] = [[round(V["roundL"][k] - V["base"][k], 4), k[0], k[1],
                               round(V["base"][k], 4), round(V["roundL"][k], 4)]
                              for k in sorted(basek, key=lambda k: V["roundL"][k] - V["base"][k])
                              if V["roundL"][k] < V["base"][k] - 1e-9]
    rep["best_I_to_L"] = [[round(V["roundL"][k] - V["roundI"][k], 4), k[0], k[1]]
                          for k in sorted(both, key=lambda k: V["roundI"][k] - V["roundL"][k])[:40]]
    rep["onto_perfect_I_to_L"] = [[k[0], k[1], round(V["roundI"][k], 4)] for k in sorted(both)
                                  if V["roundL"][k] == 1.0 and V["roundI"][k] != 1.0]
    riv = json.loads((B / "rivals/rivals.json").read_text())
    out = {}
    keep = both
    for lab in ARMS:
        if lab in rows:
            out[f"kuna_{lab}"] = classes(kuna_decs(rows[lab]), keep)
    rv_vals = {}
    for r in ("ida", "binja", "ghidra", "angr"):
        rv = {}
        for k, v in riv.items():
            for fn, x in ((v.get(r) or {}).get("values") or {}).items():
                rv[(k, fn)] = x
        rv_vals[r] = rv
        out[f"{r}_own"] = classes(rival_decs(riv, r))
    rep["classes"] = {}
    for name, (n, tp, st, stp) in out.items():
        rep["classes"][name] = {
            "n_total": sum(n.values()), "tp_total": sum(tp.values()),
            "per_class": {c: {"n": n[c], "tp": tp[c], "rate": round(100.0 * tp[c] / max(1, n[c]), 1)}
                          for c in n},
            "storage": {s: {"n": st[s], "tp": stp[s], "rate": round(100.0 * stp[s] / max(1, st[s]), 1)}
                        for s in st}}
    # ptr_struct TP by GT type name, round F -> round G
    def by_name(rowset, cls="ptr_struct"):
        c = collections.Counter()
        for k, d in kuna_decs(rowset):
            if (k, d["fn"]) in keep and d.get("cls") == cls and d["ok"] is True:
                c[d.get("gt_type")] += 1
        return c
    rep["ptr_struct_names"] = {"roundI": dict(by_name(rows["roundI"]).most_common(25)),
                               "roundL": dict(by_name(rows["roundL"]).most_common(25))}
    rep["rivals"] = {}
    for r, rv in rv_vals.items():
        com = set(rv) & both
        row = {"own": summarize(rv), "rival_on_common": summarize({k: rv[k] for k in com}), "by_opt": {}}
        for lab in ARMS:
            row[f"kuna_{lab}_on_common"] = summarize({k: V[lab][k] for k in com})
        for opt in ("O0", "O2", "O2-noinline"):
            sub = {k: x for k, x in rv.items() if k[0].split("::")[1] == opt}
            subc = {k: x for k, x in sub.items() if k in both}
            row["by_opt"][opt] = {"rival": summarize(sub),
                                  "kuna_roundL": summarize({k: V["roundL"][k] for k in subc}),
                                  "kuna_roundI": summarize({k: V["roundI"][k] for k in subc})}
        rep["rivals"][r] = row
    (OUT / "analysis.json").write_text(json.dumps(rep, indent=1))
    print(json.dumps({k: v for k, v in rep.items() if k.startswith(("headline", "moves_", "per_project", "worse_I", "onto_"))},
                     indent=1))


if __name__ == "__main__":
    main()
