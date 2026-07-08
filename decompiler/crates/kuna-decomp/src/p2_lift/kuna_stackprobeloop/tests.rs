//! Tests for the gcc stack-probe-loop SP-resolution sub-stage (kuna,
//! GH-8017/6858, S2): the shape match + `MULTIEQUAL`->`INT_ADD` rewrite of
//! [`RuleStackProbeLoop`], the gate-off (`option stackprobeloop off`) no-op, and
//! the [`parse_stack_probe_loop_form`] option parse.
//!
//! The fixture builds the post-heritage shape by hand: a spacebase `MULTIEQUAL`
//! producing the stack-pointer register, whose back-edge is `PHI + page` (the
//! probe `INT_ADD`), whose entry input is the spacebase input `SP_in`, with a
//! loop-exit `INT_NOTEQUAL(PHI + page, SP_in + limitconst)`.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace,
    UniqueSpace, VarnodeStorage,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use super::*;
use crate::action::{ActionGroupList, Rule};
use crate::context::{ArchContext, BlockId, TypeOp, VarnodeId};

// -----------------------------------------------------------------------------
// Harness: a register space + a "stack" spacebase space with a base register.
// -----------------------------------------------------------------------------

/// The stack-pointer base register: register-space offset 0x20, size 8.
const SP_OFF: u64 = 0x20;
const SP_SIZE: int4 = 8;
/// One probe page step.
const PAGE: u64 = 0x1000;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    // The processor register space (index 2) — holds the SP base register and
    // contains the "stack" spacebase.
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
    // The "stack" spacebase space (index 3, contained in "register") — becomes
    // get_stack_space().  SpacebaseSpace::new yields the AddrSpaceKind::Spacebase
    // that set_base_register/get_spacebase require.
    m.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 8, &regspc, 1, true, false)))
        .unwrap();
    m
}

fn register_space(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("register").unwrap())
}

/// Build the architecture, wiring the stack space's base register to the SP
/// register so `findSpacebaseInput` can locate the SP input.
fn build_fd() -> Funcdata {
    let manage = build_manager();
    let regspc = register_space(&manage);
    let stackspc = Rc::clone(manage.get_stack_space().unwrap());
    let sp_data = VarnodeStorage { space: Some(regspc), offset: SP_OFF, size: SP_SIZE as u32 };
    // addSpacebasePointer(stack, sp_data, SP_SIZE, stack_growth=true)
    manage.add_spacebase_pointer(&stackspc, &sp_data, SP_SIZE, true).unwrap();

    let glb = Rc::new(ArchContext::new(manage));
    let code = Rc::clone(glb.manage().get_space_by_name("register").unwrap());
    let entry = Address::new(code, 0x1000);
    Funcdata::new("func", "func", glb, entry, 0x1000_0000, 0x40).unwrap()
}

fn reg_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(register_space(fd.get_arch().manage()), off)
}

fn new_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().new_block_basic(root)
}

