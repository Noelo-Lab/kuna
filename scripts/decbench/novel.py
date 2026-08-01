"""Mine the NOVEL pool: functions kuna already wins yet still decompiles badly.

``mine.py`` finds cases where another decompiler shows kuna a better answer. This
module finds the opposite and harder pool: functions where **kuna's GED is the
best (or tied-best) of every production decompiler and is still large**. No rival
holds the answer, so a fix here is genuinely novel — the campaign tags those PRs
``(NOVEL)`` so the provenance is unambiguous.

    python3 -m scripts.decbench.novel                    # snapshot + rank
    python3 -m scripts.decbench.novel --select 12        # also emit the triage queue
    python3 -m scripts.decbench.novel --min-ged 12 --max-source-nodes 60

Outputs (under docs/decbench/):
    novel.json    the pool, one row per (opt, project, binary, function), shaped
                  like a mine.py case so ``scripts.decbench.triage`` can load it
    novel.md      human-readable ranked table, one row per dedup group
    triage-queue-novel.json   (--select) the stratified picks

Filters that keep the pool honest (all overridable):
- kuna GED >= ``--min-ged`` (default 8; the corpus median is 5)
- kuna GED <= min(GED of angr/ghidra/ida/binja/phoenix), at least
  ``--min-rivals`` (default 3) of which have a finite score
- source CFG present and between ``--min-source-nodes`` (default 4) and
  ``--max-source-nodes`` (default 60) nodes: below that the score is degenerate,
  above it decbench APPROXIMATES the GED as |Δnodes| + |Δedges| and the number
  stops being an edit distance. Needs the ``srcsizes`` cache.
"""
from __future__ import annotations

import argparse
import json
import math
import re
import sys
from collections import defaultdict
from datetime import date
from pathlib import Path

from . import config

PRODUCTION = ("angr", "ghidra", "ida", "binja", "phoenix")
MARKER = re.compile(r"^// Function: (\S+) @ (0x[0-9a-fA-F]+)\s*$", re.M)

# Output-shaped warts in kuna's OWN emission. GED cannot see most of these — a
# function can win the metric and still read badly — so they are the second,
# independent axis this pool ranks on. Weight = how much the wart costs a reader.
DEFECTS: dict[str, tuple] = {
    "goto":         (re.compile(r"\bgoto\s+\w+"), 3),
    "halt_baddata": (re.compile(r"\bhalt_baddata\b"), 20),
    "code_ptr":     (re.compile(r"\(code\s*\*\)"), 8),
    "undefined":    (re.compile(r"\bundefined[0-9]*\b"), 2),
    "unaff":        (re.compile(r"\bunaff_\w+"), 5),
    "extraout":     (re.compile(r"\bextraout_\w+"), 5),
    "in_reg":       (re.compile(r"\bin_(?:E|R)[A-Z]{2}\b|\bin_stack_\w+"), 4),
    "concat":       (re.compile(r"\bCONCAT\d+\b"), 3),
    "subpiece":     (re.compile(r"\bSUB\d+\b"), 3),
    "raw_reg":      (re.compile(r"\b(?:[ER]AX|[ER]BX|[ER]CX|[ER]DX|[ER]SI|[ER]DI|[ER]BP|[ER]SP)\b"), 6),
    "switchD":      (re.compile(r"\bswitchD\b|\bjoined_r0x\w+"), 4),
    "uninit_warn":  (re.compile(r"WARNING:.*[Uu]ninitialized|// *uninit"), 4),
}


def defect_profile(block: str) -> dict:
    """Per-wart counts + a weighted score for one decompiled function block."""
    counts = {name: len(rx.findall(block)) for name, (rx, _w) in DEFECTS.items()}
    score = sum(counts[n] * w for n, (_rx, w) in DEFECTS.items())
    return {"defects": {k: v for k, v in counts.items() if v}, "defect_score": score,
            "lines": block.count("\n") + 1}


def kuna_blocks(opt: str, project: str, binary: str) -> dict[str, str]:
    """function -> stored kuna block for one binary (the run's own artifact)."""
    path = config.results_root() / opt / project / "decompiled" / f"kuna_{binary}.c"
    if not path.is_file():
        return {}
    text = path.read_text(errors="replace")
    hits = list(MARKER.finditer(text))
    out = {}
    for i, m in enumerate(hits):
        end = hits[i + 1].start() if i + 1 < len(hits) else len(text)
        out[m.group(1)] = text[m.start():end]
    return out


def _addresses(opt: str, project: str, binary: str) -> dict[str, int]:
    """function -> address, from kuna's own decompiled artifact for that binary."""
    root = config.results_root() / opt / project / "decompiled"
    for dec in ("kuna", "ghidra", "ida", "angr", "binja"):
        path = root / f"{dec}_{binary}.c"
        if path.is_file():
            hits = {m.group(1): int(m.group(2), 16) for m in MARKER.finditer(
                path.read_text(errors="replace"))}
            if hits:
                return hits
    return {}


