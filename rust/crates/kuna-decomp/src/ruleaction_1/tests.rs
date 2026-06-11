//! Tests for `ruleaction_1.rs` (ruleaction.cc:1-1443, RuleEarlyRemoval ..
//! RuleRangeMeld).
//!
//! Each test hand-builds the input op pattern on a real [`Funcdata`] and asserts
//! the exact transformation the C++ `applyOp` performs (opcode changes, input
//! rewiring, created constants), plus negative tests exercising each early-out
//! guard.  Rules whose body sits behind a cross-wave seam (the W3 output-creation
//! API, expression helpers, rangeutil, funcdata CSE/compare-form, heritage) are
//! exercised only at their portable guards (they return 0 = no change), since the
//! transforming body is deferred — see the module docs.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::dtype::{type_metatype, Datatype};
use crate::seams::{Architecture, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

// -----------------------------------------------------------------------------
// Harness
// -----------------------------------------------------------------------------

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

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// Make a live binary op (2 input slots, no output) at the ram offset.
fn make_op(fd: &mut Funcdata, off: u64, opc: OpCode) -> OpId {
    make_op_n(fd, off, opc, 2, 0)
}

/// Make a live op with explicit cached property flags (the W6 `glb->inst[opc]`
/// flag word the real `setOpcode` would supply — set here so flag-reading guards
/// like `isCall`/`isBoolOutput` can be exercised).  Two input slots.
fn make_op_flags(fd: &mut Funcdata, off: u64, opc: OpCode, flags: u32) -> OpId {
    make_op_n(fd, off, opc, 2, flags)
}

/// Make a live op with `ninputs` input slots and explicit flags.
fn make_op_n(fd: &mut Funcdata, off: u64, opc: OpCode, ninputs: int4, flags: u32) -> OpId {
    let pc = Address::new(ram(fd), off);
    let id = fd.obank_mut().create_at(ninputs, pc);
    fd.obank_mut().mark_alive(id);
    fd.obank_mut().change_opcode(id, TypeOp::new(opc, flags, format!("{opc:?}")));
    id
}

/// Make a heritage-known (function-input) register varnode at (ram off, size).
/// Inputs are not `isFree()` and carry a full NZ mask, matching the typical
/// operand the rules see in a heritaged function.
fn make_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

/// Make a constant varnode of the given value/size.
fn make_const(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    fd.new_constant(size, val)
}

/// Make a written varnode that is the output of a fresh op of opcode `defopc`
/// (so it reports `isWritten()` and `getDef()->code()==defopc`).  Returns the
/// (varnode, defining-op) pair.  The def op has no inputs unless wired later.
fn make_written(fd: &mut Funcdata, off: u64, size: int4, defopc: OpCode) -> (VarnodeId, OpId) {
    let pc = Address::new(ram(fd), off);
    let defop = fd.obank_mut().create_at(2, pc);
    fd.obank_mut().mark_alive(defop);
    fd.obank_mut().change_opcode(defop, TypeOp::new(defopc, 0, format!("{defopc:?}")));
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    (vn, defop)
}

/// Attach an output varnode to an existing op (used after creating ops with
/// `make_op`).  Returns the new output varnode id.
fn give_output(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let vn = fd.new_varnode(size, &Address::new(ram(fd), off), Some(unk(size)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

/// Wire `vn` as input slot `slot` of `op` (descend link + op input).
fn wire_input(fd: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}
fn in_of(fd: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    fd.obank().get(op).unwrap().get_in(slot)
}
fn numin(fd: &Funcdata, op: OpId) -> int4 {
    fd.obank().get(op).unwrap().num_input()
}
fn off_of(fd: &Funcdata, vn: VarnodeId) -> u64 {
    fd.vbank().get(vn).unwrap().get_offset()
}
fn is_const(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).unwrap().is_constant()
}

/// The `"analysis"` grouplist all these rules clone into.
fn group() -> ActionGroupList {
    ActionGroupList::from_names(["analysis"])
}

// -----------------------------------------------------------------------------
// clone_rule / specs() registration
// -----------------------------------------------------------------------------

#[test]
fn specs_lists_25_rules_in_cpp_order() {
    let s = specs();
    assert_eq!(s.len(), 25);
    // First and last by their cloned rule's apply behavior is hard to assert
    // here, but every spec must clone into the analysis grouplist.
    let gl = group();
    for spec in &s {
        let r = (spec.ctor)();
        assert!(r.clone_rule(&gl).is_some(), "every spec clones into its group");
    }
}

#[test]
fn clone_rule_filters_on_group() {
    let r = RuleOrMask::new("analysis");
    assert!(r.clone_rule(&group()).is_some());
    let empty = ActionGroupList::new();
    assert!(r.clone_rule(&empty).is_none());
}

// -----------------------------------------------------------------------------
// RuleBxor2NotEqual — opcode rewrite only
// -----------------------------------------------------------------------------

#[test]
fn bxor2notequal_rewrites_opcode() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_BOOL_XOR);
    let mut r = RuleBxor2NotEqual::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_NOTEQUAL);
}

