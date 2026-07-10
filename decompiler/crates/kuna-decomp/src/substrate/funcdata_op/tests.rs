// Tests for the `funcdata_op` op-manipulation primitives (included into the
// `mod tests` in funcdata_op.rs, hence plain `//` comments here).
//
// Covered:
//   - op creation / SeqNum (uniq) allocation order via `new_op`/`new_op_seq`.
//   - def-use link integrity after each input mutation primitive.
//   - dead-list discipline (`op_insert`/`op_uninsert`/`op_unlink`/`op_destroy`).
//   - block-relative insertion ordering invariants (MULTIEQUAL-first /
//     INDIRECT-immediately-before / branch-last).
//   - boolean-flip list tracing (`op_flip_in_place_test`).
//   - scans (`get_first_return_op`, `find_primary_branch`).
//   - control-flow walks (`op_next_op`/`op_previous_op`/`op_target`).

use super::*; // Funcdata, OpCode, pcodeop_flags, OpId/VarnodeId/TypeOp from funcdata_op

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, BlockId};

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

/// Create an op with `inputs` slots and a given opcode at `off`, returning its id.
fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// The minimal flag bits the ordering-invariant tests depend on (branch ops carry
/// `branch`, BRANCHIND included; everything else 0).
fn opcode_flags(opc: OpCode) -> kuna_base::types::uint4 {
    match opc {
        OpCode::CPUI_BRANCH
        | OpCode::CPUI_CBRANCH
        | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        _ => 0,
    }
}

/// A fresh basic block under the bblocks root.
fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_vn(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(4, Address::new(ram, off), dt(4))
}

// --- op creation / SeqNum allocation ------------------------------------------

#[test]
fn new_op_allocates_uniq_monotonically() {
    let mut fd = build_fd();
    let a = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_COPY);
    let b = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let c = mk_op(&mut fd, 0, 0x200, OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(a).unwrap().get_time(), 0);
    assert_eq!(fd.obank().get(b).unwrap().get_time(), 1);
    assert_eq!(fd.obank().get(c).unwrap().get_time(), 2);
    assert_eq!(fd.obank().get_uniq_id(), 3);
    // all start on the dead list in creation order
    assert_eq!(fd.obank().iter_dead().collect::<Vec<_>>(), vec![a, b, c]);
}

#[test]
fn new_op_seq_bumps_uniqid() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let sq = SeqNum::new(Address::new(ram, 0x10), 41);
    let _id = fd.new_op_seq(1, sq);
    assert_eq!(fd.obank().get_uniq_id(), 42);
    let id2 = mk_op(&mut fd, 0, 0x20, OpCode::CPUI_COPY);
    assert_eq!(fd.obank().get(id2).unwrap().get_time(), 42);
}

// --- def-use link integrity ---------------------------------------------------

#[test]
fn op_set_input_links_descend() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    // op reads v0,v1; each varnode lists op as a descendant.
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v0));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v1));
    assert_eq!(fd.vbank().get(v0).unwrap().descend_iter().collect::<Vec<_>>(), vec![op]);
    assert_eq!(fd.vbank().get(v1).unwrap().descend_iter().collect::<Vec<_>>(), vec![op]);
}

#[test]
fn op_set_input_idempotent_when_same() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let v0 = mk_vn(&mut fd, 0x10);
    fd.op_set_input(op, v0, 0).unwrap();
    // setting the same vn again must NOT add a second descend entry.
    fd.op_set_input(op, v0, 0).unwrap();
    assert_eq!(fd.vbank().get(v0).unwrap().num_descend(), 1);
}

#[test]
fn op_set_input_replaces_old_and_fixes_descend() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, v0, 0).unwrap();
    // replace slot 0 with v1: v0 loses the descendant, v1 gains it.
    fd.op_set_input(op, v1, 0).unwrap();
    assert!(fd.vbank().get(v0).unwrap().has_no_descend());
    assert_eq!(fd.vbank().get(v1).unwrap().descend_iter().collect::<Vec<_>>(), vec![op]);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v1));
}

#[test]
fn op_unset_input_severs_descend() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let v0 = mk_vn(&mut fd, 0x10);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_unset_input(op, 0);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), None);
    assert!(fd.vbank().get(v0).unwrap().has_no_descend());
}

#[test]
fn op_swap_input_does_not_touch_descend() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_SUB);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    fd.op_swap_input(op, 0, 1);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v1));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v0));
    // descend lists unchanged (each varnode still has exactly one descendant).
    assert_eq!(fd.vbank().get(v0).unwrap().num_descend(), 1);
    assert_eq!(fd.vbank().get(v1).unwrap().num_descend(), 1);
}

