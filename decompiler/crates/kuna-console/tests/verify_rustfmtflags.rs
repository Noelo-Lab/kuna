//! Static integer formats preserve native output and the runtime refusal boundary.

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
        "rust_fmt_flags_real_x86_64"
    } else {
        "rust_fmt_flags_x86_64"
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
    program.arch_mut().rust_fmt_flags = enabled;
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
fn native_traits_flags_and_repeated_arguments_recover() {
    for (name, expected) in [
        (
            "traits",
            "println!(\"{} {:?} {:x} {:X} {:o} {:b} {:e} {:E}\"",
        ),
        ("signed", "eprintln!(\"{:+07} {:-5} {:#08x} {:#010b}\""),
        ("align", "println!(\"|{:·<8}|{:🦀^10}|{:>7.3}|\""),
        ("debug_hex", "println!(\"{:#x?} {:#X?}\""),
    ] {
        let on = output(true, name, true);
        assert!(on.contains(expected), "{name}: {on}");
        assert!(!output(true, name, false).contains(expected));
    }
    let reordered = output(true, "reorder", true);
    assert!(
        reordered.contains("{0:#08x} {1:+06} {2:>5o} {1:+06}"),
        "{reordered}"
    );
}

#[test]
fn malformed_flags_and_unknown_callbacks_keep_runtime_calls() {
    for name in ["dynamic", "generic", "floating", "pointer"] {
        assert_eq!(
            output(true, name, true),
            output(true, name, false),
            "{name}"
        );
    }
    for name in [
        "reject_unicode",
        "reject_large_unicode",
        "reject_brace",
        "reject_fill",
        "reject_reserved",
        "reject_signs",
        "reject_debug_modes",
        "reject_debug_trait",
        "reject_disabled_width",
        "reject_disabled_precision",
        "reject_zero_width",
        "reject_dynamic_width",
        "reject_dynamic_precision",
        "reject_alias",
    ] {
        let on = output(false, name, true);
        assert_eq!(on, output(false, name, false), "{name}");
        assert!(on.contains("std::io::stdio::_print("), "{name}: {on}");
    }
}

#[test]
fn version_identity_language_and_parent_option_are_required() {
    for version in [None, Some((1, 92, 0)), Some((1, 93, 2))] {
        let run = |enabled| {
            decompile(false, "hex_width", enabled, "rust", |arch| {
                arch.rustc_stable_version = version
            })
        };
        assert_eq!(run(true), run(false));
    }
    let no_identity = |enabled| {
        decompile(false, "hex_width", enabled, "rust", |arch| {
            arch.rust_integer_formatters.clear()
        })
    };
    assert_eq!(no_identity(true), no_identity(false));
    let no_parent = |enabled| {
        decompile(false, "hex_width", enabled, "rust", |arch| {
            arch.rust_fmt_args = false
        })
    };
    assert_eq!(no_parent(true), no_parent(false));
    assert_eq!(
        decompile(false, "hex_width", true, "c", |_| {}),
        decompile(false, "hex_width", false, "c", |_| {})
    );
    assert!(decompile(false, "hex_width", true, "rust", |arch| arch
        .rustc_stable_version =
        Some((1, 93, 0)))
    .contains("println!(\"{:08x}\""));
}

#[test]
fn recovered_expressions_reproduce_native_stdout_and_stderr() {
    use crate::common;
    use std::collections::BTreeMap;
    use std::process::Command;

    let pointer =
        regex::Regex::new(r"\(\((v[0-9]+) as \*const _\) as \*const ([ui](?:8|16|32|64))\)")
            .unwrap();
    let mut source = String::from("fn main() { unsafe {\n");
    for name in [
        "traits",
        "signed",
        "align",
        "debug_hex",
        "escaped",
        "reorder",
        "narrow_signed",
        "narrow_unsigned",
        "wide_signed",
        "wide_unsigned",
    ] {
        let code = output(true, name, true);
        let expression = code
            .lines()
            .find(|line| line.contains("println!("))
            .unwrap_or_else(|| panic!("{name}: {code}"));
        let pointers: BTreeMap<_, _> = pointer
            .captures_iter(expression)
            .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
            .collect();
        assert!(!pointers.is_empty(), "{expression}");
        source.push_str("{\n");
        for (var, ty) in pointers {
            let value = match name {
                "traits" => "123456789",
                "signed" => "-317",
                "align" => "-9",
                "debug_hex" => "0xabcdef",
                "escaped" => "7",
                "reorder" if ty == "u16" => "99",
                "reorder" => "7",
                "narrow_signed" => "-127",
                "narrow_unsigned" => "65535",
                "wide_signed" => "i64::MIN + 1",
                "wide_unsigned" => "u64::MAX",
                _ => unreachable!(),
            };
            source.push_str(&format!("let mut {var}_value: {ty} = {value}; let {var} = std::ptr::addr_of_mut!({var}_value);\n"));
        }
        source.push_str(expression);
        source.push_str("\n}\n");
    }
    source.push_str("} }\n");
    let path = common::scratch_file("rustfmtflags", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, &source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_formats"])
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
    let actual = Command::new(binary).output().unwrap();
    let expected = Command::new(
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_flags_real_x86_64"),
    )
    .output()
    .unwrap();
    assert!(actual.status.success() && expected.status.success());
    assert_eq!(actual.stdout, expected.stdout);
    assert_eq!(actual.stderr, expected.stderr);
}

#[test]
fn omitted_static_fields_retain_zero_values() {
    for (name, expected) in [
        ("zero_precision", "println!(\"{:.0}\""),
        ("zero_width", "println!(\"{:00}\""),
        ("debug_upper", "println!(\"{:#X?}\""),
    ] {
        let code = output(false, name, true);
        assert!(code.contains(expected), "{name}: {code}");
        assert!(!output(false, name, false).contains(expected));
    }
}
