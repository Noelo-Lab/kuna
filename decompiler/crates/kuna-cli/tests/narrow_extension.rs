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
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let src = common::scratch_file(&format!("narrowext-{}-{cc}{level}", image.name), "c");
            let exe = src.with_extension("exe");
            std::fs::write(&src, &program).unwrap();
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

#[test]
fn a_narrow_argument_or_return_round_trips_through_the_printed_c() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
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
