"""Score kuna's struct recovery — the instrument `type_match` cannot be.

`type_match` is blind to this lane by construction: a synthesized `struct_0 *`
never matches a DWARF `Word *`, so a pass that recovers a perfect layout scores
exactly what a pass that recovers nothing scores. Three measurements, all
read-only, all against the unstripped twin's DWARF:

**--layout** (RecStruct §4.1 / Tasks 1-2, function PARAMETERS only, which is
also the only place `structsynth param` can act). Ground truth is the *binary*
layout: typedefs resolved, anonymous struct/union members expanded in place,
gaps made explicit as padding (padding is reported but never scored -- it is not
a recovered field). A field matches on `(offset, size)`; layout-F1 is the
field-level P/R/F1 and nesting-F1 scores `(offset -> pointee struct)` pairs, a
pair counting as recovered when kuna both puts a pointer at the right offset and
recovers at least one field of what it points to. The predicted side is kuna's
own: `decompile-all --json` for each parameter's type, and the struct bodies
from a `decompile-project` header. **kuna synthesizes no structs today, so the
score is 0 by construction** — the number that matters right now is the
denominator it is 0 out of.

**--trex** (TRex §5.2 Figure 6). The prioritized score
`Defined? -> IsCPointer? -> PointerLevel? -> IsCStruct? -> SignIgnoredPrimitive?
-> CPrimitive?` with `f = p1*(1+p2*(1+p3*(1+p4*(1+p5*(1+p6)))))`, so an expected
`uint64_t` scores `char*` 1, `struct{…}` 3, `double` 4, `int64_t` 5, `uint64_t`
6. Scored over kuna's `--json variables[]` (the surface decbench scores) against
`decbench.metrics.type_match.extract_ground_truth_types`, paired by the same
three passes the metric uses (arg index, calibrated stack offset, name). Two
judgement calls, both flagged: an *uncommitted* spelling (`undefined8`,
`xunknown8`, `_QWORD`) counts as **defined** but of unknown primitive class, so
it can pass the pointer/struct steps and never the primitive ones
(`--strict-defined` scores it 0 instead); and a step neither side can be judged
on -- the primitive steps when both sides are structs -- passes, which is
exactly the blindness RecStruct §5 charges the metric with. Report the two
together and neither can hide the other.

**--census** (the opportunity ceiling). Over the same run's C text, a *base* is
a parameter or declared local dereferenced at two or more distinct constant
offsets (`*(T *)(B + K)`, `B[k]`, `*B`, `B->f`) — a struct candidate. Then,
using the ground truth, how many of those candidates' types kuna **already**
gets right: that is the match->miss channel a synthesis pass would spend, and it
is what makes struct synthesis a *quality* feature rather than a metric one.

Usage (needs the decbench venv python)::

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.structscore \\
        <stripped-binary>... --all                      # all three
    ... --trex --census                                 # pick the modes
    ... --unstripped <twin>                             # non-decbench layout
    ... --option structsynth param                      # measure a flip
    ... --out /tmp/structscore.json                     # the only write
    python3 -m scripts.decbench.structscore --selftest   # no decbench needed

The unstripped twin defaults to the decbench convention (``/stripped/`` ->
``/compiled/``). Nothing is written anywhere unless ``--out`` is given; the
``decompile-project`` export goes to a temporary directory that is removed.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from . import config

POINTER_SIZE = 8

# --------------------------------------------------------------------------
# type vocabulary
# --------------------------------------------------------------------------

# name -> (class, byte size). `class` is what TRex's two primitive steps
# compare; signedness is carried separately so "sign-ignored" is a real step.
PRIMITIVES: dict[str, tuple[str, int]] = {
    "void": ("void", 0), "bool": ("bool", 1), "_Bool": ("bool", 1),
    "char": ("int", 1), "signed char": ("int", 1), "unsigned char": ("int", 1),
    "short": ("int", 2), "short int": ("int", 2), "unsigned short": ("int", 2),
    "short unsigned int": ("int", 2), "wchar_t": ("int", 4),
    "int": ("int", 4), "unsigned int": ("int", 4), "unsigned": ("int", 4),
    "long": ("int", 8), "long int": ("int", 8), "unsigned long": ("int", 8),
    "long unsigned int": ("int", 8), "long long": ("int", 8),
    "long long int": ("int", 8), "unsigned long long": ("int", 8),
    "long long unsigned int": ("int", 8),
    "float": ("float", 4), "double": ("float", 8), "long double": ("float", 16),
    "size_t": ("int", 8), "ssize_t": ("int", 8), "ptrdiff_t": ("int", 8),
    "intptr_t": ("int", 8), "uintptr_t": ("int", 8),
}
for _n, _sz in (("8", 1), ("16", 2), ("32", 4), ("64", 8)):
    PRIMITIVES[f"int{_n}_t"] = ("int", _sz)
    PRIMITIVES[f"uint{_n}_t"] = ("int", _sz)
for _n, _sz in (("8", 1), ("16", 2), ("32", 4), ("64", 8)):
    PRIMITIVES[f"__int{_n}"] = ("int", _sz)
PRIMITIVES["_BOOL"] = ("bool", 1)
for _n in (1, 2, 4, 8):
    PRIMITIVES[f"int{_n}"] = ("int", _n)
    PRIMITIVES[f"uint{_n}"] = ("int", _n)
    PRIMITIVES[f"float{_n}"] = ("float", _n)

# Width-only spellings: a type was emitted, but it commits to nothing. Matched
# case-SENSITIVELY -- IDA's family always carries the leading underscore, and
# coreutils really does typedef a struct called `Word`.
UNCOMMITTED = re.compile(
    r"^(?:undefined\d*|xunknown\d*|unknown\d*|byte|word|dword|qword|code"
    r"|_BYTE|_WORD|_DWORD|_QWORD|_OWORD)$")
UNSIGNED = re.compile(r"\bunsigned\b|^u(int|char|short|long)")
QUALIFIERS = re.compile(r"\b(const|volatile|restrict|static|extern|register|struct|union|enum)\b")


def parse_type(spelling: str) -> dict:
    """A type string -> {ptr, base, kind, unsigned, size}.

    ``kind`` is ``primitive``/``struct``/``void``/``func``/``uncommitted``/
    ``none``. Anything that is neither a known primitive nor ``void`` nor a
    function is a named composite — which is what the IsCStruct step asks.
    """
    text = (spelling or "").strip()
    if not text:
        return {"ptr": 0, "base": "", "kind": "none", "unsigned": False, "size": None}
    if "(*" in text or text.startswith("FUNCTION"):
        return {"ptr": text.count("*"), "base": "FUNCTION", "kind": "func",
                "unsigned": False, "size": None}
    ptr = text.count("*")
    base = text.replace("*", " ")
    array = re.search(r"\[(\d*)\]", base)
    base = re.sub(r"\[\d*\]", " ", base)
    if array:
        ptr += 1  # `T x[N]` decays to a pointer for the purposes of this score
    unsigned = bool(UNSIGNED.search(base))
    base = QUALIFIERS.sub(" ", base)
    base = " ".join(base.split())
    if UNCOMMITTED.match(base):
        kind = "uncommitted"
    elif base in PRIMITIVES:
        kind = "void" if base == "void" else "primitive"
    elif base in ("", "..."):
        kind = "none"
    else:
        kind = "struct"
    size = PRIMITIVES.get(base, (None, None))[1]
    return {"ptr": ptr, "base": base, "kind": kind, "unsigned": unsigned,
            "size": POINTER_SIZE if ptr else size}


def gt_parse_forms(forms: list[str]) -> dict:
    """decbench's GT type *forms* -> the same record ``parse_type`` returns.

    The GT carries every name on the chain (a typedef **and** what it resolves
    to, ``"<pointee>*"`` for a pointer, an enum as ``[Name, "int"]``), so the
    rule is: the type is primitive when ANY form resolves to one, and composite
    only when none does.
    """
    best = None
    for form in forms or []:
        rec = parse_type(form)
        if rec["kind"] in ("primitive", "void") and (best is None or best["kind"] == "struct"):
            best = rec
        elif best is None:
            best = rec
    return best or {"ptr": 0, "base": "", "kind": "none", "unsigned": False, "size": None}


# --------------------------------------------------------------------------
# TRex Figure 6
# --------------------------------------------------------------------------

TREX_STEPS = ("defined", "is_c_pointer", "pointer_level", "is_c_struct",
              "sign_ignored_primitive", "c_primitive")


def trex_predicates(pred: dict | None, gt: dict, strict_defined: bool = False) -> list[bool]:
    """The six Figure-6 predicates, in order, for one variable.

    A step both sides cannot be judged on (the primitive steps when neither side
    is a primitive) passes — RecStruct §5's complaint about this metric, kept
    deliberately so the number is comparable to the published ones.
    """
    if pred is None or pred["kind"] == "none":
        return [False] * 6
    if strict_defined and pred["kind"] == "uncommitted":
        return [False] * 6
    p = [True]
    p.append((pred["ptr"] > 0) == (gt["ptr"] > 0))
    p.append(pred["ptr"] == gt["ptr"])
    p.append((pred["kind"] == "struct") == (gt["kind"] == "struct"))
    both_primitive = pred["kind"] in ("primitive", "void") and gt["kind"] in ("primitive", "void")
    if pred["kind"] == "uncommitted" or gt["kind"] == "uncommitted":
        p += [False, False]          # a width-only spelling commits to no primitive
    elif not both_primitive:
        p += [True, True]            # not applicable on either side
    else:
        pk = PRIMITIVES.get(pred["base"])
        gk = PRIMITIVES.get(gt["base"])
        same = pk is not None and pk == gk
        p.append(same)
        p.append(same and pred["unsigned"] == gt["unsigned"])
    out = []
    alive = True
    for value in p:
        out.append(bool(value) and alive)
        alive = alive and bool(value)
    return out


def trex_score(passed: list[bool]) -> int:
    """f(p1..p6) = p1*(1+p2*(1+p3*(1+p4*(1+p5*(1+p6))))), 0..6."""
    score = 0
    for value in reversed(passed):
        score = (1 + score) if value else 0
    return score


# --------------------------------------------------------------------------
# DWARF ground truth (layouts)
# --------------------------------------------------------------------------

_TRANSPARENT = ("DW_TAG_typedef", "DW_TAG_const_type", "DW_TAG_volatile_type",
                "DW_TAG_restrict_type")


def _attr(die, name):
    a = die.attributes.get(name)
    return a.value if a is not None else None


def _name(die) -> str | None:
    v = _attr(die, "DW_AT_name")
    return v.decode("utf-8", "replace") if isinstance(v, bytes) else v


def _type_die(die):
    if "DW_AT_type" not in die.attributes:
        return None
    try:
        return die.get_DIE_from_attribute("DW_AT_type")
    except Exception:  # noqa: BLE001
        return None


def _resolve(die) -> tuple[object | None, int]:
    """Strip typedefs/qualifiers and count pointer hops -> (base DIE, ptr level)."""
    ptr = 0
    seen = 0
    while die is not None and seen < 32:
        seen += 1
        if die.tag in _TRANSPARENT:
            die = _type_die(die)
        elif die.tag == "DW_TAG_pointer_type":
            ptr += 1
            die = _type_die(die)
        else:
            return die, ptr
    return die, ptr


def _die_size(die, depth: int = 0) -> int | None:
    base, ptr = _resolve(die)
    if ptr:
        return POINTER_SIZE
    if base is None or depth > 8:
        return None
    size = _attr(base, "DW_AT_byte_size")
    if size is not None:
        return size
    if base.tag == "DW_TAG_array_type":
        elem = _die_size(_type_die(base), depth + 1)
        count = None
        for sub in base.iter_children():
            n = _attr(sub, "DW_AT_upper_bound")
            if n is not None:
                count = n + 1
            elif _attr(sub, "DW_AT_count") is not None:
                count = _attr(sub, "DW_AT_count")
        return None if elem is None or count is None else elem * count
    return None


def _member_offset(die) -> int | None:
    value = _attr(die, "DW_AT_data_member_location")
    if isinstance(value, int):
        return value
    if isinstance(value, list) and value and value[0] == 0x23:  # DW_OP_plus_uconst
        out, shift = 0, 0
        for byte in value[1:]:
            out |= (byte & 0x7F) << shift
            shift += 7
            if not byte & 0x80:
                break
        return out
    return None


def struct_identity(die) -> str:
    name = _name(die)
    if name:
        return name
    return f"anon@{_attr(die, 'DW_AT_byte_size')}#{len(list(die.iter_children()))}"


def struct_layout(die, depth: int = 0) -> dict | None:
    """One ``DW_TAG_structure_type`` -> the normalized binary layout.

    RecStruct §4.1: typedefs resolved, anonymous struct/union members expanded
    at their own offsets, gaps written out as explicit padding.
    """
    if die is None or die.tag not in ("DW_TAG_structure_type", "DW_TAG_class_type"):
        return None
    if depth > 8:
        return None
    size = _attr(die, "DW_AT_byte_size")
    fields: list[dict] = []
    for member in die.iter_children():
        if member.tag != "DW_TAG_member":
            continue
        offset = _member_offset(member)
        if offset is None:
            continue
        base, ptr = _resolve(_type_die(member))
        msize = _die_size(_type_die(member))
        mname = _name(member)
        anonymous = base is not None and base.tag in (
            "DW_TAG_structure_type", "DW_TAG_union_type") and not _name(base) and not ptr
        if anonymous:
            inner = struct_layout(base, depth + 1) if base.tag == "DW_TAG_structure_type" else None
            if inner:
                for field in inner["fields"]:
                    if field["kind"] == "pad":
                        continue
                    fields.append({**field, "offset": offset + field["offset"],
                                   "name": f"{mname or 'anon'}.{field['name']}"})
                continue
            # an anonymous union is one slot of its own largest member
        pointee = None
        if ptr == 1 and base is not None and base.tag in ("DW_TAG_structure_type",
                                                          "DW_TAG_class_type"):
            pointee = struct_identity(base)
        fields.append({
            "offset": offset, "size": msize, "name": mname or f"field_{offset:#x}",
            "kind": "pointer" if ptr else (
                "struct" if base is not None and base.tag in (
                    "DW_TAG_structure_type", "DW_TAG_class_type") else "scalar"),
            "ptr": ptr, "pointee": pointee,
        })
    fields.sort(key=lambda f: (f["offset"], f["size"] or 0))
    padded, cursor = [], 0
    for field in fields:
        if field["offset"] > cursor:
            padded.append({"offset": cursor, "size": field["offset"] - cursor,
                           "name": f"_pad{cursor:#x}", "kind": "pad", "ptr": 0,
                           "pointee": None})
        padded.append(field)
        if field["size"]:
            cursor = max(cursor, field["offset"] + field["size"])
    if size and cursor < size:
        padded.append({"offset": cursor, "size": size - cursor,
                       "name": f"_pad{cursor:#x}", "kind": "pad", "ptr": 0, "pointee": None})
    return {"name": struct_identity(die), "size": size, "fields": padded}


def dwarf_functions(unstripped: Path) -> dict:
    """{low_pc: {name, params: [{index, name, ptr, struct}]}} from the twin.

    Only ``DW_TAG_subprogram`` DIEs with a ``DW_AT_low_pc``; the parameter index
    is declaration order, exactly what decbench's ``arg_index`` means.
    """
    from decbench.utils import binfmt
    out: dict[int, dict] = {}
    info = binfmt.dwarf_info(unstripped)
    if info is None:
        return out
    for cu in info.iter_CUs():
        for die in cu.get_top_DIE().iter_children():
            if die.tag != "DW_TAG_subprogram":
                continue
            low = _attr(die, "DW_AT_low_pc")
            if low is None:
                continue
            params = []
            for index, child in enumerate(
                    c for c in die.iter_children() if c.tag == "DW_TAG_formal_parameter"):
                base, ptr = _resolve(_type_die(child))
                layout = struct_layout(base) if ptr >= 1 else None
                params.append({"index": index, "name": _name(child), "ptr": ptr,
                               "struct": layout})
            out[int(low)] = {"name": _name(die) or f"sub_{low:x}", "params": params}
    return out


# --------------------------------------------------------------------------
# predicted layouts (kuna's own header)
# --------------------------------------------------------------------------

STRUCT_RE = re.compile(r"^struct\s+(\w+)\s*\{(.*?)^\};", re.M | re.S)
MEMBER_RE = re.compile(r"^\s*(?P<type>[\w\s]+?)\s*(?P<stars>\**)\s*(?P<name>\w+)"
                       r"(?:\[(?P<count>\d+)\])?;\s*$")
FIELD_OFFSET_RE = re.compile(r"^(?:field|_pad)_?(0x[0-9a-fA-F]+|\d+)$")


def header_layouts(header: str) -> dict:
    """Parse `decompile-project`'s .h into {struct name: layout}.

    Offsets come from the member names when kuna encodes them (`field_0x28`,
    `_pad4`) and from running sizes otherwise; a struct with an unsized member
    is reported with the offsets it could compute and ``complete: false``.
    """
    sizes = dict(PRIMITIVES)
    layouts: dict[str, dict] = {}
    for name, body in STRUCT_RE.findall(header):
        fields, cursor, complete = [], 0, True
        for line in body.split("\n"):
            m = MEMBER_RE.match(line)
            if not m:
                if line.strip() and not line.strip().startswith(("/*", "//", "*")):
                    complete = False
                continue
            base = " ".join(m.group("type").split())
            ptr = len(m.group("stars"))
            size = POINTER_SIZE if ptr else sizes.get(base, (None, None))[1]
            if size is not None and m.group("count"):
                size *= int(m.group("count"))
            named = FIELD_OFFSET_RE.match(m.group("name"))
            offset = int(named.group(1), 0) if named else cursor
            if size is None:
                nested = layouts.get(base)
                size = nested["size"] if nested else None
            if size is None:
                complete = False
            fields.append({"offset": offset, "size": size, "name": m.group("name"),
                           "kind": "pointer" if ptr else "scalar", "ptr": ptr,
                           "pointee": base if ptr == 1 and base in layouts else None})
            cursor = offset + (size or 0)
        layouts[name] = {"name": name, "size": cursor, "fields": fields,
                         "complete": complete}
    return layouts


# --------------------------------------------------------------------------
# running kuna
# --------------------------------------------------------------------------

def _options(options: list[list[str]]) -> list[str]:
    out = []
    for key, value in options:
        out += ["--option", key, value]
    return out


def run_json(binary: Path, options, timeout: int) -> dict:
    cmd = [config.kuna_bin(), "decompile-all", str(binary), "--json",
           "--max-fn-seconds", "120"] + _options(options)
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    if not p.stdout.strip():
        raise RuntimeError(f"kuna --json empty ({p.returncode}): {p.stderr[-300:]}")
    return json.loads(p.stdout)


def run_text(binary: Path, options, timeout: int) -> str:
    cmd = [config.kuna_bin(), "decompile-all", str(binary),
           "--max-fn-seconds", "120"] + _options(options)
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    if not p.stdout.strip():
        raise RuntimeError(f"kuna text empty ({p.returncode}): {p.stderr[-300:]}")
    return p.stdout


def run_header(binary: Path, options, timeout: int) -> str:
    """The project export's .h, written to a temporary directory and removed."""
    out = Path(tempfile.mkdtemp(prefix="structscore-"))
    try:
        cmd = [config.kuna_bin(), "decompile-project", str(binary), "-o", str(out),
               "--max-fn-seconds", "120"] + _options(options)
        subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
        headers = sorted(out.glob("*.h"))
        return headers[0].read_text() if headers else ""
    finally:
        shutil.rmtree(out, ignore_errors=True)