#[test]
fn op_remove_input_renumbers_and_unlinks() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 3, 0x100, OpCode::CPUI_INT_ADD);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    let v2 = mk_vn(&mut fd, 0x30);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    fd.op_set_input(op, v2, 2).unwrap();
    // remove slot 0: v0 unlinked, v1->0, v2->1.
    fd.op_remove_input(op, 0);
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v1));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v2));
    assert!(fd.vbank().get(v0).unwrap().has_no_descend());
}

#[test]
fn op_insert_input_shifts_and_links() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let v0 = mk_vn(&mut fd, 0x10);
    let v_new = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, v0, 0).unwrap();
    // insert v_new at slot 0: v0 shifts to slot 1.
    fd.op_insert_input(op, v_new, 0).unwrap();
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 2);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v_new));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v0));
    assert_eq!(fd.vbank().get(v_new).unwrap().descend_iter().collect::<Vec<_>>(), vec![op]);
}

#[test]
fn op_set_all_input_resets_and_relinks() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    let v2 = mk_vn(&mut fd, 0x30);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    // replace all inputs with [v2, v1, v0] (resize 2->3).  Each free varnode may
    // have at most one descendant, exactly as the C++ `addDescend` requires; the
    // old links are unset first, so reusing v0/v1 in different slots is valid.
    fd.op_set_all_input(op, &[v2, v1, v0]).unwrap();
    assert_eq!(fd.obank().get(op).unwrap().num_input(), 3);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(v2));
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(v1));
    assert_eq!(fd.obank().get(op).unwrap().get_in(2), Some(v0));
    // each varnode read exactly once now.
    assert_eq!(fd.vbank().get(v0).unwrap().num_descend(), 1);
    assert_eq!(fd.vbank().get(v1).unwrap().num_descend(), 1);
    assert_eq!(fd.vbank().get(v2).unwrap().num_descend(), 1);
}

// --- dead-list discipline -----------------------------------------------------

#[test]
fn op_insert_marks_alive_and_uninsert_restores() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    assert!(fd.obank().get(op).unwrap().is_dead());
    fd.op_insert(op, bl, None);
    assert!(!fd.obank().get(op).unwrap().is_dead());
    assert_eq!(fd.bb_op_head(bl), Some(op));
    assert_eq!(fd.obank().get(op).unwrap().get_parent(), Some(bl));
    // uninsert: back to dead, removed from block.
    fd.op_uninsert(op);
    assert!(fd.obank().get(op).unwrap().is_dead());
    assert_eq!(fd.bb_op_head(bl), None);
    assert_eq!(fd.obank().get(op).unwrap().get_parent(), None);
}

#[test]
fn op_unlink_severs_io_and_removes_from_block() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let v0 = mk_vn(&mut fd, 0x10);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_insert(op, bl, None);
    fd.op_unlink(op);
    // input severed, op back on dead list, no parent.
    assert!(fd.vbank().get(v0).unwrap().has_no_descend());
    assert!(fd.obank().get(op).unwrap().is_dead());
    assert_eq!(fd.obank().get(op).unwrap().get_parent(), None);
    assert_eq!(fd.bb_op_head(bl), None);
}

#[test]
fn op_destroy_unsets_inputs_and_marks_dead() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD);
    let v0 = mk_vn(&mut fd, 0x10);
    let v1 = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, v0, 0).unwrap();
    fd.op_set_input(op, v1, 1).unwrap();
    fd.op_insert(op, bl, None);
    fd.op_destroy(op);
    assert!(fd.vbank().get(v0).unwrap().has_no_descend());
    assert!(fd.vbank().get(v1).unwrap().has_no_descend());
    assert!(fd.obank().get(op).unwrap().is_dead());
    assert_eq!(fd.bb_op_head(bl), None);
}

// --- block-relative insertion ordering invariants -----------------------------

#[test]
fn op_insert_before_skips_preceding_indirects() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // block: [copy0, indirect1, copy2]
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let ind1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_INDIRECT);
    let copy2 = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_COPY);
    fd.op_insert(copy0, bl, None);
    fd.op_insert(ind1, bl, None);
    fd.op_insert(copy2, bl, None);
    // insert a non-INDIRECT before copy2: it must skip back over ind1 and land
    // before ind1 (INDIRECTs stay immediately before their op).
    let newop = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_INT_ADD);
    fd.op_insert_before(newop, copy2);
    assert_eq!(fd.bb_ops(bl), vec![copy0, newop, ind1, copy2]);
}

