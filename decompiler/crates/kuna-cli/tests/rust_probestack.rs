use crate::common;
use common::process;
use std::path::Path;
use std::process::Command;

const POSITIVE: &[&str] = &[
    "probe_page",
    "probe_tail",
    "probe_pages",
    "probe_observes_r11",
    "probe_zero",
    "probe_one",
    "probe_page_r11",
    "probe_plus_one",
    "probe_pages_r11",
    "probe_saved_rbp",
    "probe_return_slot",
];
const CONTROLS: &[&str] = &["probe_side_effect", "probe_observes_flags", "probe_unknown"];
const INPUTS: &[u32] = &[
    0,
    1,
    7,
    12345,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_fff8,
    u32::MAX,
];

fn records(mode: &str, language: &str) -> serde_json::Value {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let facts = common::fixture("rust_probestack.kuna");
    let selector = POSITIVE
        .iter()
        .chain(CONTROLS)
        .copied()
        .collect::<Vec<_>>()
        .join(",");
    let result = process::required_output(Command::new(binary).args([
        "decompile-all",
        &common::fixture("rust_probestack_x86_64"),
        "--json",
        "--language",
        language,
        "--option",
        "rustprobestack",
        mode,
        "--functions",
        &selector,
        "--assert",
        &format!("@{facts}"),
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let functions = doc["functions"].as_array().unwrap();
    assert_eq!(functions.len(), POSITIVE.len() + CONTROLS.len());
    for function in functions {
        assert!(function["error"].is_null(), "{function}");
    }
    doc
}

fn function<'a>(doc: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == name)
        .unwrap()
}

#[test]
fn only_verified_probes_change_in_every_output_language() {
    for language in ["c", "rust", "go"] {
        let off = records("off", language);
        let auto = records("auto", language);
        let always = records("always", language);
        assert_eq!(auto["functions"], always["functions"]);
        for name in POSITIVE {
            assert!(function(&off, name)["code"]
                .as_str()
                .unwrap()
                .contains("__rust_probestack"));
            assert!(!function(&auto, name)["code"]
                .as_str()
                .unwrap()
                .contains("__rust_probestack"));
        }
        for name in CONTROLS {
            assert_eq!(function(&off, name), function(&auto, name));
        }
    }
}

fn values(binary: &Path, input: u32) -> Vec<u32> {
    let result = process::required_output(Command::new(binary).arg(input.to_string()));
    String::from_utf8(result.stdout)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_emitted_bodies_preserve_values_and_probe_stack_writes() {
    let auto = records("auto", "c");
    let bodies = POSITIVE
        .iter()
        .map(|name| function(&auto, name)["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let source = common::scratch_file("rust-probestack", "c");
    let calls = POSITIVE
        .iter()
        .map(|name| format!("{name}(x)"))
        .collect::<Vec<_>>()
        .join(",");
    std::fs::write(&source, format!(
        "#include <stdint.h>\n#include <stdio.h>\n#include <stdlib.h>\n{bodies}\nint main(int argc,char **argv) {{ unsigned int x=(unsigned int)strtoul(argv[1],0,10); printf(\"{}\\n\",{calls}); }}\n",
        vec!["%u"; POSITIVE.len()].join(" ")
    )).unwrap();
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
            for &input in INPUTS {
                let native = values(Path::new(&common::fixture("rust_probestack_x86_64")), input);
                assert_eq!(
                    values(&binary, input),
                    native[..POSITIVE.len()],
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
fn helper_spill_does_not_truncate_the_callers_reused_page() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let facts = common::fixture("rust_probestack_buffer.kuna");
    let result = process::required_output(Command::new(kuna).args([
        "decompile",
        &common::fixture("rust_probestack_buffer_x86_64"),
        "probe_buffer",
        "--json",
        "--language",
        "c",
        "--option",
        "rustprobestack",
        "auto",
        "--assert",
        &format!("@{facts}"),
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let body = doc["functions"][0]["code"].as_str().unwrap();
    assert!(body.contains("[4096]"), "{body}");
    assert!(!body.contains("__rust_probestack"), "{body}");
    let helper = include_str!("../../kuna-analysis/tests/fixtures/rust_probestack_buffer.c");
    let source = common::scratch_file("rust-probestack-buffer", "c");
    std::fs::write(&source, format!(
        "#include <stdio.h>\n#include <stdlib.h>\n{helper}\n{body}\nint main(int argc,char **argv) {{ unsigned int x=(unsigned int)strtoul(argv[1],0,10); printf(\"%u\\n\",probe_buffer(x)); }}\n"
    )).unwrap();
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
                        &format!("-O{optimization}"),
                        "-fsanitize=address",
                        "-fno-omit-frame-pointer",
                        "-no-pie",
                    ])
                    .arg(&source)
                    .arg("-o")
                    .arg(&binary),
            );
            for &input in INPUTS {
                let expected = values(
                    Path::new(&common::fixture("rust_probestack_buffer_x86_64")),
                    input,
                );
                let output = process::required_output(
                    Command::new(&binary)
                        .arg(input.to_string())
                        .env("ASAN_OPTIONS", "detect_leaks=0"),
                );
                let value: u32 = String::from_utf8(output.stdout)
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                assert_eq!(
                    [value],
                    expected.as_slice(),
                    "{compiler} O{optimization}, {input}"
                );
            }
            std::fs::remove_file(binary).unwrap();
        }
    }
    std::fs::remove_file(source).unwrap();
}

#[test]
fn optional_stack_memory_policies_retain_the_probe_and_whole_page() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let facts = common::fixture("rust_probestack_buffer.kuna");
    for policy in ["stackviews", "stackalias"] {
        let mut previous = None;
        for mode in ["off", "auto", "always"] {
            let output = process::required_output(Command::new(&kuna).args([
                "decompile",
                &common::fixture("rust_probestack_buffer_x86_64"),
                "probe_buffer",
                "--json",
                "--language",
                "c",
                "--assert",
                &format!("@{facts}"),
                "--option",
                policy,
                "on",
                "--option",
                "rustprobestack",
                mode,
            ]));
            let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let body = doc["functions"][0]["code"].as_str().unwrap();
            assert!(body.contains("[4096]"), "{policy} {mode}: {body}");
            assert!(body.contains("__rust_probestack();"), "{body}");
            if let Some(previous) = &previous {
                assert_eq!(&doc["functions"], previous);
            }
            previous = Some(doc["functions"].clone());
        }
    }
}

#[test]
fn defaults_and_presets_keep_the_summary_explicit() {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    for mode in ["auto", "reliable", "aggressive", "fast"] {
        let output = process::required_output(Command::new(&kuna).args([
            "decompile",
            &common::fixture("rust_probestack_x86_64"),
            "probe_page",
            "--language",
            "c",
            "--mode",
            mode,
            "--json",
        ]));
        let doc: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(doc["functions"][0]["code"]
            .as_str()
            .unwrap()
            .contains("__rust_probestack();"));
    }
}