def _binary_path(opt: str, project: str, binary: str) -> str | None:
    comp = config.results_root() / opt / project / "compiled"
    exact = comp / binary
    if exact.is_file():
        return str(exact)
    if comp.is_dir():
        for f in sorted(comp.iterdir()):
            if f.is_file() and f.stem == binary and f.suffix not in (".i", ".c", ".h"):
                return str(f)
    return None


def build_pool(args) -> dict:
    fr = json.loads((config.results_root() / "function_results.json").read_text())
    src_sizes = config.load_src_sizes()
    if not src_sizes:
        sys.exit(f"no source-CFG size cache at {config.src_sizes_path()} — run "
                 f"`{config.decbench_python()} -m scripts.decbench.srcsizes` first")

    cases: list[dict] = []
    groups: dict[str, list[dict]] = defaultdict(list)
    addr_cache: dict[tuple, dict[str, int]] = {}
    want_opts = set(args.opt_levels.split(","))

    for g in fr["groups"]:
        opt, project, binary = g["opt_level"], g["project"], g["binary"]
        if opt not in want_opts:
            continue
        blocks: dict[str, str] | None = None
        for f in g["functions"]:
            kv = f["values"].get("kuna", {}).get("ged")
            if kv is None or not math.isfinite(kv) or kv < args.min_ged:
                continue
            rivals = {}
            for dec in PRODUCTION:
                v = f["values"].get(dec, {}).get("ged")
                if v is not None and math.isfinite(v):
                    rivals[dec] = v
            if len(rivals) < args.min_rivals:
                continue
            best_rival = min(rivals, key=lambda d: rivals[d])
            if rivals[best_rival] < kv:
                continue  # somebody beats kuna: that is mine.py's pool, not this one
            src = config.source_cfg(src_sizes, opt, project, binary, f["function"])
            n = src["source_nodes"]
            if n is None or not (args.min_source_nodes <= n <= args.max_source_nodes):
                continue
            if src["approximated"]:
                continue
            # A GED far larger than the whole source CFG is not "kuna structured
            # this badly" — it is the decompiled graph having no counterpart at
            # all (inlining, a wrong function boundary, a metric artifact).
            if kv > n * args.max_ged_ratio:
                continue

            key = (opt, project, binary)
            if key not in addr_cache:
                addr_cache[key] = _addresses(opt, project, binary)
            address = addr_cache[key].get(f["function"])
            bpath = _binary_path(opt, project, binary)
            if blocks is None:
                blocks = kuna_blocks(opt, project, binary)
            profile = defect_profile(blocks.get(f["function"], ""))
            c = {
                "case_id": config.case_id(opt, project, binary, f["function"]),
                "group_id": config.group_id(project, f["function"]),
                "project": project, "opt_level": opt, "binary": binary,
                "binary_path": bpath,
                "stripped_path": config.stripped_path(bpath) if bpath else None,
                "function": f["function"],
                "address": address,
                "address_hex": f"0x{address:x}" if address is not None else None,
                "base": best_rival,
                "base_value": rivals[best_rival],
                "target_value": kv,
                "margin": 0.0,
                "lead": rivals[best_rival] - kv,
                "others_ged": rivals,
                "bucket": "novel",
                "artifact_suspect": False,
                "degenerate_source": False,
                **src,
                **profile,
                "size": f.get("size"),
                "labels": list(f.get("labels", [])),
                "novel": True,
            }
            cases.append(c)
            groups[c["group_id"]].append(c)

    for c in cases:
        c["siblings"] = [s["case_id"] for s in groups[c["group_id"]] if s["case_id"] != c["case_id"]]

    return {
        "schema": 1,
        "pool": "novel",
        "snapshot_date": str(date.today()),
        "results_root": str(config.results_root()),
        "run_versions": fr.get("decompiler_versions", {}),
        "filters": {
            "min_ged": args.min_ged, "min_rivals": args.min_rivals,
            "min_source_nodes": args.min_source_nodes,
            "max_source_nodes": args.max_source_nodes,
            "max_ged_ratio": args.max_ged_ratio,
            "opt_levels": args.opt_levels, "rank": args.rank,
        },
        "totals": {"cases": len(cases), "groups": len(groups)},
        "cases": cases,
    }


def _rank(doc: dict) -> list[tuple[str, list[dict]]]:
    """Rank dedup groups worst-first on the pool's ranking axis."""
    groups: dict[str, list[dict]] = defaultdict(list)
    for c in doc["cases"]:
        groups[c["group_id"]].append(c)
    axis = doc.get("filters", {}).get("rank", "defects")
    key = "defect_score" if axis == "defects" else "target_value"
    return sorted(groups.items(),
                  key=lambda kv: (-max(c[key] for c in kv[1]), -len(kv[1]), kv[0]))


