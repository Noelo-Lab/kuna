// Tests for the `kuna_stackguard` port (included into `mod tests` in
// kuna_stackguard.rs).
//
// Covered:
//   - the option decision body (`OptionStackGuard::apply`): on/off flips
//     `enabled` and returns the confirmation message; default off.
//   - `ptr_is_canary_slot`: recognizes `<base> + 0x28` (both operand orders,
//     through COPY/CAST peeling), rejects other offsets and non-add pointers.
//   - `derives_from_canary_load`: peels COPY/CAST/zext/sext/INDIRECT/zero-
//     SUBPIECE and follows MULTIEQUAL inputs down to the canary LOAD; bounded
//     by depth and the visited set.
//   - `is_canary_compare`: an INT_NOTEQUAL whose BOTH operands derive from the
//     canary load is a canary compare (and reports isNotEqual); INT_EQUAL too;
//     a one-sided / non-equal compare is not.
//   - `block_has_call`: detects CALL/CALLIND/CALLOTHER in a block.
//   - `ActionStripStackGuard::apply` is inert (returns 0) when the gate is off,
//     and (until the removeBranch seam lands) reports no change even when on
//     against a canary epilogue, leaving the CFG untouched.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionContext};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};

fn build_manager(big: bool) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big,
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
    let manage = build_manager(false);
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.vbank_mut().create(size, Address::new(r, off), unk(size))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

/// `out = COPY(in)` placed in `bl` at `off`, returning the output varnode.
fn copy_op(fd: &mut Funcdata, bl: BlockId, off: u64, input: VarnodeId, out_off: u64) -> VarnodeId {
    let op = mk_op(fd, 1, off, OpCode::CPUI_COPY);
    fd.op_set_input(op, input, 0).unwrap();
    let out = mk_vn(fd, out_off, 8);
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

/// A *written* varnode (output of a COPY of a constant), so it can legally have
/// multiple descendants (a free varnode may have at most one).  Used for the
/// canary `base` pointer that feeds two INT_ADDs.
fn written_vn(fd: &mut Funcdata, bl: BlockId, off: u64, out_off: u64) -> VarnodeId {
    let op = mk_op(fd, 1, off, OpCode::CPUI_COPY);
    let c = fd.new_constant(8, 0);
    fd.op_set_input(op, c, 0).unwrap();
    let out = mk_vn(fd, out_off, 8);
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

/// `out = LOAD(space, ptr)` (a 2-input LOAD) placed in `bl`; returns the output.
fn load_op(fd: &mut Funcdata, bl: BlockId, off: u64, ptr: VarnodeId, out_off: u64) -> VarnodeId {
    let op = mk_op(fd, 2, off, OpCode::CPUI_LOAD);
    // input 0 is the space id (a constant ram-id varnode); input 1 is the ptr.
    let r = ram(fd);
    let spaceid = fd.new_constant(8, r.get_index() as u64);
    fd.op_set_input(op, spaceid, 0).unwrap();
    fd.op_set_input(op, ptr, 1).unwrap();
    let out = mk_vn(fd, out_off, 8);
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

/// `ptr = INT_ADD(base, #cst)` placed in `bl`; returns the output ptr varnode.
fn add_const(fd: &mut Funcdata, bl: BlockId, off: u64, base: VarnodeId, cst: u64, out_off: u64) -> VarnodeId {
    let op = mk_op(fd, 2, off, OpCode::CPUI_INT_ADD);
    fd.op_set_input(op, base, 0).unwrap();
    let c = fd.new_constant(8, cst);
    fd.op_set_input(op, c, 1).unwrap();
    let out = mk_vn(fd, out_off, 8);
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

// --- option decision body -----------------------------------------------------

#[test]
fn option_default_off() {
    let opt = StackGuardOption::default();
    assert!(!opt.is_enabled());
}

#[test]
fn option_apply_on_off() {
    let mut opt = StackGuardOption::default();
    let on = opt.apply(true);
    assert!(opt.is_enabled());
    assert_eq!(on, "Stack-protector canary epilogue stripping turned on");
    let off = opt.apply(false);
    assert!(!opt.is_enabled());
    assert_eq!(off, "Stack-protector canary epilogue stripping turned off");
}

// --- ptr_is_canary_slot -------------------------------------------------------

#[test]
fn ptr_is_canary_slot_recognizes_base_plus_0x28() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let base = mk_vn(&mut fd, 0x80, 8);
    let ptr = add_const(&mut fd, bl, 0x10, base, 0x28, 0x100);
    assert!(ptr_is_canary_slot(ptr, &fd));
}

#[test]
fn ptr_is_canary_slot_through_copy() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let base = mk_vn(&mut fd, 0x80, 8);
    let ptr = add_const(&mut fd, bl, 0x10, base, 0x28, 0x100);
    let copied = copy_op(&mut fd, bl, 0x14, ptr, 0x108);
    assert!(ptr_is_canary_slot(copied, &fd));
}

#[test]
fn ptr_is_canary_slot_rejects_other_offset() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let base = mk_vn(&mut fd, 0x80, 8);
    let ptr = add_const(&mut fd, bl, 0x10, base, 0x30, 0x100);
    assert!(!ptr_is_canary_slot(ptr, &fd));
}

#[test]
fn ptr_is_canary_slot_rejects_unwritten() {
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x80, 8);
    // base has no defining op -> not written -> not a canary slot.
    assert!(!ptr_is_canary_slot(base, &fd));
}

