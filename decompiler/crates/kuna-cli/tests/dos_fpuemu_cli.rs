//! An MS-DOS executable linked with the 8087 emulator decodes its `INT 34h`-`3Dh`
//! as the x87 instructions they stand for (`--option fpuemu`, default `auto`).

use crate::common;

use std::process::{Command, Output};

/// Hand-assembled real-mode code (nasm, org 0) with each `WAIT; ESC` rewritten
/// the way the emulator fixups rewrite it. The entry calls `fles` (0x40:
/// `les bx,[bp+4]; fld dword [es:bx]; fstp qword [res]; fwait`) and `scale`
/// (0x25: `fld qword [bp+4]; fmul qword [k]; fld1; faddp; fstp qword [res];
/// fwait`); 0x53 loads through each segment override; `k` = 2.5 at 0x68, `res`
/// at 0x70.
const EMULATED: [u8; 128] = [
    0x1e, 0x68, 0x78, 0x00, 0xe8, 0x39, 0x00, 0x83, 0xc4, 0x04, 0xff, 0x36, 0x7e, 0x00, 0xff, 0x36,
    0x7c, 0x00, 0xff, 0x36, 0x7a, 0x00, 0xff, 0x36, 0x78, 0x00, 0xe8, 0x08, 0x00, 0x83, 0xc4, 0x08,
    0xb8, 0x00, 0x4c, 0xcd, 0x21, 0x55, 0x89, 0xe5, 0xcd, 0x39, 0x46, 0x04, 0xcd, 0x38, 0x0e, 0x68,
    0x00, 0xcd, 0x35, 0xe8, 0xcd, 0x3a, 0xc1, 0xcd, 0x39, 0x1e, 0x70, 0x00, 0xcd, 0x3d, 0x5d, 0xc3,
    0x55, 0x89, 0xe5, 0xc4, 0x5e, 0x04, 0xcd, 0x3c, 0xd9, 0x07, 0xcd, 0x39, 0x1e, 0x70, 0x00, 0xcd,
    0x3d, 0x5d, 0xc3, 0xcd, 0x3c, 0x19, 0x07, 0xcd, 0x3c, 0x59, 0x07, 0xcd, 0x3c, 0x9d, 0x06, 0x68,
    0x00, 0xcd, 0x3c, 0xd9, 0x05, 0xcd, 0x3d, 0xc3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x40,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40,
];

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

/// The entry calls the `INT 35h; RET` thunk of a table holding one for every
/// vector 30h-3Fh, the shape of a generic interrupt-call helper.
fn thunk_table() -> Vec<u8> {
    let mut module = vec![0x90u8; 0x10];
    module[..8].copy_from_slice(&[0xe8, 0x1c, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21]);
    for n in 0x30u8..=0x3f {
        module.extend_from_slice(&[0xcd, n, 0xc3]);
    }
    module
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

fn written(stem: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = common::scratch_file(stem, "exe");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn an_emulator_program_decodes_its_interrupts_as_x87_instructions() {
    let path = written("dos-fpuemu", &dos_image(&EMULATED));
    let exe = path.to_str().unwrap();

    let listing = stdout(&["functions", exe, "--json"]);
    for name in ["\"sub_10025\"", "\"sub_10040\""] {
        assert!(listing.contains(name), "missing {name}: {listing}");
    }

    let code = stdout(&["disassemble", exe, "0x10025-0x10068"]);
    for line in [
        "cd35e8                FLD1",
        "cd3ac1                FADDP",
        "cd3d                  WAIT",
        "cd3cd907              FLD float ptr ES:[BX]",
        "cd3c1907              FLD float ptr DS:[BX]",
        "cd3c5907              FLD float ptr SS:[BX]",
        "cd3c9d066800          FLD double ptr CS:[0x68]",
    ] {
        assert!(code.contains(line), "missing {line:?}: {code}");
    }
    let off = stdout(&["disassemble", exe, "0x10025-0x10040", "--option", "fpuemu", "off"]);
    assert!(off.contains("INT 0x35") && !off.contains("FLD1"), "{off}");

    let c = stdout(&["decompile-all", exe, "--addr", "0x10025", "--addr", "0x10040"]);
    assert!(c.contains("void sub_10025(double a0)"), "{c}");
    assert!(
        c.contains("*(double *)0x70 = (double)((long double)1 + (long double)a0 * (long double)*(double *)0x68);"),
        "{c}"
    );
    assert!(c.contains("*(double *)0x70 = (double)*a0;"), "{c}");
    assert!(!c.contains("swi(0x3"), "{c}");
    let c_off = stdout(&["decompile-all", exe, "--addr", "0x10025", "--option", "fpuemu", "off"]);
    assert!(c_off.contains("swi(0x39)"), "{c_off}");
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn an_interrupt_thunk_table_is_not_emulator_code() {
    let path = written("dos-fpuemu-thunks", &dos_image(&thunk_table()));
    let exe = path.to_str().unwrap();
    let auto = stdout(&["disassemble", exe, "0x1001f-0x10022"]);
    assert!(auto.contains("INT 0x35"), "{auto}");
    let on = stdout(&["disassemble", exe, "0x1001f-0x10022", "--option", "fpuemu", "on"]);
    assert!(on.contains("FLD ST3"), "{on}");
    std::fs::remove_file(&path).unwrap();
}
