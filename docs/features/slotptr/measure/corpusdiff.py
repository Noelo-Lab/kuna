"""Whole-binary decompile-all before/after: slotptr off vs on; classify every hunk."""
import json, subprocess, sys, collections, os, re
K = sys.argv[1]; ON = sys.argv[2]; OUT = sys.argv[3]
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = [f"O0/coreutils/stripped/{b}" for b in ("fmt", "ls", "sort")] + \
       [f"O2/coreutils/stripped/{b}" for b in ("fmt", "ls", "sort")] + \
       ["O2/bash/stripped/bash", "O0/tar/stripped/tar"]
PTR = re.compile(r"(\*|\(\*\)[^)]*)$|\(\*\)")
def run(b, v):
    p = subprocess.run([K, "decompile-all", f"{R}/{b}", "--json", "--max-fn-seconds", "120", "--option", "slotptr", v],
                       capture_output=True, text=True)
    return json.loads(p.stdout)
res = {}
for b in BINS:
    a, c = run(b, "off"), run(b, ON)
    fa = {f.get("address"): f for f in a["functions"]}
    cls = collections.Counter(); other = []
    nf = len(c["functions"]); codediff = 0; countdiff = 0
    for f in c["functions"]:
        g = fa.get(f.get("address"))
        if g is None: other.append(("missing", f.get("name"))); continue
        if g.get("code") != f.get("code"): codediff += 1; other.append(("code", f["name"]))
        if len(g.get("variables", [])) != len(f.get("variables", [])): countdiff += 1; other.append(("count", f["name"])); continue
        for x, y in zip(g["variables"], f["variables"]):
            if x == y: continue
            keys = {k for k in set(x) | set(y) if x.get(k) != y.get(k)}
            if keys == {"type"} and x["kind"] == "stack" and not x.get("line_numbers") and re.match(r"^undefined\d+$", x["type"]) and ("*" in y["type"]) :
                cls["filler undefinedN -> pointer, same size/name/offset"] += 1
            else:
                other.append(("row", f["name"], x, y))
    res[b] = {"functions": nf, "code_diffs": codediff, "count_diffs": countdiff, "classes": dict(cls), "unclassified": other[:20], "n_unclassified": len(other)}
    print(b, res[b]["functions"], "code", codediff, "count", countdiff, dict(cls), "unclassified", len(other), flush=True)
json.dump(res, open(OUT, "w"), indent=1)
