use crate::common;
use common::process;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

const NAMES: &[&str] = &[
    "narrow_guard",
    "wide_guard",
    "explicit_zext",
    "byte_guard",
    "word_guard",
    "signed_guard",
];

fn records(language: &str) -> serde_json::Value {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let facts = common::fixture("highslice_guard.kuna");
    let result = process::required_output(Command::new(binary).args([
        "decompile-all",
        &common::fixture("highslice_guard_x86_64"),
        "--json",
        "--language",
        language,
        "--option",
        "highsliceguard",
        "on",
        "--functions",
        &NAMES.join(","),
        "--assert",
        &format!("@{facts}"),
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), NAMES.len());
    for function in functions {
        assert!(function["error"].is_null(), "{function}");
    }
    doc
}

#[test]
fn high_byte_guards_recover_tables_in_every_output_language() {
    for language in ["c", "rust", "go"] {
        let doc = records(language);
        for function in doc["functions"].as_array().unwrap() {
            let code = function["code"].as_str().unwrap();
            assert!(!code.contains("jump-as-call"), "{code}");
            assert!(!code.contains("Could not recover jumptable"), "{code}");
            if language == "c" {
                assert_eq!(code.matches("switch(").count(), 1, "{code}");
                assert_eq!(code.matches("case ").count(), 41, "{code}");
            }
        }
    }
}

fn values(binary: &Path, input: u64) -> Vec<u32> {
    let result = process::required_output(Command::new(binary).arg(input.to_string()));
    String::from_utf8(result.stdout)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_recovered_switches_preserve_native_values_at_width_boundaries() {
    let doc = records("c");
    let bodies = doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let main = include_str!("../../kuna-analysis/tests/fixtures/highslice_guard.c");
    let source = common::scratch_file("highslice-guard", "c");
    std::fs::write(&source, format!("{main}\n{bodies}")).unwrap();
    let mut inputs = BTreeSet::new();
    for shift in [32, 48, 56] {
        let max = u64::MAX >> shift;
        for high in (0..44).chain([max >> 1, (max >> 1) + 1, max]) {
            for low in [0, 3, (1u64 << shift) - 1] {
                inputs.insert((high << shift) | low);
            }
        }
    }
    let native = Path::new(&common::fixture("highslice_guard_x86_64")).to_path_buf();
    let expected = inputs
        .into_iter()
        .map(|input| {
            let values = values(&native, input);
            let wanted: Vec<u32> = [32, 32, 32, 56, 48, 32]
                .into_iter()
                .map(|shift| {
                    let index = input >> shift;
                    if index <= 40 {
                        index as u32 * 7 + 3
                    } else {
                        1000
                    }
                })
                .collect();
            assert_eq!(values, wanted, "original input={input}");
            (input, values)
        })
        .collect::<Vec<_>>();
    for compiler in ["gcc", "clang"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            continue;
        }
        for optimization in ["0", "2"] {
            let binary = source.with_extension(format!("{compiler}-{optimization}"));
            process::required_output(
                Command::new(compiler)
                    .args([
                        "-std=c11",
                        "-Werror=implicit-function-declaration",
                        &format!("-O{optimization}"),
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            for (input, expected) in &expected {
                assert_eq!(
                    values(&binary, *input),
                    *expected,
                    "{compiler} O{optimization}, {input}"
                );
            }
            std::fs::remove_file(binary).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn narrow_guards_with_empty_rust_arms_preserve_enclosing_loops() {
    for compiler in ["cc", "rustc"] {
        if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            return;
        }
    }
    let assembly = include_str!("../../kuna-analysis/tests/fixtures/rust_empty_match.S");
    assert_eq!(assembly.matches("cmp $40,%rdi").count(), 4);
    let asm = common::scratch_file("highslice-empty-match", "S");
    std::fs::write(&asm, assembly.replace("cmp $40,%rdi", "cmp $40,%edi")).unwrap();
    let native = asm.with_extension("native");
    process::required_output(
        Command::new("cc")
            .args(["-g", "-no-pie"])
            .arg(&asm)
            .arg(common::fixture("rust_empty_match.c"))
            .arg("-o")
            .arg(&native),
    );
    let names = "empty_case,empty_default,empty_shared,empty_loop,loop_break_control";
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let result = process::required_output(
        Command::new(kuna)
            .args(["decompile-all", "--option", "highsliceguard", "on"])
            .arg(&native)
            .args([
                "--json",
                "--language",
                "rust",
                "--functions",
                names,
                "--assert",
            ])
            .arg(format!("@{}", common::fixture("rust_empty_match.kuna"))),
    );
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 5);
    let bodies = functions
        .iter()
        .map(|f| {
            assert!(f["error"].is_null(), "{f}");
            let code = f["code"].as_str().unwrap();
            assert!(!code.contains("jump-as-call"), "{code}");
            code
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(bodies.matches("match ").count(), 4, "{bodies}");
    assert_eq!(bodies.matches("break;").count(), 2, "{bodies}");
    let source = asm.with_extension("rs");
    std::fs::write(&source, format!("{bodies}\nfn main() {{ let args:Vec<String>=std::env::args().collect(); let x:u64=args[1].parse().unwrap(); let n:i32=args[2].parse().unwrap(); unsafe {{ println!(\"{{}} {{}} {{}} {{}} {{}}\",empty_case(x),empty_default(x),empty_shared(x),empty_loop(x,n),loop_break_control(10,n)); }} }}\n")).unwrap();
    let observe = |binary: &Path, x: u64, n: i32| -> Vec<i32> {
        let output =
            process::required_output(Command::new(binary).args([x.to_string(), n.to_string()]));
        String::from_utf8(output.stdout)
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };
    let mut expected = Vec::new();
    for high in (0..43u64).chain([0x7fff_ffff, 0x8000_0000, 0xffff_ffff]) {
        for low in [0, 3, 0xffff_ffff] {
            for count in [-2, 0, 1, 2, 7] {
                let input = (high << 32) | low;
                let ordinary = if high <= 40 { high as i32 + 7 } else { 48 };
                let values = observe(&native, input, count);
                assert_eq!(
                    values,
                    [
                        ordinary,
                        if high <= 40 { high as i32 + 8 } else { 7 },
                        if high <= 1 { 7 } else { ordinary },
                        ordinary * count.max(0),
                        3 * count.clamp(0, 4)
                    ]
                );
                expected.push((input, count, values));
            }
        }
    }
    for optimization in ["0", "2"] {
        let binary = asm.with_extension(format!("O{optimization}"));
        process::required_output(
            Command::new("rustc")
                .args([
                    "--edition=2021",
                    "--crate-name",
                    "highslice_empty_match",
                    "-C",
                    &format!("opt-level={optimization}"),
                ])
                .arg(&source)
                .arg("-o")
                .arg(&binary),
        );
        for (input, count, expected) in &expected {
            assert_eq!(
                observe(&binary, *input, *count),
                *expected,
                "O{optimization}, input={input}, count={count}"
            );
        }
        std::fs::remove_file(binary).unwrap();
    }
    for file in [asm, native, source] {
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn every_preset_keeps_high_slice_recovery_explicit() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    for mode in ["auto", "reliable", "aggressive", "fast"] {
        let mut previous = None;
        for explicit in [false, true] {
            let mut command = Command::new(&kuna);
            command.args([
                "decompile",
                &common::fixture("highslice_guard_x86_64"),
                "narrow_guard",
                "--json",
                "--mode",
                mode,
            ]);
            if explicit {
                command.args(["--option", "highsliceguard", "off"]);
            }
            let result = process::required_output(&mut command);
            let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
            assert!(doc["functions"][0]["error"].is_null());
            if let Some(previous) = &previous {
                assert_eq!(&doc["functions"], previous, "{mode}");
            }
            previous = Some(doc["functions"].clone());
        }
    }
}
