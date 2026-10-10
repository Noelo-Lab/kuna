//! Recompile stores whose pointers resolve after their memory SSA guards exist.
use crate::common;
use common::process;
use std::collections::BTreeSet;
use std::process::Command;

const FUNCTIONS: &str = "s7,s_repeat,s_alias,s_read,s_branch,s_dead,s_two";
const WANT: &str = "s7 1 92 4 9\ns_repeat 94 94 4 3 7 11\ns_alias 92 92 4 13 9\ns_other 13 92 4 81 9\ns_read 81 92 4 9\ns_branch 1 92 4 9\ns_else 1 68 0 17\ns_dead 92 92 9\ns_two 64 64 4 21\ns_distinct 92 92 4 9\n";

#[test]
fn late_resolved_stores_remain_visible_to_calls_and_aliasing_loads() {
    let source = common::fixture("storecopyeffects.c");
    let compilers: Vec<&str> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    for fixture in [
        "storecopyeffects_clang_O0_aarch64.o",
        "storecopyeffects_clang_O2_aarch64.o",
        "storecopyeffects_gcc_O0_x86_64.o",
        "storecopyeffects_gcc_O2_x86_64.o",
    ] {
        let (printed, stderr, code) = common::run_kuna(&[
            "decompile-all",
            &common::fixture(fixture),
            "--functions",
            FUNCTIONS,
        ]);
        assert_eq!(code, 0, "{fixture}: {stderr}");
        let globals: BTreeSet<&str> = printed
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .filter(|s| s.starts_with("dat_"))
            .collect();
        assert_eq!(
            globals.len(),
            1,
            "{fixture}: expected one global:\n{printed}"
        );
        let preamble = format!(
            "extern int gi;\nvoid touch(void);\n#define {} gi\n",
            globals.first().unwrap()
        );
        let src = common::scratch_file("storecopyeffects", "c");
        let exe = common::scratch_file("storecopyeffects", "exe");
        std::fs::write(&src, format!("{preamble}{printed}")).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let compile = Command::new(cc)
                    .args([
                        "-std=gnu11",
                        "-w",
                        level,
                        "-fno-strict-aliasing",
                        "-DSTORECOPY_HARNESS",
                        "-DSTORECOPY_DRIVER",
                    ])
                    .args(common::CC_GCC15_DEMOTE)
                    .arg(&src)
                    .arg(&source)
                    .arg("-o")
                    .arg(&exe)
                    .output()
                    .expect("compile the printed C");
                assert!(
                    compile.status.success(),
                    "{fixture} {cc} {level}:\n{}\n{printed}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                let run = process::required_output(&mut Command::new(&exe));
                assert_eq!(
                    String::from_utf8_lossy(&run.stdout),
                    WANT,
                    "{fixture} {cc} {level}: memory writes differ from the source:\n{printed}"
                );
            }
        }
        let _ = std::fs::remove_file(src);
        let _ = std::fs::remove_file(exe);
    }
}
