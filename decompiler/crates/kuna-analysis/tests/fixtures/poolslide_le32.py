#!/usr/bin/env python3
"""Generate `poolslide_le32` — a stripped Thumb ELF with a literal pool word in
front of a function nothing calls.

No cross toolchain is needed; the ELF and its Thumb code are assembled here.
Regenerate with:

    python3 poolslide_le32.py

One R-X PT_LOAD at 0x8000 (file offset == vma - 0x8000). `.text` opens with a
Cortex-M vector table (initial SP, then the reset vector `_start`), so the image
is Thumb throughout:

  _start  bl leaf0 .. leaf21, bl user, bl taker, bl namer, b .
  leafN   movs r3,#N; adds r0,r0,r3; bx lr
  user    ldr r0,[pc,#0]; bx lr          (loads the pool word below)
  pool    .word 0x20000124               (an SRAM address; its high half reads
                                          as `movs r0,#0`)
  lone    adds r0,r0,r1; lsls r0,r0,#1; bx lr  (nothing calls it)
  taker   adr r1,h3; adds r1,#1; bx r1   (takes h3's address, loads nothing)
  h3      movs r3,#11; adds r0,r0,r3; bx lr
  namer   ldr r1,[pc,#4]; ldr r0,[pc,#8]; bx lr  (loads the two pool words below)
  pool2   .word name; .word 0x20000124
  name    "A0pG"                         (a string `namer` loads the address of)

The high half of the pool word followed by `lone`'s `adds` reads as the leaves'
`movs; adds` prologue, so a gap walk that probes it accepts it and claims `lone`
as its body; `lone`'s own `adds; lsls` matches no other function. `adr` names
`h3`'s first word without loading it, so that word is code, not a pool. After
`pool2` the high half of 0x20000124 and the string's bytes read as
`movs r0,#0; adds r0,#0x41; bx lr` — the leaves' prologue again — and the first
address after the pool is the string.
"""
import os
import struct

BASE = 0x8000
TEXT = 0x8100
CODE = TEXT + 16
LEAVES = 22
POOL_WORD = 0x20000124


def bl(at, target):
    off = target - (at + 4)
    assert -(1 << 24) <= off < (1 << 24) and off % 2 == 0
    s = (off >> 24) & 1
    i1, i2 = (off >> 23) & 1, (off >> 22) & 1
    j1, j2 = (~i1 ^ s) & 1, (~i2 ^ s) & 1
    hi = 0xF000 | (s << 10) | ((off >> 12) & 0x3FF)
    lo = 0xD000 | (j1 << 13) | (j2 << 11) | ((off >> 1) & 0x7FF)
    return [hi, lo]


def build():
    start_len = 4 * (LEAVES + 3) + 2
    leaf_at = [CODE + start_len + 6 * i for i in range(LEAVES)]
    user = leaf_at[-1] + 6
    pad = (4 - user % 4) % 4
    user += pad
    pool = user + 4
    lone = pool + 4
    taker = lone + 8
    h3 = taker + 8
    namer = h3 + 8
    name = namer + 16
    hw = []
    for t in leaf_at + [user, taker, namer]:
        hw += bl(CODE + 2 * len(hw), t)
    hw += [0xE7FE]
    for i in range(LEAVES):
        assert CODE + 2 * len(hw) == leaf_at[i]
        hw += [0x2300 | i, 0x18C0, 0x4770]
    hw += [0xBF00] * (pad // 2)
    assert CODE + 2 * len(hw) == user and user % 4 == 0
    hw += [0x4800, 0x4770]
    hw += [POOL_WORD & 0xFFFF, POOL_WORD >> 16]
    assert CODE + 2 * len(hw) == lone
    hw += [0x1840, 0x0040, 0x4770, 0xBF00]
    assert CODE + 2 * len(hw) == taker and taker % 4 == 0
    hw += [0xA100 | ((h3 - ((taker + 4) & ~3)) // 4), 0x3101, 0x4708, 0xBF00]
    assert CODE + 2 * len(hw) == h3
    hw += [0x230B, 0x18C0, 0x4770, 0xBF00]
    assert CODE + 2 * len(hw) == namer and namer % 4 == 0
    hw += [0x4901, 0x4802, 0x4770, 0xBF00]
    hw += [name & 0xFFFF, name >> 16, POOL_WORD & 0xFFFF, POOL_WORD >> 16]
    assert CODE + 2 * len(hw) == name
    hw += [0x3041, 0x4770, 0x0000, 0x0000]
    labels = dict(user=user, pool=pool, lone=lone, taker=taker, h3=h3, namer=namer, name=name)
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
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "poolslide_le32")
    with open(path, "wb") as f:
        f.write(elf(text))
    print(" ".join(f"{k}={v:#x}" for k, v in labels.items()))
