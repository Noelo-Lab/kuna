//! Tests for the callee-body narrowing of a call's killed-register set.
//!
//! These pin the predicate on hand-built summaries: what a complete probe is
//! allowed to prove, and every way it must fail closed. The end-to-end witness
//! -- an i386 PIE whose get-PC thunk ate a live argument -- lives in
//! `tests/stages/kuna-calleepreserves.xml` and `tests/cli/get-pc-helper-loses.json`.

use super::*;

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::{ArchContext, TypeOp};
use crate::fspec::{effect_type, EffectRecord, ProtoModel};
use crate::kuna_rustabi::CalleeReturnWrites;
use kuna_num::opcodes::OpCode;

/// x86gcc's shape, reduced to what the rule reads: EBX (0x10) preserved, EDX
/// (0x08) killed.  Without a model the proto has no effect list at all, and a
/// callee that writes nothing the convention promises to preserve never
/// departs from it.
fn with_convention(fd: &Funcdata, fc: &mut FuncCallSpecs) {
    let ram = space(fd, "ram");
    let mut model = ProtoModel::new(fd.get_arch().manage());
    for (off, ty) in [(0x10u64, effect_type::UNAFFECTED), (0x08, effect_type::KILLEDBYCALL)] {
        let mut vd = kuna_num::pcoderaw::VarnodeData::default();
        vd.space = Some(std::rc::Rc::clone(&ram));
        vd.offset = off;
        vd.size = 4;
        model.push_effect(EffectRecord::from_varnode(vd, ty));
    }
    fc.proto_mut().set_model(Some(std::rc::Rc::new(model)));
}

/// A minimal fixture: a `register` (processor) space, a `stack` spacebase, and
/// the option in the requested state.
fn build_fd(on: bool) -> Funcdata {
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
    ctx.callee_preserves = on;
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

/// The witness shape, reduced to the seam: a complete probe that records only a
/// write of one register proves the OTHER register crosses the call.
#[test]
fn a_complete_probe_proves_an_unwritten_register_preserved() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let ram = space(&fd, "ram").get_index();
    // The get-PC thunk writes EBX (here 0x10) and nothing else.
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(ram, 0x10, 4)], Vec::new(), true)),
    );
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(callee_preserves_range(&fd, &fc, &edx, 4));
    let ebx = Address::new(space(&fd, "ram"), 0x10);
    assert!(!callee_preserves_range(&fd, &fc, &ebx, 4), "the register it DOES write is not narrowed");
}

/// The claim is one-sided: a walk that could not finish proves nothing, and a
/// callee with no recorded probe at all proves nothing either.
#[test]
fn an_incomplete_or_missing_probe_narrows_nothing() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(!callee_preserves_range(&fd, &fc, &edx, 4), "no probe recorded");
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(Vec::new(), Vec::new(), false)),
    );
    assert!(!callee_preserves_range(&fd, &fc, &edx, 4), "an incomplete walk proves nothing");
}

/// Only a register is answered. A stack range keeps the ABI's effect even when
/// the probe recorded no write there, because a callee's memory writes are
/// STOREs through an address the walk cannot follow.
#[test]
fn a_stack_range_is_never_narrowed() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(space(&fd, "ram").get_index(), 0x10, 4)], Vec::new(), true)),
    );
    let slot = Address::new(space(&fd, "stack"), 0xfffffff0);
    assert!(!callee_preserves_range(&fd, &fc, &slot, 4));
}

/// A prototype carrying its own effect override has had a deliberate statement
/// made about it and is left alone.
#[test]
fn an_explicit_effect_override_wins() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(space(&fd, "ram").get_index(), 0x10, 4)], Vec::new(), true)),
    );
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(callee_preserves_range(&fd, &fc, &edx, 4));
    let mut vd = kuna_num::pcoderaw::VarnodeData::default();
    vd.space = Some(space(&fd, "ram"));
    vd.offset = 0x08;
    vd.size = 4;
    fc.proto_mut()
        .push_effect_override(EffectRecord::from_varnode(vd, effect_type::KILLEDBYCALL));
    assert!(!callee_preserves_range(&fd, &fc, &edx, 4));
}

/// The gate fails closed: with the option off the same complete probe narrows
/// nothing.
#[test]
fn the_option_gates_the_whole_predicate() {
    let mut fd = build_fd(false);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let ram = space(&fd, "ram").get_index();
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(ram, 0x10, 4)], Vec::new(), true)),
    );
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(!callee_preserves_range(&fd, &fc, &edx, 4));
}

/// An indirect call has no entry address to decode, so it is never narrowed.
#[test]
fn an_indirect_call_is_never_narrowed() {
    let mut fd = build_fd(true);
    let (fc, _) = build_call(&mut fd, 0x2000);
    let indirect = FuncCallSpecs::new(fc.get_op(), Address::default());
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(!callee_preserves_range(&fd, &indirect, &edx, 4));
}

/// The option string round-trips both ways and rejects anything else.
#[test]
fn the_option_parses_on_and_off() {
    assert!(OptionCalleePreserves.apply("on").unwrap().0);
    assert!(!OptionCalleePreserves.apply("off").unwrap().0);
    assert!(OptionCalleePreserves.apply("maybe").is_err());
}

/// The load-bearing half. A body that writes only the stack pointer is what a
/// one-byte `ret` decodes to, and reading its empty write set as "every register
/// survives this call" deletes the return value of every stub and placeholder in
/// the image. It must prove nothing.
#[test]
fn a_body_that_departs_from_nothing_narrows_nothing() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let ram = space(&fd, "ram").get_index();
    // ESP (0x20) is not in the convention's effect list at all, and neither is
    // EAX (0x00) -- the return register a bare `ret` leaves alone.
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(ram, 0x20, 4), (ram, 0x00, 4)], Vec::new(), true)),
    );
    let edx = Address::new(space(&fd, "ram"), 0x08);
    assert!(!callee_preserves_range(&fd, &fc, &edx, 4));
}
