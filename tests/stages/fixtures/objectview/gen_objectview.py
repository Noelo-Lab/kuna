#!/usr/bin/env python3
"""Build the objectview fixtures from objectview.c.

objectview.bin  the linked .text (0x401000) and .rodata (0x402000) as one flat
                image, loaded with `load raw x86:LE:64:default:gcc 0x401000 0x401020`.
objectview.te   the same two sections in a UEFI TE container (x86-64, ImageBase
                0x400000, StrippedSize 0x1d8, entry RVA 0x1020).

Needs clang and ld.lld. Run from this directory: python3 gen_objectview.py
"""
import struct
import subprocess

subprocess.run(["clang", "-O1", "-ffreestanding", "-fno-pic", "-fno-asynchronous-unwind-tables",
                "-nostdlib", "-static", "-fuse-ld=lld", "-Wl,-e,entry", "-Wl,-Ttext=0x401000",
                "-Wl,--section-start=.rodata=0x402000", "-Wl,--no-rosegment",
                "objectview.c", "-o", "objectview.elf"], check=True)
for args in (["-O", "binary", "objectview.elf", "objectview.bin"],
             ["-O", "binary", "-j", ".text", "objectview.elf", "text.part"],
             ["-O", "binary", "-j", ".rodata", "objectview.elf", "rodata.part"]):
    subprocess.run(["llvm-objcopy", *args], check=True)

STRIPPED, BASE = 0x1D8, 0x400000
sections = [(b".text", 0x1000, open("text.part", "rb").read(), 0x60000020),
            (b".rdata", 0x2000, open("rodata.part", "rb").read(), 0x40000040)]
bias = STRIPPED - 40
raw = 0x400
records, body = b"", []
for name, rva, data, flags in sections:
    records += name.ljust(8, b"\0") + struct.pack("<IIII", len(data), rva, len(data), raw)
    records += b"\0" * 12 + struct.pack("<I", flags)
    body.append((raw - bias, data))
    raw = (raw + len(data) + 0x1FF) & ~0x1FF
te = bytearray(b"VZ" + struct.pack("<HBBH", 0x8664, len(sections), 10, STRIPPED)
               + struct.pack("<IIQ", 0x1020, 0x1000, BASE) + b"\0" * 16 + records)
for off, data in body:
    te += b"\0" * (off - len(te))
    te[off:off + len(data)] = data
open("objectview.te", "wb").write(te)
