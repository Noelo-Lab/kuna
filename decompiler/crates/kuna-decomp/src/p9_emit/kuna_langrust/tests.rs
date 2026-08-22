//! Tests for the rust-language profile.

use super::*;
use crate::kuna_lang::OutLang;

#[test]
fn rust_resolves_from_its_names() {
    assert_eq!(OutLang::from_print_name("rust-language"), Some(OutLang::Rust));
    assert_eq!(OutLang::from_print_name("rust"), Some(OutLang::Rust));
    assert_eq!(OutLang::from_print_name("c-language"), Some(OutLang::C));
    assert_eq!(OutLang::from_print_name("not-a-language"), None);
    assert_eq!(OutLang::Rust.print_name(), "rust-language");
    assert_eq!(OutLang::Rust.profile().name, "rust-language");
    assert_eq!(OutLang::Rust.profile().file_ext, "rs");
}

/// The capability record is the whole point of the profile: each `false` here
/// disables a construct that C emits by default and Rust cannot parse.
#[test]
fn rust_caps_forbid_the_c_only_constructs() {
    let c = OutLang::Rust.profile().caps;
    assert!(!c.goto, "Rust has no goto");
    assert!(!c.do_while, "Rust has no do/while");
    assert!(!c.c_for, "Rust has no C-style for header");
    assert!(!c.ternary, "Rust has no ?: -- if/else is an expression instead");
    assert!(!c.comma_expression, "no Rust form for condfold's comma operand");
    assert!(!c.arrow_member, "Rust raw pointers need an explicit deref");
    assert!(!c.switch_falls_through, "match arms do not fall through");
    assert!(!c.switch_default_optional, "match on an integer must be exhaustive");
    assert!(c.labeled_loop_break && c.labeled_block_break);
    assert!(c.switch_multi_label, "A | B => is the multi-label form");
}

/// The three kuna rendering defaults that are ON and produce non-Rust text.
/// Each is neutralized by a capability rather than by asking an operator to flip
/// an option, which is what makes selecting Rust a single correct choice.
#[test]
fn rust_caps_neutralize_the_default_on_c_renderings() {
    let c = OutLang::Rust.profile().caps;
    assert!(!c.implicit_bool_conditions, "truthycond DIV-37 would emit `if x` on a u32");
    assert!(!c.brace_elision, "braceelide DIV-38 would emit a braceless body");
}

/// The hazard this record exists to close: a C `switch` captures a bare `break`,
/// which is why `kuna_scope_break` retags a goto-to-switch-exit as
/// `f_break_goto`. A Rust `match` does not, so the same flag rendered naively
/// inside a `match` arm inside a loop would break the *loop*.
#[test]
fn rust_match_does_not_capture_break() {
    assert!(crate::kuna_lang::OutLang::C.profile().caps.switch_captures_break);
    assert!(!OutLang::Rust.profile().caps.switch_captures_break);
}

/// Ghidra's BOOLEAN_XOR is the invented C spelling `^^` at precedence 20
/// (between `&&` and `||`); Rust spells logical xor `^`, which binds at the
/// bitwise tier. Keeping precedence 20 while emitting `^` would make the
/// parenthesizer omit parentheses Rust needs.
#[test]
fn rust_spells_bitwise_not_as_bang() {
    let map = OutLang::Rust.profile().map_token;
    assert_eq!(tokens::BITWISE_NOT.print1, "~");
    assert_eq!(map(&tokens::BITWISE_NOT).print1, "!");
    // Same prefix-unary tier, so nothing else in the table shifts.
    assert_eq!(map(&tokens::BITWISE_NOT).precedence, tokens::BITWISE_NOT.precedence);
    assert_eq!(map(&tokens::BITWISE_NOT).token_type, tokens::BITWISE_NOT.token_type);
}

/// THE precedence hazard. C ranks `& ^ |` BELOW `== !=`; Rust ranks them ABOVE.
/// Keeping C's numbers would emit `a | b == c` for `(a | b) == c`, which Rust
/// reads as `a | (b == c)` -- a silent wrong answer, not a syntax error.
#[test]
fn rust_ranks_the_bitwise_operators_above_equality() {
    let map = OutLang::Rust.profile().map_token;
    for c_tok in [&tokens::BITWISE_AND, &tokens::BITWISE_XOR, &tokens::BITWISE_OR] {
        assert!(
            c_tok.precedence < tokens::EQUAL.precedence,
            "C ranks `{}` below equality",
            c_tok.print1
        );
        assert!(
            map(c_tok).precedence > tokens::EQUAL.precedence,
            "Rust must rank `{}` above equality",
            c_tok.print1
        );
        assert_eq!(map(c_tok).print1, c_tok.print1, "the spelling is unchanged");
    }
    // ... and in the same order relative to each other.
    assert!(map(&tokens::BITWISE_AND).precedence > map(&tokens::BITWISE_XOR).precedence);
    assert!(map(&tokens::BITWISE_XOR).precedence > map(&tokens::BITWISE_OR).precedence);
    // Still below the shifts, which both languages rank above them.
    assert!(map(&tokens::BITWISE_AND).precedence < tokens::SHIFT_LEFT.precedence);
}

