//! Logic-level tests for `kuna_is_v850_indirect_jmp` (GH-8817), exercising the
//! transformation decision on hand-built IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::context::{ArchContext, OpId, TypeOp};

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
    let glb = Rc::new(ArchContext::new(manage));
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

/// Build an op of `opc` with input-0 set to a varnode in `space` at `off`
/// (size 4).  Returns the op id.
fn build_op_with_input(
    fd: &mut Funcdata,
    opc: OpCode,
    space: Rc<AddrSpace>,
    off: u64,
) -> OpId {
    let pc = Address::new(proc_space(fd), 0x1000);
    let op = fd.obank_mut().create_at(1, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    let vn = fd
        .vbank_mut()
        .create(4, Address::new(space, off), unk_type(4));
    // Read-only test IR: wire the input directly on the op (the hook never
    // inspects descend), mirroring op.rs's raw input-setter usage in tests.
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
    op
}

#[test]
fn gate_off_returns_false() {
    let mut fd = build_fd();
    let ram = proc_space(&fd);
    let op = build_op_with_input(&mut fd, OpCode::CPUI_CALLIND, ram, 0x10);
    // gate=false: upstream byte-identical, never fires.
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, false, Some("r10")));
}

#[test]
fn callind_register_not_pc_fires() {
    let mut fd = build_fd();
    let ram = proc_space(&fd);
    let op = build_op_with_input(&mut fd, OpCode::CPUI_CALLIND, ram, 0x10);
    // The canonical V850 `jmp [r10]` shape.
    assert!(kuna_is_v850_indirect_jmp(&fd, op, true, Some("r10")));
}

#[test]
fn non_callind_returns_false() {
    let mut fd = build_fd();
    let ram = proc_space(&fd);
    // A BRANCHIND (or any non-CALLIND) is left alone.
    let op = build_op_with_input(&mut fd, OpCode::CPUI_BRANCHIND, ram, 0x10);
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("r10")));
}

#[test]
fn callind_with_no_input_returns_false() {
    let mut fd = build_fd();
    let pc = Address::new(proc_space(&fd), 0x1000);
    let op = fd.obank_mut().create_at(1, pc);
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(OpCode::CPUI_CALLIND, 0, "callind"));
    // input-0 left null
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("r10")));
}

#[test]
fn input_not_processor_space_returns_false() {
    let mut fd = build_fd();
    // input-0 in the constant space -> not a hardware register.
    let cspace = Rc::clone(fd.get_arch().manage().get_space_by_name("const").unwrap());
    let op = build_op_with_input(&mut fd, OpCode::CPUI_CALLIND, cspace, 0x10);
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("r10")));
}

#[test]
fn unnamed_register_returns_false() {
    let mut fd = build_fd();
    let ram = proc_space(&fd);
    let op = build_op_with_input(&mut fd, OpCode::CPUI_CALLIND, ram, 0x10);
    // empty name (C++ regname.empty()) -> a raw memory load, not a named reg.
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, None));
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("")));
}

#[test]
fn program_counter_excluded() {
    let mut fd = build_fd();
    let ram = proc_space(&fd);
    let op = build_op_with_input(&mut fd, OpCode::CPUI_CALLIND, ram, 0x10);
    // The other V850 CALLINDs are `call [PC]`; both spellings are excluded.
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("PC")));
    assert!(!kuna_is_v850_indirect_jmp(&fd, op, true, Some("pc")));
}

#[test]
fn option_default_is_off_and_apply_flips() {
    // Shipped default: option v850indirectbranch off (upstream byte-identical).
    let mut opt = V850IndirectBranchOption::default();
    assert!(!opt.is_enabled());
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}
