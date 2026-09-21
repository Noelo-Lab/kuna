//! Tests for the character-pointer commitment (kuna `charptr`).
//!
//! The walk needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-charptr.xml` (pass 1 `off` = the bug, pass 2 `on` = the
//! fix, with four controls).  What is pinned here are the two properties the
//! design rests on: the candidate outranks every integer vote in
//! `getLocalType`'s fold, and it refines only a pointer that points at nothing.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

/// A factory with 8-byte pointers and the core types the constructors need.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false)
        .unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false)
        .unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true)
        .unwrap();
    f.set_core_type("int8", 8, type_metatype::TYPE_INT, false)
        .unwrap();
    f.set_core_type("uint8", 8, type_metatype::TYPE_UINT, false)
        .unwrap();
    f.cache_core_types().unwrap();
    f
}

fn char_ptr(f: &TypeFactoryImpl) -> Rc<Datatype> {
    let c = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    f.get_type_pointer(8, c, 1).unwrap()
}

/// The candidate beats every integer vote, which is what turns a parameter the
/// callee declares `char *` from `unsigned long` into `char *`.
#[test]
fn candidate_folds_over_an_integer_vote() {
    let f = factory();
    let cand = char_ptr(&f);
    for meta in [
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT,
        type_metatype::TYPE_UNKNOWN,
    ] {
        let cur = f.get_base(8, meta).unwrap();
        assert!(folds_over(&cand, &cur), "char * should outrank {meta:?}");
    }
}

/// `void *` and `undefined1 *` are placeholders this rule may refine; a pointer at
/// anything named or sized is a claim and is left alone.
#[test]
fn only_a_pointer_at_nothing_is_refined() {
    let f = factory();
    let void = f.get_type_void().unwrap();
    assert!(points_at_nothing(&f.get_type_pointer(8, void, 1).unwrap()));
    let byte = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    assert!(points_at_nothing(&f.get_type_pointer(8, byte, 1).unwrap()));
    let long = f.get_base(8, type_metatype::TYPE_INT).unwrap();
    assert!(!points_at_nothing(&f.get_type_pointer(8, long, 1).unwrap()));
    let file = f.get_type_struct("FILE").unwrap();
    assert!(!points_at_nothing(&f.get_type_pointer(8, file, 1).unwrap()));
    assert!(!points_at_nothing(
        &f.get_base(8, type_metatype::TYPE_UINT).unwrap()
    ));
    // A `char *` the rule would produce is itself a claim, so a second pass over
    // an already-committed value stops at the guard rather than re-deriving it.
    assert!(!points_at_nothing(&char_ptr(&f)));
}

/// A `char *` is recognised as one wherever it arrives from — the callee-parameter
/// test the walk runs on every call argument.
#[test]
fn char_pointer_recognition_is_exact() {
    let f = factory();
    assert!(is_char_pointer(&char_ptr(&f)));
    let byte = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    assert!(!is_char_pointer(&f.get_type_pointer(8, byte, 1).unwrap()));
    let uchar = f.get_base(1, type_metatype::TYPE_UINT).unwrap();
    assert!(!is_char_pointer(&f.get_type_pointer(8, uchar, 1).unwrap()));
    assert!(!is_char_pointer(
        &f.get_base(8, type_metatype::TYPE_INT).unwrap()
    ));
}

/// The census label names the evidence a candidate rests on, and a refusal
/// outranks every piece of evidence collected before it.
#[test]
fn evidence_commits_only_without_a_refusal() {
    let mut ev = Evidence::default();
    assert!(!ev.commits());
    assert_eq!(ev.label(), "none");
    ev.byte = 2;
    assert!(ev.commits());
    assert_eq!(ev.label(), "byte");
    ev.libc = 1;
    ev.fmt = 1;
    assert_eq!(ev.label(), "fmt+byte");
    ev.refused = Some("elem-wider");
    assert!(!ev.commits());
    assert_eq!(ev.label(), "refuse:elem-wider");
}

/// Two candidates of the same shape are the same `Rc`, so nothing downstream sees
/// a type change that is only identity churn.
#[test]
fn candidate_is_stable_across_calls() {
    let f = factory();
    assert!(Rc::ptr_eq(&char_ptr(&f), &char_ptr(&f)));
}

/// A one-byte element step is character evidence only at a varying index from
/// the base.  A constant displacement is a field, whether it arrives as an
/// `INT_ADD` or, once the base is typed, as a `PTRADD`, and a step taken from
/// a field forwards without counting.
#[test]
fn ptradd_counts_only_a_varying_index_from_the_base() {
    assert_eq!(ptradd_step(Some(1), None, false), PtraddStep::Char);
    assert_eq!(ptradd_step(Some(1), None, true), PtraddStep::Offset);
    assert_eq!(ptradd_step(Some(1), Some(0x13), false), PtraddStep::Offset);
    assert_eq!(ptradd_step(Some(1), Some(1), false), PtraddStep::Offset);
    assert_eq!(ptradd_step(Some(1), Some(1), true), PtraddStep::Offset);
    assert_eq!(ptradd_step(Some(1), Some(0), false), PtraddStep::Same);
    assert_eq!(ptradd_step(Some(4), None, false), PtraddStep::Refuse);
    assert_eq!(ptradd_step(Some(4), Some(2), true), PtraddStep::Refuse);
    assert_eq!(ptradd_step(None, None, false), PtraddStep::Neutral);
}
