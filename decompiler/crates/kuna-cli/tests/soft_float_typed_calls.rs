//! A declared function-pointer prototype with floating-point values is forced
//! at an indirect call only under the convention the image states: core
//! registers on a soft-float ARM image, and not at all on a single-float image
//! or one that states no convention, even when the function computes with
//! floating-point registers.
mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
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
    let functions: Vec<(&str, Vec<u8>)> = functions
        .iter()
        .map(|(name, words)| {
            let bytes = words
                .iter()
                .flat_map(|&word| match endian {
                    Endianness::Big => word.to_be_bytes(),
                    Endianness::Little => word.to_le_bytes(),
                })
                .collect();
            (*name, bytes)
        })
        .collect();
    image_bytes(arch, endian, 0, &functions, note)
}

fn image_bytes(
    arch: Architecture,
    endian: Endianness,
    e_flags: u32,
    functions: &[(&str, Vec<u8>)],
    note: Option<(&[u8], Vec<u8>)>,
) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, arch, endian);
    if e_flags != 0 {
        object.flags = FileFlags::Elf {
            os_abi: 0,
            abi_version: 0,
            e_flags,
        };
    }
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    let mut bytes = Vec::new();
    for (name, code) in functions {
        object.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value: bytes.len() as u64,
            size: code.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
        bytes.extend(code);
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

/// `float t4(struct sops *o, float a) { return o->f5(a, 1.0f, 2.0f, 3.0f, 4.0f); }`,
/// clang -O2 for 32-bit PowerPC with `-msoft-float`, which states no
/// convention: every argument travels in r3-r7.
fn powerpc_soft_unstated() -> Vec<u8> {
    let t4 = vec![
        0x7c0802a6, 0x90010004, 0x9421fff0, 0x80630008, 0x3ca04000, 0x3cc04040, 0x3ce04080,
        0x7c6903a6, 0x7c832378, 0x3c803f80, 0x4e800421, 0x80010014, 0x38210010, 0x7c0803a6,
        0x4e800020,
    ];
    image(Architecture::PowerPc, Endianness::Big, &[("t4", t4)], None)
}

/// `int r7(struct o3 *o, int k, double *res, float *out)
/// { *out = (float)k * 3.0f; *res = o->f(k, 2.5); return 0; }`, clang -O2 for
/// armv7 with `-mfloat-abi=softfp` and no `.ARM.attributes`: it computes in
/// VFP registers but passes `2.5` in r2:r3.
fn arm_softfp_unstated() -> Vec<u8> {
    let r7 = vec![
        0xe92d4070, 0xeef00b04, 0xe1a06002, 0xe5905000, 0xe1a00001, 0xec542b30, 0xee011a10,
        0xeeb00a08, 0xeeb81ac1, 0xee210a00, 0xed830a00, 0xe1a03004, 0xe12fff35, 0xec410b30,
        0xe3a00000, 0xedc60b00, 0xe8bd8070,
    ];
    image(Architecture::Arm, Endianness::Little, &[("r7", r7)], None)
}

/// `int r5(struct o2 *o, float x, float *out, double *res)
/// { *out = x * x; *res = o->f(4, 2.5); return 0; }`, clang -O2 for RISC-V with
/// the C extension: `lp64f`/`ilp32f` pass `2.5` in integer registers, `lp64d`
/// in fa0.
fn riscv(xlen: u32, e_flags: u32) -> Vec<u8> {
    let r5: Vec<u8> = match (xlen, e_flags & 6) {
        (64, 2) => vec![
            0x41, 0x11, 0x06, 0xe4, 0x22, 0xe0, 0x53, 0x70, 0xa5, 0x10, 0x27, 0xa0, 0x05, 0x00,
            0x14, 0x61, 0x32, 0x84, 0x37, 0x15, 0x00, 0x01, 0x93, 0x15, 0x65, 0x02, 0x11, 0x45,
            0x82, 0x96, 0x08, 0xe0, 0x01, 0x45, 0xa2, 0x60, 0x02, 0x64, 0x41, 0x01, 0x82, 0x80,
        ],
        (32, 2) => vec![
            0x41, 0x11, 0x06, 0xc6, 0x22, 0xc4, 0x53, 0x70, 0xa5, 0x10, 0x27, 0xa0, 0x05, 0x00,
            0x14, 0x41, 0x32, 0x84, 0x11, 0x45, 0x37, 0x06, 0x04, 0x40, 0x81, 0x45, 0x82, 0x96,
            0x4c, 0xc0, 0x08, 0xc0, 0x01, 0x45, 0xb2, 0x40, 0x22, 0x44, 0x41, 0x01, 0x82, 0x80,
        ],
        (64, 4) => vec![
            0x41, 0x11, 0x06, 0xe4, 0x22, 0xe0, 0x53, 0x70, 0xa5, 0x10, 0x27, 0xa0, 0x05, 0x00,
            0x0c, 0x61, 0x32, 0x84, 0x37, 0x05, 0x00, 0x00, 0x07, 0x35, 0x05, 0x00, 0x11, 0x45,
            0x82, 0x95, 0x08, 0xa0, 0x01, 0x45, 0xa2, 0x60, 0x02, 0x64, 0x41, 0x01, 0x82, 0x80,
        ],
        _ => unreachable!(),
    };
    let arch = if xlen == 64 {
        Architecture::Riscv64
    } else {
        Architecture::Riscv32
    };
    image_bytes(arch, Endianness::Little, e_flags, &[("r5", r5)], None)
}

fn decompile(bytes: &[u8], functions: &[&str]) -> String {
    let mut asserts = vec![
        "typedef struct dops { double (*f)(double a, int k); float (*g)(float a, int k); \
         int (*h)(int k, double d); };"
            .to_string(),
    ];
    asserts.extend(
        functions
            .iter()
            .map(|name| format!("prototype {name} int {name}(struct dops *o, int k)")),
    );
    decompile_with(bytes, &asserts)
}

fn decompile_with(bytes: &[u8], asserts: &[String]) -> String {
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
        .arg(specs);
    for assert in asserts {
        command.args(["--assert", assert]);
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
fn powerpc_that_states_no_convention_keeps_the_recovered_arguments() {
    let text = decompile_with(
        &powerpc_soft_unstated(),
        &[
            "typedef struct sops { int (*a)(int k); int (*b)(int k); \
             float (*f5)(float a, float b, float c, float d, float e); };"
                .to_string(),
            "prototype t4 float t4(struct sops *o, float a)".to_string(),
        ],
    );
    let t4 = function(&text, "t4");
    assert!(
        t4.contains("0x3f800000,0x40000000,0x40400000,0x40800000)"),
        "{t4}"
    );
    assert!(
        !t4.contains("// f2"),
        "read an FPR the caller never set: {t4}"
    );
    let text = decompile(&powerpc_hard(), &["t3"]);
    let t3 = function(&text, "t3");
    assert!(t3.contains("((double)v1,k) + 1"), "{t3}");
}

#[test]
fn arm_that_states_no_convention_keeps_the_recovered_arguments() {
    let text = decompile_with(
        &arm_softfp_unstated(),
        &[
            "typedef struct o3 { double (*f)(int a, double b); };".to_string(),
            "prototype r7 int r7(struct o3 *o, int k, double *res, float *out)".to_string(),
        ],
    );
    let r7 = function(&text, "r7");
    assert!(r7.contains(",0,0x40040000)"), "{r7}");
    assert!(
        !r7.contains("// d0"),
        "read a VFP register the caller never set: {r7}"
    );
}

fn riscv_r5(xlen: u32, e_flags: u32) -> String {
    let text = decompile_with(
        &riscv(xlen, e_flags),
        &[
            "typedef struct o2 { double (*f)(int a, double b); };".to_string(),
            "prototype r5 int r5(struct o2 *o, float x, float *out, double *res)".to_string(),
        ],
    );
    function(&text, "r5").to_string()
}

#[test]
fn single_float_riscv_keeps_the_arguments_the_caller_sets() {
    for xlen in [64, 32] {
        let r5 = riscv_r5(xlen, 3);
        assert!(r5.contains("(4,"), "{r5}");
        assert!(
            r5.contains("0x40040000"),
            "dropped the double the caller set: {r5}"
        );
        assert!(!r5.contains("// fa0"), "read fa0 as a double: {r5}");
        assert!(
            !r5.contains("// a0\n"),
            "printed the result as unset a0: {r5}"
        );
    }
}

#[test]
fn double_float_riscv_passes_declared_doubles_in_float_registers() {
    let r5 = riscv_r5(64, 5);
    assert!(r5.contains(")(4,"), "{r5}");
    assert!(!r5.contains(",4,"), "{r5}");
}
