//! A declared integer argument or return value narrower than its register is
//! extended as the calling convention states, so the printed C computes what
//! the binary does. RISC-V and LoongArch widen it by the sign of its type to 32
//! bits and sign-extend that to the register; MIPS and Apple arm64 do the same
//! for arguments, and their compilers for return values (`narrowext compiler`).
//! With the compiler spec's own extension a negative result read as a large
//! positive one, or the rest of its register as an unassigned piece.
mod common;
use common::process;
use std::process::Command;

/// The 64-bit images' source, built with `clang -O2` (LoongArch hand-encoded
/// from the same C).
const WIDE: &str = r#"
signed char s_rsc(int k) { return k - 100; }
unsigned short s_rus(int k) { return k * 300; }
int s_ri(int k) { return k * 0x10001 - 7; }
unsigned s_ru(int k) { return k * 0x10001u - 7; }
long s_csc(int k) { return s_rsc(k) * 3L; }
long s_cus(int k) { return s_rus(k) + 1L; }
long s_ci(int k) { return s_ri(k); }
long s_cu(int k) { return (int)s_ru(k); }
unsigned long s_cuz(int k) { return s_ru(k); }
long s_pi(int x) { return x * 2L; }
long s_pus(unsigned short x) { return x + 1L; }
long s_pu(unsigned x) { return (int)x; }
long s_psc(signed char x) { return x * 3L; }
"#;

const WIDE_PROTOS: &[&str] = &[
    "signed char rsc(int k)",
    "unsigned short rus(int k)",
    "int ri(int k)",
    "unsigned int ru(int k)",
    "long csc(int k)",
    "long cus(int k)",
    "long ci(int k)",
    "long cu(int k)",
    "unsigned long cuz(int k)",
    "long pi(int x)",
    "long pus(unsigned short x)",
    "long pu(unsigned int x)",
    "long psc(signed char x)",
];

/// The 32-bit images' source, built with `clang -O2`.
const NARROW: &str = r#"
signed char s_rsc(int k) { return k - 100; }
unsigned short s_rus(int k) { return k * 300; }
bool s_rb(int k) { return k > 3; }
int s_csc(int k) { return s_rsc(k) * 3; }
int s_cus(int k) { return s_rus(k) + 1; }
int s_cb(int k) { return s_rb(k) + 5; }
int s_psc(signed char x) { return x * 3; }
int s_pus(unsigned short x) { return x + 1; }
int s_pb(bool b) { return b + 5; }
"#;

const NARROW_PROTOS: &[&str] = &[
    "signed char rsc(int k)",
    "unsigned short rus(int k)",
    "bool rb(int k)",
    "int csc(int k)",
    "int cus(int k)",
    "int cb(int k)",
    "int psc(signed char x)",
    "int pus(unsigned short x)",
    "int pb(bool b)",
];

/// The functions that return a callee's narrow result, and those that take a
/// narrow argument.
const RETURNS: &[&str] = &["csc", "cus", "ci", "cu", "cuz", "cb"];
const ARGUMENTS: &[&str] = &["pi", "pus", "pu", "psc", "pb"];

struct Image {
    name: &'static str,
    target: &'static str,
    code: &'static str,
    starts: &'static [u64],
    source: &'static str,
    protos: &'static [&'static str],
}

const BASE: u64 = 0x10000;

