"""Interleaved min-of-N speed, one binary, `--option castarith off` vs the default (on).

usage: speed.py <kuna> <specs> <N> <out.json> [name,...]
"""
import json, os, statistics, subprocess, sys, time

D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
ALL = {"fmt_O2": f"{D}/O2/coreutils/stripped/fmt", "ls_O2": f"{D}/O2/coreutils/stripped/ls",
       "sort_O2": f"{D}/O2/coreutils/stripped/sort", "bash_O2": f"{D}/O2/bash/stripped/bash"}
K, SP, N, outf = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
which = sys.argv[5].split(",") if len(sys.argv) > 5 else list(ALL)
ARMS = {"off": ["--option", "castarith", "off"], "on": []}
out = json.load(open(outf)) if os.path.exists(outf) else {}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
for name in which:
    t = {"off": [], "on": []}
    ratios = []
    for i in range(N + 1):
        order = ["off", "on"] if i % 2 == 0 else ["on", "off"]
        per = {}
        for arm in order:
            t0 = time.perf_counter()
            p = subprocess.run([K, "decompile-all", ALL[name], "--json", "--max-fn-seconds", "120"] + ARMS[arm],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            ms = round((time.perf_counter() - t0) * 1000, 1)
            if p.returncode != 0:
                print("RC", arm, name, p.returncode, flush=True)
            per[arm] = ms
        if i == 0:
            continue  # warm-up
        for arm in per:
            t[arm].append(per[arm])
        ratios.append(per["on"] / per["off"])
    r = {arm: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for arm, v in t.items()}
    r["delta_min_pct"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_of_ratios_pct"] = round((statistics.median(ratios) - 1) * 100, 2)
    r["loadavg"] = os.getloadavg()
    out[name] = r
    print(name, "min", r["delta_min_pct"], "% median-of-ratios", r["delta_median_of_ratios_pct"], "%",
          "off", r["off"]["min_ms"], "on", r["on"]["min_ms"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE", flush=True)
