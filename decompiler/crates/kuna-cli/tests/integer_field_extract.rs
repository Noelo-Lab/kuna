use crate::common;
use common::process;
use std::process::Command;

const RUST_UNCHANGED_GAPS: [&str; 5] = [
    "signed_uint8_wide",
    "signed_uint16_wide",
    "signed_uint32_wide",
    "signed_as_unsigned",
    "unsigned_as_signed",
];

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_integer_field_callers_match_source_values_and_call_counts() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let assertions = common::fixture("integer_field_extract.kuna");
    let declarations = std::fs::read_to_string(&assertions).unwrap();
    let names: Vec<_> = declarations
        .lines()
        .skip(2)
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();
    assert_eq!(names.len(), 26);
    let original = std::fs::read_to_string(common::fixture("integer_field_extract.c")).unwrap();
    let harness =
        std::fs::read_to_string(common::fixture("integer_field_extract_source_harness.c")).unwrap();
    let source = common::scratch_file("integer-field-source", "c");
    std::fs::write(
        &source,
        format!(
            "{}\n{harness}",
            original.split("int main(void)").next().unwrap()
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
    assert_eq!(String::from_utf8_lossy(&expected).lines().count(), 26 * 784);
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
    for (language, compiler, extension, version) in [
        ("c", "cc", "c", "--version"),
        ("go", "go", "go", "version"),
        ("rust", "rustc", "rs", "--version"),
    ] {
        if process::optional_output(Command::new(compiler).arg(version)).is_none() {
            continue;
        }
        let included: Vec<_> = names
            .iter()
            .copied()
            .filter(|name| language != "rust" || !RUST_UNCHANGED_GAPS.contains(name))
            .collect();
        let kuna =
            std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
        let output = process::required_output(Command::new(kuna).args([
            "decompile-all",
            &common::fixture("integer_field_extract_x86_64"),
            "--json",
            "--functions",
            &included.join(","),
            "--language",
            language,
            "--assert",
            &format!("@{assertions}"),
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
        assert_eq!(functions.len(), included.len());
        let bodies = functions
            .iter()
            .map(|function| {
                assert!(function["error"].is_null(), "{function}");
                function["code"].as_str().unwrap()
            })
            .collect::<Vec<_>>()
            .join("\n");
        let template = std::fs::read_to_string(common::fixture(&format!(
            "integer_field_extract_harness.{extension}"
        )))
        .unwrap();
        assert_eq!(template.matches("/* EMITTED_CALLERS */").count(), 1);
        let file = common::scratch_file("integer_field_emitted", extension);
        std::fs::write(&file, template.replace("/* EMITTED_CALLERS */", &bodies)).unwrap();
        let expected: Vec<_> = String::from_utf8_lossy(&expected)
            .lines()
            .enumerate()
            .filter(|(index, _)| included.contains(&names[index % names.len()]))
            .map(|(_, line)| line.to_owned())
            .collect();
        for optimization in ["0", "2"] {
            let executable = file.with_extension(format!("O{optimization}"));
            let mut build = Command::new(compiler);
            match language {
                "c" => {
                    build.args([&format!("-O{optimization}"), "-fwrapv", "-Werror"]);
                }
                "go" => {
                    build.arg("build");
                }
                _ => {
                    build.args([
                        "--edition=2021",
                        "--crate-name",
                        "integer_field_emitted",
                        "-C",
                        &format!("opt-level={optimization}"),
                    ]);
                }
            }
            process::required_output(build.arg("-o").arg(&executable).arg(&file));
            let actual = process::required_output(&mut Command::new(&executable));
            assert_eq!(
                String::from_utf8_lossy(&actual.stdout)
                    .lines()
                    .collect::<Vec<_>>(),
                expected,
                "{language} O{optimization}"
            );
            std::fs::remove_file(executable).unwrap();
            if language == "go" {
                break;
            }
        }
        std::fs::remove_file(file).unwrap();
    }
}