# --------------------------------------------------------------------------
# pairing kuna's variables to the ground truth
# --------------------------------------------------------------------------

def pair_variables(variables: list[dict], gt_vars: list[dict]):
    """GT index -> kuna variable index, by decbench's own three passes.

    Pass 1 args by ABI position, pass 2 stack variables by calibrated offset,
    pass 3 by exact name — `decbench.metrics.type_match._match_structured`'s
    order, so a pairing here is the pairing the metric would have made.
    """
    from decbench.metrics.type_match import _calibrate_shift, _effective_offset

    class _V:  # _effective_offset reads attributes, not dict keys
        def __init__(self, v):
            self.name = v.get("name") or ""
            self.stack_offset = v.get("stack_offset")
            self.arg_index = v.get("arg_index")

    offsets = [_effective_offset(_V(v)) for v in variables]
    gt_offsets = [o for gv in gt_vars for o in (gv.get("rbp_offset") or [])]
    shift = _calibrate_shift(gt_offsets, [o for o in offsets if o is not None]) or 0
    by_arg, by_off, by_name = {}, {}, {}
    for i, v in enumerate(variables):
        if v.get("arg_index") is not None:
            by_arg.setdefault(v["arg_index"], i)
        if offsets[i] is not None:
            by_off.setdefault(offsets[i] + shift, []).append(i)
        if v.get("name"):
            by_name.setdefault(v["name"], []).append(i)
    used: set[int] = set()
    pairs: dict[int, int] = {}
    for gi, gv in enumerate(gt_vars):
        if gv.get("is_arg") and gv.get("arg_index") is not None:
            i = by_arg.get(gv["arg_index"])
            if i is not None and i not in used:
                used.add(i)
                pairs[gi] = i
    for gi, gv in enumerate(gt_vars):
        if gi in pairs:
            continue
        for off in gv.get("rbp_offset") or []:
            free = [i for i in by_off.get(off, []) if i not in used]
            if free:
                used.add(free[0])
                pairs[gi] = free[0]
                break
    for gi, gv in enumerate(gt_vars):
        if gi in pairs or not gv.get("name"):
            continue
        free = [i for i in by_name.get(gv["name"], []) if i not in used]
        if free:
            used.add(free[0])
            pairs[gi] = free[0]
    return pairs


