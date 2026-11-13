//! Unit tests for the (kuna) Swift string-object predicate.

use super::*;

#[test]
fn an_ascii_literal_head_opens_a_large_literal() {
    assert!(is_large_literal_head(b"chosen when the flag is set (long)"));
    assert!(is_large_literal_head(b"level two (short"));
}

#[test]
fn a_head_shorter_than_a_large_literal_does_not() {
    assert!(!is_large_literal_head(b"fifteen bytes!!"));
    assert!(!is_large_literal_head(b""));
}

#[test]
fn a_nul_inside_the_head_ends_a_shorter_string() {
    assert!(!is_large_literal_head(b"level one\0level two"));
    assert!(!is_large_literal_head(&[0u8; MIN_LARGE_LITERAL]));
}

#[test]
fn utf8_cut_by_the_window_still_opens_a_literal() {
    let text = "a non-ASCII caf\u{e9} au lait".as_bytes();
    assert_eq!(text[15], 0xc3);
    assert!(is_large_literal_head(&text[..MIN_LARGE_LITERAL]));
}

#[test]
fn bytes_that_are_not_utf8_do_not() {
    let mut head = *b"code, not a str!";
    head[3] = 0xff;
    assert!(!is_large_literal_head(&head));
    head[3] = 0x80;
    assert!(!is_large_literal_head(&head));
}
