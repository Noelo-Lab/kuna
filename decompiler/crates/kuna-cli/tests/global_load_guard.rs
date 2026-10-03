//! A store to a global before a load through a pointer that may point at it is
//! kept when the global is stored again after the load: in a loop, on a branch,
//! for byte, short and long globals, and for two globals at once; and the load
//! stays ahead of the stores the binary makes after it.
mod common;
use common::process;
use std::process::Command;

const ALL: [&str; 17] = [
    "d_loop", "d_branch", "d_short", "d_byte", "d_long", "d_index", "d_two", "d_iter", "d_pre", "d_cond",
    "d_phi", "d_inc", "d_while", "d_sel", "d_load", "d_twice", "d_mv",
];

/// The functions each build's printed C must get right.  gcc -O2's `d_iter`
/// joins its loop counter with the global it stores (#871), so the harness
/// compiles the source's own copy of it.
const BUILDS: [(&str, &[&str]); 3] = [
    ("globalloadguard_gcc_O0_x86_64", &ALL),
    (
        "globalloadguard_gcc_O2_x86_64",
        &[
            "d_loop", "d_branch", "d_short", "d_byte", "d_long", "d_index", "d_two", "d_pre", "d_cond", "d_phi",
            "d_inc", "d_while", "d_sel", "d_load", "d_twice", "d_mv",
        ],
    ),
    ("globalloadguard_clang_O2_x86_64", &ALL),
];

const DECLS: &str = "extern int gi;\nextern int gj;\nextern short gs;\nextern unsigned char gb;\nextern long gl;\n\
                     void touch(void);\n";

const WANT: &str = "d_loop 15 4\nd_branch 5 5 4\nd_short 42 9\nd_byte 41000 9\nd_long 8999999994000000000 9\n\
                    d_index 6 9\nd_two 60 2 2\nd_iter 18 -1\nd_pre 18 8\nd_cond 42 8 0\nd_phi 5 5\nd_inc 12 5\n\
                    d_while 6 100\nd_sel 63 3 -1\nd_load 7 9\nd_twice 70 9\nd_mv 5 6 0\n";

/// `globalloadguard_x86_64.c` stores to a global, loads through a pointer and
/// stores to the global again, and its own `main` aims every pointer at the
/// global.  Each build's printed functions, compiled by gcc and clang at -O0
/// and -O2 with the globals declared as in the source, must print what the
/// binary prints.
#[test]
fn a_store_before_a_pointer_load_is_kept_when_the_global_is_stored_again() {
    let harness = common::fixture("globalloadguard_x86_64.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for (fixture, funcs) in BUILDS {
        let (printed, stderr, code) =
            common::run_kuna(&["decompile-all", &common::fixture(fixture), "--functions", &funcs.join(",")]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let keep: Vec<String> =
            ALL.iter().filter(|f| !funcs.contains(f)).map(|f| format!("-DKEEP_{f}")).collect();
        let src = common::scratch_file(fixture, "c");
        let exe = common::scratch_file(fixture, "exe");
        std::fs::write(&src, format!("{DECLS}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let out = Command::new(cc)
                    .args(["-std=gnu11", "-w", level, "-fno-strict-aliasing", "-DGLOBALLOADGUARD_HARNESS"])
                    .args(&keep)
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
