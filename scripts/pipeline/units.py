"""Build a "small-units" worklist of self-contained angr decompiler passes that make good
single-PR port targets, by **source-parsing** the angr repo (never importing/executing angr).

Two angr source trees are walked (under ``config.angr_repo()``):

  - ``angr/analyses/decompiler/peephole_optimizations/*.py`` -- ~55 expression/statement
    rewrite rules. Each file defines one class subclassing ``PeepholeOptimizationExprBase`` /
    ``PeepholeOptimizationStmtBase`` / ``PeepholeOptimizationMultiStmtBase`` with a ``NAME``
    (and usually a ``DESCRIPTION``). Empty stub files (``inlined_strcpy.py`` etc.) have no
    class and are skipped.
  - ``angr/analyses/decompiler/optimization_passes/*.py`` -- the stateless **local** passes
    (canary/save-area removers, div/mod simplifiers, const-deref, inlined-libc recovery,
    getpc/gp-setting, determine-load-sizes, ...). Each defines a class subclassing an
    ``*OptimizationPass`` base with ``ARCHES`` / ``PLATFORMS`` / ``NAME`` / ``DESCRIPTION``.
    The structural/CFG-graph passes (return-duplication, code-motion, lowered-switch, ...)
    are *not* small units and are excluded by an allowlist.

We parse with the ``ast`` module (read -> ``ast.parse`` -> walk ``ClassDef``), never executing
angr. ``DESCRIPTION = __doc__.strip()`` is common and unresolvable statically, so the class
docstring is used as the fallback description.

Optionally we best-effort scan the matching peephole test files for a concrete
(binary, function) reproduce target and attach it to the pass whose class the test imports.
When no confident target is found a unit gets ``repro="synthesize"`` and ``comparable=False``.

Output: ``docs/pipeline/units.json`` -- the schema consumed by ``scripts.pipeline.select``.

    python -m scripts.pipeline.units              # write docs/pipeline/units.json
    python -m scripts.pipeline.units --json       # also print the list to stdout
    python -m scripts.pipeline.units --limit 10   # cap entries (for testing)
"""
from __future__ import annotations

import argparse
import ast
import json
import os
import re
import sys

from . import config

# --- what we consider a "small unit" ----------------------------------------

# Peephole base classes (a ClassDef qualifies if it subclasses one of these).
_PEEPHOLE_BASES = {
    "PeepholeOptimizationExprBase",
    "PeepholeOptimizationStmtBase",
    "PeepholeOptimizationMultiStmtBase",
}

# The stateless *local* optimization passes that are good 1-PR units. The
# optimization_passes/ dir also holds structural/CFG passes (return duplication, code
# motion, lowered-switch, cross-jump, dedup, ...) -- those are not small units, so we keep
# an explicit allowlist of source files instead of taking everything that ends in
# OptimizationPass.
_OPTPASS_FILES = {
    "stack_canary_simplifier.py",
    "win_stack_canary_simplifier.py",
    "base_ptr_save_simplifier.py",
    "register_save_area_simplifier.py",
    "register_save_area_simplifier_adv.py",
    "ret_addr_save_simplifier.py",
    "div_simplifier.py",
    "mod_simplifier.py",
    "const_derefs.py",
    "x86_gcc_getpc_simplifier.py",
    "mips_gp_setting_simplifier.py",
    "inlined_memcpy_simplifier.py",
    "inlined_memset_simplifier.py",
    "inlined_strcpy_simplifier.py",
    "inlined_strlen_simplifier.py",
    "inlined_wcscpy_simplifier.py",
    "determine_load_sizes.py",
}

# Peephole test files we mine for concrete (binary, function) reproduce targets.
_PEEPHOLE_TEST_FILES = [
    "test_peephole_optimizations.py",
    "test_optimization_passes.py",
    "test_peephole_inline_memset.py",
    "test_peephole_concat.py",
    "test_peephole_wcscpy.py",
    "test_peephole_redundant_bitshifts.py",
]

# Test files whose binary-driven methods implicitly exercise one pass, but which do NOT import
# the pass class by name (they just decompile and grep the text). We map the file to the pass
# class it targets so its (binary, selector) becomes a repro target. Best-effort.
_TEST_FILE_TARGET_CLASS = {
    "test_peephole_inline_memset.py": "InlinedMemsetSimplifier",
    "test_peephole_wcscpy.py": "InlinedWcscpySimplifier",
    "test_peephole_concat.py": "ConcatSimplifier",
    "test_peephole_redundant_bitshifts.py": "RemoveRedundantShiftsAroundComparators",
}

