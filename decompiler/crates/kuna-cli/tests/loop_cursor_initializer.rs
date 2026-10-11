#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use common::process;
use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn loop_cursor_preserves_empty_branches_and_live_callback_uses() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    let Some(cc) = compilers.first() else { return };
    let dir = common::scratch_file("loop-cursor", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let native = dir.join("native");
    process::required_output(
        Command::new(cc)
            .arg(fixture("loop_cursor.S"))
            .arg(fixture("loop_cursor_harness.c"))
            .arg("-o")
            .arg(&native),
    );
    let expected = process::required_output(&mut Command::new(&native)).stdout;
    let specs = common::repo_root().join("specs");
    for for_loops in ["on", "off"] {
        let (printed, stderr, status) = common::run_kuna(&[
            "decompile",
            native.to_str().unwrap(),
            "cursor_walk",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--option",
            "analyzeforloops",
            for_loops,
            "--assert",
            "typedef struct Node { int value; };",
            "--assert",
            "typedef struct Items { unsigned int count; unsigned int pad; unsigned long reserved; unsigned long *items; };",
            "--assert",
            "prototype cursor_walk void cursor_walk(Items *array, int branch, void (*callback)(Node *item))",
        ]);
        assert_eq!(status, 0, "{stderr}");
        let source = dir.join("printed.c");
        std::fs::write(&source, format!(
            "#include <stddef.h>\n#include <stdbool.h>\n#include <stdint.h>\ntypedef struct Node {{ int value; }} Node;\ntypedef struct Items {{ unsigned int count; unsigned int pad; unsigned long reserved; unsigned long *items; }} Items;\n{printed}"
        )).unwrap();
        for cc in &compilers {
            for optimization in ["-O0", "-O2"] {
                let rebuilt = dir.join(format!("printed-{cc}-{optimization}"));
                process::required_output(
                    Command::new(cc)
                        .arg(optimization)
                        .arg(&source)
                        .arg(fixture("loop_cursor_harness.c"))
                        .arg("-o")
                        .arg(&rebuilt),
                );
                let actual = process::required_output(&mut Command::new(&rebuilt)).stdout;
                assert_eq!(
                    String::from_utf8_lossy(&actual),
                    String::from_utf8_lossy(&expected),
                    "{for_loops}, {cc}, {optimization}:\n{printed}"
                );
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
