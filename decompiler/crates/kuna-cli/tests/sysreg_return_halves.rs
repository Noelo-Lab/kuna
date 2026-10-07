//! A register an ARM or Thumb function sets for a system register (`vmsr
//! fpscr`, `msr cpsr_c`, `msr basepri`) is not the high word of its return:
//! `bl g; mov r1,#0x3000000; vmsr fpscr,r1; pop {r11,pc}` returns `g()`, not
//! `CONCAT44(0x3000000,g())`.
use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

fn object(bytes: &[u8], symbols: Vec<(String, u64, u64)>) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(text, bytes, 4);
    for (name, value, size) in symbols {
        object.add_symbol(Symbol {
            name: name.into_bytes(),
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

/// clang -target armv7a-linux-gnueabihf -marm -O2, `g` returning three times
/// the word at 0x20000000.
fn arm_image() -> Vec<u8> {
    let mut words = Vec::<u32>::new();
    let mut symbols = Vec::new();
    let mut add = |name: &str, code: &[u32]| {
        let at = words.len();
        words.extend_from_slice(code);
        symbols.push((name.to_string(), at as u64 * 4, code.len() as u64 * 4));
        at
    };
    let g = add("g", &[0xe3a00202, 0xe5900000, 0xe0800080, 0xe12fff1e]);
    let keepc = add(
        "keepc",
        &[0xe92d4800, 0xeb000000, 0xe3a01403, 0xeee11a10, 0xe8bd8800],
    );
    add("comp", &[0xe0800080, 0xe3a01403, 0xeee11a10, 0xe12fff1e]);
    add("compm", &[0xe0800080, 0xe3a01013, 0xe121f001, 0xe12fff1e]);
    let call = keepc + 1;
    words[call] = 0xeb000000 | ((g as i32 - call as i32 - 2) as u32 & 0xffffff);
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object(&bytes, symbols)
}

/// clang -target thumbv7em-none-eabihf -mcpu=cortex-m4 -O2; `raise_bp` is
/// FreeRTOS's `ulPortRaiseBASEPRI`.
fn thumb_image() -> Vec<u8> {
    let mut code = Vec::<u16>::new();
    let mut symbols = Vec::new();
    let mut add = |name: &str, body: &[u16]| {
        let at = code.len();
        code.extend_from_slice(body);
        symbols.push((name.to_string(), at as u64 * 2 + 1, body.len() as u64 * 2));
        at
    };
    let g = add("g", &[0xf04f, 0x5000, 0x6800, 0xeb00, 0x0040, 0x4770]);
    let keepc = add(
        "keepc",
        &[
            0xb580, 0xf000, 0xf800, 0xf04f, 0x7140, 0xeee1, 0x1a10, 0xbd80,
        ],
    );
    add(
        "raise_bp",
        &[
            0xf3ef, 0x8011, 0xf04f, 0x0150, 0xf381, 0x8811, 0xf3bf, 0x8f6f, 0xf3bf, 0x8f4f, 0x4770,
        ],
    );
    add(
        "lock_prim",
        &[0xf3ef, 0x8010, 0x2101, 0xf381, 0x8810, 0x4770],
    );
    let call = keepc + 1;
    let offset = (g as i64 - call as i64 - 2) * 2;
    code[call] = 0xf000 | ((offset >> 12) as u16 & 0x7ff);
    code[call + 1] = 0xf800 | ((offset >> 1) as u16 & 0x7ff);
    let bytes: Vec<_> = code.into_iter().flat_map(u16::to_le_bytes).collect();
    object(&bytes, symbols)
}

fn decompile_all(bytes: &[u8]) -> String {
    let path = common::scratch_file("sysreg-return-halves", "o");
    std::fs::write(&path, bytes).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap()])
        .output()
        .unwrap();
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

#[test]
fn arm_system_register_operand_is_no_high_word() {
    let text = decompile_all(&arm_image());
    assert!(!text.contains("CONCAT44"), "{text}");
    let keepc = function(&text, "keepc");
    assert!(
        keepc.contains("int keepc(void)") && keepc.contains("return g();"),
        "{text}"
    );
    for name in ["comp", "compm"] {
        let body = function(&text, name);
        assert!(body.contains(&format!("int {name}(int a0)")), "{text}");
        assert!(body.contains("return a0 * 3;"), "{text}");
    }
}

#[test]
fn thumb_system_register_operand_is_no_high_word() {
    let text = decompile_all(&thumb_image());
    assert!(!text.contains("CONCAT44"), "{text}");
    let keepc = function(&text, "keepc");
    assert!(
        keepc.contains("int keepc(void)") && keepc.contains("return g();"),
        "{text}"
    );
    for name in ["raise_bp", "lock_prim"] {
        assert!(
            function(&text, name).contains(&format!("unsigned int {name}(void)")),
            "{text}"
        );
    }
}
