//! Unit tests for the (kuna) empty-string-constant sub-stage.

use super::*;

/// The two witnesses, verbatim: the packed maze at 0x403550 in
/// `Sabloom Text 6.exe`, and the merged `""` at 0x7661 in the coreutils `fmt`
/// fixture that `setlocale(6,"")` points at.
const MAZE: [uint1; WINDOW] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x77, 0xdf, 0x77, 0xff, 0xfd, 0xff, 0x7f,
];
const MERGED_EMPTY: [uint1; WINDOW] = [
    0x00, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x62, 0x75, 0x67, 0x73, 0x20, 0x74, 0x6f, 0x3a,
];

#[test]
fn option_parses_on_and_off() {
    let (val, msg) = OptionEmptyStrConst.apply("on").unwrap();
    assert!(val);
    assert!(msg.contains("on"));
    let (val, msg) = OptionEmptyStrConst.apply("off").unwrap();
    assert!(!val);
    assert!(msg.contains("off"));
    assert!(OptionEmptyStrConst.apply("maybe").is_err());
}

#[test]
fn blob_neighbourhood_is_not_string_data() {
    assert!(!reads_as_string_data(&MAZE));
    assert!(declines_literal(true, 0, Some(&MAZE)));
}

#[test]
fn merged_empty_literal_keeps_its_quotes() {
    assert!(reads_as_string_data(&MERGED_EMPTY));
    assert!(!declines_literal(true, 0, Some(&MERGED_EMPTY)));
}

#[test]
fn a_literal_with_any_content_is_never_declined() {
    assert!(!declines_literal(true, 1, Some(&MAZE)));
    assert!(!declines_literal(true, 64, Some(&MAZE)));
}

#[test]
fn an_unreadable_neighbourhood_is_not_evidence() {
    assert!(!declines_literal(true, 0, None));
}

#[test]
fn option_off_restores_the_upstream_literal() {
    assert!(!declines_literal(false, 0, Some(&MAZE)));
}

#[test]
fn string_charset_is_printable_ascii_plus_the_c_whitespace_controls() {
    assert!(reads_as_string_data(&[0x00, b'\t', b'\n', b'\r', b' ', b'~']));
    assert!(!reads_as_string_data(&[b'a', 0x7f]));
    assert!(!reads_as_string_data(&[b'a', 0x1b]));
    assert!(!reads_as_string_data(&[b'a', 0x80]));
}

#[test]
fn only_the_run_after_the_terminator_is_judged() {
    // A short literal followed by binary is still a string table: the run this
    // pointer opens ends at its own NUL, and what lies past it is somebody
    // else's business.  This is `du`'s `fts_alloc(sp,"",0)`, whose "" is the
    // head of `"" "." ".."` and is followed by relocation-shaped bytes.
    assert!(reads_as_string_data(&[0x00, 0x2e, 0x00, 0x2e, 0x2e, 0x00, 0x00, 0x00, 0x89, 0x1c]));
    // And the blob is rejected on the first byte of its run that no string holds.
    assert!(!reads_as_string_data(&[0x00, 0x00, 0x77, 0xdf]));
}

#[test]
fn a_window_of_nothing_but_nuls_says_nothing() {
    // Padding at the end of a section is not evidence against the literal.
    assert!(reads_as_string_data(&[0u8; WINDOW]));
    assert!(!declines_literal(true, 0, Some(&[0u8; WINDOW])));
}
