#!/usr/bin/env python3
"""Generate `followoverlap_x86_64` -- an x86-64 ELF whose branches land inside
instructions another reachable path decodes, for `kuna disassemble --follow`.

Each function is a shape in which two paths through the same bytes read them as
different instructions, so a correct `--follow` listing holds both readings and
its rows overlap (GH #1363). Before the fix the later path was clipped at the
earlier one's rows: a branch target whose instruction ran past the next claimed
row got no row at all, while the notes said both that it was decoded from and
that it could not be re-anchored.

    audit_overlap      0x401000  the reporter's dylib bytes: `je` into the
                                 fall-through MOV, whose own decode then runs
        85 ff           test edi,edi
        74 01           je   0x401005
        b8 b8 2a 00 00  mov  eax,0x2ab8   <- fall-through; covers 0x401005
        00 c3           add  bl,al
        c3              ret
        (from 0x401005: b8 2a 00 00 00 mov eax,0x2a ; c3 ret)

    audit_chain        0x401010  a target inside a row, whose own run names a
                                 second target inside another
        74 03           je   0x401015
        b8 a1 a2 74 02  mov  eax,0x274a2a1      <- covers 0x401015
        b9 c3 b8 2a 00  mov  ecx,0x2ab8c3       <- covers 0x401019
        00 00           add  [rax],al
        c3              ret
        (from 0x401015: 74 02 je 0x401019 ; from 0x401019: mov eax,0x2a)

    audit_backward     0x401020  a jump back into the middle of an instruction
        b8 90 05 c3 90  mov  eax,0x90c30590     <- covers 0x401022
        eb fb           jmp  0x401022
        c3              ret
        (from 0x401022: 05 c3 90 eb fb add eax,0xfbeb90c3, swallowing the jmp)

    audit_undecodable  0x401030  a branch to a byte that is no instruction
        74 01           je   0x401033
        b8 06 00 00 00  mov  eax,6              <- 0x06 is invalid in 64-bit mode
        c3              ret

    audit_unreached    0x401040  a branch only unreached bytes name
        c3              ret
        eb 01           jmp  0x401044           <- nothing flows here
        b8 31 c0 c3 90  mov  eax,0x90c3c031     <- covers 0x401044

Functions are padded to 16 bytes with int3. No toolchain is needed; the ELF is
assembled here byte by byte. Regenerate with:

    python3 followoverlap_x86_64.py

Layout: PT_LOAD [0x401000, 0x401048) R+X, `.text` there, `.symtab` with one
STT_FUNC per function (st_size = its bytes, without padding), e_entry =
audit_overlap.
"""
import os
import struct

TEXT_VMA = 0x401000

FUNCS = [
    ('audit_overlap', bytes([
        0x85, 0xff, 0x74, 0x01, 0xb8, 0xb8, 0x2a, 0x00, 0x00, 0x00, 0xc3, 0xc3,
    ])),
    ('audit_chain', bytes([
        0x74, 0x03, 0xb8, 0xa1, 0xa2, 0x74, 0x02, 0xb9, 0xc3, 0xb8, 0x2a, 0x00,
        0x00, 0x00, 0xc3,
    ])),
    ('audit_backward', bytes([
        0xb8, 0x90, 0x05, 0xc3, 0x90, 0xeb, 0xfb, 0xc3,
    ])),
    ('audit_undecodable', bytes([
        0x74, 0x01, 0xb8, 0x06, 0x00, 0x00, 0x00, 0xc3,
    ])),
    ('audit_unreached', bytes([
        0xc3, 0xeb, 0x01, 0xb8, 0x31, 0xc0, 0xc3, 0x90,
    ])),
]

SHT_PROGBITS, SHT_SYMTAB, SHT_STRTAB = 1, 2, 3
SHF_ALLOC, SHF_EXECINSTR = 0x2, 0x4
PT_LOAD, PF_X, PF_R = 1, 0x1, 0x4
STB_GLOBAL, STT_FUNC = 1, 2

EHDR, PHDR, SHDR, SYM = 64, 56, 64, 24
NPH, NSH = 1, 5  # null/.text/.symtab/.strtab/.shstrtab


def layout():
    text = bytearray()
    syms = []
    for name, code in FUNCS:
        syms.append((name, TEXT_VMA + len(text), len(code)))
        text += code
        if name != FUNCS[-1][0]:
            text += b'\xcc' * (-len(text) % 16)
    return bytes(text), syms


def build():
    text, syms = layout()
    strtab = b'\0'
    symtab = bytes(SYM)
    for name, addr, size in syms:
        symtab += struct.pack('<IBBHQQ', len(strtab), (STB_GLOBAL << 4) | STT_FUNC, 0, 1,
                              addr, size)
        strtab += name.encode() + b'\0'
    shstr = b'\0'
    names = {}
    for n in ('.text', '.symtab', '.strtab', '.shstrtab'):
        names[n] = len(shstr)
        shstr += n.encode() + b'\0'

    text_off = EHDR + PHDR * NPH
    symtab_off = (text_off + len(text) + 7) & ~7
    strtab_off = symtab_off + len(symtab)
    shstr_off = strtab_off + len(strtab)
    sh_off = (shstr_off + len(shstr) + 7) & ~7

    b = bytearray()
    b += b'\x7fELF' + bytes([2, 1, 1]) + bytes(9)            # e_ident (ELF64/LSB)
    b += struct.pack('<HHI', 2, 62, 1)                        # ET_EXEC, EM_X86_64, v1
    b += struct.pack('<QQQ', syms[0][1], EHDR, sh_off)
    b += struct.pack('<I', 0)                                 # e_flags
    b += struct.pack('<HHHHHH', EHDR, PHDR, NPH, SHDR, NSH, 4)
    b += struct.pack('<IIQQQQQQ', PT_LOAD, PF_R | PF_X, text_off, TEXT_VMA, TEXT_VMA,
                     len(text), len(text), 0x1000)
    assert len(b) == text_off
    b += text
    b += bytes(symtab_off - len(b))
    b += symtab
    b += strtab
    b += shstr
    b += bytes(sh_off - len(b))

    def shdr(name, stype, flags, addr, off, size, link=0, info=0, align=1, entsize=0):
        return struct.pack('<IIQQQQIIQQ', name, stype, flags, addr, off, size, link, info,
                           align, entsize)

    b += shdr(0, 0, 0, 0, 0, 0, align=0)
    b += shdr(names['.text'], SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, TEXT_VMA, text_off,
              len(text), align=16)
    b += shdr(names['.symtab'], SHT_SYMTAB, 0, 0, symtab_off, len(symtab), link=3, info=1,
              align=8, entsize=SYM)
    b += shdr(names['.strtab'], SHT_STRTAB, 0, 0, strtab_off, len(strtab))
    b += shdr(names['.shstrtab'], SHT_STRTAB, 0, 0, shstr_off, len(shstr))
    return bytes(b)


if __name__ == '__main__':
    out = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'followoverlap_x86_64')
    with open(out, 'wb') as fh:
        fh.write(build())
    os.chmod(out, 0o755)
    print('wrote %s (%d bytes)' % (out, os.path.getsize(out)))
