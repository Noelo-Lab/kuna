"""Interleaved min-of-N speed for the element-pair rule, three arms per binary.

    KUNA_BEFORE=<main build kuna> KUNA_AFTER=<branch build kuna> SPEED_N=15 \
        SPEED_OUT=speed.json python3 docs/features/structsynth/arrays_speed.py

Arms: `off` (the branch build at its default), `before` (the main build with
`--option structsynth param`) and `after` (the branch build with the same
option). `after` vs `before` is the cost of the rule itself; `after` vs `off`
is what flipping the default would cost. Every round runs every arm of every
binary, the arm order rotating by round so no arm always runs first; the
reported number is each arm's MIN, with the median printed for scale.
`SPEED_CASES=ls O2,du O2` re-runs a subset.
"""
import json
import os
import subprocess
import time

R = "/home/mahaloz/github/decbench/results/full_run_address_2026-09-11"
BEFORE = os.environ["KUNA_BEFORE"]
AFTER = os.environ["KUNA_AFTER"]
N = int(os.environ.get("SPEED_N", "15"))
env = dict(os.environ, SLEIGHHOME=os.path.abspath("specs"), KUNA_SPECS=os.path.abspath("specs"))

CASES = [
    ("factor O2", f"{R}/O2/coreutils/stripped/factor"),
    ("fmt O2", f"{R}/O2/coreutils/stripped/fmt"),
    ("ls O2", f"{R}/O2/coreutils/stripped/ls"),
    ("du O2", f"{R}/O2/coreutils/stripped/du"),
    ("grep O0 (inert control)", f"{R}/O0/grep/stripped/grep"),
]
if os.environ.get("SPEED_CASES"):
    CASES = [c for c in CASES if c[0] in os.environ["SPEED_CASES"].split(",")]
ARMS = {
    "off": (AFTER, []),
    "before": (BEFORE, ["--option", "structsynth", "param"]),
    "after": (AFTER, ["--option", "structsynth", "param"]),
}


def run(kuna, extra, binary):
    t = time.perf_counter()
    p = subprocess.run([kuna, "decompile-all", binary] + extra,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env)
    return (time.perf_counter() - t) * 1000.0 if p.returncode == 0 else None


names = list(ARMS)
samples = {c: {a: [] for a in ARMS} for c, _ in CASES}
for i in range(N):
    order = names[i % 3:] + names[:i % 3]
    for case, binary in CASES:
        for arm in order:
            dt = run(*ARMS[arm], binary)
            if dt is not None:
                samples[case][arm].append(dt)
    print(f"round {i + 1}/{N} done", flush=True)


def pct(a, b):
    return round((a - b) / b * 100.0, 2)


out = {}
for case, _ in CASES:
    s = {a: sorted(v) for a, v in samples[case].items()}
    mins = {a: round(v[0], 1) for a, v in s.items()}
    meds = {a: round(v[len(v) // 2], 1) for a, v in s.items()}
    out[case] = {"min_ms": mins, "median_ms": meds, "n": {a: len(v) for a, v in s.items()},
                 "rule_delta_pct": pct(mins["after"], mins["before"]),
                 "flip_delta_pct": pct(mins["after"], mins["off"])}
    print(f"{case}: off {mins['off']} before {mins['before']} after {mins['after']} ms  "
          f"rule {out[case]['rule_delta_pct']:+.2f}%  flip {out[case]['flip_delta_pct']:+.2f}%",
          flush=True)
json.dump(out, open(os.environ.get("SPEED_OUT", "arrays_speed.json"), "w"), indent=1)
real = [v for k, v in out.items() if "inert" not in k]
print(f"worst rule_delta_pct={max(v['rule_delta_pct'] for v in real):+.2f}  "
      f"worst flip_delta_pct={max(v['flip_delta_pct'] for v in real):+.2f}")
