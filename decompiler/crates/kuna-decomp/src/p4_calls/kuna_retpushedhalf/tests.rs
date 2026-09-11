//! Tests for the push-only register record.
//!
//! The fixture builds what the flow build hands [`observe_instruction`]: the ops
//! of one machine instruction, in emission order. An x86-64 `push %r8` is
//! `RSP = RSP - 8` followed by `STORE(ram, RSP, R8)`; a `pop %rdx` is
//! `RDX = LOAD(ram, RSP)` followed by `RSP = RSP + 8`. The end-to-end witness is
//! `tests/stages/kuna-retpushedhalf.xml`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};

use crate::context::{ArchContext, TypeOp};

/// Register-space offsets, x86-64 SLEIGH: RAX 0x00, RDX 0x10, RSP 0x20, R8 0x80.
const RDX: u64 = 0x10;
const RSP: u64 = 0x20;
const R8: u64 = 0x80;

fn build_fd() -> Funcdata {
    build_fd_with(true)
}

fn build_fd_with(enabled: bool) -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let regspc = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    m.insert_space(Rc::clone(&regspc)).unwrap();
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &regspc, 1, true, false))).unwrap();
    let stackspc = Rc::clone(m.get_stack_space().unwrap());
    let sp = VarnodeStorage { space: Some(Rc::clone(&regspc)), offset: RSP, size: 8 };
    m.add_spacebase_pointer(&stackspc, &sp, 8, true).unwrap();

    let mut ctx = ArchContext::new(m);
    ctx.ret_pushed_half = enabled;
    let glb = Rc::new(ctx);
    let entry = Address::new(Rc::clone(glb.manage().get_space_by_name("register").unwrap()), 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn reg(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap()), off)
}

fn mk_op(fd: &mut Funcdata, ninput: int4, opc: OpCode) -> OpId {
    let addr = reg(fd, 0x1000);
    let op = fd.new_op(ninput, addr);
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

/// `RSP = RSP <opc> 8`.
fn sp_adjust(fd: &mut Funcdata, opc: OpCode) -> OpId {
    let op = mk_op(fd, 2, opc);
    let cur = fd.new_varnode(8, &reg(fd, RSP), None);
    let eight = fd.new_constant(8, 8);
    fd.op_set_input(op, cur, 0).unwrap();
    fd.op_set_input(op, eight, 1).unwrap();
    let out = reg(fd, RSP);
    fd.new_varnode_out(8, &out, op).unwrap();
    op
}

/// `STORE(ram, RSP, <register off>)` — the push's value half.
fn store_reg(fd: &mut Funcdata, off: u64, size: int4) -> OpId {
    let op = mk_op(fd, 3, OpCode::CPUI_STORE);
    let spaceid = fd.new_constant(4, 2);
    let ptr = fd.new_varnode(8, &reg(fd, RSP), None);
    let val = fd.new_varnode(size, &reg(fd, off), None);
    fd.op_set_input(op, spaceid, 0).unwrap();
    fd.op_set_input(op, ptr, 1).unwrap();
    fd.op_set_input(op, val, 2).unwrap();
    op
}

/// `<register off> = LOAD(ram, RSP)` — the pop's value half.
fn load_reg(fd: &mut Funcdata, off: u64, size: int4) -> OpId {
    let op = mk_op(fd, 2, OpCode::CPUI_LOAD);
    let spaceid = fd.new_constant(4, 2);
    let ptr = fd.new_varnode(8, &reg(fd, RSP), None);
    fd.op_set_input(op, spaceid, 0).unwrap();
    fd.op_set_input(op, ptr, 1).unwrap();
    let out = reg(fd, off);
    fd.new_varnode_out(size, &out, op).unwrap();
    op
}

fn push(fd: &mut Funcdata, off: u64) {
    let a = sp_adjust(fd, OpCode::CPUI_INT_SUB);
    let s = store_reg(fd, off, 8);
    observe_instruction(fd, &[a, s]);
}

fn pop(fd: &mut Funcdata, off: u64) {
    let l = load_reg(fd, off, 8);
    let a = sp_adjust(fd, OpCode::CPUI_INT_ADD);
    observe_instruction(fd, &[l, a]);
}

fn is_push_only(fd: &Funcdata, off: u64) -> bool {
    fd.kuna_pushed_registers().is_push_only(&reg(fd, off), 8)
}

/// The defect's idiom: the pushed register is never written, the pop lands
/// somewhere else. Only the pushed register is push-only.
#[test]
fn push_one_register_and_pop_another_marks_only_the_source() {
    let mut fd = build_fd();
    push(&mut fd, R8);
    pop(&mut fd, RDX);
    assert!(is_push_only(&fd, R8), "R8 is stored by a stack-adjusting op and never written");
    assert!(!is_push_only(&fd, RDX), "RDX is written by the pop");
    assert!(!is_push_only(&fd, RSP), "the stack pointer itself is never a saved value");
}

/// The ordinary callee-saved save/restore: the pop writes the register back, so
/// it is not push-only and the placement test is unaffected.
#[test]
fn push_and_pop_back_into_itself_is_not_push_only() {
    let mut fd = build_fd();
    push(&mut fd, R8);
    pop(&mut fd, R8);
    assert!(!is_push_only(&fd, R8));
}

/// A write anywhere in the function disqualifies the register, whatever its
/// size: the saved value was one the function produced.
#[test]
fn a_later_write_of_any_width_disqualifies_the_saved_register() {
    let mut fd = build_fd();
    push(&mut fd, R8);
    let op = mk_op(&mut fd, 1, OpCode::CPUI_COPY);
    let out = reg(&fd, R8);
    fd.new_varnode_out(4, &out, op).unwrap();
    observe_instruction(&mut fd, &[op]);
    assert!(!is_push_only(&fd, R8), "a 4-byte write overlaps the 8-byte saved register");
}

/// A store that is not part of a stack adjustment is an ordinary memory write,
/// not a push.
#[test]
fn a_store_without_a_stack_adjustment_records_nothing() {
    let mut fd = build_fd();
    let s = store_reg(&mut fd, R8, 8);
    observe_instruction(&mut fd, &[s]);
    assert!(!is_push_only(&fd, R8));
}

/// With the option off nothing is recorded, so the record can never narrow
/// `retinputhalf`.
#[test]
fn the_option_gates_the_record() {
    let mut fd = build_fd_with(false);
    push(&mut fd, R8);
    pop(&mut fd, RDX);
    assert!(!is_push_only(&fd, R8));
}
