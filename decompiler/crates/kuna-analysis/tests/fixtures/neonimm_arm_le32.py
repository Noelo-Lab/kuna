#!/usr/bin/env python3
"""Generate `neonimm_arm_le32` -- an A32 ARM image that fills a stack buffer
with two NEON immediates and hands its address to a callee.

The shape a stripped ARM PIE's `_start_main` has: one `vmov.i32 qN,#imm`
followed by a `vst1.64` writeback store, which is how a compiler zeroes the
structure it passes to `__libc_init`. The immediate is a DYNAMIC constant-space
export in SLEIGH (`ARMneon.sinc:549`, `tmp:16 = 0; export *[const]:16 tmp`), so
it lowers to a LOAD through a pointer that no rule folds, and `ActionLaneDivide`
then slices that LOAD into per-word reads at `0`, `0+4`, `0+8`, `0+0xc`.

Both halves are here on purpose. `vmov.i32 q9,#0` is the filing witness and
renders as the near-null reads; `vmov.i32 q8,#1` is the case a fold that simply
widened `RuleLoadVarnode` would silently TRUNCATE, because a 16-byte constant
does not fit the `uintb` a Varnode offset is. A fixture carrying only the zero
cannot tell the two fixes apart.

No cross toolchain is needed; the ELF is assembled here byte by byte and the
A32 bodies are hand-encoded (checked against `llvm-mc -arch=arm -mattr=+neon`).
Regenerate with:

    python3 neonimm_arm_le32.py

Layout:

  PT_LOAD [0x10000, 0x10038)  PF_R|PF_X

  .text @0x10000  SHF_ALLOC|SHF_EXECINSTR, e_entry = 0x10004
      0x10000  bx lr                      <- the callee, so the buffer escapes
      0x10004  push {r11,lr}              <- neon_init
      0x10008  mov r11,sp
      0x1000c  sub sp,sp,#32
      0x10010  vmov.i32 q8,#1             <- the NON-ZERO immediate
      0x10014  mov r1,sp
      0x10018  vst1.64 {d16,d17},[r1]!
      0x1001c  vmov.i32 q9,#0             <- the filing witness
      0x10020  vst1.64 {d18,d19},[r1]!
      0x10024  mov r0,sp
      0x10028  bl 0x10000
      0x1002c  mov sp,r11
      0x10030  pop {r11,pc}

The call is what keeps the buffer alive: without a consumer the whole
initialization is dead code and neither rendering survives to the output.
"""
import struct, os

TEXT_VMA = 0x10000
SINK = TEXT_VMA
NEON_INIT = TEXT_VMA + 4
E_ENTRY = NEON_INIT

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
SHT_PROGBITS, SHT_STRTAB = 1, 3
PF_X, PF_R = 0x1, 0x4


def bl(at, target):
    """`bl target` (A32): the PC base is `at + 8`."""
    off = (target - (at + 8)) >> 2
    return 0xEB000000 | (off & 0xFFFFFF)


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    def w(word):
        t.extend(struct.pack('<I', word))

    assert at() == SINK, hex(at())
    w(0xE12FFF1E)  # bx lr
    assert at() == NEON_INIT, hex(at())
    w(0xE92D4800)  # push {r11,lr}
    w(0xE1A0B00D)  # mov r11,sp
    w(0xE24DD020)  # sub sp,sp,#32
    w(0xF2C00051)  # vmov.i32 q8,#1
    w(0xE1A0100D)  # mov r1,sp
    w(0xF4410ACD)  # vst1.64 {d16,d17},[r1]!
    w(0xF2C02050)  # vmov.i32 q9,#0
    w(0xF4412ACD)  # vst1.64 {d18,d19},[r1]!
    w(0xE1A0000D)  # mov r0,sp
    w(bl(at(), SINK))
    w(0xE1A0D00B)  # mov sp,r11
    w(0xE8BD8800)  # pop {r11,pc}
    return bytes(t)


TEXT = build_text()

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 1, 3  # one PT_LOAD; null/.text/.shstrtab


def build():
    ph_off = EHDR
    text_off = ph_off + PHDR * NPH

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = text_off + len(TEXT)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 2)

    b += struct.pack('<IIIIIIII', 1, text_off, TEXT_VMA, TEXT_VMA,
                     len(TEXT), len(TEXT), PF_R | PF_X, 4)
    assert len(b) == text_off
    b += TEXT
    assert len(b) == shstr_off
    b += shstr
    assert len(b) == sh_off

    def shdr(name, typ, flags, addr, off, size, align=1):
        return struct.pack('<IIIIIIIIII', name, typ, flags, addr, off, size, 0, 0, align, 0)

    b += shdr(0, 0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR,
              TEXT_VMA, text_off, len(TEXT), 4)
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'neonimm_arm_le32')
    with open(out, 'wb') as fh:
        fh.write(build())
    os.chmod(out, 0o755)
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