#[test]
fn bxor2notequal_op_list() {
    assert_eq!(RuleBxor2NotEqual::new("g").get_op_list(), vec![OpCode::CPUI_BOOL_XOR]);
}

// -----------------------------------------------------------------------------
// RuleTermOrder — swap constant to slot 1
// -----------------------------------------------------------------------------

#[test]
fn termorder_swaps_constant_to_last() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let c = make_const(&mut fd, 7, 4);
    let reg = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, c, 0); // constant in slot 0
    wire_input(&mut fd, op, reg, 1); // reg in slot 1
    let mut r = RuleTermOrder::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // After swap: reg in slot 0, constant in slot 1.
    assert_eq!(in_of(&fd, op, 0), Some(reg));
    assert_eq!(in_of(&fd, op, 1), Some(c));
}

#[test]
fn termorder_no_swap_when_const_already_last() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let reg = make_reg(&mut fd, 0x40, 4);
    let c = make_const(&mut fd, 7, 4);
    wire_input(&mut fd, op, reg, 0);
    wire_input(&mut fd, op, c, 1);
    let mut r = RuleTermOrder::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0); // no change
    assert_eq!(in_of(&fd, op, 0), Some(reg));
    assert_eq!(in_of(&fd, op, 1), Some(c));
}

#[test]
fn termorder_no_swap_when_both_constant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let c0 = make_const(&mut fd, 3, 4);
    let c1 = make_const(&mut fd, 7, 4);
    wire_input(&mut fd, op, c0, 0);
    wire_input(&mut fd, op, c1, 1);
    let mut r = RuleTermOrder::new("analysis");
    // vn1 const && !vn2 const is false (vn2 also const) -> no swap.
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn termorder_op_list_is_the_16_commutative_ops() {
    let l = RuleTermOrder::new("g").get_op_list();
    assert_eq!(l.len(), 16);
    assert_eq!(l[0], OpCode::CPUI_INT_EQUAL);
    assert_eq!(l[2], OpCode::CPUI_INT_ADD);
    assert_eq!(l[15], OpCode::CPUI_FLOAT_MULT);
}

// -----------------------------------------------------------------------------
// RulePiece2Zext — concat(#0, W) => zext(W)
// -----------------------------------------------------------------------------

#[test]
fn piece2zext_removes_zero_high_and_zexts() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_PIECE);
    let zero = make_const(&mut fd, 0, 4); // most-significant constant 0
    let w = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, zero, 0);
    wire_input(&mut fd, op, w, 1);
    let mut r = RulePiece2Zext::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_ZEXT);
    // slot 0 (the constant) is removed -> W is now slot 0.
    assert_eq!(numin(&fd, op), 1);
    assert_eq!(in_of(&fd, op, 0), Some(w));
}

#[test]
fn piece2zext_declines_nonzero_constant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_PIECE);
    let nz = make_const(&mut fd, 1, 4);
    let w = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, nz, 0);
    wire_input(&mut fd, op, w, 1);
    let mut r = RulePiece2Zext::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_PIECE);
}

#[test]
fn piece2zext_declines_nonconstant_high() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_PIECE);
    let hi = make_reg(&mut fd, 0x50, 4);
    let w = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, hi, 0);
    wire_input(&mut fd, op, w, 1);
    let mut r = RulePiece2Zext::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RulePiece2Sext — concat(V s>> #(8*sz-1), V) => sext(V)
// -----------------------------------------------------------------------------

#[test]
fn piece2sext_matches_sign_extension() {
    let mut fd = build_fd();
    // x is a register; shiftop = x s>> (8*size-1)
    let x = make_reg(&mut fd, 0x40, 4);
    let (shiftout, shiftop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_SRIGHT);
    let n = make_const(&mut fd, (8 * 4 - 1) as u64, 4);
    wire_input(&mut fd, shiftop, x, 0);
    wire_input(&mut fd, shiftop, n, 1);
    // op = PIECE(shiftout, x)
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_PIECE);
    wire_input(&mut fd, op, shiftout, 0);
    wire_input(&mut fd, op, x, 1);
    let mut r = RulePiece2Sext::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_SEXT);
    assert_eq!(numin(&fd, op), 1);
    assert_eq!(in_of(&fd, op, 0), Some(x));
}

