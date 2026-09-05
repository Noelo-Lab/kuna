//! Logic-level tests for `kuna_is_frame_teardown_tail_call`, exercising the
//! prologue/epilogue stack-delta decision on hand-built raw IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    VarnodeStorage,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, OpId, TypeOp};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

use super::*;

/// const(0), unique(1), ram(2, IPTR_PROCESSOR), register(3).
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
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        addrspace_flags::hasphysical,
        0,
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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn unk_type(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn space_of(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

/// The stack-pointer register location the driver resolves from the stack
/// space's base register: `register:0x20`, 8 bytes.
fn sp_loc(fd: &Funcdata) -> VarnodeStorage {
    VarnodeStorage { space: Some(space_of(fd, "register")), offset: 0x20, size: 8 }
}

fn sp_vn(fd: &mut Funcdata) -> crate::context::VarnodeId {
    let reg = space_of(fd, "register");
    fd.vbank_mut().create(8, Address::new(reg, 0x20), unk_type(8))
}

fn const_vn(fd: &mut Funcdata, val: u64) -> crate::context::VarnodeId {
    let cst = space_of(fd, "const");
    fd.vbank_mut().create(8, Address::new(cst, val), unk_type(8))
}

/// `SP = SP <opc> k` at instruction address `at`.
fn emit_sp_arith(fd: &mut Funcdata, at: u64, opc: OpCode, k: u64) -> OpId {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(2, Address::new(ram, at));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    let s0 = sp_vn(fd);
    let c = const_vn(fd, k);
    let out = sp_vn(fd);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(s0), 0);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(c), 1);
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
    op
}

/// A COPY that touches no stack pointer, at instruction address `at`.
fn emit_filler(fd: &mut Funcdata, at: u64) -> OpId {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(1, Address::new(ram, at));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "op"));
    let reg = space_of(fd, "register");
    let src = fd.vbank_mut().create(8, Address::new(Rc::clone(&reg), 0x18), unk_type(8));
    let dst = fd.vbank_mut().create(8, Address::new(reg, 0x10), unk_type(8));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(src), 0);
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(dst));
    op
}

/// A direct `CPUI_BRANCH` to `dest` at instruction address `at`.
fn emit_branch(fd: &mut Funcdata, at: u64, dest: u64) -> OpId {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(1, Address::new(Rc::clone(&ram), at));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_BRANCH, 0, "op"));
    let vn = fd.vbank_mut().create(8, Address::new(ram, dest), unk_type(8));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
    op
}

/// `push rbx; sub rsp,0x10; mov rbx,rdi` at 0x1000/0x1001/0x1005, then
/// `add rsp,0x10; pop rbx; jmp dest` at 0x1100/0x1104/0x1105.
fn build_teardown_shape(fd: &mut Funcdata, dest: u64) -> OpId {
    emit_sp_arith(fd, 0x1000, OpCode::CPUI_INT_SUB, 8);
    emit_sp_arith(fd, 0x1001, OpCode::CPUI_INT_SUB, 0x10);
    emit_filler(fd, 0x1005);
    emit_sp_arith(fd, 0x1100, OpCode::CPUI_INT_ADD, 0x10);
    emit_sp_arith(fd, 0x1104, OpCode::CPUI_INT_ADD, 8);
    emit_branch(fd, 0x1105, dest)
}

fn entry_addr(fd: &Funcdata) -> Address {
    fd.get_address().clone()
}

fn dest_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(space_of(fd, "ram"), off)
}

#[test]
fn gate_off_never_fires() {
    let mut fd = build_fd();
    let br = build_teardown_shape(&mut fd, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, false, &e, &d, Some(&sp)));
}

#[test]
fn exact_teardown_of_the_entry_frame_fires() {
    let mut fd = build_fd();
    let br = build_teardown_shape(&mut fd, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn no_stack_pointer_location_declines() {
    let mut fd = build_fd();
    let br = build_teardown_shape(&mut fd, 0x2000);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, None));
}

#[test]
fn partial_teardown_declines() {
    // `add rsp,0x10; jmp X` with a 0x18 frame: the frame is NOT torn down, so
    // this is ordinary intraprocedural flow.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_SUB, 8);
    emit_sp_arith(&mut fd, 0x1001, OpCode::CPUI_INT_SUB, 0x10);
    emit_filler(&mut fd, 0x1005);
    emit_sp_arith(&mut fd, 0x1100, OpCode::CPUI_INT_ADD, 0x10);
    let br = emit_branch(&mut fd, 0x1104, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn frameless_leaf_never_fires() {
    // No prologue at all: every unconditional jump would look like a tail call,
    // so with no frame there is no evidence and the rule declines.
    let mut fd = build_fd();
    emit_filler(&mut fd, 0x1000);
    let br = emit_branch(&mut fd, 0x1100, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn opaque_frame_pointer_restore_declines() {
    // `leave` restores SP from the frame pointer: not `SP = SP +/- <const>`, so
    // the scan cannot account for it.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_SUB, 8);
    emit_sp_arith(&mut fd, 0x1001, OpCode::CPUI_INT_SUB, 0x10);
    emit_filler(&mut fd, 0x1005);
    let ram = space_of(&fd, "ram");
    let op = fd.obank_mut().create_at(1, Address::new(ram, 0x1100));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "op"));
    let reg = space_of(&fd, "register");
    let src = fd.vbank_mut().create(8, Address::new(reg, 0x28), unk_type(8));
    let out = sp_vn(&mut fd);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(src), 0);
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
    let br = emit_branch(&mut fd, 0x1104, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn self_tail_recursion_excluded() {
    let mut fd = build_fd();
    let br = build_teardown_shape(&mut fd, 0x1000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x1000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn already_decoded_target_excluded() {
    // 0x1005 is an address this function has already decoded: whatever the
    // stack looks like, the branch is live flow inside the body being built.
    let mut fd = build_fd();
    let br = build_teardown_shape(&mut fd, 0x1005);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x1005));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}

#[test]
fn indirect_branch_excluded() {
    let mut fd = build_fd();
    build_teardown_shape(&mut fd, 0x2000);
    let ram = space_of(&fd, "ram");
    let op = fd.obank_mut().create_at(1, Address::new(Rc::clone(&ram), 0x1106));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_BRANCHIND, 0, "op"));
    let vn = fd.vbank_mut().create(8, Address::new(ram, 0x2000), unk_type(8));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, op, true, &e, &d, Some(&sp)));
}

#[test]
fn a_gap_before_the_branch_ends_the_epilogue_run() {
    // The instruction before the branch in ADDRESS order is 0x4c bytes away, so
    // it is not its fall-through neighbour and its delta must not be borrowed.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_SUB, 8);
    emit_sp_arith(&mut fd, 0x1001, OpCode::CPUI_INT_SUB, 0x10);
    emit_filler(&mut fd, 0x1005);
    emit_sp_arith(&mut fd, 0x1100, OpCode::CPUI_INT_ADD, 0x10);
    emit_sp_arith(&mut fd, 0x1104, OpCode::CPUI_INT_ADD, 8);
    let br = emit_branch(&mut fd, 0x1150, 0x2000);
    let sp = sp_loc(&fd);
    let (e, d) = (entry_addr(&fd), dest_addr(&fd, 0x2000));
    assert!(!kuna_is_frame_teardown_tail_call(&fd, br, true, &e, &d, Some(&sp)));
}
