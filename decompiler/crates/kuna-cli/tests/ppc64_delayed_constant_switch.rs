//! A PPC64 switch whose index a call writes keeps every case of its table.

use crate::common;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

/// `select` stores 0 to a stack slot, passes its address to `get_state`, which
/// writes it, then reads the slot back, guards it and dispatches through a
/// five-entry relative table (`base + sext(table[i])`, entries 0 and 4 sharing
/// a target). The base is `bcl`/`mflr` plus `addis`/`addi`, a constant expression
/// several ops deep. Each case returns its own constant.
fn image(endian: Endianness) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, endian);
    object.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags: 2 };
    let word = |n: u32| match endian {
        Endianness::Big => n.to_be_bytes(),
        Endianness::Little => n.to_le_bytes(),
    };
    let code: Vec<u8> = [
        0x7c0802a6, 0xf8010010, 0xf821ff71, 0x39200000, 0x38a10070, 0x91210070, 0x48000091,
        0x2f83000b, 0x419e0074, 0x81210070, 0x2b890004, 0x419d0060, 0x7d2a4b78, 0x429f0005,
        0x7d0802a6, 0x3d080000, 0x39080020, 0x794a1764, 0x7d2852aa, 0x7d294214, 0x7d2903a6,
        0x4e800420, 0x00000014, 0x0000001c, 0x00000024, 0x0000002c, 0x00000014, 0x38600028,
        0x48000028, 0x38600029, 0x48000020, 0x3860002a, 0x48000018, 0x3860002b, 0x48000010,
        0x38600063, 0x48000008, 0x38600001, 0x38210090, 0xe8010010, 0x7c0803a6, 0x4e800020,
        0x90850000, 0x38600000, 0x4e800020,
    ]
    .into_iter()
    .flat_map(word)
    .collect();
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(text, &code, 4);
    for (name, value, size) in [("select", 0, 0xa8), ("get_state", 0xa8, 0xc)] {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    object.write().unwrap()
}

#[test]
fn a_call_written_switch_index_keeps_every_table_case() {
    let path = common::scratch_file("ppc64-delayed-constant-switch", "o");
    for endian in [Endianness::Big, Endianness::Little] {
        std::fs::write(&path, image(endian)).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", path.to_str().unwrap(), "select"])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        assert!(!code.contains("does not reproduce the recovered table"), "{endian:?}: {code}");
        let flat = code.split_whitespace().collect::<Vec<_>>().join(" ");
        for body in [
            "case 0: case 4: return 0x28;",
            "case 1: return 0x29;",
            "case 2: return 0x2a;",
            "case 3: return 0x2b;",
            "default: return 99;",
        ] {
            assert!(flat.contains(body), "{endian:?}: lost `{body}`: {code}");
        }
    }
    std::fs::remove_file(path).unwrap();
}
