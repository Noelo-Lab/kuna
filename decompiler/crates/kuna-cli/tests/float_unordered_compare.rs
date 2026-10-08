#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

mod common;

use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

const FUNCTIONS: &str = "above,at_least,not_above,not_at_least,pick,not_above_pair,above_two,nan_or_huge,nan_or_above,ordered_below,ordered_below_pair";

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn compile(cc: &str, level: &str, sources: &[&Path], executable: &Path) {
    process::required_output(
        Command::new(cc)
            .args([level, "-fno-fast-math", "-no-pie"])
            .args(sources)
            .args(["-lm", "-o"])
            .arg(executable),
    );
}

fn symbol(executable: &Path, name: &str) -> String {
    let symbols = process::required_output(Command::new("nm").arg(executable));
    let symbols = String::from_utf8(symbols.stdout).unwrap();
    let suffix = format!(" {name}");
    symbols
        .lines()
        .find(|line| line.ends_with(&suffix))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

#[test]
fn float_compare_flags_keep_their_unordered_case() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native float compare round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("float-unordered-compare", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let source = fixture("float_unordered_compare.c");
    let driver = fixture("float_unordered_compare_driver.c");
    let specs = common::repo_root().join("specs");
    for cc in &compilers {
        let native = dir.join(format!("native-{cc}"));
        compile(cc, "-O2", &[&source, &driver], &native);
        process::required_output(&mut Command::new(&native));
        let one = format!("data 0x{} float one", symbol(&native, "one"));
        let two = format!("data 0x{} double two", symbol(&native, "two"));
        let huge = format!("data 0x{} double huge", symbol(&native, "huge"));
        let bound = format!("data 0x{} double bound", symbol(&native, "bound"));
        let (printed, stderr, rc) = common::run_kuna(&[
            "decompile-all",
            native.to_str().unwrap(),
            "--functions",
            FUNCTIONS,
            "--mode",
            "reliable",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--assert-strict",
            "--assert",
            "prototype above int above(float x)",
            "--assert",
            "prototype at_least int at_least(float x)",
            "--assert",
            "prototype not_above int not_above(float x)",
            "--assert",
            "prototype not_at_least int not_at_least(float x)",
            "--assert",
            "prototype pick int pick(double x,double y)",
            "--assert",
            "prototype not_above_pair int not_above_pair(double x,double y)",
            "--assert",
            "prototype above_two int above_two(double x)",
            "--assert",
            "prototype nan_or_huge double nan_or_huge(double x)",
            "--assert",
            "prototype nan_or_above int nan_or_above(double x)",
            "--assert",
            "prototype ordered_below int ordered_below(float x)",
            "--assert",
            "prototype ordered_below_pair int ordered_below_pair(float x,float y)",
            "--assert",
            "prototype work double work(double x)",
            "--assert",
            &one,
            "--assert",
            &two,
            "--assert",
            &huge,
            "--assert",
            &bound,
        ]);
        assert_eq!(rc, 0, "{stderr}\n{printed}");
        let emitted = dir.join(format!("printed-{cc}.c"));
        std::fs::write(&emitted, format!(
            "#include <math.h>\n#undef NAN\n#define NAN(x) isnan(x)\ntypedef float float4;\ntypedef double float8;\ntypedef int int4;\ntypedef unsigned int uint4;\nextern const float one;\nextern const double two;\nextern const double huge;\nextern const double bound;\nextern double work(double);\n{printed}"
        )).unwrap();
        for level in ["-O0", "-O2"] {
            let executable = dir.join(format!("printed-{cc}{level}"));
            compile(cc, level, &[&emitted, &driver], &executable);
            let run = Command::new(&executable).output().unwrap();
            assert!(
                run.status.success(),
                "{cc} {level}: printed C disagrees with the binary\n{}\n{printed}",
                String::from_utf8_lossy(&run.stderr)
            );
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
