"""Read decompiler/crates/kuna-decomp/phases.toml (groups, subphases, surfaces,
settables). Uses tomllib when the interpreter has it; otherwise a line parser
covering the flat ``key = value`` subset phases.toml actually uses (quoted
strings with escapes, booleans, integers) -- same precedent as
scripts/check_test_layout.py, which must run on Python 3.10."""
import re
from pathlib import Path

from scripts.paths import repo_root

try:
    import tomllib
except ModuleNotFoundError:  # Python < 3.11
    tomllib = None

TABLES = ("group", "subphase", "surface", "settable")
_HEADER = re.compile(r"^\[\[([A-Za-z_][A-Za-z0-9_]*)\]\]\s*(?:#.*)?$")
_KEYVAL = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.+)$")
_STRING = re.compile(r'^"(?:[^"\\]|\\.)*"')


def default_path() -> Path:
    return repo_root() / "decompiler" / "crates" / "kuna-decomp" / "phases.toml"


def _parse_value(raw: str):
    raw = raw.strip()
    if raw.startswith('"'):
        match = _STRING.match(raw)
        if not match:
            raise ValueError(f"unterminated string: {raw!r}")
        import json

        return json.loads(match.group(0))
    token = raw.split("#", 1)[0].strip()
    if token == "true":
        return True
    if token == "false":
        return False
    if re.fullmatch(r"-?\d+", token):
        return int(token)
    raise ValueError(f"unsupported TOML value in fallback parser: {raw!r}")


def _fallback_parse(text: str) -> dict:
    tables: dict = {}
    current = None
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        header = _HEADER.match(stripped)
        if header:
            current = {}
            tables.setdefault(header.group(1), []).append(current)
            continue
        keyval = _KEYVAL.match(stripped)
        if keyval and current is not None:
            current[keyval.group(1)] = _parse_value(keyval.group(2))
            continue
        raise ValueError(f"fallback parser cannot read line: {line!r}")
    return tables


def load(path: Path | None = None) -> dict:
    path = path or default_path()
    text = path.read_text(encoding="utf-8")
    data = tomllib.loads(text) if tomllib else _fallback_parse(text)
    return {name: data.get(name, []) for name in TABLES}


def group_phase_map(data: dict) -> dict:
    return {row["group"]: row["phase"] for row in data["group"]}
