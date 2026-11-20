#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;
use common::process;
use std::process::Command;

/// The exact 200-store performance fixture must also preserve reloads when
/// publishing through pp changes the global pointer itself.
#[test]
fn repeated_global_pointer_stores_preserve_aliasing_and_early_returns() {
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native pointer-store round trip: no GCC or Clang");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let generated = process::required_output(
        Command::new("python3").arg(common::fixture("globalptrstores_x86_64.py"))
    );
    let original = root.path().join("original.c");
    std::fs::write(&original, generated.stdout).unwrap();
    let (body, stderr, status) = common::run_kuna(&[
        "decompile", &common::fixture("globalptrstores_x86_64.o"), "big",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let emitted = root.path().join("emitted.c");
    std::fs::write(&emitted, format!("extern int *gi;\n{body}")).unwrap();
    let globals = root.path().join("globals.c");
    std::fs::write(&globals, "int *gi;\n").unwrap();
    let driver = root.path().join("driver.c");
    std::fs::write(&driver, r#"
#include <stdio.h>
#include <stdint.h>
extern int *gi;
int big(int *, int, int **);
int main(void) {
    for (unsigned seed = 0; seed < 64; ++seed) {
        for (int k = 0; k < 10; ++k) {
            for (int alias = 0; alias < 2; ++alias) {
                int data[64];
                uint32_t state = seed + 1;
                for (int i = 0; i < 64; ++i) {
                    state = state * 1664525u + 1013904223u;
                    data[i] = seed < 3 ? (int[]){0, 1, 10}[seed] : (int)(state % 21) - 10;
                }
                gi = data + 8;
                int *other = data + 9;
                int result = big(data + 16, k, alias ? &gi : &other);
                printf("%d %ld %ld\n", result, (long)(gi - data), (long)(other - data));
            }
        }
    }
    return 0;
}
"#).unwrap();
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let mut observations = Vec::new();
            for (name, implementation) in [("original", &original), ("emitted", &emitted)] {
                let executable = root.path().join(format!("{name}-{cc}-{level}"));
                let mut command = Command::new(cc);
                command.args(["-std=gnu11", "-w", "-fno-strict-aliasing", level])
                    .args(common::CC_GCC15_DEMOTE).arg(implementation).arg(&driver);
                if name == "emitted" {
                    command.arg(&globals);
                }
                process::required_output(command.arg("-o").arg(&executable));
                observations.push(process::required_output(&mut Command::new(executable)).stdout);
            }
            assert_eq!(observations[0], observations[1], "{cc} {level}: aliased global reload or early return changed");
        }
    }
}
