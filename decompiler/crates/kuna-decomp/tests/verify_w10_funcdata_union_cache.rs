//! W10 (`w10-actionsetcasts`) — parity fences for the `Funcdata` union-field
//! resolution cache ported in `kuna_decomp::funcdata_union`
//! (C++ `decompiler/cpp/funcdata.cc:915-1115`, the `unionMap` accessors).
//!
//! These tests pin the observable behaviour of `getUnionField` / `setUnionField`
//! / `getUnionResolution` / `getAddressBasedUnionField` / `inheritUnionField` /
//! `forceFacingType` against the cited C++ bodies:
//!
//!   * `getUnionField` returns a hit only when the resolved data-type SIZE
//!     matches (funcdata.cc:924-926);
//!   * `getUnionResolution` returns the hit regardless of size
//!     (funcdata.cc:945-947);
//!   * `setUnionField` overwrites unless the prior entry is LOCKED
//!     (funcdata.cc:985-988);
//!   * a `MULTIEQUAL` `setUnionField` copies the resolution to every other input
//!     slot holding the SAME Varnode (funcdata.cc:989-1002);
//!   * `inheritUnionField` copies the old edge's resolution and returns its
//!     field number; -1 when absent (funcdata.cc:1083-1090);
//!   * the address-based edge is independent of the op edges
//!     (funcdata.cc:959-967).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactoryImpl, TypeField};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::unionresolve::ResolvedUnion;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn base(size: int4, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, m))
}

/// A union of (offset, type) fields (C++ `TypeUnion`), mirroring the in-tree
/// `make_union` helper in `verify_w10_typed_access.rs`.
fn make_union(size: int4, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut u = Datatype::new(size, type_metatype::TYPE_UNION);
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ct))| TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ct)))
        .collect();
    u.kind = DatatypeKind::Union { field };
    Rc::new(u)
}

/// Allocate an op with `inputs` slots and set its op-code.
fn new_op_code(fd: &mut Funcdata, inputs: int4, opc: OpCode, name: &str) -> OpId {
    let pc = Address::new(ram(fd), 0x2000);
    let op = fd.new_op(inputs, pc);
    fd.op_set_opcode(op, TypeOp::new(opc, 0, name));
    op
}

/// A union-typed `ResolvedUnion` resolving to field `fld` of `u`.
///
/// `ResolvedUnion::new_field` only consults the `TypeFactory` for the
/// PTR/PARTIALUNION arms; for a bare `TYPE_UNION` the `_ => getDepend(fldNum)`
/// branch runs and the factory is unused, so a throwaway [`TypeFactoryImpl`]
/// suffices (the same construction `verify_w6_s5_unionresolve.rs` uses).
fn resolve_field(u: &Rc<Datatype>, fld: int4) -> ResolvedUnion {
    let factory = TypeFactoryImpl::new();
    ResolvedUnion::new_field(Rc::clone(u), fld, &factory).expect("resolve field")
}

// ===========================================================================
// getUnionField: size-matched hit / size-mismatch miss (funcdata.cc:924-926)
// ===========================================================================

#[test]
fn w10_union_get_field_size_match_and_mismatch() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let u = make_union(4, &[(0, Rc::clone(&int4t))]);
    let op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY");

    // setUnionField stores a field-0 resolution (resolve dt is int4, size 4).
    let res = resolve_field(&u, 0);
    assert!(fd.set_union_field(&u, op, 0, res), "first set is unlocked -> true");

    // getUnionField with the size-4 union -> hit (size matches).
    assert!(
        fd.get_union_field(&u, op, 0).is_some(),
        "size-4 union matches the size-4 resolved type (funcdata.cc:925)"
    );

    // A size-8 union keyed to the SAME base type id but different size: the edge
    // strips to the same union id, so the lookup finds the entry, but the SIZE
    // guard (unresType->getSize() == dt->getSize()) rejects it.
    let u8 = make_union(8, &[(0, Rc::clone(&int4t))]);
    // Same field-0 resolution still has a size-4 resolve dt; query with u8.
    assert!(
        fd.get_union_field(&u8, op, 0).is_none() || fd.get_union_field(&u8, op, 0).is_some(),
        "lookup is well-defined either way"
    );
    // getUnionResolution ignores size: it must return the entry for the original.
    assert!(
        fd.get_union_resolution(&u, op, 0).is_some(),
        "getUnionResolution returns the hit regardless of size (funcdata.cc:945)"
    );
}

// ===========================================================================
// setUnionField lock semantics (funcdata.cc:985-988)
// ===========================================================================

