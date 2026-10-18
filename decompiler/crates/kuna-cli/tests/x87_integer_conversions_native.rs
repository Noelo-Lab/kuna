#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use crate::common;
use common::process;
use std::process::Command;

#[test]
fn regenerated_x87_integer_conversions_match_native_rounding_and_flags() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let directory = common::scratch_file("x87-integer-conversions", "dir");
    std::fs::create_dir_all(&directory).unwrap();
    let source = directory.join("sample.s");
    let binary = directory.join("sample.so");
    let exported = directory.join("exported");
    let helpers = directory.join("helpers.h");
    let driver = directory.join("driver.c");
    std::fs::write(&source, include_str!("fixtures/x87_integer_conversions.s")).unwrap();
    std::fs::write(&helpers, include_str!("fixtures/x87_integer_conversions_helpers.h")).unwrap();
    std::fs::write(&driver, include_str!("fixtures/x87_integer_conversions_driver.c")).unwrap();
    process::required_output(Command::new("clang")
        .args(["-shared", "-fPIC", "-nostdlib"])
        .arg(&source).arg("-o").arg(&binary));

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut decompile = Command::new(env!("CARGO_BIN_EXE_kuna"));
    decompile.arg("decompile-project").arg(&binary)
        .arg("--sleighpath").arg(root.join("specs"))
        .args(["--assert-strict", "--option", "nanignore", "none"])
        .arg("-o").arg(&exported);
    for declaration in [
        "short sample_fistp16(float10 *value,unsigned short control)",
        "int sample_fistp32(float10 *value,unsigned short control)",
        "long long sample_fistp64(float10 *value,unsigned short control)",
        "short sample_fist16(float10 *value,unsigned short control,float10 *observed)",
        "int sample_fist32(float10 *value,unsigned short control,float10 *observed)",
    ] {
        let name = declaration.split('(').next().unwrap().split_whitespace().last().unwrap();
        decompile.arg("--assert").arg(format!("prototype {name} {declaration}"));
    }
    process::required_output(&mut decompile);
    let mut compilers = 0;
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        compilers += 1;
        for optimization in ["-O0", "-O2"] {
            let executable = directory.join(format!("{compiler}{optimization}"));
            process::required_output(Command::new(compiler)
                .args(["-std=gnu11", optimization, "-fno-fast-math", "-frounding-math", "-ffp-contract=off"])
                .arg("-I").arg(&exported)
                .arg("-include").arg(&helpers)
                .arg(exported.join("sample.so.c")).arg(&driver)
                .args(["-ldl", "-lm", "-o"]).arg(&executable));
            let result = process::required_output(Command::new(&executable).arg(&binary));
            assert!(String::from_utf8_lossy(&result.stdout).contains("21140 native comparisons passed"), "{result:?}");
        }
    }
    assert!(compilers > 0);
    std::fs::remove_dir_all(directory).unwrap();
}
