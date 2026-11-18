//! A function of an MS-DOS program ends where its flow does, not at the next
//! function: the load module is one code range that holds the program's data
//! too, so the next entry is no bound on a function that data follows.

use crate::common;

use std::process::{Command, Output};

/// Hand-assembled real-mode code (org 0). The entry calls `get` (0x10: `mov
/// ax,[200h]; ret`, then 44 bytes of text), `pick` (0x40: `cmp bx,2; ja out;
/// shl bx,1; jmp cs:[bx+table]`, the table, `out: ret`, the three cases after
/// it, then text), `jump` (0xa0: `mov ax,[300h]; jmp ax`, then text), `tail`
/// (0xb0: `jmp [2ef2h]`, then text) and `quit` (0x80: `call die`, then zero
/// fill), where `die` (0x90) is `mov ah,4ch; int 21h`.
fn extent_module() -> Vec<u8> {
    let mut module = vec![0u8; 0xd0];
    let code: [(usize, &[u8]); 9] = [
        (0x00, &[0xe8, 0x0d, 0x00, 0xe8, 0x3a, 0x00, 0xe8, 0x97, 0x00, 0xe8, 0xa4, 0x00, 0xe8, 0x71, 0x00]),
        (0x10, &[0xa1, 0x00, 0x02, 0xc3]),
        (0x14, b"DGROUP data: not code, 0123456789abcdefghijk"),
        (
            0x40,
            &[
                0x83, 0xfb, 0x02, 0x77, 0x0d, 0xd1, 0xe3, 0x2e, 0xff, 0xa7, 0x4c, 0x00, 0x53, 0x00, 0x56, 0x00,
                0x59, 0x00, 0xc3, 0xb0, 0x01, 0xc3, 0xb0, 0x02, 0xc3, 0xb0, 0x03, 0xc3,
            ],
        ),
        (0x5c, b"strings after the switch cases..!"),
        (0x80, &[0xe8, 0x0d, 0x00]),
        (0x90, &[0xb4, 0x4c, 0xcd, 0x21]),
        (0xa0, &[0xa1, 0x00, 0x03, 0xff, 0xe0, b'm', b'o', b'r', b'e', b' ', b't', b'e', b'x', b't']),
        (0xb0, &[0xff, 0x26, 0xf2, 0x2e, b't', b'a', b'i', b'l', b' ', b't', b'e', b'x', b't']),
    ];
    for (at, bytes) in code {
        module[at..at + bytes.len()].copy_from_slice(bytes);
    }
    module
}

/// An MZ image whose load module is `module`, entry `0000:0000`, no relocations.
fn dos_image(module: &[u8]) -> Vec<u8> {
    let mut image = vec![0u8; 0x20];
    image[..2].copy_from_slice(b"MZ");
    let total = image.len() + module.len();
    let words: [(usize, u16); 5] = [
        (0x02, (total % 512) as u16),
        (0x04, total.div_ceil(512) as u16),
        (0x08, 2),
        (0x10, 0x100),
        (0x18, 0x1c),
    ];
    for (at, value) in words {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    image.extend_from_slice(module);
    image
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna")).args(args).output().expect("run kuna")
}

fn run(args: &[&str]) -> (String, String) {
    let out = kuna(args);
    let (stdout, stderr) =
        (String::from_utf8_lossy(&out.stdout).into_owned(), String::from_utf8_lossy(&out.stderr).into_owned());
    assert!(out.status.success(), "{args:?}: {stdout}\n{stderr}");
    (stdout, stderr)
}

fn written(stem: &str, extension: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = common::scratch_file(stem, extension);
    std::fs::write(&path, bytes).unwrap();
    path
}

fn sizes(json: &str) -> Vec<(String, u64)> {
    let doc: serde_json::Value = serde_json::from_str(json).expect("functions --json");
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["name"].as_str().unwrap().to_string(), f["size"].as_u64().unwrap()))
        .collect()
}