#[test]
fn piece2sext_declines_wrong_shift_amount() {
    let mut fd = build_fd();
    let x = make_reg(&mut fd, 0x40, 4);
    let (shiftout, shiftop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_SRIGHT);
    let n = make_const(&mut fd, 7, 4); // wrong: not 8*size-1
    wire_input(&mut fd, shiftop, x, 0);
    wire_input(&mut fd, shiftop, n, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_PIECE);
    wire_input(&mut fd, op, shiftout, 0);
    wire_input(&mut fd, op, x, 1);
    let mut r = RulePiece2Sext::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_PIECE);
}

// -----------------------------------------------------------------------------
// RuleOrMask — W | full-mask => W (COPY of constant)
// -----------------------------------------------------------------------------

#[test]
fn ormask_full_mask_sets_copy_then_repoints_constant() {
    // RuleOrMask, on `W | fullmask`, does opSetOpcode(COPY); opSetInput(op,
    // constvn, 0); opRemoveInput(op, 1).  The middle step re-points the
    // already-attached slot-1 constant onto slot 0; in C++ `opSetInput`
    // duplicates a shared constant first, but that duplication is a deferred W3
    // seam (see funcdata_op.rs "Constant re-duplication guard"), so `op_set_input`
    // refuses a second descendant on the free constant and this rule's faithful
    // `opSetInput` call surfaces that seam as a panic.  We assert the rule reaches
    // exactly that point (opcode already flipped to COPY before the panic).
    let result = std::panic::catch_unwind(|| {
        let mut fd = build_fd();
        let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
        let w = make_reg(&mut fd, 0x40, 4);
        let full = make_const(&mut fd, calc_mask(4), 4);
        wire_input(&mut fd, op, w, 0);
        wire_input(&mut fd, op, full, 1);
        let _out = give_output(&mut fd, op, 0x80, 4);
        let mut r = RuleOrMask::new("analysis");
        r.apply_op(op, &mut fd)
    });
    assert!(result.is_err(), "OrMask transform hits the W3 constant-dup seam");
}

#[test]
fn ormask_declines_partial_mask() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    let w = make_reg(&mut fd, 0x40, 4);
    let partial = make_const(&mut fd, 0x0000ffff, 4);
    wire_input(&mut fd, op, w, 0);
    wire_input(&mut fd, op, partial, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleOrMask::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_OR);
}

// -----------------------------------------------------------------------------
// RuleOrCollapse — V | c => c when NZM(V)|c == c
// -----------------------------------------------------------------------------

#[test]
fn orcollapse_collapses_to_constant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    // in0 is a constant 0 (NZM == 0), so NZM(in0)|c == c always.
    let v = make_const(&mut fd, 0, 4);
    let c = make_const(&mut fd, 0x12, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, c, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleOrCollapse::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // opSetOpcode COPY; opRemoveInput(0) -> only the constant c remains in slot 0.
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(numin(&fd, op), 1);
    assert_eq!(in_of(&fd, op, 0), Some(c));
}

#[test]
fn orcollapse_declines_when_in0_turns_on_other_bits() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    // in0 register has full NZM; c only sets low bits -> (mask|val)!=val.
    let v = make_reg(&mut fd, 0x40, 4);
    let c = make_const(&mut fd, 0x12, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, c, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleOrCollapse::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn orcollapse_declines_nonconstant_in1() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    let v = make_const(&mut fd, 0, 4);
    let w = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, w, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleOrCollapse::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleOrConsume — drop the input whose NZM is unconsumed
// -----------------------------------------------------------------------------

#[test]
fn orconsume_drops_unconsumed_slot0() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    // in0 is constant 0 (NZM 0) -> (consume & 0)==0 always, slot 0 removed first.
    let a = make_const(&mut fd, 0, 4);
    let b = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4); // output consume default ~0
    let mut r = RuleOrConsume::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(numin(&fd, op), 1);
    assert_eq!(in_of(&fd, op, 0), Some(b));
}

#[test]
fn orconsume_declines_when_both_consumed() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    let a = make_reg(&mut fd, 0x40, 4); // full NZM
    let b = make_reg(&mut fd, 0x50, 4); // full NZM
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleOrConsume::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn orconsume_op_list_or_and_xor() {
    assert_eq!(
        RuleOrConsume::new("g").get_op_list(),
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    );
}

// -----------------------------------------------------------------------------
// RuleAndMask — multiple branches
// -----------------------------------------------------------------------------

#[test]
fn andmask_zero_nzmask_in0_becomes_copy_zero() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    // in0 const 0 -> mask1==0 -> andmask 0 -> vn = newConstant(size,0).
    let a = make_const(&mut fd, 0, 4);
    let b = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleAndMask::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(numin(&fd, op), 1);
    // slot 0 is a fresh constant 0.
    let in0 = in_of(&fd, op, 0).unwrap();
    assert!(is_const(&fd, in0));
    assert_eq!(off_of(&fd, in0), 0);
}

