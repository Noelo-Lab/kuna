"""Interleaved min-of-N whole-binary decompile-all timing: slotptr off vs on, same build."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
KB = "/home/mahaloz/kwt/slotptr/.scratch/rel/kuna"
SP = "/home/mahaloz/kwt/slotptr/specs"
ARMS = ["off", "on"]
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
outf = sys.argv[2]
which = sys.argv[3].split(",") if len(sys.argv) > 3 else list(ALL)
out = {}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
for name in which:
    t = {a: [] for a in ARMS}
    for i in range(N):
        for lab in (ARMS if i % 2 == 0 else ARMS[::-1]):
            t0 = time.perf_counter()
            p = subprocess.run([KB, "decompile-all", ALL[name], "--json", "--max-fn-seconds", "120",
                                "--option", "slotptr", lab], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["on"]["median_ms"] - r["off"]["median_ms"]) / r["off"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, r["off"]["min_ms"], r["on"]["min_ms"], "min", r["delta_min_pct"], "median", r["delta_median_pct"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
