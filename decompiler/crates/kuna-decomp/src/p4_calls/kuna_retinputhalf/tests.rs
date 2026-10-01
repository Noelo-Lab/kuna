//! Tests for the input-parameter carve-out in the uncomputed-return terminal rule.
//!
//! These pin the *predicate* on a hand-built `Funcdata`; the end-to-end witness
//! (a `wide`/`w2` pair whose only difference is copy-vs-arithmetic in the high
//! half) lives in `tests/stages/kuna-retinputhalf.xml`. The negative control —
//! that the callee-saved-restore phantom this carve-out sits inside stays killed —
//! is `kuna-console/tests/verify_return_uncomputed.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::context::ArchContext;

fn build_fd() -> Funcdata {
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
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unwritten(fd: &mut Funcdata, off: u64, sz: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(sz, &Address::new(r, off), None)
}

/// A model-less fixture cannot claim any storage is parameter storage, so the
/// carve-out is inert and the strict terminal rule stands. This is the
/// fail-closed direction: an unfamiliar prototype keeps today's answer.
#[test]
fn without_a_prototype_model_nothing_is_an_input_parameter() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !is_input_parameter(&fd, vn),
        "no model means no parameter storage — the carve-out must not fire",
    );
}

/// A Varnode with a defining op is not the shape this rule speaks about; it is
/// classified by the op that produced it.
#[test]
fn a_written_varnode_is_never_the_input_parameter_shape() {
    use crate::context::TypeOp;
    use kuna_num::opcodes::OpCode;

    let mut fd = build_fd();
    let src = unwritten(&mut fd, 0x2000, 8);
    let r = ram(&fd);
    let op = fd.new_op(1, Address::new(Rc::clone(&r), 0x2100));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    fd.op_set_input(op, src, 0).expect("wire input");
    let out = fd.new_varnode_out(8, &Address::new(r, 0x2100), op).expect("varnode out");
    assert!(
        !is_input_parameter(&fd, out),
        "a written Varnode is classified by its defining op, not by this rule",
    );
}

/// A free Varnode — one heritage never promoted to a function input — is
/// leftover, whatever storage it sits in.
#[test]
fn a_varnode_that_is_not_a_function_input_is_not_a_parameter() {
    let mut fd = build_fd();
    let vn = unwritten(&mut fd, 0x2000, 8);
    assert!(
        !fd.vbank().get(vn).unwrap().is_input(),
        "precondition: the fixture Varnode carries no input flag",
    );
    assert!(!is_input_parameter(&fd, vn));
}

/// The placement test lives in `kuna_returnuncomputed::computes_from`, but its
/// two inputs are this predicate's answer and an address comparison, so pin the
/// address comparison here: a terminal at the return half's OWN storage is the
/// caller's register passing straight through, never a placed argument.
#[test]
fn a_terminal_at_the_return_halfs_own_storage_is_not_a_placement() {
    let mut fd = build_fd();
    let r = ram(&fd);
    let vn = unwritten(&mut fd, 0x2000, 8);
    let same = Address::new(Rc::clone(&r), 0x2000);
    let other = Address::new(r, 0x2008);
    let addr = fd.vbank().get(vn).unwrap().get_addr().clone();
    assert_eq!(addr, same, "the terminal sits at its own address");
    assert_ne!(addr, other, "a moved argument arrives from somewhere else");
}

/// Register-space offsets, ARM SLEIGH: r0 0x20, r1 0x24, r4 0x30, sp 0x54.
const R0: u64 = 0x20;
const R1: u64 = 0x24;
const R4: u64 = 0x30;
const SP: u64 = 0x54;

/// A register space and a stack space whose base is `sp`, so
/// `writes_stack_pointer` has a stack pointer to look for.
fn build_reg_fd() -> Funcdata {
    use kuna_base::space::{SpacebaseSpace, VarnodeStorage};
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let regspc = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    m.insert_space(Rc::clone(&regspc)).unwrap();
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 4, &regspc, 1, true, false))).unwrap();
    let stackspc = Rc::clone(m.get_stack_space().unwrap());
    let sp = VarnodeStorage { space: Some(Rc::clone(&regspc)), offset: SP, size: 4 };
    m.add_spacebase_pointer(&stackspc, &sp, 4, true).unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let entry = Address::new(Rc::clone(glb.manage().get_space_by_name("register").unwrap()), 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn reg(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap()), off)
}

fn input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let vn = fd.new_varnode(4, &reg(fd, off), None);
    fd.set_input_varnode(vn).unwrap()
}

