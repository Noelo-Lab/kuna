"""Mine the decbench results tree into the campaign backlog.

Wraps ``decbench improvements`` (the offline per-function miner) and snapshots
its output into the kuna repo so the campaign has a stable, committed signal:

    python3 -m scripts.decbench.mine                # snapshot + rank
    python3 -m scripts.decbench.mine --select 28    # also emit the triage queue

Outputs (all under docs/decbench/):
    cases.json         the full angr-perfect / kuna-nonzero pool, one row per
                       (opt, project, binary, function), enriched with the other
                       decompilers' GED (ghidra/ida/binja/phoenix), a triage
                       bucket, and dedup group ids
    cases-missing.json functions kuna produced no usable GED for (decompile
                       failure or metric error) + the whole-binary failure list;
                       owned by the hang-fix workstream, not case triage
    backlog.md         human-readable ranked table, one row per dedup group
    triage-queue.json  (--select) the stratified triage picks

Buckets (cross-decompiler consensus, computed vs the same function's scores):
    kuna-specific   kuna GED > ghidra GED + 2   (regression vs kuna's ancestor)
    inherited       |kuna - ghidra| <= 2         (Ghidra-family-wide gap)
    ahead           kuna < ghidra - 2            (kuna already beats ghidra)
    no-ghidra       ghidra has no finite score
An ``artifact_suspect`` flag marks cases where kuna is no worse than the best of
ghidra/ida/binja while only angr scores 0 — historically a decbench scoring
artifact (angr truncation / degenerate source CFG), not a kuna deficit.
"""
from __future__ import annotations

import argparse
import json
import math
import subprocess
import sys
from collections import defaultdict
from datetime import date
from pathlib import Path

from . import config

OTHERS = ("ghidra", "ida", "binja", "phoenix")


def _run_improvements(extra: list[str]) -> list[dict]:
    cmd = [
        config.decbench_cli(),
        "improvements",
        str(config.results_root()),
        "-b", "angr", "-t", "kuna", "-m", "ged",
        "--limit", "0", "-f", "json",
    ] + extra
    out = subprocess.run(cmd, capture_output=True, text=True, cwd=config.decbench_repo())
    if out.returncode != 0:
        sys.exit(f"decbench improvements failed:\n{out.stderr}")
    return json.loads(out.stdout)


def _load_function_results() -> dict:
    return json.loads((config.results_root() / "function_results.json").read_text())


def _other_geds(fr: dict) -> dict:
    """(project, opt, binary, function) -> {dec: ged} for the non-angr/kuna decs."""
    table: dict[tuple, dict] = {}
    kuna_scored: dict[tuple, int] = defaultdict(int)
    for g in fr["groups"]:
        for f in g["functions"]:
            key = (g["project"], g["opt_level"], g["binary"], f["function"])
            row = {}
            for dec in OTHERS:
                v = f["values"].get(dec, {}).get("ged")
                if v is not None and math.isfinite(v):
                    row[dec] = v
            table[key] = row
            kv = f["values"].get("kuna", {}).get("ged")
            if kv is not None and math.isfinite(kv):
                kuna_scored[(g["project"], g["opt_level"], g["binary"])] += 1
    table["__kuna_scored__"] = kuna_scored  # type: ignore[assignment]
    return table


def _bucket(kuna: float, others: dict) -> tuple[str, bool]:
    gh = others.get("ghidra")
    if gh is None:
        bucket = "no-ghidra"
    elif kuna > gh + 2:
        bucket = "kuna-specific"
    elif abs(kuna - gh) <= 2:
        bucket = "inherited"
    else:
        bucket = "ahead"
    consensus = [others[d] for d in ("ghidra", "ida", "binja") if d in others]
    suspect = bool(consensus) and min(consensus) >= kuna - 1
    return bucket, suspect


