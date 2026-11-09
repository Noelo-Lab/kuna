"""Builds the hand-made zip fixtures for the kuna-jvm F6 tests.

usage: python3 -I make_zips.py <work dir with base/ and v11/ classes> <out dir>
"""
import io
import os
import struct
import sys
import zipfile
import zlib

work, out = sys.argv[1], sys.argv[2]
STAMP = (1980, 1, 1, 0, 0, 0)


def cls(*parts):
    with open(os.path.join(work, *parts), "rb") as f:
        return f.read()


HELLO = cls("base", "pkg", "Hello.class")
HELLO1 = cls("base", "pkg", "Hello$1.class")
UTIL = cls("base", "pkg", "util", "Util.class")
UTIL11 = cls("v11", "pkg", "util", "Util.class")


def build(entries, comment=b""):
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w") as z:
        for name, data, method in entries:
            info = zipfile.ZipInfo(name, STAMP)
            info.compress_type = method
            if name.endswith("/"):
                info.external_attr = 0o40755 << 16 | 0x10
            z.writestr(info, data)
        z.comment = comment
    return buf.getvalue()


def write(name, data):
    with open(os.path.join(out, name), "wb") as f:
        f.write(data)


S, D = zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED
inner = build([("q/Inner.txt", b"inner\n", D)])
write("inner.jar", inner)

write("mixed.jar", build([
    ("META-INF/", b"", S),
    ("META-INF/MANIFEST.MF", b"Manifest-Version: 1.0\r\n\r\n", D),
    ("pkg/", b"", S),
    ("pkg/Hello.class", HELLO, D),
    ("pkg/Hello$1.class", HELLO1, S),
    ("pkg/util/Util.class", UTIL, D),
    ("res/data.txt", b"some data\n" * 40, S),
    ("res/empty.bin", b"", S),
    ("res/deep/er/x.properties", b"k=v\n", D),
    ("ünïcödé/naïve.txt", "naïve\n".encode(), D),
    ("libs/inner.jar", inner, S),
    ("top.class.txt", b"not a class\n", S),
]))

write("mr.jar", build([
    ("META-INF/MANIFEST.MF", b"Manifest-Version: 1.0\r\nMulti-Release: true\r\n\r\n", D),
    ("pkg/util/Util.class", UTIL, D),
    ("META-INF/versions/", b"", S),
    ("META-INF/versions/11/pkg/util/Util.class", UTIL11, D),
    ("META-INF/versions/+10/pkg/Plus.class", UTIL, S),
    ("META-INF/versions/-1/pkg/Neg.class", UTIL, S),
    ("META-INF/versions/x/pkg/Odd.class", UTIL, S),
    ("META-INF/versions/٩/pkg/Arabic.class", UTIL, S),
    ("META-INF/versions/2147483648/pkg/Big.class", UTIL, S),
    ("META-INF/versions/17/res.txt", b"seventeen\n", D),
    ("META-INF/versions/11", b"no slash\n", S),
]))

write("badver.jar", build([
    ("pkg/util/Util.class", UTIL, D),
    ("META-INF/versions/8/pkg/util/Util.class", UTIL, D),
]))

write("dup.zip", build([
    ("x.txt", b"first\n", S),
    ("y/", b"", S),
    ("x.txt", b"second, deflated\n", D),
    ("y", b"plain y\n", S),
]))

stub = b"#!/bin/sh\necho self-extracting stub\nexit 0\n"
write("prefixed.zip", stub + build([("a.txt", b"after the stub\n", D), ("b/c.txt", b"c\n", S)]))
write("trailing.zip", build([("t.txt", b"trailing junk follows\n", D)]) + b"\0JUNK" * 20)
write("comment.zip", build([("c.txt", b"commented\n", D)], comment=b"an archive comment"))


def raw_deflate(data):
    c = zlib.compressobj(9, zlib.DEFLATED, -15)
    return c.compress(data) + c.flush()


def zip64(entries):
    body, cen = bytearray(), bytearray()
    for name, data, method in entries:
        nb = name.encode()
        comp = data if method == S else raw_deflate(data)
        crc = zlib.crc32(data)
        off = len(body)
        lextra = struct.pack("<HHQQ", 1, 16, len(data), len(comp))
        body += struct.pack("<IHHHHHIIIHH", 0x04034B50, 45, 0, method, 0, 0x21, crc,
                            0xFFFFFFFF, 0xFFFFFFFF, len(nb), len(lextra)) + nb + lextra + comp
        cextra = struct.pack("<HHQQQ", 1, 24, len(data), len(comp), off)
        cen += struct.pack("<IHHHHHHIIIHHHHHII", 0x02014B50, 45, 45, 0, method, 0, 0x21, crc,
                           0xFFFFFFFF, 0xFFFFFFFF, len(nb), len(cextra), 0, 0, 0, 0,
                           0xFFFFFFFF) + nb + cextra
    cenoff = len(body)
    body += cen
    end64 = len(body)
    body += struct.pack("<IQHHIIQQQQ", 0x06064B50, 44, 45, 45, 0, 0, len(entries), len(entries),
                        len(cen), cenoff)
    body += struct.pack("<IIQI", 0x07064B50, 0, end64, 1)
    body += struct.pack("<IHHHHIIH", 0x06054B50, 0, 0, 0xFFFF, 0xFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0)
    return bytes(body)


write("zip64.zip", zip64([("a.txt", b"hello zip64\n", S), ("b/c.txt", b"deflated " * 30, D)]))
