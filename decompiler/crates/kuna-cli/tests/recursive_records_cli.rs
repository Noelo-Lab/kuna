//! GH-782: accepted recursive declarations must recover fields beyond one pointer.
use std::path::PathBuf;
use std::process::Command;

fn image() -> Vec<u8> {
    let mut b = vec![0; 0x1000];
    b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        b[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        b[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    // mov rax,[rdi]; mov rax,[rax]; mov eax,[rax+8]; ret
    let code: &[u8] = &[0x48, 0x8b, 0x07, 0x48, 0x8b, 0x00, 0x8b, 0x40, 0x08, 0xc3];
    let size = (b.len() + code.len()) as u64;
    for (at, value) in [
        (24, 0x401000u64),
        (32, 64),
        (80, 0x400000),
        (88, 0x400000),
        (96, size),
        (104, size),
        (112, 0x1000),
    ] {
        b[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    b.extend_from_slice(code);
    b
}

fn run(case: &str, records: &[&str], argument: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let dir = std::env::temp_dir().join(format!("kuna-records-{}-{case}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let binary = dir.join("fixture.elf");
    std::fs::write(&binary, image()).unwrap();
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut cmd = Command::new(kuna);
    cmd.arg("decompile-all")
        .arg(&binary)
        .args([
            "--addr",
            "0x401000",
            "--mode",
            "reliable",
            "--assert-strict",
            "--sleighpath",
        ])
        .arg(specs)
        .args(["--assert", "function 0x401000-0x40100a=read_chain"]);
    for record in records {
        cmd.args(["--assert", record]);
    }
    cmd.args([
        "--assert",
        &format!("prototype read_chain int read_chain({argument} *node)"),
    ]);
    let output = cmd.output().expect("run kuna");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn self_referencing_record_emits_deep_fields() {
    let code = run(
        "self",
        &["typedef struct Node { struct Node *next; int value; };"],
        "struct Node",
    );
    assert!(code.contains("node->next->next->value"), "{code}");
}

#[test]
fn forward_typedef_and_callback_record_emits_deep_fields() {
    let code = run(
        "forward",
        &[
            "typedef typedef struct Node Node;",
            "typedef struct Node { Node *next; int value; Node *(*visit)(Node *); };",
        ],
        "Node",
    );
    assert!(code.contains("node->next->next->value"), "{code}");
}

#[test]
fn mutually_referencing_records_emit_deep_fields() {
    let code = run(
        "mutual",
        &[
            "typedef struct A { struct B *b; int value; };",
            "typedef struct B { struct A *a; };",
        ],
        "struct A",
    );
    assert!(code.contains("node->b->a->value"), "{code}");
}
