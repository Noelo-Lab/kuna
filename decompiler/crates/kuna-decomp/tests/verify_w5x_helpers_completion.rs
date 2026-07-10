//! Adversarial verification tests for item `w5x-helpers-completion`
//! (completion of the `funcdata_op.cc` helpers the W5 wave left seamed:
//! `replaceLessequal`, `distributeIntMultAdd`, `cseElimination`/`cseEliminateList`,
//! and the `opSetInput` constant re-duplication guard).
//!
//! Written by the INDEPENDENT verifier (round 1).  These tests target the spots
//! the hunt list flagged as most fragile for THIS item — boundary/overflow inputs
//! and order-sensitive container traversals — that the porter's own tests did not
//! exercise:
//!
//!   1. `replaceLessequal` SIGNED overflow guard, `c <= x` form (const in slot 0,
//!      diff = -1): `INT_SMIN <= x` must DECLINE (cannot subtract 1 without
//!      signed underflow).  The companion `c <= x` non-boundary case rewrites.
//!   2. `replaceLessequal` UNSIGNED overflow guards: `0 <= x` (diff = -1, val = 0)
//!      declines; `x <= UINT_MAX` (diff = +1, val = calc_uint_max) declines.
//!   3. `cseEliminateList` off-by-one / at-least-once boundary: a SINGLE-element
//!      list and an EMPTY list are both no-ops (the C++ `while(liter2 != end)`
//!      never runs, with `liter2 = begin+1`).
//!   4. `distributeIntMultAdd` with BOTH add inputs constant: the all-constant
//!      fold path (no new INT_MULT ops created) — `(2 + 7) * 3 => #6 + #21`.
//!   5. `opSetInput` re-duplication uses `wrapping`-safe `new_constant` and the
//!      duplicate is independently re-shareable (a constant re-shared a THIRD
//!      time dups again, each reader getting its own single-descendant copy).

use std::rc::Rc;

use kuna_base::address::{calc_int_min, calc_uint_max, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4, uintm};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};

// ---------------------------------------------------------------------------
// harness (mirrors verify_w3_ir_funcdata_op.rs)
// ---------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode, flags: uint4) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
    op
}

/// A free, non-constant register varnode promoted to a function input (so
/// `is_free()` is false and `distributeIntMultAdd` does not bail on it).
fn mk_input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(4, Address::new(ram, off), dt(4));
    fd.vbank_mut()
        .set_input(vn, &mut |_: &mut kuna_decomp::varnode::VarnodeBank, _, _| Ok(()))
        .unwrap()
}

fn give_out(fd: &mut Funcdata, op: OpId, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.new_varnode(4, &Address::new(ram, off), Some(dt(4)));
    fd.op_set_output(op, vn).unwrap();
    fd.obank().get(op).unwrap().get_out().unwrap()
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

// ---------------------------------------------------------------------------
// 1. replaceLessequal — signed overflow guard, `c <= x` (diff = -1).
// ---------------------------------------------------------------------------
//
// C++ funcdata_op.cc:1049-1053:
//   if (op->code() == CPUI_INT_SLESSEQUAL) {
//     if ((diff==-1) && (val==calc_int_min(vn->getSize()))) return false;
//     ...
// `INT_SMIN <= x` cannot be rewritten to `(SMIN-1) < x` without signed underflow,
// so the helper must DECLINE (return false) and leave the op untouched.
#[test]
fn w5x_replace_lessequal_signed_min_in_slot0_declines() {
    let mut fd = build_fd();
    // op = (c <= x): const in SLOT 0 -> diff = -1.
    let op = mk_op(
        &mut fd,
        2,
        0x100,
        OpCode::CPUI_INT_SLESSEQUAL,
        pcodeop_flags::binary | pcodeop_flags::booloutput,
    );
    let smin = calc_int_min(4); // 0x80000000 for size 4
    let c = fd.new_constant(4, smin);
    let x = mk_input(&mut fd, 0x10);
    fd.op_set_input(op, c, 0).unwrap();
    fd.op_set_input(op, x, 1).unwrap();
    let _o = give_out(&mut fd, op, 0x40);

    assert!(!fd.replace_lessequal(op).unwrap(), "SMIN <= x cannot underflow-rewrite");
    // Op untouched: still SLESSEQUAL, slot 0 still the SMIN constant.
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SLESSEQUAL);
    let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    assert_eq!(fd.vbank().get(in0).unwrap().get_offset(), smin);
    assert!(!fd.obank().get(op).unwrap().is_canonical_lessequal(), "no provenance on decline");
}

