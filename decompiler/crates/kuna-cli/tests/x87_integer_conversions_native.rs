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

#[test]
fn truncating_control_words_become_c_conversions() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let directory = common::scratch_file("x87-truncating-conversions", "dir");
    std::fs::create_dir_all(&directory).unwrap();
    let source = directory.join("sample.s");
    let binary = directory.join("sample.so");
    let exported = directory.join("exported");
    let helpers = directory.join("helpers.h");
    let driver = directory.join("driver.c");
    std::fs::write(&source, include_str!("fixtures/x87_truncating_conversions.s")).unwrap();
    std::fs::write(&helpers, include_str!("fixtures/x87_integer_conversions_helpers.h")).unwrap();
    std::fs::write(&driver, include_str!("fixtures/x87_truncating_conversions_driver.c")).unwrap();
    process::required_output(Command::new("clang")
        .args(["-shared", "-fPIC", "-nostdlib"])
        .arg(&source).arg("-o").arg(&binary));

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut decompile = Command::new(env!("CARGO_BIN_EXE_kuna"));
    decompile.arg("decompile-project").arg(&binary)
        .arg("--sleighpath").arg(root.join("specs"))
        .arg("--assert-strict")
        .arg("-o").arg(&exported);
    for declaration in [
        "short sample_cast16(float10 *value)",
        "int sample_cast32(float10 *value)",
        "long long sample_cast64(float10 *value,unsigned short extra)",
        "int sample_floor32(float10 *value)",
        "int sample_restored32(float10 *value,void *area)",
        "int sample_xrestored32(float10 *value,void *area)",
    ] {
        let name = declaration.split('(').next().unwrap().split_whitespace().last().unwrap();
        decompile.arg("--assert").arg(format!("prototype {name} {declaration}"));
    }
    process::required_output(&mut decompile);
    let code = std::fs::read_to_string(exported.join("sample.so.c")).unwrap();
    let body = |name: &str| {
        let start = code.find(&format!("// Function: {name} ")).unwrap();
        let rest = &code[start + 1..];
        rest[..rest.find("// Function:").unwrap_or(rest.len())].to_string()
    };
    for (name, cast) in [("sample_cast16", "(short)*value"), ("sample_cast32", "(int)*value"), ("sample_cast64", "*value")] {
        let text = body(name);
        assert!(text.contains(cast) && !text.contains("x87_float_to_int") && !text.contains("FLOAT_ROUND"), "{text}");
    }
    for name in ["sample_floor32", "sample_restored32", "sample_xrestored32"] {
        assert!(body(name).contains("x87_float_to_int32("), "{code}");
    }

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
            let stdout = String::from_utf8_lossy(&result.stdout);
            assert!(stdout.contains("436 native comparisons passed") && stdout.contains("672 restored control words compared"), "{result:?}");
        }
    }
    assert!(compilers > 0);
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn integer_conversion_results_keep_their_bits_in_float_storage() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let directory = common::scratch_file("x87-integer-storage-views", "dir");
    std::fs::create_dir_all(&directory).unwrap();
    let binary = directory.join("sample.so");
    let exported = directory.join("exported");
    let source = directory.join("sample.s");
    let helpers = directory.join("helpers.h");
    let driver = directory.join("driver.c");
    std::fs::write(&source, include_str!("fixtures/x87_integer_storage_views.s")).unwrap();
    std::fs::write(&helpers, format!("{}\nextern unsigned char storage[32];\n", include_str!("fixtures/x87_integer_conversions_helpers.h"))).unwrap();
    std::fs::write(&driver, include_str!("fixtures/x87_integer_storage_views_driver.c")).unwrap();
    process::required_output(Command::new("clang").args(["-shared", "-fPIC", "-nostdlib", "-Wl,-Bsymbolic"])
        .arg(&source).arg("-o").arg(&binary));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    process::required_output(Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile-project").arg(&binary)
        .arg("--sleighpath").arg(root.join("specs"))
        .args(["--assert-strict", "--assert", "prototype sample32 float sample32(float *input,unsigned short control)",
            "--assert", "prototype sample64 double sample64(double *input,unsigned short control)",
            "--assert", "prototype sample_both void sample_both(float *input,float *bits,double *numeric,unsigned short control)",
            "--assert", "param sample_both::3 FPUControlWord unsigned short control",
            "--assert", "param sample32::1 FPUControlWord unsigned short control",
            "--assert", "param sample64::1 FPUControlWord unsigned short control",
            "--assert", "data 0x3000 unsigned char storage[32]"])
        .arg("-o").arg(&exported));
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() { continue; }
        for optimization in ["-O0", "-O2"] {
            let executable = directory.join(format!("{compiler}{optimization}"));
            process::required_output(Command::new(compiler)
                .args(["-std=gnu11", optimization, "-fno-fast-math", "-frounding-math", "-ffp-contract=off"])
                .arg("-include").arg(&helpers)
                .arg(exported.join("sample.so.c")).arg(&driver)
                .args(["-ldl", "-lm", "-o"]).arg(&executable));
            let output = process::required_output(Command::new(&executable).arg(&binary));
            assert!(String::from_utf8_lossy(&output.stdout).contains("32768 native storage comparisons passed"));
        }
    }
    std::fs::remove_dir_all(directory).unwrap();
}
