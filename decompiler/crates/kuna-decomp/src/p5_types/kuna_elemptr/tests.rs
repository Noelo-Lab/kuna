//! Tests for the element-pointer commitment (kuna `elemptr`).
//!
//! The walk needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-elemptr.xml` (pass 1 `off` = the bug, pass 2 `on` = the
//! fix, with two controls) and by the compiled round trip in
//! `kuna-cli/tests/decompile_all_cli.rs`. What is pinned here are the pieces
//! the design rests on: which votes the candidate may replace, the batch
//! agreement over globals, and the arithmetic of literal offsets.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("int8", 8, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("uint8", 8, type_metatype::TYPE_UINT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

/// An integer or unknown vote, and a pointer at nothing, may be replaced; a
/// pointer at anything named or sized is a claim that outranks use evidence.
#[test]
fn only_an_integer_or_a_pointer_at_nothing_is_replaced() {
    let f = factory();
    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT, type_metatype::TYPE_UNKNOWN] {
        assert!(may_replace(&f.get_base(8, meta).unwrap()), "{meta:?}");
    }
    let void = f.get_type_void().unwrap();
    assert!(may_replace(&f.get_type_pointer(8, void, 1).unwrap()));
    let byte = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    assert!(may_replace(&f.get_type_pointer(8, byte, 1).unwrap()));
    let int4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
    assert!(!may_replace(&f.get_type_pointer(8, int4, 1).unwrap()));
    let ch = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    let charp = f.get_type_pointer(8, ch, 1).unwrap();
    assert!(!may_replace(&f.get_type_pointer(8, charp, 1).unwrap()), "char ** is a claim");
}

fn typed(elem: &str, signed: Option<bool>, firm: bool) -> GlobalVerdict {
    GlobalVerdict::Typed { elem: elem.to_string(), signed, firm }
}

/// Two walks of one function agree only when they found the same array; any
/// difference is the function disagreeing with itself.
#[test]
fn a_function_that_says_two_things_about_a_global_refuses_it() {
    let bytes = typed("1:int", Some(true), true);
    let words = typed("4:int", Some(true), true);
    assert_eq!(bytes.clone().merge(bytes.clone()), bytes);
    assert_eq!(typed("1:int", Some(true), false).merge(bytes.clone()), bytes, "evidence in either walk is evidence");
    assert_eq!(bytes.clone().merge(typed("1:int", Some(false), true)), GlobalVerdict::Refused);
    assert_eq!(bytes.clone().merge(words), GlobalVerdict::Refused);
    assert_eq!(bytes.merge(GlobalVerdict::Refused), GlobalVerdict::Refused);
    assert_eq!(GlobalVerdict::Refused.merge(GlobalVerdict::Refused), GlobalVerdict::Refused);
}

fn said(pairs: &[(Obj, GlobalVerdict)]) -> BTreeMap<Obj, GlobalVerdict> {
    pairs.iter().cloned().collect()
}

/// A global every function that says something about agrees on is left typed;
/// one another function reads at another width is blocked for, and redone in,
/// exactly the functions that typed it.
#[test]
fn a_disagreed_global_is_blocked_where_it_was_typed() {
    let (a, b) = (Obj::Held(0x4018), Obj::Held(0x4020));
    let bytes = typed("1:int", Some(true), true);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(a, bytes.clone()), (b, bytes.clone())]));
    l.file(0x200, said(&[(a, typed("4:int", Some(true), true))]));
    l.file(0x300, said(&[(b, bytes)]));
    let redo = l.disagreements();
    assert_eq!(redo.into_iter().collect::<Vec<_>>(), vec![0x100, 0x200]);
    assert!(l.is_blocked(0x100, a) && l.is_blocked(0x200, a));
    assert!(!l.is_blocked(0x100, b) && !l.is_blocked(0x300, b));
    // Nothing new to block: the redo pass stops.
    assert!(l.disagreements().is_empty());
}

