//! Unit tests for (kuna) `cookiescramble`.

use super::*;

#[test]
fn option_accepts_on_and_off() {
    let (v, msg) = OptionCookieScramble.apply("on").expect("on parses");
    assert!(v);
    assert!(msg.contains("on"), "{msg}");
    let (v, _) = OptionCookieScramble.apply("off").expect("off parses");
    assert!(!v);
    assert!(OptionCookieScramble.apply("maybe").is_err());
}

#[test]
fn option_off_is_upstream_faithful() {
    // Every non-additive use records an escape site, the cookie scramble included.
    assert!(is_escape_site(false, OpCode::CPUI_INT_XOR));
    assert!(is_escape_site(false, OpCode::CPUI_STORE));
}

#[test]
fn a_stack_pointer_scramble_is_not_an_escape() {
    assert!(!is_escape_site(true, OpCode::CPUI_INT_XOR));
}

#[test]
fn other_non_additive_uses_are_untouched() {
    for code in [OpCode::CPUI_STORE, OpCode::CPUI_LOAD, OpCode::CPUI_CALL, OpCode::CPUI_INT_AND] {
        assert!(is_escape_site(true, code), "{code:?}");
    }
}
