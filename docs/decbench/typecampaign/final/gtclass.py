"""GT-class-aware copies of decbench's DWARF extraction and decision recorder.

`extract_ground_truth_types` mirrors decbench.metrics.type_match's traversal
exactly (same DIE walk, same filters, same dict-overwrite on duplicate names)
and only ADDS a "cls" key per variable; the matcher never reads it, so scores
are unchanged.  The recorder is typesweep's verbatim `_match_structured` copy
plus the class, and `_match_by_regex` is recorded too.
"""
from __future__ import annotations

import contextlib
from typing import Any

PTR_TAGS = ("DW_TAG_pointer_type", "DW_TAG_reference_type", "DW_TAG_rvalue_reference_type")
SKIP_TAGS = ("DW_TAG_typedef", "DW_TAG_const_type", "DW_TAG_volatile_type",
             "DW_TAG_restrict_type", "DW_TAG_atomic_type")


def _type_of(die, dwarfinfo):
    if "DW_AT_type" not in die.attributes:
        return None
    attr = die.attributes["DW_AT_type"]
    return dwarfinfo.get_DIE_from_refaddr(attr.value + die.cu.cu_offset)


def _strip(t, dwarfinfo):
    hops = 0
    while t is not None and t.tag in SKIP_TAGS and hops < 32:
        t = _type_of(t, dwarfinfo)
        hops += 1
    return t


def _enc(t):
    a = t.attributes.get("DW_AT_encoding")
    return a.value if a else None


def _size(t):
    a = t.attributes.get("DW_AT_byte_size")
    return a.value if a else 0


def _is_char(t):
    return t is not None and t.tag == "DW_TAG_base_type" and _enc(t) in (0x06, 0x08)


def _base_cls(t):
    e, s = _enc(t), _size(t)
    if e == 0x02:
        return "bool"
    if e in (0x04, 0x03):
        return "float"
    if e in (0x06, 0x08):
        return "char"
    if e in (0x05, 0x0d):
        return {1: "char", 2: "int_s2", 4: "int_s4", 8: "int_s8"}.get(s, "int_other")
    if e in (0x07, 0x0e, 0x10):
        return {1: "char", 2: "int_u2", 4: "int_u4", 8: "int_u8"}.get(s, "int_other")
    return "int_other"


def classify(var_die, dwarfinfo) -> str:
    raw = _type_of(var_die, dwarfinfo)
    t = _strip(raw, dwarfinfo)
    if t is None:
        return "void"
    tag = t.tag
    if tag == "DW_TAG_base_type":
        return _base_cls(t)
    if tag == "DW_TAG_enumeration_type":
        return "enum"
    if tag in PTR_TAGS:
        p = _strip(_type_of(t, dwarfinfo), dwarfinfo)
        if p is None:
            return "ptr_void"
        pt = p.tag
        if pt in PTR_TAGS:
            return "ptr_ptr"
        if pt == "DW_TAG_base_type":
            return "ptr_char" if _is_char(p) else "ptr_prim"
        if pt in ("DW_TAG_structure_type", "DW_TAG_class_type"):
            return "ptr_struct"
        if pt == "DW_TAG_union_type":
            return "ptr_union"
        if pt == "DW_TAG_enumeration_type":
            return "ptr_enum"
        if pt == "DW_TAG_subroutine_type":
            return "funcptr"
        return "ptr_other"
    if tag == "DW_TAG_array_type":
        e = _strip(_type_of(t, dwarfinfo), dwarfinfo)
        return "array_char" if _is_char(e) else "array"
    if tag in ("DW_TAG_structure_type", "DW_TAG_class_type"):
        return "struct_val"
    if tag == "DW_TAG_union_type":
        return "union_val"
    return "other"


def _parse_variable_die(die, dwarfinfo, is_arg=False, arg_index=None):
    from decbench.metrics.type_match import _get_location, _parse_type_die, normalize_type
    offsets, has_location = _get_location(die, dwarfinfo)
    if not has_location:
        return None
    if "DW_AT_abstract_origin" in die.attributes:
        try:
            attr = die.attributes["DW_AT_abstract_origin"]
            die = dwarfinfo.get_DIE_from_refaddr(attr.value + die.cu.cu_offset)
        except Exception:
            pass
    name = ""
    if "DW_AT_name" in die.attributes:
        name = die.attributes["DW_AT_name"].value.decode("utf-8", "replace")
    type_names: list[str] = []
    size = 0
    with contextlib.suppress(Exception):
        type_names, size = _parse_type_die(die, dwarfinfo)
    if not type_names:
        return None
    all_forms: set[str] = set()
    for t in type_names:
        all_forms.update(normalize_type(t))
    cls = "other"
    with contextlib.suppress(Exception):
        cls = classify(die, dwarfinfo)
    return {
        "name": name,
        "type": sorted(all_forms),
        "rbp_offset": sorted(set(offsets)),
        "size": size,
        "is_arg": is_arg,
        "arg_index": arg_index if is_arg else None,
        "cls": cls,
    }


