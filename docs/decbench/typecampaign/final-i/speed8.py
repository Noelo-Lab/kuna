"""Interleaved min-of-N whole-binary decompile-all timing: baseline, round G, round H, final (round I)."""
import json
import os
import statistics
import subprocess
import sys
import time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
K = {"base": ("/home/mahaloz/kwt/_baseline/kuna", "/home/mahaloz/github/kuna/specs"),
     "roundG": ("/home/mahaloz/kwt/_final-g/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundH": ("/home/mahaloz/kwt/_final-i/bh/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundI": ("/home/mahaloz/kwt/_final-i/kuna", "/home/mahaloz/kwt/_final-main/specs")}
ARMS = ["base", "roundG", "roundH", "roundI"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 11
outf = sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/_final-i/speed.json"
which = sys.argv[3].split(",") if len(sys.argv) > 3 else list(ALL)
out = {}
for name in which:
    b = ALL[name]
    t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 4:] + ARMS[:i % 4]
        for lab in order:
            kb, sp = K[lab]
            env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
            t0 = time.perf_counter()
            p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v}
         for lab, v in t.items()}
    for lab in ("roundG", "roundH", "roundI"):
        r[f"delta_min_pct_vs_base_{lab}"] = round(
            (r[lab]["min_ms"] - r["base"]["min_ms"]) / r["base"]["min_ms"] * 100, 2)
    for a2, b2 in (("roundI", "roundG"), ("roundI", "roundH")):
        r[f"delta_min_pct_{a2}_vs_{b2}"] = round(
            (r[a2]["min_ms"] - r[b2]["min_ms"]) / r[b2]["min_ms"] * 100, 2)
        r[f"delta_median_pct_{a2}_vs_{b2}"] = round(
            (r[a2]["median_ms"] - r[b2]["median_ms"]) / r[b2]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "I/G", r["delta_min_pct_roundI_vs_roundG"],
          "I/H", r["delta_min_pct_roundI_vs_roundH"], "I/base", r["delta_min_pct_vs_base_roundI"],
          "I/G median", r["delta_median_pct_roundI_vs_roundG"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
