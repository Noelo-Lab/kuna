#!/usr/bin/env python3
"""Generate `thumbswitch_le32` — a stripped Thumb ELF whose switch dispatches
through `tbb`, with a case body shaped like a function.

No cross toolchain is needed; the ELF and its Thumb code are assembled here.
Regenerate with:

    python3 thumbswitch_le32.py

One R-X PT_LOAD at 0x8000 (file offset == vma - 0x8000). `.text` opens with a
Cortex-M vector table (initial SP, then the reset vector `_start`), so the image
is Thumb throughout:

  _start     bl leaf0 .. leaf21, bl dispatch, bl dispatch2, bl dispatch3, b .
  leafN      push {r4,lr}; movs r4,#N; adds r0,r0,r4; pop {r4,pc}
  dispatch   cmp r0,#3; bhi default; tbb [pc,r0]; a four-byte offset table
             case0: movs r0,#10; adds r0,#1; b join
             case1: push {r4,lr}; movs r4,#1; adds r0,r4,r0; pop {r4,pc}
             case2: movs r0,#30; b join
             case3: movs r0,#40
             join:  bx lr
             default: movs r0,#0; bx lr
  lone       push {r4,lr}; movs r4,#7; adds r0,r0,r4; pop {r4,pc}  (nothing calls it)
  dispatch2  cmp r0,#3; bcs default2; tbb [pc,r0]; three offsets and a zero pad
             c0: movs r0,#1; b join2
             c1: push {r4,lr}; movs r4,#2; adds r0,r4,r0; pop {r4,pc}
             c2: movs r0,#3
             join2: bx lr
             default2: movs r0,#0; bx lr
  dispatch3  cmp r0,#3; bge default3; tbb [pc,r0]; three offsets, then a byte
             that reads as an offset to lone3
             cases: movs r0,#5|#6|#7; bx lr
             default3: movs r0,#0; bx lr
  lone3      push {r4,lr}; movs r4,#9; adds r0,r0,r4; pop {r4,pc}  (nothing calls it)

`case1` and `c1` open with the leaves' prologue and end in their own return, so
a walk that cannot read the tables leaves them to the gap walk, which lists them
as functions. `bhi` keeps `r0 == 3` in range (four cases) and `bcs` does not
(three): reading a fourth entry of `dispatch2`'s table reads the zero pad.
`bge` is signed, so a negative index passes it and it bounds no table: reading
a fourth entry of `dispatch3`'s would make `lone3` one of its cases.
"""
import os
import struct

BASE = 0x8000
TEXT = 0x8100
CODE = TEXT + 16
LEAVES = 22


def bl(at, target):
    off = target - (at + 4)
    assert -(1 << 24) <= off < (1 << 24) and off % 2 == 0
    s = (off >> 24) & 1
    i1, i2 = (off >> 23) & 1, (off >> 22) & 1
    j1, j2 = (~i1 ^ s) & 1, (~i2 ^ s) & 1
    hi = 0xF000 | (s << 10) | ((off >> 12) & 0x3FF)
    lo = 0xD000 | (j1 << 13) | (j2 << 11) | ((off >> 1) & 0x7FF)
    return [hi, lo]


def b16(at, target):
    off = target - (at + 4)
    assert -2048 <= off < 2048 and off % 2 == 0
    return [0xE000 | ((off >> 1) & 0x7FF)]


def bhi(at, target):
    off = target - (at + 4)
    assert -256 <= off < 256 and off % 2 == 0
    return [0xD800 | ((off >> 1) & 0xFF)]


def leaf(n):
    return [0xB510, 0x2400 | n, 0x1900, 0xBD10]


def bcs(at, target):
    off = target - (at + 4)
    assert -256 <= off < 256 and off % 2 == 0
    return [0xD200 | ((off >> 1) & 0xFF)]


def bge(at, target):
    off = target - (at + 4)
    assert -256 <= off < 256 and off % 2 == 0
    return [0xDA00 | ((off >> 1) & 0xFF)]


