use std::collections::BTreeSet;

use super::literal_len;

fn unmapped(_: u64) -> bool {
    false
}

#[test]
fn short_literal_is_read_to_its_terminator() {
    let refs = BTreeSet::new();
    assert_eq!(literal_len(b"123\0Invalid Password\0", 0x402164, 4, unmapped, &refs), Some(4));
    assert_eq!(literal_len(b"cmd\0rb\0\0", 0x40218c, 4, unmapped, &refs), Some(4));
    assert_eq!(literal_len(b"a\tb\r\n\0", 0x1001, 4, unmapped, &refs), Some(6));
}

#[test]
fn empty_unterminated_and_binary_runs_are_not_literals() {
    let refs = BTreeSet::new();
    assert_eq!(literal_len(b"\0abc\0", 0x2000, 4, unmapped, &refs), None);
    assert_eq!(literal_len(b"abc", 0x2000, 4, unmapped, &refs), None);
    assert_eq!(literal_len(b"%\x07\0", 0x2000, 4, unmapped, &refs), None);
}

#[test]
fn pointer_slot_holding_a_mapped_address_is_not_a_literal() {
    let refs = BTreeSet::new();
    let mapped = |a: u64| (0x400000..0x500000).contains(&a);
    assert_eq!(literal_len(b"@AB\0", 0x403000, 4, mapped, &refs), None);
    assert_eq!(literal_len(b"@AB\0", 0x403001, 4, mapped, &refs), Some(4));
    assert_eq!(literal_len(b"123\0", 0x403000, 4, mapped, &refs), Some(4));
}

#[test]
fn character_code_array_needs_its_second_element_referenced() {
    let mut refs = BTreeSet::new();
    let wide = b"\\\0\0\0.\0\0\0ex\0\0";
    assert_eq!(literal_len(wide, 0x403294, 4, unmapped, &refs), None);
    refs.insert(0x403298);
    assert_eq!(literal_len(wide, 0x403294, 4, unmapped, &refs), Some(2));
    let utf16 = b"C\0:\0\0\0";
    assert_eq!(literal_len(utf16, 0x403300, 4, unmapped, &BTreeSet::new()), None);
    assert_eq!(literal_len(b"e\0\0\0sc\0\0", 0x4032a0, 4, unmapped, &BTreeSet::new()), Some(2));
}
