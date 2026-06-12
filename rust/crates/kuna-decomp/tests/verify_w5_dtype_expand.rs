//! Adversarial verifier tests for item `w5-dtype-expand`.
//!
//! These target the spots the hunt list flagged as most fragile for the
//! `type.hh` `Datatype` interface port:
//!
//!   * comparator routing / totality (`Datatype::compare` /
//!     `compareDependency`), where a wrong subclass grouping silently corrupts
//!     ordering — the riskiest surface in this item;
//!   * the base `compare` body's signed `op.size - size` subtraction (the C++
//!     wrapping vs. Rust panic boundary);
//!   * `string2metatype` boundary inputs (empty string, known-first-char with a
//!     non-matching tail), which the C++ resolves via `metastring[0]` + fallthrough.
//!
//! C++ oracle: `decompiler/cpp/type.cc` (`Datatype::compare` 216-222,
//! `Datatype::compareDependency` 231-237, `TypeSpacebase::compare` 3498-3501,
//! `TypeSpacebase::compareDependency` 3504-3514, `string2metatype` 307-369) and
//! `decompiler/cpp/type.hh` (`TypeSpacebase` 799-824, declaring `compare`/
//! `compareDependency` as overrides).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{AddrSpace, ConstantSpace};
use kuna_decomp::dtype::{
    metatype2string, string2metatype, sub_metatype, type_metatype, Datatype, DatatypeKind,
};

fn const_space() -> Rc<AddrSpace> {
    Rc::new(ConstantSpace::new())
}

/// Build a `TYPE_SPACEBASE` Datatype with the given spaceid/localframe, mirroring
/// the C++ `TypeSpacebase(int4=0, align=1, TYPE_SPACEBASE)` shape (size 0).
fn spacebase(spaceid: Option<Rc<AddrSpace>>, localframe: Address) -> Datatype {
    let mut d = Datatype::new_with_align(0, 1, type_metatype::TYPE_SPACEBASE);
    d.kind = DatatypeKind::Spacebase {
        spaceid,
        localframe,
    };
    d
}

/// **The bug.** `TypeSpacebase` overrides `compare`/`compareDependency`
/// (type.hh:821-822): after the base submeta/size comparison returns 0 (which it
/// always does for two spacebases — both size 0, both SUB_SPACEBASE), the C++
/// `TypeSpacebase::compareDependency` (type.cc:3504-3514) tie-breaks on
/// `spaceid` then `localframe`, and `TypeSpacebase::compare` (3498-3501) just
/// delegates to it. The port routes `DatatypeKind::Spacebase` to the *base*
/// `compare` body, so two distinct spacebases (same submeta+size, different
/// localframe) wrongly compare **equal** (`Ok(0)`).
///
/// Since the porter's own rule seams every other subclass `compare` override to
/// `Err(SEAM(W6))`, the faithful behavior here is *either* a SEAM error *or* the
/// real tie-break — never a silent `Ok(0)`. This asserts that contract; under the
/// current routing it FAILS (the port returns `Ok(0)`), which is the divergence
/// trace backing the REJECT.
#[test]
fn w5_dtype_spacebase_compare_must_not_silently_equal_distinct_frames() {
    let spc = const_space();
    let frame_a = Address::new(Rc::clone(&spc), 0x1000);
    let frame_b = Address::new(Rc::clone(&spc), 0x2000);

    let a = spacebase(Some(Rc::clone(&spc)), frame_a);
    let b = spacebase(Some(Rc::clone(&spc)), frame_b);

    // C++ `TypeSpacebase::compareDependency`: base part is 0 (equal submeta+size),
    // spaceid equal, localframe valid and a.localframe != b.localframe ->
    // returns +/-1, NEVER 0. `compare` delegates to it.
    match a.compare(&b, 10) {
        Ok(0) => panic!(
            "Datatype::compare routed TYPE_SPACEBASE to the base body: two \
             distinct-localframe spacebases compared EQUAL (Ok(0)). C++ \
             TypeSpacebase::compare (type.cc:3498) tie-breaks on localframe and \
             cannot return 0 here. The override must be SEAM(W6)'d like every \
             other subclass compare."
        ),
        Ok(_n) => { /* a real tie-break would also be acceptable */ }
        Err(_seam) => { /* SEAM(W6) is the faithful interface-stage answer */ }
    }
}

