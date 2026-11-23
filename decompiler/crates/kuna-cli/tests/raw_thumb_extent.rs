//! Raw ARM inventory counts code reached by flow and the literal pools it
//! loads, not adjacent data; other raw languages keep the clip.

use crate::common;
use std::path::Path;

fn inventory(image: &Path, target: &str, base: &str, extra: &[&str]) -> Vec<(u64, u64)> {
    let specs = common::repo_root().join("specs");
    let mut args = vec![
        "functions", image.to_str().unwrap(), "--json", "--raw-image", "--target", target,
        "--base", base, "--sleighpath", specs.to_str().unwrap(),
    ];
    args.extend_from_slice(extra);
    let (stdout, stderr, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).expect("functions --json");
    doc["functions"]
        .as_array()
        .expect("function inventory")
        .iter()
        .map(|row| (row["address"].as_u64().unwrap(), row["size"].as_u64().unwrap()))
        .collect()
}

fn thumb_inventory(image: &Path, declarations: &[&str]) -> Vec<(u64, u64)> {
    let mut args = vec![
        "--isa", "thumb", "--entry", "0x6101", "--entry", "0x6141", "--entry", "0x6181",
    ];
    args.extend_from_slice(declarations);
    inventory(image, "ARM:LE:32:Cortex:default", "0x6000", &args)
}

