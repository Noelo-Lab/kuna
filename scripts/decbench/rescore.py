"""Per-function GED before/after — the benchmark gate for campaign PRs.

The decbench analog of ``scripts.pipeline.timeit``: where timeit records a
feature's speed cost, rescore records its benchmark (GED) win on the exact
mined case, spliced into the feature's ``record.json``.

MUST run under the decbench venv python (imports decbench.* + pyjoern):

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
        --case O2-coreutils-chown-chown_files                # self-check + fresh delta
    ... --option <name> on --record docs/features/<slug>/record.json --siblings

How a value is computed (mirrors scripts/reeval_ged.py in the decbench repo):
- source CFG: Joern on the project's O0 header-stripped ``.i`` (source is
  opt-independent); cached content-addressed under ~/.cache/decbench/kuna-src-cfg.
- BEFORE: Joern on the STORED run artifact (decompiled/kuna_<stem>.c); this
  recomputes the recorded run value (printed as a METRIC-DRIFT warning if they
  disagree — e.g. the GED code changed since the run).
- AFTER: fresh `kuna decompile-all <stripped> --json --addr 0x<addr>` with the
  current binary (+ any --option pairs), self-relabeled exactly like the run
  (own name only), SPLICED over the function's block inside a copy of the
  stored artifact (full-file context avoids single-vs-full Joern parse skew),
  then Joern + GED like BEFORE.
"""
from __future__ import annotations

import argparse
import json
import math
import os
import pickle
import re
import subprocess
import sys
import tempfile
from pathlib import Path

from . import config
from .triage import MARKER, load_case, split_blocks

SRC_CACHE = Path(os.environ.get(
    "KUNA_DECBENCH_SRC_CACHE",
    str(Path.home() / ".cache" / "decbench" / "kuna-src-cfg"),
))


def _decbench_imports():
    try:
        from decbench.metrics.ged import GEDMetric
        from decbench.utils.cfg import extract_cfgs_from_source, strip_system_headers
        from decbench.caching import stable_hash
    except ImportError as e:
        sys.exit(
            f"cannot import decbench ({e}) — run under the decbench venv python:\n"
            f"  {config.decbench_python()} -m scripts.decbench.rescore ..."
        )
    return GEDMetric, extract_cfgs_from_source, strip_system_headers, stable_hash


def _cached_cfgs(path: Path, text_for_key: str, extract, stable_hash) -> dict:
    """Joern CFGs for one file, content-cached (never stale)."""
    SRC_CACHE.mkdir(parents=True, exist_ok=True)
    key = stable_hash("kuna-src-cfg-v1", text_for_key)
    pkl = SRC_CACHE / f"{key}.pkl"
    if pkl.exists():
        try:
            return pickle.loads(pkl.read_bytes())
        except Exception:  # noqa: BLE001
            pkl.unlink(missing_ok=True)
    cfgs = extract(path) or {}
    try:
        pkl.write_bytes(pickle.dumps(cfgs))
    except Exception:  # noqa: BLE001
        pass
    return cfgs


def source_cfg(case: dict, extract, strip, stable_hash):
    """Find + parse the .i that defines the case function (O0; opt-independent)."""
    comp = config.results_root() / "O0" / case["project"] / "compiled"
    fn = case["function"]
    candidates = [
        f for f in sorted(comp.glob("*.i"))
        if "conftest" not in f.name and not f.name.startswith("a-")
    ]
    # Cheap textual pre-filter on the header-stripped text, then Joern.
    likely, rest = [], []
    probe = re.compile(rf"\b{re.escape(fn)}\s*\(")
    stripped_texts: dict[Path, str] = {}
    for f in candidates:
        text = strip(f.read_text(errors="replace"))
        stripped_texts[f] = text
        (likely if probe.search(text) else rest).append(f)
    for f in likely + rest:
        cfgs = _cached_cfgs(f, stripped_texts[f], extract, stable_hash)
        if fn in cfgs:
            return cfgs[fn], f
    return None, None


def ged_of(source, decompiled, GEDMetric) -> tuple[float, dict]:
    mv = GEDMetric().compute_for_function(None, source_cfg=source, decompiled_cfg=decompiled)
    return float(mv.value), dict(mv.metadata or {})


def artifact_path(case: dict) -> Path:
    return (config.results_root() / case["opt_level"] / case["project"] /
            "decompiled" / f"kuna_{case['binary']}.c")


def fresh_block(case: dict, options: list[list[str]], kuna_bin: str) -> str:
    cmd = [kuna_bin, "decompile-all", case["stripped_path"], "--json",
           "--addr", case["address_hex"]]
    for name, value in options:
        cmd += ["--option", name, value]
    out = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
    if out.returncode != 0:
        raise RuntimeError(f"kuna exited {out.returncode}: {out.stderr[:300]}")
    fns = json.loads(out.stdout).get("functions", [])
    if not fns or fns[0].get("error") or not fns[0].get("code"):
        raise RuntimeError(f"no fresh code: {fns[0].get('error') if fns else 'no record'}")
    rec = fns[0]
    code = re.sub(rf"\b{re.escape(rec['name'])}\b", case["function"], rec["code"])
    return f"// Function: {case['function']} @ {case['address_hex']}\n" + code


