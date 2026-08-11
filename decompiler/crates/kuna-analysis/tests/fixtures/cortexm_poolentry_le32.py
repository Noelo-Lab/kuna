#!/usr/bin/env python3
"""Generate `cortexm_poolentry_le32` — a stripped bare-metal ARM Cortex-M image
carrying, in one file, the three shapes of the `poolentry` contract: the phantom
that must MOVE, the phantom that must be KEPT because nothing replaces it, and the
one disclosed split the corpus measurement found.

No cross toolchain is needed (and none on this host emits a bare-metal STM32 link
layout), so the ELF is assembled here byte by byte and the Thumb bodies are
hand-encoded. Regenerate with:

    python3 cortexm_poolentry_le32.py

Layout:

  PT_LOAD [0x08000000, 0x080000bc)  PF_R|PF_X

  .text  @0x08000000  SHF_ALLOC|SHF_EXECINSTR
      0x08000000 [0]  0x20001000    initial SP (SRAM)        \\ the vector table,
      0x08000004 [1]  0x08000041    reset vector == e_entry  / SHIPPED signature
      0x08000008 [2..15] 0x08000041     (all -> reset, so the reserved words never
                                         fingerprint as a prologue)
      0x08000040 reset   push {r7,lr} ; bl x23 ; pop {r7,pc}
      0x080000a0 H1..H20 movs r0,#k ; movs r1,#k ; movs r2,#k ; bx lr
      0x08000140 A       push {r7,lr} ; ldr r0,[pc,#4] ; movs r1,#1 ; pop {r7,pc}
      0x08000148 POOL1   .word 0x20001000        <- PHANTOM
      0x0800014c B       movs r1,#6 ; movs r2,#7 ; movs r3,#8 ; bx lr  (returns r0)
      0x08000154 C       push {r7,lr} ; ldr r0,[pc,#4] ; movs r1,#2 ; pop {r7,pc}
      0x0800015c POOL2   .word 0x20001000        <- UNPAIRED
      0x08000160 D       movs r2,#9 ; adds r2,#1 ; adds r2,#1 ; bx lr
      0x08000168 G       push {r7,lr} ; ldr r0,[pc,#4] ; movs r1,#3 ; pop {r7,pc}
      0x08000170 F       movs r0,#7 ; movs r1,#8 ; movs r2,#1 ; movs r3,#2 ; bx lr
                                                 <- SPLIT (G's literal lands on F+0)

`H1..H20` are two gates at once. AIF refuses to run at all on a program with
fewer than `MINIMUM_FUNCTION_COUNT` (20) discovered functions, and it accepts a
candidate only when its two-mnemonic prologue fingerprint is shared by at least
`FINGERPRINT_THRESHOLD` (4) of them; twenty helpers whose first two instructions
are `movs ; movs` over four bytes clear both.

  * **PHANTOM** — `A` loads `POOL1`, whose word `0x20001000` is the STM32 SRAM
    constant of the real defect. Its HIGH halfword `0x2000` decodes as `movs r0,#0`,
    so AIF's byte-granular cursor accepts `POOL1+2` and then jumps past `B`'s body,
    which is why `B` is never discovered. `poolentry` infers the pool from `A`'s
    literal reference, emits the entry at `POOL1+4 == B`, and — because that
    replacement exists — drops the `POOL1+2` accept. A MOVE, not a delete.
  * **UNPAIRED** — `C`/`POOL2`/`D` is the same shape with one difference: `D`
    opens `movs ; adds`, a fingerprint no discovered function shares, so the
    additive half refuses to emit an entry at the pool end. With no replacement the
    pairing invariant forbids the removal and the `POOL2+2` phantom is KEPT. This
    is the assertion that would have caught the 531 real bodies unpaired
    suppression destroys corpus-wide.
  * **SPLIT** — `G`'s literal reference resolves onto `F`'s own FIRST WORD, which
    the Listing never decoded, so the inference reads those four bytes as a pool
    and plants the entry four bytes into a real function. This is the single
    disclosed residue of the corpus measurement (betaflight O2-noinline
    `SCSI_RequestSense` + 4); the fixture pins it as CURRENT BEHAVIOUR rather than
    hiding it, because the only guard that removes it costs 108 of 189 recovered
    ground-truth entries.
"""
import struct, os

TEXT_VMA = 0x08000000
E_ENTRY = TEXT_VMA + 0x41  # the reset vector, Thumb-odd

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_R = 0x1, 0x4

# Every reserved vector points at reset: a reserved slot of zeros would decode as
# two `movs`, i.e. the very fingerprint class this fixture stocks, and AIF would
# mint an entry inside the vector table.
VECTORS = [0x20001000] + [E_ENTRY] * 15

