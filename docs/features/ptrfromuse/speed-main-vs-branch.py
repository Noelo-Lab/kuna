"""Interleaved min-of-N speed A/B of two kuna builds at their default options: main (KUNA_A) vs this branch (KUNA_B).

    KUNA_A=<main kuna> KUNA_B=<branch kuna> SPEED_N=15 SPEED_OUT=out.json python3 docs/features/ptrfromuse/speed-main-vs-branch.py
"""
import json, os, subprocess, time
R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
A = os.environ["KUNA_A"]; B = os.environ["KUNA_B"]; N = int(os.environ.get("SPEED_N", "15"))
env = dict(os.environ, SLEIGHHOME=os.path.abspath("specs"), KUNA_SPECS=os.path.abspath("specs"))
CASES = [("fmt O2", f"{R}/O2/coreutils/stripped/fmt"), ("ls O2", f"{R}/O2/coreutils/stripped/ls"),
         ("sort O2", f"{R}/O2/coreutils/stripped/sort"), ("bash O2", f"{R}/O2/bash/stripped/bash")]
def run(k, b):
    t = time.perf_counter(); p = subprocess.run([k, "decompile-all", b], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    return (time.perf_counter() - t) * 1000.0 if p.returncode == 0 else None
S = {n: ([], []) for n, _ in CASES}
for i in range(N):
    for n, b in CASES:
        for k, slot in ([(A, 0), (B, 1)] if i % 2 == 0 else [(B, 1), (A, 0)]):
            ms = run(k, b)
            if ms is not None: S[n][slot].append(ms)
    print(f"round {i+1}/{N} done", flush=True)
out = {}
for n, _ in CASES:
    a, b = sorted(S[n][0]), sorted(S[n][1])
    out[n] = {"main_ms": round(a[0], 1), "branch_ms": round(b[0], 1), "delta_pct": round((b[0] - a[0]) / a[0] * 100, 2),
              "main_median": round(a[len(a)//2], 1), "branch_median": round(b[len(b)//2], 1), "pairs": min(len(a), len(b))}
    print(n, out[n], flush=True)
json.dump(out, open(os.environ.get("SPEED_OUT", "speed2.json"), "w"), indent=1)
print("worst", max(v["delta_pct"] for v in out.values()))