def spliced_cfgs(case: dict, new_block: str, extract, stable_hash) -> dict:
    """Joern-parse the stored artifact with the case block replaced by new_block."""
    art = artifact_path(case)
    text = art.read_text(errors="replace")
    hits = list(MARKER.finditer(text))
    spliced = None
    for i, m in enumerate(hits):
        if m.group(1) == case["function"]:
            end = hits[i + 1].start() if i + 1 < len(hits) else len(text)
            spliced = text[:m.start()] + new_block.rstrip() + "\n\n" + text[end:]
            break
    if spliced is None:  # function absent from the run artifact — append
        spliced = text + "\n" + new_block
    tmp = Path(tempfile.mktemp(suffix=".c"))
    tmp.write_text(spliced)
    try:
        return _cached_cfgs(tmp, spliced, extract, stable_hash)
    finally:
        tmp.unlink(missing_ok=True)


def rescore_case(case: dict, options: list[list[str]], kuna_bin: str) -> dict:
    GEDMetric, extract, strip, stable_hash = _decbench_imports()
    scfg, src_file = source_cfg(case, extract, strip, stable_hash)
    if scfg is None:
        return {"case": case["case_id"], "error": "no source CFG found (Joern)"}

    result: dict = {
        "case": case["case_id"],
        "recorded": case.get("target_value"),
        "source_file": str(src_file),
        "source_nodes": scfg.number_of_nodes(),
    }

    # BEFORE: the stored run artifact.
    art = artifact_path(case)
    art_cfgs = _cached_cfgs(art, art.read_text(errors="replace"), extract, stable_hash)
    bcfg = art_cfgs.get(case["function"])
    if bcfg is not None:
        before, bmeta = ged_of(scfg, bcfg, GEDMetric)
        result["before"] = before
        result["before_approximated"] = bool(bmeta.get("approximated"))
        rec = case.get("target_value")
        if rec is not None and math.isfinite(rec) and abs(before - rec) > 0.5:
            result["metric_drift"] = True
            print(f"[rescore] METRIC-DRIFT: recorded {rec:g} vs recomputed {before:g}",
                  file=sys.stderr)
    else:
        result["before"] = None
        result["before_error"] = "function not in stored artifact / Joern parse failed"

    # AFTER: fresh kuna (current build), spliced into the artifact.
    try:
        block = fresh_block(case, options, kuna_bin)
    except Exception as e:  # noqa: BLE001
        result["after"] = None
        result["after_error"] = str(e)
        return result
    dcfgs = spliced_cfgs(case, block, extract, stable_hash)
    acfg = dcfgs.get(case["function"])
    if acfg is None:
        result["after"] = None
        result["after_error"] = "fresh block did not Joern-parse to a CFG"
        return result
    after, ameta = ged_of(scfg, acfg, GEDMetric)
    result["after"] = after
    result["after_approximated"] = bool(ameta.get("approximated"))
    if result.get("before") is not None:
        result["delta"] = after - result["before"]
    result["perfect_after"] = after == 0.0
    return result


def splice_record(record_path: Path, summary: dict) -> None:
    rec = json.loads(record_path.read_text()) if record_path.exists() else {}
    rec.update(summary)
    record_path.parent.mkdir(parents=True, exist_ok=True)
    record_path.write_text(json.dumps(rec, indent=1) + "\n")


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--case", required=True)
    ap.add_argument("--option", nargs=2, action="append", default=[],
                    metavar=("NAME", "VALUE"),
                    help="kuna --option applied to the AFTER run")
    ap.add_argument("--siblings", action="store_true",
                    help="also rescore every sibling case in the dedup group")
    ap.add_argument("--record", type=Path, default=None,
                    help="record.json to splice ged_* fields into")
    ap.add_argument("--kuna-bin", default=None)
    args = ap.parse_args(argv)

    kuna_bin = args.kuna_bin or config.kuna_bin()
    case = load_case(args.case)
    results = [rescore_case(case, args.option, kuna_bin)]
    if args.siblings:
        for sib in case.get("siblings", []):
            results.append(rescore_case(load_case(sib), args.option, kuna_bin))

    main_r = results[0]
    kuna_commit = subprocess.run(
        ["git", "-C", str(config.repo_root()), "rev-parse", "--short", "HEAD"],
        capture_output=True, text=True).stdout.strip()
    summary = {
        "ged_case": main_r["case"],
        "ged_recorded": main_r.get("recorded"),
        "ged_before": main_r.get("before"),
        "ged_after": main_r.get("after"),
        "ged_delta": main_r.get("delta"),
        "ged_perfect_after": main_r.get("perfect_after"),
        "ged_approximated": main_r.get("after_approximated"),
        "ged_options": [f"{n}={v}" for n, v in args.option],
        "ged_kuna_commit": kuna_commit,
        "ged_results_root": str(config.results_root()),
    }
    if args.siblings:
        summary["ged_siblings"] = {
            r["case"]: {"before": r.get("before"), "after": r.get("after")}
            for r in results[1:]
        }
    print(json.dumps({"summary": summary, "details": results}, indent=1))
    if args.record:
        splice_record(args.record, summary)
        print(f"[rescore] spliced ged_* into {args.record}", file=sys.stderr)


if __name__ == "__main__":
    main()
