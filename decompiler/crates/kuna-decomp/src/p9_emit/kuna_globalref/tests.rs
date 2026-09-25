//! (kuna `globalref`) Unit tests for the decision's pure parts over hand-built
//! `Datatype`s: the range lookup, the per-address type merge, and which direct
//! accesses the declared object may share its storage with.

use super::*;
use crate::dtype::flags;

fn core(size: i32, m: type_metatype, nm: &str) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(size, -1, m);
    t.name = nm.to_string();
    t.display_name = nm.to_string();
    t.id = Datatype::hash_name(nm);
    t.flags |= flags::coretype;
    Rc::new(t)
}

fn void() -> Rc<Datatype> {
    core(0, type_metatype::TYPE_VOID, "void")
}

fn direct(start: u64, size: u64, ty: Rc<Datatype>) -> Direct {
    Direct { start, size, ty, unnamed: true }
}

#[test]
fn an_address_is_in_range_only_inside_a_section() {
    let ranges = [(0x2000u64, 0x2fffu64), (0x4040u64, 0x40bfu64)];
    assert!(in_ranges(&ranges, 0x2000));
    assert!(in_ranges(&ranges, 0x2fff));
    assert!(in_ranges(&ranges, 0x4070));
    assert!(!in_ranges(&ranges, 0x3000), "between two sections");
    assert!(!in_ranges(&ranges, 0x1));
    assert!(!in_ranges(&ranges, 0x40c0), "one past the last section");
    assert!(!in_ranges(&[], 0x2000));
}

#[test]
fn a_void_use_yields_to_the_one_type_the_function_names() {
    let uint = core(4, type_metatype::TYPE_UINT, "uint4");
    let seen = Seen::merge(Some(Seen::merge(None, void(), false)), Rc::clone(&uint), false);
    assert!(matches!(&seen, Seen::One(t) if Rc::ptr_eq(t, &uint)));
    let seen = Seen::merge(Some(seen), void(), false);
    assert!(matches!(&seen, Seen::One(t) if Rc::ptr_eq(t, &uint)), "a later void use keeps it");
    assert!(matches!(Seen::merge(None, void(), false), Seen::Void(_)));
}

#[test]
fn two_types_at_one_address_name_nothing() {
    let uint = core(4, type_metatype::TYPE_UINT, "uint4");
    let long = core(8, type_metatype::TYPE_INT, "int8");
    let seen = Seen::merge(Some(Seen::merge(None, uint, false)), long, false);
    assert!(matches!(seen, Seen::Conflict));
    assert!(Seen::merge(Some(Seen::Conflict), void(), false).object().is_none());
}

/// (kuna `elemptr`) An unknown word and the unsigned word of its size are one
/// object, and the unsigned one names it; a byte, a signed word, or the same
/// pair without the option are two types.
#[test]
fn an_unknown_word_is_the_unsigned_word_it_is_read_beside() {
    let uint2 = core(2, type_metatype::TYPE_UINT, "uint2");
    let int2 = core(2, type_metatype::TYPE_INT, "int2");
    let unk2 = core(2, type_metatype::TYPE_UNKNOWN, "xunknown2");
    let seen = Seen::merge(Some(Seen::merge(None, Rc::clone(&unk2), true)), Rc::clone(&uint2), true);
    assert!(matches!(&seen, Seen::One(t) if Rc::ptr_eq(t, &uint2)));
    let seen = Seen::merge(Some(Seen::merge(None, Rc::clone(&uint2), true)), Rc::clone(&unk2), true);
    assert!(matches!(&seen, Seen::One(t) if Rc::ptr_eq(t, &uint2)));
    assert!(matches!(Seen::merge(Some(Seen::One(Rc::clone(&unk2))), int2, true), Seen::Conflict));
    assert!(matches!(Seen::merge(Some(Seen::One(Rc::clone(&unk2))), Rc::clone(&uint2), false), Seen::Conflict));
    let ubyte = core(1, type_metatype::TYPE_UINT, "uint1");
    let unk1 = core(1, type_metatype::TYPE_UNKNOWN, "xunknown1");
    assert!(matches!(Seen::merge(Some(Seen::One(unk1)), ubyte, true), Seen::Conflict));
}

#[test]
fn a_direct_access_shares_storage_only_as_the_same_c_type() {
    let uint = core(4, type_metatype::TYPE_UINT, "uint4");
    let int = core(4, type_metatype::TYPE_INT, "int4");
    let unk = core(4, type_metatype::TYPE_UNKNOWN, "xunknown4");
    assert!(same_object(&direct(0x10, 4, Rc::clone(&uint)), 0x10, 4, &uint));
    assert!(same_object(&direct(0x10, 4, Rc::clone(&unk)), 0x10, 4, &uint), "undefined4 is unsigned int");
    assert!(!same_object(&direct(0x10, 4, Rc::clone(&int)), 0x10, 4, &uint), "signedness moves a compare");
    assert!(!same_object(&direct(0x10, 4, Rc::clone(&unk)), 0x10, 4, &int));
    assert!(!same_object(&direct(0x14, 4, Rc::clone(&uint)), 0x10, 4, &uint), "another start");
    assert!(!same_object(&direct(0x10, 8, unk), 0x10, 4, &uint), "another width");
    let ubyte = core(1, type_metatype::TYPE_UINT, "uint1");
    let unk1 = core(1, type_metatype::TYPE_UNKNOWN, "xunknown1");
    assert!(!same_object(&direct(0x10, 1, unk1), 0x10, 1, &ubyte), "a byte unknown may print as char");
}

#[test]
fn only_overlapping_storage_is_a_direct_access() {
    let b = core(1, type_metatype::TYPE_INT, "int1");
    assert!(overlaps(&direct(0x13, 1, Rc::clone(&b)), 0x10, 8));
    assert!(overlaps(&direct(0x0c, 8, Rc::clone(&b)), 0x10, 1), "a wider read that starts below");
    assert!(!overlaps(&direct(0x18, 1, Rc::clone(&b)), 0x10, 8));
    assert!(!overlaps(&direct(0x13, 1, b), 0x10, 1), "a byte past a one-byte object");
}

#[test]
fn a_void_only_address_adopts_the_one_type_it_is_read_at() {
    let long = core(8, type_metatype::TYPE_INT, "int8");
    let mut plan = Plan::default();
    plan.direct.push(direct(0x40, 8, Rc::clone(&long)));
    assert!(plan.direct_type_at(0x40).is_some_and(|t| Rc::ptr_eq(&t, &long)));
    assert!(plan.direct_type_at(0x48).is_none());
    plan.direct.push(direct(0x3c, 8, core(8, type_metatype::TYPE_UINT, "uint8")));
    assert!(plan.direct_type_at(0x40).is_none(), "a second access over it from another start");
}

#[test]
fn the_option_parses_on_and_off_only() {
    assert!(OptionGlobalRef.apply("on").unwrap().0);
    assert!(!OptionGlobalRef.apply("off").unwrap().0);
    assert!(OptionGlobalRef.apply("maybe").is_err());
}
