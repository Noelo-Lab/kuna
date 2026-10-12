//! A 64-bit parameter passed in a register pair stays whole when one of its
//! registers is reused before a call that still reads the parameter.
use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

fn image(arch: Architecture, endian: Endianness, functions: &[(&str, Vec<u32>)]) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, arch, endian);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut bytes = Vec::new();
    for (name, words) in functions {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: bytes.len() as u64,
            size: words.len() as u64 * 4,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
        for &word in words {
            bytes.extend(match endian {
                Endianness::Big => word.to_be_bytes(),
                Endianness::Little => word.to_le_bytes(),
            });
        }
    }
    object.append_section_data(text, &bytes, 4);
    object.write().unwrap()
}

/// `long long call_wide(struct ops *o, long long v) { return o->wide(v, 5) + 1; }`
/// and `long long dc_load(int *o, long long v) { return wide3(v, *o) + 1; }`,
/// clang -O2 for armv7 hard-float.
fn arm() -> Vec<u8> {
    let bl = |from: u32, to: u32| 0xeb000000 | ((to as i32 - from as i32 - 2) as u32 & 0xffffff);
    let call_wide = vec![
        0xe92d4800, 0xe1a01003, 0xe5903000, 0xe1a00002, 0xe3a02005, 0xe12fff33, 0xe2900001,
        0xe2a11000, 0xe8bd8800,
    ];
    let dc_load = vec![
        0xe92d4800,
        0xe1a01003,
        0xe1a03002,
        0xe5902000,
        0xe1a00003,
        bl(14, 18),
        0xe2900001,
        0xe2a11000,
        0xe8bd8800,
    ];
    image(
        Architecture::Arm,
        Endianness::Little,
        &[
            ("call_wide", call_wide),
            ("dc_load", dc_load),
            ("wide3", vec![0xe12fff1e]),
        ],
    )
}

/// The same two functions, clang -O2 for 32-bit big-endian PowerPC, where the
/// reused register is the parameter's high word.
fn powerpc() -> Vec<u8> {
    let bl =
        |from: u32, to: u32| 0x48000001 | (((to as i32 - from as i32) * 4) as u32 & 0x03fffffc);
    let call_wide = vec![
        0x7c0802a6, 0x90010004, 0x9421fff0, 0x80630000, 0x7cc43378, 0x7c6903a6, 0x7ca32b78,
        0x38a00005, 0x4e800421, 0x30840001, 0x7c630194, 0x80010014, 0x38210010, 0x7c0803a6,
        0x4e800020,
    ];
    let dc_load = vec![
        0x7c0802a6,
        0x90010004,
        0x9421fff0,
        0x7cc43378,
        0x7ca62b78,
        0x80a30000,
        0x7cc33378,
        bl(22, 29),
        0x30840001,
        0x7c630194,
        0x80010014,
        0x38210010,
        0x7c0803a6,
        0x4e800020,
    ];
    image(
        Architecture::PowerPc,
        Endianness::Big,
        &[
            ("call_wide", call_wide),
            ("dc_load", dc_load),
            ("wide3", vec![0x4e800020]),
        ],
    )
}

fn decompile(bytes: &[u8]) -> String {
    let path = common::scratch_file("register-pair-params", "o");
    std::fs::write(&path, bytes).unwrap();
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"));
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = Command::new(kuna)
        .arg("decompile-all")
        .arg(&path)
        .args(["--assert-strict", "--sleighpath"])
        .arg(specs)
        .args([
            "--assert",
            "typedef struct ops { long long (*wide)(long long v, int k); };",
            "--assert",
            "prototype call_wide long long call_wide(struct ops *o, long long v)",
            "--assert",
            "prototype wide3 long long wide3(long long v, int k)",
            "--assert",
            "prototype dc_load long long dc_load(int *o, long long v)",
        ])
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    text
}

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}

fn check_indirect(text: &str) {
    let body = function(text, "call_wide");
    assert!(body.contains(")(v,5) + 1;"), "{body}");
    assert!(
        !body.contains("v._"),
        "a reused register was written into v: {body}"
    );
}

fn check_direct(text: &str) {
    let body = function(text, "dc_load");
    assert!(body.contains("return wide3(v,"), "{body}");
    assert!(
        !body.contains("v._"),
        "a reused register was written into v: {body}"
    );
    assert!(
        !body.contains("(int)v)"),
        "the loaded argument became a piece of v: {body}"
    );
}

#[test]
fn arm_typed_indirect_call_keeps_the_register_pair_whole() {
    check_indirect(&decompile(&arm()));
}

#[test]
fn arm_direct_call_keeps_the_register_pair_whole() {
    check_direct(&decompile(&arm()));
}

#[test]
fn big_endian_powerpc_typed_indirect_call_keeps_the_register_pair_whole() {
    check_indirect(&decompile(&powerpc()));
}

#[test]
fn big_endian_powerpc_direct_call_keeps_the_register_pair_whole() {
    check_direct(&decompile(&powerpc()));
}
