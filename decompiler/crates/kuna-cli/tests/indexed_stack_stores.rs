//! Authored ARM loops: computed stack stores feed direct reads and calls.
mod common;
use common::process;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

#[derive(Clone, Copy, Debug)]
enum Case {
    Byte,
    SecondByte,
    WordRead,
    Disjoint,
    Direct,
    External,
    Overwritten,
}

impl Case {
    fn expected(self) -> &'static str {
        match self {
            Self::Byte => "(first & 255) != 0",
            Self::WordRead => "(bytes[0] | bytes[1] | bytes[2]) != 0",
            Self::SecondByte => "bytes[1] != 0",
            Self::Direct => "bytes[2] != 0",
            Self::Disjoint | Self::External | Self::Overwritten => "0",
        }
    }
}

fn image(case: Case) -> Vec<u8> {
    // push; reserve; set pointer; zero slot; copy three elements; read slot;
    // test; conditional helper call; restore. scan.s documents the byte form.
    let mut words: Vec<u32> = vec![
        0xe92d4010, 0xe24dd010, 0xe1a0100d, 0xe3a02000, 0xe58d2000, 0xe3a03003, 0xe4d02001,
        0xe4c12001, 0xe2533001, 0x1afffffb, 0xe5dd0000, 0xe3500000, 0x0a000000, 0xeb000002,
        0xe28dd010, 0xe8bd4010, 0xe12fff1e,
    ];
    match case {
        Case::SecondByte => words[10] = 0xe5dd0001,
        Case::WordRead => words[10] = 0xe59d0000,
        Case::Disjoint => {
            words[4] = 0xe58d200c;
            words[10] = 0xe5dd000c;
        }
        Case::Direct => words[7] = 0xe5cd2000,
        Case::External => words[2] = 0xe1a01000,
        Case::Overwritten => words.insert(10, 0xe58d3000),
        Case::Byte => (),
    }
    let helper = words.len() as u64 * 4;
    words.extend([0xe3a01205, 0xe5810000, 0xe12fff1e]);
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &bytes, 4);
    for (name, value, size) in [("inspect_bytes", 0, helper), ("helper", helper, 12)] {
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
    obj.write().unwrap()
}

#[test]
fn indexed_stack_writes_preserve_the_dependent_call() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "a C compiler is required for semantic validation"
    );
    for case in [
        Case::Byte,
        Case::SecondByte,
        Case::WordRead,
        Case::Disjoint,
        Case::Direct,
        Case::External,
        Case::Overwritten,
    ] {
        let input = common::scratch_file("indexed-stack-store", "o");
        std::fs::write(&input, image(case)).unwrap();
        for full in [false, true] {
            if full && !matches!(case, Case::Byte | Case::SecondByte | Case::WordRead) {
                continue;
            }
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
            let param = "unsigned char";
            cmd.args([
                "decompile",
                input.to_str().unwrap(),
                "inspect_bytes",
                "--isa",
                "arm",
                "--mode",
                "reliable",
                "--assert-strict",
                "--assert",
                &format!("prototype inspect_bytes void inspect_bytes({param} *)"),
                "--assert",
                "prototype helper void helper(void)",
            ]);
            if full {
                cmd.args(["--option", "indexaliasguard", "full"]);
            }
            let output = cmd.output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let printed = String::from_utf8(output.stdout).unwrap();
            let src = common::scratch_file("indexed-stack-store", "c");
            let exe = common::scratch_file("indexed-stack-store", "exe");
            let prepare = "unsigned char bytes[3] = {first, first ? first ^ 0x5a : 0, first * 13};";
            std::fs::write(
                &src,
                format!(
                    r#"
#include <stdbool.h>
typedef unsigned char uint1;
typedef unsigned short uint2;
typedef unsigned int uint4;
typedef int int4;
static unsigned calls;
void helper(void) {{ ++calls; }}
{printed}
int main(void) {{
    unsigned extra[] = {{0x100, 0x10000, 0x80000000, 0xffffffff}};
    for (unsigned i = 0; i < 260; ++i) {{
        unsigned first = i < 256 ? i : extra[i - 256];
        {prepare}
        calls = 0;
        inspect_bytes(bytes);
        if (calls != ({expected})) return 1;
    }}
    return 0;
}}
"#,
                    expected = case.expected()
                ),
            )
            .unwrap();
            for cc in &compilers {
                for level in ["-O0", "-O2"] {
                    let compile = Command::new(cc)
                        .args(["-std=c11", level])
                        .arg(&src)
                        .arg("-o")
                        .arg(&exe)
                        .output()
                        .unwrap();
                    assert!(
                        compile.status.success(),
                        "{case:?} {cc}: {}\n{printed}",
                        String::from_utf8_lossy(&compile.stderr)
                    );
                    assert!(
                        Command::new(&exe).status().unwrap().success(),
                        "{case:?} full={full} {cc} {level}: {printed}"
                    );
                }
            }
            for path in [src, exe] {
                std::fs::remove_file(path).unwrap();
            }
        }
        std::fs::remove_file(input).unwrap();
    }
}

