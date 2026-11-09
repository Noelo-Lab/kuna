"""Method correspondence: class-file methods <-> source / decompiled declarations.

Keys are ``<binary class name>#<name><descriptor>``, e.g.
``com.google.common.base.Strings#padStart(Ljava/lang/String;IC)Ljava/lang/String;``.

Types. Named types match by binary name. Anonymous and local classes:

* source side -- by LINE NUMBERS: the class file's methods' LineNumberTable
  lines must fall inside the candidate declaration, whose enclosing type must
  already map to the class's outer class; the innermost candidate wins
  (supertype and javac index order break ties);
* decompiled side (no line info) -- the anonymous/local declarations directly
  inside each mapped type are aligned in textual order with the original
  ``Outer$N`` classes in index order, allowing only pairs whose supertype (or
  local-class name) agrees (an LCS alignment, so one missing or extra anonymous
  class does not shift the rest).

Methods. Same name, then an erased parameter-type check against the
descriptor (simple names; type variables erased to their bound), allowing the
synthetic leading parameters javac adds (enum ``(String,int)``, inner-class
outer instance) and trailing captured values for local/anonymous classes;
line containment breaks remaining ties on the source side; a unique same-name,
same-arity declaration is the last resort.
"""

from __future__ import annotations

import re

from .classfile import ACC_ENUM, ACC_STATIC, ClassFile, Method
from .javasrc import MethodDecl, SourceIndex, TypeDecl
from .jtypes import descriptor_param_types, erase, normalize


def method_key(cf: ClassFile, m: Method) -> str:
    return f"{cf.dotted}#{m.name}{m.descriptor}"


def outer_of(cf: ClassFile) -> str | None:
    if cf.enclosing_method:
        return cf.enclosing_method[0]
    ic = cf.own_inner_info()
    if ic is not None and ic.outer:
        return ic.outer
    if "$" in cf.name:
        return cf.name.rsplit("$", 1)[0]
    return None


def _simple(name: str | None) -> str | None:
    if not name:
        return None
    return name.replace("/", ".").replace("$", ".").split(".")[-1]


def supertype_simple(cf: ClassFile) -> str | None:
    if cf.super_name and cf.super_name != "java/lang/Object":
        return _simple(cf.super_name)
    if cf.interfaces:
        return _simple(cf.interfaces[0])
    return "Object"


def anon_index(name: str) -> int:
    m = re.search(r"\$(\d+)([A-Za-z_$][\w$]*)?$", name)
    return int(m.group(1)) if m else 0


def _depth(name: str) -> int:
    return name.count("$")


def is_scored_method(m: Method) -> tuple[bool, str]:
    if m.code is None:
        return False, "no-code"
    if m.is_bridge:
        return False, "bridge"
    if m.is_synthetic:
        return False, "synthetic"
    if m.name == "<clinit>":
        return False, "static-initializer"
    return True, ""


# --- type mapping -------------------------------------------------------------


def _type_lines(cf: ClassFile) -> list[int]:
    lines: set[int] = set()
    for m in cf.methods:
        if m.code is not None and m.name not in ("<init>", "<clinit>") and not m.is_synthetic:
            lines.update(m.code.line_numbers())
    if not lines:
        for m in cf.methods:
            if m.code is not None and m.name == "<init>":
                lines.update(m.code.line_numbers())
    return sorted(lines)


