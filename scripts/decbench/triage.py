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


def load_case(case_id: str) -> dict:
    for path in (config.queue_path(), config.cases_path()):
        if not path.exists():
            continue
        doc = json.loads(path.read_text())
        for c in doc.get("queue", doc.get("cases", [])):
            if c["case_id"] == case_id:
                return c
    sys.exit(f"case {case_id!r} not found in {config.queue_path()} or {config.cases_path()}")


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
    cmd = [config.kuna_bin(), "decompile-all", case["stripped_path"], "--json",
           "--addr", case["address_hex"] or hex(case["address"])]
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
    header = f"// Function: {case['function']} @ {case['address_hex']}\n"
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
    ap.add_argument("--option", nargs=2, action="append", default=[],
                    metavar=("NAME", "VALUE"), help="kuna --option for the fresh run")
    ap.add_argument("--no-fresh", action="store_true", help="skip the fresh kuna run")
    args = ap.parse_args(argv)

    case = load_case(args.case)
    print(f"# triage dump: {case['case_id']}")
    print()
    print(f"- group: {case['group_id']}  siblings: {case.get('siblings', [])}")
    print(f"- recorded: angr GED={case['base_value']:g}*  kuna GED={case['target_value']:g}"
          f"  margin Δ{case['margin']:g}")
    print(f"- others: {case.get('others_ged', {})}  bucket: {case.get('bucket')}"
          f"{'  ARTIFACT-SUSPECT' if case.get('artifact_suspect') else ''}")
    print(f"- binary: {case['binary_path']}")
    print(f"- stripped: {case['stripped_path']}")
    print(f"- function: {case['function']} @ {case['address_hex']}  labels: {case.get('labels')}")
    print()

    angr_b = stored_block(case, "angr")
    kuna_b = stored_block(case, "kuna")
    rows = {"angr(stored)": metrics(angr_b), "kuna(stored)": metrics(kuna_b)}

    fresh_b, fresh_err = ("", "skipped") if args.no_fresh else fresh_kuna(case, args.option)
    if fresh_b:
        rows["kuna(fresh)"] = metrics(fresh_b)

    print("## structural metrics\n")
    print(fmt_metrics(rows))
    print()
    print("## angr (stored)\n\n```c\n" + angr_b + "```\n")
    print("## kuna (stored, run-time)\n\n```c\n" + kuna_b + "```\n")
    if fresh_b:
        print("## kuna (fresh, current build)\n\n```c\n" + fresh_b + "```\n")
    else:
        print(f"## kuna (fresh): UNAVAILABLE — {fresh_err}\n")


if __name__ == "__main__":
    main()
