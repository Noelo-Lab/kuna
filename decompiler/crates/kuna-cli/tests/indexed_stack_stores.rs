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

/// An x86-64 relocatable object defining `binary_<name>` for each function.
fn x86_64_object<'a>(
    stem: &str,
    functions: impl Iterator<Item = (&'a str, &'a [u8])>,
) -> std::path::PathBuf {
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
    let input = common::scratch_file(stem, "o");
    std::fs::write(&input, obj.write().unwrap()).unwrap();
    input
}

/// The C `kuna decompile` prints for `binary_<name>` with `options`.
fn decompile(input: &std::path::Path, name: &str, options: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile",
            input.to_str().unwrap(),
            &format!("binary_{name}"),
        ])
        .args(options)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// `name._<off>_<size>_` (also `name[k]._<off>_<size>_`) as the unsigned
/// `size`-byte lvalue at that offset, for a little-endian host (a 16-byte one
/// through a packed struct, which needs no alignment).
fn lower_pieces(c: &str) -> String {
    let mut out = String::new();
    let mut rest = c;
    while let Some(pos) = rest.find("._") {
        let (head, tail) = rest.split_at(pos);
        let fields: Vec<&str> = tail[2..].splitn(3, '_').collect();
        let mut start = head.len();
        while head[..start].ends_with(']') {
            match head[..start].rfind('[') {
                Some(open) => start = open,
                None => break,
            }
        }
        start = head[..start]
            .rfind(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .map_or(0, |p| p + 1);
        let width = |size: &str| match size {
            "1" => Some("unsigned char"),
            "2" => Some("unsigned short"),
            "4" => Some("unsigned int"),
            "8" => Some("unsigned long long"),
            "16" => Some("struct __attribute__((packed)) { unsigned __int128 v; }"),
            _ => None,
        };
        match (fields.as_slice(), start < head.len()) {
            ([off, size, _], true) if off.parse::<u8>().is_ok() && width(size).is_some() => {
                out.push_str(&head[..start]);
                let field = if *size == "16" { ".v" } else { "" };
                out.push_str(&format!(
                    "(*({} *)((char *)&({}) + {off})){field}",
                    width(size).unwrap(),
                    &head[start..]
                ));
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
    // and gcc, with the same flags, over union { unsigned w[2]; unsigned short
    // h[4]; unsigned char b[8]; } u:
    //   k11 (-O0): u.w[0] = 0x01020304; u.w[1] = 0x05060708; u.b[i & 7] = j;
    //       return u.h[0] + u.h[3] * 5;
    //   k11u (-O0): the same with u.b[i] = j, called with i < 8
    //   k4 (-O2): u.w[0] = 0x01020304; for (k = 0; k < (j & 3); k++)
    //       u.b[(i + k) & 3] = j + k; return u.b[1] | u.b[2] << 8;
    //   k8s (-O2): u.w[0] = 0x81828384; u.b[i & 3] = j;
    //       short r; memcpy(&r, &u.b[1], 2); return r;
    //   fe (-O2): union { u64 q; double d; u8 b[8]; } u; u.q = 0x400e000000000000;
    //       u.b[i & 7] = j; double x = u.d; return (int)(x + x);
    //   fi (-O2): union { float f[2]; u8 b[8]; } u; u.f[0] = 1.5f;
    //       u.f[1] = -3.25f; u.b[i & 7] = j; return (int)(u.f[1] + u.f[0]);
    //   u3 (-O2): k11's stores, then u8 *p = (j & 2) ? &u.b[0] : &u.b[4];
    //       *p ^= 1; return u.h[0] + u.h[3] * 5;
    //   w1 (-O0): k11's stores, then u8 *p = (j & 2) ? &u.b[0] : &u.b[4];
    //       for (k = 0; k < (i & 3); k++) *p++ ^= 1; return u.h[0] + u.h[3] * 5;
    // and clang, with the same flags:
    //   v8 (-O2): U8 u, v; u.q = 0x1122334455667788; v.q = 0x0102030405060708;
    //       u.b[i & 7] = j; v.b[(i >> 3) & 1] = j; U8 *p = (j & 1) ? &u : &v;
    //       return u.h[1] + v.h[3] * 3 + p->b[2];
    // Each entry carries the mask the caller applies to i.
    let functions: [(&str, &[u8], u8); 13] = [
        (
            "f_gcc",
            &[
                0x83, 0xe7, 0x03, 0xc7, 0x44, 0x24, 0xfc, 0x04, 0x03, 0x02, 0x01, 0x40, 0x88, 0x74,
                0x3c, 0xfc, 0x0f, 0xb7, 0x44, 0x24, 0xfd, 0xc3,
            ],
            15,
        ),
        (
            "fq_gcc",
            &[
                0x48, 0xb8, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x83, 0xe7, 0x07, 0x48,
                0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0x0f, 0xb7, 0x44, 0x24, 0xf9,
                0xc3,
            ],
            15,
        ),
        (
            "fk_gcc",
            &[
                0x48, 0xc7, 0x44, 0x24, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x83, 0xe7, 0x07, 0x40, 0x88,
                0x74, 0x3c, 0xf8, 0xd1, 0xfe, 0x83, 0xe6, 0x07, 0xc6, 0x44, 0x34, 0xf8, 0x05, 0x0f,
                0xb7, 0x44, 0x24, 0xf9, 0xc3,
            ],
            15,
        ),
        (
            "f_clang",
            &[
                0xc7, 0x44, 0x24, 0xf8, 0x04, 0x03, 0x02, 0x01, 0x83, 0xe7, 0x03, 0x40, 0x88, 0x74,
                0x3c, 0xf8, 0x0f, 0xb7, 0x44, 0x24, 0xf9, 0xc3,
            ],
            15,
        ),
        (
            "k11_gcc",
            &[
                0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xec, 0x89, 0x75, 0xe8, 0xc7, 0x45, 0xf8, 0x04,
                0x03, 0x02, 0x01, 0xc7, 0x45, 0xfc, 0x08, 0x07, 0x06, 0x05, 0x8b, 0x45, 0xec, 0x83,
                0xe0, 0x07, 0x8b, 0x55, 0xe8, 0x48, 0x98, 0x88, 0x54, 0x05, 0xf8, 0x0f, 0xb7, 0x45,
                0xf8, 0x0f, 0xb7, 0xc8, 0x0f, 0xb7, 0x45, 0xfe, 0x0f, 0xb7, 0xd0, 0x89, 0xd0, 0xc1,
                0xe0, 0x02, 0x01, 0xd0, 0x01, 0xc8, 0x5d, 0xc3,
            ],
            15,
        ),
        (
            "k11u_gcc",
            &[
                0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xec, 0x89, 0x75, 0xe8, 0xc7, 0x45, 0xf8, 0x04,
                0x03, 0x02, 0x01, 0xc7, 0x45, 0xfc, 0x08, 0x07, 0x06, 0x05, 0x8b, 0x45, 0xe8, 0x89,
                0xc2, 0x8b, 0x45, 0xec, 0x48, 0x98, 0x88, 0x54, 0x05, 0xf8, 0x0f, 0xb7, 0x45, 0xf8,
                0x0f, 0xb7, 0xc8, 0x0f, 0xb7, 0x45, 0xfe, 0x0f, 0xb7, 0xd0, 0x89, 0xd0, 0xc1, 0xe0,
                0x02, 0x01, 0xd0, 0x01, 0xc8, 0x5d, 0xc3,
            ],
            7,
        ),
        (
            "k4_gcc",
            &[
                0x89, 0xf1, 0xc7, 0x44, 0x24, 0xf8, 0x04, 0x03, 0x02, 0x01, 0xb8, 0x03, 0x02, 0x00,
                0x00, 0x83, 0xe1, 0x03, 0x74, 0x1c, 0x01, 0xf9, 0x29, 0xfe, 0x89, 0xf8, 0x8d, 0x14,
                0x3e, 0x83, 0xc7, 0x01, 0x83, 0xe0, 0x03, 0x88, 0x54, 0x04, 0xf8, 0x39, 0xcf, 0x75,
                0xed, 0x0f, 0xb7, 0x44, 0x24, 0xf9, 0xc3,
            ],
            15,
        ),
        (
            "k8s_gcc",
            &[
                0x83, 0xe7, 0x03, 0xc7, 0x44, 0x24, 0xf8, 0x84, 0x83, 0x82, 0x81, 0x40, 0x88, 0x74,
                0x3c, 0xf8, 0x0f, 0xbf, 0x44, 0x24, 0xf9, 0xc3,
            ],
            15,
        ),
        (
            "fe_gcc",
            &[
                0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x40, 0x83, 0xe7, 0x07, 0x48,
                0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0xf2, 0x0f, 0x10, 0x44, 0x24,
                0xf8, 0xf2, 0x0f, 0x58, 0xc0, 0xf2, 0x0f, 0x2c, 0xc0, 0xc3,
            ],
            15,
        ),
        (
            "fi_gcc",
            &[
                0x48, 0xb8, 0x00, 0x00, 0xc0, 0x3f, 0x00, 0x00, 0x50, 0xc0, 0x83, 0xe7, 0x07, 0x48,
                0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0xf3, 0x0f, 0x10, 0x44, 0x24,
                0xfc, 0xf3, 0x0f, 0x58, 0x44, 0x24, 0xf8, 0xf3, 0x0f, 0x2c, 0xc0, 0xc3,
            ],
            15,
        ),
        (
            "u3_gcc",
            &[
                0x48, 0xb8, 0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x83, 0xe7, 0x07, 0x48,
                0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0x83, 0xe6, 0x02, 0x74, 0x24,
                0x0f, 0xb6, 0x44, 0x24, 0xf8, 0x48, 0x8d, 0x54, 0x24, 0xf8, 0x83, 0xf0, 0x01, 0x88,
                0x02, 0x0f, 0xb7, 0x44, 0x24, 0xfe, 0x0f, 0xb7, 0x54, 0x24, 0xf8, 0x8d, 0x04, 0x80,
                0x01, 0xd0, 0xc3, 0x0f, 0x1f, 0x44, 0x00, 0x00, 0x0f, 0xb6, 0x44, 0x24, 0xfc, 0x48,
                0x8d, 0x54, 0x24, 0xfc, 0xeb, 0xda,
            ],
            15,
        ),
        (
            "w1_gcc",
            &[
                0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xdc, 0x89, 0x75, 0xd8, 0xc7, 0x45, 0xe8, 0x04,
                0x03, 0x02, 0x01, 0xc7, 0x45, 0xec, 0x08, 0x07, 0x06, 0x05, 0x8b, 0x45, 0xdc, 0x83,
                0xe0, 0x07, 0x8b, 0x55, 0xd8, 0x48, 0x98, 0x88, 0x54, 0x05, 0xe8, 0x8b, 0x45, 0xd8,
                0x83, 0xe0, 0x02, 0x85, 0xc0, 0x74, 0x06, 0x48, 0x8d, 0x45, 0xe8, 0xeb, 0x08, 0x48,
                0x8d, 0x45, 0xe8, 0x48, 0x83, 0xc0, 0x04, 0x48, 0x89, 0x45, 0xf8, 0xc7, 0x45, 0xf4,
                0x00, 0x00, 0x00, 0x00, 0xeb, 0x18, 0x48, 0x8b, 0x45, 0xf8, 0x48, 0x8d, 0x50, 0x01,
                0x48, 0x89, 0x55, 0xf8, 0x0f, 0xb6, 0x10, 0x83, 0xf2, 0x01, 0x88, 0x10, 0x83, 0x45,
                0xf4, 0x01, 0x8b, 0x45, 0xdc, 0x83, 0xe0, 0x03, 0x39, 0x45, 0xf4, 0x7c, 0xdd, 0x0f,
                0xb7, 0x45, 0xe8, 0x0f, 0xb7, 0xc8, 0x0f, 0xb7, 0x45, 0xee, 0x0f, 0xb7, 0xd0, 0x89,
                0xd0, 0xc1, 0xe0, 0x02, 0x01, 0xd0, 0x01, 0xc8, 0x5d, 0xc3,
            ],
            15,
        ),
        (
            "v8_clang",
            &[
                0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x48, 0x89, 0x44, 0x24,
                0xf0, 0x48, 0xb8, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x48, 0x89, 0x44,
                0x24, 0xf8, 0x89, 0xf8, 0x83, 0xe0, 0x07, 0x40, 0x88, 0x74, 0x04, 0xf0, 0xc1, 0xef,
                0x03, 0x83, 0xe7, 0x01, 0x40, 0xf6, 0xc6, 0x01, 0x48, 0x8d, 0x44, 0x24, 0xf8, 0x48,
                0x8d, 0x4c, 0x24, 0xf0, 0x48, 0x0f, 0x44, 0xc8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0x0f,
                0xb7, 0x44, 0x24, 0xf2, 0x0f, 0xb6, 0x49, 0x02, 0x01, 0xc8, 0x05, 0x06, 0x03, 0x00,
                0x00, 0xc3,
            ],
            15,
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
    let input = x86_64_object(
        "indexed-stack-words",
        functions.iter().map(|(name, code, _)| (*name, *code)),
    );
    let mut printed = String::new();
    let mut checks = String::new();
    for (name, _, mask) in functions {
        let text = decompile(&input, name, &[]);
        printed.push_str(&text.replace(&format!("binary_{name}("), &format!("printed_{name}(")));
        checks.push_str(&format!(
            "int binary_{name}(int, int);\n\
             static int check_{name}(int i, int j) {{ i &= {mask}; return printed_{name}(i, j) == binary_{name}(i, j); }}\n"
        ));
    }
    let names: Vec<_> = functions
        .iter()
        .map(|(name, _, _)| format!("check_{name}"))
        .collect();
    let src = common::scratch_file("indexed-stack-words", "c");
    let exe = common::scratch_file("indexed-stack-words", "exe");
    std::fs::write(
        &src,
        format!(
            r#"
#define builtin_memcpy __builtin_memcpy
#define builtin_memset __builtin_memset
#define builtin_strncpy __builtin_strncpy
{printed}
{checks}
int atoi(const char *);
unsigned alarm(unsigned);
int main(int argc, char **argv) {{
    int (*checks[])(int, int) = {{{names}}};
    int (*check)(int, int) = checks[atoi(argv[argc - 1])];
    alarm(20);
    for (int i = 0; i < 16; ++i)
        for (int j = 0; j < 256; ++j)
            if (!check(i, j)) return 1;
    return 0;
}}
"#,
            printed = lower_pieces(&printed),
            names = names.join(", "),
        ),
    )
    .unwrap();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=c11", "-fno-strict-aliasing", level])
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
            let failing: Vec<_> = functions
                .iter()
                .enumerate()
                .filter(|(k, _)| {
                    !Command::new(&exe)
                        .arg(k.to_string())
                        .status()
                        .unwrap()
                        .success()
                })
                .map(|(_, (name, _, _))| *name)
                .collect();
            assert!(
                failing.is_empty(),
                "{cc} {level}: {failing:?} disagree\n{printed}"
            );
        }
    }
    for path in [input, src, exe] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn float_reads_after_indexed_stores_are_no_worse_than_without_the_guard() {
    // clang -O2 -fno-stack-protector -fcf-protection=none of
    //   v3: union { unsigned w[2]; unsigned short h[4]; unsigned char b[8];
    //       float f[2]; } u; u.w[0] = 0x01020304; u.w[1] = 0x40100000;
    //       u.b[i & 7] = j; return u.h[0] + u.h[1] * 5 + (int)(u.f[1] * 2);
    //   fb: union { float f[2]; unsigned char b[8]; } u; u.f[0] = (float)j;
    //       u.f[1] = 2.25f; u.b[i & 7] = j; return (int)(u.f[1] * 8) + (int)u.f[0];
    // and gcc -O2, with the same flags, of
    //   w10: union { u64 q; double d; u16 h[4]; u8 b[8]; } u;
    //       u.q = 0x4010000000000000; u.b[i & 7] = j;
    //       return (int)u.d + u.h[0] + u.h[3] * 3;
    // A float read of a byte array prints as an integer piece, and an integer
    // read of a double local as a conversion, so these functions fall back to
    // the unguarded analysis.
    no_worse_than_without_the_guard(
        "indexed-stack-floats",
        &[
            (
                "v3_clang",
                16,
                &[
                    0x48, 0xb8, 0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x10, 0x40, 0x48, 0x89, 0x44,
                    0x24, 0xf8, 0x83, 0xe7, 0x07, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0x0f, 0xb7, 0x44,
                    0x24, 0xf8, 0x0f, 0xb7, 0x4c, 0x24, 0xfa, 0x8d, 0x0c, 0x89, 0x01, 0xc1, 0xf3,
                    0x0f, 0x10, 0x44, 0x24, 0xfc, 0xf3, 0x0f, 0x58, 0xc0, 0xf3, 0x0f, 0x2c, 0xc0,
                    0x01, 0xc8, 0xc3,
                ],
            ),
            (
                "fb_clang",
                16,
                &[
                    0xf3, 0x0f, 0x2a, 0xc6, 0xf3, 0x0f, 0x11, 0x44, 0x24, 0xf8, 0xc7, 0x44, 0x24,
                    0xfc, 0x00, 0x00, 0x10, 0x40, 0x83, 0xe7, 0x07, 0x40, 0x88, 0x74, 0x3c, 0xf8,
                    0xf3, 0x0f, 0x10, 0x44, 0x24, 0xfc, 0xf3, 0x0f, 0x58, 0xc0, 0xf3, 0x0f, 0x58,
                    0xc0, 0xf3, 0x0f, 0x58, 0xc0, 0xf3, 0x0f, 0x2c, 0xc8, 0xf3, 0x0f, 0x2c, 0x44,
                    0x24, 0xf8, 0x01, 0xc8, 0xc3,
                ],
            ),
            (
                "w10_gcc",
                16,
                &[
                    0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, 0x83, 0xe7, 0x07,
                    0x48, 0x89, 0x44, 0x24, 0xf8, 0x40, 0x88, 0x74, 0x3c, 0xf8, 0xf2, 0x0f, 0x2c,
                    0x44, 0x24, 0xf8, 0x0f, 0xb7, 0x54, 0x24, 0xf8, 0x01, 0xd0, 0x0f, 0xb7, 0x54,
                    0x24, 0xfe, 0x8d, 0x14, 0x52, 0x01, 0xd0, 0xc3,
                ],
            ),
        ],
    );
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn split_buffers_after_indexed_stores_are_no_worse_than_without_the_guard() {
    // clang -O0 -fno-stack-protector -fcf-protection=none of
    //   x6: union { u32 w[2]; u16 h[4]; u8 b[8]; u64 q; } u, v;
    //       u.q = 0x1122334455667788; v.q = 0x0102030405060708;
    //       u8 *p = (j & 1) ? u.b : v.b; for (k = 0; k < (i & 3); k++) *p++ = j;
    //       u.b[i & 7] ^= 1; return u.h[1] + v.h[3] * 3;
    // The guard keeps v's initializer, but v's bytes are written only by a
    // walk through a pointer kept in memory, which no guard covers, so the
    // layout splits v and the function falls back to the unguarded analysis.
    no_worse_than_without_the_guard(
        "indexed-stack-splits",
        &[(
            "x6_clang",
            16,
            &[
                0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xfc, 0x89, 0x75, 0xf8, 0x48, 0xb8, 0x88, 0x77,
                0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x48, 0x89, 0x45, 0xf0, 0x48, 0xb8, 0x08, 0x07,
                0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x48, 0x89, 0x45, 0xe8, 0x8b, 0x45, 0xf8, 0x83,
                0xe0, 0x01, 0x83, 0xf8, 0x00, 0x0f, 0x84, 0x0d, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x45,
                0xf0, 0x48, 0x89, 0x45, 0xd0, 0xe9, 0x08, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x45, 0xe8,
                0x48, 0x89, 0x45, 0xd0, 0x48, 0x8b, 0x45, 0xd0, 0x48, 0x89, 0x45, 0xe0, 0xc7, 0x45,
                0xdc, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x45, 0xdc, 0x8b, 0x4d, 0xfc, 0x83, 0xe1, 0x03,
                0x39, 0xc8, 0x0f, 0x8d, 0x24, 0x00, 0x00, 0x00, 0x8b, 0x45, 0xf8, 0x88, 0xc1, 0x48,
                0x8b, 0x45, 0xe0, 0x48, 0x89, 0xc2, 0x48, 0x83, 0xc2, 0x01, 0x48, 0x89, 0x55, 0xe0,
                0x88, 0x08, 0x8b, 0x45, 0xdc, 0x83, 0xc0, 0x01, 0x89, 0x45, 0xdc, 0xe9, 0xcb, 0xff,
                0xff, 0xff, 0x8b, 0x45, 0xfc, 0x83, 0xe0, 0x07, 0x48, 0x98, 0x0f, 0xb6, 0x4c, 0x05,
                0xf0, 0x83, 0xf1, 0x01, 0x88, 0x4c, 0x05, 0xf0, 0x0f, 0xb7, 0x45, 0xf2, 0x0f, 0xb7,
                0x4d, 0xee, 0x6b, 0xc9, 0x03, 0x01, 0xc8, 0x5d, 0xc3,
            ],
        )],
    );
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn unresolved_and_unbounded_stores_are_no_worse_than_without_the_guard() {
    // gcc -fno-stack-protector -fcf-protection=none, -O0 of
    //   y3u: union { u32 w[2]; u16 h[4]; u8 b[8]; u64 q; } u, v;
    //       u.w[0] = 0x01020304; u.w[1] = 0x05060708; v.q = 0x1111111111111111;
    //       *(u8 *)((uintptr_t)&v.b[0] | (j & 7)) = 0x55; u.b[i] = j;
    //       return u.h[0] + u.h[3] * 5 + v.h[1] * 3;   (i < 8)
    // and -O2 of
    //   y8: struct { u32 a; u8 b[6]; u16 c; u32 d; } s; s.a = 0x01020304;
    //       s.b[k] = k + 1 (k < 6); s.c = 0x0707; s.d = 0x08080808;
    //       ((u8 *)&s)[i] = j; return s.a + s.b[2] + s.c * 3 + (s.d >> 8);
    // and -O2 of
    //   b1: union { u8 b[512]; u32 w[128]; } u; u.w[100] = 0x01020304;
    //       u.b[i & 511] = j; return u.w[100];
    //   b4: the same union with u.w[1] = 0x0a0b0c0d; u.w[100] = 0x01020304;
    //       u8 *p = u.b; for (k = 0; k < (i & 511); k++) *p++ = j;
    //       return u.w[100] + u.w[1] * 3;
    //   o1: u8 b[12]; for (k = 0; k < 12; k++) b[k] = 0; b[i % 12] = j;
    //       if (b[0]) ext(b[0]); return b[0];   (its call bound to o1_ext)
    //   z5: u8 b[24] = {0}; b[i % 24] = j; return b[0] + b[8] * 3;
    //   v1: u8 b[16] = {0}; for (k = 0; k < (i & 15); k++) b[k] = j;
    //       return b[1] * 3 + 1;
    // y3u's OR-formed store does not resolve to stack offsets, and y8's
    // unmasked index, b1's 512-byte index and b4's walk may write the fields
    // the layout maps as other locals, so all fall back to the unguarded
    // analysis. So do o1, whose call keeps the zeroed tail of b as a local
    // apart from the eight bytes at the store's base, and z5, whose base is
    // an `undefined16` array that cannot be indexed by byte. v1's fill keeps
    // one byte array.
    no_worse_than_without_the_guard(
        "indexed-stack-unbounded",
        &[
            (
                "y3u_gcc",
                8,
                &[
                    0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xec, 0x89, 0x75, 0xe8, 0xc7, 0x45, 0xf8,
                    0x04, 0x03, 0x02, 0x01, 0xc7, 0x45, 0xfc, 0x08, 0x07, 0x06, 0x05, 0x48, 0xb8,
                    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x48, 0x89, 0x45, 0xf0, 0x48,
                    0x8d, 0x55, 0xf0, 0x8b, 0x45, 0xe8, 0x48, 0x98, 0x83, 0xe0, 0x07, 0x48, 0x09,
                    0xd0, 0xc6, 0x00, 0x55, 0x8b, 0x45, 0xe8, 0x89, 0xc2, 0x8b, 0x45, 0xec, 0x48,
                    0x98, 0x88, 0x54, 0x05, 0xf8, 0x0f, 0xb7, 0x45, 0xf8, 0x0f, 0xb7, 0xc8, 0x0f,
                    0xb7, 0x45, 0xfe, 0x0f, 0xb7, 0xd0, 0x89, 0xd0, 0xc1, 0xe0, 0x02, 0x01, 0xd0,
                    0x01, 0xc1, 0x0f, 0xb7, 0x45, 0xf2, 0x0f, 0xb7, 0xd0, 0x89, 0xd0, 0x01, 0xc0,
                    0x01, 0xd0, 0x01, 0xc8, 0x5d, 0xc3,
                ],
            ),
            (
                "y8_gcc",
                16,
                &[
                    0xb8, 0x07, 0x07, 0x00, 0x00, 0x48, 0x63, 0xff, 0xc7, 0x44, 0x24, 0xe8, 0x04,
                    0x03, 0x02, 0x01, 0x66, 0x89, 0x44, 0x24, 0xf2, 0xc6, 0x44, 0x24, 0xee, 0x03,
                    0xc7, 0x44, 0x24, 0xf4, 0x08, 0x08, 0x08, 0x08, 0x40, 0x88, 0x74, 0x3c, 0xe8,
                    0x8b, 0x44, 0x24, 0xf4, 0x0f, 0xb6, 0x54, 0x24, 0xee, 0xc1, 0xe8, 0x08, 0x01,
                    0xd0, 0x0f, 0xb7, 0x54, 0x24, 0xf2, 0x03, 0x44, 0x24, 0xe8, 0x8d, 0x14, 0x52,
                    0x01, 0xd0, 0xc3,
                ],
            ),
            (
                "b1_gcc",
                512,
                &[
                    0x48, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00, 0x81, 0xe7, 0xff, 0x01, 0x00, 0x00,
                    0xc7, 0x84, 0x24, 0x18, 0x01, 0x00, 0x00, 0x04, 0x03, 0x02, 0x01, 0x40, 0x88,
                    0x74, 0x3c, 0x88, 0x8b, 0x84, 0x24, 0x18, 0x01, 0x00, 0x00, 0x48, 0x81, 0xc4,
                    0x90, 0x01, 0x00, 0x00, 0xc3,
                ],
            ),
            (
                "b4_gcc",
                512,
                &[
                    0x48, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00, 0x89, 0xfa, 0xc7, 0x44, 0x24, 0x8c,
                    0x0d, 0x0c, 0x0b, 0x0a, 0xc7, 0x84, 0x24, 0x18, 0x01, 0x00, 0x00, 0x04, 0x03,
                    0x02, 0x01, 0x81, 0xe2, 0xff, 0x01, 0x00, 0x00, 0x74, 0x5c, 0x40, 0x0f, 0xb6,
                    0xf6, 0x48, 0x8d, 0x7c, 0x24, 0x88, 0x83, 0xfa, 0x08, 0x72, 0x1a, 0x48, 0xb9,
                    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x40, 0x0f, 0xb6, 0xc6, 0x48,
                    0x0f, 0xaf, 0xc1, 0x89, 0xd1, 0xc1, 0xe9, 0x03, 0xf3, 0x48, 0xab, 0x83, 0xe2,
                    0x07, 0x74, 0x0f, 0x31, 0xc0, 0x89, 0xc1, 0x83, 0xc0, 0x01, 0x40, 0x88, 0x34,
                    0x0f, 0x39, 0xd0, 0x72, 0xf3, 0x8b, 0x44, 0x24, 0x8c, 0x8d, 0x04, 0x40, 0x03,
                    0x84, 0x24, 0x18, 0x01, 0x00, 0x00, 0x48, 0x81, 0xc4, 0x90, 0x01, 0x00, 0x00,
                    0xc3, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0x2b,
                    0x27, 0x23, 0x1f, 0x48, 0x81, 0xc4, 0x90, 0x01, 0x00, 0x00, 0xc3,
                ],
            ),
            (
                "o1_gcc",
                24,
                &[
                    0x48, 0x63, 0xc7, 0x89, 0xfa, 0x41, 0x54, 0x45, 0x31, 0xe4, 0x48, 0x69, 0xc0,
                    0xab, 0xaa, 0xaa, 0x2a, 0xc1, 0xfa, 0x1f, 0x48, 0x83, 0xec, 0x10, 0x48, 0xc1,
                    0xf8, 0x21, 0xc7, 0x44, 0x24, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x44,
                    0x24, 0x04, 0x00, 0x00, 0x00, 0x00, 0x29, 0xd0, 0x8d, 0x04, 0x40, 0xc1, 0xe0,
                    0x02, 0x29, 0xc7, 0x48, 0x63, 0xff, 0x40, 0x88, 0x74, 0x3c, 0x04, 0x0f, 0xb6,
                    0x44, 0x24, 0x04, 0x84, 0xc0, 0x75, 0x10, 0x48, 0x83, 0xc4, 0x10, 0x44, 0x89,
                    0xe0, 0x41, 0x5c, 0xc3, 0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00, 0x44, 0x0f, 0xb6,
                    0xe0, 0x44, 0x89, 0xe7, 0xe8, 0x1c, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x10,
                    0x44, 0x89, 0xe0, 0x41, 0x5c, 0xc3,
                ],
            ),
            ("o1_ext", 0, &[0x8d, 0x44, 0x7f, 0x01, 0xc3]),
            (
                "z5_gcc",
                48,
                &[
                    0x48, 0x63, 0xc7, 0x89, 0xfa, 0x66, 0x0f, 0xef, 0xc0, 0x48, 0x69, 0xc0, 0xab,
                    0xaa, 0xaa, 0x2a, 0xc1, 0xfa, 0x1f, 0x0f, 0x29, 0x44, 0x24, 0xd8, 0x48, 0xc1,
                    0xf8, 0x22, 0x29, 0xd0, 0x8d, 0x04, 0x40, 0xc1, 0xe0, 0x03, 0x29, 0xc7, 0x48,
                    0x63, 0xff, 0x40, 0x88, 0x74, 0x3c, 0xd8, 0x0f, 0xb6, 0x44, 0x24, 0xe0, 0x0f,
                    0xb6, 0x54, 0x24, 0xd8, 0x8d, 0x04, 0x40, 0x01, 0xd0, 0xc3,
                ],
            ),
            (
                "v1_gcc",
                16,
                &[
                    0x66, 0x0f, 0xef, 0xc0, 0x89, 0xf9, 0x0f, 0x29, 0x44, 0x24, 0xe8, 0x83, 0xe1,
                    0x0f, 0x74, 0x60, 0x4c, 0x8d, 0x44, 0x24, 0xe8, 0x40, 0x0f, 0xb6, 0xf6, 0x4c,
                    0x89, 0xc2, 0x83, 0xf9, 0x08, 0x73, 0x1f, 0x83, 0xe1, 0x07, 0x74, 0x0f, 0x31,
                    0xc0, 0x89, 0xc7, 0x83, 0xc0, 0x01, 0x40, 0x88, 0x34, 0x3a, 0x39, 0xc8, 0x72,
                    0xf3, 0x0f, 0xb6, 0x44, 0x24, 0xe9, 0x8d, 0x44, 0x40, 0x01, 0xc3, 0x90, 0x48,
                    0xb8, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x40, 0x0f, 0xb6, 0xd6,
                    0x83, 0xe7, 0x08, 0x48, 0x0f, 0xaf, 0xd0, 0x31, 0xc0, 0x41, 0x89, 0xc1, 0x83,
                    0xc0, 0x08, 0x4b, 0x89, 0x14, 0x08, 0x39, 0xf8, 0x72, 0xf2, 0x49, 0x8d, 0x14,
                    0x00, 0xeb, 0xb6, 0x0f, 0x1f, 0x44, 0x00, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00,
                    0xc3,
                ],
            ),
        ],
    );
}

/// Each function's guarded print must disagree with the binary on no more
/// inputs `i < rows`, `j < 256` than its print with `stackstoreguard off`; a
/// crash or a print that does not compile counts as worse. A function with no
/// rows is a callee, `int binary_<name>(int)`, placed right after its caller.
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn no_worse_than_without_the_guard(stem: &str, functions: &[(&str, u32, &[u8])]) {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "a C compiler is required for semantic validation"
    );
    let input = x86_64_object(stem, functions.iter().map(|&(name, _, code)| (name, code)));
    let mut printed = String::new();
    let mut checks = String::new();
    for &(name, rows, _) in functions {
        if rows == 0 {
            printed.insert_str(0, &format!("int binary_{name}(int);\n"));
            continue;
        }
        for (arm, options) in [
            ("guarded", &[][..]),
            ("unguarded", &["--option", "stackstoreguard", "off"][..]),
        ] {
            let text = decompile(&input, name, options);
            printed.push_str(&text.replace(&format!("binary_{name}("), &format!("{arm}_{name}(")));
        }
        checks.push_str(&format!(
            "int binary_{name}(int, int);\n\
             static int check_{name}(void) {{\n\
                 int guarded = 0, unguarded = 0;\n\
                 for (int i = 0; i < {rows}; ++i)\n\
                     for (int j = 0; j < 256; ++j) {{\n\
                         int want = binary_{name}(i, j);\n\
                         guarded += guarded_{name}(i, j) != want;\n\
                         unguarded += unguarded_{name}(i, j) != want;\n\
                     }}\n\
                 return guarded <= unguarded;\n\
             }}\n"
        ));
    }
    let checked: Vec<_> = functions.iter().filter(|&&(_, rows, _)| rows > 0).collect();
    let names: Vec<_> = checked
        .iter()
        .map(|(name, _, _)| format!("check_{name}"))
        .collect();
    let src = common::scratch_file(stem, "c");
    let exe = common::scratch_file(stem, "exe");
    std::fs::write(
        &src,
        format!(
            r#"
{printed}
{checks}
int atoi(const char *);
unsigned alarm(unsigned);
int main(int argc, char **argv) {{
    int (*checks[])(void) = {{{names}}};
    alarm(20);
    return !checks[atoi(argv[argc - 1])]();
}}
"#,
            printed = lower_pieces(&printed),
            names = names.join(", "),
        ),
    )
    .unwrap();
    for cc in &compilers {
        for level in ["-O0", "-O2"] {
            let compile = Command::new(cc)
                .args(["-std=c11", "-fno-strict-aliasing", level])
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
            let failing: Vec<_> = checked
                .iter()
                .enumerate()
                .filter(|(k, _)| {
                    !Command::new(&exe)
                        .arg(k.to_string())
                        .status()
                        .unwrap()
                        .success()
                })
                .map(|(_, (name, _, _))| *name)
                .collect();
            assert!(
                failing.is_empty(),
                "{cc} {level}: {failing:?} disagree more often with the guard\n{printed}"
            );
        }
    }
    for path in [input, src, exe] {
        std::fs::remove_file(path).unwrap();
    }
}
