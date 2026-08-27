#!/usr/bin/env python3
"""Generate `cortexm_aifcorroborate_le32` -- the GH-313 witness for `aifcorroborate`:
a stripped bare-metal ARM Cortex-M image in which AIF's gap walk accepts a
self-contained leaf routine on nothing but four other functions sharing its
two-mnemonic prologue.

No cross toolchain is needed (and none on this host emits a bare-metal STM32 link
layout), so the ELF is assembled here byte by byte and the Thumb bodies are
hand-encoded. Regenerate with:

    python3 cortexm_aifcorroborate_le32.py

Layout:

  PT_LOAD [0x08000000, 0x080003b0)  PF_R|PF_X

  .text  @0x08000000  SHF_ALLOC|SHF_EXECINSTR
      0x08000000 [0]  0x20001000    initial SP (SRAM)      \\ the vector table,
      0x08000004 [1]  0x08000041    reset vector == e_entry / SHIPPED signature
      0x08000008 [2..15] 0x08000041    (all -> reset, so the reserved words never
                                        fingerprint as a prologue)
      0x08000040 reset  push {r7,lr} ; bl x70 ; pop {r7,pc}
      0x08000160 H1..H20  movs r0,#k ; movs r1,#k ; movs r2,#k ; bx lr
      0x08000200 G1..G50  movs r0,#k ; adds r1,#k ; adds r2,#k ; bx lr
      0x08000390 U      movs r0,#1 ; movs r1,#2 ; adds r2,#3 ; adds r3,#4 ; bx lr
      0x0800039c V      movs r0,#5 ; movs r1,#6 ; bl H1 ; bx lr
      0x080003a8 W      movs r0,#7 ; adds r1,#8 ; adds r2,#9 ; bx lr

`reset` calls all seventy helpers directly, so the recursive-descent walk reaches
them and stops at each `bx lr`; `U`, `V` and `W` are called by nobody and sit in the
undefined gap the walk leaves at the end of `.text`. The two helper blocks stock
the fingerprint histogram with two DIFFERENT counts on purpose, because the guard
under test is a threshold between them:

  * `movs ; movs` (the `H` block) is shared by **20** functions -- past AIF's
    acceptance floor of 4, below the corroboration threshold of 50;
  * `movs ; adds` (the `G` block) is shared by **50** -- exactly the threshold.

Three shapes, one per branch of `startCount >= 50 || corroborated`:

  * **THE DEFECT** -- `U` opens `movs ; movs` (count 20), calls nothing, jumps
    nowhere, and merely reaches `bx lr`. Upstream Ghidra refuses exactly this
    accept (`AggressiveInstructionFinderAnalyzer.java:367`,
    `!addsInfo && startCount < 50`); kuna never ported that half, so by default
    `U` becomes a function on a two-mnemonic coincidence. With the option on it
    is refused.

  * **THE CORROBORATED CONTROL** -- `V` opens with the SAME count-20 `movs ; movs`
    prologue but its third instruction is a `bl` into `H1`, discovered code. That
    is upstream's "calls always add info", so `V` is accepted in BOTH passes. It
    is what proves the option is a corroboration test and not a fingerprint-count
    raise: same count, opposite verdict.

  * **THE COUNT CONTROL** -- `W` is as uncorroborated as `U` (no call, no jump,
    a plain `bx lr`) but opens `movs ; adds`, the count-50 fingerprint. `50 >= 50`,
    so it too is accepted in BOTH passes -- the guard is a disjunction, not a ban
    on self-contained routines.

  * **THE CURSOR PAIRING** -- `U`'s SECOND instruction begins `movs r1,#2 ;
    adds r2,#3`, i.e. the count-50 `movs ; adds` fingerprint, and the three
    instructions from there still reach `bx lr`. So `0x08000392` -- one halfword
    inside `U`, not a function at all -- would be ACCEPTED on the count branch if
    refusing `U` handed the cursor back to `U`'s interior. It does not: an
    uncorroborated candidate is refused as an entry but still consumes its body,
    so the cursor resumes past `U`. `0x08000392` must appear in neither pass, and
    that assertion is the whole reason the interior is laid out this way (on the
    3.4 MB PE witness, dropping the pairing turned a 361-entry mid-body cut into a
    222-entry mid-body RISE).
"""
import os
import struct

TEXT_VMA = 0x08000000
E_ENTRY = TEXT_VMA + 0x41  # the reset vector, Thumb-odd

SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PF_X, PF_R = 0x1, 0x4

# Every reserved vector points at reset: a reserved slot of zeros would decode as
# two `movs`, i.e. one of the fingerprint classes this fixture stocks, and AIF
# would mint an entry inside the vector table.
VECTORS = [0x20001000] + [E_ENTRY] * 15

# --- Thumb-1 encodings -------------------------------------------------------
PUSH_R7_LR = 0xB580
POP_R7_PC = 0xBD80
BX_LR = 0x4770


def movs(rd, imm):
    return 0x2000 | (rd << 8) | (imm & 0xFF)


def adds(rd, imm):
    return 0x3000 | (rd << 8) | (imm & 0xFF)


def bl(at, target):
    """`BL <target>` (Thumb-2 T1), signed: `V` calls backwards into the helpers."""
    off = target - (at + 4)
    assert -(1 << 24) <= off < (1 << 24) and off % 2 == 0, off
    s = (off >> 24) & 1
    j1 = (~((off >> 23) & 1) ^ s) & 1
    j2 = (~((off >> 22) & 1) ^ s) & 1
    return struct.pack('<HH',
                       0xF000 | (s << 10) | ((off >> 12) & 0x3FF),
                       0xD000 | (j1 << 13) | (j2 << 11) | ((off >> 1) & 0x7FF))


def hw(*halfwords):
    return b''.join(struct.pack('<H', h) for h in halfwords)


# --- addresses (kept in one place; every one is asserted while emitting) ------
NH, NG = 20, 50
RESET = TEXT_VMA + 0x40
H = [TEXT_VMA + 0x160 + 8 * i for i in range(NH)]
G = [TEXT_VMA + 0x200 + 8 * i for i in range(NG)]
U = TEXT_VMA + 0x390          # uncorroborated, count 20 -> refused when on
U_INTERIOR = U + 2            # count 50 -- reachable only if the cursor backs off
V = TEXT_VMA + 0x39C          # corroborated (a `bl` into discovered code)
W = TEXT_VMA + 0x3A8          # uncorroborated but count 50
TEXT_END = TEXT_VMA + 0x3B0


def build_text():
    t = bytearray()

    def at():
        return TEXT_VMA + len(t)

    t += b''.join(struct.pack('<I', w) for w in VECTORS)
    assert at() == RESET, hex(at())

    # reset: call every helper so the walk discovers all seventy.
    t += hw(PUSH_R7_LR)
    for target in [*H, *G]:
        t += bl(at(), target)
    t += hw(POP_R7_PC)
    while at() < H[0]:
        t += hw(BX_LR)  # alignment padding; `bx` starts no shared fingerprint
    assert at() == H[0], hex(at())

    # H1..H20 -- the count-20 `movs ; movs` fingerprint (past AIF's floor of 4,
    # below the corroboration threshold of 50).
    for i in range(NH):
        assert at() == H[i], hex(at())
        t += hw(movs(0, i), movs(1, i), movs(2, i), BX_LR)

    # G1..G50 -- the count-50 `movs ; adds` fingerprint (exactly the threshold).
    assert at() == G[0], hex(at())
    for i in range(NG):
        assert at() == G[i], hex(at())
        t += hw(movs(0, i), adds(1, i), adds(2, i), BX_LR)

    # U -- THE DEFECT. `movs ; movs` (20), no call, no jump, a plain `bx lr`. Its
    # interior at U+2 is `movs ; adds` (50) and still reaches the same `bx lr`,
    # which is the cursor-pairing probe.
    assert at() == U, hex(at())
    t += hw(movs(0, 1), movs(1, 2), adds(2, 3), adds(3, 4), BX_LR)
    t += hw(BX_LR)  # padding to the next word

    # V -- THE CORROBORATED CONTROL. Same count-20 prologue, but it calls H1.
    assert at() == V, hex(at())
    t += hw(movs(0, 5), movs(1, 6)) + bl(at() + 4, H[0]) + hw(BX_LR)
    t += hw(BX_LR)  # padding to the next word

    # W -- THE COUNT CONTROL. As uncorroborated as U, but count 50.
    assert at() == W, hex(at())
    t += hw(movs(0, 7), adds(1, 8), adds(2, 9), BX_LR)

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
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'cortexm_aifcorroborate_le32')
    with open(out, 'wb') as f:
        f.write(build())
    print(f'wrote {out} ({os.path.getsize(out)} bytes)')
