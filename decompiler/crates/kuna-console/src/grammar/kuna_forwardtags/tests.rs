//! Tests for tags named before they are defined ([`super`]).

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_decomp::dtype::{type_metatype as meta, Datatype, TypeFactory, TypeFactoryImpl};

use super::super::{parse_c, parse_protopieces, parse_type, DataOrg};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("void", 0, meta::TYPE_VOID, false).unwrap();
    f.set_core_type("int4", 4, meta::TYPE_INT, false).unwrap();
    f.set_core_type("int8", 8, meta::TYPE_INT, false).unwrap();
    f.cache_core_types().unwrap();
    f.setup_sizes(Some(4), 8, 8);
    f
}

fn org() -> DataOrg {
    DataOrg { addr_size: 8, word_size: 1 }
}

fn declare(f: &TypeFactoryImpl, decl: &str) -> KunaResult<()> {
    parse_c(decl, f, org(), &[], |_, _| Ok(()))
}

fn record(f: &TypeFactoryImpl, name: &str) -> Rc<Datatype> {
    f.find_by_name(name).unwrap().unwrap_or_else(|| panic!("{name} is interned"))
}

/// The pointee of member `i` of `record`.
fn pointee(record: &Datatype, i: i32) -> Rc<Datatype> {
    record.get_field(i).unwrap().field_type.get_ptr_to().unwrap()
}

#[test]
fn a_record_may_point_at_itself() {
    let f = factory();
    declare(&f, "struct Node { struct Node *next; int4 val; };").unwrap();
    let node = record(&f, "Node");
    assert!(!node.is_incomplete());
    assert_eq!(node.get_size(), 16);
    assert_eq!(pointee(&node, 0).get_name(), "Node");
    assert!(f.kuna_declared_ahead(&pointee(&node, 0)));
}

/// A union's member may point at the union too.  (A function-pointer member
/// needs an architecture's prototype model; the CLI tests drive that shape.)
#[test]
fn a_union_may_point_at_itself() {
    let f = factory();
    declare(&f, "union U { union U *self; int4 v; };").unwrap();
    let u = record(&f, "U");
    assert!(!u.is_incomplete());
    assert_eq!(u.get_metatype(), meta::TYPE_UNION);
    assert_eq!(pointee(&u, 0).get_name(), "U");
}

#[test]
fn a_tag_may_be_declared_ahead_of_its_body() {
    for ahead in ["struct Node;", "struct Node Node;"] {
        let f = factory();
        declare(&f, ahead).unwrap();
        assert!(record(&f, "Node").is_incomplete());
        declare(&f, "struct Node { int4 val; Node *next; };").unwrap();
        let node = record(&f, "Node");
        assert!(!node.is_incomplete(), "{ahead}");
        assert!(f.kuna_declared_ahead(&pointee(&node, 1)));
    }
}

#[test]
fn two_records_may_point_at_each_other() {
    let f = factory();
    declare(&f, "struct A { struct B *b; int8 x; };").unwrap();
    assert!(record(&f, "B").is_incomplete());
    declare(&f, "struct B { struct A *a; int4 y; };").unwrap();
    assert!(!record(&f, "B").is_incomplete());
    assert!(!pointee(&record(&f, "B"), 0).is_incomplete());
}

/// C rejects a member of incomplete type; so does the parser, and the record
/// it was defining is not left behind.
#[test]
fn a_record_held_by_value_before_it_is_complete_is_rejected() {
    for (decl, left) in [
        ("struct Node { struct Node n; };", "Node"),
        ("struct Node { int4 v; struct Node arr[2]; };", "Node"),
        ("struct A { struct B b; };", "B"),
        ("union U { union U u; };", "U"),
    ] {
        let f = factory();
        let err = declare(&f, decl).unwrap_err();
        assert!(err.explain().contains("has incomplete type"), "{decl}: {}", err.explain());
        assert!(f.find_by_name(left).unwrap().is_none(), "{decl}: {left} withdrawn");
    }
}

/// Outside a member list an unknown tag is declared only by a declaration that
/// is the tag alone; anything else keeps upstream's error and interns nothing.
#[test]
fn an_unknown_tag_elsewhere_is_still_rejected() {
    for decl in ["struct X *p;", "struct X Y;", "extern int4 f(struct X *p);", "struct X x, y;"] {
        let f = factory();
        let err = declare(&f, decl).unwrap_err();
        assert!(
            err.explain().contains("Identifier does not represent a struct as required"),
            "{decl}: {}",
            err.explain()
        );
        assert!(f.find_by_name("X").unwrap().is_none(), "{decl}: X withdrawn");
    }
    let f = factory();
    let err = parse_type("struct X *p", &f, org()).unwrap_err();
    assert!(err.explain().contains("does not represent a struct"), "{}", err.explain());
    let err = parse_protopieces("extern int4 f(union X *p);", &f, org()).unwrap_err();
    assert!(err.explain().contains("does not represent a union"), "{}", err.explain());
    assert!(f.find_by_name("X").unwrap().is_none());
}

/// A failed parse withdraws the tags it declared.
#[test]
fn a_failed_parse_leaves_no_tag_behind() {
    let f = factory();
    assert!(declare(&f, "struct A { struct B *b; int4 };").is_err());
    assert!(f.find_by_name("B").unwrap().is_none());
    assert!(f.find_by_name("A").unwrap().is_none());
}
