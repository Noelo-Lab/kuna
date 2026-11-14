//! An interrupt that ends an MS-DOS program (`INT 20h`, `INT 27h`, `INT 21h`
//! with AH 00h, 31h or 4Ch) ends the flow, and a function every path of which
//! reaches one does not return to its caller.

use crate::common;

use std::process::{Command, Output};

/// Hand-assembled real-mode code (org 0). The entry calls `print` (0x20: `mov
/// ah,9; mov dx,100h; int 21h; ret`), `unknown` (0x30: `mov ah,[bx]; int 21h;
/// ret`), `shared` (0x40: `cmp al,0; je L; mov ah,4ch; L: int 21h; ret`, whose
/// interrupt the branch reaches with any AH) and `maybe` (0x70: `test al,al; jz
/// skip; call quit; skip: ret`), then `mov ax,3; push ax; call exit`. `exit`
/// (0x60) calls `_exit` (0x50: `mov bp,sp; mov ah,4ch; mov al,[bp+2]; int
/// 21h`) and would return after it; `quit` (0x80) is `mov ax,4c01h; int 21h`.
/// Zero fill follows each call or interrupt that does not come back.
fn exit_module() -> Vec<u8> {
    let mut module = vec![0u8; 0xb0];
    let code: [(usize, &[u8]); 8] = [
        (0x00, &[0xe8, 0x1d, 0x00, 0xe8, 0x2a, 0x00, 0xe8, 0x37, 0x00, 0xe8, 0x64, 0x00, 0xb8, 0x03, 0x00, 0x50, 0xe8, 0x4d, 0x00]),
        (0x20, &[0xb4, 0x09, 0xba, 0x00, 0x01, 0xcd, 0x21, 0xc3]),
        (0x30, &[0x8a, 0x27, 0xcd, 0x21, 0xc3]),
        (0x40, &[0x3c, 0x00, 0x74, 0x02, 0xb4, 0x4c, 0xcd, 0x21, 0xc3]),
        (0x50, &[0x8b, 0xec, 0xb4, 0x4c, 0x8a, 0x46, 0x02, 0xcd, 0x21]),
        (0x60, &[0x55, 0x8b, 0xec, 0xff, 0x76, 0x04, 0xe8, 0xe7, 0xff, 0x59, 0x5d, 0xc3]),
        (0x70, &[0x84, 0xc0, 0x74, 0x03, 0xe8, 0x09, 0x00, 0xc3]),
        (0x80, &[0xb8, 0x01, 0x4c, 0xcd, 0x21]),
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

fn stdout(args: &[&str]) -> String {
    let out = kuna(args);
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "{args:?}: {text}\n{}", String::from_utf8_lossy(&out.stderr));
    text
}

fn written(stem: &str, extension: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = common::scratch_file(stem, extension);
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn a_dos_exit_ends_the_flow_and_a_function_that_reaches_one_does_not_return() {
    let path = written("dos-exit", "exe", &dos_image(&exit_module()));
    let exe = path.to_str().unwrap();

    let entry = stdout(&["decompile", exe, "sub_10000"]);
    assert!(
        entry.contains(
            "{\n  sub_10020();\n  sub_10030();\n  sub_10040();\n  sub_10070();\n  sub_10060(3); // no-return\n}"
        ),
        "the interrupts that may return keep the entry going, exit ends it: {entry}"
    );
    for name in ["sub_10050", "sub_10080"] {
        let body = stdout(&["decompile", exe, name]);
        assert!(body.contains("{\n  (*(void (*)(void))swi(0x21))(); // no-return\n}"), "{name}: {body}");
    }
    let maybe = stdout(&["decompile", exe, "sub_10070"]);
    assert!(maybe.contains("sub_10080(); // no-return"), "{maybe}");
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_com_program_ends_at_int_20h_and_at_int_21h_function_0() {
    // org 100h: `mov ah,9; mov dx,110h; int 21h; test al,al; jz L; int 20h;
    // L: xor ah,ah; int 21h; db 'hi$'`.
    let com = [
        0xb4, 0x09, 0xba, 0x10, 0x01, 0xcd, 0x21, 0x84, 0xc0, 0x74, 0x02, 0xcd, 0x20, 0x32, 0xe4, 0xcd, 0x21, 0x68,
        0x69, 0x24,
    ];
    let path = written("dos-exit", "com", &com);
    let raw = ["--addr", "--raw-image", "--target", "x86:LE:16:Real Mode:default", "--base", "0x100"];
    let text = stdout(&[&["decompile", path.to_str().unwrap(), "0x100"][..], &raw].concat());
    assert!(
        text.contains("    (*(void (*)(void))swi(0x20))(); // no-return\n  (*(void (*)(void))swi(0x21))(); // no-return\n}"),
        "{text}"
    );

    // The same interrupt is not a DOS call outside real mode.
    let path32 = written("dos-exit-32", "bin", &[0xb4, 0x4c, 0xcd, 0x21, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3]);
    let raw32 = ["--addr", "--raw-image", "--target", "x86:LE:32:default", "--base", "0x1000"];
    let text32 = stdout(&[&["decompile", path32.to_str().unwrap(), "0x1000"][..], &raw32].concat());
    assert!(text32.contains("return 1;"), "{text32}");
    std::fs::remove_file(&path).unwrap();
    std::fs::remove_file(&path32).unwrap();
}
