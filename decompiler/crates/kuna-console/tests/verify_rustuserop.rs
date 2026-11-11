//! Execute unedited machine-operation calls against argument-recording Rust stubs.

use crate::common;
use kuna_console::engine::bootstrap_from_file;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::PathBuf;
use std::process::{Command, Output};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn run(command: &mut Command) -> Output {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn body(arch: &str, prototype: &str, name: &str) -> String {
    let program = bootstrap_from_file(
        root().join(format!("tests/fixtures/rust-userop/{arch}.xml")).to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    ).unwrap();
    let mut status = ConsoleCommands::into_status(vec![
        "option outlang rust".into(),
        format!("parse line extern {prototype};"),
        format!("lo fu {name}"),
        "decompile".into(),
        "print C".into(),
    ]);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut()
        .downcast_mut::<IfaceDecompData>().unwrap().conf = Some(program);
    for _ in 0..4 {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    let start = status.optr.len();
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    status.optr[start..].trim().to_string()
}

#[test]
fn keyword_calls_keep_identity_arguments_and_single_evaluation() {
    let mut emitted = std::fs::read_to_string(root().join("tests/fixtures/rust-userop/execute.rs")).unwrap();
    for (arch, prototype, name) in [
        ("x86", "unsigned int read_port(void)", "read_port"),
        ("x86", "unsigned int read_variable(unsigned int port)", "read_variable"),
        ("x86", "void write_port(unsigned int port, unsigned char data)", "write_port"),
        ("avr", "void avr_break(void)", "avr_break"),
        ("mips", "void cooperate(unsigned int value)", "cooperate"),
    ] {
        emitted.push_str(&body(arch, prototype, name));
        emitted.push('\n');
    }
    let source = common::scratch_file("rustuserop-emitted", "rs");
    std::fs::write(&source, &emitted).unwrap();
    for edition in ["2015", "2024"] {
        let binary = common::scratch_file("rustuserop-execute", "elf");
        run(Command::new("rustc")
            .args(["--crate-name", "userop_emitted", "--edition", edition, "-C", "opt-level=2", "-o"])
            .arg(&binary).arg(&source));
        let output = run(&mut Command::new(binary));
        assert_eq!(output.stdout, b"42 machine operation calls checked\n", "{emitted}");
    }
}
