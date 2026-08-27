#!/usr/bin/env python3
"""Generate ``te_entrythumbflow_arm.te``: a synthetic UEFI TE image, machine
``IMAGE_FILE_MACHINE_ARM`` (0x1c0), whose ``AddressOfEntryPoint`` carries the
Thumb bit and whose one ``.text`` section holds a Thumb function followed by
an A32 one.

The layout is the one every kuna TE test shares (``loadimage_te::synthetic``
in ``kuna-analysis``): ``StrippedSize`` 0x1d8, ``ImageBase`` 0x400000,
``.text`` at RVA 0x1000 with ``PointerToRawData`` 0x1000, so the code lands at
file offset 0xe50 and address 0x401000. Regenerate with
``python3 te_entrythumbflow_arm.py``; the bytes are deterministic.

    0x401000  Thumb  movs r0, #7 ; bx lr      (07 20 70 47)
    0x401004  A32    bx lr                    (1e ff 2f e1)
"""
import struct

STRIPPED_SIZE = 0x1D8
IMAGE_BASE = 0x400000
CODE_RVA = 0x1000
CODE_RAW_POINTER = 0x1000
CODE_FILE_OFFSET = CODE_RAW_POINTER - (STRIPPED_SIZE - 40)
CODE = bytes([0x07, 0x20, 0x70, 0x47, 0x1E, 0xFF, 0x2F, 0xE1])


def build() -> bytes:
    image = bytearray(CODE_FILE_OFFSET + len(CODE))
    image[0:2] = b"VZ"
    struct.pack_into("<H", image, 2, 0x01C0)  # IMAGE_FILE_MACHINE_ARM
    image[4] = 1  # NumberOfSections
    image[5] = 10  # IMAGE_SUBSYSTEM_EFI_APPLICATION
    struct.pack_into("<H", image, 6, STRIPPED_SIZE)
    struct.pack_into("<I", image, 8, CODE_RVA | 1)  # AddressOfEntryPoint, Thumb bit set
    struct.pack_into("<I", image, 12, CODE_RVA)  # BaseOfCode
    struct.pack_into("<Q", image, 16, IMAGE_BASE)
    image[40:45] = b".text"
    struct.pack_into("<I", image, 48, len(CODE))  # VirtualSize
    struct.pack_into("<I", image, 52, CODE_RVA)  # VirtualAddress
    struct.pack_into("<I", image, 56, len(CODE))  # SizeOfRawData
    struct.pack_into("<I", image, 60, CODE_RAW_POINTER)  # PointerToRawData
    struct.pack_into("<I", image, 76, 0x60000020)  # CODE | EXECUTE | READ
    image[CODE_FILE_OFFSET:] = CODE
    return bytes(image)


if __name__ == "__main__":
    with open("te_entrythumbflow_arm.te", "wb") as out:
        out.write(build())
