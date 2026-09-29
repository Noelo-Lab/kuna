//! A byte-writing callee must not shrink its caller's full-width object.
mod common;
use common::process;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

#[test]
fn pointer_conversions_follow_the_printed_callee_in_either_order() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| {
            process::optional_output(Command::new(cc).arg("--version")).is_some()
        })
        .collect();
    assert!(
        !compilers.is_empty(),
        "the round trip requires a C compiler"
    );
    for callee_first in [true, false] {
        let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
        let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        // sink: xor byte [rdi],1; ret. change: push rdi; mov rdi,rsp;
        // call sink; pop rax; ret. Only the low byte changes.
        let (code, sink, change) = if callee_first {
            (
                vec![
                    0x80, 0x37, 1, 0xc3, 0x57, 0x48, 0x89, 0xe7, 0xe8, 0xf3, 0xff, 0xff, 0xff,
                    0x58, 0xc3,
                ],
                0,
                4,
            )
        } else {
            (
                vec![
                    0x57, 0x48, 0x89, 0xe7, 0xe8, 2, 0, 0, 0, 0x58, 0xc3, 0x80, 0x37, 1, 0xc3,
                ],
                11,
                0,
            )
        };
        obj.append_section_data(section, &code, 1);
        for (name, value, size) in [("sink", sink, 4), ("change", change, 11)] {
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value,
                size,
                kind: SymbolKind::Text,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Section(section),
                flags: SymbolFlags::None,
            });
        }
        let input = common::scratch_file("pointer-call", "o");
        let src = common::scratch_file("pointer-call", "c");
        let exe = common::scratch_file("pointer-call", "exe");
        std::fs::write(&input, obj.write().unwrap()).unwrap();
        for order in ["off", "types"] {
            let decomp = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile-all",
                    input.to_str().unwrap(),
                    "--functions",
                    "sink,change",
                    "--option",
                    "protoorder",
                    order,
                    "--option",
                    "castobject",
                    "off",
                    "--assert",
                    "prototype change unsigned long change(unsigned long)",
                    "--assert-strict",
                ])
                .output()
                .unwrap();
            assert!(
                decomp.status.success(),
                "{}",
                String::from_utf8_lossy(&decomp.stderr)
            );
            let printed = String::from_utf8(decomp.stdout).unwrap();
            assert!(printed.contains("sink((unsigned char *)&"), "{printed}");
            assert!(printed.contains("unsigned long v"), "{printed}");
            std::fs::write(
                &src,
                format!(
                    r#"
void sink(unsigned char *);
{printed}
int main(void) {{
    unsigned long values[] = {{0, 1, 0x123456789abcdef0UL, ~0UL, 0x8000000000000000UL}};
    for (unsigned i = 0; i < sizeof(values)/sizeof(*values); ++i)
        if (change(values[i]) != (values[i] ^ 1)) return 1;
    return 0;
}}
"#
                ),
            )
            .unwrap();
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let compile = Command::new(cc)
                        .args(["-std=c11", "-Werror=incompatible-pointer-types", level])
                        .arg(&src)
                        .arg("-o")
                        .arg(&exe)
                        .output()
                        .unwrap();
                    assert!(
                        compile.status.success(),
                        "{cc} {level}: {}\n{printed}",
                        String::from_utf8_lossy(&compile.stderr)
                    );
                    assert!(
                        Command::new(&exe).status().unwrap().success(),
                        "{order} {cc} {level}: {printed}"
                    );
                }
            }
        }
        for file in [input, src, exe] {
            std::fs::remove_file(file).unwrap();
        }
    }
}

#[test]
fn worker_pointer_conversions_match_the_serial_document() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/castwiden_clang_O2_x86_64");
    let run = |jobs: &str| {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile-all",
                fixture.to_str().unwrap(),
                "--functions",
                "sink,assign_call,assign_size",
                "--option",
                "protoorder",
                "off",
                "--jobs",
                jobs,
                "--jobs-chunk",
                "1",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let serial = run("1");
    assert!(serial.contains("sink((unsigned char *)&v1)"), "{serial}");
    assert_eq!(run("2"), serial);
}
