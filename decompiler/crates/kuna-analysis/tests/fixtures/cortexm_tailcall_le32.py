#!/usr/bin/env python3
"""Generate `cortexm_tailcall_le32` — a stripped bare-metal ARM Cortex-M image
whose only path to one routine is a tail `B`, plus three near-miss shapes the
`tailcallentry` containment model must keep rejecting.

No cross toolchain on this host emits a bare-metal STM32 link layout, so the ELF
is assembled here byte by byte and the Thumb bodies are hand-encoded. The vector
table deliberately matches the **shipped** signature (executable load, SRAM stack
word, `word[1] == e_entry`), so nothing here depends on `cortexmvectors`.
Regenerate with:

    python3 cortexm_tailcall_le32.py

Layout — `.text` @ 0x08008000, all Thumb:

  0x00 _start   bl helper            ; e_entry|1, the only seeded root
  0x04          b  TAIL              ; the tail call: TAIL's ONLY in-edge
  0x10 helper   bl loopfn
  0x14          b  SPIN
  0x20 TAIL     movs r0,#0x2a ; bx lr    <- ACCEPTED (the fix)
  0x30 loopfn   bl last
  0x34          b  .Lbody
  0x38 .Lbody   movs r0,#0x63 ; bx lr    <- REJECTED: inside loopfn's own region
  0x40 last     bl final
  0x44          b  EPI
  0x50 final    movs r0,#9 ; bx lr
  0x58 EPI      pop {r4,pc}              <- REJECTED: opens with a stack restore
  0x60 SPIN     movs r0,#0 ; b SPIN      <- REJECTED: region never terminates

Every one of `TAIL` / `.Lbody` / `EPI` / `SPIN` is reached only by an
unconditional `B`, so the naive rule accepts all four; the containment model
accepts exactly `TAIL`.
"""
import struct, os

TABLE_VMA = 0x08000000
TEXT_VMA = 0x08008000
E_ENTRY = TEXT_VMA + 1  # `_start`, Thumb-odd — and the reset vector

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_R = 0x1, 0x4

TABLE = [
    0x20008000,   # [0] initial SP in the architectural SRAM window
    E_ENTRY,      # [1] reset vector == e_entry (the shipped signature)
    0, 0, 0, 0, 0, 0,
]

# (offset, halfwords) — everything not listed is 0x0000 alignment padding.
CODE = {
    0x00: (0xF000, 0xF806),   # _start:  bl 0x10 (helper)
    0x04: (0xE00C,),          #          b  0x20 (TAIL)
    0x10: (0xF000, 0xF80E),   # helper:  bl 0x30 (loopfn)
    0x14: (0xE024,),          #          b  0x60 (SPIN)
    0x20: (0x202A, 0x4770),   # TAIL:    movs r0,#0x2a ; bx lr
    0x30: (0xF000, 0xF806),   # loopfn:  bl 0x40 (last)
    0x34: (0xE000,),          #          b  0x38 (.Lbody)
    0x38: (0x2063, 0x4770),   # .Lbody:  movs r0,#0x63 ; bx lr
    0x40: (0xF000, 0xF806),   # last:    bl 0x50 (final)
    0x44: (0xE008,),          #          b  0x58 (EPI)
    0x50: (0x2009, 0x4770),   # final:   movs r0,#9 ; bx lr
    0x58: (0xBD10,),          # EPI:     pop {r4,pc}
    0x60: (0x2000, 0xE7FD),   # SPIN:    movs r0,#0 ; b 0x60
}
TEXT_LEN = 0x64

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 2, 4  # 2 PT_LOAD; null/.isr_vector/.text/.shstrtab


def text():
    buf = bytearray(TEXT_LEN)
    for off, hws in CODE.items():
        for i, hw in enumerate(hws):
            struct.pack_into('<H', buf, off + 2 * i, hw)
    return bytes(buf)


def build():
    table = b''.join(struct.pack('<I', w) for w in TABLE)
    body = text()
    ph_off = EHDR
    table_off = ph_off + PHDR * NPH
    text_off = table_off + len(table)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.isr_vector', '.text'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = text_off + len(body)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    def phdr(off, vaddr, size, flags):
        return struct.pack('<IIIIIIII', 1, off, vaddr, vaddr, size, size, flags, 4)

    b += phdr(table_off, TABLE_VMA, len(table), PF_R | PF_X)
    b += phdr(text_off, TEXT_VMA, len(body), PF_R | PF_X)
    assert len(b) == table_off
    b += table
    b += body
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.isr_vector'], 1, SHF_ALLOC | SHF_EXECINSTR, TABLE_VMA, table_off,
              len(table))
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(body))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_tailcall_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
