//! A declared variadic callee that returns a `double` takes a variadic `double`
//! in the register it returns in: `xmm0` on x86-64, `d0` on AArch64 and `f1` on
//! 32-bit PowerPC. The x86-64 build is compiled back from the printed C and run
//! against the source. A vector count of zero in `al` (`crclr 6` on PowerPC)
//! keeps a value left in the register out of the call, and a count of one
//! (`crset 6`) keeps another call's result in it. A constant moved into `d0`
//! prints as a `double`.
use crate::common;
use common::process;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

const SOURCE: &str = r#"
int s_w1(int k) { return (int)vr(k, k + 1, (double)k); }
double s_w2(int k, double *p) { return vr(k, *p + *p); }
int s_w3(int k, double *p) { return (int)vr(k, p[0], p[1], k); }
double s_w4(int k) { return vr(k, g(k)); }
double s_w5(int k, double *p) { return vr(k, vr(k, *p)); }
double s_w6(int k) { double a = g(k); return vr(k, a) + a; }
double s_w7(int k) { double a = g(k); return vr(k, k + 1) + a; }
"#;

const PROTOTYPES: [&str; 12] = [
    "prototype vr double vr(int n, ...)",
    "prototype g double g(int k)",
    "prototype w1 int w1(int k)",
    "prototype w2 double w2(int k, double *p)",
    "prototype w3 int w3(int k, double *p)",
    "prototype w4 double w4(int k)",
    "prototype w5 double w5(int k, double *p)",
    "prototype w6 double w6(int k)",
    "prototype w7 double w7(int k)",
    "prototype w8 int w8(int k)",
    "prototype w0 int w0(int k)",
    "prototype w0x int w0x(int k)",
];

/// The callers, and the relocations of their calls to an undefined `vr` or `g`
/// (none when the image defines them itself).
struct Image {
    arch: Architecture,
    endian: Endianness,
    text: Vec<u8>,
    functions: &'static [(&'static str, u64, u64)],
    calls: &'static [(u64, &'static str, u32, i64)],
}

