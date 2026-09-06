//! Explicit ARM context reaches each CLI loading path, including JSON and inspection commands.

#[path = "common/arm_images.rs"]
mod arm_images;
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

#[test]
fn explicit_thumb_applies_without_elf_section_headers() {
    let mut bytes = arm_images::elf(&[0x07, 0x20, 0x70, 0x47], &[], &[]);
    bytes[32..36].fill(0); // e_shoff
    bytes[46..52].fill(0); // e_shentsize, e_shnum, e_shstrndx
    let path = common::scratch_file("sectionless-thumb", "elf");
    std::fs::write(&path, bytes).unwrap();
    for command in ["disassemble", "read"] {
        let output = run(
            command,
            path.to_str().unwrap(),
            &[
                "0x10000", "--as", "code", "--count", "2", "--json", "--isa", "thumb",
            ],
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(text.matches("\"size\": 2").count(), 2, "{command}: {text}");
        assert!(text.contains("\"bytes\": \"0720\""), "{command}: {text}");
        assert!(text.contains("\"bytes\": \"7047\""), "{command}: {text}");
    }
    let output = run(
        "decompile",
        path.to_str().unwrap(),
        &["0x10000", "--isa", "thumb", "--json"],
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        output.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("return 7;"), "{text}");
    std::fs::remove_file(path).unwrap();
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

#[test]
fn explicit_thumb_survives_graph_xrefs_and_discovery() {
    let path = common::scratch_file("arm-metadata-override", "elf");
    // push {lr}; bl 0x10008; pop {pc}; movs r0,#7; bx lr
    let code = [
        0x00, 0xb5, 0x00, 0xf0, 0x01, 0xf8, 0x00, 0xbd, 0x07, 0x20, 0x70, 0x47,
    ];
    std::fs::write(
        &path,
        arm_images::elf(&code, &[(0, "$a")], &[(0, "entry", 8)]),
    )
    .unwrap();
    for (command, args, expected) in [
        ("functions", vec!["--json"], "\"address_hex\": \"0x10008\""),
        (
            "xrefs",
            vec!["--from", "entry", "--json"],
            "\"kind\": \"call\"",
        ),
        (
            "decompile-graph",
            vec!["--addr", "0x10008", "--option", "fast_funcdisc", "on"],
            "movs r0,#0x7",
        ),
    ] {
        let mut args = args;
        args.extend(["--isa", "thumb"]);
        let out = run(command, path.to_str().unwrap(), &args);
        assert!(
            out.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.contains(expected), "{command}: {text}");
        if command == "decompile-graph" {
            assert!(text.contains("return 7;"), "{text}");
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn explicit_arm_survives_thumb_metadata_in_graph() {
    let path = common::scratch_file("thumb-metadata-override", "elf");
    let code = [0x07, 0x00, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1];
    std::fs::write(
        &path,
        arm_images::elf(&code, &[(0, "$t")], &[(0, "entry", 8)]),
    )
    .unwrap();
    let out = run("decompile-graph", path.to_str().unwrap(), &["--isa", "arm"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("return 7;"), "{text}");
    assert!(text.contains("mov r0,#0x7"), "{text}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn auto_preserves_mixed_arm_thumb_metadata() {
    let path = common::scratch_file("mixed-arm-thumb", "elf");
    let code = [
        0x07, 0x00, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1, 0x09, 0x20, 0x70, 0x47,
    ];
    std::fs::write(
        &path,
        arm_images::elf(
            &code,
            &[(0, "$a"), (8, "$t")],
            &[(0, "arm_entry", 8), (8, "thumb_entry", 4)],
        ),
    )
    .unwrap();
    let out = run("decompile-graph", path.to_str().unwrap(), &[]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(
        text.contains("return 7;") && text.contains("return 9;"),
        "{text}"
    );
    assert!(
        text.contains("mov r0,#0x7") && text.contains("movs r0,#0x9"),
        "{text}"
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn thumb_coff_reaches_loading_and_inspection_surfaces() {
    let path = common::scratch_file("thumb-coff", "obj");
    std::fs::write(&path, arm_images::thumb_coff()).unwrap();
    for (command, args, expected) in [
        ("functions", vec![], "entry"),
        (
            "functions",
            vec!["--target", "ARM:LE:32:v4t:default"],
            "entry",
        ),
        ("decompile", vec!["entry", "--json"], "return 7;"),
        ("decompile", vec!["entry"], "return 7;"),
        ("decompile-graph", vec![], "return 7;"),
        ("disassemble", vec!["entry", "--count", "2"], "movs r0,#0x7"),
        ("xrefs", vec!["--from", "entry", "--json"], "\"xrefs\":"),
        ("strings", vec!["--json"], "\"strings\":"),
    ] {
        let out = run(command, path.to_str().unwrap(), &args);
        assert!(
            out.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let text = String::from_utf8(out.stdout).unwrap();
        assert!(text.contains(expected), "{command}: {text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn thumb_coff_project_keeps_section_metadata() {
    let path = common::scratch_file("thumb-coff-project", "obj");
    let directory = common::scratch_file("thumb-coff-project", "output");
    std::fs::write(&path, arm_images::thumb_coff()).unwrap();
    let out = run(
        "decompile-project",
        path.to_str().unwrap(),
        &["-o", directory.to_str().unwrap()],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let readme = std::fs::read_to_string(directory.join("README.md")).unwrap();
    assert!(
        readme.contains("`.text`") && readme.contains("`.rdata`"),
        "{readme}"
    );
    std::fs::remove_file(path).unwrap();
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn armnt_machine_decodes_thumb_without_flags() {
    let mut bytes = std::fs::read(
        repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armv4t_thumb_pe.exe"),
    )
    .unwrap();
    let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
    bytes[pe + 4..pe + 6].copy_from_slice(&0x01c4u16.to_le_bytes());
    let path = common::scratch_file("armnt-auto", "exe");
    std::fs::write(&path, bytes).unwrap();
    let out = run("decompile", path.to_str().unwrap(), &["0x401000", "--json"]);
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(
        out.status.success(),
        "{text}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(text.contains("return 7;"), "ARMNT decoded as A32: {text}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn strings_refuses_an_isa_it_would_never_apply() {
    let binary = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware");
    let output = run(
        "strings",
        binary.to_str().unwrap(),
        &["--no-xrefs", "--isa", "thumb"],
    );
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--isa has no effect with --no-xrefs"), "{stderr}");
}
