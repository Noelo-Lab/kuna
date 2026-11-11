//! Compare emitted Rust signed byte operations against native compiler execution.

use crate::common;
use kuna_console::engine::{bootstrap_from_file, bootstrap_from_object};
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
fn signed_byte_operators_match_native_values_and_effects() {
    let source = root().join("tests/fixtures/rust-signed-bytes/signed.rs");
    let native = common::scratch_file("rustsigned-native", "elf");
    run(Command::new("rustc")
        .args(["--crate-name", "signed_bytes", "-C", "opt-level=3", "-C", "debuginfo=2", "-C", "overflow-checks=off", "-o"])
        .arg(&native)
        .arg(&source));
    let expected = run(&mut Command::new(&native));
    assert_eq!(String::from_utf8_lossy(&expected.stdout).lines().count(), 65536);
    let original = std::fs::read_to_string(&source).unwrap();
    let start = original.find("#[no_mangle]\n#[inline(never)]\npub extern \"C\" fn byte_less").unwrap();
    let mut emitted = original[..start].to_string();
    for name in ["byte_less", "byte_lesseq", "byte_shift", "byte_unsigned", "byte_logical", "byte_store", "byte_call", "byte_loop"] {
        emitted.push_str(&body(&native, name));
        emitted.push('\n');
    }
    emitted.push_str(&original[original.find("fn main()").unwrap()..]);
    let code = common::scratch_file("rustsigned-emitted", "rs");
    let binary = common::scratch_file("rustsigned-execute", "elf");
    std::fs::write(&code, &emitted).unwrap();
    run(Command::new("rustc")
        .args(["--crate-name", "signed_bytes_emitted", "-C", "opt-level=3", "-C", "overflow-checks=off", "-o"])
        .arg(&binary)
        .arg(&code));
    let actual = run(&mut Command::new(binary));
    assert_eq!(actual.stdout, expected.stdout, "{emitted}");
    assert_eq!(actual.stderr, expected.stderr, "{emitted}");
}

#[test]
fn signed_machine_operations_override_unsigned_primitive_types() {
    let source = root().join("tests/fixtures/rust-signed-bytes/unsigned.rs");
    let native = common::scratch_file("rustsigned-unsigned-native", "elf");
    run(Command::new("rustc").args(["--crate-name", "signed_unsigned", "-O", "-o"])
        .arg(&native).arg(&source));
    let expected = run(&mut Command::new(&native));
    let mut emitted = String::new();
    let prototypes = [
        "unsigned int unsigned_less8(unsigned char a, unsigned char b)",
        "unsigned char unsigned_shift8(unsigned char a, unsigned int n)",
        "unsigned int unsigned_less16(unsigned short a, unsigned short b)",
        "unsigned short unsigned_shift16(unsigned short a, unsigned int n)",
        "unsigned int unsigned_less32(unsigned int a, unsigned int b)",
        "unsigned int unsigned_shift32(unsigned int a, unsigned int n)",
        "unsigned int unsigned_less64(unsigned long a, unsigned long b)",
        "unsigned long unsigned_shift64(unsigned long a, unsigned int n)",
        "unsigned char unsigned_loop(char a, unsigned int n)",
    ];
    for prototype in prototypes {
        let name = prototype.split('(').next().unwrap().split_whitespace().last().unwrap();
        let program = bootstrap_from_file(
            root().join("tests/fixtures/rust-signed-bytes/unsigned.xml").to_str().unwrap(),
            "", &[root().join("specs").display().to_string()],
        ).unwrap();
        let mut status = ConsoleCommands::into_status(vec![
            "option outlang rust".into(), format!("parse line extern {prototype};"),
            format!("lo fu {name}"), "decompile".into(), "print C".into(),
        ]);
        register_decomp_commands(&mut status);
        kuna_console::kuna_console::register_kuna_commands(&mut status);
        status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut()
            .downcast_mut::<IfaceDecompData>().unwrap().conf = Some(program);
        for _ in 0..4 { execute(&mut status); assert!(!status.inerror, "{}", status.optr); }
        let start = status.optr.len();
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
        emitted.push_str(status.optr[start..].trim());
        emitted.push('\n');
    }
    let original = std::fs::read_to_string(&source).unwrap();
    emitted.push_str(&original[original.find("fn main()").unwrap()..]);
    let code = common::scratch_file("rustsigned-unsigned-emitted", "rs");
    let binary = common::scratch_file("rustsigned-unsigned-execute", "elf");
    std::fs::write(&code, &emitted).unwrap();
    run(Command::new("rustc").args(["--crate-name", "signed_unsigned_emitted", "-O", "-o"])
        .arg(&binary).arg(&code));
    let actual = run(&mut Command::new(binary));
    assert_eq!(actual.stdout, expected.stdout, "{emitted}");
    assert_eq!(actual.stderr, expected.stderr, "{emitted}");
}