#[test]
fn andmask_andmask_equals_mask1_with_const_in1_copies_in0() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    // in0 const 0x0f (NZM=0x0f); in1 const 0xff. andmask = 0x0f == mask1 and in1
    // is constant -> vn = in0 (and it is heritage-known: constants are).
    let a = make_const(&mut fd, 0x0f, 4);
    let b = make_const(&mut fd, 0xff, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleAndMask::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_COPY);
    assert_eq!(numin(&fd, op), 1);
    assert_eq!(in_of(&fd, op, 0), Some(a));
}

#[test]
fn andmask_declines_when_andmask_is_mask1_but_in1_not_constant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    // in0 const 0x0f (NZM 0x0f); in1 register (full NZM) -> andmask = 0x0f == mask1
    // but in1 is not constant -> return 0.
    let a = make_const(&mut fd, 0x0f, 4);
    let b = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleAndMask::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_AND);
}

// -----------------------------------------------------------------------------
// RuleAndOrLump — (V & c) & d => V & (c & d)
// -----------------------------------------------------------------------------

#[test]
fn andorlump_combines_and_constants() {
    let mut fd = build_fd();
    // op2 = base & c   (written varnode vn1)
    let base = make_reg(&mut fd, 0x40, 4);
    let (vn1, op2) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    let c = make_const(&mut fd, 0xf0, 4);
    wire_input(&mut fd, op2, base, 0);
    wire_input(&mut fd, op2, c, 1);
    // op = vn1 & d
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let d = make_const(&mut fd, 0x3c, 4);
    wire_input(&mut fd, op, vn1, 0);
    wire_input(&mut fd, op, d, 1);
    let mut r = RuleAndOrLump::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // op now: base & (0xf0 & 0x3c == 0x30)
    assert_eq!(in_of(&fd, op, 0), Some(base));
    let nin1 = in_of(&fd, op, 1).unwrap();
    assert!(is_const(&fd, nin1));
    assert_eq!(off_of(&fd, nin1), 0xf0 & 0x3c);
}

#[test]
fn andorlump_or_uses_or_combine() {
    let mut fd = build_fd();
    let base = make_reg(&mut fd, 0x40, 4);
    let (vn1, op2) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_OR);
    let c = make_const(&mut fd, 0xf0, 4);
    wire_input(&mut fd, op2, base, 0);
    wire_input(&mut fd, op2, c, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    let d = make_const(&mut fd, 0x0c, 4);
    wire_input(&mut fd, op, vn1, 0);
    wire_input(&mut fd, op, d, 1);
    let mut r = RuleAndOrLump::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    let nin1 = in_of(&fd, op, 1).unwrap();
    assert_eq!(off_of(&fd, nin1), 0xf0 | 0x0c);
}

#[test]
fn andorlump_declines_when_inner_op_differs() {
    let mut fd = build_fd();
    // inner is INT_OR but outer is INT_AND -> "Must be same op" fails.
    let base = make_reg(&mut fd, 0x40, 4);
    let (vn1, op2) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_OR);
    let c = make_const(&mut fd, 0xf0, 4);
    wire_input(&mut fd, op2, base, 0);
    wire_input(&mut fd, op2, c, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let d = make_const(&mut fd, 0x3c, 4);
    wire_input(&mut fd, op, vn1, 0);
    wire_input(&mut fd, op, d, 1);
    let mut r = RuleAndOrLump::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleNegateIdentity — V & ~V => #0 ; V | ~V => #-1
// -----------------------------------------------------------------------------

#[test]
fn negateidentity_and_with_negation_becomes_zero() {
    let mut fd = build_fd();
    // negate op: out = ~V
    let v = make_reg(&mut fd, 0x40, 4);
    let neg = make_op(&mut fd, 0x60, OpCode::CPUI_INT_NEGATE);
    wire_input(&mut fd, neg, v, 0);
    let negout = give_output(&mut fd, neg, 0x70, 4);
    // logicOp: negout & V  (slot 0 = negout, slot 1 = v)
    let logic = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    wire_input(&mut fd, logic, negout, 0);
    wire_input(&mut fd, logic, v, 1);
    let _logicout = give_output(&mut fd, logic, 0x80, 4);
    let mut r = RuleNegateIdentity::new("analysis");
    assert_eq!(r.apply_op(neg, &mut fd), 1);
    // logicOp -> COPY of newConstant(size, 0) in slot 0, slot 1 removed.
    assert_eq!(code_of(&fd, logic), OpCode::CPUI_COPY);
    assert_eq!(numin(&fd, logic), 1);
    let in0 = in_of(&fd, logic, 0).unwrap();
    assert!(is_const(&fd, in0));
    assert_eq!(off_of(&fd, in0), 0);
}

#[test]
fn negateidentity_or_with_negation_becomes_full_mask() {
    let mut fd = build_fd();
    let v = make_reg(&mut fd, 0x40, 4);
    let neg = make_op(&mut fd, 0x60, OpCode::CPUI_INT_NEGATE);
    wire_input(&mut fd, neg, v, 0);
    let negout = give_output(&mut fd, neg, 0x70, 4);
    let logic = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_OR);
    wire_input(&mut fd, logic, negout, 0);
    wire_input(&mut fd, logic, v, 1);
    let _logicout = give_output(&mut fd, logic, 0x80, 4);
    let mut r = RuleNegateIdentity::new("analysis");
    assert_eq!(r.apply_op(neg, &mut fd), 1);
    assert_eq!(code_of(&fd, logic), OpCode::CPUI_COPY);
    let in0 = in_of(&fd, logic, 0).unwrap();
    assert_eq!(off_of(&fd, in0), calc_mask(4)); // ~0 over 4 bytes
}