/// The same width at two signs that evidence decided is a disagreement: a body
/// reading the element at the other sign would compute another value against
/// the one declaration.
#[test]
fn a_table_evidence_reads_at_two_signs_is_blocked() {
    let t = Obj::Table(0x339e0);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(t, typed("1:int", Some(false), true))]));
    l.file(0x200, said(&[(t, typed("1:int", Some(true), true))]));
    assert_eq!(l.disagreements().into_iter().collect::<Vec<_>>(), vec![0x100, 0x200]);
    assert!(l.is_blocked(0x100, t) && l.is_blocked(0x200, t));
}

/// A sign only the default chose takes the one evidence chose: the defaulted
/// function is decided again at that sign.  Defaults that differ take the
/// majority, unsigned on a tie; a default of another width is blocked.
#[test]
fn a_defaulted_sign_takes_the_evidenced_one() {
    let t = Obj::Table(0x40c0);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(t, typed("4:int", Some(false), true))]));
    l.file(0x200, said(&[(t, typed("4:int", Some(true), false))]));
    l.file(0x300, said(&[(t, typed("4:int", Some(false), false))]));
    l.file(0x400, said(&[(t, typed("2:int", Some(false), false))]));
    assert_eq!(l.disagreements().into_iter().collect::<Vec<_>>(), vec![0x200, 0x400]);
    assert_eq!(l.adopted(0x200, t), Some(false));
    assert!(l.is_blocked(0x400, t) && !l.is_blocked(0x200, t));
    // The redone function now reads it unsigned: nothing more to do.
    l.file(0x200, said(&[(t, typed("4:int", Some(false), false))]));
    assert!(l.disagreements().is_empty());
    let u = Obj::Table(0x5000);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(u, typed("4:int", Some(true), false))]));
    l.file(0x200, said(&[(u, typed("4:int", Some(false), false))]));
    assert_eq!(l.disagreements().into_iter().collect::<Vec<_>>(), vec![0x100]);
    assert_eq!(l.adopted(0x100, u), Some(false));
}

/// A refusal disagrees with a type about a global; saying nothing does not.
/// About a table, a function that reads it another way prints its own cast, so
/// only two types disagree.
#[test]
fn a_refusal_disagrees_and_silence_does_not() {
    let g = Obj::Held(0x4018);
    let bytes = typed("1:int", Some(true), true);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(g, bytes.clone())]));
    l.file(0x200, said(&[]));
    assert!(l.disagreements().is_empty());
    l.file(0x300, said(&[(g, GlobalVerdict::Refused)]));
    assert_eq!(l.disagreements().into_iter().collect::<Vec<_>>(), vec![0x100]);
    let t = Obj::Table(0x4018);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(t, bytes)]));
    l.file(0x300, said(&[(t, GlobalVerdict::Refused)]));
    assert!(l.disagreements().is_empty());
}

/// A function decompiled again replaces what it said the first time.
#[test]
fn a_redo_replaces_what_the_function_said() {
    let g = Obj::Held(0x4018);
    let bytes = typed("1:int", Some(true), true);
    let mut l = Ledger::default();
    l.file(0x100, said(&[(g, bytes.clone())]));
    l.file(0x200, said(&[(g, GlobalVerdict::Refused)]));
    l.file(0x200, said(&[(g, bytes)]));
    assert!(l.disagreements().is_empty());
}

/// Literal offsets are signed at their own width.
#[test]
fn a_literal_offset_is_read_at_its_own_width() {
    assert_eq!(sign_extend(0xff, 1), -1);
    assert_eq!(sign_extend(0xfffffffc, 4), -4);
    assert_eq!(sign_extend(0x10, 8), 0x10);
    assert_eq!(sign_extend(0xffff_ffff_ffff_fff8, 8), -8);
}

/// A byte compared against printable ASCII is a character; a byte compared
/// against a control value or 0x80 is not.
#[test]
fn a_character_literal_is_printable_ascii() {
    assert!(is_char_literal(b'=' as u64));
    assert!(is_char_literal(b' ' as u64));
    assert!(!is_char_literal(0));
    assert!(!is_char_literal(0x7f));
    assert!(!is_char_literal(0x80));
}
