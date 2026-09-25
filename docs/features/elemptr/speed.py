"""Interleaved min-of-N whole-binary decompile-all timing: --option elemptr off vs the default (on), one frozen build."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
BINS = [("fmt", f"{R}/coreutils/stripped/fmt"), ("ls", f"{R}/coreutils/stripped/ls"),
        ("sort", f"{R}/coreutils/stripped/sort"), ("bash", f"{R}/bash/stripped/bash")]
KB = sys.argv[3] if len(sys.argv) > 3 else "decompiler/target/release/kuna"
SP = os.environ.get("SLEIGHHOME", "specs")
ARMS = {"off": ["--option", "elemptr", "off"], "on": []}
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
out = {}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
for name, b in BINS:
    t = {a: [] for a in ARMS}
    subprocess.run([KB, "decompile-all", b, "--json"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    for i in range(N):
        order = ["off", "on"] if i % 2 == 0 else ["on", "off"]
        for lab in order:
            t0 = time.perf_counter()
            p = subprocess.run([KB, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + ARMS[lab],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    ratios = [a / b for a, b in zip(t["on"], t["off"])]
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_of_ratios_pct"] = round((statistics.median(ratios) - 1) * 100, 2)
    r["loadavg"] = [round(x, 1) for x in os.getloadavg()]
    out[name] = r
    print(name, r["off"]["min_ms"], r["on"]["min_ms"], r["delta_min_pct"], r["delta_median_of_ratios_pct"], r["loadavg"], flush=True)
json.dump(out, open(sys.argv[2], "w"), indent=1)
print("SPEED_DONE")
