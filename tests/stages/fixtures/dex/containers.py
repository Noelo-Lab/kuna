#!/usr/bin/env python3
"""Build the DEX container fixtures of tests/stages/kuna-dex-containers.xml.

  python3 containers.py        # writes hello.odex, hello.vdex, hello.oat, tries.cdex

hello.odex wraps hello.dex in a 40-byte DexOptHeader; hello.vdex is a version
027 VDEX (Android 12) whose DEX section holds hello.dex; hello.oat is an ELF
shared object (gcc) whose `oatdata` starts with an `oat\n088\0` header and
embeds hello.dex; tries.cdex is tries.dex rewritten as compact DEX
(libdexfile/dex/compact_dex_file.h): the 0x88-byte header, the data section
starting at 0x18, and each code item packed into four 4-bit sizes and an
11-bit instruction count, with the preheader ART writes for larger values.
"""
import os
import struct
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))


def uleb(b, p):
    r = s = 0
    while True:
        x = b[p]
        p += 1
        r |= (x & 0x7F) << s
        s += 7
        if x < 0x80:
            return r, p


def uleb_enc(v):
    out = bytearray()
    while True:
        b = v & 0x7F
        v >>= 7
        if v:
            out.append(b | 0x80)
        else:
            out.append(b)
            return bytes(out)


def dex_to_cdex(src):
    shift = 0x18
    new = bytearray(src[:0x70] + bytes(shift) + src[0x70:])
    new[0:8] = b"cdex001\0"
    struct.pack_into("<I", new, 0x24, 0x88)
    for o in (0x38, 0x40, 0x48, 0x50, 0x58, 0x60):
        off = struct.unpack_from("<I", src, o + 4)[0]
        if off:
            struct.pack_into("<I", new, o + 4, off + shift)
    struct.pack_into("<I", new, 0x6C, shift)
    count, cdoff = struct.unpack_from("<II", src, 0x60)
    appended = bytearray()
    for i in range(count):
        data = struct.unpack_from("<I", src, cdoff + 32 * i + 24)[0]
        if not data:
            continue
        p = data
        sf, p = uleb(src, p)
        inf, p = uleb(src, p)
        dm, p = uleb(src, p)
        vm, p = uleb(src, p)
        enc = bytearray(uleb_enc(sf) + uleb_enc(inf) + uleb_enc(dm) + uleb_enc(vm))
        for _ in range(sf + inf):
            a, p = uleb(src, p)
            b, p = uleb(src, p)
            enc += uleb_enc(a) + uleb_enc(b)
        for group in (dm, vm):
            for _ in range(group):
                a, p = uleb(src, p)
                flags, p = uleb(src, p)
                code, p = uleb(src, p)
                if code:
                    regs, ins, outs, tries = struct.unpack_from("<HHHH", src, code)
                    units = struct.unpack_from("<I", src, code + 12)[0]
                    regs -= ins
                    fields = ((regs & 0xF) << 12) | ((ins & 0xF) << 8) | ((outs & 0xF) << 4) | (tries & 0xF)
                    low = units & 0x7FF
                    pre_flags = 0
                    pre = []
                    if units - low:
                        pre_flags |= 0x10
                        pre += [(units - low) & 0xFFFF, (units - low) >> 16]
                    for value, flag in ((regs & ~0xF, 1), (ins & ~0xF, 2), (outs & ~0xF, 4), (tries & ~0xF, 8)):
                        if value:
                            pre_flags |= flag
                            pre.append(value)
                    item = code + shift + 12
                    struct.pack_into("<HH", new, item, fields, (low << 5) | pre_flags)
                    q = item
                    for value in pre:
                        q -= 2
                        struct.pack_into("<H", new, q, value)
                    code += 12
                enc += uleb_enc(a) + uleb_enc(flags) + uleb_enc(code)
        while (len(new) + len(appended)) % 4:
            appended.append(0)
        struct.pack_into("<I", new, cdoff + shift + 32 * i + 24, len(new) + len(appended) - shift)
        appended += enc
    new += appended
    struct.pack_into("<I", new, 0x20, len(new))
    struct.pack_into("<I", new, 0x68, len(new) - shift)
    return bytes(new)


def main():
    hello = open(os.path.join(HERE, "hello.dex"), "rb").read()
    tries = open(os.path.join(HERE, "tries.dex"), "rb").read()
    n = len(hello)
    odex = b"dey\n036\0" + struct.pack("<8I", 40, n, 40 + n, 0, 40 + n, 0, 0, 0) + hello
    sections = 4
    dexoff = 12 + 12 * sections + 4
    vdex = (
        b"vdex027\0"
        + struct.pack("<I", sections)
        + struct.pack("<III", 0, 12 + 12 * sections, 4)
        + struct.pack("<III", 1, dexoff, n)
        + struct.pack("<III", 2, dexoff + n, 0)
        + struct.pack("<III", 3, dexoff + n, 0)
        + struct.pack("<I", 0)
        + hello
    )
    asm = (
        "    .section .rodata\n    .balign 4096\n    .globl oatdata\n    .type oatdata, @object\n"
        'oatdata:\n    .ascii "oat\\n088\\0"\n    .long 0, 0, 0, 0\n    .balign 4\n'
        '    .incbin "hello.dex"\n    .size oatdata, . - oatdata\n'
    )
    for name, data in (("hello.odex", odex), ("hello.vdex", vdex), ("tries.cdex", dex_to_cdex(tries))):
        open(os.path.join(HERE, name), "wb").write(data)
    subprocess.run(
        ["gcc", "-shared", "-nostdlib", "-x", "assembler", "-", "-o", "hello.oat"],
        input=asm.encode(), cwd=HERE, check=True,
    )


if __name__ == "__main__":
    main()
