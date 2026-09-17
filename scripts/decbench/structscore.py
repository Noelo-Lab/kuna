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
`decbench.metrics.type_match.extract_ground_truth_types`, paired by
`pair_variables` — the metric's own three passes (arg index, calibrated stack
offset, name), its binary-wide calibration shift, and its type-preferring
`claim()`, so a variable is scored against the ground-truth variable the metric
would have scored it against.

The last step is the only one that turns on signedness, and decbench's ground
truth cannot answer it: `normalize_type` strips the `unsigned` qualifier, so a
DWARF `long unsigned int` and a `long int` arrive spelled the same way. The sign
therefore comes from the twin's own `DW_AT_encoding` (the DIE walk is already
there for the layouts). Where DWARF carries no encoding the step is scored as
failed and counted in `sign_unjudged`, which makes `mean` a lower bound;
`mean_0_5` drops the step entirely and is exact for every variable — it is the
number to quote for anything that is not a signedness claim.

Two further judgement calls, both flagged: an *uncommitted* spelling
(`undefined8`, `xunknown8`, `_QWORD`) counts as **defined** but of unknown
primitive class, so it can pass the pointer/struct steps and never the primitive
ones (`--strict-defined` scores it 0 instead); and a step neither side can be
judged on -- the primitive steps when both sides are structs -- passes, which is
exactly the blindness RecStruct §5 charges the metric with. Report them together
and none can hide the others.

**--census** (the opportunity ceiling). Over the same run's C text, a *base* is
any identifier — a parameter, a declared local, or a file-scope global (`dat_*`,
counted separately and never on the JSON surface) — *accessed* at two or more
distinct offsets (`*(T *)(B ± K)`, `B[k]`, `*B`, `B->f`). Declaration and
prototype lines are dropped first: the `*` in `char *v1;` is not a dereference.
Offsets are BYTES: `B[k]` is multiplied by the element size the declaration
gives `B`, and only where nothing gives one does an index keep an index key
(`[3]`), a named field its name (`.next`).

The pool is then **split by evidence**, because the two halves answer different
questions. `*(T *)(B + K)` and `B->f` commit to a field; `B[k]` and `*B` do not
— `char *s; s[0]; s[1]` is a string walk, and 1,123 of the 1,310 candidates in
the recorded census have no other evidence. Each half is then measured against
the ground truth for how many of its candidates kuna **already** types right:
that is the match->miss channel a synthesis pass would spend, and on the
field-committed half it is ~zero (1/80 at O0, 0/69 at O2), which is what makes
struct synthesis a *quality* feature rather than a metric one.