fn mk_op(fd: &mut Funcdata, bl: BlockId, off: u64, ninput: int4, opc: OpCode) -> OpId {
    let op = fd.new_op(ninput, reg_addr(fd, off));
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

/// The spacebase input Varnode `SP_in` at the SP register location.
fn make_sp_in(fd: &mut Funcdata) -> VarnodeId {
    let vn = fd.new_varnode(SP_SIZE, &reg_addr(fd, SP_OFF), None);
    fd.set_input_varnode(vn).unwrap()
}

/// Build the canonical probe-loop shape into `bl`.  Returns the `MULTIEQUAL` op
/// and the `SP_in` input.  `limitconst` is `SP_in + limitconst == limit`.
fn build_probe_loop(fd: &mut Funcdata, bl: BlockId, limitconst: u64) -> (OpId, VarnodeId) {
    let sp_in = make_sp_in(fd);

    // limit = INT_ADD(SP_in, limitconst)
    let limop = mk_op(fd, bl, 0x10, 2, OpCode::CPUI_INT_ADD);
    let limc = fd.new_constant(SP_SIZE, limitconst);
    fd.op_set_input(limop, sp_in, 0).unwrap();
    fd.op_set_input(limop, limc, 1).unwrap();
    let limit = fd.new_varnode(SP_SIZE, &reg_addr(fd, 0x40), None);
    fd.op_set_output(limop, limit).unwrap();

    // phiout = MULTIEQUAL(SP_in, step)   (output at the SP register location)
    let phi = fd.new_op(2, reg_addr(fd, 0x20));
    fd.op_set_opcode(phi, TypeOp::new(OpCode::CPUI_MULTIEQUAL, 0, "MULTIEQUAL"));
    let phiout = fd.new_varnode(SP_SIZE, &reg_addr(fd, SP_OFF), None);
    fd.op_set_output(phi, phiout).unwrap();
    fd.op_insert_end(phi, bl);

    // step = INT_ADD(phiout, PAGE)   (the probe decrement / back edge)
    let stepop = mk_op(fd, bl, 0x24, 2, OpCode::CPUI_INT_ADD);
    let pagec = fd.new_constant(SP_SIZE, PAGE);
    fd.op_set_input(stepop, phiout, 0).unwrap();
    fd.op_set_input(stepop, pagec, 1).unwrap();
    let step = fd.new_varnode(SP_SIZE, &reg_addr(fd, 0x48), None);
    fd.op_set_output(stepop, step).unwrap();

    // MULTIEQUAL inputs: entry = SP_in (stack-relative, off 0); back-edge = step
    fd.op_set_input(phi, sp_in, 0).unwrap();
    fd.op_set_input(phi, step, 1).unwrap();

    // cmp = INT_NOTEQUAL(step, limit)   (the loop-exit comparison)
    let cmp = mk_op(fd, bl, 0x28, 2, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(cmp, step, 0).unwrap();
    fd.op_set_input(cmp, limit, 1).unwrap();
    let boolvn = fd.new_unique_out(1, cmp).unwrap();
    let _ = boolvn;

    (phi, sp_in)
}

fn rule(enabled: bool) -> RuleStackProbeLoop {
    RuleStackProbeLoop::new(enabled, "analysis")
}

// -----------------------------------------------------------------------------
// getOpList / clone
// -----------------------------------------------------------------------------

#[test]
fn op_list_is_multiequal() {
    assert_eq!(rule(true).get_op_list(), vec![OpCode::CPUI_MULTIEQUAL]);
}

#[test]
fn clone_filters_by_group() {
    let r = rule(true);
    let inc = ActionGroupList::from_names(["analysis"]);
    let exc = ActionGroupList::from_names(["cleanup"]);
    assert!(r.clone_rule(&inc).is_some());
    assert!(r.clone_rule(&exc).is_none());
}

// -----------------------------------------------------------------------------
// The transformation
// -----------------------------------------------------------------------------

#[test]
fn rewrites_multiequal_to_constant_add() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let limitconst: u64 = 0x3000;
    let (phi, sp_in) = build_probe_loop(&mut fd, bl, limitconst);

    let changed = rule(true).apply_op(phi, &mut fd);
    assert_eq!(changed, 1, "the canonical probe loop must be rewritten");

    // The MULTIEQUAL became INT_ADD(SP_in, limitconst - PAGE).
    let o = fd.obank().get(phi).unwrap();
    assert_eq!(o.code(), OpCode::CPUI_INT_ADD);
    assert_eq!(o.num_input(), 2);
    assert_eq!(o.get_in(0), Some(sp_in));
    let cvn = o.get_in(1).unwrap();
    let c = fd.vbank().get(cvn).unwrap();
    assert!(c.is_constant());
    let expected = limitconst.wrapping_sub(PAGE) & calc_mask(SP_SIZE);
    assert_eq!(c.get_offset(), expected);
}

// -----------------------------------------------------------------------------
// Gate-off: option stackprobeloop off -> inert (byte-identical upstream)
// -----------------------------------------------------------------------------

#[test]
fn gate_off_is_inert() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let (phi, _sp_in) = build_probe_loop(&mut fd, bl, 0x3000);

    let changed = rule(false).apply_op(phi, &mut fd);
    assert_eq!(changed, 0, "gate-off must decline");
    // The op is still a MULTIEQUAL — untouched.
    assert_eq!(fd.obank().get(phi).unwrap().code(), OpCode::CPUI_MULTIEQUAL);
}

