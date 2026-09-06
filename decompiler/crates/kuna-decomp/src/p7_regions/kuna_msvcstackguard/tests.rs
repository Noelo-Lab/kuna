// Tests for `kuna_msvcstackguard` (included into `mod tests` in
// kuna_msvcstackguard.rs).
//
// Covered:
//   - `stack_pointer_offset`: the entry stack pointer answers 0, a PTRSUB /
//     INT_ADD / PTRADD chain answers its signed byte offset, a COPY/CAST is
//     peeled, and an unrelated register answers None.
//   - `sign_extend`: a negative frame delta at 8 and 4 bytes.
//   - `cookie_cancel`: `(K ^ SP) ^ SP` matches and reports the entry-side
//     scramble op; the same shape with the two stack-pointer references at
//     DIFFERENT frame offsets does not (the XORs would not cancel); a scramble
//     of a constant does not; a single XOR does not.
//   - `ActionStripMsvcStackGuard::apply` is inert when the gate is off.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};
use kuna_base::types::int4;

use crate::action::{Action, ActionContext};
use crate::context::{ArchContext, BlockId, TypeOp};
use crate::dtype::{type_metatype, Datatype};

const SP_OFF: u64 = 0x20;
const SP_SIZE: int4 = 8;

fn build_manager() -> AddrSpaceManager {
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
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &regspc, 1, true, false)))
        .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let regspc = Rc::clone(manage.get_space_by_name("register").unwrap());
    let stackspc = Rc::clone(manage.get_stack_space().unwrap());
    let sp_data = VarnodeStorage { space: Some(regspc), offset: SP_OFF, size: SP_SIZE as u32 };
    manage.add_spacebase_pointer(&stackspc, &sp_data, SP_SIZE, true).unwrap();
    let glb = Rc::new(ArchContext::new(manage));
    let code = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let entry = Address::new(code, 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn sp(fd: &Funcdata) -> (Rc<AddrSpace>, uintb, int4) {
    stack_pointer_storage(fd).expect("the fixture has a stack space")
}

fn reg(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap()), off)
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let a = ram_addr(fd, off);
    let op = fd.new_op(inputs, a);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

/// The function's entry stack-pointer Varnode.
fn sp_input(fd: &mut Funcdata) -> VarnodeId {
    let a = reg(fd, SP_OFF);
    let vn = fd.new_varnode(SP_SIZE, &a, None);
    let vn = fd.set_input_varnode(vn).unwrap();
    fd.spacebase();
    vn
}

