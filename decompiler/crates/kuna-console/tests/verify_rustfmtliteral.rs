//! Rust 1.93 literal formatting: exact output and refusal boundaries.

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
    version: Option<(u16, u16, u16)>,
) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixture = if function.starts_with("scoped_") {
        "rust_fmt_literal_scopes_x86_64"
    } else {
        "rust_fmt_literals_x86_64"
    };
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
    program.arch_mut().rustc_stable_version = version;
    program.arch_mut().rust_fmt_literal = enabled;
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
    let data = status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap();
    data.conf = Some(program);
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn literal_macros_preserve_output_bytes() {
    for (name, expected) in [
        ("literal_lf", "println!(\"just a literal\");"),
        ("literal_plain", "print!(\"plain\");"),
        ("literal_stderr", "eprint!(\"plain\");"),
        ("literal_stderr_lf", "eprintln!(\"just a literal\");"),
        ("literal_empty", "print!(\"\");"),
        ("literal_unicode", "println!(\"café\");"),
        ("literal_braces", "println!(\"{{a}}\\\"\\\\\\0\\n\");"),
    ] {
        let on = decompile(name, true, true, Some((1, 93, 1)));
        let off = decompile(name, false, true, Some((1, 93, 1)));
        assert!(on.contains(expected), "{name}: {on}");
        assert!(!off.contains(expected), "{name}: {off}");
        assert!(off.contains("std::io::stdio::_"), "{name}: {off}");
    }
}

#[test]
fn invalid_literals_and_unrelated_sinks_keep_runtime_calls() {
    for name in [
        "reject_utf8",
        "reject_mutable",
        "reject_template",
        "reject_large",
        "reject_sink",
        "reject_null",
        "reject_unmapped_span",
        "reject_variable",
        "reject_result",
        "reject_extra_argument",
    ] {
        assert_eq!(
            decompile(name, false, true, Some((1, 93, 1))),
            decompile(name, true, true, Some((1, 93, 1))),
            "{name}"
        );
    }
}

#[test]
fn unknown_or_other_versions_and_c_output_are_unchanged() {
    for version in [None, Some((1, 92, 0)), Some((1, 94, 0)), Some((1, 93, 2))] {
        assert_eq!(
            decompile("literal_lf", false, true, version),
            decompile("literal_lf", true, true, version)
        );
    }
    assert_eq!(
        decompile("literal_lf", false, false, Some((1, 93, 1))),
        decompile("literal_lf", true, false, Some((1, 93, 1)))
    );
}

#[test]
fn emitted_macros_compile_and_reproduce_stdout_and_stderr() {
    use crate::common;
    use std::process::Command;
    let mut source = String::new();
    let mut main = String::from("fn main() { unsafe {");
    for function in [
        "literal_lf",
        "literal_plain",
        "literal_stderr",
        "literal_stderr_lf",
        "literal_empty",
        "literal_unicode",
        "literal_braces",
        "scoped_stdout",
        "scoped_stderr",
    ] {
        let output = decompile(function, true, true, Some((1, 93, 1)));
        source.push_str(&output[output.find("#[allow(").unwrap()..]);
        main.push_str(&format!("let _ = {function}();"));
    }
    source.push_str(&main);
    source.push_str("} }\n");
    let path = common::scratch_file("rustfmtliteral", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_print"])
        .arg(&path)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let result = Command::new(&binary).output().unwrap();
    assert!(result.status.success());
    assert_eq!(
        result.stdout,
        b"just a literal\nplaincaf\xc3\xa9\n{a}\"\\\0\n\nscoped literal\n"
    );
    assert_eq!(result.stderr, b"plainjust a literal\nscoped literal\n");
}

#[test]
fn relocated_sinks_require_the_exact_standard_namespace() {
    for (name, expected) in [
        ("scoped_stdout", "println!(\"scoped literal\");"),
        ("scoped_stderr", "eprintln!(\"scoped literal\");"),
    ] {
        let on = decompile(name, true, true, Some((1, 93, 1)));
        let off = decompile(name, false, true, Some((1, 93, 1)));
        assert!(on.contains(expected), "{name}: {on}");
        assert!(!off.contains(expected), "{name}: {off}");
        assert_eq!(
            decompile(name, true, false, Some((1, 93, 1))),
            decompile(name, false, false, Some((1, 93, 1)))
        );
    }
    for name in [
        "scoped_other",
        "scoped_incomplete",
        "scoped_nested",
        "scoped_global",
    ] {
        assert_eq!(
            decompile(name, true, true, Some((1, 93, 1))),
            decompile(name, false, true, Some((1, 93, 1))),
            "{name}"
        );
    }
}
