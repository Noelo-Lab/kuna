"""Interleaved min-of-N whole-binary decompile-all timing: two frozen release binaries (main vs the fix).

usage: speed.py N OUT.json MAIN_KUNA FIX_KUNA   (SPEED_BINS=fmt,ls,sort,bash; SLEIGHHOME set)
"""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
BINS = [(n, p) for n, p in [("fmt", f"{R}/coreutils/stripped/fmt"), ("ls", f"{R}/coreutils/stripped/ls"),
        ("sort", f"{R}/coreutils/stripped/sort"), ("bash", f"{R}/bash/stripped/bash")]
        if n in os.environ.get("SPEED_BINS", "fmt,ls,sort,bash").split(",")]
ARMS = {"main": sys.argv[3], "fix": sys.argv[4]}
SP = os.environ.get("SLEIGHHOME", "specs")
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
out = {}
for name, b in BINS:
    t = {a: [] for a in ARMS}
    for kb in ARMS.values():
        subprocess.run([kb, "decompile-all", b, "--json"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    for i in range(N):
        for lab in (["main", "fix"] if i % 2 == 0 else ["fix", "main"]):
            t0 = time.perf_counter()
            p = subprocess.run([ARMS[lab], "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            if p.returncode == 0:
                t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
    ratios = [a / b for a, b in zip(t["fix"], t["main"])]
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["fix"]["min_ms"] - r["main"]["min_ms"]) / r["main"]["min_ms"] * 100, 2)
    r["delta_median_of_ratios_pct"] = round((statistics.median(ratios) - 1) * 100, 2)
    r["loadavg"] = [round(x, 1) for x in os.getloadavg()]
    out[name] = r
    print(name, r["main"]["min_ms"], r["fix"]["min_ms"], r["delta_min_pct"], r["delta_median_of_ratios_pct"], r["loadavg"], flush=True)
json.dump(out, open(sys.argv[2], "w"), indent=1)
print("SPEED_DONE")
