//! Tests for the `PrintC` c-language token data and the self-contained
//! constant/type formatters (`printc.rs`).
//!
//! These exercise the part of `PrintC` that is faithfully ported now and is
//! print-fidelity-critical: the operator-token precedence/associativity table
//! (which drives `parentheses`), the negate cross-links, the keyword constants,
//! the byte-for-byte integer / float / char token strings, the generic
//! name builders, and the opcode→token dispatch — plus the `resetDefaultsPrintC`
//! defaults including the kuna `arraynotation` default-on.  The RPN/`Emit`
//! driver bodies are the W9 stub (see the module docs) and are not testable
//! until `prettyprint` lands.

use super::*;
use crate::printlanguage::{parentheses, ReversePolish};

fn rpn(tok: &'static OpToken, visited: int4) -> ReversePolish {
    ReversePolish { tok, visited, paren: false, op: None, id: 0, id2: 0 }
}

// ===========================================================================
// VERIFIER (w10-input-prototype) adversarial tests for `declarator_parts`
// (the C-declarator builder transcribing buildTypeStack / pushTypeStart /
// pushTypeEnd). These probe the fragile pure logic: base/anonymous naming,
// pointer-front vs array-tail placement, and the ptr-inside-array
// parenthesisation the RPN ptr_expr/array_expr tokens encode.
// ===========================================================================
mod w10_input_prototype_declarator {
    use crate::dtype::{type_metatype, Datatype, DatatypeKind};
    use crate::printc::declarator_parts;
    use std::rc::Rc;

    fn named(size: i32, m: type_metatype, nm: &str) -> Rc<Datatype> {
        // C++ TypeBase sets displayName = name; mirror that (declarator_parts
        // reads get_display_name() for the base but get_name() for the
        // base-detection loop — both must be the identifier).
        let mut t = Datatype::new_with_align(size, -1, m);
        t.name = nm.to_string();
        t.display_name = nm.to_string();
        Rc::new(t)
    }

    fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
        let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
        Rc::new(p)
    }

    fn array_of(arrayof: Rc<Datatype>, n: i32) -> Rc<Datatype> {
        let elt = arrayof.get_size().max(1);
        let mut a = Datatype::new_with_align(elt * n, -1, type_metatype::TYPE_ARRAY);
        a.kind = DatatypeKind::Array { arrayof, arraysize: n };
        Rc::new(a)
    }

    /// A bare named base type renders `("<name>", "")` with no trailing space
    /// (the caller adds the space before the identifier).  This is the
    /// `noident && typestack.size()==1` -> type_expr_nospace path.
    #[test]
    fn base_type_no_modifier() {
        let t = named(8, type_metatype::TYPE_INT, "int8");
        assert_eq!(
            declarator_parts(&t, crate::printc::RealTypeCtx::OFF),
            ("int8".to_string(), String::new())
        );
    }

    /// A pointer puts a `*` on the front glued to the base name with a single
    /// space: `("twostruct *", "")` -> `twostruct *ptr`.  The `want_space`
    /// caller suppresses a second space after `*`.
    #[test]
    fn pointer_front_star() {
        let base = named(8, type_metatype::TYPE_STRUCT, "twostruct");
        let p = ptr_to(base);
        assert_eq!(
            declarator_parts(&p, crate::printc::RealTypeCtx::OFF),
            ("twostruct *".to_string(), String::new())
        );
    }

    /// An array goes on the tail: `int4[4]` -> `("int4", "[4]")` -> `int4 a[4]`.
    /// No parenthesisation since there is no pending pointer.
    #[test]
    fn array_tail_no_paren() {
        let base = named(4, type_metatype::TYPE_INT, "int4");
        let a = array_of(base, 4);
        assert_eq!(
            declarator_parts(&a, crate::printc::RealTypeCtx::OFF),
            ("int4".to_string(), "[4]".to_string())
        );
    }

    /// DIVERGENCE (verdict F1 / LOSS): the pointer/array parenthesisation in
    /// `declarator_parts` is INVERTED relative to the C++ `pushTypeStart` RPN
    /// (`ptr_expr` vs `array_expr` precedence).
    ///
    /// C++ renders a *pointer-to-array* `int4 (*)[1]` as `int4 (*a)[1]`
    /// (the `*` parenthesised inside the `[]`), and an *array-of-pointer*
    /// `int4 *[1]` as `int4 *a[1]` (no parens).  The Rust `pending_ptr` walk
    /// (base->outer) only wraps when an ARRAY modifier sees a *preceding*
    /// pointer, which is the array-of-pointer ordering — so the two C nestings
    /// come out SWAPPED:
    ///   * pointer-to-array `int4 (*)[1]` -> WRONG `("int4 *", "[1]")`
    ///     (renders `int4 *a[1]`, an array-of-pointer)
    ///   * array-of-pointer `int4 *[1]`   -> WRONG `("int4 (*", ")[1]")`
    ///     (renders `int4 (*a)[1]`, a pointer-to-array)
    /// The doc-comment example on `declarator_parts` (`int4 (*)[1]` ->
    /// `("int4 (*", ")[1]")`) describes the CORRECT C++ output, which the code
    /// does NOT produce.  Latent: `ptrtoarray.xml` declares such params
    /// (`int4 (*a)[1]`) but never emits them as a decompiled function HEADER,
    /// so no passing assertion depends on it today.  This test pins the actual
    /// (buggy) output so a future fix flips it deliberately.
    #[test]
    fn pointer_to_array_paren_inverted_divergence() {
        let base = named(4, type_metatype::TYPE_INT, "int4");
        // pointer-to-array int4 (*)[1] — C++ would give ("int4 (*", ")[1]").
        let pta = ptr_to(array_of(base.clone(), 1));
        assert_eq!(
            declarator_parts(&pta, crate::printc::RealTypeCtx::OFF),
            ("int4 *".to_string(), "[1]".to_string()),
            "BUG: pointer-to-array renders as array-of-pointer (paren inverted)"
        );
        // array-of-pointer int4 *[1] — C++ would give ("int4 *", "[1]").
        let aop = array_of(ptr_to(base), 1);
        assert_eq!(
            declarator_parts(&aop, crate::printc::RealTypeCtx::OFF),
            ("int4 (*".to_string(), ")[1]".to_string()),
            "BUG: array-of-pointer renders as pointer-to-array (paren inverted)"
        );
    }

    /// An anonymous base type with no name falls back to genericTypeName:
    /// TYPE_VOID -> "void", else "undefined<size>".  A pointer to an anonymous
    /// 4-byte type renders `undefined4 *`.
    #[test]
    fn anonymous_base_generic_name() {
        let anon = Rc::new(Datatype::new_with_align(4, -1, type_metatype::TYPE_UNKNOWN));
        let p = ptr_to(anon);
        assert_eq!(
            declarator_parts(&p, crate::printc::RealTypeCtx::OFF),
            ("undefined4 *".to_string(), String::new())
        );
        let v = Rc::new(Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID));
        assert_eq!(
            declarator_parts(&v, crate::printc::RealTypeCtx::OFF),
            ("void".to_string(), String::new())
        );
    }

    /// Pointer-to-pointer keeps both stars on the front, no parens:
    /// `char **` -> `("char **", "")`.
    #[test]
    fn pointer_to_pointer() {
        let base = named(1, type_metatype::TYPE_INT, "char");
        let pp = ptr_to(ptr_to(base));
        assert_eq!(
            declarator_parts(&pp, crate::printc::RealTypeCtx::OFF),
            ("char **".to_string(), String::new())
        );
    }

    /// (kuna DIV-6) With the `realtypes` context ON, a residual TYPE_UNKNOWN base
    /// is relabelled to a real C type by size, and a pointer-to-unknown reads
    /// `void *`.  Sizes with no natural single C type keep `undefined<N>`.
    #[test]
    fn realtypes_relabels_unknown_bases() {
        let on = crate::printc::RealTypeCtx { enabled: true, long_is_8: true };
        let unk = |sz: i32| Rc::new(Datatype::new_with_align(sz, -1, type_metatype::TYPE_UNKNOWN));
        // Scalars: 1->char, 2->unsigned short, 4->unsigned int, 8->unsigned long.
        assert_eq!(declarator_parts(&unk(1), on).0, "char");
        assert_eq!(declarator_parts(&unk(2), on).0, "unsigned short");
        assert_eq!(declarator_parts(&unk(4), on).0, "unsigned int");
        assert_eq!(declarator_parts(&unk(8), on).0, "unsigned long");
        // LLP64: 8-byte unknown reads `unsigned long long` when long is 4 bytes.
        let llp64 = crate::printc::RealTypeCtx { enabled: true, long_is_8: false };
        assert_eq!(declarator_parts(&unk(8), llp64).0, "unsigned long long");
        // Odd size keeps the placeholder.
        assert_eq!(declarator_parts(&unk(3), on).0, "undefined3");
        // Pointer-to-unknown -> `void *`; pointer-to-pointer-to-unknown -> `void **`.
        assert_eq!(declarator_parts(&ptr_to(unk(8)), on), ("void *".to_string(), String::new()));
        assert_eq!(
            declarator_parts(&ptr_to(ptr_to(unk(1))), on),
            ("void **".to_string(), String::new())
        );
        // Array of unknown -> `char [8]` (element relabelled, count preserved).
        assert_eq!(declarator_parts(&array_of(unk(1), 8), on), ("char".to_string(), "[8]".to_string()));
        // Gate OFF leaves the placeholder untouched.
        assert_eq!(declarator_parts(&unk(4), crate::printc::RealTypeCtx::OFF).0, "undefined4");
    }
}

