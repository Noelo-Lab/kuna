//! A table of pointers whose first entry's bytes happen to be printable is
//! still the table: the printed C indexes, passes and stores its address, never
//! a short string literal.
mod common;
use common::process;
use std::process::Command;

const FUNCS: &str = "call,pass,keep";

const DECLS: &str = "extern char tbl[];\nvoid pick();\n";

const WANT: &str = "call 0 11\npass 0 11\ncall 1 22\npass 1 22\ncall 2 33\npass 2 33\nkeep 1\n";

/// `ptrslot.c` holds a table of three function pointers whose first entry is
/// stored as `26 42 40 00 ..` (gcc, `"&B@"`) or `30 42 40 00 ..` (clang,
/// `"0B@"`). `call` indexes the table, `pass` hands its address to `pick` and
/// `keep` stores it, which is the shape of a constructor storing a vtable. The
/// large-model build reaches the table through `movabs`. Each build's printed
/// functions, compiled by gcc and clang at -O0 and -O2 against the fixture's
/// harness, must do what the binary does. The printed indirect call is spelled
/// `(**(void **)p)()`, which does not compile (#869), so that one spelling is
/// rewritten to call through a function pointer before compiling.
#[test]
fn a_pointer_table_with_printable_entries_round_trips() {
    let harness = common::fixture("ptrslot.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in [
        "ptrslot_gcc_O1_x86_64",
        "ptrslot_gcc_O1_large_x86_64",
        "ptrslot_clang_O1_x86_64",
    ] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        assert!(
            !printed.contains('"'),
            "{fixture}: the table prints as a string literal:\n{printed}"
        );
        let callable = printed.replace("(**(void **)", "(**(void (**)(void))");
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{callable}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-fno-strict-aliasing", "-DPTRSLOT_HARNESS"])
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

/// The i386 build of `ptrslot.c` is linked at 0x21414140, so all four bytes of
/// every table entry are printable (`@AA!KAA!VAA!` and a NULL entry). The
/// string pass's own scan finds that run, not just the scalar-operand pass.
#[test]
fn a_pointer_table_at_a_printable_base_is_not_a_string() {
    let fixture = "ptrslot_gcc_O1_i386";
    let (printed, stderr, code) =
        common::run_kuna(&["decompile-all", &common::fixture(fixture), "--functions", FUNCS]);
    assert_eq!(code, 0, "{fixture}: {stderr}");
    assert!(!printed.contains('"'), "{fixture}: the table prints as a string literal:\n{printed}");
    for want in ["&tbl[a0 * 4]", "pick(tbl,a0);", "*a0 = tbl;"] {
        assert!(printed.contains(want), "{fixture}: missing `{want}`:\n{printed}");
    }
}
