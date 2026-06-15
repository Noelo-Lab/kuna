//! Adversarial verifier tests for item `w10-rel-pointer`.
//!
//! Targets the spots the hunt list flagged as most fragile for the relative-
//! pointer port (`type.cc` TypePointerRel::getPtrInto / getPtrToFromParent,
//! `printc.cc` opPtrsub relptr arm, `ifacedecomp.cc` IfcPointerSetting):
//!
//!   * the do-while at-least-once + null-fallback semantics of
//!     `getPtrToFromParent` (type.cc:3157-3171): the loop runs once even for a
//!     positive offset that lands on no field, and a null `getSubType` result
//!     must fall to `getBase(1,TYPE_UNKNOWN)` — NOT escape as the original base;
//!   * the multi-iteration drill (nested struct) where `curoff != 0` keeps the
//!     do-while going, which a naive `if`-once port would get wrong;
//!   * `getPtrInto` (type.cc:3060-3070) STRUCT vs UNION vs non-composite arm
//!     selection (off=0/ptrto vs off=offset/parent);
//!   * the non-positive offset fallback (`off <= 0` -> unknown1), the boundary
//!     between the `off > 0` and the else arm.
//!
//! C++ oracle: decompiler/cpp/type.cc (TypePointerRel::getPtrInto 3060-3070,
//! getPtrToFromParent 3157-3171), decompiler/cpp/type.cc TypeStruct::getSubType
//! 1894-1904 (the field walk these drive).

use std::rc::Rc;

use kuna_decomp::dtype::{
    type_metatype, Datatype, DatatypeKind, TypeFactory, TypeFactoryImpl, TypeField,
};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

/// Build a TYPE_STRUCT `Rc<Datatype>` with the given fields/size, by hand (no
/// interning needed — `getPtrToFromParent` only walks `getSubType` on it).
fn make_struct(size: i32, fields: Vec<TypeField>) -> Rc<Datatype> {
    let mut s = Datatype::new_with_align(size, 4, type_metatype::TYPE_STRUCT);
    s.kind = DatatypeKind::Struct { field: fields, bitfield: vec![] };
    Rc::new(s)
}

/// getPtrToFromParent: a positive offset that hits no field boundary (lands in a
/// hole, or past the last field within `size`) must still fall through to
/// `getBase(1,TYPE_UNKNOWN)` — the C++ do-while runs at least once, getSubType
/// returns null (no field iter), so `base == 0` -> unknown1.  This pins that the
/// port did NOT short-circuit the positive arm or return the original base.
#[test]
fn w10rel_get_ptr_to_from_parent_hole_offset_is_unknown1_not_base() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let unk1 = f.get_base(1, TYPE_UNKNOWN).unwrap();
    // struct { int a@0; int b@8; } size 16 — bytes 4..8 and 12..16 are holes.
    let s = make_struct(
        16,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&i4)),
            TypeField::new(1, 8, "b", Rc::clone(&i4)),
        ],
    );
    // offset 5 lands in the 4..8 hole: getFieldIter finds no field -> getSubType
    // returns null -> getBase(1,TYPE_UNKNOWN), NOT the struct or field `a`.
    let at5 = f.get_ptr_to_from_parent(Rc::clone(&s), 5).unwrap();
    assert!(Rc::ptr_eq(&at5, &unk1), "hole offset 5 -> unknown1, got {:?}", at5.get_metatype());
    assert!(!Rc::ptr_eq(&at5, &s), "hole offset must not escape as the original container");
    // offset 0 is the non-positive boundary -> unknown1 (NOT field a).
    let at0 = f.get_ptr_to_from_parent(Rc::clone(&s), 0).unwrap();
    assert!(Rc::ptr_eq(&at0, &unk1), "offset 0 (off<=0 arm) -> unknown1");
    // A negative offset also takes the else arm -> unknown1 (off is int4).
    let neg = f.get_ptr_to_from_parent(Rc::clone(&s), -4).unwrap();
    assert!(Rc::ptr_eq(&neg, &unk1), "negative offset -> unknown1");
}

