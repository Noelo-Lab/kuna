//! Compare emitted Rust integer extensions against native compiler execution.

use crate::common;
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use std::path::{Path, PathBuf};
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

fn body(binary: &Path, name: &str) -> String {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let mut status = ConsoleCommands::into_status(vec![
        "option outlang rust".into(),
        format!("lo fu {name}"),
        "decompile".into(),
        "print C".into(),
    ]);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
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
    let start = status.optr.len();
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    status.optr[start..].trim().to_string()
}

#[test]
fn signed_text_storage_widens_before_arithmetic() {
    let source = root().join("tests/fixtures/rust-extensions/extensions.rs");
    let native = common::scratch_file("rustextensions-native", "elf");
    run(Command::new("rustc")
        .args(["--crate-name", "extensions", "-C", "opt-level=3", "-C", "debuginfo=2", "-C", "overflow-checks=off", "-o"])
        .arg(&native)
        .arg(&source));
    let expected = run(&mut Command::new(&native));
    assert_eq!(String::from_utf8_lossy(&expected.stdout).lines().count(), 1578);
    let mut emitted = String::from("use std::hint::black_box;\n");
    for name in ["sign8", "sign16", "sign32", "narrow_sum", "narrow_product", "zero8"] {
        emitted.push_str(&body(&native, name));
        emitted.push('\n');
    }
    let original = std::fs::read_to_string(source).unwrap();
    emitted.push_str(&original[original.find("fn main()").unwrap()..]);
    let code = common::scratch_file("rustextensions-emitted", "rs");
    let binary = common::scratch_file("rustextensions-execute", "elf");
    std::fs::write(&code, &emitted).unwrap();
    run(Command::new("rustc")
        .args(["--crate-name", "extensions_emitted", "-C", "opt-level=3", "-C", "overflow-checks=off", "-o"])
        .arg(&binary)
        .arg(&code));
    let actual = run(&mut Command::new(binary));
    assert_eq!(actual.stdout, expected.stdout, "{emitted}");
    assert_eq!(actual.stderr, expected.stderr, "{emitted}");
}
