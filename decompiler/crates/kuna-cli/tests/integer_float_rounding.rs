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
fn printed_integer_conversions_match_native_rounding() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native rounding round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("integer-float-rounding", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let assembly = fixture("integer_float_rounding_x86_64.S");
    let driver = fixture("integer_float_rounding_driver.c");
    let native = dir.join("native");
    process::required_output(
        Command::new(compilers[0])
            .args(["-O0", "-no-pie"])
            .arg(&assembly)
            .arg(&driver)
            .arg("-o")
            .arg(&native),
    );
    process::required_output(&mut Command::new(&native));
    let specs = common::repo_root().join("specs");
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all",
        native.to_str().unwrap(),
        "--functions",
        "rounded32,narrowed64,widened16,exact32,trunc_after_narrow",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--assert-strict",
        "--assert",
        "prototype rounded32 double rounded32(int value)",
        "--assert",
        "prototype narrowed64 float narrowed64(long value)",
        "--assert",
        "prototype widened16 double widened16(short value)",
        "--assert",
        "prototype exact32 float exact32(int value)",
        "--assert",
        "prototype trunc_after_narrow int trunc_after_narrow(int value,float *out)",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let source = dir.join("printed.c");
    std::fs::write(&source, format!(
        "typedef float float4;\ntypedef double float8;\ntypedef short int2;\ntypedef int int4;\ntypedef long long int8;\n\
         #define rounded32 printed_rounded32\n#define narrowed64 printed_narrowed64\n\
         #define widened16 printed_widened16\n#define exact32 printed_exact32\n#define trunc_after_narrow printed_trunc_after_narrow\n{printed}"
    )).unwrap();
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let executable = dir.join(format!("printed-{cc}-{level}"));
            process::required_output(
                Command::new(cc)
                    .args([
                        level,
                        "-fno-fast-math",
                        "-ffp-contract=off",
                        "-no-pie",
                        "-DROUNDTRIP",
                    ])
                    .arg(&source)
                    .arg(&assembly)
                    .arg(&driver)
                    .arg("-o")
                    .arg(&executable),
            );
            process::required_output(&mut Command::new(executable));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