def write_table(doc: dict) -> str:
    ranked = _rank(doc)
    fl = doc["filters"]
    lines = [
        "# decbench NOVEL backlog — kuna is best of all decompilers and still bad",
        "",
        "Generated by `python3 -m scripts.decbench.novel` from "
        f"`{doc['results_root']}` ({doc['snapshot_date']}).",
        f"Filters: kuna GED >= {fl['min_ged']} and <= {fl['max_ged_ratio']}x the source "
        f"CFG, kuna <= every rival, {fl['min_source_nodes']} <= source nodes <= "
        f"{fl['max_source_nodes']}, opt in {fl['opt_levels']}. Ranked by {fl['rank']}.",
        f"{doc['totals']['cases']} cases in {doc['totals']['groups']} groups. "
        "Do not edit by hand.",
        "",
        "| # | group | defects | score | kuna GED | best rival | src nodes | cases | opts |",
        "|---|-------|---------|-------|----------|------------|-----------|-------|------|",
    ]
    axis = "defect_score" if fl["rank"] == "defects" else "target_value"
    for i, (gid, cs) in enumerate(ranked, 1):
        rep = max(cs, key=lambda c: c[axis])
        opts = ",".join(sorted({c["opt_level"] for c in cs}))
        marks = ",".join(f"{k}x{v}" for k, v in sorted(rep["defects"].items())) or "-"
        lines.append(
            f"| {i} | {gid} | {marks} | {rep['defect_score']} | {rep['target_value']:g} | "
            f"{rep['base']} {rep['base_value']:g} | {rep['source_nodes']} | "
            f"{len(cs)} | {opts} |"
        )
    return "\n".join(lines) + "\n"


def select_queue(doc: dict, want: int, per_project: int) -> list[dict]:
    """Deterministic pick: worst-scoring groups first, capped per project."""
    axis = "defect_score" if doc["filters"]["rank"] == "defects" else "target_value"
    seen: dict[str, int] = defaultdict(int)
    queue = []
    for gid, cs in _rank(doc):
        rep = max(cs, key=lambda c: (c[axis], c["case_id"]))
        if rep["address_hex"] is None or rep["stripped_path"] is None:
            continue
        if seen[rep["project"]] >= per_project:
            continue
        seen[rep["project"]] += 1
        queue.append({"tier": "N", "group_id": gid,
                      "group_ged": max(c["target_value"] for c in cs), **rep})
        if len(queue) >= want:
            break
    return queue


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--select", type=int, default=0, metavar="N",
                    help="also write triage-queue-novel.json with N picks")
    ap.add_argument("--min-ged", type=float, default=6.0)
    ap.add_argument("--min-rivals", type=int, default=3)
    ap.add_argument("--min-source-nodes", type=int, default=4)
    ap.add_argument("--max-source-nodes", type=int, default=60)
    ap.add_argument("--max-ged-ratio", type=float, default=2.5,
                    help="drop cases whose GED exceeds this multiple of the source CFG")
    ap.add_argument("--opt-levels", default="O0,O2-noinline",
                    help="comma-separated opt levels; O2 is excluded by default because "
                         "inlining detaches the decompiled CFG from the source function")
    ap.add_argument("--rank", choices=("defects", "ged"), default="defects",
                    help="ranking axis: warts in kuna's own output (default) or raw GED")
    ap.add_argument("--per-project", type=int, default=2,
                    help="cap on queue picks from one project (default 2)")
    args = ap.parse_args(argv)

    config.campaign_dir().mkdir(parents=True, exist_ok=True)
    doc = build_pool(args)
    config.novel_path().write_text(json.dumps(doc, indent=1) + "\n")
    (config.campaign_dir() / "novel.md").write_text(write_table(doc))
    t = doc["totals"]
    print(f"[novel] {t['cases']} cases in {t['groups']} groups -> {config.novel_path()}")

    if args.select:
        queue = select_queue(doc, args.select, args.per_project)
        path = config.campaign_dir() / "triage-queue-novel.json"
        path.write_text(json.dumps(
            {"schema": 1, "pool": "novel", "snapshot_date": doc["snapshot_date"],
             "queue": queue}, indent=1) + "\n")
        print(f"[novel] queue: {len(queue)} cases -> {path}")
        for q in queue:
            marks = ",".join(f"{k}x{v}" for k, v in sorted(q["defects"].items())) or "-"
            print(f"   [N] {q['case_id']}  score={q['defect_score']} [{marks}] "
                  f"GED={q['target_value']:g} vs {q['base']}={q['base_value']:g} "
                  f"(src {q['source_nodes']} nodes, {q['lines']} lines)")


if __name__ == "__main__":
    main()
