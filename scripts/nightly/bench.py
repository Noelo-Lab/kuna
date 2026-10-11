"""Time kuna over the nightly sample and score its output with decbench's own code.

Mirrors how decbench scores kuna for the published leaderboard: kuna sees a
fully-stripped copy (`strip --strip-all`) of each binary, runs as
`kuna decompile-all <stripped> --json --max-fn-seconds 600` in its default mode, its
output is parsed by decbench's kuna backend, re-labeled to the DWARF names of the
unstripped binary (`scripts/run_benchmark.py::_relabel_to_dwarf` in the decbench
checkout), and scored for GED against the dataset's published source CFGs and for
type_match against DWARF.

Timing runs one decompile at a time so a binary's CPU time is not inflated by its
neighbours. With `--baseline-kuna` (last night's build) each binary is decompiled by
both builds back to back, alternating which goes first, so the speed comparison is an
A/B on one machine rather than a comparison across two runners. Scoring then runs in
parallel and uses tonight's output only.

Must run under a Python that imports `decbench` and `rust_joern`:

    python -m scripts.nightly.bench --sample sample.json --data data/ \
        --kuna kuna --specs specs/ --decbench-root decbench/ --out out/ \
        [--baseline-kuna base/kuna --baseline-specs base/specs]
"""
from __future__ import annotations

import argparse
import concurrent.futures
import datetime
import importlib.util
import json
import multiprocessing
import os
import platform
import shutil
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

MAX_FN_SECONDS = 600
METRICS = ("ged", "type_match")


def strip_copy(src: Path, dest: Path) -> Path:
    """A copy with no symbols and no DWARF, which is all decbench ever hands a decompiler."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dest)
    for cmd in (["strip", "--strip-all", str(dest)],
                ["objcopy", "--strip-all", str(dest), str(dest)]):
        if subprocess.run(cmd, capture_output=True).returncode == 0:
            break
    else:
        raise RuntimeError(f"could not strip {src}")
    if dest.read_bytes()[:4] == b"\x7fELF":
        from elftools.elf.elffile import ELFFile

        with open(dest, "rb") as f:
            names = {s.name for s in ELFFile(f).iter_sections()}
        leaked = names & {".symtab", ".debug_info", ".debug_line"}
        if leaked:
            raise RuntimeError(f"{dest} still carries {sorted(leaked)} after strip")
    return dest


def run_kuna(kuna: Path, specs: Path, binary: Path, out: Path, timeout: float) -> dict:
    """One timed `decompile-all`; CPU time comes from the child's own rusage."""
    env = dict(os.environ, KUNA_SPECS=str(specs))
    cmd = [str(kuna), "decompile-all", str(binary), "--json",
           "--max-fn-seconds", str(MAX_FN_SECONDS)]
    err = out.with_suffix(".stderr")
    start = time.perf_counter()
    with open(out, "wb") as fo, open(err, "wb") as fe:
        proc = subprocess.Popen(cmd, stdout=fo, stderr=fe, env=env, start_new_session=True)
        timer = threading.Timer(timeout, lambda: os.killpg(proc.pid, signal.SIGKILL))
        timer.start()
        try:
            _, status, usage = os.wait4(proc.pid, 0)
        finally:
            timer.cancel()
    wall = time.perf_counter() - start
    proc.returncode = os.waitstatus_to_exitcode(status)
    record = {
        "wall": round(wall, 4),
        "cpu": round(usage.ru_utime + usage.ru_stime, 4),
        "maxrss_kb": usage.ru_maxrss,
        "exit": proc.returncode,
    }
    if proc.returncode != 0:
        record["error"] = err.read_text(errors="replace")[-400:]
    if wall >= timeout:
        record["error"] = f"timeout after {timeout:.0f}s"
    return record


def time_sample(entries: list[dict], work: Path, arms: list[tuple[str, Path, Path]],
                timeout: float) -> dict[str, dict]:
    """Decompile every binary with every arm, sequentially; keeps only the `new` arm's JSON."""
    timings: dict[str, dict] = {}
    for i, entry in enumerate(entries):
        key = entry["binary_path"]
        stripped = work / key / "stripped" / Path(key).name
        order = arms if i % 2 == 0 else list(reversed(arms))
        timings[key] = {}
        for label, kuna, specs in order:
            out = work / key / f"{label}.json"
            timings[key][label] = run_kuna(kuna, specs, stripped, out, timeout)
            if label != "new":
                out.unlink(missing_ok=True)
        new = timings[key]["new"]
        base = timings[key].get("baseline")
        line = f"[{i + 1}/{len(entries)}] {key}: {new['cpu']:.2f}s"
        if base:
            line += f" (baseline {base['cpu']:.2f}s)"
        print(line, file=sys.stderr, flush=True)
    return timings


