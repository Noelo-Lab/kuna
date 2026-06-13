//! Tests for the `PrintLanguage` base machinery (`printlanguage.rs`).
//!
//! The `parentheses` tests exercise the print-fidelity-critical
//! parenthesization decision (printlanguage.cc:270-327) across each token-type
//! arm; the static-helper tests check `mostNaturalBase`/`formatBinary`/
//! `unicodeNeedsEscape`/`unnamedField`; the stack tests check the mod/scope
//! save/restore and `setIntegerFormat`.

use super::modifiers::*;
use super::*;

/// A static `OpToken` builder for the precedence/associativity tests.  These
/// stand in for the `printc` token table singletons; `parentheses` reads only
/// these fields.
fn tok(
    print1: &'static str,
    precedence: int4,
    associative: bool,
    token_type: TokenType,
) -> OpToken {
    OpToken {
        print1,
        print2: "",
        stage: 1,
        precedence,
        associative,
        token_type,
        spacing: 1,
        bump: 0,
        negate: None,
    }
}

// Static singletons so `std::ptr::eq` (the C++ `topToken == op2`) is meaningful.
static MULT: OpToken = OpToken {
    print1: "*",
    print2: "",
    stage: 1,
    precedence: 50,
    associative: true,
    token_type: TokenType::Binary,
    spacing: 1,
    bump: 0,
    negate: None,
};
static ADD: OpToken = OpToken {
    print1: "+",
    print2: "",
    stage: 1,
    precedence: 40,
    associative: true,
    token_type: TokenType::Binary,
    spacing: 1,
    bump: 0,
    negate: None,
};

fn rpn(tok: &'static OpToken, visited: int4) -> ReversePolish {
    ReversePolish { tok, visited, paren: false, op: None, id: 0, id2: 0 }
}

#[test]
fn parentheses_binary_precedence() {
    // Higher-precedence top binding a lower-precedence input -> parens.
    // top = MULT(50), op2 = ADD(40): 50 > 40 -> true.
    assert!(parentheses(&rpn(&MULT, 1), &ADD, None));
    // top = ADD(40), op2 = MULT(50): 40 < 50 -> false.
    assert!(!parentheses(&rpn(&ADD, 1), &MULT, None));
}

#[test]
fn parentheses_binary_associative_same_token() {
    // Equal precedence, associative, and the SAME token pointer -> no parens
    // (a*(b*c) renders a*b*c).
    assert!(!parentheses(&rpn(&MULT, 1), &MULT, None));
    // A distinct token of equal precedence (not the same pointer) -> parens.
    let other_mult = tok("/", 50, false, TokenType::Binary);
    assert!(parentheses(&rpn(&MULT, 1), &other_mult, None));
}

#[test]
fn parentheses_binary_postsurround_first_stage() {
    // Equal precedence, op2 is a postsurround and top is in stage 0 -> op2 is
    // printed first so no parens.
    let post = tok("[", 50, false, TokenType::Postsurround);
    assert!(!parentheses(&rpn(&MULT, 0), &post, None));
    // In stage 1 the postsurround comes after -> parens.
    assert!(parentheses(&rpn(&MULT, 1), &post, None));
}

#[test]
fn parentheses_unary_adjacent_sign_gh2786() {
    // (kuna) GH-2786: a unary '-' inside a unary '-' must parenthesize, else the
    // two tokens merge into '--'.
    static NEG: OpToken = OpToken {
        print1: "-",
        print2: "",
        stage: 1,
        precedence: 60,
        associative: false,
        token_type: TokenType::UnaryPrefix,
        spacing: 0,
        bump: 0,
        negate: None,
    };
    assert!(parentheses(&rpn(&NEG, 0), &NEG, None));
    // A '-' inside a '+' (different sign) does NOT need the fix; equal precedence
    // unary_prefix -> no parens.
    let plus = tok("+", 60, false, TokenType::UnaryPrefix);
    assert!(!parentheses(&rpn(&NEG, 0), &plus, None));
}

