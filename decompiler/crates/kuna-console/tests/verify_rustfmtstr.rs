//! Proven string Display arguments and their runtime formatting behavior.

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
        RustFmtMode::On
    } else {
        RustFmtMode::Off
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

const NATIVE: &str = "rust_fmt_str_real_x86_64";

#[test]
fn proven_string_views_recover_print_result_and_panic_macros() {
    for (name, expected) in [
        ("str_plain", "print!(\"{}\""),
        ("str_line", "println!(\"{{é🙂}} {}\\n\""),
        ("str_stderr", "eprintln!(\"stderr {}\""),
        ("str_stderr_pair", "eprintln!(\"{}: {}\""),
        ("str_flags", "println!(\"|{0:·<8}|{0:🦀^10.3}|{0:>7.3}|\""),
        ("str_pair", "println!(\"{} / {}\""),
        ("str_integer", "format!(\"{}-{}\""),
        ("str_result", "format!(\"[{}]\""),
        ("str_panic", "panic!(\"bad {}\""),
    ] {
        let on = output(NATIVE, name, true);
        let off = output(NATIVE, name, false);
        assert!(on.contains(expected), "{name}: {on}");
        assert!(
            on.contains("std::str::from_utf8_unchecked(std::slice::from_raw_parts("),
            "{on}"
        );
        assert!(!off.contains(expected), "{name}: {off}");
    }
}

#[test]
fn unknown_wrappers_and_dynamic_formats_remain_explicit() {
    for name in ["str_custom", "str_debug", "str_reference", "str_owned"] {
        let on = output(NATIVE, name, true);
        assert_eq!(on, output(NATIVE, name, false), "{name}: {on}");
        assert!(!on.contains("from_utf8_unchecked"), "{name}: {on}");
    }
    let dynamic = output(NATIVE, "str_dynamic", true);
    assert!(
        dynamic.contains("_print(") && !dynamic.contains("from_utf8_unchecked"),
        "{dynamic}"
    );
}

#[test]
fn language_layout_and_formatter_facts_bound_string_recovery() {
    for language in ["c", "go"] {
        let run = |enabled| decompile(NATIVE, "str_plain", enabled, language, |_| {}, |_| {});
        assert_eq!(run(true), run(false), "{language}");
    }
    for mutation in 0..5 {
        let run = |enabled| {
            decompile(
                NATIVE,
                "str_plain",
                enabled,
                "rust",
                |arch| match mutation {
                    0 => arch.rust_str_display_formatters.clear(),
                    1 => arch.rustc_stable_version = None,
                    2 => arch.rustc_stable_version = Some((1, 90, 0)),
                    3 => arch.rustc_stable_version = Some((1, 99, 1)),
                    _ => {
                        arch.source_lang_forced = Some(kuna_decomp::kuna_sourcelang::SourceLang::C)
                    }
                },
                |_| {},
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
    }
}

#[test]
fn complete_object_identity_and_instruction_body_are_required() {
    use object::{Object, ObjectSection, ObjectSymbol, SymbolKind};
    let bytes = std::fs::read(
        root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(NATIVE),
    )
    .unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let metadata = kuna_analysis::sourcelang::formatter_metadata(&bytes);
    let &address = metadata.strings.first().unwrap();
    let section = file
        .sections()
        .find(|s| s.address() <= address && address < s.address() + s.size())
        .unwrap();
    let offset = (section.file_range().unwrap().0 + address - section.address()) as usize;
    for byte in 0..18 {
        let mut other = bytes.clone();
        other[offset + byte] ^= 0x10;
        assert!(
            !kuna_analysis::sourcelang::formatter_metadata(&other)
                .strings
                .contains(&address),
            "changed instruction or target byte {byte}"
        );
    }
    let target = kuna_decomp::kuna_rustfmtstr::forwarded_target(address, &bytes[offset..]).unwrap();
    let target_name = file
        .symbols()
        .find(|s| s.address() == target)
        .unwrap()
        .name()
        .unwrap()
        .as_bytes()
        .to_vec();
    let target_offset = bytes
        .windows(target_name.len())
        .position(|w| w == target_name)
        .unwrap();
    let mut other = bytes.clone();
    assert_eq!(&other[target_offset..target_offset + 8], b"_ZN4core");
    other[target_offset + 4..target_offset + 8].copy_from_slice(b"fake");
    assert!(!kuna_analysis::sourcelang::formatter_metadata(&other)
        .strings
        .contains(&address));
    let alias = file
        .symbols()
        .find(|s| {
            s.kind() == SymbolKind::Text
                && s.is_definition()
                && s.address() != address
                && s.address() != target
                && s.name().is_ok_and(|n| !n.contains("fmt"))
        })
        .unwrap();
    let symtab = file
        .section_by_name(".symtab")
        .unwrap()
        .file_range()
        .unwrap()
        .0 as usize;
    let mut other = bytes.clone();
    let value = symtab + alias.index().0 * 24 + 8;
    other[value..value + 8].copy_from_slice(&address.to_le_bytes());
    assert!(!kuna_analysis::sourcelang::formatter_metadata(&other)
        .strings
        .contains(&address));
    for mutation in 0..3 {
        let mut other = bytes.clone();
        match mutation {
            0 => other[18..20].copy_from_slice(&183u16.to_le_bytes()),
            1 => other[16..18].copy_from_slice(&1u16.to_le_bytes()),
            _ => other[5] = 2,
        }
        assert!(kuna_analysis::sourcelang::formatter_metadata(&other)
            .strings
            .is_empty());
    }
}

#[test]
fn every_live_string_and_argument_word_is_required() {
    use kuna_num::opcodes::OpCode;
    for index in 0..4 {
        let run = |enabled| {
            decompile(
                NATIVE,
                "str_plain",
                enabled,
                "rust",
                |_| {},
                |fd| {
                    let stack = fd
                        .get_arch()
                        .manage()
                        .get_stack_space()
                        .unwrap()
                        .get_index();
                    let cells: Vec<_> = fd
                        .obank()
                        .iter_alive()
                        .filter(|&id| {
                            let op = fd.obank().get(id).unwrap();
                            op.code() == OpCode::CPUI_INDIRECT
                                && !op.is_indirect_creation()
                                && op
                                    .get_out()
                                    .and_then(|v| fd.vbank().get(v))
                                    .is_some_and(|v| v.get_space().get_index() == stack)
                        })
                        .collect();
                    assert_eq!(cells.len(), 4);
                    fd.obank_mut()
                        .get_mut(cells[index])
                        .unwrap()
                        .set_flag(kuna_decomp::op::pcodeop_flags::indirect_creation);
                },
            )
        };
        assert_eq!(run(true), run(false), "missing word {index}");
    }
}

#[test]
fn unchanged_string_macros_reproduce_native_streams_and_panic_payloads() {
    use crate::common;
    use std::process::Command;
    if Command::new("rustc")
        .arg("--version")
        .output()
        .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound)
    {
        return;
    }
    let string = "let v1=text.as_ptr() as u64;let v3=text.len() as u64;";
    let mixed = "let v3=&value as *const u32;let v2=text.as_ptr() as u64;let v4=text.len() as u64;";
    let mut source = String::new();
    for (name, parameters, bindings, result) in [
        ("str_plain", "text:&str", string, false),
        ("str_line", "text:&str", string, false),
        ("str_stderr", "text:&str", string, false),
        ("str_stderr_pair", "value:u32,text:&str", mixed, false),
        ("str_flags", "text:&str", string, false),
        ("str_pair", "left:&str,right:&str",
            "let v1=left.as_ptr() as u64;let v4=left.len() as u64;let v2=right.as_ptr() as u64;let v5=right.len() as u64;", false),
        ("str_integer", "value:u32,text:&str", mixed, true),
        ("str_result", "text:&str", string, true),
        ("str_panic", "text:&str", string, false),
    ] {
        let body = output(NATIVE, name, true);
        let lines: Vec<_> = body.lines().filter(|line| line.contains("from_utf8_unchecked")).collect();
        assert_eq!(lines.len(), 1, "{name}: {body}");
        let returns = if result { " -> String" } else { "" };
        let before = if result {
            "let mut destination=std::mem::MaybeUninit::<String>::uninit();let a0=destination.as_mut_ptr() as u64;"
        } else { "" };
        let after = if result { "destination.assume_init()" } else { "" };
        source.push_str(&format!("fn {name}({parameters}){returns} {{ unsafe {{ {bindings}{before}\n{}\n{after}\n}} }}\n", lines[0]));
    }
    let original = std::fs::read_to_string(
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_str.rs"),
    )
    .unwrap();
    let main = &original[original.find("fn main() {").unwrap()..];
    source.push_str(&main[..main.find("    if std::hint::black_box(false)").unwrap()]);
    source.push_str("}\n");
    let path = common::scratch_file("rustfmtstr", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_strings"])
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
    for arguments in [Vec::<&str>::new(), vec!["panic"]] {
        let recovered = Command::new(&binary).args(&arguments).output().unwrap();
        let native = Command::new(
            root()
                .join("decompiler/crates/kuna-analysis/tests/fixtures")
                .join(NATIVE),
        )
        .args(&arguments)
        .output()
        .unwrap();
        assert!(recovered.status.success() && native.status.success());
        assert_eq!(recovered.stdout, native.stdout, "{arguments:?} stdout");
        assert_eq!(recovered.stderr, native.stderr, "{arguments:?} stderr");
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(binary).unwrap();
}
