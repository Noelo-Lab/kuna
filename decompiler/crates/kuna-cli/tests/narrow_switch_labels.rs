//! Switch labels name values the printed selector can hold. A compiled jump
//! table over a `short`, `signed char`, `char`, `int` and `unsigned short`
//! selector, each with cases at or above the narrow sign bit, round-trips
//! through the printed C and the printed Rust: rustc rejects a `match` pattern
//! outside the scrutinee's type, and a C label above a signed selector's range
//! never matches after integer promotion.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use std::path::Path;
use std::process::Command;

const SOURCE: &str = r#"
volatile int sink;
#define PICK(T, NAME, A, B, C, D, E, F)                  \
  int NAME(T s) {                                        \
    switch (s) {                                         \
    case A: sink = 1; return 11;                         \
    case B: sink = 2; return 22;                         \
    case C: sink = 3; return 33;                         \
    case D: sink = 4; return 44;                         \
    case E: sink = 5; return 55;                         \
    case F: sink = 6; return 66;                         \
    default: sink = 7; return 77;                        \
    }                                                    \
  }
PICK(short, pick16, -13, -12, -11, -10, -9, -7)
PICK(signed char, pick8, -13, -12, -11, -10, -9, -7)
PICK(char, pickc, -13, -12, -11, -10, -9, -7)
PICK(int, pick32, -13, -12, -11, -10, -9, -7)
PICK(unsigned short, picku16, 0xfff3, 0xfff4, 0xfff5, 0xfff6, 0xfff7, 0xfff9)
"#;

const PROTOTYPES: &[(&str, &str)] = &[
    ("pick16", "short"),
    ("pick8", "signed char"),
    ("pickc", "char"),
    ("pick32", "int"),
    ("picku16", "unsigned short"),
];

const C_MAIN: &str = r#"
int native_pick16(short), native_pick8(signed char), native_pickc(char);
int native_pick32(int), native_picku16(unsigned short);
int main(void) {
    for (unsigned v = 0; v <= 0xffff; ++v) {
        int want[5] = {native_pick16((short)v), native_pick8((signed char)v),
                       native_pickc((char)v), native_pick32((short)v),
                       native_picku16((unsigned short)v)};
        int got[5] = {pick16((short)v), pick8((signed char)v), pickc((char)v),
                      pick32((short)v), picku16((unsigned short)v)};
        for (int k = 0; k < 5; ++k)
            if (want[k] != got[k]) return 1 + k;
    }
    return 0;
}
"#;

const RUST_HARNESS: &str = r#"
#![allow(non_upper_case_globals)]
#[no_mangle]
static mut sink: i32 = 0;
extern "C" {
    fn native_pick16(s: i16) -> i32;
    fn native_pick8(s: i8) -> i32;
    fn native_pickc(s: i8) -> i32;
    fn native_pick32(s: i32) -> i32;
    fn native_picku16(s: u16) -> i32;
}
fn main() { unsafe {
    for v in 0..=0xffffu32 {
        let w = v as u16;
        assert_eq!(native_pick16(w as i16), pick16(w as i16), "pick16 {v:#x}");
        assert_eq!(native_pick8(w as u8 as i8), pick8(w as u8 as i8), "pick8 {v:#x}");
        assert_eq!(native_pickc(w as u8 as i8), pickc(w as u8), "pickc {v:#x}");
        assert_eq!(native_pick32(w as i16 as i32), pick32(w as i16 as i32), "pick32 {v:#x}");
        assert_eq!(native_picku16(w), picku16(w), "picku16 {v:#x}");
    }
} }
"#;

fn decompile(binary: &Path, language: &str) -> String {
    let functions: Vec<_> = PROTOTYPES.iter().map(|(name, _)| *name).collect();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.arg("decompile-all")
        .arg(binary)
        .args(["--functions", &functions.join(","), "--language", language, "--assert-strict"]);
    for (name, selector) in PROTOTYPES {
        cmd.args(["--assert", &format!("prototype {name} int {name}({selector} s)")]);
    }
    let out = process::required_output(&mut cmd);
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn narrow_signed_switch_labels_round_trip_through_c_and_rust() {
    let cc = ["gcc", "clang"]
        .into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .expect("a C compiler is required for the native oracle");
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("pick.c");
    let binary = dir.path().join("pick.elf");
    let native = dir.path().join("native.o");
    std::fs::write(&source, SOURCE).unwrap();
    process::required_output(
        Command::new(cc)
            .args(["-O2", "-fno-pic", "-fno-asynchronous-unwind-tables", "-fcf-protection=none"])
            .args(["-nostdlib", "-no-pie", "-Wl,--build-id=none", "-Wl,-e,pick16"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let mut renames: Vec<String> =
        PROTOTYPES.iter().map(|(name, _)| format!("-D{name}=native_{name}")).collect();
    renames.push("-Dsink=native_sink".into());
    process::required_output(
        Command::new(cc).args(["-O2", "-c"]).args(&renames).arg(&source).arg("-o").arg(&native),
    );

    let printed = decompile(&binary, "c");
    let c_source = dir.path().join("printed.c");
    let c_exe = dir.path().join("printed-c");
    std::fs::write(&c_source, format!("volatile int sink;\n{printed}\n{C_MAIN}")).unwrap();
    process::required_output(
        Command::new(cc)
            .args(["-O0", "-w", "-no-pie"])
            .args(common::CC_GCC15_DEMOTE)
            .arg(&c_source)
            .arg(&native)
            .arg("-o")
            .arg(&c_exe),
    );
    let status = Command::new(&c_exe).status().unwrap();
    assert!(status.success(), "printed C diverges ({status}):\n{printed}");

    let rust = decompile(&binary, "rust");
    let rust_source = dir.path().join("printed.rs");
    let rust_exe = dir.path().join("printed-rs");
    std::fs::write(&rust_source, format!("{RUST_HARNESS}\n{rust}")).unwrap();
    let compile = Command::new("rustc")
        .args(["--crate-name", "narrow_switch_labels", "--edition", "2021", "-O"])
        .arg(&rust_source)
        .arg("-C")
        .arg(format!("link-arg={}", native.display()))
        .arg("-o")
        .arg(&rust_exe)
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "rustc: {}\n{rust}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let out = Command::new(&rust_exe).output().unwrap();
    assert!(
        out.status.success(),
        "printed Rust diverges: {}\n{rust}",
        String::from_utf8_lossy(&out.stderr)
    );
}
