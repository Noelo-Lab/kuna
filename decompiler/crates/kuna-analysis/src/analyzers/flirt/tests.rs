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
fn names_compare_without_decorations() {
    assert!(same_name("_strlen", "strlen"));
    assert!(same_name("j___imp__CreateFileW@28", "CreateFileW"));
    assert!(!same_name("strlen", "strnlen"));
}
