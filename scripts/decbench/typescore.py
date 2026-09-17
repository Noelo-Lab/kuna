"""Price one feature's option on `type_match` and splice the block into its record.

Every type-recovery PR has to answer the same question — *over the corpus, how
many functions did this option move to a perfect type recovery, and how many did
it move off?* — in the same shape, so the answers can be read side by side. This
runs that measurement for a feature bundle and emits the ``benchmark`` block of
``docs/features/libcsigs/record.json``:

    projects: {<project>: n, perfect_off, perfect_on, mean_off, mean_on,
               improved, worse}          (projects only — pooled is its own key)
    pooled:   the same seven over every project
    control:  the harness controls, of which only
              ``identical_variables_scored_differently`` can invalidate a run
    plus the decbench commit the metric came from and the DECBENCH_NO_CACHE note.

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

The cached rows carry the provenance they were measured under — the kuna binary
by content, the decbench commit, the results tree and both arm values — and a
cache that disagrees with the current run is dropped rather than mixed into it.
Everything the block says about the corpus is derived from the rows it
summarizes, so a re-report can never name a project or a binary that did not run.

Nothing is written anywhere unless ``--out`` (the sweep's working directory, by
default a per-option, per-corpus cache under $HOME) or ``--write`` (the record)
is given.
"""
from __future__ import annotations

import argparse
import hashlib
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


def binary_identity(path: str) -> dict:
    """Which kuna produced a row, by content — a path is not an identity.

    `make binaries` rewrites the same path in place, so a cache keyed on the
    path alone republishes yesterday's rows under today's binary.
    """
    out = {"path": str(path), "sha256": None, "size": None}
    try:
        data = Path(path).read_bytes()
    except OSError:
        return out
    out["sha256"] = hashlib.sha256(data).hexdigest()[:16]
    out["size"] = len(data)
    return out


def provenance(option: str, off: str, on: str) -> dict:
    """Everything a row's numbers depend on other than the corpus slice itself."""
    return {
        "kuna": binary_identity(config.kuna_bin()),
        "decbench": decbench_commit(),
        "results_root": str(config.results_root()),
        "option": option, "off_value": off, "on_value": on,
        "metric": "type_match",
    }


def corpus_key(projects: list[str], opts: list[str]) -> str:
    """The corpus half of the default cache directory name.

    Without it one directory holds every corpus the option was ever measured on,
    and a narrower ``--project`` re-reports the wider run's rows.
    """
    seed = "|".join([str(config.results_root())] + sorted(projects) + sorted(opts))
    return hashlib.sha1(seed.encode()).hexdigest()[:8]


def rows_corpus(rows: dict) -> tuple[list[str], list[str]]:
    """The projects and opt levels the ROWS cover — never the requested ones."""
    projects, opts = set(), set()
    for key, value in rows.items():
        sl = value.get("slice") or {}
        parts = key.split("::")
        projects.add(sl.get("project") or (parts[0] if parts else "?"))
        opts.add(sl.get("opt") or (parts[1] if len(parts) > 1 else "?"))
    return sorted(projects), sorted(opts)


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
        projects = list(prior) or list(DEFAULT_PROJECTS)
    return sorted(projects), sorted(opts or DEFAULT_OPTS)


