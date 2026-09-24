"""moved.csv (round G -> round I) and report-slices.md (base/B/C/D/E/F/G/H/I per project x opt)."""
import collections, csv, json
from pathlib import Path
S = {"base": "/home/mahaloz/kwt/_final/sweep-base/rows.json",
     "roundB": "/home/mahaloz/kwt/_final-c/sweep-bctl/rows.json",
     "roundC": "/home/mahaloz/kwt/_final-c/sweep-c/rows.json",
     "roundD": "/home/mahaloz/kwt/_final-d/sweep-d/rows.json",
     "roundE": "/home/mahaloz/kwt/_final-e/sweep-e/rows.json",
     "roundF": "/home/mahaloz/kwt/_final-f/sweep-f/rows.json",
     "roundG": "/home/mahaloz/kwt/_final-g/sweep-g/rows.json",
     "roundH": "/home/mahaloz/kwt/_final-i/sweep-h/rows.json",
     "roundI": "/home/mahaloz/kwt/_final-i/sweep-i/rows.json"}
def vals(p):
    r = json.loads(Path(p).read_text()); v = {}
    for k, x in r.items():
        for fn, val in ((x.get("base") or {}).get("values") or {}).items():
            v[(k, fn)] = val
    return v
V = {k: vals(p) for k, p in S.items()}
both = set(V["roundG"]) & set(V["roundI"])
with open("/home/mahaloz/kwt/_final-i/moved.csv", "w", newline="") as f:
    w = csv.writer(f); w.writerow(["slice", "function", "round_g", "round_i", "delta"])
    for k in sorted(both):
        if abs(V["roundI"][k] - V["roundG"][k]) > 1e-9:
            w.writerow([k[0], k[1], round(V["roundG"][k], 4), round(V["roundI"][k], 4),
                        round(V["roundI"][k] - V["roundG"][k], 4)])
g = collections.defaultdict(lambda: collections.defaultdict(lambda: [0, 0, 0.0]))
for k in set().union(*[set(v) for v in V.values()]):
    proj, opt, _ = k[0].split("::")
    for lab in S:
        if k in V[lab]:
            r = g[(proj, opt)][lab]; r[0] += 1; r[1] += V[lab][k] == 1.0; r[2] += V[lab][k]
lines = ["| project | opt | n | " + " | ".join(f"perfect {l}" for l in ("base","B","C","D","E","F","G","H","I")) + " | " + " | ".join(f"mean {l}" for l in ("base","B","C","D","E","F","G","H","I")) + " |",
         "|---|---|---:|" + "---:|" * 18]
for (proj, opt) in sorted(g):
    d = g[(proj, opt)]
    n = d["roundI"][0]
    lines.append(f"| {proj} | {opt} | {n} | " + " | ".join(str(d[l][1]) for l in S) + " | "
                 + " | ".join(f"{d[l][2]/max(1,d[l][0]):.4f}" for l in S) + " |")
Path("/home/mahaloz/kwt/_final-i/report-slices.md").write_text("\n".join(lines) + "\n")
print("\n".join(lines))
