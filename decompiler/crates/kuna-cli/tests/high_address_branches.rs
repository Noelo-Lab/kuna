//! Branch targets are unsigned addresses, including across bit 63 (GH #1505).
use crate::common;
use serde_json::Value;

fn macho(base: u64, instruction: u32) -> std::path::PathBuf {
    let mut bytes = vec![0; 0x4000];
    let mut cursor = 0;
    for value in [
        0xfeedfacfu32,
        0x0100000c,
        0x80000002,
        2,
        2,
        176,
        0x00200085,
        0,
        0x19,
        152,
    ] {
        bytes[cursor..cursor + 4].copy_from_slice(&value.to_le_bytes());
        cursor += 4;
    }
    bytes[40..46].copy_from_slice(b"__TEXT");
    cursor = 56;
    for value in [base, 0x4000, 0, 0x4000] {
        bytes[cursor..cursor + 8].copy_from_slice(&value.to_le_bytes());
        cursor += 8;
    }
    for value in [5u32, 5, 1, 0] {
        bytes[cursor..cursor + 4].copy_from_slice(&value.to_le_bytes());
        cursor += 4;
    }
    bytes[104..110].copy_from_slice(b"__text");
    bytes[120..126].copy_from_slice(b"__TEXT");
    bytes[136..144].copy_from_slice(&(base + 0x1000).to_le_bytes());
    bytes[144..152].copy_from_slice(&0x3000u64.to_le_bytes());
    bytes[152..156].copy_from_slice(&0x1000u32.to_le_bytes());
    bytes[156..160].copy_from_slice(&2u32.to_le_bytes());
    bytes[168..172].copy_from_slice(&0x80000400u32.to_le_bytes());
    cursor = 184;
    for value in [2u32, 24, 0x300, 2, 0x320, 17] {
        bytes[cursor..cursor + 4].copy_from_slice(&value.to_le_bytes());
        cursor += 4;
    }
    for (offset, name, address) in [(0x300, 1u32, base + 0x1000), (0x310, 9, base + 0x1384)] {
        bytes[offset..offset + 4].copy_from_slice(&name.to_le_bytes());
        bytes[offset + 4] = 15;
        bytes[offset + 5] = 1;
        bytes[offset + 8..offset + 16].copy_from_slice(&address.to_le_bytes());
    }
    bytes[0x320..0x331].copy_from_slice(b"\0_caller\0_callee\0");
    for chunk in bytes[0x1000..].chunks_exact_mut(4) {
        chunk.copy_from_slice(&0xd503201fu32.to_le_bytes());
    }
    bytes[0x1000..0x1004].copy_from_slice(&instruction.to_le_bytes());
    for offset in [0x1004, 0x1384] {
        bytes[offset..offset + 4].copy_from_slice(&0xd65f03c0u32.to_le_bytes());
    }
    let path = common::scratch_file("high-address-branch", "macho");
    std::fs::write(&path, bytes).unwrap();
    path
}

#[test]
fn arm64_calls_keep_unsigned_targets_in_text_json_and_xrefs() {
    for base in [
        0x100000000,
        0x7fff00000000,
        0x1000000000000000,
        0x7fffffffffffed00,
        0x7ffffffffffff000,
        0x8000000000000000,
        0xffff000000000000,
        0xfffffe0009000000,
    ] {
        let path = macho(base, 0x940000e1);
        let binary = path.to_str().unwrap();
        let specs = common::repo_root().join("specs");
        let specs = specs.to_str().unwrap();
        let site = format!("0x{:x}", base + 0x1000);
        let target = format!("0x{:x}", base + 0x1384);
        let (text, stderr, code) = common::run_kuna(&[
            "disassemble",
            binary,
            &site,
            "--addr",
            "--count",
            "1",
            "--sleighpath",
            specs,
        ]);
        assert_eq!(code, 0, "{stderr}");
        let row = text.lines().find(|line| !line.starts_with('#')).unwrap();
        let columns: Vec<_> = row.split_whitespace().collect();
        assert_eq!(
            columns,
            [site.as_str(), "e1000094", "bl", target.as_str()],
            "{text}"
        );
        let (json, stderr, code) = common::run_kuna(&[
            "disassemble",
            binary,
            &site,
            "--addr",
            "--count",
            "1",
            "--sleighpath",
            specs,
            "--json",
        ]);
        assert_eq!(code, 0, "{stderr}");
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(doc["instructions"][0]["operands"], target, "{json}");
        let (json, stderr, code) = common::run_kuna(&[
            "xrefs",
            binary,
            "--to",
            &target,
            "--json",
            "--sleighpath",
            specs,
        ]);
        assert_eq!(code, 0, "{stderr}");
        let doc: Value = serde_json::from_str(&json).unwrap();
        assert!(
            doc["xrefs"].as_array().unwrap().iter().any(|row| {
                row["kind"] == "call"
                    && row["from_address"] == base + 0x1000
                    && row["to_address"] == base + 0x1384
                    && row["instruction"] == format!("bl {target}")
            }),
            "{json}"
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn arm64_branch_forms_keep_high_targets_in_both_directions() {
    let base = 0xfffffe0009000000;
    let specs = common::repo_root().join("specs");
    for (forward, backward) in [
        (0x940000e1u32, 0x97ffff1f_u32),
        (0x140000e1, 0x17ffff1f),
        (0x54001c20, 0x54ffe3e0),
        (0x34001c20, 0x34ffe3e0),
        (0x36001c20, 0x3607e3e0),
    ] {
        let path = macho(base, forward);
        let mut bytes = std::fs::read(&path).unwrap();
        bytes[0x1384..0x1388].copy_from_slice(&backward.to_le_bytes());
        std::fs::write(&path, bytes).unwrap();
        for (site, target) in [(0x1000, 0x1384), (0x1384, 0x1000)] {
            let site = format!("0x{:x}", base + site);
            let target = format!("0x{:x}", base + target);
            let (json, stderr, code) = common::run_kuna(&[
                "disassemble",
                path.to_str().unwrap(),
                &site,
                "--addr",
                "--count",
                "1",
                "--json",
                "--sleighpath",
                specs.to_str().unwrap(),
            ]);
            assert_eq!(code, 0, "{stderr}");
            let doc: Value = serde_json::from_str(&json).unwrap();
            assert!(
                doc["instructions"][0]["operands"]
                    .as_str()
                    .unwrap()
                    .ends_with(&target),
                "{json}"
            );
        }
        std::fs::remove_file(path).unwrap();
    }
}
