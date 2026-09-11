#!/usr/bin/env python3
"""Generate `phimarkerrun_pe_i386.exe`: a minimal PE32 carrying the protected
stub at `0x41cd08` from the reported image (RE-need
`block-processing-panics-out`, round-3 challenge 629a286b33c5d45b75903c7a).

The 116 bytes are the packer stub verbatim, so they keep the absolute
self-references (`0x41cd4c`, `0x41cd50`, `0x41cd54`, `0x41cd5d`, `0x41cd70`)
that make the block layout what it is -- which is why the section is mapped at
`0x41c000` rather than at the usual `0x401000`.  It is a read/write/execute
section with neither content flag set, the shape the original packer section
`.afa` has.

    0041cd08  .afa  entry  pop [0x41cd5d] / push eax / push ecx / pushfd /
                           three xchg-decodes / popfd / pop ecx / pop eax /
                           the decoded tail, `into`, and `ret 8`

`kuna decompile <exe> 0x41cd08 --addr` on the unpatched tree exits 1 with
`index out of bounds` out of `substrate/block.rs`: the stack-pointer solve
rewrites one of the block's MULTIEQUALs into an `INT_ADD` in place, and the
unreachable-block sweep then stops resyncing phi arity at that op.

    python3 phimarkerrun_pe_i386.py
"""
import os
import struct
import sys

IMAGE_BASE = 0x400000
SECT_ALIGN = 0x1000
FILE_ALIGN = 0x200
DIRS = 16
OPT_SIZE = 0xE0

TEXT_RVA = 0x1C000
TEXT_SIZE = 0x1000
ENTRY_RVA = 0x1C000
STUB_RVA = 0x1CD08
STUB_OFF = STUB_RVA - TEXT_RVA

# MEM_EXECUTE | MEM_READ | MEM_WRITE, no content flag -- the packer's own.
SECT_CHARS = 0xE0000000

STUB = bytes.fromhex(
    '8f055dcd410050519cb870cd41008b0881f121000000870d4ccd410083c004'
    '8b0881f194000000870d50cd410083c0048b0881f19a000000870d54cd4100'
    '83c0049d595803da2eb6b14d32449a77ce119051549cb9c57d96678d490587'
    '0c24518b4c240c9dc20800ac45e4506b156c50da0090908f05bccd41005051'
    '9cb8cfcd41008b0881f15e000000870dafcd410083c0048b0881f112000000'
    '870db3cd410083c0049d5958080f4eb848760c9c9051549cb9dcf696338d49'
    '01870c24518b4c240c9dc20800ddc40883eaff90908f0530ce410050519cb8'
    '43ce41008b0881f141000000870d1fce410083c0048b0881f187000000870d'
    '23ce410083c0048b0881f191000000870d27ce410083c0049d595890909090'
    '083c6c21ff9440602e2c93a29051549cb97fcffb708d4900870c24518b4c24'
    '0c9dc208004e84b25d79ff8b75759090908f05a4ce410050519cb8b7ce4100'
    '8b0881f1a1000000870d93ce410083c0048b0881f13a000000870d97ce4100'
    '83c0048b0881f1c5000000870d9bce410083c0049d59581ccf9822c4610816'
    '6ba718f59051549cb9146eed568d4903870c24518b4c240c9dc208002875ec'
    '56fc45fc01559090908f0503cf410050519cb816cf41008b0881f1e7000000'
    '870df6ce410083c0048b0881f102000000870dface410083c0049d5958845e'
    'edf1fd804a389051549cb954f38ed68d4900870c24518b4c240c9dc208000f'
    '1466fefd9090908f055ecf410050519cb871cf41008b0881f1e1000000870d'
    '51cf410083c0048b0881f1d2000000870d55cf410083c0049d5958a52ba4b7'
    'f1e042819051549cb9e03d2cbf8d4900870c24518b4c240c9dc208006a0683'
    'c4d69090908f05b9cf410050519cb8cccf41008b0881f19a000000870daccf'
    '410083c0048b0881f176000000870db0cf410083c0049d595882be5cc52f24'
    'bc769051549cb9069a6fb18d4900870c24518b4c240c9dc2080011ceff5072'
    '9090908f0529d0410050519cb83cd041008b0881f139000000870d18d04100'
    '83c0048b0881f170000000870d1cd041'
)


def build_entry():
    """Call both protected stubs so each gets a start and the first is bounded
    by the second -- in the reported image the two are reached the same way."""
    here = IMAGE_BASE + ENTRY_RVA
    first = struct.pack('<i', (IMAGE_BASE + STUB_RVA) - (here + 9))
    second = struct.pack('<i', (IMAGE_BASE + 0x1CD7C) - (here + 14))
    return (b'\x6A\x00\x6A\x00' + b'\xE8' + first +
            b'\xE8' + second + b'\xC3')


def build_section():
    body = bytearray(TEXT_SIZE)
    entry = build_entry()
    body[ENTRY_RVA - TEXT_RVA:ENTRY_RVA - TEXT_RVA + len(entry)] = entry
    body[STUB_OFF:STUB_OFF + len(STUB)] = STUB
    return bytes(body)


def build():
    section = build_section()
    headers_size = 0x400
    out = bytearray()
    out += b'MZ' + bytes(0x3A)
    out += struct.pack('<I', 0x40)
    out += b'PE\0\0'
    out += struct.pack('<HHIIIHH', 0x14C, 1, 0, 0, 0, OPT_SIZE, 0x0102)
    out += struct.pack('<HBBIIIII', 0x10B, 0, 0, TEXT_SIZE, 0, 0,
                       ENTRY_RVA, TEXT_RVA)
    out += struct.pack('<IIIHHHHHHIIIIHH', TEXT_RVA, IMAGE_BASE, SECT_ALIGN,
                       FILE_ALIGN, 4, 0, 0, 0, 4, 0, 0,
                       TEXT_RVA + SECT_ALIGN, headers_size, 0, 2)
    out += struct.pack('<IIIIIII', 0x100000, 0x1000, 0x100000, 0x1000, 0,
                       DIRS, 0)
    out += bytes(8 * DIRS)
    out += b'.afa\0\0\0\0'
    out += struct.pack('<IIII', TEXT_SIZE, TEXT_RVA, TEXT_SIZE, headers_size)
    out += struct.pack('<IIHHI', 0, 0, 0, 0, SECT_CHARS)
    out += bytes(headers_size - len(out))
    out += section
    return bytes(out)


def main():
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                        'phimarkerrun_pe_i386.exe')
    with open(path, 'wb') as fh:
        fh.write(build())
    print(f'wrote {path} ({os.path.getsize(path)} bytes)', file=sys.stderr)


if __name__ == '__main__':
    main()
