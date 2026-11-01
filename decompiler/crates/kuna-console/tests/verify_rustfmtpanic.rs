//! Verified panic messages, primitive arguments, and conservative call boundaries.

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::funcdata::Funcdata;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn decompile(
    real: bool,
    name: &str,
    enabled: bool,
    language: &str,
    configure: impl FnOnce(&mut Architecture),
    edit: impl FnOnce(&mut Funcdata),
) -> String {
    let fixture = if real {
        "rust_fmt_panic_real_x86_64"
    } else {
        "rust_fmt_panic_x86_64"
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
    program.arch_mut().rust_fmt_panic = enabled;
    configure(program.arch_mut());
    program.commit_pending_analysis().unwrap();
    let commands = vec![
        format!("option setlanguage {language}-language"),
        format!("load function {name}"),
        "decompile".into(),
        "print C".into(),
    ];
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..3 {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    edit(
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .fd
            .as_mut()
            .unwrap(),
    );
    status.optr.clear();
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    status.optr
}

fn output(real: bool, name: &str, enabled: bool) -> String {
    decompile(real, name, enabled, "rust", |_| {}, |_| {})
}

#[test]
fn native_panic_macros_preserve_newlines_and_integer_formats() {
    for (name, expected) in [
        ("panic_literal", "panic!(\"literal {{é🙂}}\\n\\n\")"),
        ("panic_empty", "panic!(\"\")"),
        ("panic_decimal", "panic!(\"value={}\\n\""),
        ("panic_signed", "panic!(\"signed={:+}\""),
        ("panic_hex", "panic!(\"hex={:#010x}\""),
        (
            "panic_reorder",
            "panic!(\"{0:#08x} {1:+06} {2:>5o} {1:+06}\"",
        ),
        ("panic_narrow", "panic!(\"{:+} {:#x} {:08b}\""),
        ("panic_escaped", "panic!(\"{{|"),
    ] {
        let on = output(true, name, true);
        let off = output(true, name, false);
        assert!(on.contains(expected), "{name}: {on}");
        assert!(!on.contains("panic_fmt("), "{name}: {on}");
        assert!(off.contains("panic_fmt("), "{name}: {off}");
        assert!(on.contains("// no-return"), "{name}: {on}");
    }
    for name in ["literal_panic", "scoped_panic", "static_mutable_location"] {
        assert!(output(false, name, true).contains("panic!(\"{{x}}\\n\")"));
    }
}

#[test]
fn unsupported_templates_locations_and_names_remain_runtime_calls() {
    for (real, names) in [
        (
            true,
            vec!["panic_generic", "panic_float", "panic_forwarded"],
        ),
        (
            false,
            vec![
                "reject_root",
                "reject_missing",
                "reject_extra",
                "reject_global",
                "reject_null",
                "reject_dynamic_location",
                "reject_unmapped_location",
                "reject_mutable_text",
                "reject_invalid_utf8",
                "reject_even_tag",
                "reject_excess_length",
            ],
        ),
    ] {
        for name in names {
            let on = output(real, name, true);
            assert_eq!(on, output(real, name, false), "{name}");
            assert!(on.contains("panic_fmt("), "{name}: {on}");
        }
    }
    let on = output(true, "panic_dynamic", true);
    let off = output(true, "panic_dynamic", false);
    assert!(on.contains("panic!(\"Formatting argument out of range\")"));
    assert_eq!(on.matches("panic_fmt(").count(), 1);
    assert_eq!(off.matches("panic_fmt(").count(), 2);
    assert_eq!(
        on.lines().filter(|line| line.contains("panic_fmt(")).last(),
        off.lines()
            .filter(|line| line.contains("panic_fmt("))
            .last()
    );
}

#[test]
fn language_version_and_formatting_options_bound_recovery() {
    for source in [
        kuna_decomp::kuna_sourcelang::SourceLang::C,
        kuna_decomp::kuna_sourcelang::SourceLang::Go,
    ] {
        let run = |enabled| {
            decompile(
                true,
                "panic_literal",
                enabled,
                "rust",
                |arch| arch.source_lang_forced = Some(source),
                |_| {},
            )
        };
        assert_eq!(run(true), run(false));
    }
    for version in [None, Some((1, 92, 0)), Some((1, 93, 2)), Some((1, 99, 1))] {
        let run = |enabled| {
            decompile(
                true,
                "panic_literal",
                enabled,
                "rust",
                |arch| arch.rustc_stable_version = version,
                |_| {},
            )
        };
        assert_eq!(run(true), run(false));
    }
    for language in ["c", "go"] {
        assert_eq!(
            decompile(true, "panic_hex", true, language, |_| {}, |_| {}),
            decompile(true, "panic_hex", false, language, |_| {}, |_| {})
        );
    }
    for name in ["panic_literal", "panic_decimal"] {
        assert!(decompile(
            true,
            name,
            true,
            "rust",
            |arch| arch.rustc_stable_version = Some((1, 93, 0)),
            |_| {}
        )
        .contains("panic!("));
    }
    for name in ["panic_literal", "panic_empty"] {
        assert!(decompile(
            true,
            name,
            true,
            "rust",
            |arch| {
                arch.rust_fmt_literal = false;
                arch.rust_fmt_args = false;
                arch.rust_fmt_flags = false;
            },
            |_| {}
        )
        .contains("panic!("));
    }
    for name in ["panic_decimal", "panic_hex"] {
        let run = |enabled| {
            decompile(
                true,
                name,
                enabled,
                "rust",
                |arch| arch.rust_fmt_args = false,
                |_| {},
            )
        };
        assert_eq!(run(true), run(false));
    }
    let no_flags = |enabled| {
        decompile(
            true,
            "panic_hex",
            enabled,
            "rust",
            |arch| arch.rust_fmt_flags = false,
            |_| {},
        )
    };
    assert_eq!(no_flags(true), no_flags(false));
    assert!(decompile(
        true,
        "panic_decimal",
        true,
        "rust",
        |arch| arch.rust_fmt_flags = false,
        |_| {}
    )
    .contains("panic!("));
    let no_identity = |enabled| {
        decompile(
            true,
            "panic_hex",
            enabled,
            "rust",
            |arch| {
                arch.rust_display_formatters.clear();
                arch.rust_integer_formatters.clear();
            },
            |_| {},
        )
    };
    assert_eq!(no_identity(true), no_identity(false));
    let volatile = |enabled| {
        decompile(
            false,
            "literal_panic",
            enabled,
            "rust",
            |arch| {
                let space = arch.manage().get_default_data_space().unwrap().clone();
                arch.symboltab.set_property_range(
                    kuna_decomp::varnode::varnode_flags::volatil,
                    &kuna_base::address::Address::new(space.clone(), 0x2018),
                    &kuna_base::address::Address::new(space, 0x2030),
                );
            },
            |_| {},
        )
    };
    assert_eq!(volatile(true), volatile(false));
    assert!(!volatile(true).contains("panic!("));
}

#[test]
fn panic_recovery_requires_no_return_and_exact_argument_count() {
    use kuna_decomp::op::pcodeop_flags;
    for mutation in 0..3 {
        let run = |enabled| {
            decompile(
                true,
                "panic_literal",
                enabled,
                "rust",
                |_| {},
                |fd| {
                    assert_eq!(fd.num_calls(), 1);
                    let id = fd.get_call_specs(0).get_op();
                    match mutation {
                        0 => {
                            fd.get_call_specs_mut(0).proto_mut().set_no_return(false);
                            let block = fd.obank().get(id).unwrap().get_parent().unwrap();
                            let halt = fd.bb_op_tail(block).unwrap();
                            fd.obank_mut()
                                .get_mut(halt)
                                .unwrap()
                                .clear_flag(pcodeop_flags::noreturn);
                        }
                        1 => fd.op_remove_input(id, 3),
                        2 => {
                            let value = fd.new_constant(8, 7);
                            fd.op_insert_input(id, value, 4).unwrap();
                        }
                        _ => unreachable!(),
                    }
                },
            )
        };
        assert_eq!(run(true), run(false));
        assert!(!run(true).contains("panic!("));
    }
}

#[test]
fn recovered_panics_match_native_payloads() {
    use crate::common;
    use std::collections::BTreeMap;
    use std::process::Command;
    let mut source = String::from("fn capture(f: impl FnOnce() + std::panic::UnwindSafe) { let p = std::panic::catch_unwind(f).unwrap_err(); let s = p.downcast_ref::<&str>().copied().unwrap_or_else(|| p.downcast_ref::<String>().unwrap().as_str()); println!(\"{s:?}\"); }\n");
    for name in ["panic_literal", "panic_empty"] {
        source.push_str(&output(true, name, true));
    }
    source.push_str("fn main() { std::panic::set_hook(Box::new(|_| {})); capture(|| unsafe { panic_literal() }); capture(|| unsafe { panic_empty() });\n");
    let pointer =
        regex::Regex::new(r"\(\((v[0-9]+) as \*const _\) as \*const ([ui](?:8|16|32|64))\)")
            .unwrap();
    for name in [
        "panic_decimal",
        "panic_signed",
        "panic_hex",
        "panic_reorder",
        "panic_narrow",
        "panic_escaped",
    ] {
        let code = output(true, name, true);
        let start = code.find("panic!(").unwrap();
        let expression = &code[start..start + code[start..].find(");").unwrap() + 2];
        source.push_str("capture(|| unsafe {\n");
        let pointers: BTreeMap<_, _> = pointer
            .captures_iter(expression)
            .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
            .collect();
        assert!(!pointers.is_empty(), "{expression}");
        for (var, ty) in pointers {
            let value = match (name, ty.as_str()) {
                ("panic_decimal", _) => "u64::MAX",
                ("panic_signed", _) => "i64::MIN + 1",
                ("panic_hex", _) => "42",
                ("panic_reorder", "u16") => "99",
                ("panic_narrow", _) => "-127",
                _ => "7",
            };
            source.push_str(&format!("let mut {var}_value: {ty} = {value}; let {var} = std::ptr::addr_of_mut!({var}_value);\n"));
        }
        source.push_str(expression);
        source.push_str("\n});\n");
    }
    source.push_str("}\n");
    let path = common::scratch_file("rustfmtpanic", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, &source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_panics"])
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
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_panic_real_x86_64"),
    )
    .output()
    .unwrap();
    assert!(actual.status.success() && expected.status.success());
    assert_eq!(actual.stdout, expected.stdout);
    assert_eq!(actual.stderr, expected.stderr);
}
