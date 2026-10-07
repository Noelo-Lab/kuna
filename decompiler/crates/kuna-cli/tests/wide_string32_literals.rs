//! A 4-byte wide literal (`wchar_t` on ELF, `char32_t`) passed to the image's
//! own function prints as `L"..."` (`widestrings32`, carried by the aggressive
//! preset small images default to), while an `int` table of character codes
//! keeps its name.
mod common;
use common::process;
use std::process::Command;

const FUNCS: &str = "wide,wide32,ornull,suffix,first,table,weekly";

const DECLS: &str = "#include <stddef.h>\nextern long out;\nextern const int codes[], weeks[];\n\
                     long hashw();\nlong hash32();\nlong sum();\n";

const WANT: &str = "wide 9286557868\nwide32 1023670406297060324\nornull0 7432334113\nornull1 298771414\n\
                    suffix 952240110\nfirst 275065641774781\ntable 0 0\nweekly 0 0\ntable 1 72\nweekly 1 52\ntable 2 173\nweekly 2 105\n\
                    table 3 281\nweekly 3 157\ntable 4 389\nweekly 4 209\ntable 5 500\nweekly 5 261\n\
                    table 6 500\nweekly 6 314\n";

/// `widestr32.c` passes `L"hellow"`, `U"char32-text"`, `L"(NULL)"`, `L"xbind"`
/// and its tail `L"bind"`, and `L"first-msg"` to its own hash functions and
/// `codes` (`int {72, 101, 108, 108, 111, 0}`, "Hello") and `weeks` (`int {52,
/// 53, ..}`) to `sum`. Each x86-64 build's printed functions, compiled by gcc and
/// clang at -O0 and -O2 against the fixture's harness, must do what the binary
/// does.
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
        for want in [
            "hashw(L\"hellow\")",
            "hash32(L\"char32-text\")",
            "= L\"(NULL)\";",
            "hashw(L\"xbind\")",
            "hashw(L\"bind\")",
            "hashw(L\"first-msg\")",
            "sum(codes,",
            "sum(weeks,",
        ] {
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

/// Stripped x86-64 builds (an operand points at each literal) and relocatable
/// AArch64 and ARM objects (their `.rodata.str4.4`) spell every literal, the
/// tail `L"bind"` of `L"xbind"` included, and the big-endian MIPS image the one
/// its pointer table `msgs` holds; none reads `weeks`, the rows of `rows` or the
/// switch table of `code` as text.
#[test]
fn other_builds_and_targets_spell_the_literals() {
    for fixture in [
        "widestr32_gcc_O2_x86_64",
        "widestr32_clang_O2_x86_64",
        "widestr32_gcc_O2_stripped_x86_64",
        "widestr32_clang_O2_stripped_x86_64",
        "widestr32_aarch64_O2.o",
        "widestr32_arm32_O2.o",
        "widestr32_mips32_be_O2",
    ] {
        let (printed, stderr, code) = common::run_kuna(&["decompile-all", &common::fixture(fixture)]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let wants: &[&str] = if fixture == "widestr32_mips32_be_O2" {
            &["(L\"first-msg\")"]
        } else {
            &["(L\"hellow\")", "(L\"char32-text\")", "L\"(NULL)\"", "(L\"xbind\")", "(L\"bind\")", "(L\"first-msg\")"]
        };
        for want in wants {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
        for text in ["L\"4544", "L\"helpz", "L\"alpha", "L\"bravo", "L\"qhellow"] {
            assert!(!printed.contains(text), "{fixture}: a table prints as `{text}`:\n{printed}");
        }
    }
}

/// `widestr32_tables.c`'s int tables of character codes run on past their zero,
/// and the code reads past it: passed whole to a function, indexed through a
/// pointer an -O0 build keeps in memory, indexed and passed from an element
/// inside, walked by a loop gcc peels, held by a struct. `widestr32_named.c`'s
/// go on with an element the code also names (`&tbl[6]`, a struct's count),
/// `widestr32_fields.c`'s with a struct field it names (a negative delta, a
/// string pointer, two negative bounds that read as offsets into the code).
/// None prints as a wide literal, by default or with `operand_refs` off;
/// `L"control"` does.
#[test]
fn tables_past_their_zero_never_print_as_text() {
    let alone: &[&str] = &["--mode", "reliable", "--option", "widestrings32", "on"];
    for fixture in [
        "widestr32_tables_gcc_O2_stripped",
        "widestr32_tables_gcc_O2_nopie_stripped",
        "widestr32_tables_gcc_O0_stripped",
        "widestr32_tables_clang_O2_stripped",
        "widestr32_named_gcc_O2_stripped",
        "widestr32_named_gcc_O0_stripped",
        "widestr32_named_clang_O2_nopie_stripped",
        "widestr32_fields_gcc_O2_nopie_stripped",
        "widestr32_fields_gcc_O2_stripped",
        "widestr32_fields_clang_O2_nopie_stripped",
        "widestr32_fields_clang_O2_stripped",
    ] {
        for extra in [&[][..], alone] {
            let path = common::fixture(fixture);
            let mut args = vec!["decompile-all", path.as_str()];
            args.extend_from_slice(extra);
            let (printed, stderr, code) = common::run_kuna(&args);
            assert_eq!(code, 0, "{fixture} {extra:?}: {stderr}");
            let wide = printed.matches("L\"").count();
            let control = printed.matches("(L\"control\")").count();
            assert!(control == 1 && wide == control, "{fixture} {extra:?}: a table prints as text:\n{printed}");
        }
    }
}
