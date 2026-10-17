use crate::common;

fn targets(bytes: &[u8]) -> Vec<u64> {
    let path = common::scratch_file("thumbswitch-guard", "elf");
    std::fs::write(&path, bytes).unwrap();
    let (stdout, stderr, code) = common::run_kuna(&[
        "xrefs", path.to_str().unwrap(), "--from", "0x822a", "--json",
    ]);
    std::fs::remove_file(path).unwrap();
    assert_eq!(code, 0, "{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    doc["xrefs"].as_array().unwrap().iter().map(|r| r["to_address"].as_u64().unwrap()).collect()
}

#[test]
fn only_an_unsigned_upper_bound_may_size_a_thumb_switch() {
    let mut bytes = std::fs::read(common::fixture("thumbswitch_le32")).unwrap();
    assert_eq!(&bytes[0x228..0x22a], &[0x0e, 0xd8]);
    for condition in 0..16u8 {
        bytes[0x229] = 0xd0 | condition;
        let expected: &[u64] = match condition {
            2 => &[0x8232, 0x8238, 0x8240],
            8 => &[0x8232, 0x8238, 0x8240, 0x8244],
            _ => &[],
        };
        assert_eq!(targets(&bytes), expected, "Thumb condition {condition:x}");
    }
}

#[test]
fn a_guard_that_branches_to_the_dispatch_does_not_bound_it() {
    let mut bytes = std::fs::read(common::fixture("thumbswitch_le32")).unwrap();
    bytes[0x228] = 0xff;
    assert!(targets(&bytes).is_empty());
}
