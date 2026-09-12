#!/usr/bin/env python3
"""Regenerate the headerless x86-64 Linux compatibility-syscall fixture."""

from pathlib import Path


# ENDBR64; MOV EAX,1; MOV EBX,0; INT 0x80; RET.
# In a 64-bit Linux process INT 0x80 uses the i386 compatibility ABI, so this
# is SYS_exit(0), not native x86-64 syscall 1 (write).
IMAGE = bytes.fromhex("f3 0f 1e fa b8 01 00 00 00 bb 00 00 00 00 cd 80 c3")


if __name__ == "__main__":
    Path(__file__).with_suffix("").write_bytes(IMAGE)
