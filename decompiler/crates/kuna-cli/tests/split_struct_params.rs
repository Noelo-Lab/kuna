//! A struct parameter whose pieces span registers and the stack stays the
//! parameter: a later write of one of its registers is not a write into it,
//! and copying its register pieces to memory is not lost.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

fn image(arch: Architecture, e_flags: u32, functions: &[(&str, Vec<u32>)]) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, arch, Endianness::Little);
    object.flags = FileFlags::Elf {
        os_abi: 0,
        abi_version: 0,
        e_flags,
    };
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
            bytes.extend(word.to_le_bytes());
        }
    }
    object.append_section_data(text, &bytes, 4);
    object.write().unwrap()
}

/// `int d1(int *k, int a, struct s3 s) { return take3(s, *k) + a; }` and
/// `int d4(int a, int b, struct s3 s) { return use3(&s) + a; }` with
/// `struct s3 { int x, y, z; }`, clang -O2 for armv7 hard-float: `s` is r2,
/// r3 and the first stack word.
fn arm() -> Vec<u8> {
    let bl = |from: u32, to: u32| 0xeb000000 | ((to as i32 - from as i32 - 2) as u32 & 0xffffff);
    let d1 = vec![
        0xe92d4070,
        0xe1a04003,
        0xe1a05002,
        0xe5903000,
        0xe1a06001,
        0xe59d2010,
        0xe1a00005,
        0xe1a01004,
        bl(8, 22),
        0xe0800006,
        0xe8bd8070,
    ];
    let d4 = vec![
        0xe92d4010,
        0xe24dd010,
        0xe1a04000,
        0xe59d0018,
        0xe58d000c,
        0xe28d0004,
        0xe98d000c,
        bl(18, 23),
        0xe0800004,
        0xe28dd010,
        0xe8bd8010,
    ];
    image(
        Architecture::Arm,
        0x05000000,
        &[
            ("d1", d1),
            ("d4", d4),
            ("take3", vec![0xe12fff1e]),
            ("use3", vec![0xe12fff1e]),
        ],
    )
}

/// The same two functions, clang -O2 -fno-pic for little-endian MIPS o32: `s`
/// is a2, a3 and the stack word at 0x10.
fn mips() -> Vec<u8> {
    let bal = |from: u32, to: u32| 0x04110000 | ((to as i32 - from as i32 - 1) as u32 & 0xffff);
    let d1 = vec![
        0x27bdffe8,
        0xafbf0014,
        0xafb00010,
        0x00e00825,
        0x00c01025,
        0x00a08025,
        0x8c870000,
        0x8fa60028,
        0x00402025,
        bal(9, 31),
        0x00202825,
        0x00501021,
        0x8fb00010,
        0x8fbf0014,
        0x03e00008,
        0x27bd0018,
    ];
    let d4 = vec![
        0x27bdffd8,
        0xafbf0024,
        0xafb00020,
        0x00808025,
        0x8fa10038,
        0xafa10018,
        0xafa70014,
        0xafa60010,
        bal(24, 33),
        0x27a40010,
        0x00501021,
        0x8fb00020,
        0x8fbf0024,
        0x03e00008,
        0x27bd0028,
    ];
    let ret = vec![0x03e00008, 0x00000000];
    image(
        Architecture::Mips,
        0x70001001,
        &[
            ("d1", d1),
            ("d4", d4),
            ("take3", ret.clone()),
            ("use3", ret),
        ],
    )
}

/// `long r1(long *k, long b, long c, long d, long e, long f, long g, struct l2 s)
/// { return take8(0, b, c, d, e, f, g, *k) * s.a + s.b; }` with
/// `struct l2 { long a, b; }` for RISC-V lp64d, as clang -O2 -march=rv64g
/// emits it but loading `*k` straight into a7: `s` is a7 and the first stack
/// doubleword.
fn riscv() -> Vec<u8> {
    let jal = |from: u32, to: u32| {
        let off = ((to as i32 - from as i32) * 4) as u32;
        (((off >> 20) & 1) << 31)
            | (((off >> 1) & 0x3ff) << 21)
            | (((off >> 11) & 1) << 20)
            | (((off >> 12) & 0xff) << 12)
            | (1 << 7)
            | 0x6f
    };
    let r1 = vec![
        0xfe010113,
        0x00113c23,
        0x00813823,
        0x00913423,
        0x02013483,
        0x00088413,
        0x00053883,
        0x00000513,
        jal(8, 17),
        0x00000013,
        0x02850533,
        0x00950533,
        0x01813083,
        0x01013403,
        0x00813483,
        0x02010113,
        0x00008067,
    ];
    image(
        Architecture::Riscv64,
        4,
        &[("r1", r1), ("take8", vec![0x00008067])],
    )
}

fn decompile(bytes: &[u8], asserts: &[&str]) -> String {
    let path = common::scratch_file("split-struct-params", "o");
    std::fs::write(&path, bytes).unwrap();
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| common::repo_root().join("specs"));
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(kuna);
    command
        .arg("decompile-all")
        .arg(&path)
        .args(["--assert-strict", "--sleighpath"])
        .arg(specs);
    for directive in asserts {
        command.args(["--assert", directive]);
    }
    let output = command.output().unwrap();
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

const S3: [&str; 5] = [
    "typedef struct s3 { int x; int y; int z; };",
    "prototype take3 int take3(struct s3 s, int k)",
    "prototype use3 int use3(struct s3 *p)",
    "prototype d1 int d1(int *k, int a, struct s3 s)",
    "prototype d4 int d4(int a, int b, struct s3 s)",
];

fn assert_not_written(body: &str, fields: &[&str]) {
    for field in fields {
        let write = format!("s.{field} =");
        assert!(
            !body.contains(&write),
            "a reused register was written into s: {body}"
        );
    }
}

fn check_reuse(text: &str) {
    let body = function(text, "d1");
    assert!(body.contains("return take3(s,"), "{body}");
    assert_not_written(body, &["x", "y", "z"]);
}

fn check_copy(text: &str) {
    let body = function(text, "d4");
    for field in ["x", "y", "z"] {
        assert!(
            body.contains(&format!(" = s.{field};")),
            "the copy of s.{field} was lost: {body}"
        );
    }
    assert_not_written(body, &["x", "y", "z"]);
}

#[test]
fn arm_reused_register_is_not_written_into_the_split_struct() {
    check_reuse(&decompile(&arm(), &S3));
}

#[test]
fn arm_copy_of_the_split_struct_keeps_its_register_fields() {
    check_copy(&decompile(&arm(), &S3));
}

#[test]
fn mips_reused_register_is_not_written_into_the_split_struct() {
    check_reuse(&decompile(&mips(), &S3));
}

#[test]
fn mips_copy_of_the_split_struct_keeps_its_register_fields() {
    check_copy(&decompile(&mips(), &S3));
}

#[test]
fn riscv_reused_register_is_not_written_into_the_split_struct() {
    let text = decompile(
        &riscv(),
        &[
            "typedef struct l2 { long a; long b; };",
            "prototype take8 long take8(long a, long b, long c, long d, long e, long f, long g, long h)",
            "prototype r1 long r1(long *k, long b, long c, long d, long e, long f, long g, struct l2 s)",
        ],
    );
    let body = function(&text, "r1");
    assert!(body.contains(") * s.a + s.b;"), "{body}");
    assert_not_written(body, &["a", "b"]);
}
