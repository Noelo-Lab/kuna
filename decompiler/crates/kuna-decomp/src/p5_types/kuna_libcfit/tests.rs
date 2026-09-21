//! Tests for the libc-aggregate fit rule (kuna `libctypes`, `kuna_libcfit`).
//!
//! The walk needs a decompiled function and is covered end to end by
//! `tests/stages/kuna-libctypes.xml` (the `kwset`-shaped container passes). What
//! is pinned here is which declared types the rule speaks about at all.

use super::*;

use crate::dtype::{flags, TypeFactory, TypeFactoryImpl};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn sized_ptr(f: &TypeFactoryImpl, name: &str, size: i32) -> Rc<Datatype> {
    let shell = f.get_type_struct(name).unwrap();
    let shell = if size > 0 {
        f.set_fields_struct_raw(&shell, Vec::new(), Vec::new(), size, 8, flags::type_incomplete).unwrap()
    } else {
        shell
    };
    f.get_type_pointer(8, shell, 1).unwrap()
}

#[test]
fn a_libc_aggregate_pointer_is_measured_against_its_width() {
    let f = factory();
    assert_eq!(libc_pointee_size(&sized_ptr(&f, "obstack", 88)), Some(88));
    assert_eq!(libc_pointee_size(&sized_ptr(&f, "_IO_FILE", 216)), Some(216));
}

#[test]
fn a_program_struct_or_a_primitive_is_not_this_rules_business() {
    let f = factory();
    assert_eq!(libc_pointee_size(&sized_ptr(&f, "struct_3", 40)), None);
    let int4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
    assert_eq!(libc_pointee_size(&f.get_type_pointer(8, int4, 1).unwrap()), None);
}

#[test]
fn a_zero_width_declaration_has_no_end_to_read_past() {
    let f = factory();
    assert_eq!(libc_pointee_size(&sized_ptr(&f, "stat", 0)), None);
}

#[test]
fn an_offset_past_the_end_or_before_the_start_does_not_fit() {
    assert!(fits(0x18, 0, 8, 88));
    assert!(fits(88, 0, 0, 88), "the one-past-the-end address is still the object's");
    assert!(!fits(88, 0, 8, 88), "kwset's first word after its obstack");
    assert!(!fits(-8, 0, 4, 144));
}

#[test]
fn a_step_must_be_a_whole_number_of_aggregates() {
    assert!(!fits(-8, 152, 4, 144), "wc: `failed` before `&fstatus[i].st`");
    assert!(fits(8, 16, 8, 16), "an array of timespec keeps the name");
    assert!(fits(0x88 - 144, 144, 4, 144), "`p[-1].field` of an array of stat");
    assert!(!fits(0x8c - 144, 144, 8, 144), "an access straddling two elements");
    assert!(fits(0x2c, 1, 1, 60), "an unknown index proves nothing");
    assert!(fits(0, 8, 8, 144), "a word-at-a-time copy of a stat");
    assert!(fits(0x10, 8, 4, 60), "a phi between two termios fields");
}
