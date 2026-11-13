use super::*;

/// A version-6 signature with one module: prefix `55 48 89 e5`, CRC over the
/// four bytes at offset 32, length 40, public `myfunc`.
const TINY: &str = "49444153474e0600000000000000000000000000000000000000000000000000000007000001000000746573746c6962010400554889e50004bc1628006d7966756e6300";

fn tiny() -> Signature {
    let bytes: Vec<u8> = (0..TINY.len()).step_by(2).map(|i| u8::from_str_radix(&TINY[i..i + 2], 16).unwrap()).collect();
    Signature::parse(&bytes).expect("tiny sig parses")
}

fn code() -> Vec<u8> {
    let mut c = vec![0x55, 0x48, 0x89, 0xe5];
    c.extend([0u8; 28]);
    c.extend([0x90u8; 8]);
    c
}

#[test]
fn parses_the_header_and_the_one_module() {
    let s = tiny();
    assert_eq!(s.header.library_name, "testlib");
    assert_eq!(s.header.pattern_len, 32);
    assert_eq!(s.modules.len(), 1);
    assert_eq!(s.modules[0].length, 40);
    assert_eq!(s.modules[0].publics[0].name, b"myfunc");
}

#[test]
fn startup_and_foreign_file_type_signatures_do_not_apply() {
    let mut h = vec![0u8; 41];
    h[..6].copy_from_slice(b"IDASGN");
    assert!(applies_to(&h, Some(18)), "an unset file-type mask applies to every file");
    h[8..12].copy_from_slice(&(1u32 << 11).to_le_bytes());
    assert!(applies_to(&h, Some(11)) && !applies_to(&h, Some(18)), "a PE-only library is not tried on an ELF");
    h[16] = 1;
    assert!(!applies_to(&h, Some(11)), "a startup signature names no function");
}

#[test]
fn trie_crc_and_length_decide_the_match() {
    let s = tiny();
    let matcher = Matcher::new(&s, Endian::Little);
    let good = code();
    let m = matcher.match_at(&good);
    assert_eq!(m.len(), 1);
    assert_eq!(function_name(m[0], 32, false).as_deref(), Some("myfunc"));
    let mut bad_crc = good.clone();
    bad_crc[33] = 0xcc;
    assert!(matcher.match_at(&bad_crc).is_empty());
    let mut bad_prefix = good.clone();
    bad_prefix[1] = 0x00;
    assert!(matcher.match_at(&bad_prefix).is_empty());
    assert!(matcher.match_at(&good[..36]).is_empty(), "shorter than the module");
}

#[test]
fn a_return_behind_a_landing_pad_is_a_bare_return() {
    use crate::listing::{FlowKind, FlowType, Insn};
    let insn = |addr: u64, len: u32, kind: FlowKind| Insn {
        addr,
        len,
        fall_through: (kind == FlowKind::Fallthrough).then_some(addr + u64::from(len)),
        flow: FlowType {
            kind,
            is_terminal: kind == FlowKind::Return,
            has_fallthrough: kind == FlowKind::Fallthrough,
            ..Default::default()
        },
        flows: Vec::new(),
        mnemonic: String::new().into(),
        operands: String::new(),
        pcode: None,
    };
    let bytes = [0xf3, 0x0f, 0x1e, 0xfa, 0xc3, 0xc3, 0xf3, 0x0f, 0x1e, 0xfa, 0x31, 0xc0, 0xc3];
    let listing = Listing::from_insns_for_test(
        vec![
            insn(0x1000, 4, FlowKind::Fallthrough),
            insn(0x1004, 1, FlowKind::Return),
            insn(0x1005, 1, FlowKind::Return),
            insn(0x1006, 4, FlowKind::Fallthrough),
            insn(0x100a, 2, FlowKind::Fallthrough),
            insn(0x100c, 1, FlowKind::Return),
        ],
        false,
    );
    let image = Image { spans: vec![(0x1000, &bytes)], x86_fields: true, pads: X86_PADS };
    assert!(bare_return(&listing, &image, 0x1000), "endbr64; ret");
    assert!(bare_return(&listing, &image, 0x1005), "ret");
    assert!(!bare_return(&listing, &image, 0x1006), "endbr64; xor eax, eax; ret");
    let other = Image { spans: vec![(0x1000, &bytes)], x86_fields: false, pads: &[] };
    assert!(!bare_return(&listing, &other, 0x1000), "no landing pads off x86 and AArch64");
}

#[test]
fn names_compare_without_decorations() {
    assert!(same_name("_strlen", "strlen"));
    assert!(same_name("j___imp__CreateFileW@28", "CreateFileW"));
    assert!(!same_name("strlen", "strnlen"));
}