/// Same divergence via `compareDependency` directly (type.cc:3504-3514): the
/// localframe tie-break is in `compareDependency`, so this is the primary site.
#[test]
fn w5_dtype_spacebase_compare_dependency_distinguishes_localframe() {
    let spc = const_space();
    let a = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x10));
    let b = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x20));

    if let Ok(0) = a.compare_dependency(&b) {
        panic!(
            "Datatype::compareDependency routed TYPE_SPACEBASE to the base body: \
             distinct-localframe spacebases compared EQUAL. C++ \
             TypeSpacebase::compareDependency (type.cc:3504) tie-breaks on \
             spaceid/localframe."
        );
    }
}

/// The kinds that genuinely use the base `compare` body (Base/Void/Unknown — no
/// C++ override) must reproduce type.cc:216-222 exactly: submeta drives the
/// sign, then `op.size - size` (signed `int4`, wrapping). This confirms the
/// *correct* path is faithful and brackets the Spacebase mis-routing.
#[test]
fn w5_dtype_base_compare_submeta_then_size_subtraction() {
    // INT (SUB_INT_PLAIN=17) vs UINT (SUB_UINT_PLAIN=16): uint is more specific,
    // so int.compare(uint): submeta 17 != 16, 17 < 16 is false -> +1.
    let int4_t = Datatype::new(4, type_metatype::TYPE_INT);
    let uint4_t = Datatype::new(4, type_metatype::TYPE_UINT);
    assert_eq!(int4_t.get_sub_meta(), sub_metatype::SUB_INT_PLAIN);
    assert_eq!(uint4_t.get_sub_meta(), sub_metatype::SUB_UINT_PLAIN);
    assert_eq!(int4_t.compare(&uint4_t, 10).unwrap(), 1);
    assert_eq!(uint4_t.compare(&int4_t, 10).unwrap(), -1);

    // Same submeta, different size: returns op.size - size (bigger types earlier).
    let int1_t = Datatype::new(1, type_metatype::TYPE_INT);
    let int9_t = Datatype::new(9, type_metatype::TYPE_INT);
    // int1.compare(int9): op.size(9) - size(1) = 8
    assert_eq!(int1_t.compare(&int9_t, 10).unwrap(), 8);
    // int9.compare(int1): op.size(1) - size(9) = -8
    assert_eq!(int9_t.compare(&int1_t, 10).unwrap(), -8);
    // Equal: 0.
    assert_eq!(int4_t.compare(&int4_t, 10).unwrap(), 0);

    // The unknown / void kinds also use the base body (no C++ override).
    let unk = Datatype::new(4, type_metatype::TYPE_UNKNOWN);
    let void = Datatype::new(0, type_metatype::TYPE_VOID);
    // SUB_UNKNOWN(21) < SUB_VOID(23) -> unknown.compare(void) == -1.
    assert_eq!(unk.compare(&void, 10).unwrap(), -1);
}