/// `<out> = <opc>(inputs)` made by the instruction at `pc`.
fn def_at(fd: &mut Funcdata, pc: u64, opc: kuna_num::opcodes::OpCode, inputs: &[VarnodeId], out: &Address) -> VarnodeId {
    use crate::context::TypeOp;
    let op = fd.new_op(inputs.len() as int4, reg(fd, pc));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4).unwrap();
    }
    fd.new_varnode_out(4, out, op).unwrap()
}

fn copy_at(fd: &mut Funcdata, pc: u64, src: VarnodeId, out: &Address) -> VarnodeId {
    def_at(fd, pc, kuna_num::opcodes::OpCode::CPUI_COPY, &[src], out)
}

/// `sp = sp + 8` made by the instruction at `pc`.
fn sp_write_at(fd: &mut Funcdata, pc: u64) {
    let cur = fd.new_varnode(4, &reg(fd, SP), None);
    let eight = fd.new_constant(4, 8);
    let out = reg(fd, SP);
    def_at(fd, pc, kuna_num::opcodes::OpCode::CPUI_INT_ADD, &[cur, eight], &out);
}

/// A RETURN outside any block, for the walks that never look for a call.
fn ret_op(fd: &mut Funcdata) -> OpId {
    use crate::context::TypeOp;
    let op = fd.new_op(1, reg(fd, 0x200));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_RETURN, 0, "RETURN".to_string()));
    op
}

/// `mov r4,r1; bl ext; mov r1,r4`: the argument left r1 and came back.
#[test]
fn an_argument_moved_out_to_another_register_and_back_is_moved_back() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let r1 = input(&mut fd, R1);
    let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
    let held = copy_at(&mut fd, 0x104, r1, &r4a);
    let back = copy_at(&mut fd, 0x114, held, &r1a);
    assert!(moved_back(&fd, back, &r1a, 4, ret));
    assert!(!moved_back(&fd, back, &reg(&fd, R0), 4, ret), "the input it reaches is r1's, not r0's");
}

/// The register the RETURN reads untouched, or copied onto itself (ARM's
/// `setISAMode` fixup at `bx lr` is `r0 = r0`), never left.
#[test]
fn an_untouched_register_is_not_moved_back() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let r1 = input(&mut fd, R1);
    let r1a = reg(&fd, R1);
    assert!(!moved_back(&fd, r1, &r1a, 4, ret));
    let same = copy_at(&mut fd, 0x11c, r1, &r1a);
    assert!(!moved_back(&fd, same, &r1a, 4, ret));
}

/// SPARC's `save` and `restore` copy `%o1` to `%i1` and back, and both move the
/// stack pointer: that is the register window, not a move the function made.
#[test]
fn a_move_by_an_instruction_that_writes_the_stack_pointer_is_not_a_move() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let o1 = input(&mut fd, R1);
    let (o1a, i1a) = (reg(&fd, R1), reg(&fd, R4));
    let saved = copy_at(&mut fd, 0x100, o1, &i1a);
    sp_write_at(&mut fd, 0x100);
    let restored = copy_at(&mut fd, 0x10c, saved, &o1a);
    sp_write_at(&mut fd, 0x10c);
    assert!(!moved_back(&fd, restored, &o1a, 4, ret));
}

/// Xtensa's `call8` swaps the argument registers out and back with copies the
/// compiler spec marks incidental.
#[test]
fn an_incidental_copy_is_not_a_move() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let a3 = input(&mut fd, R1);
    let (a3a, t3a) = (reg(&fd, R1), reg(&fd, R4));
    let swapped = copy_at(&mut fd, 0x103, a3, &t3a);
    let restored = copy_at(&mut fd, 0x103, swapped, &a3a);
    for vn in [swapped, restored] {
        let op = fd.vbank().get(vn).unwrap().get_def().unwrap();
        fd.obank_mut().mark_incidental_copy(op, op);
    }
    assert!(!moved_back(&fd, restored, &a3a, 4, ret));
}

/// A move staged through a temporary still passes through `r4`.
#[test]
fn a_move_staged_through_a_temporary_counts() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let r1 = input(&mut fd, R1);
    let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
    let tmp = Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("unique").unwrap()), 0x100);
    let held = copy_at(&mut fd, 0x104, r1, &r4a);
    let staged = copy_at(&mut fd, 0x114, held, &tmp);
    let back = copy_at(&mut fd, 0x114, staged, &r1a);
    assert!(moved_back(&fd, back, &r1a, 4, ret));
}

