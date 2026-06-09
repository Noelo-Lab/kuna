"""Sweep the worklist: compare the reference vs kuna across many testcases, then rank
where the reference is better into a backlog the worker (and a human) can act on.

Writes:
  - docs/pipeline/opportunities.json  -- the ranked backlog (machine-readable)
  - docs/pipeline/matrix.md           -- the human-readable angr-vs-kuna recovery matrix

The sweep is deliberately corpus-agnostic and resumable: filter by arch / confidence /
limit, run N comparisons concurrently (each is two independent subprocesses), and tolerate
per-case timeouts/errors (they are recorded, not fatal). The full comparison texts are NOT
stored here (they would bloat the backlog); the worker re-runs `compare` for the case it
picks, which is cheap and deterministic.

    python -m kuna.pipeline.sweep                          # full sweep -> opportunities.json + matrix.md
    python -m kuna.pipeline.sweep --arch x86_64 --jobs 6   # x86_64/i386 subset, 6 concurrent
    python -m kuna.pipeline.sweep --limit 20 --print       # quick look, no write
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed

from . import config
from .worklist import build_worklist
from .compare import compare
from .rank import rank

# x86 family decompiles fastest and has the best kuna SLEIGH-spec overlap; good first pass.
_X86 = {"x86_64", "i386"}


def _entry_arch_family(entry):
    return entry.binary_rel[0] if entry.binary_rel else ""


def _run_one(entry):
    try:
        res = compare(
            entry.binary_path, entry.func_selector,
            arch=entry.arch, load_debug=entry.load_debug,
            test_name=entry.test_name,
        )
        d = res.to_dict()
        # capture comparability + the kuna-failed signal BEFORE dropping the heavy texts
        d["comparable"] = bool(d.get("ref_text")) and bool(d.get("kuna_text"))
        d["kuna_failed"] = bool(d.get("ref_text")) and not d.get("kuna_text")
        d.pop("ref_text", None)
        d.pop("kuna_text", None)
        d["arch"] = entry.arch or _entry_arch_family(entry)
        d["confidence"] = entry.confidence
        d["custom_options"] = entry.custom_options
        return d
    except Exception as e:  # noqa: BLE001
        return {
            "test_name": entry.test_name, "selector": entry.func_selector,
            "binary": entry.binary_path, "reference": "angr",
            "ref_text": None, "kuna_text": None,
            "ref_error": "sweep exception: %s: %s" % (type(e).__name__, e),
            "kuna_error": None, "signals": [], "ref_metrics": {}, "kuna_metrics": {},
        }


def sweep(*, arch=None, high_confidence=False, skip_custom=False, limit=None,
          jobs=6, progress=False):
    entries = [e for e in build_worklist() if e.binary_exists]
    if arch:
        if arch in ("x86", "x86_64", "i386"):
            entries = [e for e in entries if _entry_arch_family(e) in _X86]
        else:
            entries = [e for e in entries if _entry_arch_family(e) == arch]
    if high_confidence:
        entries = [e for e in entries if e.confidence == "high"]
    if skip_custom:
        entries = [e for e in entries if not e.custom_options]
    # de-dup identical (binary, selector) targets that several tests share
    seen = set()
    uniq = []
    for e in entries:
        key = (e.binary_path, e.func_selector)
        if key not in seen:
            seen.add(key)
            uniq.append(e)
    entries = uniq
    if limit:
        entries = entries[:limit]

    results = []
    done = 0
    with ThreadPoolExecutor(max_workers=jobs) as ex:
        futs = {ex.submit(_run_one, e): e for e in entries}
        for fut in as_completed(futs):
            results.append(fut.result())
            done += 1
            if progress:
                print("  [%d/%d] %s" % (done, len(entries), futs[fut].test_name),
                      file=sys.stderr)
    return rank(results)


def to_matrix_md(ranked, ref_version):
    lines = []
    lines.append("# angr-vs-kuna recovery matrix")
    lines.append("")
    lines.append("**Generated** by `python -m kuna.pipeline.sweep` -- do not edit by hand.")
    lines.append("")
    lines.append("Each row is one angr testcase function. `Score` ranks where angr's "
                 "decompilation looks structurally better than kuna's (higher = bigger "
                 "gap = better feature candidate). `Hint` notes whether an existing kuna "
                 "option may already address it. The worker turns a top row into one "
                 "option-gated kuna feature + PR.")
    lines.append("")
    lines.append("Reference: angr %s. Comparison is structural (gotos / labels / switch / "
                 "loops / loc), never raw-text." % (ref_version or "?"))
    lines.append("")
    n_opp = sum(1 for r in ranked if r["score"] > 0)
    lines.append("**%d testcases swept; %d show an angr advantage.**" % (len(ranked), n_opp))
    lines.append("")
    lines.append("| # | Score | Testcase :: function | Kinds | Why angr is better | Coverage hint |")
    lines.append("|---|---|---|---|---|---|")
    for r in ranked:
        if r["score"] <= 0:
            continue
        why = "; ".join(r["reasons"])[:160]
        kinds = ", ".join(r["kinds"])
        lines.append("| %d | %d | `%s :: %s` | %s | %s | %s |" % (
            r["rank"], r["score"], r["test_name"], r["func_name"] or r["selector"],
            kinds, why, r["covered_hint"]))
    lines.append("")
    # also list the cases where kuna failed outright (separate, high-value)
    failed = [r for r in ranked if not r["comparable"] and r.get("ref_error") is None]
    if failed:
        lines.append("## kuna produced no output (reference succeeded)")
        lines.append("")
        for r in failed:
            lines.append("- `%s :: %s` -- %s" % (
                r["test_name"], r["func_name"] or r["selector"], r.get("kuna_error") or "?"))
        lines.append("")
    return "\n".join(lines)


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.sweep",
                                description="Sweep angr-vs-kuna and rank the opportunities.")
    p.add_argument("--arch", default=None, help="filter: x86 (x86_64+i386) or a family name (mips/armel/...)")
    p.add_argument("--high-confidence", action="store_true", help="only single-target high-confidence entries")
    p.add_argument("--skip-custom", action="store_true", help="skip tests that pass non-default angr options")
    p.add_argument("--limit", type=int, default=None)
    p.add_argument("--jobs", type=int, default=6)
    p.add_argument("--out", default=None, help="opportunities JSON (default: docs/pipeline/opportunities.json)")
    p.add_argument("--matrix", default=None, help="matrix markdown (default: docs/pipeline/matrix.md)")
    p.add_argument("--print", action="store_true", dest="do_print", help="print top opportunities, don't write")
    p.add_argument("--progress", action="store_true", help="per-case progress to stderr")
    args = p.parse_args(argv)

    ranked = sweep(arch=args.arch, high_confidence=args.high_confidence,
                   skip_custom=args.skip_custom, limit=args.limit,
                   jobs=args.jobs, progress=args.progress)

    # the reference version (pull from the first comparable result's metrics call)
    from .reference import get_reference
    ref_version = get_reference("angr").version()

    if args.do_print:
        top = [r for r in ranked if r["score"] > 0][:25]
        print("top %d opportunities (of %d swept):" % (len(top), len(ranked)))
        for r in top:
            print("  [%2d] score=%d  %s :: %s  | %s"
                  % (r["rank"], r["score"], r["test_name"],
                     r["func_name"] or r["selector"], "; ".join(r["reasons"])[:90]))
        return 0

    out = args.out or str(config.pipeline_docs_dir() / "opportunities.json")
    matrix = args.matrix or str(config.pipeline_docs_dir() / "matrix.md")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    payload = {
        "reference": "angr",
        "ref_version": ref_version,
        "note": "Ranked angr-vs-kuna opportunities. Structural comparison; reference "
                "version pinned for reproducibility. Worker re-runs compare for full text.",
        "swept": len(ranked),
        "opportunities": sum(1 for r in ranked if r["score"] > 0),
        "ranked": ranked,
    }
    with open(out, "w") as fh:
        json.dump(payload, fh, indent=2)
    with open(matrix, "w") as fh:
        fh.write(to_matrix_md(ranked, ref_version))
    print("wrote %s (%d swept, %d opportunities) and %s"
          % (out, payload["swept"], payload["opportunities"], matrix))
    return 0


if __name__ == "__main__":
    sys.exit(main())