/// `string2metatype` boundary inputs. C++ switches on `metastring[0]`
/// (type.cc:308); an empty string in C++11 reads the NUL terminator -> `default`
/// -> throw. The Rust port uses `.first()` -> `None` -> error. A known first
/// char with a non-matching full string falls through the inner ifs and still
/// throws (the C++ `break` then final `throw`).
#[test]
fn w5_dtype_string2metatype_boundary_and_fallthrough() {
    // Empty string: both C++ (NUL -> default -> throw) and Rust error.
    assert!(string2metatype("").is_err());
    // Known first char 'p', unknown tail -> falls through inner ifs -> error.
    assert!(string2metatype("ptrz").is_err());
    assert!(string2metatype("partbogus").is_err());
    // 's' has two distinct matches (struct/spacebase) via separate ifs in C++.
    assert_eq!(string2metatype("struct").unwrap(), type_metatype::TYPE_STRUCT);
    assert_eq!(
        string2metatype("spacebase").unwrap(),
        type_metatype::TYPE_SPACEBASE
    );
    // 'u' three-way (unknown/uint/union).
    assert_eq!(string2metatype("union").unwrap(), type_metatype::TYPE_UNION);
    // The documented upstream asymmetry: "partenum" is emitted but not decodable.
    assert_eq!(
        metatype2string(type_metatype::TYPE_PARTIALENUM).unwrap(),
        "partenum"
    );
    assert!(string2metatype("partenum").is_err());
}

// =============================================================================
// Round 2 verifier additions (independent re-derivation of the override routing)
// =============================================================================
//
// Round 1 REJECTed on F1: `DatatypeKind::Spacebase` was routed to the base
// `compare`/`compareDependency` body, silently returning Ok(0) for two distinct
// spacebases. These tests independently pin the *fixed* routing and the rest of
// the subclass-override partition, against type.hh's virtual-override map:
//
//   compare/compareDependency overriders : Pointer, Array, Struct, Union, Enum,
//       Code, PartialStruct, PartialUnion, PartialEnum, PointerRel, Spacebase
//   getSubType overriders                : Pointer, Array, Struct, Spacebase,
//       PartialStruct, Code  (Union's getSubType is commented out -> base/null;
//       Enum has none -> base/null)
//   findCompatibleResolve overriders     : Pointer, Array, Struct, Union,
//       PartialUnion  (type.hh:294/488/522/608/637/714)
//   isPtrsubMatching overriders          : Pointer, PointerRel
//   numDepend/getDepend for PointerRel inherit TypePointer's (1 / ptrto).

use kuna_decomp::dtype::TypeField;

/// F1 FIX CONFIRMATION (now flipped by w6-s5-type-2). The F1 fix demanded that
/// `Spacebase` must NOT use the *base* `compare` body (which would equate
/// distinct-localframe spacebases as `Ok(0)`). w6-s5-type-2 ports the real
/// `TypeSpacebase::compare`/`compareDependency` (type.cc:3498-3514), which
/// tie-breaks on spaceid then localframe — so distinct localframes now compare
/// non-equal, and identical ones compare equal.  The pin flips from "must Err"
/// to "must distinguish localframes" (the property the F1 fix actually protects).
#[test]
fn w5r2_spacebase_compare_distinguishes_localframe() {
    let spc = const_space();
    let a = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x1000));
    let b = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x2000));
    // Distinct localframes must NOT equate (the F1 invariant); 0x1000 < 0x2000.
    assert_eq!(a.compare(&b, 10).unwrap(), -1);
    assert_eq!(b.compare(&a, 10).unwrap(), 1);
    assert_eq!(a.compare_dependency(&b).unwrap(), -1);
    // A self-spacebase compares equal (same spaceid, same localframe).
    assert_eq!(a.compare(&a.clone(), 10).unwrap(), 0);
}

