//! Win64 XMM6-XMM9 saves stay out of C while the caller-owned Rect remains live.
//! GCC's O0 helper uses home spills; Clang's O0 helper uses private frame spills.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

mod common;

use object::{Object, ObjectSymbol};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const TARGET: &str = "x86:LE:64:default:windows";
const SOURCE_LINK: &str = "-Wl,--section-start=.saved_xmm_rect=0x700000";
const SAVED_LANES: &[i64] = &[
    -0x48, -0x44, -0x40, -0x3c, -0x38, -0x34, -0x30, -0x2c, -0x28, -0x24, -0x20, -0x1c, -0x18,
    -0x14, -0x10, -0xc,
];

fn compilers() -> Vec<&'static str> {
    ["gcc", "clang"]
        .into_iter()
        .filter(|cc| {
            common::process::optional_output(&mut Command::new(cc).arg("--version")).is_some()
        })
        .collect()
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(common::fixture(name))
}

fn required(command: &mut Command) {
    common::process::required_output(command);
}

fn build_native(compiler: &str, optimization: &str) -> PathBuf {
    build_native_case(compiler, optimization, &[])
}

fn build_native_case(compiler: &str, optimization: &str, defines: &[&str]) -> PathBuf {
    let image = common::scratch_file("saved-xmm-msabi-native", "elf");
    required(
        Command::new(compiler)
            .args(["-std=c11", optimization, "-no-pie", SOURCE_LINK])
            .args(defines)
            .arg(fixture("savedxmm_msabi.S"))
            .arg(fixture("savedxmm_msabi.c"))
            .arg("-o")
            .arg(&image),
    );
    image
}

fn symbol_address(image: &Path, name: &str) -> u64 {
    let bytes = std::fs::read(image).expect("read fixture ELF");
    let file = object::File::parse(bytes.as_slice()).expect("parse fixture ELF");
    file.symbols()
        .find(|symbol| symbol.name().ok() == Some(name))
        .unwrap_or_else(|| panic!("fixture has no ELF symbol {name}"))
        .address()
}

fn decompile(image: &Path) -> Value {
    let address = |name| format!("0x{:x}", symbol_address(image, name));
    let assertions = [
        "typedef struct Rect { float x; float y; float width; float height; };".to_string(),
        format!(
            "prototype {} float fixture_spills_rect(void)",
            address("fixture_spills_rect")
        ),
        format!(
            "prototype {} void fixture_copy_rect(Rect *destination,const Rect *source)",
            address("fixture_copy_rect")
        ),
        format!("data {} Rect rect_source", address("rect_source")),
    ];
    let specs = common::repo_root().join("specs");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile",
        image.to_str().unwrap(),
        "fixture_spills_rect",
        "--target",
        TARGET,
        "--mode",
        "reliable",
        "--json",
        "--assert-strict",
        "--option",
        "stackviews",
        "on",
        "--option",
        "structdefs",
        "on",
        "--sleighpath",
        specs.to_str().unwrap(),
    ]);
    for assertion in &assertions {
        command.args(["--assert", assertion]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(25),
    )
    .expect("bounded saved-XMM decompilation");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let document: Value = serde_json::from_slice(&output.stdout).expect("Kuna JSON output");
    assert_eq!(
        document["assertions"].as_array().unwrap().len(),
        assertions.len(),
        "{document}"
    );
    assert!(
        document["assertions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["status"] == "applied"),
        "prototype or type contract was not applied: {document}"
    );
    document
}

fn overlaps_saved_lane(offset: i64, size: i64) -> bool {
    SAVED_LANES
        .iter()
        .any(|lane| offset < lane + 4 && offset + size > *lane)
}

fn stack_offsets_in_comments(code: &str) -> Vec<i64> {
    code.lines()
        .filter_map(|line| {
            let comment = line.split_once("//")?.1;
            let stack = comment.split_once("stack")?.1.trim_start();
            let (sign, value) = if let Some(value) = stack.strip_prefix('-') {
                (-1, value.trim_start())
            } else if let Some(value) = stack.strip_prefix('+') {
                (1, value.trim_start())
            } else {
                return None;
            };
            let value = value.strip_prefix("0x")?;
            let digits: String = value
                .chars()
                .take_while(|ch| ch.is_ascii_hexdigit())
                .collect();
            let magnitude = i64::from_str_radix(&digits, 16).ok()?;
            Some(sign * magnitude)
        })
        .collect()
}

fn split_emitted_code(code: &str) -> (&str, &str) {
    let signature = "float4 fixture_spills_rect(";
    let body_start = code
        .find(signature)
        .unwrap_or_else(|| panic!("missing emitted function signature: {code}"));
    let (prefix, body) = code.split_at(body_start);
    assert!(
        prefix.contains("Rect"),
        "structdefs must emit the Rect preamble before the function: {code}"
    );
    (prefix, body)
}

fn function<'a>(document: &'a Value, name: &str) -> &'a Value {
    document["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == name)
        .unwrap_or_else(|| panic!("missing function {name}: {document}"))
}

