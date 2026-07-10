//! Logic-level tests for `kuna_is_sparc_struct_ret_trap` (GH-6882), exercising
//! the positional pre-SSA walk to the trap CALLOTHER on hand-built dead-list IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::op::pcodeop_flags;
use crate::funcdata::Funcdata;
use crate::context::{ArchContext, OpId, TypeOp};

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

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("const").unwrap())
}

/// Create an op at `pc_off` with the given opcode; if `start` set the
/// instruction-start mark.  Returns the op id (appended to the dead list).
fn make_op(fd: &mut Funcdata, opc: OpCode, pc_off: u64, ninputs: int4, start: bool) -> OpId {
    let pc = Address::new(ram(fd), pc_off);
    let op = fd.obank_mut().create_at(ninputs, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    if start {
        fd.obank_mut().get_mut(op).unwrap().set_flag(pcodeop_flags::startmark);
    }
    op
}

/// Set op input-`slot` to a constant varnode of `value` (the user-op id).
fn set_const_input(fd: &mut Funcdata, op: OpId, slot: int4, value: uintb) {
    let cspace = const_space(fd);
    let vn = fd
        .vbank_mut()
        .create(4, Address::new(cspace, value), unk_type(4));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

/// The canonical SPARC `unimp` idiom in one instruction:
///   CALLOTHER(IllegalInstructionTrap id) [instruction-start]
///   BRANCHIND [dest]                       (last op)
/// Returns the BRANCHIND op id.  `trap_id` is the constant user-op id on the
/// CALLOTHER's input 0.
fn build_unimp_idiom(fd: &mut Funcdata, trap_id: uintb) -> OpId {
    let callother = make_op(fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(fd, callother, 0, trap_id);
    // BRANCHIND in the same instruction (not an instruction-start).
    make_op(fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false)
}

/// userop resolver: id 7 -> "IllegalInstructionTrap", everything else -> a
/// different name (a non-null UserPcodeOp with a different name).
fn resolver(id: u32) -> Option<String> {
    match id {
        7 => Some("IllegalInstructionTrap".to_string()),
        9 => Some("SomeOtherTrap".to_string()),
        _ => None, // null UserPcodeOp *
    }
}

#[test]
fn gate_off_returns_false() {
    let mut fd = build_fd();
    let branchind = build_unimp_idiom(&mut fd, 7);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, false, resolver));
}

#[test]
fn canonical_unimp_idiom_fires() {
    let mut fd = build_fd();
    let branchind = build_unimp_idiom(&mut fd, 7);
    assert!(kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn non_branchind_returns_false() {
    let mut fd = build_fd();
    // Build the idiom but probe the CALLOTHER itself (not a BRANCHIND).
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    set_const_input(&mut fd, callother, 0, 7);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, callother, true, resolver));
}

#[test]
fn callother_with_different_userop_returns_false() {
    let mut fd = build_fd();
    // id 9 resolves to a non-null UserPcodeOp with a different name.
    let branchind = build_unimp_idiom(&mut fd, 9);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn callother_with_null_userop_returns_false() {
    let mut fd = build_fd();
    // id 3 resolves to null (no UserPcodeOp registered).
    let branchind = build_unimp_idiom(&mut fd, 3);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn non_constant_userop_input_returns_false() {
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 1, true);
    // input-0 is a register (non-constant), not a user-op id constant.
    let ram_space = ram(&fd);
    let reg = fd
        .vbank_mut()
        .create(4, Address::new(ram_space, 0x40), unk_type(4));
    fd.obank_mut().get_mut(callother).unwrap().set_input(Some(reg), 0);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn walk_stops_at_instruction_start() {
    // A trap CALLOTHER from a *previous* instruction must NOT match: the walk
    // breaks at the current instruction's start before reaching it.
    let mut fd = build_fd();
    // Previous instruction (0x0FFC): a trap CALLOTHER.
    let prev_trap = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x0FFC, 1, true);
    set_const_input(&mut fd, prev_trap, 0, 7);
    // Current instruction (0x1000): a COPY [instruction-start] then a BRANCHIND.
    let _copy = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1, true);
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, false);
    // The walk visits BRANCHIND, then COPY (instruction-start -> break) and
    // never reaches prev_trap.
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn branchind_is_itself_instruction_start_returns_false() {
    // A lone BRANCHIND that is itself the instruction start (no preceding
    // CALLOTHER in the same instruction) does not match.
    let mut fd = build_fd();
    let branchind = make_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x1000, 1, true);
    set_const_input(&mut fd, branchind, 0, 7); // a register/dest, not a CALLOTHER
    assert!(!kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn walk_reaches_dead_list_head() {
    // The trap CALLOTHER is the very first op in the function (dead-list head)
    // and is the instruction start; the BRANCHIND follows it.  The walk reaches
    // the CALLOTHER (which is also instruction-start) and matches.
    let mut fd = build_fd();
    let branchind = build_unimp_idiom(&mut fd, 7);
    assert!(kuna_is_sparc_struct_ret_trap(&fd, branchind, true, resolver));
}

#[test]
fn option_default_is_off_and_apply_flips() {
    // Shipped default: option sparcstructret off (upstream byte-identical).
    let mut opt = SparcStructRetOption::default();
    assert!(!opt.is_enabled());
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}
