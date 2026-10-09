#!/usr/bin/env python3
"""Reject integration-test source files omitted from explicitly declared suites."""
from collections import Counter
from pathlib import Path
import re
import sys

try:
    import tomllib
except ModuleNotFoundError:
    tomllib = None

REGISTRATION = re.compile(r'^\s*#\[path\s*=\s*"([^"]+)"\]\s*\n\s*mod\s+\w+;', re.M)


def load_manifest(text: str) -> dict:
    """Parse a Cargo manifest; without tomllib, read only what the check needs."""
    if tomllib is not None:
        return tomllib.loads(text)
    config = {"package": {}, "test": []}
    table = None
    for line in text.splitlines():
        line = line.split("#", 1)[0].strip()
        if line == "[[test]]":
            table = {}
            config["test"].append(table)
        elif line.startswith("["):
            table = config["package"] if line == "[package]" else None
        elif table is not None and "=" in line:
            key, value = (part.strip() for part in line.split("=", 1))
            table[key] = {"true": True, "false": False}.get(value, value.strip('"'))
    return config


def check(root: Path) -> list[str]:
    errors = []
    for manifest in sorted((root / "decompiler/crates").glob("*/Cargo.toml")):
        config = load_manifest(manifest.read_text())
        if config.get("package", {}).get("autotests", True):
            continue
        crate = manifest.parent
        references = Counter()
        for target in config.get("test", []):
            path = crate / target.get("path", f"tests/{target['name']}.rs")
            references[path] += 1
            if not path.is_file():
                errors.append(f"{path.relative_to(root)}: missing test entry point")
                continue
            if target["name"] == "integration":
                for relative in REGISTRATION.findall(path.read_text()):
                    references[path.parent / relative] += 1
        for path in sorted(set((crate / "tests").glob("*.rs")) | set(references)):
            if not path.is_file():
                errors.append(f"{path.relative_to(root)}: missing suite module")
            elif references[path] != 1:
                errors.append(f"{path.relative_to(root)}: expected one registration, found {references[path]}")
    return errors


if __name__ == "__main__":
    errors = check(Path(__file__).resolve().parents[1])
    if errors:
        print("\n".join(errors), file=sys.stderr)
        raise SystemExit(1)
    print("Integration-test layout: all source files registered exactly once")
