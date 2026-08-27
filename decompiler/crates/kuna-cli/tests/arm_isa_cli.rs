//! Explicit ARM context reaches each CLI loading path, including JSON and inspection commands.

mod common;

use std::path::PathBuf;
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn run(command: &str, binary: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([command, binary])
        .args(args)
        .args(["--mode", "fast", "--sleighpath"])
        .arg(repo_root().join("specs"))
        .output()
        .expect("run kuna")
}

fn unmarked_thumb_pe() -> PathBuf {
    let mut bytes = std::fs::read(
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe"),
    )
    .unwrap();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    bytes[pe + 4..pe + 6].copy_from_slice(&0x01c0u16.to_le_bytes());
    let path = common::scratch_file("unmarked-thumb", "exe");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn explicit_thumb_reaches_single_function_json_and_graph() {
    let path = unmarked_thumb_pe();
    for (command, args) in [
        ("decompile", vec!["0x401000", "--json"]),
        ("decompile-graph", vec!["--addr", "0x401000"]),
    ] {
        let mut args = args;
        args.extend([
            "--isa",
            "thumb",
            "--target",
            "ARM:LE:32:v4t:default",
            "--define-function",
            "0x401000-0x401004=thumb_entry",
        ]);
        let output = run(command, path.to_str().unwrap(), &args);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("\"functions\":"), "{command}: {text}");
        assert!(text.contains("return 7;"), "{command}: {text}");
        assert!(
            text.contains("thumb_entry"),
            "{command}: declared name was lost: {text}"
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn explicit_thumb_controls_disassembly_and_read_code_view() {
    let path = unmarked_thumb_pe();
    for command in ["disassemble", "read"] {
        let output = run(
            command,
            path.to_str().unwrap(),
            &[
                "0x401000",
                "--as",
                "code",
                "--count",
                "2",
                "--json",
                "--isa",
                "thumb",
                "--target",
                "ARM:LE:32:v4t:default",
            ],
        );
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("\"count\": 2"), "{command}: {text}");
        assert_eq!(text.matches("\"size\": 2").count(), 2, "{command}: {text}");
        assert!(text.contains("\"bytes\": \"0720\""), "{command}: {text}");
        assert!(text.contains("\"bytes\": \"7047\""), "{command}: {text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn inspection_commands_forward_isa_to_target_validation() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware");
    for (command, args) in [
        ("disassemble", vec!["main"]),
        ("read", vec!["main"]),
        ("xrefs", vec!["--from", "main"]),
        ("strings", vec![]),
    ] {
        let mut args = args;
        args.extend(["--isa", "thumb"]);
        let output = run(command, binary.to_str().unwrap(), &args);
        assert_eq!(output.status.code(), Some(1), "{command}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("requires a 32-bit ARM SLEIGH target"),
            "{command}: {stderr}"
        );
    }
}
