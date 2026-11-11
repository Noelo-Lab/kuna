//! Compare emitted Go arithmetic with its native compiler across narrow boundaries.

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
    program
        .arch_mut()
        .set_kuna_option("godwarfsig", "on")
        .unwrap();
    program.commit_pending_analysis().unwrap();
    let mut status = ConsoleCommands::into_status(vec![
        "option outlang go".into(),
        format!("lo fu main.{name}"),
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
fn widening_precedes_arithmetic_and_keeps_narrow_overflow() {
    match Command::new("go").arg("version").output() {
        Ok(output) => assert!(output.status.success()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Go toolchain absent; external execution check skipped");
            return;
        }
        Err(error) => panic!("go: {error}"),
    }
    let source = root().join("tests/fixtures/go-extensions/extensions.go");
    let native = common::scratch_file("goextensions-native", "elf");
    run(Command::new("go")
        .args(["build", "-trimpath", "-buildvcs=false", "-o"])
        .arg(&native)
        .arg(&source));
    let expected = run(&mut Command::new(&native));
    assert_eq!(
        String::from_utf8_lossy(&expected.stderr).lines().count(),
        1578
    );
    let mut emitted = String::from("package main\n");
    for name in [
        "shift8",
        "sign8",
        "sign16",
        "sign32",
        "zero8",
        "signToUnsigned",
        "narrowSum",
        "narrowProduct",
        "lessDelta",
        "lessDelta32",
    ] {
        emitted.push_str(&body(&native, name));
        emitted.push('\n');
    }
    let original = std::fs::read_to_string(source).unwrap();
    emitted.push_str(&original[original.find("func main()").unwrap()..]);
    let code = common::scratch_file("goextensions-emitted", "go");
    let binary = common::scratch_file("goextensions-execute", "elf");
    std::fs::write(&code, &emitted).unwrap();
    run(Command::new("go")
        .args(["build", "-o"])
        .arg(&binary)
        .arg(&code));
    let actual = run(&mut Command::new(binary));
    assert_eq!(actual.stdout, expected.stdout, "{emitted}");
    assert_eq!(actual.stderr, expected.stderr, "{emitted}");
}
