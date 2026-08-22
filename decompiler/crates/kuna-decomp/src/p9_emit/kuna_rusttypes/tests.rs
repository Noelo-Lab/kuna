//! Tests for the Rust type speller.
//!
//! The interesting content is in the cases where Rust cannot say what C says:
//! widths Rust has no integer for, the text types, and the prefix/postfix
//! inversion of the declarator.

use super::*;
use crate::dtype::DatatypeKind;
use crate::kuna_lang::OutLang;

fn ctx() -> SpellCtx {
    SpellCtx { lang: OutLang::Rust, ..SpellCtx::OFF }
}

fn base(size: int4, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, -1, m))
}

fn named(size: int4, m: type_metatype, nm: &str) -> Rc<Datatype> {
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

fn array_of(arrayof: Rc<Datatype>, n: int4) -> Rc<Datatype> {
    let elt = arrayof.get_size().max(1);
    let mut a = Datatype::new_with_align(elt * n, -1, type_metatype::TYPE_ARRAY);
    a.kind = DatatypeKind::Array { arrayof, arraysize: n };
    Rc::new(a)
}

fn spell(ct: &Rc<Datatype>) -> String {
    RUST_SPELLER.declarator(&ctx(), ct).0
}

#[test]
fn integers_spell_by_width_and_sign() {
    for (sz, s, u) in [(1, "i8", "u8"), (2, "i16", "u16"), (4, "i32", "u32"), (8, "i64", "u64")] {
        assert_eq!(spell(&base(sz, type_metatype::TYPE_INT)), s);
        assert_eq!(spell(&base(sz, type_metatype::TYPE_UINT)), u);
    }
    assert_eq!(spell(&base(4, type_metatype::TYPE_FLOAT)), "f32");
    assert_eq!(spell(&base(8, type_metatype::TYPE_FLOAT)), "f64");
    assert_eq!(spell(&base(1, type_metatype::TYPE_BOOL)), "bool");
}

/// C spells an un-inferred 3-byte value `undefined3`. Rust has no 3-byte
/// integer, and `[u8; 3]` names the storage exactly rather than rounding to a
/// width the value does not have.
#[test]
fn widths_rust_cannot_name_become_byte_arrays() {
    for sz in [3, 5, 6, 7] {
        assert_eq!(spell(&base(sz, type_metatype::TYPE_UNKNOWN)), format!("[u8; {sz}]"));
    }
    // x87 long double and the 16-byte float: no Rust type, so storage-exact.
    assert_eq!(spell(&base(10, type_metatype::TYPE_FLOAT)), "[u8; 10]");
    assert_eq!(spell(&base(16, type_metatype::TYPE_FLOAT)), "[u8; 16]");
}

/// Sign is genuinely unknown for an un-inferred value, so unsigned -- the same
/// judgement the C `realtypes` relabel makes.
#[test]
fn unknown_widths_rust_can_name_are_unsigned() {
    assert_eq!(spell(&base(1, type_metatype::TYPE_UNKNOWN)), "u8");
    assert_eq!(spell(&base(4, type_metatype::TYPE_UNKNOWN)), "u32");
    assert_eq!(spell(&base(8, type_metatype::TYPE_UNKNOWN)), "u64");
}

/// A Rust `char` is a 4-byte Unicode scalar value with a validity invariant. A
/// decompiled byte is not one, and spelling it `char` would be a claim the
/// recovery cannot support.
#[test]
fn text_types_spell_as_unsigned_integers_not_char() {
    let mut c = Datatype::new_with_align(1, -1, type_metatype::TYPE_INT);
    c.name = "char".to_string();
    c.display_name = "char".to_string();
    c.flags |= crate::dtype::flags::chartype;
    let c = Rc::new(c);
    assert!(c.is_char_print());
    assert_eq!(spell(&c), "u8");
}

/// The inversion that makes `TypeSpeller::declarator` return a String rather
/// than C's `(front, back)` pair: C wraps the identifier, Rust prefixes it.
#[test]
fn pointers_and_arrays_are_pure_prefixes() {
    let i32_ = named(4, type_metatype::TYPE_INT, "int4");
    assert_eq!(spell(&ptr_to(i32_.clone())), "*mut i32");
    assert_eq!(spell(&array_of(i32_.clone(), 4)), "[i32; 4]");
    // C renders this `int4 (*a)[1]`; the parenthesisation is a C declarator
    // artifact with no Rust counterpart.
    assert_eq!(spell(&ptr_to(array_of(i32_.clone(), 1))), "*mut [i32; 1]");
    assert_eq!(spell(&array_of(ptr_to(i32_.clone()), 2)), "[*mut i32; 2]");
    assert_eq!(spell(&ptr_to(ptr_to(i32_))), "*mut *mut i32");
    // `back` is always empty, which is the contract a prefix language keeps.
    let (_, back) = RUST_SPELLER.declarator(&ctx(), &ptr_to(base(4, type_metatype::TYPE_INT)));
    assert!(back.is_empty());
}

#[test]
fn void_is_the_unit_type() {
    assert_eq!(spell(&base(0, type_metatype::TYPE_VOID)), "()");
    assert_eq!(spell(&ptr_to(base(0, type_metatype::TYPE_VOID))), "*mut ()");
}

/// `code` only reaches the output through a pointer, and the recovered prototype
/// is not carried down this path -- an `fn()` of the wrong arity would be worse
/// than an honest opaque pointer.
#[test]
fn code_is_an_opaque_pointer() {
    assert_eq!(spell(&base(1, type_metatype::TYPE_CODE)), "*const ()");
}

/// A recovered aggregate name can carry C spellings Rust identifiers forbid.
#[test]
fn aggregate_names_are_sanitized() {
    let s = named(8, type_metatype::TYPE_STRUCT, "std::pair<int,int>");
    assert_eq!(spell(&s), "std__pair_int_int_");
    let leading_digit = named(4, type_metatype::TYPE_STRUCT, "2big");
    assert_eq!(spell(&leading_digit), "_2big");
}

/// A recovered type graph can be cyclic through a struct pointer; the walk must
/// terminate rather than recurse to the stack limit.
#[test]
fn deep_pointer_chains_terminate() {
    let mut t = named(4, type_metatype::TYPE_INT, "int4");
    for _ in 0..40 {
        t = ptr_to(t);
    }
    let out = spell(&t);
    assert!(out.contains("*mut"));
    assert!(out.ends_with("*const ()"), "guard rail should cap the walk: {out}");
}

/// Unlike C, the Rust speller does not consult `realtypes`/`ctypes`: those exist
/// to preserve upstream Ghidra's `xunknownN`/`int4` vocabulary on demand, and
/// Rust has no such vocabulary to preserve.
#[test]
fn rust_ignores_the_realtypes_gates() {
    let off = SpellCtx { lang: OutLang::Rust, ..SpellCtx::OFF };
    let on = SpellCtx { lang: OutLang::Rust, enabled: true, ctypes: true, ..SpellCtx::OFF };
    let t = base(4, type_metatype::TYPE_UNKNOWN);
    assert_eq!(
        RUST_SPELLER.declarator(&off, &t).0,
        RUST_SPELLER.declarator(&on, &t).0
    );
}
