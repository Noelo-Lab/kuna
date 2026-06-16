//! VERIFIER adversarial tests for `w10-union-render` (round 2).
//!
//! Round 1 REJECTed on F1: `scoreTrialUp` had no `CPUI_PTRADD` arm, so a union
//! field defined by `CPUI_PTRADD` fell into `_ => -10` instead of the C++
//! `{10, 2, -5, 1}`-by-metatype scoring (`unionresolve.cc:900-912`).  The branch
//! now adds the arm (commit `901ccef`).  These round-2 tests pin the *sub-cases*
//! of that arm — the round-1 repro only exercised the align-MATCH `TYPE_PTR`
//! branch (+10).  A faithful arm must also reproduce:
//!
//!   * the align-MISMATCH `TYPE_PTR` branch (+2, still positive, still beats the
//!     -1 union baseline for a PTRADD edge → the pointer field still wins);
//!   * the non-pointer "doesn't fit" branch (`TYPE_FLOAT`/`TYPE_STRUCT`/… = -5),
//!     so a FLOAT field defined by PTRADD does NOT win (the union baseline wins);
//!   * the `scoreTrialUp` `CPUI_SUBPIECE` arm (`unionresolve.cc:890-898`), the
//!     other live-read up arm the F1 fix shipped alongside PTRADD.
//!
//! All three assert the C++ outcome and PASS on the fixed branch.

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

/// A pointer (TYPE_PTR, 8 bytes) to `ptrto`; align_size taken from `ptrto`.
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

fn const_vn(fd: &mut Funcdata, val: uintb, size: int4) -> VarnodeId {
    let cspace = Rc::clone(fd.get_arch().manage().get_constant_space().unwrap());
    let m = Address::new(cspace, val);
    fd.new_varnode(size, &m, None)
}

fn set_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4, ty: Rc<Datatype>) {
    let r = ram(fd);
    let m = Address::new(r, off);
    let out = fd.new_varnode(size, &m, Some(ty));
    fd.op_set_output(op, out).unwrap();
}

// ===========================================================================
// R2-A: PTRADD-up, align MISMATCH — C++ `score = 2` (unionresolve.cc:904-905).
// Still positive, so the pointer field still beats the -1 union baseline.  A
// regressed arm that hardcoded only the +10 (align-match) branch, or fell back
// to -10, would make the union win instead.
// ===========================================================================

#[test]
fn w10r2_ptradd_up_align_mismatch_still_scores_positive() {
    let (mut fd, tf) = build_fd();
    // union { int8 f0; void* f1; }  — ptrto align_size = 1.
    let int8t = base(8, type_metatype::TYPE_INT);
    let ptrt = ptr_to(base(1, type_metatype::TYPE_UNKNOWN)); // align_size(ptrto)=1
    let u = make_union(8, &[(0, Rc::clone(&int8t)), (0, Rc::clone(&ptrt))]);

    let op = new_op_code(&mut fd, 3, OpCode::CPUI_PTRADD, "PTRADD");
    let _arrbase = wire_input(&mut fd, op, 0, 0x400, 8, Rc::clone(&ptrt));
    let _index = {
        let c = const_vn(&mut fd, 2, 8);
        fd.op_set_input(op, c, 1).unwrap();
        c
    };
    // in(2) = elsize 8 != align_size(ptrto)=1  -> C++ PTRADD-up gives +2 (NOT 10).
    let _elsize = {
        let c = const_vn(&mut fd, 8, 8);
        fd.op_set_input(op, c, 2).unwrap();
        c
    };
    set_output(&mut fd, op, 0x500, 8, Rc::clone(&u));

    let result = {
        let scorer = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1).unwrap();
        scorer.into_result()
    };
    // +2 for the pointer field > -1 union baseline (and the int field, fitting up
    // through PTRADD as a non-PTR, scores -5).  Pointer field (index 1) wins.
    assert_eq!(
        result.get_field_num(),
        1,
        "PTRADD-up align-mismatch must still score the pointer field +2 (> -1 union \
         baseline); got fieldNum {}",
        result.get_field_num()
    );
}

