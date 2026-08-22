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
    assert_eq!(p.null_literal, "NULL");
}

/// The C forms are the ones `printc.rs` had before the plane existed; a wrong
/// value here would silently reroute the C path to an emitter written for
/// another language.
#[test]
fn c_forms_are_the_c_forms() {
    let f = OutLang::C.profile().forms;
    assert_eq!(f.proto, ProtoForm::CPrefixReturn);
    assert_eq!(f.decl, DeclForm::CTypeThenName);
    assert_eq!(f.switch, SwitchForm::CSwitch);
    assert_eq!(f.inf_loop, InfLoopForm::CDoWhileTrue);
    assert_eq!(f.while_loop, WhileForm::CParenWhile);
    assert_eq!(f.do_while, DoWhileForm::CDoWhile);
    assert_eq!(f.for_loop, ForLoopForm::CForHeader);
    assert_eq!(f.goto, GotoForm::CGoto);
    assert_eq!(f.label, LabelForm::CColon);
    assert_eq!(f.member, MemberForm::CArrow);
    assert_eq!(f.cast, CastForm::PrefixParen);
    assert_eq!(f.char_lit, CharForm::CQuoted);
    assert_eq!(f.string_escape, StringEscape::CEscapes);
    assert!(OutLang::C.profile().fn_attributes.is_empty());
}

/// `paren_before_angle` is a Rust-only rule; a C token that set it would start
/// parenthesising comparisons and change the corpus.
#[test]
fn no_c_token_forces_parens() {
    for tok in [
        &tokens::BITWISE_NOT,
        &tokens::BOOLEAN_AND,
        &tokens::BOOLEAN_OR,
        &tokens::BOOLEAN_XOR,
        &tokens::ADDRESSOF,
        &tokens::DEREFERENCE,
        &tokens::TYPECAST,
        &tokens::LESS_THAN,
        &tokens::SHIFT_LEFT,
    ] {
        assert!(!tok.paren_before_angle, "C token `{}` must not force parens", tok.print1);
    }
}

/// `printlanguage::parentheses` decides parenthesization by `std::ptr::eq` on the
/// token, so a mapping that merely *spells* a token the same is not enough — the
/// C mapping must hand back the very same static, for every token in the table.
#[test]
fn c_token_mapping_is_the_identity() {
    let map = OutLang::C.profile().map_token;
    for tok in [
        &tokens::BITWISE_NOT,
        &tokens::BOOLEAN_AND,
        &tokens::BOOLEAN_OR,
        &tokens::BOOLEAN_XOR,
        &tokens::BITWISE_AND,
        &tokens::BITWISE_XOR,
        &tokens::BITWISE_OR,
        &tokens::LESS_THAN,
        &tokens::LESS_EQUAL,
        &tokens::GREATER_THAN,
        &tokens::GREATER_EQUAL,
        &tokens::EQUAL,
        &tokens::NOT_EQUAL,
        &tokens::ADDRESSOF,
        &tokens::DEREFERENCE,
        &tokens::TYPECAST,
        &tokens::MULTIPLY,
        &tokens::BINARY_PLUS,
        &tokens::SHIFT_LEFT,
        &tokens::ASSIGNMENT,
    ] {
        assert!(std::ptr::eq(map(tok), tok), "C must not remap `{}`", tok.print1);
    }
    assert!(std::ptr::eq(OutLang::C.profile().tok_typecast, &tokens::TYPECAST));
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
    assert!(c.paren_conditions);
    assert!(c.integer_suffixes);
    assert!(!c.labeled_loop_break);
    assert!(!c.labeled_block_break);
}