#[test]
fn parentheses_unary_prefix_lower_op2() {
    // top unary prefix prec 60, op2 binary prec 40 -> 60 > 40 -> parens.
    let top_neg = tok("!", 60, false, TokenType::UnaryPrefix);
    assert!(parentheses(&rpn_owned(&top_neg, 0), &ADD, None));
    // op2 unary_prefix / presurround at equal-or-higher precedence -> no parens.
    let presur = tok("(cast)", 60, false, TokenType::Presurround);
    assert!(!parentheses(&rpn_owned(&top_neg, 0), &presur, None));
}

#[test]
fn parentheses_postsurround_inside_surround() {
    let post = tok("[", 70, false, TokenType::Postsurround);
    // stage 1 = inside the surround -> never parens.
    assert!(!parentheses(&rpn_owned(&post, 1), &MULT, None));
}

#[test]
fn parentheses_presurround_inside_surround() {
    let pre = tok("(cast)", 70, false, TokenType::Presurround);
    // stage 0 = inside the surround -> never parens.
    assert!(!parentheses(&rpn_owned(&pre, 0), &MULT, None));
}

#[test]
fn parentheses_hiddenfunction() {
    let hidden = tok("", 0, false, TokenType::HiddenFunction);
    // No previous token (stack size 1) -> returns true.
    assert!(parentheses(&rpn_owned(&hidden, 0), &ADD, None));
    // Previous token is a non-binary/non-unary (e.g. postsurround) -> false.
    let prev_post = tok("[", 70, false, TokenType::Postsurround);
    assert!(!parentheses(&rpn_owned(&hidden, 0), &ADD, Some(&prev_post)));
    // Previous binary with LOWER precedence than op2 -> false.
    let prev_low = tok("+", 30, true, TokenType::Binary);
    assert!(!parentheses(&rpn_owned(&hidden, 0), &MULT, Some(&prev_low)));
    // Previous binary with higher-or-equal precedence -> true.
    let prev_high = tok("*", 60, true, TokenType::Binary);
    assert!(parentheses(&rpn_owned(&hidden, 0), &ADD, Some(&prev_high)));
}

/// Build a `ReversePolish` over a *non-static* token for the cases where pointer
/// identity is not exercised.  `parentheses` only stores `&'static`, so we leak
/// a boxed token (test-only) to satisfy the lifetime.
fn rpn_owned(t: &OpToken, visited: int4) -> ReversePolish {
    let leaked: &'static OpToken = Box::leak(Box::new(OpToken {
        print1: t.print1,
        print2: t.print2,
        stage: t.stage,
        precedence: t.precedence,
        associative: t.associative,
        token_type: t.token_type,
        spacing: t.spacing,
        bump: t.bump,
        negate: None,
    }));
    rpn(leaked, visited)
}

// -- Static helpers ---------------------------------------------------------

#[test]
fn most_natural_base_cases() {
    // 0 -> decimal.
    assert_eq!(most_natural_base(0), 10);
    // 1000000 has a run of decimal zeros -> decimal.
    assert_eq!(most_natural_base(1000000), 10);
    // 0xff00 has a run of hex zeros/f's -> hex.
    assert_eq!(most_natural_base(0xff00), 16);
    // 12345 ends in 5 (no decimal 0/9 run) -> countdec 0 -> hex.
    assert_eq!(most_natural_base(12345), 16);
    // 99 has a run of 9's -> decimal.
    assert_eq!(most_natural_base(99), 10);
    // 0xdeadbeef -> hex.
    assert_eq!(most_natural_base(0xdeadbeef), 16);
}

#[test]
fn format_binary_widths() {
    let mut s = String::new();
    format_binary(&mut s, 0);
    assert_eq!(s, "0");

    let mut s = String::new();
    format_binary(&mut s, 5); // msb 2 -> width 8
    assert_eq!(s, "00000101");

    let mut s = String::new();
    format_binary(&mut s, 0x100); // msb 8 -> width 16
    assert_eq!(s, "0000000100000000");

    let mut s = String::new();
    format_binary(&mut s, 1); // msb 0 -> width 8
    assert_eq!(s, "00000001");
}

