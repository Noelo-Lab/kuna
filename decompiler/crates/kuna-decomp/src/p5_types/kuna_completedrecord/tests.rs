//! Unit tests for resolving a pointer to a record's pre-completion stub.

use super::*;
use crate::dtype::{TypeFactoryImpl, TypeField};

fn core_factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("int4", 4, type_metatype::TYPE_INT, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

/// `struct Node { struct Node *next; int4 val; }`: the member names the stub,
/// and the stub resolves to the record that completed it.
#[test]
fn a_records_pointer_to_its_own_stub_resolves_to_the_record() {
    let f = core_factory();
    let int4 = f.find_by_name("int4").unwrap().unwrap();
    let stub = f.get_type_struct("Node").unwrap();
    let next = f.get_type_pointer(8, Rc::clone(&stub), 1).unwrap();
    let node = f
        .assign_raw_fields_struct(
            &stub,
            vec![TypeField::new(0, -1, "next", Rc::clone(&next)), TypeField::new(1, -1, "val", int4)],
            Vec::new(),
        )
        .unwrap();
    let member = node.get_field(0).unwrap().field_type.get_ptr_to().unwrap();
    assert!(member.is_incomplete(), "the member was built against the stub");
    let resolved = resolve_completed_pointer(&f, &next).unwrap();
    assert!(Rc::ptr_eq(&resolved.get_ptr_to().unwrap(), &node));
    assert_eq!(resolved.get_size(), 8);
    let complete = f.get_type_pointer(8, Rc::clone(&node), 1).unwrap();
    assert!(resolve_completed_pointer(&f, &complete).is_none(), "already complete");
}

/// A tag that was never completed, and one whose name now holds a different
/// kind of record, stay as they are.
#[test]
fn a_stub_nothing_completed_is_left_alone() {
    let f = core_factory();
    let opaque = f.get_type_struct("ctx").unwrap();
    let ptr = f.get_type_pointer(8, opaque, 1).unwrap();
    assert!(resolve_completed_pointer(&f, &ptr).is_none());

    let int4 = f.find_by_name("int4").unwrap().unwrap();
    let ustub = f.get_type_union("U").unwrap();
    let uptr = f.get_type_pointer(8, Rc::clone(&ustub), 1).unwrap();
    f.destroy_type(&ustub).unwrap();
    let st = f.get_type_struct("U").unwrap();
    f.assign_raw_fields_struct(&st, vec![TypeField::new(0, -1, "a", int4)], Vec::new()).unwrap();
    assert!(resolve_completed_pointer(&f, &uptr).is_none(), "a struct does not complete a union");
    assert!(resolve_completed_pointer(&f, &f.find_by_name("int4").unwrap().unwrap()).is_none());
}

/// `struct A { struct B *b; }; struct B { struct A *a; };`: B does not point at
/// its own stub, so the stub A holds resolves only when a C declaration declared
/// B ahead of its body; a stub matched to a definition by name alone does not.
#[test]
fn a_sibling_stub_resolves_only_when_declared_ahead() {
    let f = core_factory();
    let int4 = f.find_by_name("int4").unwrap().unwrap();
    let b_stub = f.get_type_struct("B").unwrap();
    let b_ptr = f.get_type_pointer(8, Rc::clone(&b_stub), 1).unwrap();
    let a = f.get_type_struct("A").unwrap();
    let a = f
        .assign_raw_fields_struct(&a, vec![TypeField::new(0, -1, "b", Rc::clone(&b_ptr))], Vec::new())
        .unwrap();
    let a_ptr = f.get_type_pointer(8, a, 1).unwrap();
    f.assign_raw_fields_struct(
        &b_stub,
        vec![TypeField::new(0, -1, "a", a_ptr), TypeField::new(1, -1, "y", int4)],
        Vec::new(),
    )
    .unwrap();
    assert!(resolve_completed_pointer(&f, &b_ptr).is_none(), "a name match alone");
    f.kuna_note_declared_ahead(&b_stub);
    let resolved = resolve_completed_pointer(&f, &b_ptr).unwrap();
    assert!(!resolved.get_ptr_to().unwrap().is_incomplete());
}
