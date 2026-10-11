#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use common::process;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn compile(cc: &str, level: &str, source: &Path, driver: &Path, executable: &Path) {
    process::required_output(
        Command::new(cc)
            .args([level, "-fno-fast-math", "-no-pie"])
            .arg(source)
            .arg(driver)
            .args(["-lm", "-o"])
            .arg(executable),
    );
}

#[test]
fn minss_keeps_source_nan_payload_and_signed_zero() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native MINSS round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("sse-minss", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let assembly = fixture("sse_minss_x86_64.S");
    let driver = fixture("sse_minss_driver.c");
    let native = dir.join("native");
    compile(compilers[0], "-O0", &assembly, &driver, &native);
    process::required_output(&mut Command::new(&native));
    let symbols = process::required_output(Command::new("nm").arg(&native));
    let symbols = String::from_utf8(symbols.stdout).unwrap();
    let constant = symbols
        .lines()
        .find(|line| line.ends_with(" clamp_one"))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    let data = format!("data 0x{constant} float clamp_one");

    for nanignore in [None, Some("none")] {
        let specs = common::repo_root().join("specs");
        let mut args = vec![
            "decompile-all",
            native.to_str().unwrap(),
            "--functions",
            "scalar_min,ratio_clamp,ordered_above",
            "--mode",
            "reliable",
            "--sleighpath",
            specs.to_str().unwrap(),
            "--assert-strict",
            "--assert",
            "prototype scalar_min float scalar_min(float a,float b)",
            "--assert",
            "prototype ratio_clamp float ratio_clamp(float value,float lo,float hi)",
            "--assert",
            "prototype ordered_above int ordered_above(float a,float b)",
            "--assert",
            &data,
        ];
        if let Some(value) = nanignore {
            args.extend(["--option", "nanignore", value]);
        }
        let (printed, stderr, rc) = common::run_kuna(&args);
        assert_eq!(rc, 0, "{stderr}\n{printed}");
        let source = dir.join("printed.c");
        std::fs::write(&source, format!(
            "#include <math.h>\n#undef NAN\n#define NAN(x) isnan(x)\ntypedef float float4;\ntypedef int int4;\ntypedef unsigned int uint4;\nconst float clamp_one=1.0f;\n{printed}"
        )).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let executable = dir.join(format!("printed-{cc}-{level}"));
                compile(cc, level, &source, &driver, &executable);
                process::required_output(&mut Command::new(executable));
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