fn emitted_c_round_trip(
    function_body: &str,
    type_prefix: &str,
    compiler: &str,
    optimization: &str,
) {
    let source = common::scratch_file("saved-xmm-msabi-emitted", "c");
    let executable = common::scratch_file("saved-xmm-msabi-emitted", "elf");
    let fixture_c = fixture("savedxmm_msabi.c");
    let include_path = fixture_c
        .to_str()
        .unwrap()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    let translation_unit = format!(
        "#include <stdint.h>\ntypedef float float4;\n__attribute__((ms_abi)) float fixture_spills_rect(void);\n{type_prefix}\n#define KUNA_EMITTED_RECT_DEFINED 1\n#include \"{include_path}\"\n__attribute__((ms_abi))\n{function_body}\n"
    );
    std::fs::write(&source, translation_unit).expect("write emitted-C harness");
    required(
        Command::new(compiler)
            .args([
                "-std=c11",
                optimization,
                "-no-pie",
                SOURCE_LINK,
                "-Werror=int-conversion",
                "-Werror=incompatible-pointer-types",
                "-DSAVED_XMM_FUNCTION=native_fixture_spills_rect",
                "-DSAVED_XMM_PROBE_TARGET=fixture_spills_rect",
            ])
            .arg(fixture("savedxmm_msabi.S"))
            .arg(&source)
            .arg("-o")
            .arg(&executable),
    );
    required(&mut Command::new(&executable));
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
}

#[test]
fn saved_xmm_spills_are_not_emitted_as_live_locals() {
    let compilers = compilers();
    let Some(first_compiler) = compilers.first().copied() else {
        eprintln!("no GCC or Clang available; skipping saved-XMM fixture");
        return;
    };

    let mut decompile_image = None;
    let mut unoptimized_image = None;
    for compiler in &compilers {
        for optimization in ["-O0", "-O2"] {
            let image = build_native(compiler, optimization);
            required(&mut Command::new(&image));
            if decompile_image.is_none() && *compiler == first_compiler && optimization == "-O2" {
                decompile_image = Some(image);
            } else if *compiler == "gcc" && optimization == "-O0" {
                unoptimized_image = Some(image);
            } else {
                std::fs::remove_file(image).unwrap();
            }
        }
    }
    let image = decompile_image.expect("built native decompilation fixture");
    assert_eq!(symbol_address(&image, "rect_source"), 0x700000);
    for image in std::iter::once(image).chain(unoptimized_image) {
        let document = decompile(&image);
        let target = function(&document, "fixture_spills_rect");
        assert!(target["error"].is_null(), "{document}");
        let code = target["code"].as_str().unwrap();
        let (type_prefix, function_body) = split_emitted_code(code);
        assert!(!code.contains("WARNING:"), "{code}");
        assert!(
            code.lines().any(|line| {
                line.contains("fixture_copy_rect(")
                    && (line.contains("rect_source") || line.contains("0x700000"))
            }),
            "the disjoint caller-owned Rect must still reach its helper: {code}"
        );
        assert!(
            code.lines().any(|line| line.contains("return ")
                && line.contains(".x + ")
                && line.contains(".y;")),
            "the emitted result must remain the 3.75f rectangle expression: {code}"
        );
        for variable in target["variables"].as_array().unwrap() {
            let name = variable["name"].as_str().unwrap();
            if !function_body
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .any(|word| word == name)
            {
                continue;
            }
            if let Some(offset) = variable["stack_offset"].as_i64() {
                let size = variable["size"].as_i64().unwrap_or(1).max(1);
                assert!(
                !overlaps_saved_lane(offset, size),
                "saved Win64 XMM lanes became emitted storage at {offset:#x} size {size}: {code}"
            );
            }
        }
        for offset in stack_offsets_in_comments(code) {
            assert!(
                !overlaps_saved_lane(offset, 4),
                "saved Win64 XMM lane at {offset:#x} remains in an emitted storage comment: {code}"
            );
        }

        for compiler in &compilers {
            for optimization in ["-O0", "-O2"] {
                emitted_c_round_trip(function_body, type_prefix, compiler, optimization);
            }
        }
        std::fs::remove_file(image).unwrap();
    }

    for compiler in &compilers {
        for define in [
            "-DSAVED_XMM_READ_HOME",
            "-DSAVED_XMM_ESCAPE_HOME",
            "-DSAVED_XMM_ESCAPE_FRAME_LOW",
        ] {
            let image = build_native_case(compiler, "-O0", &[define]);
            required(&mut Command::new(&image));
            let document = decompile(&image);
            let target = function(&document, "fixture_spills_rect");
            assert!(target["error"].is_null(), "{document}");
            let code = target["code"].as_str().unwrap();
            let offsets = stack_offsets_in_comments(code);
            for start in [-0x48, -0x38, -0x28, -0x18] {
                assert!(
                    offsets.iter().any(|offset| (start..start + 16).contains(offset)),
                    "observing homes or exposing frame addresses must retain every saved XMM register ({define}, {start:#x}): {code}"
                );
            }
            std::fs::remove_file(image).unwrap();
        }
    }
}
