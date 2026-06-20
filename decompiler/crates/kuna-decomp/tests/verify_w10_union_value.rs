//! Adversarial verifier tests for item `w10-union-value`.
//!
//! The item's whole production change is: match `DatatypeKind::PointerRel`
//! identically to `DatatypeKind::Pointer` in the three type-resolution
//! dispatchers, because `TypePointerRel : public TypePointer` (type.hh:724) does
//! NOT override `resolveInFlow`/`findResolve`/`findCompatibleResolve`, so a
//! relative pointer dispatches the inherited `TypePointer` body:
//!
//!   * `Datatype::find_compatible_resolve` (dtype.rs) <- TypePointer::
//!     findCompatibleResolve (type.cc:1347-1354),
//!   * `Funcdata::find_resolve_facing` (funcdata_facing.rs) <- TypePointer::
//!     findResolve (type.cc:1335-1345),
//!   * `Funcdata::resolve_in_flow` (funcdata_resolveflow.rs) <- TypePointer::
//!     resolveInFlow (type.cc:1314-1333).
//!
//! These tests pin the directly-testable dispatcher (`find_compatible_resolve`)
//! and the accessor invariant the two Funcdata dispatchers rely on, with the
//! fragile spots the change touches:
//!
//!   * the C++ reads `((TypePointer *)ct)->ptrto` — the RELATIVE ptrto member,
//!     NOT the stripped plain pointer. A port that reached for `stripped` would
//!     resolve against the wrong (union) type. (at1, at2)
//!   * a `PointerRel` RECEIVER must recurse on its OWN relative `ptrto`, byte-
//!     identically to a plain `Pointer` with the same ptrto (the inherited
//!     body). (at3)
//!   * the union-arm gate is `ptrto.get_metatype() == TYPE_UNION`; a
//!     PointerRel-to-NON-union must NOT be treated as a union edge (the gate is
//!     on the ptrto metatype, shared by both pointer kinds). (at4)
//!   * a `PointerRel` whose ct argument is a plain `Pointer` (and vice-versa)
//!     must cross-recurse on the literal `ptrto` of each — the cast is on the
//!     argument, independent of the receiver kind. (at5)

use std::rc::Rc;

use kuna_base::types::int4;
use kuna_decomp::dtype::{
    flags, type_metatype, Datatype, DatatypeKind, TypeFactory, TypeFactoryImpl, TypeField,
};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

/// Build a completed N-field union `{ int4 myint@0; float4 myfloat@0; }`, the
/// shape behind the `(ptr->value).myint` datatest renders.  Built by hand (no
/// interning) and carrying `needs_resolution` — the flag the real
/// `getTypeUnion` sets (type.hh:625) and the precondition for the dispatcher
/// union arms.  `name` is accepted for readability but unused (no interning).
fn make_union(_name: &str, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut u = Datatype::new(4, type_metatype::TYPE_UNION);
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ct))| TypeField::new(i as int4, *off, format!("m{i}"), Rc::clone(ct)))
        .collect();
    u.kind = DatatypeKind::Union { field };
    u.flags |= flags::needs_resolution;
    Rc::new(u)
}

/// at1: `find_compatible_resolve` with a `PointerRel` ct must read ct's RELATIVE
/// `ptrto` (the field type), exactly like the C++ `((TypePointer *)ct)->ptrto`.
/// A plain pointer-to-int receiver, given a ptr-rel-to-int ct, must recurse
/// int<->int (compatible). The mutation guard: if the port had used `stripped`
/// (a TypePointer to the union/container) instead of `ptrto`, the recursion
/// would compare int against a pointer and the result would differ.
#[test]
fn w10_union_value_compat_resolve_reads_ct_relative_ptrto_not_stripped() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let f4 = f.get_base(4, TYPE_FLOAT).unwrap();
    let u = make_union("valU", &[(0, Rc::clone(&i4)), (0, Rc::clone(&f4))]);
    // parentPtr: plain pointer to the union (the container the relptr points into).
    let pu = f.get_type_pointer(8, Rc::clone(&u), 1).unwrap();
    // ptr-rel-to-int: ptrto = i4 (the FIELD type), parent = union, offset 0.
    let relptr = f
        .get_type_pointer_rel(Rc::clone(&pu), Rc::clone(&i4), 0)
        .unwrap();
    assert!(relptr.is_pointer_rel(), "constructed a PointerRel");
    // get_ptr_to() must yield the relative ptrto (i4), NOT the stripped pointer.
    let rel_ptrto = relptr.get_ptr_to().unwrap();
    assert!(Rc::ptr_eq(&rel_ptrto, &i4), "PointerRel get_ptr_to is the field type i4");

    // Receiver: a plain pointer-to-int. ct: the ptr-rel-to-int.
    // C++ TypePointer::findCompatibleResolve(ct): ct is TYPE_PTR ->
    // ptrto(i4).findCompatibleResolve(((TypePointer*)ct)->ptrto == i4).
    let pint = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();
    let got = pint.find_compatible_resolve(&relptr).unwrap();

    // Reference: the same call against a PLAIN pointer-to-int ct must be byte-
    // identical (both read the literal int4 ptrto).
    let plain_int_ct = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();
    let want = pint.find_compatible_resolve(&plain_int_ct).unwrap();
    assert_eq!(got, want, "PointerRel ct must resolve via its relative ptrto, like a plain pointer");
}

