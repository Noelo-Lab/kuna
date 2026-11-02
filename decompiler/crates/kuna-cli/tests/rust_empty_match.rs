use crate::common;
use common::process;
use std::path::Path;
use std::process::Command;

const NAMES: &[&str] = &[
    "empty_case",
    "empty_default",
    "empty_shared",
    "empty_loop",
    "loop_break_control",
];

fn records() -> serde_json::Value {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let result = process::required_output(Command::new(binary).args([
        "decompile-all",
        &common::fixture("rust_empty_match_x86_64"),
        "--json",
        "--language",
        "rust",
        "--functions",
        &NAMES.join(","),
        "--assert",
        &format!("@{}", common::fixture("rust_empty_match.kuna")),
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(doc["functions"].as_array().unwrap().len(), NAMES.len());
    for function in doc["functions"].as_array().unwrap() {
        assert!(function["error"].is_null(), "{function}");
    }
    doc
}

#[test]
fn empty_match_exits_leave_real_loop_breaks_intact() {
    let doc = records();
    for function in doc["functions"].as_array().unwrap() {
        let name = function["name"].as_str().unwrap();
        let code = function["code"].as_str().unwrap();
        let expected = usize::from(matches!(name, "empty_loop" | "loop_break_control"));
        assert_eq!(code.matches("break;").count(), expected, "{code}");
        assert_eq!(
            code.matches("match ").count(),
            usize::from(name != "loop_break_control"),
            "{code}"
        );
    }
}

fn values(binary: &Path, input: u64, count: i32) -> Vec<i32> {
    let result =
        process::required_output(Command::new(binary).args([input.to_string(), count.to_string()]));
    String::from_utf8(result.stdout)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_rust_bodies_preserve_empty_arms_and_loop_iterations() {
    if process::optional_output(Command::new("rustc").arg("--version")).is_none() {
        return;
    }
    let doc = records();
    let bodies = doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let source = common::scratch_file("rust-empty-match", "rs");
    std::fs::write(&source, format!("{bodies}\nfn main() {{ let args: Vec<String> = std::env::args().collect(); let x: u64 = args[1].parse().unwrap(); let n: i32 = args[2].parse().unwrap(); unsafe {{ println!(\"{{}} {{}} {{}} {{}} {{}}\", empty_case(x),empty_default(x),empty_shared(x),empty_loop(x,n),loop_break_control(10,n)); }} }}\n")).unwrap();
    let original = Path::new(&common::fixture("rust_empty_match_x86_64")).to_path_buf();
    let mut expected = Vec::new();
    for high in (0..43u64).chain([0x7fff_ffff, 0x8000_0000, 0xffff_ffff]) {
        for low in [0, 3, 0xffff_ffff] {
            for count in [-2, 0, 1, 2, 7] {
                let input = (high << 32) | low;
                let native = values(&original, input, count);
                let ordinary = if high <= 40 { high as i32 + 7 } else { 48 };
                let default = if high <= 40 { high as i32 + 8 } else { 7 };
                let shared = if high <= 1 { 7 } else { ordinary };
                assert_eq!(
                    native,
                    [
                        ordinary,
                        default,
                        shared,
                        ordinary * count.max(0),
                        3 * count.clamp(0, 4)
                    ]
                );
                expected.push((input, count, native));
            }
        }
    }
    for optimization in ["0", "2"] {
        let binary = source.with_extension(format!("O{optimization}"));
        process::required_output(
            Command::new("rustc")
                .args([
                    "--edition=2021",
                    "--crate-name",
                    "recovered_empty_match",
                    "-C",
                    &format!("opt-level={optimization}"),
                ])
                .arg(&source)
                .arg("-o")
                .arg(&binary),
        );
        for (input, count, expected) in &expected {
            assert_eq!(
                values(&binary, *input, *count),
                *expected,
                "O{optimization}, input={input}, count={count}"
            );
        }
        std::fs::remove_file(binary).unwrap();
    }
    std::fs::remove_file(source).unwrap();
}