def map_types(classes: dict[str, ClassFile], indexes: list[SourceIndex], use_lines: bool) -> dict[str, TypeDecl]:
    """internal class name -> TypeDecl, for the classes given (one source unit)."""
    by_binary: dict[str, TypeDecl] = {}
    all_types: list[TypeDecl] = []
    for idx in indexes:
        for t in idx.types:
            all_types.append(t)
            if t.kind not in ("anon", "local") and t.binary:
                by_binary.setdefault(t.binary.replace(".", "/"), t)
    out: dict[str, TypeDecl] = {}
    rev: dict[int, str] = {}
    pending: list[ClassFile] = []
    for name in sorted(classes, key=_depth):
        cf = classes[name]
        if cf.is_anonymous or cf.is_local:
            pending.append(cf)
            continue
        t = by_binary.get(name)
        if t is not None:
            out[name] = t
            rev[id(t)] = name
    if not pending:
        return out
    if use_lines:
        for cf in sorted(pending, key=lambda c: (_depth(c.name), anon_index(c.name))):
            outer = outer_of(cf)
            lines = _type_lines(cf)
            local_name = cf.own_inner_info().simple_name if cf.is_local else None
            cands = []
            for t in all_types:
                if id(t) in rev:
                    continue
                if cf.is_anonymous and t.kind != "anon":
                    continue
                if cf.is_local and (t.kind != "local" or t.name != local_name):
                    continue
                if t.parent is None or rev.get(id(t.parent)) != outer:
                    continue
                if lines and not (t.start_line <= lines[0] and t.end_line >= lines[-1]):
                    continue
                cands.append(t)
            if not cands:
                continue
            if lines:
                span = min(t.end_line - t.start_line for t in cands)
                cands = [t for t in cands if t.end_line - t.start_line == span]
            if len(cands) > 1:
                sup = supertype_simple(cf)
                c2 = [t for t in cands if t.supertype == sup]
                cands = c2 or cands
            if len(cands) > 1 and not lines:
                continue
            cands.sort(key=lambda t: (t.anon_index, t.local_index, t.start_line))
            t = cands[0]
            out[cf.name] = t
            rev[id(t)] = cf.name
        return out
    # decompiled side: ordered alignment per outer type
    children: dict[int, list[TypeDecl]] = {}
    for t in all_types:
        if t.kind in ("anon", "local") and t.parent is not None:
            children.setdefault(id(t.parent), []).append(t)
    progress = True
    while progress and pending:
        progress = False
        groups: dict[str, list[ClassFile]] = {}
        for cf in pending:
            groups.setdefault(outer_of(cf) or "", []).append(cf)
        still: list[ClassFile] = []
        for outer, cfs in groups.items():
            ot = out.get(outer)
            if ot is None:
                still.extend(cfs)
                continue
            kids = sorted(children.get(id(ot), []), key=lambda t: (t.start_line, t.anon_index, t.local_index))
            # javac numbers anonymous classes in attribution order, which is textual
            # except for deferred (diamond / lambda-argument) bodies, so align within
            # each supertype group where the relative order is stable
            aligned = []
            sups = {supertype_simple(c) for c in cfs if c.is_anonymous}
            for sup in sorted(sups, key=str):
                ks = [t for t in kids if t.kind == "anon" and (t.supertype or "Object") == sup]
                cs = sorted([c for c in cfs if c.is_anonymous and supertype_simple(c) == sup],
                            key=lambda c: anon_index(c.name))
                aligned += _align(ks, cs, "anon")
            local_names = {c.own_inner_info().simple_name for c in cfs if c.is_local}
            for ln in sorted(local_names):
                ks = [t for t in kids if t.kind == "local" and t.name == ln]
                cs = sorted([c for c in cfs if c.is_local and c.own_inner_info().simple_name == ln],
                            key=lambda c: anon_index(c.name))
                aligned += _align(ks, cs, "local")
            for t, cf in aligned:
                out[cf.name] = t
                rev[id(t)] = cf.name
                progress = True
            still.extend(c for c in cfs if c.name not in out)
        pending = [c for c in still if c.name not in out and outer_of(c) not in out] if progress else []
    return out


def _align(ts: list[TypeDecl], cs: list[ClassFile], kind: str) -> list[tuple[TypeDecl, ClassFile]]:
    def ok(t: TypeDecl, c: ClassFile) -> bool:
        if kind == "local":
            return t.name == c.own_inner_info().simple_name
        sup = supertype_simple(c)
        return t.supertype is None or sup is None or t.supertype == sup or (
            t.supertype == "Object" and sup == "Object")

    n, m = len(ts), len(cs)
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(n - 1, -1, -1):
        for j in range(m - 1, -1, -1):
            best = max(dp[i + 1][j], dp[i][j + 1])
            if ok(ts[i], cs[j]):
                best = max(best, 1 + dp[i + 1][j + 1])
            dp[i][j] = best
    out = []
    i = j = 0
    while i < n and j < m:
        if ok(ts[i], cs[j]) and dp[i][j] == 1 + dp[i + 1][j + 1]:
            out.append((ts[i], cs[j]))
            i += 1
            j += 1
        elif dp[i + 1][j] >= dp[i][j + 1]:
            i += 1
        else:
            j += 1
    return out


# --- method mapping -------------------------------------------------------------


def erased_param_names(md: MethodDecl) -> list[str]:
    tp = md.scope_type_params()
    out = []
    for p in md.params:
        t = normalize(p.type_text + ("[]" if p.varargs else "") + "[]" * p.dims)
        out.append(erase(t, tp))
    return out


def _offsets(cf: ClassFile, m: Method) -> tuple[list[int], bool]:
    if m.name != "<init>":
        return [0], False
    ic = cf.own_inner_info()
    if cf.access & ACC_ENUM and cf.super_name == "java/lang/Enum":
        return [2, 0], False
    if cf.is_anonymous or cf.is_local:
        return [0, 1, 2], True
    if ic is not None and ic.outer and not (ic.access & ACC_STATIC):
        return [1, 0], False
    return [0], False


