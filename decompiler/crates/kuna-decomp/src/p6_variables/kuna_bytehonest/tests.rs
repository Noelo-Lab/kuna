//! Unit tests for the `bytehonest` predicate: what counts as an uncommitted
//! byte, and the two neighbours it must never swallow (a wider unknown, an
//! array of unknown).

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};

use super::is_uncommitted_byte;

/// A minimally-configured factory carrying the core types a real architecture
/// mints for these cases.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined2", 2, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined4", 4, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.set_core_type("bool", 1, type_metatype::TYPE_BOOL, false).unwrap();
    f.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn unknown(f: &TypeFactoryImpl, size: i32) -> Rc<Datatype> {
    f.get_base(size, type_metatype::TYPE_UNKNOWN).unwrap()
}

#[test]
fn a_size_one_unknown_is_the_uncommitted_byte() {
    let f = factory();
    assert!(is_uncommitted_byte(&unknown(&f, 1)));
}

#[test]
fn a_wider_unknown_is_not_touched() {
    // `undefined4`/`undefined8` already report their width through the printer;
    // only the one-byte carrier is re-spelled, because only it renders as a
    // committed C type (`char`).
    let f = factory();
    for size in [2, 4, 8] {
        assert!(!is_uncommitted_byte(&unknown(&f, size)), "size {size} must be left alone");
    }
}

#[test]
fn an_array_of_unknown_is_not_touched() {
    // The `framelayout` slot speller reports `char[264]` as `undefined264`; this
    // predicate must not, or a 264-byte buffer would lose its array-ness.
    let f = factory();
    let arr = f.get_type_array(264, unknown(&f, 1)).unwrap();
    assert_eq!(arr.get_size(), 264);
    assert!(!is_uncommitted_byte(&arr));
    // Nor the one-element case, whose size IS 1.
    let one = f.get_type_array(1, unknown(&f, 1)).unwrap();
    assert!(!is_uncommitted_byte(&one), "an array is never the bare byte carrier");
}

#[test]
fn a_committed_byte_is_not_touched() {
    // The three one-byte types the recovery DID commit to: a re-spelling here
    // would destroy information rather than stop asserting it.
    let f = factory();
    assert!(!is_uncommitted_byte(&f.get_type_char(1).unwrap()));
    assert!(!is_uncommitted_byte(&f.get_base(1, type_metatype::TYPE_BOOL).unwrap()));
    assert!(!is_uncommitted_byte(&f.get_base(1, type_metatype::TYPE_UINT).unwrap()));
}

#[test]
fn the_predicate_reads_the_datatype_not_its_spelling() {
    // `realtypes` renders the size-1 unknown as `char`, the same string the real
    // `char` renders as; the predicate separates them, so it behaves the same
    // whatever the rendering options say.
    let f = factory();
    let u = unknown(&f, 1);
    let c = f.get_type_char(1).unwrap();
    assert_eq!(u.get_size(), c.get_size());
    assert_ne!(u.get_metatype(), c.get_metatype());
    assert!(is_uncommitted_byte(&u) && !is_uncommitted_byte(&c));
}
