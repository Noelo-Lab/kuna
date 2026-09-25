"""Interleaved min-of-N whole-binary decompile-all: main vs callpush default vs callpush off."""
import json, os, statistics, subprocess, sys, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2"
ALL = {"fmt": f"{R}/coreutils/stripped/fmt", "ls": f"{R}/coreutils/stripped/ls",
       "sort": f"{R}/coreutils/stripped/sort", "bash": f"{R}/bash/stripped/bash",
       "cp": f"{R}/coreutils/stripped/cp", "gzip": f"{R}/gzip/stripped/gzip"}
SP = "/home/mahaloz/kwt/castslots/specs"
K = {"main": ["/home/mahaloz/kwt/castbench/bin-c960fb18d/kuna"],
     "on": ["/home/mahaloz/kwt/castslots/.scratch/bin-v3/kuna"],
     "off": ["/home/mahaloz/kwt/castslots/.scratch/bin-v3/kuna", "--option", "callpush", "off"]}
ARMS = ["main", "on", "off"]
N = int(sys.argv[1]); outf = sys.argv[2]; which = sys.argv[3].split(",")
out = {}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
for name in which:
    b = ALL[name]; t = {a: [] for a in ARMS}
    for i in range(N):
        order = ARMS[i % 3:] + ARMS[:i % 3]
        for lab in order:
            cmd = [K[lab][0], "decompile-all", b, "--json", "--max-fn-seconds", "120"] + K[lab][1:]
            t0 = time.perf_counter()
            p = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v} for lab, v in t.items()}
    r["delta_min_pct_on_vs_main"] = round((r["on"]["min_ms"] - r["main"]["min_ms"]) / r["main"]["min_ms"] * 100, 2)
    r["delta_min_pct_on_vs_off"] = round((r["on"]["min_ms"] - r["off"]["min_ms"]) / r["off"]["min_ms"] * 100, 2)
    out[name] = r
    print(name, {lab: r[lab]["min_ms"] for lab in ARMS}, "on/main", r["delta_min_pct_on_vs_main"],
          "on/off", r["delta_min_pct_on_vs_off"], os.getloadavg(), flush=True)
    json.dump(out, open(outf, "w"), indent=1)
print("SPEED_DONE")