// ===========================================================================
// (kuna decompile-project) `doc_type_definitions` pure renderers —
// `compose_type_body` / `compose_enum_body` / `render_type_definitions` /
// `sanitize_type_name` over hand-built Datatypes + `RealTypeCtx::OFF` (the
// same pattern as `w10_input_prototype_declarator` above).
// ===========================================================================
mod decompile_project_type_render {
    use crate::dtype::{flags, type_metatype, Datatype, DatatypeKind, TypeField};
    use crate::printc::{
        compose_enum_body, compose_type_body, render_type_definitions, sanitize_type_name,
        RealTypeCtx,
    };
    use std::rc::Rc;

    fn named(size: i32, m: type_metatype, nm: &str) -> Rc<Datatype> {
        let mut t = Datatype::new_with_align(size, -1, m);
        t.name = nm.to_string();
        t.display_name = nm.to_string();
        t.id = Datatype::hash_name(nm);
        Rc::new(t)
    }

    fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
        let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
        Rc::new(p)
    }

    fn array_of(arrayof: Rc<Datatype>, n: i32) -> Rc<Datatype> {
        let elt = arrayof.get_size().max(1);
        let mut a = Datatype::new_with_align(elt * n, -1, type_metatype::TYPE_ARRAY);
        a.kind = DatatypeKind::Array { arrayof, arraysize: n };
        Rc::new(a)
    }

    /// A complete named struct with the given (offset, name, type) fields and
    /// total size.
    fn struct_of(nm: &str, size: i32, fields: &[(i32, &str, Rc<Datatype>)]) -> Rc<Datatype> {
        let mut s = Datatype::new_with_align(size, -1, type_metatype::TYPE_STRUCT);
        s.name = nm.to_string();
        s.display_name = nm.to_string();
        s.id = Datatype::hash_name(nm);
        s.kind = DatatypeKind::Struct {
            field: fields
                .iter()
                .enumerate()
                .map(|(i, (off, fnm, ft))| TypeField::new(i as i32, *off, *fnm, Rc::clone(ft)))
                .collect(),
            bitfield: vec![],
        };
        Rc::new(s)
    }

    fn make_enum(nm: &str, size: i32, m: type_metatype, entries: &[(u64, &str)]) -> Rc<Datatype> {
        let mut dt = Datatype::new_with_align(size, size, m);
        dt.name = nm.to_string();
        dt.display_name = nm.to_string();
        dt.id = Datatype::hash_name(nm);
        dt.flags |= flags::enumtype;
        let mut namemap = std::collections::BTreeMap::new();
        for (v, n) in entries {
            namemap.insert(*v, (*n).to_string());
        }
        dt.kind = DatatypeKind::Enum { namemap };
        Rc::new(dt)
    }

    /// Struct body with a scalar, a pointer, an array, an interior padding gap
    /// AND trailing padding: every member/padding shape at once.
    #[test]
    fn struct_body_scalar_pointer_array_gap_trailing_pad() {
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let ch = named(1, type_metatype::TYPE_INT, "char");
        // struct mystruct { int4 a@0; <gap 4>; char *p@8; int4 arr[4]@16;
        //                   <trailing pad 8> } size 40.
        let s = struct_of(
            "mystruct",
            40,
            &[
                (0, "a", Rc::clone(&i4)),
                (8, "p", ptr_to(ch)),
                (16, "arr", array_of(i4, 4)),
            ],
        );
        let body = compose_type_body(&s, "mystruct", RealTypeCtx::OFF);
        assert_eq!(
            body,
            "struct mystruct {\n\
             \x20   int4 a;\n\
             \x20   undefined1 _pad4[4];\n\
             \x20   char *p;\n\
             \x20   int4 arr[4];\n\
             \x20   undefined1 _pad20[8];\n\
             };\n",
            "struct body: scalar + gap pad + glued pointer + array tail + trailing pad"
        );
    }

    /// Union body: every field at offset 0, NO padding members.
    #[test]
    fn union_body_no_padding() {
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let f8 = named(8, type_metatype::TYPE_FLOAT, "float8");
        let mut u = Datatype::new_with_align(8, -1, type_metatype::TYPE_UNION);
        u.name = "myunion".to_string();
        u.display_name = "myunion".to_string();
        u.id = Datatype::hash_name("myunion");
        u.kind = DatatypeKind::Union {
            field: vec![
                TypeField::new(0, 0, "as_int", i4),
                TypeField::new(1, 0, "as_float", f8),
            ],
        };
        let u = Rc::new(u);
        let body = compose_type_body(&u, "myunion", RealTypeCtx::OFF);
        assert_eq!(
            body,
            "union myunion {\n\
             \x20   int4 as_int;\n\
             \x20   float8 as_float;\n\
             };\n"
        );
        assert!(!body.contains("_pad"), "unions carry no padding members");
    }

    /// Enum values: the TYPE_INT (decode-time TYPE_ENUM_INT) form prints
    /// SIGNED decimal sign-extended by the enum size; the TYPE_UINT form hex.
    #[test]
    fn enum_signed_and_unsigned_value_forms() {
        // Signed 4-byte enum: 0xffffffff sign-extends to -1.
        let se = make_enum(
            "serrs",
            4,
            type_metatype::TYPE_INT,
            &[(0, "OK"), (0xffff_ffff, "FAIL")],
        );
        let body = compose_enum_body(&se, "serrs");
        assert_eq!(
            body,
            "typedef enum serrs {\n\
             \x20   OK = 0,\n\
             \x20   FAIL = -1\n\
             } serrs;\n"
        );
        // Unsigned 4-byte enum: hex values.
        let ue = make_enum(
            "uflags",
            4,
            type_metatype::TYPE_UINT,
            &[(0x1, "LO"), (0x80, "HI")],
        );
        let body = compose_enum_body(&ue, "uflags");
        assert_eq!(
            body,
            "typedef enum uflags {\n\
             \x20   LO = 0x1,\n\
             \x20   HI = 0x80\n\
             } uflags;\n"
        );
    }

    /// A typedef renders through the declarator builder; a plain scalar
    /// typedef and a pointer typedef both lay out correctly.
    #[test]
    fn typedef_renders_via_declarator() {
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let td = {
            let mut t = (*i4).clone();
            t.name = "myint".to_string();
            t.display_name = "myint".to_string();
            t.id = Datatype::hash_name("myint");
            t.typedef_imm = Some(Rc::clone(&i4));
            Rc::new(t)
        };
        let out = render_type_definitions(&[Rc::clone(&i4), Rc::clone(&td)], RealTypeCtx::OFF);
        assert!(
            out.contains("typedef int4 myint;\n"),
            "scalar typedef line missing:\n{out}"
        );
        // Pointer typedef: `typedef char *mystr;` (the `*` glues to the name).
        let ch = named(1, type_metatype::TYPE_INT, "char");
        let pstr = ptr_to(ch);
        let tdp = {
            let mut t = (*pstr).clone();
            t.name = "mystr".to_string();
            t.display_name = "mystr".to_string();
            t.id = Datatype::hash_name("mystr");
            t.typedef_imm = Some(Rc::clone(&pstr));
            Rc::new(t)
        };
        let out = render_type_definitions(&[tdp], RealTypeCtx::OFF);
        assert!(
            out.contains("typedef char *mystr;\n"),
            "pointer typedef line missing:\n{out}"
        );
    }

    /// An INCOMPLETE struct emits ONLY the forward declaration, annotated
    /// `/* opaque */` — never a body.  A complete struct gets forward decl
    /// (un-annotated) FIRST, then its body.
    #[test]
    fn incomplete_struct_forward_decl_only() {
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let complete = struct_of("solid", 4, &[(0, "a", i4)]);
        let opaque = {
            let mut s = Datatype::new_with_align(1, -1, type_metatype::TYPE_STRUCT);
            s.name = "mystery".to_string();
            s.display_name = "mystery".to_string();
            s.id = Datatype::hash_name("mystery");
            s.flags |= flags::type_incomplete;
            s.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
            Rc::new(s)
        };
        let out =
            render_type_definitions(&[Rc::clone(&opaque), Rc::clone(&complete)], RealTypeCtx::OFF);
        assert!(
            out.contains("typedef struct mystery mystery; /* opaque */\n"),
            "opaque forward decl missing:\n{out}"
        );
        assert!(!out.contains("struct mystery {"), "opaque struct must have NO body:\n{out}");
        assert!(
            out.contains("typedef struct solid solid;\n"),
            "complete struct forward decl missing (and must not be /* opaque */):\n{out}"
        );
        assert!(out.contains("struct solid {"), "complete struct body missing:\n{out}");
        // Forward-decl block precedes ALL bodies (the self-reference guarantee).
        let fwd_pos = out.find("typedef struct solid").unwrap();
        let body_pos = out.find("struct solid {").unwrap();
        assert!(fwd_pos < body_pos, "forward declarations must precede bodies:\n{out}");
    }

    /// Name sanitisation: non-identifier characters become `_` (with the
    /// `/* renamed from … */` note); a digit-first name gains a `_` prefix.
    #[test]
    fn sanitize_type_names() {
        assert_eq!(sanitize_type_name("good_Name2"), "good_Name2");
        assert!(matches!(
            sanitize_type_name("good_Name2"),
            std::borrow::Cow::Borrowed(_)
        ));
        assert_eq!(sanitize_type_name("bad-name$1"), "bad_name_1");
        assert_eq!(sanitize_type_name("9lives"), "_9lives");
        assert_eq!(sanitize_type_name("std::vector<int>"), "std__vector_int_");

        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let s = struct_of("bad-name$1", 4, &[(0, "a", i4)]);
        let out = render_type_definitions(&[s], RealTypeCtx::OFF);
        assert!(
            out.contains(
                "typedef struct bad_name_1 bad_name_1; /* renamed from \"bad-name$1\" */"
            ),
            "renamed forward decl missing:\n{out}"
        );
        assert!(out.contains("struct bad_name_1 {"), "renamed body missing:\n{out}");
    }

    /// Duplicate type names: the FIRST definition wins; a later same-named
    /// complete type emits only the `/* duplicate type name skipped */` note.
    #[test]
    fn duplicate_type_name_first_wins() {
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let i8t = named(8, type_metatype::TYPE_INT, "int8");
        let first = struct_of("dup", 4, &[(0, "a", i4)]);
        let second = struct_of("dup", 8, &[(0, "b", i8t)]);
        let out = render_type_definitions(&[first, second], RealTypeCtx::OFF);
        assert_eq!(
            out.matches("typedef struct dup dup;").count(),
            1,
            "exactly one forward decl for the shared tag:\n{out}"
        );
        assert_eq!(
            out.matches("struct dup {").count(),
            1,
            "exactly one body for the shared tag:\n{out}"
        );
        assert!(out.contains("int4 a;"), "the FIRST definition's body wins:\n{out}");
        assert!(!out.contains("int8 b;"), "the second definition must be skipped:\n{out}");
        assert!(
            out.contains("/* duplicate type name skipped: dup */"),
            "duplicate note missing:\n{out}"
        );
    }

    /// Core types and unnamed types never render; an enum name colliding with
    /// a struct tag is skipped as a duplicate (the tag's forward typedef
    /// already claims the identifier).
    #[test]
    fn core_unnamed_and_cross_kind_collisions_skipped() {
        // A core type (flagged) is skipped even though it is a composite.
        let i4 = named(4, type_metatype::TYPE_INT, "int4");
        let core_struct = {
            let s = struct_of("sys", 4, &[(0, "a", Rc::clone(&i4))]);
            let mut c = (*s).clone();
            c.flags |= flags::coretype;
            Rc::new(c)
        };
        // An unnamed struct is skipped.
        let anon = {
            let mut s = Datatype::new_with_align(4, -1, type_metatype::TYPE_STRUCT);
            s.kind = DatatypeKind::Struct {
                field: vec![TypeField::new(0, 0, "a", Rc::clone(&i4))],
                bitfield: vec![],
            };
            Rc::new(s)
        };
        let named_struct = struct_of("tagged", 4, &[(0, "a", Rc::clone(&i4))]);
        let colliding_enum = make_enum("tagged", 4, type_metatype::TYPE_UINT, &[(1, "ONE")]);
        let out = render_type_definitions(
            &[core_struct, anon, Rc::clone(&named_struct), colliding_enum],
            RealTypeCtx::OFF,
        );
        assert!(!out.contains("sys"), "core types must be skipped:\n{out}");
        assert!(out.contains("struct tagged {"), "named struct body missing:\n{out}");
        assert!(
            out.contains("/* duplicate type name skipped: tagged */"),
            "enum colliding with a struct tag must be skipped as a duplicate:\n{out}"
        );
        assert!(!out.contains("typedef enum tagged"), "colliding enum must not define:\n{out}");
    }
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
    // (kuna) DIV-35: `out OP= y` compound assignments render by default
    // (upstream ships the flag off and never consumed it; `option inplaceops
    // off` restores the `out = out OP y` form).
    assert!(o.inplace_ops);
    assert!(!o.nocasts);
    // (kuna) DIV-34: zero pointer constants render as NULL (upstream
    // option_NULL default off; `option nullprinting off` restores).
    assert!(o.null);
    assert!(!o.unplaced);
    // (kuna) DIV-2 default-on: &base[index] for standalone PTRADD (GH-558).
    assert!(o.array_notation);
    // (kuna) DIV-33: no blank line between the prototype and `{` (upstream
    // skip_line; `option braceformat function skip` restores it).
    assert_eq!(o.brace_func, BraceStyle::NextLine);
    assert_eq!(o.brace_ifelse, BraceStyle::SameLine);
    assert_eq!(o.brace_loop, BraceStyle::SameLine);
    assert_eq!(o.brace_switch, BraceStyle::SameLine);
}

