#!/usr/bin/env python3
"""Interleaved (alternating order) min-of-N wall clock of `kuna decompile-all <bin> --json`, main build vs fix build (paths are this lane's worktree; edit W/BASE/FIX to rerun)."""
import subprocess, sys, time, os, json, statistics
N = int(os.environ.get("ROUNDS", "15"))
W = "/home/mahaloz/kwt/foldcallret-sc"
BASE, FIX = f"{W}/.scratch/base-bin/kuna", f"{W}/.scratch/fix-bin/kuna"
env = dict(os.environ, SLEIGHHOME=f"{W}/specs", KUNA_SPECS=f"{W}/specs")
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
cases = [("O2/coreutils/fmt", f"{R}/O2/coreutils/stripped/fmt"),
         ("O2/coreutils/ls", f"{R}/O2/coreutils/stripped/ls"),
         ("O2/coreutils/sort", f"{R}/O2/coreutils/stripped/sort"),
         ("O2/bash/bash", f"{R}/O2/bash/stripped/bash")]
if len(sys.argv) > 2:
    cases = [c for c in cases if c[0] in sys.argv[2].split(",")]
def run(k, b):
    t = time.perf_counter()
    subprocess.run([k, "decompile-all", b, "--json"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    return (time.perf_counter() - t) * 1000
out = {}
for label, b in cases:
    a, f = [], []
    for i in range(N):
        if i % 2 == 0:
            a.append(run(BASE, b)); f.append(run(FIX, b))
        else:
            f.append(run(FIX, b)); a.append(run(BASE, b))
    out[label] = {"n": N, "base_min_ms": round(min(a), 1), "fix_min_ms": round(min(f), 1),
                  "delta_min_pct": round((min(f) - min(a)) / min(a) * 100, 2),
                  "base_median_ms": round(statistics.median(a), 1), "fix_median_ms": round(statistics.median(f), 1),
                  "delta_median_pct": round((statistics.median(f) - statistics.median(a)) / statistics.median(a) * 100, 2)}
    print(label, json.dumps(out[label]), flush=True)
json.dump(out, open(sys.argv[1], "w"), indent=1)
