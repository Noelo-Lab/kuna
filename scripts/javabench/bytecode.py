"""Per-method bytecode listings and decbench-style similarity.

Two listings per method, both constant-pool-layout independent:

* ``similarity listing``: one line per instruction; constant-pool operands are
  rendered symbolically (``invokevirtual java/lang/String.length:()I``), short
  forms are canonicalized (``iload_1`` -> ``iload 1``, ``goto_w`` -> ``goto``,
  ``wide`` folded), branch / switch targets are BLANKED (decbench blanks direct
  branch targets), compiler-numbered synthetic names are normalized
  (``lambda$foo$3`` -> ``lambda$foo$#``, ``access$000`` -> ``access$#``,
  anonymous ``Outer$7`` -> ``Outer$#``), and one ``catch <type>`` line per
  exception-table entry is appended.
* ``exact listing``: the same with branch targets kept as instruction indices
  and exception ranges kept as instruction-index ranges.

``similarity`` = shared / (a_only + shared + b_only) over a line diff
(diff_match_patch, exactly as decbench's byte_match); ``exact`` = the exact
listings are equal.
"""

from __future__ import annotations

import re

from .classfile import NEWARRAY_TYPES, ClassFile, Method, decode_code

_SYN = [
    (re.compile(r"lambda\$([\w$]*?)\$\d+\b"), r"lambda$\1$#"),
    (re.compile(r"access\$\d+"), "access$#"),
    (re.compile(r"\$\d+(?=[;:.$/<>\s\]),]|$)"), "$#"),
    (re.compile(r"\$\d+([A-Za-z_])"), r"$#\1"),
]


def _norm_names(s: str) -> str:
    for rx, rep in _SYN:
        s = rx.sub(rep, s)
    return s


_SHORT = re.compile(r"^([ilfda])(load|store)_(\d)$")


def listings(cf: ClassFile, m: Method) -> tuple[list[str], list[str]]:
    code = m.code
    if code is None:
        return [], []
    insns = decode_code(code.code)
    pc_to_idx = {ins.pc: i for i, ins in enumerate(insns)}
    end_idx = len(insns)
    cp = cf.cp
    sim: list[str] = []
    exact: list[str] = []

    def tgt(pc: int) -> str:
        return str(pc_to_idx.get(pc, -1))

    for ins in insns:
        op = ins.op
        mm = _SHORT.match(op)
        if mm:
            op, arg = f"{mm.group(1)}{mm.group(2)}", int(mm.group(3))
            line = f"{op} {arg}"
            sim.append(line)
            exact.append(line)
            continue
        if op == "goto_w":
            op = "goto"
        elif op == "jsr_w":
            op = "jsr"
        elif op == "ldc_w":
            op = "ldc"
        if ins.targets and op not in ("tableswitch", "lookupswitch"):
            sim.append(op)
            exact.append(f"{op} ->{tgt(ins.targets[0])}")
            continue
        if op == "tableswitch":
            low, high = ins.args
            sim.append(f"tableswitch {low}..{high}")
            exact.append(f"tableswitch {low}..{high} " + ",".join(tgt(t) for t in ins.targets))
            continue
        if op == "lookupswitch":
            keys = ",".join(str(k) for k in ins.args)
            sim.append(f"lookupswitch [{keys}]")
            exact.append(f"lookupswitch [{keys}] " + ",".join(tgt(t) for t in ins.targets))
            continue
        if op in ("ldc", "ldc2_w"):
            line = f"{op} {_norm_names(cp.describe(ins.args[0]))}"
        elif op in ("getstatic", "putstatic", "getfield", "putfield", "invokevirtual",
                    "invokespecial", "invokestatic", "invokeinterface", "new", "anewarray",
                    "checkcast", "instanceof"):
            line = f"{op} {_norm_names(cp.describe(ins.args[0]))}"
        elif op == "multianewarray":
            line = f"{op} {_norm_names(cp.describe(ins.args[0]))} {ins.args[1]}"
        elif op == "invokedynamic":
            line = f"invokedynamic {_norm_names(_indy(cf, ins.args[0]))}"
        elif op == "newarray":
            line = f"newarray {NEWARRAY_TYPES.get(ins.args[0], ins.args[0])}"
        elif ins.args:
            line = op + " " + " ".join(str(a) for a in ins.args)
        else:
            line = op
        sim.append(line)
        exact.append(line)
    for e in code.exceptions:
        ct = _norm_names(e.catch_type) if e.catch_type else "any"
        sim.append(f"catch {ct}")
        s = pc_to_idx.get(e.start_pc, -1)
        en = pc_to_idx.get(e.end_pc, end_idx)
        h = pc_to_idx.get(e.handler_pc, -1)
        exact.append(f"catch {ct} [{s},{en})->{h}")
    return sim, exact


