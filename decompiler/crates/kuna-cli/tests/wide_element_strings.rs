//! An array of `int`, `short` or `wchar_t` character codes whose first element
//! reads as a one-character string is still the array: the printed C passes
//! its address, never a `"A"` literal, while genuine one-character literals laid
//! out side by side keep printing as literals.
use crate::common;
use common::process;
use std::process::Command;

const FUNCS: &str = "go,go16,gomixed,narrow,opened,wide";

const DECLS: &str = "extern int out;\nextern const int codes[], mixed[];\nextern const short halves[];\n\
                     int sum();\nint sum16();\nint pair();\nlong mode();\n";

const WANT: &str = "go 0 0\ngo 1 65\ngo 2 131\ngo 3 198\ngo 4 266\n\
                    go16 0 0\ngo16 1 104\ngo16 2 209\ngo16 3 315\n\
                    gomixed 0 0\ngomixed 1 65\ngomixed 2 1065\n\
                    narrow 120121\nopened 0 120119\nopened 1 120097\n";

/// `widecodes.c` passes `codes` (`int {65, 66, 67, 68}`), `halves`
/// (`short {104, 105, 106, 0}`), `mixed` (`int {65, 1000}`) and `L"hellow"`
/// by address, and the literals `"x"` and `"y"`, which sit beside `"w"` and
/// `"a"`, reached only through a table of pointers. Each build's printed
/// functions, compiled by gcc and clang at -O0 and -O2 against the fixture's
/// harness, must do what the binary does; `wide` passes an address with no
/// symbol to declare, so it is only checked for the literal.
#[test]
fn character_code_arrays_round_trip_beside_one_character_literals() {
    let harness = common::fixture("widecodes.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in ["widecodes_gcc_O1_x86_64", "widecodes_clang_O2_x86_64"] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for literal in ["\"A\"", "\"h\""] {
            assert!(
                !printed.contains(literal),
                "{fixture}: an array prints as {literal}:\n{printed}"
            );
        }
        for want in ["pair(\"x\",\"y\")", "pair(\"x\",v"] {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
        let wide = printed.find("// Function: wide @").expect("wide is printed");
        let compiled = &printed[..wide];
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{compiled}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-DWIDECODES_HARNESS"])
                    .args(["-Wno-error=int-conversion", "-Wno-error=incompatible-pointer-types"])
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

/// Stripped, nothing bounds `codes`, `halves` or `L"hellow"` but their bytes,
/// two or more character codes at their own width, which is enough; `mixed`'s
/// second element is no character, so only its symbol told it apart.
#[test]
fn stripped_character_code_arrays_are_not_one_character_strings() {
    for fixture in ["widecodes_gcc_O1_stripped_x86_64", "widecodes_clang_O2_stripped_x86_64"] {
        let (printed, stderr, code) =
            common::run_kuna(&["decompile-all", &common::fixture(fixture)]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        assert!(!printed.contains("\"h\""), "{fixture}: an array prints as \"h\":\n{printed}");
        assert_eq!(
            printed.matches("(\"A\",").count(),
            1,
            "{fixture}: only `mixed` may still print as \"A\":\n{printed}"
        );
        for want in ["(\"x\",\"y\")", "(\"x\",v"] {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
    }
}

/// A one-character literal laid beside another that only clang's lookup table
/// of 32-bit relative offsets reaches (`widecodes_switch.c`, "a" then "b"), and
/// one that ends gcc's merged string block right before an `int` table
/// (`widecodes_tail.c`, `41 00 00 00 42 00 00 00`), keep their literals.
#[test]
fn literals_beside_a_lookup_table_or_an_int_table_keep_their_strings() {
    for (fixture, funcs, want) in [
        ("widecodes_switch_clang_O2_x86_64", "usea,usee", ["settag(\"a\")", "settag(\"e\")"]),
        ("widecodes_tail_gcc_O2_x86_64", "fa,get", ["sink(\"A\")", "&t[a0 * 4L]"]),
    ] {
        let (printed, stderr, code) =
            common::run_kuna(&["decompile-all", &common::fixture(fixture), "--functions", funcs]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for want in want {
            assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
        }
    }
}

/// Ints that read as a table of offsets do not keep a wide string's first
/// character: `widecodes_rec.c` puts `L"hellow"` after the ints 8 and 12, which
/// as offsets from the struct point at its "h" and "e", and in
/// `widecodes_overread_*.c` reading past clang's lookup table takes `L"Pest"`'s
/// 'P' for an offset onto `L"word"`'s second unit.
#[test]
fn ints_shaped_like_offsets_do_not_keep_a_wide_strings_first_character() {
    for (fixture, funcs) in [
        ("widecodes_rec_gcc_O2_x86_64", Some("usename,userec")),
        ("widecodes_overread_clang_O2_stripped_x86_64", None),
    ] {
        let path = common::fixture(fixture);
        let mut args = vec!["decompile-all", path.as_str()];
        if let Some(funcs) = funcs {
            args.extend(["--functions", funcs]);
        }
        let (printed, stderr, code) = common::run_kuna(&args);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for literal in ["(\"h\")", "(\"P\")", "(\"w\")"] {
            assert!(
                !printed.contains(literal),
                "{fixture}: a wide string prints as {literal}:\n{printed}"
            );
        }
    }
}
