//! Logic-level tests for [`is_int3_callind`], the option parse and the warning
//! text, on hand-built dead-list IR (the shape walk is
//! `kuna_fastfailnoreturn`'s, read for vector 3).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{ArchContext, OpId, TypeOp};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;

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

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
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

fn set_const_input(fd: &mut Funcdata, op: OpId, slot: int4, value: uintb, size: int4) {
    let cspace = space(fd, "const");
    let vn = fd.vbank_mut().create(size, Address::new(cspace, value), unk_type(size));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn unique_vn(fd: &mut Funcdata, off: u64) -> crate::context::VarnodeId {
    let uspace = space(fd, "unique");
    fd.vbank_mut().create(8, Address::new(uspace, off), unk_type(8))
}

/// The `INT3` idiom in one instruction:
///   CALLOTHER(swi_id, #vector:1) -> intloc   [instruction-start]
///   CALLIND [intloc]
/// Returns the CALLIND op id.
fn build_int_idiom(fd: &mut Funcdata, swi_id: uintb, vector: uintb) -> OpId {
    let callother = make_op(fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(fd, callother, 0, swi_id, 4);
    set_const_input(fd, callother, 1, vector, 1);
    let out = unique_vn(fd, 0x9bd00);
    fd.obank_mut().get_mut(callother).unwrap().set_output(Some(out));

    let callind = make_op(fd, OpCode::CPUI_CALLIND, 0x1000, 1, false);
    let target = unique_vn(fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    callind
}

/// userop resolver: 3 -> "swi", 5 -> another registered user-op, else null.
fn resolver(id: u32) -> Option<String> {
    match id {
        3 => Some("swi".to_string()),
        5 => Some("lock".to_string()),
        _ => None,
    }
}

#[test]
fn elem_id_in_kuna_range() {
    assert_eq!(ELEM_INT3PAD.get_id(), 4150);
    assert_eq!(ELEM_INT3PAD.get_name(), "int3pad");
}

#[test]
fn canonical_int3_idiom_fires() {
    let mut fd = build_fd();
    let callind = build_int_idiom(&mut fd, 3, INT3_VECTOR);
    assert!(is_int3_callind(&fd, callind, resolver));
}

#[test]
fn other_vectors_do_not_fire() {
    // int 0x29 is fastfailnoreturn's, int 0x80 is linuxsyscall's.
    for vector in [0x29, 0x80, 0x1, 0x4] {
        let mut fd = build_fd();
        let callind = build_int_idiom(&mut fd, 3, vector);
        assert!(!is_int3_callind(&fd, callind, resolver), "vector {vector:#x} fired");
    }
}

#[test]
fn a_wider_vector_constant_does_not_fire() {
    // The vector must be the ONE-byte constant the `INT` lowering writes.
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(&mut fd, callother, 0, 3, 4);
    set_const_input(&mut fd, callother, 1, INT3_VECTOR, 4);
    let out = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callother).unwrap().set_output(Some(out));
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, false);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_int3_callind(&fd, callind, resolver));
}

#[test]
fn another_userop_does_not_fire() {
    let mut fd = build_fd();
    let callind = build_int_idiom(&mut fd, 5, INT3_VECTOR);
    assert!(!is_int3_callind(&fd, callind, resolver));
}

#[test]
fn a_callother_in_another_instruction_does_not_fire() {
    // The positional walk stops at the CALLIND's own instruction start.
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(&mut fd, callother, 0, 3, 4);
    set_const_input(&mut fd, callother, 1, INT3_VECTOR, 1);
    let out = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callother).unwrap().set_output(Some(out));

    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1004, 1, true);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_int3_callind(&fd, callind, resolver));
}

#[test]
fn a_plain_indirect_call_does_not_fire() {
    let mut fd = build_fd();
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, true);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_int3_callind(&fd, callind, resolver));
}

#[test]
fn mode_parse_and_predicates() {
    let opt = OptionInt3Pad;
    assert_eq!(opt.apply("off").unwrap().0, Int3PadMode::Off);
    assert_eq!(opt.apply("warn").unwrap().0, Int3PadMode::Warn);
    assert_eq!(opt.apply("halt").unwrap().0, Int3PadMode::Halt);
    assert!(opt.apply("on").is_err());
    assert_eq!(Int3PadMode::default(), Int3PadMode::Warn);

    assert!(!Int3PadMode::Off.warns() && !Int3PadMode::Off.halts());
    assert!(Int3PadMode::Warn.warns() && !Int3PadMode::Warn.halts());
    assert!(Int3PadMode::Halt.warns() && Int3PadMode::Halt.halts());
}

#[test]
fn warning_text_carries_the_run_and_no_earlier_digit() {
    // The P9 slug table reads the first integer AFTER the `int3pad:` tag as the
    // ` xN` suffix, so nothing between the tag and the count may be a digit.
    let long = warning_text(4);
    assert_eq!(crate::printc::warning_slug(&format!("WARNING: {long}")), "int3-pad x4");
    let short = warning_text(1);
    assert_eq!(crate::printc::warning_slug(&format!("WARNING: {short}")), "int3-pad");
    assert_eq!(crate::printc::warning_slug(&format!("WARNING: {}", warning_text(0))), "int3-pad");
}
