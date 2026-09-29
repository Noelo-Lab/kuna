//! Unit tests for the (kuna) `conststr` literals.

use super::*;

/// The linker's tail merge: `""` is the NUL that ends `"tab\there\n"`.
#[test]
fn the_nul_ending_a_string_is_a_real_empty_string() {
    let mut before = vec![0x02, 0xff, 0x00];
    before.extend_from_slice(b"tab\there\n");
    assert!(terminates_string(&before));
    let long = vec![b'x'; TAIL_WINDOW];
    assert!(terminates_string(&long), "a string longer than the window");
}

/// A zero byte after binary, a lone character, or a window that could not be
/// read back to the string's start is not a string's terminator.
#[test]
fn a_zero_after_binary_is_not_a_string_end() {
    assert!(!terminates_string(&[]));
    assert!(!terminates_string(&[0x00]));
    assert!(!terminates_string(&[0x00, b'A']), "one character");
    assert!(!terminates_string(&[0x00, 0x77, 0xdf, 0x01, 0x02]));
    assert!(!terminates_string(&[b'o', b'k']), "a short read that never reached a NUL");
    assert!(!terminates_string(&[0x00, b'h', 0x07, b'i']), "a control byte in the run");
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

/// kmod's command table: the eight bytes at the constant are the address of a
/// `.rodata` string (`ba b1 01 00 ...` = 0x1b1ba), so they are a pointer, not
/// the string `"\xba\xb1\x01"`; gnulib's GB18030 quotes are not.
#[test]
fn a_word_that_addresses_the_image_is_a_pointer() {
    let data = [(0x1a000u64, 0x1c3ffu64), (0x28000u64, 0x2a0ffu64)];
    let code = [(0x3000u64, 0x18fffu64)];
    assert!(is_image_pointer(&[0xba, 0xb1, 0x01, 0, 0, 0, 0, 0], false, &data, &code));
    assert!(is_image_pointer(&[0x70, 0x3e, 0, 0, 0, 0, 0, 0], false, &data, &code), "a code pointer");
    assert!(!is_image_pointer(&[0xa1, 0x07, 0x65, 0, 0xa1, 0xaf, 0, 0], false, &data, &code));
    assert!(!is_image_pointer(&[0; 8], false, &data, &code), "null");
    assert!(is_image_pointer(&[0, 0x01, 0xb1, 0xba], true, &data, &code), "big-endian");
}
