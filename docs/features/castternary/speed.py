"""Interleaved min-of-N whole-binary decompile-all timing: castternary off vs the default (on)."""
import json, os, statistics, subprocess, sys, time
D = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
R = f"{D}/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash",
       "dash_O2ni": f"{D}/O2-noinline/dash/stripped/dash", "cmp_O0": f"{D}/O0/diffutils/stripped/cmp",
       "cf2_O2ni": f"{D}/O2-noinline/crazyflie/stripped/cf2.elf",
       "kmod_O2ni": f"{D}/O2-noinline/kmod/stripped/kmod",
       "crontab_O2ni": f"{D}/O2-noinline/cronie/stripped/crontab",
       "dpkgdivert_O2": f"{R}/dpkg/stripped/dpkg-divert"}
K = sys.argv[1]; N = int(sys.argv[2]); outf = sys.argv[3]
which = sys.argv[4].split(",") if len(sys.argv) > 4 else list(ALL)
SP = os.environ.get("SLEIGHHOME", "specs")
ARMS = {"off": ["--option", "castternary", "off"], "on": []}
out = json.load(open(outf)) if os.path.exists(outf) else {}
for name in which:
    b = ALL[name]; t = {a: [] for a in ARMS}
    for i in range(N):
        order = list(ARMS) if i % 2 == 0 else list(ARMS)[::-1]
        for lab in order:
            env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
            t0 = time.perf_counter()
            p = subprocess.run([K, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + ARMS[lab],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0: print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["on"]["median_ms"] - r["off"]["median_ms"]) / r["off"]["median_ms"] * 100, 2)
    r["loadavg"] = os.getloadavg()
    out[name] = r
    print(name, r["off"]["min_ms"], r["on"]["min_ms"], "min", r["delta_min_pct"], "median", r["delta_median_pct"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE", flush=True)
