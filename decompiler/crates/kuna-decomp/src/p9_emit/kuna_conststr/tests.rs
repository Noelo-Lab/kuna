//! Unit tests for the (kuna) `conststr` literals.

use super::*;

#[test]
fn the_string_parameter_table_is_sorted_for_its_binary_search() {
    assert!(STRING_PARAMS.windows(2).all(|w| w[0].0 < w[1].0), "STRING_PARAMS is not sorted by name");
}

/// A parameter that reads its argument only as a string, and one that keeps a
/// pointer into it (`strchr`'s subject, `strtol`'s), or a user function.
#[test]
fn only_a_parameter_read_as_a_string_is_listed() {
    assert!(reads_only_as_string("setlocale", 1));
    assert!(!reads_only_as_string("setlocale", 0), "the category");
    assert!(reads_only_as_string("strcmp", 0) && reads_only_as_string("strcmp", 1));
    assert!(reads_only_as_string("strpbrk", 1), "the set");
    assert!(!reads_only_as_string("strpbrk", 0), "the result points into the subject");
    assert!(reads_only_as_string("__printf_chk", 1), "the format after the flag");
    assert!(reads_only_as_string("nanf", 0));
    assert!(!reads_only_as_string("strchr", 0));
    assert!(!reads_only_as_string("strtol", 0), "the end pointer points into it");
    assert!(!reads_only_as_string("memcpy", 1), "read by length");
    assert!(!reads_only_as_string("count", 1), "a user function");
}

#[test]
fn a_byte_string_prints_every_byte_it_holds() {
    assert_eq!(byte_literal(b""), None);
    assert_eq!(byte_literal(b"\xa1\x07e").as_deref(), Some("\"\\xa1\\ae\""));
    assert_eq!(byte_literal(b"\xa1\xaf").as_deref(), Some("\"\\xa1\\xaf\""));
    assert_eq!(byte_literal(b"\x01\x02\xff").as_deref(), Some("\"\\x01\\x02\\xff\""));
    assert_eq!(byte_literal(b"\xe2\x80\x98a\xe2\x80\x99").as_deref(), Some("\"‘a’\""));
}

/// C's `\x` escape takes every hex digit that follows it, so a literal whose
/// next character is one is split there.
#[test]
fn a_hex_escape_never_swallows_the_next_character() {
    assert_eq!(byte_literal(b"\xa1e").as_deref(), Some("\"\\xa1\" \"e\""));
    assert_eq!(byte_literal(b"\xff\x1bA").as_deref(), Some("\"\\xff\\x1b\" \"A\""));
    assert_eq!(byte_literal(b"\xffz").as_deref(), Some("\"\\xffz\""));
}

/// A valid two-byte character upstream would escape by codepoint (`\x80` for
/// `c2 80`) is spelled byte by byte, so the literal holds the image's bytes.
#[test]
fn an_escaped_multibyte_character_keeps_its_bytes() {
    assert_eq!(byte_literal(b"\xc2\x80\xff").as_deref(), Some("\"\\xc2\\x80\\xff\""));
}

/// An overlong form (`c1 81` and `e0 81 81` both decode to `A`) or a surrogate
/// (`ed a0 80`, U+D800) is not the encoding of the character it decodes to, so
/// each byte is escaped and the literal keeps all ten bytes.
#[test]
fn an_overlong_or_surrogate_sequence_keeps_its_bytes() {
    assert_eq!(byte_literal(b"\xc1\x81").as_deref(), Some("\"\\xc1\\x81\""));
    assert_eq!(byte_literal(b"\xe0\x81\x81").as_deref(), Some("\"\\xe0\\x81\\x81\""));
    assert_eq!(byte_literal(b"\xed\xa0\x80").as_deref(), Some("\"\\xed\\xa0\\x80\""));
    assert_eq!(
        byte_literal(b"\xa1\xc1\x81\xe0\x81\x81\xed\xa0\x80z").as_deref(),
        Some("\"\\xa1\\xc1\\x81\\xe0\\x81\\x81\\xed\\xa0\\x80z\"")
    );
}

#[test]
fn a_relocated_slot_anywhere_in_the_bytes_is_found() {
    let slots = [(0x100u64, 0x107u64), (0x200u64, 0x207u64)];
    assert!(overlaps(&slots, 0x0f0, 0x100));
    assert!(overlaps(&slots, 0x107, 0x110));
    assert!(overlaps(&slots, 0x1f0, 0x300));
    assert!(!overlaps(&slots, 0x108, 0x1ff));
    assert!(!overlaps(&slots, 0x208, 0x300));
    assert!(!overlaps(&[], 0, u64::MAX));
}