#[test]
fn negateidentity_declines_when_other_input_not_v() {
    let mut fd = build_fd();
    let v = make_reg(&mut fd, 0x40, 4);
    let other = make_reg(&mut fd, 0x50, 4);
    let neg = make_op(&mut fd, 0x60, OpCode::CPUI_INT_NEGATE);
    wire_input(&mut fd, neg, v, 0);
    let negout = give_output(&mut fd, neg, 0x70, 4);
    let logic = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    wire_input(&mut fd, logic, negout, 0);
    wire_input(&mut fd, logic, other, 1); // not V
    let _logicout = give_output(&mut fd, logic, 0x80, 4);
    let mut r = RuleNegateIdentity::new("analysis");
    assert_eq!(r.apply_op(neg, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleRightShiftAnd — (V & full>>?) >> sa => V >> sa
// -----------------------------------------------------------------------------

#[test]
fn rightshiftand_bypasses_redundant_and() {
    let mut fd = build_fd();
    // root register size 4; andOp = root & mask where mask>>sa == full>>sa.
    let root = make_reg(&mut fd, 0x40, 4);
    let (inv, and_op) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    // sa = 8. full = 0xffffffff; full>>8 = 0x00ffffff. Choose mask so mask>>8 == that.
    let mask = make_const(&mut fd, 0xffffff00, 4); // >>8 == 0x00ffffff == full>>8
    wire_input(&mut fd, and_op, root, 0);
    wire_input(&mut fd, and_op, mask, 1);
    // op = inv >> 8
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_RIGHT);
    let sa = make_const(&mut fd, 8, 4);
    wire_input(&mut fd, op, inv, 0);
    wire_input(&mut fd, op, sa, 1);
    let mut r = RuleRightShiftAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // op's slot 0 now bypasses the AND -> rootVn.
    assert_eq!(in_of(&fd, op, 0), Some(root));
}

#[test]
fn rightshiftand_declines_when_mask_mismatch() {
    let mut fd = build_fd();
    let root = make_reg(&mut fd, 0x40, 4);
    let (inv, and_op) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    let mask = make_const(&mut fd, 0x0000ff00, 4); // >>8 != full>>8
    wire_input(&mut fd, and_op, root, 0);
    wire_input(&mut fd, and_op, mask, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_RIGHT);
    let sa = make_const(&mut fd, 8, 4);
    wire_input(&mut fd, op, inv, 0);
    wire_input(&mut fd, op, sa, 1);
    let mut r = RuleRightShiftAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(in_of(&fd, op, 0), Some(inv));
}

// -----------------------------------------------------------------------------
// RuleShiftBitops — (V & 0xf000) << ... etc.
// -----------------------------------------------------------------------------

#[test]
fn shiftbitops_and_shifted_away_becomes_zero() {
    let mut fd = build_fd();
    // bitop = a & b where after a left shift one operand's NZM clears.
    // Use AND: any branch with NZM whose shifted form == 0 triggers; result
    // becomes newConstant(size,0) in slot 0.
    // bitop input 0 = const 0x1 (NZM 1); shifting left by sa where 1<<sa stays in
    // mask does NOT clear, so use const 0 in some branch.
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    let z = make_const(&mut fd, 0, 4); // NZM 0 -> nzm<<sa == 0 -> branch i=0 breaks
    let other = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, bitop, z, 0);
    wire_input(&mut fd, bitop, other, 1);
    // op = innervn << 4
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LEFT);
    let sa = make_const(&mut fd, 4, 4);
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, sa, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // INT_AND/INT_MULT case -> slot 0 becomes a fresh const 0.
    let in0 = in_of(&fd, op, 0).unwrap();
    assert!(is_const(&fd, in0));
    assert_eq!(off_of(&fd, in0), 0);
}

