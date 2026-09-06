//! Explicit decoder selection remains independent of object header layout.

#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;
mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn run(command: &str, binary: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg(command)
        .arg(binary)
        .args(args)
        .args(["--mode", "fast", "--sleighpath"])
        .arg(repo_root().join("specs"))
        .output()
        .expect("run kuna")
}

fn successful(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn inferred_thumb_metadata_preserves_explicit_x86_decoder() {
    let source = std::fs::read(
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe"),
    )
    .unwrap();
    let path = common::scratch_file("pe-decoder-override", "exe");
    for machine in [0x01c2u16, 0x01c4] {
        let mut bytes = source.clone();
        let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        let section =
            pe + 24 + u16::from_le_bytes(bytes[pe + 20..pe + 22].try_into().unwrap()) as usize;
        bytes[pe + 4..pe + 6].copy_from_slice(&machine.to_le_bytes());
        bytes[pe + 40..pe + 44].copy_from_slice(&0x1000u32.to_le_bytes());
        bytes[section + 8..section + 12].copy_from_slice(&6u32.to_le_bytes());
        bytes[0x200..0x206].copy_from_slice(&[0xb8, 7, 0, 0, 0, 0xc3]);
        std::fs::write(&path, bytes).unwrap();
        for json in [false, true] {
            let mut args = vec!["0x401000", "--target", "x86:LE:32:default:gcc"];
            if json {
                args.push("--json");
            }
            let text = successful(run("decompile", &path, &args));
            assert!(text.contains("return 7;"), "{text}");
        }
        for isa in ["arm", "thumb"] {
            let output = run(
                "functions",
                &path,
                &["--target", "x86:LE:32:default:gcc", "--isa", isa],
            );
            assert!(!output.status.success());
            let error = String::from_utf8(output.stderr).unwrap();
            assert!(
                error.contains("requires a 32-bit ARM SLEIGH target"),
                "{error}"
            );
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn elf32_accepts_16_bit_x86_decoder() {
    // .code16: mov ax,7; ret, carried in an ELF32 executable.
    let mut bytes = arm_images::elf(&[0xb8, 0x07, 0x00, 0xc3], &[], &[(0, "entry", 4)]);
    bytes[18..20].copy_from_slice(&object::elf::EM_386.to_le_bytes());
    bytes[36..40].fill(0); // x86 e_flags
    let path = common::scratch_file("elf32-code16", "elf");
    std::fs::write(&path, bytes).unwrap();
    for command in ["disassemble", "read"] {
        let json = successful(run(
            command,
            &path,
            &[
                "0x10000",
                "--as",
                "code",
                "--count",
                "2",
                "--json",
                "--target",
                "x86:LE:16:Real Mode:default",
            ],
        ));
        assert!(json.contains("\"bytes\": \"b80700\""), "{json}");
        assert!(json.contains("\"size\": 3"), "{json}");
        assert!(json.contains("\"bytes\": \"c3\""), "{json}");
        assert!(json.contains("\"size\": 1"), "{json}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn default_target_matches_automatic_elf_and_pe_loading() {
    let code: Vec<u8> = [0xe3a00007u32, 0xe12fff1e]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    let elf = common::scratch_file("default-target", "elf");
    std::fs::write(&elf, arm_images::elf(&code, &[], &[(0, "entry", 8)])).unwrap();
    let pe = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe");
    for (path, address) in [(&elf, "0x10000"), (&pe, "0x401000")] {
        for (command, args) in [
            ("functions", vec!["--json"]),
            ("decompile", vec![address, "--json"]),
        ] {
            let automatic = successful(run(command, path, &args));
            let mut explicit = args;
            explicit.extend(["--target", "default"]);
            let default = successful(run(command, path, &explicit));
            assert_eq!(
                default, automatic,
                "{command}: default changed automatic selection"
            );
            if command == "decompile" {
                assert!(default.contains("return 7;"), "{default}");
            }
        }
    }
    std::fs::remove_file(elf).unwrap();
}

#[test]
fn fixed_a32_languages_accept_explicit_arm() {
    let code: Vec<u8> = [0xe3a00007u32, 0xe1a0f00e]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    let path = common::scratch_file("fixed-a32", "elf");
    std::fs::write(&path, arm_images::elf(&code, &[], &[(0, "entry", 8)])).unwrap();
    for target in ["ARM:LE:32:v4:default", "ARM:LE:32:v5:default"] {
        for (command, args) in [
            ("decompile", vec!["entry", "--json", "--target", target]),
            (
                "disassemble",
                vec!["entry", "--count", "2", "--json", "--target", target],
            ),
        ] {
            let automatic = successful(run(command, &path, &args));
            let mut explicit = args;
            explicit.extend(["--isa", "arm"]);
            let arm = successful(run(command, &path, &explicit));
            assert_eq!(arm, automatic, "{target}: {command}");
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn fixed_a32_languages_diagnose_unsupported_thumb() {
    let path = common::scratch_file("fixed-a32-thumb", "elf");
    for executable in [true, false] {
        let mut bytes = arm_images::elf(&[0x07, 0x20, 0x70, 0x47], &[], &[]);
        if !executable {
            bytes[32..36].fill(0);
            bytes[46..52].fill(0);
            let phoff = u32::from_le_bytes(bytes[28..32].try_into().unwrap()) as usize;
            bytes[phoff + 24..phoff + 28].copy_from_slice(&object::elf::PF_R.to_le_bytes());
        }
        std::fs::write(&path, bytes).unwrap();
        for target in ["ARM:LE:32:v4:default", "ARM:LE:32:v5:default"] {
            let output = run(
                "disassemble",
                &path,
                &[
                    "0x10000", "--count", "2", "--json", "--target", target, "--isa", "thumb",
                ],
            );
            assert!(!output.status.success());
            let error = String::from_utf8(output.stderr).unwrap();
            assert!(error.contains("does not support Thumb"), "{error}");
            assert!(error.contains(target), "{error}");
            assert!(error.contains("--target"), "{error}");
            assert!(!error.contains("Non-existent context variable"), "{error}");
        }
    }
    std::fs::remove_file(path).unwrap();
}
