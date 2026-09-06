use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

fn symbol(
    name: &str,
    value: u64,
    size: u64,
    section: object::write::SectionId,
    kind: SymbolKind,
) -> Symbol {
    Symbol {
        name: name.as_bytes().to_vec(),
        value,
        size,
        kind,
        scope: if kind == SymbolKind::Text {
            SymbolScope::Linkage
        } else {
            SymbolScope::Compilation
        },
        weak: false,
        section: SymbolSection::Section(section),
        flags: SymbolFlags::None,
    }
}

/// Generate an ELF32 executable with code at 0x10000 and caller-supplied mode metadata.
pub fn elf(code: &[u8], markers: &[(u64, &str)], functions: &[(u64, &str, u64)]) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, code, 4);
    for &(offset, name) in markers {
        obj.add_symbol(symbol(name, offset, 0, text, SymbolKind::Label));
    }
    for &(offset, name, size) in functions {
        obj.add_symbol(symbol(name, offset, size, text, SymbolKind::Text));
    }
    let mut bytes = obj.write().unwrap();
    let get32 = |b: &[u8], p| u32::from_le_bytes(b[p..p + 4].try_into().unwrap());
    let shoff = get32(&bytes, 32) as usize;
    let shnum = u16::from_le_bytes(bytes[48..50].try_into().unwrap()) as usize;
    let mut text_offset = 0;
    for i in 1..shnum {
        let sh = shoff + i * 40;
        if get32(&bytes, sh + 8) & 4 != 0 {
            text_offset = get32(&bytes, sh + 16);
            bytes[sh + 12..sh + 16].copy_from_slice(&0x10000u32.to_le_bytes());
        }
        if get32(&bytes, sh + 4) == 2 {
            let begin = get32(&bytes, sh + 16) as usize;
            let end = begin + get32(&bytes, sh + 20) as usize;
            for sym in (begin..end).step_by(16) {
                if u16::from_le_bytes(bytes[sym + 14..sym + 16].try_into().unwrap()) != 0 {
                    let value = get32(&bytes, sym + 4) + 0x10000;
                    bytes[sym + 4..sym + 8].copy_from_slice(&value.to_le_bytes());
                }
            }
        }
    }
    assert_ne!(text_offset, 0);
    let phoff = bytes.len() as u32;
    bytes[16..18].copy_from_slice(&2u16.to_le_bytes());
    bytes[24..28].copy_from_slice(&0x10000u32.to_le_bytes());
    bytes[28..32].copy_from_slice(&phoff.to_le_bytes());
    bytes[42..44].copy_from_slice(&32u16.to_le_bytes());
    bytes[44..46].copy_from_slice(&1u16.to_le_bytes());
    for word in [
        1,
        0,
        0x10000 - text_offset,
        0x10000 - text_offset,
        phoff,
        phoff,
        5,
        1,
    ] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}

/// A synthetic THUMB COFF object, including a string for the inspection path.
pub fn thumb_coff() -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Coff, Architecture::Arm, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0x07, 0x20, 0x70, 0x47], 4);
    obj.add_symbol(symbol("entry", 0, 4, text, SymbolKind::Text));
    let data = obj.add_section(Vec::new(), b".rdata".to_vec(), SectionKind::ReadOnlyData);
    obj.append_section_data(data, b"synthetic string\0", 1);
    let mut bytes = obj.write().unwrap();
    bytes[..2].copy_from_slice(&0x01c2u16.to_le_bytes());
    bytes
}