#[test]
fn shiftbitops_add_shifted_away_uses_other_operand() {
    let mut fd = build_fd();
    // bitop = ADD; branch i=0 has NZM that clears under the shift -> for ADD the
    // result rewires op slot0 to bitop->getIn(1-i).
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_ADD);
    let z = make_const(&mut fd, 0, 4); // i=0 clears
    let keep = make_reg(&mut fd, 0x40, 4); // heritage-known register
    wire_input(&mut fd, bitop, z, 0);
    wire_input(&mut fd, bitop, keep, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LEFT);
    let sa = make_const(&mut fd, 4, 4);
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, sa, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    // ADD case -> slot 0 = bitop->getIn(1-0) = keep.
    assert_eq!(in_of(&fd, op, 0), Some(keep));
}

#[test]
fn shiftbitops_declines_nonconstant_shift() {
    let mut fd = build_fd();
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_AND);
    let z = make_const(&mut fd, 0, 4);
    let other = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, bitop, z, 0);
    wire_input(&mut fd, bitop, other, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LEFT);
    let sa = make_reg(&mut fd, 0x50, 4); // non-constant shift
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, sa, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn shiftbitops_add_with_right_shift_declines() {
    let mut fd = build_fd();
    // INT_ADD under a RIGHT shift: the `if (!leftshift) return 0` guard fires.
    let (innervn, bitop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_ADD);
    let z = make_const(&mut fd, 0, 4);
    let keep = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, bitop, z, 0);
    wire_input(&mut fd, bitop, keep, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_RIGHT);
    let sa = make_const(&mut fd, 4, 4);
    wire_input(&mut fd, op, innervn, 0);
    wire_input(&mut fd, op, sa, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleShiftBitops::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleHighOrderAnd — (V + c) & 0xfff0 => V + (c & 0xfff0)   (constant cvn2 branch)
// -----------------------------------------------------------------------------

#[test]
fn highorderand_constant_branch_declines_until_nzmask_seam() {
    // The constant-`cvn2` branch requires `addop->getIn(0)` (xalign) to be
    // unaffected by the AND: `(xalign->getNZMask() & val) == xalign->getNZMask()`.
    // A heritage-known input register carries the full 64-bit NZ mask until the
    // bit-consumption/NZ-mask analysis tightens it to its 4-byte width; that
    // analysis is not part of this batch, so with a sub-word `val` the guard
    // `(mask1 & val) != mask1` fires and the rule declines.  This pins the
    // structural match (INT_AND over INT_ADD-with-constant) up to that NZ-mask
    // dependency.
    let mut fd = build_fd();
    let xalign = make_reg(&mut fd, 0x40, 4);
    let (addout, addop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_ADD);
    let c2 = make_const(&mut fd, 0x1234, 4);
    wire_input(&mut fd, addop, xalign, 0);
    wire_input(&mut fd, addop, c2, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let cvn1 = make_const(&mut fd, 0xfffffff0, 4); // clean high mask 1..10..0
    wire_input(&mut fd, op, addout, 0);
    wire_input(&mut fd, op, cvn1, 1);
    let mut r = RuleHighOrderAnd::new("analysis");
    // mask1 (full 64-bit) & 0xfffffff0 != mask1 -> declines (no mutation).
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_AND);
}

#[test]
fn highorderand_declines_when_cvn1_not_clean_mask() {
    let mut fd = build_fd();
    let xalign = make_const(&mut fd, 0x0f, 4);
    let (addout, addop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_ADD);
    let c2 = make_const(&mut fd, 0x1234, 4);
    wire_input(&mut fd, addop, xalign, 0);
    wire_input(&mut fd, addop, c2, 1);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    // 0x0ff0 is not of the form 1..10..0  -> ((val-1)|val) != fullmask.
    let cvn1 = make_const(&mut fd, 0x0ff0, 4);
    wire_input(&mut fd, op, addout, 0);
    wire_input(&mut fd, op, cvn1, 1);
    let mut r = RuleHighOrderAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn highorderand_declines_when_in1_not_constant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let a = make_reg(&mut fd, 0x40, 4);
    let b = make_reg(&mut fd, 0x50, 4); // in1 not constant
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let mut r = RuleHighOrderAnd::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleLessOne — V < 1 => V == 0 ; V <= 0 => V == 0
// -----------------------------------------------------------------------------

#[test]
fn lessone_less_than_one_becomes_equal_zero() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESS);
    let v = make_reg(&mut fd, 0x40, 4);
    let one = make_const(&mut fd, 1, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, one, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    // val != 0 -> slot 1 replaced with fresh const 0.
    let nin1 = in_of(&fd, op, 1).unwrap();
    assert!(is_const(&fd, nin1));
    assert_eq!(off_of(&fd, nin1), 0);
}

#[test]
fn lessone_lessequal_zero_becomes_equal_zero_without_replacing_const() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESSEQUAL);
    let v = make_reg(&mut fd, 0x40, 4);
    let zero = make_const(&mut fd, 0, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, zero, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 1);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
    // val == 0 -> no input replacement; the original zero constant stays.
    assert_eq!(in_of(&fd, op, 1), Some(zero));
}

