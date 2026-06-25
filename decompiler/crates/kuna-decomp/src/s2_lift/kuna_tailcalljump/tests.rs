//! Logic-level tests for `kuna_is_tail_call_branch`, exercising the tail-call
//! recognition decision on hand-built IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{Architecture, OpId, TypeOp};

use super::*;

/// const(0), unique(1), ram(2, IPTR_PROCESSOR).
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
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn unk_type(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn proc_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// Build an op of `opc` with input-0 set to a code-ref-like varnode in the
/// processor space at `off`.  Returns the op id.
fn build_branch_op(fd: &mut Funcdata, opc: OpCode, off: u64) -> OpId {
    let pc = Address::new(proc_space(fd), 0x1000);
    let op = fd.obank_mut().create_at(1, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    let ram = proc_space(fd);
    let vn = fd
        .vbank_mut()
        .create(8, Address::new(ram, off), unk_type(8));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
    op
}

#[test]
fn gate_off_returns_false() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    // gate=false: default-pipeline byte-identical, never fires.
    assert!(!kuna_is_tail_call_branch(&fd, op, false, true, false));
}

#[test]
fn direct_branch_to_known_function_fires() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    // The canonical `jmp setlocale@plt` tail-jump shape: a direct BRANCH whose
    // target is another known function's entry.
    assert!(kuna_is_tail_call_branch(&fd, op, true, true, false));
}

#[test]
fn target_not_a_known_function_returns_false() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x1100);
    // An ordinary intraprocedural jump targets a mid-function address (no
    // function entry there) -> not a tail call.
    assert!(!kuna_is_tail_call_branch(&fd, op, true, false, false));
}

#[test]
fn self_tail_recursion_excluded() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x1000);
    // dest == the current function's own entry: left as an ordinary back-edge.
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, true));
}

#[test]
fn indirect_branch_returns_false() {
    let mut fd = build_fd();
    // A BRANCHIND (indirect jump) is not a direct tail-call jmp.
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x18d0);
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, false));
}

#[test]
fn call_returns_false() {
    let mut fd = build_fd();
    // A real CALL is already a call, not a jump to recover.
    let op = build_branch_op(&mut fd, OpCode::CPUI_CALL, 0x18d0);
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, false));
}

#[test]
fn option_default_is_off_and_apply_flips() {
    // Shipped default: option tailcalljump off (kept opt-in; default-on regresses
    // 2 datatests, Long double #1/#2).
    let mut opt = TailCallJumpOption::default();
    assert!(!opt.is_enabled());
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}
