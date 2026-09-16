"""Price one feature's option on `type_match` and splice the block into its record.

Every type-recovery PR has to answer the same question — *over the corpus, how
many functions did this option move to a perfect type recovery, and how many did
it move off?* — in the same shape, so the answers can be read side by side. This
runs that measurement for a feature bundle and emits the ``benchmark`` block of
``docs/features/libcsigs/record.json``:

    per project: n, perfect_off, perfect_on, mean_off, mean_on, improved, worse
    plus the same seven pooled, the decbench commit the metric came from, and
    the DECBENCH_NO_CACHE note.

The measurement itself is ``scripts.decbench.typesweep`` — the same workers, the
same ``TypeMatchMetric.compute_for_binary`` call, the same harness controls. The
one difference is that both arms carry the option **explicitly** (``off`` and
``on``), so the numbers do not silently depend on what the build's default was
on the day it ran.

Usage (needs the decbench venv python)::

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.typescore \\
        --record docs/features/<slug>/record.json          # print the block
    ... --write                                            # splice it in
    ... --project bzip2 --opt O2                           # narrow the corpus
    ... --option libcsigs --off-value off --on-value on     # no record needed
    ... --report-only                                      # re-report from --out
    python3 -m scripts.decbench.typescore --selftest        # no decbench needed

Which corpus is measured, in order of precedence: ``--project``/``--opt``, then
the record's own ``benchmark.projects`` keys (so a re-measurement covers exactly
what the last one did), then the five projects below.

Nothing is written anywhere unless ``--out`` (the sweep's working directory, by
default a cache under $HOME) or ``--write`` (the record) is given.
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

from . import config, typesweep

# The five C projects the libcsigs block measured: coreutils is 327 binaries x 2
# arms (~11h) and dash carries no DWARF ground truth in the frozen tree.
DEFAULT_PROJECTS = ("grep", "diffutils", "gzip", "bzip2", "findutils")
DEFAULT_OPTS = ("O0", "O2", "O2-noinline")

WORKDIR = Path(os.environ.get(
    "KUNA_TYPESCORE_CACHE",
    str(Path.home() / ".cache" / "decbench" / "kuna-typescore"),
))


def decbench_commit() -> dict:
    """The decbench checkout the metric is being read from."""
    repo = config.decbench_repo()
    out = {"repo": str(repo)}
    for key, cmd in (("sha", ["rev-parse", "--short", "HEAD"]),
                     ("branch", ["rev-parse", "--abbrev-ref", "HEAD"])):
        try:
            p = subprocess.run(["git", "-C", str(repo)] + cmd, capture_output=True,
                               text=True)
            out[key] = p.stdout.strip() if p.returncode == 0 else None
        except OSError:
            out[key] = None
    return out


def arm_values(record: dict, off: str | None, on: str | None) -> tuple[str, str]:
    """The (off, on) values of the record's option.

    ``values`` is the ``phases.toml`` spelling (``on|off``, ``off|byte|void``);
    the disabled value is whichever of them is ``off``, and the enabled value is
    the record's ``default_value`` unless that IS ``off``, in which case it is
    the first value that is not.
    """
    values = [v.strip() for v in str(record.get("values") or "on|off").split("|") if v.strip()]
    off_value = off or ("off" if "off" in values else values[0])
    if on:
        return off_value, on
    default = str(record.get("default_value") or "").strip()
    if default and default != off_value:
        return off_value, default
    rest = [v for v in values if v != off_value]
    if not rest:
        raise SystemExit(f"option {record.get('option')!r} has no enabled value in "
                         f"{record.get('values')!r} -- pass --on-value")
    return off_value, rest[0]


def corpus(record: dict, projects: list[str], opts: list[str]) -> tuple[list[str], list[str]]:
    if not projects:
        prior = ((record.get("benchmark") or {}).get("projects") or {})
        projects = [p for p in prior if p != "pooled"] or list(DEFAULT_PROJECTS)
    return sorted(projects), sorted(opts or DEFAULT_OPTS)


def summarize(rows: dict) -> dict:
    """typesweep rows -> the record's ``benchmark.projects`` block.

    ``base`` is the option's OFF arm and ``test`` its ON arm (typescore hands
    both arms an explicit value). A function counts only when both arms scored
    it; ``improved``/``worse`` are the functions whose score moved either way,
    perfect or not.
    """
    per: dict[str, dict] = {}
    for key, arms in sorted(rows.items()):
        project = key.split("::")[0]
        off = (arms.get("base") or {}).get("values") or {}
        on = (arms.get("test") or {}).get("values") or {}
        acc = per.setdefault(project, {"n": 0, "perfect_off": 0, "perfect_on": 0,
                                       "_sum_off": 0.0, "_sum_on": 0.0,
                                       "improved": 0, "worse": 0})
        for fn, ov in off.items():
            nv = on.get(fn)
            if nv is None:
                continue
            acc["n"] += 1
            acc["_sum_off"] += ov
            acc["_sum_on"] += nv
            acc["perfect_off"] += int(ov == 1.0)
            acc["perfect_on"] += int(nv == 1.0)
            acc["improved"] += int(nv > ov)
            acc["worse"] += int(nv < ov)
    pooled = {"n": 0, "perfect_off": 0, "perfect_on": 0, "_sum_off": 0.0,
              "_sum_on": 0.0, "improved": 0, "worse": 0}
    for acc in per.values():
        for k in pooled:
            pooled[k] += acc[k]
    out = {}
    for name, acc in list(per.items()) + [("pooled", pooled)]:
        n = max(1, acc["n"])
        out[name] = {
            "n": acc["n"],
            "perfect_off": acc["perfect_off"],
            "perfect_on": acc["perfect_on"],
            "mean_off": round(acc["_sum_off"] / n, 4),
            "mean_on": round(acc["_sum_on"] / n, 4),
            "improved": acc["improved"],
            "worse": acc["worse"],
        }
    return out


def control(rows: dict) -> dict:
    """typesweep's two harness controls, carried into the block.

    ``identical_vars_scored_identically`` is the one that invalidates a run: two
    arms with the same variable count scoring differently means the numbers came
    from somewhere other than the option.
    """
    same = diff = 0
    for arms in rows.values():
        off, on = arms.get("base") or {}, arms.get("test") or {}
        for fn, nv in (off.get("nvars") or {}).items():
            if (on.get("nvars") or {}).get(fn) != nv:
                continue
            ov = (off.get("values") or {}).get(fn)
            tv = (on.get("values") or {}).get(fn)
            if ov is None or tv is None:
                continue
            if ov == tv:
                same += 1
            else:
                diff += 1
    return {"same_variable_count_functions": same + diff,
            "same_variable_count_scored_differently": diff}


def benchmark_block(rows: dict, option: str, off: str, on: str,
                    projects: list[str], opts: list[str], errors: list[str]) -> dict:
    dc = decbench_commit()
    return {
        "corpus": (f"decbench corpus at {config.results_root()}, optimisation levels "
                   f"{', '.join(opts)}, projects {', '.join(projects)}, scored with "
                   f"decbench {dc.get('branch')} @{dc.get('sha')}, DECBENCH_NO_CACHE=1"),
        "metric": "type_match",
        "option": f"{option} {off} (off arm) vs {option} {on} (on arm)",
        "kuna_bin": config.kuna_bin(),
        "projects": summarize(rows),
        "pooled": summarize(rows).get("pooled", {}),
        "control": control(rows),
        "note": ("Both arms pass the option explicitly, so the numbers do not depend on "
                 "the build's default. DECBENCH_NO_CACHE=1 is set by typescore itself -- "
                 "the metric's content-addressed cache otherwise serves a stored value. "
                 "Functions only one arm scored are excluded."
                 + (f" Slice errors: {'; '.join(errors[:5])}" if errors else "")),
    }


def run(option: str, off: str, on: str, projects: list[str], opts: list[str],
        out: Path, workers: int, timeout: int, limit: int,
        report_only: bool) -> tuple[dict, list[str]]:
    out.mkdir(parents=True, exist_ok=True)
    cache = out / "rows.json"
    rows = json.loads(cache.read_text()) if cache.exists() else {}
    errors: list[str] = []
    if not report_only:
        slices = typesweep.collect_slices(config.results_root(), set(projects), set(opts))
        if limit:
            slices = slices[:limit]
        arms = {"base": [(option, off)], "test": [(option, on)]}
        todo = [(p, o, s, b, a, [], timeout, arms, False)
                for (p, o, s, b, a) in slices
                if typesweep.slice_key(p, o, s) not in rows]
        print(f"slices: {len(slices)} total, {len(todo)} to run, "
              f"arms={{off: {option} {off}, on: {option} {on}}}", flush=True)
        done = 0
        with ProcessPoolExecutor(max_workers=workers) as ex:
            futs = {ex.submit(typesweep.score_slice, t): t for t in todo}
            for fut in as_completed(futs):
                done += 1
                try:
                    key, value = fut.result()
                except Exception as e:  # noqa: BLE001
                    errors.append(f"worker: {str(e)[:120]}")
                    print(f"[{done}/{len(todo)}] WORKER-ERR {e}", flush=True)
                    continue
                rows[key] = value
                for arm in ("base", "test"):
                    if (value.get(arm) or {}).get("error"):
                        errors.append(f"{key}/{arm}: {value[arm]['error'][:120]}")
                cache.write_text(json.dumps(rows))
                print(f"[{done}/{len(todo)}] {key}", flush=True)
    return rows, errors


def splice(record_path: Path, block: dict) -> None:
    record = json.loads(record_path.read_text())
    record["benchmark"] = block
    record_path.write_text(json.dumps(record, indent=2) + "\n")


def selftest() -> int:
    """Aggregation-only checks; they need no decbench, no corpus and no kuna."""
    rows = {
        "grep::O0::grep": {
            "base": {"values": {"a": 1.0, "b": 0.5, "c": 0.0, "only_off": 0.25},
                     "nvars": {"a": 3, "b": 3, "c": 3}},
            "test": {"values": {"a": 0.75, "b": 1.0, "c": 0.0},
                     "nvars": {"a": 3, "b": 4, "c": 3}},
        },
        "gzip::O2::gzip": {
            "base": {"values": {"d": 0.0}, "nvars": {"d": 2}},
            "test": {"values": {"d": 1.0}, "nvars": {"d": 2}},
        },
    }
    got = summarize(rows)
    ctl = control(rows)
    checks = [
        ("a function only one arm scored is excluded", got["grep"]["n"] == 3),
        ("perfect counted per arm",
         (got["grep"]["perfect_off"], got["grep"]["perfect_on"]) == (1, 1)),
        ("means over the shared set",
         (got["grep"]["mean_off"], got["grep"]["mean_on"]) == (0.5, 0.5833)),
        ("improved/worse count both directions",
         (got["grep"]["improved"], got["grep"]["worse"]) == (1, 1)),
        ("pooled sums the projects",
         (got["pooled"]["n"], got["pooled"]["perfect_on"], got["pooled"]["improved"])
         == (4, 2, 2)),
        ("the control only looks at same-variable-count functions",
         (ctl["same_variable_count_functions"],
          ctl["same_variable_count_scored_differently"]) == (3, 2)),
        ("off/on values from a record",
         arm_values({"values": "on|off", "default_value": "on"}, None, None) == ("off", "on")),
        ("a three-valued option picks the first non-off",
         arm_values({"values": "off|byte|void"}, None, None) == ("off", "byte")),
        ("an explicit on-value wins",
         arm_values({"values": "off|byte|void"}, None, "void") == ("off", "void")),
        ("corpus falls back to the record's own projects",
         corpus({"benchmark": {"projects": {"gzip": {}, "pooled": {}}}}, [], [])
         == (["gzip"], sorted(DEFAULT_OPTS))),
    ]
    for name, ok in checks:
        print(f"[{'ok ' if ok else 'FAIL'}] {name}")
    bad = [n for n, ok in checks if not ok]
    print(f"selftest: {len(checks) - len(bad)}/{len(checks)} passed")
    return 1 if bad else 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Writes nothing unless --out or --write is given.")
    ap.add_argument("--record", type=Path, help="docs/features/<slug>/record.json")
    ap.add_argument("--option", help="option name (default: the record's)")
    ap.add_argument("--off-value", help="the disabled value (default: off)")
    ap.add_argument("--on-value", help="the enabled value (default: the record's)")
    ap.add_argument("--project", action="append", default=[])
    ap.add_argument("--opt", action="append", default=[])
    ap.add_argument("--workers", type=int, default=12)
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--limit", type=int, default=0, help="cap the slices (debug)")
    ap.add_argument("--out", type=Path, help="sweep working directory")
    ap.add_argument("--report-only", action="store_true",
                    help="re-report from --out's rows.json")
    ap.add_argument("--write", action="store_true",
                    help="splice the block into --record")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    record = json.loads(args.record.read_text()) if args.record else {}
    option = args.option or record.get("option")
    if not option:
        ap.error("need --option or a --record carrying one")
    off, on = arm_values(record, args.off_value, args.on_value)
    projects, opts = corpus(record, args.project, args.opt)

    # The metric caches by content; a stale entry would report the wrong arm.
    os.environ.setdefault("DECBENCH_NO_CACHE", "1")
    typesweep._imports()

    out = args.out or (WORKDIR / f"{option}-{off}-{on}")
    rows, errors = run(option, off, on, projects, opts, out, args.workers,
                       args.timeout, args.limit, args.report_only)
    block = benchmark_block(rows, option, off, on, projects, opts, errors)
    print(json.dumps(block, indent=2))
    if args.write:
        if not args.record:
            ap.error("--write needs --record")
        splice(args.record, block)
        print(f"[typescore] spliced benchmark block into {args.record}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