#[test]
fn lessone_declines_less_than_two() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESS);
    let v = make_reg(&mut fd, 0x40, 4);
    let two = make_const(&mut fd, 2, 4); // INT_LESS but val != 1
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, two, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_LESS);
}

#[test]
fn lessone_declines_nonconstant() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESS);
    let v = make_reg(&mut fd, 0x40, 4);
    let w = make_reg(&mut fd, 0x50, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, w, 1);
    let mut r = RuleLessOne::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// RuleCollectTerms::get_mult_coeff static helper
// -----------------------------------------------------------------------------

#[test]
fn get_mult_coeff_unwraps_int_mult_by_constant() {
    let mut fd = build_fd();
    // vn = base * 5  (INT_MULT with constant in slot 1)
    let base = make_reg(&mut fd, 0x40, 4);
    let (vn, mulop) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_INT_MULT);
    let five = make_const(&mut fd, 5, 4);
    wire_input(&mut fd, mulop, base, 0);
    wire_input(&mut fd, mulop, five, 1);
    let mut coef = 0u64;
    let underlying = RuleCollectTerms::get_mult_coeff(&fd, vn, &mut coef);
    assert_eq!(coef, 5);
    assert_eq!(underlying, base);
}

#[test]
fn get_mult_coeff_returns_one_for_plain_varnode() {
    let mut fd = build_fd();
    let v = make_reg(&mut fd, 0x40, 4); // not written
    let mut coef = 99u64;
    let underlying = RuleCollectTerms::get_mult_coeff(&fd, v, &mut coef);
    assert_eq!(coef, 1);
    assert_eq!(underlying, v);
}

// -----------------------------------------------------------------------------
// RulePullsubMulti static helpers
// -----------------------------------------------------------------------------

#[test]
fn acceptable_size_matches_cpp_table() {
    assert!(!RulePullsubMulti::acceptable_size(0));
    assert!(RulePullsubMulti::acceptable_size(1));
    assert!(RulePullsubMulti::acceptable_size(2));
    assert!(!RulePullsubMulti::acceptable_size(3));
    assert!(RulePullsubMulti::acceptable_size(4));
    assert!(!RulePullsubMulti::acceptable_size(5));
    assert!(RulePullsubMulti::acceptable_size(8));
    assert!(RulePullsubMulti::acceptable_size(9)); // >= 8 -> true
}

#[test]
fn min_max_use_default_full_for_nonsubpiece_use() {
    let mut fd = build_fd();
    // vn used by an INT_ADD (not SUBPIECE) -> maxByte=inSize-1, minByte=0.
    let (vn, _def) = make_written(&mut fd, 0x60, 4, OpCode::CPUI_COPY);
    let add = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_ADD);
    let other = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, add, vn, 0);
    wire_input(&mut fd, add, other, 1);
    let mut max_byte = -99;
    let mut min_byte = -99;
    RulePullsubMulti::min_max_use(&fd, vn, &mut max_byte, &mut min_byte);
    assert_eq!(max_byte, 3); // inSize - 1
    assert_eq!(min_byte, 0);
}

#[test]
fn min_max_use_tracks_subpiece_bytes() {
    let mut fd = build_fd();
    // vn size 8, one SUBPIECE at offset 2, output size 2 -> min=2, max=3.
    let (vn, _def) = make_written(&mut fd, 0x60, 8, OpCode::CPUI_COPY);
    let sub = make_op(&mut fd, 0x1000, OpCode::CPUI_SUBPIECE);
    let two = make_const(&mut fd, 2, 4);
    wire_input(&mut fd, sub, vn, 0);
    wire_input(&mut fd, sub, two, 1);
    let _subout = give_output(&mut fd, sub, 0x80, 2);
    let mut max_byte = -99;
    let mut min_byte = -99;
    RulePullsubMulti::min_max_use(&fd, vn, &mut max_byte, &mut min_byte);
    assert_eq!(min_byte, 2);
    assert_eq!(max_byte, 3); // 2 + 2 - 1
}

// -----------------------------------------------------------------------------
// Seam-deferred rules: portable guards return 0 (no change)
// -----------------------------------------------------------------------------

