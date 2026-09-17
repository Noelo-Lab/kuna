"""Per-case triage dump: everything a triage agent needs, in one command.

    python3 -m scripts.decbench.triage --case O2-coreutils-chown-chown_files
    python3 -m scripts.decbench.triage --case <id> --option loweredswitch off

Prints, for the one case: the recorded scores and paths, angr's stored
decompilation block, kuna's stored (run-time) block, a FRESH kuna decompilation
of the same function off the stripped binary (self-relabeled exactly like the
benchmark run), and structural metrics for all three side by side.

The fresh run is the verify-first step: the benchmark ran kuna 0.1.0 (Jun 29);
if the fresh output already matches angr structurally, the case is likely
`already-fixed` and needs a rescore, not a feature.
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

from . import config
from ..pipeline.compare import metrics

MARKER = re.compile(r"^// Function: (\S+) @ 0x([0-9a-fA-F]+)\s*$", re.M)


def load_case(case_id: str, metric: str | None = None) -> dict:
    """Resolve a case id against every mined pool under docs/decbench/.

    A pool's ``metric`` lives on the document, not the row, so it is stamped
    onto the row here — every number printed below is labelled with it.

    A case id carries no metric, so the same id can sit in a GED pool and in a
    ``type_match`` one. ``metric`` picks which; with none, the GED row wins
    (that is what every id minted before the pools split resolved to, so mining
    a second pool cannot re-point one), and an id found under more than one
    metric says so on stderr.
    """
    paths = config.pool_paths()
    hits = []
    for path in paths:
        doc = json.loads(path.read_text())
        for c in doc.get("queue", doc.get("cases", [])):
            if c["case_id"] == case_id:
                c.setdefault("metric", doc.get("metric") or "ged")
                hits.append((path, c))
                break
    if not hits:
        sys.exit(f"case {case_id!r} not found in any pool under {config.campaign_dir()} "
                 f"({len(paths)} searched)")
    chosen = next((c for _, c in hits if c["metric"] == (metric or "ged")), None)
    if chosen is None:
        if metric:
            sys.exit(f"case {case_id!r} is in no {metric} pool (it is in "
                     + ", ".join(f"{p.name} ({c['metric']})" for p, c in hits) + ")")
        chosen = hits[0][1]
    if len({c["metric"] for _, c in hits}) > 1:
        print(f"[triage] {case_id} is in {len(hits)} pools ("
              + ", ".join(f"{p.name}: {c['metric']}" for p, c in hits)
              + f") -- reading the {chosen['metric']} one; --metric picks another",
              file=sys.stderr)
    return chosen


def case_addr(case: dict) -> str | None:
    """The address a mined case is decompiled at, in kuna's ``--addr`` (VA) form."""
    return config.kuna_addr(case.get("stripped_path"),
                            case.get("address_hex") or case.get("address"))


def split_blocks(c_path: Path) -> dict[str, str]:
    """function name -> its block (marker line included) from a stored artifact."""
    if not c_path.exists():
        return {}
    text = c_path.read_text(errors="replace")
    hits = list(MARKER.finditer(text))
    out = {}
    for i, m in enumerate(hits):
        end = hits[i + 1].start() if i + 1 < len(hits) else len(text)
        out[m.group(1)] = text[m.start():end].rstrip() + "\n"
    return out


def stored_block(case: dict, dec: str) -> str:
    root = config.results_root()
    c_path = root / case["opt_level"] / case["project"] / "decompiled" / f"{dec}_{case['binary']}.c"
    return split_blocks(c_path).get(case["function"], f"(no {dec} block for {case['function']})\n")


def fresh_kuna(case: dict, options: list[list[str]]) -> tuple[str, str | None]:
    """Fresh decompile of the case function; returns (relabeled block, error)."""
    addr = case_addr(case)
    cmd = [config.kuna_bin(), "decompile-all", case["stripped_path"], "--json",
           "--addr", addr]
    for name, value in options:
        cmd += ["--option", name, value]
    out = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    if out.returncode != 0:
        return "", f"kuna exited {out.returncode}: {out.stderr[:500]}"
    doc = json.loads(out.stdout)
    fns = doc.get("functions", [])
    if not fns:
        return "", "kuna returned no function for that address"
    rec = fns[0]
    if rec.get("error"):
        return "", f"per-function error: {rec['error']}"
    code = rec.get("code") or ""
    # Self-relabel exactly like the benchmark run (own name only; callees stay sub_*).
    code = re.sub(rf"\b{re.escape(rec['name'])}\b", case["function"], code)
    header = f"// Function: {case['function']} @ {addr}\n"
    return header + code, None


