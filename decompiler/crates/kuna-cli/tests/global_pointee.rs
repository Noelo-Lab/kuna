//! A pointer the binary stores to a global and keeps using from its register
//! is dereferenced through its own variable, so the printed C means the same
//! whatever type the global is declared with.
mod common;
use common::process;
use std::process::Command;

const FUNCS: &str = "p_int,r_short,r_char,r_rec,r_long,r_void,w_long,w_short,m_call,m_join,m_write,m_loop,g_reread,g_alias";

const DECLS: &str = "struct rec { int a; short b; long c; };\nextern char *gc;\nextern short *gs;\nextern int *gi;\n\
                     extern long *gl;\nextern struct rec *gr;\nextern void *gv;\nvoid touch(void);\n";

const WANT: &str =
    "p_int 51 8\nr_short 20 6\nr_char 72 4\nr_rec 199986 16\nr_long 7000021 16\nr_void 179 20\n\
                    w_long 3000009 -9 -8 24\nw_short 1 21 42 4\nm_call 65 12 1\nm_join 44 4 0 4 3\n\
                    m_write 29 77 78 16 4\nm_loop 221 32\ng_reread 37 4\ng_alias 22 8 0 8 0\n";

/// `globalpointee_x86_64.c` stores `int *`, `short *`, `char *`, `long *` and
/// record pointers to `char *`, `short *`, `int *`, `long *`, `struct rec *` and
/// `void *` globals and reads or writes through the value, around a call, a
/// branch and a loop.  Each build's printed functions, compiled by gcc and clang
/// at -O0 and -O2 against the fixture's own `main` with every global declared
/// as in the source, must print what the binary prints.  Only `g_reread` and
/// `g_alias`, which load the global back, may dereference a global; `g_alias`
/// is called with a pointer to `gi` itself, so its load must stay a load.
#[test]
fn a_pointer_stored_to_a_global_is_not_dereferenced_through_the_global() {
    let harness = common::fixture("globalpointee_x86_64.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in [
        "globalpointee_gcc_O0_x86_64",
        "globalpointee_gcc_O2_x86_64",
        "globalpointee_clang_O2_x86_64",
    ] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        for body in printed
            .split("// Function: ")
            .filter(|b| !b.starts_with("g_"))
        {
            let through_global = ["gc", "gs", "gi", "gl", "gr", "gv"]
                .iter()
                .any(|g| dereferences(body, g));
            assert!(
                !through_global,
                "{fixture}: a value stored to a global is read back through it:\n{body}"
            );
        }
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args([
                        "-std=gnu11",
                        "-w",
                        level,
                        "-fno-strict-aliasing",
                        "-DGLOBALPOINTEE_HARNESS",
                    ])
                    .args([
                        "-Wno-error=int-conversion",
                        "-Wno-error=incompatible-pointer-types",
                    ])
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
                    "{fixture} built by {cc} {level} computes a different value than the binary:\n{printed}"
                );
            }
        }
        let _ = std::fs::remove_file(src);
        let _ = std::fs::remove_file(exe);
    }
}

/// Does `body` index, dereference or offset the global `name`?
fn dereferences(body: &str, name: &str) -> bool {
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    body.match_indices(name).any(|(at, _)| {
        let before = body[..at].chars().next_back();
        let after = &body[at + name.len()..];
        if before.is_some_and(ident) || after.starts_with(ident) {
            return false;
        }
        before == Some('*')
            || after.starts_with('[')
            || after.starts_with("->")
            || after.starts_with(" +")
    })
}
