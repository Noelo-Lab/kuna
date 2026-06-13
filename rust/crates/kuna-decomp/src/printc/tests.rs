//! Tests for the `PrintC` c-language token data and the self-contained
//! constant/type formatters (`printc.rs`).
//!
//! These exercise the part of `PrintC` that is faithfully ported now and is
//! print-fidelity-critical: the operator-token precedence/associativity table
//! (which drives `parentheses`), the negate cross-links, the keyword constants,
//! the byte-for-byte integer / float / char token strings, the generic
//! name builders, and the opcode→token dispatch — plus the `resetDefaultsPrintC`
//! defaults including the kuna `arraynotation` default-on.  The RPN/`Emit`
//! driver bodies are the W9 seam (see the module docs) and are not testable
//! until `prettyprint` lands.

use super::*;
use crate::printlanguage::{parentheses, ReversePolish};

fn rpn(tok: &'static OpToken, visited: int4) -> ReversePolish {
    ReversePolish { tok, visited, paren: false, op: None, id: 0, id2: 0 }
}

// ---------------------------------------------------------------------------
// Operator token table (printc.cc:24-78)
// ---------------------------------------------------------------------------

#[test]
fn token_table_precedence_and_type() {
    use tokens::*;
    // Spot-check the precedence ladder that defines C parenthesization
    // (printc.cc table).  Multiply/divide bind tighter than add/sub, which bind
    // tighter than shift, comparison, equality, bitwise, boolean, assignment,
    // comma.
    assert_eq!(MULTIPLY.precedence, 54);
    assert_eq!(BINARY_PLUS.precedence, 50);
    assert_eq!(SHIFT_LEFT.precedence, 46);
    assert_eq!(LESS_THAN.precedence, 42);
    assert_eq!(EQUAL.precedence, 38);
    assert_eq!(BITWISE_AND.precedence, 34);
    assert_eq!(BITWISE_XOR.precedence, 30);
    assert_eq!(BITWISE_OR.precedence, 26);
    assert_eq!(BOOLEAN_AND.precedence, 22);
    assert_eq!(BOOLEAN_OR.precedence, 18);
    assert_eq!(ASSIGNMENT.precedence, 14);
    assert_eq!(COMMA.precedence, 2);

    // Associativity flags (printc.cc).
    assert!(MULTIPLY.associative);
    assert!(!DIVIDE.associative);
    assert!(BINARY_PLUS.associative);
    assert!(!BINARY_MINUS.associative);
    assert!(BITWISE_AND.associative);
    assert!(!BOOLEAN_AND.associative);

    // Token types.
    assert_eq!(SUBSCRIPT.token_type, TokenType::Postsurround);
    assert_eq!(FUNCTION_CALL.token_type, TokenType::Postsurround);
    assert_eq!(TYPECAST.token_type, TokenType::Presurround);
    assert_eq!(DEREFERENCE.token_type, TokenType::UnaryPrefix);
    assert_eq!(HIDDEN.token_type, TokenType::HiddenFunction);
    assert_eq!(NEW_OP.token_type, TokenType::Space);

    // Printed characters.
    assert_eq!(SHIFT_LEFT.print1, "<<");
    assert_eq!(POINTER_MEMBER.print1, "->");
    assert_eq!(SUBSCRIPT.print1, "[");
    assert_eq!(SUBSCRIPT.print2, "]");
    assert_eq!(FUNCTION_CALL.bump, 10);
    assert_eq!(ASSIGNMENT.bump, 5);
}

