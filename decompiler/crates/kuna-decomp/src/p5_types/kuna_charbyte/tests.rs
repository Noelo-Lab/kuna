//! Tests for the char-pointer byte rule (kuna `charbyte`).
//!
//! The edge decision needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-charbyte.xml` (pass 1 off = `uint1`, pass 2 on = `char`).
//! What is pinned here is the lattice fact the rule exists to override and the
//! type predicates that keep it off every other byte.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

/// A factory with the one-byte core types in the order
/// `Architecture::build_core_types` registers them, so `get_base(1, TYPE_INT)`
/// is the ASCII `char` here for the same reason it is in a real decompile.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("bool", 1, type_metatype::TYPE_BOOL, false).unwrap();
    f.set_core_type("uint1", 1, type_metatype::TYPE_UINT, false).unwrap();
    f.set_core_type("int1", 1, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("xunknown1", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.cache_core_types().unwrap();
    f
}

/// Why the rule exists: in the lattice `uint1` outranks `char`, so the
/// zero-extension's vote beats the `char *` the byte was loaded through.
#[test]
fn the_zext_vote_outranks_char_in_the_lattice() {
    let f = factory();
    let c = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    let u = f.get_base(1, type_metatype::TYPE_UINT).unwrap();
    assert_eq!(c.get_name(), "char");
    assert!(u.type_order(&c).unwrap() < 0);
    assert!(c.type_order(&u).unwrap() > 0);
}

#[test]
fn only_the_ascii_one_byte_int_is_char() {
    let f = factory();
    assert!(is_char(&f.get_base(1, type_metatype::TYPE_INT).unwrap()));
    assert!(!is_char(&f.get_base_no_char(1, type_metatype::TYPE_INT).unwrap()));
    assert!(!is_char(&f.get_base(1, type_metatype::TYPE_UINT).unwrap()));
    assert!(!is_char(&f.get_base(1, type_metatype::TYPE_BOOL).unwrap()));
    assert!(!is_char(&f.get_base(4, type_metatype::TYPE_INT).unwrap()));
}

#[test]
fn only_the_plain_one_byte_uint_is_uint1() {
    let f = factory();
    assert!(is_uint1(&f.get_base(1, type_metatype::TYPE_UINT).unwrap()));
    assert!(!is_uint1(&f.get_base(1, type_metatype::TYPE_INT).unwrap()));
    assert!(!is_uint1(&f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap()));
    assert!(!is_uint1(&f.get_base(2, type_metatype::TYPE_UINT).unwrap()));
}

/// Recording is scoped to one propagation: nothing is kept once taken, and an
/// empty record asks for no second pass.
#[test]
fn a_record_is_taken_once() {
    let vn = VarnodeId::from(slotmap::KeyData::from_ffi(1));
    NOTED.with(|n| *n.borrow_mut() = Some(vec![vn, vn]));
    assert_eq!(take_noted(), Some(vec![vn]));
    assert_eq!(take_noted(), None);
    NOTED.with(|n| *n.borrow_mut() = Some(Vec::new()));
    assert_eq!(take_noted(), None);
}
