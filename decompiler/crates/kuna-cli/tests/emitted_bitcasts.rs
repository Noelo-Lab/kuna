//! Bit-for-bit round trips through the emitted C on three instruction sets.
use crate::common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

// The stage bytes were built at -O2 from these source operations:
// bits32: union { unsigned u; float f; } v = {.u = x ^ 0x80000000u}; return v.f;
// unbits32: the reverse union member transfer; bits64/unbits64 use unsigned
// long long and double. numeric32/trunc32 are ordinary C numeric casts.
#[test]
fn storage_transfers_preserve_float_bits_and_numeric_conversions_keep_their_values() {
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
    let cases: &[(&str, Architecture, bool, &str, &[(&str, u64, u64)])] = &[
        (
            "arm",
            Architecture::Arm,
            false,
            include_str!("../../../../tests/stages/kuna-bit-reinterpret-arm.xml"),
            &[
                ("bits32", 0, 12),
                ("unbits32", 12, 8),
                ("bits64", 20, 8),
                ("unbits64", 28, 8),
                ("numeric32", 36, 12),
                ("trunc32", 48, 12),
            ],
        ),
        (
            "thumb",
            Architecture::Arm,
            true,
            include_str!("../../../../tests/stages/kuna-bit-reinterpret-thumb.xml"),
            &[
                ("bits32", 0, 10),
                ("unbits32", 12, 12),
                ("bits64", 24, 6),
                ("unbits64", 32, 6),
                ("numeric32", 40, 10),
                ("trunc32", 52, 10),
            ],
        ),
        (
            "x64",
            Architecture::X86_64,
            false,
            include_str!("../../../../tests/stages/kuna-bit-reinterpret-x64.xml"),
            &[
                ("bits32", 0, 15),
                ("unbits32", 16, 9),
                ("bits64", 32, 10),
                ("unbits64", 48, 10),
                ("numeric32", 64, 13),
                ("trunc32", 80, 9),
            ],
        ),
    ];
    for &(name, architecture, thumb, stage, symbols) in cases {
        for signed in [false, true] {
            let hex = stage
                .split("offset=\"0x1000\">")
                .nth(1)
                .unwrap()
                .split("</bytechunk>")
                .next()
                .unwrap();
            let bytes: Vec<_> = hex
                .as_bytes()
                .chunks_exact(2)
                .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
                .collect();
            let mut obj = Object::new(BinaryFormat::Elf, architecture, Endianness::Little);
            if architecture == Architecture::Arm {
                obj.flags = FileFlags::Elf {
                    os_abi: 0,
                    abi_version: 0,
                    e_flags: 0x05000400,
                };
            }
            let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
            obj.append_section_data(text, &bytes, 4);
            for &(symbol, offset, size) in symbols {
                obj.add_symbol(Symbol {
                    name: symbol.as_bytes().to_vec(),
                    value: offset | u64::from(thumb),
                    size,
                    kind: SymbolKind::Text,
                    scope: SymbolScope::Linkage,
                    weak: false,
                    section: SymbolSection::Section(text),
                    flags: SymbolFlags::None,
                });
            }
            let input = common::scratch_file("bit-transfers", "o");
            let src = common::scratch_file("bit-transfers", "c");
            let exe = common::scratch_file("bit-transfers", "exe");
            std::fs::write(&input, obj.write().unwrap()).unwrap();
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
            cmd.args([
                "decompile-all",
                input.to_str().unwrap(),
                "--functions",
                "bits32,unbits32,bits64,unbits64,numeric32,trunc32",
                "--option",
                "protoorder",
                "off",
            ]);
            for (function, prototype) in [
                ("bits32", "float bits32(unsigned int)"),
                (
                    "unbits32",
                    if signed {
                        "int unbits32(float)"
                    } else {
                        "unsigned int unbits32(float)"
                    },
                ),
                ("bits64", "double bits64(unsigned long long)"),
                (
                    "unbits64",
                    if signed {
                        "long long unbits64(double)"
                    } else {
                        "unsigned long long unbits64(double)"
                    },
                ),
                ("numeric32", "float numeric32(int)"),
                ("trunc32", "int trunc32(float)"),
            ] {
                cmd.args(["--assert", &format!("prototype {function} {prototype}")]);
            }
            let output = cmd.arg("--assert-strict").output().unwrap();
            assert!(
                output.status.success(),
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let printed = String::from_utf8(output.stdout).unwrap();
            assert_eq!(printed.matches("union {").count(), 4, "{name}: {printed}");
            std::fs::write(&src, format!(r#"
#include <stdint.h>
#include <string.h>
_Static_assert(sizeof(float) == 4 && sizeof(double) == 8, "IEEE storage widths");
{printed}
int main(void) {{
    uint32_t words[] = {{0, 0x80000000, 0x3f800000, 0xc0200000, 1, 0x7f800000, 0xff800000, 0x7fc12345, 0xffa23456}};
    uint64_t quads[] = {{0, 0x8000000000000000ULL, 0x3ff0000000000000ULL, 0xc004000000000000ULL, 1,
        0x7ff0000000000000ULL, 0xfff0000000000000ULL, 0x7ff8123456789abcULL, 0xfff123456789abcdULL}};
    for (unsigned i = 0; i < sizeof(words)/sizeof(*words); ++i) {{
        float input, result = bits32(words[i]); uint32_t bits;
        memcpy(&bits, &result, 4); if (bits != (words[i] ^ 0x80000000u)) return 1;
        memcpy(&input, &words[i], 4); if (unbits32(input) != words[i]) return 2;
    }}
    for (unsigned i = 0; i < sizeof(quads)/sizeof(*quads); ++i) {{
        double input, result = bits64(quads[i]); uint64_t bits;
        memcpy(&bits, &result, 8); if (bits != quads[i]) return 3;
        memcpy(&input, &quads[i], 8); if (unbits64(input) != quads[i]) return 4;
    }}
    if (numeric32(-123) != -123.0f || numeric32(456) != 456.0f) return 5;
    if (trunc32(3.75f) != 3 || trunc32(-3.75f) != -3) return 6;
    return 0;
}}
"#)).unwrap();
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let compile = Command::new(cc)
                        .args(["-std=c11", "-Werror", level])
                        .arg(&src)
                        .arg("-o")
                        .arg(&exe)
                        .output()
                        .unwrap();
                    assert!(
                        compile.status.success(),
                        "{name} {cc} {level}: {}\n{printed}",
                        String::from_utf8_lossy(&compile.stderr)
                    );
                    let status = Command::new(&exe).status().unwrap();
                    assert!(status.success(), "{name} {cc} {level}: {status}\n{printed}");
                }
            }
            let rust = cmd.args(["--language", "rust"]).output().unwrap();
            assert!(
                rust.status.success(),
                "{name}: {}",
                String::from_utf8_lossy(&rust.stderr)
            );
            let rust = String::from_utf8(rust.stdout).unwrap();
            let rust_src = common::scratch_file("bit-transfers", "rs");
            std::fs::write(&rust_src, format!(r#"
{rust}
fn main() {{ unsafe {{
    for bits in [0u32, 0x80000000, 0x3f800000, 0xc0200000, 1, 0x7f800000, 0xff800000, 0x7fc12345, 0xffa23456] {{
        assert_eq!(bits32(bits).to_bits(), bits ^ 0x80000000);
        assert_eq!(unbits32(f32::from_bits(bits)) as u32, bits);
    }}
    for bits in [0u64, 0x8000000000000000, 0x3ff0000000000000, 0xc004000000000000, 1,
                 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8123456789abc, 0xfff123456789abcd] {{
        assert_eq!(bits64(bits).to_bits(), bits);
        assert_eq!(unbits64(f64::from_bits(bits)) as u64, bits);
    }}
    assert_eq!(numeric32(-123), -123.0);
    assert_eq!(trunc32(-3.75), -3);
}} }}
"#)).unwrap();
            for level in ["0", "2"] {
                let compile = Command::new("rustc")
                    .args([
                        "--crate-name",
                        "bit_transfers",
                        "-C",
                        "overflow-checks=off",
                        "-C",
                    ])
                    .arg(format!("opt-level={level}"))
                    .arg(&rust_src)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .unwrap();
                assert!(
                    compile.status.success(),
                    "{name} rustc {level}: {}\n{rust}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                assert!(
                    Command::new(&exe).status().unwrap().success(),
                    "{name} rustc {level}: {rust}"
                );
            }
            std::fs::remove_file(rust_src).unwrap();
            for file in [input, src, exe] {
                std::fs::remove_file(file).unwrap();
            }
        }
    }
}

#[test]
fn floating_fields_are_reinterpreted_before_integer_truncation() {
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/ftol_i386.obj");
    let decomp = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            input.to_str().unwrap(),
            "--functions",
            "ftol_conv",
            "--option",
            "msvcftol",
            "on",
        ])
        .output()
        .unwrap();
    assert!(
        decomp.status.success(),
        "{}",
        String::from_utf8_lossy(&decomp.stderr)
    );
    let printed = String::from_utf8(decomp.stdout).unwrap();
    let src = common::scratch_file("float-fields", "c");
    let exe = common::scratch_file("float-fields", "exe");
    std::fs::write(
        &src,
        format!(
            r#"
{printed}
int main(void) {{
    unsigned int fields[8] = {{0, 0, 0, 0, 0, 0, 0x42f78000u, 0xc0580000u}};
    ftol_conv(fields);
    return fields[0] != 123 || fields[1] != (unsigned)-3;
}}
"#
        ),
    )
    .unwrap();
    let mut compilers = 0;
    for cc in ["gcc", "clang"] {
        if !process::optional_output(Command::new(cc).arg("--version")).is_some()
        {
            continue;
        }
        compilers += 1;
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=c11", "-Werror", level])
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
                "{cc} {level}: {printed}"
            );
        }
    }
    assert!(compilers > 0);
    std::fs::remove_file(src).unwrap();
    std::fs::remove_file(exe).unwrap();
}
