"""Round-G aggregate: baseline vs rounds B-F vs round G, per slice, per GT class, rivals."""
import collections
import json
from pathlib import Path

OUT = Path("/home/mahaloz/kwt/_final-g")
F = Path("/home/mahaloz/kwt/_final-f")
E = Path("/home/mahaloz/kwt/_final-e")
D = Path("/home/mahaloz/kwt/_final-d")
C = Path("/home/mahaloz/kwt/_final-c")
B = Path("/home/mahaloz/kwt/_final")
SETS = {"base": B / "sweep-base/rows.json", "roundB": C / "sweep-bctl/rows.json",
        "roundC": C / "sweep-c/rows.json", "roundD": D / "sweep-d/rows.json",
        "roundE": E / "sweep-e/rows.json", "roundF": F / "sweep-f/rows.json",
        "roundG": OUT / "sweep-g/rows.json",
        "roundFctl": OUT / "sweep-fctl/rows.json",
        "roundCctl": OUT / "sweep-cctl/rows.json"}
ARMS = ["base", "roundB", "roundC", "roundD", "roundE", "roundF", "roundG"]


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
    for a, b in (("roundF", "roundG"), ("roundE", "roundG"), ("roundD", "roundG"),
                 ("roundC", "roundG"), ("roundB", "roundG"), ("base", "roundG"),
                 ("roundF", "roundFctl"), ("roundC", "roundCctl")):
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
    both = set(V["roundF"]) & set(V["roundG"])
    pw = collections.defaultdict(lambda: [0, 0])
    for k in both:
        p = k[0].split("::")[0]
        pw[p][0] += V["roundG"][k] > V["roundF"][k] + 1e-9
        pw[p][1] += V["roundG"][k] < V["roundF"][k] - 1e-9
    rep["per_project_moves_F_to_G"] = {p: {"improved": a, "worse": b} for p, (a, b) in sorted(pw.items())}
    rep["worse_F_to_G"] = [[round(V["roundG"][k] - V["roundF"][k], 4), k[0], k[1],
                            round(V["roundF"][k], 4), round(V["roundG"][k], 4)]
                           for k in sorted(both, key=lambda k: V["roundG"][k] - V["roundF"][k])
                           if V["roundG"][k] < V["roundF"][k] - 1e-9]
    basek = set(V["base"]) & set(V["roundG"])
    rep["worse_base_to_G"] = [[round(V["roundG"][k] - V["base"][k], 4), k[0], k[1],
                               round(V["base"][k], 4), round(V["roundG"][k], 4)]
                              for k in sorted(basek, key=lambda k: V["roundG"][k] - V["base"][k])
                              if V["roundG"][k] < V["base"][k] - 1e-9]
    rep["best_F_to_G"] = [[round(V["roundG"][k] - V["roundF"][k], 4), k[0], k[1]]
                          for k in sorted(both, key=lambda k: V["roundF"][k] - V["roundG"][k])[:40]]
    riv = json.loads((B / "rivals/rivals.json").read_text())
    out = {}
    keep = both
    for lab in ARMS:
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
    rep["ptr_struct_names"] = {"roundF": dict(by_name(rows["roundF"]).most_common(25)),
                               "roundG": dict(by_name(rows["roundG"]).most_common(25))}
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
                                  "kuna_roundG": summarize({k: V["roundG"][k] for k in subc}),
                                  "kuna_roundF": summarize({k: V["roundF"][k] for k in subc})}
        rep["rivals"][r] = row
    (OUT / "analysis.json").write_text(json.dumps(rep, indent=1))
    print(json.dumps({k: rep[k] for k in ("headline", "moves_roundF_to_roundG", "moves_roundF_to_roundFctl",
                                          "moves_roundC_to_roundCctl", "moves_roundC_to_roundG",
                                          "moves_base_to_roundG", "per_project_moves_F_to_G", "worse_F_to_G")
                      if k in rep}, indent=1))


if __name__ == "__main__":
    main()
