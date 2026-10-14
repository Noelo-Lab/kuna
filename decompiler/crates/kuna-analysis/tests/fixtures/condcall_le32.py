#!/usr/bin/env python3
"""Generate `condcall_le32` — a stripped Thumb ELF with predicated calls.

No cross toolchain is needed; the ELF and its Thumb code are assembled here.
Regenerate with:

    python3 condcall_le32.py

One R-X PT_LOAD at 0x8000 (file offset == vma - 0x8000). `.text` opens with a
Cortex-M vector table (initial SP, then the reset vector `_start`), so the image
is Thumb throughout:

  _start  bl f1; bl f2; bl f3; bl other; b .
  fN      push {r7,lr}; bl ext; cmp r0,#N; it gt; blgt ext; pop {r7,pc}
  other   push {r4,lr}; bl ext; mov r4,r0; adds r0,#1; bl ext; add r0,r4; pop {r4,pc}
  ext     adds r0,#1; bx lr

`ext` returns. Each `blgt ext` lifts to a branch to the next instruction (the
skip when the condition fails) ahead of the call.
"""
import os
import struct

BASE = 0x8000
TEXT = 0x8100
CODE = TEXT + 16


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
    start_len = 4 * 4 + 2
    f_at = [CODE + start_len + 16 * i for i in range(3)]
    other = f_at[-1] + 16
    ext = other + 18
    hw = []
    for t in f_at + [other]:
        hw += bl(CODE + 2 * len(hw), t)
    hw += [0xE7FE]
    for n, at in enumerate(f_at, 1):
        assert CODE + 2 * len(hw) == at
        hw += [0xB580] + bl(at + 2, ext) + [0x2800 | n, 0xBFC8] + bl(at + 10, ext) + [0xBD80]
    assert CODE + 2 * len(hw) == other
    hw += [0xB510] + bl(other + 2, ext) + [0x4604, 0x3001] + bl(other + 10, ext) + [0x4420, 0xBD10]
    assert CODE + 2 * len(hw) == ext
    hw += [0x3001, 0x4770]
    labels = dict(f1=f_at[0], f2=f_at[1], f3=f_at[2], other=other, ext=ext)
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
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "condcall_le32")
    with open(path, "wb") as f:
        f.write(elf(text))
    print(" ".join(f"{k}={v:#x}" for k, v in labels.items()))
