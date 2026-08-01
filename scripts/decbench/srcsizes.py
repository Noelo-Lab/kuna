"""Dump per-function SOURCE CFG sizes from the results tree's ``ged_src`` pickles.

The GED metric compares a decompiled function's CFG against the Joern-parsed CFG
of the ORIGINAL source. When that source CFG is degenerate (a single node — Joern
failed to parse the body, or the function is a one-liner), the score carries no
structural information: every decompiler that emits more than one basic block
scores badly and the "gap" is a decbench artifact, not a kuna deficit
(``docs/decbench-loop.md`` → *Caveats*).

The run stores those graphs at ``<results>/ged_src/<project>.pkl`` as
``{translation_unit: {function: networkx.DiGraph}}`` — one entry per function
Joern saw in that preprocessed ``.i``, which includes every prototype pulled in
from a header (those come out as 1-node stubs). So the size of a *function* is
the largest CFG carrying its name in the project: the TU that defines it.
Unpickling needs networkx, so this module runs under the decbench venv and
writes a plain-JSON cache the rest of the campaign tooling (plain python3) reads:

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.srcsizes

    -> ~/.cache/decbench/kuna-src-sizes.json
       {"by_function": {"<project>::<fn>": [nodes, edges, bodies]},
        "by_case":     {"<opt>::<project>::<binary>::<fn>": [nodes, edges, over60]}}

``by_function`` is an UPPER BOUND (the largest CFG carrying that name in the
project); ``bodies`` > 1 means the name is ambiguous inside the project (a static
function defined in several TUs), so the number is only a bound. It stays sound
for the question that matters — ``nodes <= 1`` means *every* candidate body is a
single block.

``by_case`` is exact but partial: the run's ``ged_large_graph_audit.json`` records
node/edge counts for the pairs where either graph exceeded 60 nodes — precisely
the regime where decbench stops computing a real edit distance and APPROXIMATES
the GED as |dnodes| + |dedges|. A case listed there with ``over60`` carries a
score that must not be read as an edit distance.

Override the location with ``KUNA_DECBENCH_SRC_SIZES``. Regenerate after a new
benchmark run; it is a cache, never committed.
"""
from __future__ import annotations

import json
import pickle
import sys

from . import config


def fold(project: str, per_tu: dict, sizes: dict[str, list[int]]) -> None:
    """Fold one project's {TU: {fn: DiGraph}} into project::fn -> [n, e, bodies]."""
    for fns in (per_tu or {}).values():
        for fn, graph in (fns or {}).items():
            try:
                n, e = int(graph.number_of_nodes()), int(graph.number_of_edges())
            except Exception:  # noqa: BLE001
                continue
            key = f"{project}::{fn}"
            cur = sizes.get(key)
            bodies = (cur[2] if cur else 0) + (1 if n > 1 else 0)
            if cur is None or n > cur[0]:
                sizes[key] = [n, e, bodies]
            else:
                cur[2] = bodies


def build() -> dict[str, list[int]]:
    src_dir = config.results_root() / "ged_src"
    if not src_dir.is_dir():
        sys.exit(f"no ged_src directory under {config.results_root()}")
    sizes: dict[str, list[int]] = {}
    for pkl in sorted(src_dir.glob("*.pkl")):
        try:
            per_tu = pickle.loads(pkl.read_bytes())
        except Exception as e:  # noqa: BLE001
            print(f"[srcsizes] skip {pkl.name}: {e}", file=sys.stderr)
            continue
        fold(pkl.stem, per_tu, sizes)
    return sizes


def build_by_case() -> dict[str, list[int]]:
    """Exact source CFG sizes for the >60-node (approximated-GED) pairs."""
    audit = config.results_root() / "ged_large_graph_audit.json"
    if not audit.is_file():
        return {}
    records = json.loads(audit.read_text()).get("records", {})
    out: dict[str, list[int]] = {}
    for key, rec in records.items():
        parts = key.split("::")
        if len(parts) != 5:
            continue
        opt, project, binary, _dec, fn = parts
        nodes = rec.get("corrected_source_nodes", rec.get("historical_source_nodes"))
        edges = rec.get("corrected_source_edges", rec.get("historical_source_edges"))
        if nodes is None:
            continue
        over = bool(rec.get("corrected_over_60", rec.get("historical_over_60")))
        out[f"{opt}::{project}::{binary}::{fn}"] = [int(nodes), int(edges or 0), int(over)]
    return out


def main(argv=None) -> None:
    by_function = build()
    by_case = build_by_case()
    out = config.src_sizes_path()
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps({"by_function": by_function, "by_case": by_case}))
    degenerate = sum(1 for v in by_function.values() if v[0] <= 1)
    ambiguous = sum(1 for v in by_function.values() if v[2] > 1)
    print(f"[srcsizes] {len(by_function)} source functions ({degenerate} degenerate, "
          f"{ambiguous} name-ambiguous); {len(by_case)} exact large-graph cases -> {out}")


if __name__ == "__main__":
    main()