def build():
    start_len = 4 * (LEAVES + 3) + 2
    leaf_at = [CODE + start_len + 8 * i for i in range(LEAVES)]
    dispatch = leaf_at[-1] + 8
    table = dispatch + 8
    case0 = table + 4
    case1 = case0 + 6
    case2 = case1 + 8
    case3 = case2 + 4
    join = case3 + 2
    default = join + 2
    lone = default + 4
    dispatch2 = lone + 8
    table2 = dispatch2 + 8
    c0 = table2 + 4
    c1 = c0 + 4
    c2 = c1 + 8
    join2 = c2 + 2
    default2 = join2 + 2
    dispatch3 = default2 + 4
    table3 = dispatch3 + 8
    cases3 = [table3 + 4 + 4 * i for i in range(3)]
    default3 = cases3[-1] + 4
    lone3 = default3 + 4
    hw = []
    for t in leaf_at + [dispatch, dispatch2, dispatch3]:
        hw += bl(CODE + 2 * len(hw), t)
    hw += [0xE7FE]
    assert CODE + 2 * len(hw) == leaf_at[0]
    for i in range(LEAVES):
        hw += leaf(i)
    assert CODE + 2 * len(hw) == dispatch
    hw += [0x2803] + bhi(dispatch + 2, default) + [0xE8DF, 0xF000]
    entries = [(c - table) // 2 for c in (case0, case1, case2, case3)]
    hw += [entries[0] | entries[1] << 8, entries[2] | entries[3] << 8]
    assert CODE + 2 * len(hw) == case0
    hw += [0x200A, 0x3001] + b16(case0 + 4, join)
    hw += [0xB510, 0x2401, 0x1820, 0xBD10]
    assert CODE + 2 * len(hw) == case2
    hw += [0x201E] + b16(case2 + 2, join)
    hw += [0x2028, 0x4770]
    hw += [0x2000, 0x4770]
    assert CODE + 2 * len(hw) == lone
    hw += [0xB510, 0x2407, 0x1900, 0xBD10]
    assert CODE + 2 * len(hw) == dispatch2
    hw += [0x2803] + bcs(dispatch2 + 2, default2) + [0xE8DF, 0xF000]
    entries = [(c - table2) // 2 for c in (c0, c1, c2)]
    hw += [entries[0] | entries[1] << 8, entries[2]]
    assert CODE + 2 * len(hw) == c0
    hw += [0x2001] + b16(c0 + 2, join2)
    hw += [0xB510, 0x2402, 0x1820, 0xBD10]
    assert CODE + 2 * len(hw) == c2
    hw += [0x2003, 0x4770]
    assert CODE + 2 * len(hw) == default2
    hw += [0x2000, 0x4770]
    assert CODE + 2 * len(hw) == dispatch3
    hw += [0x2803] + bge(dispatch3 + 2, default3) + [0xE8DF, 0xF000]
    entries = [(c - table3) // 2 for c in cases3] + [(lone3 - table3) // 2]
    hw += [entries[0] | entries[1] << 8, entries[2] | entries[3] << 8]
    for n, c in enumerate(cases3):
        assert CODE + 2 * len(hw) == c
        hw += [0x2005 + n, 0x4770]
    assert CODE + 2 * len(hw) == default3
    hw += [0x2000, 0x4770]
    assert CODE + 2 * len(hw) == lone3
    hw += [0xB510, 0x2409, 0x1900, 0xBD10]
    labels = dict(dispatch=dispatch, case1=case1, lone=lone, dispatch2=dispatch2, c1=c1,
                  dispatch3=dispatch3, lone3=lone3)
    vectors = struct.pack("<IIII", 0x20001000, CODE | 1, 0, 0)
    return vectors + b"".join(struct.pack("<H", h) for h in hw), labels


def elf(text):
    shstr = b"\0.text\0.shstrtab\0"
    text_off = TEXT - BASE
    shstr_off = text_off + len(text)
    shoff = (shstr_off + len(shstr) + 3) & ~3
    ehdr = struct.pack("<16sHHIIIIIHHHHHH",
                       b"\x7fELF\x01\x01\x01" + b"\0" * 9, 2, 40, 1, CODE | 1, 52, shoff,
                       0x05000200, 52, 32, 1, 40, 3, 2)
    phdr = struct.pack("<IIIIIIII", 1, 0, BASE, BASE, text_off + len(text),
                       text_off + len(text), 5, 0x1000)
    out = bytearray(ehdr + phdr)
    out += b"\0" * (text_off - len(out)) + text + shstr
    out += b"\0" * (shoff - len(out))
    out += b"\0" * 40
    out += struct.pack("<IIIIIIIIII", 1, 1, 6, TEXT, text_off, len(text), 0, 0, 2, 0)
    out += struct.pack("<IIIIIIIIII", 7, 3, 0, 0, shstr_off, len(shstr), 0, 0, 1, 0)
    return bytes(out)


if __name__ == "__main__":
    text, labels = build()
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "thumbswitch_le32")
    with open(path, "wb") as f:
        f.write(elf(text))
    print(" ".join(f"{k}={v:#x}" for k, v in labels.items()))
