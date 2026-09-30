//! Synthetic ELF32 aliases and an import sharing the definition's name.
//! A test-time generator, not a fixture source: included by the loader's
//! `alias_tests` and by `kuna-cli/tests/elf_symbol_aliases.rs`.
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

fn get32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}
fn put32(b: &mut [u8], at: usize, n: u32) {
    b[at..at + 4].copy_from_slice(&n.to_le_bytes());
}

pub fn image(veneer: bool, reverse: bool, linked: bool) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut bytes: Vec<u8> = if veneer {
        // ldr ip,[pc,#4]; add ip,ip,pc; bx ip; relative Thumb address; movs r0,#7; bx lr.
        [0xe59fc004u32, 0xe08cc00f, 0xe12fff1c, 5]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect()
    } else {
        [0xe3a00007u32, 0xe12fff1e]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect()
    };
    if veneer {
        bytes.extend_from_slice(&[7, 0x20, 0x70, 0x47]);
    }
    object.append_section_data(text, &bytes, 4);
    let mut names = vec![
        ("__answer_from_arm", SymbolScope::Compilation),
        ("answer", SymbolScope::Linkage),
        ("answer_alias", SymbolScope::Linkage),
    ];
    if reverse {
        names.reverse();
    }
    let mut answer = None;
    for (name, scope) in names {
        let id = object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: 0,
            size: if veneer { 16 } else { 8 },
            kind: SymbolKind::Text,
            scope,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
        if name == "answer" {
            answer = Some(id);
        }
    }
    if veneer {
        object.add_symbol(Symbol {
            name: b"__real_answer".to_vec(),
            value: 17,
            size: 4,
            kind: SymbolKind::Text,
            scope: SymbolScope::Compilation,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    if !linked {
        return object.write().unwrap();
    }
    let plt = object.add_section(Vec::new(), b".plt".to_vec(), SectionKind::Text);
    // add ip,pc,#0; add ip,ip,#0; ldr pc,[ip,#0xff8]! -> GOT 0x3000.
    let stub: Vec<u8> = [0xe28fc000u32, 0xe28cc000, 0xe5bcfff8]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    object.append_section_data(plt, &stub, 4);
    let got = object.add_section(Vec::new(), b".got".to_vec(), SectionKind::Data);
    object.append_section_data(got, &[0; 4], 4);
    object
        .add_relocation(
            got,
            Relocation {
                offset: 0,
                symbol: answer.unwrap(),
                addend: 0,
                flags: object::RelocationFlags::Elf {
                    r_type: object::elf::R_ARM_JUMP_SLOT,
                },
            },
        )
        .unwrap();
    object.add_section(
        Vec::new(),
        b".dynsym".to_vec(),
        SectionKind::Elf(object::elf::SHT_DYNSYM),
    );
    let mut b = object.write().unwrap();
    let shoff = get32(&b, 32) as usize;
    let shnum = u16::from_le_bytes(b[48..50].try_into().unwrap()) as usize;
    b[16..18].copy_from_slice(&object::elf::ET_DYN.to_le_bytes());
    put32(&mut b, 24, 0x1000);
    let mut symtab = 0;
    let mut dynsym = 0;
    for i in 1..shnum {
        let sh = shoff + i * 40;
        let kind = get32(&b, sh + 4);
        if i <= 3 {
            put32(&mut b, sh + 12, i as u32 * 0x1000);
        }
        if kind == object::elf::SHT_DYNSYM {
            dynsym = i;
        }
        if kind == object::elf::SHT_SYMTAB {
            symtab = sh;
            let begin = get32(&b, sh + 16) as usize;
            let end = begin + get32(&b, sh + 20) as usize;
            for at in (begin..end).step_by(16) {
                let section = u16::from_le_bytes(b[at + 14..at + 16].try_into().unwrap());
                if (1..=3).contains(&section) {
                    let value = get32(&b, at + 4) + u32::from(section) * 0x1000;
                    put32(&mut b, at + 4, value);
                }
            }
        }
    }
    assert_ne!(symtab, 0);
    assert_ne!(dynsym, 0);
    for field in [16, 20, 24, 28, 32, 36] {
        let value = get32(&b, symtab + field);
        put32(&mut b, shoff + dynsym * 40 + field, value);
    }
    for i in 1..shnum {
        let sh = shoff + i * 40;
        if get32(&b, sh + 4) == object::elf::SHT_REL {
            put32(&mut b, sh + 24, dynsym as u32);
            let at = get32(&b, sh + 16) as usize;
            put32(&mut b, at, 0x3000);
        }
    }
    let phoff = b.len() as u32;
    put32(&mut b, 28, phoff);
    b[42..44].copy_from_slice(&32u16.to_le_bytes());
    b[44..46].copy_from_slice(&3u16.to_le_bytes());
    for i in 1..=3 {
        let sh = shoff + i * 40;
        let (offset, size) = (get32(&b, sh + 16), get32(&b, sh + 20));
        for value in [
            1,
            offset,
            i as u32 * 0x1000,
            i as u32 * 0x1000,
            size,
            size,
            if i == 3 { 6 } else { 5 },
            1,
        ] {
            b.extend_from_slice(&value.to_le_bytes());
        }
    }
    b
}
