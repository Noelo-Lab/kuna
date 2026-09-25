mod common;

use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

fn get32(b: &[u8], p: usize) -> u32 { u32::from_le_bytes(b[p..p + 4].try_into().unwrap()) }
fn get64(b: &[u8], p: usize) -> u64 { u64::from_le_bytes(b[p..p + 8].try_into().unwrap()) }
fn put64(b: &mut [u8], p: usize, n: u64) { b[p..p + 8].copy_from_slice(&n.to_le_bytes()); }

/// A linked ELF with a dynamically interposable function and its PLT stub.
fn image(definitions: usize, named_stub: bool) -> Vec<u8> {
    let mut o = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = o.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    o.append_section_data(text, &[0xb8, 7, 0, 0, 0, 0xc3, 0xb8, 9, 0, 0, 0, 0xc3], 1);
    let plt = o.add_section(Vec::new(), b".plt".to_vec(), SectionKind::Text);
    o.append_section_data(plt, &[0xff, 0x25, 0xfa, 0x0f, 0, 0], 1);
    let got = o.add_section(Vec::new(), b".got".to_vec(), SectionKind::Data);
    o.append_section_data(got, &[0; 8], 8);
    let mut symbol = None;
    for i in 0..definitions.max(1) {
        symbol = Some(o.add_symbol(Symbol {
            name: b"answer".to_vec(), value: (i * 6) as u64, size: 6,
            kind: SymbolKind::Text, scope: SymbolScope::Linkage, weak: false,
            section: if definitions == 0 { SymbolSection::Undefined } else { SymbolSection::Section(text) },
            flags: SymbolFlags::None,
        }));
    }
    o.add_relocation(got, Relocation { offset: 0, symbol: symbol.unwrap(), addend: 0,
        flags: object::RelocationFlags::Elf { r_type: object::elf::R_X86_64_JUMP_SLOT },
    }).unwrap();
    if named_stub {
        o.add_symbol(Symbol {
            name: b"answer".to_vec(), value: 0, size: 6, kind: SymbolKind::Text,
            scope: SymbolScope::Linkage, weak: false, section: SymbolSection::Section(plt),
            flags: SymbolFlags::None,
        });
    }
    let mut b = o.write().unwrap();
    let shoff = get64(&b, 40) as usize;
    let shnum = u16::from_le_bytes(b[60..62].try_into().unwrap()) as usize;
    b[16..18].copy_from_slice(&3u16.to_le_bytes());
    put64(&mut b, 24, 0x1000);
    for i in 1..shnum {
        let sh = shoff + i * 64;
        let kind = get32(&b, sh + 4);
        let begin = get64(&b, sh + 24) as usize;
        let end = begin + get64(&b, sh + 32) as usize;
        if i <= 3 { put64(&mut b, sh + 16, i as u64 * 0x1000); }
        if kind == object::elf::SHT_SYMTAB {
            b[sh + 4..sh + 8].copy_from_slice(&object::elf::SHT_DYNSYM.to_le_bytes());
            for sym in (begin..end).step_by(24) {
                let section = u16::from_le_bytes(b[sym + 6..sym + 8].try_into().unwrap());
                if matches!(section, 1 | 2) {
                    let value = get64(&b, sym + 8) + section as u64 * 0x1000;
                    put64(&mut b, sym + 8, value);
                }
            }
        } else if kind == object::elf::SHT_RELA {
            for reloc in (begin..end).step_by(24) { put64(&mut b, reloc, 0x3000); }
        }
    }
    let phoff = b.len() as u64;
    put64(&mut b, 32, phoff);
    b[54..56].copy_from_slice(&56u16.to_le_bytes());
    b[56..58].copy_from_slice(&3u16.to_le_bytes());
    for i in 1..=3 {
        let sh = shoff + i * 64;
        let (offset, size) = (get64(&b, sh + 24), get64(&b, sh + 32));
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&(if i == 3 { 6u32 } else { 5u32 }).to_le_bytes());
        for n in [offset, i as u64 * 0x1000, i as u64 * 0x1000, size, size, 1] {
            b.extend_from_slice(&n.to_le_bytes());
        }
    }
    b
}

#[test]
fn definition_wins_over_its_plt_but_real_definitions_remain_ambiguous() {
    for (definitions, named_stub) in [(1, false), (1, true), (0, false), (2, false)] {
        let path = common::scratch_file("definition-and-plt", "elf");
        std::fs::write(&path, image(definitions, named_stub)).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", path.to_str().unwrap(), "answer", "--mode", "reliable", "--json"])
            .output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if definitions == 2 {
            assert!(!output.status.success(), "{stdout}");
            assert!(stderr.contains("ambiguous"), "{stderr}");
        } else {
            assert!(output.status.success(), "{stdout}\n{stderr}");
            let addr = if definitions == 1 { "0x1000" } else { "0x2000" };
            assert!(stdout.contains(&format!("\"address_hex\": \"{addr}\"")), "{stdout}");
            if definitions == 1 {
                assert!(stdout.contains("return 7;"), "{stdout}");
                for selector in ["0x1000", "0x2000"] {
                    let selected = Command::new(env!("CARGO_BIN_EXE_kuna"))
                        .args(["decompile", path.to_str().unwrap(), selector, "--addr", "--mode", "reliable", "--json"])
                        .output().unwrap();
                    let text = String::from_utf8_lossy(&selected.stdout);
                    assert!(selected.status.success(), "{text}\n{}", String::from_utf8_lossy(&selected.stderr));
                    assert!(text.contains(&format!("\"address_hex\": \"{selector}\"")), "{text}");
                }
                let text = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args(["decompile", path.to_str().unwrap(), "answer", "--mode", "reliable"])
                    .output().unwrap();
                assert!(text.status.success(), "{}", String::from_utf8_lossy(&text.stderr));
                assert!(String::from_utf8_lossy(&text.stdout).contains("return 7;"));
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn name_lookup_agrees_with_decompile_selection() {
    let specs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    for (definitions, expected) in [(0, Some(0x2000)), (1, Some(0x1000)), (2, None)] {
        let path = common::scratch_file("definition-name-lookup", "elf");
        std::fs::write(&path, image(definitions, true)).unwrap();
        let prog = kuna_console::engine::bootstrap_from_file(
            path.to_str().unwrap(), "", &[specs.to_str().unwrap().into()],
        ).unwrap();
        let entry = prog.find_entry_by_name("answer");
        assert_eq!(entry.map(|entry| entry.addr.get_offset()), expected);
        std::fs::remove_file(path).unwrap();
    }
}
