//! VERIFIER adversarial tests for `w10-union-render` (round 1).
//!
//! These target the spot the hunt list flagged as most fragile: the
//! `scoreTrialUp` opcode partition.  The C++ `scoreTrialUp` switch
//! (`unionresolve.cc:735-916`) handles `CPUI_PTRADD` with a **live varnode read**
//! (`def->getIn(2)->getOffset()`, lines 900-912), so it can NOT be a pure
//! metatype->score table arm.  The Rust port correctly returns `None` from
//! `score_trial_up_pure` for `CPUI_PTRADD` (routing it to the driver), but the
//! driver `match` in `ScoreUnionFields::score_trial_up`
//! (`unionresolve_run.rs:767-801`) has **no `CPUI_PTRADD` arm** — so a PTRADD-def
//! up-trial falls into `_ => score = -10`, the opposite sign of the C++ result for
//! a `TYPE_PTR`/`TYPE_INT` field.
//!
//! `w10_ptradd_up_ptr_field_scores_positive_like_cpp` reproduces the divergence
//! through the public `ScoreUnionFields::new_edge(slot = -1)` entry: the union is
//! the OUTPUT of a PTRADD, so every field becomes an up-trial whose `def` is the
//! PTRADD.  C++ scores the pointer field positively (it should beat the whole-union
//! baseline of `scores[0] = -1`); the buggy Rust scores it `-10` and the whole
//! union wins instead.  This test asserts the C++ (correct) outcome, so it FAILS
//! against the current branch — the failing test the REJECT requires.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactoryImpl, TypeField};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};
use kuna_decomp::unionresolve_run::ScoreUnionFields;

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

fn build_fd() -> (Funcdata, Rc<TypeFactoryImpl>) {
    let manage = build_manager();
    let mut arch = Architecture::new(manage);
    let tf = Rc::new(TypeFactoryImpl::new());
    arch.types = Some(Rc::clone(&tf));
    let glb = Rc::new(arch);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    (Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap(), tf)
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn base(size: int4, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, m))
}

/// A pointer data-type (TYPE_PTR, 8 bytes) to `ptrto`, align_size = 8.
fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
    let mut p = Datatype::new_with_align(8, 8, type_metatype::TYPE_PTR);
    p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(p)
}

fn make_union(size: int4, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut u = Datatype::new(size, type_metatype::TYPE_UNION);
    u.flags |= 0x800; // needs_resolution
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ct))| TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ct)))
        .collect();
    u.kind = DatatypeKind::Union { field };
    Rc::new(u)
}

fn new_op_code(fd: &mut Funcdata, inputs: int4, opc: OpCode, name: &str) -> OpId {
    let pc = Address::new(ram(fd), 0x2000);
    let op = fd.new_op(inputs, pc);
    fd.op_set_opcode(op, TypeOp::new(opc, 0, name));
    op
}

fn wire_input(
    fd: &mut Funcdata,
    op: OpId,
    slot: int4,
    off: u64,
    size: int4,
    ty: Rc<Datatype>,
) -> VarnodeId {
    let r = ram(fd);
    let m = Address::new(r, off);
    let vn = fd.new_varnode(size, &m, Some(ty));
    fd.op_set_input(op, vn, slot).unwrap();
    vn
}

/// Build a constant Varnode (constant space) of `size` bytes holding `val`.
fn const_vn(fd: &mut Funcdata, val: uintb, size: int4) -> VarnodeId {
    let cspace = Rc::clone(fd.get_arch().manage().get_constant_space().unwrap());
    let m = Address::new(cspace, val);
    fd.new_varnode(size, &m, None)
}

// ===========================================================================
// F1: scoreTrialUp has no CPUI_PTRADD arm; a pointer field defined by PTRADD
// scores -10 (wrong sign) instead of the C++ +10/+2.
// ===========================================================================

