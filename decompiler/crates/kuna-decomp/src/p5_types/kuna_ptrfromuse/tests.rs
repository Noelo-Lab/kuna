//! Tests for the use-derived parameter pointer (kuna `ptrfromuse`).
//!
//! The walk itself needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-ptrfromuse.xml` (pass 1 off = `int8 a0`, pass 2 `byte` =
//! `char *a0`).  What can be pinned here is the option surface and the one
//! property the whole design rests on: the candidate is offered to
//! `getLocalType`'s fold, so a more specific vote is kept rather than clobbered.

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

fn byte_ptr(f: &TypeFactoryImpl) -> Rc<Datatype> {
    let to = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    f.get_type_pointer(8, to, 1).unwrap()
}

#[test]
fn option_parses_its_three_values() {
    assert_eq!(OptionPtrFromUse.apply("off").unwrap().0, PtrFromUseMode::Off);
    assert_eq!(OptionPtrFromUse.apply("byte").unwrap().0, PtrFromUseMode::Byte);
    assert_eq!(OptionPtrFromUse.apply("void").unwrap().0, PtrFromUseMode::Void);
    assert!(OptionPtrFromUse.apply("on").is_err());
    assert!(OptionPtrFromUse.apply("").is_err());
}

#[test]
fn off_is_the_default_and_is_not_on() {
    assert_eq!(PtrFromUseMode::default(), PtrFromUseMode::Off);
    assert!(!PtrFromUseMode::Off.is_on());
    assert!(PtrFromUseMode::Byte.is_on());
    assert!(PtrFromUseMode::Void.is_on());
}

/// The candidate beats every integer vote — that is what makes a dereferenced-only
/// parameter stop being `long`.
#[test]
fn candidate_folds_over_an_integer_vote() {
    let f = factory();
    let cand = byte_ptr(&f);
    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT, type_metatype::TYPE_UNKNOWN] {
        let cur = f.get_base(8, meta).unwrap();
        assert!(folds_over(&cand, &cur), "pointer should outrank {meta:?}");
    }
}

/// ...and loses to a named aggregate pointer, which is why the candidate is folded
/// instead of installed as a replacement seed: `libctypes`' `FILE *` is
/// `SUB_PTR_STRUCT` and keeps its place.
#[test]
fn candidate_does_not_clobber_a_named_struct_pointer() {
    let f = factory();
    let cand = byte_ptr(&f);
    let file = f.get_type_struct("FILE").unwrap();
    let file_ptr = f.get_type_pointer(8, file, 1).unwrap();
    assert!(!folds_over(&cand, &file_ptr));
    assert!(folds_over(&file_ptr, &cand));
}

/// Two candidates of the same shape are the same `Rc`, so nothing downstream sees
/// a type change that is only identity churn.
#[test]
fn candidate_is_stable_across_calls() {
    let f = factory();
    assert!(Rc::ptr_eq(&byte_ptr(&f), &byte_ptr(&f)));
}
