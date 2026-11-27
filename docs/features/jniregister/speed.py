"""Measure the JNI registration option on the repository's stripped AArch64 mock.

From the repo root, pin to one available CPU and run
`taskset -c 30 python3 -m docs.features.jniregister.speed`.
"""
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


def sample(console, fixture, state):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter_ns()
    source = decompile.decompile(fixture, "0x48c0", by_address=True,
                                 options=[("jniregister", state)], decomp_dbg=console)
    if ("countBytes" in source) != (state == "on"):
        raise RuntimeError(f"unexpected decompilation with jniregister {state}")
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
    if len(os.sched_getaffinity(0)) != 1:
        raise SystemExit("Pin this benchmark to one available CPU with taskset -c CPU.")
    fixture = paths.repo_root() / "tests/fixtures/android-jni/aarch64-linux-android.so"
    samples = {"off": [], "on": []}
    with tempfile.TemporaryDirectory() as directory:
        console = shutil.copy2(paths.decomp_dbg(), Path(directory) / "decomp_dbg")
        for pair in range(-6, 100):
            for state in (("off", "on") if pair % 2 == 0 else ("on", "off")):
                times = sample(console, fixture, state)
                if pair >= 0:
                    samples[state].append(times)
    off_wall, off_cpu = zip(*samples["off"])
    on_wall, on_cpu = zip(*samples["on"])
    print(json.dumps({"pairs": len(off_wall), "wall": summarize(off_wall, on_wall),
                      "child_cpu": summarize(off_cpu, on_cpu)}, indent=2))


if __name__ == "__main__":
    main()
