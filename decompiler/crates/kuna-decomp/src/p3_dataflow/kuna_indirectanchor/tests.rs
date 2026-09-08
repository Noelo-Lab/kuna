//! Tests for the INDIRECT-marker insertion anchor.
//!
//! The shape the redirect exists for — an op inserted after a call's guard
//! INDIRECT landing between the guards and their CALL, where it hides the
//! return-value trial — is built here against a real `Funcdata` and run through
//! [`check_output_trial_use`](crate::funcdata_callsite::check_output_trial_use)
//! with the option both ways.  The end-to-end witness is
//! `tests/stages/kuna-indirectanchor.xml` and
//! `tests/cli/bcryptgetproperty-status-tested-through.json`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace,
};
use kuna_base::types::int4;

use crate::context::{ArchContext, TypeOp};
use crate::fspec::FuncCallSpecs;
use crate::funcdata_callsite::check_output_trial_use;

/// The return storage the convention names (`ram:0x00`, 4 bytes).
const RET_OFF: u64 = 0x00;
const RET_SIZE: int4 = 4;

fn build_fd(on: bool) -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.indirect_anchor = on;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// A CALL preceded by two guard INDIRECTs, the first of which is the return
/// trial's creation.  `refine` inserts one non-INDIRECT op after the SECOND
/// guard, which is where heritage's frame refinement puts a piece-defining
/// SUBPIECE.
fn build_call(fd: &mut Funcdata, refine: bool) -> FuncCallSpecs {
    let sp = ram(fd);
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);

    let call = fd.new_op(1, Address::new(Rc::clone(&sp), 0x1010));
    fd.obank_mut()
        .change_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CPUI_CALL".to_string()));
    let entry = Address::new(Rc::clone(&sp), 0x2000);
    let target = fd.new_code_ref(&entry);
    let _ = fd.op_set_input(call, target, 0);
    fd.op_insert(call, bl, None);

    let ret_addr = Address::new(Rc::clone(&sp), RET_OFF);
    fd.new_indirect_creation(call, &ret_addr, RET_SIZE, true);
    let second = fd.new_indirect_creation(call, &Address::new(Rc::clone(&sp), 0x800), 8, true);

    if refine {
        let g = fd.new_op(1, Address::new(Rc::clone(&sp), 0x1010));
        let src = fd.new_constant(8, 0);
        let _ = fd.op_set_input(g, src, 0);
        let _ = fd.new_varnode_out(4, &Address::new(Rc::clone(&sp), 0x804), g);
        fd.op_set_opcode_code(g, OpCode::CPUI_SUBPIECE);
        fd.op_insert_after(g, second);
    }

    let mut fc = FuncCallSpecs::new(call, entry);
    fc.init_active_output();
    fc.get_active_output().register_trial(&ret_addr, RET_SIZE);
    fc
}

/// Did the return-trial walk find the Varnode standing behind the trial?
fn recovers_return(on: bool, refine: bool) -> bool {
    let mut fd = build_fd(on);
    let mut fc = build_call(&mut fd, refine);
    let trialvn = check_output_trial_use(&mut fc, &mut fd);
    trialvn.first().copied().flatten().is_some()
}

#[test]
fn an_unrefined_guard_run_recovers_the_return_either_way() {
    assert!(recovers_return(true, false));
    assert!(recovers_return(false, false));
}

#[test]
fn off_the_refinement_op_lands_in_the_run_and_hides_the_trial() {
    assert!(!recovers_return(false, true));
}

#[test]
fn on_the_refinement_op_anchors_to_the_call_and_the_trial_survives() {
    assert!(recovers_return(true, true));
}

#[test]
fn the_anchor_is_the_op_the_marker_speaks_for() {
    let mut fd = build_fd(true);
    let sp = ram(&fd);
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);
    let call = fd.new_op(1, Address::new(Rc::clone(&sp), 0x1010));
    fd.obank_mut()
        .change_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CPUI_CALL".to_string()));
    fd.op_insert(call, bl, None);
    let ind = fd.new_indirect_creation(call, &Address::new(Rc::clone(&sp), RET_OFF), RET_SIZE, true);

    assert_eq!(anchor_of(&fd, ind), Some(call), "the marker resolves to its CALL");
    assert_eq!(anchor_of(&fd, call), None, "a CALL is not a marker");
}

#[test]
fn off_nothing_is_redirected() {
    let mut fd = build_fd(false);
    let sp = ram(&fd);
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);
    let call = fd.new_op(1, Address::new(Rc::clone(&sp), 0x1010));
    fd.obank_mut()
        .change_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CPUI_CALL".to_string()));
    fd.op_insert(call, bl, None);
    let ind = fd.new_indirect_creation(call, &Address::new(Rc::clone(&sp), RET_OFF), RET_SIZE, true);

    assert_eq!(anchor_of(&fd, ind), None);
}

#[test]
fn option_surface() {
    assert_eq!(OptionIndirectAnchor::NAME, "indirectanchor");
    assert!(OptionIndirectAnchor.apply("on").unwrap().0);
    assert!(!OptionIndirectAnchor.apply("off").unwrap().0);
    assert!(OptionIndirectAnchor.apply("maybe").is_err());
}