def build_cases() -> tuple[dict, dict]:
    rows = _run_improvements(["--perfect-only"])
    fr = _load_function_results()
    others = _other_geds(fr)
    kuna_scored = others.pop("__kuna_scored__")

    groups: dict[str, list[dict]] = defaultdict(list)
    cases = []
    for r in rows:
        key = (r["project"], r["opt_level"], r["binary"], r["function"])
        odict = others.get(key, {})
        bucket, suspect = _bucket(r["target_value"], odict)
        c = dict(r)
        c["case_id"] = config.case_id(r["opt_level"], r["project"], r["binary"], r["function"])
        c["group_id"] = config.group_id(r["project"], r["function"])
        c["stripped_path"] = config.stripped_path(r["binary_path"]) if r.get("binary_path") else None
        c["others_ged"] = odict
        c["bucket"] = bucket
        c["artifact_suspect"] = suspect
        cases.append(c)
        groups[c["group_id"]].append(c)
    for c in cases:
        c["siblings"] = [s["case_id"] for s in groups[c["group_id"]] if s["case_id"] != c["case_id"]]

    # The kuna-missing pool: everything angr scored that kuna has no usable GED
    # for. Not part of case triage (the hang-fix workstream owns it).
    missing_rows = [r for r in _run_improvements(["--include-target-missing"]) if r["target_missing"]]
    whole_binary = sorted(
        f"{opt}/{proj}/{binary}"
        for g in fr["groups"]
        for (proj, opt, binary) in [(g["project"], g["opt_level"], g["binary"])]
        if kuna_scored.get((proj, opt, binary), 0) == 0
        and any(
            v.get("ged") is not None and math.isfinite(v["ged"])
            for f in g["functions"]
            for v in [f["values"].get("angr", {})]
        )
    )

    meta = {
        "schema": 1,
        "snapshot_date": str(date.today()),
        "results_root": str(config.results_root()),
        "run_versions": fr.get("decompiler_versions", {}),
        "totals": {"cases": len(cases), "groups": len(groups)},
    }
    cases_doc = {**meta, "cases": cases}
    missing_doc = {
        **meta,
        "totals": {"missing_functions": len(missing_rows), "whole_binary_failures": len(whole_binary)},
        "whole_binary_failures": whole_binary,
        "missing": missing_rows,
    }
    return cases_doc, missing_doc


def write_backlog(cases_doc: dict) -> str:
    groups: dict[str, list[dict]] = defaultdict(list)
    for c in cases_doc["cases"]:
        groups[c["group_id"]].append(c)
    ranked = sorted(
        groups.items(),
        key=lambda kv: (-max(c["margin"] for c in kv[1]), kv[0]),
    )
    lines = [
        "# decbench backlog — angr GED-perfect, kuna nonzero",
        "",
        f"Generated by `python3 -m scripts.decbench.mine` from `{cases_doc['results_root']}`",
        f"({cases_doc['snapshot_date']}). One row per dedup group (same project+function);",
        f"{cases_doc['totals']['cases']} cases in {cases_doc['totals']['groups']} groups. Do not edit by hand.",
        "",
        "| # | group | margin | bucket | artifact? | cases | opts |",
        "|---|-------|--------|--------|-----------|-------|------|",
    ]
    for i, (gid, cs) in enumerate(ranked, 1):
        rep = max(cs, key=lambda c: c["margin"])
        opts = ",".join(sorted({c["opt_level"] for c in cs}))
        lines.append(
            f"| {i} | {gid} | {rep['margin']:g} | {rep['bucket']} | "
            f"{'yes' if rep['artifact_suspect'] else ''} | {len(cs)} | {opts} |"
        )
    return "\n".join(lines) + "\n"


def _pick_representative(cs: list[dict]) -> dict:
    """Prefer the O0 instance (easiest to root-cause), else largest margin."""
    o0 = [c for c in cs if c["opt_level"] == "O0"]
    pool = o0 or cs
    return max(pool, key=lambda c: (c["margin"], c["case_id"]))