#[test]
fn w10_ptradd_up_ptr_field_scores_positive_like_cpp() {
    let (mut fd, tf) = build_fd();

    // union { int8 f0; void* f1; }  (both 8 bytes, offset 0).
    let int8t = base(8, type_metatype::TYPE_INT);
    let voidt = base(1, type_metatype::TYPE_UNKNOWN);
    let ptrt = ptr_to(Rc::clone(&voidt)); // align_size(ptrto) = 1
    let u = make_union(8, &[(0, Rc::clone(&int8t)), (0, Rc::clone(&ptrt))]);

    // out = PTRADD(arrbase, index, elsize)  with out typed as the union.
    let op = new_op_code(&mut fd, 3, OpCode::CPUI_PTRADD, "PTRADD");
    let _arrbase = wire_input(&mut fd, op, 0, 0x400, 8, Rc::clone(&ptrt));
    let _index = {
        let c = const_vn(&mut fd, 2, 8);
        fd.op_set_input(op, c, 1).unwrap();
        c
    };
    // in(2) = elsize constant == align_size(ptrto) == 1  -> C++ PTRADD-up gives +10.
    let _elsize = {
        let c = const_vn(&mut fd, 1, 8);
        fd.op_set_input(op, c, 2).unwrap();
        c
    };
    let r = ram(&fd);
    let outm = Address::new(r, 0x500);
    let out = fd.new_varnode(8, &outm, Some(Rc::clone(&u)));
    fd.op_set_output(op, out).unwrap();

    // Score the union against the WRITE edge (slot = -1): each field becomes an
    // up-trial whose def is the PTRADD.
    let result = {
        let scorer = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1).unwrap();
        scorer.into_result()
    };

    // C++ truth: the pointer field (index 1) is fitted up through PTRADD and
    // scores +10 (align_size(ptrto)=1 == in(2).offset=1), comfortably beating the
    // whole-union baseline (scores[0] starts at -1 for a PTRADD edge).  So the
    // resolution MUST pick the pointer field, not the whole union.
    //
    // The branch's `score_trial_up` has no PTRADD arm and scores the pointer field
    // -10, so the whole union (fieldNum == -1) wins instead.  This assertion fails
    // on the branch, documenting the divergence.
    assert_eq!(
        result.get_field_num(),
        1,
        "F1: a TYPE_PTR field defined by PTRADD must win via the C++ scoreTrialUp \
         PTRADD arm (+10); the branch scores it -10 (missing arm) so the union \
         (fieldNum {}) wins instead",
        result.get_field_num()
    );
}

// ===========================================================================
// Control: the same union written by a non-PTRADD op the driver DOES handle
// resolves correctly — isolating the defect to the missing PTRADD-up arm.
// ===========================================================================

#[test]
fn w10_int_field_up_through_int_add_resolves_control() {
    let (mut fd, tf) = build_fd();
    // union { int8 f0; float8 f1; }
    let int8t = base(8, type_metatype::TYPE_INT);
    let float8t = base(8, type_metatype::TYPE_FLOAT);
    let u = make_union(8, &[(0, Rc::clone(&int8t)), (0, Rc::clone(&float8t))]);

    // out = INT_ADD(a, b), out typed as the union (slot = -1 up-trials).
    let op = new_op_code(&mut fd, 2, OpCode::CPUI_INT_ADD, "INT_ADD");
    let _a = wire_input(&mut fd, op, 0, 0x600, 8, Rc::clone(&int8t));
    let _b = wire_input(&mut fd, op, 1, 0x608, 8, Rc::clone(&int8t));
    let r = ram(&fd);
    let outm = Address::new(r, 0x700);
    let out = fd.new_varnode(8, &outm, Some(Rc::clone(&u)));
    fd.op_set_output(op, out).unwrap();

    let result = {
        let scorer = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1).unwrap();
        scorer.into_result()
    };
    // INT_ADD-up: int field scores +5, float field scores -5 -> int field wins.
    assert_eq!(
        result.get_field_num(),
        0,
        "control: INT_ADD-up (a driver-handled arm) resolves to the int field"
    );
}

// ===========================================================================
// Order/boundary: an empty-input PTRADD edge must not panic (boundary input the
// hunt list flags for the live-read arms).
// ===========================================================================

#[test]
fn w10_ptradd_up_missing_inputs_does_not_panic() {
    let (mut fd, tf) = build_fd();
    let int8t = base(8, type_metatype::TYPE_INT);
    let ptrt = ptr_to(base(1, type_metatype::TYPE_UNKNOWN));
    let u = make_union(8, &[(0, int8t), (0, ptrt)]);

    // A PTRADD with only slot 0 wired (no index / no elsize) — the live reads of
    // in(1)/in(2) must degrade gracefully, not panic.
    let op = new_op_code(&mut fd, 3, OpCode::CPUI_PTRADD, "PTRADD");
    let _arrbase = wire_input(&mut fd, op, 0, 0x800, 8, Rc::clone(&u));
    let r = ram(&fd);
    let outm = Address::new(r, 0x900);
    let out = fd.new_varnode(8, &outm, Some(Rc::clone(&u)));
    fd.op_set_output(op, out).unwrap();

    // Just must not panic.
    let _ = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1);
}