/// A register Varnode written by a binary op placed in `bl`.
fn binop(
    fd: &mut Funcdata,
    bl: BlockId,
    at: u64,
    opc: OpCode,
    a: VarnodeId,
    b: VarnodeId,
    out_reg: u64,
) -> VarnodeId {
    let op = mk_op(fd, 2, at, opc);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    let addr = reg(fd, out_reg);
    let out = fd.vbank_mut().create(8, addr, unk(8));
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

/// `out = <base> + cst`, the shape stack-pointer normalization leaves behind.
fn frame_ptr(
    fd: &mut Funcdata,
    bl: BlockId,
    at: u64,
    base: VarnodeId,
    cst: u64,
    out_reg: u64,
) -> VarnodeId {
    let c = fd.new_constant(8, cst);
    binop(fd, bl, at, OpCode::CPUI_PTRSUB, base, c, out_reg)
}

/// A global read standing in for `mov rax,[__security_cookie]`.
fn cookie_read(fd: &mut Funcdata, bl: BlockId, at: u64, out_reg: u64) -> VarnodeId {
    let op = mk_op(fd, 1, at, OpCode::CPUI_COPY);
    let a = ram_addr(fd, 0x40_2000);
    let global = fd.vbank_mut().create(8, a, unk(8));
    fd.op_set_input(op, global, 0).unwrap();
    let addr = reg(fd, out_reg);
    let out = fd.vbank_mut().create(8, addr, unk(8));
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

// --- stack_pointer_offset -----------------------------------------------------

#[test]
fn entry_stack_pointer_is_offset_zero() {
    let mut fd = build_fd();
    let spin = sp_input(&mut fd);
    let sp = sp(&fd);
    assert_eq!(stack_pointer_offset(spin, WALK_DEPTH, &fd, &sp), Some(0));
}

#[test]
fn ptrsub_chain_reports_signed_frame_offset() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let spin = sp_input(&mut fd);
    // `sub rsp,0x48` normalized to SP - 0x48.
    let frame = frame_ptr(&mut fd, bl, 0x10, spin, (-0x48i64) as u64, 0x100);
    let sp = sp(&fd);
    assert_eq!(stack_pointer_offset(frame, WALK_DEPTH, &fd, &sp), Some(-0x48));
}

#[test]
fn cast_between_the_chain_is_peeled() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let spin = sp_input(&mut fd);
    let frame = frame_ptr(&mut fd, bl, 0x10, spin, (-0x48i64) as u64, 0x100);
    let op = mk_op(&mut fd, 1, 0x14, OpCode::CPUI_CAST);
    fd.op_set_input(op, frame, 0).unwrap();
    let addr = reg(&fd, 0x108);
    let out = fd.vbank_mut().create(8, addr, unk(8));
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    let sp = sp(&fd);
    assert_eq!(stack_pointer_offset(out, WALK_DEPTH, &fd, &sp), Some(-0x48));
}

#[test]
fn unrelated_register_is_not_a_stack_pointer() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let other = cookie_read(&mut fd, bl, 0x10, 0x100);
    let sp = sp(&fd);
    assert_eq!(stack_pointer_offset(other, WALK_DEPTH, &fd, &sp), None);
}

#[test]
fn sign_extend_reads_negative_deltas() {
    let mut fd = build_fd();
    let c8 = fd.new_constant(8, (-0x48i64) as u64);
    assert_eq!(sign_extend(&fd, c8), Some(-0x48));
    let c4 = fd.new_constant(4, 0xffff_ffb8);
    assert_eq!(sign_extend(&fd, c4), Some(-0x48));
}

// --- cookie_cancel ------------------------------------------------------------

/// Build `(K ^ SP+scramble_off) ^ SP+cancel_off` and return the outer XOR's out.
fn build_cancel(fd: &mut Funcdata, scramble_off: i64, cancel_off: i64) -> (VarnodeId, OpId) {
    let bl = mk_block(fd);
    let spin = sp_input(fd);
    let k = cookie_read(fd, bl, 0x10, 0x100);
    let f1 = frame_ptr(fd, bl, 0x14, spin, scramble_off as u64, 0x108);
    let saved = binop(fd, bl, 0x18, OpCode::CPUI_INT_XOR, k, f1, 0x110);
    let init = fd.vbank().get(saved).unwrap().get_def().unwrap();
    let f2 = frame_ptr(fd, bl, 0x20, spin, cancel_off as u64, 0x118);
    let out = binop(fd, bl, 0x24, OpCode::CPUI_INT_XOR, saved, f2, 0x120);
    (out, init)
}

#[test]
fn cancel_at_the_same_frame_offset_matches() {
    let mut fd = build_fd();
    let (out, init) = build_cancel(&mut fd, -0x48, -0x48);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(out, &fd, &sp), Some(vec![init]));
}

#[test]
fn cancel_at_a_different_frame_offset_declines() {
    let mut fd = build_fd();
    let (out, _) = build_cancel(&mut fd, -0x48, -0x40);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(out, &fd, &sp), None);
}