def summarize(rows: dict) -> tuple[dict, dict]:
    """typesweep rows -> ``(benchmark.projects, benchmark.pooled)``.

    ``base`` is the option's OFF arm and ``test`` its ON arm (typescore hands
    both arms an explicit value). A function counts only when both arms scored
    it; ``improved``/``worse`` are the functions whose score moved either way,
    perfect or not. ``projects`` carries the projects and nothing else — the
    pooled row is returned separately because the libcsigs record keeps it at
    the top level, and anything that sums ``projects`` must not double-count it.
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

    def row(acc: dict) -> dict:
        n = max(1, acc["n"])
        return {
            "n": acc["n"],
            "perfect_off": acc["perfect_off"],
            "perfect_on": acc["perfect_on"],
            "mean_off": round(acc["_sum_off"] / n, 4),
            "mean_on": round(acc["_sum_on"] / n, 4),
            "improved": acc["improved"],
            "worse": acc["worse"],
        }

    return {name: row(acc) for name, acc in per.items()}, row(pooled)


def control(rows: dict, published: dict | None = None) -> dict:
    """typesweep's harness controls, carried into the block.

    Only ``identical_variables_scored_differently`` can invalidate a run. Two
    arms that handed the metric byte-identical ``variables`` — the same names,
    types, sizes, stack offsets and argument positions, in the same order — must
    produce the same score; anything else means the numbers came from somewhere
    other than the option. It must be 0.

    The other two fields are descriptive, not gates. ``retyped_functions``
    counts the functions the option actually rewrote, which is the *expected*
    signal for a retyping option and says nothing about validity;
    ``off_arm_vs_published`` is typesweep's second control, the off arm against
    the tree's stored ``type_match`` verdict. It agrees only where the off arm
    is what the tree was scored with, so a low rate means kuna has moved since
    (or the off value is not the default), not that the run is bad.
    """
    identical = identical_diff = retyped = unknown = 0
    for arms in rows.values():
        off, on = arms.get("base") or {}, arms.get("test") or {}
        off_sig, on_sig = off.get("vars_sig") or {}, on.get("vars_sig") or {}
        for fn, ov in (off.get("values") or {}).items():
            tv = (on.get("values") or {}).get(fn)
            if tv is None:
                continue
            a, b = off_sig.get(fn), on_sig.get(fn)
            if a is None or b is None:
                unknown += 1
            elif a == b:
                identical += 1
                identical_diff += int(abs(tv - ov) >= 1e-9)
            else:
                retyped += 1
    out = {
        "identical_variables_functions": identical,
        "identical_variables_scored_differently": identical_diff,
        "retyped_functions": retyped,
        "functions_without_a_variable_signature": unknown,
    }
    if published is not None:
        agree = differ = 0
        for key, arms in rows.items():
            for fn, ov in ((arms.get("base") or {}).get("values") or {}).items():
                pv = published.get(f"{key}::{fn}")
                if pv is None:
                    continue
                if abs(pv - ov) < 1e-9:
                    agree += 1
                else:
                    differ += 1
        out["off_arm_vs_published"] = {
            "agree": agree, "differ": differ,
            "rate": round(agree / (agree + differ), 4) if agree + differ else None,
        }
    return out


def benchmark_block(rows: dict, prov: dict, errors: list[str],
                    published: dict | None = None) -> dict:
    """The record's ``benchmark`` block, every field derived from ``rows``.

    ``prov`` is the provenance the rows were MEASURED under (the stored one on a
    ``--report-only`` re-report, the current one otherwise), and the corpus line
    names the projects and levels the rows actually cover — not the ones this
    invocation asked for. A block that says grep while holding bzip2's numbers
    is a wrong-numbers channel, and this is the only thing standing in its way.
    """
    dc = prov.get("decbench") or {}
    projects, opts = rows_corpus(rows)
    per_project, pooled = summarize(rows)
    kuna = prov.get("kuna") or {}
    return {
        "corpus": (f"decbench corpus at {prov.get('results_root')}, optimisation levels "
                   f"{', '.join(opts) or 'none'}, projects {', '.join(projects) or 'none'}, "
                   f"{len(rows)} slices, scored with decbench {dc.get('branch')} "
                   f"@{dc.get('sha')}, DECBENCH_NO_CACHE=1"),
        "metric": prov.get("metric", "type_match"),
        "option": (f"{prov.get('option')} {prov.get('off_value')} (off arm) vs "
                   f"{prov.get('option')} {prov.get('on_value')} (on arm)"),
        "kuna_bin": kuna.get("path"),
        "kuna_sha256": kuna.get("sha256"),
        "projects": per_project,
        "pooled": pooled,
        "control": control(rows, published),
        "note": ("Both arms pass the option explicitly, so the numbers do not depend on "
                 "the build's default. DECBENCH_NO_CACHE=1 is set by typescore itself -- "
                 "the metric's content-addressed cache otherwise serves a stored value. "
                 "Functions only one arm scored are excluded. Every field above is "
                 "derived from the cached rows, which carry the provenance they were "
                 "measured under; rows from another binary, decbench or arm are dropped."
                 + (f" Slice errors: {'; '.join(errors[:5])}" if errors else "")),
    }


def load_rows(cache: Path, want: dict, projects: list[str], opts: list[str],
              report_only: bool) -> tuple[dict, dict | None]:
    """Cached rows plus the provenance they were measured under.

    A cache file that predates provenance, or one measured with another kuna,
    decbench or pair of arm values, is dropped rather than mixed into this run.
    On a ``--report-only`` re-report nothing is dropped — the rows are what there
    is to report — but a provenance that differs from the environment is said out
    loud, and it is the STORED one that stamps the block.
    """
    if not cache.exists():
        return {}, None
    doc = json.loads(cache.read_text())
    stored = doc.get("provenance") if isinstance(doc, dict) else None
    rows = doc.get("rows") if isinstance(stored, dict) else None
    if rows is None:
        print(f"[typescore] {cache} carries no provenance -- ignoring it and "
              f"re-measuring", file=sys.stderr)
        return {}, None
    if report_only:
        if stored != want:
            print(f"[typescore] REPORTING STORED ROWS: they were measured with "
                  f"{json.dumps(stored)}, not with the current "
                  f"{json.dumps(want)}", file=sys.stderr)
        return rows, stored
    if stored != want:
        print(f"[typescore] cached rows were measured with {json.dumps(stored)} -- "
              f"dropping all {len(rows)} and re-measuring", file=sys.stderr)
        return {}, None
    keep = {k: v for k, v in rows.items()
            if ((v.get("slice") or {}).get("project") or k.split("::")[0]) in projects
            and ((v.get("slice") or {}).get("opt") or k.split("::")[1]) in opts}
    if len(keep) != len(rows):
        print(f"[typescore] {len(rows) - len(keep)} cached row(s) outside this run's "
              f"corpus -- not reported", file=sys.stderr)
    return keep, stored


def run(option: str, off: str, on: str, projects: list[str], opts: list[str],
        out: Path, workers: int, timeout: int, limit: int,
        report_only: bool) -> tuple[dict, list[str], dict]:
    out.mkdir(parents=True, exist_ok=True)
    cache = out / "rows.json"
    want = provenance(option, off, on)
    rows, stored = load_rows(cache, want, projects, opts, report_only)
    prov = stored if report_only and stored else want
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
                task = futs[fut]
                done += 1
                try:
                    key, value = fut.result()
                except Exception as e:  # noqa: BLE001
                    errors.append(f"worker: {str(e)[:120]}")
                    print(f"[{done}/{len(todo)}] WORKER-ERR {e}", flush=True)
                    continue
                value["slice"] = {"project": task[0], "opt": task[1], "stem": task[2]}
                rows[key] = value
                for arm in ("base", "test"):
                    if (value.get(arm) or {}).get("error"):
                        errors.append(f"{key}/{arm}: {value[arm]['error'][:120]}")
                cache.write_text(json.dumps({"provenance": want, "rows": rows}))
                print(f"[{done}/{len(todo)}] {key}", flush=True)
    return rows, errors, prov


def splice(record_path: Path, block: dict) -> None:
    record = json.loads(record_path.read_text())
    record["benchmark"] = block
    record_path.write_text(json.dumps(record, indent=2) + "\n")


class _V:
    """A stand-in for decbench's VariableInfo, for the signature check below."""

    def __init__(self, name, type_):
        self.name, self.type = name, type_
        self.size = self.stack_offset = self.arg_index = self.kind = None


