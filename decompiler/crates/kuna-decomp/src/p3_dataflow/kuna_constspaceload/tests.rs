//! Tests for `RuleConstSpaceLoad` (kuna `constspaceload`): the
//! `LOAD(const, p)` -> value-of-`p` rewrite at each output width, and every
//! clause that declines it.

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
use crate::context::{ArchContext, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::varnode::{DefOpInfo, VarnodeBank};

/// Manager index of the constant space (inserted first) and of `ram`.
const CONST_SPACE_IDX: u64 = 0;
const RAM_SPACE_IDX: u64 = 4;

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

fn build_fd(gate: bool) -> Funcdata {
    let manage = build_manager();
    let mut ctx = ArchContext::new(manage);
    ctx.const_space_load_fold = gate;
    let glb = Rc::new(ctx);
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

fn no_replace(
) -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(())
}

fn mk_op(fd: &mut Funcdata, off: u64, inputs: int4, opc: OpCode) -> OpId {
    let r = ram(fd);
    let pc = Address::new(r, off);
    let op = fd.obank_mut().create_at(inputs, pc);
    fd.obank_mut().mark_alive(op);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn set_def(fd: &mut Funcdata, vn: VarnodeId, defop: OpId) -> VarnodeId {
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    vn
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// Build `out:outsize = LOAD(#spaceidx, ptr:ptrsize)`, where `ptr` is either a
/// defined temporary (the SLEIGH dynamic-export shape) or a constant.
fn build_load(
    fd: &mut Funcdata,
    space_idx: u64,
    ptr_size: int4,
    out_size: int4,
    ptr_is_constant: bool,
) -> (OpId, VarnodeId) {
    let op = mk_op(fd, 0x100, 2, OpCode::CPUI_LOAD);
    let spc = fd.new_constant(4, space_idx);
    wire(fd, spc, op, 0);
    let ptr = if ptr_is_constant {
        let c = fd.new_constant(ptr_size, 0);
        wire(fd, c, op, 1);
        c
    } else {
        let ptrdef = mk_op(fd, 0x0f8, 1, OpCode::CPUI_COPY);
        let p = fd.new_unique(ptr_size, Some(unk(ptr_size)));
        let p = set_def(fd, p, ptrdef);
        wire(fd, p, op, 1);
        p
    };
    let out = fd.new_unique(out_size, Some(unk(out_size)));
    let _out = set_def(fd, out, op);
    (op, ptr)
}

fn shape(fd: &Funcdata, op: OpId) -> (OpCode, Option<VarnodeId>, int4) {
    let o = fd.obank().get(op).unwrap();
    (o.code(), o.get_in(0), o.num_input())
}

#[test]
fn a_same_width_const_space_load_becomes_a_copy_of_the_pointer() {
    // `tmp:16 = 0; export *[const]:16 tmp;` — the ARM NEON vmov.i32 shape.
    let mut fd = build_fd(true);
    let (op, ptr) = build_load(&mut fd, CONST_SPACE_IDX, 16, 16, false);

    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 1);

    let (code, in0, n) = shape(&fd, op);
    assert_eq!(code, OpCode::CPUI_COPY);
    assert_eq!(in0, Some(ptr), "the exported value IS the address operand");
    assert_eq!(n, 1);
}

#[test]
fn a_narrower_read_is_declined() {
    let mut fd = build_fd(true);
    let (op, _ptr) = build_load(&mut fd, CONST_SPACE_IDX, 8, 4, false);

    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0, "a width mismatch is not the identity");

    let (code, _, _) = shape(&fd, op);
    assert_eq!(code, OpCode::CPUI_LOAD);
}

#[test]
fn a_wider_read_is_declined() {
    let mut fd = build_fd(true);
    let (op, _ptr) = build_load(&mut fd, CONST_SPACE_IDX, 4, 8, false);

    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0, "a width mismatch is not the identity");

    let (code, _, _) = shape(&fd, op);
    assert_eq!(code, OpCode::CPUI_LOAD);
}

#[test]
fn a_real_memory_load_is_left_alone() {
    let mut fd = build_fd(true);
    let (op, _ptr) = build_load(&mut fd, RAM_SPACE_IDX, 8, 8, false);
    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(shape(&fd, op).0, OpCode::CPUI_LOAD);
}

#[test]
fn a_constant_pointer_is_left_to_ruleloadvarnode() {
    // RuleLoadVarnode also resolves the spacebase-placeholder tail, which a
    // bare COPY would drop.
    let mut fd = build_fd(true);
    let (op, _ptr) = build_load(&mut fd, CONST_SPACE_IDX, 8, 8, true);
    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(shape(&fd, op).0, OpCode::CPUI_LOAD);
}

#[test]
fn the_gate_off_leaves_the_load_alone() {
    let mut fd = build_fd(false);
    let (op, _ptr) = build_load(&mut fd, CONST_SPACE_IDX, 16, 16, false);
    let mut rule = RuleConstSpaceLoad::new(false);
    assert_eq!(rule.apply_op(op, &mut fd), 0);
    assert_eq!(shape(&fd, op).0, OpCode::CPUI_LOAD);
}
