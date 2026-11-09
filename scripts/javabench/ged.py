"""Structural GED, exactly decbench's ``metrics/ged.py`` decision procedure.

1. directed isomorphism including entry/exit roles (decbench's
   ``_is_isomorphic`` with joint WL refinement colors) -> 0;
2. else, if either graph has more than ``GED_MAX_NODES`` (200) nodes, the
   size lower bound ``max(1, |dV| + |dE|)``;
3. else VJ-GED (cfgutils cost model, SciPy assignment solver), floored at 1.

decbench's two functions are imported from the ``decbench`` package (a checkout
named by ``$DECBENCH_SRC`` is put on the path first); without it, the vendored
copies below are used (they need ``cfgutils`` for the cost constant).
"""

from __future__ import annotations

import os
import sys

GED_MAX_NODES = int(os.environ.get("DECBENCH_GED_MAX_NODES") or "200")
_SRC = os.environ.get("DECBENCH_SRC")

try:
    if _SRC and _SRC not in sys.path:
        sys.path.append(_SRC)
    from decbench.metrics.ged import _is_isomorphic  # type: ignore
    from decbench.metrics.vj_ged import vj_ged  # type: ignore

    IMPLEMENTATION = f"decbench ({_SRC or 'installed'})"
except Exception:  # pragma: no cover - fallback
    IMPLEMENTATION = "vendored"
    import networkx as nx

    def _node_role(node):
        return (bool(getattr(node, "is_entrypoint", False)), bool(getattr(node, "is_exitpoint", False)))

    def _role_graph(cfg):
        g = nx.DiGraph()
        g.add_nodes_from((n, {"role": _node_role(n)}) for n in cfg.nodes())
        g.add_edges_from(cfg.edges())
        return g

    def _is_isomorphic(a, b):
        ga, gb = _role_graph(a), _role_graph(b)
        nm = nx.algorithms.isomorphism.categorical_node_match("role", (False, False))
        return bool(nx.is_isomorphic(ga, gb, node_match=nm))

    def vj_ged(source_cfg, decompiled_cfg):
        import numpy as np
        from cfgutils.similarity.ged import INVALID_CHOICE_PENALTY
        from scipy.optimize import linear_sum_assignment

        sn, dn = list(source_cfg.nodes), list(decompiled_cfg.nodes)
        sr, dr = [_node_role(n) for n in sn], [_node_role(n) for n in dn]
        s, d = len(sn), len(dn)
        c = np.zeros((s + d, s + d))
        c[s:, :d] = np.inf
        c[:s, d:] = np.inf
        for i, n in enumerate(dn):
            c[s + i, i] = 1 + decompiled_cfg.in_degree(n) + decompiled_cfg.out_degree(n)
        for i, n in enumerate(sn):
            c[i, d + i] = 1 + source_cfg.in_degree(n) + source_cfg.out_degree(n)
        for i, a in enumerate(sn):
            for j, b in enumerate(dn):
                cost = abs(source_cfg.out_degree(a) - decompiled_cfg.out_degree(b))
                cost += abs(source_cfg.in_degree(a) - decompiled_cfg.in_degree(b))
                if sr[i] != dr[j]:
                    cost += INVALID_CHOICE_PENALTY
                c[i, j] = cost
        r, k = linear_sum_assignment(c)
        return float(c[r, k].sum())


def ged(src_json: dict, dec_json: dict) -> dict:
    from .cfg import CFG

    s = CFG.from_json(src_json).to_networkx()
    d = CFG.from_json(dec_json).to_networkx()
    sn, dn = s.number_of_nodes(), d.number_of_nodes()
    se, de = s.number_of_edges(), d.number_of_edges()
    meta = {"src_blocks": sn, "src_edges": se, "dec_blocks": dn, "dec_edges": de}
    if _is_isomorphic(s, d):
        return {"value": 0.0, "method": "isomorphism", **meta}
    if sn > GED_MAX_NODES or dn > GED_MAX_NODES:
        return {"value": max(1.0, float(abs(sn - dn) + abs(se - de))), "method": "size_lower_bound", **meta}
    raw = float(vj_ged(s, d))
    return {"value": max(1.0, raw), "method": "vj_ged", "raw": raw, **meta}


def missing_penalty(src_json: dict) -> float:
    """GED of the source CFG against an empty graph (insert every block + edge):
    the value charged to a method the decompiler did not produce, for means."""
    return float(len(src_json["roles"]) + len(src_json["edges"]))


def ged_task(pair):
    key, s, d = pair
    try:
        return key, ged(s, d)
    except Exception as e:  # pragma: no cover
        return key, {"value": None, "method": "error", "error": str(e)}