/// getPtrToFromParent: the multi-iteration drill.  A nested struct forces the
/// C++ do-while to iterate more than once (curoff != 0 after the first
/// getSubType), descending into the inner struct's field.  A naive single-step
/// port would stop at the inner struct instead of the leaf field.
#[test]
fn w10rel_get_ptr_to_from_parent_nested_struct_drills_to_leaf() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let f4 = f.get_base(4, TYPE_FLOAT).unwrap();
    // inner struct { int x@0; float y@4; } size 8.
    let inner = make_struct(
        8,
        vec![
            TypeField::new(0, 0, "x", Rc::clone(&i4)),
            TypeField::new(1, 4, "y", Rc::clone(&f4)),
        ],
    );
    // outer struct { int head@0; inner nest@8; } size 16.
    let outer = make_struct(
        16,
        vec![
            TypeField::new(0, 0, "head", Rc::clone(&i4)),
            TypeField::new(1, 8, "nest", Rc::clone(&inner)),
        ],
    );
    // offset 8 hits `nest` exactly -> getSubType yields (inner, curoff=0): the
    // do-while exits on curoff==0, returning the inner struct itself.
    let at8 = f.get_ptr_to_from_parent(Rc::clone(&outer), 8).unwrap();
    assert!(Rc::ptr_eq(&at8, &inner), "offset 8 resolves to the nested struct");
    // offset 12 = nest@8 + 4 -> first getSubType yields (inner, curoff=4); the
    // do-while iterates again into inner.getSubType(4) -> (float y, 0).
    let at12 = f.get_ptr_to_from_parent(Rc::clone(&outer), 12).unwrap();
    assert!(Rc::ptr_eq(&at12, &f4), "offset 12 drills through nest into float y");
    // offset 8 + 2 = 10 lands mid-`x` of inner (curoff=2 into the int): inner's
    // getSubType(2) on an int finds no field -> null -> unknown1.
    let unk1 = f.get_base(1, TYPE_UNKNOWN).unwrap();
    let at10 = f.get_ptr_to_from_parent(Rc::clone(&outer), 10).unwrap();
    assert!(Rc::ptr_eq(&at10, &unk1), "mid-field offset 10 -> unknown1");
}

/// getPtrInto (type.cc:3060-3070): the three-way arm selection.  STRUCT ptrto and
/// UNION ptrto both return (ptrto, off=0); any other ptrto returns (parent,
/// off=offset).  A port that conflated the arms (or dropped the UNION case) would
/// return the wrong (datatype, offset) pair.
#[test]
fn w10rel_get_ptr_into_struct_union_and_scalar_arms() {
    use type_metatype::*;
    let parent = make_struct(
        32,
        vec![TypeField::new(0, 0, "p0", Rc::new(Datatype::new(4, TYPE_INT)))],
    );

    // helper to build a relptr with a given ptrto and offset.
    let mk_relptr = |ptrto: Rc<Datatype>, offset: i32| -> Datatype {
        let mut pr = Datatype::new_with_align(8, -1, TYPE_PTRREL);
        pr.kind = DatatypeKind::PointerRel {
            ptrto,
            wordsize: 1,
            stripped: None,
            parent: Rc::clone(&parent),
            offset,
        };
        pr
    };

    // ptrto = STRUCT -> (ptrto, 0).
    let struct_to = make_struct(16, vec![]);
    let pr_s = mk_relptr(Rc::clone(&struct_to), 12);
    let (into, off) = pr_s.get_ptr_into().unwrap().unwrap();
    assert_eq!(off, 0, "STRUCT ptrto: off=0");
    assert!(Rc::ptr_eq(&into, &struct_to), "STRUCT ptrto: returns ptrto, not parent");

    // ptrto = UNION -> (ptrto, 0).  This is the arm a STRUCT-only port would miss.
    let mut u = Datatype::new_with_align(8, 4, TYPE_UNION);
    u.kind = DatatypeKind::Union { field: vec![] };
    let union_to = Rc::new(u);
    let pr_u = mk_relptr(Rc::clone(&union_to), 4);
    let (into_u, off_u) = pr_u.get_ptr_into().unwrap().unwrap();
    assert_eq!(off_u, 0, "UNION ptrto: off=0");
    assert!(Rc::ptr_eq(&into_u, &union_to), "UNION ptrto: returns ptrto, not parent");

    // ptrto = scalar INT -> (parent, offset).
    let int_to = Rc::new(Datatype::new(4, TYPE_INT));
    let pr_i = mk_relptr(Rc::clone(&int_to), 7);
    let (into_i, off_i) = pr_i.get_ptr_into().unwrap().unwrap();
    assert_eq!(off_i, 7, "scalar ptrto: off=offset");
    assert!(Rc::ptr_eq(&into_i, &parent), "scalar ptrto: returns parent, not ptrto");

    // A plain pointer (non-rel) still returns (ptrto, 0) via the Pointer arm,
    // proving the relptr arm did not capture the ordinary pointer path.
    let mut p = Datatype::new_with_align(8, -1, TYPE_PTR);
    p.kind = DatatypeKind::Pointer {
        ptrto: Rc::clone(&int_to),
        spaceid: None,
        truncate: None,
        wordsize: 1,
    };
    let (pinto, poff) = p.get_ptr_into().unwrap().unwrap();
    assert_eq!(poff, 0, "plain pointer: off=0");
    assert!(Rc::ptr_eq(&pinto, &int_to), "plain pointer: returns ptrto");
}