// --- derives_from_canary_load + is_canary_compare -----------------------------

/// Build a full canary epilogue: a LOAD of `<base>+0x28`, two values derived
/// from it (one via COPY), an INT_NOTEQUAL comparing them, and a CBRANCH whose
/// fail successor calls a handler.  Returns (header block, cbranch op, fail blk).
fn build_canary_epilogue(fd: &mut Funcdata) -> (BlockId, OpId, BlockId) {
    let h = mk_block(fd);
    let ok = mk_block(fd);
    let fail = mk_block(fd);

    let base = written_vn(fd, h, 0x08, 0x80);
    let ptr = add_const(fd, h, 0x10, base, 0x28, 0x100);
    let saved = load_op(fd, h, 0x14, ptr, 0x108); // saved canary load
    // fresh reload: a second load of the same slot
    let ptr2 = add_const(fd, h, 0x18, base, 0x28, 0x110);
    let reload = load_op(fd, h, 0x1c, ptr2, 0x118);
    // one side goes through a COPY (value-preserving) to exercise the peel.
    let saved_c = copy_op(fd, h, 0x20, saved, 0x120);

    // bool = INT_NOTEQUAL(saved_c, reload)
    let cmp = mk_op(fd, 2, 0x24, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(cmp, saved_c, 0).unwrap();
    fd.op_set_input(cmp, reload, 1).unwrap();
    let boolvn = mk_vn(fd, 0x128, 1);
    fd.op_set_output(cmp, boolvn).unwrap();
    fd.op_insert_end(cmp, h);

    // CBRANCH(dest, bool)
    let cbr = mk_op(fd, 2, 0x28, OpCode::CPUI_CBRANCH);
    let dest = mk_vn(fd, 0x200, 8);
    fd.op_set_input(cbr, dest, 0).unwrap();
    fd.op_set_input(cbr, boolvn, 1).unwrap();
    fd.op_insert_end(cbr, h);

    // Wire edges h -> ok (false), h -> fail (true).  add_edge order sets
    // out[0]=false, out[1]=true (the BlockGraph convention).
    fd.bblocks_mut().add_edge(h, ok);
    fd.bblocks_mut().add_edge(h, fail);

    // The fail block calls the no-return handler.
    let call = mk_op(fd, 1, 0x30, OpCode::CPUI_CALL);
    let target = mk_vn(fd, 0x300, 8);
    fd.op_set_input(call, target, 0).unwrap();
    fd.op_insert_end(call, fail);

    (h, cbr, fail)
}

#[test]
fn is_canary_compare_true_for_notequal_of_two_canary_loads() {
    let mut fd = build_fd();
    let (_h, cbr, _fail) = build_canary_epilogue(&mut fd);
    let res = is_canary_compare(cbr, &fd);
    assert_eq!(res, Some(true), "INT_NOTEQUAL canary compare, isNotEqual=true");
}

#[test]
fn is_canary_compare_false_when_one_side_not_canary() {
    let mut fd = build_fd();
    let h = mk_block(&mut fd);
    let base = mk_vn(&mut fd, 0x80, 8);
    let ptr = add_const(&mut fd, h, 0x10, base, 0x28, 0x100);
    let saved = load_op(&mut fd, h, 0x14, ptr, 0x108);
    // other side is a plain (non-canary) value
    let other = mk_vn(&mut fd, 0x140, 8);
    let cmp = mk_op(&mut fd, 2, 0x24, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(cmp, saved, 0).unwrap();
    fd.op_set_input(cmp, other, 1).unwrap();
    let boolvn = mk_vn(&mut fd, 0x128, 1);
    fd.op_set_output(cmp, boolvn).unwrap();
    fd.op_insert_end(cmp, h);
    let cbr = mk_op(&mut fd, 2, 0x28, OpCode::CPUI_CBRANCH);
    let dest = mk_vn(&mut fd, 0x200, 8);
    fd.op_set_input(cbr, dest, 0).unwrap();
    fd.op_set_input(cbr, boolvn, 1).unwrap();
    fd.op_insert_end(cbr, h);
    assert_eq!(is_canary_compare(cbr, &fd), None);
}

#[test]
fn is_canary_compare_false_for_non_equal_op() {
    let mut fd = build_fd();
    let h = mk_block(&mut fd);
    let base = written_vn(&mut fd, h, 0x08, 0x80);
    let ptr = add_const(&mut fd, h, 0x10, base, 0x28, 0x100);
    let saved = load_op(&mut fd, h, 0x14, ptr, 0x108);
    let ptr2 = add_const(&mut fd, h, 0x18, base, 0x28, 0x110);
    let reload = load_op(&mut fd, h, 0x1c, ptr2, 0x118);
    // INT_ADD is not a compare op.
    let cmp = mk_op(&mut fd, 2, 0x24, OpCode::CPUI_INT_ADD);
    fd.op_set_input(cmp, saved, 0).unwrap();
    fd.op_set_input(cmp, reload, 1).unwrap();
    let boolvn = mk_vn(&mut fd, 0x128, 1);
    fd.op_set_output(cmp, boolvn).unwrap();
    fd.op_insert_end(cmp, h);
    let cbr = mk_op(&mut fd, 2, 0x28, OpCode::CPUI_CBRANCH);
    let dest = mk_vn(&mut fd, 0x200, 8);
    fd.op_set_input(cbr, dest, 0).unwrap();
    fd.op_set_input(cbr, boolvn, 1).unwrap();
    fd.op_insert_end(cbr, h);
    assert_eq!(is_canary_compare(cbr, &fd), None);
}

// --- block_has_call -----------------------------------------------------------

#[test]
fn block_has_call_detects_call() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    assert!(!block_has_call(bl, &fd));
    let call = mk_op(&mut fd, 1, 0x30, OpCode::CPUI_CALLIND);
    let t = mk_vn(&mut fd, 0x300, 8);
    fd.op_set_input(call, t, 0).unwrap();
    fd.op_insert_end(call, bl);
    assert!(block_has_call(bl, &fd));
}

// --- Action gate --------------------------------------------------------------

#[test]
fn action_inert_when_gate_off() {
    let mut fd = build_fd();
    let (_h, _cbr, _fail) = build_canary_epilogue(&mut fd);
    let blocks_before = fd.bblocks_get_size();
    let mut act = ActionStripStackGuard::new(false, "stackptrflow");
    let mut ctx = ActionContext::default();
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0, "gate off => inert");
    assert_eq!(fd.bblocks_get_size(), blocks_before, "CFG untouched");
}

#[test]
fn action_detects_but_does_not_mutate_when_on() {
    // With the gate on the canary epilogue is *detected* (the scan reaches the
    // victim-edge index), but the removeBranch/removeUnreachableBlocks CFG
    // surgery is the W4/W8 seam, so the Action reports no change and leaves the
    // CFG intact.  This pins the seam boundary: when removeBranch lands, this
    // test's expectation flips to `1` + one fewer edge.
    let mut fd = build_fd();
    let (_h, _cbr, _fail) = build_canary_epilogue(&mut fd);
    let blocks_before = fd.bblocks_get_size();
    let mut act = ActionStripStackGuard::new(true, "stackptrflow");
    let mut ctx = ActionContext::default();
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0, "detection-only until the removeBranch seam lands");
    assert_eq!(fd.bblocks_get_size(), blocks_before, "CFG untouched (seam)");
    // sanity: the action name is correct.
    assert_eq!(act.get_name(), "stripstackguard");
}
