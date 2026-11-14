//! An MS-DOS program's reads and writes of the interrupt vector table, the BIOS
//! data area and video memory are volatile: the machine changes or reads that
//! memory behind the program's back.

use crate::common;

use std::process::{Command, Output};

/// Hand-assembled real-mode code (org 0). The entry calls `tick` (0x10: `mov
/// ax,40h; mov es,ax; mov bx,6ch; mov al,es:[bx]; L: cmp al,es:[bx]; jz L;
/// ret`, a wait for the BIOS timer tick) and `vga` (0x30: `mov ax,0a000h; mov
/// es,ax; mov al,es:[0]; mov byte es:[0],1; mov byte es:[0],2; ret`, a latch
/// load and two writes to one VGA address) and `hook` (0x50: `xor ax,ax; mov
/// es,ax; mov word es:[24h],1234h; mov word es:[24h],5678h; ret`, two writes
/// to the INT 9 vector), then exits.
fn module() -> Vec<u8> {
    let mut module = vec![0u8; 0x80];
    let code: [(usize, &[u8]); 4] = [
        (0x00, &[0xe8, 0x0d, 0x00, 0xe8, 0x2a, 0x00, 0xe8, 0x47, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21]),
        (0x10, &[0xb8, 0x40, 0x00, 0x8e, 0xc0, 0xbb, 0x6c, 0x00, 0x26, 0x8a, 0x07, 0x26, 0x3a, 0x07, 0x74, 0xfb, 0xc3]),
        (0x30, &[
            0xb8, 0x00, 0xa0, 0x8e, 0xc0, 0x26, 0xa0, 0x00, 0x00, 0x26, 0xc6, 0x06, 0x00, 0x00, 0x01, 0x26, 0xc6, 0x06,
            0x00, 0x00, 0x02, 0xc3,
        ]),
        (0x50, &[
            0x31, 0xc0, 0x8e, 0xc0, 0x26, 0xc7, 0x06, 0x24, 0x00, 0x34, 0x12, 0x26, 0xc7, 0x06, 0x24, 0x00, 0x78, 0x56,
            0xc3,
        ]),
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

#[test]
fn a_dos_program_reads_the_bios_and_video_memory_as_volatile() {
    let path = common::scratch_file("dos-pcmemory", "exe");
    std::fs::write(&path, dos_image(&module())).unwrap();
    let exe = path.to_str().unwrap();

    let tick = stdout(&["decompile", exe, "sub_10010"]);
    assert!(
        tick.contains("v1 = dat_46c;\n  do {\n  } while (v1 == dat_46c);"),
        "the wait for the next timer tick: {tick}"
    );
    let vga = stdout(&["decompile", exe, "sub_10030"]);
    assert!(
        vga.contains("v1 = dat_a0000;\n  dat_a0000 = 1;\n  dat_a0000 = 2;"),
        "the latch load and both writes: {vga}"
    );
    let hook = stdout(&["decompile", exe, "sub_10050"]);
    assert!(hook.contains("dat_24 = 0x1234;\n  dat_24 = 0x5678;"), "both writes to the vector: {hook}");
    std::fs::remove_file(&path).unwrap();
}