/// at2: the STRIPPED-vs-PTRTO mutation, made OBSERVABLE with a concrete index.
///
/// Receiver = a NON-resolution union `{ int4 m0@0; }`.  Its
/// `findCompatibleResolve` (TypeUnion, `!ct.needs_resolution()` arm,
/// type.cc:2629-2649) returns the field INDEX when `field[i].type == ct` by
/// pointer identity.  Drive it with a ptr-rel-to-int ct: the C++
/// `((TypePointer *)ct)->ptrto` is the RELATIVE ptrto (== `i4`, the field's own
/// `Rc`), so the identity match fires and the result is `0` (the field index).
///
/// If the port had read the STRIPPED plain pointer (a distinct `TypePointer`
/// `Rc`, not `i4`) the identity test would miss and the result would be `-1`.
/// So `0` vs `-1` is the live mutation discriminator; we also confirm a plain
/// ptr-to-int ct (built the same way) yields the identical `0`.
#[test]
fn w10_union_value_compat_resolve_ptrto_is_relative_field_not_stripped() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    // A NON-resolution union `{ int4 m0@0; }`: TypeUnion::findCompatibleResolve
    // takes the identity arm (`field[i].type == ct && offset==0` -> index i).
    let mut u = Datatype::new(4, TYPE_UNION);
    u.kind = DatatypeKind::Union { field: vec![TypeField::new(0, 0, "m0", Rc::clone(&i4))] };
    let single_union = Rc::new(u);
    assert!(!single_union.needs_resolution(), "this union must NOT need resolution for the identity arm");

    // RECEIVER = pointer-to-(single-int)-union.  Its Pointer arm recurses:
    // union.findCompatibleResolve(((TypePointer*)ct)->ptrto).
    let pu_recv = f.get_type_pointer(8, Rc::clone(&single_union), 1).unwrap();

    // A throwaway parent pointer for the relptr (parent type is irrelevant here).
    let parent_u = make_union("valU2", &[(0, Rc::clone(&i4))]);
    let pu_parent = f.get_type_pointer(8, Rc::clone(&parent_u), 1).unwrap();
    // ptr-rel whose RELATIVE ptrto is the SAME `i4` Rc as the union field.
    let relptr_to_int = f.get_type_pointer_rel(Rc::clone(&pu_parent), Rc::clone(&i4), 0).unwrap();
    // sanity: the stripped plain pointer is a DISTINCT Rc from the int field type.
    if let Some(st) = relptr_to_int.get_stripped() {
        assert!(!Rc::ptr_eq(&st, &i4), "stripped is a plain pointer, distinct from the int field type");
    }

    // ct = the relptr.  C++ reads ((TypePointer*)ct)->ptrto == i4 (the relative
    // ptrto), recurses union.findCompatibleResolve(i4) -> identity match -> 0.
    // A `stripped`-via read would pass a plain pointer (!= i4) -> -1.
    let via_relptr = pu_recv.find_compatible_resolve(&relptr_to_int).unwrap();
    assert_eq!(via_relptr, 0, "PointerRel ct resolves via its relative ptrto (union field index 0), not stripped (-1)");

    // Reference: a plain ptr-to-int ct (same i4) yields the identical 0.
    let plain_int_ct = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();
    let via_plain = pu_recv.find_compatible_resolve(&plain_int_ct).unwrap();
    assert_eq!(via_plain, via_relptr, "plain Pointer and PointerRel cts resolve identically via ptrto");
}

/// at3: a `PointerRel` RECEIVER recurses on its own relative ptrto, byte-
/// identically to a plain `Pointer` with the same ptrto (the inherited body —
/// no override exists). This is the literal new match-arm (`Pointer | PointerRel`)
/// exercised from the receiver side.
#[test]
fn w10_union_value_compat_resolve_pointerrel_receiver_matches_plain_pointer() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let u = make_union("valU3", &[(0, Rc::clone(&i4))]);
    let pu = f.get_type_pointer(8, Rc::clone(&u), 1).unwrap();
    // RECEIVER = ptr-rel-to-int.
    let relptr = f
        .get_type_pointer_rel(Rc::clone(&pu), Rc::clone(&i4), 0)
        .unwrap();
    // ct = plain ptr-to-int.
    let pint_ct = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();

    let from_rel = relptr.find_compatible_resolve(&pint_ct).unwrap();
    // The inherited TypePointer body: a plain ptr-to-int receiver with the same
    // ptrto must produce the identical result.
    let plain_recv = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();
    let from_plain = plain_recv.find_compatible_resolve(&pint_ct).unwrap();
    assert_eq!(
        from_rel, from_plain,
        "PointerRel receiver dispatches the inherited TypePointer::findCompatibleResolve"
    );
}