Usage (needs the decbench venv python)::

    ~/.virtualenvs/decbench/bin/python -m scripts.decbench.structscore \\
        <stripped-binary>... --all                      # all three
    ... --trex --census                                 # pick the modes
    ... --unstripped <twin>                             # non-decbench layout
    ... --option structsynth param                      # measure a flip
    ... --out /tmp/structscore.json                     # the only write
    python3 -m scripts.decbench.structscore --selftest   # decbench only for pairing

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

    An array is scored as its element type on BOTH sides: ``char[3]`` parses to
    the same record as ``char``, and so does the ground truth, which decbench
    hands back as ``['char', 'char[3]']``. Decaying ``T[N]`` to a pointer here
    only (the earlier convention) scored an exactly correct ``char[3]`` 1 of 6,
    and it would not agree with the metric the campaign steers by either:
    ``normalize_type('char[3]')`` is ``{'char[3]'}``, which no pointer form
    matches. The six predicates never ask about extents, so dropping them costs
    nothing.
    """
    text = (spelling or "").strip()
    if not text:
        return {"ptr": 0, "base": "", "kind": "none", "unsigned": False, "size": None}
    if "(*" in text or text.startswith("FUNCTION"):
        return {"ptr": text.count("*"), "base": "FUNCTION", "kind": "func",
                "unsigned": False, "size": None}
    ptr = text.count("*")
    base = re.sub(r"\[\d*\]", " ", text.replace("*", " "))
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
    only when none does. An array arrives as both forms (``['char', 'char[3]']``)
    and ``parse_type`` reads them identically, so which one wins is moot.
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


def trex_predicates(pred: dict | None, gt: dict, strict_defined: bool = False,
                    gt_sign: bool | None = None) -> tuple[list[bool], bool]:
    """The six Figure-6 predicates, in order, for one variable.

    Returns ``(passed, sign_unknown)``. ``gt_sign`` is the ground truth's
    signedness and must come from DWARF ``DW_AT_encoding``, NOT from the form
    list: decbench's ``normalize_type`` strips the ``unsigned`` qualifier and
    returns a sorted set, so a DWARF ``long unsigned int`` arrives as
    ``['long int', 'long long', 'long unsigned int']`` and reading a sign off it
    resolves to "signed" every time — which would score the wrong spelling 6 and
    the right one 5. When the sign is unknown the last step is not judged: it is
    reported as failed (so the mean is a lower bound) and counted separately.

    A step both sides cannot be judged on (the primitive steps when neither side
    is a primitive) passes — RecStruct §5's complaint about this metric, kept
    deliberately so the number is comparable to the published ones.
    """
    if pred is None or pred["kind"] == "none":
        return [False] * 6, False
    if strict_defined and pred["kind"] == "uncommitted":
        return [False] * 6, False
    sign_unknown = False
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
        sign_unknown = same and gt_sign is None
        p.append(same and gt_sign is not None and pred["unsigned"] == gt_sign)
    out = []
    alive = True
    for value in p:
        out.append(bool(value) and alive)
        alive = alive and bool(value)
    return out, sign_unknown and all(out[:5])


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


# DWARF base-type encodings (DWARF5 §7.8) grouped by the question the last TRex
# step actually asks: does the ground truth's own C spelling carry `unsigned`?
# `unsigned char` does and `char`, `_Bool` and `double` do not, whatever their
# representation. A pointer/struct/array base answers nothing, and neither does
# an enum with no explicit underlying type.
_UNSIGNED_ENCODINGS = {0x07, 0x08, 0x0E}         # unsigned, unsigned_char, unsigned_fixed
_SIGNED_ENCODINGS = {0x01, 0x02, 0x03, 0x04,     # address, boolean, complex, float
                     0x05, 0x06, 0x0D, 0x0F,     # signed, signed_char, signed_fixed, decimal
                     0x10, 0x11, 0x12}           # UTF, UCS, ASCII


def die_sign(die, depth: int = 0) -> bool | None:
    """Does the DWARF type a DIE resolves to spell itself ``unsigned``? Or None.

    This is the ground truth the last TRex step needs. It cannot be read off
    decbench's form list: ``normalize_type`` strips ``unsigned``, so an unsigned
    GT type and its signed twin arrive spelled the same way.
    """
    base, _ = _resolve(die)
    if base is None or depth > 8:
        return None
    if base.tag == "DW_TAG_enumeration_type":
        inner = _type_die(base)
        return die_sign(inner, depth + 1) if inner is not None else None
    if base.tag != "DW_TAG_base_type":
        return None
    enc = _attr(base, "DW_AT_encoding")
    enc = int(enc) if isinstance(enc, int) else None
    if enc in _UNSIGNED_ENCODINGS:
        return True
    if enc in _SIGNED_ENCODINGS:
        return False
    return None


def _collect_signs(die, out: dict, depth: int = 0) -> None:
    """name -> signedness for every parameter/local under one subprogram DIE.

    Two same-named locals in different lexical blocks that disagree leave the
    name unsigned-unknown rather than guessing which one decbench paired.
    """
    if depth > 8:
        return
    for child in die.iter_children():
        if child.tag in ("DW_TAG_formal_parameter", "DW_TAG_variable"):
            name = _name(child)
            if not name:
                continue
            sign = die_sign(_type_die(child))
            if name in out and out[name] != sign:
                out[name] = None
            else:
                out.setdefault(name, sign)
        elif child.tag in ("DW_TAG_lexical_block", "DW_TAG_inlined_subroutine"):
            _collect_signs(child, out, depth + 1)


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
    """{low_pc: {name, params, var_signs}} from the twin.

    Only ``DW_TAG_subprogram`` DIEs with a ``DW_AT_low_pc``; the parameter index
    is declaration order, exactly what decbench's ``arg_index`` means.
    ``var_signs`` maps each parameter/local name to its DWARF signedness, which
    is the only sound source for the last TRex step.
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
            signs: dict[str, bool | None] = {}
            _collect_signs(die, signs)
            out[int(low)] = {"name": _name(die) or f"sub_{low:x}", "params": params,
                             "var_signs": signs}
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

class _Var:
    """A stand-in for decbench's ``VariableInfo`` over one kuna JSON variable.

    ``_effective_offset`` and ``_uncommitted_size`` read attributes, not dict
    keys, and the fields map one-for-one onto the JSON surface the metric is
    handed (``typesweep.build_result`` builds the same object).
    """

    def __init__(self, v: dict):
        self.name = v.get("name") or ""
        self.type = v.get("type") or ""
        self.size = v.get("size")
        self.stack_offset = v.get("stack_offset")
        self.arg_index = v.get("arg_index")
        self.kind = v.get("kind")


def effective_offsets(variables: list[dict]) -> list:
    from decbench.metrics.type_match import _effective_offset
    return [_effective_offset(_Var(v)) for v in variables]


def binary_calibration_shift(payload: dict, gt_by_name: dict, addr2name: dict):
    """decbench's binary-wide stack shift (`TypeMatchMetric._calibrate_binary_shift`).

    The metric calibrates ONE additive shift over every function of the binary
    and hands that to each function; the per-function shift overrides it only
    where the binary-wide one aligns nothing and the per-function one aligns
    something. Pairing on the per-function shift alone is a different pairing —
    over the census corpus it moves 453 of 9,629 ground-truth variables — so it
    is computed once per binary here and passed to `pair_variables`.
    """
    from decbench.metrics.type_match import _calibrate_shift_multi
    pairs = []
    for fn in payload.get("functions") or []:
        gt_vars = gt_by_name.get(addr2name.get(int(fn.get("address") or -1)) or "")
        if not gt_vars:
            continue
        gt = [o for gv in gt_vars for o in (gv.get("rbp_offset") or [])]
        dec = [o for o in effective_offsets(fn.get("variables") or []) if o is not None]
        if gt and dec:
            pairs.append((gt, dec))
    return _calibrate_shift_multi(pairs)