def gt_matches_kuna(pred_type: str, pred_size, gt_forms: list[str]) -> bool:
    """decbench's own match rule, including the width-only free pass."""
    from decbench.metrics.type_match import (_SIZE_SCALARS, _uncommitted_size,
                                             normalize_type)

    class _V:
        def __init__(self, t, s):
            self.type = t
            self.size = s

    forms: set[str] = set()
    for form in gt_forms or []:
        forms |= normalize_type(form)
    if normalize_type(pred_type) & forms:
        return True
    size = _uncommitted_size(_V(pred_type, pred_size))
    return size is not None and bool(_SIZE_SCALARS.get(size, set()) & forms)


# --------------------------------------------------------------------------
# the three measurements
# --------------------------------------------------------------------------

def measure_trex(payload: dict, gt_by_name: dict, addr2name: dict,
                 strict_defined: bool) -> dict:
    steps = {s: {"reached": 0, "passed": 0} for s in TREX_STEPS}
    total = scored = 0
    unmatched = 0
    histogram = {i: 0 for i in range(7)}
    for fn in payload.get("functions") or []:
        name = addr2name.get(int(fn.get("address") or -1))
        gt_vars = gt_by_name.get(name or "")
        if not gt_vars:
            continue
        variables = fn.get("variables") or []
        pairs = pair_variables(variables, gt_vars)
        for gi, gv in enumerate(gt_vars):
            gt = gt_parse_forms(gv.get("type") or [])
            i = pairs.get(gi)
            pred = parse_type(variables[i].get("type")) if i is not None else None
            if pred is None:
                unmatched += 1
            passed = trex_predicates(pred, gt, strict_defined)
            value = trex_score(passed)
            total += 1
            scored += value
            histogram[value] += 1
            reached = True
            for step, ok in zip(TREX_STEPS, passed):
                if not reached:
                    break
                steps[step]["reached"] += 1
                steps[step]["passed"] += int(ok)
                reached = ok
    return {
        "gt_variables": total,
        "unmatched_gt_variables": unmatched,
        "mean": round(scored / total, 4) if total else 0.0,
        "score_histogram": {str(k): v for k, v in histogram.items()},
        "steps": {s: {**v, "rate": round(v["passed"] / v["reached"], 4) if v["reached"] else 0.0}
                  for s, v in steps.items()},
    }


