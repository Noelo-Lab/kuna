mod common;
#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;

use std::process::Command;

#[test]
fn undefined_function_still_reports_an_external_symbol() {
    use object::write::{Object, Symbol, SymbolSection};
    use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &[0xe8, 0, 0, 0, 0, 0xc3], 1);
    let outside = obj.add_symbol(Symbol {
        name: b"outside".to_vec(), value: 0, size: 0, kind: SymbolKind::Text,
        scope: SymbolScope::Linkage, weak: false, section: SymbolSection::Undefined,
        flags: SymbolFlags::None,
    });
    obj.add_relocation(text, object::write::Relocation {
        offset: 1, symbol: outside, addend: -4,
        flags: object::RelocationFlags::Generic {
            kind: object::RelocationKind::Relative,
            encoding: object::RelocationEncoding::X86Branch,
            size: 32,
        },
    }).unwrap();
    let path = common::scratch_file("undefined-function", "o");
    std::fs::write(&path, obj.write().unwrap()).unwrap();
    for json in [false, true] {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
        cmd.args(["decompile", path.to_str().unwrap(), "outside"]);
        if json { cmd.arg("--json"); }
        let output = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stdout}\n{stderr}");
        assert!(stdout.contains("external symbol"), "{stdout}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mapped_decode_failure_is_not_an_external_symbol() {
    let path = common::scratch_file("mapped-thumb-code", "elf");
    // movs r0, #7; bx lr, deliberately without ARM/Thumb metadata.
    let mut elf = arm_images::elf(&[7, 0x20, 0x70, 0x47], &[], &[]);
    let ph = u32::from_le_bytes(elf[28..32].try_into().unwrap()) as usize;
    let base = u32::from_le_bytes(elf[ph + 8..ph + 12].try_into().unwrap());
    for (field, value) in [(4, 0x10000 - base), (8, 0x10000), (12, 0x10000), (16, 4), (20, 4)] {
        elf[ph + field..ph + field + 4].copy_from_slice(&value.to_le_bytes());
    }
    std::fs::write(&path, elf).unwrap();
    for json in [false, true] {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
        cmd.args(["decompile", path.to_str().unwrap(), "0x10000", "--addr"]);
        if json {
            cmd.arg("--json");
        }
        let output = cmd.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{stdout}\n{stderr}");
        assert!(!stdout.contains("external symbol"), "{stdout}");
        assert!(stderr.contains("Unable to load"), "{stderr}");
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", path.to_str().unwrap(), "0x10000", "--addr", "--isa", "thumb"])
        .output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).contains("return 7;"));
    std::fs::remove_file(path).unwrap();
}
