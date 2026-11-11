"""Variable-type recovery: extraction, normalization and comparison.

Scored variables per method: the return type (methods only), every declared
parameter (by position) and every local declared in the body -- local variable
declarations, for / enhanced-for variables, catch parameters, try-with-resources
variables and pattern bindings -- excluding anything inside lambda bodies and
anonymous/local class bodies (those are other methods in bytecode).

Ground truth = the ORIGINAL SOURCE's declared type. Where the source declares
no type (``var``), ground truth comes from the class file's
LocalVariableTypeTable signature, else its LocalVariableTable descriptor, matched
by name and declaration order; with no table entry the variable is not scored.

Normalization (applied to both sides): annotations, ``final`` and whitespace
dropped, package/outer qualifiers dropped (``java.util.Map.Entry`` -> ``Entry``),
varargs ``T...`` -> ``T[]``, ``? extends Object`` -> ``?``, multi-catch
alternatives sorted. ``exact`` compares that form; ``erasure`` additionally
drops type arguments and replaces type variables by the erasure of their first
bound (from the declaring side's own type-parameter scope).
"""

from __future__ import annotations

import re
from dataclasses import dataclass

from .javasrc import MethodDecl, node_text

_PRIMS = {"B": "byte", "C": "char", "D": "double", "F": "float", "I": "int", "J": "long",
          "S": "short", "Z": "boolean", "V": "void"}


@dataclass
class Var:
    role: str  # return / param / local
    index: int  # param position, or local declaration order
    name: str
    type_text: str
    is_var: bool = False


def _strip_annotations(t: str) -> str:
    out = []
    i = 0
    n = len(t)
    while i < n:
        ch = t[i]
        if ch == "@":
            j = i + 1
            while j < n and (t[j].isalnum() or t[j] in "_$."):
                j += 1
            k = j
            while k < n and t[k].isspace():
                k += 1
            if k < n and t[k] == "(":
                depth = 0
                while k < n:
                    if t[k] == "(":
                        depth += 1
                    elif t[k] == ")":
                        depth -= 1
                        if depth == 0:
                            k += 1
                            break
                    k += 1
                j = k
            i = j
            continue
        out.append(ch)
        i += 1
    return "".join(out)


_QUAL = re.compile(r"(?:[A-Za-z_$][\w$]*\s*\.\s*)+([A-Za-z_$][\w$]*)")
_WS = re.compile(r"\s+")


def _drop_generic_outer(t: str) -> str:
    """``Outer<K, V>.Inner<X>`` -> ``Inner<X>`` (the same type, qualified)."""
    while True:
        i = t.find(">.")
        if i < 0:
            return t
        depth = 0
        j = i
        while j >= 0:
            if t[j] == ">":
                depth += 1
            elif t[j] == "<":
                depth -= 1
                if depth == 0:
                    break
            j -= 1
        k = j - 1
        while k >= 0 and (t[k].isalnum() or t[k] in "_$."):
            k -= 1
        t = t[: k + 1] + t[i + 2 :]


def normalize(t: str) -> str:
    # order matters: qualifiers are stripped while whitespace still separates
    # ``? extends java.lang.Number``; generic outers once it is gone
    t = _strip_annotations(t)
    t = re.sub(r"\bfinal\b", " ", t)
    t = t.replace("...", "[]")
    t = _QUAL.sub(r"\1", t)
    t = _WS.sub("", t)
    t = _drop_generic_outer(t)
    t = t.replace("?extendsObject", "?")
    if "|" in t and "<" not in t:
        t = "|".join(sorted(t.split("|")))
    return t


def _strip_generics(t: str) -> str:
    out = []
    depth = 0
    for ch in t:
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
        elif depth == 0:
            out.append(ch)
    return "".join(out)