def measure_layout(payload: dict, header: str, functions: dict) -> dict:
    """Field-level layout F1 and nesting F1 over pointer-to-struct parameters."""
    layouts = header_layouts(header)
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    gt_by_identity = {p["struct"]["name"]: p["struct"]
                      for info in functions.values() for p in info["params"] if p["struct"]}
    gt_fields = pred_fields = tp = 0
    gt_nest = pred_nest = tp_nest = 0
    params = predicted_params = 0
    for low, info in functions.items():
        fn = by_addr.get(low)
        for param in info["params"]:
            layout = param["struct"]
            if not layout:
                continue
            params += 1
            gt_real = [f for f in layout["fields"] if f["kind"] != "pad"]
            gt_fields += len(gt_real)
            gt_nest += sum(1 for f in gt_real if f["pointee"])
            predicted = None
            if fn:
                for v in fn.get("variables") or []:
                    if v.get("arg_index") == param["index"]:
                        rec = parse_type(v.get("type"))
                        if rec["kind"] == "struct":
                            predicted = layouts.get(rec["base"])
                        break
            if not predicted:
                continue
            predicted_params += 1
            pred_fields += len(predicted["fields"])
            gt_keys = {(f["offset"], f["size"]) for f in gt_real}
            for field in predicted["fields"]:
                if (field["offset"], field["size"]) in gt_keys:
                    tp += 1
            for field in predicted["fields"]:
                if not field["pointee"]:
                    continue
                pred_nest += 1
                inner = layouts.get(field["pointee"])
                target = next((f for f in gt_real if f["offset"] == field["offset"]
                               and f["pointee"]), None)
                if not (inner and target):
                    continue
                gt_inner = gt_by_identity.get(target["pointee"])
                if gt_inner and {(f["offset"], f["size"]) for f in inner["fields"]} & \
                        {(f["offset"], f["size"]) for f in gt_inner["fields"] if f["kind"] != "pad"}:
                    tp_nest += 1
    def f1(tp_, pred_, gt_):
        precision = tp_ / pred_ if pred_ else 0.0
        recall = tp_ / gt_ if gt_ else 0.0
        return {"tp": tp_, "predicted": pred_, "ground_truth": gt_,
                "precision": round(precision, 4), "recall": round(recall, 4),
                "f1": round(2 * precision * recall / (precision + recall), 4)
                if precision + recall else 0.0}
    return {
        "ptr_to_struct_params": params,
        "params_kuna_typed_as_a_struct": predicted_params,
        "structs_in_header": len(layouts),
        "layout": f1(tp, pred_fields, gt_fields),
        "nesting": f1(tp_nest, pred_nest, gt_nest),
    }