#[test]
fn option_setters() {
    let mut o = PrintCOptions::new();
    o.set_null_printing(false);
    o.set_inplace_ops(false);
    o.set_no_cast_printing(true);
    o.set_array_notation(false);
    assert!(!o.null);
    assert!(!o.inplace_ops);
    assert!(o.nocasts);
    assert!(!o.array_notation());
    o.set_brace_format_function(BraceStyle::SkipLine);
    assert_eq!(o.brace_func, BraceStyle::SkipLine);
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

/// The signed-integer format-resolution layer the Convert B1 substrate feeds: a
/// signed (`TYPE_INT` read-facing) equated constant must negate two's-complement
/// under the equate's forced display format.  C++ `pushConstant` passes
/// `sign=true` for `TYPE_INT` (`printc.cc:1818-1836`), so the equated `int4` value
/// `0xfffffe00` with a forced `dec` format renders `-512`, and the forced
/// `oct`/`bin` formats negate likewise (printc.cc:1381-1391).  This pins the
/// generic `resolve_integer_format`/`format_integer_token` free functions (NOT the
/// reserved `printc.rs` render dispatch — see the B3 stub in PROGRESS.md); generic
/// over the value/format, no convert-specific constants baked in.
#[test]
fn resolve_integer_signed_equate_negates_under_forced_format() {
    // 0xfffffe00 as int4 == -512: dec → (negsign, 512, FORCE_DEC).
    let (neg, val, fmt) =
        resolve_integer_format(0xffff_fe00, 4, true, display_format::FORCE_DEC, false, false);
    assert!(neg);
    assert_eq!(val, 512);
    assert_eq!(fmt, display_format::FORCE_DEC);
    assert_eq!(format_integer_token(neg, val, fmt, 4, false, false, false, ""), "-512");

    // Same value forced to octal: -0333 (512 == 0o1000... actually 512 dec, octal of
    // the magnitude).  The format is honored, the sign negated.
    let (neg, val, fmt) =
        resolve_integer_format(0xffff_fe00, 4, true, display_format::FORCE_OCT, false, false);
    assert!(neg);
    assert_eq!(format_integer_token(neg, val, fmt, 4, false, false, false, ""), "-01000");

    // An UNSIGNED (`TYPE_UINT`) read-facing constant does NOT negate: the same bits
    // print as the full unsigned value (4294966784).
    let (neg, val, fmt) =
        resolve_integer_format(0xffff_fe00, 4, false, display_format::FORCE_DEC, false, false);
    assert!(!neg);
    assert_eq!(val, 0xffff_fe00);
    assert_eq!(fmt, display_format::FORCE_DEC);

    // sign is suppressed for a `force_char` equate (printc.cc:1381): no negation.
    let (neg, _val, _fmt) =
        resolve_integer_format(0xffff_fe00, 4, true, display_format::FORCE_CHAR, false, false);
    assert!(!neg);
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
    // FLOAT_INT2FLOAT has a hand-written override (PrintC::opFloatInt2Float,
    // printc.cc:850 — the `(floatN)input` cast with INT_ZEXT absorption), so it
    // is Custom and dispatched by the `op_float_int2float_ir` arm of op_push_ir.
    assert!(matches!(op_emit_kind(CPUI_FLOAT_INT2FLOAT), OpEmitKind::Custom));
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
// braces driven through the Emit primitives.  The body is the W9-emit stub.
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

// ---------------------------------------------------------------------------
// The PrintLanguage RPN driver (push_op / push_atom / op_binary / op_unary /
// emit_op / emit_atom / parentheses), realized in printc.rs.  These drive the
// real `EmitNoMarkup` back-end and assert byte-faithful token emission against
// the C++ `emitOp`/`emitAtom`/`parentheses` logic (printlanguage.cc:129-580),
// independent of the stubbed IR.
// ---------------------------------------------------------------------------

use crate::printlanguage::{Atom as PlAtom, SyntaxHighlight as PlHl, TagType as PlTag};

/// A variable-name leaf atom.
fn var_atom(name: &str) -> PlAtom {
    PlAtom::syntax(name, PlTag::VarToken, PlHl::var_color)
}
/// A constant-syntax leaf atom (a literal already rendered to its string).
fn const_atom(name: &str) -> PlAtom {
    PlAtom::syntax(name, PlTag::Syntax, PlHl::const_color)
}

/// Drive `f` on a fresh PrintC and return the emitted text.
fn emit_expr<F: FnOnce(&mut PrintC)>(f: F) -> String {
    let mut p = PrintC::new();
    p.set_output_stream();
    f(&mut p);
    p.emit_mut().output_str().to_string()
}

#[test]
fn rpn_single_binary() {
    // a + b  -> "a + b" (binary spacing == 1 around the operator).
    let out = emit_expr(|p| {
        p.op_binary(&tokens::BINARY_PLUS, None, &var_atom("a"), &var_atom("b"));
    });
    assert_eq!(out, "a + b", "binary plus: {out:?}");
    // The stack must be fully drained.
}

#[test]
fn rpn_assignment_form() {
    // x = a  -> "x = a".
    let out = emit_expr(|p| {
        p.op_binary(&tokens::ASSIGNMENT, None, &var_atom("x"), &var_atom("a"));
    });
    assert_eq!(out, "x = a", "assignment: {out:?}");
}

#[test]
fn rpn_unary_prefix_no_space() {
    // -a  -> "-a" (unary minus spacing == 0).
    let out = emit_expr(|p| {
        p.op_unary(&tokens::UNARY_MINUS, None, &var_atom("a"));
    });
    assert_eq!(out, "-a", "unary minus: {out:?}");
    // ~a -> "~a".
    let out = emit_expr(|p| {
        p.op_unary(&tokens::BITWISE_NOT, None, &var_atom("a"));
    });
    assert_eq!(out, "~a", "bitwise not: {out:?}");
}

#[test]
fn rpn_precedence_forces_parens() {
    // a * (b + c): push MULTIPLY(a, <subexpr b+c>).  The lower-precedence '+'
    // as the right operand of '*' must be parenthesized.
    let out = emit_expr(|p| {
        // Build manually: push '*', push 'a', then a nested '+' subexpression.
        p.push_op(&tokens::MULTIPLY, None);
        p.push_atom(&var_atom("a"));
        // Now the right operand is a '+' expression; push it as a nested op.
        p.push_op(&tokens::BINARY_PLUS, None);
        p.push_atom(&var_atom("b"));
        p.push_atom(&var_atom("c"));
    });
    assert_eq!(out, "a * (b + c)", "precedence parens: {out:?}");
}

#[test]
fn rpn_no_parens_when_higher_precedence_nested() {
    // a + b * c: the higher-precedence '*' on the right needs no parens.
    let out = emit_expr(|p| {
        p.push_op(&tokens::BINARY_PLUS, None);
        p.push_atom(&var_atom("a"));
        p.push_op(&tokens::MULTIPLY, None);
        p.push_atom(&var_atom("b"));
        p.push_atom(&var_atom("c"));
    });
    assert_eq!(out, "a + b * c", "no parens for higher prec: {out:?}");
}

#[test]
fn rpn_associative_no_parens() {
    // a + b + c: '+' is associative; nesting the same token on the right
    // (top==op2 && associative) prints without parens.
    let out = emit_expr(|p| {
        p.push_op(&tokens::BINARY_PLUS, None);
        p.push_atom(&var_atom("a"));
        p.push_op(&tokens::BINARY_PLUS, None);
        p.push_atom(&var_atom("b"));
        p.push_atom(&var_atom("c"));
    });
    assert_eq!(out, "a + b + c", "associative: {out:?}");
}

#[test]
fn rpn_negate_token_flips_comparison() {
    // negatetoken modifier flips '<' to '>='.
    let out = emit_expr(|p| {
        p.context.set_mod(crate::printlanguage::modifiers::NEGATETOKEN);
        p.op_binary(&tokens::LESS_THAN, None, &var_atom("a"), &var_atom("b"));
    });
    assert_eq!(out, "a >= b", "negate flip: {out:?}");
}

#[test]
fn rpn_comparison_constant_operand() {
    // dat <= 10  (the boolless target shape) using a const-syntax operand.
    let out = emit_expr(|p| {
        p.op_binary(&tokens::LESS_EQUAL, None, &var_atom("dat_52"), &const_atom("10"));
    });
    assert_eq!(out, "dat_52 <= 10", "comparison: {out:?}");
}

#[test]
fn rpn_stack_drains_to_empty() {
    let mut p = PrintC::new();
    p.set_output_stream();
    p.op_binary(&tokens::BINARY_PLUS, None, &var_atom("a"), &var_atom("b"));
    assert!(p.is_stack_empty(), "RPN stack not drained after a complete expression");
}

// ---------------------------------------------------------------------------
// w10-enum-render VERIFIER adversarial tests.
//
// Target `PrintC::push_enum_constant_ir` — the enum arm of `pushConstant`
// transcribing C++ `PrintC::pushEnumConstant` (printc.cc:1735-1756).  These
// drive the REAL RPN/`EmitNoMarkup` back-end (`emit_expr`) so the assertion is
// the actual rendered C string, then re-derive the expectation from
// `Datatype::get_matches` + the C++ push order:
//   - flag-OR decomposition (`(A|B|C)`, MSB-first name order),
//   - a single matched flag (no `|`, no parens-from-cat),
//   - a value with a numeric remainder no flag covers (fall back to the raw
//     integer literal — `match_name` empty),
//   - bitwise-complement (`~`),
//   - GENERIC over an arbitrary namemap (no hardcoded flag names/values): the
//     same value through two unrelated enums renders each enum's own names.
//
// C++ oracle: `decompiler/cpp/printc.cc:1735-1756`, `type.cc:1537-1586`.
// ---------------------------------------------------------------------------
mod w10_enum_render {
    use super::*;
    use crate::dtype::{flags, type_metatype, Datatype, DatatypeKind};
    use crate::context::{OpId, VarnodeId};
    use std::collections::BTreeMap;

    /// Build an enum (value -> name) the way `TypeEnum` looks after decode:
    /// metatype TYPE_UINT plus the `enumtype` flag (dtype.rs:5244-5247).
    fn make_enum(size: i32, entries: &[(u64, &str)]) -> Datatype {
        let mut dt = Datatype::new_with_align(size, size, type_metatype::TYPE_UINT);
        dt.flags |= flags::enumtype;
        let mut namemap: BTreeMap<u64, String> = BTreeMap::new();
        for (v, n) in entries {
            namemap.insert(*v, (*n).to_string());
        }
        dt.kind = DatatypeKind::Enum { namemap };
        dt
    }

    /// Render a single enum constant through the live RPN/emit driver.
    fn render_enum(ct: &Datatype, val: u64) -> String {
        emit_expr(|p| {
            p.push_enum_constant_ir(ct, val, OpId::default(), VarnodeId::default());
        })
    }

    /// Flag-OR: 0x2c = 0x20|0x8|0x4 over the enum.xml `flags`-style enum must
    /// render `(FLAG_20|FLAG_8|FLAG_4)` — the MSB-first OR the C++
    /// `pushEnumConstant` emits (one `enum_cat` `|` per gap, names in forward
    /// order, the whole thing parenthesised by the surrounding `&`/`==`).  We
    /// assert the bare expression here; the parens come from the parent op.
    #[test]
    fn flag_or_decomposition_msb_first() {
        let e = make_enum(
            8,
            &[(0x4, "FLAG_4"), (0x8, "FLAG_8"), (0x20, "FLAG_20"), (0x100, "FLAG_100")],
        );
        // Cross-check the data layer drives the render (no hardcoding).
        let rep = e.get_matches(0x2c).unwrap();
        assert_eq!(
            rep.match_name,
            vec!["FLAG_20".to_string(), "FLAG_8".to_string(), "FLAG_4".to_string()],
            "get_matches must yield MSB-first names"
        );
        assert!(!rep.complement && rep.shift_amount == 0);
        assert_eq!(render_enum(&e, 0x2c), "FLAG_20|FLAG_8|FLAG_4");
    }

    /// A single matched flag emits exactly one name — no `|`, no `enum_cat`.
    #[test]
    fn single_flag_no_cat() {
        let e = make_enum(4, &[(0x1, "A"), (0x2, "B"), (0x4, "C")]);
        let rep = e.get_matches(0x4).unwrap();
        assert_eq!(rep.match_name, vec!["C".to_string()]);
        assert_eq!(render_enum(&e, 0x4), "C");
    }

    /// A value with a bit no flag covers has NO representation (`match_name`
    /// empty) and falls back to the raw integer literal — NOT a partial
    /// "names + remainder" mix (C++ pushEnumConstant else-branch, printc.cc:1753).
    #[test]
    fn numeric_remainder_falls_back_to_raw_integer() {
        let e = make_enum(4, &[(0x1, "A"), (0x2, "B"), (0x4, "C")]);
        // 0x10 has no covering flag in {1,2,4} within the 4-byte mask.
        let rep = e.get_matches(0x10).unwrap();
        assert!(rep.match_name.is_empty(), "no representation -> empty match_name");
        // The render is the bare integer (default hex/dec format), no flag names.
        let out = render_enum(&e, 0x10);
        assert!(
            !out.contains('A') && !out.contains('B') && !out.contains('C') && !out.contains('|'),
            "remainder must render as a raw integer, got {out:?}"
        );
        assert_eq!(out, "0x10", "raw integer fallback: {out:?}");
    }

    /// The complement path: a value better represented as `~(flags)` emits a
    /// leading `~` (C++ printc.cc:1744-1745).  For enum3 {EIGHT=8} (size 8) the
    /// value `~8` (= 0xfffffffffffffff7) is `~EIGHT`.
    #[test]
    fn complement_emits_tilde() {
        let e = make_enum(8, &[(0, "ZERO"), (1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")]);
        let val = 0x8u64 ^ kuna_base::address::calc_mask(8); // ~8 in the 8-byte mask
        let rep = e.get_matches(val).unwrap();
        assert_eq!(rep.match_name, vec!["EIGHT".to_string()]);
        assert!(rep.complement, "the ~ path must be taken");
        assert_eq!(render_enum(&e, val), "~EIGHT");
    }

    /// NO SPECIAL-CASING: the arm is generic over the namemap.  The SAME value
    /// 0x5 through two unrelated enums renders each enum's own names — there is
    /// no hardcoded `FLAG_*`/value/function dependence anywhere in the arm.
    #[test]
    fn generic_over_namemap_no_hardcoding() {
        let red = make_enum(4, &[(0x1, "RED_LO"), (0x4, "RED_HI")]);
        let blue = make_enum(4, &[(0x1, "BLUE_BIT0"), (0x4, "BLUE_BIT2")]);
        // 0x5 = 0x4 | 0x1 in both, but the names differ entirely.
        assert_eq!(render_enum(&red, 0x5), "RED_HI|RED_LO");
        assert_eq!(render_enum(&blue, 0x5), "BLUE_BIT2|BLUE_BIT0");
    }
}

// ---------------------------------------------------------------------------
// w10-printc-body VERIFIER adversarial tests (round 1).
// Target the most fragile transcription points: the postsurround id2
// mirror-back across the two emit_op calls on one stack entry, deep
// non-associative right-nesting (parens at every level + stack drain order),
// and the op_binary negate-flip fallback divergence (C++ throws; Rust
// unwrap_or(tok)).  See docs/rust-port/reviews/w10-printc-body.md.
// ---------------------------------------------------------------------------

/// Postsurround (function-call `( )`) wrapping a lower-precedence binary
/// subexpression.  This is the only RPN path where one stack entry's `id2` is
/// SET by `emit_op` at `visited==1` (openParen) and then READ at `visited==2`
/// (closeParen).  The Rust port clones the entry for each `emit_op` and mirrors
/// `id2` back onto the live stack top; if that mirror is wrong the close paren
/// would be lost/mismatched.  C++ emitOp: postsurround visited==1 -> openParen,
/// visited==2 -> closeParen(print2,id2) (printlanguage.cc:347-357).
#[test]
fn w10_postsurround_call_id2_roundtrips() {
    // foo(a + b): push FUNCTION_CALL, the func-name leaf, then a nested '+'.
    let out = emit_expr(|p| {
        p.push_op(&tokens::FUNCTION_CALL, None);
        p.push_atom(&var_atom("foo"));
        p.push_op(&tokens::BINARY_PLUS, None);
        p.push_atom(&var_atom("a"));
        p.push_atom(&var_atom("b"));
    });
    assert_eq!(out, "foo(a + b)", "postsurround call: {out:?}");
}

/// Deep right-nested non-associative subtraction.  `BINARY_MINUS` is
/// non-associative (printc.cc:41), so each nested `-` on the right operand must
/// be parenthesized — exercising `parentheses` (equal-precedence,
/// non-associative -> true) at every level and the `push_atom` drain `do/while`
/// loop popping multiple completed entries when the final atom lands.
#[test]
fn w10_deep_nonassoc_right_nesting_parenthesizes_each_level() {
    // a - (b - (c - d))
    let out = emit_expr(|p| {
        p.push_op(&tokens::BINARY_MINUS, None);
        p.push_atom(&var_atom("a"));
        p.push_op(&tokens::BINARY_MINUS, None);
        p.push_atom(&var_atom("b"));
        p.push_op(&tokens::BINARY_MINUS, None);
        p.push_atom(&var_atom("c"));
        p.push_atom(&var_atom("d"));
    });
    assert_eq!(out, "a - (b - (c - d))", "deep nonassoc: {out:?}");
    // When the SAME non-assoc token is the nested operator while the outer
    // minus is still at its first stage (visited==0), C++ `parentheses`
    // (Binary arm: equal precedence + NON-associative -> return true,
    // printlanguage.cc:278-287) DOES parenthesize it.  So this push order
    // (outer minus, then inner minus as its left subexpression) prints
    // `(a - b) - c`, matching the C++ emitter.  (Associativity only suppresses
    // parens for an associative token feeding itself, e.g. `a + b + c`.)
    let out = emit_expr(|p| {
        p.push_op(&tokens::BINARY_MINUS, None);
        p.push_op(&tokens::BINARY_MINUS, None);
        p.push_atom(&var_atom("a"));
        p.push_atom(&var_atom("b"));
        p.push_atom(&var_atom("c"));
    });
    assert_eq!(out, "(a - b) - c", "nested non-assoc minus parenthesizes: {out:?}");
}

/// op_binary negate-flip on a token with NO complement.  C++ `opBinary`
/// (printlanguage.cc:556-561) throws `LowlevelError("Could not find fliptoken")`
/// when `tok->negate == 0` under `negatetoken`.  The Rust port uses
/// `token_negate(tok).unwrap_or(tok)` — it silently keeps the ORIGINAL token
/// (and clears the modifier) rather than aborting.  This test PINS the current
/// Rust behavior so a future restoration to the C++ throw is a visible change;
/// it is the divergence recorded as F1 in the verdict (latent: op_binary is not
/// yet IR-driven, so the path is unreachable in production today).
#[test]
fn w10_negate_flip_no_complement_keeps_token_does_not_panic() {
    use crate::printlanguage::modifiers::NEGATETOKEN;
    // BINARY_PLUS has no negate complement (token_negate -> None).
    assert!(crate::printc::token_negate(&tokens::BINARY_PLUS).is_none());
    let out = emit_expr(|p| {
        p.context.set_mod(NEGATETOKEN);
        // C++ would THROW here; Rust keeps '+' and clears the modifier.
        p.op_binary(&tokens::BINARY_PLUS, None, &var_atom("a"), &var_atom("b"));
    });
    assert_eq!(out, "a + b", "no-complement flip falls back to original token: {out:?}");
    // The modifier must be cleared regardless (matches C++ unsetMod ordering).
    let mut p = PrintC::new();
    p.set_output_stream();
    p.context.set_mod(NEGATETOKEN);
    p.op_binary(&tokens::BINARY_PLUS, None, &var_atom("a"), &var_atom("b"));
    assert!(!p.context.is_set(NEGATETOKEN), "negatetoken modifier not cleared");
}

// ===========================================================================
// VERIFIER tests for `rport/w10-float-family` (in-crate: these reach the
// PRIVATE ported logic `absorb_zext` / `op_float_int2float_ir` / the cast-type
// text, none of which is reachable from an integration test).
//
// Oracle: TypeOpFloatInt2Float::absorbZext (typeop.cc:1874) and
// PrintC::opFloatInt2Float (printc.cc:850).
//
//   const Varnode *vn0 = op->getIn(0);
//   if (vn0->isWritten() && vn0->isImplied()) {
//     const PcodeOp *zextOp = vn0->getDef();
//     if (zextOp->code() == CPUI_INT_ZEXT) return zextOp;
//   }
//   return 0;
//
// The hunt-list fragile spots for this item: (1) the AND of BOTH flags
// (isWritten && isImplied) — dropping either gates the wrong way; (2) the
// opcode equality (must be INT_ZEXT, not INT_SEXT or any other extension);
// (3) the def-null robustness (C++ derefs getDef() unguarded after isWritten,
// the Rust uses `?`); (4) the `(floatN)` cast text for a scalar float base.
// ===========================================================================
mod w10_float_family {
    use super::*;
    use crate::dtype::{type_metatype, Datatype};
    use crate::funcdata::Funcdata;
    use crate::printc::{absorb_zext, declarator_parts};
    use crate::context::{ArchContext, TypeOp};
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_num::opcodes::OpCode;
    use std::rc::Rc;

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn unk(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> super::super::OpId {
        let ram = ram(fd);
        let op = fd.new_op(inputs, Address::new(ram, off));
        fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        op
    }

    /// Build a FLOAT_INT2FLOAT op whose in0 is the (written) output of a
    /// definer op with opcode `def_opc`; the produced varnode's implied flag is
    /// `implied`.  Returns the (i2f op, definer op).
    fn build_i2f_over(
        fd: &mut Funcdata,
        def_opc: OpCode,
        implied: bool,
    ) -> (super::super::OpId, super::super::OpId) {
        let r = ram(fd);
        // The definer op (e.g. INT_ZEXT) producing a written varnode.
        let defop = mk_op(fd, 1, 0x10, def_opc);
        let src = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x200), Some(unk(4)));
        fd.op_set_input(defop, src, 0).unwrap();
        let widened = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x300), Some(unk(8)));
        fd.op_set_output(defop, widened).unwrap();
        // op_set_output marks `widened` written; re-fetch its id (set_def may
        // remap) from the definer's output.
        let widened = fd.obank().get(defop).unwrap().get_out().unwrap();
        if implied {
            fd.vbank_mut().get_mut(widened).unwrap().set_implied();
        }
        // The FLOAT_INT2FLOAT reading `widened` in slot 0.
        let i2f = mk_op(fd, 1, 0x20, OpCode::CPUI_FLOAT_INT2FLOAT);
        fd.op_set_input(i2f, widened, 0).unwrap();
        let out = fd.new_varnode(8, &Address::new(r, 0x400), Some(unk(8)));
        fd.op_set_output(i2f, out).unwrap();
        (i2f, defop)
    }

    /// in0 is an implied, written INT_ZEXT output -> absorb (return the ZEXT).
    #[test]
    fn absorb_zext_positive_implied_written_intzext() {
        let mut fd = build_fd();
        let (i2f, zext) = build_i2f_over(&mut fd, OpCode::CPUI_INT_ZEXT, true);
        assert_eq!(
            absorb_zext(&fd, i2f),
            Some(zext),
            "implied+written INT_ZEXT must be absorbed"
        );
    }

    /// NOT implied (but written INT_ZEXT) -> NO absorption.  C++ requires BOTH
    /// isWritten() AND isImplied(); dropping the implied check is a finding.
    #[test]
    fn absorb_zext_negative_not_implied() {
        let mut fd = build_fd();
        let (i2f, _zext) = build_i2f_over(&mut fd, OpCode::CPUI_INT_ZEXT, false);
        assert_eq!(
            absorb_zext(&fd, i2f),
            None,
            "a written-but-not-implied INT_ZEXT must NOT be absorbed"
        );
    }

    /// Implied + written but the definer is INT_SEXT (sign-extend), not
    /// INT_ZEXT -> NO absorption.  The opcode equality must be exact: a
    /// sign-extension is a different conversion the cast cannot silently absorb.
    #[test]
    fn absorb_zext_negative_wrong_opcode_intsext() {
        let mut fd = build_fd();
        let (i2f, _def) = build_i2f_over(&mut fd, OpCode::CPUI_INT_SEXT, true);
        assert_eq!(
            absorb_zext(&fd, i2f),
            None,
            "an INT_SEXT (not INT_ZEXT) must NOT be absorbed"
        );
    }

    /// in0 is a free (un-written) input varnode -> NO absorption (isWritten()
    /// is false, so getDef() is never reached).  This is also the def-null
    /// robustness boundary: C++ would not deref getDef() here either.
    #[test]
    fn absorb_zext_negative_free_input() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let i2f = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_FLOAT_INT2FLOAT);
        let free_in = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x500), Some(unk(8)));
        // A free varnode marked implied (implied alone must not trigger absorb).
        fd.vbank_mut().get_mut(free_in).unwrap().set_implied();
        fd.op_set_input(i2f, free_in, 0).unwrap();
        let out = fd.new_varnode(8, &Address::new(r, 0x600), Some(unk(8)));
        fd.op_set_output(i2f, out).unwrap();
        assert_eq!(
            absorb_zext(&fd, i2f),
            None,
            "a free (un-written) input must NOT be absorbed even if implied"
        );
    }

    /// The cast-type text for a scalar `floatN` base is exactly its display
    /// name, no modifiers — `push_cast_type` builds `(front,back)` =
    /// `("float8","")` -> the `(float8)` half of the cast.  A trailing `*`/`[]`
    /// here would corrupt the rendered cast.
    #[test]
    fn cast_type_text_scalar_float_is_display_name() {
        let mut t = Datatype::new_with_align(8, -1, type_metatype::TYPE_FLOAT);
        t.name = "float8".to_string();
        t.display_name = "float8".to_string();
        let t = Rc::new(t);
        let (front, back) = declarator_parts(&t, crate::printc::RealTypeCtx::OFF);
        assert_eq!(front, "float8");
        assert_eq!(back, "");
        // float4 likewise (the conversion-source-width family).
        let mut t4 = Datatype::new_with_align(4, -1, type_metatype::TYPE_FLOAT);
        t4.name = "float4".to_string();
        t4.display_name = "float4".to_string();
        let (f4, b4) = declarator_parts(&Rc::new(t4), crate::printc::RealTypeCtx::OFF);
        assert_eq!((f4.as_str(), b4.as_str()), ("float4", ""));
    }
}