#[test]
fn token_table_inplace_and_type_exprs() {
    use tokens::*;
    // In-place ops all bind at assignment precedence with bump 5 (printc.cc:62-71).
    for t in [
        &MULTEQUAL, &DIVEQUAL, &REMEQUAL, &PLUSEQUAL, &MINUSEQUAL, &LEFTEQUAL, &RIGHTEQUAL,
        &ANDEQUAL, &OREQUAL, &XOREQUAL,
    ] {
        assert_eq!(t.precedence, 14);
        assert_eq!(t.bump, 5);
        assert_eq!(t.token_type, TokenType::Binary);
    }
    assert_eq!(PLUSEQUAL.print1, "+=");
    assert_eq!(LEFTEQUAL.print1, "<<=");

    // Type-declaration tokens (printc.cc:74-78).
    assert_eq!(TYPE_EXPR_SPACE.spacing, 1);
    assert_eq!(TYPE_EXPR_NOSPACE.spacing, 0);
    assert_eq!(PTR_EXPR.print1, "*");
    assert_eq!(PTR_EXPR.token_type, TokenType::UnaryPrefix);
    assert_eq!(ARRAY_EXPR.token_type, TokenType::Postsurround);
    assert_eq!(ENUM_CAT.print1, "|");
}

#[test]
fn negate_cross_links() {
    use tokens::*;
    // printc.cc:130-135 — the six comparison flip pairs.
    assert!(std::ptr::eq(token_negate(&LESS_THAN).unwrap(), &GREATER_EQUAL));
    assert!(std::ptr::eq(token_negate(&LESS_EQUAL).unwrap(), &GREATER_THAN));
    assert!(std::ptr::eq(token_negate(&GREATER_THAN).unwrap(), &LESS_EQUAL));
    assert!(std::ptr::eq(token_negate(&GREATER_EQUAL).unwrap(), &LESS_THAN));
    assert!(std::ptr::eq(token_negate(&EQUAL).unwrap(), &NOT_EQUAL));
    assert!(std::ptr::eq(token_negate(&NOT_EQUAL).unwrap(), &EQUAL));
    // A token with no complement (C++ negate stays null).
    assert!(token_negate(&MULTIPLY).is_none());
    assert!(token_negate(&BINARY_PLUS).is_none());
    // The negate of a token's negate is itself (the pairs are symmetric).
    let n = token_negate(&LESS_THAN).unwrap();
    assert!(std::ptr::eq(token_negate(n).unwrap(), &LESS_THAN));
}

// ---------------------------------------------------------------------------
// parentheses driven by the real PrintC token table
// ---------------------------------------------------------------------------

#[test]
fn parentheses_uses_printc_token_precedence() {
    use tokens::*;
    // `a * (b + c)`: the `+` (op2, prec 50) feeds the `*` (top, prec 54).
    // top.precedence > op2.precedence -> parentheses required.
    let top = rpn(&MULTIPLY, 1);
    assert!(parentheses(&top, &BINARY_PLUS, None));

    // `a + b * c`: the `*` (op2, prec 54) feeds the `+` (top, prec 50).
    // top.precedence < op2.precedence -> no parentheses.
    let top = rpn(&BINARY_PLUS, 1);
    assert!(!parentheses(&top, &MULTIPLY, None));

    // Same associative token (`a * b * c` with `*` associative): no parens.
    let top = rpn(&MULTIPLY, 0);
    assert!(!parentheses(&top, &MULTIPLY, None));

    // Same precedence, NON-associative: `a - (b - c)` needs parens on the rhs.
    let top = rpn(&BINARY_MINUS, 1);
    assert!(parentheses(&top, &BINARY_MINUS, None));
}

#[test]
fn parentheses_unary_adjacent_sign_kuna_fix() {
    use tokens::*;
    // (kuna) GH-2786: a unary minus feeding a unary minus would merge into `--`;
    // parentheses must intervene.  Both tokens are UNARY_MINUS (print1 "-").
    let top = rpn(&UNARY_MINUS, 0);
    assert!(parentheses(&top, &UNARY_MINUS, None));
    // A unary minus feeding a different unary prefix (bitwise not) does not.
    let top = rpn(&UNARY_MINUS, 0);
    assert!(!parentheses(&top, &BITWISE_NOT, None));
}

// ---------------------------------------------------------------------------
// Keyword / capability constants
// ---------------------------------------------------------------------------

