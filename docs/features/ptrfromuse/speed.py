"""Interleaved min-of-N speed A/B for the `ptrfromuse` default: `off` vs `void`.

    SPEED_N=15 SPEED_OUT=speed.json KUNA_BIN=<kuna> python3 docs/features/ptrfromuse/speed.py

Both arms name the value explicitly, so the measurement does not depend on which
value is the shipped default.  One (off, void) pair per round, the order swapped
every round, rounds interleaved so slow drift in machine load hits both arms
equally.  The reported number is the MIN of each arm; the median is printed too.
"""
import json, os, subprocess, time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
K = os.environ.get("KUNA_BIN") or os.path.abspath("decompiler/target/release/kuna")
N = int(os.environ.get("SPEED_N", "15"))
ON = os.environ.get("SPEED_ON", "void")
env = dict(os.environ, SLEIGHHOME=os.path.abspath("specs"), KUNA_SPECS=os.path.abspath("specs"))

CASES = [
    ("fmt O2", f"{R}/O2/coreutils/stripped/fmt"),
    ("ls O2", f"{R}/O2/coreutils/stripped/ls"),
    ("sort O2", f"{R}/O2/coreutils/stripped/sort"),
    ("bash O2 (1.3 MB, auto = reliable)", f"{R}/O2/bash/stripped/bash"),
]


def run(binary, value):
    cmd = [K, "decompile-all", binary, "--option", "ptrfromuse", value]
    t = time.perf_counter()
    p = subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    dt = (time.perf_counter() - t) * 1000.0
    return dt if p.returncode == 0 else None


samples = {name: ([], []) for name, _ in CASES}
for i in range(N):
    for name, binary in CASES:
        order = [("off", 0), (ON, 1)] if i % 2 == 0 else [(ON, 1), ("off", 0)]
        for value, slot in order:
            ms = run(binary, value)
            if ms is not None:
                samples[name][slot].append(ms)
    print(f"round {i + 1}/{N} done", flush=True)

out = {}
for name, _ in CASES:
    off, on = sorted(samples[name][0]), sorted(samples[name][1])
    mo, mn = off[0], on[0]
    d = (mn - mo) / mo * 100.0
    out[name] = {"off_ms": round(mo, 1), "on_ms": round(mn, 1), "delta_pct": round(d, 2),
                 "off_median": round(off[len(off) // 2], 1), "on_median": round(on[len(on) // 2], 1),
                 "pairs": min(len(off), len(on))}
    print(f"{name}: off {mo:.1f} ms  {ON} {mn:.1f} ms  {d:+.2f}%  "
          f"(median {off[len(off) // 2]:.1f} -> {on[len(on) // 2]:.1f}, n={len(off)})", flush=True)
json.dump(out, open(os.environ.get("SPEED_OUT", "speed.json"), "w"), indent=1)
print("worst", max(v["delta_pct"] for v in out.values()))
