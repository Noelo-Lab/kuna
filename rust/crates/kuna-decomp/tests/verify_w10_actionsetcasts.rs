//! VERIFIER adversarial fences for item `w10-actionsetcasts` (round 1).
//!
//! Scope note: this branch lands only the **prerequisite** `Funcdata`
//! union-field resolution cache (`kuna_decomp::funcdata_union`, C++
//! `decompiler/cpp/funcdata.cc:915-1115`).  The `ActionSetCasts::apply` body
//! itself is still a seam stub that returns 0 (no CAST/PTRSUB ops emitted), so
//! there is no new datatest parity to fence here.  These tests therefore attack
//! the union-cache accessors at the *edges the porter's own test punted on*:
//!
//!   * the `getUnionField` SIZE guard producing a genuine MISS where the edge
//!     key still HITS (the in-tree test used a tautology there);
//!   * the `MULTIEQUAL` duplicate-slot copy being gated on `slot >= 0` — an
//!     OUTPUT edge (slot -1) must NOT fan out;
//!   * the `MULTIEQUAL` fan-out NOT touching unrelated op edges that merely
//!     share the same union id but a different op time;
//!   * `update`/`emplace` overwrite semantics for an UNLOCKED entry changing the
//!     cached field number (funcdata.cc:984-987 the `update()==true` arm);
//!   * `inheritUnionField`'s `slot < 0 && oldOp->isMarker()` slot rewrite
//!     (funcdata.cc:1082) landing the inherited resolution on slot 0.
//!
//! Every assertion is two-sided (asserts the present AND absent edges) so it
//! cannot be satisfied by a no-op accessor.

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
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
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
    let glb = Rc::new(Architecture::new(manage));
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

/// A union of (offset, type) fields (C++ `TypeUnion`).
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

fn new_op_code(fd: &mut Funcdata, inputs: int4, opc: OpCode, name: &str, off: int4) -> OpId {
    new_op_code_flags(fd, inputs, opc, name, off, 0)
}

/// Like [`new_op_code`] but threads explicit PcodeOp flag bits into the W6
/// `TypeOp` seam.  The seam does not auto-derive `PcodeOp` flags from the
/// op-code (it carries `opcode`/`flags`/`name` verbatim), so a test that needs
/// `isMarker()` must request the `marker` bit here — exactly the bit the C++
/// `setOpcode` ORs in from `behave->getFlags()` for a MULTIEQUAL.
fn new_op_code_flags(
    fd: &mut Funcdata,
    inputs: int4,
    opc: OpCode,
    name: &str,
    off: int4,
    flags: kuna_base::types::uint4,
) -> OpId {
    let pc = Address::new(ram(fd), 0x2000 + off as u64);
    let op = fd.new_op(inputs, pc);
    fd.op_set_opcode(op, TypeOp::new(opc, flags, name));
    op
}

fn resolve_field(u: &Rc<Datatype>, fld: int4) -> ResolvedUnion {
    let factory = TypeFactoryImpl::new();
    ResolvedUnion::new_field(Rc::clone(u), fld, &factory).expect("resolve field")
}

// ===========================================================================
// (1) getUnionField SIZE guard: a GENUINE miss where the edge key still HITS.
//     funcdata.cc:924-926 `if (unresType->getSize() == dt->getSize())`.
//
// The in-tree test (`w10_union_get_field_size_match_and_mismatch`) asserted a
// tautology for the mismatch arm.  Here we force the resolved data-type size to
// DIFFER from the unresolved union size while keeping the SAME union id, so the
// `unionMap.find` succeeds and only the size guard rejects.
// ===========================================================================

