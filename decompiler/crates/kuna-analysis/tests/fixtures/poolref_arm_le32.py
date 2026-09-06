#!/usr/bin/env python3
"""Generate `poolref_arm_le32` — a minimal A32 ARM image whose only reference to
each of its literals goes through a PC-relative literal pool, plus the three
shapes the follow must REFUSE.

No ARM cross toolchain is needed (and none is installed on this host), so the ELF
is assembled here byte by byte and the A32 bodies are hand-encoded. Regenerate
with:

    python3 poolref_arm_le32.py

Layout:

  .text   @0x00010000  SHF_ALLOC|SHF_EXECINSTR   (read-only)
      0x00010000 _start          push {lr} ; bl x4 ; pop {pc}
      0x00010020 uses_prompt     ldr  r0,[POOL_PROMPT] ; bx lr
      0x00010028 POOL_PROMPT     .word PROMPT           <- FOLLOWED
      0x0001002c reads_slot      ldr  r0,[SLOT]        ; bx lr
      0x00010034 narrow_read     ldrh r0,[POOL_NARROW] ; bx lr
      0x0001003c POOL_NARROW     .word NARROW           <- refused: 2-byte read
      0x00010040 reads_number    ldr  r0,[POOL_NUMBER] ; bx lr
      0x00010048 POOL_NUMBER     .word 42               <- refused: not an address
  .rodata @0x00010100  SHF_ALLOC                 (read-only)
      PROMPT / NARROW / HIDDEN, three NUL-terminated literals
  .data   @0x00010180  SHF_ALLOC|SHF_WRITE
      0x00010180 SLOT            .word HIDDEN           <- refused: writable

`uses_prompt` is the whole defect in four bytes: the address of PROMPT occurs
nowhere in any instruction, only in the pool word, so a scan over decode-time
constants reports the literal as referenced by nothing. The other three are the
guards — a narrow load is reading a number out of a pool, a small word is a
number however well it lands, and a writable slot holds whatever the loader or
the program last put there, not what the image says.
"""
import struct, os

TEXT_VMA = 0x00010000
RODATA_VMA = 0x00010100
DATA_VMA = 0x00010180

SHF_WRITE, SHF_ALLOC, SHF_EXECINSTR = 0x1, 0x2, 0x4
PF_X, PF_W, PF_R = 0x1, 0x2, 0x4

BX_LR = 0xE12FFF1E
PUSH_LR = 0xE52DE004          # str lr,[sp,#-4]!
POP_PC = 0xE49DF004           # ldr pc,[sp],#4


def ldr_pc(at, rd, target):
    """`ldr rd,[pc,#imm12]` — the A32 literal load; the base is `at + 8`."""
    off = target - (at + 8)
    assert 0 <= off < 4096 and off % 4 == 0, (hex(at), hex(target), off)
    return 0xE59F0000 | (rd << 12) | off


def ldrh_pc(at, rd, target):
    """`ldrh rd,[pc,#imm8]` — the same load two bytes wide."""
    off = target - (at + 8)
    assert 0 <= off < 256, (hex(at), hex(target), off)
    return 0xE1DF00B0 | (rd << 12) | ((off & 0xF0) << 4) | (off & 0xF)


def bl(at, target):
    off = (target - (at + 8)) >> 2
    return 0xEB000000 | (off & 0xFFFFFF)


def w(*words):
    return b''.join(struct.pack('<I', x) for x in words)


PROMPT = RODATA_VMA
NARROW = RODATA_VMA + 0x20
HIDDEN = RODATA_VMA + 0x40

START = TEXT_VMA
USES_PROMPT = TEXT_VMA + 0x20
POOL_PROMPT = TEXT_VMA + 0x28
READS_SLOT = TEXT_VMA + 0x2C
NARROW_READ = TEXT_VMA + 0x34
POOL_NARROW = TEXT_VMA + 0x3C
READS_NUMBER = TEXT_VMA + 0x40
POOL_NUMBER = TEXT_VMA + 0x48
TEXT_END = TEXT_VMA + 0x4C


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    t += w(PUSH_LR)
    for target in (USES_PROMPT, READS_SLOT, NARROW_READ, READS_NUMBER):
        t += w(bl(at(), target))
    t += w(POP_PC)
    t += bytes(USES_PROMPT - at())

    assert at() == USES_PROMPT, hex(at())
    t += w(ldr_pc(at(), 0, POOL_PROMPT), BX_LR)
    assert at() == POOL_PROMPT, hex(at())
    t += w(PROMPT)

    assert at() == READS_SLOT, hex(at())
    t += w(ldr_pc(at(), 0, DATA_VMA), BX_LR)

    assert at() == NARROW_READ, hex(at())
    t += w(ldrh_pc(at(), 0, POOL_NARROW), BX_LR)
    t += bytes(POOL_NARROW - at())
    assert at() == POOL_NARROW, hex(at())
    t += w(NARROW)

    assert at() == READS_NUMBER, hex(at())
    t += w(ldr_pc(at(), 0, POOL_NUMBER), BX_LR)
    assert at() == POOL_NUMBER, hex(at())
    t += w(42)

    assert at() == TEXT_END, hex(at())
    return bytes(t)


def build_rodata():
    r = bytearray(0x60)
    r[0x00:] = b'kuna poolref prompt\0'.ljust(0x20, b'\0')
    r[0x20:] = b'kuna poolref narrow\0'.ljust(0x20, b'\0')
    r[0x40:] = b'kuna poolref hidden\0'.ljust(0x20, b'\0')
    return bytes(r[:0x60])


TEXT = build_text()
RODATA = build_rodata()
DATA = w(HIDDEN)

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 2, 5  # PT_LOAD r-x + PT_LOAD rw-; null/.text/.rodata/.data/.shstrtab


def build():
    ph_off = EHDR
    text_off = ph_off + PHDR * NPH
    rodata_off = text_off + (RODATA_VMA - TEXT_VMA)
    data_off = text_off + (DATA_VMA - TEXT_VMA)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.text', '.rodata', '.data'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = data_off + len(DATA)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', START, ph_off, sh_off)
    b += struct.pack('<I', 0x05000000)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 4)

    ro_end = RODATA_VMA + len(RODATA)
    b += struct.pack('<IIIIIIII', 1, text_off, TEXT_VMA, TEXT_VMA,
                     ro_end - TEXT_VMA, ro_end - TEXT_VMA, PF_R | PF_X, 4)
    b += struct.pack('<IIIIIIII', 1, data_off, DATA_VMA, DATA_VMA,
                     len(DATA), len(DATA), PF_R | PF_W, 4)
    assert len(b) == text_off
    b += TEXT
    b += bytes(rodata_off - len(b))
    b += RODATA
    b += bytes(data_off - len(b))
    b += DATA
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(TEXT))
    b += shdr(names['.rodata'], 1, SHF_ALLOC, RODATA_VMA, rodata_off, len(RODATA))
    b += shdr(names['.data'], 1, SHF_ALLOC | SHF_WRITE, DATA_VMA, data_off, len(DATA))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'poolref_arm_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
