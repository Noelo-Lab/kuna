"""Churn & ownership: per-file change aggregation over the full history, in
time windows, with agent-vs-human share and top authors -- the data behind
the treemap page. Only currently-tracked files are kept (the treemap's area
is live size); vendored specs/ and binary paths are skipped."""
import subprocess
from datetime import datetime, timedelta, timezone
from pathlib import Path

from scripts.observatory import common, gitstats

WINDOW_DAYS = (30, 90, 365)
AREAS = ("decompiler", "scripts", "tools", "integrations", "docs", "tests")
_EMPTY = {"added": 0, "removed": 0, "commits": 0, "agentCommits": 0}


def _classify(path: str):
    parts = path.split("/")
    if parts[0] not in AREAS:
        return None
    crate = parts[2] if path.startswith("decompiler/crates/") and len(parts) > 3 else None
    phase = None
    if crate == "kuna-decomp" and path.startswith("decompiler/crates/kuna-decomp/src/") and len(parts) > 5:
        top = parts[4]
        if top[0] == "p" and top[1:2].isdigit():
            phase = f"P{top[1]}"
        elif top in ("substrate", "infra"):
            phase = top
    return {"area": parts[0], "crate": crate, "phase": phase}


def _line_count(path: Path) -> int:
    try:
        return path.read_bytes().count(b"\n")
    except OSError:
        return 0


def aggregate(commits: list, tracked: set, now: datetime) -> list:
    cutoffs = {f"d{days}": (now - timedelta(days=days)).date().isoformat() for days in WINDOW_DAYS}
    files: dict = {}
    for commit in commits:
        for path, added, removed in commit["files"]:
            if path not in tracked:
                continue
            tags = _classify(path)
            if tags is None:
                continue
            entry = files.get(path)
            if entry is None:
                entry = files[path] = {
                    "path": path,
                    **tags,
                    "loc": 0,
                    "last": commit["date"],
                    "authors": {},
                    "windows": {name: dict(_EMPTY) for name in ("all", *cutoffs)},
                }
            entry["last"] = max(entry["last"], commit["date"])
            entry["authors"][commit["author"]] = entry["authors"].get(commit["author"], 0) + 1
            buckets = ["all"] + [name for name, cutoff in cutoffs.items() if commit["date"] >= cutoff]
            for name in buckets:
                window = entry["windows"][name]
                window["added"] += added
                window["removed"] += removed
                window["commits"] += 1
                window["agentCommits"] += 1 if commit["agent"] else 0
    for entry in files.values():
        top = sorted(entry["authors"].items(), key=lambda item: (-item[1], item[0]))[:3]
        entry["authors"] = [{"name": name, "commits": count} for name, count in top]
    return sorted(files.values(), key=lambda entry: entry["path"])


def generate(root: Path | None = None) -> dict:
    root = root or common.repo_root()
    tracked_list = subprocess.run(
        ["git", "ls-files"], cwd=root, capture_output=True, text=True, check=True
    ).stdout.splitlines()
    tracked = set(tracked_list)
    rows = aggregate(
        gitstats.parse_commits(gitstats.read_log(root)),
        tracked,
        datetime.now(timezone.utc),
    )
    for entry in rows:
        entry["loc"] = _line_count(root / entry["path"])
    return {
        "schemaVersion": common.SCHEMA_VERSION,
        "meta": common.meta(root),
        "windowsDays": list(WINDOW_DAYS),
        "areas": list(AREAS),
        "files": rows,
    }
