"""Execute the authored ARM fixture against independent guard/loop models.

Requires arm-none-eabi-binutils and the Python unicorn package. Assemble
fixture.S at .text=0x10000 before passing the ELF to this script.
"""
import argparse
import itertools
import struct
import subprocess
import tempfile
from pathlib import Path

from unicorn import UC_ARCH_ARM, UC_MODE_ARM, Uc
from unicorn.arm_const import (UC_ARM_REG_LR, UC_ARM_REG_R0, UC_ARM_REG_R1,
                               UC_ARM_REG_R2, UC_ARM_REG_SP)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('elf', type=Path)
    args = parser.parse_args()
    symbols = {}
    for line in subprocess.check_output(['arm-none-eabi-nm', args.elf], text=True).splitlines():
        parts = line.split()
        if len(parts) == 3:
            symbols[parts[2]] = int(parts[0], 16)
    with tempfile.TemporaryDirectory() as directory:
        binary = Path(directory) / 'fixture.bin'
        subprocess.run(['arm-none-eabi-objcopy', '-O', 'binary', args.elf, binary], check=True)
        code = binary.read_bytes()
    memory = {
        0x200000: struct.pack('<II', 0, 0xdead0000),
        0x200010: struct.pack('<II', 1, 0x200100),
        0x200020: struct.pack('<II', 3, 0x200110),
        0x200030: struct.pack('<II', 1, 0x200120),
        0x200100: struct.pack('<III', 0, 13, 29),
        0x200110: struct.pack('<III', 1, 17, 31),
        0x200120: struct.pack('<III', 1, 7, 37),
    }
    cases = 0

    def check(name, arguments, expected, extra=None):
        nonlocal cases
        uc = Uc(UC_ARCH_ARM, UC_MODE_ARM)
        uc.mem_map(0x10000, 0x10000)
        uc.mem_write(0x10000, code)
        uc.mem_map(0x200000, 0x10000)
        uc.mem_map(0x300000, 0x10000)
        for address, data in (memory | (extra or {})).items():
            uc.mem_write(address, data)
        uc.reg_write(UC_ARM_REG_SP, 0x30fff0)
        uc.reg_write(UC_ARM_REG_LR, 0x1fff0)
        uc.reg_write(UC_ARM_REG_R1, 0xdead0000)
        for register, value in zip((UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2), arguments):
            uc.reg_write(register, value)
        uc.emu_start(symbols[name], 0x1fff0, count=10000)
        actual = uc.reg_read(UC_ARM_REG_R0)
        assert actual == expected, (name, arguments, actual, expected)
        cases += 1

    choices = (0, 0x200000, 0x200010, 0x200020, 0x200030)
    values = (0, 0, 0, 17, 7)
    for entity, value in zip(choices, values):
        check('candidate', [entity], value)
        check('nested', [entity], 31 if entity == 0x200020 else 0)
    for byte in range(256):
        extra = {0x200110: struct.pack('<III', byte, 17, 31)}
        check('candidate', [0x200020], 17 if byte else 0, extra)
        check('nested', [0x200020], 31 if byte else 0, extra)
    for count in range(5):
        for indices in itertools.product(range(5), repeat=count):
            entities = [choices[i] for i in indices]
            extra = {0x200200: struct.pack('<' + 'I' * count, *entities) or b'\0'}
            check('scan', [0x200200, count], sum(values[i] for i in indices), extra)
            remembered = next((values[i] for i in reversed(indices) if values[i]), 13)
            check('remember', [0x200200, count, 0x200100], remembered, extra)
    extra = {0x200200: struct.pack('<II', 0x200110, 0x200120)}
    check('unconditional', [0x200200], 17, extra)
    for flag in range(3):
        check('distinct', [flag, 0x200200, 0x200204], 17 if flag else 7, extra)
        for previous in range(32):
            check('retained', [flag, previous], 19 if flag else previous)
    print(f'Authored ARM/model matched {cases} cases')


if __name__ == '__main__':
    main()
