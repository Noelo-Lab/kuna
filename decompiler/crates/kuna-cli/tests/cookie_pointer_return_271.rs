//! A cookie-check call must not erase a preceding pointer return.
#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

mod common;

use object::{Object, ObjectSymbol};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

fn available_compilers() -> Vec<&'static str> {
    ["gcc", "clang"]
        .into_iter()
        .filter(|cc| common::process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect()
}

fn assemble_image(compiler: &str) -> PathBuf {
    let source = common::fixture("cookie_pointer_return_271.S");
    let image = common::scratch_file("cookie-pointer-return-271", "elf");
    common::process::required_output(
        Command::new(compiler)
            .args(["-nostdlib", "-no-pie", "-Wl,-e,pointer_after_checks"])
            .arg(source)
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

/// Leave ordinary callees inferred so their memory guards retain the cookie chain.
fn decompile(image: &Path, options: &[(&str, &str)]) -> String {
    let address = |symbol| format!("0x{:x}", symbol_address(image, symbol));
    let contracts = [
        format!(
            "prototype {} void * pointer_after_checks(void)",
            address("pointer_after_checks")
        ),
        format!(
            "prototype {} void * make_cookie_object(void)",
            address("make_cookie_object")
        ),
        format!(
            "prototype {} void cookie_check(unsigned long long cookie)",
            address("cookie_check")
        ),
    ];
    let specs = common::repo_root().join("specs");
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args([
        "decompile",
        image.to_str().unwrap(),
        "pointer_after_checks",
        "--mode",
        "reliable",
        "--json",
        "--assert-strict",
        "--sleighpath",
        specs.to_str().unwrap(),
    ]);
    for directive in &contracts {
        command.args(["--assert", directive]);
    }
    for (option, value) in options {
        command.args(["--option", option, value]);
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(25),
    )
    .expect("bounded cookie-pointer decompilation");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let document: Value = serde_json::from_slice(&output.stdout).expect("Kuna JSON output");
    assert_eq!(
        document["assertions"].as_array().unwrap().len(),
        contracts.len(),
        "{document}"
    );
    assert!(
        document["assertions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["status"] == "applied"),
        "prototype contract was not applied: {document}"
    );
    assert!(document["functions"][0]["error"].is_null(), "{document}");
    document["functions"][0]["code"]
        .as_str()
        .expect("function C output")
        .to_owned()
}

fn returns_factory_pointer(code: &str) -> bool {
    if code
        .lines()
        .any(|line| line.contains("return make_cookie_object();"))
    {
        return true;
    }
    code.lines().any(|assignment| {
        let Some((lhs, _)) = assignment.split_once("= make_cookie_object();") else {
            return false;
        };
        let Some(name) = lhs.split_whitespace().last() else {
            return false;
        };
        let name = name.trim_start_matches('*');
        code.lines().any(|line| {
            let line = line.trim_start();
            line.starts_with(&format!("return {name};"))
                || line.starts_with(&format!("return (void *){name};"))
        })
    })
}

fn c_round_trip(strip_code: &str, source: &Path, compiler: &str) {
    let asm_object = common::scratch_file("cookie-pointer-return-native-alias", "o");
    common::process::required_output(
        Command::new(compiler)
            .args(["-DENTRY_NAME=native_pointer_after_checks", "-c"])
            .arg(source)
            .arg("-o")
            .arg(&asm_object),
    );

    let c_source = common::scratch_file("cookie-pointer-return-roundtrip", "c");
    let executable = common::scratch_file("cookie-pointer-return-roundtrip", "exe");
    let harness = format!(
        r#"#include <stdint.h>
#include <stddef.h>
typedef int8_t int1; typedef int16_t int2; typedef int32_t int4; typedef int64_t int8;
typedef uint8_t uint1; typedef uint16_t uint2; typedef uint32_t uint4; typedef uint64_t uint8;
typedef uint8_t undefined1; typedef uint16_t undefined2; typedef uint32_t undefined4; typedef uint64_t undefined8;
extern void *make_cookie_object(void);
extern void *native_pointer_after_checks(void);
extern void cookie_noop(void);
extern volatile unsigned long long cookie_noop_count;
{strip_code}
int main(void) {{
    void *expected = make_cookie_object();
    cookie_noop_count = 0;
    if (native_pointer_after_checks() != expected) return 1;
    if (cookie_noop_count != 271) return 2;
    cookie_noop_count = 0;
    if (pointer_after_checks() != expected) return 3;
    if (cookie_noop_count != 271) return 4;
    return 0;
}}
"#
    );
    std::fs::write(&c_source, harness).expect("write round-trip harness");
    for level in ["-O0", "-O2"] {
        common::process::required_output(
            Command::new(compiler)
                .args([
                    "-std=c11",
                    "-Werror=int-conversion",
                    "-Werror=incompatible-pointer-types",
                    level,
                ])
                .arg(&c_source)
                .arg(&asm_object)
                .arg("-no-pie")
                .arg("-o")
                .arg(&executable),
        );
        common::process::required_output(&mut Command::new(&executable));
    }
    for path in [asm_object, c_source, executable] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn checker_preserves_or_the_cookie_option_recovers_the_pointer_return() {
    let compilers = available_compilers();
    let Some(image_compiler) = compilers.first().copied() else {
        eprintln!("no GCC or Clang available; skipping cookie-pointer fixture");
        return;
    };
    let source = PathBuf::from(common::fixture("cookie_pointer_return_271.S"));
    let image = assemble_image(image_compiler);

    let legacy = decompile(
        &image,
        &[("msvcstackguard", "off"), ("calleeretpreserves", "off")],
    );
    assert!(
        legacy.contains("cookie_check("),
        "checker unexpectedly removed:\n{legacy}"
    );
    assert!(
        legacy
            .lines()
            .any(|line| line.trim().starts_with("make_cookie_object();")),
        "generic call guarding no longer exposes the lost result:\n{legacy}"
    );
    assert!(
        !returns_factory_pointer(&legacy),
        "legacy output uses the factory result:\n{legacy}"
    );

    let preserved = decompile(
        &image,
        &[("msvcstackguard", "off"), ("calleeretpreserves", "on")],
    );
    assert!(
        preserved.contains("cookie_check("),
        "checker unexpectedly removed:\n{preserved}"
    );
    assert!(
        returns_factory_pointer(&preserved),
        "callee return preservation lost pointer identity:\n{preserved}"
    );

    let stripped = decompile(
        &image,
        &[("msvcstackguard", "on"), ("calleeretpreserves", "off")],
    );
    assert!(
        !stripped.contains("cookie_check("),
        "recognized checker remains:\n{stripped}"
    );
    assert!(
        returns_factory_pointer(&stripped),
        "cookie stripping lost factory pointer identity:\n{stripped}"
    );
    assert_eq!(
        stripped
            .lines()
            .filter(|line| line.trim() == "cookie_noop();")
            .count(),
        271,
        "the emitted C must retain all ordinary calls:\n{stripped}"
    );

    for compiler in compilers {
        c_round_trip(&stripped, &source, compiler);
    }
    std::fs::remove_file(image).unwrap();
}