def pair_variables(variables: list[dict], gt_vars: list[dict], binary_shift=None):
    """GT index -> kuna variable index, exactly as `_match_structured` pairs them.

    Pass 1 args by ABI position, pass 2 stack variables by calibrated offset,
    pass 3 by exact name. Passes 2 and 3 go through decbench's `claim()`, which
    takes the first unused candidate that TYPE-MATCHES the ground truth and only
    then falls back to the first unused one — taking `free[0]` instead pairs a
    different variable wherever an offset or a name carries more than one.

    ``binary_shift`` is `binary_calibration_shift`'s value for the whole binary;
    the per-function shift replaces it only under the metric's own rule (the
    binary-wide shift aligns nothing and the per-function one aligns something).
    Called without it, the pairing is the metric's only on binaries whose two
    shifts agree.
    """
    from decbench.metrics.type_match import (_SIZE_SCALARS, _calibrate_shift,
                                             _uncommitted_size, normalize_type)
    offsets = effective_offsets(variables)
    decomp_offsets = [o for o in offsets if o is not None]
    gt_offsets = [o for gv in gt_vars for o in (gv.get("rbp_offset") or [])]
    gt_off_set = set(gt_offsets)

    def aligned(k) -> int:
        if k is None or not decomp_offsets:
            return 0
        return len({d + k for d in decomp_offsets} & gt_off_set)

    shift = binary_shift if binary_shift is not None else 0
    func_shift = _calibrate_shift(gt_offsets, decomp_offsets)
    if func_shift is not None and aligned(shift) == 0 and aligned(func_shift) > 0:
        shift = func_shift
    k = shift if shift is not None else 0

    var_types = [normalize_type(v.get("type") or "") for v in variables]
    var_unc = [_uncommitted_size(_Var(v)) for v in variables]
    by_arg, by_off, by_name = {}, {}, {}
    for i, v in enumerate(variables):
        if v.get("arg_index") is not None:
            by_arg.setdefault(v["arg_index"], i)
        if offsets[i] is not None:
            by_off.setdefault(offsets[i] + k, []).append(i)
        if v.get("name"):
            by_name.setdefault(v["name"], []).append(i)

    def matches(gt_forms: set, i: int) -> bool:
        if gt_forms & var_types[i]:
            return True
        size = var_unc[i]
        return size is not None and bool(_SIZE_SCALARS.get(size, set()) & gt_forms)

    used: set = set()
    pairs: dict = {}

    def claim(candidates: list, gt_forms: set):
        avail = [i for i in candidates if i not in used]
        if not avail:
            return None
        hit = next((i for i in avail if matches(gt_forms, i)), None)
        i = hit if hit is not None else avail[0]
        used.add(i)
        return i

    for gi, gv in enumerate(gt_vars):
        if not gv.get("is_arg") or gv.get("arg_index") is None:
            continue
        i = by_arg.get(gv["arg_index"])
        if i is None or i in used:
            continue
        used.add(i)
        pairs[gi] = i
    for gi, gv in enumerate(gt_vars):
        if gi in pairs:
            continue
        candidates: list = []
        for off in gv.get("rbp_offset") or []:
            candidates.extend(by_off.get(off, []))
        i = claim(candidates, set(gv.get("type") or []))
        if i is not None:
            pairs[gi] = i
    for gi, gv in enumerate(gt_vars):
        if gi in pairs or not gv.get("name"):
            continue
        i = claim(by_name.get(gv["name"], []), set(gv.get("type") or []))
        if i is not None:
            pairs[gi] = i
    return pairs


def gt_matches_kuna(pred_type: str, pred_size, gt_forms: list[str]) -> bool:
    """decbench's `_matches`, including the width-only free pass.

    ``gt_forms`` are decbench's ground-truth forms, which
    `extract_ground_truth_types` has already run through `normalize_type` — they
    are used exactly as the metric uses them, unnormalized a second time.
    """
    from decbench.metrics.type_match import (_SIZE_SCALARS, _uncommitted_size,
                                             normalize_type)
    forms = set(gt_forms or [])
    if normalize_type(pred_type) & forms:
        return True
    size = _uncommitted_size(_Var({"type": pred_type, "size": pred_size}))
    return size is not None and bool(_SIZE_SCALARS.get(size, set()) & forms)


# --------------------------------------------------------------------------
# the three measurements
# --------------------------------------------------------------------------

