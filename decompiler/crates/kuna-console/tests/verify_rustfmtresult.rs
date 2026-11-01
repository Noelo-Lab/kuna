//! String result macros preserve hidden storage, returns and exact contents.

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::architecture::Architecture;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn decompile(
    real: bool,
    function: &str,
    enabled: bool,
    language: &str,
    configure: impl FnOnce(&mut Architecture),
) -> String {
    let fixture = if real {
        "rust_fmt_result_real_x86_64"
    } else {
        "rust_fmt_result_x86_64"
    };
    let binary = root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fixture);
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    assert_eq!(program.arch().rustc_stable_version, Some((1, 93, 1)));
    assert!(!program.arch().rust_integer_formatters.is_empty());
    program.arch_mut().rust_fmt_result = enabled;
    configure(program.arch_mut());
    program.commit_pending_analysis().unwrap();
    let commands = vec![
        format!("option setlanguage {language}-language"),
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

fn output(real: bool, function: &str, enabled: bool) -> String {
    decompile(real, function, enabled, "rust", |_| {})
}

#[test]
fn native_result_macros_keep_initialization_returns_and_newlines() {
    for (name, literal) in [
        ("hex_width", "format!(\"0x{:08x} {:>5}\""),
        ("decimal", "format!(\"value {}\\n\""),
        ("reordered", "format!(\"{0:#06x}/{1:+}/{0:#06x}\""),
    ] {
        let on = output(true, name, true);
        let off = output(true, name, false);
        assert!(on.contains(literal), "{name}: {on}");
        assert!(on.contains("std::ptr::write("), "{name}: {on}");
        assert!(
            on.contains("return a0;") && off.contains("return a0;"),
            "{name}: {on}"
        );
        assert!(!off.contains(literal), "{name}: {off}");
    }
    for name in ["result_decimal", "result_lf", "result_scoped"] {
        let on = output(false, name, true);
        assert!(on.contains("std::ptr::write("), "{name}: {on}");
        assert!(!output(false, name, false).contains("std::ptr::write("));
    }
}

#[test]
fn local_or_unknown_storage_and_unsupported_arguments_stay_explicit() {
    for name in ["local", "constant_value", "generic", "dynamic", "floating"] {
        let on = output(true, name, true);
        assert_eq!(on, output(true, name, false), "{name}");
        assert!(on.contains("format_inner("), "{name}: {on}");
    }
    for name in [
        "reject_root",
        "reject_missing",
        "reject_extra",
        "reject_global",
        "reject_null",
        "reject_constant",
        "reject_local",
        "reject_unaligned",
        "reject_computed",
        "reject_returned",
        "reject_value",
        "reject_template",
        "reject_mutable",
        "reject_missing_cell",
    ] {
        let on = output(false, name, true);
        assert_eq!(on, output(false, name, false), "{name}");
        assert!(on.contains("format_inner("), "{name}: {on}");
    }
}

#[test]
fn result_recovery_retains_version_language_and_option_boundaries() {
    for version in [None, Some((1, 92, 0)), Some((1, 93, 2))] {
        let run = |enabled| {
            decompile(true, "hex_width", enabled, "rust", |arch| {
                arch.rustc_stable_version = version
            })
        };
        assert_eq!(run(true), run(false));
    }
    let no_parent = |enabled| {
        decompile(true, "hex_width", enabled, "rust", |arch| {
            arch.rust_fmt_args = false
        })
    };
    assert_eq!(no_parent(true), no_parent(false));
    let no_flags = |enabled| {
        decompile(true, "hex_width", enabled, "rust", |arch| {
            arch.rust_fmt_flags = false
        })
    };
    assert_eq!(no_flags(true), no_flags(false));
    assert!(
        decompile(true, "decimal", true, "rust", |arch| arch.rust_fmt_flags =
            false)
        .contains("format!(")
    );
    let no_identity = |enabled| {
        decompile(true, "hex_width", enabled, "rust", |arch| {
            arch.rust_integer_formatters.clear()
        })
    };
    assert_eq!(no_identity(true), no_identity(false));
    assert_eq!(
        decompile(true, "hex_width", true, "c", |_| {}),
        decompile(true, "hex_width", false, "c", |_| {})
    );
    assert!(decompile(true, "hex_width", true, "rust", |arch| arch
        .rustc_stable_version =
        Some((1, 93, 0)))
    .contains("format!("));
}

fn compile_and_run(source: &str, stem: &str) -> std::process::Output {
    use crate::common;
    use std::process::Command;
    let path = common::scratch_file(stem, "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_results"])
        .arg(path)
        .arg("-o")
        .arg(&binary)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{source}\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let result = Command::new(binary).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result
}