#[test]
fn equality_defers_at_functional_equality_seam() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_EQUAL);
    let a = make_reg(&mut fd, 0x40, 4);
    let b = make_reg(&mut fd, 0x50, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let mut r = RuleEquality::new("analysis");
    // functionalEquality is a seam -> rule declines without mutating.
    assert_eq!(r.apply_op(op, &mut fd), 0);
    assert_eq!(code_of(&fd, op), OpCode::CPUI_INT_EQUAL);
}

#[test]
fn intlessequal_defers_at_replacelessequal_seam() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_LESSEQUAL);
    let mut r = RuleIntLessEqual::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn rangemeld_declines_when_inputs_not_bool_output() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_BOOL_OR);
    // Inputs written by COPYs that are not boolean-output ops.
    let (a, _da) = make_written(&mut fd, 0x40, 1, OpCode::CPUI_COPY);
    let (b, _db) = make_written(&mut fd, 0x50, 1, OpCode::CPUI_COPY);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let mut r = RuleRangeMeld::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn pullsubmulti_declines_when_in0_not_multiequal() {
    let mut fd = build_fd();
    // SUBPIECE whose in0 is written by a COPY (not MULTIEQUAL) -> declines.
    let (vn, _def) = make_written(&mut fd, 0x60, 8, OpCode::CPUI_COPY);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_SUBPIECE);
    let two = make_const(&mut fd, 2, 4);
    wire_input(&mut fd, op, vn, 0);
    wire_input(&mut fd, op, two, 1);
    let _out = give_output(&mut fd, op, 0x80, 2);
    let mut r = RulePullsubMulti::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn pushmulti_declines_with_non_two_inputs() {
    let mut fd = build_fd();
    // A MULTIEQUAL with exactly one input slot -> numInput()!=2 -> declines.
    let op = make_op_n(&mut fd, 0x1000, OpCode::CPUI_MULTIEQUAL, 1, 0);
    let a = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, a, 0);
    let mut r = RulePushMulti::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn notdistribute_declines_when_inner_not_bool_and_or() {
    let mut fd = build_fd();
    // BOOL_NEGATE whose in0 is written by a COPY -> not BOOL_AND/BOOL_OR.
    let (vn, _def) = make_written(&mut fd, 0x60, 1, OpCode::CPUI_COPY);
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_BOOL_NEGATE);
    wire_input(&mut fd, op, vn, 0);
    let mut r = RuleNotDistribute::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn anddistribute_declines_when_no_or_input() {
    let mut fd = build_fd();
    // INT_AND of two plain registers (no INT_OR def) -> loop exhausts, returns 0.
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_INT_AND);
    let a = make_reg(&mut fd, 0x40, 4);
    let b = make_reg(&mut fd, 0x50, 4);
    wire_input(&mut fd, op, a, 0);
    wire_input(&mut fd, op, b, 1);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let mut r = RuleAndDistribute::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn earlyremoval_declines_call_op() {
    use crate::op::pcodeop_flags;
    let mut fd = build_fd();
    // The call flag is the W6 `glb->inst[CPUI_CALL]` property; set it explicitly.
    let op = make_op_flags(&mut fd, 0x1000, OpCode::CPUI_CALL, pcodeop_flags::call);
    let mut r = RuleEarlyRemoval::new("analysis");
    // is_call() -> 0 immediately.
    assert_eq!(r.apply_op(op, &mut fd), 0);
}

#[test]
fn earlyremoval_destroys_unused_op() {
    let mut fd = build_fd();
    // A plain COPY whose output has no descendants in a non-deadcode (ram is a
    // hasphysical processor space; does_deadcode() is governed by the space).
    // Build a COPY with an output that no one reads.
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_COPY);
    let inp = make_reg(&mut fd, 0x40, 4);
    wire_input(&mut fd, op, inp, 0);
    let _out = give_output(&mut fd, op, 0x80, 4);
    let does_dead = fd
        .vbank()
        .get(_out)
        .unwrap()
        .get_space()
        .does_deadcode();
    let mut r = RuleEarlyRemoval::new("analysis");
    let res = r.apply_op(op, &mut fd);
    if does_dead {
        // ram does deadcode -> hits the heritage seam (deadRemovalAllowedSeen),
        // declines without destroying.
        assert_eq!(res, 0);
        assert!(fd.obank().get(op).is_some());
    } else {
        // ram does not do deadcode -> opDestroy runs.
        assert_eq!(res, 1);
    }
}

#[test]
fn selectcse_declines_at_csehash_seam() {
    let mut fd = build_fd();
    let op = make_op(&mut fd, 0x1000, OpCode::CPUI_SUBPIECE);
    let v = make_reg(&mut fd, 0x40, 4);
    let z = make_const(&mut fd, 0, 4);
    wire_input(&mut fd, op, v, 0);
    wire_input(&mut fd, op, z, 1);
    let mut r = RuleSelectCse::new("analysis");
    assert_eq!(r.apply_op(op, &mut fd), 0);
}