# Prefer x86 targets for the repro (kuna's strongest arch); these map a path component to a
# kuna --arch value.
_ARCH_BY_DIR = {
    "x86_64": "x86_64",
    "i386": "x86",
    "x86": "x86",
}
_PREFERRED_ARCH_DIRS = ("x86_64", "i386", "x86")


# --- ast helpers ------------------------------------------------------------

def _str_const(node):
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value
    return None


def _base_names(classdef):
    """Names referenced in a ClassDef's base list (both ``Name`` and ``Attr.attr``)."""
    names = []
    for b in classdef.bases:
        if isinstance(b, ast.Name):
            names.append(b.id)
        elif isinstance(b, ast.Attribute):
            names.append(b.attr)
    return names


def _class_attr(classdef, name):
    """Return the AST value node of a top-level ``name = ...`` class attribute, or None."""
    for stmt in classdef.body:
        if isinstance(stmt, ast.Assign):
            for tgt in stmt.targets:
                if isinstance(tgt, ast.Name) and tgt.id == name:
                    return stmt.value
        elif isinstance(stmt, ast.AnnAssign):
            if isinstance(stmt.target, ast.Name) and stmt.target.id == name and stmt.value is not None:
                return stmt.value
    return None


def _string_list(node):
    """Resolve a list/tuple of string constants to a Python list, else None."""
    if isinstance(node, (ast.List, ast.Tuple)):
        out = []
        for el in node.elts:
            s = _str_const(el)
            if s is None:
                return None
            out.append(s)
        return out
    return None


def _first_line(text):
    if not text:
        return ""
    for line in text.strip().splitlines():
        line = line.strip()
        if line:
            return line
    return ""


def _description_of(classdef):
    """Best-effort DESCRIPTION: the literal if present, else the docstring, else NAME-ish."""
    desc = _class_attr(classdef, "DESCRIPTION")
    s = _str_const(desc) if desc is not None else None
    if s:
        return _first_line(s)
    doc = ast.get_docstring(classdef)
    if doc:
        return _first_line(doc)
    return ""


# --- pass discovery ---------------------------------------------------------

class _Pass:
    """A discovered candidate pass (peephole rule or local optimization pass)."""

    def __init__(self, *, kind, file_rel, class_name, name, description, arches):
        self.kind = kind                 # "peephole" | "optpass"
        self.file_rel = file_rel         # path relative to angr repo
        self.class_name = class_name
        self.name = name or ""
        self.description = description or ""
        self.arches = arches or []       # list[str] (optpass ARCHES), may be empty


def _parse_peephole_file(path, file_rel):
    """Yield the single qualifying peephole class in a file (skip empty/stub files)."""
    try:
        with open(path) as fh:
            tree = ast.parse(fh.read(), filename=path)
    except (SyntaxError, OSError):
        return []
    out = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.ClassDef):
            continue
        if not (set(_base_names(node)) & _PEEPHOLE_BASES):
            continue
        name = _str_const(_class_attr(node, "NAME"))
        # require a NAME (the base classes have a placeholder NAME but subclasses override);
        # a real rule always sets one.
        if not name:
            continue
        out.append(_Pass(
            kind="peephole",
            file_rel=file_rel,
            class_name=node.name,
            name=name,
            description=_description_of(node),
            arches=[],
        ))
    return out


def _parse_optpass_file(path, file_rel):
    """Yield qualifying *OptimizationPass classes (those with NAME) in an allowlisted file."""
    try:
        with open(path) as fh:
            tree = ast.parse(fh.read(), filename=path)
    except (SyntaxError, OSError):
        return []
    out = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.ClassDef):
            continue
        bases = _base_names(node)
        if not any(b.endswith("OptimizationPass") for b in bases):
            continue
        name = _str_const(_class_attr(node, "NAME"))
        if not name:
            continue  # engine/state helper classes have no NAME
        arches_node = _class_attr(node, "ARCHES")
        arches = _string_list(arches_node) if arches_node is not None else None
        out.append(_Pass(
            kind="optpass",
            file_rel=file_rel,
            class_name=node.name,
            name=name,
            description=_description_of(node),
            arches=arches or [],
        ))
    return out


def discover_passes():
    """Walk both source dirs and return the list of candidate _Pass objects."""
    repo = config.angr_repo()
    passes = []

    ph_dir = repo / "angr" / "analyses" / "decompiler" / "peephole_optimizations"
    if ph_dir.is_dir():
        for fn in sorted(os.listdir(ph_dir)):
            if not fn.endswith(".py") or fn in ("__init__.py", "base.py", "utils.py"):
                continue
            p = ph_dir / fn
            rel = os.path.relpath(p, repo)
            passes.extend(_parse_peephole_file(p, rel))

    op_dir = repo / "angr" / "analyses" / "decompiler" / "optimization_passes"
    if op_dir.is_dir():
        for fn in sorted(os.listdir(op_dir)):
            if fn not in _OPTPASS_FILES:
                continue
            p = op_dir / fn
            rel = os.path.relpath(p, repo)
            passes.extend(_parse_optpass_file(p, rel))

    return passes


