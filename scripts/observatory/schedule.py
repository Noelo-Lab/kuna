"""The real pass order for the atlas: parse the committed, test-enforced
listing snapshot (tests/fixtures/list_action_decompile_snapshot.txt), then
attribute every pass to a defining module/folder and an action-group phase by
reading the declarative registry in infra/universalaction.rs (rrow! rows,
act! group literals, SchedNode container names) plus the ActionBase::new
definition sites across the source tree.

Line-format facts (print_action_line / print_rule_line, universalaction.rs):
action lines are index(4) + repeat(8) + 3 flag cols + depth*5+2 indent + name,
rule lines are index(4) + 2 flag cols + depth*5+2 indent + name -- so the name
column is ==2 (mod 5) on action lines and ==3 (mod 5) on rule lines, which
lets the parser recover kind and depth from column arithmetic alone."""
import re
from pathlib import Path

from scripts.paths import repo_root

SNAPSHOT = "decompiler/crates/kuna-decomp/tests/fixtures/list_action_decompile_snapshot.txt"
UNIVERSAL = "decompiler/crates/kuna-decomp/src/infra/universalaction.rs"
SRC = "decompiler/crates/kuna-decomp/src"

_RROW = re.compile(r'rrow!\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*((?:crate::)?[\w:]+)')
_BOXED = re.compile(r"act!\(\s*(?:[\w:]+::)*(\w+)::boxed\(\s*\"([^\"]+)\"\s*\)")
_NEWBOX = re.compile(r"act!\(\s*Box::new\(\s*(?:[\w:]+::)*(\w+)::new\(([^)]*)\)", re.S)
_CONTAINER = re.compile(r'name:\s*"([^"]+)"')
_DEFSITE = re.compile(r'(\w+)\s*\{\s*base:\s*ActionBase::new\(\s*[^,]*?,\s*"([^"]+)"\s*,\s*("[^"]+"|[\w.]+)\s*,?\s*\)', re.S)
_FOLDER = re.compile(r"^(p\d_\w+|substrate|infra)$")

LEGEND = {
    "repeat": "repeatapply: the node loops until no member applies",
    "!": "once per function",
    "S/A": "breakpoint columns (start / per-apply)",
    "D": "rule disabled under the listing's group filter",
}


def parse_snapshot(text: str) -> list:
    rows = []
    for line in text.splitlines():
        if not line.strip():
            continue
        name = line.split()[-1]
        col = len(line) - len(name)
        index = int(line[:4])
        flags = set(line[4:col].split())
        if col % 5 == 2 and col >= 17:
            kind, depth = "action-format", (col - 17) // 5
        elif col % 5 == 3 and col >= 8:
            kind, depth = "rule-format", (col - 8) // 5
        else:
            raise ValueError(f"unrecognized listing column {col}: {line!r}")
        rows.append(
            {
                "index": index,
                "name": name,
                "depth": depth,
                "lineKind": kind,
                "repeat": "repeat" in flags,
                "once": "!" in flags,
                "disabled": "D" in flags,
            }
        )
    return rows


def _module_file(root: Path, ctor_path: str) -> Path | None:
    parts = [part for part in ctor_path.split("::") if part and part != "crate"]
    while parts and not parts[-1][0].isupper():
        parts.pop()
    if len(parts) < 2:
        return None
    module = parts[-2]
    if _FOLDER.match(parts[0]) and len(parts) >= 3:
        candidate = root / SRC / parts[0] / f"{parts[1]}.rs"
        if candidate.is_file():
            return candidate
    hits = [
        path
        for path in (root / SRC).rglob(f"{module}.rs")
        if path.name != "mod.rs"
    ] + list((root / SRC).rglob(f"{module}/mod.rs"))
    return hits[0] if len(hits) == 1 else None


def _folder_of(root: Path, path: Path) -> str:
    rel = path.relative_to(root / SRC)
    top = rel.parts[0]
    return top if _FOLDER.match(top) else "src"


def extract_registry(root: Path | None = None) -> dict:
    """Return {name: {kind, group, definedIn}} from the source tree."""
    root = root or repo_root()
    ua_text = (root / UNIVERSAL).read_text(encoding="utf-8")
    known: dict = {}

    for name, group, ctor in _RROW.findall(ua_text):
        module = _module_file(root, ctor)
        known[name] = {
            "kind": "rule",
            "group": group,
            "definedIn": str(module.relative_to(root)) if module else None,
        }

    struct_group = dict(_BOXED.findall(ua_text))
    for struct, args in _NEWBOX.findall(ua_text):
        literals = re.findall(r'"([^"]+)"', args)
        if literals:
            struct_group.setdefault(struct, literals[-1])

    for path in sorted((root / SRC).rglob("*.rs")):
        if path.stem == "tests" or path.stem.endswith("_tests") or "tests" in path.parts:
            continue
        text = path.read_text(encoding="utf-8")
        if "ActionBase::new" not in text:
            continue
        for struct, name, group_arg in _DEFSITE.findall(text):
            if name in known:
                continue
            group = group_arg.strip('"') if group_arg.startswith('"') else struct_group.get(struct)
            known[name] = {
                "kind": "action",
                "group": group,
                "definedIn": str(path.relative_to(root)),
            }

    for name in _CONTAINER.findall(ua_text):
        known.setdefault(name, {"kind": "container", "group": None, "definedIn": UNIVERSAL})
    return known


def _grep_folder(root: Path, name: str) -> str | None:
    needle = f'"{name}"'
    folders = set()
    for path in (root / SRC).rglob("*.rs"):
        if needle in path.read_text(encoding="utf-8"):
            folders.add(_folder_of(root, path))
    return folders.pop() if len(folders) == 1 else None


def _phase_of_folder(folder: str | None) -> str | None:
    if folder and folder[0] == "p" and folder[1].isdigit():
        return f"P{folder[1]}"
    return folder


def build(group_phase: dict, root: Path | None = None) -> dict:
    root = root or repo_root()
    rows = parse_snapshot((root / SNAPSHOT).read_text(encoding="utf-8"))
    registry = extract_registry(root)
    passes, unmapped = [], []
    for row in rows:
        info = registry.get(row["name"])
        defined_in = info["definedIn"] if info else None
        if defined_in is None:
            folder = _grep_folder(root, row["name"])
            if folder is None:
                unmapped.append(row["name"])
            elif info is None:
                info = {"kind": None, "group": None, "definedIn": None}
        else:
            folder = _folder_of(root, root / defined_in) if SRC in defined_in else None
        group = info["group"] if info else None
        passes.append(
            {
                "index": row["index"],
                "name": row["name"],
                "kind": info["kind"] if info else None,
                "depth": row["depth"],
                "repeat": row["repeat"],
                "once": row["once"],
                "disabled": row["disabled"],
                "group": group,
                "phase": group_phase.get(group),
                "definedIn": defined_in,
                "folderPhase": _phase_of_folder(folder),
            }
        )
    return {
        "source": SNAPSHOT,
        "note": "default `decompile` action listing (D = disabled under those groups)",
        "legend": LEGEND,
        "passes": passes,
        "unmapped": sorted(set(unmapped)),
    }
