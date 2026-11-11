use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_go_integer_stores_match_native_source_and_call_counts() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none()
        || process::optional_output(Command::new("go").arg("version")).is_none()
    {
        return;
    }
    let declarations = std::fs::read_to_string(common::fixture("go_store_value.kuna")).unwrap();
    let names: Vec<_> = declarations
        .lines()
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .filter(|name| !name.starts_with("counted_") && *name != "nested_store")
        .collect();
    assert_eq!(names.len(), 33);
    let original = std::fs::read_to_string(common::fixture("go_store_value.c")).unwrap();
    let helpers = std::fs::read_to_string(common::fixture("go_store_value_helpers.c")).unwrap();
    let harness = std::fs::read_to_string(common::fixture("go_store_value_harness.c")).unwrap();
    let source = common::scratch_file("go_store_source", "c");
    std::fs::write(
        &source,
        format!(
            "{}\n{}\n{harness}",
            original.split("int main(void)").next().unwrap(),
            helpers.lines().skip(1).collect::<Vec<_>>().join("\n"),
        ),
    )
    .unwrap();
    let executable = source.with_extension("out");
    process::required_output(
        Command::new("cc")
            .args(["-O2", "-fwrapv", "-Werror"])
            .arg(&source)
            .arg("-o")
            .arg(&executable),
    );
    let expected = process::required_output(&mut Command::new(&executable)).stdout;
    assert_eq!(String::from_utf8_lossy(&expected).lines().count(), 33 * 324);
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
    let kuna = std::env::var_os("KUNA_TEST_BIN")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(Command::new(kuna).args([
        "decompile-all",
        &common::fixture("go_store_value_x86_64"),
        "--json",
        "--functions",
        &names.join(","),
        "--language",
        "go",
        "--assert",
        &format!("@{}", common::fixture("go_store_value.kuna")),
        "--option",
        "ctypes",
        "off",
        "--option",
        "realtypes",
        "off",
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
    let template = std::fs::read_to_string(common::fixture("go_store_value_harness.go")).unwrap();
    assert_eq!(template.matches("/* EMITTED_BODIES */").count(), 1);
    let file = common::scratch_file("go_store_emitted", "go");
    std::fs::write(&file, template.replace("/* EMITTED_BODIES */", &bodies)).unwrap();
    let executable = file.with_extension("out");
    process::required_output(Command::new("go").arg("build").arg("-o").arg(&executable).arg(&file));
    let actual = process::required_output(&mut Command::new(&executable));
    assert_eq!(actual.stdout, expected);
    std::fs::remove_file(executable).unwrap();
    std::fs::remove_file(file).unwrap();
}
