//! Unit tests for the (kuna) stack-cookie scramble alias exemption.

use super::*;

#[test]
fn the_layout_gather_is_upstream_faithful() {
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