/// Rust puts every comparison at ONE non-associative level; C ranks the
/// relational operators above equality. Flattening is what parenthesises
/// `(a < b) != 0` instead of emitting the chained `a < b != 0`.
#[test]
fn rust_flattens_the_comparison_tier() {
    let map = OutLang::Rust.profile().map_token;
    assert!(tokens::LESS_THAN.precedence > tokens::EQUAL.precedence, "C ranks them apart");
    for c_tok in [
        &tokens::LESS_THAN,
        &tokens::LESS_EQUAL,
        &tokens::GREATER_THAN,
        &tokens::GREATER_EQUAL,
    ] {
        assert_eq!(map(c_tok).precedence, tokens::EQUAL.precedence);
        assert!(!map(c_tok).associative, "Rust comparisons do not chain");
        assert_eq!(map(c_tok).print1, c_tok.print1);
    }
}

/// The negate pairing survives the remap, so `!(a < b)` still folds to `a >= b`.
/// The Rust statics carry it on `OpToken::negate` rather than through the C
/// pointer table, which cannot see them.
#[test]
fn rust_comparisons_keep_their_negations() {
    let map = OutLang::Rust.profile().map_token;
    for (a, b) in [
        (&tokens::LESS_THAN, ">="),
        (&tokens::LESS_EQUAL, ">"),
        (&tokens::GREATER_THAN, "<="),
        (&tokens::GREATER_EQUAL, "<"),
    ] {
        let neg = crate::printc::token_negate(map(a)).expect("negation pair");
        assert_eq!(neg.print1, b);
    }
}

#[test]
fn rust_boolean_xor_is_the_bitwise_operator() {
    let map = OutLang::Rust.profile().map_token;
    assert_eq!(tokens::BOOLEAN_XOR.print1, "^^", "Ghidra's invented C spelling");
    let r = map(&tokens::BOOLEAN_XOR);
    assert_eq!(r.print1, "^");
    // Rust has one `^`, so logical and bitwise xor rank together.
    assert_eq!(r.precedence, map(&tokens::BITWISE_XOR).precedence);
}

/// `x as T` binds tighter than `*`/`/` and looser than the unary operators.
#[test]
fn rust_cast_sits_between_unary_and_multiplicative() {
    let p = OutLang::Rust.profile();
    assert_eq!(p.tok_typecast.print1, " as ");
    assert!(p.tok_typecast.precedence < tokens::DEREFERENCE.precedence);
    assert!(p.tok_typecast.precedence > tokens::MULTIPLY.precedence);
}

/// A keyword with no Rust spelling is empty, and the matching capability forbids
/// the form that would emit it -- so an empty spelling can never reach output.
#[test]
fn absent_keywords_are_paired_with_a_forbidding_cap() {
    let p = OutLang::Rust.profile();
    assert!(p.kw_goto.is_empty() && !p.caps.goto);
    assert!(p.kw_do.is_empty() && !p.caps.do_while);
    assert!(p.kw_for.is_empty() && !p.caps.c_for);
    assert!(p.kw_case.is_empty() && !p.caps.switch_falls_through);
    assert_eq!(p.kw_switch, "match");
    assert_eq!(p.kw_default, "_");
}

/// The forms are the whole reason a second language does not need a second
/// emitter: each names the shape `printc.rs` should reach for.
#[test]
fn rust_forms_select_the_rust_shapes() {
    use crate::kuna_lang::*;
    let f = OutLang::Rust.profile().forms;
    assert_eq!(f.proto, ProtoForm::RustFnArrow);
    assert_eq!(f.decl, DeclForm::RustLetColon);
    assert_eq!(f.switch, SwitchForm::RustMatch);
    assert_eq!(f.inf_loop, InfLoopForm::RustLoop);
    assert_eq!(f.while_loop, WhileForm::RustBareWhile);
    assert_eq!(f.do_while, DoWhileForm::RustLoopBreakIf);
    assert_eq!(f.for_loop, ForLoopForm::DegradeToWhile);
    assert_eq!(f.goto, GotoForm::Unrepresentable);
    assert_eq!(f.label, LabelForm::CommentOnly);
    assert_eq!(f.member, MemberForm::RustDerefParen);
    assert_eq!(f.cast, CastForm::PostfixAs);
    assert_eq!(f.char_lit, CharForm::RustByte);
    assert_eq!(f.string_escape, StringEscape::RustEscapes);
}

/// `x as i32 < 5` parses `i32 <` as the start of generic arguments. Precedence
/// cannot express that -- `as` genuinely binds tighter than `<` -- so the token
/// carries the rule directly.
#[test]
fn the_cast_forces_parens_before_an_angle_bracket() {
    assert!(OutLang::Rust.profile().tok_typecast.paren_before_angle);
    assert!((OutLang::Rust.profile().map_token)(&crate::printc::tokens::TYPECAST).paren_before_angle);
    assert!(!crate::printc::tokens::TYPECAST.paren_before_angle);
}

#[test]
fn rust_spells_null_as_a_raw_pointer() {
    assert_eq!(OutLang::Rust.profile().null_literal, "core::ptr::null_mut()");
    assert_eq!(OutLang::C.profile().null_literal, "NULL");
}

/// Decompiled output does not follow Rust naming and does not use every binding.
#[test]
fn rust_emits_an_allow_attribute() {
    let attrs = OutLang::Rust.profile().fn_attributes;
    assert_eq!(attrs.len(), 1);
    assert!(attrs[0].starts_with("#[allow("));
    assert!(attrs[0].contains("non_snake_case"));
    assert!(attrs[0].contains("unused_parens"));
}
