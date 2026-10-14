"""Compare release builds on lifetime fixtures, alternating run order.

python3 docs/features/lifetimes/speed.py BEFORE_KUNA AFTER_KUNA results.json
Both binaries need their matching decomp_dbg and decomp_test_dbg siblings.
The checkout must contain built processor specs.
"""

import argparse
import json
from pathlib import Path
import resource
import statistics
import subprocess
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("before", type=Path)
    parser.add_argument("after", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--samples", type=int, default=40)
    args = parser.parse_args()
    if args.samples < 1:
        parser.error("--samples must be positive")
    root = Path(__file__).resolve().parents[3]
    fixtures = root / "decompiler/crates/kuna-analysis/tests/fixtures"

    def command(fixture, functions):
        image = fixtures / fixture
        return [
            "decompile-all", str(image), "--functions", functions,
            "--mode", "reliable", "--sleighpath", str(root / "specs"),
            "--json", "--assert-strict", "--assert", "@" + str(image) + ".kuna",
        ]

    reuse = command("lifetimes_objects_x86_64", "obj_reuse")
    registers = command("lifetimes_x86_64", "register_names")
    frames = command("lifetimes_frame_events_x86_64", "frame_alias,frame_seed,frame_cached")
    cases = [
        ("default_reuse", args.before, reuse, args.after, reuse),
        ("default_registers", args.before, registers, args.after, registers),
        ("default_frame_events", args.before, frames, args.after, frames),
        ("opt_in_reuse", args.after, reuse, args.after, reuse + ["--option", "stackviews", "on"]),
    ]
    results = {}
    for name, before, before_args, after, after_args in cases:
        samples = {key: {"wall": [], "cpu": []} for key in ["before", "after"]}
        codes = {}
        for index in range(args.samples + 2):
            for key in (["before", "after"] if index % 2 else ["after", "before"]):
                binary, command_args = (before, before_args) if key == "before" else (after, after_args)
                cpu_start = resource.getrusage(resource.RUSAGE_CHILDREN)
                start = time.perf_counter()
                run = subprocess.run([str(binary.resolve()), *command_args], capture_output=True, timeout=60)
                wall = time.perf_counter() - start
                cpu_end = resource.getrusage(resource.RUSAGE_CHILDREN)
                cpu = cpu_end.ru_utime + cpu_end.ru_stime - cpu_start.ru_utime - cpu_start.ru_stime
                if run.returncode:
                    raise RuntimeError(f"{name} {key}: {run.stderr.decode()}")
                document = json.loads(run.stdout)
                if document.get("error") or any(row.get("error") for row in document["functions"]):
                    raise RuntimeError(document)
                if any(row["status"] != "applied" for row in document.get("assertions", [])):
                    raise RuntimeError(document["assertions"])
                codes[key] = [row["code"] for row in document["functions"]]
                if index >= 2:
                    samples[key]["wall"].append(wall)
                    samples[key]["cpu"].append(cpu)
        result = {"same_c": codes["before"] == codes["after"], "samples_seconds": samples}
        for metric in ["wall", "cpu"]:
            medians = {key: statistics.median(values[metric]) for key, values in samples.items()}
            result[metric] = {
                "medians_seconds": medians,
                "delta_percent": 100 * (medians["after"] / medians["before"] - 1),
            }
        results[name] = result
        args.output.write_text(json.dumps(results, indent=2) + "\n")
        print(name, {key: value for key, value in result.items() if key != "samples_seconds"}, flush=True)


if __name__ == "__main__":
    main()