/// at4: the union-arm gate is on `ptrto.get_metatype()`, NOT on the pointer
/// kind. A PointerRel-to-NON-union ct: `find_compatible_resolve` against a
/// non-pointer receiver returns -1 (base default); against a pointer receiver it
/// recurses. The gate must behave identically for Pointer and PointerRel cts of
/// the same ptrto. Mutation guard: a non-TYPE_PTR ct (e.g. a bare int) returns
/// -1, proving the `ct.get_metatype() == TYPE_PTR` guard still fires.
#[test]
fn w10_union_value_compat_resolve_non_pointer_ct_returns_minus_one() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let u = make_union("valU4", &[(0, Rc::clone(&i4))]);
    let pu = f.get_type_pointer(8, Rc::clone(&u), 1).unwrap();
    let relptr = f
        .get_type_pointer_rel(Rc::clone(&pu), Rc::clone(&i4), 0)
        .unwrap();
    // A bare (non-pointer) ct: the TYPE_PTR guard fails -> -1, for both kinds.
    let bare_int = f.get_base(4, TYPE_INT).unwrap();
    assert_eq!(
        relptr.find_compatible_resolve(&bare_int).unwrap(),
        -1,
        "PointerRel receiver, non-pointer ct -> -1 (TYPE_PTR guard)"
    );
    let plain = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();
    assert_eq!(
        plain.find_compatible_resolve(&bare_int).unwrap(),
        -1,
        "plain Pointer receiver, non-pointer ct -> -1 (parity)"
    );
}

/// at5: cross-kind recursion. The `((TypePointer*)ct)->ptrto` cast is on the
/// ARGUMENT and works for either ct kind; symmetrically the receiver arm fires
/// for either receiver kind. So Pointer-recv/PointerRel-ct and
/// PointerRel-recv/Pointer-ct of matching ptrto must agree, and both must equal
/// the plain/plain case.
#[test]
fn w10_union_value_compat_resolve_cross_kind_symmetry() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let u = make_union("valU5", &[(0, Rc::clone(&i4))]);
    let pu = f.get_type_pointer(8, Rc::clone(&u), 1).unwrap();
    let relptr = f
        .get_type_pointer_rel(Rc::clone(&pu), Rc::clone(&i4), 0)
        .unwrap();
    let plain = f.get_type_pointer(8, Rc::clone(&i4), 1).unwrap();

    let plain_plain = plain.find_compatible_resolve(&plain).unwrap();
    let plain_rel = plain.find_compatible_resolve(&relptr).unwrap();
    let rel_plain = relptr.find_compatible_resolve(&plain).unwrap();
    let rel_rel = relptr.find_compatible_resolve(&relptr).unwrap();
    assert_eq!(plain_plain, plain_rel, "plain-recv: Pointer ct == PointerRel ct");
    assert_eq!(plain_plain, rel_plain, "PointerRel-recv: Pointer ct == plain-recv");
    assert_eq!(plain_plain, rel_rel, "both PointerRel == both plain");
}

/// Sanity: the union we built really carries `needs_resolution` (the precondition
/// for any of the three dispatchers' union arms to fire on a real pointer-to-
/// union). If this flag is ever dropped (cf. LOSS-184) the whole feature is
/// dormant, so pin it here next to the change.
#[test]
fn w10_union_value_union_carries_needs_resolution_flag() {
    use type_metatype::*;
    let f = factory();
    let i4 = f.get_base(4, TYPE_INT).unwrap();
    let u = make_union("valU6", &[(0, Rc::clone(&i4))]);
    assert_eq!(u.get_metatype(), TYPE_UNION);
    assert!(
        u.needs_resolution(),
        "a completed union must keep needs_resolution (else resolveInFlow never fires)"
    );
    // And a pointer-to-union exposes the union as its ptrto metatype — the gate
    // every dispatcher's union arm checks.
    let pu = f.get_type_pointer(8, Rc::clone(&u), 1).unwrap();
    assert_eq!(pu.get_ptr_to().unwrap().get_metatype(), TYPE_UNION);
    // The PointerRel-to-union likewise: ptrto metatype is UNION (so the new arm
    // reaches the same union gate as a plain pointer).
    let relptr = f.get_type_pointer_rel(Rc::clone(&pu), Rc::clone(&u), 0).unwrap();
    assert!(matches!(relptr.kind, DatatypeKind::PointerRel { .. }));
    assert_eq!(relptr.get_ptr_to().unwrap().get_metatype(), TYPE_UNION);
}
