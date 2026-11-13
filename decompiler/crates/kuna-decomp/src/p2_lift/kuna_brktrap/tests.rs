//! Logic-level tests for [`brk_immediate`] and [`is_compiler_trap_immediate`],
//! exercising the positional pre-SSA walk on hand-built dead-list IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, OpId, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;

use super::*;

const BRK_USEROP: uintb = 0x54;
const OTHER_USEROP: uintb = 0x55;
const PC: u64 = 0x0;
const X0: u64 = 0x4000;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    for (index, name) in [(2, "ram"), (3, "register")] {
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            false,
            8,
            1,
            index,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
    }
    m
}

fn build_fd() -> Funcdata {
    let glb = Rc::new(ArchContext::new(build_manager()));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    Funcdata::new("func", "func", glb, Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
}

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

fn unk_type(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn make_op(fd: &mut Funcdata, opc: OpCode, pc_off: u64, ninputs: int4, start: bool) -> OpId {
    let pc = Address::new(space(fd, "ram"), pc_off);
    let op = fd.obank_mut().create_at(ninputs, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    if start {
        fd.obank_mut().get_mut(op).unwrap().set_flag(pcodeop_flags::startmark);
    }
    op
}

fn constant(fd: &mut Funcdata, value: uintb, size: int4) -> VarnodeId {
    let cspace = space(fd, "const");
    fd.vbank_mut().create(size, Address::new(cspace, value), unk_type(size))
}

fn register(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let rspace = space(fd, "register");
    fd.vbank_mut().create(size, Address::new(rspace, off), unk_type(size))
}

fn set_input(fd: &mut Funcdata, op: OpId, slot: int4, vn: VarnodeId) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn set_output(fd: &mut Funcdata, op: OpId, vn: VarnodeId) {
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
}

/// `pc = CALLOTHER(#userop:4, #imm:imm_size, #inst_next:8)` at `at`, as the
/// first op of its instruction.
fn breakpoint(fd: &mut Funcdata, at: u64, userop: uintb, imm: uintb, imm_size: int4) -> OpId {
    let callother = make_op(fd, OpCode::CPUI_CALLOTHER, at, 3, true);
    let id = constant(fd, userop, 4);
    let imm = constant(fd, imm, imm_size);
    let next = constant(fd, at + 4, 8);
    set_input(fd, callother, 0, id);
    set_input(fd, callother, 1, imm);
    set_input(fd, callother, 2, next);
    let out = register(fd, PC, 8);
    set_output(fd, callother, out);
    callother
}

/// `goto [reg]` at `at`, following the ops already in the instruction.
fn branchind(fd: &mut Funcdata, at: u64, reg: u64) -> OpId {
    let op = make_op(fd, OpCode::CPUI_BRANCHIND, at, 1, false);
    let target = register(fd, reg, 8);
    set_input(fd, op, 0, target);
    op
}

/// The lifted `brk #imm` at 0x1000.
fn brk(fd: &mut Funcdata, imm: uintb) -> OpId {
    breakpoint(fd, 0x1000, BRK_USEROP, imm, 2);
    branchind(fd, 0x1000, PC)
}

#[test]
fn compiler_trap_immediates() {
    for imm in [0x1, 0x3e8, 0x5500, 0x5501, 0x55ff, 0xc470, 0xc473, 0xf003] {
        assert!(is_compiler_trap_immediate(imm), "{imm:#x}");
    }
    for imm in [0x0, 0x2, 0x4, 0x5, 0x400, 0x800, 0x900, 0x3e7, 0x3e9, 0x54ff, 0x5600, 0x8000, 0xc46f, 0xc474, 0xf000, 0xf001, 0xf002, 0xf004] {
        assert!(!is_compiler_trap_immediate(imm), "{imm:#x}");
    }
}

#[test]
fn lifted_brk_reports_its_immediate() {
    for imm in [0x1, 0xf000, 0x0] {
        let mut fd = build_fd();
        let op = brk(&mut fd, imm);
        assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), Some(imm));
    }
}

#[test]
fn only_trap_immediates_are_traps() {
    let mut fd = build_fd();
    let op = brk(&mut fd, 0x1);
    assert!(is_brk_trap_branchind(&fd, op, BRK_USEROP as u32));
    let mut fd = build_fd();
    let op = brk(&mut fd, 0xf000);
    assert!(!is_brk_trap_branchind(&fd, op, BRK_USEROP as u32));
}

#[test]
fn another_userop_is_not_a_brk() {
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x1000, OTHER_USEROP, 0x1, 2);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x1000, 0x99, 0x1, 2);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
}

#[test]
fn a_wide_immediate_is_not_a_brk() {
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x1000, BRK_USEROP, 0x1, 4);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
}

#[test]
fn a_jump_through_other_storage_is_not_a_brk() {
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x1000, BRK_USEROP, 0x1, 2);
    let op = branchind(&mut fd, 0x1000, X0);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
}

#[test]
fn a_write_between_the_userop_and_the_jump_declines() {
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x1000, BRK_USEROP, 0x1, 2);
    let copy = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1, false);
    let src = register(&mut fd, X0, 8);
    set_input(&mut fd, copy, 0, src);
    let out = register(&mut fd, PC, 8);
    set_output(&mut fd, copy, out);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
}

#[test]
fn the_walk_stays_in_the_instruction() {
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x0ffc, BRK_USEROP, 0x1, 2);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
    let mut fd = build_fd();
    breakpoint(&mut fd, 0x0ffc, BRK_USEROP, 0x1, 2);
    make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1, true);
    let op = branchind(&mut fd, 0x1000, PC);
    assert_eq!(brk_immediate(&fd, op, BRK_USEROP as u32), None);
}

#[test]
fn only_a_branchind_is_matched() {
    let mut fd = build_fd();
    let callother = breakpoint(&mut fd, 0x1000, BRK_USEROP, 0x1, 2);
    assert_eq!(brk_immediate(&fd, callother, BRK_USEROP as u32), None);
}
