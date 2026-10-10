//! A struct parameter whose pieces span registers and the stack stays the
//! parameter: a later write of one of its registers is not a write into it,
//! and copying its register pieces to memory is not lost. On AArch64, where
//! only Windows variadic functions split one, a struct that no longer fits in
//! the registers left goes wholly on the stack.
use crate::common;
use object::write::{Object, StandardSection, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SymbolFlags, SymbolKind, SymbolScope,
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
    text_image(object, functions)
}

fn text_image(mut object: Object, functions: &[(&str, Vec<u32>)]) -> Vec<u8> {
    let text = object.section_id(StandardSection::Text);
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

/// clang -O2 for aarch64-linux-gnu, with `struct pair { long a, b; }` and
/// `struct hfa { double x, y; }`: `last` and `after` take `pair s` after seven
/// longs, so `s` is the first two stack doublewords and `z` the third; `callit`
/// passes one; `six` has `s` in x6 and x7; `hfa7` has `h` in d0 and d1 and `z`
/// in x7.
fn aarch64_functions() -> Vec<(&'static str, Vec<u32>)> {
    let bl = |from: u32, to: u32| 0x94000000 | ((to as i32 - from as i32) as u32 & 0x3ffffff);
    let ret = 0xd65f03c0;
    vec![
        ("last", vec![0xa94027e8, 0x8b080508, 0x8b090100, ret]),
        (
            "after",
            vec![
                0xa94023e9, 0xf9400bea, 0x8b090529, 0x8b080128, 0x8b0a0949, 0x8b090100, ret,
            ],
        ),
        ("six", vec![0x8b0604c8, 0x8b070100, ret]),
        (
            "hfa7",
            vec![0x1e601002, 0x9e6200e3, 0x1f420400, 0x1e632800, ret],
        ),
        (
            "callit",
            vec![
                0xd10083ff,
                0xa9017bfd,
                0x910043fd,
                0xaa0103e8,
                0xaa0003e9,
                0x52800020,
                0x52800041,
                0x52800062,
                0x52800083,
                0x528000a4,
                0x528000c5,
                0x528000e6,
                0xa90023e9,
                bl(32, 0),
                0xa9417bfd,
                0x91000400,
                0x910083ff,
                ret,
            ],
        ),
    ]
}

/// A Windows ARM64 object with the same `last` and a variadic `vlast` laid out
/// the way that ABI passes it: `s.a` in x7 and `s.b` on the stack.
fn aarch64_windows() -> Vec<u8> {
    let object = Object::new(
        BinaryFormat::Coff,
        Architecture::Aarch64,
        Endianness::Little,
    );
    let functions = aarch64_functions();
    text_image(
        object,
        &[
            ("last", functions[0].1.clone()),
            (
                "vlast",
                vec![0xf94003e8, 0x8b0704e9, 0x8b080120, 0xd65f03c0],
            ),
        ],
    )
}

/// An arm64e Mach-O object with clang's `last` for arm64e-apple-macos.
fn aarch64_apple() -> Vec<u8> {
    let mut object = Object::new(
        BinaryFormat::MachO,
        Architecture::Aarch64,
        Endianness::Little,
    );
    object.set_macho_cpu_subtype(object::macho::CPU_SUBTYPE_ARM64E);
    text_image(
        object,
        &[("last", vec![0xa94023e9, 0x8b090529, 0x8b080120, 0xd65f03c0])],
    )
}

fn decompile(bytes: &[u8], asserts: &[&str]) -> String {
    decompile_with(bytes, &[], asserts)
}

fn decompile_with(bytes: &[u8], args: &[&str], asserts: &[&str]) -> String {
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
        .arg(specs)
        .args(args);
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

const SEVEN: &str =
    "long long a1, long long a2, long long a3, long long a4, long long a5, long long a6, long long a7";

fn pair_asserts(prefix: &str) -> Vec<String> {
    vec![
        "typedef struct pair { long long a; long long b; };".to_string(),
        format!("prototype {prefix}last long long last({SEVEN}, struct pair s)"),
    ]
}

#[test]
fn aarch64_struct_without_two_registers_left_is_wholly_on_the_stack() {
    let functions = aarch64_functions();
    let mut asserts = pair_asserts("");
    asserts.extend([
        "typedef struct hfa { double x; double y; };".to_string(),
        format!("prototype after long after({SEVEN}, struct pair s, long z)"),
        "prototype six long six(long a1, long a2, long a3, long a4, long a5, long a6, struct pair s)"
            .to_string(),
        format!("prototype hfa7 double hfa7({SEVEN}, struct hfa h, long z)"),
        "prototype callit long callit(long x, long y)".to_string(),
    ]);
    let asserts: Vec<&str> = asserts.iter().map(String::as_str).collect();
    let text = decompile(&image(Architecture::Aarch64, 0, &functions), &asserts);
    let last = function(&text, "last");
    assert!(last.contains("return s.a * 3 + s.b;"), "{last}");
    let after = function(&text, "after");
    assert!(after.contains("return s.a * 3 + s.b + z * 5;"), "{after}");
    let six = function(&text, "six");
    assert!(six.contains("return s.a * 3 + s.b;"), "{six}");
    let hfa7 = function(&text, "hfa7");
    assert!(
        hfa7.contains("return h.y + h.x * 2.0 + (double)z;"),
        "{hfa7}"
    );
    let callit = function(&text, "callit");
    for line in ["s.a = x;", "s.b = y;", "return last(1,2,3,4,5,6,7,s) + 1;"] {
        assert!(callit.contains(line), "{callit}");
    }
}

#[test]
fn aarch64_windows_splits_the_struct_only_in_a_variadic_function() {
    let mut asserts = pair_asserts("");
    asserts.push(format!(
        "prototype vlast long long vlast({SEVEN}, struct pair s, ...)"
    ));
    let asserts: Vec<&str> = asserts.iter().map(String::as_str).collect();
    let text = decompile(&aarch64_windows(), &asserts);
    for name in ["last", "vlast"] {
        let body = function(&text, name);
        assert!(body.contains("return s.a * 3 + s.b;"), "{body}");
    }
}

#[test]
fn aarch64_apple_struct_without_two_registers_left_is_wholly_on_the_stack() {
    let asserts = pair_asserts("_");
    let asserts: Vec<&str> = asserts.iter().map(String::as_str).collect();
    let text = decompile_with(
        &aarch64_apple(),
        &["--option", "macho-arm64e", "on"],
        &asserts,
    );
    let body = function(&text, "_last");
    assert!(body.contains("return s.a * 3 + s.b;"), "{body}");
}
