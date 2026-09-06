#!/usr/bin/env python3
"""Generate `picpool_arm_le32` — a minimal position-independent A32 ARM image
whose only reference to each literal is the `ldr rX,[pool] ; add rX,pc,rX` pair,
plus the shape the composition must REFUSE.

No ARM cross toolchain is needed (and none is installed on this host), so the ELF
is assembled here byte by byte and the A32 bodies are hand-encoded. Regenerate
with:

    python3 picpool_arm_le32.py

The whole image is mapped LOW — under 0x1000, exactly as the filing Android PIE
(crackmes.one 68d40081224c0ec5dcedc2d2, `.text` at 0x4c4) is. That is not
decoration: `ScalarOperandAnalyzer.checkOperands`' "below 4096 could be a number"
floor rejects every address in such an image, so a composition that applied the
floor to its result would report nothing here.

Layout:

  .rodata @0x00000300  SHF_ALLOC                 (read-only)
      0x00000300 PROMPT   "kuna picpool prompt"   <- reached by an adjacent pair
      0x00000320 SECOND   "kuna picpool second"   <- reached by a scheduled pair
      0x00000340 NUMBER   "kuna picpool number"   <- reached by nothing
  .text   @0x00000400  SHF_ALLOC|SHF_EXECINSTR   (read-only)
      0x00000400 _start        push {lr} ; bl x3 ; pop {pc}
      0x00000420 uses_prompt   ldr r0,[0x42c] ; add r0,pc,r0 ; bx lr
      0x0000042c               .word PROMPT - 0x42c          <- FOLLOWED
      0x00000430 scheduled     ldr r0,[0x444] ; mov ; mov ; add r0,pc,r0 ; bx lr
      0x00000444               .word SECOND - 0x444          <- FOLLOWED
      0x00000448 no_pc         ldr r0,[0x454] ; add r0,r0,#4 ; bx lr
      0x00000454               .word NUMBER - 4              <- refused: no PC

`uses_prompt` is the whole defect in eight bytes: the address of PROMPT is in
neither instruction and in the pool word least of all — the word is the signed
distance -0x12c, which is not an address and lands in no section, so following
it as a pointer (`kuna_poolref`) correctly declines and the literal ends up
referenced by nothing.

`scheduled` is the same pair with two instructions between the load and the add,
which is what instruction scheduling does when a function forms several
references at once.

`no_pc` is the guard: its pool word plus four IS a mapped address, so the only
thing separating it from a real reference is that the program never added the
PC to it. A composition that folded any arithmetic would report it.
"""
import struct, os

RODATA_VMA = 0x00000300
TEXT_VMA = 0x00000400

SHF_WRITE, SHF_ALLOC, SHF_EXECINSTR = 0x1, 0x2, 0x4
PF_X, PF_W, PF_R = 0x1, 0x2, 0x4

BX_LR = 0xE12FFF1E
PUSH_LR = 0xE52DE004          # str lr,[sp,#-4]!
POP_PC = 0xE49DF004           # ldr pc,[sp],#4
MOV_R1_1 = 0xE3A01001         # mov r1,#1
MOV_R2_2 = 0xE3A02002         # mov r2,#2


def ldr_pc(at, rd, slot):
    """`ldr rd,[pc,#imm12]` — the A32 literal load; the base is `at + 8`."""
    off = slot - (at + 8)
    assert 0 <= off < 4096 and off % 4 == 0, (hex(at), hex(slot), off)
    return 0xE59F0000 | (rd << 12) | off


def add_pc(rd):
    """`add rd,pc,rd` — the other half of the pair; `pc` reads as `at + 8`."""
    return 0xE08F0000 | (rd << 12) | rd


def add_imm(rd, rn, imm):
    """`add rd,rn,#imm` — the same arithmetic with no PC in it."""
    assert 0 <= imm < 256
    return 0xE2800000 | (rn << 16) | (rd << 12) | imm


def bl(at, target):
    off = (target - (at + 8)) >> 2
    return 0xEB000000 | (off & 0xFFFFFF)


def w(*words):
    return b''.join(struct.pack('<I', x) for x in words)


PROMPT = RODATA_VMA
SECOND = RODATA_VMA + 0x20
NUMBER = RODATA_VMA + 0x40

START = TEXT_VMA
USES_PROMPT = TEXT_VMA + 0x20
POOL_PROMPT = TEXT_VMA + 0x2C
SCHEDULED = TEXT_VMA + 0x30
POOL_SECOND = TEXT_VMA + 0x44
NO_PC = TEXT_VMA + 0x48
POOL_NUMBER = TEXT_VMA + 0x54
TEXT_END = TEXT_VMA + 0x58


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    t += w(PUSH_LR)
    for target in (USES_PROMPT, SCHEDULED, NO_PC):
        t += w(bl(at(), target))
    t += w(POP_PC)
    t += bytes(USES_PROMPT - at())

    # The pair, adjacent: `add`'s PC is POOL_PROMPT, so the word is the signed
    # distance from there to the literal.
    assert at() == USES_PROMPT, hex(at())
    t += w(ldr_pc(at(), 0, POOL_PROMPT), add_pc(0), BX_LR)
    assert at() == POOL_PROMPT, hex(at())
    t += w((PROMPT - POOL_PROMPT) & 0xFFFFFFFF)

    # The same pair with the load scheduled away from the add.
    assert at() == SCHEDULED, hex(at())
    t += w(ldr_pc(at(), 0, POOL_SECOND), MOV_R1_1, MOV_R2_2, add_pc(0), BX_LR)
    assert at() == POOL_SECOND, hex(at())
    t += w((SECOND - POOL_SECOND) & 0xFFFFFFFF)

    # Arithmetic on the same word with no PC in it.
    assert at() == NO_PC, hex(at())
    t += w(ldr_pc(at(), 0, POOL_NUMBER), add_imm(0, 0, 4), BX_LR)
    assert at() == POOL_NUMBER, hex(at())
    t += w(NUMBER - 4)

    assert at() == TEXT_END, hex(at())
    return bytes(t)


def build_rodata():
    r = bytearray(0x60)
    r[0x00:] = b'kuna picpool prompt\0'.ljust(0x20, b'\0')
    r[0x20:] = b'kuna picpool second\0'.ljust(0x20, b'\0')
    r[0x40:] = b'kuna picpool number\0'.ljust(0x20, b'\0')
    return bytes(r[:0x60])


TEXT = build_text()
RODATA = build_rodata()

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 1, 4  # one PT_LOAD r-x; null/.rodata/.text/.shstrtab


def build():
    ph_off = EHDR
    rodata_off = RODATA_VMA
    text_off = TEXT_VMA

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text', '.rodata'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = text_off + len(TEXT)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', START, ph_off, sh_off)
    b += struct.pack('<I', 0x05000000)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    # One segment, mapped from file offset 0 at virtual address 0 — the layout a
    # PIE built for Android carries, and the reason every address here is small.
    text_end = TEXT_VMA + len(TEXT)
    b += struct.pack('<IIIIIIII', 1, 0, 0, 0, text_end, text_end, PF_R | PF_X, 4)
    assert len(b) == ph_off + PHDR * NPH
    b += bytes(rodata_off - len(b))
    b += RODATA
    b += bytes(text_off - len(b))
    b += TEXT
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.rodata'], 1, SHF_ALLOC, RODATA_VMA, rodata_off, len(RODATA))
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(TEXT))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'picpool_arm_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