/// `find_compatible_resolve` partition (type.hh:294/488/522/608/637/714). Of the
/// FIVE overriders, Pointer/Array (w6-s5-type-1) and Struct/Union/PartialUnion
/// (w6-s5-type-2) are now implemented; every other kind returns the base default
/// -1 (type.cc:600-604). PointerRel, Code, Spacebase, PartialStruct,
/// PartialEnum are NOT overriders -> base -1.
#[test]
fn w5r2_find_compatible_resolve_override_partition() {
    let int_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));

    // Base default -1 for non-overriders.
    assert_eq!(Datatype::new(4, type_metatype::TYPE_INT).find_compatible_resolve(&int_t).unwrap(), -1);
    assert_eq!(Datatype::new(0, type_metatype::TYPE_VOID).find_compatible_resolve(&int_t).unwrap(), -1);
    let mut code = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
    code.kind = DatatypeKind::Code { proto: None };
    assert_eq!(code.find_compatible_resolve(&int_t).unwrap(), -1);
    let pe_parent = Rc::new(Datatype::new(4, type_metatype::TYPE_ENUM_INT));
    let mut pe = Datatype::new_with_align(2, -1, type_metatype::TYPE_PARTIALENUM);
    pe.kind = DatatypeKind::PartialEnum {
        stripped: Rc::clone(&int_t),
        parent: pe_parent,
        offset: 0,
    };
    assert_eq!(pe.find_compatible_resolve(&int_t).unwrap(), -1);

    // Struct/Union/PartialUnion are now implemented (w6-s5-type-2).
    // TypeStruct::findCompatibleResolve (type.cc:2300-2311): field[0].type == ct
    // (a struct that needs resolution has exactly one field) -> 0.
    let mut s = Datatype::new_with_align(4, -1, type_metatype::TYPE_STRUCT);
    s.kind = DatatypeKind::Struct {
        field: vec![TypeField::new(0, 0, "a", Rc::clone(&int_t))],
        bitfield: vec![],
    };
    assert_eq!(s.find_compatible_resolve(&int_t).unwrap(), 0);
    // A struct whose only field is a different type -> -1.
    let other_struct_field = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    assert_eq!(s.find_compatible_resolve(&other_struct_field).unwrap(), -1);
    // TypeUnion::findCompatibleResolve (type.cc:2629-2649): the field == ct at
    // offset 0 -> its index.
    let mut u = Datatype::new_with_align(4, -1, type_metatype::TYPE_UNION);
    u.kind = DatatypeKind::Union {
        field: vec![TypeField::new(0, 0, "a", Rc::clone(&int_t))],
    };
    assert_eq!(u.find_compatible_resolve(&int_t).unwrap(), 0);
    assert_eq!(u.find_compatible_resolve(&other_struct_field).unwrap(), -1);

    // TypePointer::findCompatibleResolve (type.cc:1347-1354): ct is a plain int
    // (not TYPE_PTR), so the override returns -1.
    let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    p.kind = DatatypeKind::Pointer {
        ptrto: Rc::clone(&int_t),
        spaceid: None,
        truncate: None,
        wordsize: 1,
    };
    assert_eq!(p.find_compatible_resolve(&int_t).unwrap(), -1);

    // TypeArray::findCompatibleResolve (type.cc:1480-1490): arrayof IS int_t
    // (same Rc allocation), and int has no needsResolution, so `arrayof == ct`
    // -> 0.
    let mut a = Datatype::new_with_align(8, -1, type_metatype::TYPE_ARRAY);
    a.kind = DatatypeKind::Array { arrayof: Rc::clone(&int_t), arraysize: 2 };
    assert_eq!(a.find_compatible_resolve(&int_t).unwrap(), 0);
    // An array over a *different* element returns -1 (not the same pointer).
    let other = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    assert_eq!(a.find_compatible_resolve(&other).unwrap(), -1);
}

