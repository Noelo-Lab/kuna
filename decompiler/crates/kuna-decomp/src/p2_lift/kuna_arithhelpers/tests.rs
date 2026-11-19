//! Unit tests for the arithmetic-helper call-fixup payloads.

use super::*;

fn parsed(abi: HelperAbi) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    decode_payloads(abi, |root| {
        let name = String::from_utf8_lossy(root.get_attribute_value("name")?).into_owned();
        let mut targets = Vec::new();
        for child in root.get_children().iter() {
            if child.get_name() == "target" {
                targets
                    .push(String::from_utf8_lossy(child.get_attribute_value("name")?).into_owned());
            }
        }
        out.push((name, targets));
        Ok(())
    })
    .expect("payloads must parse");
    out
}

#[test]
fn every_payload_is_prefixed_and_every_target_is_unique() {
    for abi in [HelperAbi::X86, HelperAbi::Arm { big_endian: false }] {
        let fixups = parsed(abi);
        assert!(fixups.iter().all(|(n, _)| n.starts_with(FIXUP_PREFIX)));
        let mut targets: Vec<&String> = fixups.iter().flat_map(|(_, t)| t).collect();
        let total = targets.len();
        targets.sort();
        targets.dedup();
        assert_eq!(targets.len(), total, "{abi:?} maps a helper to two bodies");
    }
}

#[test]
fn x86_cdecl_bodies_pop_the_return_address_and_stdcall_bodies_their_arguments() {
    let xml = payload_xml(HelperAbi::X86).join("\n");
    let udiv = xml.lines().find(|l| l.contains("__udivdi3")).unwrap();
    assert!(udiv.contains("ESP = ESP + 4;") && !udiv.contains("ESP + 16;"));
    let alldiv = xml.lines().find(|l| l.contains("\"__alldiv\"")).unwrap();
    assert!(alldiv.contains("a s/ b") && alldiv.ends_with("ESP = ESP + 16;]]></body></pcode></callfixup>"));
}

#[test]
fn arm_pairs_follow_the_byte_order() {
    let le = payload_xml(HelperAbi::Arm { big_endian: false }).join("\n");
    let be = payload_xml(HelperAbi::Arm { big_endian: true }).join("\n");
    assert!(le.contains("a:8 = (zext(r1) << 32) | zext(r0);"));
    assert!(be.contains("a:8 = (zext(r0) << 32) | zext(r1);"));
}

#[test]
fn guard_admits_32_bit_x86_and_arm_only() {
    let all = |_: &[u8]| true;
    let arm = |n: &[u8]| ARM_REGISTERS.iter().any(|r| r.as_bytes() == n);
    assert_eq!(helper_abi(all, 4, false), Some(HelperAbi::X86));
    assert_eq!(helper_abi(all, 8, false), None);
    assert_eq!(helper_abi(arm, 4, true), Some(HelperAbi::Arm { big_endian: true }));
    assert_eq!(helper_abi(|_| false, 4, false), None);
}
