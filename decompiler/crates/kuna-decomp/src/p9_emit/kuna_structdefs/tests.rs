//! (kuna `structdefs`) Unit tests for the preamble's type walk and per-type
//! definition text, over hand-built `Datatype`s (the same pattern the
//! `doc_type_definitions` renderer tests use — no `Architecture` needed).

use super::*;
use crate::dtype::{flags, type_metatype, TypeField};
use crate::printc::RealTypeCtx;

fn named(size: i32, m: type_metatype, nm: &str) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(size, -1, m);
    t.name = nm.to_string();
    t.display_name = nm.to_string();
    t.id = Datatype::hash_name(nm);
    Rc::new(t)
}

fn core(size: i32, m: type_metatype, nm: &str) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(size, -1, m);
    t.name = nm.to_string();
    t.display_name = nm.to_string();
    t.id = Datatype::hash_name(nm);
    t.flags |= flags::coretype;
    Rc::new(t)
}

fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
    let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(p)
}

fn struct_of(nm: &str, size: i32, fields: &[(i32, &str, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut s = Datatype::new_with_align(size, -1, type_metatype::TYPE_STRUCT);
    s.name = nm.to_string();
    s.display_name = nm.to_string();
    s.id = Datatype::hash_name(nm);
    s.kind = DatatypeKind::Struct {
        field: fields
            .iter()
            .enumerate()
            .map(|(i, (off, fnm, ft))| TypeField::new(i as i32, *off, *fnm, Rc::clone(ft)))
            .collect(),
        bitfield: vec![],
    };
    Rc::new(s)
}

fn opaque(nm: &str) -> Rc<Datatype> {
    let mut s = Datatype::new_with_align(0, -1, type_metatype::TYPE_STRUCT);
    s.name = nm.to_string();
    s.display_name = nm.to_string();
    s.id = Datatype::hash_name(nm);
    s.flags |= flags::type_incomplete;
    s.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
    Rc::new(s)
}

fn walk(roots: &[Rc<Datatype>]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut order = Vec::new();
    for r in roots {
        visit(r, &mut seen, &mut order);
    }
    order.iter().map(|t| t.get_name().to_string()).collect()
}

/// A parameter type is a POINTER to the struct, so the struct is only reachable
/// through the pointee — the whole point of the walk.
#[test]
fn a_pointer_parameter_pulls_in_its_pointee_struct() {
    let i4 = core(4, type_metatype::TYPE_INT, "int4");
    let s = struct_of("struct_0", 16, &[(0, "field_0x0", Rc::clone(&i4))]);
    assert_eq!(walk(&[ptr_to(s)]), vec!["struct_0".to_string()]);
}

/// Definition-before-use: an inner struct reached through an outer struct's
/// field lands BEFORE the outer one.
#[test]
fn a_nested_struct_is_ordered_before_the_struct_that_holds_it() {
    let i4 = core(4, type_metatype::TYPE_INT, "int4");
    let inner = struct_of("inner", 4, &[(0, "a", Rc::clone(&i4))]);
    let outer = struct_of("outer", 16, &[(0, "in", Rc::clone(&inner)), (8, "p", ptr_to(inner))]);
    assert_eq!(walk(&[outer]), vec!["inner".to_string(), "outer".to_string()]);
}

/// A self-referential struct terminates and appears once.
#[test]
fn a_pointer_cycle_terminates_with_one_entry_per_type() {
    let mut a = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
    a.name = "node".to_string();
    a.display_name = "node".to_string();
    a.id = Datatype::hash_name("node");
    let a = Rc::new(a);
    let selfptr = ptr_to(Rc::clone(&a));
    // Rebuild `node` with a field pointing at the same Rc (the interned shape a
    // recursive struct has in the factory).
    let node = struct_of("node", 8, &[(0, "next", selfptr)]);
    assert_eq!(walk(&[node]), vec!["node".to_string()]);
}

/// Core types, unnamed types and internal partials are never defined.
#[test]
fn core_and_unnamed_types_are_never_defined() {
    let i4 = core(4, type_metatype::TYPE_INT, "int4");
    let anon = struct_of("", 8, &[]);
    let plain = named(8, type_metatype::TYPE_UINT, "ulong");
    assert!(!is_definable(&i4));
    assert!(!is_definable(&anon));
    assert!(!is_definable(&plain));
    assert!(walk(&[i4, anon, plain]).is_empty());
}

/// An incomplete struct prints as the `.h`'s forward declaration, not a body.
#[test]
fn an_incomplete_struct_defines_as_the_opaque_forward_declaration() {
    let f = opaque("FILE");
    assert!(is_definable(&f));
    assert_eq!(definition_text(&f, RealTypeCtx::OFF), "typedef struct FILE FILE; /* opaque */\n");
}

/// A complete struct defines as its body — the text `structscore.py`'s layout
/// parser reads out of the JSON.
#[test]
fn a_complete_struct_defines_as_its_body() {
    let i4 = core(4, type_metatype::TYPE_INT, "int4");
    let u1 = core(1, type_metatype::TYPE_UINT, "undefined1");
    let s = struct_of(
        "struct_0",
        12,
        &[(0, "field_0x0", ptr_to(u1)), (8, "field_0x8", Rc::clone(&i4))],
    );
    assert_eq!(
        definition_text(&s, RealTypeCtx::OFF),
        "struct struct_0 {\n    undefined1 *field_0x0;\n    int4 field_0x8;\n};\n"
    );
}

/// A DWARF image holds both the forward-declared and the defined `_IO_FILE`;
/// the machine-readable surface reports the complete one, once.
#[test]
fn the_complete_definition_wins_one_record_per_name() {
    let i4 = core(4, type_metatype::TYPE_INT, "int4");
    let fwd = opaque("_IO_FILE");
    let full = struct_of("_IO_FILE", 216, &[(0, "_flags", Rc::clone(&i4))]);
    let other = opaque("_IO_marker");
    let out = dedup_by_name(&[Rc::clone(&fwd), Rc::clone(&other), Rc::clone(&full)]);
    let names: Vec<&str> = out.iter().map(|t| t.get_name()).collect();
    assert_eq!(names, vec!["_IO_marker", "_IO_FILE"]);
    assert_eq!(out[1].get_size(), 216);
    // An opaque type with no complete twin is still reported.
    let only_fwd = dedup_by_name(&[Rc::clone(&other)]);
    assert_eq!(only_fwd.len(), 1);
}
