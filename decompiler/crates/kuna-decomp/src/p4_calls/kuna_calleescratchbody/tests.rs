//! Tests for the scratch-only clause of `calleepreserves`'s positive-evidence
//! gate.
//!
//! These pin the predicate on hand-built summaries: which bodies count as
//! bodies, and every way the clause must fail closed. The end-to-end witness --
//! MSVC's out-of-line stack probe eating the frame size the caller loaded --
//! lives in `tests/stages/kuna-calleescratchbody.xml`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::{ArchContext, TypeOp};
use crate::fspec::{effect_type, EffectRecord, FuncCallSpecs, ProtoModel};
use crate::kuna_rustabi::CalleeReturnWrites;
use crate::p4_calls::kuna_calleepreserves::callee_preserves_range;
use kuna_num::opcodes::OpCode;

/// x86-64-win's shape, reduced to what the rule reads: RBX (0x18) preserved,
/// RAX (0x00) killed.  Everything else -- 0x90 standing for R10 -- is SCRATCH:
/// named by neither list, which is why the shipped gate cannot see a helper
/// that clobbers only it.
fn with_convention(fd: &Funcdata, fc: &mut FuncCallSpecs) {
    let ram = space(fd, "ram");
    let mut model = ProtoModel::new(fd.get_arch().manage());
    for (off, ty) in [(0x18u64, effect_type::UNAFFECTED), (0x00, effect_type::KILLEDBYCALL)] {
        let mut vd = kuna_num::pcoderaw::VarnodeData::default();
        vd.space = Some(Rc::clone(&ram));
        vd.offset = off;
        vd.size = 8;
        model.push_effect(EffectRecord::from_varnode(vd, ty));
    }
    fc.proto_mut().set_model(Some(Rc::new(model)));
}

/// A minimal fixture: a `register` (processor) space, a `stack` spacebase, and
/// both options in the requested state.
fn build_fd(scratch_body: bool) -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        4,
        1,
        3,
        0,
        1,
        1,
    )))
    .unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.callee_preserves = true;
    ctx.callee_scratch_body = scratch_body;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn space(fd: &Funcdata, name: &str) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(name).unwrap())
}

/// A direct CALL and the call spec that names its callee entry.
fn build_call(fd: &mut Funcdata, entry_off: u64) -> (FuncCallSpecs, Address) {
    let ram = space(fd, "ram");
    let root = fd.bblocks_root_pub();
    let bl = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().set_start_block(root, bl);
    let call = fd.new_op(1, Address::new(Rc::clone(&ram), 0x1010));
    fd.obank_mut()
        .change_opcode(call, TypeOp::new(OpCode::CPUI_CALL, 0, "CPUI_CALL".to_string()));
    let entry = Address::new(Rc::clone(&ram), entry_off);
    let target = fd.new_code_ref(&entry);
    let _ = fd.op_set_input(call, target, 0);
    fd.op_insert(call, bl, None);
    let mut fc = FuncCallSpecs::new(call, entry.clone());
    with_convention(fd, &mut fc);
    (fc, entry)
}

/// The stack-probe summary: nineteen instructions, a spill into `ram`, and
/// writes only to a register the convention names in neither list.
fn probe_summary(fd: &Funcdata, instructions: u32, stores: bool) -> Rc<CalleeReturnWrites> {
    let ram = space(fd, "ram").get_index();
    // The spill lands in memory, not in the register file -- as it does on a
    // real processor, where `proves_untouched` would otherwise read a STORE
    // into the queried space as "may write anything here".
    let store_spaces = if stores { vec![space(fd, "stack").get_index()] } else { Vec::new() };
    Rc::new(CalleeReturnWrites::from_parts_sized(
        vec![(ram, 0x90, 8)],
        store_spaces,
        true,
        instructions,
    ))
}

/// The witness shape: a helper that spills a scratch register and never touches
/// RAX proves the caller's RAX crosses the call.
#[test]
fn a_scratch_only_helper_that_spills_is_a_body() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(&entry, probe_summary(&fd, 19, true));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(callee_preserves_range(&fd, &fc, &rax, 8));
    let r10 = Address::new(space(&fd, "ram"), 0x90);
    assert!(
        !callee_preserves_range(&fd, &fc, &r10, 8),
        "the register it DOES write is still not narrowed"
    );
}

/// The clause is what the option turns off: with it off the shipped gate is the
/// only one that speaks, and it declines a body that departs from nothing.
#[test]
fn the_option_off_restores_the_convention_only_gate() {
    let mut fd = build_fd(false);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(&entry, probe_summary(&fd, 19, true));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_range(&fd, &fc, &rax, 8));
}

/// A body that writes no memory is what a stub, a placeholder and an entry
/// decoded at the wrong address all decode to, and its silence is not evidence.
#[test]
fn a_body_that_stores_nothing_is_not_a_body() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(&entry, probe_summary(&fd, 19, false));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_range(&fd, &fc, &rax, 8));
}

/// One decoded instruction is a bare `ret`, whatever else it happens to record.
#[test]
fn a_one_instruction_body_is_not_a_body() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(&entry, probe_summary(&fd, 1, true));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_range(&fd, &fc, &rax, 8));
}

/// The clause widens which bodies count as evidence, never what the evidence
/// proves: an incomplete walk still narrows nothing.
#[test]
fn an_incomplete_walk_still_proves_nothing() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let ram = space(&fd, "ram").get_index();
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts_sized(Vec::new(), Vec::new(), false, 19)),
    );
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_range(&fd, &fc, &rax, 8));
    let _ = ram;
}
