"""Regenerate md1img_mipsel_test.img: a MediaTek md1img with a hand-assembled
MIPS32 LE md1rom (scale at 0x90000000, modem_main at 0x90000018) and an
LZMA md1_dbginfo naming both."""
import lzma, struct

WORDS = [0x00041040, 0x00441021, 0x3C039001, 0x8C630000, 0x03E00008, 0x00431021,
         0x27BDFFE8, 0xAFBF0014, 0x3C039001, 0x8C620000, 0x00441021, 0x0C000000,
         0xAC620000, 0x8FBF0014, 0x3842005A, 0x03E00008, 0x27BD0018]


def section(name, data, align=16, hdr=0x200):
    h = struct.pack("<II32sIIIIIIIIII", 0x58881688, len(data), name.encode(), 0, 0xFFFFFFFF,
                    0x58891689, hdr, 1, 0, 0, align, 0, 0)
    return h + b"\0" * (hdr - len(h)) + data + b"\0" * ((align - len(data) % align) % align)


rom = b"".join(struct.pack("<I", w) for w in WORDS)
syms = b"".join(n.encode() + b"\0" + struct.pack("<II", a, a + 0x18)
                for n, a in [("scale", 0x90000000), ("modem_main", 0x90000018)])
body = b"\0" * 0x1C + b"MT6789_SP\0mt6789\0MOLY.TEST.01\0 2026/10/30\0"
body += struct.pack("<II", 0, len(body) + 8 + len(syms)) + syms
img = section("md1rom", rom) + section("md1_dbginfo", lzma.compress(body, format=lzma.FORMAT_ALONE)) \
    + section("md1drdi", b"\x11" * 40)
open("md1img_mipsel_test.img", "wb").write(img)
