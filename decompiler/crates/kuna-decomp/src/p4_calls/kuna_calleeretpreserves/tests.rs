//! Tests for the callee-body answer about a call's RETURN register.
//!
//! These pin the ways the predicate must fail closed on hand-built summaries.
//! The gates that consult the prototype MODEL -- the range being the call's
//! output, the body writing no part of the return storage, the evidence write
//! being a register the convention names -- need a built resource list, which
//! the reduced fixture here cannot supply; they are pinned end to end, against a
//! real architecture and with an `endbr64; ret` negative control, by
//! `tests/stages/kuna-calleeretpreserves.xml`. The end-to-end witness -- an MSVC `/GS` `main` returning its cookie
//! check instead of the zero it set -- lives in
//! `tests/stages/kuna-calleeretpreserves.xml` and
//! `tests/cli/main-returns-invented-cookie.json`.

use super::*;

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::{ArchContext, TypeOp};
use crate::dtype::{type_class, type_metatype, Datatype};
use crate::fspec::{effect_type, EffectRecord, ParamEntry, ProtoModel};
use crate::kuna_rustabi::CalleeReturnWrites;
use kuna_num::opcodes::OpCode;

/// The convention, reduced to what the rule reads: RAX (here 0x00) killed by
/// call, RBX (0x10) preserved.
fn with_convention(fd: &Funcdata, fc: &mut FuncCallSpecs) {
    let ram = space(fd, "ram");
    let mut model = ProtoModel::new(fd.get_arch().manage());
    model.build_param_list("standard").unwrap();
    let input = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&ram),
        0x20,
        8,
        1,
        0,
        0,
        true,
        false,
        &[],
        fd.get_arch().manage(),
    )
    .unwrap();
    model.input_mut().push_entry(input);
    model.input_mut().finish_decode();
    let output = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&ram),
        0x00,
        8,
        1,
        0,
        0,
        true,
        false,
        &[],
        fd.get_arch().manage(),
    )
    .unwrap();
    model.output_mut().push_entry(output);
    model.output_mut().finish_decode();
    for (off, ty) in [
        (0x00u64, effect_type::KILLEDBYCALL),
        (0x08, effect_type::KILLEDBYCALL),
        (0x10, effect_type::UNAFFECTED),
    ] {
        let mut vd = kuna_num::pcoderaw::VarnodeData::default();
        vd.space = Some(Rc::clone(&ram));
        vd.offset = off;
        vd.size = 8;
        model.push_effect(EffectRecord::from_varnode(vd, ty));
    }
    let void_ty = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    fc.proto_mut().set_internal(Rc::new(model), void_ty);
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
        8,
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
        8,
        1,
        3,
        0,
        1,
        1,
    )))
    .unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.callee_ret_preserves = on;
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

/// Record the cookie checker's shape: it rolls a scratch register and sets the
/// flags, and never touches the return register.
fn cookie_checker(fd: &Funcdata) -> Rc<CalleeReturnWrites> {
    let ram = space(fd, "ram").get_index();
    // RCX (0x20) plus a flag byte (0x200), and the stack pointer every RET writes.
    Rc::new(CalleeReturnWrites::from_parts(
        vec![(ram, 0x20, 8), (ram, 0x200, 1), (ram, 0x80, 8)],
        Vec::new(),
        true,
    ))
}

#[test]
fn an_incomplete_or_missing_probe_narrows_nothing() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_never_writes(&fd, &fc, &rax, 8), "no probe recorded");
    let ram = space(&fd, "ram").get_index();
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(vec![(ram, 0x20, 8)], Vec::new(), false)),
    );
    assert!(!callee_never_writes(&fd, &fc, &rax, 8), "an incomplete walk proves nothing");
}

#[test]
fn a_locked_void_declaration_keeps_the_abi_return_storage_available_to_the_body_proof() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    fd.kuna_set_callee_ret_writes(&entry, cookie_checker(&fd));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(callee_preserves_return_storage(&fd, &fc, &rax, 8));
}

#[test]
fn a_scratch_register_outside_the_abi_output_is_not_globally_preserved() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    fd.kuna_set_callee_ret_writes(&entry, cookie_checker(&fd));
    let scratch = Address::new(space(&fd, "ram"), 0x08);
    assert!(!callee_preserves_return_storage(&fd, &fc, &scratch, 8));
}

#[test]
fn a_wide_killed_range_resolves_only_the_exact_locked_void_output_slice() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    fd.kuna_set_callee_ret_writes(&entry, cookie_checker(&fd));

    let whole_xmm = Address::new(space(&fd, "ram"), 0x00);
    let upper_scratch = Address::new(space(&fd, "ram"), 0x08);
    assert_eq!(
        characterize_preserved_output(&fc, &whole_xmm, 16),
        Containment::ContainedBy
    );
    assert!(callee_preserves_return_storage(&fd, &fc, &whole_xmm, 8));
    assert!(!callee_preserves_return_storage(&fd, &fc, &whole_xmm, 16));
    assert!(exact_cookie_preserves_return_storage(&fd, &fc, &whole_xmm, 8));
    assert!(!exact_cookie_preserves_return_storage(&fd, &fc, &whole_xmm, 16));
    assert!(!exact_cookie_preserves_return_storage(&fd, &fc, &upper_scratch, 8));

    let generic = callee_preserved_output_within(&fd, &fc, &whole_xmm, 16)
        .expect("the complete body proof resolves the exact output slice");
    assert_eq!(generic.offset, 0x00);
    assert_eq!(generic.size, 8);
    let exact = exact_cookie_preserved_output_within(&fd, &fc, &whole_xmm, 16)
        .expect("the 8-byte ABI output inside the 16-byte machine register");
    assert_eq!(exact.offset, 0x00);
    assert_eq!(exact.size, 8);
}