#[test]
fn keyword_and_capability_constants() {
    use keywords::*;
    assert_eq!(KEYWORD_VOID, "void");
    assert_eq!(KEYWORD_IF, "if");
    assert_eq!(KEYWORD_ELSE, "else");
    assert_eq!(KEYWORD_RETURN, "return");
    assert_eq!(DOTDOTDOT, "...");
    assert_eq!(OPEN_CURLY, "{");
    assert_eq!(TYPE_POINTER_REL_TOKEN, "ADJ");
    assert_eq!(CAPABILITY_NAME, "c-language");
    // C++ `isdefault = true` (printc.cc:113).  Read through a runtime binding so
    // clippy doesn't treat it as an assert on a literal constant.
    let is_default = std::hint::black_box(CAPABILITY_IS_DEFAULT);
    assert!(is_default);
}

// ---------------------------------------------------------------------------
// PrintC option defaults (printc.cc:1649-1664)
// ---------------------------------------------------------------------------

#[test]
fn option_defaults_match_reset_defaults_printc() {
    let o = PrintCOptions::new();
    assert!(o.convention);
    assert!(o.hide_exts);
    assert!(!o.inplace_ops);
    assert!(!o.nocasts);
    assert!(!o.null);
    assert!(!o.unplaced);
    // (kuna) DIV-2 default-on: &base[index] for standalone PTRADD (GH-558).
    assert!(o.array_notation);
    assert_eq!(o.brace_func, BraceStyle::SkipLine);
    assert_eq!(o.brace_ifelse, BraceStyle::SameLine);
    assert_eq!(o.brace_loop, BraceStyle::SameLine);
    assert_eq!(o.brace_switch, BraceStyle::SameLine);
}

#[test]
fn option_setters() {
    let mut o = PrintCOptions::new();
    o.set_null_printing(true);
    o.set_inplace_ops(true);
    o.set_no_cast_printing(true);
    o.set_array_notation(false);
    assert!(o.null);
    assert!(o.inplace_ops);
    assert!(o.nocasts);
    assert!(!o.array_notation());
    o.set_brace_format_function(BraceStyle::NextLine);
    assert_eq!(o.brace_func, BraceStyle::NextLine);
}

// ---------------------------------------------------------------------------
// Integer constant formatting (printc.cc:1359-1436)
// ---------------------------------------------------------------------------

#[test]
fn format_integer_hex_dec_oct_bin() {
    // Hex: lowercase, 0x prefix.
    assert_eq!(
        format_integer_token(false, 0xdead_beef, display_format::FORCE_HEX, 4, false, false, false, ""),
        "0xdeadbeef"
    );
    // Decimal.
    assert_eq!(
        format_integer_token(false, 42, display_format::FORCE_DEC, 4, false, false, false, ""),
        "42"
    );
    // Octal: leading 0.
    assert_eq!(
        format_integer_token(false, 8, display_format::FORCE_OCT, 4, false, false, false, ""),
        "010"
    );
    // Binary: 0b prefix, padded to power-of-two width.
    assert_eq!(
        format_integer_token(false, 5, display_format::FORCE_BIN, 1, false, false, false, ""),
        "0b00000101"
    );
}

#[test]
fn format_integer_negative_unsigned_sized() {
    // Negative sign prefix.
    assert_eq!(
        format_integer_token(true, 5, display_format::FORCE_DEC, 4, false, false, false, ""),
        "-5"
    );
    // Force-unsigned 'U' suffix.
    assert_eq!(
        format_integer_token(false, 7, display_format::FORCE_DEC, 4, true, false, false, ""),
        "7U"
    );
    // Sized suffix.
    assert_eq!(
        format_integer_token(false, 7, display_format::FORCE_DEC, 8, false, true, false, "LL"),
        "7LL"
    );
}

