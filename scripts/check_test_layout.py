#!/usr/bin/env python3
"""Reject integration-test source files omitted from explicitly declared suites."""
from collections import Counter
from pathlib import Path
import re
import sys
import tomllib


def check(root: Path) -> list[str]:
    errors = []
    for manifest in sorted((root / "decompiler/crates").glob("*/Cargo.toml")):
        config = tomllib.loads(manifest.read_text())
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
                for relative in re.findall(r'#\[path\s*=\s*"([^"]+)"\]', path.read_text()):
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
