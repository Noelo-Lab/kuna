use crate::common;
use common::process;
use std::process::Command;

fn records() -> serde_json::Value {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(Command::new(kuna).args([
        "decompile-all",
        &common::fixture("joined_callback_return_x86_64"),
        "--json",
        "--functions",
        "make_pair,first,second,both,forward,narrow_second,scalar,invoke_void",
        "--assert",
        &format!("@{}", common::fixture("joined_callback_return.kuna")),
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
    assert_eq!(functions.len(), 8);
    for function in functions {
        assert!(function["error"].is_null(), "{function}");
    }
    document
}

#[test]
fn typed_callbacks_retain_each_joined_return_component() {
    let document = records();
    let functions = document["functions"].as_array().unwrap();
    for (name, field) in [
        ("first", "first"),
        ("second", "second"),
        ("narrow_second", "second"),
    ] {
        let code = functions.iter().find(|f| f["name"] == name).unwrap()["code"]
            .as_str()
            .unwrap();
        assert!(code.contains(&format!(".{field}")), "{code}");
        assert_eq!(code.matches("(*fn)(a,b)").count(), 1, "{code}");
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_callback_bodies_match_native_values_and_call_counts() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let document = records();
    let bodies = document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let source = std::fs::read_to_string(common::fixture("joined_callback_return.c")).unwrap();
    let source = source.split("int main(void)").next().unwrap();
    let harness =
        std::fs::read_to_string(common::fixture("joined_callback_return_harness.c")).unwrap();
    let emitted = format!(
        "#include <stdint.h>\ntypedef int64_t int8; typedef uint64_t uint8; typedef int32_t int4;\ntypedef struct Pair {{ int8 first; int8 second; }} Pair;\nuint64_t calls;\n{bodies}\n{harness}"
    );
    let mut expected = None;
    for (arm, code) in [
        ("source", format!("{source}\n{harness}")),
        ("emitted", emitted),
    ] {
        let file = common::scratch_file(&format!("joined-callback-{arm}"), "c");
        std::fs::write(&file, code).unwrap();
        for optimization in ["-O0", "-O2"] {
            let binary = file.with_extension(&optimization[1..]);
            process::required_output(
                Command::new("cc")
                    .args([optimization, "-fwrapv", "-Werror"])
                    .arg(&file)
                    .arg("-o")
                    .arg(&binary),
            );
            let actual = process::required_output(&mut Command::new(&binary));
            assert_eq!(String::from_utf8_lossy(&actual.stdout).lines().count(), 169);
            if let Some(expected) = &expected {
                assert_eq!(&actual.stdout, expected, "{arm} {optimization}");
            } else {
                expected = Some(actual.stdout);
            }
            std::fs::remove_file(binary).unwrap();
        }
        std::fs::remove_file(file).unwrap();
    }
}