#[test]
fn op_insert_before_indirect_stays_put() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let ind1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_INDIRECT);
    let copy2 = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_COPY);
    fd.op_insert(copy0, bl, None);
    fd.op_insert(ind1, bl, None);
    fd.op_insert(copy2, bl, None);
    // inserting an INDIRECT before copy2 does NOT skip back; lands right before it.
    let newind = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_INDIRECT);
    fd.op_insert_before(newind, copy2);
    assert_eq!(fd.bb_ops(bl), vec![copy0, ind1, newind, copy2]);
}

#[test]
fn op_insert_after_skips_following_multiequals() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // block: [copy0, multi1, copy2]
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let multi1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_MULTIEQUAL);
    let copy2 = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_COPY);
    fd.op_insert(copy0, bl, None);
    fd.op_insert(multi1, bl, None);
    fd.op_insert(copy2, bl, None);
    // insert a non-MULTIEQUAL after copy0: skip forward over multi1, land before copy2.
    let newop = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_INT_ADD);
    fd.op_insert_after(newop, copy0);
    assert_eq!(fd.bb_ops(bl), vec![copy0, multi1, newop, copy2]);
}

#[test]
fn op_insert_begin_respects_multiequal_first() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let multi0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_MULTIEQUAL);
    let copy1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_COPY);
    fd.op_insert(multi0, bl, None);
    fd.op_insert(copy1, bl, None);
    // a plain op inserted at begin lands after the leading MULTIEQUAL.
    let newop = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_INT_ADD);
    fd.op_insert_begin(newop, bl);
    assert_eq!(fd.bb_ops(bl), vec![multi0, newop, copy1]);
    // a MULTIEQUAL inserted at begin lands first.
    let newmulti = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_MULTIEQUAL);
    fd.op_insert_begin(newmulti, bl);
    assert_eq!(fd.bb_ops(bl), vec![newmulti, multi0, newop, copy1]);
}

#[test]
fn op_insert_end_keeps_branch_last() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let br = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_BRANCH);
    fd.op_insert(copy0, bl, None);
    fd.op_insert(br, bl, None);
    // inserting at end lands BEFORE the trailing branch (flow-break stays last).
    let newop = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_INT_ADD);
    fd.op_insert_end(newop, bl);
    assert_eq!(fd.bb_ops(bl), vec![copy0, newop, br]);
}

#[test]
fn op_insert_end_no_branch_appends() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    fd.op_insert(copy0, bl, None);
    let newop = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_INT_ADD);
    fd.op_insert_end(newop, bl);
    assert_eq!(fd.bb_ops(bl), vec![copy0, newop]);
    // on an empty block, insert at end == only op.
    let bl2 = mk_block(&mut fd);
    let solo = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_COPY);
    fd.op_insert_end(solo, bl2);
    assert_eq!(fd.bb_ops(bl2), vec![solo]);
}

// --- boolean-flip list tracing ------------------------------------------------

#[test]
fn op_flip_in_place_test_int_equal_normalizes_to_one() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_EQUAL);
    let mut fliplist = Vec::new();
    // INT_EQUAL -> push op, return 1 (denormalizing/ambivalent).
    assert_eq!(fd.op_flip_in_place_test(op, &mut fliplist, false), 1);
    assert_eq!(fliplist, vec![op]);
}

#[test]
fn op_flip_in_place_test_notequal_returns_zero() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_NOTEQUAL);
    let mut fliplist = Vec::new();
    assert_eq!(fd.op_flip_in_place_test(op, &mut fliplist, false), 0);
    assert_eq!(fliplist, vec![op]);
}

#[test]
fn op_flip_in_place_test_bool_negate_needs_removal_flag() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_BOOL_NEGATE);
    let mut fl = Vec::new();
    // not allowed to remove -> 2 (impossible), nothing pushed.
    assert_eq!(fd.op_flip_in_place_test(op, &mut fl, false), 2);
    assert!(fl.is_empty());
    // allowed -> push op, return 0.
    assert_eq!(fd.op_flip_in_place_test(op, &mut fl, true), 0);
    assert_eq!(fl, vec![op]);
}

#[test]
fn op_flip_in_place_test_less_const_first_normalizes() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESS);
    // in0 is a constant -> return 0; otherwise 1.
    let cspace = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
    let cvn = fd.vbank_mut().create(4, Address::new(cspace, 5), dt(4));
    fd.op_set_input(op, cvn, 0).unwrap();
    let mut fl = Vec::new();
    assert_eq!(fd.op_flip_in_place_test(op, &mut fl, false), 0);
    assert_eq!(fl, vec![op]);
}

// --- scans --------------------------------------------------------------------