def _parse_lexical_block(die, dwarfinfo):
    out = []
    for child in die.iter_children():
        if child.tag == "DW_TAG_lexical_block":
            out.extend(_parse_lexical_block(child, dwarfinfo))
        elif child.tag in ("DW_TAG_formal_parameter", "DW_TAG_variable"):
            v = _parse_variable_die(child, dwarfinfo)
            if v:
                out.append(v)
    return out


def _parse_function_die(die, dwarfinfo):
    from decbench.utils import binfmt
    func_name = binfmt.die_str_attr(die, "DW_AT_name")
    if func_name is None:
        return None, []
    variables = []
    arg_index = 0
    for child in die.iter_children():
        if child.tag in ("DW_TAG_lexical_block", "DW_TAG_inlined_subroutine"):
            variables.extend(_parse_lexical_block(child, dwarfinfo))
        elif child.tag == "DW_TAG_formal_parameter":
            v = _parse_variable_die(child, dwarfinfo, is_arg=True, arg_index=arg_index)
            arg_index += 1
            if v:
                variables.append(v)
        elif child.tag == "DW_TAG_variable":
            v = _parse_variable_die(child, dwarfinfo)
            if v:
                variables.append(v)
    return func_name, variables


def extract_ground_truth_types(binary_path) -> dict[str, list[dict[str, Any]]]:
    from decbench.utils import binfmt
    result: dict[str, list[dict[str, Any]]] = {}
    try:
        dwarfinfo = binfmt.dwarf_info(binary_path)
        if dwarfinfo is None:
            return result
        for CU in dwarfinfo.iter_CUs():
            top = CU.get_top_DIE()
            for DIE in top.iter_children():
                if DIE.tag != "DW_TAG_subprogram":
                    continue
                fn, vs = _parse_function_die(DIE, dwarfinfo)
                if fn and vs:
                    result[fn] = vs
    except Exception:
        pass
    return result