const IMAGES: &[Image] = &[
    Image {
        name: "rv64",
        target: "RISCV:LE:64:RV64GC:gcc",
        code: "1b05c5f96215619582809305c0123305b502c165f1356d8d82809b1505012d9d653582809b1505\
               012d9d65358280411106e497000000e780e0fc931515002e95a26041018280411106e497000000\
               e78000fc0505a26041018280411106e497000000e780c0fba26041018280411106e497000000e7\
               8040fba26041018280411106e497000000e78020fa02150191a260410182800605828005058280\
               8280931515002e958280",
        starts: &[0x0, 0xa, 0x1a, 0x24, 0x2e, 0x46, 0x5a, 0x6c, 0x7e, 0x94, 0x98, 0x9c, 0x9e],
        source: WIDE,
        protos: WIDE_PROTOS,
    },
    Image {
        name: "la64",
        target: "Loongarch:LE:64:lp64d:default",
        code: "8470be02845c00002000004c0cb0840284301c008400cf002000004c8cc040008430100084e4bf\
               022000004c8cc040008430100084e4bf022000004c63c0ff026120c029ffbfff578c04410084b0\
               10006120c0286340c0022000004c63c0ff026120c029ffabff578404c0026120c0286340c00220\
               00004c63c0ff026120c029ff9fff576120c0286340c0022000004c63c0ff026120c029ff97ff57\
               6120c0286340c0022000004c63c0ff026120c029ff7fff578400df006120c0286340c002200000\
               4c840441002000004c8404c0022000004c2000004c8c04410084b010002000004c",
        starts: &[0x0, 0xc, 0x1c, 0x2c, 0x3c, 0x5c, 0x78, 0x90, 0xa8, 0xc4, 0xcc, 0xd4, 0xd8],
        source: WIDE,
        protos: WIDE_PROTOS,
    },
    Image {
        name: "mips64el",
        target: "MIPS:LE:64:default:default",
        code: "9cff81240800e0032014017c0000000080080400c0100400210841004011040021084100001204\
               00210841000800e003fcff223000000000000c0400210824000800e003f9ff2224000c04002108\
               24000800e003f9ff2224f0ffbd670800bfff0040000c0000000000080200781001002d10410008\
               00bfdf0800e0031000bd67f0ffbd670800bfff0440000c0000000003f8417c010022640800bfdf\
               0800e0031000bd6700000000f0ffbd670800bfff0e40000c00000000001002000800bfdf0800e0\
               031000bd67f0ffbd670800bfff1240000c00000000001002000800bfdf0800e0031000bd67f0ff\
               bd670800bfff1240000c0000000003f8427c0800bfdf0800e0031000bd670800e0037810040008\
               00e003010082640800e00325108000780804000800e0032d102400",
        starts: &[0x0, 0x10, 0x38, 0x48, 0x58, 0x80, 0xa8, 0xc8, 0xe8, 0x108, 0x110, 0x118, 0x120],
        source: WIDE,
        protos: WIDE_PROTOS,
    },
    Image {
        name: "rv32",
        target: "RISCV:LE:32:RV32GC:gcc",
        code: "1305c5f96205618582809305c0123305b502c165f1156d8d82808d4533a5a5008280411106c6\
               97000000e780a0fd931515002e95b24041018280411106c697000000e780c0fc0505b2404101\
               8280411106c697000000e78080fc1505b24041018280931515002e9582800505828015058280",
        starts: &[0x0, 0xa, 0x1a, 0x22, 0x3a, 0x4e, 0x62, 0x6a, 0x6e],
        source: NARROW,
        protos: NARROW_PROTOS,
    },
    Image {
        name: "mipsel",
        target: "MIPS:LE:32:default:default",
        code: "9cff81240800e0032014017c80080400c010040021084100401104002108410000120400210841\
               000800e003fcff2230030001240800e0032a102400e8ffbd271400bfaf0040000c000000004008\
               0200211022001400bf8f0800e0031800bd27e8ffbd271400bfaf0340000c000000000100422414\
               00bf8f0800e0031800bd27e8ffbd271400bfaf0c40000c0000000005000124060003240b086200\
               251020001400bf8f0800e0031800bd27400804000800e003211024000800e00301008224050002\
               24060001240800e0030b102400",
        starts: &[0x0, 0xc, 0x30, 0x3c, 0x60, 0x80, 0xac, 0xb8, 0xc0],
        source: NARROW,
        protos: NARROW_PROTOS,
    },
    Image {
        name: "apple",
        target: "AARCH64:LE:64:AppleSilicon:default",
        code: "08900151001d0013c0035fd688258052087c081b00351e12c0035fd61f0c0071e0d79f1ac003\
               5fd6fd7bbfa9fd030091f4ffff970004000bfd7bc1a8c0035fd6fd7bbfa9fd030091f1ffff97\
               00040011fd7bc1a8c0035fd6fd7bbfa9fd030091efffff971f000071a80080520005881afd7b\
               c1a8c0035fd60004000bc0035fd600040011c0035fd61f000071a80080520005881ac0035fd6",
        starts: &[0x0, 0xc, 0x1c, 0x28, 0x40, 0x58, 0x78, 0x80, 0x88],
        source: NARROW,
        protos: NARROW_PROTOS,
    },
];