# --- Thumb-1 encodings -------------------------------------------------------
PUSH_R7_LR = 0xB580
POP_R7_PC = 0xBD80
BX_LR = 0x4770


def movs(rd, imm):
    return 0x2000 | (rd << 8) | (imm & 0xFF)


def adds(rd, imm):
    return 0x3000 | (rd << 8) | (imm & 0xFF)


def ldr_pc(at, rd, target):
    """`ldr rd,[pc,#imm]` (T1): base is the word-aligned PC (`at + 4`)."""
    off = target - ((at + 4) & ~3)
    assert 0 <= off <= 1020 and off % 4 == 0, (hex(at), hex(target), off)
    return 0x4800 | (rd << 8) | (off >> 2)


def bl(at, target):
    """`BL <target>` (Thumb-2 T1), for a small positive offset."""
    off = target - (at + 4)
    assert 0 <= off < (1 << 22) and off % 2 == 0, off
    return struct.pack('<HH', 0xF000 | ((off >> 12) & 0x3FF), 0xF800 | ((off >> 1) & 0x7FF))


def hw(*halfwords):
    return b''.join(struct.pack('<H', h) for h in halfwords)


# --- addresses (kept in one place; every one is asserted while emitting) ------
NHELPERS = 20
RESET = TEXT_VMA + 0x40
H = [TEXT_VMA + 0xA0 + 8 * i for i in range(NHELPERS)]
A = TEXT_VMA + 0x140
POOL1 = TEXT_VMA + 0x148
B = TEXT_VMA + 0x14C
C = TEXT_VMA + 0x154
POOL2 = TEXT_VMA + 0x15C
D = TEXT_VMA + 0x160
G = TEXT_VMA + 0x168
F = TEXT_VMA + 0x170
TEXT_END = TEXT_VMA + 0x17C

# The SRAM constant that makes the defect: its high halfword is `movs r0,#0`.
SRAM_WORD = 0x20001000


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    t += b''.join(struct.pack('<I', w) for w in VECTORS)
    assert at() == RESET, hex(at())

    # reset: call every discovered function so the walk finds them all.
    t += hw(PUSH_R7_LR)
    for target in [*H, A, C, G]:
        t += bl(at(), target)
    t += hw(POP_R7_PC)
    assert at() == H[0], hex(at())

    # H1..H20 — the `movs ; movs` fingerprint stock, and the 20-function floor.
    for i in range(NHELPERS):
        assert at() == H[i], hex(at())
        t += hw(movs(0, i), movs(1, i), movs(2, i), BX_LR)

    # A / POOL1 / B — the PHANTOM shape.
    assert at() == A, hex(at())
    t += hw(PUSH_R7_LR, ldr_pc(at() + 2, 0, POOL1), movs(1, 1), POP_R7_PC)
    assert at() == POOL1, hex(at())
    t += struct.pack('<I', SRAM_WORD)
    assert at() == B, hex(at())
    # B never writes r0, so the phantom's dead leading `movs r0,#0` is visible in
    # the C: the true entry returns its argument, the phantom returns 0.
    t += hw(movs(1, 6), movs(2, 7), movs(3, 8), BX_LR)

    # C / POOL2 / D — the UNPAIRED shape (`D` opens `movs ; adds`).
    assert at() == C, hex(at())
    t += hw(PUSH_R7_LR, ldr_pc(at() + 2, 0, POOL2), movs(1, 2), POP_R7_PC)
    assert at() == POOL2, hex(at())
    t += struct.pack('<I', SRAM_WORD)
    assert at() == D, hex(at())
    t += hw(movs(2, 9), adds(2, 1), adds(2, 1), BX_LR)

    # G / F — the SPLIT shape: the literal resolves onto F's own first word.
    assert at() == G, hex(at())
    t += hw(PUSH_R7_LR, ldr_pc(at() + 2, 0, F), movs(1, 3), POP_R7_PC)
    assert at() == F, hex(at())
    t += hw(movs(0, 7), movs(1, 8), movs(2, 1), movs(3, 2), BX_LR)

    t += hw(BX_LR)  # 4-align the section end
    assert at() == TEXT_END, hex(at())
    return bytes(t)


TEXT = build_text()

EHDR, PHDR, SHDR = 52, 32, 40
NPH, NSH = 1, 3  # 1 PT_LOAD; null/.text/.shstrtab


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
    b += shstr

    def shdr(name, stype, flags, addr, off, size):
        return struct.pack('<IIIIIIIIII', name, stype, flags, addr, off, size, 0, 0, 4, 0)

    assert len(b) == sh_off
    b += shdr(0, 0, 0, 0, 0, 0)
    b += shdr(names['.text'], 1, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off, len(TEXT))
    b += shdr(names['.shstrtab'], 3, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_poolentry_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