#[test]
fn unicode_needs_escape_cases() {
    assert!(unicode_needs_escape(0x07)); // control char
    assert!(!unicode_needs_escape(b'A' as int4)); // printable ASCII
    assert!(unicode_needs_escape(b'"' as int4)); // double quote
    assert!(unicode_needs_escape(b'\'' as int4)); // single quote
    assert!(unicode_needs_escape(92)); // back-slash
    assert!(!unicode_needs_escape(0xa1)); // printable Latin-1
    assert!(unicode_needs_escape(0x7f)); // DEL
    assert!(unicode_needs_escape(0x3000)); // ideographic space
    assert!(!unicode_needs_escape(0xfffc)); // object replacement (not escaped)
    assert!(unicode_needs_escape(0xfeff)); // BOM
    assert!(unicode_needs_escape(0x1680)); // ogham space mark
}

#[test]
fn unnamed_field_format() {
    assert_eq!(unnamed_field(4, 2), "_4_2_");
    assert_eq!(unnamed_field(0, 8), "_0_8_");
}

// -- PrintContext state -----------------------------------------------------

#[test]
fn mod_stack_save_restore() {
    let mut ctx = PrintContext::new();
    assert_eq!(ctx.mods(), 0);
    ctx.set_mod(FORCE_HEX);
    ctx.push_mod();
    ctx.set_mod(FLAT);
    assert!(ctx.is_set(FORCE_HEX) && ctx.is_set(FLAT));
    ctx.pop_mod();
    // After pop, only the pushed-snapshot mods remain.
    assert!(ctx.is_set(FORCE_HEX));
    assert!(!ctx.is_set(FLAT));
    ctx.unset_mod(FORCE_HEX);
    assert_eq!(ctx.mods(), 0);
}

#[test]
fn scope_stack_curscope() {
    let mut ctx = PrintContext::new();
    assert_eq!(ctx.cur_scope(), None);
    ctx.push_scope(1);
    ctx.push_scope(2);
    assert_eq!(ctx.cur_scope(), Some(2));
    ctx.pop_scope();
    assert_eq!(ctx.cur_scope(), Some(1));
    ctx.pop_scope();
    assert_eq!(ctx.cur_scope(), None);
}

#[test]
fn set_integer_format_toggles() {
    let mut ctx = PrintContext::new();
    ctx.set_integer_format("hex").unwrap();
    assert!(ctx.is_set(FORCE_HEX) && !ctx.is_set(FORCE_DEC));
    ctx.set_integer_format("dec").unwrap();
    assert!(ctx.is_set(FORCE_DEC) && !ctx.is_set(FORCE_HEX));
    ctx.set_integer_format("best").unwrap();
    assert!(!ctx.is_set(FORCE_HEX) && !ctx.is_set(FORCE_DEC));
    assert!(ctx.set_integer_format("octal").is_err());
}

#[test]
fn reset_defaults_internal_values() {
    let ctx = PrintContext::new();
    assert_eq!(ctx.mods(), 0);
    assert_eq!(ctx.line_comment_indent(), 20);
    assert_eq!(ctx.namespace_strategy(), NamespaceStrategy::MinimalNamespaces);
    assert_eq!(
        ctx.header_comment(),
        comment_type::HEADER | comment_type::WARNINGHEADER
    );
    assert_eq!(
        ctx.instruction_comment(),
        comment_type::USER2 | comment_type::WARNING
    );
}

#[test]
fn set_flat_toggles_flat_mod() {
    let mut ctx = PrintContext::new();
    ctx.set_flat(true);
    assert!(ctx.is_set(FLAT));
    ctx.set_flat(false);
    assert!(!ctx.is_set(FLAT));
}
