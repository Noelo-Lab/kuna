#!/usr/bin/env python3
"""Generate `pyinstaller_cookie_x86_64`: a small ELF with a PyInstaller CArchive
cookie appended (RE-need `rthints`).

The host is `arraycoverwidth_x86_64` unchanged; the tail is the 88-byte cookie
a PyInstaller >= 2.1 bootloader searches for: the `MEI\\x0c\\x0b\\x0a\\x0b\\x0e`
magic, the package length, TOC offset and TOC length (all big-endian), the
Python version as `major*100+minor` (312), and a 64-byte library name.

    python3 pyinstaller_cookie_x86_64.py
"""
import os
import struct

HERE = os.path.dirname(os.path.abspath(__file__))

host = open(os.path.join(HERE, "arraycoverwidth_x86_64"), "rb").read()
cookie = b"MEI\x0c\x0b\x0a\x0b\x0e" + struct.pack(">IIII", 88, 0, 0, 312)
cookie += b"libpython3.12.so".ljust(64, b"\0")
out = os.path.join(HERE, "pyinstaller_cookie_x86_64")
with open(out, "wb") as f:
    f.write(host + cookie)
os.chmod(out, 0o755)
