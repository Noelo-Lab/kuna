"""Assemble atlas.json: the P0-P9 phase cards (role, folder, spec chapter,
modules, groups, subphases, options, surfaces, sizes), the crate inventory,
and the real pass schedule. Contract documented in docs/observatory.md."""
from pathlib import Path

from scripts.observatory import common, crates, phasesdoc, registry, schedule, specmap


def _by_phase(rows: list, key: str = "phase") -> dict:
    grouped: dict = {}
    for row in rows:
        grouped.setdefault(row.get(key, ""), []).append(row)
    return grouped


def assemble(doc: dict, reg: dict, code: dict, owners: dict, sched: dict, meta: dict) -> dict:
    groups = _by_phase(reg["group"])
    subphases = _by_phase(reg["subphase"])
    surfaces = _by_phase(reg["surface"])
    settables = _by_phase(reg["settable"])

    option_fields = ("option", "values", "default", "tier", "subphase", "summary", "symptoms")
    phases = []
    for phase in doc["phases"]:
        folder = phase["folder"]
        stats = code["folders"].get(folder, {"files": 0, "loc": 0, "modules": []})
        phases.append(
            {
                **phase,
                "specChapter": specmap.chapter_for(owners, "kuna-decomp", folder),
                "files": stats["files"],
                "loc": stats["loc"],
                "modules": stats["modules"],
                "groups": [
                    {"group": row["group"], "subphase": row["subphase"], "note": row["note"]}
                    for row in groups.get(phase["id"], [])
                ],
                "subphases": subphases.get(phase["id"], []),
                "options": [
                    {field: row.get(field, "") for field in option_fields}
                    for row in settables.get(phase["id"], [])
                ],
                "surfaces": [
                    {"surface": row["surface"], "subphase": row["subphase"]}
                    for row in surfaces.get(phase["id"], [])
                ],
            }
        )

    extra = []
    for info in doc["extraFolders"]:
        stats = code["folders"].get(info["folder"], {"files": 0, "loc": 0, "modules": []})
        extra.append(
            {
                **info,
                "specChapter": specmap.chapter_for(owners, "kuna-decomp", info["folder"]),
                "files": stats["files"],
                "loc": stats["loc"],
                "modules": stats["modules"],
            }
        )

    return {
        "schemaVersion": common.SCHEMA_VERSION,
        "meta": meta,
        "phases": phases,
        "extraFolders": extra,
        "crates": code["crates"],
        "schedule": sched,
        "counts": {name: len(reg[name]) for name in registry.TABLES},
    }


def generate(root: Path | None = None, out: Path | None = None) -> dict:
    root = root or common.repo_root()
    reg = registry.load(root / "decompiler" / "crates" / "kuna-decomp" / "phases.toml")
    return assemble(
        doc=phasesdoc.parse(root / "docs" / "phases.md"),
        reg=reg,
        code=crates.collect(root),
        owners=specmap.folder_chapters(root),
        sched=schedule.build(registry.group_phase_map(reg), root),
        meta=common.meta(root),
    )
