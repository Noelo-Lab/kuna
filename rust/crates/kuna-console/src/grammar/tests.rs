//! Tests for the C-declaration parser ([`super`]).
//!
//! The corpus is the representative `parse line` payloads grepped from
//! `decompiler/datatests/` plus the prototypes `testfuncproto.cc` /
//! `testparamstore.cc` feed `parse_protopieces`, together with the lexer's
//! token-level rules and the rejection (error-text) paths.

use kuna_decomp::dtype::{type_metatype as meta, TypeFactory, TypeFactoryImpl};

use super::{
    flags, parse_c_integer, parse_protopieces, parse_type, tok, CParse, DataOrg, DocType,
    GrammarLexer, GrammarToken,
};

// -- test helpers -----------------------------------------------------------

/// Build a factory wired with the alignment map, a max base size of 8, and the
/// named core data-types the datatest / unit-test corpus references (`int4`,
/// `char`, `void`, `float4`, ...).  These names are exactly what
/// `findByName` resolves during `parse line`.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    // void / undefined first (cacheCoreTypes order is irrelevant for findByName).
    f.set_core_type("void", 0, meta::TYPE_VOID, false).unwrap();
    f.set_core_type("char", 1, meta::TYPE_INT, true).unwrap();
    f.set_core_type("bool", 1, meta::TYPE_BOOL, false).unwrap();
    f.set_core_type("uint1", 1, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("int1", 1, meta::TYPE_INT, false).unwrap();
    f.set_core_type("int2", 2, meta::TYPE_INT, false).unwrap();
    f.set_core_type("uint2", 2, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("int4", 4, meta::TYPE_INT, false).unwrap();
    f.set_core_type("uint4", 4, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("int8", 8, meta::TYPE_INT, false).unwrap();
    f.set_core_type("uint8", 8, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("float4", 4, meta::TYPE_FLOAT, false).unwrap();
    f.set_core_type("float8", 8, meta::TYPE_FLOAT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

/// A 4-byte / word-size-1 default data space (the common little-endian 32-bit
/// data organization).
fn org() -> DataOrg {
    DataOrg { addr_size: 4, word_size: 1 }
}

/// Collect the lexer's token kinds for `input` (each kind only, value checked
/// separately where it matters).
fn lex_kinds(input: &str) -> Vec<u32> {
    let mut lx = GrammarLexer::new(4096);
    lx.push_file("t", input.as_bytes().to_vec());
    let mut out = Vec::new();
    loop {
        let mut t = GrammarToken::new();
        lx.get_next_token(&mut t).unwrap();
        out.push(t.get_type());
        if t.get_type() == tok::ENDOFFILE || t.get_type() == tok::BADTOKEN {
            break;
        }
    }
    out
}

// ===========================================================================
// Lexer (GrammarLexer / GrammarToken)
// ===========================================================================

#[test]
fn lex_punctuation_and_identifiers() {
    let k = lex_kinds("int4 *x;");
    assert_eq!(
        k,
        vec![tok::IDENTIFIER, tok::STAR, tok::IDENTIFIER, tok::SEMICOLON, tok::ENDOFFILE]
    );
}

#[test]
fn lex_dotdotdot_and_scoperes() {
    assert_eq!(lex_kinds("..."), vec![tok::DOTDOTDOT, tok::ENDOFFILE]);
    assert_eq!(
        lex_kinds("a::b"),
        vec![tok::IDENTIFIER, tok::SCOPERES, tok::IDENTIFIER, tok::ENDOFFILE]
    );
}

#[test]
fn lex_single_colon_is_not_scoperes() {
    // A lone ':' (struct bitfield separator) resolves to the ':' punctuation.
    assert_eq!(lex_kinds("a : 3"), vec![tok::IDENTIFIER, b':' as u32, tok::INTEGER, tok::ENDOFFILE]);
}

#[test]
fn lex_comments_are_skipped() {
    // `//` to end of line and `/* */` are dropped entirely.
    assert_eq!(lex_kinds("a // comment\nb"), vec![tok::IDENTIFIER, tok::IDENTIFIER, tok::ENDOFFILE]);
    assert_eq!(lex_kinds("a /* c */ b"), vec![tok::IDENTIFIER, tok::IDENTIFIER, tok::ENDOFFILE]);
}

/// Read one token from a lexer (test helper).
fn read_tok(lx: &mut GrammarLexer) -> GrammarToken {
    let mut t = GrammarToken::new();
    lx.get_next_token(&mut t).unwrap();
    t
}

#[test]
fn lex_integer_decimal_hex_octal() {
    let mut lx = GrammarLexer::new(4096);
    lx.push_file("t", b"10 0x14 010 -5".to_vec());
    let a = read_tok(&mut lx);
    assert_eq!(a.get_type(), tok::INTEGER);
    assert_eq!(a.get_integer(), 10);
    let b = read_tok(&mut lx);
    assert_eq!(b.get_integer(), 0x14);
    let c = read_tok(&mut lx);
    assert_eq!(c.get_integer(), 8); // octal 010
    let d = read_tok(&mut lx);
    assert_eq!(d.get_integer() as i64, -5);
}

#[test]
fn lex_char_constant_token_kind() {
    // Char constants resolve to the CHARCONSTANT token.  The *value* decode in
    // the C++ reads `buffer[bufstart..bufend-1]` where the single-quote start
    // state (grammar.cc:393-395) does NOT reset `bufstart` — so the decoded
    // value depends on the prior token's start offset within the (per-line)
    // buffer, a C++ fragility.  No `parse line` / proto payload in the corpus
    // uses a char constant (they appear only in `enum { X = 'c' }`, which is the
    // seam'd newEnum path), so only the token kind is pinned here; the value
    // decode mirrors the C++ algorithm faithfully (see `set_with`).
    let k = lex_kinds("'A'");
    assert_eq!(k, vec![tok::CHARCONSTANT, tok::ENDOFFILE]);
}

#[test]
fn lex_illegal_x_in_number() {
    // `x` is only legal as the 0x hex indicator; `1x` is a lexer syntax error.
    assert_eq!(lex_kinds("1x"), vec![tok::BADTOKEN]);
}

#[test]
fn lex_string_constant_resolves_but_parser_rejects() {
    // The lexer recognizes a string literal token; the parser's lex() rejects it.
    assert_eq!(lex_kinds("\"abc\""), vec![tok::STRINGVAL, tok::ENDOFFILE]);
}

#[test]
fn parse_c_integer_bases() {
    assert_eq!(parse_c_integer(b"0"), 0);
    assert_eq!(parse_c_integer(b"255"), 255);
    assert_eq!(parse_c_integer(b"0xff"), 255);
    assert_eq!(parse_c_integer(b"0XFF"), 255);
    assert_eq!(parse_c_integer(b"0777"), 0o777);
    assert_eq!(parse_c_integer(b"-1") as i64, -1);
}

#[test]
fn parse_c_integer_prefix_edge_cases() {
    // All values cross-checked against `g++ -std=c++11 istringstream >> intb`.
    // Bare `0x`/`0xg`: the `0` is parsed, then the scan stops -> 0.
    assert_eq!(parse_c_integer(b"0x"), 0);
    assert_eq!(parse_c_integer(b"0xg"), 0);
    // No leading digit -> stream fails, value-initialized 0.
    assert_eq!(parse_c_integer(b"z"), 0);
    assert_eq!(parse_c_integer(b"_"), 0);
    assert_eq!(parse_c_integer(b"0_"), 0);
    // Underscore / letter mid-token stops the scan.
    assert_eq!(parse_c_integer(b"1_2_3"), 1);
    assert_eq!(parse_c_integer(b"12_34"), 12);
    // Sign-only -> 0.
    assert_eq!(parse_c_integer(b"+") as i64, 0);
    assert_eq!(parse_c_integer(b"-") as i64, 0);
    assert_eq!(parse_c_integer(b"+5"), 5);
    assert_eq!(parse_c_integer(b"-5z") as i64, -5);
    assert_eq!(parse_c_integer(b"-08") as i64, 0);
    // Positive overflow saturates to i64::MAX; negative to i64::MIN.
    assert_eq!(parse_c_integer(b"0xffffffffffffffff") as i64, i64::MAX);
    assert_eq!(parse_c_integer(b"99999999999999999999") as i64, i64::MAX);
    assert_eq!(parse_c_integer(b"9223372036854775808") as i64, i64::MAX);
    assert_eq!(parse_c_integer(b"-99999999999999999999") as i64, i64::MIN);
    assert_eq!(parse_c_integer(b"-9223372036854775809") as i64, i64::MIN);
    // Exact boundaries are representable, not saturated.
    assert_eq!(parse_c_integer(b"9223372036854775807") as i64, i64::MAX);
    assert_eq!(parse_c_integer(b"-9223372036854775808") as i64, i64::MIN);
}

// ---------------------------------------------------------------------------
// w9-con-grammar — parse_c_integer prefix-parse parity with C++
// `istringstream >> intb` (grammar.cc:1796-1804).  REPAIR of LOSS-124.
//
// The C++ `operator>>` performs a PREFIX parse: it consumes as many valid
// digits in the auto-detected base as it can and STOPS at the first invalid
// char, leaving the rest in the stream.  These tests previously documented a
// divergence (the port full-string-parsed and saturated to i64::MAX); the port
// now matches the C++ leading-prefix scan, so they assert the oracle values.
//
// The lexer's `number` state (grammar.cc:2094-2125 / move_state LexState::Number)
// accepts ASCII letters and `_` as "part of the number", so tokens like `1z`,
// `123abc`, `08`, `5_0` are single INTEGER tokens — these inputs ARE reachable.
//
// C++ oracle (verified with a standalone istringstream program):
//   "1z"     -> 1
//   "123abc" -> 123
//   "08"     -> 0      (octal stops at non-octal digit 8)
//   "5_0"    -> 5
//   "007_0"  -> 7
// ---------------------------------------------------------------------------

#[test]
fn w9_con_grammar_parse_int_letter_suffix() {
    // C++ istringstream prefix-parses "1z" as 1.
    assert_eq!(parse_c_integer(b"1z"), 1, "C++ prefix-parses 1z -> 1");
}

#[test]
fn w9_con_grammar_parse_int_trailing_alpha() {
    assert_eq!(parse_c_integer(b"123abc"), 123, "C++ prefix-parses 123abc -> 123");
}

#[test]
fn w9_con_grammar_parse_int_bad_octal_digit() {
    assert_eq!(parse_c_integer(b"08"), 0, "C++ reads octal 0 then stops at 8 -> 0");
}

#[test]
fn w9_con_grammar_parse_int_underscore() {
    assert_eq!(parse_c_integer(b"5_0"), 5, "C++ prefix-parses 5_0 -> 5");
    assert_eq!(parse_c_integer(b"007_0"), 7, "C++ prefix-parses octal 007_0 -> 7");
}

#[test]
fn w9_con_grammar_array_size_letter_suffix() {
    // End-to-end: `int4 a[1z]` is a valid `int4[1]` in C++ (array size 1).  The
    // lexer accretes `1z` into one NUMBER token; parse_c_integer prefix-parses
    // it to 1, so ArrayModifier::isValid (`arraysize>0`) holds and the declarator
    // is a valid `int4[1]` array.
    let f = factory();
    let (ty, name) = parse_type("int4 a[1z]", &f, org()).expect("C++ accepts int4 a[1z] as int4[1]");
    assert_eq!(ty.get_metatype(), meta::TYPE_ARRAY, "expected an array type");
    assert_eq!(ty.get_size(), 4, "int4[1] is 4 bytes (one int4 element)");
    assert_eq!(name, "a");
}

// ===========================================================================
// parse_protopieces — the testfuncproto / testparamstore path
// ===========================================================================

#[test]
fn proto_simple_two_int_params() {
    // testfuncproto funcproto_register: "void func(int4 a,int4 b);".
    let f = factory();
    let p = parse_protopieces("void func(int4 a,int4 b);", &f, org()).unwrap();
    assert_eq!(p.name, "func");
    assert_eq!(p.outtype.as_ref().unwrap().get_metatype(), meta::TYPE_VOID);
    assert_eq!(p.intypes.len(), 2);
    assert_eq!(p.intypes[0].get_name(), "int4");
    assert_eq!(p.intypes[1].get_name(), "int4");
    assert_eq!(p.innames, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(p.first_var_arg_slot, -1);
}

#[test]
fn proto_mixed_param_types() {
    // testfuncproto funcproto_smallregister.
    let f = factory();
    let p = parse_protopieces("int4 func(char a,int4 b,int2 c,int4 d);", &f, org()).unwrap();
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "int4");
    assert_eq!(p.intypes.len(), 4);
    assert_eq!(p.intypes[0].get_name(), "char");
    assert_eq!(p.intypes[1].get_name(), "int4");
    assert_eq!(p.intypes[2].get_name(), "int2");
    assert_eq!(p.intypes[3].get_name(), "int4");
    assert_eq!(p.innames, vec!["a", "b", "c", "d"]);
}

#[test]
fn proto_pointer_param() {
    // testfuncproto funcproto_stackalign: trailing "int1 *g".
    let f = factory();
    let p =
        parse_protopieces("int4 func(int4 a,int4 b,int4 c,int4 d,int4 e,int2 f,int1 *g);", &f, org())
            .unwrap();
    assert_eq!(p.intypes.len(), 7);
    let g = &p.intypes[6];
    assert_eq!(g.get_metatype(), meta::TYPE_PTR);
    assert_eq!(g.get_ptr_to().unwrap().get_name(), "int1");
    assert_eq!(g.get_size(), org().addr_size);
}

#[test]
fn proto_void_params_become_empty() {
    // "f(void)" -> the lone void parameter is dropped (FunctionModifier ctor).
    let f = factory();
    let p = parse_protopieces("int4 noargs(void);", &f, org()).unwrap();
    assert_eq!(p.intypes.len(), 0);
    assert_eq!(p.innames.len(), 0);
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "int4");
}

#[test]
fn proto_varargs() {
    // datatest noforloop_alias: "extern void printf(char *,...);".
    let f = factory();
    let p = parse_protopieces("extern void printf(char *,...);", &f, org()).unwrap();
    assert_eq!(p.name, "printf");
    assert_eq!(p.intypes.len(), 1);
    assert_eq!(p.intypes[0].get_metatype(), meta::TYPE_PTR);
    assert_eq!(p.intypes[0].get_ptr_to().unwrap().get_name(), "char");
    // firstVarArgSlot = number of fixed params.
    assert_eq!(p.first_var_arg_slot, 1);
}

#[test]
fn proto_double_pointer() {
    // datatest deindirect2: "extern int2 deind26(int4 **ptr,char *nm);".
    let f = factory();
    let p = parse_protopieces("extern int2 deind26(int4 **ptr,char *nm);", &f, org()).unwrap();
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "int2");
    assert_eq!(p.intypes.len(), 2);
    // int4 ** : pointer to pointer to int4.
    let pp = &p.intypes[0];
    assert_eq!(pp.get_metatype(), meta::TYPE_PTR);
    let inner = pp.get_ptr_to().unwrap();
    assert_eq!(inner.get_metatype(), meta::TYPE_PTR);
    assert_eq!(inner.get_ptr_to().unwrap().get_name(), "int4");
    assert_eq!(p.intypes[1].get_metatype(), meta::TYPE_PTR);
}

#[test]
fn proto_pointer_return_type() {
    // datatest deindirect2: "extern int4 *obtainPtr(char *nm);" — pointer return.
    let f = factory();
    let p = parse_protopieces("extern int4 *obtainPtr(char *nm);", &f, org()).unwrap();
    assert_eq!(p.name, "obtainPtr");
    let ret = p.outtype.as_ref().unwrap();
    assert_eq!(ret.get_metatype(), meta::TYPE_PTR);
    assert_eq!(ret.get_ptr_to().unwrap().get_name(), "int4");
}

#[test]
fn proto_namespaced_name() {
    // datatest namespace: "extern int4 a::b::assign_vals(int4 spam);".
    let f = factory();
    let p = parse_protopieces("extern int4 a::b::assign_vals(int4 spam);", &f, org()).unwrap();
    assert_eq!(p.name, "a::b::assign_vals");
    assert_eq!(p.intypes.len(), 1);
    assert_eq!(p.innames, vec!["spam"]);
}

#[test]
fn proto_many_float_int_params() {
    // datatest mixfloatint: "extern float8 dldlll(float8 a,int4 b,float8 c,int4 d,int4 e,int4 f);".
    let f = factory();
    let p = parse_protopieces(
        "extern float8 dldlll(float8 a,int4 b,float8 c,int4 d,int4 e,int4 f);",
        &f,
        org(),
    )
    .unwrap();
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "float8");
    assert_eq!(p.intypes.len(), 6);
    assert_eq!(p.intypes[0].get_name(), "float8");
    assert_eq!(p.intypes[5].get_name(), "int4");
}

#[test]
fn proto_bool_return() {
    // datatest condconst2: "extern bool condmove(int4 cval,char *cptr);".
    let f = factory();
    let p = parse_protopieces("extern bool condmove(int4 cval,char *cptr);", &f, org()).unwrap();
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "bool");
    assert_eq!(p.intypes.len(), 2);
    assert_eq!(p.intypes[1].get_metatype(), meta::TYPE_PTR);
}

#[test]
fn proto_uint1_pointer_param() {
    // datatest ccmp: "extern void ccmp(uint1 *ptr,int4 val);".
    let f = factory();
    let p = parse_protopieces("extern void ccmp(uint1 *ptr,int4 val);", &f, org()).unwrap();
    assert_eq!(p.intypes.len(), 2);
    assert_eq!(p.intypes[0].get_metatype(), meta::TYPE_PTR);
    assert_eq!(p.intypes[0].get_ptr_to().unwrap().get_name(), "uint1");
}

// ===========================================================================
// parse_type — the single-type entry point
// ===========================================================================

#[test]
fn parse_type_plain_named() {
    let f = factory();
    let (ty, name) = parse_type("int4 x", &f, org()).unwrap();
    assert_eq!(ty.get_name(), "int4");
    assert_eq!(name, "x");
}

#[test]
fn parse_type_pointer() {
    let f = factory();
    let (ty, name) = parse_type("char *p", &f, org()).unwrap();
    assert_eq!(ty.get_metatype(), meta::TYPE_PTR);
    assert_eq!(ty.get_ptr_to().unwrap().get_name(), "char");
    assert_eq!(name, "p");
}

#[test]
fn parse_type_array() {
    // direct_declarator '[' assignment_expression ']'.
    let f = factory();
    let (ty, name) = parse_type("int4 a[32]", &f, org()).unwrap();
    assert_eq!(ty.get_metatype(), meta::TYPE_ARRAY);
    assert_eq!(name, "a");
}

#[test]
fn parse_type_pointer_to_array() {
    // datatest offsetarray-style: `int4 (*a)[16]` — pointer to array[16] of int4.
    let f = factory();
    let (ty, name) = parse_type("int4 (*a)[16]", &f, org()).unwrap();
    assert_eq!(name, "a");
    assert_eq!(ty.get_metatype(), meta::TYPE_PTR);
    let arr = ty.get_ptr_to().unwrap();
    assert_eq!(arr.get_metatype(), meta::TYPE_ARRAY);
    assert_eq!(arr.get_size(), 16 * 4); // 16 int4 elements
}

#[test]
fn parse_type_abstract_pointer_to_array() {
    // datatest displayformat-style: `float4 (*a)[1]` parameter (abstract or named).
    let f = factory();
    let (ty, _name) = parse_type("float4 (*)[1]", &f, org()).unwrap();
    assert_eq!(ty.get_metatype(), meta::TYPE_PTR);
    assert_eq!(ty.get_ptr_to().unwrap().get_metatype(), meta::TYPE_ARRAY);
}

#[test]
fn parse_type_abstract_no_name() {
    // A bare type with no identifier (abstract declarator).
    let f = factory();
    let (ty, name) = parse_type("int4", &f, org()).unwrap();
    assert_eq!(ty.get_name(), "int4");
    assert_eq!(name, "");
}

#[test]
fn parse_type_abstract_pointer() {
    let f = factory();
    let (ty, name) = parse_type("int4 *", &f, org()).unwrap();
    assert_eq!(ty.get_metatype(), meta::TYPE_PTR);
    assert_eq!(name, "");
}

// ===========================================================================
// old-struct / old-union / old-enum lookups (the read-only construction half)
// ===========================================================================

#[test]
fn old_struct_lookup_succeeds() {
    // Register a struct named "mystruct" (via the factory's getTypeStruct stub,
    // which is what `parse line struct {...}` interns by name), then parse
    // "extern mystruct returnbig(int4 num);" (datatest retspecial) — the
    // TYPE_NAME path resolves the struct by name.
    let f = factory();
    f.get_type_struct("mystruct").unwrap();
    let p = parse_protopieces("extern mystruct returnbig(int4 num);", &f, org()).unwrap();
    assert_eq!(p.outtype.as_ref().unwrap().get_name(), "mystruct");
    assert_eq!(p.outtype.as_ref().unwrap().get_metatype(), meta::TYPE_STRUCT);
    assert_eq!(p.intypes.len(), 1);
}

#[test]
fn struct_pointer_param_resolves_named_type() {
    // datatest offsetarray: "extern void populate_mystruct(mystruct *ptr);".
    let f = factory();
    f.get_type_struct("mystruct").unwrap();
    let p = parse_protopieces("extern void populate_mystruct(mystruct *ptr);", &f, org()).unwrap();
    assert_eq!(p.intypes.len(), 1);
    assert_eq!(p.intypes[0].get_metatype(), meta::TYPE_PTR);
    assert_eq!(p.intypes[0].get_ptr_to().unwrap().get_name(), "mystruct");
}

// ===========================================================================
// Rejection / error-text paths
// ===========================================================================

#[test]
fn reject_string_constant() {
    // grammar.cc:1300-1303: "Illegal string constant".
    let f = factory();
    let err = parse_protopieces("extern void f(\"oops\");", &f, org()).unwrap_err();
    assert!(err.explain().contains("Illegal string constant"), "got: {}", err.explain());
}

#[test]
fn reject_unknown_qualifier_is_not_a_qualifier() {
    // A bogus declaration with no valid specifier is a syntax error.
    let f = factory();
    let err = parse_protopieces("notatype foo(int4 a);", &f, org()).unwrap_err();
    // "notatype" lexes as an IDENTIFIER, which cannot begin declaration_specifiers.
    assert!(err.explain().contains("Syntax error"), "got: {}", err.explain());
}

#[test]
fn reject_multiple_type_specifiers() {
    // Two TYPE_NAMEs in one specifier list -> "Multiple type specifiers".
    let f = factory();
    let err = parse_protopieces("int4 char f(void);", &f, org()).unwrap_err();
    assert!(err.explain().contains("Multiple type specifiers"), "got: {}", err.explain());
}

#[test]
fn reject_old_struct_not_a_struct() {
    // `struct nosuchstruct` (no body) where the identifier names no struct ->
    // the oldStruct error text (grammar.cc:1087-1094).  The name must lex as an
    // IDENTIFIER (an unknown name), since a registered type lexes as TYPE_NAME
    // and `struct TYPE_NAME` is a plain syntax error.
    let f = factory();
    let err = parse_protopieces("extern struct nosuchstruct f(void);", &f, org()).unwrap_err();
    assert!(
        err.explain().contains("Identifier does not represent a struct as required"),
        "got: {}",
        err.explain()
    );
}

#[test]
fn reject_struct_type_name_is_syntax_error() {
    // `struct int4`: int4 is a TYPE_NAME, so neither `STRUCT IDENTIFIER` nor
    // `STRUCT '{' ... '}'` matches -> syntax error (matches C++).
    let f = factory();
    let err = parse_protopieces("extern struct int4 f(void);", &f, org()).unwrap_err();
    assert!(err.explain().contains("Syntax error"), "got: {}", err.explain());
}

#[test]
fn reject_multiple_storage_specifiers() {
    // TypeDeclarator::isValid throws ParseError("Multiple storage specifiers")
    // (grammar.cc:841-842) when two storage classes coexist; the entry point's
    // `decl->isValid()` call surfaces it.
    let f = factory();
    let err = parse_protopieces("typedef extern int4 x;", &f, org()).unwrap_err();
    assert!(
        err.explain().contains("Multiple storage specifiers"),
        "got: {}",
        err.explain()
    );
}

#[test]
fn reject_incomplete_declaration() {
    // Missing terminating ';' -> syntax error at EOF.
    let f = factory();
    let err = parse_protopieces("void f(int4 a)", &f, org()).unwrap_err();
    assert!(!err.explain().is_empty());
}

#[test]
fn struct_construction_lands_in_factory() {
    // `parse line struct { ... }` now constructs the type and interns it into the
    // factory (the W10 store-write un-seam): the type is findable by name with its
    // computed size (two int4 fields => 8 bytes) and completed (no longer
    // incomplete).
    let f = factory();
    super::parse_c("struct mystruct { int4 a; int4 b; };", &f, org(), |_| {
        panic!("a bare struct is not an extern prototype")
    })
    .expect("struct construction should succeed");
    let ct = f
        .find_by_name("mystruct")
        .unwrap()
        .expect("mystruct should exist in the factory");
    assert_eq!(ct.get_metatype(), meta::TYPE_STRUCT);
    assert_eq!(ct.get_size(), 8);
    assert!(!ct.is_incomplete(), "struct should be complete after field assignment");
}

#[test]
fn enum_construction_lands_in_factory() {
    // `parse line enum { ... }` interns a completed enum: the value->name map is
    // installed so the type is no longer just a stub.  The enum size (and thus the
    // value mask) comes from `setup_sizes` (C++ `enumsize = sizeOfInt`); without it
    // the size-0 mask would collapse all values, exactly as the C++ would.
    let f = factory();
    f.setup_sizes(Some(4), 4, 4);
    super::parse_c("enum mycolor { RED=1, GREEN=2, BLUE };", &f, org(), |_| {
        panic!("a bare enum is not an extern prototype")
    })
    .expect("enum construction should succeed");
    let ct = f
        .find_by_name("mycolor")
        .unwrap()
        .expect("mycolor should exist in the factory");
    assert!(ct.is_enum_type());
}

#[test]
fn typedef_lands_in_factory() {
    // `parse line typedef int4 myint;` creates a typedef interned under the new
    // name.
    let f = factory();
    super::parse_c("typedef int4 myint;", &f, org(), |_| {
        panic!("a typedef is not an extern prototype")
    })
    .expect("typedef should succeed");
    let ct = f
        .find_by_name("myint")
        .unwrap()
        .expect("myint typedef should exist in the factory");
    assert_eq!(ct.get_size(), 4);
}

// ===========================================================================
// Direct CParse driving (the parse-document surface the console will use)
// ===========================================================================

#[test]
fn cparse_declaration_then_eof() {
    let f = factory();
    let mut parser = CParse::new(&f, org(), 4096);
    let ok = parser
        .parse_stream(b"extern int4 f(int4 a);".to_vec(), DocType::Declaration)
        .unwrap();
    assert!(ok);
    let decls = parser.get_result_declarations().unwrap();
    assert_eq!(decls.len(), 1);
    assert!(decls[0].has_property(flags::F_EXTERN));
    assert_eq!(decls[0].get_identifier(), "f");
}

// ===========================================================================
// w9-con-grammar round-2 verifier adversarial tests (parse_c_integer repair,
// LOSS-124).  Ground truth comes from a standalone `g++ -std=c++11`
// `istringstream >> intb` oracle that mirrors GrammarToken::set's integer
// branch (grammar.cc:1796-1804) byte-for-byte.
// ===========================================================================

/// V1: sign applied to a HEX/octal magnitude (the C++ `num_get` applies the
/// leading `-` AFTER base auto-detection, not just to decimal).  Oracle:
///   "-0x1"               -> 0xffffffffffffffff (signed -1)
///   "-0xFFFFFFFFFFFFFFFF" -> 0x8000000000000000 (i64::MIN, neg overflow)
///   "-010"               -> -8 (octal 10 = 8, negated)
#[test]
fn w9_con_grammar_v1_sign_applies_to_nondecimal_base() {
    assert_eq!(parse_c_integer(b"-0x1") as i64, -1);
    assert_eq!(parse_c_integer(b"-0xFFFFFFFFFFFFFFFF"), 0x8000_0000_0000_0000);
    assert_eq!(parse_c_integer(b"-010") as i64, -8);
    // and the magnitude-only i64::MIN boundary is exactly representable.
    assert_eq!(parse_c_integer(b"-9223372036854775808") as i64, i64::MIN);
    // one past it saturates (does not wrap to a positive number).
    assert_eq!(parse_c_integer(b"-9223372036854775809") as i64, i64::MIN);
}

/// V2: positive overflow saturates to i64::MAX and re-tags unsigned — it must
/// NOT wrap (the round-1 full-string parse saturated to i64::MAX too, but the
/// repair must reproduce the per-digit saturation point identically).  Oracle:
///   "0xffffffffffffffff" -> 0x7fffffffffffffff (i64::MAX)
///   "18446744073709551615" -> i64::MAX
#[test]
fn w9_con_grammar_v2_positive_overflow_saturates_no_wrap() {
    assert_eq!(parse_c_integer(b"0xffffffffffffffff") as i64, i64::MAX);
    assert_eq!(parse_c_integer(b"18446744073709551615") as i64, i64::MAX);
    // exact i64::MAX is representable, not saturated-from-above.
    assert_eq!(parse_c_integer(b"9223372036854775807") as i64, i64::MAX);
    // a long hex run well past 64 bits still pins at i64::MAX.
    assert_eq!(parse_c_integer(b"0xfffffffffffffffffff") as i64, i64::MAX);
}

/// V3: leading whitespace is skipped (the C++ `operator>>` skips leading ws);
/// and `0x`-with-no-hex-digit yields 0 (the `0` is parsed, scan stops at `x`).
#[test]
fn w9_con_grammar_v3_whitespace_and_bare_prefixes() {
    assert_eq!(parse_c_integer(b"   42"), 42);
    assert_eq!(parse_c_integer(b"\t7z"), 7);
    assert_eq!(parse_c_integer(b"0x"), 0);
    assert_eq!(parse_c_integer(b"0X"), 0);
    assert_eq!(parse_c_integer(b"0xx"), 0);
    assert_eq!(parse_c_integer(b"0x10g5"), 16);
}

/// V4: end-to-end — an array size whose token prefix-parses to a NEGATIVE int4
/// (via `(int4)` truncation of a >2^31 prefix) is rejected by
/// `ArrayModifier::isValid` (`arraysize>0`), exactly as the C++ does.
/// `4294967296` = 0x1_0000_0000; `(int4)` truncates the low 32 bits -> 0,
/// which is NOT > 0, so the array is invalid.  Distinguishes the repaired
/// prefix-parse path from any "saturate-to-MAX" shortcut.
#[test]
fn w9_con_grammar_v4_array_size_int4_truncation_zero_rejected() {
    let f = factory();
    // low-32-bits of 0x1_0000_0000 is 0 -> arraysize 0 -> invalid.
    let err = parse_type("int4 a[4294967296]", &f, org())
        .expect_err("0x1_0000_0000 truncates to int4 0 -> invalid array");
    assert!(err.explain().to_lowercase().contains("invalid"), "got: {}", err.explain());
    // but 0x1_0000_0001 truncates to 1 -> a valid int4[1].
    let (ty, _name) = parse_type("int4 b[4294967297]", &f, org())
        .expect("0x1_0000_0001 truncates to int4 1 -> valid int4[1]");
    assert_eq!(ty.get_metatype(), meta::TYPE_ARRAY);
    assert_eq!(ty.get_size(), 4);
}

/// V5 (differential): the repaired `parse_c_integer` must agree with the C++
/// `istringstream >> intb` oracle on a 4000-token random corpus drawn from the
/// lexer's number alphabet (digits, A-Z, a-z, `_`, `x`/`X`, leading `-`).
/// The corpus + expected values were generated by a standalone g++ program;
/// any single mismatch fails with the offending token.
#[test]
fn w9_con_grammar_v5_differential_against_cpp_oracle() {
    let corpus = include_str!("w9_oracle_corpus.txt");
    let mut checked = 0usize;
    for line in corpus.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // format: "<len> b0 b1 ... b<len-1>\t<expected-u64-decimal>"
        let (lhs, rhs) = line.split_once('\t').expect("malformed corpus line");
        let mut it = lhs.split_whitespace();
        let len: usize = it.next().unwrap().parse().unwrap();
        let bytes: Vec<u8> = it.map(|t| t.parse::<u32>().unwrap() as u8).collect();
        assert_eq!(bytes.len(), len, "corpus length header mismatch");
        let expected: u64 = rhs.parse().unwrap();
        let got = parse_c_integer(&bytes);
        assert_eq!(
            got,
            expected,
            "parse_c_integer divergence on token {:?} (bytes {:?}): got {} want {}",
            String::from_utf8_lossy(&bytes),
            bytes,
            got,
            expected
        );
        checked += 1;
    }
    assert_eq!(checked, 4000, "expected the full 4000-token oracle corpus");
}