# --- best-effort repro-target mining ----------------------------------------

def _resolve_join(call):
    """``os.path.join(test_location, "a", "b")`` -> [components] after the base var, or None.

    Accepts test_location / bin_location as the base; requires all components be literals.
    """
    if not (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)
            and call.func.attr == "join"):
        return None
    args = call.args
    if not args:
        return None
    base = args[0]
    if not (isinstance(base, ast.Name) and base.id in ("test_location", "bin_location")):
        return None
    comps = []
    for a in args[1:]:
        s = _str_const(a)
        if s is None:
            return None
        comps.append(s)
    return comps


def _selector_from_subscript(sub):
    """``X.functions[<sel>]`` -> selector string ("name" / "0x..."), or None."""
    if not isinstance(sub, ast.Subscript):
        return None
    val = sub.value
    if not (isinstance(val, ast.Attribute) and val.attr == "functions"):
        return None
    key = sub.slice
    if isinstance(key, ast.Constant):
        if isinstance(key.value, str):
            return key.value
        if isinstance(key.value, int):
            return hex(key.value)
    return None


def _imported_pass_classes(tree):
    """Names imported ``from ...peephole_optimizations import X`` / ``...optimization_passes``."""
    names = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom) and node.module:
            if "peephole_optimizations" in node.module or "optimization_passes" in node.module:
                for alias in node.names:
                    names.add(alias.name)
    return names


def _arch_from_comps(comps):
    for c in comps:
        if c in _ARCH_BY_DIR:
            return _ARCH_BY_DIR[c]
    return ""


def _comps_preference(comps):
    """Lower is better: prefer x86_64/i386/x86 dirs for the repro binary."""
    for i, d in enumerate(_PREFERRED_ARCH_DIRS):
        if d in comps:
            return i
    return len(_PREFERRED_ARCH_DIRS)


def _mine_method_targets(node, test_loc):
    """Yield (preference, exists, binary_abs, selector, arch) for a binary-driven test method.

    A method qualifies when it has both an ``os.path.join(test_location, ...)`` binary and a
    ``X.functions[<sel>]`` selector. We keep the most-preferred-arch binary in the method.
    """
    bins = []        # list[ [components] ]
    selectors = []   # list[str]
    for sub in ast.walk(node):
        if isinstance(sub, ast.Call):
            comps = _resolve_join(sub)
            if comps is not None:
                bins.append(comps)
        sel = _selector_from_subscript(sub)
        if sel is not None:
            selectors.append(sel)
    if not (bins and selectors):
        return None
    bins.sort(key=_comps_preference)
    comps = bins[0]
    selector = selectors[0]
    arch = _arch_from_comps(comps)
    binary_abs = str(test_loc.joinpath(*comps))
    return (_comps_preference(comps), os.path.exists(binary_abs), binary_abs, selector, arch)


def mine_repro_targets():
    """Best-effort map ``class_name -> {binary, selector, arch, exists}`` from the test files.

    The binary-driven peephole tests don't name a specific pass -- they decompile a whole
    function and grep the text -- so two signals connect a pass to a (binary, selector):

      1. *file-level single import*: a test file that imports exactly one peephole/optpass
         class is taken to exercise that class with its binary-driven methods;
      2. *explicit file->class map* (``_TEST_FILE_TARGET_CLASS``): for the inline-libc /
         concat / redundant-bitshift files that import nothing but clearly target one pass.

    When several candidates exist for a class we keep the one whose binary is on disk, then the
    most x86-preferred arch. Partial/empty is fine -- callers fall back to repro="synthesize".
    """
    test_dir = config.angr_repo() / "tests" / "analyses" / "decompiler"
    test_loc = config.test_location()
    # class_name -> list of (preference, exists, binary_abs, selector, arch)
    candidates = {}

    for fn in _PEEPHOLE_TEST_FILES:
        path = test_dir / fn
        if not path.exists():
            continue
        try:
            with open(path) as fh:
                tree = ast.parse(fh.read(), filename=str(path))
        except (SyntaxError, OSError):
            continue

        imported = _imported_pass_classes(tree)
        # which class(es) this file's binary-driven methods should attach to
        target_classes = set()
        if len(imported) == 1:
            target_classes |= imported
        if fn in _TEST_FILE_TARGET_CLASS:
            target_classes.add(_TEST_FILE_TARGET_CLASS[fn])
        if not target_classes:
            continue

        for node in ast.walk(tree):
            if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                continue
            if not node.name.startswith("test"):
                continue
            cand = _mine_method_targets(node, test_loc)
            if cand is None:
                continue
            for cls in target_classes:
                candidates.setdefault(cls, []).append(cand)

    # collapse: prefer (exists, then preferred arch)
    out = {}
    for cls, cands in candidates.items():
        cands.sort(key=lambda t: (not t[1], t[0]))  # exists-first, then arch-preference
        pref, exists, binary_abs, selector, arch = cands[0]
        out[cls] = {"binary": binary_abs, "selector": selector, "arch": arch, "exists": exists}
    return out


