//! Toolchain-free ELF32/BE executable: main calls two 12-byte SPARC PLT slots.

pub const PLT_OFFSET: usize = 0x200;
pub const PLT_VMA: u32 = 0x10200;
pub const SECTION_HEADERS: usize = 0x400;

pub fn put32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
}

fn put16(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
}

/// `machine` is EM_SPARC (2) or EM_SPARC32PLUS (18). Like GNU ld, .plt has
/// sh_entsize=0: the ELF class, not an optional section hint, determines its ABI.
pub fn elf(machine: u16) -> Vec<u8> {
    let mut bytes = vec![0; SECTION_HEADERS + 7 * 40];
    bytes[..7].copy_from_slice(b"\x7fELF\x01\x02\x01");
    put16(&mut bytes, 16, 2); // ET_EXEC
    put16(&mut bytes, 18, machine);
    put32(&mut bytes, 20, 1);
    put32(&mut bytes, 24, 0x10100);
    put32(&mut bytes, 28, 52);
    put32(&mut bytes, 32, SECTION_HEADERS as u32);
    put32(&mut bytes, 36, if machine == 18 { 0x100 } else { 0 });
    for (offset, value) in [(40, 52), (42, 32), (44, 1), (46, 40), (48, 7), (50, 1)] {
        put16(&mut bytes, offset, value);
    }
    let length = bytes.len() as u32;
    for (index, value) in [1, 0, 0x10000, 0x10000, length, length, 7, 0x1000].into_iter().enumerate() {
        put32(&mut bytes, 52 + index * 4, value);
    }

    // save %sp,-96,%sp; mov %i0,%o0; call puts; nop;
    // mov %i0,%o0; call printf; nop; ret; restore.
    for (index, word) in [
        0x9de3bfa0, 0x90100018, 0x4000004a, 0x01000000,
        0x90100018, 0x4000004a, 0x01000000, 0x81c7e008, 0x81e80000,
    ].into_iter().enumerate() {
        put32(&mut bytes, 0x100 + 4 * index, word);
    }
    for index in 0..2 {
        let offset = 48 + 12 * index;
        put32(&mut bytes, PLT_OFFSET + offset, 0x03000000 | offset as u32);
        put32(&mut bytes, PLT_OFFSET + offset + 4, 0x30bffff3 - 3 * index as u32);
        put32(&mut bytes, PLT_OFFSET + offset + 8, 0x01000000);
    }

    let names = b"\0.shstrtab\0.text\0.plt\0.dynstr\0.dynsym\0.rela.plt\0";
    bytes[0x280..0x280 + names.len()].copy_from_slice(names);
    let strings = b"\0main\0puts\0printf\0";
    bytes[0x2c0..0x2c0 + strings.len()].copy_from_slice(strings);
    for (index, name, value, size, section) in [
        (1, 1, 0x10100, 36, 2), (2, 6, 0, 0, 0), (3, 11, 0, 0, 0),
    ] {
        let offset = 0x300 + index * 16;
        put32(&mut bytes, offset, name);
        put32(&mut bytes, offset + 4, value);
        put32(&mut bytes, offset + 8, size);
        bytes[offset + 12] = 0x12; // STB_GLOBAL | STT_FUNC
        put16(&mut bytes, offset + 14, section);
    }
    for index in 0..2 {
        put32(&mut bytes, 0x350 + index * 12, PLT_VMA + 48 + index as u32 * 12);
        put32(&mut bytes, 0x354 + index * 12, ((index as u32 + 2) << 8) | 21);
    }
    // sh_name, type, flags, addr, offset, size, link, info, align, entsize.
    for (index, header) in [
        [1, 3, 0, 0, 0x280, names.len() as u32, 0, 0, 1, 0],
        [11, 1, 6, 0x10100, 0x100, 36, 0, 0, 4, 0],
        [17, 1, 7, PLT_VMA, PLT_OFFSET as u32, 72, 0, 0, 4, 0],
        [22, 3, 2, 0x102c0, 0x2c0, strings.len() as u32, 0, 0, 1, 0],
        [30, 11, 2, 0x10300, 0x300, 64, 4, 1, 4, 16],
        [38, 4, 2, 0x10350, 0x350, 24, 5, 3, 4, 12],
    ].into_iter().enumerate() {
        for (field, value) in header.into_iter().enumerate() {
            put32(&mut bytes, SECTION_HEADERS + (index + 1) * 40 + field * 4, value);
        }
    }
    bytes
}
