//! Unit tests for `slotptr`: which pointer types a slot may take, and what counts
//! as a foreign access to the slot.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};

use super::{admit, Decline, SlotEvidence, SlotStore};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("int8", 8, type_metatype::TYPE_INT, false).unwrap();
    f.set_core_type("code", 1, type_metatype::TYPE_CODE, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn ptr(f: &TypeFactoryImpl, to: Rc<Datatype>, size: i32) -> Rc<Datatype> {
    f.get_type_pointer(size, to, 1).unwrap()
}

#[test]
fn a_committed_pointer_of_the_slot_width_is_admitted() {
    let f = factory();
    let char_t = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    let int_t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
    assert_eq!(admit(&ptr(&f, char_t.clone(), 8), 8), Ok(()));
    assert_eq!(admit(&ptr(&f, int_t, 8), 8), Ok(()));
    let pp = ptr(&f, ptr(&f, char_t, 8), 8);
    assert_eq!(admit(&pp, 8), Ok(()), "a pointer to a pointer is a committed pointer");
    assert_eq!(admit(&ptr(&f, f.get_type_void().unwrap(), 8), 8), Ok(()), "void * is admitted");
}

#[test]
fn a_scalar_never_replaces_the_width_only_spelling() {
    // A width-only `undefined8` is credited against a same-width integer; a
    // committed scalar would trade that for a guess.
    let f = factory();
    assert_eq!(admit(&f.get_base(8, type_metatype::TYPE_INT).unwrap(), 8), Err(Decline::NotPointer));
    assert_eq!(admit(&f.get_base(8, type_metatype::TYPE_UNKNOWN).unwrap(), 8), Err(Decline::NotPointer));
}

#[test]
fn a_pointer_to_unknown_bytes_or_to_code_is_declined() {
    let f = factory();
    let unk = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    assert_eq!(admit(&ptr(&f, unk, 8), 8), Err(Decline::Pointee));
    let code = f.get_base(1, type_metatype::TYPE_CODE).unwrap();
    assert_eq!(admit(&ptr(&f, code, 8), 8), Err(Decline::Pointee));
}

#[test]
fn a_pointer_of_another_width_is_declined() {
    let f = factory();
    let char_t = f.get_base(1, type_metatype::TYPE_INT).unwrap();
    assert_eq!(admit(&ptr(&f, char_t, 4), 8), Err(Decline::Width));
}

#[test]
fn an_overlapping_access_at_another_offset_or_width_is_foreign() {
    let mut ev = SlotEvidence::default();
    ev.note_access(-0x20, 8);
    assert!(!ev.foreign_access(-0x20, 8), "the slot's own range is not foreign");
    ev.note_access(-0x18, 4);
    assert!(!ev.foreign_access(-0x20, 8), "an adjacent range does not overlap");
    ev.note_access(-0x1c, 4);
    assert!(ev.foreign_access(-0x20, 8), "a narrower read inside the slot is foreign");
    let mut wide = SlotEvidence::default();
    wide.note_access(-0x28, 16);
    assert!(wide.foreign_access(-0x20, 8), "a wider access covering the slot is foreign");
}

#[test]
fn stores_are_kept_per_exact_range_and_deduplicated() {
    let mut ev = SlotEvidence::default();
    ev.note_store(-0x20, 8, SlotStore::Null);
    ev.note_store(-0x20, 8, SlotStore::Null);
    ev.note_store(-0x20, 4, SlotStore::Opaque);
    assert_eq!(ev.stores_at(-0x20, 8), &[SlotStore::Null]);
    assert_eq!(ev.stores_at(-0x20, 4), &[SlotStore::Opaque]);
    assert!(ev.stores_at(-0x18, 8).is_empty());
    ev.clear();
    assert!(ev.stores_at(-0x20, 8).is_empty());
}
