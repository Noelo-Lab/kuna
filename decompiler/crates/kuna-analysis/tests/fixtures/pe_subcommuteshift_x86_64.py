#!/usr/bin/env python3
"""Generate the minimal PE used by the `cancelbytearithmetic` CLI acceptance.

The `.text` payload is the exact 9,024-byte `crackme_shroud.exe`
`sub_1406fa160` bytechunk already pinned by
`tests/stages/kuna-subcommuteshift.xml`.  It is mapped at its original VMA so
all RIP-relative references retain their original meaning.  A virtual `.bss`
section covers the referenced globals, including `usage` at `0x1408f1c08`.

Regenerate with:

    python3 pe_subcommuteshift_x86_64.py
"""

from pathlib import Path
import re
import struct


IMAGE_BASE = 0x140000000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
TEXT_RVA = 0x6FA000
ENTRY_RVA = 0x6FA160
BSS_RVA = 0x8F1000
BSS_SIZE = 0x2000


def align(value: int, amount: int) -> int:
    return (value + amount - 1) // amount * amount


def witness_bytes() -> bytes:
    root = Path(__file__).resolve().parents[5]
    stage = (root / "tests/stages/kuna-subcommuteshift.xml").read_text()
    match = re.search(
        r'<bytechunk space="ram" offset="0x1406fa160">(.*?)</bytechunk>',
        stage,
        re.DOTALL,
    )
    if match is None:
        raise RuntimeError("exact cancelbytearithmetic stage bytechunk not found")
    payload = bytes.fromhex(match.group(1))
    if len(payload) != 9024:
        raise RuntimeError(f"expected 9024 witness bytes, got {len(payload)}")
    return payload


def build() -> bytes:
    payload = witness_bytes()
    text = bytes(ENTRY_RVA - TEXT_RVA) + payload

    dos = bytearray(0x40)
    dos[0:2] = b"MZ"
    struct.pack_into("<I", dos, 0x3C, 0x40)

    nsec = 2
    opt_size = 240
    header_len = 0x40 + 4 + 20 + opt_size + 40 * nsec
    headers_size = align(header_len, FILE_ALIGN)
    text_off = headers_size
    text_raw_size = align(len(text), FILE_ALIGN)
    image_size = align(BSS_RVA + BSS_SIZE, SECT_ALIGN)

    result = bytearray(dos)
    result += b"PE\0\0"
    result += struct.pack("<HHIIIHH", 0x8664, nsec, 0, 0, 0, opt_size, 0x0022)

    opt = bytearray()
    opt += struct.pack(
        "<HBBIIIII",
        0x20B,
        14,
        0,
        len(text),
        0,
        BSS_SIZE,
        ENTRY_RVA,
        TEXT_RVA,
    )
    opt += struct.pack("<Q", IMAGE_BASE)
    opt += struct.pack(
        "<IIHHHHHHIIIIHHQQQQII",
        SECT_ALIGN,
        FILE_ALIGN,
        6,
        0,
        0,
        0,
        6,
        0,
        0,
        image_size,
        headers_size,
        0,
        3,
        0x8160,
        0x100000,
        0x1000,
        0x100000,
        0x1000,
        0,
        16,
    )
    opt += bytes(16 * 8)  # empty PE data-directory table
    if len(opt) != opt_size:
        raise RuntimeError(f"optional header is {len(opt)} bytes")
    result += opt

    def section(name: str, virtual_size: int, rva: int, raw_size: int, raw_off: int, chars: int) -> bytes:
        return name.encode().ljust(8, b"\0") + struct.pack(
            "<IIIIIIHHI", virtual_size, rva, raw_size, raw_off, 0, 0, 0, 0, chars
        )

    result += section(".text", len(text), TEXT_RVA, text_raw_size, text_off, 0x60000020)
    result += section(".bss", BSS_SIZE, BSS_RVA, 0, 0, 0xC0000080)
    result += bytes(headers_size - len(result))
    result += text.ljust(text_raw_size, b"\0")
    return bytes(result)


if __name__ == "__main__":
    output = Path(__file__).with_name("pe_subcommuteshift_x86_64.exe")
    output.write_bytes(build())
    print(f"wrote {output} ({output.stat().st_size} bytes)")