#[test]
fn w10_get_union_field_size_guard_rejects_but_resolution_still_hits() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    // Union of size 8 whose field-0 is a size-4 int.  resolve_field(.,0) ->
    // resolve dt is the size-4 int field; the union itself is size 8.
    let u8 = make_union(8, &[(0, Rc::clone(&int4t))]);
    let op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY", 0);

    let res = resolve_field(&u8, 0);
    assert_eq!(res.get_datatype().get_size(), 4, "resolved field dt is size 4");
    assert_eq!(u8.get_size(), 8, "the union being resolved is size 8");
    assert!(fd.set_union_field(&u8, op, 0, res), "unlocked first set");

    // Same union id => the edge HITS in unionMap, but getSize() 8 != 4 => the
    // size guard rejects: getUnionField returns None.
    assert!(
        fd.get_union_field(&u8, op, 0).is_none(),
        "size guard rejects (8 != 4) even though the edge key matches (funcdata.cc:925)"
    );
    // getUnionResolution ignores the size guard => it MUST return the entry,
    // proving the miss above was the guard, not a missing edge.
    assert!(
        fd.get_union_resolution(&u8, op, 0).is_some(),
        "getUnionResolution returns the same entry regardless of size (funcdata.cc:945)"
    );
}

// ===========================================================================
// (2) MULTIEQUAL fan-out is gated on `slot >= 0` (funcdata.cc:988).
//     An OUTPUT edge (slot == -1) must NOT copy to other slots, even when other
//     slots hold the same Varnode.
// ===========================================================================

#[test]
fn w10_multiequal_output_edge_does_not_fan_out() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let u = make_union(4, &[(0, int4t)]);

    let phi = new_op_code(&mut fd, 2, OpCode::CPUI_MULTIEQUAL, "MULTIEQUAL", 0x10);
    let r = ram(&fd);
    let same: VarnodeId = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x300), None);
    fd.obank_mut().get_mut(phi).unwrap().set_input(Some(same), 0);
    fd.obank_mut().get_mut(phi).unwrap().set_input(Some(same), 1);

    // Set on the OUTPUT edge (slot -1): the `slot >= 0` guard is false, so NO
    // fan-out happens to slots 0/1 even though both hold `same`.
    let res = resolve_field(&u, 0);
    assert!(fd.set_union_field(&u, phi, -1, res));

    assert!(fd.get_union_resolution(&u, phi, -1).is_some(), "output edge is set");
    assert!(
        fd.get_union_resolution(&u, phi, 0).is_none(),
        "slot 0 NOT touched by an output-edge set (funcdata.cc:988 slot>=0 guard)"
    );
    assert!(
        fd.get_union_resolution(&u, phi, 1).is_none(),
        "slot 1 NOT touched by an output-edge set (funcdata.cc:988 slot>=0 guard)"
    );
}

// ===========================================================================
// (3) MULTIEQUAL fan-out is op-LOCAL: it must not bleed onto a DIFFERENT op that
//     shares the same union id but has a different op time (the edge `opTime`
//     component distinguishes them — funcdata.cc:991-1001 iterate `op->numInput`
//     of THIS op only).
// ===========================================================================

#[test]
fn w10_multiequal_fanout_is_op_local() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let u = make_union(4, &[(0, int4t)]);

    let phi = new_op_code(&mut fd, 2, OpCode::CPUI_MULTIEQUAL, "MULTIEQUAL", 0x20);
    let other_phi = new_op_code(&mut fd, 2, OpCode::CPUI_MULTIEQUAL, "MULTIEQUAL", 0x28);
    let r = ram(&fd);
    let same: VarnodeId = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x400), None);
    for slot in 0..2 {
        fd.obank_mut().get_mut(phi).unwrap().set_input(Some(same), slot);
        fd.obank_mut().get_mut(other_phi).unwrap().set_input(Some(same), slot);
    }

    let res = resolve_field(&u, 0);
    assert!(fd.set_union_field(&u, phi, 0, res));

    // phi's own slots 0 and 1 are populated...
    assert!(fd.get_union_resolution(&u, phi, 0).is_some(), "phi slot 0 set");
    assert!(fd.get_union_resolution(&u, phi, 1).is_some(), "phi slot 1 fanned out");
    // ...but `other_phi` (different opTime) is entirely untouched.
    assert!(
        fd.get_union_resolution(&u, other_phi, 0).is_none(),
        "a different MULTIEQUAL op is not affected by phi's fan-out (opTime distinguishes edges)"
    );
    assert!(
        fd.get_union_resolution(&u, other_phi, 1).is_none(),
        "a different MULTIEQUAL op is not affected by phi's fan-out (opTime distinguishes edges)"
    );
}

