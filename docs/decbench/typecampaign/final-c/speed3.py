"""Interleaved min-of-N whole-binary decompile-all timing: baseline, round B, round C."""
import json
import os
import statistics
import subprocess
import sys
import time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
BINS = [("fmt", f"{R}/coreutils/stripped/fmt"), ("ls", f"{R}/coreutils/stripped/ls"),
        ("sort", f"{R}/coreutils/stripped/sort"), ("bash", f"{R}/bash/stripped/bash")]
K = {"base": ("/home/mahaloz/kwt/_baseline/kuna", "/home/mahaloz/github/kuna/specs"),
     "roundB": ("/home/mahaloz/kwt/_final/kuna", "/home/mahaloz/kwt/_final-main/specs"),
     "roundC": ("/home/mahaloz/kwt/_final-c/kuna", "/home/mahaloz/kwt/_final-main/specs")}
ARMS = ["base", "roundB", "roundC"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 11
out = {}
for name, b in BINS:
    t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 3:] + ARMS[:i % 3]
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
    for lab in ("roundB", "roundC"):
        r[f"delta_min_pct_vs_base_{lab}"] = round(
            (r[lab]["min_ms"] - r["base"]["min_ms"]) / r["base"]["min_ms"] * 100, 2)
    r["delta_min_pct_C_vs_B"] = round(
        (r["roundC"]["min_ms"] - r["roundB"]["min_ms"]) / r["roundB"]["min_ms"] * 100, 2)
    r["delta_median_pct_C_vs_B"] = round(
        (r["roundC"]["median_ms"] - r["roundB"]["median_ms"]) / r["roundB"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "C/B", r["delta_min_pct_C_vs_B"],
          "C/base", r["delta_min_pct_vs_base_roundC"], os.getloadavg(), flush=True)
json.dump(out, open(sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/_final-c/speed.json", "w"), indent=1)
print("SPEED_DONE")