DEREF_PATTERNS = (
    re.compile(r"\*\s*\(\s*[\w\s*]+\*\s*\)\s*\(\s*(?P<base>[A-Za-z_]\w*)\s*\+\s*"
               r"(?P<off>0x[0-9a-fA-F]+|\d+)\s*\)"),
    re.compile(r"(?P<base>[A-Za-z_]\w*)\s*->\s*\w*_?(?P<off>0x[0-9a-fA-F]+|\d+)"),
    re.compile(r"(?P<base>[A-Za-z_]\w*)\s*\[\s*(?P<off>0x[0-9a-fA-F]+|\d+)\s*\]"),
)
ZERO_DEREF = re.compile(r"\*\s*\(\s*[\w\s*]+\*\s*\)\s*(?P<base>[A-Za-z_]\w*)\b|"
                        r"\*(?P<base2>[A-Za-z_]\w*)\b")
ARROW_FIELD = re.compile(r"(?P<base>[A-Za-z_]\w*)\s*->\s*(?P<field>\w+)")


def census_function(chunk: str) -> dict:
    """Bases dereferenced at >= 2 distinct constant offsets in one function."""
    lines = chunk.split("\n")
    body = "\n".join(l.split("//")[0] for l in lines[1:])
    offsets: dict[str, set] = {}
    for pattern in DEREF_PATTERNS:
        for m in pattern.finditer(body):
            offsets.setdefault(m.group("base"), set()).add(int(m.group("off"), 0))
    for m in ZERO_DEREF.finditer(body):
        base = m.group("base") or m.group("base2")
        if base:
            offsets.setdefault(base, set()).add(0)
    for m in ARROW_FIELD.finditer(body):
        offsets.setdefault(m.group("base"), set()).add(hash(m.group("field")) & 0xFFFF)
    header = lines[0].strip()
    addr = re.search(r"@ (0x[0-9a-fA-F]+)", header)
    return {
        "address": int(addr.group(1), 16) if addr else None,
        "candidates": {b: sorted(o) for b, o in offsets.items() if len(o) >= 2},
    }


