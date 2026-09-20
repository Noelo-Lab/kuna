"""Interleaved min-of-N `decompile-project` timing: the export before and after the order change.

    python3 docs/features/projectorder/speed.py 15 <out.json>

The two arms alternate every round, so a busy moment costs both.  The export
writes into a fresh temp folder each round; the folder is removed afterwards.
"""
import json
import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BINS = [("fmt O2", f"{R}/O2/coreutils/stripped/fmt"),
        ("sort O2", f"{R}/O2/coreutils/stripped/sort"),
        ("du O0", f"{R}/O0/coreutils/stripped/du")]
WT = "/home/mahaloz/kwt/layoutvote"
K = {"before": f"{WT}/.scratch/bin/kuna-base", "after": f"{WT}/decompiler/target/release/kuna"}
ARMS = ["before", "after"]
SPECS = f"{WT}/specs"

N = int(sys.argv[1]) if len(sys.argv) > 1 else 15
out = {}
for name, b in BINS:
    t = {a: [] for a in ARMS}
    for i in range(N):
        for lab in (ARMS if i % 2 == 0 else ARMS[::-1]):
            d = tempfile.mkdtemp(prefix="projectorder-")
            env = dict(os.environ, SLEIGHHOME=SPECS, KUNA_SPECS=SPECS)
            t0 = time.perf_counter()
            p = subprocess.run([K[lab], "decompile-project", b, "-o", d, "--max-fn-seconds", "120"],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
            t[lab].append(round((time.perf_counter() - t0) * 1000, 1))
            shutil.rmtree(d, ignore_errors=True)
            if p.returncode != 0:
                print("RC", lab, name, p.returncode, flush=True)
    r = {lab: {"min_ms": min(v), "median_ms": statistics.median(v), "samples": v}
         for lab, v in t.items()}
    r["delta_min_pct"] = round((r["after"]["min_ms"] - r["before"]["min_ms"]) / r["before"]["min_ms"] * 100, 2)
    r["delta_median_pct"] = round(
        (r["after"]["median_ms"] - r["before"]["median_ms"]) / r["before"]["median_ms"] * 100, 2)
    out[name] = r
    print(name, {k: v for k, v in r.items() if k.startswith("delta")}, flush=True)
if len(sys.argv) > 2:
    json.dump(out, open(sys.argv[2], "w"), indent=1)