_RELABEL = None


def _relabel_fn(decbench_root: Path):
    """decbench's own `_relabel_to_dwarf`, loaded from its run driver so the two never drift."""
    global _RELABEL
    if _RELABEL is None:
        spec = importlib.util.spec_from_file_location(
            "decbench_run_benchmark", decbench_root / "scripts" / "run_benchmark.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        _RELABEL = module._relabel_to_dwarf
    return _RELABEL


def score_binary(entry: dict, data: Path, work: Path, decbench_root: Path,
                 source_stems: list[str]) -> dict:
    """Per-function GED and type_match for one binary's `new` output.

    A function decbench can measure (non-degenerate source CFG for GED, DWARF
    ground truth for type_match) that kuna's output does not yield a value for is
    recorded as `null`: a failure, not an abstention.
    """
    from decbench.decompilers.base import DecompilerConfig
    from decbench.decompilers.provenance import sanitize_native_provenance
    from decbench.decompilers.raw.kuna_raw import RawKunaDecompiler
    from decbench.metrics import type_match as tm
    from decbench.pipeline.evaluate import evaluate_decompilation
    from decbench.publish.cfg_export import rebuild_cfg
    from decbench.utils import binfmt
    from decbench.utils.cfg import best_source_by_name, resolved_source_for_binary
    from decbench.utils.dwarf_policy import dwarf_follow_abstract_origin

    key = entry["binary_path"]
    original = data / key
    stripped = work / key / "stripped" / Path(key).name
    stem = entry["binary"]
    try:
        payload = json.loads((work / key / "new.json").read_text())
    except (OSError, ValueError):
        payload = None

    owners = binfmt.source_function_owners(
        original, set(source_stems), follow_abstract_origin=dwarf_follow_abstract_origin())
    addr2name = {addr: name for addr, (name, _tu) in owners.items()}
    name2addr: dict[str, int] = {}
    for addr, name in addr2name.items():
        name2addr.setdefault(name, addr)

    raw = json.loads((data / entry["source_cfg_path"]).read_text()).get("functions", {})
    by_binary = {stem: {name: rebuild_cfg(cfg) for name, cfg in raw.items()}}
    source_cfgs = resolved_source_for_binary(stem, by_binary, best_source_by_name(by_binary))

    emitted = 0
    scored = {}
    if payload is not None:
        decompiler = RawKunaDecompiler(DecompilerConfig())
        decompiler._payload_cache[str(stripped)] = payload
        result = decompiler.decompile_binary(stripped, function_names=set(addr2name) or None)
        sanitize_native_provenance(result, stripped, defer_unavailable=True)
        if addr2name:
            _relabel_fn(decbench_root)(result, addr2name, original)
        else:
            result.binary_path = original
        sanitize_native_provenance(result, original)
        emitted = len(result.functions)
        scored = evaluate_decompilation(result, source_cfgs, list(METRICS))

    info = binfmt.detect(original)
    arch = info.arch if info is not None else ""
    gt_index = tm.extract_ground_truth_type_index(original)
    functions: dict[str, dict] = {}
    for name in entry["functions"]:
        record: dict[str, float | None] = {}
        if name in source_cfgs:
            value = scored["ged"].function_results.get(name) if "ged" in scored else None
            record["ged"] = None if value is None else float(value.value)
        value = scored["type_match"].function_results.get(name) if "type_match" in scored else None
        if value is not None:
            record["type_match"] = float(value.value)
        elif name in name2addr and tm._ground_truth_for_function(gt_index, name, name2addr[name], arch):
            record["type_match"] = None
        if record:
            functions[f"{entry['project']}/{stem}/{name}"] = record
    return {
        "functions": functions,
        "emitted": emitted,
        "errors": {m: r.errors[:5] for m, r in scored.items() if r.errors},
    }


def _score_task(args: tuple) -> tuple[str, dict]:
    entry = args[0]
    try:
        return entry["binary_path"], score_binary(*args)
    except Exception as e:  # noqa: BLE001
        return entry["binary_path"], {"functions": {}, "fatal": f"{type(e).__name__}: {e}"}


def git_rev(path: Path) -> str | None:
    try:
        return subprocess.run(["git", "-C", str(path), "rev-parse", "HEAD"], capture_output=True,
                              text=True, check=True).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def cpu_model() -> str:
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return platform.processor() or "unknown"


def kuna_version(kuna: Path) -> str:
    p = subprocess.run([str(kuna), "--version"], capture_output=True, text=True)
    return (p.stdout or p.stderr).strip()


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--sample", type=Path, required=True)
    ap.add_argument("--data", type=Path, required=True)
    ap.add_argument("--kuna", type=Path, required=True)
    ap.add_argument("--specs", type=Path, required=True)
    ap.add_argument("--baseline-kuna", type=Path)
    ap.add_argument("--baseline-specs", type=Path)
    ap.add_argument("--baseline-label", default="", help="what the baseline build is (sha/run)")
    ap.add_argument("--label", default="", help="what tonight's build is (sha)")
    ap.add_argument("--decbench-root", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--jobs", type=int, default=os.cpu_count() or 1, help="scoring workers")
    ap.add_argument("--timeout", type=float, default=900, help="per-binary decompile cap (s)")
    args = ap.parse_args(argv)

    os.environ["KUNA_BIN"] = str(args.kuna.resolve())
    sample = json.loads(args.sample.read_text())
    entries = sample["binaries"]
    work = args.out / "work"
    started = datetime.datetime.now(datetime.timezone.utc)

    for entry in entries:
        strip_copy(args.data / entry["binary_path"],
                   work / entry["binary_path"] / "stripped" / Path(entry["binary_path"]).name)

    arms = [("new", args.kuna.resolve(), args.specs.resolve())]
    if args.baseline_kuna:
        arms.append(("baseline", args.baseline_kuna.resolve(),
                     (args.baseline_specs or args.specs).resolve()))
    t0 = time.perf_counter()
    timings = time_sample(entries, work, arms, args.timeout)
    timing_seconds = time.perf_counter() - t0

    stems = {p: sorted({Path(s).stem for s in paths}) for p, paths in sample["sources"].items()}
    tasks = [(e, args.data.resolve(), work.resolve(), args.decbench_root.resolve(),
              stems.get(e["project"], [])) for e in entries]
    t0 = time.perf_counter()
    scored: dict[str, dict] = {}
    ctx = multiprocessing.get_context("spawn")
    with concurrent.futures.ProcessPoolExecutor(max_workers=args.jobs, mp_context=ctx) as pool:
        for key, res in pool.map(_score_task, tasks):
            scored[key] = res
            if res.get("fatal"):
                print(f"score {key}: {res['fatal']}", file=sys.stderr, flush=True)
    scoring_seconds = time.perf_counter() - t0

    functions: dict[str, dict] = {}
    binaries: dict[str, dict] = {}
    for entry in entries:
        key = entry["binary_path"]
        res = scored.get(key, {})
        functions.update(res.get("functions", {}))
        binaries[key] = {
            "time": timings[key]["new"],
            "baseline_time": timings[key].get("baseline"),
            "functions": len(entry["functions"]),
            "emitted": res.get("emitted"),
            "fatal": res.get("fatal"),
            "metric_errors": res.get("errors") or None,
        }

    results = {
        "schema": 1,
        "meta": {
            "label": args.label,
            "kuna_version": kuna_version(args.kuna),
            "baseline_label": args.baseline_label if args.baseline_kuna else None,
            "baseline_version": kuna_version(args.baseline_kuna) if args.baseline_kuna else None,
            "dataset": sample["dataset"],
            "dataset_revision": sample["revision"],
            "config": sample["config"],
            "sample_digest": sample["digest"],
            "sample_seed": sample["seed"],
            "decbench_commit": git_rev(args.decbench_root),
            "rust_joern": os.environ.get("RUST_JOERN_REV"),
            "max_fn_seconds": MAX_FN_SECONDS,
            "cpu_model": cpu_model(),
            "nproc": os.cpu_count(),
            "python": platform.python_version(),
            "started": started.isoformat(timespec="seconds"),
            "timing_seconds": round(timing_seconds, 1),
            "scoring_seconds": round(scoring_seconds, 1),
        },
        "binaries": binaries,
        "functions": functions,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "results.json").write_text(json.dumps(results, indent=1, sort_keys=True) + "\n")
    fatal = [k for k, b in binaries.items() if b["fatal"]]
    print(f"timed {len(entries)} binaries in {timing_seconds:.0f}s, scored "
          f"{len(functions)} functions in {scoring_seconds:.0f}s; {len(fatal)} scoring failures",
          file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