fn bytes(hex: &str) -> Vec<u8> {
    let hex: String = hex.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
}

fn name(proto: &str) -> &str {
    proto.split('(').next().unwrap().rsplit(' ').next().unwrap()
}

fn decompile(image: &Image, mode: &str) -> String {
    let path = common::scratch_file(&format!("narrowext-{}", image.name), "bin");
    std::fs::write(&path, bytes(image.code)).unwrap();
    let base = format!("{BASE:#x}");
    let mut args: Vec<String> = ["decompile-all", path.to_str().unwrap(), "--raw-image", "--target", image.target]
        .into_iter()
        .map(String::from)
        .collect();
    args.extend(["--base".into(), base, "--assert-strict".into(), "--option".into(), "narrowext".into(), mode.into()]);
    for (proto, start) in image.protos.iter().zip(image.starts) {
        let at = format!("{:#x}", BASE + start);
        args.extend(["--entry".into(), at.clone(), "--define-function".into(), format!("{at}={}", name(proto))]);
        args.extend(["--assert".into(), format!("prototype {} {proto}", name(proto))]);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{}: {text}\n{}", image.name, String::from_utf8_lossy(&output.stderr));
    text
}

/// The printed function `name`, from its header comment to the next.
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let start = text.find(&format!("// Function: {name} @")).unwrap_or_else(|| panic!("no {name}:\n{text}"));
    let rest = &text[start + 1..];
    &text[start..start + 1 + rest.find("// Function:").unwrap_or(rest.len())]
}

/// The argument `proto` takes, built from the loop variable `v`.
fn argument(proto: &str) -> String {
    let param = proto.split('(').nth(1).unwrap();
    let ty = param.rsplit_once(' ').unwrap().0;
    format!("({ty})v")
}

/// Compiles the printed callees and `checked` with the source, and runs each
/// of `checked` against its source over a range of inputs.
fn round_trip(image: &Image, text: &str, checked: &[&str], compilers: &[&str]) -> Result<(), String> {
    let mut printed = String::new();
    let mut checks = String::new();
    for proto in image.protos {
        let n = name(proto);
        let callee = !RETURNS.contains(&n) && !ARGUMENTS.contains(&n);
        if callee || checked.contains(&n) {
            printed.push_str(function(text, n));
        }
        if checked.contains(&n) {
            let arg = argument(proto);
            checks.push_str(&format!("    CHECK({n}({arg}), s_{n}({arg}));\n"));
        }
    }
    let program = format!(
        "#include <stdio.h>\n#include <stdbool.h>\n{printed}\n{}\n\
         static const int V[] = {{0, 1, -1, 3, 4, 99, 100, 101, 127, 128, 227, 228, -28, -29, 0x7fff, 0x8000,\n\
         0xffff, 0x10000, 0x7fffffff, (int)0x80000000, -7, 0x12345678, (int)0xdeadbeef}};\n\
         #define CHECK(got, want) do {{ long long g = (got), w = (want); if (g != w) {{ \\\n\
         printf(\"%s v=%#x: %llx, want %llx\\n\", #got, v, g, w); bad++; }} }} while (0)\n\
         int main(void) {{\n  int bad = 0;\n  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {{\n\
         int v = V[i];\n{checks}  }}\n  return bad != 0;\n}}\n",
        image.source
    );
    compile_and_run(image.name, &program, compilers)
}