def fmt_metrics(rows: dict[str, dict]) -> str:
    keys = ["loc", "gotos", "labels", "switches", "cases", "ifs", "loops", "ternaries", "casts"]
    lines = ["| output | " + " | ".join(keys) + " |",
             "|--------|" + "|".join(["---"] * len(keys)) + "|"]
    for name, m in rows.items():
        lines.append(f"| {name} | " + " | ".join(str(m[k]) for k in keys) + " |")
    return "\n".join(lines)


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--case", required=True)
    ap.add_argument("--metric", choices=("ged", "type_match"),
                    help="which pool to read the case from (default: ged)")
    ap.add_argument("--option", nargs=2, action="append", default=[],
                    metavar=("NAME", "VALUE"), help="kuna --option for the fresh run")
    ap.add_argument("--no-fresh", action="store_true", help="skip the fresh kuna run")
    ap.add_argument("--also", default="", metavar="D1,D2",
                    help="extra stored decompilers to print (e.g. ida,ghidra)")
    args = ap.parse_args(argv)

    case = load_case(args.case, args.metric)
    base = case.get("base") or "angr"
    metric = case.get("metric") or "ged"
    label = "GED" if metric == "ged" else metric
    print(f"# triage dump: {case['case_id']}")
    print()
    print(f"- group: {case['group_id']}  siblings: {case.get('siblings', [])}")
    if case.get("novel"):
        print(f"- NOVEL pool: kuna {label}={case['target_value']:g} is best of all; "
              f"next best {base}={case['base_value']:g} (lead {case.get('lead', 0):g})")
    else:
        print(f"- recorded: {base} {label}={case['base_value']:g}*  "
              f"kuna {label}={case['target_value']:g}  margin Δ{case['margin']:g}")
    others = case.get(f"others_{metric}") or case.get("others_ged") or {}
    print(f"- others: {others}  bucket: {case.get('bucket')}"
          f"{'  ARTIFACT-SUSPECT' if case.get('artifact_suspect') else ''}")
    src_n = case.get("source_nodes")
    if src_n is not None:
        # A degenerate source CFG invalidates a STRUCTURAL score only; it says
        # nothing about whether the recovered types are right.
        note = ""
        if metric == "ged":
            note = (" DEGENERATE — score is meaningless" if src_n <= 1
                    else "  (>60: GED is APPROXIMATED as |dnodes|+|dedges|)"
                    if src_n > 60 else "")
        print(f"- source CFG: {src_n} nodes / {case.get('source_edges')} edges{note}")
    print(f"- binary: {case['binary_path']}")
    print(f"- stripped: {case['stripped_path']}")
    va = case_addr(case)
    shown = case.get("address_hex") or va
    rebased = "" if va == shown else f" (PE RVA; kuna --addr {va})"
    print(f"- function: {case['function']} @ {shown}{rebased}  "
          f"labels: {case.get('labels')}")
    print()

    extra = [d for d in args.also.split(",") if d.strip()]
    stored = {d: stored_block(case, d) for d in dict.fromkeys([base] + extra)}
    kuna_b = stored_block(case, "kuna")
    rows = {f"{d}(stored)": metrics(b) for d, b in stored.items()}
    rows["kuna(stored)"] = metrics(kuna_b)

    fresh_b, fresh_err = ("", "skipped") if args.no_fresh else fresh_kuna(case, args.option)
    if fresh_b:
        rows["kuna(fresh)"] = metrics(fresh_b)

    print("## structural metrics\n")
    print(fmt_metrics(rows))
    print()
    for d, b in stored.items():
        print(f"## {d} (stored)\n\n```c\n" + b + "```\n")
    print("## kuna (stored, run-time)\n\n```c\n" + kuna_b + "```\n")
    if fresh_b:
        print("## kuna (fresh, current build)\n\n```c\n" + fresh_b + "```\n")
    else:
        print(f"## kuna (fresh): UNAVAILABLE — {fresh_err}\n")


if __name__ == "__main__":
    main()