// -----------------------------------------------------------------------------
// Shape rejections
// -----------------------------------------------------------------------------

#[test]
fn non_two_input_multiequal_declines() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let _sp = make_sp_in(&mut fd);
    // A 3-input MULTIEQUAL at the SP location: not a 2-edge loop header.
    let phi = fd.new_op(3, reg_addr(&fd, 0x20));
    fd.op_set_opcode(phi, TypeOp::new(OpCode::CPUI_MULTIEQUAL, 0, "MULTIEQUAL"));
    let out = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(phi, out).unwrap();
    fd.op_insert_end(phi, bl);
    for i in 0..3 {
        let c = fd.new_constant(SP_SIZE, i as u64);
        fd.op_set_input(phi, c, i).unwrap();
    }
    assert_eq!(rule(true).apply_op(phi, &mut fd), 0);
}

#[test]
fn missing_exit_compare_declines() {
    // A probe loop whose back-edge exists but whose step Varnode feeds no
    // INT_EQUAL/INT_NOTEQUAL against a stack-relative limit: no limit found.
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let sp_in = make_sp_in(&mut fd);

    let phi = fd.new_op(2, reg_addr(&fd, 0x20));
    fd.op_set_opcode(phi, TypeOp::new(OpCode::CPUI_MULTIEQUAL, 0, "MULTIEQUAL"));
    let phiout = fd.new_varnode(SP_SIZE, &reg_addr(&fd, SP_OFF), None);
    fd.op_set_output(phi, phiout).unwrap();
    fd.op_insert_end(phi, bl);

    // back-edge step, but its output is read by nothing comparable.
    let stepop = mk_op(&mut fd, bl, 0x24, 2, OpCode::CPUI_INT_ADD);
    let pagec = fd.new_constant(SP_SIZE, PAGE);
    fd.op_set_input(stepop, phiout, 0).unwrap();
    fd.op_set_input(stepop, pagec, 1).unwrap();
    let _step = fd.new_varnode(SP_SIZE, &reg_addr(&fd, 0x48), None);
    fd.op_set_output(stepop, _step).unwrap();

    fd.op_set_input(phi, sp_in, 0).unwrap();
    fd.op_set_input(phi, _step, 1).unwrap();

    assert_eq!(rule(true).apply_op(phi, &mut fd), 0);
}

// -----------------------------------------------------------------------------
// Option parse (OptionStackProbeLoop::apply)
// -----------------------------------------------------------------------------

#[test]
fn option_parse_on_off() {
    let (on, msg_on) = parse_stack_probe_loop_form("on").unwrap();
    assert_eq!(on, StackProbeLoopForm::On);
    assert!(on.model_stack_probe_loop());
    assert_eq!(msg_on, "Stack-probe-loop stack-pointer resolution turned on");

    let (off, msg_off) = parse_stack_probe_loop_form("off").unwrap();
    assert_eq!(off, StackProbeLoopForm::Off);
    assert!(!off.model_stack_probe_loop());
    assert_eq!(msg_off, "Stack-probe-loop stack-pointer resolution turned off");

    // Empty string defaults to "on" (onOrOff convention).
    assert_eq!(parse_stack_probe_loop_form("").unwrap().0, StackProbeLoopForm::On);
    // Garbage is a parse error.
    assert!(parse_stack_probe_loop_form("maybe").is_err());
}