/// Builds `program` with each of `compilers` at -O0 and -O2 and runs it; it
/// exits nonzero, printing the mismatches, when a check fails.
fn compile_and_run(stem: &str, program: &str, compilers: &[&str]) -> Result<(), String> {
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let src = common::scratch_file(&format!("narrowext-{stem}-{cc}{level}"), "c");
            let exe = src.with_extension("exe");
            std::fs::write(&src, program).unwrap();
            let out = Command::new(cc)
                .args(["-std=gnu11", "-w", "-fwrapv", level, "-o", exe.to_str().unwrap(), src.to_str().unwrap()])
                .output()
                .expect("spawn the C compiler");
            if !out.status.success() {
                return Err(format!("{cc} {level} rejects the printed C:\n{}", String::from_utf8_lossy(&out.stderr)));
            }
            let run = Command::new(&exe).output().expect("run the round trip");
            if !run.status.success() {
                return Err(format!("{cc} {level}:\n{}", String::from_utf8_lossy(&run.stdout)));
            }
        }
    }
    Ok(())
}

fn host_compilers() -> Vec<&'static str> {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    compilers
}

#[test]
fn a_narrow_argument_or_return_round_trips_through_the_printed_c() {
    let compilers = host_compilers();
    let all: Vec<&str> = RETURNS.iter().chain(ARGUMENTS).copied().collect();
    for image in IMAGES {
        let present = |set: &[&'static str]| -> Vec<&'static str> {
            set.iter().copied().filter(|n| image.protos.iter().any(|p| name(p) == *n)).collect()
        };
        let documented_returns = matches!(image.name, "rv64" | "la64" | "rv32");
        let default = if documented_returns { present(&all) } else { present(ARGUMENTS) };
        let text = decompile(image, "abi");
        round_trip(image, &text, &default, &compilers).unwrap_or_else(|e| panic!("{}: {e}\n{text}", image.name));
        let text = decompile(image, "compiler");
        round_trip(image, &text, &present(&all), &compilers)
            .unwrap_or_else(|e| panic!("{} under compiler: {e}\n{text}", image.name));
        let text = decompile(image, "off");
        assert!(
            round_trip(image, &text, &default, &compilers).is_err(),
            "{}: the compiler spec's extension already round-trips:\n{text}",
            image.name
        );
    }
}

/// `narrowext_rv64.cpp`'s functions in C, and the names its printed C uses.
const SIGNED_SOURCE: &str = r#"
#include <stdbool.h>
typedef signed char neg1;
enum { NEGA = -100, NEGB = 100 };
long s_c16(int k) { return (unsigned short)(k * 300 + 5) + 1L; }
long s_ce2(int k) { return (unsigned short)(k * 1000) + 1L; }
long s_cn(int k) { return (signed char)(k - 100) / 3L; }
long s_widen16(unsigned short c) { return c; }
bool s_hi_sur(unsigned short c) { return c >= 0xD800 && c <= 0xDBFF; }
long s_pe2(unsigned short x) { return x + 1L; }
long s_pn(signed char x) { return x / 3L; }
"#;

/// Each checked function of `narrowext_rv64.cpp` with the argument it takes,
/// and the callees its printed C calls.
const SIGNED_CHECKS: &[(&str, &str)] = &[
    ("c16", "v"),
    ("ce2", "v"),
    ("cn", "v"),
    ("widen16", "(unsigned short)v"),
    ("hi_sur", "(unsigned short)v"),
    ("pe2", "(unsigned short)v"),
    ("pn", "(neg1)v"),
];
const SIGNED_CALLEES: &[&str] = &["r16", "re2", "rn"];

/// The sign the rule extends a value by is the one its type states, so the
/// type's sign must be the source's: `char16_t` is unsigned whether DWARF
/// (`DW_ATE_UTF`) or a mangled name (`Ds`) states it, and an enum without
/// `DW_AT_encoding`, which is how clang describes one, takes the sign of the
/// integer its `DW_AT_type` names.
#[test]
fn a_char16_t_or_short_enum_extends_by_the_sign_of_its_source_type() {
    let compilers = host_compilers();
    for (fixture, checked) in [
        ("narrowext_rv64.o", SIGNED_CHECKS.iter().map(|c| c.0).collect::<Vec<_>>()),
        ("narrowext_rv64_nodebug.o", vec!["widen16", "hi_sur"]),
    ] {
        let path = common::fixture(fixture);
        let (text, err, rc) = common::run_kuna(&["decompile-all", &path]);
        assert_eq!(rc, 0, "{fixture}: {err}");
        let mut printed = String::new();
        let mut checks = String::new();
        if checked.contains(&"c16") {
            for callee in SIGNED_CALLEES {
                printed.push_str(function(&text, callee));
            }
        }
        for (n, arg) in SIGNED_CHECKS.iter().filter(|c| checked.contains(&c.0)) {
            printed.push_str(function(&text, n));
            checks.push_str(&format!("    CHECK({n}({arg}), s_{n}({arg}));\n"));
        }
        let program = format!(
            "#include <stdio.h>\n{SIGNED_SOURCE}\n{printed}\n\
             static const int V[] = {{0, 1, -1, 99, 100, 101, 127, 128, 0x7fff, 0x8000, 0xd7ff, 0xd800,\n\
             0xdbff, 0xdc00, 0xffff, 0x10000, 40000, 50000, -100, 200}};\n\
             #define CHECK(got, want) do {{ long long g = (got), w = (want); if (g != w) {{ \\\n\
             printf(\"%s v=%#x: %llx, want %llx\\n\", #got, v, g, w); bad++; }} }} while (0)\n\
             int main(void) {{\n  int bad = 0;\n  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {{\n\
             int v = V[i];\n{checks}  }}\n  return bad != 0;\n}}\n"
        );
        compile_and_run(fixture, &program, &compilers).unwrap_or_else(|e| panic!("{fixture}: {e}\n{text}"));
    }
}

/// A recovered 64-bit return whose high half the function zeroes, and two
/// callers that widen a result. clang -O0 and -O2 for RISC-V (calls relocated
/// by hand, the callees `noinline`), and LoongArch hand-encoded from the -O2
/// shape (`alsl.d`, `bstrpick.d a0,a0,31,0`, `alsl.w`, `bl`).
const ZEXT_SOURCE: &str = r#"
unsigned long s_z32m(unsigned long x) { return (x * 3) & 0xffffffffUL; }
unsigned long s_zu(unsigned int x) { return x; }
unsigned s_ru(unsigned x) { return x * 3; }
long s_cz(unsigned long x) { return s_z32m(x) + 1; }
unsigned long s_cru(unsigned x) { return s_ru(x); }
"#;

/// Each checked function with the argument it takes; `ru` is only a callee.
const ZEXT_ARGS: &[(&str, &str)] =
    &[("z32m", "(unsigned long)v"), ("zu", "(unsigned)v"), ("cz", "(unsigned long)v"), ("cru", "(unsigned)v")];
const ZEXT_CALLEES: &[&str] = &["z32m", "zu", "ru"];

struct ZextImage {
    name: &'static str,
    target: &'static str,
    code: &'static str,
    functions: &'static [(&'static str, u64)],
    checked: &'static [&'static str],
}