#[test]
fn get_first_return_op_skips_dead_and_halt() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // dead RETURN (never inserted) is skipped.
    let dead_ret = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_RETURN);
    // halt RETURN is skipped.
    let halt_ret = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_RETURN);
    fd.op_insert(halt_ret, bl, None);
    fd.op_mark_halt(halt_ret, pcodeop_flags::halt).unwrap();
    // a live, non-halt RETURN is returned.
    let good_ret = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_RETURN);
    fd.op_insert(good_ret, bl, None);
    let _ = dead_ret;
    assert_eq!(fd.get_first_return_op(), Some(good_ret));
}

#[test]
fn find_primary_branch_picks_expected() {
    let mut fd = build_fd();
    let copy = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let call = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_CALL);
    let ret = mk_op(&mut fd, 1, 0x120, OpCode::CPUI_RETURN);
    let ops = vec![copy, call, ret];
    // findcall -> the CALL.
    assert_eq!(fd.find_primary_branch(&ops, false, true, false), Some(call));
    // findreturn -> the RETURN.
    assert_eq!(fd.find_primary_branch(&ops, false, false, true), Some(ret));
    // none requested -> None.
    assert_eq!(fd.find_primary_branch(&ops, false, false, false), None);
}

// --- control-flow walks -------------------------------------------------------

#[test]
fn op_previous_and_next_within_block() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let a = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let b = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_COPY);
    fd.op_insert(a, bl, None);
    fd.op_insert(b, bl, None);
    assert_eq!(fd.op_previous_op(b), Some(a));
    assert_eq!(fd.op_previous_op(a), None);
    // single-out flow is needed for next across blocks; within the block, nextOp
    // of `a` is `b`.
    assert_eq!(fd.op_next_op(a), Some(b));
    // `b` is last in the only block (no out edges) -> next is None.
    assert_eq!(fd.op_next_op(b), None);
}

#[test]
fn op_target_finds_instruction_start_in_deadlist() {
    let mut fd = build_fd();
    // dead ops in creation order: start(startmark), mid, tail.  target() of tail
    // walks back to the startmark op.
    let start = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    fd.obank_mut().get_mut(start).unwrap().set_flag(pcodeop_flags::startmark);
    let mid = mk_op(&mut fd, 0, 0x104, OpCode::CPUI_COPY);
    let tail = mk_op(&mut fd, 0, 0x108, OpCode::CPUI_COPY);
    assert_eq!(fd.op_target(tail), start);
    assert_eq!(fd.op_target(mid), start);
    assert_eq!(fd.op_target(start), start);
}

// --- comparison canonicalization / multiply distribution / CSE (helpers) ------

/// Make an op with `inputs` slots, opcode, and an EXPLICIT flag word (the
/// `glb->inst[opc]` opflags the W6 table would supply — set here so eval-type
/// reading guards like `get_eval_type` see the right `binary`/`booloutput` bits).
fn mk_op_flags(
    fd: &mut Funcdata,
    inputs: int4,
    off: u64,
    opc: OpCode,
    flags: kuna_base::types::uint4,
) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
    op
}

/// Attach a fresh output Varnode (at `off`, size 4) to an op via op_set_output.
fn give_out(fd: &mut Funcdata, op: OpId, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.new_varnode(4, &Address::new(ram, off), Some(dt(4)));
    fd.op_set_output(op, vn).unwrap();
    op_out(fd, op)
}

fn op_out(fd: &Funcdata, op: OpId) -> VarnodeId {
    fd.obank().get(op).unwrap().get_out().unwrap()
}

/// Make a non-free (function-input) Varnode at (ram off, size 4) so it may carry
/// multiple descendants and is not `isFree()`.
fn mk_input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let vn = mk_vn(fd, off);
    fd.vbank_mut()
        .set_input(vn, &mut |_: &mut crate::varnode::VarnodeBank, _, _| Ok(()))
        .unwrap()
}

#[test]
fn replace_lessequal_rewrites_x_le_c() {
    // `x <= 5` (unsigned, const in slot 1) => `x < 6`.
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let x = mk_vn(&mut fd, 0x10);
    let c = fd.new_constant(4, 5);
    fd.op_set_input(op, x, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    let _o = give_out(&mut fd, op, 0x40);
    assert!(fd.replace_lessequal(op).unwrap(), "rewrites");
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_LESS);
    let newc = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(newc).unwrap().get_offset(), 6, "c+1");
    assert!(fd.obank().get(op).unwrap().is_canonical_lessequal(), "provenance recorded");
}

#[test]
fn replace_lessequal_declines_non_constant() {
    // Neither operand constant => no replacement.
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_LESSEQUAL);
    let a = mk_vn(&mut fd, 0x10);
    let b = mk_vn(&mut fd, 0x20);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    assert!(!fd.replace_lessequal(op).unwrap());
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_LESSEQUAL);
}

