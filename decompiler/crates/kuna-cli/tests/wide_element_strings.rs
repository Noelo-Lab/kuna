//! Arrays of `int`, `short` and `wchar_t` character codes keep their element
//! widths beside genuine one-character strings. Native consumers distinguish
//! a correct array/literal address from a shortened literal without pinning
//! the emitted argument's cast spelling.
use crate::common;
use common::process;
use object::{Object, ObjectSymbol};
use std::process::Command;

const FUNCS: &str = "go,go16,gomixed,narrow,opened,wide";

const DECLS: &str = "extern int out;\nextern const int codes[], mixed[];\nextern const short halves[];\n\
                     int sum(const int *, int);\nint sum16(const short *, int);\n\
                     int pair(const char *, const char *);\nconst char *mode(int);\n";

const WANT: &str = "go 0 0\ngo 1 65\ngo 2 131\ngo 3 198\ngo 4 266\n\
                    go16 0 0\ngo16 1 104\ngo16 2 209\ngo16 3 315\n\
                    gomixed 0 0\ngomixed 1 65\ngomixed 2 1065\n\
                    narrow 120121\nopened 0 120119\nopened 1 120097\n";

/// `widecodes.c` passes `codes` (`int {65, 66, 67, 68}`), `halves`
/// (`short {104, 105, 106, 0}`), `mixed` (`int {65, 1000}`) and `L"hellow"`
/// by address, and the literals `"x"` and `"y"`, which sit beside `"w"` and
/// `"a"`, reached only through a table of pointers. Each build's printed
/// functions, compiled by gcc and clang at -O0 and -O2 against the fixture's
/// actual callee interfaces and harness, must do what the binary does.
/// The stripped round trip below also exercises the complete wide string.
#[test]
fn character_code_arrays_round_trip_beside_one_character_literals() {
    let harness = common::fixture("widecodes.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    for fixture in ["widecodes_gcc_O1_x86_64", "widecodes_clang_O2_x86_64"] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let wide = printed.find("// Function: wide @").expect("wide is printed");
        let compiled = &printed[..wide];
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{compiled}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-DWIDECODES_HARNESS"])
                    .args(common::CC_GCC15_DEMOTE)
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

/// Stripped arrays and genuine literals must still work through the actual
/// emitted callees. The symbolful twin supplies entry/data addresses only;
/// inference runs on the stripped image, with its whole-program context.
/// `mixed` remains ambiguous without a symbol (its bytes also begin "A\0"),
/// so its semantic acceptance is covered by the symbolful round trip above,
/// not by counting one particular spelling of that ambiguous literal.
#[test]
fn stripped_character_code_arrays_are_not_one_character_strings() {
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "the round trip requires a C compiler");
    let source = include_str!("../../kuna-analysis/tests/fixtures/widecodes.c");
    for (fixture, twin) in [
        ("widecodes_gcc_O1_stripped_x86_64", "widecodes_gcc_O1_x86_64"),
        ("widecodes_clang_O2_stripped_x86_64", "widecodes_clang_O2_x86_64"),
    ] {
        let (printed, stderr, code) =
            common::run_kuna(&["decompile-all", &common::fixture(fixture)]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let bytes = std::fs::read(common::fixture(twin)).unwrap();
        let image = object::File::parse(bytes.as_slice()).unwrap();
        let address = |name: &str| image.symbols()
            .find(|symbol| symbol.name().ok() == Some(name))
            .unwrap_or_else(|| panic!("{twin} has no {name}")).address();
        let mut program = format!("#define main reference_main\n{source}\n#undef main\n");
        for (name, expression) in [("out", "out"), ("codes", "codes[0]"), ("halves", "halves[0]")] {
            program.push_str(&format!("#define dat_{:x} {expression}\n", address(name)));
        }
        // Keep support preambles and newlines intact, including any actual
        // record definitions or helper guards emitted with a selected body.
        program.push_str(printed.split("// Function:").next().unwrap());
        for name in ["sum", "sum16", "lenw", "pair", "go", "go16", "wide", "narrow"] {
            let marker = format!(" @ 0x{:x}\n", address(name));
            let start = printed.find(&marker)
                .and_then(|end| printed[..end].rfind("// Function:"))
                .unwrap_or_else(|| panic!("{fixture} has no emitted {name}:\n{printed}"));
            let rest = &printed[start + "// Function:".len()..];
            let end = rest.find("// Function:").map_or(printed.len(), |next| start + "// Function:".len() + next);
            let function = &printed[start..end];
            let emitted_name = rest.split_whitespace().next().unwrap();
            program.push_str(&format!("#define tested_{name} {emitted_name}\n"));
            program.push_str(function);
            program.push('\n');
        }
        program.push_str(r#"
int main(void) {
    for (int n = 0; n <= 4; ++n) {
        go(n); int expected = out; out = -1;
        tested_go(n); if (out != expected) return 1;
    }
    for (int n = 0; n <= 3; ++n) {
        go16(n); int expected = out; out = -1;
        tested_go16(n); if (out != expected) return 2;
    }
    wide(); int expected = out; out = -1;
    tested_wide(); if (out != expected || out != 6) return 3;
    narrow(); expected = out; out = -1;
    tested_narrow(); if (out != expected || out != 120121) return 4;
    return 0;
}
"#);
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, &program).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args(["-std=gnu11", "-w", level])
                    .args(common::CC_GCC15_DEMOTE)
                    .arg(&src).arg("-o").arg(&exe).output().unwrap();
                assert!(compile.status.success(),
                    "{fixture} {cc} {level} rejected stripped consumers:\n{}\n{program}",
                    String::from_utf8_lossy(&compile.stderr));
                let run = Command::new(&exe).output().unwrap();
                assert!(run.status.success(),
                    "{fixture} {cc} {level} changed array/literal behavior: {:?}\n{}\n{program}",
                    run.status.code(), String::from_utf8_lossy(&run.stderr));
            }
        }
        std::fs::remove_file(src).unwrap();
        std::fs::remove_file(exe).unwrap();
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
