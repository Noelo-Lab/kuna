//! Native selector-map switches must keep the selector and case labels in one domain.
use std::path::PathBuf;
use std::process::Command;

fn fixture(map: &[u8], targets: &[u32]) -> Vec<u8> {
    // Original MSABI machine code: bounded mode-5, byte lookup, relative target
    // lookup, and a stored-mode backedge. No compiler or external fixture needed.
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
    let mut image = vec![0; 0x1000];
    image[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
    for (at, value) in [(16, 2u16), (18, 62), (52, 64), (54, 56), (56, 1)] {
        image[at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    for (at, value) in [(20, 1u32), (64, 1), (68, 5)] {
        image[at..at + 4].copy_from_slice(&value.to_le_bytes());
    }
    let size = (table_at + targets.len() * 4) as u64;
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
    image.extend(code);
    image.extend(map);
    image.resize(table_at, 0);
    for target in targets {
        image.extend(target.to_le_bytes());
    }
    image
}

fn run(map: &[u8], targets: &[u32], guarded: bool) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let specs = std::env::var_os("SLEIGHHOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    let dir = std::env::temp_dir().join(format!(
        "kuna-compact-switch-{}-{}",
        std::process::id(),
        map.len()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let binary = dir.join("fixture.elf");
    std::fs::write(&binary, fixture(map, targets)).unwrap();
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let mut command = Command::new(kuna);
    command.args(["decompile-all"]).arg(binary);
    if !guarded {
        command.args(["--option", "switchmultipred", "off"]);
    }
    let out=command
        .args(["--addr","0x401000","--mode","reliable","--assert-strict","--sleighpath"])
        .arg(specs)
        .args(["--assert","typedef struct ModeStore { unsigned int mode; };",
               "--assert","function 0x401000-0x401055=compact_dispatch",
               "--assert","prototype compact_dispatch int MSABI compact_dispatch(struct ModeStore *store, unsigned int mode)"])
        .output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

#[test]
fn permuted_byte_map_keeps_original_index_labels() {
    // index2 -> map1 -> return240; index3 -> map2 -> stored-mode retry.
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
