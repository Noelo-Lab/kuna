//! Authored SPARC32 ELF; no assembler or cross compiler is needed.
use crate::common;
use serde_json::Value;

fn fixture(prefix: &[u32], split: bool) -> std::path::PathBuf {
    let mut text = vec![0u8; 0x420];
    let mut put = |address: usize, words: &[u32]| {
        for (i, word) in words.iter().enumerate() {
            text[address - 0x1000 + i * 4..address - 0x1000 + i * 4 + 4]
                .copy_from_slice(&word.to_be_bytes());
        }
    };
    let mut caller = vec![0x9de3_bfa0];
    for i in 0..20u32 {
        let entry = 0x1100 + i * 0x20;
        let site = 0x1004 + i * 8;
        caller.extend([0x4000_0000 | ((entry - site) / 4), 0x0100_0000]);
        put(entry as usize, &[0x9de3_bfa0, 0xf027_bffc, 0xf007_bffc, 0x81c7_e008, 0x81e8_0000]);
    }
    caller.extend([0x81c7_e008, 0x81e8_0000]);
    put(0x1000, &caller);
    put(0x1400, prefix);
    put(0x1408, &[0x9de3_8001, 0xf027_bffc, 0xf007_bffc, 0x81c7_e008, 0x81e8_0000]);
    let names = b"\0.text\0.shstrtab\0.tail\0";
    let mut bytes = vec![0u8; 0x630];
    bytes[..16].copy_from_slice(b"\x7fELF\x01\x02\x01\0\0\0\0\0\0\0\0\0");
    for (offset, value) in [(16, 2u16), (18, 2), (40, 52), (42, 32), (44, 1),
        (46, 40), (48, if split { 4 } else { 3 }), (50, 2)] {
        bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
    }
    for (offset, value) in [(20, 1u32), (24, 0x1000), (28, 52), (32, 0x580),
        (52, 1), (56, 0x100), (60, 0x1000), (64, 0x1000), (68, 0x420),
        (72, 0x420), (76, 5), (80, 4),
        (0x5a8, 1), (0x5ac, 1), (0x5b0, 6), (0x5b4, 0x1000),
        (0x5b8, 0x100), (0x5bc, if split { 0x408 } else { 0x420 }), (0x5c8, 4),
        (0x5d0, 7), (0x5d4, 3), (0x5e0, 0x540), (0x5e4, names.len() as u32), (0x5f0, 1),
        (0x5f8, 17), (0x5fc, 1), (0x600, 6), (0x604, 0x1408),
        (0x608, 0x508), (0x60c, 0x18), (0x618, 4)] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
    bytes[0x100..0x520].copy_from_slice(&text);
    bytes[0x540..0x540 + names.len()].copy_from_slice(names);
    let path = common::scratch_file("sparc-prologue-start", "elf");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn inventory_and_decompile_own_the_constant_prefix() {
    let path = fixture(&[0x033f_fff7, 0x8210_6388], false);
    let name = path.to_str().unwrap();
    let (out, err, status) = common::run_kuna(&["functions", name, "--json"]);
    assert_eq!(status, 0, "{err}");
    let doc: Value = serde_json::from_str(&out).unwrap();
    let addresses: Vec<_> = doc["functions"].as_array().unwrap().iter()
        .map(|f| f["address"].as_u64().unwrap()).collect();
    assert!(addresses.contains(&0x1400), "{out}");
    assert!(!addresses.contains(&0x1408), "{out}");
    let unmatched = fixture(&[0x033f_fff7, 0x8410_6388], false);
    let (control, err, status) = common::run_kuna(&["functions", unmatched.to_str().unwrap(), "--json"]);
    assert_eq!(status, 0, "{err}");
    let control: Value = serde_json::from_str(&control).unwrap();
    let expected: Vec<_> = control["functions"].as_array().unwrap().iter()
        .map(|f| match f["address"].as_u64().unwrap() { 0x1408 => 0x1400, address => address })
        .collect();
    assert_eq!(addresses, expected, "only the accepted SAVE entry may move");
    std::fs::remove_file(unmatched).unwrap();
    let (out, err, status) = common::run_kuna(&["decompile", name, "--addr", "0x1400"]);
    assert_eq!(status, 0, "{err}");
    assert!(!out.contains("g1"), "constant must not be an undefined entry input: {out}");
    assert!(!out.contains("sub_1408"), "body must not truncate at SAVE: {out}");
    assert!(out.contains("return"), "{out}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mismatched_transferred_and_section_split_prefixes_keep_the_old_candidate() {
    for (prefix, split) in [
        ([0x033f_fff7, 0x8410_6388], false),
        ([0x033f_fff7, 0x8210_a388], false),
        ([0x073f_fff7, 0x8610_e388], false),
        ([0x033f_fff7, 0x4000_0002], false),
        ([0x033f_fff7, 0x8210_6388], true),
    ] {
        let path = fixture(&prefix, split);
        let (out, err, status) = common::run_kuna(&["functions", path.to_str().unwrap(), "--json"]);
        assert_eq!(status, 0, "{err}");
        let doc: Value = serde_json::from_str(&out).unwrap();
        let addresses: Vec<_> = doc["functions"].as_array().unwrap().iter()
            .map(|f| f["address"].as_u64().unwrap()).collect();
        assert!(!addresses.contains(&0x1400), "unmatched prefix gained ownership: {out}");
        assert!(addresses.contains(&0x1408), "existing SAVE candidate lost: {out}");
        std::fs::remove_file(path).unwrap();
    }
}