def compatible(md: MethodDecl, cf: ClassFile, m: Method) -> bool:
    want = erased_param_names(md)
    have = descriptor_param_types(m.descriptor)
    offs, trailing = _offsets(cf, m)
    for off in offs:
        seg = have[off : off + len(want)]
        if len(seg) != len(want):
            continue
        if not trailing and off + len(want) != len(have):
            continue
        if seg == want:
            return True
    return False


def _arity_ok(md: MethodDecl, cf: ClassFile, m: Method) -> bool:
    """Loose fallback: same arity, and primitive-vs-reference agreement at every
    position (only reference-type spellings may be unresolvable)."""
    have = descriptor_param_types(m.descriptor)
    want = erased_param_names(md)
    offs, trailing = _offsets(cf, m)
    for off in offs:
        n = len(have) - off
        if not (n == len(want) or (trailing and n >= len(want))):
            continue
        seg = have[off : off + len(want)]
        if all((a in _PRIM) == (b in _PRIM) and (a not in _PRIM or a == b) for a, b in zip(seg, want)):
            return True
    return False


_PRIM = {"int", "long", "short", "byte", "char", "boolean", "float", "double"}


def _desc_fqns(desc: str) -> list[str]:
    args = desc[1 : desc.index(")")]
    out = []
    i = 0
    while i < len(args):
        dims = 0
        while args[i] == "[":
            dims += 1
            i += 1
        if args[i] == "L":
            j = args.index(";", i)
            out.append(args[i + 1 : j].replace("/", ".").replace("$", ".") + "[]" * dims)
            i = j + 1
        else:
            out.append(args[i] + "[]" * dims)
            i += 1
    return out


def _qualified(md: MethodDecl, imports: dict[str, str]) -> list[tuple[str, str] | None]:
    """Per parameter, what the written type pins down about the descriptor class:
    ("fqn", name) when resolved through an import or written fully qualified,
    ("suffix", Outer.Inner) when written with an outer-class qualifier, else None."""
    from .jtypes import _strip_annotations, _strip_generics

    tp = md.scope_type_params()
    out: list[tuple[str, str] | None] = []
    for p in md.params:
        t = _strip_generics(_strip_annotations(p.type_text)).replace(" ", "").replace("\n", "")
        dims = "[]" * (t.count("[]") + (1 if p.varargs else 0) + p.dims)
        t = t.replace("[]", "")
        if t in _PRIM or t in tp:
            out.append(None)
            continue
        head, _, rest = t.partition(".")
        if head in imports:
            out.append(("fqn", imports[head] + ("." + rest if rest else "") + dims))
        elif rest and head[:1].islower():
            out.append(("fqn", t + dims))
        elif rest:
            out.append(("suffix", t + dims))
        else:
            out.append(None)
    return out


def _by_imports(cands: list[MethodDecl], m: Method, imports: dict[str, str] | None) -> list[MethodDecl]:
    have = _desc_fqns(m.descriptor)
    keep = []
    for md in cands:
        q = _qualified(md, imports or {})
        tail = have[len(have) - len(q):] if len(q) <= len(have) else []
        ok = len(tail) == len(q)
        for x, y in zip(q, tail):
            if x is None:
                continue
            kind, v = x
            if kind == "fqn" and v != y:
                ok = False
            elif kind == "suffix" and not (y == v or y.endswith("." + v)):
                ok = False
        if ok:
            keep.append(md)
    return keep or cands


def import_map(idx: SourceIndex) -> dict[str, str]:
    out = {}
    for imp in idx.imports:
        if imp.startswith("static") or imp.endswith(".*"):
            continue
        out[imp.rsplit(".", 1)[-1]] = imp
    return out


def map_method(t: TypeDecl, cf: ClassFile, m: Method, use_lines: bool,
               imports: dict[str, str] | None = None) -> tuple[MethodDecl | None, str]:
    cands = [md for md in t.methods if md.name == m.name]
    if not cands:
        return None, "no-source-decl"
    good = [md for md in cands if compatible(md, cf, m)]
    if len(good) > 1:
        good = _by_imports(good, m, imports)
    if len(good) > 1 and use_lines and m.code is not None and m.code.lines:
        lines = m.code.line_numbers()
        inside = [md for md in good if md.start_line <= lines[0] and md.end_line >= lines[-1]]
        if not inside:
            inside = [md for md in good if any(md.start_line <= ln <= md.end_line for ln in lines)]
        good = inside or good
    if len(good) == 1:
        return good[0], ""
    if len(good) > 1:
        return None, "ambiguous-decl"
    loose = [md for md in cands if _arity_ok(md, cf, m)]
    if len(loose) > 1 and use_lines and m.code is not None and m.code.lines:
        lines = m.code.line_numbers()
        loose = [md for md in loose if md.start_line <= lines[0] and md.end_line >= lines[-1]] or loose
    if len(loose) == 1:
        return loose[0], ""
    return None, "no-compatible-decl" if not loose else "ambiguous-decl"