#[test]
fn the_broad_guard_and_explicit_off_settings_remain_available() {
    let input = common::scratch_file("stack-store-options", "o");
    std::fs::write(&input, image(Case::Byte)).unwrap();
    for (level, narrow, call) in [
        ("load", "off", false),
        ("off", "on", false),
        ("full", "off", true),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                input.to_str().unwrap(),
                "inspect_bytes",
                "--isa",
                "arm",
                "--mode",
                "reliable",
                "--option",
                "indexaliasguard",
                level,
                "--option",
                "stackstoreguard",
                narrow,
                "--assert",
                "prototype helper void helper(void)",
                "--assert-strict",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert_eq!(text.contains("helper();"), call, "{level} {narrow}: {text}");
    }
    std::fs::remove_file(input).unwrap();
}

#[test]
fn byte_reads_after_indexed_stores_keep_their_offsets_on_mips() {
    // clang -O0 for MIPS: zero two stack words, store 1 and 2 at computed
    // byte offsets, then read two fixed bytes and return first * 10 + second.
    let mut words: Vec<u32> = vec![
        0x27bdffe8, 0xafbf0014, 0xafbe0010, 0x03a0f025, 0xafc4000c, 0xafc50008, 0x24010000,
        0xafc00000, 0x24010000, 0xafc00004, 0x27c10000, 0x8fc2000c, 0x24030007, 0x00431024,
        0x00221021, 0x24010001, 0xa0410000, 0x27c10000, 0x8fc20008, 0x24030007, 0x00431024,
        0x00221021, 0x24010002, 0xa0410000, 0x93c10000, 0x302100ff, 0x2402000a, 0x70220802,
        0x93c20001, 0x304200ff, 0x00221021, 0x03c0e825, 0x8fbe0010, 0x8fbf0014, 0x27bd0018,
        0x03e00008, 0x00000000,
    ];
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "a C compiler is required for semantic validation"
    );
    for (target, first) in [("MIPS:BE:32:default", 0), ("MIPS:LE:32:default", 2)] {
        words[24] = 0x93c10000 | first;
        let bytes: Vec<u8> = if target.contains(":BE:") {
            words.iter().flat_map(|w| w.to_be_bytes()).collect()
        } else {
            words.iter().flat_map(|w| w.to_le_bytes()).collect()
        };
        let input = common::scratch_file("indexed-stack-bytes", "bin");
        std::fs::write(&input, bytes).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args([
                "decompile",
                input.to_str().unwrap(),
                "0x1000",
                "--raw-image",
                "--target",
                target,
                "--base",
                "0x1000",
                "--define-function",
                "0x1000=two_idx",
                "--assert",
                "prototype two_idx int two_idx(int,int)",
                "--assert-strict",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{target}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let printed = String::from_utf8(output.stdout).unwrap();
        let src = common::scratch_file("indexed-stack-bytes", "c");
        let exe = common::scratch_file("indexed-stack-bytes", "exe");
        std::fs::write(
            &src,
            format!(
                r#"
{printed}
int main(void) {{
    for (int i = 0; i < 16; ++i)
        for (int j = 0; j < 16; ++j) {{
            unsigned char b[8] = {{0}};
            b[i & 7] = 1;
            b[j & 7] = 2;
            if (two_idx(i, j) != b[{first}] * 10 + b[1]) return 1;
        }}
    return 0;
}}
"#
            ),
        )
        .unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args(["-std=c11", level])
                    .arg(&src)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .unwrap();
                assert!(
                    compile.status.success(),
                    "{target} {cc}: {}\n{printed}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                assert!(
                    Command::new(&exe).status().unwrap().success(),
                    "{target} {cc} {level}: {printed}"
                );
            }
        }
        for path in [input, src, exe] {
            std::fs::remove_file(path).unwrap();
        }
    }
}

