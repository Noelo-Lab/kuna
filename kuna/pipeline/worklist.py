"""Build a (binary, function) worklist by statically parsing a reference decompiler's
test suite -- for angr, ``test_decompiler.py``.

We AST-parse (never execute) the test file. For each ``test_*`` method we recover:
  - the target binary, from ``os.path.join(test_location|bin_location, "...", ...)``;
  - the target function, from the selector passed to the Decompiler analysis
    (``f = cfg.functions[<sel>]`` then ``p.analyses[Decompiler](...)(f, ...)``), falling
    back to any ``.functions[<sel>]`` subscript in the body;
  - Project kwargs that change the decompilation: ``arch=`` and ``load_debug_info=``;
  - whether the test passes a non-default ``options=`` / ``optimization_passes=`` (flagged,
    so the sweep can note lower fidelity -- we still run angr with its defaults).

AST parsing (not regex) is what lets us capture arch/load_debug/custom-options faithfully.
Output is ``docs/pipeline/worklist.json``.

    python -m kuna.pipeline.worklist                      # write docs/pipeline/worklist.json
    python -m kuna.pipeline.worklist --only-existing      # drop binaries not on disk
    python -m kuna.pipeline.worklist --print              # human summary to stdout
"""
from __future__ import annotations

import argparse
import ast
import json
import os
import sys
from dataclasses import dataclass, field, asdict
from typing import Optional

from . import config

# os.path.join base variables in test_decompiler.py and how they resolve.
_BASE_VARS = {"test_location", "bin_location", "bin_priv_location"}


@dataclass
class WorklistEntry:
    source_decompiler: str            # "angr"
    test_name: str                    # the test_* method name
    test_class: str                   # enclosing class, or ""
    test_file: str                    # absolute path to the test file
    lineno: int                       # method definition line
    binary_rel: list                  # path components after the base var
    binary_base: str                  # which base var ("test_location"/...)
    binary_path: str                  # resolved absolute path
    binary_exists: bool
    func_selector: str                # name / "0x..." / "@entry"
    arch: Optional[str] = None        # angr.Project(arch=...) override
    load_debug: bool = False          # angr.Project(load_debug_info=True)
    custom_options: bool = False      # test passes non-default options/optimization_passes
    confidence: str = "high"          # "high" (single target) | "low" (ambiguous/heuristic)
    notes: list = field(default_factory=list)

    @property
    def id(self) -> str:
        return "%s::%s" % (self.test_name, self.func_selector)


def _str_const(node):
    return node.value if isinstance(node, ast.Constant) and isinstance(node.value, str) else None


def _join_to_path(call):
    """Resolve an ``os.path.join(BASE, "a", "b")`` Call to (base_var, [components])."""
    if not (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute)
            and call.func.attr == "join"):
        return None
    args = call.args
    if not args:
        return None
    base = args[0]
    base_name = base.id if isinstance(base, ast.Name) else None
    if base_name not in _BASE_VARS:
        return None
    comps = []
    for a in args[1:]:
        s = _str_const(a)
        if s is None:
            return None  # a non-literal component -> we can't resolve statically
        comps.append(s)
    return base_name, comps


def _selector_from_subscript(sub):
    """``X.functions[<sel>]`` -> the selector string, or None if not a functions[...]."""
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
    # p.entry / proj.entry
    if isinstance(key, ast.Attribute) and key.attr == "entry":
        return "@entry"
    return None


def _is_decompiler_call(call):
    """True if ``call`` is the Decompiler analysis invocation (returns the target arg)."""
    if not isinstance(call, ast.Call):
        return None
    # Pattern A: p.analyses[Decompiler].prep(...)(<f>, ...)  -- call.func is itself a Call
    # Pattern B: angr.analyses.Decompiler(<f>, ...)
    def mentions_decompiler(node):
        for n in ast.walk(node):
            if isinstance(n, ast.Name) and n.id == "Decompiler":
                return True
            if isinstance(n, ast.Attribute) and n.attr == "Decompiler":
                return True
        return False
    if mentions_decompiler(call.func) and call.args:
        return call.args[0]
    return None


