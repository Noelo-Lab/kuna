use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

// Authored ARM: put8(p, c) stores the low byte of r1; fwd(p, c) passes
// c + 1 to put8 and returns 0 (`add r1, r1, #1; bl put8`).
fn image() -> Vec<u8> {
    let words: [u32; 7] =
        [0xe5c01000, 0xe12fff1e, 0xe92d4010, 0xe2811001, 0xebfffffa, 0xe3a00000, 0xe8bd8010];
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(
        text,
        &words.iter().copied().flat_map(u32::to_le_bytes).collect::<Vec<_>>(),
        4,
    );
    for (name, value, size) in [("put8", 0, 8), ("fwd", 8, 20)] {
        obj.add_symbol(Symbol {
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
    obj.write().unwrap()
}

fn decompile(options: &[&str]) -> String {
    let path = common::scratch_file("narrow-args", "o");
    std::fs::write(&path, image()).unwrap();
    let mut args = vec!["decompile-all", path.to_str().unwrap(), "--mode", "aggressive"];
    for pair in options.chunks(2) {
        args.extend(["--option", pair[0], pair[1]]);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap()
}

/// A caller whose parameter reaches only the byte its callee stores is
/// declared at that byte; with the option off it stays a register-wide `int`.
#[test]
fn a_forwarded_parameter_takes_the_callee_width() {
    let on = decompile(&[]);
    assert!(on.contains("unsigned int fwd(char *a0,char a1)"), "{on}");
    assert!(on.contains("put8(a0,a1 + '\\x01');"), "{on}");
    let off = decompile(&["narrowargs", "off"]);
    assert!(off.contains("unsigned int fwd(char *a0,int a1)"), "{off}");
    assert!(off.contains("put8(a0,a1 + 1);"), "{off}");
}
