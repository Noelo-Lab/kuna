//! Tests for the truth-valued byte type (kuna `boolbyte`).
//!
//! The walk itself needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-boolbyte.xml` (pass 1 off = `char v1`, pass 2 on =
//! `bool v1`).  What can be pinned here is the fold the whole design rests on:
//! the candidate is offered to `getLocalType`'s fold, so `char` and `uint1` lose
//! to it while a more specific vote is kept.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

/// A factory carrying the one-byte core types this rule chooses between.  `char`
/// is registered last among the one-byte `TYPE_INT`s exactly as
/// `Architecture::build_core_types` does, so `get_base(1, TYPE_INT)` answers
/// `char` here for the same reason it does in a real decompile.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("bool", 1, type_metatype::TYPE_BOOL, false).unwrap();
    f.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    f.set_core_type("int1", 1, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("xunknown1", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.cache_core_types().unwrap();
    f
}

/// The reason a truth-valued byte comes out `char` in the first place: the
/// one-byte `TYPE_INT` the comparison ops vote for IS the ASCII `char`, because
/// `cacheCoreTypes` prefers a character type over `int1`.
#[test]
fn the_one_byte_int_vote_is_the_ascii_char() {
    let f = factory();
    let vote = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    assert_eq!(vote.get_name(), "char");
}

/// The candidate beats every one-byte integer vote — that is what makes a flag
/// byte stop being `char`/`unsigned char`.
#[test]
fn candidate_folds_over_a_one_byte_integer_vote() {
    let f = factory();
    let cand = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    for meta in [
        type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT,
        type_metatype::TYPE_UNKNOWN,
    ] {
        let cur = f.get_base(1, meta).unwrap();
        assert!(folds_over(&cand, &cur), "bool should outrank {meta:?}");
        assert!(!folds_over(&cur, &cand), "{meta:?} should not outrank bool");
    }
}

/// ...and loses to a named type, which is why the candidate is folded instead of
/// installed as a replacement seed: a DWARF or callee-supplied enum keeps its
/// place.
#[test]
fn candidate_does_not_clobber_a_named_type() {
    let f = factory();
    let cand = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    let named = f.get_type_struct("flags_t").unwrap();
    assert!(!folds_over(&cand, &named));
}

/// Two candidates are the same `Rc`, so a second propagation pass sees no type
/// change that is only identity churn (`write_back` compares by pointer first).
#[test]
fn candidate_is_stable_across_calls() {
    let f = factory();
    let a = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    let b = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    assert!(Rc::ptr_eq(&a, &b));
}

/// The printer arm: a truncation into this rule's `bool` prints as a cast rather
/// than as the raw `SUB41` intrinsic, and only with the option on.
#[test]
fn a_truncation_into_bool_prints_as_a_cast() {
    let f = factory();
    let b = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    let u4 = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
    assert!(truncation_prints_as_cast(true, &b, &u4, 0));
    assert!(!truncation_prints_as_cast(false, &b, &u4, 0));
}

/// It speaks only for a bool destination at offset 0: a truncation of a
/// non-least-significant piece, or into anything else, is left to
/// `CastStrategyC::is_subpiece_cast` exactly as upstream wrote it.
#[test]
fn the_printer_arm_speaks_only_for_a_low_piece_into_bool() {
    let f = factory();
    let b = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    let u4 = f.get_base(4, type_metatype::TYPE_UINT).unwrap();
    let c = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    let st = f.get_type_struct("flags_t").unwrap();
    assert!(!truncation_prints_as_cast(true, &b, &u4, 1));
    assert!(!truncation_prints_as_cast(true, &c, &u4, 0));
    assert!(!truncation_prints_as_cast(true, &b, &st, 0));
}