const ZEXT_IMAGES: &[ZextImage] = &[
    ZextImage {
        name: "rv64-O0",
        target: "RISCV:LE:64:RV64GC:gcc",
        code: "011106ec22e800102334a4fe833584fe1b9515002d9d02150191e260426405618280011106ec22e8\
               00102326a4fe0365c4fee260426405618280011106ec22e800102326a4fe8325c4fe1b9515002d9d\
               e260426405618280011106ec22e800102334a4fe033584fe97000000e78080f90505e26042640561\
               8280011106ec22e800102326a4fe0325c4fe97000000e78000fb02150191e260426405618280",
        functions: &[("z32m", 0x0), ("zu", 0x22), ("ru", 0x3a), ("cz", 0x58), ("cru", 0x7a)],
        checked: &["z32m", "zu", "cz", "cru"],
    },
    ZextImage {
        name: "rv64-O2",
        target: "RISCV:LE:64:RV64GC:gcc",
        code: "9b1515002d9d0215019182800215019182809b1515002d9d8280411106e497000000e78020fe0505\
               a26041018280411106e497000000e78000fe02150191a26041018280",
        functions: &[("z32m", 0x0), ("zu", 0xc), ("ru", 0x12), ("cz", 0x1a), ("cru", 0x2e)],
        checked: &["z32m", "zu"],
    },
    ZextImage {
        name: "la64",
        target: "Loongarch:LE:64:lp64d:default",
        code: "84102c008400df002000004c8400df002000004c841004002000004c63c0ff026120c029ffdfff57\
               8404c0026120c0286340c0022000004c63c0ff026120c029ffd7ff578400df006120c0286340c002\
               2000004c",
        functions: &[("z32m", 0x0), ("zu", 0xc), ("ru", 0x14), ("cz", 0x1c), ("cru", 0x38)],
        checked: &["z32m", "zu", "cz", "cru"],
    },
];

