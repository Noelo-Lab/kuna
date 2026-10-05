"""Generate an authored PE32+ with a readonly format and an imported variadic call.

The entry point loads, increments and stores *RCX, then calls the IAT with EDX.
No Windows SDK, DLL, application image or external asset is needed.
"""
import struct
import sys
from pathlib import Path

image = bytearray(0x600)


def put(offset, format, *values):
    struct.pack_into("<" + format, image, offset, *values)


image[:2] = b"MZ"
put(0x3c, "I", 0x80)
image[0x80:0x84] = b"PE\0\0"
put(0x84, "HHIIIHH", 0x8664, 2, 0, 0, 0, 0xf0, 0x22)
optional = 0x98
put(optional, "H", 0x20b)
put(optional + 4, "III", 0x200, 0x200, 0)
put(optional + 16, "IIQ", 0x1000, 0x1000, 0x140000000)
put(optional + 32, "II", 0x1000, 0x200)
put(optional + 40, "HHHHHH", 6, 0, 0, 0, 6, 0)
put(optional + 56, "III", 0x3000, 0x200, 0)
put(optional + 68, "HH", 3, 0)
put(optional + 72, "QQQQ", 0x100000, 0x1000, 0x100000, 0x1000)
put(optional + 108, "I", 16)
put(optional + 112 + 8, "II", 0x2000, 40)
put(optional + 112 + 12 * 8, "II", 0x2050, 16)
for offset, name, rva, raw, flags in [
    (0x188, b".text", 0x1000, 0x200, 0x60000020),
    (0x1b0, b".rdata", 0x2000, 0x400, 0x40000040),
]:
    image[offset:offset + len(name)] = name
    put(offset + 8, "IIIIIIHHI", 0x200, rva, 0x200, raw, 0, 0, 0, 0, flags)

code = bytes.fromhex("4883ec288b1183c2018911488d0d8e100000ff15381000004883c428c3")
image[0x200:0x200 + len(code)] = code
put(0x400, "IIIII", 0x2040, 0, 0, 0x2090, 0x2050)
put(0x440, "QQ", 0x2080, 0)
put(0x450, "QQ", 0x2080, 0)
name = b"render_value\0"
image[0x482:0x482 + len(name)] = name
image[0x490:0x49b] = b"reader.dll\0"
image[0x4a0:0x4a3] = b"%i\0"
Path(sys.argv[1]).write_bytes(image)