#[test]
fn a_body_write_to_the_abi_return_storage_wins() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    let ram = space(&fd, "ram").get_index();
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(
            vec![(ram, 0x00, 8), (ram, 0x20, 8)],
            Vec::new(),
            true,
        )),
    );
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_return_storage(&fd, &fc, &rax, 8));
}

#[test]
fn an_exact_cookie_marker_does_not_override_a_known_return_storage_write() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    let ram = space(&fd, "ram").get_index();
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(
            vec![(ram, 0x00, 8), (ram, 0x20, 8)],
            Vec::new(),
            false,
        )),
    );
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!exact_cookie_preserves_return_storage(&fd, &fc, &rax, 8));
}

#[test]
fn an_exact_cookie_marker_yields_to_an_incomplete_output_space_store() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    let ram = space(&fd, "ram");
    let summary = Rc::new(CalleeReturnWrites::from_parts(
        Vec::new(),
        vec![ram.get_index()],
        false,
    ));
    assert!(summary.written_ranges().is_empty());
    assert_eq!(summary.store_spaces(), &[ram.get_index()]);
    assert!(!summary.is_complete());
    fd.kuna_set_callee_ret_writes(&entry, summary);

    let rax = Address::new(ram, 0x00);
    assert!(!exact_cookie_preserves_return_storage(&fd, &fc, &rax, 8));
}

#[test]
fn an_incomplete_store_outside_the_output_processor_space_is_not_a_write_veto() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    let stack = space(&fd, "stack");
    fd.kuna_set_callee_ret_writes(
        &entry,
        Rc::new(CalleeReturnWrites::from_parts(
            Vec::new(),
            vec![stack.get_index()],
            false,
        )),
    );

    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(exact_cookie_preserves_return_storage(&fd, &fc, &rax, 8));
}

/// A prototype carrying its own effect override has had a deliberate statement
/// made about the call, so the inferred body proof cannot replace it.
#[test]
fn an_explicit_effect_override_wins_over_the_body_proof() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    fd.kuna_set_callee_ret_writes(&entry, cookie_checker(&fd));
    let mut vd = kuna_num::pcoderaw::VarnodeData::default();
    vd.space = Some(space(&fd, "ram"));
    vd.offset = 0x00;
    vd.size = 8;
    fc.proto_mut()
        .push_effect_override(EffectRecord::from_varnode(vd, effect_type::KILLEDBYCALL));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_preserves_return_storage(&fd, &fc, &rax, 8));
}

#[test]
fn an_exact_cookie_marker_does_not_override_an_explicit_effect() {
    let mut fd = build_fd(true);
    let (mut fc, entry) = build_call(&mut fd, 0x2000);
    fc.proto_mut().set_output_lock(true);
    fd.kuna_set_callee_ret_writes(&entry, cookie_checker(&fd));
    let mut vd = kuna_num::pcoderaw::VarnodeData::default();
    vd.space = Some(space(&fd, "ram"));
    vd.offset = 0x00;
    vd.size = 8;
    fc.proto_mut()
        .push_effect_override(EffectRecord::from_varnode(vd, effect_type::KILLEDBYCALL));
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!exact_cookie_preserves_return_storage(&fd, &fc, &rax, 8));
}

/// The load-bearing half. A body that writes only the stack pointer is what a
/// stub, a placeholder and an entry decoded at the wrong address all look like,
#[test]
fn a_stack_range_is_never_narrowed() {
    let mut fd = build_fd(true);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let w = cookie_checker(&fd);
    fd.kuna_set_callee_ret_writes(&entry, w);
    let slot = Address::new(space(&fd, "stack"), 0xffffffffffffffe0);
    assert!(!callee_never_writes(&fd, &fc, &slot, 8));
}

/// The option gate remains the outermost prerequisite.
#[test]
fn the_option_gates_the_whole_predicate() {
    let mut fd = build_fd(false);
    let (fc, entry) = build_call(&mut fd, 0x2000);
    let w = cookie_checker(&fd);
    fd.kuna_set_callee_ret_writes(&entry, w);
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_never_writes(&fd, &fc, &rax, 8));
}

/// An indirect call has no entry address to decode, so it is never narrowed.
#[test]
fn an_indirect_call_is_never_narrowed() {
    let mut fd = build_fd(true);
    let (fc, _) = build_call(&mut fd, 0x2000);
    let indirect = FuncCallSpecs::new(fc.get_op(), Address::default());
    let rax = Address::new(space(&fd, "ram"), 0x00);
    assert!(!callee_never_writes(&fd, &indirect, &rax, 8));
}

/// The option string round-trips both ways and rejects anything else.
#[test]
fn the_option_parses_on_and_off() {
    assert!(OptionCalleeRetPreserves.apply("on").unwrap().0);
    assert!(!OptionCalleeRetPreserves.apply("off").unwrap().0);
    assert!(OptionCalleeRetPreserves.apply("maybe").is_err());
}
