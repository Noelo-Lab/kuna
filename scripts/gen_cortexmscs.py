#!/usr/bin/env python3
"""Generate the Cortex-M System Control Space register tables.

Source: Arm CMSIS-Core(M) device headers, CMSIS_5 release 5.9.0 (Apache-2.0),
https://github.com/ARM-software/CMSIS_5/tree/5.9.0/CMSIS/Core/Include
  core_cm7.h   the ARMv7-M layout (a superset of core_cm3.h and core_cm4.h)
  core_cm33.h  the ARMv8-M Mainline layout
The Flash Patch and Breakpoint unit has no CMSIS structure; its three register
kinds are stated here from the ARMv7-M Architecture Reference Manual (DDI 0403,
C1.11) and the ARMv8-M one (DDI 0553, D1.2 FP_CTRL / FP_REMAP / FP_COMPn).

Usage: gen_cortexmscs.py DIR_WITH_HEADERS > kuna_cortexmscs_tables.rs

Every `#define NAME ((NAME_Type *) NAME_BASE)` block below 0xE0100000 is laid
out with natural alignment and checked against the header's `Offset:` comments.
CMSIS structures overlap (NVIC_Type reaches STIR at 0xE000EF00, past SCB), so
the registers are merged by address: a block keeps the registers up to the
first one that lies beyond another block's base, and each register after that
becomes its own global `<Block>_<Register>`. A block's global starts at its
first register, not at a reserved word before it (0xE000E000 itself, the base
of SCnSCB, is a constant code compares against). A register two blocks both name
belongs to the block listed first in ORDER. An anonymous union is its first
member, and a conditional member takes its `#if` branch.
"""
import re
import sys
from pathlib import Path

LAYOUTS = [("V7M", "core_cm7.h"), ("V8M", "core_cm33.h")]
ORDER = ["FPU", "SCB", "MPU", "SAU", "DCB", "DIB", "CoreDebug", "SysTick", "NVIC", "SCnSCB", "ITM", "DWT", "FPB", "TPI"]
FPB = (0xE0002000, [("CTRL", 0x0, 4, 1), ("REMAP", 0x4, 4, 1), ("COMP", 0x8, 4, 8)])
SIZES = {"uint32_t": 4, "uint16_t": 2, "uint8_t": 1}
QUALIFIERS = {"__IM", "__OM", "__IOM", "__I", "__O", "__IO", "volatile", "const"}
PPB_END = 0xE0100000


def number(text, defines):
    total = 0
    for term in text.replace("(", " ").replace(")", " ").split("+"):
        term = term.strip()
        if term in defines:
            total += defines[term]
        else:
            total += int(term.rstrip("UL"), 0)
    return total


def statements(body):
    depth, start = 0, 0
    for i, ch in enumerate(body):
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        elif ch == ";" and depth == 0:
            yield body[start:i].strip()
            start = i + 1


def member(stmt):
    """(name, element size, count) of one structure member."""
    inner = re.search(r"\{(.*)\}", stmt, re.S)
    if inner:
        inside = [member(s) for s in statements(inner.group(1))]
        size = max(m[1] for m in inside)
        tail = stmt[inner.end():]
        if not tail.strip():
            return inside[0]
    else:
        words = [w for w in re.split(r"\s+", stmt.split("[")[0]) if w and w not in QUALIFIERS]
        size, tail = SIZES[words[0]], stmt[stmt.index(words[0]) + len(words[0]):]
    m = re.match(r"\s*(\w+)\s*(?:\[\s*(\w+)\s*\])?\s*$", tail)
    return m.group(1), size, int(m.group(2).rstrip("UL"), 0) if m.group(2) else 1


def structure(body):
    stated = {m.group(1): int(m.group(2), 16) for m in re.finditer(r"(\w+)\s*(?:\[\w+\])?;\s*/\*!< Offset: (0x[0-9A-Fa-f]+)", body)}
    body = re.sub(r"/\*.*?\*/", "", body, flags=re.S)
    body = re.sub(r"#else.*?#endif", "", body, flags=re.S)
    body = re.sub(r"^\s*#.*$", "", body, flags=re.M)
    fields, at = [], 0
    for stmt in statements(body):
        name, size, count = member(stmt)
        at = (at + size - 1) // size * size
        if name in stated and stated[name] != at:
            raise SystemExit(f"{name}: laid out at {at:#x}, header says {stated[name]:#x}")
        if not re.match(r"RE?SERVED", name.upper()):
            fields.append((name, at, size, count))
        at += size * count
    return fields


def blocks(text):
    defines = {}
    for m in re.finditer(r"#define\s+(\w+_BASE)\s+\(([^)]*(?:\([^)]*\))?[^)]*)\)", text):
        try:
            defines.setdefault(m.group(1), number(m.group(2), defines))
        except ValueError:
            pass
    types = {m.group(2): structure(m.group(1)) for m in re.finditer(r"typedef struct\s*\{(.*?)\}\s*(\w+_Type)\s*;", text, re.S)}
    out = {}
    for m in re.finditer(r"#define\s+(\w+)\s+\(\((\w+_Type)\s*\*\)\s*(\w+_BASE)\s*\)", text):
        name, ty, base = m.groups()
        if name.endswith("_NS") or defines.get(base, PPB_END) >= PPB_END or name in out:
            continue
        out[name] = (defines[base], types[ty])
    out["FPB"] = FPB
    return out


def merge(found):
    """Split the blocks into non-overlapping globals: (name, type, base, fields)."""
    owner = {}
    for name in ORDER:
        if name not in found:
            continue
        base, fields = found[name]
        for f in fields:
            lo, hi = base + f[1], base + f[1] + f[2] * f[3]
            if all(hi <= a or lo >= b for (a, b) in owner):
                owner[(lo, hi)] = (name, f)
    missing = sorted(set(found) - set(ORDER))
    if missing:
        raise SystemExit(f"blocks not in ORDER: {missing}")
    bases = sorted(found[n][0] for n in found)
    out = []
    for name in ORDER:
        if name not in found:
            continue
        base = found[name][0]
        limit = min([b for b in bases if b > base], default=PPB_END)
        mine = sorted((lo, f) for (lo, _), (n, f) in owner.items() if n == name)
        head = [(f[0], lo, f[2], f[3]) for lo, f in mine if lo + f[2] * f[3] <= limit]
        if head:
            start = head[0][1]
            out.append((name, f"{name}_Type", start, [(n, lo - start, size, count) for n, lo, size, count in head]))
        for lo, f in mine:
            if lo + f[2] * f[3] > limit:
                out.append((f"{name}_{f[0]}", "", lo, [(f[0], 0, f[2], f[3])]))
    return sorted(out, key=lambda b: b[2])


def main():
    root = Path(sys.argv[1])
    print("// Generated by scripts/gen_cortexmscs.py from CMSIS_5 5.9.0 (Apache-2.0); do not edit.")
    print()
    print("use super::Block;")
    print()
    for const, header in LAYOUTS:
        text = (root / header).read_text()
        print(f"/// From `{header}`.")
        print(f"pub(super) const {const}: &[Block] = &[")
        for name, ty, base, fields in merge(blocks(text)):
            cells = ", ".join(f'("{n}", {off:#x}, {size}, {count})' for n, off, size, count in fields)
            print(f'    Block {{ name: "{name}", type_name: "{ty}", base: {base:#010x}, fields: &[{cells}] }},')
        print("];")
        if const != LAYOUTS[-1][0]:
            print()


if __name__ == "__main__":
    main()