/// `get_sub_type` partition: Union and Enum are NOT getSubType overriders
/// (Union's is commented out in type.hh:619; Enum has none), so they take the
/// base body -> (None, off). Array/Pointer ARE overriders, implemented in
/// w6-s5-type-1; Struct is a type-2 overrider, now implemented (w6-s5-type-2).
#[test]
fn w5r2_get_sub_type_union_enum_are_base_not_seam() {
    let mut u = Datatype::new_with_align(8, -1, type_metatype::TYPE_UNION);
    u.kind = DatatypeKind::Union { field: vec![] };
    let (sub, newoff) = u.get_sub_type(5).unwrap();
    assert!(sub.is_none(), "TypeUnion::getSubType is base (null) — type.hh:619 commented out");
    assert_eq!(newoff, 5, "base getSubType passes offset through unchanged");

    let mut e = Datatype::new_with_align(4, -1, type_metatype::TYPE_ENUM_INT);
    e.kind = DatatypeKind::Enum { namemap: std::collections::BTreeMap::new() };
    let (sub, newoff) = e.get_sub_type(3).unwrap();
    assert!(sub.is_none());
    assert_eq!(newoff, 3);

    // Struct DOES override (type-2, now implemented): off 0 lands in field "a"
    // (offset 0, an int4), passing back the field type with newoff 0.
    let int_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let mut s = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
    s.kind = DatatypeKind::Struct {
        field: vec![TypeField::new(0, 0, "a", Rc::clone(&int_t))],
        bitfield: vec![],
    };
    let (sub, newoff) = s.get_sub_type(0).unwrap();
    assert!(sub.is_some());
    assert_eq!(newoff, 0);
    // An offset past the single field's extent (off 4 >= field end) -> base body.
    let (sub_oob, newoff_oob) = s.get_sub_type(4).unwrap();
    assert!(sub_oob.is_none());
    assert_eq!(newoff_oob, 4);
}

/// `PointerRel` inherits `TypePointer::numDepend()==1` and `getDepend(0)==ptrto`
/// (it does NOT override them — type.hh:724-770). The port must report 1 / ptrto,
/// and `is_ptrsub_matching` (an overrider for both Pointer and PointerRel) SEAMs.
#[test]
fn w5r2_pointer_rel_inherits_pointer_depend_and_ptrsub_seams() {
    let ptrto = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
    let mut pr = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTRREL);
    pr.kind = DatatypeKind::PointerRel {
        ptrto: Rc::clone(&ptrto),
        wordsize: 1,
        stripped: None,
        parent: Rc::clone(&parent),
        offset: 4,
    };
    assert_eq!(pr.num_depend(), 1, "PointerRel inherits TypePointer::numDepend()==1");
    assert_eq!(pr.get_depend(0).unwrap().get_size(), 4, "inherited getDepend(0)==ptrto");
    assert_eq!(pr.get_ptr_to().unwrap().get_size(), 4);
    assert_eq!(pr.get_byte_offset(), Some(4));
    assert_eq!(pr.get_rel_parent().unwrap().get_size(), 16);
    // isPtrsubMatching overrides for PointerRel -> SEAM.
    assert!(pr.is_ptrsub_matching(0, 0, 1).is_err());
    // getPtrInto overrides for PointerRel -> SEAM (relative offset math is W6).
    assert!(pr.get_ptr_into().is_err());
}

/// LOSS-050 RESTORED (w6-s5-type-1). The base C++ `resolveInFlow`/`findResolve`
/// return `this` for every type without a union override; the W6 port now honors
/// that identity via the `self: &Rc<Datatype>` receiver. A plain int has no
/// override, so both return the receiver unchanged (same `Rc` allocation). The
/// union/pointer-to-union/array structured paths stay SEAM(W6) (they still need
/// the `Funcdata` registry).
#[test]
fn w5r2_resolve_in_flow_base_is_seamed_loss_f2() {
    let int_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    // OpId is a slotmap key newtype; its default (null) key is a valid opaque handle
    // (the identity path never reads it).
    let op = kuna_decomp::seams::OpId::default();
    let resolved = int_t.resolve_in_flow(op, 0).expect("base resolveInFlow returns `this`");
    assert!(
        Rc::ptr_eq(&resolved, &int_t),
        "base resolveInFlow returns the receiver unchanged (C++ `return this`)"
    );
    let found = int_t.find_resolve(op, -1).expect("base findResolve returns `this`");
    assert!(
        Rc::ptr_eq(&found, &int_t),
        "base findResolve returns the receiver unchanged (C++ `return this`)"
    );
}