#[test]
fn format_integer_char() {
    // Plain printable char.
    assert_eq!(
        format_integer_token(false, b'A' as u64, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'A'"
    );
    // Escape char (newline).
    assert_eq!(
        format_integer_token(false, b'\n' as u64, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'\\n'"
    );
    // High byte (>= 0x80) with sz==1 uses the hex escape.
    assert_eq!(
        format_integer_token(false, 0x80, display_format::FORCE_CHAR, 1, false, false, false, ""),
        "'\\x80'"
    );
    // Wide-char prefix when sz > 1.
    assert_eq!(
        format_integer_token(false, b'A' as u64, display_format::FORCE_CHAR, 2, false, false, true, ""),
        "L'A'"
    );
}

#[test]
fn resolve_integer_format_sign_and_base() {
    // Signed value whose top bit is set, sz 1 (0xff = -1).
    let (neg, val, fmt) = resolve_integer_format(0xff, 1, true, 0, false, false);
    assert!(neg);
    assert_eq!(val, 1); // magnitude
    assert_eq!(fmt, display_format::FORCE_DEC); // val<=10

    // Small positive prints decimal regardless of natural base.
    let (neg, val, fmt) = resolve_integer_format(7, 4, false, 0, false, false);
    assert!(!neg);
    assert_eq!(val, 7);
    assert_eq!(fmt, display_format::FORCE_DEC);

    // force_hex mod wins.
    let (_, _, fmt) = resolve_integer_format(1000, 4, false, 0, true, false);
    assert_eq!(fmt, display_format::FORCE_HEX);

    // An explicit display-format override is preserved.
    let (_, _, fmt) =
        resolve_integer_format(1000, 4, false, display_format::FORCE_OCT, false, false);
    assert_eq!(fmt, display_format::FORCE_OCT);

    // 0xff with no force: most natural base of 255 is hex.
    let (_, _, fmt) = resolve_integer_format(0xff, 4, false, 0, false, false);
    assert_eq!(fmt, display_format::FORCE_HEX);
}

#[test]
fn resolve_then_format_full_path() {
    // The full push_integer path for a signed -1 (sz 4, 0xffffffff).
    let (neg, val, fmt) = resolve_integer_format(0xffff_ffff, 4, true, 0, false, false);
    let s = format_integer_token(neg, val, fmt, 4, false, false, false, "");
    assert_eq!(s, "-1");
}

// ---------------------------------------------------------------------------
// Char hex escape / unicode (printc.cc:1494-1591)
// ---------------------------------------------------------------------------

#[test]
fn char_hex_escape_widths() {
    let mut s = String::new();
    print_char_hex_escape(&mut s, 0x7);
    assert_eq!(s, "\\x07");
    let mut s = String::new();
    print_char_hex_escape(&mut s, 0x1ff);
    assert_eq!(s, "\\x01ff");
    let mut s = String::new();
    print_char_hex_escape(&mut s, 0x1_0000);
    assert_eq!(s, "\\x00010000");
}

#[test]
fn print_unicode_escapes_and_raw() {
    let mut s = String::new();
    print_unicode(&mut s, 0);
    assert_eq!(s, "\\0");
    let mut s = String::new();
    print_unicode(&mut s, 9);
    assert_eq!(s, "\\t");
    let mut s = String::new();
    print_unicode(&mut s, 92);
    assert_eq!(s, "\\\\");
    let mut s = String::new();
    print_unicode(&mut s, 0x22);
    assert_eq!(s, "\\\"");
    // A printable ASCII char passes through raw.
    let mut s = String::new();
    print_unicode(&mut s, b'A' as int4);
    assert_eq!(s, "A");
    // A printable non-ASCII codepoint passes through as UTF-8.
    let mut s = String::new();
    print_unicode(&mut s, 0x00e9); // é (A1-FF printable range)
    assert_eq!(s, "é");
}

// ---------------------------------------------------------------------------
// Float constant formatting (printc.cc:1448-1492)
// ---------------------------------------------------------------------------

#[test]
fn format_float_special_classes() {
    assert_eq!(format_float_token(FloatClass::Unknown, false, "", false), "FLOAT_UNKNOWN");
    assert_eq!(format_float_token(FloatClass::Infinity, false, "", false), "INFINITY");
    assert_eq!(format_float_token(FloatClass::Infinity, true, "", false), "-INFINITY");
    assert_eq!(format_float_token(FloatClass::Nan, false, "", false), "NAN");
    assert_eq!(format_float_token(FloatClass::Nan, true, "", false), "-NAN");
}

#[test]
fn format_float_normal_dot_zero_suffix() {
    // A decimal that already looks like a float is untouched.
    assert_eq!(format_float_token(FloatClass::Normal, false, "3.14159", false), "3.14159");
    assert_eq!(format_float_token(FloatClass::Normal, false, "1e+10", false), "1e+10");
    // A bare integer-looking decimal gets ".0" forced on (printc.cc:1485-1487).
    assert_eq!(format_float_token(FloatClass::Normal, false, "5", false), "5.0");
    assert_eq!(format_float_token(FloatClass::Normal, false, "0", false), "0.0");
    // Scientific (force_scinote): used verbatim, no .0 fix-up.
    assert_eq!(
        format_float_token(FloatClass::Normal, false, "5.00000e+00", true),
        "5.00000e+00"
    );
}

#[test]
fn format_float_consumes_floatformat_print_decimal() {
    // The real round-trip path: kuna-num FloatFormat::print_decimal -> the
    // token-string fix-up.  An IEEE-754 double-precision format.
    let fmt = kuna_num::float::FloatFormat::new(8);
    // 0.5 round-trips to "0.5", already float-looking.
    let dec = fmt.print_decimal(0.5, false);
    assert_eq!(format_float_token(FloatClass::Normal, false, &dec, false), "0.5");
    // 2.0 round-trips to "2"; push_float forces ".0".
    let dec = fmt.print_decimal(2.0, false);
    assert_eq!(format_float_token(FloatClass::Normal, false, &dec, false), "2.0");
}

// ---------------------------------------------------------------------------
// Generic names (printc.cc:3516-3558)
// ---------------------------------------------------------------------------

#[test]
fn generic_type_name_cases() {
    use type_metatype::*;
    assert_eq!(generic_type_name(TYPE_INT, 4), "unkint4");
    assert_eq!(generic_type_name(TYPE_UINT, 2), "unkuint2");
    assert_eq!(generic_type_name(TYPE_UNKNOWN, 1), "unkbyte1");
    assert_eq!(generic_type_name(TYPE_FLOAT, 8), "unkfloat8");
    assert_eq!(generic_type_name(TYPE_SPACEBASE, 4), "BADSPACEBASE");
    assert_eq!(generic_type_name(TYPE_STRUCT, 4), "BADTYPE");
}

#[test]
fn generic_function_name_invalid_addr() {
    // The non-kuna `func_<addr>` branch.  A null address renders the raw form
    // as "invalid_addr".
    let addr = kuna_base::address::Address::new_invalid();
    assert_eq!(generic_function_name(&addr).unwrap(), "func_invalid_addr");
}

// ---------------------------------------------------------------------------
// Opcode -> token dispatch (printc.hh:289-351)
// ---------------------------------------------------------------------------

#[test]
fn op_emit_kind_dispatch() {
    use kuna_num::opcodes::OpCode::*;
    use tokens::*;

    let kind = op_emit_kind(CPUI_INT_ADD);
    match kind {
        OpEmitKind::Binary(t) => assert!(std::ptr::eq(t, &BINARY_PLUS)),
        _ => panic!("INT_ADD should be Binary(BINARY_PLUS)"),
    }
    // Signed and unsigned div both map to '/'.
    for op in [CPUI_INT_DIV, CPUI_INT_SDIV] {
        match op_emit_kind(op) {
            OpEmitKind::Binary(t) => assert!(std::ptr::eq(t, &DIVIDE)),
            _ => panic!("div should be Binary(DIVIDE)"),
        }
    }
    // Signed/unsigned less both map to '<'.
    for op in [CPUI_INT_LESS, CPUI_INT_SLESS] {
        match op_emit_kind(op) {
            OpEmitKind::Binary(t) => assert!(std::ptr::eq(t, &LESS_THAN)),
            _ => panic!("less should be Binary(LESS_THAN)"),
        }
    }
    // 2COMP and NEGATE are unary.
    match op_emit_kind(CPUI_INT_2COMP) {
        OpEmitKind::Unary(t) => assert!(std::ptr::eq(t, &UNARY_MINUS)),
        _ => panic!("2COMP should be Unary(UNARY_MINUS)"),
    }
    match op_emit_kind(CPUI_INT_NEGATE) {
        OpEmitKind::Unary(t) => assert!(std::ptr::eq(t, &BITWISE_NOT)),
        _ => panic!("NEGATE should be Unary(BITWISE_NOT)"),
    }
    // CARRY/SCARRY/SBORROW and most float-only ops are functional.
    assert!(matches!(op_emit_kind(CPUI_INT_CARRY), OpEmitKind::Func));
    assert!(matches!(op_emit_kind(CPUI_FLOAT_NAN), OpEmitKind::Func));
    assert!(matches!(op_emit_kind(CPUI_FLOAT_SQRT), OpEmitKind::Func));
    assert!(matches!(op_emit_kind(CPUI_POPCOUNT), OpEmitKind::Func));
    assert!(matches!(op_emit_kind(CPUI_LZCOUNT), OpEmitKind::Func));
    assert!(matches!(op_emit_kind(CPUI_PIECE), OpEmitKind::Func));
    // CAST / FLOAT2FLOAT / FLOAT_TRUNC use type-cast syntax.
    assert!(matches!(op_emit_kind(CPUI_CAST), OpEmitKind::TypeCast));
    assert!(matches!(op_emit_kind(CPUI_FLOAT_FLOAT2FLOAT), OpEmitKind::TypeCast));
    assert!(matches!(op_emit_kind(CPUI_FLOAT_TRUNC), OpEmitKind::TypeCast));
    // Hand-written / no-op overrides are Custom.
    assert!(matches!(op_emit_kind(CPUI_LOAD), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_STORE), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_CALL), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_MULTIEQUAL), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_INDIRECT), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_SUBPIECE), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_PTRADD), OpEmitKind::Custom));
    assert!(matches!(op_emit_kind(CPUI_PTRSUB), OpEmitKind::Custom));
}