def erase(t: str, tparams: dict[str, str], _depth: int = 0) -> str:
    """Erasure of a NORMALIZED type string."""
    if "|" in t:
        return "|".join(sorted(erase(x, tparams, _depth) for x in t.split("|")))
    base = _strip_generics(t)
    dims = ""
    while base.endswith("[]"):
        dims += "[]"
        base = base[:-2]
    if "&" in base:
        base = base.split("&")[0]
    if base in tparams and _depth < 8:
        bound = normalize(tparams[base])
        if bound.split("<")[0] == base:
            return "Object" + dims
        return erase(bound, tparams, _depth + 1) + dims
    return base + dims


def descriptor_param_types(desc: str) -> list[str]:
    """``(ILjava/util/List;[[B)V`` -> ['int', 'List', 'byte[][]'] (simple names)."""
    args = desc[1 : desc.index(")")]
    return _desc_list(args)


def descriptor_return_type(desc: str) -> str:
    return _desc_list(desc[desc.index(")") + 1 :])[0]


def _desc_list(s: str) -> list[str]:
    out = []
    i = 0
    while i < len(s):
        dims = 0
        while s[i] == "[":
            dims += 1
            i += 1
        if s[i] == "L":
            j = s.index(";", i)
            name = s[i + 1 : j].replace("/", ".").replace("$", ".").split(".")[-1]
            i = j + 1
        else:
            name = _PRIMS[s[i]]
            i += 1
        out.append(name + "[]" * dims)
    return out


def signature_to_java(sig: str) -> str:
    """Field/local generic signature (JVMS 4.7.9.1) -> normalized Java type text."""
    pos = [0]

    def ref() -> str:
        c = sig[pos[0]]
        if c == "[":
            pos[0] += 1
            return ref() + "[]"
        if c in _PRIMS:
            pos[0] += 1
            return _PRIMS[c]
        if c == "T":
            j = sig.index(";", pos[0])
            name = sig[pos[0] + 1 : j]
            pos[0] = j + 1
            return name
        if c == "L":
            pos[0] += 1
            parts = []
            cur = ""
            args = ""
            while True:
                ch = sig[pos[0]]
                if ch == "<":
                    pos[0] += 1
                    a = []
                    while sig[pos[0]] != ">":
                        a.append(targ())
                    pos[0] += 1
                    args = "<" + ",".join(a) + ">"
                elif ch == "." or ch == ";":
                    parts.append(cur.replace("/", ".").replace("$", ".").split(".")[-1] + args)
                    cur = ""
                    args = ""
                    pos[0] += 1
                    if ch == ";":
                        break
                else:
                    cur += ch
                    pos[0] += 1
            return parts[-1]
        raise ValueError(sig)

    def targ() -> str:
        c = sig[pos[0]]
        if c == "*":
            pos[0] += 1
            return "?"
        if c == "+":
            pos[0] += 1
            b = ref()
            return "?" if b == "Object" else "?extends" + b
        if c == "-":
            pos[0] += 1
            return "?super" + ref()
        return ref()

    return ref()


def descriptor_to_java(desc: str) -> str:
    return _desc_list(desc)[0]


# --- extraction --------------------------------------------------------------

_OPAQUE = {"lambda_expression", "class_body", "class_declaration", "interface_declaration",
           "enum_declaration", "record_declaration", "annotation_type_declaration"}


def method_vars(m: MethodDecl) -> list[Var]:
    out: list[Var] = []
    if m.kind == "method" and m.ret_type is not None:
        out.append(Var("return", 0, "<return>", m.ret_type))
    for i, p in enumerate(m.params):
        t = p.type_text + ("[]" if p.varargs else "") + "[]" * p.dims
        out.append(Var("param", i, p.name, t, t.strip() == "var"))
    if m.body is not None:
        locs: list[Var] = []
        _collect_locals(m.body, locs)
        for i, v in enumerate(locs):
            v.index = i
        out.extend(locs)
    return out


def _add(locs: list[Var], tnode, name_node, dims_node=None) -> None:
    if tnode is None or name_node is None:
        return
    t = node_text(tnode)
    if dims_node is not None:
        t += "[]" * node_text(dims_node).count("[")
    name = node_text(name_node)
    if name == "_":
        return
    locs.append(Var("local", 0, name, t, normalize(t) == "var"))