/// PowerPC's `mr` lifts to `or rA,rS,rS`, and MIPS, SPARC and RISC-V moves to
/// an `or` or `add` with zero.
#[test]
fn a_self_or_and_an_add_of_zero_are_moves() {
    use kuna_num::opcodes::OpCode;
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let r1 = input(&mut fd, R1);
    let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
    let held = def_at(&mut fd, 0x104, OpCode::CPUI_INT_OR, &[r1, r1], &r4a);
    let zero = fd.new_constant(4, 0);
    let back = def_at(&mut fd, 0x114, OpCode::CPUI_INT_ADD, &[held, zero], &r1a);
    assert!(moved_back(&fd, back, &r1a, 4, ret));
    let one = fd.new_constant(4, 1);
    let bumped = def_at(&mut fd, 0x118, OpCode::CPUI_INT_ADD, &[held, one], &r1a);
    assert!(!moved_back(&fd, bumped, &r1a, 4, ret), "adding one computes a value");
}

/// The record names a register, at any width but its own nothing.
#[test]
fn the_record_is_per_register() {
    let mut fd = build_reg_fd();
    let ret = ret_op(&mut fd);
    let (r0a, r1a) = (reg(&fd, R0), reg(&fd, R1));
    fd.kuna_set_moved_back_returns(vec![(r1a.clone(), 4)]);
    assert!(is_moved_back(&fd, &r1a, 4));
    assert!(!is_moved_back(&fd, &r0a, 4));
    assert!(!is_moved_back(&fd, &r1a, 8));
}

fn block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().new_block_basic(root)
}

/// An op with no inputs at the end of `bl`.
fn op_in(fd: &mut Funcdata, bl: BlockId, pc: u64, opc: OpCode) -> OpId {
    use crate::context::TypeOp;
    let op = fd.new_op(0, reg(fd, pc));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

/// Put the op that defines `vn` at the end of `bl`.
fn place(fd: &mut Funcdata, vn: VarnodeId, bl: BlockId) {
    let op = fd.vbank().get(vn).unwrap().get_def().unwrap();
    fd.op_insert_end(op, bl);
}

/// `int f(int a, int b) { ext(); r1 = b; svc 0; return r0 + 1; }`: the move
/// into r1 feeds the system call, whose p-code reads no register.
#[test]
fn a_move_back_followed_by_a_callother_is_not_moved_back() {
    let mut fd = build_reg_fd();
    let r1 = input(&mut fd, R1);
    let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
    let b0 = block(&mut fd);
    let held = copy_at(&mut fd, 0x104, r1, &r4a);
    place(&mut fd, held, b0);
    op_in(&mut fd, b0, 0x108, OpCode::CPUI_CALLOTHER);
    let back = copy_at(&mut fd, 0x10c, held, &r1a);
    place(&mut fd, back, b0);
    op_in(&mut fd, b0, 0x110, OpCode::CPUI_CALLOTHER);
    let ret = op_in(&mut fd, b0, 0x114, OpCode::CPUI_RETURN);
    assert!(!moved_back(&fd, back, &r1a, 4, ret), "svc after the move back");

    let mut fd = build_reg_fd();
    let r1 = input(&mut fd, R1);
    let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
    let b0 = block(&mut fd);
    let held = copy_at(&mut fd, 0x104, r1, &r4a);
    place(&mut fd, held, b0);
    op_in(&mut fd, b0, 0x108, OpCode::CPUI_CALLOTHER);
    let back = copy_at(&mut fd, 0x10c, held, &r1a);
    place(&mut fd, back, b0);
    let ret = op_in(&mut fd, b0, 0x110, OpCode::CPUI_RETURN);
    assert!(moved_back(&fd, back, &r1a, 4, ret), "a CALLOTHER before the move back is not its reader");
}

/// A call on one path from the move back to the RETURN is enough; one on a
/// path that never reaches the RETURN is not.
#[test]
fn only_a_callother_on_a_path_to_the_return_stops_the_move() {
    for reaches in [true, false] {
        let mut fd = build_reg_fd();
        let r1 = input(&mut fd, R1);
        let (r1a, r4a) = (reg(&fd, R1), reg(&fd, R4));
        let (top, side, exit) = (block(&mut fd), block(&mut fd), block(&mut fd));
        fd.bblocks_mut().add_edge(top, side);
        fd.bblocks_mut().add_edge(top, exit);
        if reaches {
            fd.bblocks_mut().add_edge(side, exit);
        }
        let held = copy_at(&mut fd, 0x104, r1, &r4a);
        place(&mut fd, held, top);
        let back = copy_at(&mut fd, 0x10c, held, &r1a);
        place(&mut fd, back, top);
        op_in(&mut fd, side, 0x120, OpCode::CPUI_CALLOTHER);
        let ret = op_in(&mut fd, exit, 0x130, OpCode::CPUI_RETURN);
        assert_eq!(moved_back(&fd, back, &r1a, 4, ret), !reaches, "reaches = {reaches}");
    }
}
