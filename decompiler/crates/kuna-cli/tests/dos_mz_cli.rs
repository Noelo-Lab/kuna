//! An MS-DOS `MZ` executable loads without `--raw-image`: its load module is
//! relocated at segment 0x1000 and its `CS:IP` seeds discovery.

use crate::common;

use std::process::{Command, Output};

/// Entry `0000:0000` far-calls `0002:0000`, whose segment word at module offset
/// 3 is the one relocation; `0002:0000` returns 9 with `RETF`.
fn dos_image(extra_header: &[u8]) -> Vec<u8> {
    let mut module = vec![0x90u8; 0x24];
    module[..10].copy_from_slice(&[0x9a, 0x00, 0x00, 0x02, 0x00, 0xb8, 0x00, 0x4c, 0xcd, 0x21]);
    module[0x20..0x24].copy_from_slice(&[0xb8, 0x09, 0x00, 0xcb]);
    let mut header = vec![0u8; 0x20];
    header[..2].copy_from_slice(b"MZ");
    header[0x1c..0x20].copy_from_slice(&[0x03, 0x00, 0x00, 0x00]);
    header.extend_from_slice(extra_header);
    header.resize(header.len().div_ceil(16) * 16, 0);
    let total = header.len() + module.len();
    let words: [(usize, u16); 6] = [
        (0x02, (total % 512) as u16),
        (0x04, total.div_ceil(512) as u16),
        (0x06, 1),
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

/// A load module of 0x10004 bytes whose entry `0100:0000` (linear 0x11000
/// once loaded at 0x1000) calls -0x1003 and returns far: IP 3 wraps inside the
/// segment to `0100:F000`, linear 0x20000, past the 64 KiB line, where a
/// decoder that ignored the segment would land on 0x0 (a 16-bit mask) or
/// 0x10000 (the linear sum). `0100:F000` returns 7.
fn segmented_dos_image() -> Vec<u8> {
    let mut module = vec![0u8; 0x10004];
    module[0x1000..0x1004].copy_from_slice(&[0xe8, 0xfd, 0xef, 0xcb]);
    module[0x10000..0x10004].copy_from_slice(&[0xb8, 0x07, 0x00, 0xc3]);
    let mut header = vec![0u8; 0x20];
    header[..2].copy_from_slice(b"MZ");
    let total = header.len() + module.len();
    let words: [(usize, u16); 7] = [
        (0x02, (total % 512) as u16),
        (0x04, total.div_ceil(512) as u16),
        (0x08, (header.len() / 16) as u16),
        (0x10, 0x100),
        (0x14, 0x0000),
        (0x16, 0x0100),
        (0x18, 0x1c),
    ];
    for (at, value) in words {
        header[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    header.extend_from_slice(&module);
    header
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna")).args(args).output().expect("run kuna")
}

fn written(stem: &str, bytes: &[u8]) -> std::path::PathBuf {
    let path = common::scratch_file(stem, "exe");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn a_dos_executable_loads_at_its_entry_with_relocated_far_calls() {
    let path = written("dos-mz", &dos_image(&[]));
    let exe = path.to_str().unwrap();

    let functions = kuna(&["functions", exe, "--json"]);
    let listing = String::from_utf8_lossy(&functions.stdout);
    assert!(functions.status.success(), "{listing}\n{}", String::from_utf8_lossy(&functions.stderr));
    for name in ["\"sub_10000\"", "\"sub_10020\""] {
        assert!(listing.contains(name), "missing {name}: {listing}");
    }

    for args in [vec!["decompile", exe, "sub_10020"], vec!["decompile-all", exe, "--addr", "0x10020"]] {
        let out = kuna(&args);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success(), "{args:?}: {text}\n{}", String::from_utf8_lossy(&out.stderr));
        assert!(text.contains("return 9;"), "{args:?}: {text}");
    }

    let strings = kuna(&["strings", exe]);
    assert!(!strings.status.success());
    assert!(String::from_utf8_lossy(&strings.stderr).contains("MS-DOS MZ input has no object-file view"));
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_pklite_stub_is_named_on_load() {
    let path = written("dos-mz-pklite", &dos_image(b"PKLITE Copr. 1990"));
    let out = kuna(&["functions", path.to_str().unwrap()]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("PKLITE-compressed") && stderr.contains("run `kuna unpack`"), "{stderr}");
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_near_call_wraps_inside_the_entry_code_segment() {
    let path = written("dos-mz-segment", &segmented_dos_image());
    let exe = path.to_str().unwrap();

    let functions = kuna(&["functions", exe, "--json"]);
    let listing = String::from_utf8_lossy(&functions.stdout);
    assert!(functions.status.success(), "{listing}\n{}", String::from_utf8_lossy(&functions.stderr));
    assert!(listing.contains("\"sub_20000\""), "the wrapped call target is missing: {listing}");
    assert!(!listing.contains("\"sub_10000\""), "the linear sum is not the target: {listing}");

    let out = kuna(&["decompile-all", exe, "--addr", "0x11000"]);
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{text}\n{}", String::from_utf8_lossy(&out.stderr));
    assert!(text.contains("sub_20000();"), "{text}");
    assert!(text.contains(" = 3;"), "the CALL pushes IP 3, not the linear low half 0x1003: {text}");
    std::fs::remove_file(&path).unwrap();
}