class _TestVisitor(ast.NodeVisitor):
    def __init__(self):
        self.path_vars = {}        # var -> (base, [components])
        self.proj_kwargs = {}      # arch / load_debug from any angr.Project(...)
        self.func_vars = {}        # var -> selector (from f = X.functions[sel])
        self.decomp_targets = []   # selectors passed to a Decompiler call (ordered)
        self.all_selectors = []    # every .functions[sel] selector seen (fallback)
        self.custom_options = False

    def visit_Assign(self, node):
        # var = os.path.join(...)
        if len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            var = node.targets[0].id
            j = _join_to_path(node.value)
            if j is not None:
                self.path_vars[var] = j
            sel = _selector_from_subscript(node.value)
            if sel is not None:
                self.func_vars[var] = sel
        self.generic_visit(node)

    def visit_Call(self, node):
        # angr.Project(path, arch=..., load_debug_info=...)
        f = node.func
        is_project = (isinstance(f, ast.Attribute) and f.attr == "Project") or (
            isinstance(f, ast.Name) and f.id == "Project")
        if is_project:
            for kw in node.keywords:
                if kw.arg == "arch":
                    s = _str_const(kw.value)
                    if s:
                        self.proj_kwargs["arch"] = s
                elif kw.arg == "load_debug_info":
                    if isinstance(kw.value, ast.Constant) and kw.value.value is True:
                        self.proj_kwargs["load_debug"] = True
        # the Decompiler analysis target
        tgt = _is_decompiler_call(node)
        if tgt is not None:
            sel = self._resolve_target(tgt)
            if sel is not None:
                self.decomp_targets.append(sel)
        # non-default options / optimization_passes anywhere
        for kw in node.keywords:
            if kw.arg in ("options", "optimization_passes"):
                # default passthrough is the param `decompiler_options` (a Name) or None.
                v = kw.value
                default_like = (
                    (isinstance(v, ast.Name) and v.id == "decompiler_options")
                    or (isinstance(v, ast.Constant) and v.value is None)
                )
                if not default_like:
                    self.custom_options = True
        # record any functions[...] selector for the fallback
        for n in ast.walk(node):
            s = _selector_from_subscript(n)
            if s is not None and s not in self.all_selectors:
                self.all_selectors.append(s)
        self.generic_visit(node)

    def _resolve_target(self, tgt):
        if isinstance(tgt, ast.Name):
            return self.func_vars.get(tgt.id)
        return _selector_from_subscript(tgt)


def _parse_method(method, klass, tree_file):
    v = _TestVisitor()
    for stmt in method.body:
        v.visit(stmt)

    # also catch functions[...] selectors assigned but used outside calls
    for n in ast.walk(method):
        s = _selector_from_subscript(n)
        if s is not None and s not in v.all_selectors:
            v.all_selectors.append(s)

    # pick the binary: prefer a path var that feeds a Project; else the only path var
    paths_found = list(v.path_vars.values())
    if not paths_found:
        return []

    base, comps = paths_found[0]
    # if several distinct paths, keep the first but flag low confidence
    distinct = {tuple([b] + c) for (b, c) in paths_found}
    multi_bin = len(distinct) > 1

    # pick selectors: prefer Decompiler targets; else all functions[] selectors
    selectors = v.decomp_targets or v.all_selectors
    if not selectors:
        return []
    # de-dup, preserve order
    seen = set()
    selectors = [s for s in selectors if not (s in seen or seen.add(s))]

    entries = []
    for sel in selectors:
        conf = "high"
        notes = []
        if multi_bin:
            conf = "low"
            notes.append("test references multiple binaries; using the first")
        if len(selectors) > 1 and not v.decomp_targets:
            conf = "low"
            notes.append("multiple functions[] selectors; Decompiler target not isolated")
        if base == "bin_priv_location":
            notes.append("binaries-private path (likely unavailable)")
        e = WorklistEntry(
            source_decompiler="angr",
            test_name=method.name,
            test_class=klass,
            test_file=tree_file,
            lineno=method.lineno,
            binary_rel=comps,
            binary_base=base,
            binary_path="",          # resolved by caller
            binary_exists=False,
            func_selector=sel,
            arch=v.proj_kwargs.get("arch"),
            load_debug=v.proj_kwargs.get("load_debug", False),
            custom_options=v.custom_options,
            confidence=conf,
            notes=notes,
        )
        entries.append(e)
    return entries


