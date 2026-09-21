"""Interleaved min-of-N decompile-all timing: main vs this build (param) vs this build (nest)."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
SP = "/home/mahaloz/kwt/structnest/specs"
W = "/home/mahaloz/kwt/structnest/.scratch"
K = {"main": (f"{W}/base-bin/kuna", []),
     "param": (f"{W}/kuna-final/kuna", ["--option", "structsynth", "param"]),
     "nest": (f"{W}/kuna-final/kuna", ["--option", "structsynth", "nest"])}
ARMS = list(K)
N = int(sys.argv[1]); outf = sys.argv[2]; which = sys.argv[3].split(",")
out = {}
for name in which:
    b = ALL[name]; t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 3:] + ARMS[:i % 3]
        for lab in order:
            kb, extra = K[lab]
            env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
            t0 = time.perf_counter()
            p = subprocess.run([kb, "decompile-all", b, "--json", "--max-fn-seconds", "120"] + extra,
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0: print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct_nest_vs_param"] = round((r["nest"]["min_ms"] - r["param"]["min_ms"]) / r["param"]["min_ms"] * 100, 2)
    r["delta_min_pct_nest_vs_main"] = round((r["nest"]["min_ms"] - r["main"]["min_ms"]) / r["main"]["min_ms"] * 100, 2)
    r["delta_min_pct_param_vs_main"] = round((r["param"]["min_ms"] - r["main"]["min_ms"]) / r["main"]["min_ms"] * 100, 2)
    out[name] = r
    print(name, {k: v for k, v in r.items() if k.startswith("delta")}, {l: r[l]["min_ms"] for l in ARMS}, flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
