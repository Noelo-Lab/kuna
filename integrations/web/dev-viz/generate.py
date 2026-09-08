#!/usr/bin/env python3
"""Export a reproducible, public development snapshot from tracked Kuna evidence.

Requires Python 3.11+ and complete git history. No GitHub API, private campaign
state, author identities, or local binary paths are included in the output.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import datetime, timezone
import json
import math
from pathlib import Path
import re
import subprocess
import tomllib


REPO = Path(__file__).resolve().parents[3]
CATALOG = "decompiler/crates/kuna-decomp/phases.toml"
PHASES = [
    ("P0", "Knowledge", "Configuration, symbols & assertions"),
    ("P1", "Partition", "Images, code boundaries & entry points"),
    ("P2", "Lift & flow", "Instructions, control flow & jump tables"),
    ("P3", "Dataflow", "Definitions, SSA & simplification"),
    ("P4", "Calls", "Prototypes, arguments & calling conventions"),
    ("P5", "Types", "Value ranges & type inference"),
    ("P6", "Variables", "Storage, stack frames & variable merging"),
    ("P7", "Regions", "The region hierarchy"),
    ("P8", "Structure", "Statements, loops & goto quality"),
    ("P9", "Emit", "Readable output & surface refinement"),
]
MILESTONES = [
    {"date": "2026-06-05", "title": "Ghidra foundation", "commit": "17b3c5eb",
     "detail": "The Ghidra C++ decompiler and its regression corpus are vendored."},
    {"date": "2026-06-10", "title": "Rust port begins", "commit": "6a095f8a",
     "detail": "A six-crate Rust workspace and a differential verification harness."},
    {"date": "2026-06-19", "title": "675 / 675 parity", "commit": "bca4ae8d",
     "detail": "The initial Rust port matches all upstream datatest assertions."},
    {"date": "2026-06-20", "title": "Rust takes over", "commit": "9346a1a5",
     "detail": "The C++ engine is removed after verification; Rust is the engine."},
    {"date": "2026-07-04", "title": "DecBench loop", "commit": "6b270a3e",
     "detail": "Real-binary benchmark differences drive the mine, triage and rescore loop."},
]


def git(repo: Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(repo), *args], text=True, encoding="utf-8", errors="replace"
    )


def tracked(repo: Path, *paths: str) -> list[str]:
    return [p for p in git(repo, "ls-files", "-z", "--", *paths).split("\0") if p]


def phase_names(paths: list[str]) -> dict[str, str]:
    owners: dict[str, set[str]] = defaultdict(set)
    for path in paths:
        match = re.search(r"kuna-decomp/src/p([0-9])_[^/]+/", path)
        if match and Path(path).suffix == ".rs":
            owners[Path(path).stem].add("P" + match[1])
    return {name: next(iter(phases)) for name, phases in owners.items()
            if len(phases) == 1 and name not in {"mod", "lib", "tests"}}


def destination(path: str) -> str:
    path = re.sub(r"\{[^{}]* => ([^{}]*)\}", r"\1", path)
    return path.rsplit(" => ", 1)[-1]


def phase_for(path: str, names: dict[str, str]) -> str | None:
    match = re.search(r"kuna-decomp/src/[ps]([0-9])_[^/]+/", path)
    if match:
        return "P" + match[1]
    if "kuna-decomp/src/" in path or path.startswith("decompiler/cpp/"):
        return names.get(Path(path).stem)
    return None


def is_source(path: str) -> bool:
    return path.startswith(("decompiler/", "rust/")) and Path(path).suffix in {
        ".rs", ".cc", ".hh", ".cpp", ".h", ".c"
    }


def has_agent_attribution(message: str) -> bool:
    if "[AUTOMATED]" in message.upper():
        return True
    return any(re.search(r"\b(claude|codex|chatgpt|copilot)\b", line, re.I)
               for line in message.splitlines() if re.match(r"co-authored-by:", line, re.I))


def parse_commits(log: str, names: dict[str, str]) -> list[dict]:
    commits = []
    for chunk in log.split("\x1e"):
        if not chunk.strip():
            continue
        sha, stamp, message, stats = chunk.split("\x1f", 3)
        day = datetime.fromisoformat(stamp).astimezone(timezone.utc).date().isoformat()
        added = removed = 0
        phases: dict[str, dict] = {}
        records = []
        for line in stats.splitlines():
            fields = line.split("\t", 2)
            if len(fields) != 3:
                continue
            plus, minus, raw_path = fields
            path = destination(raw_path)
            if re.fullmatch(r"docs/features/[^/]+/record\.json", path):
                records.append(Path(path).parent.name)
            if plus == "-" or minus == "-" or not is_source(path):
                continue
            a, d = int(plus), int(minus)
            added += a
            removed += d
            phase = phase_for(path, names)
            if phase:
                bucket = phases.setdefault(phase, {"added": 0, "removed": 0})
                bucket["added"] += a
                bucket["removed"] += d
        commits.append({"sha": sha, "date": day, "subject": message.splitlines()[0],
                        "agent": has_agent_attribution(message), "added": added,
                        "removed": removed, "phases": phases, "records": records})
    return sorted(commits, key=lambda c: (c["date"], c["sha"]))


def finite(value) -> bool:
    return type(value) in (int, float) and math.isfinite(value)


def frontmatter(text: str) -> dict[str, str]:
    match = re.match(r"---\r?\n(.*?)\r?\n---", text, re.S)
    if not match:
        return {}
    return {m[1]: m[2].strip().strip("\"'") for line in match[1].splitlines()
            if (m := re.match(r"([\w-]+):\s*(.+)", line))}


def export(repo: Path) -> dict:
    if git(repo, "rev-parse", "--is-shallow-repository").strip() == "true":
        raise ValueError("dev-viz requires full git history (checkout fetch-depth: 0)")
    sha = git(repo, "rev-parse", "HEAD").strip()
    names = phase_names(tracked(repo, "decompiler/crates/kuna-decomp/src"))
    log = git(repo, "log", "HEAD", "--no-merges", "--find-renames", "--numstat",
              "--format=%x1e%H%x1f%cI%x1f%B%x1f")
    commits = parse_commits(log, names)
    if not commits:
        raise ValueError("dev-viz requires at least one non-merge commit")

    text = (repo / CATALOG).read_text()
    rows = tomllib.loads(text)["settable"]
    lines = {m[1]: i for i, line in enumerate(text.splitlines(), 1)
             if (m := re.match(r'option = "([^"]+)"', line))}
    options = [{"name": r["option"], "phase": r["phase"], "default": r["default"],
                "source": r["source_decompiler"], "inspiration": r["inspiration"],
                "summary": r["summary"], "tier": r["tier"], "line": lines[r["option"]],
                "decbench": "decbench:" in r["inspiration"].lower()}
               for r in rows]
    options.sort(key=lambda r: r["name"])
    option_names = {o["name"] for o in options}

    triage = []
    for path in tracked(repo, "docs/decbench/triage"):
        if not path.endswith(".md"):
            continue
        fm = frontmatter((repo / path).read_text())
        if not fm.get("case_id"):
            continue
        triage.append({"case": fm["case_id"], "pool": fm.get("pool", "unrecorded"),
                       "status": fm.get("status", "unrecorded"), "path": path,
                       "feature": fm.get("feature_slug", "")})
    novel_slugs = {r["feature"] for r in triage if r["pool"].lower() == "novel"}
    first_records = {}
    for commit in commits:
        for slug in commit.pop("records"):
            first_records.setdefault(slug, {"date": commit["date"], "sha": commit["sha"]})

    records = []
    for path in tracked(repo, "docs/features"):
        if not re.fullmatch(r"docs/features/[^/]+/record\.json", path):
            continue
        rec = json.loads((repo / path).read_text())
        slug = Path(path).parent.name
        option = rec.get("option")
        metadata = " ".join(str(rec.get(k, "")) for k in ("pool", "test_name", "inspiration"))
        item = {"slug": slug, "path": path,
                "option": option if isinstance(option, str) and option in option_names else None,
                "source": rec.get("source_decompiler") or "unrecorded",
                "novel": slug in novel_slugs or bool(re.search(r"\bnovel\b", metadata, re.I)),
                "firstRecorded": first_records.get(slug),
                "speed": rec["speed_delta_pct"] if finite(rec.get("speed_delta_pct")) else None}
        if finite(rec.get("ged_before")) and finite(rec.get("ged_after")):
            item["ged"] = {"before": rec["ged_before"], "after": rec["ged_after"],
                           "approximated": rec.get("ged_approximated"),
                           "case": rec.get("ged_case") or rec.get("case_id") or rec.get("opportunity") or slug,
                           "commit": rec.get("ged_kuna_commit")}
        records.append(item)

    baseline = json.loads((repo / "docs/baseline.json").read_text())
    stages = json.loads((repo / "docs/baseline-stages.json").read_text())
    novel_text = (repo / "docs/decbench/novel.md").read_text()
    novel_match = re.search(r"([\d,]+) cases in ([\d,]+) groups", novel_text)
    phase_counts = Counter(o["phase"] for o in options)
    return {
        "schemaVersion": 1,
        "meta": {"sha": sha, "repository": "https://github.com/Noelo-Lab/kuna",
                 "start": commits[0]["date"], "end": commits[-1]["date"],
                 "portEnd": "2026-06-19", "timezone": "UTC"},
        "phases": [{"id": pid, "name": name, "description": desc, "options": phase_counts[pid]}
                   for pid, name, desc in PHASES],
        "milestones": MILESTONES,
        "commits": commits,
        "options": options,
        "records": records,
        "triage": triage,
        "novelPool": {
            "cases": int(novel_match[1].replace(",", "")) if novel_match else None,
            "groups": int(novel_match[2].replace(",", "")) if novel_match else None,
            "path": "docs/decbench/novel.md",
        },
        "baselines": {"upstream": baseline["data_footer"], "stages": stages["data_footer"]},
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=REPO)
    parser.add_argument("--output", type=Path, default=Path(__file__).with_name("data.json"))
    args = parser.parse_args()
    data = export(args.repo)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(data, ensure_ascii=False, separators=(",", ":"), allow_nan=False) + "\n")
    print(f"dev-viz: {len(data['commits'])} commits, {len(data['options'])} options, "
          f"{len(data['records'])} feature records → {args.output}")


if __name__ == "__main__":
    main()
