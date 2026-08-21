//! Tests for the output-language profile.
//!
//! The load-bearing one is [`profile_matches_legacy_constants`]: it is what makes
//! "read the C profile instead of the constant" a provably byte-identical
//! rewrite, and therefore what lets the migration touch `printc.rs` without
//! re-pinning `docs/baseline.json`.

use super::*;

#[test]
fn profile_matches_legacy_constants() {
    let p = OutLang::C.profile();

    assert_eq!(p.name, "c-language");
    assert_eq!(p.file_ext, "c");

    assert_eq!(p.kw_open_curly, keywords::OPEN_CURLY);
    assert_eq!(p.kw_close_curly, keywords::CLOSE_CURLY);
    assert_eq!(p.kw_semicolon, keywords::SEMICOLON);
    assert_eq!(p.kw_colon, keywords::COLON);
    assert_eq!(p.kw_comma, keywords::COMMA);
    assert_eq!(p.kw_dotdotdot, keywords::DOTDOTDOT);
    assert_eq!(p.kw_void, keywords::KEYWORD_VOID);
    assert_eq!(p.kw_true, keywords::KEYWORD_TRUE);
    assert_eq!(p.kw_false, keywords::KEYWORD_FALSE);
    assert_eq!(p.kw_if, keywords::KEYWORD_IF);
    assert_eq!(p.kw_else, keywords::KEYWORD_ELSE);
    assert_eq!(p.kw_do, keywords::KEYWORD_DO);
    assert_eq!(p.kw_while, keywords::KEYWORD_WHILE);
    assert_eq!(p.kw_for, keywords::KEYWORD_FOR);
    assert_eq!(p.kw_goto, keywords::KEYWORD_GOTO);
    assert_eq!(p.kw_break, keywords::KEYWORD_BREAK);
    assert_eq!(p.kw_continue, keywords::KEYWORD_CONTINUE);
    assert_eq!(p.kw_case, keywords::KEYWORD_CASE);
    assert_eq!(p.kw_switch, keywords::KEYWORD_SWITCH);
    assert_eq!(p.kw_default, keywords::KEYWORD_DEFAULT);
    assert_eq!(p.kw_return, keywords::KEYWORD_RETURN);
    assert_eq!(p.kw_type_pointer_rel, keywords::TYPE_POINTER_REL_TOKEN);
}

/// `printlanguage::parentheses` decides parenthesization by `std::ptr::eq` on the
/// token, so a profile that merely *spells* a token the same is not enough — the
/// C profile must hand back the very same static.
#[test]
fn profile_tokens_are_the_legacy_statics() {
    let p = OutLang::C.profile();
    assert!(std::ptr::eq(p.tok_bitwise_not, &tokens::BITWISE_NOT));
    assert!(std::ptr::eq(p.tok_boolean_and, &tokens::BOOLEAN_AND));
    assert!(std::ptr::eq(p.tok_boolean_or, &tokens::BOOLEAN_OR));
    assert!(std::ptr::eq(p.tok_boolean_xor, &tokens::BOOLEAN_XOR));
    assert!(std::ptr::eq(p.tok_addressof, &tokens::ADDRESSOF));
    assert!(std::ptr::eq(p.tok_dereference, &tokens::DEREFERENCE));
    assert!(std::ptr::eq(p.tok_typecast, &tokens::TYPECAST));
}

#[test]
fn c_is_the_default_language() {
    assert_eq!(OutLang::default(), OutLang::C);
    assert_eq!(OutLang::C.print_name(), OutLang::C.profile().name);
}

/// The C capability record must describe C, or every gate built on it is wrong
/// in the direction that silently changes C output.
#[test]
fn c_caps_describe_c() {
    let c = OutLang::C.profile().caps;
    assert!(c.goto);
    assert!(c.switch_captures_break);
    assert!(c.switch_falls_through);
    assert!(c.switch_default_optional);
    assert!(c.do_while);
    assert!(c.c_for);
    assert!(c.ternary);
    assert!(c.comma_expression);
    assert!(c.implicit_bool_conditions);
    assert!(c.brace_elision);
    assert!(c.arrow_member);
    assert!(!c.labeled_loop_break);
    assert!(!c.labeled_block_break);
}
