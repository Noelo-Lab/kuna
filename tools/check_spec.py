#!/usr/bin/env python3
"""check_spec.py — keep docs/spec/ honest against the live tree.

Checks (run from the repo root; exit 1 on any failure):
  1. Anchor paths resolve: every path inside a chapter's ```yaml Anchors:``` block,
     and every `decompiler/crates/...` path mentioned anywhere in docs/spec/ prose,
     exists on disk.
  2. Folder coverage: every top-level directory under kuna-decomp/src and
     kuna-analysis/src is owned by exactly one chapter's Anchors block.
  3. (--strict) Option mention: every `option = "..."` in phases.toml settableTable
     appears at least once in docs/spec/ (so no gated algorithm goes undescribed).

Usage: python3 tools/check_spec.py [--strict]
"""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SPEC = os.path.join(ROOT, "docs", "spec")
STRICT = "--strict" in sys.argv[1:]

failures = []


def chapters():
    for name in sorted(os.listdir(SPEC)):
        if name.endswith(".md") and name != "README.md":
            yield name, open(os.path.join(SPEC, name), encoding="utf-8").read()


def anchor_block(text):
    m = re.search(r"```yaml\nAnchors:\n(.*?)```", text, re.S)
    if not m:
        return []
    return [ln.strip().lstrip("- ").strip() for ln in m.group(1).splitlines() if ln.strip().startswith("-")]


# --- 1. anchors + inline paths resolve -------------------------------------
inline_path = re.compile(r"\bdecompiler/crates/[A-Za-z0-9_\-./]+")
owned = {}  # top-level dir -> [chapter]
for name, text in chapters():
    anchors = anchor_block(text)
    if not anchors:
        failures.append(f"{name}: no Anchors block")
    for a in anchors:
        if not os.path.exists(os.path.join(ROOT, a)):
            failures.append(f"{name}: anchor does not exist: {a}")
    for p in set(inline_path.findall(text)):
        p_clean = p.rstrip(".,)")
        if not os.path.exists(os.path.join(ROOT, p_clean)):
            failures.append(f"{name}: mentioned path does not exist: {p_clean}")
    for a in anchors:
        # Only directory anchors confer folder ownership; a file anchor (e.g.
        # substrate/dtype.rs cross-referenced from the types chapter) does not.
        if not os.path.isdir(os.path.join(ROOT, a)):
            continue
        for src_root in ("decompiler/crates/kuna-decomp/src", "decompiler/crates/kuna-analysis/src"):
            if a == src_root or a.startswith(src_root + "/"):
                rel = a[len(src_root):].strip("/")
                top = rel.split("/")[0] if rel else "*"
                owned.setdefault((src_root, top), []).append(name)

# --- 2. folder coverage ------------------------------------------------------
for src_root in ("decompiler/crates/kuna-decomp/src", "decompiler/crates/kuna-analysis/src"):
    full = os.path.join(ROOT, src_root)
    whole_crate_owners = owned.get((src_root, "*"), [])
    for entry in sorted(os.listdir(full)):
        if not os.path.isdir(os.path.join(full, entry)):
            continue
        owners = owned.get((src_root, entry), []) + whole_crate_owners
        if len(owners) == 0:
            failures.append(f"uncovered folder: {src_root}/{entry} (no chapter owns it)")
        elif len(set(owners)) > 1:
            failures.append(f"multiply-owned folder: {src_root}/{entry} -> {sorted(set(owners))}")

# --- 3. option mention (strict) ----------------------------------------------
if STRICT:
    toml = open(os.path.join(ROOT, "decompiler/crates/kuna-decomp/phases.toml"), encoding="utf-8").read()
    spec_all = " ".join(t for _, t in chapters())
    in_settable = False
    for line in toml.splitlines():
        if line.strip().startswith("[["):
            in_settable = line.strip() == "[[settable]]"
        elif in_settable and line.startswith('option = "'):
            opt = line.split('"')[1]
            if opt not in spec_all:
                failures.append(f"option not mentioned anywhere in docs/spec/: {opt}")

if failures:
    print(f"check-spec: {len(failures)} failure(s)")
    for f in failures:
        print(f"  {f}")
    sys.exit(1)
print(f"check-spec OK ({'strict' if STRICT else 'lenient'} mode)")
