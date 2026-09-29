#!/usr/bin/env python3
"""Generate `passthroughpair_le32` -- a minimal ARM32 ELF whose forwarders hand
back a 64-bit result their callee returns in the register pair r1:r0.

The ARM twin of `passthroughpair_x86_64.py`: `passthrough` took a tail-called
callee's return value only when it sat in one register, so `fwd_wide` below
came out as `void fwd_wide(int a0,int a1) { wide(a0,a1); }` while `fwd_narrow`
got `return narrow(a0,a1);`.

No cross toolchain is needed; the ELF is assembled here byte by byte and the A32
bodies are hand-encoded. Regenerate with:

    python3 passthroughpair_le32.py

Layout (one R-X PT_LOAD, .text at 0x10000):

  0x10000  wide:       add r2,r0,r1; sub r1,r0,r1; mov r0,r2; bx lr   r1:r0
  0x10020  fwd_wide:   b wide
  0x10040  narrow:     add r0,r0,r1; bx lr                            r0
  0x10060  fwd_narrow: b narrow
  0x10080  inc64:      adds r0,r0,#1; adc r1,r1,#0; bx lr             r1:r0 in and out
  0x100a0  fwd_inc64:  b inc64
  0x100c0  use_wide:   push {r4,lr}; bl fwd_wide; add r0,r0,r1; pop {r4,pc}
"""

import os
import struct

BASE = 0x10000


def build_text():
    code = bytearray(0xe0)

    def w(off, *words):
        for i, x in enumerate(words):
            code[off + 4 * i:off + 4 * i + 4] = struct.pack("<I", x)

    def br(off, tgt, link=False):
        w(off, (0xEB000000 if link else 0xEA000000) | (((tgt - (off + 8)) >> 2) & 0xFFFFFF))

    w(0x00, 0xE0802001, 0xE0401001, 0xE1A00002, 0xE12FFF1E)
    br(0x20, 0x00)
    w(0x40, 0xE0800001, 0xE12FFF1E)
    br(0x60, 0x40)
    w(0x80, 0xE2900001, 0xE2A11000, 0xE12FFF1E)
    br(0xA0, 0x80)
    w(0xC0, 0xE92D4010); br(0xC4, 0x20, True); w(0xC8, 0xE0800001, 0xE8BD8010)
    syms = {
        "wide": 0x00, "fwd_wide": 0x20, "narrow": 0x40, "fwd_narrow": 0x60,
        "inc64": 0x80, "fwd_inc64": 0xA0, "use_wide": 0xC0,
    }
    return code, syms


def build_elf(code, syms):
    strtab = b"\0" + b"".join(n.encode() + b"\0" for n in syms)
    symtab, o = bytearray(16), 1
    for n, a in syms.items():
        symtab += struct.pack("<IIIBBH", o, BASE + a, 8, 0x12, 0, 1)
        o += len(n) + 1
    shstr = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    t_off = 0x1000
    s_off = t_off + len(code)
    st_off = s_off + len(symtab)
    sh_off_str = st_off + len(strtab)
    shoff = (sh_off_str + len(shstr) + 7) & ~7
    eh = b"\x7fELF\x01\x01\x01" + bytes(9) + struct.pack(
        "<HHIIIIIHHHHHH", 2, 40, 1, BASE, 52, shoff, 0x05000000, 52, 32, 1, 40, 5, 4)
    ph = struct.pack("<IIIIIIII", 1, t_off, BASE, BASE, len(code), len(code), 5, 0x1000)
    sh = [bytes(40),
          struct.pack("<IIIIIIIIII", 1, 1, 6, BASE, t_off, len(code), 0, 0, 16, 0),
          struct.pack("<IIIIIIIIII", 7, 2, 0, 0, s_off, len(symtab), 3, 1, 4, 16),
          struct.pack("<IIIIIIIIII", 15, 3, 0, 0, st_off, len(strtab), 0, 0, 1, 0),
          struct.pack("<IIIIIIIIII", 23, 3, 0, 0, sh_off_str, len(shstr), 0, 0, 1, 0)]
    out = bytearray(shoff + 40 * len(sh))
    out[0:52], out[52:84] = eh, ph
    out[t_off:t_off + len(code)] = code
    out[s_off:s_off + len(symtab)] = symtab
    out[st_off:st_off + len(strtab)] = strtab
    out[sh_off_str:sh_off_str + len(shstr)] = shstr
    out[shoff:] = b"".join(sh)
    return bytes(out)


def main():
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "passthroughpair_le32")
    with open(out, "wb") as f:
        f.write(build_elf(*build_text()))


if __name__ == "__main__":
    main()
