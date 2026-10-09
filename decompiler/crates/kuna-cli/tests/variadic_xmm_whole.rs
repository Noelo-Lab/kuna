//! An x86-64 variadic call whose caller counts its vector registers in `al`
//! takes a `double` from every counted `xmm` register, however the register was
//! written: whole by `pxor`, `movq` or `cvtsi2sd` after `pxor`, or in 4-byte
//! lanes by `movaps`. The printed C is compiled back with gcc and clang and run
//! against the source. A register written whole past the count stays out.
mod common;
use common::process;
use object::write::{Object, Relocation, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, RelocationFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

const SOURCE: &str = r#"
double s_x1(int k, double x, double y) { return vr(k, y, x); }
double s_x2(int k) { return vr(k, (double)k * k); }
double s_x3(int k) { return vr(k, 0.0); }
double s_x4(int k, double x, double y) { return vr(k, y, x); }
double s_x5(int k) { return vr(k, g(k)); }
int s_x6(int k, double a) { return (int)vr(k, k, a + a, (double)k); }
double s_x7(int k, double x, double y) { return vr(k, y, x, y); }
double s_z0(int k) { return vr(k); }
double s_z2(int k) { return vr(k, (double)k); }
"#;

const PROTOTYPES: [&str; 11] = [
    "prototype vr double vr(int n, ...)",
    "prototype g double g(int k)",
    "prototype x1 double x1(int k, double x, double y)",
    "prototype x2 double x2(int k)",
    "prototype x3 double x3(int k)",
    "prototype x4 double x4(int k, double x, double y)",
    "prototype x5 double x5(int k)",
    "prototype x6 int x6(int k, double a)",
    "prototype x7 double x7(int k, double x, double y)",
    "prototype z0 double z0(int k)",
    "prototype z2 double z2(int k)",
];

const FUNCTIONS: [(&str, u64, u64); 9] = [
    ("x1", 0x00, 0x10),
    ("x7", 0x10, 0x10),
    ("x2", 0x20, 0x1a),
    ("x3", 0x40, 0x12),
    ("x6", 0x60, 0x29),
    ("x4", 0x90, 0x46),
    ("x5", 0xe0, 0x3e),
    ("z0", 0x120, 0x0b),
    ("z2", 0x130, 0x1a),
];

const CALLS: [(u64, &str); 10] = [
    (0x0c, "vr"),
    (0x1c, "vr"),
    (0x36, "vr"),
    (0x4e, "vr"),
    (0x7c, "vr"),
    (0xc6, "vr"),
    (0xf5, "g"),
    (0x10e, "vr"),
    (0x127, "vr"),
    (0x146, "vr"),
];

fn hex(text: &str) -> Vec<u8> {
    let hex: String = text.split_whitespace().collect();
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

/// clang -O2 of `x1` and `x7` of [`SOURCE`], which swap the doubles with
/// `movaps`; gcc -O2 of `x2`, `x3` and `x6`, which clear the register with
/// `pxor`; gcc -O0 of `x4` and `x5`, which move the double in with `movq`.
/// `z0` is `pxor %xmm0,%xmm0; xor %eax,%eax; jmp vr`, and `z2` converts `k`
/// into both `xmm1` and `xmm0` after a `pxor` of each but sets `al` to one.
fn object() -> Vec<u8> {
    let text = hex(
        "0f28d00f28c10f28cab002e9000000000f28d10f28c80f28c2b003e900000000\
         f30f1efa660fefc0b801000000f20f2ac7f20f59c0e900000000cccccccccccc\
         f30f1efa660fefc0b801000000e900000000cccccccccccccccccccccccccccc\
         f30f1efa660fefc9f20f58c04883ec0889fef20f2acfb802000000e800000000\
         4883c408f20f2cc0c3ccccccccccccccf30f1efa554889e54883ec20897dfcf2\
         0f1145f0f20f114de8f20f1045f0488b55e88b45fc660f28c866480f6ec289c7\
         b802000000e80000000066480f7ec066480f6ec0c9c3cccccccccccccccccccc\
         f30f1efa554889e54883ec10897dfc8b45fc89c7e80000000066480f7ec08b55\
         fc66480f6ec089d7b801000000e80000000066480f7ec066480f6ec0c9c3cccc\
         660fefc031c0e900000000cccccccccc660fefc9f20f2acf660fefc0f20f2ac7\
         b801000000e900000000",
    );
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let section = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(section, &text, 16);
    for (name, value, size) in FUNCTIONS {
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
    let mut undefined = std::collections::BTreeMap::new();
    for (offset, name) in CALLS {
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
            section,
            Relocation {
                offset,
                symbol,
                addend: -4,
                flags: RelocationFlags::Elf {
                    r_type: object::elf::R_X86_64_PLT32,
                },
            },
        )
        .unwrap();
    }
    obj.write().unwrap()
}

fn decompile(stem: &str) -> String {
    let path = common::scratch_file(stem, "o");
    std::fs::write(&path, object()).unwrap();
    let names: Vec<&str> = FUNCTIONS.iter().map(|f| f.0).collect();
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
        cmd.args(["--assert", directive]);
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
fn a_variadic_double_in_a_whole_or_laned_xmm_register_round_trips() {
    let stem = "variadic-xmm-whole";
    let printed = decompile(stem);
    for (name, call) in [
        ("x1", "vr(k,y,x)"),
        ("x7", "vr(k,y,x,y)"),
        ("x2", "vr(k,(double)k * (double)k)"),
        ("x3", "vr(k,0.0)"),
        ("x6", "vr(k,a + a,(double)k,"),
        ("x4", "vr(k,y,x)"),
        ("x5", "vr(k,v1)"),
        ("z0", "vr(k)"),
        ("z2", "vr(k,(double)k)"),
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
    for (int k = -3; k < 4; ++k) {{
        double x = k * 1.25 + 0.0625, y = 7.5 - k * 0.375;
        fmt = "dd";
        if (x1(k, x, y) != s_x1(k, x, y)) return 1;
        if (x4(k, x, y) != s_x4(k, x, y)) return 4;
        fmt = "d";
        if (x2(k) != s_x2(k)) return 2;
        if (x3(k) != s_x3(k)) return 3;
        if (x5(k) != s_x5(k)) return 5;
        if (z2(k) != s_z2(k)) return 9;
        fmt = "idd";
        if (x6(k, x) != s_x6(k, x)) return 6;
        fmt = "ddd";
        if (x7(k, x, y) != s_x7(k, x, y)) return 7;
        fmt = "";
        if (z0(k) != s_z0(k)) return 8;
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
                .args(common::CC_GCC15_DEMOTE)
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