#[test]
fn distribute_int_mult_add_distributes_coeff() {
    // op = (x + 7) * 3  with addop = INT_ADD(x, #7).  After distribution:
    //   op becomes INT_ADD; in0 = (x*3) [new INT_MULT], in1 = #21 (=3*7).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    // addop = INT_ADD(x, #7) with a written output.
    let addop = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, pcodeop_flags::binary);
    let c7 = fd.new_constant(4, 7);
    fd.op_set_input(addop, x, 0).unwrap();
    fd.op_set_input(addop, c7, 1).unwrap();
    let addout = give_out(&mut fd, addop, 0x40);
    fd.op_insert(addop, bl, None);
    // op = INT_MULT(addout, #3).
    let op = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_MULT, pcodeop_flags::binary);
    let c3 = fd.new_constant(4, 3);
    fd.op_set_input(op, addout, 0).unwrap();
    fd.op_set_input(op, c3, 1).unwrap();
    let _out = give_out(&mut fd, op, 0x44);
    fd.op_insert(op, bl, None);

    assert!(fd.distribute_int_mult_add(op).unwrap(), "distributes");
    // op is now INT_ADD.
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_ADD);
    // in1 became constant 3*7 = 21.
    let in1 = fd.obank().get(op).unwrap().get_in(1).unwrap();
    assert!(fd.vbank().get(in1).unwrap().is_constant());
    assert_eq!(fd.vbank().get(in1).unwrap().get_offset(), 21);
    // in0 is the output of a new INT_MULT(x, #3).
    let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
    let multop = fd.vbank().get(in0).unwrap().get_def().unwrap();
    assert_eq!(fd.obank().get(multop).unwrap().code(), OpCode::CPUI_INT_MULT);
    assert_eq!(fd.obank().get(multop).unwrap().get_in(0), Some(x));
    let mc = fd.obank().get(multop).unwrap().get_in(1).unwrap();
    assert_eq!(fd.vbank().get(mc).unwrap().get_offset(), 3, "coefficient");
}

#[test]
fn cse_eliminate_list_collapses_same_block_match() {
    // Two identical INT_ADD(x, #1) ops in the same block; cseEliminateList keeps
    // the earlier one (lower seq order) and destroys the later, recording the
    // survivor's output.  The heritaged predicate returns true for both outputs.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    let bin = pcodeop_flags::binary;
    // op1 = INT_ADD(x, #1).
    let op1 = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let c1a = fd.new_constant(4, 1);
    fd.op_set_input(op1, x, 0).unwrap();
    fd.op_set_input(op1, c1a, 1).unwrap();
    let out1 = give_out(&mut fd, op1, 0x40);
    fd.op_insert(op1, bl, None);
    // op2 = INT_ADD(x, #1) (a separate but functionally identical op).
    let op2 = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, bin);
    let c1b = fd.new_constant(4, 1);
    fd.op_set_input(op2, x, 0).unwrap();
    fd.op_set_input(op2, c1b, 1).unwrap();
    let out2 = give_out(&mut fd, op2, 0x44);
    fd.op_insert(op2, bl, None);

    // Build the (hash, op) list the rule would (both hashes equal -> CSE match).
    let h1 = crate::op::get_cse_hash(fd.obank().get(op1).unwrap(), fd.vbank());
    let h2 = crate::op::get_cse_hash(fd.obank().get(op2).unwrap(), fd.vbank());
    assert_eq!(h1, h2, "identical ops hash equal");
    assert_ne!(h1, 0, "INT_ADD is cse-hashable");
    let mut list = vec![(h1, op1), (h2, op2)];
    let mut outlist: Vec<VarnodeId> = Vec::new();
    fd.cse_eliminate_list(&mut list, &mut outlist, |_, _| true).unwrap();

    // op1 (earlier seq) survives; op2 is destroyed; its readers (none) repointed.
    assert_eq!(outlist, vec![out1], "survivor output recorded");
    assert!(fd.obank().get(op2).unwrap().is_dead(), "duplicate op destroyed");
    assert!(!fd.obank().get(op1).unwrap().is_dead(), "dominator survives");
    // out2 was fully freed by op_destroy -> destroyVarnode (C++ opDestroy frees
    // the output Varnode, not merely unsets it), so it is gone from the bank.
    assert!(fd.vbank().get(out2).is_none(), "destroyed op output is freed");
}

