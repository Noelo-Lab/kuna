"""Interleaved min-of-N `decompile-all --jobs 8` timing on tar O2: param vs default locals, same build.

    python3 jobs.py <kuna> <N> <out.json> <outdir>

Also keeps each arm's first output and a serial run of each, so jobs == serial can be checked with cmp.
"""
import json, os, statistics, subprocess, sys, time
B = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11/O2/tar/stripped/tar"
kuna, N, outf, od = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4]
SP = os.environ["KUNA_SPECS"]
ARMS = {"param": ["--option", "structsynth", "param"], "locals": []}
env = dict(os.environ, SLEIGHHOME=SP, KUNA_SPECS=SP)
os.makedirs(od, exist_ok=True)
for lab, extra in ARMS.items():
    with open(f"{od}/serial.{lab}.c", "w") as fo:
        subprocess.run([kuna, "decompile-all", B] + extra, stdout=fo, stderr=subprocess.DEVNULL, env=env)
t = {a: [] for a in ARMS}
for i in range(N):
    order = list(ARMS) if i % 2 == 0 else list(ARMS)[::-1]
    for lab in order:
        with open(f"{od}/jobs.{lab}.c", "w") as fo, open(f"{od}/jobs.{lab}.err", "w") as fe:
            t0 = time.perf_counter()
            p = subprocess.run([kuna, "decompile-all", B, "--jobs", "8"] + ARMS[lab], stdout=fo, stderr=fe, env=env)
            t[lab].append(round(time.perf_counter() - t0, 2))
        if p.returncode != 0: print("RC", lab, p.returncode, flush=True)
r = {lab: {"min_s": min(v), "median_s": statistics.median(v), "samples": v} for lab, v in t.items()}
r["delta_min_pct_locals_vs_param"] = round((r["locals"]["min_s"] - r["param"]["min_s"]) / r["param"]["min_s"] * 100, 2)
json.dump(r, open(outf, "w"), indent=1)
print({k: (v["min_s"] if isinstance(v, dict) else v) for k, v in r.items()}, flush=True)
print("JOBS_DONE")