fn hex(text: &str) -> Vec<u8> {
    let hex: String = text.split_whitespace().collect();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// clang -O2 of `w1` to `w3` of [`SOURCE`] for x86-64, then `w0` and `w0x`:
/// `w1` with `al` set to zero by `mov $0,%al` and by `xor %eax,%eax`, so the
/// `(double)k` left in `xmm0` is not an argument. Then clang -O2 of `w4` to `w7`.
fn x86_64() -> Image {
    const PLT: u32 = object::elf::R_X86_64_PLT32;
    Image {
        arch: Architecture::X86_64,
        endian: Endianness::Little,
        text: hex(
            "508d7701f20f2ac7b001e800000000f20f2cc059c3662e0f1f84000000000090\
                   f20f1006f20f58c0b001e90000000090\
                   50f20f1006f20f104e0889feb002e800000000f20f2cc059c3cccccccccccccc\
                   508d7701f20f2ac7b000e800000000f20f2cc059c3cccccccccccccccccccccc\
                   508d7701f20f2ac731c0e800000000f20f2cc059c3cccccccccccccccccccccc\
                   5389fbe80000000089dfb0015be900000000662e0f1f8400000000000f1f4000\
                   5389fbf20f1006b001e80000000089dfb0015be9000000000f1f840000000000\
                   534883ec1089fbe800000000f20f1144240889dfb001e800000000f20f584424\
                   084883c4105bc3660f1f840000000000\
                   534883ec1089fbe800000000f20f114424088d730189df31c0e800000000f20f\
                   584424084883c4105bc3",
        ),
        functions: &[
            ("w1", 0x00, 0x15),
            ("w2", 0x20, 0x0f),
            ("w3", 0x30, 0x19),
            ("w0", 0x50, 0x15),
            ("w0x", 0x70, 0x15),
            ("w4", 0x90, 0x12),
            ("w5", 0xb0, 0x18),
            ("w6", 0xd0, 0x27),
            ("w7", 0x100, 0x2a),
        ],
        calls: &[
            (0x0b, "vr", PLT, -4),
            (0x2b, "vr", PLT, -4),
            (0x3f, "vr", PLT, -4),
            (0x5b, "vr", PLT, -4),
            (0x7b, "vr", PLT, -4),
            (0x94, "g", PLT, -4),
            (0x9e, "vr", PLT, -4),
            (0xba, "vr", PLT, -4),
            (0xc4, "vr", PLT, -4),
            (0xd8, "g", PLT, -4),
            (0xe7, "vr", PLT, -4),
            (0x108, "g", PLT, -4),
            (0x11a, "vr", PLT, -4),
        ],
    }
}

/// clang -O2 of `w1` to `w3` of [`SOURCE`] for AArch64, then of
/// `int w8(int k) { return (int)vr(k, k + 1, 0.5); }`, which moves the `0.5`
/// into `d0` with `fmov`.
fn aarch64() -> Image {
    Image {
        arch: Architecture::Aarch64,
        endian: Endianness::Little,
        text: hex(
            "fd7bbfa90000621e01040011fd030091000000940000781efd7bc1a8c0035fd6\
                   200040fd0028601e00000014\
                   fd7bbfa92004406de103002afd030091000000940000781efd7bc1a8c0035fd6\
                   fd7bbfa9fd0300910104001100106c1e000000940000781efd7bc1a8c0035fd6",
        ),
        functions: &[
            ("w1", 0x00, 0x20),
            ("w2", 0x20, 0x0c),
            ("w3", 0x2c, 0x20),
            ("w8", 0x4c, 0x20),
        ],
        calls: &[
            (0x10, "vr", object::elf::R_AARCH64_CALL26, 0),
            (0x28, "vr", object::elf::R_AARCH64_JUMP26, 0),
            (0x3c, "vr", object::elf::R_AARCH64_CALL26, 0),
            (0x5c, "vr", object::elf::R_AARCH64_CALL26, 0),
        ],
    }
}

/// clang -O2 of `w2`, `w3`, `w4`, `w6` and `w7` for 32-bit big-endian PowerPC,
/// each `bl` aimed at a `vr` or `g` that returns at once. `w4` and `w6` set CR
/// bit 6 (`crset 6`), `w7` clears it.
fn powerpc() -> Image {
    let bl = |from: u32, to: u32| 0x48000001 | ((to.wrapping_sub(from)) & 0x03fffffc);
    let words: Vec<u32> = [
        vec![0x4e800020],
        vec![
            0x7c0802a6,
            0x90010004,
            0x9421fff0,
            0xc8040000,
            0x4cc63242,
            0xfc20002a,
            bl(0x1c, 0),
            0x80010014,
            0x38210010,
            0x7c0803a6,
            0x4e800020,
        ],
        vec![
            0x7c0802a6,
            0x90010004,
            0x9421fff0,
            0xc8240000,
            0x4cc63242,
            0xc8440008,
            0x7c641b78,
            bl(0x4c, 0),
            0xfc00081e,
            0xd8010008,
            0x8061000c,
            0x80010014,
            0x38210010,
            0x7c0803a6,
            0x4e800020,
        ],
        vec![0x4e800020],
        vec![
            0x7c0802a6,
            0x90010004,
            0x9421ffe0,
            0x93a10014,
            0x7c7d1b78,
            bl(0x84, 0x6c),
            0x4cc63242,
            0x7fa3eb78,
            bl(0x90, 0),
            0x83a10014,
            0x80010024,
            0x38210020,
            0x7c0803a6,
            0x4e800020,
        ],
        vec![
            0x7c0802a6,
            0x90010004,
            0x9421ffe0,
            0x93a1000c,
            0x7c7d1b78,
            0xdbe10018,
            bl(0xc0, 0x6c),
            0x4cc63242,
            0x7fa3eb78,
            0xffe00890,
            bl(0xd0, 0),
            0xfc3f082a,
            0xcbe10018,
            0x83a1000c,
            0x80010024,
            0x38210020,
            0x7c0803a6,
            0x4e800020,
        ],
        vec![
            0x7c0802a6,
            0x90010004,
            0x9421ffe0,
            0x93a1000c,
            0x7c7d1b78,
            0xdbe10018,
            bl(0x108, 0x6c),
            0x389d0001,
            0x4cc63182,
            0x7fa3eb78,
            0xffe00890,
            bl(0x11c, 0),
            0xfc3f082a,
            0xcbe10018,
            0x83a1000c,
            0x80010024,
            0x38210020,
            0x7c0803a6,
            0x4e800020,
        ],
    ]
    .concat();
    Image {
        arch: Architecture::PowerPc,
        endian: Endianness::Big,
        text: words.iter().flat_map(|w| w.to_be_bytes()).collect(),
        functions: &[
            ("vr", 0x00, 0x04),
            ("w2", 0x04, 0x2c),
            ("w3", 0x30, 0x3c),
            ("g", 0x6c, 0x04),
            ("w4", 0x70, 0x38),
            ("w6", 0xa8, 0x48),
            ("w7", 0xf0, 0x4c),
        ],
        calls: &[],
    }
}

fn object(image: &Image) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, image.arch, image.endian);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(text, &image.text, 16);
    for &(name, value, size) in image.functions {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    let mut undefined = std::collections::BTreeMap::new();
    for &(offset, name, r_type, addend) in image.calls {
        let symbol = *undefined.entry(name).or_insert_with(|| {
            obj.add_symbol(Symbol {
                name: name.as_bytes().to_vec(),
                value: 0,
                size: 0,
                kind: SymbolKind::Unknown,
                scope: SymbolScope::Linkage,
                weak: false,
                section: SymbolSection::Undefined,
                flags: SymbolFlags::None,
            })
        });
        obj.add_relocation(
            text,
            Relocation {
                offset,
                symbol,
                addend,
                flags: RelocationFlags::Elf { r_type },
            },
        )
        .unwrap();
    }
    obj.write().unwrap()
}