#[test]
fn cse_eliminate_list_respects_heritaged_predicate() {
    // Same two matching ops, but the heritaged predicate is false -> no elimination.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    let bin = pcodeop_flags::binary;
    let op1 = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let c1a = fd.new_constant(4, 1);
    fd.op_set_input(op1, x, 0).unwrap();
    fd.op_set_input(op1, c1a, 1).unwrap();
    let _out1 = give_out(&mut fd, op1, 0x40);
    fd.op_insert(op1, bl, None);
    let op2 = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, bin);
    let c1b = fd.new_constant(4, 1);
    fd.op_set_input(op2, x, 0).unwrap();
    fd.op_set_input(op2, c1b, 1).unwrap();
    let _out2 = give_out(&mut fd, op2, 0x44);
    fd.op_insert(op2, bl, None);

    let h = crate::op::get_cse_hash(fd.obank().get(op1).unwrap(), fd.vbank());
    let mut list = vec![(h, op1), (h, op2)];
    let mut outlist: Vec<VarnodeId> = Vec::new();
    fd.cse_eliminate_list(&mut list, &mut outlist, |_, _| false).unwrap();
    assert!(outlist.is_empty(), "no elimination when outputs are not heritaged");
    assert!(!fd.obank().get(op2).unwrap().is_dead(), "duplicate survives");
}

// --- op_is_moveable (for-loop reroll support; PcodeOp::isMoveable, op.cc:201) ---

#[test]
fn op_is_moveable_identity_is_true() {
    // this == point: no movement necessary.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let bin = pcodeop_flags::binary;
    let op = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let x = mk_input(&mut fd, 0x10);
    let c = fd.new_constant(4, 1);
    fd.op_set_input(op, x, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    let _ = give_out(&mut fd, op, 0x40);
    fd.op_insert_end(op, bl);
    assert!(fd.op_is_moveable(op, op));
}

#[test]
fn op_is_moveable_special_store_is_not_moveable() {
    // A STORE is a special op (not LOAD) -> never moveable.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let special = pcodeop_flags::special;
    let store = mk_op_flags(&mut fd, 3, 0x100, OpCode::CPUI_STORE, special);
    let point = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, pcodeop_flags::binary);
    fd.op_insert_end(store, bl);
    fd.op_insert_end(point, bl);
    assert!(!fd.op_is_moveable(store, point), "special STORE is not moveable");
}

#[test]
fn op_is_moveable_cross_block_is_false() {
    // this and point in different blocks -> not moveable.
    let mut fd = build_fd();
    let bl1 = mk_block(&mut fd);
    let bl2 = mk_block(&mut fd);
    let bin = pcodeop_flags::binary;
    let a = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let b = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, bin);
    let _ = give_out(&mut fd, a, 0x40);
    let _ = give_out(&mut fd, b, 0x44);
    fd.op_insert_end(a, bl1);
    fd.op_insert_end(b, bl2);
    assert!(!fd.op_is_moveable(a, b), "ops in different blocks are not moveable");
}

#[test]
fn op_is_moveable_simple_forward_move_ok() {
    // Two independent normal ops in the same block: the first can move to after
    // the second (no reader of the first's output lies before the point).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let bin = pcodeop_flags::binary;
    let x = mk_input(&mut fd, 0x10);
    let op_a = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let ca = fd.new_constant(4, 1);
    fd.op_set_input(op_a, x, 0).unwrap();
    fd.op_set_input(op_a, ca, 1).unwrap();
    let _ = give_out(&mut fd, op_a, 0x40);
    fd.op_insert_end(op_a, bl);
    let op_b = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, bin);
    let cb = fd.new_constant(4, 2);
    fd.op_set_input(op_b, x, 0).unwrap();
    fd.op_set_input(op_b, cb, 1).unwrap();
    let _ = give_out(&mut fd, op_b, 0x44);
    fd.op_insert_end(op_b, bl);
    assert!(fd.op_is_moveable(op_a, op_b), "independent normal op moves past another");
}

#[test]
fn op_is_moveable_blocked_when_output_read_before_point() {
    // op_a's output is read by op_b (which sits before `point`): op_a cannot move
    // past its own reader.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let bin = pcodeop_flags::binary;
    let x = mk_input(&mut fd, 0x10);
    let op_a = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_INT_ADD, bin);
    let ca = fd.new_constant(4, 1);
    fd.op_set_input(op_a, x, 0).unwrap();
    fd.op_set_input(op_a, ca, 1).unwrap();
    let out_a = give_out(&mut fd, op_a, 0x40);
    fd.op_insert_end(op_a, bl);
    // op_b reads out_a.
    let op_b = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INT_ADD, bin);
    let cb = fd.new_constant(4, 2);
    fd.op_set_input(op_b, out_a, 0).unwrap();
    fd.op_set_input(op_b, cb, 1).unwrap();
    let _ = give_out(&mut fd, op_b, 0x44);
    fd.op_insert_end(op_b, bl);
    // point is op_b; op_a's output is read at op_b (order <= point) -> not moveable.
    assert!(!fd.op_is_moveable(op_a, op_b), "cannot move past a reader of own output");
}

