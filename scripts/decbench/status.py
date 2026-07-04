"""Campaign tracker: pool -> triaged -> features -> PRs -> GED deltas.

    python3 -m scripts.decbench.status [--json]

Self-maintaining like scripts/port_status.py: campaign features are detected
from stages.toml provenance (``inspiration = "decbench:<case-id>; ..."``),
triage state from docs/decbench/triage/*.md front-matter, and benchmark deltas
from docs/features/*/record.json ``ged_*`` blocks. Nothing is hand-listed.
"""
from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path

from . import config

STAGES_TOML = config.repo_root() / "decompiler" / "crates" / "kuna-decomp" / "stages.toml"
FEATURES_DIR = config.repo_root() / "docs" / "features"


def campaign_settables() -> list[dict]:
    """[[settable]] blocks whose inspiration carries a decbench: case ref."""
    if not STAGES_TOML.exists():
        return []
    out = []
    for block in STAGES_TOML.read_text().split("\n[[settable]]\n"):
        insp = re.search(r'^inspiration\s*=\s*"(.*)"', block, re.M)
        if not insp or "decbench:" not in insp.group(1):
            continue
        def g(key: str) -> str:
            m = re.search(rf'^{key}\s*=\s*"(.*)"', block, re.M)
            return m.group(1) if m else ""
        out.append({
            "option": g("option"),
            "default": g("default"),
            "change_kind": g("change_kind"),
            "inspiration": insp.group(1),
        })
    return out


def triage_records() -> list[dict]:
    recs = []
    tdir = config.triage_dir()
    if not tdir.is_dir():
        return recs
    for f in sorted(tdir.glob("*.md")):
        text = f.read_text(errors="replace")
        m = re.match(r"---\n(.*?)\n---", text, re.S)
        if not m:
            continue
        fm: dict = {"file": f.name}
        for line in m.group(1).splitlines():
            kv = re.match(r"^(\w+):\s*(.+?)\s*$", line)
            if kv:
                fm[kv.group(1)] = kv.group(2)
        recs.append(fm)
    return recs


def feature_records() -> list[dict]:
    out = []
    if not FEATURES_DIR.is_dir():
        return out
    for rj in sorted(FEATURES_DIR.glob("*/record.json")):
        try:
            rec = json.loads(rj.read_text())
        except Exception:  # noqa: BLE001
            continue
        if any(k.startswith("ged_") for k in rec):
            out.append({"slug": rj.parent.name, **{k: rec[k] for k in rec if k.startswith("ged_")}})
    return out


def main(argv=None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)

    pool = {}
    if config.cases_path().exists():
        doc = json.loads(config.cases_path().read_text())
        pool = {"cases": doc["totals"]["cases"], "groups": doc["totals"]["groups"],
                "snapshot": doc["snapshot_date"]}
    triage = triage_records()
    statuses = Counter(r.get("status", "?") for r in triage)
    settables = campaign_settables()
    features = feature_records()

    if args.json:
        print(json.dumps({
            "pool": pool, "triaged": len(triage), "statuses": dict(statuses),
            "campaign_settables": settables, "feature_records": features,
        }, indent=1))
        return

    print("== decbench campaign status ==")
    if pool:
        print(f"pool: {pool['cases']} cases / {pool['groups']} groups (snapshot {pool['snapshot']})")
    print(f"triaged: {len(triage)}")
    for s, n in statuses.most_common():
        print(f"  {s:<20} {n}")
    print(f"campaign settables (stages.toml, decbench: provenance): {len(settables)}")
    for s in settables:
        print(f"  {s['option']:<24} default={s['default']:<4} {s['change_kind']}")
    print(f"feature GED records: {len(features)}")
    for f in features:
        print(f"  {f['slug']:<28} before={f.get('ged_before')} after={f.get('ged_after')} "
              f"delta={f.get('ged_delta')}")


if __name__ == "__main__":
    main()