// Companion: a NON-boundary `c <= x` signed case rewrites to `(c-1) < x`.
#[test]
fn w5x_replace_lessequal_signed_const_slot0_rewrites_diff_minus1() {
    let mut fd = build_fd();
    let op = mk_op(
        &mut fd,
        2,
        0x100,
        OpCode::CPUI_INT_SLESSEQUAL,
        pcodeop_flags::binary | pcodeop_flags::booloutput,
    );
    // c = 5 (well away from SMIN); `5 <= x`  =>  `4 < x`.
    let c = fd.new_constant(4, 5);
    let x = mk_input(&mut fd, 0x10);
    fd.op_set_input(op, c, 0).unwrap();
    fd.op_set_input(op, x, 1).unwrap();
    let _o = give_out(&mut fd, op, 0x40);

    assert!(fd.replace_lessequal(op).unwrap());
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SLESS);
    // The rewritten constant landed back in SLOT 0 (i == 0) with value c-1 = 4.
    let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    assert!(fd.vbank().get(in0).unwrap().is_constant());
    assert_eq!(fd.vbank().get(in0).unwrap().get_offset(), 4, "c-1");
    // x stayed in slot 1.
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(x));
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal());
}

// ---------------------------------------------------------------------------
// 2. replaceLessequal — unsigned overflow guards (both boundaries).
// ---------------------------------------------------------------------------
//
// C++ funcdata_op.cc:1055-1058:
//   else { // unsigned
//     if ((diff==-1) && (val==0)) return false;
//     if ((diff==1) && (val==calc_uint_max(vn->getSize()))) return false;
//     ...
#[test]
fn w5x_replace_lessequal_unsigned_zero_and_uintmax_decline() {
    // (a) `0 <= x` (const slot 0, diff=-1, val=0) -> decline.
    {
        let mut fd = build_fd();
        let op = mk_op(
            &mut fd,
            2,
            0x100,
            OpCode::CPUI_INT_LESSEQUAL,
            pcodeop_flags::binary | pcodeop_flags::booloutput,
        );
        let zero = fd.new_constant(4, 0);
        let x = mk_input(&mut fd, 0x10);
        fd.op_set_input(op, zero, 0).unwrap();
        fd.op_set_input(op, x, 1).unwrap();
        let _o = give_out(&mut fd, op, 0x40);
        assert!(!fd.replace_lessequal(op).unwrap(), "0 <= x cannot underflow-rewrite");
        assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_LESSEQUAL);
    }
    // (b) `x <= UINT_MAX` (const slot 1, diff=+1, val=calc_uint_max) -> decline.
    {
        let mut fd = build_fd();
        let op = mk_op(
            &mut fd,
            2,
            0x100,
            OpCode::CPUI_INT_LESSEQUAL,
            pcodeop_flags::binary | pcodeop_flags::booloutput,
        );
        let x = mk_input(&mut fd, 0x10);
        let umax = fd.new_constant(4, calc_uint_max(4)); // 0xffffffff for size 4
        fd.op_set_input(op, x, 0).unwrap();
        fd.op_set_input(op, umax, 1).unwrap();
        let _o = give_out(&mut fd, op, 0x40);
        assert!(!fd.replace_lessequal(op).unwrap(), "x <= UINT_MAX cannot overflow-rewrite");
        assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_LESSEQUAL);
    }
}

// ---------------------------------------------------------------------------
// 3. cseEliminateList — off-by-one / at-least-once boundary (empty + single).
// ---------------------------------------------------------------------------
//
// C++ funcdata_op.cc:1465-1470:  if (list.empty()) return; ...; liter2=begin+1;
//   while(liter2 != end) {...}
// An empty list returns immediately; a single-element list never enters the loop
// (liter2 == end at once).  Both must leave `outlist` untouched and the op alive.
#[test]
fn w5x_cse_eliminate_list_empty_and_single_are_noops() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);

    // Empty list: outlist stays empty.
    let mut empty: Vec<(uintm, OpId)> = Vec::new();
    let mut outlist: Vec<VarnodeId> = Vec::new();
    fd.cse_eliminate_list(&mut empty, &mut outlist, |_, _| true).unwrap();
    assert!(outlist.is_empty(), "empty list -> no output");

    // Single element: build one real op, list it once.
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, pcodeop_flags::binary);
    let c1 = fd.new_constant(4, 1);
    fd.op_set_input(op, x, 0).unwrap();
    fd.op_set_input(op, c1, 1).unwrap();
    let _o = give_out(&mut fd, op, 0x40);
    fd.op_insert(op, bl, None);
    let h = kuna_decomp::op::get_cse_hash(fd.obank().get(op).unwrap(), fd.vbank());
    let mut single = vec![(h, op)];
    fd.cse_eliminate_list(&mut single, &mut outlist, |_, _| true).unwrap();
    assert!(outlist.is_empty(), "single element -> loop body never runs");
    assert!(!fd.obank().get(op).unwrap().is_dead(), "single op survives untouched");
}