#[test]
fn a_dos_function_ends_where_its_flow_does() {
    let path = written("dos-extent", "exe", &dos_image(&extent_module()));
    let exe = path.to_str().unwrap();

    let (inventory, _) = run(&["functions", exe, "--json"]);
    let expected: Vec<(String, u64)> = [
        ("sub_10000", 15),
        ("sub_10010", 4),
        ("sub_10040", 28),
        ("sub_10080", 3),
        ("sub_10090", 4),
        ("sub_100a0", 16),
        ("sub_100b0", 4),
    ]
    .iter()
    .map(|&(name, size)| (name.to_string(), size))
    .collect();
    assert_eq!(sizes(&inventory), expected, "{inventory}");

    let (listing, notes) = run(&["disassemble", exe, "sub_10010"]);
    assert!(listing.starts_with("# 2 instructions at sub_10010 @ 0x10010 (0x10010..0x10014, 4 bytes)"), "{listing}");
    assert!(
        notes.contains(
            "note: sub_10010 ends at 0x10014, after the last instruction its flow reaches; its flow reaches none \
             of the 44 bytes from there to 0x10040 (the range 0x10014-0x10040 lists them)"
        ),
        "{notes}"
    );
    let (_, notes) = run(&["disassemble", exe, "0x10013"]);
    assert!(notes.contains("note: 0x10013 is in sub_10010, which ends at 0x10014, after"), "{notes}");
    let (range, _) = run(&["disassemble", exe, "0x10010-0x10040"]);
    assert!(
        range.contains("\n0x10014 ") && range.contains("\n0x10039 "),
        "an explicit range lists what it asks for: {range}"
    );
    let (switch, _) = run(&["disassemble", exe, "sub_10040"]);
    assert!(switch.contains("(0x10040..0x1005c, 28 bytes)") && switch.contains("0x10059 "), "{switch}");

    let (_, past) = run(&["decompile", exe, "0x10020", "--addr"]);
    assert!(!past.contains("interior to"), "text after a function's end is not its interior: {past}");
    assert!(
        past.contains(
            "note: 0x10020 is past the end of sub_10010 at 0x10014, after the last instruction its flow reaches, \
             and in no function kuna discovered"
        ),
        "{past}"
    );
    let (_, inside) = run(&["decompile", exe, "0x10013", "--addr"]);
    assert!(inside.contains("0x10013 is interior to sub_10010 (0x10010+0x3)"), "{inside}");

    let project = common::scratch_file("dos-extent-project", "d");
    run(&["decompile-project", exe, "-o", project.to_str().unwrap()]);
    let asm_name = format!("{}.asm", path.file_name().unwrap().to_str().unwrap());
    let asm = std::fs::read_to_string(project.join(asm_name)).unwrap();
    let tail = asm.split("\nsub_10010:").nth(1).expect("sub_10010 label");
    let end = "  ; end of sub_10010: its flow reaches none of the 44 bytes from here to 0x10040\n";
    assert!(tail.contains(&format!("RET\n{end}")), "the .asm marks where sub_10010's flow ends: {tail}");
    std::fs::remove_dir_all(&project).unwrap();
    std::fs::remove_file(&path).unwrap();
}

/// org 0: the entry calls `e1` (0x10: `mov ax,1; jmp tail`) and `e2` (0x20:
/// `test ax,ax; jnz e1; ret`, three bytes of text, `tail: inc ax; ret`).
/// `tail` is in `e2`'s span and reached from it only through `e1`'s entry.
fn branch_into_entry_module() -> Vec<u8> {
    let mut module = vec![0u8; 0x40];
    let code: [(usize, &[u8]); 4] = [
        (0x00, &[0xe8, 0x0d, 0x00, 0xe8, 0x1a, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21]),
        (0x10, &[0xb8, 0x01, 0x00, 0xeb, 0x13]),
        (0x20, &[0x85, 0xc0, 0x75, 0xec, 0xc3, b'g', b'a', b'p', 0x40, 0xc3]),
        (0x2a, b"trailing text"),
    ];
    for (at, bytes) in code {
        module[at..at + bytes.len()].copy_from_slice(bytes);
    }
    module
}

#[test]
fn a_conditional_branch_into_another_entry_is_followed_as_the_decompiler_follows_it() {
    let path = written("dos-extent-cbranch", "exe", &dos_image(&branch_into_entry_module()));
    let exe = path.to_str().unwrap();
    let (inventory, _) = run(&["functions", exe, "--json"]);
    let expected: Vec<(String, u64)> =
        [("sub_10000", 11), ("sub_10010", 5), ("sub_10020", 10)].iter().map(|&(n, s)| (n.to_string(), s)).collect();
    assert_eq!(sizes(&inventory), expected, "{inventory}");
    let (c, _) = run(&["decompile", exe, "sub_10020", "--raw"]);
    assert!(c.contains("\n0x00010028:"), "the decompiler's flow reaches the tail too: {c}");
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_com_program_ends_before_its_string() {
    // org 100h: `mov ah,9; mov dx,110h; int 21h; test al,al; jz L; int 20h;
    // L: xor ah,ah; int 21h; db 'hi$'`.
    let com = [
        0xb4, 0x09, 0xba, 0x10, 0x01, 0xcd, 0x21, 0x84, 0xc0, 0x74, 0x02, 0xcd, 0x20, 0x32, 0xe4, 0xcd, 0x21, 0x68,
        0x69, 0x24,
    ];
    let path = written("dos-extent", "com", &com);
    let raw = ["--raw-image", "--target", "x86:LE:16:Real Mode:default", "--base", "0x100", "--entry", "0x100"];
    let (inventory, _) = run(&[&["functions", path.to_str().unwrap(), "--json"][..], &raw].concat());
    assert_eq!(sizes(&inventory), vec![("sub_100".to_string(), 17)], "{inventory}");
    std::fs::remove_file(&path).unwrap();
}
