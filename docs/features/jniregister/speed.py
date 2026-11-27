"""Measure Android analysis options on the repository's stripped AArch64 mocks.

From the repo root, pin to one available CPU and run
`taskset -c 30 python3 -m docs.features.jniregister.speed`.
"""
import argparse
import json
import os
from pathlib import Path
import random
import resource
import shutil
from statistics import median
import tempfile
import time

from scripts import decompile, paths


CASES = {
    "jniregister": ("android-jni", "0x48c0", "countBytes"),
    "jnicalls": ("android-java-calls", "Java_demo_Bridge_cached__Ldemo_Player_2I", "/* Ldemo/Player;->seek(I)V */"),
    "ndkbinder": ("android-binder", "0x4ac4", "readLevel"),
}


def sample(console, fixture, option, selector, marker, state):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter_ns()
    source = decompile.decompile(fixture, selector, by_address=selector.startswith("0x"),
                                 options=[(option, state)], decomp_dbg=console)
    if (marker in source) != (state == "on"):
        raise RuntimeError(f"unexpected decompilation with {option} {state}: {source}")
    wall_ms = (time.perf_counter_ns() - start) / 1e6
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu_ms = 1000 * ((after.ru_utime - before.ru_utime) + (after.ru_stime - before.ru_stime))
    return wall_ms, cpu_ms


def summarize(off, on):
    def delta(off_ms, on_ms):
        return 100 * (on_ms / off_ms - 1)

    off_ms, on_ms = median(off), median(on)
    rng = random.Random(20261127)
    bootstrap = []
    for _ in range(4000):
        indices = rng.choices(range(len(off)), k=len(off))
        bootstrap.append(delta(median(off[i] for i in indices), median(on[i] for i in indices)))
    bootstrap.sort()
    return {
        "off_ms": round(off_ms, 3),
        "on_ms": round(on_ms, 3),
        "delta_pct": round(delta(off_ms, on_ms), 2),
        "delta_pct_ci95": [round(bootstrap[100], 2), round(bootstrap[3899], 2)],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--option", choices=CASES, default="jniregister")
    args = parser.parse_args()
    if len(os.sched_getaffinity(0)) != 1:
        raise SystemExit("Pin this benchmark to one available CPU with taskset -c CPU.")
    directory, selector, marker = CASES[args.option]
    fixture = paths.repo_root() / "tests/fixtures" / directory / "aarch64-linux-android.so"
    samples = {"off": [], "on": []}
    with tempfile.TemporaryDirectory() as directory:
        console = shutil.copy2(paths.decomp_dbg(), Path(directory) / "decomp_dbg")
        for pair in range(-6, 100):
            for state in (("off", "on") if pair % 2 == 0 else ("on", "off")):
                times = sample(console, fixture, args.option, selector, marker, state)
                if pair >= 0:
                    samples[state].append(times)
    off_wall, off_cpu = zip(*samples["off"])
    on_wall, on_cpu = zip(*samples["on"])
    print(json.dumps({"pairs": len(off_wall), "wall": summarize(off_wall, on_wall),
                      "child_cpu": summarize(off_cpu, on_cpu)}, indent=2))


if __name__ == "__main__":
    main()