#[test]
fn a_constant_scrambled_with_the_stack_pointer_is_not_a_cookie() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let spin = sp_input(&mut fd);
    let k = fd.new_constant(8, 0x1234_5678);
    let f1 = frame_ptr(&mut fd, bl, 0x14, spin, (-0x48i64) as u64, 0x108);
    let saved = binop(&mut fd, bl, 0x18, OpCode::CPUI_INT_XOR, k, f1, 0x110);
    let f2 = frame_ptr(&mut fd, bl, 0x20, spin, (-0x48i64) as u64, 0x118);
    let out = binop(&mut fd, bl, 0x24, OpCode::CPUI_INT_XOR, saved, f2, 0x120);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(out, &fd, &sp), None);
}

#[test]
fn one_xor_is_not_a_cancel() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let spin = sp_input(&mut fd);
    let k = cookie_read(&mut fd, bl, 0x10, 0x100);
    let f1 = frame_ptr(&mut fd, bl, 0x14, spin, (-0x48i64) as u64, 0x108);
    let saved = binop(&mut fd, bl, 0x18, OpCode::CPUI_INT_XOR, k, f1, 0x110);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(saved, &fd, &sp), None);
}

/// `phi = MULTIEQUAL(a, b)` placed at the head of `bl`.
fn phi(fd: &mut Funcdata, bl: BlockId, at: u64, a: VarnodeId, b: VarnodeId, out_reg: u64) -> VarnodeId {
    let op = mk_op(fd, 2, at, OpCode::CPUI_MULTIEQUAL);
    fd.op_set_input(op, a, 0).unwrap();
    fd.op_set_input(op, b, 1).unwrap();
    let addr = reg(fd, out_reg);
    let out = fd.vbank_mut().create(8, addr, unk(8));
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_begin(op, bl);
    out
}

/// Two scrambles of the same cookie joining at a phi, then one cancel.
fn build_phi_cancel(fd: &mut Funcdata, second_off: i64) -> (VarnodeId, Vec<OpId>) {
    let bl = mk_block(fd);
    let spin = sp_input(fd);
    let k = cookie_read(fd, bl, 0x10, 0x100);
    let f1 = frame_ptr(fd, bl, 0x14, spin, (-0x48i64) as u64, 0x108);
    let s1 = binop(fd, bl, 0x18, OpCode::CPUI_INT_XOR, k, f1, 0x110);
    let f2 = frame_ptr(fd, bl, 0x1c, spin, second_off as u64, 0x120);
    let s2 = binop(fd, bl, 0x20, OpCode::CPUI_INT_XOR, k, f2, 0x128);
    let i1 = fd.vbank().get(s1).unwrap().get_def().unwrap();
    let i2 = fd.vbank().get(s2).unwrap().get_def().unwrap();
    let joined = phi(fd, bl, 0x24, s1, s2, 0x130);
    let f3 = frame_ptr(fd, bl, 0x28, spin, (-0x48i64) as u64, 0x138);
    let out = binop(fd, bl, 0x2c, OpCode::CPUI_INT_XOR, joined, f3, 0x140);
    (out, vec![i1, i2])
}

#[test]
fn a_join_of_scrambles_at_one_offset_matches() {
    let mut fd = build_fd();
    let (out, inits) = build_phi_cancel(&mut fd, -0x48);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(out, &fd, &sp), Some(inits));
}

#[test]
fn a_join_with_one_scramble_at_another_offset_declines() {
    let mut fd = build_fd();
    let (out, _) = build_phi_cancel(&mut fd, -0x40);
    let sp = sp(&fd);
    assert_eq!(cookie_cancel(out, &fd, &sp), None);
}

// --- the Action gate ----------------------------------------------------------

#[test]
fn action_is_inert_when_the_option_is_off() {
    let mut fd = build_fd();
    let (_out, _init) = build_cancel(&mut fd, -0x48, -0x48);
    let mut act = ActionStripMsvcStackGuard::new(false, "returnsplit");
    let mut ctx = ActionContext::default();
    assert_eq!(act.apply(&mut fd, &mut ctx), 0);
}