// ---------------------------------------------------------------------------
// 4. distributeIntMultAdd — both add inputs constant (all-constant fold).
// ---------------------------------------------------------------------------
//
// C++ funcdata_op.cc:1091-1119: when BOTH vn0 and vn1 are constant, each branch
// takes the `newConstant(sz, coeff*off)` arm — NO new INT_MULT op is built, and
// `op` becomes `INT_ADD(#coeff*v0, #coeff*v1)`.  `(2 + 7) * 3 => #6 + #21`.
#[test]
fn w5x_distribute_int_mult_add_both_const_folds_no_new_mult() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // addop = INT_ADD(#2, #7).
    let addop = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, pcodeop_flags::binary);
    let c2 = fd.new_constant(4, 2);
    let c7 = fd.new_constant(4, 7);
    fd.op_set_input(addop, c2, 0).unwrap();
    fd.op_set_input(addop, c7, 1).unwrap();
    let addout = give_out(&mut fd, addop, 0x40);
    fd.op_insert(addop, bl, None);
    // op = INT_MULT(addout, #3).
    let op = mk_op(&mut fd, 2, 0x104, OpCode::CPUI_INT_MULT, pcodeop_flags::binary);
    let c3 = fd.new_constant(4, 3);
    fd.op_set_input(op, addout, 0).unwrap();
    fd.op_set_input(op, c3, 1).unwrap();
    let _out = give_out(&mut fd, op, 0x44);
    fd.op_insert(op, bl, None);

    let ops_before = fd.obank().num_alive();
    assert!(fd.distribute_int_mult_add(op).unwrap(), "distributes");
    // No new INT_MULT op was created (both branches folded to constants).
    assert_eq!(fd.obank().num_alive(), ops_before, "all-constant fold creates no new op");
    // op is now INT_ADD(#6, #21).
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_ADD);
    let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    let in1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(in0).unwrap().is_constant() && fd.vbank().get(in1).unwrap().is_constant());
    assert_eq!(fd.vbank().get(in0).unwrap().get_offset(), 6, "3*2");
    assert_eq!(fd.vbank().get(in1).unwrap().get_offset(), 21, "3*7");
}

// ---------------------------------------------------------------------------
// 5. opSetInput re-duplication — a constant re-shared a THIRD time dups again.
// ---------------------------------------------------------------------------
//
// Each reader of a constant must end up with its own single-descendant copy.
// This stresses the guard's `!hasNoDescend()` re-evaluation: after the first
// dedup the ORIGINAL still has exactly one descendant, so a third link of the
// SAME original constant dups a SECOND time (not aliasing the first duplicate).
#[test]
fn w5x_op_set_input_third_reshare_dups_independently() {
    let mut fd = build_fd();
    let a = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY, 0);
    let b = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_COPY, 0);
    let c = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_COPY, 0);
    let k = fd.new_constant(4, 0x2a);

    // First link: k has no descendant yet -> linked directly (no dup).
    fd.op_set_input(a, k, 0).unwrap();
    let ka = fd.obank().get(a).unwrap().get_in(0).unwrap();
    assert_eq!(ka, k, "first reader links the original directly");

    // Second link: k now has 1 descendant -> dup.
    fd.op_set_input(b, k, 0).unwrap();
    let kb = fd.obank().get(b).unwrap().get_in(0).unwrap();
    assert_ne!(kb, k, "second reader gets a fresh dup");

    // Third link of the ORIGINAL k again: k still has exactly 1 descendant -> dup
    // AGAIN, distinct from both k and kb.
    fd.op_set_input(c, k, 0).unwrap();
    let kc = fd.obank().get(c).unwrap().get_in(0).unwrap();
    assert_ne!(kc, k, "third reader does not alias the original");
    assert_ne!(kc, kb, "third reader does not alias the second duplicate");

    // Every distinct constant carries the same value and exactly one reader.
    for v in [k, kb, kc] {
        assert_eq!(fd.vbank().get(v).unwrap().get_offset(), 0x2a, "value preserved");
        assert_eq!(fd.vbank().get(v).unwrap().num_descend(), 1, "single descendant each");
    }
}