// ---------------------------------------------------------------------------
// PrintC::doc_function shell (w9x-arch-engine-glue): a real signature + matched
// braces driven through the Emit primitives.  The body is the W9-emit seam.
// ---------------------------------------------------------------------------

#[test]
fn doc_function_emits_a_structurally_sane_shell() {
    let mut p = PrintC::new();
    let out = p.doc_function("main", None, "void", &[]);
    // Signature: the name + a () param list.
    assert!(out.contains("main"), "missing function name: {out}");
    assert!(out.contains('(') && out.contains(')'), "missing param parens: {out}");
    assert!(out.contains("void"), "missing void return/params: {out}");
    // Matched braces.
    assert_eq!(out.matches('{').count(), 1, "want one open brace: {out}");
    assert_eq!(out.matches('}').count(), 1, "want one close brace: {out}");
    // void return + void params (no recovered prototype).
    assert!(out.contains("void main(void)"), "want `void main(void)`: {out}");
}

#[test]
fn doc_function_renders_a_real_prototype() {
    let mut p = PrintC::new();
    let params =
        vec![("int".to_string(), "a".to_string()), ("char".to_string(), "b".to_string())];
    let out = p.doc_function("foo", Some("__stdcall"), "int", &params);
    assert!(out.contains("int"), "{out}");
    assert!(out.contains("foo"), "{out}");
    assert!(out.contains('a') && out.contains('b'), "params missing: {out}");
    // The convention/model name is printed (option_convention default-on).
    assert!(out.contains("__stdcall"), "model name missing: {out}");
    assert_eq!(out.matches('{').count(), out.matches('}').count());
}
