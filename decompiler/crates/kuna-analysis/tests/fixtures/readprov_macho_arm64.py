"""Round __LINKEDIT's vmsize up to the 16 KiB page, as Apple's ld64 writes it.

ld64.lld sizes __LINKEDIT's memory to its file content exactly; Apple's ld64
rounds it to the page, so the segment declares more memory than the file holds.
GH #1401's fixture is the second layout. Usage: python3 readprov_macho_arm64.py IN OUT
"""
import struct
import sys

LC_SEGMENT_64 = 0x19
PAGE = 0x4000


def main(src, dst):
    data = bytearray(open(src, "rb").read())
    magic, _, _, _, ncmds, _, _, _ = struct.unpack_from("<IiiIIIII", data, 0)
    assert magic == 0xFEEDFACF, "not a 64-bit little-endian Mach-O"
    at = 32
    for _ in range(ncmds):
        cmd, size = struct.unpack_from("<II", data, at)
        if cmd == LC_SEGMENT_64 and data[at + 8:at + 24].rstrip(b"\0") == b"__LINKEDIT":
            filesize = struct.unpack_from("<Q", data, at + 48)[0]
            struct.pack_into("<Q", data, at + 32, (filesize + PAGE - 1) // PAGE * PAGE)
            open(dst, "wb").write(data)
            return
        at += size
    sys.exit("no __LINKEDIT segment")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
