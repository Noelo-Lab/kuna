//! A 4-byte wide literal (`wchar_t` on ELF, `char32_t`) passed to the image's
//! own function prints as `L"..."` (`widestrings32`, carried by the aggressive
//! preset small images default to), while an `int` table of character codes
//! keeps its name.
mod common;
use common::process;
use std::process::Command;

const FUNCS: &str = "wide,wide32,ornull,table,weekly";

const DECLS: &str = "#include <stddef.h>\nextern long out;\nextern const int codes[], weeks[];\n\
                     long hashw();\nlong hash32();\nlong sum();\n";

const WANT: &str = "wide 9286557868\nwide32 1023670406297060324\nornull0 7432334113\nornull1 298771414\n\
                    table 0 0\nweekly 0 0\ntable 1 72\nweekly 1 52\ntable 2 173\nweekly 2 105\n\
                    table 3 281\nweekly 3 157\ntable 4 389\nweekly 4 209\ntable 5 500\nweekly 5 261\n\
                    table 6 500\nweekly 6 314\n";

/// `widestr32.c` passes `L"hellow"`, `U"char32-text"` and `L"(NULL)"` to its own
/// hash functions and `codes` (`int {72, 101, 108, 108, 111, 0}`, "Hello") and
/// `weeks` (`int {52, 53, ..}`) to `sum`. Each x86-64 build's printed functions,
/// compiled by gcc and clang at -O0 and -O2 against the fixture's harness, must
/// do what the binary does.
#[test]
fn wide_literals_round_trip_beside_int_tables() {
    let harness = common::fixture("widestr32.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in ["widestr32_gcc_O2_x86_64", "widestr32_clang_O2_x86_64"] {
        let (printed, stderr, code) =
            common::run_kuna(&["decompile-all", &common::fixture(fixture), "--functions", FUNCS]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for want in ["hashw(L\"hellow\")", "hash32(L\"char32-text\")", "= L\"(NULL)\";", "sum(codes,", "sum(weeks,"] {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-DWIDESTR32_HARNESS"])
                    .arg("-o")
                    .arg(&exe)
                    .arg(&src)
                    .arg(&harness)
                    .output()
                    .expect("spawn the C compiler");
                assert!(
                    out.status.success(),
                    "{cc} {level} rejected the printed C ({fixture}):\n{}\n{printed}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} built by {cc} {level} does not do what the binary does:\n{printed}"
                );
            }
        }
        let _ = std::fs::remove_file(src);
        let _ = std::fs::remove_file(exe);
    }
}

/// With the option off the literal prints as the address it was before.
#[test]
fn option_off_prints_the_address() {
    let fixture = common::fixture("widestr32_gcc_O2_x86_64");
    let (printed, stderr, code) =
        common::run_kuna(&["decompile-all", &fixture, "--functions", "wide", "--option", "widestrings32", "off"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(printed.contains("hashw(0x402004)"), "{printed}");
    assert!(!printed.contains("L\"hellow\""), "{printed}");
}

/// Stripped x86-64 builds (an operand points at each literal), relocatable
/// AArch64 and ARM objects (their `.rodata.str4.4`) and a big-endian MIPS image
/// (its symbol table) spell every literal, the tail `L"bind"` of `L"xbind"`
/// included, and never read `weeks` as text.
#[test]
fn other_builds_and_targets_spell_the_literals() {
    for fixture in [
        "widestr32_gcc_O2_stripped_x86_64",
        "widestr32_clang_O2_stripped_x86_64",
        "widestr32_aarch64_O2.o",
        "widestr32_arm32_O2.o",
        "widestr32_mips32_be_O2",
    ] {
        let (printed, stderr, code) = common::run_kuna(&["decompile-all", &common::fixture(fixture)]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for want in ["(L\"hellow\")", "(L\"char32-text\")", "L\"(NULL)\"", "(L\"xbind\")", "(L\"bind\")"] {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
        assert!(!printed.contains("L\"4544"), "{fixture}: weeks prints as text:\n{printed}");
    }
}