def measure_trex(payload: dict, gt_by_name: dict, addr2name: dict,
                 strict_defined: bool, signs_by_name: dict | None = None,
                 binary_shift=None) -> dict:
    """The Fig. 6 mean, plus the sign-free 0-5 mean the sign step cannot move.

    ``mean`` is the full 0-6 score with the ground-truth signedness read from
    DWARF; where DWARF carries no encoding the last step is scored as failed, so
    ``mean`` is a lower bound and ``sign_unjudged`` says over how many variables.
    ``mean_0_5`` drops the last step entirely and is exact for every variable —
    quote it for any claim that is not about signedness.

    ``loss`` is what the mean is made of, and it is the number to read before
    picking a step to work on: a variable whose first failure is step *i*
    (0-based) scores *i*, so that failure costs ``6 - i`` points, and
    ``points_possible - points_lost`` is ``mean * gt_variables`` exactly. A step
    late in the order can fail far more often than an early one and still be
    worth a fraction of it.

    ``gt_variables`` counts the ground truth of every kuna function that resolves
    to a DWARF name, which is the set the metric scores too: a name two
    subprograms share is counted once per resolving address, and a subprogram
    ``dwarf_functions`` cannot name is not counted at all.
    """
    signs_by_name = signs_by_name or {}
    steps = {s: {"reached": 0, "passed": 0} for s in TREX_STEPS}
    total = scored = scored5 = 0
    unmatched = sign_unjudged = 0
    histogram = {i: 0 for i in range(7)}
    for fn in payload.get("functions") or []:
        name = addr2name.get(int(fn.get("address") or -1))
        gt_vars = gt_by_name.get(name or "")
        if not gt_vars:
            continue
        signs = signs_by_name.get(name or "") or {}
        variables = fn.get("variables") or []
        pairs = pair_variables(variables, gt_vars, binary_shift)
        for gi, gv in enumerate(gt_vars):
            gt = gt_parse_forms(gv.get("type") or [])
            i = pairs.get(gi)
            pred = parse_type(variables[i].get("type")) if i is not None else None
            if pred is None:
                unmatched += 1
            passed, unknown = trex_predicates(pred, gt, strict_defined,
                                              signs.get(gv.get("name") or ""))
            value = trex_score(passed)
            total += 1
            scored += value
            scored5 += trex_score(passed[:5])
            sign_unjudged += int(unknown)
            histogram[value] += 1
            reached = True
            for step, ok in zip(TREX_STEPS, passed):
                if not reached:
                    break
                steps[step]["reached"] += 1
                steps[step]["passed"] += int(ok)
                reached = ok
    loss, lost = {}, 0
    for i, step in enumerate(TREX_STEPS):
        failures = steps[step]["reached"] - steps[step]["passed"]
        loss[step] = {"failures": failures, "weight": 6 - i,
                      "points": failures * (6 - i)}
        lost += failures * (6 - i)
    return {
        "gt_variables": total,
        "unmatched_gt_variables": unmatched,
        "mean": round(scored / total, 4) if total else 0.0,
        "mean_0_5": round(scored5 / total, 4) if total else 0.0,
        "sign_source": "DWARF DW_AT_encoding",
        "sign_unjudged": sign_unjudged,
        "score_histogram": {str(k): v for k, v in histogram.items()},
        "steps": {s: {**v, "rate": round(v["passed"] / v["reached"], 4) if v["reached"] else 0.0}
                  for s, v in steps.items()},
        "points_possible": 6 * total,
        "points_lost": lost,
        "loss": loss,
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


# An access whose offset is a byte offset the code itself wrote down: a cast
# dereference at a constant displacement, either sign. This is the evidence that
# says "fields", because nothing about an array produces it.
OFFSET_DEREF = re.compile(
    r"\*\s*\(\s*[\w\s*]+\*\s*\)\s*\(\s*(?P<base>[A-Za-z_]\w*)\s*"
    r"(?P<sign>[-+])\s*(?P<off>0x[0-9a-fA-F]+|\d+)\s*\)")
# An access whose offset is an ELEMENT index. `B[k]` on a `char *` is what every
# string walk looks like, so on its own it is not evidence of a struct.
INDEX_DEREF = re.compile(
    r"(?P<base>[A-Za-z_]\w*)\s*\[\s*(?P<off>0x[0-9a-fA-F]+|\d+)\s*\]")
ZERO_DEREF = re.compile(r"\*\s*\(\s*[\w\s*]+\*\s*\)\s*(?P<base>[A-Za-z_]\w*)\b|"
                        r"\*(?P<base2>[A-Za-z_]\w*)\b")
ARROW_FIELD = re.compile(r"(?P<base>[A-Za-z_]\w*)\s*->\s*(?P<field>\w+)")
# `field_0x28` / `_pad4` carry their own offset; any other field name does not,
# and is keyed by the name itself rather than by a number invented for it.
OFFSET_FIELD = re.compile(r"^(?:field|_pad|off)?_?(0x[0-9a-fA-F]+|\d+)$")
# A declaration (`char *v1;`) and a prototype (`void f(struct_0 *a0)`) both
# carry a `*` that is not a dereference. Neither is an access, so neither may
# create the offset-0 access that turns a one-offset base into a candidate --
# but both do say what a base points AT, which is what sizes `B[k]`.
DECL_LINE = re.compile(
    r"^\s*(?!return\b|goto\b|break\b|continue\b|else\b|do\b|case\b|default\b)"
    r"[A-Za-z_]\w*(?:\s+[A-Za-z_]\w*)*\s*\**\s*"
    r"[A-Za-z_]\w*\s*(?:\[\s*\d*\s*\])?\s*;\s*$")
DECLARATOR = re.compile(
    r"(?P<type>[A-Za-z_]\w*(?:\s+[A-Za-z_]\w*)*)\s*(?P<stars>\**)\s*"
    r"(?P<name>[A-Za-z_]\w*)\s*(?P<arr>\[\s*(?:\d+|0x[0-9a-fA-F]+)?\s*\])?\s*(?=[;,)])")
# kuna spells a file-scope datum `dat_<hex>`; it is an identifier like any other
# and is counted as a base, but it is never on the `--json` variable surface.
GLOBAL_NAME = re.compile(r"^dat_[0-9a-fA-F]+$")
WIDTH_SUFFIX = re.compile(r"^(?:undefined|xunknown|unknown|int|uint|float)(\d+)$")


def type_width(spelling: str) -> int | None:
    """Bytes of one value of this type, or None where the table cannot say.

    Covers the primitives and the width-only spellings (`undefined8`), which is
    everything an element size can be read off; a named composite cannot be
    sized from its name and returns None.
    """
    base = " ".join(QUALIFIERS.sub(" ", spelling or "").split())
    if base in PRIMITIVES:
        return PRIMITIVES[base][1] or None
    m = WIDTH_SUFFIX.match(base)
    return int(m.group(1)) // 8 if m and int(m.group(1)) >= 8 else None


def declared_element_sizes(chunk: str) -> dict:
    """base -> element size in bytes, from the prototype and the declarations.

    `B[k]` is an index, not a byte offset, and the two only become comparable
    through the size of what `B` points at: a pointer-to-pointer steps by a
    pointer, a pointer (or an array) by its element type. The prototype carries
    the parameters and the declaration lines the locals; a global has neither,
    so it keeps its index keys.
    """
    lines = [l.split("//")[0] for l in chunk.split("\n")[1:]]
    brace = next((i for i, l in enumerate(lines) if l.strip() == "{"), 0)
    pieces = lines[:brace] + [l for l in lines[brace + 1:] if DECL_LINE.match(l)]
    sizes: dict = {}
    for piece in pieces:
        for m in DECLARATOR.finditer(piece):
            stars = m.group("stars")
            if not stars and not m.group("arr"):
                continue          # a scalar is not indexable
            size = POINTER_SIZE if len(stars) > 1 else type_width(m.group("type"))
            if size:
                sizes.setdefault(m.group("name"), size)
    return sizes


def _access_key(text: str, sign: str = "+") -> str:
    """A constant byte offset -> ``0x10`` / ``-0x10``; keys are strings."""
    value = int(text, 0) * (-1 if sign == "-" else 1)
    return f"-{-value:#x}" if value < 0 else f"{value:#x}"


def _key_order(key: str) -> tuple:
    """Byte offsets first (by value), then index keys, then field names.

    Sorting the keys as plain strings put ``0x28`` before ``0x4``; the order is
    cosmetic but it is what a reader of the JSON sees.
    """
    if key.startswith("["):
        return (1, int(key[1:-1]), key)
    if key.startswith("."):
        return (2, 0, key)
    return (0, int(key, 16), key)


def census_body(chunk: str) -> str:
    """The statements of one `// Function:` chunk — no marker, prototype or decls.

    Everything up to and including the opening brace is the prototype, and a
    declaration line is not an access; both would otherwise contribute a bogus
    offset-0 dereference through the bare ``*B`` arm of ``ZERO_DEREF``.
    """
    lines = [l.split("//")[0] for l in chunk.split("\n")[1:]]
    start = next((i + 1 for i, l in enumerate(lines) if l.strip() == "{"), 1)
    return "\n".join(l for l in lines[start:] if not DECL_LINE.match(l))


def census_function(chunk: str) -> dict:
    """Bases accessed at >= 2 distinct offsets in one function, by evidence kind.

    Every access is keyed by a BYTE offset where the function says what the base
    points at: `*(T *)(B + K)` and `B->field_0xNN` write the offset down, and
    `B[k]` is multiplied by the element size from `B`'s declaration (`B[3]` on a
    `char **` is `0x18`). Where nothing says — a global, a pointer to a named
    composite — an index access keeps an index key (``[3]``) and a named field
    its own name (``.next``), so the two unit systems never share a key space.

    ``offset_or_field`` marks the bases with at least one access that commits to
    a field: a cast dereference at a constant displacement or an arrow. A base
    seen only through `B[k]` and `*B` is an array walk as much as a struct, and
    the two pools answer completely different questions — see the census section
    of `docs/decbench/typecampaign/baseline.md`.
    """
    body = census_body(chunk)
    sizes = declared_element_sizes(chunk)
    keys: dict = {}
    committed: set = set()
    for m in OFFSET_DEREF.finditer(body):
        base = m.group("base")
        keys.setdefault(base, set()).add(_access_key(m.group("off"), m.group("sign")))
        committed.add(base)
    for m in INDEX_DEREF.finditer(body):
        base = m.group("base")
        size = sizes.get(base)
        index = int(m.group("off"), 0)
        keys.setdefault(base, set()).add(
            _access_key(str(index * size)) if size else f"[{index}]")
    for m in ZERO_DEREF.finditer(body):
        base = m.group("base") or m.group("base2")
        if base:
            keys.setdefault(base, set()).add("0x0")
    for m in ARROW_FIELD.finditer(body):
        base = m.group("base")
        named = OFFSET_FIELD.match(m.group("field"))
        keys.setdefault(base, set()).add(
            _access_key(named.group(1)) if named else f".{m.group('field')}")
        committed.add(base)
    addr = re.search(r"@ (0x[0-9a-fA-F]+)", chunk.split("\n")[0].strip())
    return {
        "address": int(addr.group(1), 16) if addr else None,
        "candidates": {b: {"offsets": sorted(o, key=_key_order),
                           "offset_or_field": b in committed}
                       for b, o in keys.items() if len(o) >= 2},
    }


def _channel() -> dict:
    return {"functions_with_a_candidate": 0, "candidates": 0,
            "candidate_is_an_arg": 0, "candidate_is_a_named_local": 0,
            "candidate_is_a_global": 0, "candidate_not_on_the_json_surface": 0,
            "candidates_paired_with_ground_truth": 0,
            "candidates_kuna_already_types_correctly": 0,
            "candidates_whose_truth_is_a_struct_pointer": 0}


def measure_census(text: str, payload: dict, gt_by_name: dict, addr2name: dict,
                   binary_shift=None) -> dict:
    """The candidate pool, split by what the evidence for it was.

    ``offset_or_field`` is the struct-shaped pool (at least one `*(T *)(B + K)`
    or `B->f`); ``index_only`` is everything whose only evidence is `B[k]` / `*B`
    — an array or string walk reads exactly the same. ``all`` is the union, and
    the match->miss channel is only meaningful per pool.
    """
    by_addr = {int(f.get("address") or -1): f for f in payload.get("functions") or []}
    channels = {"offset_or_field": _channel(), "index_only": _channel(), "all": _channel()}
    functions = candidate_functions = 0
    for chunk in text.split("// Function: ")[1:]:
        functions += 1
        row = census_function(chunk)
        if not row["candidates"]:
            continue
        candidate_functions += 1
        channels["all"]["functions_with_a_candidate"] += 1
        for kind in {"offset_or_field" if ev["offset_or_field"] else "index_only"
                     for ev in row["candidates"].values()}:
            channels[kind]["functions_with_a_candidate"] += 1
        fn = by_addr.get(row["address"] or -1)
        variables = (fn or {}).get("variables") or []
        gt_vars = gt_by_name.get(addr2name.get(row["address"] or -1) or "") or []
        pairs = pair_variables(variables, gt_vars, binary_shift) if (fn and gt_vars) else {}
        back = {i: gi for gi, i in pairs.items()}
        for base, ev in row["candidates"].items():
            buckets = [channels["all"],
                       channels["offset_or_field" if ev["offset_or_field"] else "index_only"]]
            index = None
            if GLOBAL_NAME.match(base):
                kind = "candidate_is_a_global"
            elif re.fullmatch(r"a\d+", base):
                index = next((i for i, v in enumerate(variables)
                              if v.get("arg_index") == int(base[1:])), None)
                kind = "candidate_is_an_arg"
            else:
                index = next((i for i, v in enumerate(variables)
                              if v.get("name") == base), None)
                kind = "candidate_is_a_named_local"
            paired = index is not None and index in back
            gv = gt_vars[back[index]] if paired else None
            already = paired and gt_matches_kuna(
                variables[index].get("type"), variables[index].get("size"),
                gv.get("type") or [])
            gt = gt_parse_forms(gv.get("type") or []) if paired else None
            struct_ptr = bool(gt and gt["ptr"] >= 1 and gt["kind"] == "struct")
            for acc in buckets:
                acc["candidates"] += 1
                acc[kind] += 1
                acc["candidate_not_on_the_json_surface"] += int(index is None)
                acc["candidates_paired_with_ground_truth"] += int(paired)
                acc["candidates_kuna_already_types_correctly"] += int(already)
                acc["candidates_whose_truth_is_a_struct_pointer"] += int(struct_ptr)
    return {
        "functions": functions,
        "functions_with_a_candidate": candidate_functions,
        "candidate_bases": channels["all"]["candidates"],
        "channels": channels,
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
    # One shift for the whole binary, as `compute_for_binary` does it.
    shift = binary_calibration_shift(payload, gt_by_name, addr2name)
    out["calibration_shift"] = shift
    if "trex" in modes:
        signs_by_name = {info["name"]: info.get("var_signs") or {}
                         for info in functions.values()}
        out["trex"] = measure_trex(payload, gt_by_name, addr2name, strict_defined,
                                   signs_by_name, shift)
    if "layout" in modes:
        out["layout"] = measure_layout(payload, run_header(binary, options, timeout),
                                       functions)
    if "census" in modes:
        out["census"] = measure_census(run_text(binary, options, timeout), payload,
                                       gt_by_name, addr2name, shift)
    return out


def report(rows: list[dict]) -> str:
    lines = ["# structscore", ""]
    if any("trex" in r for r in rows):
        lines += ["## TRex Fig. 6 (mean per binary; sign from DWARF DW_AT_encoding)", "",
                  "| binary | GT vars | unpaired | mean 0-6 | mean 0-5 | sign unjudged | " +
                  " | ".join(TREX_STEPS) + " |",
                  "|---|" + "---|" * (5 + len(TREX_STEPS))]
        for r in rows:
            t = r.get("trex")
            if not t:
                continue
            rates = " | ".join(f"{t['steps'][s]['passed']}/{t['steps'][s]['reached']}"
                               for s in TREX_STEPS)
            lines.append(f"| {Path(r['binary']).name} | {t['gt_variables']} | "
                         f"{t['unmatched_gt_variables']} | {t['mean']} | "
                         f"{t.get('mean_0_5')} | {t.get('sign_unjudged')} | {rates} |")
        lines += ["", "Where the score is lost — points, and the share of this "
                  "binary's loss. A first failure at step *i* costs `6 - i`, so "
                  "an early step is worth several late ones and "
                  "`points_possible - points_lost` is `mean x GT vars` exactly.", "",
                  "| binary | points lost | " +
                  " | ".join(f"{s} x{6 - i}" for i, s in enumerate(TREX_STEPS)) + " |",
                  "|---|---|" + "---|" * len(TREX_STEPS)]
        for r in rows:
            t = r.get("trex")
            if not t or not t.get("loss"):
                continue
            lost = t["points_lost"] or 1
            cells = " | ".join(
                f"{t['loss'][s]['points']} ({t['loss'][s]['points'] / lost:.0%})"
                for s in TREX_STEPS)
            lines.append(f"| {Path(r['binary']).name} | {t['points_lost']} | {cells} |")
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
        for channel, title in (
                ("offset_or_field",
                 "## struct-candidate census — OFFSET/FIELD evidence "
                 "(`*(T *)(B + K)` or `B->f`)"),
                ("index_only",
                 "## struct-candidate census — INDEX-ONLY evidence "
                 "(`B[k]` / `*B`; an array walk reads the same)")):
            lines += [title, "",
                      "| binary | fns | fns with a candidate | candidates | paired to GT | "
                      "already right | GT is struct* |", "|---|---|---|---|---|---|---|"]
            for r in rows:
                c = r.get("census")
                if not c:
                    continue
                ch = c["channels"][channel]
                lines.append(
                    f"| {Path(r['binary']).name} | {c['functions']} | "
                    f"{ch['functions_with_a_candidate']} | {ch['candidates']} | "
                    f"{ch['candidates_paired_with_ground_truth']} | "
                    f"{ch['candidates_kuna_already_types_correctly']} | "
                    f"{ch['candidates_whose_truth_is_a_struct_pointer']} |")
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

SELFTEST_DECL = """// Function: sub_4000 @ 0x4000
long sub_4000(long a0)
{
  char *v1;
  v1 = (char *)sub_5000();
  return *(int *)(v1 + 0x10);
}
"""

SELFTEST_INDEX = """// Function: sub_4600 @ 0x4600
int sub_4600(char *a0,unsigned int *a1,long a2)
{
  int v1;
  v1 = a0[0] + a0[1];
  v1 = v1 + a1[1] + a1[10];
  v1 = v1 + *(int *)(a2 - 8) + *(int *)(a2 + 8);
  v1 = v1 + dat_1000[1] + dat_1000[3];
  return v1;
}
"""

SELFTEST_ARROW = """// Function: sub_4300 @ 0x4300
void sub_4300(struct_0 *a0)
{
  a0->field_0x28 = 1;
  a0->next = 0;
}
"""


def _score(spelling: str, gt: dict, strict=False, gt_sign=None) -> int:
    return trex_score(trex_predicates(parse_type(spelling), gt, strict, gt_sign)[0])


def pairing_checks() -> list:
    """The pairing checks — skipped where decbench is not importable."""
    try:
        import decbench.metrics.type_match  # noqa: F401
    except ImportError:
        print("[skip] pairing checks (decbench not importable)")
        return []
    # Two variables share an offset and only the second one type-matches; the
    # metric's claim() takes that one, `free[0]` would take the first.
    tied = [{"name": "v1", "type": "int", "size": 4, "stack_offset": -0x10},
            {"name": "v2", "type": "char *", "size": 8, "stack_offset": -0x10}]
    tied_gt = [{"name": "s", "type": ["char *", "char*"], "rbp_offset": [-0x10],
                "size": 8, "is_arg": False, "arg_index": None}]
    # Per-function calibration prefers 8 (it aligns two offsets); the binary-wide
    # 0 aligns one, which under the metric's rule is enough to keep it.
    shifted = [{"name": "v1", "type": "int", "size": 4, "stack_offset": -0x10},
               {"name": "v2", "type": "int", "size": 4, "stack_offset": -0x20}]
    shifted_gt = [{"name": "at8", "type": ["int"], "rbp_offset": [-0x8], "size": 4,
                   "is_arg": False, "arg_index": None},
                  {"name": "at24", "type": ["int"], "rbp_offset": [-0x18], "size": 4,
                   "is_arg": False, "arg_index": None},
                  {"name": "at16", "type": ["int"], "rbp_offset": [-0x10], "size": 4,
                   "is_arg": False, "arg_index": None}]
    # One arg kuna gets right and one stack variable it calls `undefined8`
    # against a GT `char *`: the loss is one is_c_pointer failure, 5 points.
    payload = {"functions": [{"address": 0x1000, "variables": [
        {"name": "a0", "type": "char *", "size": 8, "arg_index": 0, "kind": "arg"},
        {"name": "v1", "type": "undefined8", "size": 8, "stack_offset": -0x10}]}]}
    gt_by_name = {"f": [
        {"name": "p", "type": ["char *", "char*"], "rbp_offset": [], "size": 8,
         "is_arg": True, "arg_index": 0},
        {"name": "q", "type": ["char *", "char*"], "rbp_offset": [-0x10], "size": 8,
         "is_arg": False, "arg_index": None}]}
    trex = measure_trex(payload, gt_by_name, {0x1000: "f"}, False,
                        signs_by_name={}, binary_shift=0)
    return [
        ("the loss model reconstructs the mean exactly",
         trex["points_possible"] - trex["points_lost"]
         == round(trex["mean"] * trex["gt_variables"])
         and trex["loss"]["is_c_pointer"] == {"failures": 1, "weight": 5, "points": 5}),
        ("claim() prefers the type-matching candidate over the first free one",
         pair_variables(tied, tied_gt) == {0: 1}),
        ("a binary-wide shift that aligns anything is kept",
         pair_variables(shifted, shifted_gt, 0) == {2: 0}
         and pair_variables(shifted, shifted_gt) == {2: 0}),
        ("the per-function shift overrides a binary-wide shift that aligns nothing",
         pair_variables(shifted, shifted_gt, 100) == {0: 0, 1: 1}),
    ]


def selftest() -> int:
    gt_u64 = {"ptr": 0, "base": "unsigned long", "kind": "primitive",
              "unsigned": True, "size": 8}
    scores = {spelling: _score(spelling, gt_u64, gt_sign=True)
              for spelling in ("char *", "struct_0", "double", "long", "unsigned long")}
    # Exactly the reviewer's case: decbench hands back a sign-stripped, sorted
    # form list for a DWARF `long unsigned int`. Reading the sign off that list
    # scored the WRONG spelling 6; with the sign from DWARF it scores 5.
    gt_stripped = gt_parse_forms(["long int", "long long", "long unsigned int"])
    header = header_layouts(SELFTEST_HEADER)
    census = census_function(SELFTEST_TEXT.split("// Function: ")[1])
    index_census = census_function(SELFTEST_INDEX.split("// Function: ")[1])["candidates"]
    checks = [
        # TRex Figure 6's own worked example: against an expected uint64_t the
        # types char*, struct{..}, double, int64_t, uint64_t score 1,3,4,5,6.
        ("Figure 6 example: char* scores 1", scores["char *"] == 1),
        ("Figure 6 example: a struct scores 3", scores["struct_0"] == 3),
        ("Figure 6 example: double scores 4", scores["double"] == 4),
        ("Figure 6 example: the signed twin scores 5", scores["long"] == 5),
        ("Figure 6 example: the exact type scores 6", scores["unsigned long"] == 6),
        ("an unmatched variable scores 0",
         trex_score(trex_predicates(None, gt_u64)[0]) == 0),
        ("undefined8 is defined but commits to no primitive",
         _score("undefined8", gt_u64, gt_sign=True) == 4),
        ("--strict-defined scores it 0",
         _score("undefined8", gt_u64, strict=True, gt_sign=True) == 0),
        ("a pointer level mismatch stops at step 2",
         _score("char **", {"ptr": 1, "base": "char", "kind": "primitive",
                            "unsigned": False, "size": 8}, gt_sign=False) == 2),
        ("the DWARF sign decides the last step, not the form list",
         (_score("unsigned long", gt_stripped, gt_sign=True),
          _score("long", gt_stripped, gt_sign=True)) == (6, 5)),
        ("an unknown sign caps the score at 5 and is counted",
         (lambda r: trex_score(r[0]) == 5 and r[1] is True)(
             trex_predicates(parse_type("unsigned long"), gt_stripped, False, None))),
        ("mean_0_5 is blind to the sign",
         trex_score(trex_predicates(parse_type("long"), gt_u64, False, True)[0][:5])
         == trex_score(trex_predicates(parse_type("unsigned long"), gt_u64,
                                       False, True)[0][:5]) == 5),
        ("an exactly right array scores 6, not 1",
         _score("char[3]", gt_parse_forms(["char", "char[3]"]), gt_sign=False) == 6),
        ("an array of pointers reads the same on both sides",
         _score("char *[2]", gt_parse_forms(["char*", "char*[2]"]), gt_sign=False) == 6),
        ("an array of a composite stays composite",
         _score("infomap[7]", gt_parse_forms(["infomap", "infomap[7]"])) == 6),
        ("a pointer where the truth is an array still fails IsCPointer",
         _score("char *", gt_parse_forms(["char", "char[3]"]), gt_sign=False) == 1),
        ("GT forms prefer the resolved primitive over the typedef name",
         gt_parse_forms(["size_t", "unsigned long"])["kind"] == "primitive"),
        ("GT pointer-to-struct is composite",
         (lambda r: r["ptr"] == 1 and r["kind"] == "struct")(gt_parse_forms(["Word*"]))),
        ("header offsets come from the field names",
         [f["offset"] for f in header["struct_0"]["fields"]] == [0, 8]),
        ("a struct-pointer member records its pointee",
         header["nest_1"]["fields"][0]["pointee"] == "struct_0"),
        ("a base with 3 distinct offsets is a candidate",
         census["candidates"].get("a0")
         == {"offsets": ["0x0", "0x8", "0xc"], "offset_or_field": True}),
        ("a declaration's star is not a dereference",
         census_function(SELFTEST_DECL.split("// Function: ")[1])["candidates"] == {}),
        ("an offset-named field keeps its own offset, a plain one its name",
         census_function(SELFTEST_ARROW.split("// Function: ")[1])["candidates"]
         == {"a0": {"offsets": ["0x28", ".next"], "offset_or_field": True}}),
        ("`B[k]` alone is index-only evidence, never offset/field",
         index_census["a0"] == {"offsets": ["0x0", "0x1"], "offset_or_field": False}),
        ("an index is scaled to a byte offset by the declared element size",
         index_census["a1"] == {"offsets": ["0x4", "0x28"], "offset_or_field": False}),
        ("an unsized base keeps index keys, so the two units never collide",
         index_census["dat_1000"] == {"offsets": ["[1]", "[3]"], "offset_or_field": False}),
        ("a negative constant displacement is an access too",
         index_census["a2"] == {"offsets": ["-0x8", "0x8"], "offset_or_field": True}),
        ("f(p) is p1*(1+p2*(1+...))", trex_score([True] * 6) == 6
         and trex_score([True, True, False, True, True, True]) == 2),
        ("a first failure at step i costs 6-i points",
         all(trex_score([True] * i + [False] * (6 - i)) == i for i in range(7))),
    ] + pairing_checks()
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
    ap.add_argument("--census", action="store_true",
                    help="struct-candidate census (>= 2 distinct offsets/fields)")
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
