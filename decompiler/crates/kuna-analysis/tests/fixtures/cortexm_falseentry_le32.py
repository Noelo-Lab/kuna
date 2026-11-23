#!/usr/bin/env python3
"""Generate `cortexm_falseentry_le32` — a stripped bare-metal ARM Cortex-M image
whose prologue-pattern scan plants two entries inside real functions, next to a
pattern-only function that must stay an entry.

The ELF is assembled here byte by byte and the Thumb bodies are hand-encoded, as
for the sibling `cortexm_tailcall_le32.py`; the vector table matches the shipped
Cortex-M signature. Regenerate with:

    python3 cortexm_falseentry_le32.py

Layout — `.text` @ 0x08008000, all Thumb:

  0x00 _start   bl clamp ; bl pick ; bl fatal ; b .
  0x20 clamp    cmp r0,#0 ; it eq ; bxeq lr      shrink-wrapped early return
  0x26          push {r4,lr} ; mov r4,r0 ...      <- `bx lr; push; mov` pattern:
                                                     planted, run into by the
                                                     predicated return's skip
  0x40 pick     cbz r0,0x46 ; movs r0,#7 ; bx lr
  0x46          push {r4,lr} ; mov r4,r1 ...      <- planted, reached only by
                                                     pick's cbz past its return
  0x60 fatal    push {r3,lr} ; bl spin ; nop      never returns
  0x68 orphan   push {r4,lr} ; mov r4,r0 ...      <- planted, a real function:
                                                     only padding and a call
                                                     that never returns precede it
  0x70 spin     b .
"""
import struct, os

TABLE_VMA = 0x08000000
TEXT_VMA = 0x08008000
E_ENTRY = TEXT_VMA + 1

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_R = 0x1, 0x4

TABLE = [0x20008000, E_ENTRY, 0, 0, 0, 0, 0, 0]


def bl(at, to):
    off = to - (at + 4)
    s = 1 if off < 0 else 0
    imm = off & 0x1FFFFFF
    i1, i2 = (imm >> 23) & 1, (imm >> 22) & 1
    j1, j2 = (1 - i1) ^ s, (1 - i2) ^ s
    return (0xF000 | (s << 10) | ((imm >> 12) & 0x3FF),
            0xD000 | (j1 << 13) | (j2 << 11) | ((imm >> 1) & 0x7FF))


CODE = {
    0x00: bl(0x00, 0x20),        # _start: bl clamp
    0x04: bl(0x04, 0x40),        #         bl pick
    0x08: bl(0x08, 0x60),        #         bl fatal
    0x0C: (0xE7FE,),             #         b .
    0x20: (0x2800,               # clamp:  cmp r0,#0
           0xBF08,               #         it eq
           0x4770,               #         bxeq lr
           0xB510,               # 0x26:   push {r4,lr}
           0x4604,               #         mov r4,r0
           0x1C60,               #         adds r0,r4,#1
           0x4360,               #         muls r0,r4
           0xBD10),              #         pop {r4,pc}
    0x40: (0xB108,               # pick:   cbz r0,0x46
           0x2007,               #         movs r0,#7
           0x4770,               #         bx lr
           0xB510,               # 0x46:   push {r4,lr}
           0x460C,               #         mov r4,r1
           0x1C60,               #         adds r0,r4,#1
           0xBD10),              #         pop {r4,pc}
    0x60: (0xB508,),             # fatal:  push {r3,lr}
    0x62: bl(0x62, 0x70),        #         bl spin
    0x66: (0xBF00,),             #         nop
    0x68: (0xB510,               # orphan: push {r4,lr}
           0x4604,               #         mov r4,r0
           0x1E60,               #         subs r0,r4,#1
           0xBD10),              #         pop {r4,pc}
    0x70: (0xE7FE,),             # spin:   b .
    0x80: (0x6843,               # handler: ldr r3,[r0,#4]
           0xB123,               #          cbz r3,0x8e
           0xB510,               # 0x84:    push {r4,lr}
           0x4604,               #          mov r4,r0
           0x4798,               #          blx r3
           0x1C20,               #          adds r0,r4,#0
           0xBD10,               #          pop {r4,pc}
           0x2000,               # 0x8e:    movs r0,#0
           0x4770),              #          bx lr
}
TEXT_LEN = 0x94
RODATA_VMA = 0x08009000
RODATA = [TEXT_VMA + 0x80 + 1]

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 3, 5


def text():
    buf = bytearray(TEXT_LEN)
    for off, hws in CODE.items():
        for i, hw in enumerate(hws):
            struct.pack_into('<H', buf, off + 2 * i, hw)
    return bytes(buf)


def build():
    table = b''.join(struct.pack('<I', w) for w in TABLE)
    body = text()
    rodata = b''.join(struct.pack('<I', w) for w in RODATA)
    ph_off = EHDR
    table_off = ph_off + PHDR * NPH
    text_off = table_off + len(table)
    rodata_off = text_off + len(body)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.isr_vector', '.text', '.rodata'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = rodata_off + len(rodata)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)
    b += struct.pack('<HHI', 2, 40, 1)
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    def phdr(off, vaddr, size, flags):
        return struct.pack('<IIIIIIII', 1, off, vaddr, vaddr, size, size, flags, 4)

    b += phdr(table_off, TABLE_VMA, len(table), PF_R | PF_X)
    b += phdr(text_off, TEXT_VMA, len(body), PF_R | PF_X)
    b += phdr(rodata_off, RODATA_VMA, len(rodata), PF_R)
    assert len(b) == table_off
    b += table
    b += body
    b += rodata
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.isr_vector'], 1, SHF_ALLOC | SHF_EXECINSTR, TABLE_VMA, table_off, len(table))
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(body))
    b += shdr(names['.rodata'], 1, SHF_ALLOC, RODATA_VMA, rodata_off, len(rodata))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_falseentry_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
