"""Map kuna-decomp src folders (and the kuna-analysis crate) to their owning
docs/spec chapter via each chapter's ``Anchors:`` block. Reimplements the
~15-line parse from tools/check_spec.py (that file runs its checks at import
time as a CI gate, so it is deliberately not imported here); the unit test
cross-pins the two by asserting every folder resolves to exactly one chapter."""
import re
from pathlib import Path

from scripts.paths import repo_root

_ANCHORS = re.compile(r"```yaml\nAnchors:\n(.*?)```", re.S)
_ROOTS = {
    "kuna-decomp": "decompiler/crates/kuna-decomp/src",
    "kuna-analysis": "decompiler/crates/kuna-analysis/src",
}


def chapters(spec_dir: Path | None = None):
    spec_dir = spec_dir or repo_root() / "docs" / "spec"
    for path in sorted(spec_dir.glob("*.md")):
        if path.name != "README.md":
            yield path, path.read_text(encoding="utf-8")


def folder_chapters(root: Path | None = None) -> dict:
    """Return {crate: {folder-or-"*": "docs/spec/<chapter>.md"}} from directory anchors."""
    root = root or repo_root()
    owners: dict = {crate: {} for crate in _ROOTS}
    for path, text in chapters(root / "docs" / "spec"):
        chapter = f"docs/spec/{path.name}"
        for block in _ANCHORS.finditer(text):
            for line in block.group(1).splitlines():
                anchor = line.strip().lstrip("-").strip()
                if not anchor or not (root / anchor).is_dir():
                    continue
                for crate, src in _ROOTS.items():
                    if anchor == src:
                        owners[crate]["*"] = chapter
                    elif anchor.startswith(src + "/"):
                        top = anchor[len(src) + 1 :].split("/", 1)[0]
                        owners[crate].setdefault(top, chapter)
    return owners


def chapter_for(owners: dict, crate: str, folder: str) -> str | None:
    crate_map = owners.get(crate, {})
    return crate_map.get(folder) or crate_map.get("*")
