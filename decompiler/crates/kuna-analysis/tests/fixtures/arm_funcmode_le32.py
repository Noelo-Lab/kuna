#!/usr/bin/env python3
"""Generate the `armfuncmode` fixtures (GH-842): ARM32 ELFs with function
symbols, hand-encoded on the `arm_interwork_le32` layout. Regenerate with:

    python3 arm_funcmode_le32.py

Layout (one R+X PT_LOAD, `.text` at 0x02000000):

    0x02000000  push {lr}            ; entry         (STT_FUNC 0x02000000)
    0x02000004  blx  0x02000040
    0x02000008  bl   0x02000080
    0x0200000c  pop  {pc}
    0x02000040  movs r0, #7          ; thumb_helper  (STT_FUNC 0x02000041)
    0x02000042  bx   lr
    0x02000080  add  r0, r0, #1      ; arm_helper    (STT_FUNC 0x02000080)
    0x02000084  bx   lr

* `arm_funcmode_le32`: no mapping symbols (GH-842's image).
* `arm_funcmode_nocall_le32`: the same symbols, but the `blx` is a `nop`, so
  only the odd `thumb_helper` symbol says anything about Thumb.
* `arm_funcmode_mapsyms_le32`: GH-842's image plus `$a`/`$t`/`$a` mapping
  symbols at 0x02000000/0x02000040/0x02000080.
* `arm_funcmode_sizes_le32`: GH-842's image with `entry` sized 0x50, past the
  Thumb symbol at 0x02000041, and `arm_helper` sized 0.
"""
from pathlib import Path
import struct

BASE = 0x02000000
FUNCS = [(b"entry", BASE, 0x10), (b"thumb_helper", BASE + 0x41, 4), (b"arm_helper", BASE + 0x80, 8)]
MAPS = [(b"$a", BASE), (b"$t", BASE + 0x40), (b"$a", BASE + 0x80)]


def image(call=True, maps=False, sizes=None):
    text = bytearray(0x88)
    struct.pack_into("<4I", text, 0, 0xE92D4000, 0xFA00000D if call else 0xE1A00000, 0xEB00001C, 0xE8BD8000)
    struct.pack_into("<2H", text, 0x40, 0x2007, 0x4770)
    struct.pack_into("<2I", text, 0x80, 0xE2800001, 0xE12FFF1E)
    strtab = bytearray(b"\0")
    symbols = [bytes(16)]

    def symbol(name, value, size, info):
        strtab.extend(name + b"\0")
        symbols.append(struct.pack("<IIIBBH", len(strtab) - len(name) - 1, value, size, info, 0, 1))

    for name, value in MAPS if maps else []:
        symbol(name, value, 0, 0x00)
    first_global = len(symbols)
    for name, value, size in FUNCS:
        symbol(name, value, (sizes or {}).get(name, size), 0x12)
    names = b"\0.text\0.symtab\0.strtab\0.shstrtab\0"
    contents = bytearray(84)
    sections = [bytes(40)]

    def section(name, data, kind, flags=0, address=0, link=0, info=0, entsize=0):
        contents.extend(bytes((-len(contents)) % 4))
        offset = len(contents)
        contents.extend(data)
        sections.append(struct.pack("<10I", names.index(name + b"\0"), kind, flags, address,
                                    offset, len(data), link, info, 4, entsize))
        return len(sections) - 1, offset

    _, text_offset = section(b".text", text, 1, 6, BASE)
    section(b".symtab", b"".join(symbols), 2, link=3, info=first_global, entsize=16)
    section(b".strtab", bytes(strtab), 3)
    names_index, _ = section(b".shstrtab", names, 3)
    contents.extend(bytes((-len(contents)) % 4))
    section_offset = len(contents)
    contents.extend(b"".join(sections))
    contents[:52] = struct.pack("<16sHHIIIIIHHHHHH", b"\x7fELF\x01\x01\x01" + bytes(9),
                               2, 40, 1, BASE, 52, section_offset, 0x05000000,
                               52, 32, 1, 40, len(sections), names_index)
    contents[52:84] = struct.pack("<8I", 1, text_offset, BASE, BASE, len(text), len(text), 5, 4)
    return bytes(contents)


if __name__ == "__main__":
    here = Path(__file__).parent
    (here / "arm_funcmode_le32").write_bytes(image())
    (here / "arm_funcmode_nocall_le32").write_bytes(image(call=False))
    (here / "arm_funcmode_mapsyms_le32").write_bytes(image(maps=True))
    (here / "arm_funcmode_sizes_le32").write_bytes(image(sizes={b"entry": 0x50, b"arm_helper": 0}))
