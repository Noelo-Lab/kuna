use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_rust_array_addresses_match_native_without_memory_reads() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let declarations = std::fs::read_to_string(common::fixture("array_address.kuna")).unwrap();
    let names: Vec<_> = declarations.lines()
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .filter(|name| !name.starts_with("counted_")).collect();
    assert_eq!(names.len(), 33);
    let native = common::scratch_file("rust_array_address_reference", "out");
    let mut expected = None;
    for optimization in ["-O0", "-O2"] {
        process::required_output(Command::new("cc")
            .args([optimization, "-fwrapv", "-fno-inline", "-std=c11", "-Wall", "-Wextra", "-Werror"])
            .arg(common::fixture("array_address_harness.c"))
            .arg(common::fixture("array_address_helpers.c"))
            .arg("-o").arg(&native));
        let output = process::required_output(&mut Command::new(&native)).stdout;
        assert_eq!(String::from_utf8_lossy(&output).lines().count(), 2 * (556 + 1));
        if let Some(previous) = &expected { assert_eq!(&output, previous); }
        expected = Some(output);
    }
    std::fs::remove_file(&native).unwrap();
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(Command::new(kuna).args([
        "decompile-all", &common::fixture("array_address_x86_64"),
        "--json", "--functions", &names.join(","), "--language", "rust",
        "--assert", &format!("@{}", common::fixture("array_address.kuna")),
        "--option", "ctypes", "off", "--option", "realtypes", "off",
    ]));
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    for assertion in document["assertions"].as_array().unwrap() {
        assert_eq!(assertion["status"], "applied", "{assertion}");
    }
    let functions = document["functions"].as_array().unwrap();
    assert_eq!(functions.len(), names.len());
    let bodies = functions.iter().map(|function| {
        assert!(function["error"].is_null(), "{function}");
        function["code"].as_str().unwrap()
    }).collect::<Vec<_>>().join("\n");
    let template = std::fs::read_to_string(common::fixture("array_address_harness.rs")).unwrap();
    assert_eq!(template.matches("/* EMITTED_BODIES */").count(), 1);
    let file = common::scratch_file("rust_array_address_emitted", "rs");
    std::fs::write(&file, template.replace("/* EMITTED_BODIES */", &bodies)).unwrap();
    let emitted = file.with_extension("out");
    for optimization in ["0", "2"] {
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "rust_array_address_emitted", "-C", &format!("opt-level={optimization}"), "-C", "overflow-checks=off"])
            .arg(&file).arg("-o").arg(&emitted));
        let actual = process::required_output(&mut Command::new(&emitted));
        assert_eq!(&actual.stdout, expected.as_ref().unwrap());
    }
    std::fs::remove_file(emitted).unwrap();
    std::fs::remove_file(file).unwrap();
}