# --- entry shaping ----------------------------------------------------------

# Heuristic change_kind buckets keyed on filename / class-name substrings.
_REMOVES_CODE = (
    "stack_canary", "base_ptr_save", "register_save_area", "ret_addr_save",
    "getpc", "gp_setting",
)
_INLINED_LIBC = ("inlined_mem", "inlined_str", "inlined_wcs")
_CORRECTNESS = ("div_simplifier", "mod_simplifier", "optimized_div", "modulo_simplifier",
                "sar_to_signed_div", "a_sub_a_div")


def _change_kind(p: "_Pass"):
    f = p.file_rel.lower()
    if any(k in f for k in _REMOVES_CODE):
        return "opt-in-tool"
    if any(k in f for k in _INLINED_LIBC):
        return "opt-in-tool"
    if any(k in f for k in _CORRECTNESS):
        return "correctness-fix"
    return "presentation-default"


def _test_id(p: "_Pass"):
    if p.kind == "peephole":
        slug = re.sub(r"[^a-zA-Z0-9]+", "_", p.name.strip()).strip("_").lower() or p.class_name.lower()
        return "angr_peephole_%s" % slug
    return "angr_optpass_%s" % p.class_name


def build_entries(limit=None):
    passes = discover_passes()
    repro = mine_repro_targets()

    entries = []
    for p in passes:
        target = repro.get(p.class_name)
        has_repro = bool(target and target.get("exists"))
        kinds = ["small-unit", p.kind]
        change_kind = _change_kind(p)

        if has_repro:
            selector = target["selector"]
            binary = target["binary"]
            arch = target.get("arch") or ""
            comparable = True
            repro_mode = "binary"
        else:
            selector = p.name        # no concrete target: use the rule NAME as the selector
            binary = ""
            arch = ""
            # ARCHES gives a hint even without a binary; surface x86 when present
            if any(a in ("X86", "AMD64") for a in p.arches):
                arch = "x86_64"
            comparable = False
            repro_mode = "synthesize"

        entry = {
            "test_name": _test_id(p),
            "selector": selector,
            "binary": binary,
            "arch": arch,
            "score": 5,
            "comparable": comparable,
            "kinds": kinds,
            "reasons": [p.description] if p.description else [p.name],
            "source_decompiler": "angr",
            "inspiration": "%s::%s (NAME=%s)" % (p.file_rel, p.class_name, p.name),
            "change_kind": change_kind,
            "repro": repro_mode,
        }
        entries.append(entry)

    # stable de-dup on test_name (peephole NAME slugs are unique; guard anyway)
    seen = set()
    deduped = []
    for e in entries:
        if e["test_name"] in seen:
            continue
        seen.add(e["test_name"])
        deduped.append(e)
    entries = deduped

    # comparable (real repro) first, then peephole before optpass, then by test_name
    entries.sort(key=lambda e: (not e["comparable"],
                                0 if "peephole" in e["kinds"] else 1,
                                e["test_name"]))
    if limit is not None:
        entries = entries[:limit]
    return entries


def main(argv=None):
    p = argparse.ArgumentParser(
        prog="python -m scripts.pipeline.units",
        description="Generate the angr small-units (peephole + local optimization pass) worklist.")
    p.add_argument("--json", action="store_true", help="also print the entry list to stdout")
    p.add_argument("--limit", type=int, default=None, help="cap the number of entries (testing)")
    p.add_argument("--out", default=None, help="output path (default: docs/pipeline/units.json)")
    args = p.parse_args(argv)

    entries = build_entries(limit=args.limit)

    out = args.out or str(config.pipeline_docs_dir() / "units.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    payload = {
        "generated_by": "scripts.pipeline.units",
        "count": len(entries),
        "ranked": entries,
    }
    with open(out, "w") as fh:
        json.dump(payload, fh, indent=2)

    print("wrote %d units to %s" % (len(entries), out), file=sys.stderr)
    if args.json:
        print(json.dumps(entries, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