def install(sink: list) -> None:
    """Patch the metric: GT extraction with classes + recording matchers."""
    import decbench.metrics.type_match as tm
    from decbench.metrics.type_match import (
        TypeMatchMetric, normalize_type, _uncommitted_size, _effective_offset,
        _SIZE_SCALARS, _calibrate_shift, extract_types_from_decompiled_code,
    )
    tm.extract_ground_truth_types = extract_ground_truth_types

    def rec(fn, gv, ok, v, pas, n):
        sink.append({
            "fn": fn, "ok": ok, "pass": pas, "cls": gv.get("cls"),
            "kuna_type": (v.type if v is not None else None),
            "kuna_size": (v.size if v is not None else None),
            "gt_type": (gv.get("type") or [None])[0], "gt_size": gv.get("size"),
            "gt_name": gv.get("name"), "gt_is_arg": bool(gv.get("is_arg")),
            "gt_has_off": bool(gv.get("rbp_offset")), "gt_vars": n,
        })

    def _match_structured(self, decompiled, ground_truth_vars, gt_stack_vars,
                          decomp_stack_vars, calibration_shift=None):
        gt_offsets = []
        for gv in ground_truth_vars:
            gt_offsets.extend(gv.get("rbp_offset", []))
        var_offsets = [_effective_offset(v) for v in decompiled.variables]
        decomp_offsets = [o for o in var_offsets if o is not None]
        gt_off_set = set(gt_offsets)

        def _aligned(kk):
            if kk is None or not decomp_offsets:
                return 0
            return len({d + kk for d in decomp_offsets} & gt_off_set)

        shift = calibration_shift if calibration_shift is not None else 0
        fs = _calibrate_shift(gt_offsets, decomp_offsets)
        if fs is not None and _aligned(shift) == 0 and _aligned(fs) > 0:
            shift = fs
        k = shift if shift is not None else 0
        var_types = [normalize_type(v.type) for v in decompiled.variables]
        var_unc = [_uncommitted_size(v) for v in decompiled.variables]
        by_arg_index, by_off, by_name = {}, {}, {}
        for i, v in enumerate(decompiled.variables):
            if v.arg_index is not None and v.arg_index not in by_arg_index:
                by_arg_index[v.arg_index] = i
            if var_offsets[i] is not None:
                by_off.setdefault(var_offsets[i] + k, []).append(i)
            if v.name:
                by_name.setdefault(v.name, []).append(i)

        def _matches(gt_forms, i):
            if gt_forms & var_types[i]:
                return True
            sz = var_unc[i]
            return sz is not None and bool(_SIZE_SCALARS.get(sz, set()) & gt_forms)

        used = set()

        def claim(cands, gt_types):
            avail = [i for i in cands if i not in used]
            if not avail:
                return None, None
            hit = next((i for i in avail if _matches(gt_types, i)), None)
            if hit is not None:
                used.add(hit)
                return True, hit
            used.add(avail[0])
            return False, avail[0]

        n = len(ground_truth_vars)
        verdicts = [None] * n
        decided = [False] * n
        V = decompiled.variables
        for gi, gv in enumerate(ground_truth_vars):
            ai = gv.get("arg_index")
            if not gv.get("is_arg") or ai is None:
                continue
            di = by_arg_index.get(ai)
            if di is None or di in used:
                continue
            used.add(di)
            decided[gi] = True
            verdicts[gi] = _matches(set(gv.get("type", [])), di)
            rec(decompiled.name, gv, verdicts[gi], V[di], "arg", n)
        for gi, gv in enumerate(ground_truth_vars):
            if decided[gi]:
                continue
            c = []
            for off in gv.get("rbp_offset", []):
                c.extend(by_off.get(off, []))
            if not c:
                continue
            v, di = claim(c, set(gv.get("type", [])))
            if v is not None:
                decided[gi] = True
                verdicts[gi] = v
                rec(decompiled.name, gv, v, V[di], "offset", n)
        for gi, gv in enumerate(ground_truth_vars):
            if decided[gi]:
                continue
            gn = gv.get("name", "")
            if not gn:
                continue
            v, di = claim(by_name.get(gn, []), set(gv.get("type", [])))
            if v is not None:
                decided[gi] = True
                verdicts[gi] = v
                rec(decompiled.name, gv, v, V[di], "name", n)
        for gi, gv in enumerate(ground_truth_vars):
            if not decided[gi]:
                rec(decompiled.name, gv, None, None, "miss", n)
        tp = sum(1 for d, v in zip(decided, verdicts) if d and v)
        fp = sum(1 for d, v in zip(decided, verdicts) if d and not v)
        fn = sum(1 for d in decided if not d)
        return self._build_result(tp, fp, fn, ground_truth_vars, len(decompiled.variables),
                                  "structured", calibration_shift, gt_stack_vars,
                                  decomp_stack_vars)

    def _match_by_regex(self, decompiled, ground_truth_vars, gt_stack_vars, decomp_stack_vars):
        decomp_vars = extract_types_from_decompiled_code(decompiled.decompiled_code)
        n = len(ground_truth_vars)
        if not decomp_vars:
            for gv in ground_truth_vars:
                rec(decompiled.name, gv, None, None, "rx-miss", n)
            return self._build_result(0, 0, n, ground_truth_vars, 0, "regex", None,
                                      gt_stack_vars, decomp_stack_vars)
        type_by_name: dict[str, set[str]] = {}
        for var in decomp_vars:
            type_by_name.setdefault(var["name"], set()).update(var["type"])
        tp = fp = fn = 0
        for gv in ground_truth_vars:
            gn = gv.get("name", "")
            gts = set(gv.get("type", []))
            if gn in type_by_name:
                ok = bool(gts.intersection(type_by_name[gn]))
                tp += ok
                fp += not ok
                rec(decompiled.name, gv, ok, None, "rx", n)
            else:
                fn += 1
                rec(decompiled.name, gv, None, None, "rx-miss", n)
        return self._build_result(tp, fp, fn, ground_truth_vars, len(decomp_vars), "regex",
                                  None, gt_stack_vars, decomp_stack_vars)

    TypeMatchMetric._match_structured = _match_structured
    TypeMatchMetric._match_by_regex = _match_by_regex