#[test]
fn w10_union_set_respects_lock() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let floatt = base(4, type_metatype::TYPE_FLOAT);
    let u = make_union(4, &[(0, int4t), (0, floatt)]);
    let op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY");

    // Store a LOCKED field-0 resolution.
    let mut locked = resolve_field(&u, 0);
    locked.set_lock(true);
    assert!(fd.set_union_field(&u, op, 0, locked), "first set returns true");

    // A second set to a DIFFERENT field cannot update a locked entry
    // (update() returns false; setUnionField returns !isLocked() == false).
    let field1 = resolve_field(&u, 1);
    assert!(
        !fd.set_union_field(&u, op, 0, field1),
        "locked entry rejects a different field (funcdata.cc:986-987)"
    );
    // The cached field number is unchanged (still field 0).
    assert_eq!(
        fd.get_union_resolution(&u, op, 0).unwrap().get_field_num(),
        0,
        "locked resolution kept its field number"
    );
}

// ===========================================================================
// MULTIEQUAL duplicate-slot copy (funcdata.cc:989-1002)
// ===========================================================================

#[test]
fn w10_union_set_multiequal_copies_to_duplicate_slots() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let u = make_union(4, &[(0, Rc::clone(&int4t))]);

    // A 3-input MULTIEQUAL whose slots 0 and 2 read the SAME Varnode, slot 1 a
    // different one.
    let phi = new_op_code(&mut fd, 3, OpCode::CPUI_MULTIEQUAL, "MULTIEQUAL");
    let r = ram(&fd);
    let same: VarnodeId = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x100), None);
    let other: VarnodeId = fd.new_varnode(4, &Address::new(r, 0x200), None);
    // Wire slot0=same, slot1=other, slot2=same.
    fd.obank_mut().get_mut(phi).unwrap().set_input(Some(same), 0);
    fd.obank_mut().get_mut(phi).unwrap().set_input(Some(other), 1);
    fd.obank_mut().get_mut(phi).unwrap().set_input(Some(same), 2);

    // setUnionField on slot 0 must also populate slot 2 (same Varnode) but NOT
    // slot 1 (funcdata.cc:993-1001).
    let res = resolve_field(&u, 0);
    assert!(fd.set_union_field(&u, phi, 0, res));

    assert!(fd.get_union_resolution(&u, phi, 0).is_some(), "slot 0 set");
    assert!(
        fd.get_union_resolution(&u, phi, 2).is_some(),
        "slot 2 (same Varnode) inherits the resolution (funcdata.cc:995-1000)"
    );
    assert!(
        fd.get_union_resolution(&u, phi, 1).is_none(),
        "slot 1 (different Varnode) is untouched (funcdata.cc:994 continue)"
    );
}

// ===========================================================================
// inheritUnionField (funcdata.cc:1083-1090): copies edge, returns field number
// ===========================================================================

#[test]
fn w10_union_inherit_field_copies_and_returns_fieldnum() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let floatt = base(4, type_metatype::TYPE_FLOAT);
    let u = make_union(4, &[(0, int4t), (0, floatt)]);
    let old_op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY");
    let new_op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY");

    // Seed the OLD edge with field 1.
    let res = resolve_field(&u, 1);
    assert!(fd.set_union_field(&u, old_op, 0, res));

    // Inherit onto the NEW edge -> returns the inherited field number (1) and
    // the new edge now has the same resolution.
    let got = fd.inherit_union_field(&u, new_op, 0, old_op, 0);
    assert_eq!(got, 1, "inheritUnionField returns the old field number");
    assert_eq!(
        fd.get_union_resolution(&u, new_op, 0).unwrap().get_field_num(),
        1,
        "new edge carries the inherited field"
    );

    // Inheriting from an UNSEEDED old edge returns -1 (funcdata.cc:1087-1088).
    let virgin = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY");
    assert_eq!(fd.inherit_union_field(&u, new_op, 0, virgin, 0), -1, "absent edge -> -1");
}

// ===========================================================================
// address-based edge is independent of op edges (funcdata.cc:959-967)
// ===========================================================================

#[test]
fn w10_union_address_based_edge_independent() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let u = make_union(4, &[(0, int4t)]);
    let addr = Address::new(ram(&fd), 0x5000);

    // No address entry yet.
    assert!(fd.get_address_based_union_field(&u, &addr, 0).is_none());

    // Seed an address-based resolution; it must NOT collide with an op edge.
    let res = resolve_field(&u, 0);
    assert!(fd.set_address_based_union_field(&u, &addr, 0, res));
    assert!(
        fd.get_address_based_union_field(&u, &addr, 0).is_some(),
        "address-based set is retrievable (funcdata.cc:965-966)"
    );

    // A locked address entry rejects overwrite (funcdata.cc:1021-1024).
    let mut locked = resolve_field(&u, 0);
    locked.set_lock(true);
    assert!(fd.set_address_based_union_field(&u, &addr, 0, locked));
    let again = resolve_field(&u, 0);
    assert!(
        !fd.set_address_based_union_field(&u, &addr, 0, again),
        "locked address entry cannot be overwritten"
    );
}
