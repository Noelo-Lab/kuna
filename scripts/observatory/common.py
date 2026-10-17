"""Shared helpers for the observatory generators: repo paths, atomic JSON
writes, and HTML-safe JSON inlining (pages are self-contained file:// pages,
so data is inlined, never fetched)."""
import json
import os
import subprocess
from datetime import datetime, timezone
from pathlib import Path

from scripts.paths import repo_root

SCHEMA_VERSION = 1


def git_sha(root: Path | None = None) -> str:
    root = root or repo_root()
    out = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True
    )
    return out.stdout.strip()


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def meta(root: Path | None = None) -> dict:
    return {"sha": git_sha(root), "generated": utc_now()}


def write_json(path: Path, obj) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    tmp.write_text(json.dumps(obj, allow_nan=False, separators=(",", ":")), encoding="utf-8")
    os.replace(tmp, path)


def inline_json(obj) -> str:
    """JSON safe to embed inside a <script> element (escapes ``</``)."""
    return json.dumps(obj, allow_nan=False, separators=(",", ":")).replace("</", "<\\/")
