"""Interleaved min-of-N speed A/B for `--option structsynth param` vs off.

    SPEED_N=15 SPEED_OUT=speed.json python3 docs/features/structsynth/speed.py

Run from the worktree root with the release `kuna` built and the pinned decbench
results tree in place.

One (off, on) pair per round, rounds interleaved so any slow drift in machine
load hits both arms equally.  The reported number is the MIN of each arm (the
least-contended sample); the median is printed too, for scale.
"""
import json, os, subprocess, sys, time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
K = os.path.abspath("decompiler/target/release/kuna")
N = int(os.environ.get("SPEED_N", "15"))
env = dict(os.environ, SLEIGHHOME=os.path.abspath("specs"),
           KUNA_SPECS=os.path.abspath("specs"))

CASES = [
    ("fmt O2",  f"{R}/O2/coreutils/stripped/fmt"),
    ("ls O2",   f"{R}/O2/coreutils/stripped/ls"),
    ("sort O2", f"{R}/O2/coreutils/stripped/sort"),
    ("du O2",   f"{R}/O2/coreutils/stripped/du"),
    ("fmt O0",  f"{R}/O0/coreutils/stripped/fmt"),
    ("sort O0", f"{R}/O0/coreutils/stripped/sort"),
    ("grep O0 (inert control)", f"{R}/O0/grep/stripped/grep"),
]


def run(binary, on):
    cmd = [K, "decompile-all", binary]
    if on:
        cmd += ["--option", "structsynth", "param"]
    t = time.perf_counter()
    p = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    dt = (time.perf_counter() - t) * 1000.0
    return dt if p.returncode == 0 else None


samples = {name: ([], []) for name, _ in CASES}
for i in range(N):
    for name, binary in CASES:
        a = run(binary, False)
        b = run(binary, True)
        if a is not None:
            samples[name][0].append(a)
        if b is not None:
            samples[name][1].append(b)
    print(f"round {i+1}/{N} done", flush=True)

out = {}
for name, _ in CASES:
    off, on = samples[name]
    off.sort(); on.sort()
    mo, mn = off[0], on[0]
    med_o = off[len(off)//2]; med_n = on[len(on)//2]
    d = (mn - mo) / mo * 100.0
    out[name] = {"off_ms": round(mo, 1), "on_ms": round(mn, 1),
                 "delta_pct": round(d, 2),
                 "off_median": round(med_o, 1), "on_median": round(med_n, 1),
                 "pairs": len(off)}
    print(f"{name}: off {mo:.1f} ms  on {mn:.1f} ms  {d:+.2f}%  "
          f"(median {med_o:.1f} -> {med_n:.1f}, n={len(off)})", flush=True)
json.dump(out, open(os.environ.get("SPEED_OUT", ".scratch/speed6.json"), "w"), indent=1)
worst = max(v["delta_pct"] for k, v in out.items() if "inert" not in k)
print(f"worst_delta_pct={worst:+.2f}")