#[test]
fn recovered_result_expressions_match_native_string_contents() {
    use std::collections::BTreeMap;
    use std::process::Command;
    let pointer =
        regex::Regex::new(r"\(\((v[0-9]+) as \*const _\) as \*const ([ui](?:8|16|32|64))\)")
            .unwrap();
    let mut source = String::from("fn main() { unsafe {\n");
    for name in ["hex_width", "decimal", "reordered"] {
        let code = output(true, name, true);
        let expression = code
            .lines()
            .find(|line| line.contains("std::ptr::write("))
            .unwrap_or_else(|| panic!("{code}"));
        source.push_str("{ let mut result = std::mem::MaybeUninit::<String>::uninit(); let a0 = result.as_mut_ptr() as u64;\n");
        let pointers: BTreeMap<_, _> = pointer
            .captures_iter(expression)
            .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
            .collect();
        assert!(!pointers.is_empty(), "{expression}");
        for (var, ty) in pointers {
            let value = match name {
                "hex_width" => "31",
                "decimal" => "-123",
                "reordered" if ty == "u16" => "99",
                "reordered" => "-7",
                _ => unreachable!(),
            };
            source.push_str(&format!("let mut {var}_value: {ty} = {value}; let {var} = std::ptr::addr_of_mut!({var}_value);\n"));
        }
        source.push_str(expression);
        source.push_str(
            "\nlet result = result.assume_init(); println!(\"{:?}\", result); drop(result); }\n",
        );
    }
    source.push_str("} }\n");
    let actual = compile_and_run(&source, "rustfmtresult-native");
    let expected = Command::new(
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_result_real_x86_64"),
    )
    .output()
    .unwrap();
    assert!(expected.status.success());
    assert_eq!(actual.stdout, expected.stdout);
    assert_eq!(actual.stderr, expected.stderr);
}

#[test]
fn pointer_and_integer_destinations_initialize_exactly_one_string() {
    let mut source = String::from("fn main() { unsafe {\n");
    for name in ["result_decimal", "result_lf", "result_scoped"] {
        let code = output(false, name, true);
        let expression = code
            .lines()
            .find(|line| line.contains("std::ptr::write("))
            .unwrap_or_else(|| panic!("{code}"));
        source.push_str("{ let mut result = std::mem::MaybeUninit::<String>::uninit();\n");
        if expression.contains("(a0 as *mut _)") {
            source.push_str("let a0 = result.as_mut_ptr().cast::<u64>();\n");
        } else {
            source.push_str("let a0 = result.as_mut_ptr() as u64;\n");
        }
        source.push_str("let mut v1 = u64::MAX; let v2 = std::ptr::addr_of_mut!(v1);\n");
        source.push_str(expression);
        source.push_str(
            "\nlet result = result.assume_init(); println!(\"{:?}\", result); drop(result); }\n",
        );
    }
    source.push_str("} }\n");
    let actual = compile_and_run(&source, "rustfmtresult-destinations");
    assert_eq!(
        actual.stdout,
        b"\"18446744073709551615\"\n\"18446744073709551615\\n\"\n\"18446744073709551615\"\n"
    );
    assert!(actual.stderr.is_empty());
}
