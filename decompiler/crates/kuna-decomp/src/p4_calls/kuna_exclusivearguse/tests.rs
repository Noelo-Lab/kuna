//! Unit tests for `exclusivearguse`'s predicate: which accesses a call's input
//! trial may ignore, and the CFG shapes the reachability test must decline.  The
//! end-to-end behaviour is `tests/stages/gh dec-exclusivearguse.xml` and
//! `tests/cli/bytecode-append-call-loses.json`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;

use kuna_base::error::KunaResult;

use crate::context::{ArchContext, TypeOp};
use crate::dtype::{type_metatype, Datatype};
use crate::varnode::{DefOpInfo, VarnodeBank};

use super::*;

// --- fixtures ---------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
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
    m
}

fn build_fd(exclusive_arg_use: bool) -> Funcdata {
    let mut ctx = ArchContext::new(build_manager());
    ctx.exclusive_arg_use = exclusive_arg_use;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, bl: BlockId, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert(op, bl, None);
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
}

fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// A Varnode defined by an `INT_ADD` in `bl` — the shape the rule demands, a
/// value produced in the block that branches.
fn mk_defined_ptr(fd: &mut Funcdata, bl: BlockId, off: u64) -> VarnodeId {
    let op = mk_op(fd, bl, 2, off, OpCode::CPUI_INT_ADD);
    let base = mk_vn(fd, 0x80, 8);
    let disp = fd.new_constant(8, 8);
    wire(fd, base, op, 0);
    wire(fd, disp, op, 1);
    let r = ram(fd);
    let out = fd.new_varnode(8, &Address::new(r, 0x40), Some(unk(8)));
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    let out = fd.vbank_mut().set_def(out, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(out));
    out
}

/// `STORE(spaceid, ptr, value)` at `off`.
fn store(fd: &mut Funcdata, bl: BlockId, off: u64, ptr: VarnodeId, value: VarnodeId) -> OpId {
    let op = mk_op(fd, bl, 3, off, OpCode::CPUI_STORE);
    let spc = fd.new_constant(4, 0);
    wire(fd, spc, op, 0);
    wire(fd, ptr, op, 1);
    wire(fd, value, op, 2);
    op
}

/// `out = LOAD(spaceid, ptr)` at `off`.
fn load(fd: &mut Funcdata, bl: BlockId, off: u64, ptr: VarnodeId) -> OpId {
    let op = mk_op(fd, bl, 2, off, OpCode::CPUI_LOAD);
    let spc = fd.new_constant(4, 0);
    wire(fd, spc, op, 0);
    wire(fd, ptr, op, 1);
    op
}

/// `CALL(target, arg)` at `off`.
fn call(fd: &mut Funcdata, bl: BlockId, off: u64, arg: VarnodeId) -> OpId {
    let op = mk_op(fd, bl, 2, off, OpCode::CPUI_CALL);
    let r = ram(fd);
    let target = fd.new_code_ref(&Address::new(r, 0x8000));
    wire(fd, target, op, 0);
    wire(fd, arg, op, 1);
    op
}

/// The witness CFG: `head` branches to `fast` and `slow`, both joining at
/// `join`.  Returns `(head, fast, slow, join)`.
fn diamond(fd: &mut Funcdata) -> (BlockId, BlockId, BlockId, BlockId) {
    let head = mk_block(fd);
    let fast = mk_block(fd);
    let slow = mk_block(fd);
    let join = mk_block(fd);
    fd.bblocks_mut().add_edge(head, fast);
    fd.bblocks_mut().add_edge(head, slow);
    fd.bblocks_mut().add_edge(fast, join);
    fd.bblocks_mut().add_edge(slow, join);
    (head, fast, slow, join)
}

// --- the witness ------------------------------------------------------------

