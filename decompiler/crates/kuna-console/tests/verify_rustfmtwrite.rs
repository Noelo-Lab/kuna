//! String mutation and byte results at verified Rust formatting calls.

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_rustfmt::RustFmtMode;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn decompile(
    fixture: &str,
    name: &str,
    enabled: bool,
    language: &str,
    configure: impl FnOnce(&mut Architecture),
    edit: impl FnOnce(&mut Funcdata),
) -> String {
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
    program.arch_mut().rust_fmt = if enabled {
        RustFmtMode::All
    } else {
        RustFmtMode::On
    };
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

fn output(fixture: &str, name: &str, enabled: bool) -> String {
    decompile(fixture, name, enabled, "rust", |_| {}, |_| {})
}

const NATIVE: &str = "rust_fmt_write_real_x86_64";

#[test]
fn known_string_writes_preserve_arguments_and_result_bytes() {
    for (name, macro_name, literal) in [
        ("write_unsigned", "write!", "[{}]"),
        ("write_signed", "writeln!", "{}"),
        ("write_flags", "writeln!", "{:#010x}/{:·>12}"),
        ("write_literal", "writeln!", "{{é🙂}}\\0\\n"),
        ("write_empty", "write!", ""),
        ("write_string", "write!", "|{:🦀^10.3}|"),
        ("write_pair", "write!", "{}: {}"),
    ] {
        let on = output(NATIVE, name, true);
        let off = output(NATIVE, name, false);
        assert!(
            on.contains(macro_name) && on.contains(&format!("\"{literal}\"")),
            "{name}: {on}"
        );
        assert!(
            on.contains("use std::fmt::Write as _;") && on.contains(".is_err()"),
            "{on}"
        );
        assert!(on.contains("*mut std::string::String"), "{on}");
        assert!(!on.contains("core::fmt::write("), "{on}");
        assert!(off.contains("core::fmt::write("), "{off}");
        assert!(!off.contains(macro_name), "{off}");
    }
}

#[test]
fn custom_writers_and_unsupported_arguments_remain_explicit() {
    for name in [
        "write_custom",
        "write_dynamic",
        "write_unknown",
        "write_reject",
    ] {
        let on = output(NATIVE, name, true);
        assert_eq!(on, output(NATIVE, name, false), "{name}: {on}");
        assert!(!on.contains("use std::fmt::Write as _;"), "{on}");
    }
}

#[test]
fn layout_language_and_complete_method_identity_bound_recovery() {
    for language in ["c", "go"] {
        let run = |enabled| decompile(NATIVE, "write_unsigned", enabled, language, |_| {}, |_| {});
        assert_eq!(run(true), run(false), "{language}");
    }
    for mutation in 0..8 {
        let run = |enabled| {
            decompile(
                NATIVE,
                "write_unsigned",
                enabled,
                "rust",
                |arch| {
                    use kuna_decomp::kuna_rustfmtwrite::WriterMethod;
                    match mutation {
                        0 => arch.rustc_stable_version = None,
                        1 => arch.rustc_stable_version = Some((1, 93, 2)),
                        2 => {
                            arch.source_lang_forced =
                                Some(kuna_decomp::kuna_sourcelang::SourceLang::C)
                        }
                        3 => arch.rust_string_writer_methods.clear(),
                        index => {
                            let missing = [
                                WriterMethod::DropString,
                                WriterMethod::WriteStr,
                                WriterMethod::WriteChar,
                                WriterMethod::WriteFmt,
                            ][index - 4];
                            arch.rust_string_writer_methods
                                .retain(|_, method| *method != missing);
                        }
                    }
                },
                |_| {},
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
    }
}

#[test]
fn ambiguous_object_symbols_never_establish_a_string_writer() {
    use kuna_decomp::kuna_rustfmtwrite::{writer_symbols, WriterMethod};
    let names = [
        (
            "core::ptr::drop_in_place<alloc::string::String>",
            WriterMethod::DropString,
        ),
        (
            "<alloc::string::String as core::fmt::Write>::write_str",
            WriterMethod::WriteStr,
        ),
        (
            "<alloc::string::String as core::fmt::Write>::write_char",
            WriterMethod::WriteChar,
        ),
        ("core::fmt::Write::write_fmt", WriterMethod::WriteFmt),
    ];
    for (name, method) in names {
        let known = (0x1000, Some(name.to_owned()));
        assert_eq!(writer_symbols(&[known.clone()]).get(&0x1000), Some(&method));
        for alias in [
            None,
            Some("user::write_fmt".into()),
            Some("core::ptr::drop_in_place<Other>".into()),
        ] {
            assert!(writer_symbols(&[known.clone(), (0x1000, alias.clone())]).is_empty());
            assert!(writer_symbols(&[(0x1000, alias), known.clone()]).is_empty());
        }
    }
}

#[test]
fn writer_address_call_shape_and_result_width_are_required() {
    for mutation in 0..8 {
        let run = |enabled| {
            decompile(
                NATIVE,
                "write_unsigned",
                enabled,
                "rust",
                |_| {},
                |fd| {
                    let index = (0..fd.num_calls())
                        .find(|&index| {
                            matches!(
                                fd.get_call_specs(index).get_name(),
                                "write" | "core::fmt::write"
                            )
                        })
                        .unwrap();
                    let id = fd.get_call_specs(index).get_op();
                    assert_eq!(fd.obank().get(id).unwrap().num_input(), 5);
                    assert_eq!(
                        fd.vbank()
                            .get(fd.obank().get(id).unwrap().get_out().unwrap())
                            .unwrap()
                            .get_size(),
                        1
                    );
                    match mutation {
                        0 => fd.op_remove_input(id, 4),
                        1 => {
                            let value = fd.new_constant(8, 0);
                            fd.op_insert_input(id, value, 5).unwrap();
                        }
                        2 => {
                            let value = fd.new_constant(8, 0);
                            fd.op_set_input(id, value, 1).unwrap();
                        }
                        3..=5 => {
                            let value = fd.new_constant(8, [0, 1, u64::MAX - 7][mutation - 3]);
                            fd.op_set_input(id, value, 2).unwrap();
                        }
                        6 => {
                            let value = fd.new_unique(8, None);
                            fd.op_set_output(id, value).unwrap();
                        }
                        _ => fd.op_unset_output(id),
                    }
                },
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
        assert!(!run(true).contains("use std::fmt::Write as _;"));
    }
}

#[test]
fn volatile_writer_tables_keep_runtime_writes() {
    let volatile = |enabled| {
        decompile(
            NATIVE,
            "write_unsigned",
            enabled,
            "rust",
            |arch| {
                let (lo, hi) = arch.rust_fmt_write_relro.unwrap();
                let space = arch.manage().get_default_data_space().unwrap().clone();
                arch.symboltab.set_property_range(
                    kuna_decomp::varnode::varnode_flags::volatil,
                    &kuna_base::address::Address::new(space.clone(), lo),
                    &kuna_base::address::Address::new(space, hi - 1),
                );
            },
            |_| {},
        )
    };
    assert_eq!(volatile(true), volatile(false));
}

#[test]
fn unchanged_write_expressions_append_native_bytes_and_preserve_success() {
    use crate::common;
    use std::process::Command;
    if Command::new("rustc")
        .arg("--version")
        .output()
        .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound)
    {
        return;
    }
    let primitive =
        regex::Regex::new(r"&\*\(\((\w+) as \*const _\) as \*const (u32|i64)\)").unwrap();
    let string =
        regex::Regex::new(r"from_raw_parts\((\w+) as \*const u8,(\w+) as usize\)").unwrap();
    let mut source = String::new();
    for (name, parameters) in [
        ("write_unsigned", "out:&mut String,value:u32"),
        ("write_signed", "out:&mut String,value:i64"),
        ("write_flags", "out:&mut String,value:u32"),
        ("write_literal", "out:&mut String"),
        ("write_empty", "out:&mut String"),
        ("write_string", "out:&mut String,text:&str"),
        ("write_pair", "out:&mut String,value:u32,text:&str"),
    ] {
        let body = output(NATIVE, name, true);
        let expressions: Vec<_> = body
            .lines()
            .filter(|line| line.contains("use std::fmt::Write as _;"))
            .collect();
        assert_eq!(expressions.len(), 1, "{name}: {body}");
        let expression = expressions[0]
            .trim()
            .strip_prefix("if ")
            .unwrap()
            .strip_suffix(" == b'\\0' {")
            .unwrap();
        let mut bindings = String::from("let a0=out as *mut String as u64;");
        for argument in primitive.captures_iter(expression) {
            bindings.push_str(&format!(
                "let {}=&value as *const {};",
                &argument[1], &argument[2]
            ));
        }
        for argument in string.captures_iter(expression) {
            bindings.push_str(&format!(
                "let {}=text.as_ptr() as u64;let {}=text.len() as u64;",
                &argument[1], &argument[2]
            ));
        }
        source.push_str(&format!("fn {name}({parameters})->std::fmt::Result {{unsafe{{{bindings}assert_eq!({expression},0u8);}}Ok(())}}\n"));
    }
    let original = std::fs::read_to_string(
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_write.rs"),
    )
    .unwrap();
    let main = &original[original.find("fn main() {").unwrap()..];
    source.push_str(&main[..main.find("    if std::hint::black_box(false)").unwrap()]);
    source.push_str("}\n");
    let path = common::scratch_file("rustfmtwrite", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_writes"])
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
    let recovered = Command::new(&binary).output().unwrap();
    let native = Command::new(
        root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(NATIVE),
    )
    .output()
    .unwrap();
    assert!(recovered.status.success() && native.status.success());
    assert_eq!(recovered.stdout, native.stdout);
    assert_eq!(recovered.stderr, native.stderr);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(binary).unwrap();
}