def measure_census(text: str, payload: dict, gt_by_name: dict, addr2name: dict) -> dict:
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    functions = candidate_functions = candidates = 0
    args = stack = unmapped = 0
    matched = already_right = gt_is_struct_ptr = 0
    for chunk in text.split("// Function: ")[1:]:
        functions += 1
        row = census_function(chunk)
        if not row["candidates"]:
            continue
        candidate_functions += 1
        candidates += len(row["candidates"])
        fn = by_addr.get(row["address"] or -1)
        if not fn:
            unmapped += len(row["candidates"])
            continue
        variables = fn.get("variables") or []
        gt_vars = gt_by_name.get(addr2name.get(row["address"] or -1) or "") or []
        pairs = pair_variables(variables, gt_vars) if gt_vars else {}
        back = {i: gi for gi, i in pairs.items()}
        for base in row["candidates"]:
            index = None
            if re.fullmatch(r"a\d+", base):
                want = int(base[1:])
                index = next((i for i, v in enumerate(variables)
                              if v.get("arg_index") == want), None)
                args += 1
            else:
                index = next((i for i, v in enumerate(variables)
                              if v.get("name") == base), None)
                if index is None:
                    unmapped += 1
                else:
                    stack += 1
            if index is None or index not in back:
                continue
            gv = gt_vars[back[index]]
            matched += 1
            if gt_matches_kuna(variables[index].get("type"), variables[index].get("size"),
                               gv.get("type") or []):
                already_right += 1
            gt = gt_parse_forms(gv.get("type") or [])
            if gt["ptr"] >= 1 and gt["kind"] == "struct":
                gt_is_struct_ptr += 1
    return {
        "functions": functions,
        "functions_with_a_candidate": candidate_functions,
        "candidate_bases": candidates,
        "candidate_is_an_arg": args,
        "candidate_is_a_named_local": stack,
        "candidate_not_on_the_json_surface": unmapped,
        "candidates_paired_with_ground_truth": matched,
        "candidates_kuna_already_types_correctly": already_right,
        "candidates_whose_truth_is_a_struct_pointer": gt_is_struct_ptr,
    }