#[test]
fn store_through_the_pointer_on_the_other_arm_does_not_compete() {
    let mut fd = build_fd(true);
    let (head, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let datum = mk_vn(&mut fd, 0x48, 1);
    let st = store(&mut fd, fast, 0x2000, ptr, datum);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(access_cannot_reach_call(&fd, cl, st, ptr));
}

#[test]
fn load_through_the_pointer_on_the_other_arm_does_not_compete() {
    let mut fd = build_fd(true);
    let (head, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn option_off_keeps_the_upstream_rejection() {
    let mut fd = build_fd(false);
    let (head, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

// --- what the rule declines -------------------------------------------------

#[test]
fn a_long_lived_scratch_copy_still_competes() {
    // The measured false positive: the pointer is produced in an earlier block
    // and merely still live at the branch, not set up for it.  phantomgate.exe's
    // `random_device` constructor opens `mov r8,rcx` and dereferences r8 on the
    // arms that succeed; without this test every throw call gained an argument.
    let mut fd = build_fd(true);
    let entry = mk_block(&mut fd);
    let (head, fast, slow, _) = diamond(&mut fd);
    fd.bblocks_mut().add_edge(entry, head);
    let ptr = mk_defined_ptr(&mut fd, entry, 0x1e00);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn a_function_input_still_competes() {
    // No defining op at all, so nothing says the value was set up for the branch.
    let mut fd = build_fd(true);
    let (_, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_vn(&mut fd, 0x40, 8);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn an_access_in_a_block_that_reaches_the_call_still_competes() {
    // The `calleearity` family's shape: the competing use is in a block that
    // dominates the call, so the two plainly co-execute.
    let mut fd = build_fd(true);
    let head = mk_block(&mut fd);
    let tail = mk_block(&mut fd);
    fd.bblocks_mut().add_edge(head, tail);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, head, 0x2000, ptr);
    let cl = call(&mut fd, tail, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn two_arms_inside_a_loop_still_compete() {
    // The back edge makes each arm reachable from the other, so an iteration
    // that takes the call can be followed by one that takes the access.
    let mut fd = build_fd(true);
    let (head, fast, slow, join) = diamond(&mut fd);
    fd.bblocks_mut().add_edge(join, head);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn an_access_in_the_calls_own_block_still_competes() {
    let mut fd = build_fd(true);
    let (head, _, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, slow, 0x2000, ptr);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, ld, ptr));
}

#[test]
fn a_store_of_the_value_is_spillargtrials_and_still_competes() {
    // The trial Varnode as the stored DATUM, not the address: a caller-save
    // spill, which `spillargtrial` owns.  This rule never admits it.
    let mut fd = build_fd(true);
    let (head, fast, slow, _) = diamond(&mut fd);
    let value = mk_defined_ptr(&mut fd, head, 0x1f00);
    let slot = mk_vn(&mut fd, 0x48, 8);
    let st = store(&mut fd, fast, 0x2000, slot, value);
    let cl = call(&mut fd, slow, 0x2100, value);
    assert!(!access_cannot_reach_call(&fd, cl, st, value));
}

#[test]
fn a_return_match_is_not_this_rules_business() {
    // Output trials keep the upstream treatment; this is an input-trial rule.
    let mut fd = build_fd(true);
    let (head, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let ld = load(&mut fd, fast, 0x2000, ptr);
    let ret = mk_op(&mut fd, slow, 2, 0x2100, OpCode::CPUI_RETURN);
    wire(&mut fd, ptr, ret, 1);
    assert!(!access_cannot_reach_call(&fd, ret, ld, ptr));
}

#[test]
fn a_branch_on_the_other_arm_is_not_this_rules_business() {
    // Only LOAD/STORE relax; a CBRANCH reading the value keeps rejecting.
    let mut fd = build_fd(true);
    let (head, fast, slow, _) = diamond(&mut fd);
    let ptr = mk_defined_ptr(&mut fd, head, 0x1f00);
    let br = mk_op(&mut fd, fast, 2, 0x2000, OpCode::CPUI_CBRANCH);
    wire(&mut fd, ptr, br, 1);
    let cl = call(&mut fd, slow, 0x2100, ptr);
    assert!(!access_cannot_reach_call(&fd, cl, br, ptr));
}