// --- VERIFIER adversarial tests for op_is_moveable -----------------------------
// These exercise op_is_moveable branches the 5 above leave uncovered:
// the LOAD `movingLoad` special path (op.cc:206-211 + the STORE/LOAD restriction
// arms at 252-265), and the run-off-block-end bail the Rust port added for the
// C++ do-while that has no terminating guard (op.cc:247-292).

// Helper: a 2-input INT_ADD `x + c` with an output at `out_off`, fully wired.
fn mk_add(fd: &mut Funcdata, off: u64, x: VarnodeId, cval: u64, out_off: u64) -> OpId {
    let op = mk_op_flags(fd, 2, off, OpCode::CPUI_INT_ADD, pcodeop_flags::binary);
    let c = fd.new_constant(4, cval);
    fd.op_set_input(op, x, 0).unwrap();
    fd.op_set_input(op, c, 1).unwrap();
    let _ = give_out(fd, op, out_off);
    op
}

#[test]
fn vfy_op_is_moveable_load_is_special_but_allowed() {
    // C++ op.cc:206-211: a LOAD is special, but `movingLoad=true` lets it move
    // (unlike STORE which returns false). With a non-addr-tied output and a
    // benign normal op as the move point, the LOAD is moveable.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let special = pcodeop_flags::special;
    let ptr = mk_input(&mut fd, 0x10);
    let load = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_LOAD, special);
    let spc = fd.new_constant(4, 0); // space-id annotation slot 0
    fd.op_set_input(load, spc, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let _ = give_out(&mut fd, load, 0x40); // plain (not addr-tied) output
    fd.op_insert_end(load, bl);
    // A benign normal op as the move point (no STORE/CALL between).
    let x = mk_input(&mut fd, 0x14);
    let point = mk_add(&mut fd, 0x104, x, 2, 0x44);
    fd.op_insert_end(point, bl);
    assert!(
        fd.op_is_moveable(load, point),
        "a LOAD (movingLoad) with non-tied output is moveable past a benign op"
    );
}