fn decompile(stem: &str, image: &Image) -> String {
    let path = common::scratch_file(stem, "o");
    std::fs::write(&path, object(image)).unwrap();
    let names: Vec<&str> = image
        .functions
        .iter()
        .map(|f| f.0)
        .filter(|name| !["vr", "g"].contains(name))
        .collect();
    let mut cmd = Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    );
    cmd.args([
        "decompile-all",
        path.to_str().unwrap(),
        "--assert-strict",
        "--functions",
    ])
    .arg(names.join(","));
    for directive in PROTOTYPES {
        let name = directive.split_whitespace().nth(1).unwrap();
        if ["vr", "g"].contains(&name) || names.contains(&name) {
            cmd.args(["--assert", directive]);
        }
    }
    let output = cmd.output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{stem}: {text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::remove_file(path).unwrap();
    text
}

fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let marker = format!("// Function: {name} @");
    text.split(&marker)
        .nth(1)
        .unwrap_or_else(|| panic!("no {name}:\n{text}"))
        .split("// Function:")
        .next()
        .unwrap()
}

#[test]
fn an_x86_64_double_in_xmm0_round_trips_through_the_printed_c() {
    let stem = "variadic-xmm0";
    let printed = decompile(stem, &x86_64());
    for (name, call) in [
        ("w1", "vr(k,(double)k,(unsigned long)(k + 1))"),
        ("w2", "vr(k,v1 + v1)"),
        ("w3", "vr(k,v1,v2,(unsigned long)(unsigned int)k)"),
        ("w0", "vr(k,(unsigned long)(k + 1))"),
        ("w0x", "vr(k,(unsigned long)(k + 1))"),
        ("w4", "vr(k,v1)"),
        ("w5", "vr(k,v1)"),
        ("w6", "vr(k,v1) + v1"),
        ("w7", "vr(k,(unsigned long)(k + 1)) + v1"),
    ] {
        assert!(
            function(&printed, name).contains(call),
            "{name}: `{call}`\n{printed}"
        );
    }
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(
        !compilers.is_empty(),
        "the round trip requires a C compiler"
    );
    let src = common::scratch_file(stem, "c");
    let exe = common::scratch_file(stem, "exe");
    std::fs::write(
        &src,
        format!(
            r#"
#include <stdarg.h>
static const char *fmt;
double vr(int n, ...) {{
    va_list ap;
    va_start(ap, n);
    double s = n;
    for (const char *f = fmt; *f; ++f)
        s = s * 3 + (*f == 'i' ? va_arg(ap, int) : va_arg(ap, double));
    va_end(ap);
    return s;
}}
double g(int k) {{ return k * 0.75 + 0.125; }}
{printed}
{SOURCE}
int main(void) {{
    double p[2] = {{1.25, -7.5}};
    for (int k = -3; k < 4; ++k) {{
        fmt = "id";
        if (w1(k) != s_w1(k)) return 1;
        fmt = "d";
        if (w2(k, p) != s_w2(k, p)) return 2;
        fmt = "ddi";
        if (w3(k, p) != s_w3(k, p)) return 3;
        fmt = "d";
        if (w4(k) != s_w4(k)) return 4;
        if (w5(k, p) != s_w5(k, p)) return 5;
        if (w6(k) != s_w6(k)) return 6;
        fmt = "i";
        if (w7(k) != s_w7(k)) return 7;
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
                .args(["-std=gnu11", "-w", level, "-o"])
                .arg(&exe)
                .arg(&src)
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{stem} {cc} {level}: {}\n{printed}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let status = Command::new(&exe).status().unwrap();
            assert!(status.success(), "{stem} {cc} {level}: {status}\n{printed}");
        }
    }
    std::fs::remove_file(src).unwrap();
    std::fs::remove_file(exe).unwrap();
}

#[test]
fn aarch64_and_powerpc_pass_the_double_in_their_return_register() {
    let printed = decompile("variadic-d0", &aarch64());
    for (name, call) in [
        ("w1", "vr(k,(double)k,(unsigned long)(k + 1))"),
        ("w2", "vr(k,v1 + v1)"),
        ("w3", "vr(k,v1,v2,(unsigned long)(unsigned int)k)"),
        ("w8", "vr(k,0.5,(unsigned long)(k + 1))"),
    ] {
        assert!(
            function(&printed, name).contains(call),
            "aarch64 {name}: `{call}`\n{printed}"
        );
    }
    let printed = decompile("variadic-f1", &powerpc());
    for (name, call) in [
        ("w2", "vr(k,v1 + v1)"),
        ("w3", "vr(k,v1,v2,k)"),
        ("w4", "vr(k,v1)"),
        ("w6", "vr(k,v1)"),
        ("w7", "vr(k,k + 1)"),
    ] {
        assert!(
            function(&printed, name).contains(call),
            "powerpc {name}: `{call}`\n{printed}"
        );
    }
}