// ===========================================================================
// (4) UNLOCKED overwrite actually changes the cached field number
//     (funcdata.cc:984-987: emplace finds existing, update() returns true =>
//     setUnionField returns true and the new field sticks).
// ===========================================================================

#[test]
fn w10_unlocked_overwrite_changes_field_number() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let floatt = base(4, type_metatype::TYPE_FLOAT);
    let u = make_union(4, &[(0, int4t), (0, floatt)]);
    let op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY", 0x30);

    let f0 = resolve_field(&u, 0);
    assert!(fd.set_union_field(&u, op, 0, f0), "first set true");
    assert_eq!(fd.get_union_resolution(&u, op, 0).unwrap().get_field_num(), 0);

    // Overwrite with a DIFFERENT field on an UNLOCKED entry: update() returns
    // true, setUnionField returns true, and field number flips to 1.
    let f1 = resolve_field(&u, 1);
    assert!(
        fd.set_union_field(&u, op, 0, f1),
        "unlocked overwrite returns true (funcdata.cc:984-987 update()==true arm)"
    );
    assert_eq!(
        fd.get_union_resolution(&u, op, 0).unwrap().get_field_num(),
        1,
        "the cached field number is now the overwriting field"
    );

    // Re-setting the SAME field is a no-op update (update() returns false) but
    // still returns true (not locked) — funcdata.cc:985-986.
    let f1_again = resolve_field(&u, 1);
    assert!(fd.set_union_field(&u, op, 0, f1_again), "no-op same-field set still returns true");
    assert_eq!(fd.get_union_resolution(&u, op, 0).unwrap().get_field_num(), 1);
}

// ===========================================================================
// (5) inheritUnionField slot rewrite: `slot < 0 && oldOp->isMarker()` => slot 0
//     (funcdata.cc:1082).  A MULTIEQUAL is a marker, so inheriting onto a write
//     edge (slot -1) from a marker old-op lands the resolution on slot 0.
// ===========================================================================

#[test]
fn w10_inherit_marker_rewrites_write_slot_to_zero() {
    let mut fd = build_fd();
    let int4t = base(4, type_metatype::TYPE_INT);
    let floatt = base(4, type_metatype::TYPE_FLOAT);
    let u = make_union(4, &[(0, int4t), (0, floatt)]);

    // old_op is a MULTIEQUAL (isMarker() == true); seed its slot-0 read edge.
    // The marker bit must be set explicitly (the seam does not derive it).
    let old_op = new_op_code_flags(
        &mut fd,
        1,
        OpCode::CPUI_MULTIEQUAL,
        "MULTIEQUAL",
        0x40,
        kuna_decomp::op::pcodeop_flags::marker,
    );
    let new_op = new_op_code(&mut fd, 1, OpCode::CPUI_COPY, "COPY", 0x48);
    let seed = resolve_field(&u, 1);
    assert!(fd.set_union_field(&u, old_op, 0, seed));

    // Inherit onto new_op with slot == -1.  Because old_op is a marker, the slot
    // is rewritten to 0 before the setUnionField, so the resolution lands on the
    // slot-0 read edge of new_op, NOT the -1 write edge.
    let got = fd.inherit_union_field(&u, new_op, -1, old_op, 0);
    assert_eq!(got, 1, "inherited field number is returned");
    assert!(
        fd.get_union_resolution(&u, new_op, 0).is_some(),
        "marker rewrite landed the resolution on slot 0 (funcdata.cc:1082)"
    );
    assert!(
        fd.get_union_resolution(&u, new_op, -1).is_none(),
        "the -1 write edge was rewritten to 0, so it is NOT populated"
    );
}
