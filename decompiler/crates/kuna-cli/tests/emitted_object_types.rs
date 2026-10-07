//! Runtime checks for scalar objects reached through saved pointers.
use crate::common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope};
use std::process::Command;

#[test]
fn an_address_only_scalar_has_the_type_of_its_saved_pointer() {
    // Same handwritten SysV code as the stage case; build a redistributable
    // object directly so the fixture does not depend on compiler stack layout.
    let stage = include_str!("../../../../tests/stages/kuna-address-only-local.xml");
    let hex = stage.split("offset=\"0x1000\">").nth(1).unwrap().split("</bytechunk>").next().unwrap();
    let code: Vec<u8> = hex.as_bytes().chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &code, 1);
    for (name, value, size) in [("write_word", 0, 11), ("read_escaped", 11, code.len() as u64 - 11)] {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(), value, size, kind: SymbolKind::Text,
            scope: SymbolScope::Linkage, weak: false, section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let input = common::scratch_file("escaped-scalar", "o");
    std::fs::write(&input, obj.write().unwrap()).unwrap();
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some()).collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    for opt in ["off", "on"] {
        let decomp = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", input.to_str().unwrap(), "read_escaped", "--option", "castobject", opt,
                   "--assert", "prototype write_word int write_word(int, int *, int)",
                   "--assert", "prototype read_escaped int read_escaped(int)", "--assert-strict"])
            .output().unwrap();
        assert!(decomp.status.success(), "{}", String::from_utf8_lossy(&decomp.stderr));
        let printed = String::from_utf8(decomp.stdout).unwrap();
        assert!(printed.contains("int v1;"), "{printed}");
        let src = common::scratch_file("escaped-scalar", "c");
        let exe = common::scratch_file("escaped-scalar", "exe");
        std::fs::write(&src, format!(r#"
int write_word(int x, int *p, int o) {{ *p = (unsigned)x << 8; return x + o; }}
{printed}
int main(void) {{
    int values[] = {{0, 1, 9, 255, 256, 0x123456, -1}};
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i) {{
        int x = values[i];
        if (read_escaped(x) != (x < 0 ? -1 : x & 255)) return 1;
    }}
    return 0;
}}
"#)).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args(["-std=c11", "-Werror=incompatible-pointer-types", level])
                    .arg(&src).arg("-o").arg(&exe).output().unwrap();
                assert!(compile.status.success(), "{cc} {level}: {}\n{printed}", String::from_utf8_lossy(&compile.stderr));
                assert!(Command::new(&exe).status().unwrap().success(), "{opt} {cc} {level}: {printed}");
            }
        }
        std::fs::remove_file(src).unwrap();
        std::fs::remove_file(exe).unwrap();
    }
    std::fs::remove_file(input).unwrap();
}
