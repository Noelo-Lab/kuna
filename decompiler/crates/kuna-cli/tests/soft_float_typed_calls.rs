//! A declared function-pointer prototype with floating-point values is forced
//! at an indirect call only under the convention the image uses: core
//! registers on a soft-float ARM image, and on an image that states no
//! convention, only when the function computes with floating-point registers.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::path::PathBuf;
use std::process::Command;

fn attributes(vendor: &[u8], tags: &[u8], endian: Endianness) -> Vec<u8> {
    let put = |v: usize| match endian {
        Endianness::Big => (v as u32).to_be_bytes(),
        Endianness::Little => (v as u32).to_le_bytes(),
    };
    let mut data = vec![b'A'];
    data.extend(put(4 + vendor.len() + 1 + 5 + tags.len()));
    data.extend(vendor);
    data.extend([0, 1]);
    data.extend(put(5 + tags.len()));
    data.extend(tags);
    data
}

fn image(
    arch: Architecture,
    endian: Endianness,
    functions: &[(&str, Vec<u32>)],
    note: Option<(&[u8], Vec<u8>)>,
) -> Vec<u8> {
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
    if let Some((name, data)) = note {
        let id = object.add_section(Vec::new(), name.to_vec(), SectionKind::Other);
        object.append_section_data(id, &data, 1);
    }
    object.write().unwrap()
}

/// `int t1(struct dops *o, int k) { return (int)o->f(2.5, k + 1); }` and
/// `int t2(struct dops *o, int k) { return (int)o->g(1.5f, k * 3); }`,
/// clang -O2 for armv7 with `-mfloat-abi=soft` and `.ARM.attributes`
/// that names the CPU but not `Tag_ABI_VFP_args`.
fn arm_soft() -> Vec<u8> {
    let bl = |from: u32, to: u32| 0xeb000000 | ((to as i32 - from as i32 - 2) as u32 & 0xffffff);
    let t1 = vec![
        0xe92d4800,
        0xe5903000,
        0xe2812001,
        0xe3001000,
        0xe3a00000,
        0xe3441004,
        0xe12fff33,
        bl(7, 16),
        0xe8bd8800,
    ];
    let t2 = vec![
        0xe92d4800,
        0xe5902004,
        0xe0811081,
        0xe3a005ff,
        0xe12fff32,
        bl(14, 17),
        0xe8bd8800,
    ];
    image(
        Architecture::Arm,
        Endianness::Little,
        &[
            ("t1", t1),
            ("t2", t2),
            ("__aeabi_d2iz", vec![0xe12fff1e]),
            ("__aeabi_f2iz", vec![0xe12fff1e]),
        ],
        Some((
            b".ARM.attributes",
            attributes(b"aeabi", &[6, 10], Endianness::Little),
        )),
    )
}

/// The same two functions for armv7 hard-float (`Tag_ABI_VFP_args=1`).
fn arm_hard() -> Vec<u8> {
    let t1 = vec![
        0xe92d4800, 0xeeb00b04, 0xe5902000, 0xe2810001, 0xe12fff32, 0xeebd0bc0, 0xee100a10,
        0xe8bd8800,
    ];
    let t2 = vec![
        0xe92d4800, 0xeeb70a08, 0xe5902004, 0xe0810081, 0xe12fff32, 0xeebd0ac0, 0xee100a10,
        0xe8bd8800,
    ];
    image(
        Architecture::Arm,
        Endianness::Little,
        &[("t1", t1), ("t2", t2)],
        Some((
            b".ARM.attributes",
            attributes(b"aeabi", &[6, 10, 28, 1], Endianness::Little),
        )),
    )
}

