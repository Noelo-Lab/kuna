"""Roll the cargo-llvm-cov per-file summary up into coverage.json: totals,
per crate, per kuna-decomp phase folder, per file. Invoked by
scripts/coverage.sh; unit tests feed canned summaries."""
import argparse
import json
from pathlib import Path

from scripts.observatory import common

CRATES_PREFIX = "decompiler/crates/"
DECOMP_SRC = "decompiler/crates/kuna-decomp/src/"


def _bucket(path: str):
    crate = phase = None
    if path.startswith(CRATES_PREFIX):
        crate = path[len(CRATES_PREFIX) :].split("/", 1)[0]
    if path.startswith(DECOMP_SRC):
        top = path[len(DECOMP_SRC) :].split("/", 1)[0]
        if top.endswith(".rs"):
            top = "(crate root)"
        phase = f"P{top[1]}" if top[0] == "p" and top[1:2].isdigit() else top
    return crate, phase


def _add(stats: dict, lines: dict, functions: dict) -> None:
    stats["lines"]["count"] += lines["count"]
    stats["lines"]["covered"] += lines["covered"]
    stats["functions"]["count"] += functions["count"]
    stats["functions"]["covered"] += functions["covered"]


def _fresh() -> dict:
    return {"lines": {"count": 0, "covered": 0}, "functions": {"count": 0, "covered": 0}}


def _pct(stats: dict) -> None:
    for key in ("lines", "functions"):
        count = stats[key]["count"]
        stats[key]["percent"] = round(100.0 * stats[key]["covered"] / count, 2) if count else 0.0


def rollup(summary: dict, root: Path, runs_failed, meta: dict) -> dict:
    totals = _fresh()
    crates: dict = {}
    phases: dict = {}
    files = []
    for entry in summary["data"][0]["files"]:
        path = entry["filename"]
        try:
            rel = str(Path(path).relative_to(root))
        except ValueError:
            continue
        lines = entry["summary"]["lines"]
        functions = entry["summary"]["functions"]
        crate, phase = _bucket(rel)
        _add(totals, lines, functions)
        if crate:
            _add(crates.setdefault(crate, _fresh()), lines, functions)
        if phase:
            _add(phases.setdefault(phase, _fresh()), lines, functions)
        files.append(
            {
                "path": rel,
                "crate": crate,
                "phase": phase,
                "lines": {"count": lines["count"], "covered": lines["covered"]},
            }
        )
    _pct(totals)
    for stats in list(crates.values()) + list(phases.values()):
        _pct(stats)
    return {
        "schemaVersion": common.SCHEMA_VERSION,
        "meta": {**meta, "runsFailed": bool(int(runs_failed))},
        "totals": totals,
        "crates": [{"name": name, **stats} for name, stats in sorted(crates.items())],
        "phases": [{"folder": name, **stats} for name, stats in sorted(phases.items())],
        "files": sorted(files, key=lambda item: item["path"]),
    }


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(prog="python -m scripts.observatory.coverage")
    parser.add_argument("--summary", type=Path, required=True)
    parser.add_argument("--runs-failed", default="0")
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args(argv)
    root = common.repo_root()
    summary = json.loads(args.summary.read_text(encoding="utf-8"))
    data = rollup(summary, root, args.runs_failed, common.meta(root))
    common.write_json(args.out / "data" / "coverage.json", data)
    lines = data["totals"]["lines"]
    print(
        f"coverage: {lines['covered']}/{lines['count']} lines"
        f" ({lines['percent']}%) -> {args.out}/data/coverage.json"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
