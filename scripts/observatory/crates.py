"""Workspace crate inventory (cargo metadata --no-deps) plus line counts for
tracked Rust sources, per crate and per kuna-decomp phase folder/module."""
import json
import subprocess
from pathlib import Path

from scripts.paths import repo_root

DECOMP = "kuna-decomp"


def _tracked_rs(root: Path) -> list:
    out = subprocess.run(
        ["git", "ls-files", "decompiler/crates/*.rs", "decompiler/crates/**/*.rs"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    )
    return [line for line in out.stdout.splitlines() if line.endswith(".rs")]


def _line_count(path: Path) -> int:
    return path.read_text(encoding="utf-8", errors="replace").count("\n")


def collect(root: Path | None = None) -> dict:
    """Return {"crates": [...], "folders": {folder: {files, loc, modules[]}}}."""
    root = root or repo_root()
    meta = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=root / "decompiler",
            capture_output=True,
            text=True,
            check=True,
        ).stdout
    )

    counts: dict = {}
    folders: dict = {}
    src_prefix = f"decompiler/crates/{DECOMP}/src/"
    for rel in _tracked_rs(root):
        crate = rel.split("/")[2]
        loc = _line_count(root / rel)
        stat = counts.setdefault(crate, {"files": 0, "loc": 0})
        stat["files"] += 1
        stat["loc"] += loc
        if rel.startswith(src_prefix):
            parts = rel[len(src_prefix) :].split("/")
            if len(parts) > 1:
                folder = folders.setdefault(parts[0], {"files": 0, "loc": 0, "modules": []})
                folder["files"] += 1
                folder["loc"] += loc
                folder["modules"].append(
                    {
                        "file": "/".join(parts[1:]),
                        "loc": loc,
                        "native": parts[-1].startswith("kuna_"),
                    }
                )
    for folder in folders.values():
        folder["modules"].sort(key=lambda module: -module["loc"])

    crates = []
    for package in sorted(meta["packages"], key=lambda item: item["name"]):
        path = Path(package["manifest_path"]).parent
        stat = counts.get(path.name, {"files": 0, "loc": 0})
        crates.append(
            {
                "name": package["name"],
                "path": str(path.relative_to(root)),
                "description": package.get("description") or "",
                "files": stat["files"],
                "loc": stat["loc"],
                "deps": sorted(
                    dep["name"] for dep in package["dependencies"] if dep["name"].startswith("kuna-")
                ),
            }
        )
    return {"crates": crates, "folders": folders}
