//! Native switches must keep the selector and its case labels in one domain.
use std::path::PathBuf;
use std::process::Command;

/// An x86-64 executable whose single segment maps `body` at 0x401000.
fn elf(body: &[u8]) -> Vec<u8> {
    let mut image = vec![0; 0x1000];
    image[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let size = (0x1000 + body.len()) as u64;
    for (at, value) in [
        (24, 0x401000u64),
        (32, 64),
        (80, 0x400000),
        (88, 0x400000),
        (96, size),
        (104, size),
        (112, 0x1000),
    ] {
        image[at..at + 8].copy_from_slice(&value.to_le_bytes());
    }
    image.extend(body);
    image
}

/// MSABI dispatch: bounded `mode - 5`, a byte map, a relative target table,
/// and a stored-mode retry backedge.
fn compact_fixture(map: &[u8], targets: &[u32]) -> Vec<u8> {
    let mut code = vec![
        0x83, 0xc2, 0xfb, 0x83, 0xfa, 0x03, 0x0f, 0x87, 0x43, 0, 0, 0, 0x4c, 0x8d, 0x05, 0xed,
        0xef, 0xff, 0xff, 0x48, 0x63, 0xc2, 0x41, 0x0f, 0xb6, 0x84, 0, 0x55, 0x10, 0, 0, 0x41,
        0x8b, 0x94, 0x80, 0x5c, 0x10, 0, 0, 0x4c, 0x01, 0xc2, 0xff, 0xe2, 0xb8, 0x2c, 0x01, 0, 0,
        0xc3, 0xb8, 0xf0, 0, 0, 0, 0xc3, 0x8b, 0x11, 0x83, 0xfa, 0x08, 0x0f, 0x84, 0x0c, 0, 0, 0,
        0x83, 0xc2, 0xfb, 0x83, 0xfa, 0x03, 0x0f, 0x86, 0xc4, 0xff, 0xff, 0xff, 0xb8, 0x0e, 0x01,
        0, 0, 0xc3,
    ];
    assert_eq!(code.len(), 85);
    code[5] = (map.len() - 1) as u8;
    code[72] = (map.len() - 1) as u8;
    let table_at = (0x1000 + code.len() + map.len() + 3) & !3;
    code[35..39].copy_from_slice(&(table_at as u32).to_le_bytes());
    code.extend(map);
    code.resize(table_at - 0x1000, 0);
    for target in targets {
        code.extend(target.to_le_bytes());
    }
    elf(&code)
}

/// SysV `pick(p, a, b)`: `k = *p - 'A'` is bounded to 0..3 three branches
/// before a relative table dispatch on `k`, out of reach of the guard search,
/// so the selector range is the full byte and the table ends at its first
/// far entry.
fn far_guard_fixture() -> Vec<u8> {
    let mut code = vec![
        0x0f, 0xb6, 0x07, 0x83, 0xe8, 0x41, 0x3c, 0x03, 0x77, 0x38, 0x85, 0xf6, 0x74, 0x3a, 0x85,
        0xd2, 0x74, 0x3c, 0x83, 0xfe, 0x05, 0x74, 0x3d, 0x0f, 0xb6, 0xc0, 0x48, 0x8d, 0x0d, 0x3b,
        0, 0, 0, 0x48, 0x63, 0x04, 0x81, 0x48, 0x01, 0xc8, 0xff, 0xe0,
    ];
    for value in [100u32, 101, 102, 103, 0xffff_ffff, 7, 8, 9] {
        code.push(0xb8);
        code.extend(value.to_le_bytes());
        code.push(0xc3);
    }
    assert_eq!(code.len(), 0x5a);
    code.resize(0x5c, 0);
    for entry in [-0x32i32, -0x2c, -0x26, -0x20] {
        code.extend(entry.to_le_bytes());
    }
    for _ in 4..256 {
        code.extend(0x4000_0000i32.to_le_bytes());
    }
    elf(&code)
}

fn decompile(image: &[u8], asserts: &[&str], options: &[&str]) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let scratch = tempfile::tempdir().unwrap();
    let binary = scratch.path().join("fixture.elf");
    std::fs::write(&binary, image).unwrap();
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(kuna);
    command.args(["decompile-all"]).arg(&binary);
    for option in options {
        command.args(["--option", option, "off"]);
    }
    command
        .args(["--addr", "0x401000", "--mode", "reliable", "--assert-strict", "--sleighpath"])
        .arg(specs);
    for assertion in asserts {
        command.args(["--assert", assertion]);
    }
    let out = command.output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn run(map: &[u8], targets: &[u32], guarded: bool) -> String {
    let options: &[&str] = if guarded { &[] } else { &["switchmultipred"] };
    decompile(
        &compact_fixture(map, targets),
        &[
            "typedef struct ModeStore { unsigned int mode; };",
            "function 0x401000-0x401055=compact_dispatch",
            "prototype compact_dispatch int MSABI compact_dispatch(struct ModeStore *store, unsigned int mode)",
        ],
        options,
    )
}

/// Index 2 reads map byte 1 (return 240); index 3 reads map byte 2 (retry).
#[test]
fn permuted_byte_map_keeps_original_index_labels() {
    let c = run(&[0, 3, 1, 2], &[0x102c, 0x1032, 0x1038, 0x104f], true);
    assert!(
        c.contains("switch(mode)"),
        "selector is in a different domain: {c}"
    );
    assert!(
        c.contains("case 2:\n        return 0xf0;"),
        "lost index2/240: {c}"
    );
    assert!(
        !c.contains("switch(*(char *)"),
        "kept mapped byte with raw-index labels: {c}"
    );
}

#[test]
fn compressed_map_with_duplicate_default_targets_keeps_all_index_labels() {
    let c = run(
        &[
            0, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 4, 4, 0, 4, 4, 4, 2, 2, 4, 0, 4, 4, 4, 4, 4, 4, 3,
            4, 4, 4, 4, 4, 4, 4, 0,
        ],
        &[0x102c, 0x1032, 0x1038, 0x104f, 0x104f],
        true,
    );
    assert!(
        c.contains("switch(mode)"),
        "selector is in a different domain: {c}"
    );
    assert!(c.contains("case 0xb:"), "lost index11/240: {c}");
    assert!(
        c.contains("case 0x12:") && c.contains("case 0x13:"),
        "lost retry cases: {c}"
    );
    assert!(
        c.contains("case 0xe:") && c.contains("case 0x24:"),
        "lost duplicate flight cases: {c}"
    );
}

#[test]
fn missing_guard_recovery_retains_raw_target_labels() {
    let c = run(&[0, 3, 1, 2], &[0x102c, 0x1032, 0x1038, 0x104f], false);
    assert!(
        c.contains("case 0x401032:"),
        "lost raw return240 target: {c}"
    );
    assert!(c.contains("case 0x40104f:"), "lost raw default target: {c}");
    assert!(
        c.contains("if (4 <= mode)"),
        "folded an unverified range guard: {c}"
    );
    assert!(
        !c.contains("case 2:"),
        "reused original-index labels on a raw target: {c}"
    );
}

#[test]
fn wider_late_selector_range_keeps_index_labels() {
    let c = decompile(
        &far_guard_fixture(),
        &[
            "function 0x401000-0x40105a=pick",
            "prototype pick int pick(unsigned char *p, int a, int b)",
        ],
        &[],
    );
    assert!(
        c.contains("case 0:\n        return 100;") && c.contains("case 3:\n        return 0x67;"),
        "lost index labels: {c}"
    );
    assert!(!c.contains("case 0x40"), "labelled cases by address: {c}");
}
