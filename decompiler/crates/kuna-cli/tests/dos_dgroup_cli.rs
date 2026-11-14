//! An MS-DOS executable's functions start with DS holding the data segment its
//! startup code loads (option `mzdgroup`), so their near data pointers name the
//! program's globals.

use crate::common;

use std::process::{Command, Output};

/// `sum` at module offset 0x40 adds the words at DS:2 and DS:4. The data
/// segment is paragraph 0x10 of the module, linear 0x10100 once loaded at
/// segment 0x1000.
const SUM: [u8; 8] = [0xa1, 0x02, 0x00, 0x03, 0x06, 0x04, 0x00, 0xc3];

/// A DOS executable whose entry code is `entry`, with `sum` at 0x40, data at
/// 0x100 and a relocation for each module offset in `relocated`.
fn dos_image(entry: &[u8], relocated: &[u16]) -> Vec<u8> {
    let mut module = vec![0x90u8; 0x110];
    module[..entry.len()].copy_from_slice(entry);
    module[0x40..0x48].copy_from_slice(&SUM);
    module[0x100..0x106].copy_from_slice(&[0x00, 0x00, 0x05, 0x00, 0x07, 0x00]);
    let mut header = vec![0u8; 0x1c];
    header[..2].copy_from_slice(b"MZ");
    for &at in relocated {
        header.extend_from_slice(&at.to_le_bytes());
        header.extend_from_slice(&0u16.to_le_bytes());
    }
    header.resize(header.len().div_ceil(16) * 16, 0);
    let total = header.len() + module.len();
    let words: [(usize, u16); 6] = [
        (0x02, (total % 512) as u16),
        (0x04, total.div_ceil(512) as u16),
        (0x06, relocated.len() as u16),
        (0x08, (header.len() / 16) as u16),
        (0x10, 0x100),
        (0x18, 0x1c),
    ];
    for (at, value) in words {
        header[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    header.extend_from_slice(&module);
    header
}

/// `mov bx, [2]; mov ax, seg DATA; mov ds, ax; mov [6], bx; call sum; mov ax,
/// 4c00h; int 21h`: the entry reads the PSP's word 2 before it loads DS.
const MASM: [u8; 21] = [
    0x8b, 0x1e, 0x02, 0x00, 0xb8, 0x10, 0x00, 0x8e, 0xd8, 0x89, 0x1e, 0x06, 0x00, 0xe8, 0x30, 0x00, 0xb8, 0x00, 0x4c,
    0xcd, 0x21,
];

/// Turbo C's c0: `mov dx, DGROUP; mov ah, 30h; int 21h; mov bp, [2]; mov ds,
/// dx; call sum; mov ax, 4c00h; int 21h`. The DOS version call returns in AX,
/// BX and CX, so DX still holds DGROUP.
const TURBO_C: [u8; 21] = [
    0xba, 0x10, 0x00, 0xb4, 0x30, 0xcd, 0x21, 0x8b, 0x2e, 0x02, 0x00, 0x8e, 0xda, 0xe8, 0x30, 0x00, 0xb8, 0x00, 0x4c,
    0xcd, 0x21,
];

/// `mov ah, 30h; int 21h; cmp al, 2; jae ok; push cs; pop ds; mov dx, 100h;
/// mov ah, 9; int 21h; int 20h; ok: call sum; ...`: only the DOS 1 path, which
/// exits, loads DS, so `sum` runs with the PSP's.
const DS_ON_ONE_PATH: [u8; 27] = [
    0xb4, 0x30, 0xcd, 0x21, 0x3c, 0x02, 0x73, 0x0b, 0x0e, 0x1f, 0xba, 0x00, 0x01, 0xb4, 0x09, 0xcd, 0x21, 0xcd, 0x20,
    0xe8, 0x2a, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21,
];

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna")).args(args).output().expect("run kuna")
}

fn decompiled(stem: &str, image: &[u8], name: &str, extra: &[&str]) -> String {
    let path = common::scratch_file(stem, "exe");
    std::fs::write(&path, image).unwrap();
    let out = kuna(&[&["decompile", path.to_str().unwrap(), name][..], extra].concat());
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
    std::fs::remove_file(&path).unwrap();
    text
}

#[test]
fn a_function_reads_the_data_segment_the_startup_code_loads() {
    let masm = dos_image(&MASM, &[5]);
    let named = decompiled("dos-dgroup-masm", &masm, "sub_10040", &[]);
    assert!(named.contains("return dat_10102 + dat_10104;"), "{named}");
    let off = decompiled("dos-dgroup-masm-off", &masm, "sub_10040", &["--option", "mzdgroup", "off"]);
    assert!(off.contains("*(short *)0x2") && off.contains("*(short *)0x4") && !off.contains("dat_1010"), "{off}");
    let entry = decompiled("dos-dgroup-masm-entry", &masm, "sub_10000", &[]);
    assert!(
        entry.contains("dat_10106 = *(unsigned short *)0x2;"),
        "the entry starts with DS holding the PSP and loads DGROUP itself: {entry}"
    );

    let turbo_c = dos_image(&TURBO_C, &[1]);
    let named = decompiled("dos-dgroup-turboc", &turbo_c, "sub_10040", &[]);
    assert!(named.contains("return dat_10102 + dat_10104;"), "{named}");
}

#[test]
fn a_segment_only_an_unrelocated_or_exiting_path_loads_is_not_the_data_segment() {
    let unrelocated = decompiled("dos-dgroup-unrelocated", &dos_image(&MASM, &[]), "sub_10040", &[]);
    assert!(!unrelocated.contains("dat_1010"), "{unrelocated}");
    let one_path = decompiled("dos-dgroup-one-path", &dos_image(&DS_ON_ONE_PATH, &[]), "sub_10040", &[]);
    assert!(!one_path.contains("dat_10"), "{one_path}");
}
