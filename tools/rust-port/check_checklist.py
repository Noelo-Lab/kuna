#!/usr/bin/env python3
"""Validate docs/rust-port/checklist.json (the Rust-port backlog).

Checks, in order:
  - top-level shape (``version == 1``, ``items`` list) and per-item schema
    (required fields, types, enum values);
  - unique item ids;
  - every ``depends_on`` reference resolves to an existing item;
  - the dependency graph is acyclic;
  - every referenced module path exists in the repository;
  - ``cpp_blob_sha`` keys are a subset of the item's ``modules``;
  - every ``port`` item has at least one paired ``verify`` item.

On success prints per-wave item counts and the total port LOC. With
``--verify-sha`` it additionally re-hashes every referenced module with
``git hash-object`` and reports drift against the recorded blob shas
(useful before re-porting after an upstream sync). Exits nonzero on any
validation error.

CLI:
    python3 tools/rust-port/check_checklist.py [--checklist FILE] [--root DIR]
        [--verify-sha]
"""
import argparse
import json
import os
import re
import subprocess
import sys

REQUIRED_FIELDS = {
    "id": str,
    "kind": str,
    "wave": str,
    "crate": str,
    "title": str,
    "modules": list,
    "depends_on": list,
    "status": str,
    "attempts": int,
    "gate": str,
    "loc_estimate": int,
    "cpp_blob_sha": dict,
    "notes": str,
}
KINDS = ("port", "verify", "infra")
# "landed" (port/infra item merged) and "verified" (verdict accepted+merged) are the
# orchestrator's terminal statuses; "done" is kept for compatibility. The port-mode
# selector (kuna.pipeline.select_port) treats all three as satisfying a dependency.
STATUSES = ("todo", "in-progress", "blocked", "done", "landed", "verified")
WAVE_RE = re.compile(r"^W(\d|10|11)$")
WAVE_ORDER = ["W%d" % i for i in range(12)]


def _item_errors(item, index):
    """Return a list of schema error strings for one item dict."""
    errors = []
    where = "items[%d]" % index
    if not isinstance(item, dict):
        return ["%s: not an object" % where]
    if isinstance(item.get("id"), str):
        where = "items[%d] (%s)" % (index, item["id"])
    for field, ftype in REQUIRED_FIELDS.items():
        if field not in item:
            errors.append("%s: missing field %r" % (where, field))
        elif not isinstance(item[field], ftype) or isinstance(item[field], bool):
            errors.append("%s: field %r must be %s" % (where, field, ftype.__name__))
    for field in sorted(set(item) - set(REQUIRED_FIELDS)):
        errors.append("%s: unknown field %r" % (where, field))
    if errors:
        return errors
    if item["kind"] not in KINDS:
        errors.append("%s: kind %r not in %s" % (where, item["kind"], KINDS))
    if not WAVE_RE.match(item["wave"]):
        errors.append("%s: wave %r not W0..W11" % (where, item["wave"]))
    if item["status"] not in STATUSES:
        errors.append("%s: status %r not in %s" % (where, item["status"], STATUSES))
    if item["attempts"] < 0:
        errors.append("%s: attempts < 0" % where)
    if item["loc_estimate"] < 0:
        errors.append("%s: loc_estimate < 0" % where)
    for seq in ("modules", "depends_on"):
        if not all(isinstance(x, str) for x in item[seq]):
            errors.append("%s: %s entries must be strings" % (where, seq))
    for path, blob in item["cpp_blob_sha"].items():
        if not (isinstance(blob, str) and re.match(r"^[0-9a-f]{40}$", blob)):
            errors.append("%s: cpp_blob_sha[%r] is not a 40-hex sha" % (where, path))
        if path not in item["modules"]:
            errors.append("%s: cpp_blob_sha key %r not in modules" % (where, path))
    return errors


def _find_cycle(by_id):
    """Return a cycle as a list of ids, or None (iterative three-color DFS)."""
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {i: WHITE for i in by_id}
    for root in by_id:
        if color[root] != WHITE:
            continue
        stack = [(root, iter(by_id[root]["depends_on"]))]
        color[root] = GRAY
        path = [root]
        while stack:
            node, deps = stack[-1]
            advanced = False
            for dep in deps:
                if dep not in by_id:
                    continue  # reported separately
                if color[dep] == GRAY:
                    return path[path.index(dep):] + [dep]
                if color[dep] == WHITE:
                    color[dep] = GRAY
                    path.append(dep)
                    stack.append((dep, iter(by_id[dep]["depends_on"])))
                    advanced = True
                    break
            if not advanced:
                color[node] = BLACK
                path.pop()
                stack.pop()
    return None