def _resolve_binary(entry: WorklistEntry):
    if entry.binary_base == "test_location":
        root = config.test_location()
    elif entry.binary_base == "bin_location":
        root = config.bin_root()
    else:  # bin_priv_location -- not on disk here; resolve under bin_root/../binaries-private
        root = config.bin_root().parent / "binaries-private"
    p = root.joinpath(*entry.binary_rel)
    entry.binary_path = str(p)
    entry.binary_exists = p.exists()


def build_worklist(test_file=None):
    """Parse the reference test file into a list of WorklistEntry."""
    test_file = str(test_file or config.test_file())
    with open(test_file) as fh:
        tree = ast.parse(fh.read(), filename=test_file)

    entries = []
    # top-level test_* functions and methods inside classes
    def handle_func(fn, klass):
        if fn.name.startswith("test"):
            entries.extend(_parse_method(fn, klass, test_file))

    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            handle_func(node, "")
        elif isinstance(node, ast.ClassDef):
            for sub in node.body:
                if isinstance(sub, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    handle_func(sub, node.name)

    for e in entries:
        _resolve_binary(e)
    return entries


def main(argv=None):
    p = argparse.ArgumentParser(prog="python -m kuna.pipeline.worklist",
                                description="Build the angr testcase (binary, function) worklist.")
    p.add_argument("--test-file", default=None, help="reference test file (default: angr test_decompiler.py)")
    p.add_argument("--out", default=None, help="output JSON (default: docs/pipeline/worklist.json)")
    p.add_argument("--only-existing", action="store_true", help="drop entries whose binary is not on disk")
    p.add_argument("--high-confidence", action="store_true", help="keep only single-target high-confidence entries")
    p.add_argument("--print", action="store_true", dest="do_print", help="print a human summary instead of writing")
    args = p.parse_args(argv)

    entries = build_worklist(args.test_file)
    if args.only_existing:
        entries = [e for e in entries if e.binary_exists]
    if args.high_confidence:
        entries = [e for e in entries if e.confidence == "high"]

    if args.do_print:
        existing = sum(1 for e in entries if e.binary_exists)
        print("worklist: %d entries (%d with binary on disk)" % (len(entries), existing))
        for e in entries[:60]:
            mark = "" if e.binary_exists else "  [MISSING]"
            extra = []
            if e.arch:
                extra.append("arch=%s" % e.arch)
            if e.load_debug:
                extra.append("load_debug")
            if e.custom_options:
                extra.append("custom-opts")
            if e.confidence != "high":
                extra.append(e.confidence)
            print("  %-44s %-22s %s%s" % (
                e.test_name[:44], e.func_selector,
                " ".join(extra), mark))
        if len(entries) > 60:
            print("  ... (%d more)" % (len(entries) - 60))
        return 0

    out = args.out or str(config.pipeline_docs_dir() / "worklist.json")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    payload = {
        "source_decompiler": "angr",
        "test_file": str(config.test_file() if args.test_file is None else args.test_file),
        "bin_root": str(config.bin_root()),
        "count": len(entries),
        "count_existing": sum(1 for e in entries if e.binary_exists),
        "entries": [asdict(e) for e in entries],
    }
    with open(out, "w") as fh:
        json.dump(payload, fh, indent=2)
    print("wrote %s (%d entries, %d with binary on disk)"
          % (out, payload["count"], payload["count_existing"]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