/// V6: lexer index-arithmetic boundary safety (hunt list: signed `int4`
/// `bufend-1`/`bufend-2`/`bufend-bufstart` must not underflow-then-index).
/// A tiny buffer forces the "Line too long" path; a bare `/` at EOF and a
/// `/*`-without-close force the EOF "Incomplete token" resolution; none may
/// panic.
#[test]
fn w9_con_grammar_v6_lexer_boundary_no_panic() {
    // "Line too long": buffersize 4, feed a 10-char identifier.
    let mut lx = GrammarLexer::new(4);
    lx.push_file("t", b"abcdefghij".to_vec());
    let mut t = GrammarToken::new();
    lx.get_next_token(&mut t).unwrap();
    assert_eq!(t.get_type(), tok::BADTOKEN);
    // Unterminated C comment at EOF -> "Incomplete token" (state != Start).
    assert_eq!(lex_kinds("/* unclosed"), vec![tok::BADTOKEN]);
    // A lone '/' at EOF: Slash state never resolves -> Incomplete token.
    assert_eq!(lex_kinds("/"), vec![tok::BADTOKEN]);
    // A '..' (two dots) at EOF: Dot2 state -> Incomplete token.
    assert_eq!(lex_kinds(".."), vec![tok::BADTOKEN]);
    // C-comment `*/` exactly at the start of the buffer (bufend==2 guard on the
    // `buffer[bufend-2]=='*'` check): "/**/" is an empty, valid comment.
    assert_eq!(lex_kinds("/**/a"), vec![tok::IDENTIFIER, tok::ENDOFFILE]);
}

/// V7: end-to-end parity for a multi-pointer + array declarator, exercising the
/// pointer-vector reverse() order and the buildType reverse-mod walk together.
/// `int4 **a[3]` is a `int4*[3]` ... actually `int4 (**a)[3]`-style binding is
/// what the grammar yields; we pin the resulting top-level metatype + size and
/// the inner pointer chain so an order regression in `pointer()`/`build_type`
/// would surface.
#[test]
fn w9_con_grammar_v7_pointer_array_modifier_order() {
    let f = factory();
    // `int4 *a[2]`: direct_declarator binds the array tighter than the pointer,
    // so this is an array[2] of (int4*) -> array size = 2 * ptr_size.
    let (ty, name) = parse_type("int4 *a[2]", &f, org()).expect("int4 *a[2] parses");
    assert_eq!(name, "a");
    assert_eq!(ty.get_metatype(), meta::TYPE_ARRAY);
    let elem = ty.get_array_base().expect("array element");
    assert_eq!(elem.get_metatype(), meta::TYPE_PTR, "elements are int4*");
    assert_eq!(elem.get_ptr_to().unwrap().get_name(), "int4");
    assert_eq!(ty.get_size(), 2 * org().addr_size, "two pointer-sized elements");
}
