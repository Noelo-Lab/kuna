//! Primitive Rust format arguments retain type, order and refusal boundaries.

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::PathBuf;

fn decompile(
    function: &str,
    enabled: bool,
    rust: bool,
    evidence: bool,
    version: Option<(u16, u16, u16)>,
) -> String {
    decompile_fixture(
        "rust_fmt_args_x86_64",
        function,
        enabled,
        rust,
        evidence,
        version,
    )
}

fn decompile_fixture(
    fixture: &str,
    function: &str,
    enabled: bool,
    rust: bool,
    evidence: bool,
    version: Option<(u16, u16, u16)>,
) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let binary = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fixture);
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root.join("specs").display().to_string()],
    )
    .unwrap();
    assert_eq!(program.arch().rustc_stable_version, Some((1, 93, 1)));
    assert!(!program.arch().rust_display_formatters.is_empty());
    program.arch_mut().rustc_stable_version = version;
    program.arch_mut().rust_fmt_args = enabled;
    if !evidence {
        program.arch_mut().rust_display_formatters.clear();
    }
    program.commit_pending_analysis().unwrap();
    let language = if rust { "rust-language" } else { "c-language" };
    let commands = vec![
        format!("option setlanguage {language}"),
        format!("load function {function}"),
        "decompile".into(),
        "print C".into(),
    ];
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn compiled_rust_keeps_formatter_traits_distinct() {
    let output = |name, enabled| {
        decompile_fixture(
            "rust_fmt_args_real_x86_64",
            name,
            enabled,
            true,
            true,
            Some((1, 93, 1)),
        )
    };
    for (name, expected) in [
        ("print_one", "println!(\"value = {}\""),
        ("reorder", "println!(\"{0} {1} {0}\""),
        ("signed", "eprintln!(\"signed {}\""),
    ] {
        let on = output(name, true);
        assert!(on.contains(expected), "{name}: {on}");
        assert!(!output(name, false).contains(expected));
    }
    for name in ["exponent", "debug", "width"] {
        assert_eq!(output(name, true), output(name, false), "{name}");
    }
}

fn output(function: &str, enabled: bool) -> String {
    decompile(function, enabled, true, true, Some((1, 93, 1)))
}

#[test]
fn primitive_macros_preserve_signedness_and_indices() {
    for (function, expected, ty) in [
        ("decimal", "print!(\"{}\"", "u64"),
        ("decimal_lf", "println!(\"{}\"", "u64"),
        ("decimal_stderr", "eprint!(\"{}\"", "u64"),
        ("signed_lf", "eprintln!(\"{}\"", "i64"),
        ("narrow", "print!(\"{}\"", "u32"),
        ("reorder", "println!(\"{0} {1} {0}\"", "u32"),
    ] {
        let on = output(function, true);
        let off = output(function, false);
        assert!(on.contains(expected), "{function}: {on}");
        assert!(on.contains(&format!("*const {ty}")), "{function}: {on}");
        assert!(!off.contains(expected), "{function}: {off}");
        assert!(off.contains("std::io::stdio::_"), "{function}: {off}");
    }
}

#[test]
fn unsupported_callbacks_and_incomplete_descriptors_keep_runtime_calls() {
    for function in [
        "reject_utf8",
        "reject_head",
        "reject_flags",
        "reject_index",
        "reject_unterminated",
        "reject_mutable",
        "reject_trait",
        "reject_unknown",
        "reject_callback",
        "reject_sink",
        "reject_global_value",
        "reject_result",
        "reject_clobber",
    ] {
        assert_eq!(
            output(function, true),
            output(function, false),
            "{function}"
        );
    }
}

#[test]
fn identity_version_and_output_language_are_required() {
    for version in [None, Some((1, 92, 0)), Some((1, 93, 2))] {
        assert_eq!(
            decompile("decimal", true, true, true, version),
            decompile("decimal", false, true, true, version)
        );
    }
    assert_eq!(
        decompile("decimal", true, true, false, Some((1, 93, 1))),
        decompile("decimal", false, true, false, Some((1, 93, 1)))
    );
    assert_eq!(
        decompile("decimal", true, false, true, Some((1, 93, 1))),
        decompile("decimal", false, false, true, Some((1, 93, 1)))
    );
}

#[test]
fn emitted_macro_expressions_compile_and_reproduce_primitive_output() {
    use crate::common;
    use std::process::Command;
    let mut source = String::from("fn main() { unsafe {\n");
    for function in [
        "decimal",
        "decimal_lf",
        "decimal_stderr",
        "signed_lf",
        "narrow",
        "reorder",
    ] {
        let code = output(function, true);
        let expression = code
            .lines()
            .find(|line| line.contains("print!(") || line.contains("println!("))
            .unwrap_or_else(|| panic!("{code}"));
        source.push_str("{\n");
        if function == "reorder" {
            source.push_str("let mut v1: u32 = 99; let mut v2: u32 = 7;\n");
            source.push_str("let v3 = std::ptr::addr_of_mut!(v2); let v5 = std::ptr::addr_of_mut!(v1).cast::<u8>();\n");
        } else {
            source.push_str("let mut v1: u64 = u64::MAX;\n");
            source.push_str("let v2 = std::ptr::addr_of_mut!(v1).cast::<u8>();\n");
        }
        source.push_str(expression);
        source.push_str("\n}\n");
    }
    source.push_str("} }\n");
    let path = common::scratch_file("rustfmtargs", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, &source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_arguments"])
        .arg(&path)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{source}\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let result = Command::new(&binary).output().unwrap();
    assert!(result.status.success());
    assert_eq!(
        result.stdout,
        b"1844674407370955161518446744073709551615\n42949672957 99 7\n"
    );
    assert_eq!(result.stderr, b"18446744073709551615-1\n");
}

#[test]
fn real_macros_reproduce_the_compiled_fixture_output() {
    use crate::common;
    use std::process::Command;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut source = String::from("fn main() { unsafe {\n");
    for name in ["print_one", "reorder", "signed"] {
        let code = decompile_fixture(
            "rust_fmt_args_real_x86_64",
            name,
            true,
            true,
            true,
            Some((1, 93, 1)),
        );
        let expression = code
            .lines()
            .find(|line| line.contains("println!("))
            .unwrap_or_else(|| panic!("{code}"));
        source.push_str("{\n");
        match name {
            "reorder" => source.push_str("let mut v1 = 99u32; let mut v2 = 7u32; let v3 = std::ptr::addr_of_mut!(v2); let v5 = std::ptr::addr_of_mut!(v1).cast::<u8>();\n"),
            _ => {
                let value = if name == "signed" { "(-9223372036854775807i64) as u64" } else { "123456789u64" };
                source.push_str(&format!("let mut v1 = {value}; let v2 = std::ptr::addr_of_mut!(v1).cast::<u8>();\n"));
            }
        }
        source.push_str(expression);
        source.push_str("\n}\n");
    }
    source.push_str("} }\n");
    let path = common::scratch_file("rustfmtargs-real", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, &source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "real_arguments"])
        .arg(&path)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{source}\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let expected = Command::new(
        root.join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_args_real_x86_64"),
    )
    .output()
    .unwrap();
    let actual = Command::new(binary).output().unwrap();
    assert!(expected.status.success() && actual.status.success());
    assert_eq!(actual.stdout, expected.stdout);
    assert_eq!(actual.stderr, expected.stderr);
}
