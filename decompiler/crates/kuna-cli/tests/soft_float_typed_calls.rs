//! A declared function-pointer prototype with floating-point values is forced
//! at an indirect call only under the convention the image states: core
//! registers on a soft-float ARM image, and not at all on a single-float image
//! or one that states no convention, even when the function computes with
//! floating-point registers.  A variadic prototype with a floating-point or
//! register-pair fixed value, and a return value narrower than a register the
//! model does not extend by type, keep the recovered call too, unless the caller
//! extends it, which an Apple arm64 caller does not below 32 bits.  A declared
//! or DWARF-described prototype of a function or of a direct callee follows the
//! same soft-float convention on ARM.
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
    object_bytes(object, text, functions, note)
}

fn object_bytes(
    mut object: Object,
    text: object::write::SectionId,
    functions: &[(&str, Vec<u8>)],
    note: Option<(&[u8], Vec<u8>)>,
) -> Vec<u8> {
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
    decompile_args(bytes, &[], asserts)
}

fn decompile_args(bytes: &[u8], args: &[&str], asserts: &[String]) -> String {
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
        .args(args)
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

/// `int w2(struct wo *o, int k) { return (int)o->vr(k, k + 1, 0.5); }` and
/// `int j1(struct jo *o, int k) { return o->vj(5LL, k, 1.5) + 1; }`, clang -O2
/// for armv7 hard-float: a variadic call passes every value in core registers.
fn arm_hard_variadic() -> Vec<u8> {
    let w2 = vec![
        0xe92d4800, 0xeeb60b00, 0xe590e004, 0xe1a0c001, 0xe2811001, 0xe1a0000c, 0xec532b10,
        0xe12fff3e, 0xec410b10, 0xeebd0bc0, 0xee100a10, 0xe8bd8800,
    ];
    let j1 = vec![
        0xe92d4800, 0xe24dd008, 0xe1a02001, 0xe3001000, 0xe5903000, 0xe3431ff8, 0xe3a00000,
        0xe1cd00f0, 0xe3a00005, 0xe3a01000, 0xe12fff33, 0xe2800001, 0xe28dd008, 0xe8bd8800,
    ];
    image(
        Architecture::Arm,
        Endianness::Little,
        &[("w2", w2), ("j1", j1)],
        Some((
            b".ARM.attributes",
            attributes(b"aeabi", &[6, 10, 28, 1], Endianness::Little),
        )),
    )
}

#[test]
fn variadic_prototype_with_float_or_pair_storage_keeps_the_recovered_call() {
    let text = decompile_with(
        &arm_hard_variadic(),
        &[
            "typedef struct wo { int (*vi)(int n, ...); double (*vr)(int n, ...); };".to_string(),
            "typedef struct jo { int (*vj)(long long a, ...); };".to_string(),
            "prototype w2 int w2(struct wo *o, int k)".to_string(),
            "prototype j1 int j1(struct jo *o, int k)".to_string(),
        ],
    );
    let w2 = function(&text, "w2");
    assert!(w2.contains("(double)(*v1)(k,k + 1,0,0x3fe00000)"), "{w2}");
    assert!(!w2.contains("CONCAT44"), "read the result from unset r0/r1: {w2}");
    let j1 = function(&text, "j1");
    assert!(j1.contains("(*v1)(5,0,k,"), "{j1}");
    assert!(j1.contains("0x3ff80000)"), "dropped the stacked 1.5: {j1}");
}

/// `long r1(struct ro *o, int k) { return o->fi(k); }` and
/// `long r6(struct ro *o, int k, long *p) { return p[o->fi(k)]; }`, clang -O2
/// for RISC-V lp64d: the callee sign-extends the `int` it returns in a0.
fn riscv_int_return() -> Vec<u8> {
    let r1 = vec![
        0x41, 0x11, 0x06, 0xe4, 0x10, 0x61, 0x2e, 0x85, 0x02, 0x96, 0xa2, 0x60, 0x41, 0x01,
        0x82, 0x80,
    ];
    let r6 = vec![
        0x41, 0x11, 0x06, 0xe4, 0x22, 0xe0, 0x14, 0x61, 0x32, 0x84, 0x2e, 0x85, 0x82, 0x96,
        0x0e, 0x05, 0x22, 0x95, 0x08, 0x61, 0xa2, 0x60, 0x02, 0x64, 0x41, 0x01, 0x82, 0x80,
    ];
    image_bytes(
        Architecture::Riscv64,
        Endianness::Little,
        5,
        &[("r1", r1), ("r6", r6)],
        None,
    )
}

/// `int c3(struct co *o, int k) { return o->sc(k) * 3; }`, clang -O2 for
/// little-endian MIPS o32: the callee sign-extends the `signed char` in v0.
fn mips_char_return() -> Vec<u8> {
    let c3: Vec<u8> = [
        0x27bdffe8u32,
        0xafbf0014,
        0x8c990008,
        0x0320f809,
        0x00a02025,
        0x00020840,
        0x00221021,
        0x8fbf0014,
        0x03e00008,
        0x27bd0018,
    ]
    .iter()
    .flat_map(|word| word.to_le_bytes())
    .collect();
    image_bytes(
        Architecture::Mips,
        Endianness::Little,
        0x70001007,
        &[("c3", c3)],
        None,
    )
}

#[test]
fn return_narrower_than_an_unextended_register_keeps_the_recovered_call() {
    let text = decompile_with(
        &riscv_int_return(),
        &[
            "typedef struct ro { int (*fi)(int k); };".to_string(),
            "prototype r1 long r1(struct ro *o, int k)".to_string(),
            "prototype r6 long r6(struct ro *o, int k, long *p)".to_string(),
        ],
    );
    let r1 = function(&text, "r1");
    assert!(!r1.contains("(unsigned int)"), "zero-extended the signed int: {r1}");
    let r6 = function(&text, "r6");
    assert!(r6.contains("p[(*v1)(k"), "{r6}");
    assert!(!r6.contains("(unsigned int)"), "zero-extended the signed index: {r6}");
    let text = decompile_with(
        &mips_char_return(),
        &[
            "typedef struct co { char (*nc)(char c); short (*ns)(short s); \
             signed char (*sc)(int k); };"
                .to_string(),
            "prototype c3 int c3(struct co *o, int k)".to_string(),
        ],
    );
    let c3 = function(&text, "c3");
    assert!(c3.contains("(*v1)(k) * 3"), "{c3}");
    assert!(!c3.contains("// v0"), "read unset bits of v0: {c3}");
}

fn apple_arm64_c1() -> [u32; 8] {
    [
        0xa9bf7bfd, 0x910003fd, 0xf9400008, 0xaa0103e0, 0xd63f0100, 0x0b000400, 0xa8c17bfd,
        0xd65f03c0,
    ]
}

/// `int c1(struct co *o, int k) { return o->sc(k) * 3; }`,
/// `long c6(struct co *o, int k, long *p) { return p[o->sc(k)]; }` and
/// `long e1(struct io *o, int k, long *p) { return p[o->si(k, 2.5)]; }`, clang
/// -O2 for arm64-apple-macos: the callee sign-extends the `signed char` it
/// returns to 32 bits, and the caller reads `w0` without extending it.
fn apple_arm64() -> Vec<u8> {
    let words = |words: &[u32]| -> Vec<u8> { words.iter().flat_map(|w| w.to_le_bytes()).collect() };
    let c1 = words(&apple_arm64_c1());
    let c6 = words(&[
        0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0203f3, 0xf9400008, 0xaa0103e0, 0xd63f0100,
        0xf860da60, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
    ]);
    let e1 = words(&[
        0xa9be4ff4, 0xa9017bfd, 0x910043fd, 0xaa0203f3, 0xf9400008, 0x1e609000, 0xaa0103e0,
        0xd63f0100, 0xf860da60, 0xa9417bfd, 0xa8c24ff4, 0xd65f03c0,
    ]);
    let mut object = Object::new(BinaryFormat::MachO, Architecture::Aarch64, Endianness::Little);
    let text = object.section_id(object::write::StandardSection::Text);
    object_bytes(object, text, &[("c1", c1), ("c6", c6), ("e1", e1)], None)
}

#[test]
fn apple_arm64_return_narrower_than_32_bits_keeps_the_recovered_call() {
    let text = decompile_with(
        &apple_arm64(),
        &[
            "typedef struct co { signed char (*sc)(int k); };".to_string(),
            "typedef struct io { int (*si)(int k, double d); };".to_string(),
            "prototype _c1 int _c1(struct co *o, int k)".to_string(),
            "prototype _c6 long _c6(struct co *o, int k, long *p)".to_string(),
            "prototype _e1 long _e1(struct io *o, int k, long *p)".to_string(),
        ],
    );
    let c1 = function(&text, "_c1");
    assert!(c1.contains("(*v1)(k) * 3"), "{c1}");
    assert!(!c1.contains("(unsigned char)"), "zero-extended the signed char: {c1}");
    let c6 = function(&text, "_c6");
    assert!(c6.contains("p[(int)(*v1)(k)]"), "{c6}");
    assert!(!c6.contains("(unsigned char)"), "zero-extended the signed index: {c6}");
    let e1 = function(&text, "_e1");
    assert!(e1.contains("p[(*v1)(k,2.5)]"), "an int return keeps the declared call: {e1}");
    let c1: Vec<u8> = apple_arm64_c1().iter().flat_map(|w| w.to_le_bytes()).collect();
    let text = decompile_args(
        &c1,
        &[
            "--raw-image",
            "--target",
            "AARCH64:LE:64:v8A:default",
            "--base",
            "0",
            "--entry",
            "0",
            "--define-function",
            "0=c1",
        ],
        &[
            "typedef struct co { signed char (*sc)(int k); };".to_string(),
            "prototype c1 int c1(struct co *o, int k)".to_string(),
        ],
    );
    let c1 = function(&text, "c1");
    assert!(c1.contains("(*v1)(k) * 3"), "a raw image states no platform: {c1}");
    assert!(!c1.contains("(unsigned char)"), "zero-extended the signed char: {c1}");
}

/// `double s4(double a, int k) { return dmix(a, k * 2) * a; }` and
/// `void s9(double a) { vsink(a, 1.5, a); }` calling `dmix` and `vsink`
/// directly: clang -O2 for armv7 with `-mfloat-abi=soft` (`.ARM.attributes`
/// without `Tag_ABI_VFP_args`) or, with `vfp`, `-mfloat-abi=hard`.
fn arm_direct(vfp: bool) -> Vec<u8> {
    let bl = |from: u32, to: u32| 0xeb000000 | ((to as i32 - from as i32 - 2) as u32 & 0xffffff);
    let ret = vec![0xe12fff1e];
    let (functions, tags) = if vfp {
        let s4 = vec![
            0xe92d4800,
            0xed2d8b02,
            0xe1a00080,
            0xeeb08b40,
            bl(4, 11),
            0xee200b08,
            0xecbd8b02,
            0xe8bd8800,
        ];
        let s9 = vec![0xeeb71b08, 0xeeb02b40, 0xea000000];
        (vec![("s4", s4), ("s9", s9), ("dmix", ret.clone()), ("vsink", ret)], vec![6, 10, 28, 1])
    } else {
        let s4 = vec![
            0xe92d4830,
            0xe1a02082,
            0xe1a04001,
            0xe1a05000,
            bl(4, 18),
            0xe1a02005,
            0xe1a03004,
            bl(7, 20),
            0xe8bd8830,
        ];
        let s9 = vec![
            0xe92d4800,
            0xe24dd008,
            0xe3003000,
            0xe3a02000,
            0xe3433ff8,
            0xe88d0003,
            bl(15, 19),
            0xe28dd008,
            0xe8bd8800,
        ];
        let functions = vec![
            ("s4", s4),
            ("s9", s9),
            ("dmix", ret.clone()),
            ("vsink", ret.clone()),
            ("__aeabi_dmul", ret),
        ];
        (functions, vec![6, 10])
    };
    image(
        Architecture::Arm,
        Endianness::Little,
        &functions,
        Some((
            b".ARM.attributes",
            attributes(b"aeabi", &tags, Endianness::Little),
        )),
    )
}

fn direct_prototypes() -> Vec<String> {
    [
        "prototype dmix double dmix(double a, int k)",
        "prototype vsink void vsink(double a, double b, double c)",
        "prototype s4 double s4(double a, int k)",
        "prototype s9 void s9(double a)",
    ]
    .map(String::from)
    .to_vec()
}

#[test]
fn soft_float_arm_lays_declared_prototypes_out_in_core_registers() {
    let text = decompile_with(&arm_direct(false), &direct_prototypes());
    let s4 = function(&text, "s4");
    assert!(s4.contains("v1 = dmix(a,k << 1);"), "{s4}");
    assert!(s4.contains("__aeabi_dmul(SUB84(v1,0),"), "{s4}");
    let s9 = function(&text, "s9");
    assert!(s9.contains("vsink(a,1.5,a);"), "{s9}");
    for register in ["// d0", "// d1", "// d2", "// s0", "// s1"] {
        assert!(
            !text.contains(register),
            "read a VFP register the caller never set: {text}"
        );
    }
}

#[test]
fn hard_float_arm_lays_declared_prototypes_out_in_vfp_registers() {
    let text = decompile_with(&arm_direct(true), &direct_prototypes());
    assert!(function(&text, "s4").contains("return dmix(a,k << 1) * a;"), "{text}");
    assert!(function(&text, "s9").contains("vsink(a,1.5,a);"), "{text}");
}

#[test]
fn soft_float_arm_lays_dwarf_prototypes_out_in_core_registers() {
    let bytes = std::fs::read(common::fixture("softfloat_dwarf_armel.o")).unwrap();
    let text = decompile_with(&bytes, &[]);
    let dmix = function(&text, "dmix");
    assert!(dmix.contains("__aeabi_i2d(k)"), "{dmix}");
    assert!(dmix.contains(" = a;"), "{dmix}");
    let s4 = function(&text, "s4");
    assert!(s4.contains("v1 = dmix(a,k << 1);"), "{s4}");
    assert!(!text.contains("// r1"), "read a core register as unset: {text}");
}