/// `t1` and `t2` for 32-bit PowerPC with `-msoft-float`, optionally carrying
/// `.gnu.attributes` that state the soft-float convention.
fn powerpc_soft(stated: bool) -> Vec<u8> {
    let bl =
        |from: u32, to: u32| 0x48000001 | (((to as i32 - from as i32) * 4) as u32 & 0x03fffffc);
    let t1 = vec![
        0x7c0802a6,
        0x90010004,
        0x9421fff0,
        0x80630000,
        0x38a40001,
        0x38800000,
        0x7c6903a6,
        0x3c604004,
        0x4e800421,
        bl(9, 27),
        0x80010014,
        0x38210010,
        0x7c0803a6,
        0x4e800020,
    ];
    let t2 = vec![
        0x7c0802a6,
        0x90010004,
        0x9421fff0,
        0x80630004,
        0x1c840003,
        0x7c6903a6,
        0x3c603fc0,
        0x4e800421,
        bl(22, 28),
        0x80010014,
        0x38210010,
        0x7c0803a6,
        0x4e800020,
    ];
    let note = stated.then(|| {
        (
            &b".gnu.attributes"[..],
            attributes(b"gnu", &[4, 2], Endianness::Big),
        )
    });
    image(
        Architecture::PowerPc,
        Endianness::Big,
        &[
            ("t1", t1),
            ("t2", t2),
            ("__fixdfsi", vec![0x4e800020]),
            ("__fixsfsi", vec![0x4e800020]),
        ],
        note,
    )
}

/// `int t3(struct dops *o, int k) { return o->h(k, 0.25) + 1; }`, clang -O2
/// for hard-float 32-bit PowerPC, which states no convention: the constant is
/// loaded into f1.
fn powerpc_hard() -> Vec<u8> {
    let t3 = vec![
        0x7c0802a6, 0x90010004, 0x9421fff0, 0x93c10008, 0x48000005, 0x7fc802a6, 0x80beffe8,
        0x7fc5f214, 0x80630008, 0x80be8008, 0x7c6903a6, 0xc0250000, 0x7c832378, 0x4e800421,
        0x38630001, 0x80010014, 0x83c10008, 0x38210010, 0x7c0803a6, 0x4e800020,
    ];
    image(Architecture::PowerPc, Endianness::Big, &[("t3", t3)], None)
}

fn decompile(bytes: &[u8], functions: &[&str]) -> String {
    let path = common::scratch_file("soft-float-typed-calls", "o");
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
        .args([
            "--assert",
            "typedef struct dops { double (*f)(double a, int k); float (*g)(float a, int k); \
             int (*h)(int k, double d); };",
        ]);
    for name in functions {
        command.args([
            "--assert",
            &format!("prototype {name} int {name}(struct dops *o, int k)"),
        ]);
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

#[test]
fn soft_float_arm_passes_declared_floats_in_core_registers() {
    let text = decompile(&arm_soft(), &["t1", "t2"]);
    let t1 = function(&text, "t1");
    assert!(t1.contains("(*o->f)(2.5,k + 1)"), "{t1}");
    assert!(
        !t1.contains("// d0"),
        "read a VFP register the caller never set: {t1}"
    );
    let t2 = function(&text, "t2");
    assert!(t2.contains("(*o->g)(1.5,k * 3)"), "{t2}");
    assert!(
        !t2.contains("// s0"),
        "read a VFP register the caller never set: {t2}"
    );
}

#[test]
fn hard_float_arm_passes_declared_floats_in_vfp_registers() {
    let text = decompile(&arm_hard(), &["t1", "t2"]);
    assert!(function(&text, "t1").contains(")(2.5,k + 1)"), "{text}");
    assert!(function(&text, "t2").contains(")(1.5,k * 3)"), "{text}");
}

#[test]
fn soft_float_powerpc_keeps_the_arguments_the_caller_sets() {
    for stated in [false, true] {
        let text = decompile(&powerpc_soft(stated), &["t1", "t2"]);
        let t1 = function(&text, "t1");
        assert!(t1.contains("(*o->f)(0x40040000,0,k + 1)"), "{t1}");
        let t2 = function(&text, "t2");
        assert!(t2.contains("(*o->g)(0x3fc00000,k * 3)"), "{t2}");
        assert!(
            !text.contains("// f1"),
            "read an FPR the caller never set: {text}"
        );
    }
}

#[test]
fn powerpc_float_code_keeps_the_declared_prototype() {
    let text = decompile(&powerpc_hard(), &["t3"]);
    let t3 = function(&text, "t3");
    assert!(t3.contains(")(k,"), "{t3}");
    assert!(!t3.contains(")(k,k"), "{t3}");
}
