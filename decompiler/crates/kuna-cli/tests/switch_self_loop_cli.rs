//! A switch that is its own loop header keeps every iteration, whether or not
//! some of its cases leave the loop or restart an enclosing one, and when it
//! is itself a case of another switch.
use crate::common;
use common::process;

use std::process::Command;

const FUNCTIONS: [&str; 6] = ["self_loop", "self_loop_exit", "mixed_loop", "direct_mixed", "direct_loop", "nested_switch"];
const OBJECTS: [&str; 2] = ["switch_self_loop_gcc_x86_64.o", "switch_self_loop_clang_x86_64.o"];

fn kuna() -> Command {
    Command::new(
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
    )
}

fn decompile(object: &str, function: &str) -> String {
    let mut command = kuna();
    command
        .args(["decompile", &common::fixture(object), function, "--sleighpath"])
        .arg(common::repo_root().join("specs"))
        .args(["--assert", &format!("prototype {function} void {function}(int x)")]);
    String::from_utf8(process::required_output(&mut command).stdout).unwrap()
}

/// Each row feeds selectors until a default (7) or a leaving case returns;
/// the driver prints how many selectors were read and which callees ran.
fn driver(function: &str) -> String {
    format!(
        r#"#include <stdio.h>
void {function}(int x);
static const int rows[][12] = {{
  {{0, 3, 3, 5, 1, 0, 1, 4, 7, 7, 7, 7}}, {{2, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7}},
  {{4, 0, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7}}, {{5, 5, 5, 3, 1, 7, 7, 7, 7, 7, 7, 7}},
  {{7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7}}, {{1, 3, 0, 5, 2, 0, 7, 7, 7, 7, 7, 7}},
  {{-1, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7}}, {{3, 1, 1, 0, 4, 2, 7, 7, 7, 7, 7, 7}},
  {{0, 6, 4, 0, 6, 2, 6, 7, 7, 7, 7, 7}},
}};
static int row, at, count, calls[64];
int next_selector(int x) {{ return rows[row][at++] + x; }}
void t0(void) {{ calls[count++] = 0; }}
void t1(void) {{ calls[count++] = 1; }}
void t2(void) {{ calls[count++] = 2; }}
void t3(void) {{ calls[count++] = 3; }}
void t4(void) {{ calls[count++] = 4; }}
void t5(void) {{ calls[count++] = 5; }}
int main(void) {{
  for (row = 0; row < 9; row++) {{
    at = count = 0;
    {function}(0);
    printf("%d:", at);
    for (int i = 0; i < count; i++) printf(" %d", calls[i]);
    printf("\n");
  }}
  return 0;
}}
"#
    )
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn compilers() -> Vec<&'static str> {
    let found: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!found.is_empty(), "the round trip requires a C compiler");
    found
}

/// Run `sources` linked into one executable and return its output.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn run(cc: &str, level: &str, sources: &[&std::path::Path]) -> String {
    let executable = common::scratch_file("switch-self-loop", "exe");
    let mut compile = Command::new(cc);
    compile
        .args(["-w", "-no-pie", level])
        .args(common::CC_GCC15_DEMOTE)
        .args(sources)
        .arg("-o")
        .arg(&executable);
    process::required_output(&mut compile);
    let output = process::output_with_timeout(
        &mut Command::new(&executable),
        std::time::Duration::from_secs(10),
        std::time::Duration::from_millis(10),
    )
    .expect("the printed loop terminates");
    assert!(output.status.success(), "{cc} {level}: {}", output.status);
    String::from_utf8(output.stdout).unwrap()
}

/// The emitted C and the original object agree on every row, compiled with
/// each available compiler at -O0 and -O2.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[test]
fn a_switch_that_heads_its_own_loop_keeps_every_iteration() {
    for object in OBJECTS {
        for function in FUNCTIONS {
            let printed = decompile(object, function);
            let source = common::scratch_file("switch-self-loop", "c");
            std::fs::write(
                &source,
                format!(
                    "#include <stdbool.h>\nextern int next_selector(int);\n\
                     extern void t0(void), t1(void), t2(void), t3(void), t4(void), t5(void);\n{}",
                    printed.lines().filter(|l| !l.starts_with("// ")).collect::<Vec<_>>().join("\n")
                ),
            )
            .unwrap();
            let harness = common::scratch_file("switch-self-loop-driver", "c");
            std::fs::write(&harness, driver(function)).unwrap();
            let original = std::path::PathBuf::from(common::fixture(object));
            for cc in compilers() {
                let want = run(cc, "-O0", &[&original, &harness]);
                assert_eq!(want.lines().count(), 9, "{want}");
                for level in ["-O0", "-O2"] {
                    assert_eq!(
                        run(cc, level, &[&source, &harness]),
                        want,
                        "{object} {function} {cc} {level}:\n{printed}"
                    );
                }
            }
        }
    }
}
