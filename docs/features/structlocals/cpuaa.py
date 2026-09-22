"""Interleaved min-of-N decompile-all wall AND cpu time with an A/A control: param, default locals and param again, one build.

    python3 cpuaa.py <N> <out.json> sort[,fmt,...] <kuna>

CPU time is less sensitive than wall time to a box shared with other jobs.
"""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
SP = os.environ["KUNA_SPECS"]
N = int(sys.argv[1]); outf = sys.argv[2]; which = sys.argv[3].split(","); kuna = sys.argv[4]
K = {"param": ["--option", "structsynth", "param"], "locals": [], "param_b": ["--option", "structsynth", "param"]}
ARMS = list(K)
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
out = {}
for name in which:
    wall = {a: [] for a in ARMS}; cpu = {a: [] for a in ARMS}
    for i in range(N):
        for lab in (ARMS[i % 3:] + ARMS[:i % 3]):
            t0 = time.perf_counter()
            p = subprocess.Popen([kuna, "decompile-all", ALL[name], "--json", "--max-fn-seconds", "120"] + K[lab],
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            _, status, ru = os.wait4(p.pid, 0)
            wall[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            cpu[lab].append(round((ru.ru_utime + ru.ru_stime) * 1000, 1))
            if status != 0: print("RC", lab, name, status, flush=True)
    r = {"wall": {a: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for a, v in wall.items()},
         "cpu": {a: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for a, v in cpu.items()}}
    for k in ("wall", "cpu"):
        r[f"delta_min_pct_{k}_aa"] = round((r[k]["param_b"]["min_ms"] - r[k]["param"]["min_ms"]) / r[k]["param"]["min_ms"] * 100, 2)
        r[f"delta_min_pct_{k}"] = round((r[k]["locals"]["min_ms"] - r[k]["param"]["min_ms"]) / r[k]["param"]["min_ms"] * 100, 2)
        r[f"delta_median_pct_{k}"] = round((r[k]["locals"]["median_ms"] - r[k]["param"]["median_ms"]) / r[k]["param"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {k: v for k, v in r.items() if k.startswith("delta")}, flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("CPUSPEED_DONE")