def select_queue(cases_doc: dict, want: int) -> list[dict]:
    """Stratified, deterministic pick of triage cases (see docs/decbench-loop.md)."""
    groups: dict[str, list[dict]] = defaultdict(list)
    for c in cases_doc["cases"]:
        groups[c["group_id"]].append(c)
    reps = {gid: _pick_representative(cs) for gid, cs in groups.items()}
    gmargin = {gid: max(c["margin"] for c in cs) for gid, cs in groups.items()}

    def order(gids, key):
        return sorted(gids, key=key)

    picked: list[tuple[str, str]] = []  # (tier, group_id)
    per_project: dict[str, int] = defaultdict(int)

    def take(gid: str, tier: str) -> bool:
        proj = reps[gid]["project"]
        if per_project[proj] >= 4 or any(g == gid for _, g in picked):
            return False
        picked.append((tier, gid))
        per_project[proj] += 1
        return True

    # Tier L: margin >= 100, kuna-specific first, cap 2/project within the tier.
    l_pool = order(
        (g for g, m in gmargin.items() if m >= 100),
        key=lambda g: (reps[g]["bucket"] != "kuna-specific", -gmargin[g], g),
    )
    tier_proj: dict[str, int] = defaultdict(int)
    for g in l_pool:
        if len([1 for t, _ in picked if t == "L"]) >= 8:
            break
        if tier_proj[reps[g]["project"]] >= 2:
            continue
        if take(g, "L"):
            tier_proj[reps[g]["project"]] += 1

    # Tier M: 20 <= margin < 100, spread projects (cap 2 within the tier).
    m_pool = order(
        (g for g, m in gmargin.items() if 20 <= m < 100),
        key=lambda g: (-gmargin[g], g),
    )
    tier_proj = defaultdict(int)
    for g in m_pool:
        if len([1 for t, _ in picked if t == "M"]) >= 10:
            break
        if tier_proj[reps[g]["project"]] >= 2:
            continue
        if take(g, "M"):
            tier_proj[reps[g]["project"]] += 1

    # Tier S: 5 <= margin < 20 (the fat middle), round-robin projects, prefer
    # multi-opt groups; reserve the last 3 slots for artifact-suspects.
    s_pool = [g for g, m in gmargin.items() if 5 <= m < 20]
    multi = {g for g in s_pool if len({c["opt_level"] for c in groups[g]}) > 1}
    by_proj: dict[str, list[str]] = defaultdict(list)
    for g in order(s_pool, key=lambda g: (g not in multi, -gmargin[g], g)):
        by_proj[reps[g]["project"]].append(g)
    s_target = 7
    rr = order(by_proj, key=lambda p: (-len(by_proj[p]), p))
    ptr = {p: 0 for p in rr}
    while len([1 for t, _ in picked if t == "S"]) < s_target and any(
        ptr[p] < len(by_proj[p]) for p in rr
    ):
        for p in rr:
            if len([1 for t, _ in picked if t == "S"]) >= s_target:
                break
            while ptr[p] < len(by_proj[p]):
                g = by_proj[p][ptr[p]]
                ptr[p] += 1
                if take(g, "S"):
                    break

    suspects = order(
        (g for g in gmargin if reps[g]["artifact_suspect"]),
        key=lambda g: (-gmargin[g], g),
    )
    for g in suspects:
        if len([1 for t, _ in picked if t == "A"]) >= 3:
            break
        take(g, "A")

    # Top up to the requested size from the ranked remainder.
    for g in order(gmargin, key=lambda g: (-gmargin[g], g)):
        if len(picked) >= want:
            break
        take(g, "X")

    queue = []
    for tier, gid in picked[:want]:
        rep = reps[gid]
        queue.append({
            "tier": tier,
            "group_id": gid,
            "group_margin": gmargin[gid],
            **{k: rep[k] for k in (
                "case_id", "project", "opt_level", "binary", "binary_path",
                "stripped_path", "function", "address", "address_hex", "margin",
                "base_value", "target_value", "others_ged", "bucket",
                "artifact_suspect", "labels", "siblings",
            )},
        })
    return queue


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--select", type=int, default=0, metavar="N",
                    help="also write triage-queue.json with N stratified picks")
    args = ap.parse_args(argv)

    config.campaign_dir().mkdir(parents=True, exist_ok=True)
    cases_doc, missing_doc = build_cases()
    config.cases_path().write_text(json.dumps(cases_doc, indent=1) + "\n")
    config.missing_path().write_text(json.dumps(missing_doc, indent=1) + "\n")
    (config.campaign_dir() / "backlog.md").write_text(write_backlog(cases_doc))
    t = cases_doc["totals"]
    print(f"[mine] {t['cases']} cases in {t['groups']} groups -> {config.cases_path()}")
    print(f"[mine] {missing_doc['totals']['missing_functions']} kuna-missing functions, "
          f"{missing_doc['totals']['whole_binary_failures']} whole-binary failures "
          f"-> {config.missing_path()}")

    if args.select:
        queue = select_queue(cases_doc, args.select)
        config.queue_path().write_text(json.dumps(
            {"schema": 1, "snapshot_date": cases_doc["snapshot_date"], "queue": queue},
            indent=1) + "\n")
        by_tier = defaultdict(int)
        for q in queue:
            by_tier[q["tier"]] += 1
        print(f"[mine] queue: {len(queue)} cases {dict(sorted(by_tier.items()))} "
              f"-> {config.queue_path()}")
        for q in queue:
            print(f"   [{q['tier']}] {q['case_id']}  Δ{q['margin']:g} {q['bucket']}"
                  f"{' ARTIFACT?' if q['artifact_suspect'] else ''}")


if __name__ == "__main__":
    main()