def _indy(cf: ClassFile, idx: int) -> str:
    cp = cf.cp
    e = cp.entries[idx]
    bsm_i, nt = e[1], e[2]
    name, desc = cp.name_and_type(nt)
    parts = [f"{name}:{desc}"]
    if bsm_i < len(cf.bootstrap):
        ref, args = cf.bootstrap[bsm_i]
        parts.append("bsm=" + cp.describe(ref))
        parts.append("args=[" + ";".join(cp.describe(a) for a in args) + "]")
    return " ".join(parts)


def similarity(a: list[str], b: list[str]) -> tuple[float, int]:
    """decbench byte_match: shared/(a_only+shared+b_only) over a line diff."""
    if not a and not b:
        return 1.0, 0
    if not a or not b:
        return 0.0, len(a) + len(b)
    try:
        from diff_match_patch import diff_match_patch
    except ImportError:  # pragma: no cover
        import difflib

        sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
        shared = sum(bl.size for bl in sm.get_matching_blocks())
        total = len(a) + len(b) - shared
        return shared / total, len(a) + len(b) - 2 * shared
    dmp = diff_match_patch()
    dmp.Diff_Timeout = 5.0
    ta = "\n".join(a) + "\n"
    tb = "\n".join(b) + "\n"
    ca, cb, arr = dmp.diff_linesToChars(ta, tb)
    diffs = dmp.diff_main(ca, cb, False)
    a_only = shared = b_only = 0
    for op, text in diffs:
        n = len(text)  # one char per line after linesToChars
        if op == -1:
            a_only += n
        elif op == 0:
            shared += n
        else:
            b_only += n
    total = a_only + shared + b_only
    return (shared / total if total else 1.0), a_only + b_only


# --- matching recompiled classes to original classes ---------------------------


def _anon_key(cf: ClassFile) -> tuple:
    outer = cf.enclosing_method[0] if cf.enclosing_method else (cf.name.rsplit("$", 1)[0])
    sup = cf.super_name if cf.super_name != "java/lang/Object" or not cf.interfaces else cf.interfaces[0]
    ic = cf.own_inner_info()
    simple = ic.simple_name if ic else None
    return outer, sup, simple


def _index_suffix(name: str) -> int:
    m = re.search(r"\$(\d+)([A-Za-z_$][\w$]*)?$", name)
    return int(m.group(1)) if m else 0


def match_classes(original: dict[str, ClassFile], recompiled: dict[str, ClassFile]) -> dict[str, str]:
    """Map recompiled internal class names -> original internal names.

    Named classes match by name. Anonymous / local classes are grouped by
    (outer class, super type, local simple name) and paired in javac index order,
    so a decompiler that re-orders anonymous classes is not penalized for the
    renumbering itself.
    """
    out: dict[str, str] = {}
    o_groups: dict[tuple, list[str]] = {}
    r_groups: dict[tuple, list[str]] = {}
    for name, cf in recompiled.items():
        if not (cf.is_anonymous or cf.is_local):
            if name in original:
                out[name] = name
            continue
        r_groups.setdefault(_anon_key(cf), []).append(name)
    for name, cf in original.items():
        if cf.is_anonymous or cf.is_local:
            o_groups.setdefault(_anon_key(cf), []).append(name)
    # outer names inside keys refer to recompiled names: translate via `out`
    for key, rnames in r_groups.items():
        okey = (out.get(key[0], key[0]),) + key[1:]
        onames = sorted(o_groups.get(okey, []), key=_index_suffix)
        for r, o in zip(sorted(rnames, key=_index_suffix), onames):
            out[r] = o
    return out