#[test]
fn raw_thumb_sizes_require_complete_decoded_flow() {
    let mut bytes = [0xa5; 0x200];
    let code: &[(usize, &[u16])] = &[
        // movs; adds; lsls; adds; bx lr; then data up to the next entry.
        (0x000, &[0x2005, 0x3001, 0x0040, 0x3002, 0x4770]),
        // push {lr}; cmp; beq 0x6114; bl 0x61c0; movs; pop {pc}.
        (0x100, &[0xb500, 0x2800, 0xd006, 0xf000, 0xf85b, 0x2003, 0xbd00]),
        // The taken arm leaves its clip at 0x6190 then jumps back to 0x611c.
        (0x114, &[0x2009, 0xe03b]),
        (0x11c, &[0xbd00]),
        // it eq; bxeq r3; bx lr. The known skip does not resolve the jump.
        (0x140, &[0xbf08, 0x4718, 0x4770]),
        // b 0x6200, exactly one byte address past the mapped bytes.
        (0x180, &[0xe03e]),
        (0x190, &[0xe7c4]),
        // Discovered by the BL. Its unresolved jump cannot affect its caller.
        (0x1c0, &[0x4718]),
    ];
    for &(start, halfwords) in code {
        for (i, halfword) in halfwords.iter().enumerate() {
            let at = start + i * 2;
            bytes[at..at + 2].copy_from_slice(&halfword.to_le_bytes());
        }
    }
    let image = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(image.path(), bytes).unwrap();

    let natural_flag = ["--define-function", "0x6001=short_leaf"];
    let natural_alias = ["--assert", "function 0x6001=short_leaf"];
    let bounded_flag = ["--define-function", "0x6000-0x6020=short_leaf"];
    let bounded_alias = ["--assert", "function 0x6000-0x6020=short_leaf"];
    let mut expected = [(0x6000, 10), (0x6100, 0x1e), (0x6140, 0x40), (0x6180, 0x40), (0x61c0, 0x40)];
    assert_eq!(thumb_inventory(image.path(), &natural_flag), expected);
    assert_eq!(thumb_inventory(image.path(), &natural_alias), expected);
    // A later entry-only assertion still clears the earlier flag's bound,
    // including when the two start values differ only by the Thumb state bit.
    assert_eq!(thumb_inventory(image.path(), &[bounded_flag, natural_alias].concat()), expected);
    expected[0].1 = 0x20;
    assert_eq!(thumb_inventory(image.path(), &bounded_flag), expected);
    assert_eq!(thumb_inventory(image.path(), &bounded_alias), expected);
    assert_eq!(thumb_inventory(image.path(), &[natural_flag, bounded_alias].concat()), expected);

    // Previewing a declaration must not publish its symbol before an earlier
    // assertion. A callee the ordinary sweep found is still visible.
    let specs = common::repo_root().join("specs");
    let (stdout, stderr, status) = common::run_kuna(&[
        "decompile", image.path().to_str().unwrap(), "0x6101", "--addr", "--json",
        "--raw-image", "--target", "ARM:LE:32:Cortex:default", "--isa", "thumb",
        "--base", "0x6000", "--sleighpath", specs.to_str().unwrap(),
        "--assert", "prototype 0x6000 int leaf(void)",
        "--assert", "prototype 0x61c1 void callee(void)",
        "--assert", "function 0x6001=short_leaf",
        "--assert", "function 0x61c0=known_callee",
    ]);
    assert_eq!(status, 0, "{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let outcomes = doc["assertions"].as_array().unwrap();
    assert_eq!(outcomes[0]["status"], "rejected");
    for outcome in &outcomes[1..] {
        assert_eq!(outcome["status"], "applied", "{outcome}");
    }
}

#[test]
fn raw_thumb_sizes_keep_the_literal_pools_their_code_loads() {
    let mut bytes = [0xa5; 0x100];
    let code: &[(usize, &[u16])] = &[
        // ldr r0,[pc,#4]; bx lr; two data halfwords; the word it loads.
        (0x00, &[0x4801, 0x4770]),
        (0x08, &[0x5678, 0x1234]),
        // vldr s0,[pc,#8]; bx lr; the single it loads at 0x604c.
        (0x40, &[0xed9f, 0x0a02, 0x4770]),
        (0x4c, &[0x0000, 0x3f80]),
        // adr r3,0x6090; ldrd r2,r3,[r3]; bx lr; the doubleword it addresses.
        (0x80, &[0xa303, 0xe9d3, 0x2300, 0x4770]),
        (0x90, &[0x0000, 0x0000, 0x0000, 0x3ff0]),
        // movs r0,#1; bx lr; then data nothing reads.
        (0xc0, &[0x2001, 0x4770]),
    ];
    for &(start, halfwords) in code {
        for (i, halfword) in halfwords.iter().enumerate() {
            let at = start + i * 2;
            bytes[at..at + 2].copy_from_slice(&halfword.to_le_bytes());
        }
    }
    let image = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(image.path(), bytes).unwrap();
    let rows = inventory(
        image.path(),
        "ARM:LE:32:Cortex:default",
        "0x6000",
        &["--isa", "thumb", "--entry", "0x6001", "--entry", "0x6041", "--entry", "0x6081", "--entry", "0x60c1"],
    );
    assert_eq!(rows, [(0x6000, 0xc), (0x6040, 0x10), (0x6080, 0x18), (0x60c0, 4)]);
}

#[test]
fn raw_thumb_inventory_keeps_unmeasured_shared_tail_clips() {
    const ENTRIES: usize = 12;
    const CLIP: usize = 8;
    const BASE: u64 = 0x7000;
    let mut bytes = [0xcc; ENTRIES * CLIP];
    let entries: Vec<_> = (0..ENTRIES)
        .map(|i| format!("{:#x}", BASE + (i * CLIP) as u64 + 1))
        .collect();
    let mut args = vec!["--isa", "thumb"];
    for (i, entry) in entries.iter().enumerate() {
        let offset = i * CLIP;
        let halfword: u16 = if i + 1 == ENTRIES { 0x4770 } else { 0xe002 }; // bx lr; b next entry
        bytes[offset..offset + 2].copy_from_slice(&halfword.to_le_bytes());
        args.extend(["--entry", entry.as_str()]);
    }
    let image = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(image.path(), bytes).unwrap();
    let rows = inventory(image.path(), "ARM:LE:32:Cortex:default", "0x7000", &args);
    // Each veneer follows the shared suffix; the first four walks consume
    // 12 + 11 + 10 + 9 visits. The fifth cannot finish in the remaining six
    // of the 4 * 12 aggregate visits, so it and subsequent entries keep the clip.
    let expected: Vec<_> = (0..ENTRIES)
        .map(|i| (BASE + (i * CLIP) as u64, if i < 4 { 2 } else { CLIP as u64 }))
        .collect();
    assert_eq!(rows, expected);
}

#[test]
fn raw_x86_inventory_keeps_its_clips() {
    const ENTRIES: usize = 4;
    const CLIP: usize = 8;
    const BASE: u64 = 0x7000;
    let mut bytes = [0xcc; ENTRIES * CLIP];
    let entries: Vec<_> = (0..ENTRIES)
        .map(|i| format!("{:#x}", BASE + (i * CLIP) as u64))
        .collect();
    let mut args = Vec::new();
    for (i, entry) in entries.iter().enumerate() {
        let offset = i * CLIP;
        if i + 1 == ENTRIES {
            bytes[offset] = 0xc3; // ret
        } else {
            bytes[offset..offset + 2].copy_from_slice(&[0xeb, 6]); // jmp next entry
        }
        args.extend(["--entry", entry.as_str()]);
    }
    let image = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(image.path(), bytes).unwrap();
    let rows = inventory(image.path(), "x86:LE:32:default", "0x7000", &args);
    let expected: Vec<_> = (0..ENTRIES).map(|i| (BASE + (i * CLIP) as u64, CLIP as u64)).collect();
    assert_eq!(rows, expected);
}

#[test]
fn raw_arm_mode_inventory_skips_off_word_call_targets() {
    // bx lr; then data whose words spell `blx 0x800e` and `blx 0x803a`.
    let words: [u32; 16] = [
        0xe12fff1e, 0xfb000000, 0xe3a00001, 0xe12fff1e, 0xe1a00000, 0xe1a00000, 0xe1a00000, 0xe1a00000,
        0xfb000004, 0, 0, 0, 0, 0, 0, 0,
    ];
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    let image = tempfile::Builder::new().suffix(".bin").tempfile().unwrap();
    std::fs::write(image.path(), bytes).unwrap();
    let rows = inventory(image.path(), "ARM:LE:32:v7:default", "0x8000", &["--isa", "arm", "--entry", "0x8000"]);
    assert_eq!(rows, [(0x8000, 4)]);
}