fn decompile_unprototyped(image: &ZextImage, mode: &str) -> String {
    let path = common::scratch_file(&format!("zextword-{}", image.name), "bin");
    std::fs::write(&path, bytes(image.code)).unwrap();
    let mut args: Vec<String> = ["decompile-all", path.to_str().unwrap(), "--raw-image", "--target", image.target]
        .into_iter()
        .map(String::from)
        .collect();
    args.extend(["--base".into(), format!("{BASE:#x}"), "--option".into(), "narrowext".into(), mode.into()]);
    for (name, start) in image.functions {
        let at = format!("{:#x}", BASE + start);
        args.extend(["--entry".into(), at.clone(), "--define-function".into(), format!("{at}={name}")]);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().unwrap();
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{}: {text}\n{}", image.name, String::from_utf8_lossy(&output.stderr));
    text
}

fn zext_round_trip(image: &ZextImage, text: &str, compilers: &[&str]) -> Result<(), String> {
    let mut printed = String::new();
    let mut checks = String::new();
    for (n, _) in image.functions {
        if ZEXT_CALLEES.contains(n) || image.checked.contains(n) {
            printed.push_str(function(text, n));
        }
    }
    for (n, arg) in ZEXT_ARGS.iter().filter(|c| image.checked.contains(&c.0)) {
        checks.push_str(&format!("    CHECK({n}({arg}), s_{n}({arg}));\n"));
    }
    let program = format!(
        "#include <stdio.h>\n{ZEXT_SOURCE}\n{printed}\n\
         static const int V[] = {{0, 1, -1, 3, 0x2aaaaaab, 0x7fffffff, (int)0x80000000, -7, 0x12345678,\n\
         (int)0xdeadbeef}};\n\
         #define CHECK(got, want) do {{ long long g = (got), w = (want); if (g != w) {{ \\\n\
         printf(\"%s v=%#x: %llx, want %llx\\n\", #got, v, g, w); bad++; }} }} while (0)\n\
         int main(void) {{\n  int bad = 0;\n  for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {{\n\
         int v = V[i];\n{checks}  }}\n  return bad != 0;\n}}\n"
    );
    compile_and_run(&format!("zextword-{}", image.name), &program, compilers)
}

/// RISC-V and LoongArch LP64 sign-extend every 32-bit return value, so a
/// function that zero-extends a word whose sign bit may be set returns the
/// whole register, and its callers widen the result as the binary does. With
/// the compiler spec's extension (`narrowext off`) it reads as `int`.
#[test]
fn a_zero_extended_word_return_round_trips_through_the_printed_c() {
    let compilers = host_compilers();
    for image in ZEXT_IMAGES {
        let text = decompile_unprototyped(image, "abi");
        zext_round_trip(image, &text, &compilers).unwrap_or_else(|e| panic!("{}: {e}\n{text}", image.name));
        let text = decompile_unprototyped(image, "off");
        assert!(
            zext_round_trip(image, &text, &compilers).is_err(),
            "{}: the compiler spec's extension already round-trips:\n{text}",
            image.name
        );
    }
}

/// `charsign.c`'s functions, renamed so the printed ones can sit beside them.
const CHARSIGN_SOURCE: &str = r#"
typedef unsigned char u8;
int s_fmt(char *b, const char *f, unsigned char c) { return sprintf(b, f, c); }
int s_fmts(char *b, const char *f, signed char c) { return sprintf(b, f, c); }
int s_fmt8(char *b, const char *f, u8 c) { return sprintf(b, f, c); }
int s_fmtc(char *b, const char *f, char c) { return sprintf(b, f, c); }
int s_is_hi(unsigned char c) { return c >= 0x80; }
long s_wid(unsigned char c) { return c; }
long s_wids(signed char c) { return c; }
long s_wus(unsigned short c) { return c; }
long s_wss(short c) { return c * 3L; }
int s_wb(_Bool b) { return b + 5; }
int rd1(unsigned char *b, int v) { b[0] = v; return 1; }
int s_getc1(int v) { unsigned char b[1]; rd1(b, v); return b[0]; }
int s_seq(signed char c) { return c == -56; }
"#;

/// Each round-tripped function of `charsign.c` and the parameter type it takes.
const CHARSIGN_FORMATS: &[(&str, &str)] =
    &[("fmt", "unsigned char"), ("fmts", "signed char"), ("fmt8", "u8"), ("fmtc", "char")];
const CHARSIGN_VALUES: &[(&str, &str)] = &[
    ("is_hi", "unsigned char"),
    ("wid", "unsigned char"),
    ("wids", "signed char"),
    ("wus", "unsigned short"),
    ("wss", "short"),
    ("wb", "_Bool"),
    ("getc1", "int"),
    ("seq", "signed char"),
];

/// Compiles the printed `checked` functions of `charsign.c` beside its source
/// and runs each against its source over a range of inputs.
fn charsign_round_trip(stem: &str, text: &str, checked: &[&str], compilers: &[&str]) -> Result<(), String> {
    let mut printed = String::new();
    let mut checks = String::new();
    for (n, ty) in CHARSIGN_FORMATS.iter().filter(|c| checked.contains(&c.0)) {
        printed.push_str(function(text, n));
        checks.push_str(&format!(
            "    {n}(x, \"%d\", ({ty})v); s_{n}(y, \"%d\", ({ty})v);\n    \
             if (strcmp(x, y)) {{ printf(\"{n} v=%#x: %s, want %s\\n\", v, x, y); bad++; }}\n"
        ));
    }
    for (n, ty) in CHARSIGN_VALUES.iter().filter(|c| checked.contains(&c.0)) {
        printed.push_str(function(text, n));
        checks.push_str(&format!("    CHECK({n}(({ty})v), s_{n}(({ty})v));\n"));
    }
    let program = format!(
        "#include <stdio.h>\n#include <string.h>\n#include <stdbool.h>\n{CHARSIGN_SOURCE}\n{printed}\n\
         static const int V[] = {{0, 1, -1, 5, 99, 127, 128, 200, 202, 255, 256, 0x7fff, 0x8000, 0xffff, -200}};\n\
         #define CHECK(got, want) do {{ long long g = (got), w = (want); if (g != w) {{ \\\n\
         printf(\"%s v=%#x: %llx, want %llx\\n\", #got, v, g, w); bad++; }} }} while (0)\n\
         int main(void) {{\n  int bad = 0;\n  char x[32], y[32];\n\
         for (unsigned i = 0; i < sizeof V / sizeof V[0]; i++) {{\n    int v = V[i];\n{checks}  }}\n\
         return bad != 0;\n}}\n"
    );
    compile_and_run(stem, &program, compilers)
}

/// A DWARF `unsigned char` or `signed char` parameter keeps the sign it was
/// declared with; it used to read as plain `char`, so `sprintf(b,f,c)` with
/// `char c` printed 202 as -54 when compiled. A one-element `unsigned char`
/// array read whole extends as its element (`(int)b[0]`, not `ZEXT14(b[0])`).
/// clang -O2 also reads a narrow parameter as the 32-bit value its caller
/// extended it to, which only `narrowext compiler` states on x86-64: by default
/// (and with it off) that prints `CONCAT31(v1,c)` with an unassigned `v1`.
/// clang's `getc1` keeps its buffer in the slot of an alignment push, which
/// kuna reads wrongly on main too, so only gcc's is checked.
#[test]
fn a_dwarf_char_parameter_keeps_its_declared_sign() {
    let compilers = host_compilers();
    let formats: Vec<&str> = CHARSIGN_FORMATS.iter().map(|c| c.0).collect();
    let all: Vec<&str> = formats.iter().copied().chain(CHARSIGN_VALUES.iter().map(|c| c.0)).collect();
    let widened = ["wid", "wids", "wus", "wss", "wb"];
    let run = |fixture: &str, extra: &[&str]| {
        let path = common::fixture(fixture);
        let mut args = vec!["decompile-all", path.as_str()];
        args.extend_from_slice(extra);
        let (text, err, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{fixture}: {err}");
        text
    };

    let gcc = run("charsign_x86_64_gcc_O2", &[]);
    charsign_round_trip("charsign-gcc", &gcc, &all, &compilers).unwrap_or_else(|e| panic!("gcc -O2: {e}\n{gcc}"));
    assert!(function(&gcc, "seq").contains("c == -0x38"), "a signed char byte above 0x7f is a number:\n{gcc}");
    let rust = run("charsign_x86_64_gcc_O2", &["--language", "rust", "--functions", "wid,wids"]);
    for want in ["fn wid(mut c: u8) -> i64", "fn wids(mut c: i8) -> i64", "c as i64"] {
        assert!(rust.contains(want), "Rust output has no `{want}`:\n{rust}");
    }

    let clang = run("charsign_x86_64_clang_O2", &[]);
    let all: Vec<&str> = all.into_iter().filter(|n| *n != "getc1").collect();
    let by_default: Vec<&str> = all.iter().copied().filter(|n| !widened.contains(n)).collect();
    charsign_round_trip("charsign-clang", &clang, &by_default, &compilers)
        .unwrap_or_else(|e| panic!("clang -O2: {e}\n{clang}"));
    let compiler = run("charsign_x86_64_clang_O2", &["--option", "narrowext", "compiler"]);
    charsign_round_trip("charsign-clang-compiler", &compiler, &all, &compilers)
        .unwrap_or_else(|e| panic!("clang -O2 under compiler: {e}\n{compiler}"));
    let off = run("charsign_x86_64_clang_O2", &["--option", "narrowext", "off"]);
    for n in widened {
        assert!(function(&off, n).contains("CONCAT"), "{n} under off:\n{off}");
    }

    let arm = run("charsign_aarch64_O2.o", &[]);
    for want in [
        "int fmt(char *b,char *f,unsigned char c)",
        "int fmts(char *b,char *f,signed char c)",
        "int fmt8(char *b,char *f,unsigned char c)",
        "int fmtc(char *b,char *f,char c)",
        "unsigned long ulen(char *s)",
    ] {
        assert!(arm.contains(want), "AArch64 has no `{want}`:\n{arm}");
    }
}
