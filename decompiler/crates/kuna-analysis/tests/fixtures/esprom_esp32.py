#!/usr/bin/env python3
"""Write esprom_esp32.bin: a hand-assembled ESP32 application image.

Two segments, as `esptool elf2image` lays them out: DROM (the application
descriptor magic and a string) and IRAM (the code below). The code calls the
ROM's memcpy (0x4000c2c8) through a literal and reads the UART0 status
register (0x3ff4001c) twice.

Run: python3 esprom_esp32.py   (writes esprom_esp32.bin beside this file)
"""

import pathlib
import struct

IRAM = 0x40080000
DROM = 0x3F400020

CODE = bytes.fromhex(
    "c8c20040"  # 40080000  .long 0x4000c2c8          ; ROM memcpy
    "1c00f43f"  # 40080004  .long 0x3ff4001c          ; UART0 status
    "f02000f020003df0"  # 40080008  nop padding
    "364100"    # 40080010  start: entry a1, 32
    "e50000"    # 40080013  call8 copy16
    "a50200"    # 40080016  call8 uart_status
    "1df0"      # 40080019  retw.n
    "f020003df0"  # 4008001b  nop padding
    "364100"    # 40080020  copy16: entry a1, 32
    "ad02"      # 40080023  mov.n a10, a2
    "bd03"      # 40080025  mov.n a11, a3
    "1c0c"      # 40080027  movi.n a12, 16
    "81f5ff"    # 40080029  l32r a8, 0x40080000
    "e00800"    # 4008002c  callx8 a8
    "2d0a"      # 4008002f  mov.n a2, a10
    "1df0"      # 40080031  retw.n
    "f02000f02000f02000f0200000"  # 40080033  padding
    "364100"    # 40080040  uart_status: entry a1, 32
    "81f0ff"    # 40080043  l32r a8, 0x40080004
    "9808"      # 40080046  l32i.n a9, a8, 0
    "2808"      # 40080048  l32i.n a2, a8, 0
    "9a22"      # 4008004a  add.n a2, a2, a9
    "1df0"      # 4008004c  retw.n
    "0000"      # 4008004e  padding
)

RODATA = struct.pack("<I", 0xABCD5432) + b"esprom fixture\0\0"


def segment(addr, data):
    return struct.pack("<II", addr, len(data)) + data


def main():
    assert len(CODE) == 0x50, hex(len(CODE))
    header = struct.pack("<BBBBI", 0xE9, 2, 2, 0x20, IRAM + 0x10)
    extended = struct.pack("<B3xHB8xB", 0xEE, 0x0000, 0, 0)  # chip id 0: ESP32
    image = header + extended + segment(DROM, RODATA) + segment(IRAM, CODE)
    image += b"\0" * (-(len(image) + 1) % 16)
    checksum = 0xEF
    for data in (RODATA, CODE):
        for byte in data:
            checksum ^= byte
    image += bytes([checksum])
    (pathlib.Path(__file__).parent / "esprom_esp32.bin").write_bytes(image)


if __name__ == "__main__":
    main()
