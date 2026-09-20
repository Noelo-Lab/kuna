//! Tests for the character-pointer commitment (kuna `charptr`).
//!
//! The walk needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-charptr.xml` (pass 1 `off` = `unsigned long a0`, pass 2
//! `uses` = `char *a0`).  What is pinned here is the option surface and the two
//! properties the design rests on: the candidate outranks every integer vote in
//! `getLocalType`'s fold, and it refines only a pointer that points at nothing.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

/// A factory with 8-byte pointers and the core types the constructors need.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.set_core_type("int8", 8, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("uint8", 8, type_metatype::TYPE_UINT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn char_ptr(f: &TypeFactoryImpl) -> Rc<Datatype> {
    let c = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    f.get_type_pointer(8, c, 1).unwrap()
}

#[test]
fn option_parses_its_three_values() {
    assert_eq!(OptionCharPtr.apply("off").unwrap().0, CharPtrMode::Off);
    assert_eq!(OptionCharPtr.apply("libc").unwrap().0, CharPtrMode::Libc);
    assert_eq!(OptionCharPtr.apply("uses").unwrap().0, CharPtrMode::Uses);
    assert!(OptionCharPtr.apply("on").is_err());
    assert!(OptionCharPtr.apply("byte").is_err());
    assert!(OptionCharPtr.apply("").is_err());
}

#[test]
fn off_is_the_default_and_only_uses_reads_dereferences() {
    assert_eq!(CharPtrMode::default(), CharPtrMode::Off);
    assert!(!CharPtrMode::Off.is_on());
    assert!(CharPtrMode::Libc.is_on());
    assert!(!CharPtrMode::Libc.reads_uses());
    assert!(CharPtrMode::Uses.reads_uses());
}

/// The candidate beats every integer vote — that is what makes a parameter passed
/// to `strlen` stop being `unsigned long`.
#[test]
fn candidate_folds_over_an_integer_vote() {
    let f = factory();
    let cand = char_ptr(&f);
    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT, type_metatype::TYPE_UNKNOWN] {
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
    assert!(!points_at_nothing(&f.get_base(8, type_metatype::TYPE_UINT).unwrap()));
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
    assert!(!is_char_pointer(&f.get_base(8, type_metatype::TYPE_INT).unwrap()));
}

/// Two candidates of the same shape are the same `Rc`, so nothing downstream sees
/// a type change that is only identity churn.
#[test]
fn candidate_is_stable_across_calls() {
    let f = factory();
    assert!(Rc::ptr_eq(&char_ptr(&f), &char_ptr(&f)));
}