// ===========================================================================
// R2-B: PTRADD-up, a pointer field vs a FLOAT field — the FLOAT field hits the
// C++ "doesn't fit" branch `score = -5` (ARRAY|STRUCT|UNION|CODE|FLOAT,
// unionresolve.cc:908-909) while the pointer field hits the +10/+2 branch.  The
// pointer field must win; the FLOAT field must NOT.  A regressed arm that scored
// every PTRADD-up field positively (or that swapped the metatype partition) would
// pick the float field or the union.  This isolates the *sign* of the -5 branch.
//
// (Whole-union baseline: scores[0] = -1 (PTRADD edge) plus the union's own
// up-trial through PTRADD = TYPE_UNION = -5, so -6 — below both fields; the
// pointer field's +10 is the unambiguous winner.)
// ===========================================================================

#[test]
fn w10r2_ptradd_up_ptr_field_beats_float_field() {
    let (mut fd, tf) = build_fd();
    // union { float8 f0; void* f1; } — float scores -5, ptr scores +10 up PTRADD.
    let float8t = base(8, type_metatype::TYPE_FLOAT);
    let ptrt = ptr_to(base(1, type_metatype::TYPE_UNKNOWN)); // align_size(ptrto)=1
    let u = make_union(8, &[(0, Rc::clone(&float8t)), (0, Rc::clone(&ptrt))]);

    let op = new_op_code(&mut fd, 3, OpCode::CPUI_PTRADD, "PTRADD");
    let _arrbase = wire_input(&mut fd, op, 0, 0xA00, 8, Rc::clone(&ptrt));
    let _index = {
        let c = const_vn(&mut fd, 1, 8);
        fd.op_set_input(op, c, 1).unwrap();
        c
    };
    // in(2) = elsize 1 == align_size(ptrto)=1  -> ptr field PTRADD-up = +10.
    let _elsize = {
        let c = const_vn(&mut fd, 1, 8);
        fd.op_set_input(op, c, 2).unwrap();
        c
    };
    set_output(&mut fd, op, 0xB00, 8, Rc::clone(&u));

    let result = {
        let scorer = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1).unwrap();
        scorer.into_result()
    };
    // float field f0 = -5 (the "doesn't fit" branch); ptr field f1 = +10.  The
    // pointer field (index 1) wins decisively — the -5 branch keeps the float
    // field out of contention.
    assert_eq!(
        result.get_field_num(),
        1,
        "PTRADD-up: the pointer field (+10) must beat the FLOAT field (-5, the \
         doesn't-fit branch); got fieldNum {}",
        result.get_field_num()
    );
}

// ===========================================================================
// R2-C: SUBPIECE-up, the *other* live-read up arm shipped with the F1 fix
// (unionresolve.cc:890-898).  A union field defined by SUBPIECE with in(1)
// offset 0 scores +3 for an INT/UINT/BOOL field (likely truncation), so the int
// field wins over a non-int field that scores -5.
// ===========================================================================

#[test]
fn w10r2_subpiece_up_int_field_scores_likely_truncation() {
    let (mut fd, tf) = build_fd();
    // union { int4 f0; float4 f1; } — 4 bytes each.
    let int4t = base(4, type_metatype::TYPE_INT);
    let float4t = base(4, type_metatype::TYPE_FLOAT);
    let u = make_union(4, &[(0, Rc::clone(&int4t)), (0, Rc::clone(&float4t))]);

    // out = SUBPIECE(src, 0) ; out typed as the union (slot = -1 up-trials).
    let op = new_op_code(&mut fd, 2, OpCode::CPUI_SUBPIECE, "SUBPIECE");
    let _src = wire_input(&mut fd, op, 0, 0xC00, 8, base(8, type_metatype::TYPE_INT));
    // in(1) = truncation offset 0  -> C++ SUBPIECE-up gives +3 for INT/UINT/BOOL.
    let _off = {
        let c = const_vn(&mut fd, 0, 8);
        fd.op_set_input(op, c, 1).unwrap();
        c
    };
    set_output(&mut fd, op, 0xD00, 4, Rc::clone(&u));

    let result = {
        let scorer = ScoreUnionFields::new_edge(&fd, tf.as_ref(), Rc::clone(&u), op, -1).unwrap();
        scorer.into_result()
    };
    // int field: +3 (SUBPIECE-up, offset 0, INT). float field: -5. union baseline
    // for a SUBPIECE edge: wordSize==0 -> scores[0] -= 1 => -1.  Int field wins.
    assert_eq!(
        result.get_field_num(),
        0,
        "SUBPIECE-up (offset 0) must score the INT field +3 (likely truncation) and \
         win; got fieldNum {}",
        result.get_field_num()
    );
}