def provenance_checks() -> list:
    """The cache-provenance checks: the reviewer's own wrong-numbers scenario."""
    import tempfile
    mine = {"kuna": {"path": "/tmp/new-kuna", "sha256": "bbbb", "size": 2},
            "decbench": {"sha": "cafe", "branch": "main"},
            "results_root": "/results", "option": "libcsigs", "off_value": "off",
            "on_value": "on", "metric": "type_match"}
    theirs = {**mine, "kuna": {"path": "/tmp/old-kuna", "sha256": "aaaa", "size": 1}}
    row = {"base": {"values": {"f": 0.0}}, "test": {"values": {"f": 1.0}},
           "slice": {"project": "bzip2", "opt": "O0", "stem": "bzip2"}}
    out = []
    with tempfile.TemporaryDirectory() as tmp:
        cache = Path(tmp) / "rows.json"
        cache.write_text(json.dumps({"bzip2::O0::bzip2": row}))
        legacy, _ = load_rows(cache, mine, ["bzip2"], ["O0"], False)
        cache.write_text(json.dumps({"provenance": theirs,
                                     "rows": {"bzip2::O0::bzip2": row}}))
        stale, _ = load_rows(cache, mine, ["bzip2"], ["O0"], False)
        kept, stored = load_rows(cache, mine, ["grep"], ["O0"], True)
        cache.write_text(json.dumps({"provenance": mine,
                                     "rows": {"bzip2::O0::bzip2": row}}))
        narrowed, _ = load_rows(cache, mine, ["grep"], ["O0"], False)
        block = benchmark_block(kept, stored, [])
        out += [
            ("a cache with no provenance is ignored", legacy == {}),
            ("rows measured with another kuna are dropped, not merged", stale == {}),
            ("--report-only reports the stored rows under the STORED provenance",
             len(kept) == 1 and stored == theirs),
            ("rows outside the requested corpus are not reported", narrowed == {}),
            ("the block names the corpus the ROWS cover, not the one requested",
             "projects bzip2" in block["corpus"] and "grep" not in block["corpus"]
             and set(block["projects"]) == {"bzip2"}),
            ("the block names the kuna that produced the rows",
             (block["kuna_bin"], block["kuna_sha256"]) == ("/tmp/old-kuna", "aaaa")),
            ("the default cache key separates two corpora",
             corpus_key(["grep"], ["O0"]) != corpus_key(["bzip2"], ["O0"])),
        ]
    return out


