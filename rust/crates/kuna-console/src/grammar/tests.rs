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
fn struct_construction_is_a_seam() {
    // `parse line struct { ... }` parses but the field-assigning factory
    // orchestrator is the W6 seam: the build errs with the documented message.
    let f = factory();
    let err = parse_protopieces("struct mystruct { int4 a; int4 b; };", &f, org()).unwrap_err();
    assert!(
        err.explain().contains("struct construction") && err.explain().contains("not yet ported"),
        "got: {}",
        err.explain()
    );
}

// ===========================================================================
// Direct CParse driving (the parse-document surface the console will use)
// ===========================================================================

#[test]
fn cparse_declaration_then_eof() {
    let f = factory();
    let mut parser = CParse::new(&f, 4096);
    let ok = parser
        .parse_stream(b"extern int4 f(int4 a);".to_vec(), DocType::Declaration)
        .unwrap();
    assert!(ok);
    let decls = parser.get_result_declarations().unwrap();
    assert_eq!(decls.len(), 1);
    assert!(decls[0].has_property(flags::F_EXTERN));
    assert_eq!(decls[0].get_identifier(), "f");
}
