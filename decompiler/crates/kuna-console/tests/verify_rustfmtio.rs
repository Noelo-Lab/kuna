//! Receiver layouts and owned error words at verified Stdout formatting calls.

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
    program.arch_mut().analysis_listing = true;
    program.arch_mut().rust_fmt_io = enabled;
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

const NATIVE: &str = "rust_fmt_io_x86_64";

#[test]
fn stdout_formats_preserve_the_receiver_and_owned_result_word() {
    for (name, macro_name, literal) in [
        ("write_unsigned", "write!", "[{}]"),
        ("write_signed", "writeln!", "{}"),
        ("write_flags", "writeln!", "{:#010x}/{:·>12}"),
        ("write_literal", "writeln!", "{{é🙂}}\\0\\n"),
        ("write_string", "write!", "|{:🦀^10.3}|"),
        ("write_pair", "write!", "{}: {}"),
    ] {
        let on = output(NATIVE, name, true);
        let off = output(NATIVE, name, false);
        assert!(
            on.contains(macro_name) && on.contains(&format!("\"{literal}\"")),
            "{name}: {on}"
        );
        assert!(on.contains("use std::io::Write as _;"), "{on}");
        assert!(on.contains("*mut &std::io::Stdout"), "{on}");
        assert!(on.contains("std::result::Result::Ok(()) => 0"), "{on}");
        assert!(
            on.contains("std::mem::transmute::<std::io::Error, *mut ()>"),
            "{on}"
        );
        assert!(!on.contains("Stdout::write_fmt("), "{on}");
        assert!(off.contains("Stdout::write_fmt("), "{off}");
        assert!(!off.contains(macro_name), "{off}");
        assert!(!on.contains("std::io::stdout()"), "{on}");
    }
}

#[test]
fn dynamic_formats_and_other_writers_remain_explicit() {
    for name in ["write_empty", "write_dynamic", "write_unknown", "write_vec"] {
        let on = output(NATIVE, name, true);
        assert_eq!(on, output(NATIVE, name, false), "{name}: {on}");
        assert!(!on.contains("use std::io::Write as _;"), "{on}");
    }
}