def selftest() -> int:
    """Aggregation-only checks; they need no decbench, no corpus and no kuna."""
    rows = {
        "grep::O0::grep": {
            # `a` keeps its variables (a harness bug if it moves), `b` and `c`
            # are retyped, `only_off` is scored by one arm only.
            "base": {"values": {"a": 1.0, "b": 0.5, "c": 0.0, "only_off": 0.25},
                     "vars_sig": {"a": "aa", "b": "bb", "c": "cc"}},
            "test": {"values": {"a": 0.75, "b": 1.0, "c": 0.0},
                     "vars_sig": {"a": "aa", "b": "b2", "c": "c2"}},
        },
        "gzip::O2::gzip": {
            "base": {"values": {"d": 0.0}, "vars_sig": {"d": "dd"}},
            "test": {"values": {"d": 1.0}, "vars_sig": {"d": "d2"}},
        },
    }
    got, pooled = summarize(rows)
    ctl = control(rows, {"grep::O0::grep::a": 1.0, "grep::O0::grep::b": 0.25})
    sig = typesweep.variables_signature
    checks = [
        ("a function only one arm scored is excluded", got["grep"]["n"] == 3),
        ("perfect counted per arm",
         (got["grep"]["perfect_off"], got["grep"]["perfect_on"]) == (1, 1)),
        ("means over the shared set",
         (got["grep"]["mean_off"], got["grep"]["mean_on"]) == (0.5, 0.5833)),
        ("improved/worse count both directions",
         (got["grep"]["improved"], got["grep"]["worse"]) == (1, 1)),
        ("pooled sums the projects",
         (pooled["n"], pooled["perfect_on"], pooled["improved"]) == (4, 2, 2)),
        ("pooled is NOT an entry of projects", set(got) == {"grep", "gzip"}),
        ("the control flags only byte-identical variables that moved",
         (ctl["identical_variables_functions"],
          ctl["identical_variables_scored_differently"],
          ctl["retyped_functions"]) == (1, 1, 3)),
        ("the published control compares the off arm",
         (ctl["off_arm_vs_published"]["agree"],
          ctl["off_arm_vs_published"]["differ"]) == (1, 1)),
        ("a variable signature is order- and field-sensitive",
         sig([_V("a", "int"), _V("b", "char *")]) != sig([_V("b", "char *"), _V("a", "int")])
         and sig([_V("a", "int")]) != sig([_V("a", "long")])
         and sig([_V("a", "int")]) == sig([_V("a", "int")])),
        ("off/on values from a record",
         arm_values({"values": "on|off", "default_value": "on"}, None, None) == ("off", "on")),
        ("a three-valued option picks the first non-off",
         arm_values({"values": "off|byte|void"}, None, None) == ("off", "byte")),
        ("an explicit on-value wins",
         arm_values({"values": "off|byte|void"}, None, "void") == ("off", "void")),
        ("corpus falls back to the record's own projects",
         corpus({"benchmark": {"projects": {"gzip": {}}}}, [], [])
         == (["gzip"], sorted(DEFAULT_OPTS))),
    ] + provenance_checks()
    for name, ok in checks:
        print(f"[{'ok ' if ok else 'FAIL'}] {name}")
    bad = [n for n, ok in checks if not ok]
    print(f"selftest: {len(checks) - len(bad)}/{len(checks)} passed")
    return 1 if bad else 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="A measuring run writes rows.json into --out, or into "
               "$KUNA_TYPESCORE_CACHE (default ~/.cache/decbench/kuna-typescore/"
               "<option>-<off>-<on>-<corpus>/) when --out is not given. Nothing "
               "is written inside the repo or the decbench results tree; only "
               "--write touches a tracked file, the --record it splices.")
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

    out = args.out or (WORKDIR / f"{option}-{off}-{on}-{corpus_key(projects, opts)}")
    rows, errors, prov = run(option, off, on, projects, opts, out, args.workers,
                             args.timeout, args.limit, args.report_only)
    try:
        pub = typesweep.published(config.results_root())
    except Exception as e:  # noqa: BLE001
        print(f"[typescore] no published verdicts to control against ({e})",
              file=sys.stderr)
        pub = None
    block = benchmark_block(rows, prov, errors, pub)
    print(json.dumps(block, indent=2))
    if args.write:
        if not args.record:
            ap.error("--write needs --record")
        splice(args.record, block)
        print(f"[typescore] spliced benchmark block into {args.record}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
