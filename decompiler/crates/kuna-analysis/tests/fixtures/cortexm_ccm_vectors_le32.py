#!/usr/bin/env python3
"""Generate `cortexm_ccm_vectors_le32` — a stripped bare-metal ARM Cortex-M
firmware image that reproduces, in one file, all three reasons kuna's shipped
vector-table signature rejects real STM32 firmware.

No cross toolchain is needed (and none of the ones on this host emits a
bare-metal STM32 link layout), so the ELF is assembled here byte by byte and the
Thumb bodies are hand-encoded. Regenerate with:

    python3 cortexm_ccm_vectors_le32.py

Layout — the cleanflight/betaflight/crazyflie/nuttx union:

  PT_LOAD [0x08000000, 0x08000020)  PF_R          <- table load: NOT executable
  PT_LOAD [0x08008000, 0x08008014)  PF_R|PF_X

  .isr_vector  @0x08000000  sh_flags = SHF_ALLOC       (A only, not AX)
      [0] 0x1000fff0   initial SP in STM32F4 CCM RAM   (outside 0x20000000+)
      [1] 0x08008001   Reset_Handler   | Thumb bit     (!= e_entry)
      [2] 0x08008005   NMI_Handler     | Thumb bit
      [3] 0x08008009   HardFault_Handler
      [4] 0x0800800d   SysTick_Handler
      [5..7] 0         reserved slots

  .text        @0x08008000  sh_flags = SHF_ALLOC|SHF_EXECINSTR, Thumb bodies:
      0x08008000 Reset_Handler      movs r0,#1  ; bx lr
      0x08008004 NMI_Handler        movs r0,#7  ; bx lr
      0x08008008 HardFault_Handler  movs r0,#11 ; bx lr
      0x0800800c SysTick_Handler    movs r0,#21 ; bx lr
      0x08008010 _start             movs r0,#0  ; bx lr     <- e_entry|1

  e_entry = 0x08008011 (`_start`, Thumb-odd) — deliberately NOT the reset vector.
"""
import struct, os

TABLE_VMA = 0x08000000
TEXT_VMA = 0x08008000
E_ENTRY = TEXT_VMA + 0x10 + 1  # `_start`, Thumb-odd — not the reset vector

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_W, PF_R = 0x1, 0x2, 0x4

TABLE = [
    0x1000FFF0,      # [0] initial SP: STM32F4 CCM RAM, not the 0x20000000 block
    TEXT_VMA + 0x01,  # [1] Reset_Handler | 1
    TEXT_VMA + 0x05,  # [2] NMI_Handler | 1
    TEXT_VMA + 0x09,  # [3] HardFault_Handler | 1
    TEXT_VMA + 0x0D,  # [4] SysTick_Handler | 1
    0, 0, 0,          # [5..7] reserved
]

def thumb_ret_const(k):
    """`movs r0,#k ; bx lr` (Thumb-1: 0x20kk, 0x4770)."""
    return struct.pack('<HH', 0x2000 | (k & 0xFF), 0x4770)

TEXT = b''.join(thumb_ret_const(k) for k in (1, 7, 11, 21, 0))

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 2, 4  # 2 PT_LOAD; null/.isr_vector/.text/.shstrtab


def build():
    table = b''.join(struct.pack('<I', w) for w in TABLE)
    ph_off = EHDR
    table_off = ph_off + PHDR * NPH
    text_off = table_off + len(table)

    shstr = b'\0'
    names = {}
    for n in ('.shstrtab', '.isr_vector', '.text'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'
    shstr_off = text_off + len(TEXT)
    sh_off = shstr_off + len(shstr)

    b = bytearray()
    b += b'\x7fELF' + bytes([1, 1, 1]) + bytes(9)          # e_ident (ELF32/LSB)
    b += struct.pack('<HHI', 2, 40, 1)                      # ET_EXEC, EM_ARM, v1
    b += struct.pack('<III', E_ENTRY, ph_off, sh_off)
    b += struct.pack('<I', 0x05000200)                      # e_flags: EABI5
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 3)

    def phdr(off, vaddr, size, flags):
        return struct.pack('<IIIIIIII', 1, off, vaddr, vaddr, size, size, flags, 4)

    b += phdr(table_off, TABLE_VMA, len(table), PF_R)        # NOT executable
    b += phdr(text_off, TEXT_VMA, len(TEXT), PF_R | PF_X)
    assert len(b) == table_off
    b += table
    b += TEXT
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.isr_vector'], 1, SHF_ALLOC, TABLE_VMA, table_off, len(table))
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(TEXT))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_ccm_vectors_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
