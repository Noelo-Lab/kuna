"""Clear the instruction attributes of one Mach-O object's __TEXT,__const.

LLVM's assembler marks a section S_ATTR_SOME_INSTRUCTIONS when an instruction
is emitted into it, and ld64.lld carries the attribute into the output; Apple's
ld64 links __TEXT,__const without it. GH #1406's fixture is the second layout.
Usage: python3 unflagged_macho_arm64.py OBJECT (rewritten in place)
"""
import struct
import sys

LC_SEGMENT_64 = 0x19
INSTRUCTIONS = 0x80000400  # S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS


def main(path):
    data = bytearray(open(path, "rb").read())
    magic, _, _, _, ncmds, _, _, _ = struct.unpack_from("<IiiIIIII", data, 0)
    assert magic == 0xFEEDFACF, "not a 64-bit little-endian Mach-O"
    at = 32
    for _ in range(ncmds):
        cmd, size = struct.unpack_from("<II", data, at)
        if cmd == LC_SEGMENT_64:
            nsects = struct.unpack_from("<I", data, at + 64)[0]
            for i in range(nsects):
                sec = at + 72 + 80 * i
                if data[sec:sec + 16].rstrip(b"\0") == b"__const" and data[sec + 16:sec + 32].rstrip(b"\0") == b"__TEXT":
                    flags = struct.unpack_from("<I", data, sec + 64)[0]
                    struct.pack_into("<I", data, sec + 64, flags & ~INSTRUCTIONS)
                    open(path, "wb").write(data)
                    return
        at += size
    sys.exit("no __TEXT,__const section")


if __name__ == "__main__":
    main(sys.argv[1])
