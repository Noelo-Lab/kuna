#!/usr/bin/env python3
"""Generate `cortexm_ptrentry_le32` — a stripped bare-metal ARM Cortex-M image
carrying, in one file, both halves of the `ptrentry` contract: a frameless leaf
that ONLY a `.rodata` function pointer reaches, and a switch-table-shaped pointer
whose target must stay undiscovered.

No cross toolchain is needed (and none on this host emits a bare-metal STM32 link
layout), so the ELF is assembled here byte by byte and the Thumb bodies are
hand-encoded. Regenerate with:

    python3 cortexm_ptrentry_le32.py

Layout:

  PT_LOAD [0x08000000, 0x08000060)  PF_R|PF_X
  PT_LOAD [0x08000060, 0x08000068)  PF_R

  .text  @0x08000000  SHF_ALLOC|SHF_EXECINSTR
      0x08000000 [0] 0x20001000     initial SP (SRAM)         \\ the vector table
      0x08000004 [1] 0x08000041     reset vector == e_entry   / the shipped oracle
      0x08000008 [2] 0x08000041         confirms, so the image decodes as Thumb
      0x0800000c [3] 0x08000041         with no `cortexmvectors` involvement
      0x08000010 [4..15] 0          reserved slots
      0x08000040 reset  push {r7,lr} ; bl 0x08000050 ; pop {r7,pc}
      0x08000048 LEAF   movs r0,#7 ; bx lr        <- pointer-only, no frame, 4 bytes
      0x0800004c        0                          (alignment pad)
      0x08000050 callee push {r7,lr} ; movs r0,#3 ; pop {r7,pc}
      0x08000058        0x0800005d               <- switch-table-shaped pointer:
      0x0800005c SWCASE movs r0,#9 ; bx lr          site and target share `callee`

  .rodata @0x08000060  SHF_ALLOC
      0x08000060 0x08000049    <- the fops-struct pointer at LEAF | Thumb bit
      0x08000064 0

`LEAF` is invisible to every shipped discovery stage: no `BL` reaches it, its
first halfword is not a Thumb prologue (so Stage 2's `PUSH {..,lr}` scan and
Stage 3's frame-prologue gate both skip it), and it is two instructions long (so
`check_valid_subroutine`'s `MIN_SUBROUTINE_INSNS` would reject it anyway).
`SWCASE` is identical in shape and is reachable by a code-pointer word too — the
difference is that its pointer lies in the same discovered function as itself,
which is the `ldr pc,[pc,r]` switch table, so `ptrentry` must leave it alone.
"""
import struct, os

TEXT_VMA = 0x08000000
RODATA_VMA = 0x08000060
E_ENTRY = TEXT_VMA + 0x41  # the reset vector, Thumb-odd

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_R = 0x1, 0x4

VECTORS = [0x20001000, E_ENTRY, E_ENTRY, E_ENTRY] + [0] * 12


def thumb_ret_const(k):
    """`movs r0,#k ; bx lr` (Thumb-1: 0x20kk, 0x4770)."""
    return struct.pack('<HH', 0x2000 | (k & 0xFF), 0x4770)


def bl(at, target):
    """`BL <target>` (Thumb-2 T1) encoded at `at`, for a small positive offset."""
    off = target - (at + 4)
    assert 0 <= off < (1 << 22) and off % 2 == 0, off
    imm11 = (off >> 1) & 0x7FF
    imm10 = (off >> 12) & 0x3FF
    return struct.pack('<HH', 0xF000 | imm10, 0xF800 | imm11)


TEXT = bytearray()
TEXT += b''.join(struct.pack('<I', w) for w in VECTORS)          # 0x00..0x3f
assert len(TEXT) == 0x40
TEXT += struct.pack('<H', 0xB580)                                 # 0x40 push {r7,lr}
TEXT += bl(TEXT_VMA + 0x42, TEXT_VMA + 0x50)                      # 0x42 bl callee
TEXT += struct.pack('<H', 0xBD80)                                 # 0x46 pop {r7,pc}
assert len(TEXT) == 0x48
TEXT += thumb_ret_const(7)                                        # 0x48 LEAF
TEXT += struct.pack('<I', 0)                                      # 0x4c pad
TEXT += struct.pack('<HHH', 0xB580, 0x2003, 0xBD80)               # 0x50 callee
TEXT += struct.pack('<H', 0)                                      # 0x56 pad
TEXT += struct.pack('<I', TEXT_VMA + 0x5D)                        # 0x58 switch slot
TEXT += thumb_ret_const(9)                                        # 0x5c SWCASE
assert len(TEXT) == 0x60, hex(len(TEXT))

RODATA = struct.pack('<II', TEXT_VMA + 0x49, 0)

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 2, 4  # 2 PT_LOAD; null/.text/.rodata/.shstrtab


def build():
    ph_off = EHDR
    text_off = ph_off + PHDR * NPH
    rodata_off = text_off + len(TEXT)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text', '.rodata'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = rodata_off + len(RODATA)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    def phdr(off, vaddr, size, flags):
        return struct.pack('<IIIIIIII', 1, off, vaddr, vaddr, size, size, flags, 4)

    b += phdr(text_off, TEXT_VMA, len(TEXT), PF_R | PF_X)
    b += phdr(rodata_off, RODATA_VMA, len(RODATA), PF_R)
    assert len(b) == text_off
    b += TEXT
    b += RODATA
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(TEXT))
    b += shdr(names['.rodata'], 1, SHF_ALLOC, RODATA_VMA, rodata_off, len(RODATA))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_ptrentry_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