// ===========================================================================
// VERIFIER adversarial tests for `rport/w10-printc-cast-render` (round 1).
//
// Oracle: PrintC::opTypeCast (printc.cc:468):
//
//   Datatype *dt = op->getOut()->getHighTypeDefFacing();
//   if (dt->isPointerToArray()) { ... addressof ... }      // STUB (not these)
//   if (!option_nocasts) { pushOp(&typecast,op); pushType(dt); }
//   pushVn(op->getIn(0),op,mods);
//
// shared by opCast / opFloatFloat2Float / opFloatTrunc (printc.hh:332-341).
//
// These tests reach the PRIVATE `op_type_cast_ir` end-to-end (build a real op
// with a typed output varnode, drive the routine, capture the emitted text) so
// they pin the THREE things the diff must get right and nothing else:
//   (1) FAITHFULNESS — the target type spelling comes from the OUTPUT varnode's
//       def-facing type (`op->getOut()->getHighTypeDefFacing()`), NOT a
//       hardcoded / opcode-keyed string: the SAME op renders a different cast
//       text when only its output type changes;
//   (2) PARENTHESIZATION / OPERAND RECURSION — the cast is a `presurround`
//       op-token (prec 2): a lower-precedence operand subexpression (`a + b`)
//       MUST be parenthesized `(T)(a + b)`, an atom MUST NOT (`(T)k`);
//   (3) the `!option_nocasts` gate — with nocasts the `(T)` is suppressed and
//       only the operand flows through.
// The fragile spot here is the type-name plumbing (a hardcode would pass a
// single fixture but fail test (1)'s two-type contrast) and the operand
// precedence (the cast is presurround, not a prefix unary).
// ===========================================================================
mod w10_printc_cast_render {
    use super::*;
    use crate::dtype::{type_metatype, Datatype};
    use crate::funcdata::Funcdata;
    use crate::context::{ArchContext as ContextArch, TypeOp};
    use kuna_base::address::Address;
    use kuna_base::error::{KunaError, KunaResult};
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_num::opcodes::OpCode;
    use kuna_sleigh::globalcontext::ContextInternal;
    use kuna_sleigh::loadimage::LoadImage;
    use kuna_sleigh::sleigh::Sleigh;
    use std::rc::Rc;