def _collect_locals(n, locs: list[Var]) -> None:
    stack = [n]
    while stack:
        x = stack.pop()
        ty = x.type
        if ty in _OPAQUE:
            continue
        if ty == "object_creation_expression":
            for c in reversed(x.named_children):
                if c.type != "class_body":
                    stack.append(c)
            continue
        if ty == "local_variable_declaration":
            tn = x.child_by_field_name("type")
            for d in x.named_children:
                if d.type == "variable_declarator":
                    _add(locs, tn, d.child_by_field_name("name"), d.child_by_field_name("dimensions"))
        elif ty == "enhanced_for_statement":
            _add(locs, x.child_by_field_name("type"), x.child_by_field_name("name"),
                 x.child_by_field_name("dimensions"))
        elif ty == "catch_formal_parameter":
            ct = next((c for c in x.named_children if c.type == "catch_type"), None)
            _add(locs, ct, x.child_by_field_name("name"), x.child_by_field_name("dimensions"))
        elif ty == "resource":
            tn = x.child_by_field_name("type")
            if tn is not None:
                _add(locs, tn, x.child_by_field_name("name"), x.child_by_field_name("dimensions"))
        elif ty == "instanceof_expression":
            nm = x.child_by_field_name("name")
            if nm is not None:
                _add(locs, x.child_by_field_name("right"), nm)
        elif ty == "type_pattern":
            kids = x.named_children
            if len(kids) >= 2 and kids[-1].type == "identifier":
                _add(locs, kids[0], kids[-1])
        # pre-order, textual order
        kids = x.named_children
        for c in reversed(kids):
            stack.append(c)


# --- comparison --------------------------------------------------------------


def pair_vars(src: list[Var], dec: list[Var]) -> list[tuple[Var, Var | None]]:
    """Type-blind correspondence: return<->return, params by position, locals by
    name (same-name declarations in order), then remaining locals by order."""
    pairs: list[tuple[Var, Var | None]] = []
    d_ret = [v for v in dec if v.role == "return"]
    d_par = [v for v in dec if v.role == "param"]
    d_loc = [v for v in dec if v.role == "local"]
    s_par = [v for v in src if v.role == "param"]
    off = 0
    if len(d_par) > len(s_par):
        # decompiler surfaced synthetic leading params (enum/inner ctors)
        off = len(d_par) - len(s_par)
    for v in src:
        if v.role == "return":
            pairs.append((v, d_ret[0] if d_ret else None))
    for i, v in enumerate(s_par):
        j = i + off
        pairs.append((v, d_par[j] if j < len(d_par) else None))
    s_loc = [v for v in src if v.role == "local"]
    used: set[int] = set()
    by_name: dict[str, list[int]] = {}
    for j, v in enumerate(d_loc):
        by_name.setdefault(v.name, []).append(j)
    residual_src = []
    matched: dict[int, int] = {}
    for i, v in enumerate(s_loc):
        cands = [j for j in by_name.get(v.name, []) if j not in used]
        if cands:
            used.add(cands[0])
            matched[i] = cands[0]
        else:
            residual_src.append(i)
    residual_dec = [j for j in range(len(d_loc)) if j not in used]
    for k, i in enumerate(residual_src):
        if k < len(residual_dec):
            matched[i] = residual_dec[k]
    for i, v in enumerate(s_loc):
        pairs.append((v, d_loc[matched[i]] if i in matched else None))
    return pairs


def compare_types(src_t: str, src_tp: dict[str, str], dec_t: str | None, dec_tp: dict[str, str]) -> tuple[bool, bool]:
    if dec_t is None:
        return False, False
    a = normalize(src_t)
    b = normalize(dec_t)
    if b == "var":
        return False, False
    exact = a == b
    er = exact or erase(a, src_tp) == erase(b, dec_tp)
    return exact, er
