#!/usr/bin/env python3
"""Build rawlzma_headerless_pe_x86_64.exe -- a PE64 carrying a UPX LZMA1 stream
with no PackHeader anywhere in the file.

This is the shape `kuna unpack --raw-lzma` exists for: the payload is perfectly
decodable and the metadata that would find it is simply not there, so nothing
short of an analyst-supplied range recovers it.

The stream is not synthesized. It is block 0 of the vendored
`upx_packed_lzma_x86_64` witness, lifted byte for byte, so the fixture exercises
a real UPX encoder's output -- including the property that makes the length-less
decode necessary: UPX writes no end-of-stream marker, so the decode ends on the
input and not on a marker. The block decodes to 792 bytes beginning `\x7fELF`.

    python3 rawlzma_headerless_pe_x86_64.py
"""
import struct
from pathlib import Path

HERE = Path(__file__).resolve().parent
BLOCK_AT = 0x100  # the first b_info in the witness, right after l_info + p_info

IMAGE_BASE = 0x140000000
SECT_RVA = 0x2000
HEADERS = 0x200
RAW_AT = 0x200
RAW_SIZE = 0x200


def payload() -> bytes:
    packed = (HERE / "upx_packed_lzma_x86_64").read_bytes()
    _u_len, c_len = struct.unpack_from("<II", packed, BLOCK_AT)
    assert packed[BLOCK_AT + 8] == 14, "the witness block is not LZMA"
    block = packed[BLOCK_AT + 12 : BLOCK_AT + 12 + c_len]
    assert block[:2] == b"\x1a\x03", "unexpected UPX property prefix"
    assert len(block) <= RAW_SIZE
    return block


def build(block: bytes) -> bytes:
    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    opt = struct.pack(
        "<HBBIIIIIQ",
        0x20B, 14, 0, RAW_SIZE, 0, 0, 0, SECT_RVA, IMAGE_BASE,
    ) + struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        0x1000, 0x200, 6, 0, 0, 0, 6, 0, 0,
        SECT_RVA + 0x1000, HEADERS, 0, 3, 0,
        0x100000, 0x1000, 0x100000, 0x1000, 0, 16,
    ) + b"\0" * (16 * 8)
    assert len(opt) == 240, len(opt)

    coff = struct.pack("<IHHIIIHH", 0x4550, 0x8664, 1, 0, 0, 0, len(opt), 0x0022)
    sect = struct.pack(
        "<8sIIIIIIHHI",
        b"UPX1", len(block), SECT_RVA, RAW_SIZE, RAW_AT, 0, 0, 0, 0, 0xE0000040,
    )

    image = bytearray(RAW_AT + RAW_SIZE)
    image[0 : len(dos)] = dos
    image[0x40 : 0x40 + len(coff) + len(opt) + len(sect)] = coff + opt + sect
    image[RAW_AT : RAW_AT + len(block)] = block
    return bytes(image)


if __name__ == "__main__":
    block = payload()
    out = HERE / "rawlzma_headerless_pe_x86_64.exe"
    out.write_bytes(build(block))
    start = IMAGE_BASE + SECT_RVA
    print(f"{out.name}: {out.stat().st_size} bytes")
    print(f"  --raw-lzma {start:#x}:{start + len(block):#x}  ({len(block)} compressed bytes)")