/// `name._<off>_<size>_` read as the bytes at that offset, for a little-endian host.
fn lower_piece_reads(c: &str) -> String {
    let mut out = String::new();
    let mut rest = c;
    while let Some(pos) = rest.find("._") {
        let (head, tail) = rest.split_at(pos);
        let fields: Vec<&str> = tail[2..].splitn(3, '_').collect();
        let start = head
            .rfind(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .map_or(0, |p| p + 1);
        match (fields.as_slice(), start < head.len()) {
            ([off, size, _], true) if off.parse::<u8>().is_ok() && size.parse::<u8>().is_ok() => {
                out.push_str(&head[..start]);
                out.push_str(&format!("kuna_piece(&{}, {off}, {size})", &head[start..]));
                rest = &tail[2 + off.len() + 1 + size.len() + 1..];
            }
            _ => {
                out.push_str(&rest[..pos + 2]);
                rest = &rest[pos + 2..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn word_reads_after_indexed_stores_match_the_x86_64_binary() {
    // gcc and clang -O2 -fno-stack-protector -fcf-protection=none of
    //   f:  union { unsigned w; unsigned char b[4]; } u; u.w = 0x01020304;
    //       u.b[i & 3] = j; return u.b[1] | u.b[2] << 8;
    //   fq: the same over eight bytes, u.q = 0x0102030405060708, u.b[i & 7] = j
    //   fk: u.q = 0; u.b[i & 7] = j; u.b[(j >> 1) & 7] = 5;
    //       return (int)(u.q >> 8) & 0xffff;
    let functions: [(&str, &[u8]); 4] = [
        (
            "f_gcc",
            &[
                0x83, 0xe7, 0x03, 0xc7, 0x44, 0x24, 0xfc, 0x04, 0x03, 0x02, 0x01, 0x40, 0x88, 0x74,
                0x3c, 0xfc, 0x0f, 0xb7, 0x44, 0x24, 0xfd, 0xc3,
            ],
        ),
        (
            "fq_gcc",
            &[
                0x48, 0xb8, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x83, 0xe7, 0x07, 0x48,
                0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0x0f, 0xb7, 0x44, 0x24, 0xf9,
                0xc3,
            ],
        ),
        (
            "fk_gcc",
            &[
                0x48, 0xc7, 0x44, 0x24, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x83, 0xe7, 0x07, 0x40, 0x88,
                0x74, 0x3c, 0xf8, 0xd1, 0xfe, 0x83, 0xe6, 0x07, 0xc6, 0x44, 0x34, 0xf8, 0x05, 0x0f,
                0xb7, 0x44, 0x24, 0xf9, 0xc3,
            ],
        ),
        (
            "f_clang",
            &[
                0xc7, 0x44, 0x24, 0xf8, 0x04, 0x03, 0x02, 0x01, 0x83, 0xe7, 0x03, 0x40, 0x88, 0x74,
                0x3c, 0xf8, 0x0f, 0xb7, 0x44, 0x24, 0xf9, 0xc3,
            ],
        ),
    ];
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "a C compiler is required for semantic validation"
    );
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    for (name, code) in functions {
        let value = obj.append_section_data(section, code, 32);
        obj.add_symbol(Symbol {
            name: format!("binary_{name}").into_bytes(),
            value,
            size: code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(section),
            flags: SymbolFlags::None,
        });
    }
    let input = common::scratch_file("indexed-stack-words", "o");
    std::fs::write(&input, obj.write().unwrap()).unwrap();
    let mut printed = String::new();
    let mut checks = String::new();
    for (name, _) in functions {
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", input.to_str().unwrap(), &format!("binary_{name}")])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        printed.push_str(&text.replace(&format!("binary_{name}("), &format!("printed_{name}(")));
        checks.push_str(&format!(
            "int binary_{name}(int, int);\n\
             static int check_{name}(int i, int j) {{ return printed_{name}(i, j) == binary_{name}(i, j); }}\n"
        ));
    }
    let names: Vec<_> = functions.iter().map(|(name, _)| format!("check_{name}")).collect();
    let src = common::scratch_file("indexed-stack-words", "c");
    let exe = common::scratch_file("indexed-stack-words", "exe");
    std::fs::write(
        &src,
        format!(
            r#"
#include <string.h>
static unsigned long long kuna_piece(const void *p, int off, int size) {{
    unsigned long long v = 0;
    memcpy(&v, (const char *)p + off, size);
    return v;
}}
{printed}
{checks}
int main(void) {{
    int (*checks[])(int, int) = {{{names}}};
    for (unsigned k = 0; k < sizeof checks / sizeof *checks; ++k)
        for (int i = 0; i < 16; ++i)
            for (int j = 0; j < 256; ++j)
                if (!checks[k](i, j)) return 1 + k;
    return 0;
}}
"#,
            printed = lower_piece_reads(&printed),
            names = names.join(", "),
        ),
    )
    .unwrap();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=c11", level])
                .arg(&src)
                .arg(&input)
                .arg("-o")
                .arg(&exe)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{cc}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let run = Command::new(&exe).status().unwrap();
            assert!(run.success(), "{cc} {level}: {run}\n{printed}");
        }
    }
    for path in [input, src, exe] {
        std::fs::remove_file(path).unwrap();
    }
}
