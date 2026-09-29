#!/usr/bin/env python3
"""Generate `passthroughpair_x86_64` -- a minimal x86-64 ELF whose forwarders
hand back a result their callee returns in the register pair rdx:rax.

`passthrough` gives a function that only forwards its registers to a callee the
callee's parameters, and, when every RETURN hands back what a call left, the
callee's return value. That return value was taken only when it sat in one
register: `fwd_wide` below came out as `void fwd_wide(a0,a1) { wide(a0,a1); }`
while `fwd_narrow` got `return narrow(a0,a1);`.

No toolchain is needed; the ELF is assembled here byte by byte. Regenerate with:

    python3 passthroughpair_x86_64.py

Layout (one R-X PT_LOAD, .text at 0x401000):

  0x401000  wide:         mov rax,rdi; mov rdx,rsi; ret      result in rdx:rax
  0x401010  fwd_wide:     jmp wide
  0x401020  wide2:        rax = rdi+rsi; rdx = rsi+rdi; ret  both halves computed
  0x401030  fwd_wide2:    jmp wide2
  0x401040  narrow:       rax = rdi+rsi; ret                 result in rax
  0x401050  fwd_narrow:   jmp narrow
  0x401060  use_wide:     call fwd_wide; rax += rdx; ret     a caller of fwd_wide
  0x401080  callfwd_wide: call wide; ret                     the same without a tail jump
  0x4010a0  lofwd:        call wide; xor edx,edx; ret        writes rdx: no claim
  0x4010c0  notfwd:       call wide; rax += 1; ret           uses the result itself
"""

import os
import struct

BASE = 0x401000


def build_text():
    code = bytearray(b"\xcc" * 0xe0)

    def put(off, hexs):
        b = bytes.fromhex(hexs)
        code[off:off + len(b)] = b

    def rel(opc, off, tgt):
        code[off:off + 5] = bytes([opc]) + struct.pack("<i", tgt - (off + 5))

    put(0x00, "4889f84889f2c3")
    rel(0xE9, 0x10, 0x00)
    put(0x20, "4889f84801f04889f24801fac3")
    rel(0xE9, 0x30, 0x20)
    put(0x40, "4889f84801f0c3")
    rel(0xE9, 0x50, 0x40)
    put(0x60, "4883ec08"); rel(0xE8, 0x64, 0x10); put(0x69, "4801d04883c408c3")
    put(0x80, "4883ec08"); rel(0xE8, 0x84, 0x00); put(0x89, "4883c408c3")
    put(0xA0, "4883ec08"); rel(0xE8, 0xA4, 0x00); put(0xA9, "31d24883c408c3")
    put(0xC0, "4883ec08"); rel(0xE8, 0xC4, 0x00); put(0xC9, "4883c0014883c408c3")
    syms = {
        "wide": 0x00, "fwd_wide": 0x10, "wide2": 0x20, "fwd_wide2": 0x30,
        "narrow": 0x40, "fwd_narrow": 0x50, "use_wide": 0x60,
        "callfwd_wide": 0x80, "lofwd": 0xA0, "notfwd": 0xC0,
    }
    return code, syms


def build_elf(code, syms):
    strtab = b"\0" + b"".join(n.encode() + b"\0" for n in syms)
    symtab, o = bytearray(24), 1
    for n, a in syms.items():
        symtab += struct.pack("<IBBHQQ", o, 0x12, 0, 1, BASE + a, 8)
        o += len(n) + 1
    shstr = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    t_off = 0x1000
    s_off = t_off + len(code)
    st_off = s_off + len(symtab)
    sh_off_str = st_off + len(strtab)
    shoff = (sh_off_str + len(shstr) + 7) & ~7
    eh = b"\x7fELF\x02\x01\x01" + bytes(9) + struct.pack(
        "<HHIQQQIHHHHHH", 2, 62, 1, BASE, 64, shoff, 0, 64, 56, 1, 64, 5, 4)
    ph = struct.pack("<IIQQQQQQ", 1, 5, t_off, BASE, BASE, len(code), len(code), 0x1000)
    sh = [bytes(64),
          struct.pack("<IIQQQQIIQQ", 1, 1, 6, BASE, t_off, len(code), 0, 0, 16, 0),
          struct.pack("<IIQQQQIIQQ", 7, 2, 0, 0, s_off, len(symtab), 3, 1, 8, 24),
          struct.pack("<IIQQQQIIQQ", 15, 3, 0, 0, st_off, len(strtab), 0, 0, 1, 0),
          struct.pack("<IIQQQQIIQQ", 23, 3, 0, 0, sh_off_str, len(shstr), 0, 0, 1, 0)]
    out = bytearray(shoff + 64 * len(sh))
    out[0:64], out[64:120] = eh, ph
    out[t_off:t_off + len(code)] = code
    out[s_off:s_off + len(symtab)] = symtab
    out[st_off:st_off + len(strtab)] = strtab
    out[sh_off_str:sh_off_str + len(shstr)] = shstr
    out[shoff:] = b"".join(sh)
    return bytes(out)


def main():
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "passthroughpair_x86_64")
    with open(out, "wb") as f:
        f.write(build_elf(*build_text()))


if __name__ == "__main__":
    main()
