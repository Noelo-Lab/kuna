//! Logic-level tests for [`is_fastfail_callind`] and [`archid_is_windows`],
//! exercising the positional pre-SSA walk on hand-built dead-list IR.

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

/// A unique-space Varnode at `off`, size 8 — the `intloc` the `swi` writes and
/// the `CALLIND` reads (two distinct free Varnodes over the same storage, which
/// is the pre-SSA shape).
fn unique_vn(fd: &mut Funcdata, off: u64) -> crate::context::VarnodeId {
    let uspace = space(fd, "unique");
    fd.vbank_mut().create(8, Address::new(uspace, off), unk_type(8))
}

/// The canonical `INT imm8` idiom in one instruction:
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
    assert_eq!(ELEM_FASTFAILNORETURN.get_id(), 4144);
    assert_eq!(ELEM_FASTFAILNORETURN.get_name(), "fastfailnoreturn");
}

#[test]
fn canonical_int29_idiom_fires() {
    let mut fd = build_fd();
    let callind = build_int_idiom(&mut fd, 3, 0x29);
    assert!(is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn other_vectors_do_not_fire() {
    // int 0x80 is linuxsyscall's; int3 and int1 return by their own semantics.
    for vector in [0x80, 0x03, 0x01, 0x2d, 0x00] {
        let mut fd = build_fd();
        let callind = build_int_idiom(&mut fd, 3, vector);
        assert!(!is_fastfail_callind(&fd, callind, resolver), "vector {vector:#x}");
    }
}

#[test]
fn other_userop_does_not_fire() {
    let mut fd = build_fd();
    let callind = build_int_idiom(&mut fd, 5, 0x29); // resolves, but not `swi`
    assert!(!is_fastfail_callind(&fd, callind, resolver));
    let mut fd = build_fd();
    let callind = build_int_idiom(&mut fd, 9, 0x29); // null UserPcodeOp *
    assert!(!is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn wide_vector_constant_does_not_fire() {
    // The lifted vector is a 1-byte constant; a 4-byte 0x29 is some other shape.
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(&mut fd, callother, 0, 3, 4);
    set_const_input(&mut fd, callother, 1, 0x29, 4);
    let out = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callother).unwrap().set_output(Some(out));
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, false);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn callind_reading_other_storage_does_not_fire() {
    // An ordinary indirect call that merely follows an interrupt in the same
    // instruction must not be truncated: it does not read what `swi` wrote.
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(&mut fd, callother, 0, 3, 4);
    set_const_input(&mut fd, callother, 1, 0x29, 1);
    let out = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callother).unwrap().set_output(Some(out));
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, false);
    let elsewhere = unique_vn(&mut fd, 0x1234);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(elsewhere), 0);
    assert!(!is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn non_callind_does_not_fire() {
    let mut fd = build_fd();
    let callother = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x1000, 2, true);
    set_const_input(&mut fd, callother, 0, 3, 4);
    set_const_input(&mut fd, callother, 1, 0x29, 1);
    assert!(!is_fastfail_callind(&fd, callother, resolver));
}

#[test]
fn walk_stops_at_instruction_start() {
    // A `swi(0x29)` from the *previous* instruction must not truncate this one's
    // indirect call.
    let mut fd = build_fd();
    let prev = make_op(&mut fd, OpCode::CPUI_CALLOTHER, 0x0ffe, 2, true);
    set_const_input(&mut fd, prev, 0, 3, 4);
    set_const_input(&mut fd, prev, 1, 0x29, 1);
    let out = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(prev).unwrap().set_output(Some(out));

    let _copy = make_op(&mut fd, OpCode::CPUI_COPY, 0x1000, 1, true);
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, false);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn lone_callind_does_not_fire() {
    let mut fd = build_fd();
    let callind = make_op(&mut fd, OpCode::CPUI_CALLIND, 0x1000, 1, true);
    let target = unique_vn(&mut fd, 0x9bd00);
    fd.obank_mut().get_mut(callind).unwrap().set_input(Some(target), 0);
    assert!(!is_fastfail_callind(&fd, callind, resolver));
}

#[test]
fn windows_gate_accepts_only_the_windows_cspecs() {
    assert!(archid_is_windows("x86:LE:64:default:windows"));
    assert!(archid_is_windows("x86:LE:32:default:windows"));
    assert!(archid_is_windows("x86:LE:64:default:clangwindows"));
    assert!(!archid_is_windows("x86:LE:64:default:gcc"));
    assert!(!archid_is_windows("x86:LE:32:default:borlandcpp"));
    assert!(!archid_is_windows("ARM:LE:32:v8:default"));
    assert!(!archid_is_windows(""));
}
