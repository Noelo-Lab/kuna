"""Parse the two pipe tables in docs/phases.md: the P0-P9 phase table (name,
owned artifact, Band B / orthogonal markers) and the source-layout table
(phase -> folder -> "reads as"). Parsed, not hardcoded, so the atlas tracks
the committed doc."""
import re
from pathlib import Path

from scripts.paths import repo_root

_PHASE_ID = re.compile(r"^P\d$")
_EMPHASIS = re.compile(r"\s*\*\((.*?)\)\*\s*", re.S)


def default_path() -> Path:
    return repo_root() / "docs" / "phases.md"


def _rows(text: str, heading: str) -> list:
    section = text.split(f"\n## {heading}", 1)
    if len(section) != 2:
        raise ValueError(f"docs/phases.md: missing section '## {heading}'")
    rows = []
    for line in section[1].splitlines():
        if line.startswith("|"):
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
            if cells and not set("".join(cells)) <= {"-", " ", ":"}:
                rows.append(cells)
        elif rows:
            break
    return rows


def parse(path: Path | None = None) -> dict:
    text = (path or default_path()).read_text(encoding="utf-8")

    phases = []
    for cells in _rows(text, "Phases"):
        if len(cells) < 3 or not _PHASE_ID.match(cells[0]):
            continue
        name_cell = cells[1]
        notes = [match.group(1) for match in _EMPHASIS.finditer(name_cell)]
        phases.append(
            {
                "id": cells[0],
                "name": _EMPHASIS.sub(" ", name_cell).strip(),
                "artifact": cells[2],
                "band": "B" if any("Band B" in note for note in notes) else None,
                "orthogonal": any("orthogonal" in note for note in notes),
            }
        )

    folders = {}
    extra = []
    for cells in _rows(text, "Source layout"):
        if len(cells) < 3 or cells[1].count("`") < 2:
            continue
        folder = cells[1].strip("`").rstrip("/").strip("`")
        if _PHASE_ID.match(cells[0]):
            folders[cells[0]] = {"folder": folder, "readsAs": cells[2]}
        else:
            extra.append({"folder": folder, "readsAs": cells[2]})

    for phase in phases:
        layout = folders.get(phase["id"])
        if layout is None:
            raise ValueError(f"docs/phases.md: no source-layout row for {phase['id']}")
        phase.update(layout)
    return {"phases": phases, "extraFolders": extra}
