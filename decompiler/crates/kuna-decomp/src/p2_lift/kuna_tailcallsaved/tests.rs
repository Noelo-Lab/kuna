//! Logic-level tests for the save/restore accounting behind `tailcallsaved`,
//! on hand-built raw IR shaped like the x86 `push`/`pop`/`add rsp,N` lowerings.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    VarnodeStorage,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, TypeOp};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

use super::*;

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

fn reg_vn(fd: &mut Funcdata, off: u64, size: int4) -> crate::context::VarnodeId {
    let reg = space_of(fd, "register");
    fd.vbank_mut().create(size, Address::new(reg, off), unk_type(size))
}

fn at(fd: &Funcdata, off: u64) -> Address {
    Address::new(space_of(fd, "ram"), off)
}

/// `SP = SP <opc> k` at instruction address `addr`.
fn emit_sp_arith(fd: &mut Funcdata, addr: u64, opc: OpCode, k: u64) {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(2, Address::new(ram, addr));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    let s0 = sp_vn(fd);
    let c = const_vn(fd, k);
    let out = sp_vn(fd);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(s0), 0);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(c), 1);
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
}

/// `*(ram, ptr) = <register of `size` bytes>` at instruction address `addr`.
fn emit_store(fd: &mut Funcdata, addr: u64, ptr: crate::context::VarnodeId, size: int4) {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(3, Address::new(Rc::clone(&ram), addr));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_STORE, 0, "op"));
    let cst = space_of(fd, "const");
    let spc = fd.vbank_mut().create(8, Address::new(cst, ram.get_index() as u64), unk_type(8));
    let val = reg_vn(fd, 0x08, size);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(spc), 0);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(ptr), 1);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(val), 2);
}

/// `<register of `size` bytes> = *(ram, ptr)` at instruction address `addr`.
fn emit_load(fd: &mut Funcdata, addr: u64, ptr: crate::context::VarnodeId, size: int4) {
    let ram = space_of(fd, "ram");
    let op = fd.obank_mut().create_at(2, Address::new(Rc::clone(&ram), addr));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_LOAD, 0, "op"));
    let cst = space_of(fd, "const");
    let spc = fd.vbank_mut().create(8, Address::new(cst, ram.get_index() as u64), unk_type(8));
    let out = reg_vn(fd, 0x08, size);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(spc), 0);
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(ptr), 1);
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
}

/// `push reg`: `SP = SP - 8; *(ram, SP) = reg`.
fn emit_push(fd: &mut Funcdata, addr: u64) {
    emit_sp_arith(fd, addr, OpCode::CPUI_INT_SUB, 8);
    let ptr = sp_vn(fd);
    emit_store(fd, addr, ptr, 8);
}

/// `pop reg`: `reg = *(ram, SP); SP = SP + 8`.
fn emit_pop(fd: &mut Funcdata, addr: u64) {
    let ptr = sp_vn(fd);
    emit_load(fd, addr, ptr, 8);
    emit_sp_arith(fd, addr, OpCode::CPUI_INT_ADD, 8);
}

#[test]
fn a_push_saves_its_width() {
    let mut fd = build_fd();
    emit_push(&mut fd, 0x1000);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_saved_bytes(&fd, &a, &sp, -8), 8);
    assert_eq!(kuna_stack_restored_bytes(&fd, &a, &sp, -8), 0);
}

#[test]
fn a_pop_restores_its_width() {
    let mut fd = build_fd();
    emit_pop(&mut fd, 0x1000);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_restored_bytes(&fd, &a, &sp, 8), 8);
    assert_eq!(kuna_stack_saved_bytes(&fd, &a, &sp, 8), 0);
}

#[test]
fn argument_cleanup_restores_nothing() {
    // `add esp,8` after a call: the stack pointer moves, no register comes back.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_ADD, 8);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_restored_bytes(&fd, &a, &sp, 8), 0);
}

#[test]
fn frame_allocation_saves_nothing() {
    // `sub rsp,0x10` reserves space; it stores no register into it.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_SUB, 0x10);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_saved_bytes(&fd, &a, &sp, -0x10), 0);
}

#[test]
fn a_store_that_does_not_move_the_stack_pointer_saves_nothing() {
    // `mov [rsp],rax` writes through the stack pointer without pushing.
    let mut fd = build_fd();
    let ptr = sp_vn(&mut fd);
    emit_store(&mut fd, 0x1000, ptr, 8);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_saved_bytes(&fd, &a, &sp, 0), 0);
}

#[test]
fn a_load_off_the_stack_pointer_restores_nothing() {
    // `mov rax,[rsp+8]` reads through a computed pointer, not the stack pointer.
    let mut fd = build_fd();
    let ptr = reg_vn(&mut fd, 0x30, 8);
    emit_load(&mut fd, 0x1000, ptr, 8);
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_ADD, 8);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_restored_bytes(&fd, &a, &sp, 8), 0);
}

#[test]
fn the_transfer_cannot_exceed_the_stack_pointer_motion() {
    // A multi-word access through the stack pointer accounts for at most what
    // the instruction actually moved.
    let mut fd = build_fd();
    emit_sp_arith(&mut fd, 0x1000, OpCode::CPUI_INT_SUB, 4);
    let ptr = sp_vn(&mut fd);
    emit_store(&mut fd, 0x1000, ptr, 8);
    let (sp, a) = (sp_loc(&fd), at(&fd, 0x1000));
    assert_eq!(kuna_stack_saved_bytes(&fd, &a, &sp, -4), 4);
}

#[test]
fn the_gate_off_keeps_the_delta_only_decision() {
    assert!(kuna_teardown_restores_saves(false, 8, 0));
    assert!(!kuna_teardown_restores_saves(true, 8, 0));
}

#[test]
fn a_frame_that_saved_nothing_demands_nothing() {
    assert!(kuna_teardown_restores_saves(true, 0, 0));
}

#[test]
fn restoring_everything_saved_is_a_teardown() {
    assert!(kuna_teardown_restores_saves(true, 8, 8));
    assert!(kuna_teardown_restores_saves(true, 8, 16));
    assert!(!kuna_teardown_restores_saves(true, 16, 8));
}
