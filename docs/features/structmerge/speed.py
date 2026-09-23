"""Interleaved min-of-N whole-binary decompile-all timing: structmerge off vs siblings."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash"}
KUNA = "/home/mahaloz/kwt/layoutrecall/.scratch/kuna-A"
SPECS = "/home/mahaloz/kwt/layoutrecall/specs"
ARMS = {"off": ["--option", "structmerge", "off"],
        "siblings": ["--option", "structmerge", "siblings"]}
NAMES = list(ARMS)
N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
outf = sys.argv[2] if len(sys.argv) > 2 else "/home/mahaloz/kwt/layoutrecall/.scratch/speed.json"
which = sys.argv[3].split(",") if len(sys.argv) > 3 else list(ALL)
out = {}
for name in which:
    b = ALL[name]
    t = {a: [] for a in NAMES}
    for i in range(N):
        order = NAMES[i % 2:] + NAMES[:i % 2]
        for lab in order:
            env = dict(os.environ, SLEIGHHOME=SPECS, KUNA_SPECS=SPECS)
            t0 = time.perf_counter()
            p = subprocess.run([KUNA, "decompile-all", b, "--json", "--max-fn-seconds", "120"]
                               + ARMS[lab], stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v}
         for lab, v in t.items()}
    r["delta_min_pct"] = round((r["siblings"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round((r["siblings"]["median_ms"] - r["off"]["median_ms"]) / r["off"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, "off", r["off"]["min_ms"], "siblings", r["siblings"]["min_ms"],
          "delta_min", r["delta_min_pct"], "delta_median", r["delta_median_pct"], flush=True)
json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