# --------------------------------------------------------------------------
# driver
# --------------------------------------------------------------------------

def twin_of(binary: Path) -> Path:
    return Path(str(binary).replace("/stripped/", "/compiled/"))


def measure(binary: Path, unstripped: Path, modes: set, options, timeout: int,
            strict_defined: bool) -> dict:
    from decbench.metrics.type_match import extract_ground_truth_types
    functions = dwarf_functions(unstripped)
    addr2name = {addr: info["name"] for addr, info in functions.items()}
    gt_by_name = extract_ground_truth_types(unstripped)
    out = {
        "binary": str(binary), "unstripped": str(unstripped),
        "kuna_bin": config.kuna_bin(), "options": options,
        "dwarf_functions": len(functions),
        "dwarf_functions_with_variables": len(gt_by_name),
    }
    payload = run_json(binary, options, timeout)
    out["kuna_functions"] = len(payload.get("functions") or [])
    if "trex" in modes:
        out["trex"] = measure_trex(payload, gt_by_name, addr2name, strict_defined)
    if "layout" in modes:
        out["layout"] = measure_layout(payload, run_header(binary, options, timeout),
                                       functions)
    if "census" in modes:
        out["census"] = measure_census(run_text(binary, options, timeout), payload,
                                       gt_by_name, addr2name)
    return out


def report(rows: list[dict]) -> str:
    lines = ["# structscore", ""]
    if any("trex" in r for r in rows):
        lines += ["## TRex Fig. 6 (mean per binary, 0-6)", "",
                  "| binary | GT vars | unpaired | mean | " +
                  " | ".join(TREX_STEPS) + " |",
                  "|---|" + "---|" * (3 + len(TREX_STEPS))]
        for r in rows:
            t = r.get("trex")
            if not t:
                continue
            rates = " | ".join(f"{t['steps'][s]['passed']}/{t['steps'][s]['reached']}"
                               for s in TREX_STEPS)
            lines.append(f"| {Path(r['binary']).name} | {t['gt_variables']} | "
                         f"{t['unmatched_gt_variables']} | {t['mean']} | {rates} |")
        lines.append("")
    if any("layout" in r for r in rows):
        lines += ["## layout / nesting F1 vs DWARF (pointer-to-struct parameters)", "",
                  "| binary | GT params | kuna struct params | GT fields | layout F1 | "
                  "GT nestings | nesting F1 |", "|---|---|---|---|---|---|---|"]
        for r in rows:
            l = r.get("layout")
            if not l:
                continue
            lines.append(f"| {Path(r['binary']).name} | {l['ptr_to_struct_params']} | "
                         f"{l['params_kuna_typed_as_a_struct']} | "
                         f"{l['layout']['ground_truth']} | {l['layout']['f1']} | "
                         f"{l['nesting']['ground_truth']} | {l['nesting']['f1']} |")
        lines.append("")
    if any("census" in r for r in rows):
        lines += ["## struct-candidate census (>= 2 distinct constant offsets on one base)", "",
                  "| binary | fns | fns with a candidate | candidates | paired to GT | "
                  "already right | GT is struct* |", "|---|---|---|---|---|---|---|"]
        for r in rows:
            c = r.get("census")
            if not c:
                continue
            lines.append(
                f"| {Path(r['binary']).name} | {c['functions']} | "
                f"{c['functions_with_a_candidate']} | {c['candidate_bases']} | "
                f"{c['candidates_paired_with_ground_truth']} | "
                f"{c['candidates_kuna_already_types_correctly']} | "
                f"{c['candidates_whose_truth_is_a_struct_pointer']} |")
        lines.append("")
    return "\n".join(lines) + "\n"


SELFTEST_HEADER = """
struct struct_0 {
    char *field_0x0;
    unsigned int field_0x8;
};

struct nest_1 {
    struct_0 *field_0x0;
    long field_0x8;
};
"""