#[test]
fn producer_language_and_current_image_evidence_are_required() {
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
                |arch| match mutation {
                    0 => arch.rustc_stable_version = None,
                    1 => arch.rustc_stable_version = Some((1, 93, 0)),
                    2 => {
                        arch.source_lang_forced = Some(kuna_decomp::kuna_sourcelang::SourceLang::C)
                    }
                    3 => arch.rust_fmt_args = false,
                    4 => arch.rust_io_writer_profiles.clear(),
                    5 => {
                        for profile in arch.rust_io_writer_profiles.values_mut() {
                            profile.protected[0].bytes[0] ^= 1;
                        }
                    }
                    6 => {
                        for profile in arch.rust_io_writer_profiles.values_mut() {
                            profile.protected.clear();
                        }
                    }
                    _ => {
                        let spans: Vec<_> = arch
                            .rust_io_writer_profiles
                            .values()
                            .flat_map(|profile| profile.protected.iter())
                            .map(|span| (span.address, span.bytes.len()))
                            .collect();
                        assert!(!spans.is_empty());
                        let space = arch.manage().get_default_data_space().unwrap().clone();
                        for (address, size) in spans {
                            arch.symboltab.set_property_range(
                                kuna_decomp::varnode::varnode_flags::volatil,
                                &kuna_base::address::Address::new(space.clone(), address),
                                &kuna_base::address::Address::new(
                                    space.clone(),
                                    address + size as u64 - 1,
                                ),
                            );
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
fn direct_call_shape_nonnull_receiver_and_eight_byte_result_are_required() {
    for mutation in 0..6 {
        let run = |enabled| {
            decompile(
                NATIVE,
                "write_unsigned",
                enabled,
                "rust",
                |_| {},
                |fd| {
                    let index = (0..fd.num_calls())
                        .find(|&index| fd.get_call_specs(index).get_name().ends_with("write_fmt"))
                        .unwrap();
                    let id = fd.get_call_specs(index).get_op();
                    assert_eq!(fd.obank().get(id).unwrap().num_input(), 4);
                    match mutation {
                        0 => fd.op_remove_input(id, 3),
                        1 => {
                            let value = fd.new_constant(8, 0);
                            fd.op_insert_input(id, value, 4).unwrap();
                        }
                        2 => {
                            let value = fd.new_constant(8, 0);
                            fd.op_set_input(id, value, 1).unwrap();
                        }
                        3 => {
                            let value = fd.new_unique(1, None);
                            fd.op_set_output(id, value).unwrap();
                        }
                        4 => fd.op_unset_output(id),
                        _ => {
                            let address = fd.get_call_specs(index).get_entry_address().get_offset();
                            let target = fd.new_constant(8, address);
                            fd.op_set_input(id, target, 0).unwrap();
                            fd.op_set_opcode(
                                id,
                                kuna_decomp::typeop::type_op_for(
                                    kuna_num::opcodes::OpCode::CPUI_CALLIND,
                                ),
                            );
                        }
                    }
                },
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
    }
}

#[test]
fn stdout_recovery_obeys_underlying_format_options() {
    for (name, option) in [
        ("write_literal", 0),
        ("write_flags", 1),
        ("write_string", 2),
    ] {
        let run = |enabled| {
            decompile(
                NATIVE,
                name,
                enabled,
                "rust",
                |arch| match option {
                    0 => arch.rust_fmt_literal = false,
                    1 => arch.rust_fmt_flags = false,
                    _ => arch.rust_fmt_str = false,
                },
                |_| {},
            )
        };
        assert_eq!(run(true), run(false), "{name}");
    }
}

#[cfg(unix)]
#[test]
fn unchanged_expressions_match_native_stdout_and_broken_pipe_results() {
    use crate::common;
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;
    use std::process::{Command, Stdio};
    if Command::new("rustc")
        .arg("--version")
        .output()
        .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
    {
        return;
    }
    let primitive =
        regex::Regex::new(r"&\*\(\((\w+) as \*const _\) as \*const (u32|i64)\)").unwrap();
    let string =
        regex::Regex::new(r"from_raw_parts\((\w+) as \*const u8,(\w+) as usize\)").unwrap();
    let mut source = String::new();
    for (name, parameters) in [
        ("write_unsigned", "out:&mut &std::io::Stdout,value:u32"),
        ("write_signed", "out:&mut &std::io::Stdout,value:i64"),
        ("write_flags", "out:&mut &std::io::Stdout,value:u32"),
        ("write_literal", "out:&mut &std::io::Stdout"),
        ("write_string", "out:&mut &std::io::Stdout,text:&str"),
        (
            "write_pair",
            "out:&mut &std::io::Stdout,value:u32,text:&str",
        ),
    ] {
        let body = output(NATIVE, name, true);
        let expressions: Vec<_> = body
            .lines()
            .filter(|line| line.contains("use std::io::Write as _;"))
            .collect();
        assert_eq!(expressions.len(), 1, "{name}: {body}");
        let expression = expressions[0]
            .trim()
            .strip_prefix("return ")
            .unwrap()
            .split("; //")
            .next()
            .unwrap()
            .trim_end_matches(';');
        let mut bindings = String::from("let a0=out as *mut &std::io::Stdout as u64;");
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
        source.push_str(&format!("fn {name}({parameters})->std::io::Result<()> {{unsafe{{{bindings}let raw={expression};if raw==0{{Ok(())}}else{{Err(std::mem::transmute::<*mut (),std::io::Error>(raw as usize as *mut ()))}}}}}}\n"));
    }
    let original = std::fs::read_to_string(
        root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_fmt_io.rs"),
    )
    .unwrap();
    let main = &original[original.find("fn main() {").unwrap()..];
    let main = main[..main.find("    if std::hint::black_box(false)").unwrap()]
        .replace("write_empty(&mut out),", "")
        + "}\n";
    source.push_str("use std::io;\n");
    source.push_str(&main);
    let path = common::scratch_file("rustfmtio", "rs");
    let binary = path.with_extension("exe");
    std::fs::write(&path, source).unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "recovered_stdout"])
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
    let native_source = path.with_extension("native.rs");
    let native = path.with_extension("native.exe");
    std::fs::write(
        &native_source,
        original[..original.find("fn main() {").unwrap()].to_owned() + &main,
    )
    .unwrap();
    let build = Command::new("rustc")
        .args(["--edition=2021", "--crate-name", "original_stdout"])
        .arg(&native_source)
        .arg("-o")
        .arg(&native)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    for broken_pipe in [false, true] {
        let execute = |executable: &std::path::Path| {
            let mut command = Command::new(executable);
            if broken_pipe {
                let (reader, writer) = UnixStream::pair().unwrap();
                drop(reader);
                command.stdout(Stdio::from(OwnedFd::from(writer)));
            }
            command.output().unwrap()
        };
        let recovered = execute(&binary);
        let expected = execute(&native);
        assert!(
            recovered.status.success() && expected.status.success(),
            "broken pipe {broken_pipe}"
        );
        assert_eq!(
            recovered.stdout, expected.stdout,
            "broken pipe {broken_pipe}"
        );
        assert_eq!(
            recovered.stderr, expected.stderr,
            "broken pipe {broken_pipe}"
        );
        if broken_pipe {
            assert!(String::from_utf8_lossy(&expected.stderr).contains("true"));
        }
    }
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(binary).unwrap();
    std::fs::remove_file(native_source).unwrap();
    std::fs::remove_file(native).unwrap();
}

#[test]
fn promoted_receiver_uses_the_original_inner_pointer() {
    for name in ["write_value", "write_hex", "write_binary", "write_debug"] {
        let on = output("rust_fmt_io_promoted_x86_64", name, true);
        let off = output("rust_fmt_io_promoted_x86_64", name, false);
        assert!(on.contains("use std::io::Write as _;"), "{on}");
        assert!(
            on.contains("std::mem::transmute::<*mut (), std::io::Stdout>"),
            "{on}"
        );
        assert!(
            on.contains("std::mem::transmute::<std::io::Error, *mut ()>"),
            "{on}"
        );
        assert!(!on.contains("Stdout::write_fmt("), "{on}");
        assert!(!on.contains("std::io::stdout()"), "{on}");
        assert!(off.contains("Stdout::write_fmt("), "{off}");
    }
}