def validate(data, root, verify_sha=False):
    """Return (errors, items). errors is a list of strings."""
    errors = []
    if not isinstance(data, dict):
        return (["top level: not an object"], [])
    if data.get("version") != 1:
        errors.append("top level: version must be 1 (got %r)" % data.get("version"))
    items = data.get("items")
    if not isinstance(items, list):
        errors.append("top level: 'items' must be a list")
        return (errors, [])

    for index, item in enumerate(items):
        errors.extend(_item_errors(item, index))
    if errors:
        return (errors, items)  # schema must hold before graph checks

    by_id = {}
    for item in items:
        if item["id"] in by_id:
            errors.append("duplicate id %r" % item["id"])
        by_id[item["id"]] = item

    for item in items:
        for dep in item["depends_on"]:
            if dep not in by_id:
                errors.append("%s: depends_on %r does not exist" % (item["id"], dep))

    cycle = _find_cycle(by_id)
    if cycle is not None:
        errors.append("dependency cycle: %s" % " -> ".join(cycle))

    checked = set()
    for item in items:
        for path in item["modules"]:
            if path in checked:
                continue
            checked.add(path)
            if not os.path.exists(os.path.join(root, path)):
                errors.append("%s: module path %r does not exist" % (item["id"], path))

    verified = set()
    for item in items:
        for dep in item["depends_on"]:
            if item["kind"] == "verify" and dep in by_id and by_id[dep]["kind"] == "port":
                verified.add(dep)
    for item in items:
        if item["kind"] == "port" and item["id"] not in verified:
            errors.append("%s: port item has no paired verify item" % item["id"])

    if verify_sha and not errors:
        recorded = {}
        for item in items:
            for path, blob in item["cpp_blob_sha"].items():
                recorded.setdefault(path, blob)
        for path in sorted(recorded):
            current = subprocess.check_output(
                ["git", "-C", root, "hash-object", path], text=True).strip()
            if current != recorded[path]:
                errors.append("sha drift: %s is %s, checklist recorded %s"
                              % (path, current[:12], recorded[path][:12]))
    return (errors, items)


def summarize(items, out=sys.stdout):
    """Print per-wave item counts (by kind) and total LOC of port items."""
    waves = {}
    for item in items:
        bucket = waves.setdefault(item["wave"], {"port": 0, "verify": 0,
                                                 "infra": 0, "loc": 0})
        bucket[item["kind"]] += 1
        if item["kind"] == "port":
            bucket["loc"] += item["loc_estimate"]
    print("%-5s %5s %7s %6s %9s" % ("wave", "port", "verify", "infra", "port-LOC"),
          file=out)
    total_loc = 0
    for wave in WAVE_ORDER + sorted(set(waves) - set(WAVE_ORDER)):
        if wave not in waves:
            continue
        b = waves[wave]
        total_loc += b["loc"]
        print("%-5s %5d %7d %6d %9d" % (wave, b["port"], b["verify"],
                                        b["infra"], b["loc"]), file=out)
    print("total items=%d  port LOC=%d" % (len(items), total_loc), file=out)


def main(argv=None):
    default_root = os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__))))
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--checklist",
                        default=os.path.join(default_root, "docs/rust-port/checklist.json"),
                        help="path to checklist.json (default: docs/rust-port/checklist.json)")
    parser.add_argument("--root", default=default_root,
                        help="repository root for module-path checks")
    parser.add_argument("--verify-sha", action="store_true",
                        help="re-hash modules with `git hash-object` and report drift")
    args = parser.parse_args(argv)

    try:
        with open(args.checklist) as f:
            data = json.load(f)
    except (OSError, ValueError) as e:
        print("error: cannot load %s: %s" % (args.checklist, e), file=sys.stderr)
        return 2

    errors, items = validate(data, args.root, verify_sha=args.verify_sha)
    if errors:
        for line in errors:
            print("error: %s" % line, file=sys.stderr)
        print("FAIL: %d error(s)" % len(errors), file=sys.stderr)
        return 1
    summarize(items)
    print("OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