SELFTEST_TEXT = """// Function: sub_3000 @ 0x3000
void sub_3000(long a0)
{
  int v1;
  v1 = *(int *)(a0 + 8);
  *(char **)(a0 + 0) = "x";
  v1 = v1 + *(int *)(a0 + 0xc);
}
"""


def selftest() -> int:
    gt_u64 = {"ptr": 0, "base": "unsigned long", "kind": "primitive",
              "unsigned": True, "size": 8}
    scores = {spelling: trex_score(trex_predicates(parse_type(spelling), gt_u64))
              for spelling in ("char *", "struct_0", "double", "long", "unsigned long")}
    header = header_layouts(SELFTEST_HEADER)
    census = census_function(SELFTEST_TEXT.split("// Function: ")[1])
    checks = [
        # TRex Figure 6's own worked example: against an expected uint64_t the
        # types char*, struct{..}, double, int64_t, uint64_t score 1,3,4,5,6.
        ("Figure 6 example: char* scores 1", scores["char *"] == 1),
        ("Figure 6 example: a struct scores 3", scores["struct_0"] == 3),
        ("Figure 6 example: double scores 4", scores["double"] == 4),
        ("Figure 6 example: the signed twin scores 5", scores["long"] == 5),
        ("Figure 6 example: the exact type scores 6", scores["unsigned long"] == 6),
        ("an unmatched variable scores 0", trex_score(trex_predicates(None, gt_u64)) == 0),
        ("undefined8 is defined but commits to no primitive",
         trex_score(trex_predicates(parse_type("undefined8"), gt_u64)) == 4),
        ("--strict-defined scores it 0",
         trex_score(trex_predicates(parse_type("undefined8"), gt_u64, True)) == 0),
        ("a pointer level mismatch stops at step 2",
         trex_score(trex_predicates(parse_type("char **"),
                                    {"ptr": 1, "base": "char", "kind": "primitive",
                                     "unsigned": False, "size": 8})) == 2),
        ("GT forms prefer the resolved primitive over the typedef name",
         gt_parse_forms(["size_t", "unsigned long"])["kind"] == "primitive"),
        ("GT pointer-to-struct is composite",
         (lambda r: r["ptr"] == 1 and r["kind"] == "struct")(gt_parse_forms(["Word*"]))),
        ("header offsets come from the field names",
         [f["offset"] for f in header["struct_0"]["fields"]] == [0, 8]),
        ("a struct-pointer member records its pointee",
         header["nest_1"]["fields"][0]["pointee"] == "struct_0"),
        ("a base with 3 distinct offsets is a candidate",
         census["candidates"].get("a0") == [0, 8, 12]),
        ("f(p) is p1*(1+p2*(1+...))", trex_score([True] * 6) == 6
         and trex_score([True, True, False, True, True, True]) == 2),
    ]
    for name, ok in checks:
        print(f"[{'ok ' if ok else 'FAIL'}] {name}")
    bad = [n for n, ok in checks if not ok]
    print(f"selftest: {len(checks) - len(bad)}/{len(checks)} passed")
    return 1 if bad else 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Writes nothing unless --out is given.")
    ap.add_argument("binary", nargs="*", type=Path, help="stripped binary to score")
    ap.add_argument("--unstripped", type=Path,
                    help="the DWARF twin (default: /stripped/ -> /compiled/)")
    ap.add_argument("--layout", action="store_true", help="layout + nesting F1")
    ap.add_argument("--trex", action="store_true", help="TRex Fig. 6 prioritized score")
    ap.add_argument("--census", action="store_true", help="struct-candidate census")
    ap.add_argument("--all", action="store_true", help="all three")
    ap.add_argument("--strict-defined", action="store_true",
                    help="score an uncommitted spelling (undefined8) as undefined")
    ap.add_argument("--option", nargs=2, action="append", metavar=("NAME", "VALUE"),
                    default=[], help="kuna option for this run (repeatable)")
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--out", type=Path, help="write the full JSON here")
    ap.add_argument("--selftest", action="store_true")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()
    if not args.binary:
        ap.error("need at least one binary")
    modes = {m for m in ("layout", "trex", "census") if getattr(args, m)}
    if args.all or not modes:
        modes = {"layout", "trex", "census"}

    try:
        import decbench.metrics.type_match  # noqa: F401
    except ImportError as e:
        sys.exit(f"cannot import decbench ({e}) -- run under the decbench venv python:\n"
                 f"  {config.decbench_python()} -m scripts.decbench.structscore ...")
    os.environ.setdefault("DECBENCH_NO_CACHE", "1")

    rows = []
    for binary in args.binary:
        twin = args.unstripped or twin_of(binary)
        if not twin.exists():
            print(f"[structscore] no unstripped twin for {binary} ({twin})", file=sys.stderr)
            continue
        rows.append(measure(binary, twin, modes, args.option, args.timeout,
                            args.strict_defined))
        print(f"[structscore] {binary.name} done", file=sys.stderr)
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(rows, indent=1) + "\n")
    print(report(rows), end="")
    return 0


if __name__ == "__main__":
    sys.exit(main())
