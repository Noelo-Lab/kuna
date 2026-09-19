"""Interleaved min-of-N whole-binary decompile-all timing, baseline vs final."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
BINS = [("fmt", f"{R}/coreutils/stripped/fmt"), ("ls", f"{R}/coreutils/stripped/ls"),
        ("sort", f"{R}/coreutils/stripped/sort"), ("bash", f"{R}/bash/stripped/bash")]
K = {"base": ("/home/mahaloz/kwt/_baseline/kuna", "/home/mahaloz/github/kuna/specs"),
     "final": ("/home/mahaloz/kwt/_final/kuna", "/home/mahaloz/kwt/_final-main/specs")}
N = int(sys.argv[1]) if len(sys.argv) > 1 else 11
out = {}
for name, b in BINS:
    t = {"base": [], "final": []}
    for i in range(N):
        order = ("base", "final") if i % 2 == 0 else ("final", "base")
        for lab in order:
            kb, sp = K[lab]
            env = dict(os.environ, SLEIGHHOME=sp, KUNA_SPECS=sp)
            t0 = time.perf_counter()
            p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            dt = time.perf_counter() - t0
            t[lab].append(round(dt * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["final"]["min_ms"] - r["base"]["min_ms"]) / r["base"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["final"]["median_ms"] - r["base"]["median_ms"]) / r["base"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, r["base"]["min_ms"], r["final"]["min_ms"], r["delta_min_pct"], r["delta_median_pct"], os.getloadavg(), flush=True)
json.dump(out, open(sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/_final/speed.json", "w"), indent=1)
print("SPEED_DONE")
