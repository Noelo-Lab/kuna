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
            .args([level, "-fno-fast-math", "-ffp-contract=off", "-no-pie"])
            .arg(source)
            .arg(driver)
            .args(["-o"])
            .arg(executable),
    );
}

#[test]
fn widened_scalar_and_simd_arithmetic_matches_native_bits() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping native precision round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("float-precision", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let assembly = fixture("float_precision_x86_64.S");
    let driver = fixture("float_precision_driver.c");
    let native = dir.join("native");
    compile(compilers[0], "-O0", &assembly, &driver, &native);
    process::required_output(&mut Command::new(&native));

    let specs = common::repo_root().join("specs");
    let (printed, stderr, rc) = common::run_kuna(&[
        "decompile-all", native.to_str().unwrap(),
        "--functions", "widened_update,packed_update,float_update,widen_roundtrip,widened_product,widened_quotient",
        "--sleighpath", specs.to_str().unwrap(),
        "--assert-strict",
        "--assert", "prototype widened_update float widened_update(float accum,float neg_theta,float local_axis)",
        "--assert", "prototype packed_update void packed_update(float *out,float *accum,float *neg_theta,float *local_axis)",
        "--assert", "prototype float_update float float_update(float accum,float neg_theta,float local_axis)",
        "--assert", "prototype widen_roundtrip float widen_roundtrip(float value)",
        "--assert", "prototype widened_product float widened_product(float a,float b,float c)",
        "--assert", "prototype widened_quotient float widened_quotient(float a,float b,float c)",
    ]);
    assert_eq!(rc, 0, "{stderr}\n{printed}");
    let source = dir.join("printed.c");
    std::fs::write(
        &source,
        format!(
            "typedef float float4;\ntypedef double float8;\n\
         #define widened_update printed_widened_update\n\
         #define packed_update printed_packed_update\n\
         #define float_update printed_float_update\n\
         #define widen_roundtrip printed_widen_roundtrip\n\
         #define widened_product printed_widened_product\n\
         #define widened_quotient printed_widened_quotient\n{printed}"
        ),
    )
    .unwrap();
    for cc in &compilers {
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