    /// A minimal LoadImage so a bare `architecture::Architecture` can be built
    /// (the render `arch` arg is never dereferenced for a constant operand).
    struct DummyImg;
    impl LoadImage for DummyImg {
        fn get_file_name(&self) -> &str {
            "dummy"
        }
        fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
            Err(KunaError::data_unavail("dummy"))
        }
        fn get_arch_type(&self) -> Vec<u8> {
            Vec::new()
        }
        fn adjust_vma(&mut self, _adjust: i64) {}
    }

    fn bare_arch() -> crate::architecture::Architecture {
        crate::architecture::Architecture::new(
            "t",
            Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new())),
        )
    }

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ContextArch::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    /// A named base type whose `display_name` is `nm` (what `(nm)` must spell).
    fn named(nm: &str, size: int4, meta: type_metatype) -> Rc<Datatype> {
        let mut t = Datatype::new_with_align(size, -1, meta);
        t.name = nm.to_string();
        t.display_name = nm.to_string();
        Rc::new(t)
    }

    fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> super::super::OpId {
        let r = ram(fd);
        let op = fd.new_op(inputs, Address::new(r, off));
        fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        op
    }

    /// Build a 1-input cast op (`opc`) whose in0 is a constant `k` (renders as a
    /// literal, deterministic) and whose OUTPUT carries the data-type `out_ty`.
    fn build_cast_over_const(
        fd: &mut Funcdata,
        opc: OpCode,
        k: u64,
        out_ty: Rc<Datatype>,
    ) -> super::super::OpId {
        let r = ram(fd);
        let op = mk_op(fd, 1, 0x20, opc);
        let cst = fd.new_constant(8, k);
        fd.op_set_input(op, cst, 0).unwrap();
        let out = fd.new_varnode(out_ty.get_size(), &Address::new(r, 0x400), Some(out_ty));
        fd.op_set_output(op, out).unwrap();
        op
    }

    /// Drive the PRIVATE `op_type_cast_ir` on a fresh PrintC and return the text.
    /// The render `arch` is a throwaway bare `architecture::Architecture`: the
    /// constant-operand cast path never dereferences it (it only flows to
    /// `push_vn_explicit_ir`, whose constant arm ignores `arch`).
    fn render_cast(fd: &Funcdata, op: super::super::OpId, nocasts: bool) -> String {
        let arch = bare_arch();
        let mut p = PrintC::new();
        p.set_no_cast_printing(nocasts);
        p.set_output_stream();
        p.op_type_cast_ir(fd, &arch, op);
        p.emit_mut().output_str().to_string()
    }

    /// FAITHFULNESS (1a): a `CPUI_CAST` whose output is `int8` renders
    /// `(int8)<operand>` — the `(` `)` typecast token (printc.cc:36) wrapping the
    /// type name from the OUTPUT varnode and the in0 operand.
    #[test]
    fn cast_renders_output_type_int8() {
        let mut fd = build_fd();
        let op = build_cast_over_const(
            &mut fd,
            OpCode::CPUI_CAST,
            0x2a,
            named("int8", 8, type_metatype::TYPE_INT),
        );
        assert_eq!(render_cast(&fd, op, false), "(int8)0x2a", "CAST -> (int8)operand");
    }

    /// FAITHFULNESS (1b) — the ANTI-HARDCODE contrast: the SAME opcode and the
    /// SAME constant operand, but the OUTPUT varnode is `float4` instead of
    /// `int8`, must render `(float4)...`.  If the cast text were keyed on the
    /// opcode (or hardcoded), this would still say `(int8)` — it must not.  This
    /// is the test the `op->getOut()->getHighTypeDefFacing()` plumbing exists to
    /// pass and a special-case would fail.
    #[test]
    fn cast_text_follows_output_type_not_opcode() {
        let mut fd = build_fd();
        let op = build_cast_over_const(
            &mut fd,
            OpCode::CPUI_CAST,
            0x2a,
            named("float4", 4, type_metatype::TYPE_FLOAT),
        );
        let out = render_cast(&fd, op, false);
        assert_eq!(out, "(float4)0x2a", "cast text must come from the OUTPUT type");
        assert!(!out.contains("int8"), "cast text must NOT be a hardcoded/opcode-keyed name");
        // A second, exotic type-name proves it is the varnode display-name verbatim.
        let mut fd2 = build_fd();
        let op2 = build_cast_over_const(
            &mut fd2,
            OpCode::CPUI_CAST,
            0x7,
            named("MyWeirdType", 8, type_metatype::TYPE_UNKNOWN),
        );
        assert_eq!(render_cast(&fd2, op2, false), "(MyWeirdType)7");
    }

    /// FAITHFULNESS (2): all THREE opcodes that route through opTypeCast
    /// (CPUI_CAST / CPUI_FLOAT_FLOAT2FLOAT / CPUI_FLOAT_TRUNC, printc.hh:332-341)
    /// render the SAME cast-notation form `(T)operand` — none of them emits the
    /// functional `OPC(args)`.  (op_emit_kind routes them all to TypeCast; this
    /// confirms the rendered text, not just the table cell.)
    #[test]
    fn all_three_typecast_opcodes_render_cast_notation() {
        for opc in [
            OpCode::CPUI_CAST,
            OpCode::CPUI_FLOAT_FLOAT2FLOAT,
            OpCode::CPUI_FLOAT_TRUNC,
        ] {
            let mut fd = build_fd();
            let op = build_cast_over_const(
                &mut fd,
                opc,
                0x9,
                named("float8", 8, type_metatype::TYPE_FLOAT),
            );
            let out = render_cast(&fd, op, false);
            assert_eq!(out, "(float8)9", "{opc:?} must render (float8)operand");
            // The opcode name must NOT appear as a functional call `OPC(...)`.
            assert!(
                !out.contains("CAST") && !out.contains("FLOAT"),
                "{opc:?} must NOT render the functional OPC(args) form: {out}"
            );
        }
    }

    /// PARENTHESIZATION (3): the cast over a LOWER-precedence operand
    /// subexpression must parenthesize the operand.  `typecast` is a presurround
    /// op-token at precedence 2 (printc.cc:36); an `INT_ADD` (`+`, prec ~19,
    /// lower) operand pushed under it forces `(T)(a + b)`.  Here the operand is a
    /// nested implied INT_ADD whose two inputs are constants, so the operand
    /// renders `0x1 + 0x2`; the cast must wrap it in parentheses.
    #[test]
    fn cast_parenthesizes_lower_precedence_operand() {
        let mut fd = build_fd();
        let r = ram(&fd);
        // out = CAST( add ), add = INT_ADD(1, 2), `add`'s output is implied so
        // pushVn recurses into the INT_ADD (the binary `0x1 + 0x2`).
        let add = mk_op(&mut fd, 2, 0x10, OpCode::CPUI_INT_ADD);
        let c1 = fd.new_constant(8, 1);
        let c2 = fd.new_constant(8, 2);
        fd.op_set_input(add, c1, 0).unwrap();
        fd.op_set_input(add, c2, 1).unwrap();
        let mid = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x300), Some(named("int8", 8, type_metatype::TYPE_INT)));
        fd.op_set_output(add, mid).unwrap();
        let mid = fd.obank().get(add).unwrap().get_out().unwrap();
        fd.vbank_mut().get_mut(mid).unwrap().set_implied();
        let cast = mk_op(&mut fd, 1, 0x20, OpCode::CPUI_CAST);
        fd.op_set_input(cast, mid, 0).unwrap();
        let out = fd.new_varnode(8, &Address::new(r, 0x400), Some(named("float8", 8, type_metatype::TYPE_FLOAT)));
        fd.op_set_output(cast, out).unwrap();
        let rendered = render_cast(&fd, cast, false);
        assert_eq!(
            rendered, "(float8)(1 + 2)",
            "a lower-precedence operand must be parenthesized under the cast"
        );
    }

    /// `option_nocasts` GATE: with nocasts set, the `(T)` half is suppressed and
    /// ONLY the operand prints — the underlying value flows through (printc.cc:479
    /// `if (!option_nocasts)`).  This pins the gate so a port that always emitted
    /// the cast (or always dropped it) is caught.
    #[test]
    fn nocasts_suppresses_the_cast_keeps_operand() {
        let mut fd = build_fd();
        let op = build_cast_over_const(
            &mut fd,
            OpCode::CPUI_CAST,
            0x2a,
            named("int8", 8, type_metatype::TYPE_INT),
        );
        assert_eq!(render_cast(&fd, op, true), "0x2a", "nocasts: only the operand prints");
        // And with casts ON the SAME op DOES wear the cast (control).
        assert_eq!(render_cast(&fd, op, false), "(int8)0x2a");
    }
}
