#!/usr/bin/env python3
"""Generate `arm_interwork_le32` - a stripped ARM32 ELF whose A32 entry calls a
Thumb helper with `blx` and then an A32 helper with `bl` (GH-780).

No symbol table and no mapping symbols, so nothing but the two call
instructions says which mode each helper is in. The `blx` must select Thumb for
its own target only; the `bl` keeps the entry's A32 mode for its target.

No cross toolchain is needed; the bytes are hand-encoded. Regenerate with:

    python3 arm_interwork_le32.py

Layout (one R+X PT_LOAD, `.text` at 0x02000000):

    0x02000000  push {lr}
    0x02000004  blx  0x02000040      ; Thumb helper
    0x02000008  bl   0x02000080      ; A32 helper
    0x0200000c  pop  {pc}
    0x02000040  movs r0, #7          ; Thumb
    0x02000042  bx   lr
    0x02000080  add  r0, r0, #1      ; A32
    0x02000084  bx   lr
"""
from pathlib import Path
import struct

BASE = 0x02000000


def image():
    text = bytearray(0x88)
    struct.pack_into("<4I", text, 0, 0xE92D4000, 0xFA00000D, 0xEB00001C, 0xE8BD8000)
    struct.pack_into("<2H", text, 0x40, 0x2007, 0x4770)
    struct.pack_into("<2I", text, 0x80, 0xE2800001, 0xE12FFF1E)
    names = b"\0.text\0.shstrtab\0"
    contents = bytearray(84)
    sections = [bytes(40)]

    def section(name, data, kind, flags=0, address=0):
        contents.extend(bytes((-len(contents)) % 4))
        offset = len(contents)
        contents.extend(data)
        sections.append(struct.pack("<10I", names.index(name + b"\0"), kind, flags, address,
                                    offset, len(data), 0, 0, 4, 0))
        return len(sections) - 1, offset

    _, text_offset = section(b".text", text, 1, 6, BASE)
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
    (Path(__file__).parent / "arm_interwork_le32").write_bytes(image())