#[test]
fn vfy_moving_load_blocked_by_intervening_store() {
    // C++ op.cc:257-259: when movingLoad is set and a STORE lies between `this`
    // and `point`, the move is rejected (a LOAD must not be reordered past a
    // STORE).  This tests STORE-as-obstacle for a moving LOAD.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let special = pcodeop_flags::special;
    let ptr = mk_input(&mut fd, 0x10);
    let load = mk_op_flags(&mut fd, 2, 0x100, OpCode::CPUI_LOAD, special);
    let spc = fd.new_constant(4, 0);
    fd.op_set_input(load, spc, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let _ = give_out(&mut fd, load, 0x40);
    fd.op_insert_end(load, bl);
    // Intervening STORE (special, 3 inputs, no output).
    let store = mk_op_flags(&mut fd, 3, 0x104, OpCode::CPUI_STORE, special);
    let spc2 = fd.new_constant(4, 0);
    let dval = mk_input(&mut fd, 0x18);
    fd.op_set_input(store, spc2, 0).unwrap();
    fd.op_set_input(store, ptr, 1).unwrap();
    fd.op_set_input(store, dval, 2).unwrap();
    fd.op_insert_end(store, bl);
    // Move point after the STORE.
    let x = mk_input(&mut fd, 0x14);
    let point = mk_add(&mut fd, 0x108, x, 2, 0x44);
    fd.op_insert_end(point, bl);
    assert!(!fd.op_is_moveable(load, point), "a moving LOAD cannot cross a STORE");
}

#[test]
fn vfy_op_is_moveable_point_must_follow_this() {
    // The Rust port replaced the C++ do-while (op.cc:247-292, which derefs past
    // end() if `point` precedes `this` — UB) with a `None => return false` bail
    // when the block-op walk runs off the end before reaching `point`. Pin that
    // bail: with `point` placed BEFORE `this` in the block, the forward walk
    // never reaches it, so the op is reported not-moveable (no panic, no UB).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    // point is inserted FIRST (earlier in the block) ...
    let point = mk_add(&mut fd, 0x100, x, 1, 0x40);
    fd.op_insert_end(point, bl);
    // ... `this` op is inserted AFTER point.
    let this_op = mk_add(&mut fd, 0x104, x, 2, 0x44);
    fd.op_insert_end(this_op, bl);
    assert!(
        !fd.op_is_moveable(this_op, point),
        "forward walk that never reaches an earlier point bails to false (no UB)"
    );
}

// --- VERIFIER (w10-forloop-reroll) adversarial tests -------------------------
// The 5 porter unit tests + 3 porter "vfy_*" tests above cover: identity,
// special-STORE reject, cross-block reject, simple forward move, reader-before-
// point reject, the movingLoad allow path, moving-LOAD-blocked-by-STORE, and the
// run-off-end bail.  The op.cc:251-278 special-op SWITCH arms reached while
// scanning forward from `this` to `point` are otherwise UNCOVERED.  These pin
// the three remaining arms of that switch on a *normal* op crossing an
// intervening special op (movingLoad=false, no tied list):
//   * CPUI_INDIRECT/SEGMENTOP/CPOOLREF -> let through (op.cc:267-270, `break`),
//   * CPUI_CALL/CALLIND/NEW            -> allowed iff crossCalls (op.cc:271-275),
//   * default special op (e.g. CALLOTHER) -> reject (op.cc:276-277).

#[test]
fn vfy_w10_forloop_normal_op_crosses_indirect_lets_through() {
    // op.cc:267-270: an INDIRECT (special) between `this` and `point` is in the
    // explicit let-through set.  A benign normal op (non-tied out, const inputs)
    // is moveable across it.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    let this_op = mk_add(&mut fd, 0x100, x, 1, 0x40);
    fd.op_insert_end(this_op, bl);
    // Intervening INDIRECT (special, 2 inputs, with an unrelated output).
    let indirect = mk_op_flags(&mut fd, 2, 0x104, OpCode::CPUI_INDIRECT, pcodeop_flags::special);
    let iv = mk_input(&mut fd, 0x18);
    let iarg = fd.new_constant(4, 0);
    fd.op_set_input(indirect, iv, 0).unwrap();
    fd.op_set_input(indirect, iarg, 1).unwrap();
    let _ = give_out(&mut fd, indirect, 0x50);
    fd.op_insert_end(indirect, bl);
    // Move point after the INDIRECT.
    let y = mk_input(&mut fd, 0x14);
    let point = mk_add(&mut fd, 0x108, y, 2, 0x44);
    fd.op_insert_end(point, bl);
    assert!(
        fd.op_is_moveable(this_op, point),
        "a normal op crosses an INDIRECT (the op.cc let-through set)"
    );
}

#[test]
fn vfy_w10_forloop_normal_op_crosses_call_when_crosscalls() {
    // op.cc:271-275: a CALL between `this` and `point` is allowed ONLY if
    // crossCalls is set.  For a normal op whose output and all inputs are
    // neither addr-tied nor persist (op.cc:227-238) crossCalls is TRUE, so the
    // move across the CALL is permitted.  This pins the permissive cross-call
    // arm (the porter's tests never cross a CALL).
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    let this_op = mk_add(&mut fd, 0x100, x, 1, 0x40); // non-tied out, const/input inputs
    fd.op_insert_end(this_op, bl);
    // Intervening CALL (special, 1 input target, no output).
    let call = mk_op_flags(&mut fd, 1, 0x104, OpCode::CPUI_CALL, pcodeop_flags::special);
    let target = fd.new_constant(8, 0x400500);
    fd.op_set_input(call, target, 0).unwrap();
    fd.op_insert_end(call, bl);
    let y = mk_input(&mut fd, 0x14);
    let point = mk_add(&mut fd, 0x108, y, 2, 0x44);
    fd.op_insert_end(point, bl);
    assert!(
        fd.op_is_moveable(this_op, point),
        "a fully-untied normal op (crossCalls=true) may cross a CALL"
    );
}

#[test]
fn vfy_w10_forloop_unknown_special_op_blocks_move() {
    // op.cc:276-277 (`default: return false`): a special op NOT in the
    // let-through / LOAD / STORE / CALL set (here CALLOTHER) between `this` and
    // `point` rejects the move outright.  Pins the catch-all reject arm.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let x = mk_input(&mut fd, 0x10);
    let this_op = mk_add(&mut fd, 0x100, x, 1, 0x40);
    fd.op_insert_end(this_op, bl);
    // Intervening CALLOTHER (special, not in any allowed arm -> default reject).
    let other = mk_op_flags(&mut fd, 1, 0x104, OpCode::CPUI_CALLOTHER, pcodeop_flags::special);
    let sel = fd.new_constant(4, 7);
    fd.op_set_input(other, sel, 0).unwrap();
    fd.op_insert_end(other, bl);
    let y = mk_input(&mut fd, 0x14);
    let point = mk_add(&mut fd, 0x108, y, 2, 0x44);
    fd.op_insert_end(point, bl);
    assert!(
        !fd.op_is_moveable(this_op, point),
        "an unhandled special op (CALLOTHER) in the path blocks the move (default reject)"
    );
}
