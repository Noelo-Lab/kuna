//! Tests for `RuleBoolSignShift` (kuna GH-1282): the `(b << k) s>> k` ->
//! `INT_2COMP(b)` rewrite and its gate-off / non-matching behaviour.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::action::Rule;
use crate::dtype::{type_metatype, Datatype};
use crate::context::{ArchContext, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

// --- harness (mirrors funcdata_varnode tests) -------------------------------

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

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()>
{
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(())
}

/// A fresh op with the given opcode at `off`, with `inputs` input slots.
fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

/// Make `vn` the written output of `defop` (bank-level set_def, bypassing the
/// xref split-borrow seam — same trick the funcdata_varnode tests use).
fn set_def(fd: &mut Funcdata, vn: VarnodeId, defop: OpId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

/// Wire `vn` as input `slot` of `op` (descend + input link).
fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Wire `vn` as input `slot` of `op` WITHOUT recording the descend edge.  Used
/// for the constant `boolvn` so that, when the rule relinks it to the rewritten
/// op, the constant is not (yet) a multiple-descendant *free* varnode — the C++
/// `opSetInput` duplicates a shared constant, but that constant-dup branch is a
/// deferred seam in the merged `funcdata_op::op_set_input` (lines noting "the
/// duplication body is the deferred part").  Reading `leftshift->getIn(0)` does
/// not consult the descend list, so the rule's match is unaffected.
fn wire_no_descend(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Build the canonical idiom graph:
///   boolvn (const `boolval`, size 1, NZMask = boolval)
///   leftshift = INT_LEFT(boolvn, #sa)  -> shiftin (unique, size 1)
///   op        = INT_SRIGHT(shiftin, #sa) -> out (unique, size 1)
/// Returns (op, boolvn).
fn build_idiom(fd: &mut Funcdata, sa: u64, boolval: u64) -> (OpId, VarnodeId) {
    let r = ram(fd);

    // boolvn: a known small constant so its NZMask == its value (== boolval).
    let boolvn = fd.new_constant(1, boolval);

    // leftshift = INT_LEFT(boolvn, #sa).  boolvn is wired without a descend edge
    // (see `wire_no_descend`) so the rule can relink the shared constant.
    let leftshift = mk_op(fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let sa_const_l = fd.new_constant(1, sa);
    wire_no_descend(fd, boolvn, leftshift, 0);
    wire(fd, sa_const_l, leftshift, 1);
    let shiftin = fd.new_unique(1, Some(unk(1)));
    let shiftin = set_def(fd, shiftin, leftshift);

    // op = INT_SRIGHT(shiftin, #sa)
    let op = mk_op(fd, 0x104, 2, OpCode::CPUI_INT_SRIGHT);
    let sa_const_r = fd.new_constant(1, sa);
    wire(fd, shiftin, op, 0);
    wire(fd, sa_const_r, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _out = set_def(fd, out, op);
    let _ = r;

    (op, boolvn)
}

#[test]
fn transforms_bool_sign_shift_to_2comp() {
    let mut fd = build_fd();
    // sa=7 (not a multiple of 8), boolean value 1 (NZMask 1 fits low (8-7)=1 bit).
    let (op, boolvn) = build_idiom(&mut fd, 7, 1);

    let mut rule = RuleBoolSignShift::new(true);
    let changed = rule.apply_op(op, &mut fd);
    assert_eq!(changed, 1, "rule must fire and report a change");

    // op is now INT_2COMP(boolvn) with a single input.
    let o = fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_2COMP);
    assert_eq!(o.num_input(), 1);
    assert_eq!(o.get_in(0), Some(boolvn));
}

#[test]
fn gate_off_is_noop() {
    let mut fd = build_fd();
    let (op, _bool) = build_idiom(&mut fd, 7, 1);

    let mut rule = RuleBoolSignShift::new(false); // gate off
    let changed = rule.apply_op(op, &mut fd);
    assert_eq!(changed, 0, "gate off: byte-identical to upstream (no change)");
    // op unchanged: still INT_SRIGHT with two inputs.
    let o = fd.obank().get(op).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_SRIGHT);
    assert_eq!(o.num_input(), 2);
}

#[test]
fn byte_aligned_shift_defers_to_leftright() {
    let mut fd = build_fd();
    // sa=8 is a multiple of 8 -> RuleLeftRight handles it; this rule declines.
    let (op, _bool) = build_idiom(&mut fd, 8, 1);

    let mut rule = RuleBoolSignShift::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SRIGHT);
}

#[test]
fn nzmask_above_low_bits_declines() {
    let mut fd = build_fd();
    // sa=7, but boolean value 0xff has NZMask bits above the low (8-7)=1 bit.
    let (op, _bool) = build_idiom(&mut fd, 7, 0xff);

    let mut rule = RuleBoolSignShift::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(fd.obank().get(op).unwrap().code(), OpCode::CPUI_INT_SRIGHT);
}

#[test]
fn mismatched_shift_amounts_decline() {
    let mut fd = build_fd();
    let r = ram(&fd);
    // leftshift uses sa=7, but the s>> uses sa=6 -> no match.
    let boolvn = fd.new_constant(1, 1);
    let leftshift = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_LEFT);
    let l_sa = fd.new_constant(1, 7);
    wire(&mut fd, boolvn, leftshift, 0);
    wire(&mut fd, l_sa, leftshift, 1);
    let shiftin = fd.new_unique(1, Some(unk(1)));
    let shiftin = set_def(&mut fd, shiftin, leftshift);

    let op = mk_op(&mut fd, 0x104, 2, OpCode::CPUI_INT_SRIGHT);
    let r_sa = fd.new_constant(1, 6); // different amount
    wire(&mut fd, shiftin, op, 0);
    wire(&mut fd, r_sa, op, 1);
    let out = fd.new_unique(1, Some(unk(1)));
    let _ = set_def(&mut fd, out, op);
    let _ = r;

    let mut rule = RuleBoolSignShift::new(true);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
}

#[test]
fn specs_has_one_rule_in_analysis() {
    let specs = specs();
    assert_eq!(specs.len(), 1);
    // C++ registers RuleBoolSignShift("analysis") in oppool1 (coreaction.cc:5811).
    assert_eq!(specs[0].group, "analysis");
    // The constructed rule offers exactly INT_SRIGHT.
    let rule = (specs[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_INT_SRIGHT]);
}
