//! Adversarial verification tests for item `w10-inline-inject`
//! (the inline/injection/override exec-failure seam: ports of
//! `decompiler/cpp/flow.cc` flow-override processing, `funcdata_op.cc`
//! `Funcdata::overrideFlow`, `override.cc` `Override::stringToType`/store, and
//! `pcodeinject.cc`/`inject_sleigh.cc` call-fixup registration + `getPayloadId`).
//!
//! Written by the INDEPENDENT verifier, targeting the spots the hunt list and
//! the C++ comparison flagged as most fragile and NOT already covered by the
//! porter's three `verify_w3_ir_funcdata_op.rs` cases (CALL->BRANCH,
//! CALL_RETURN insert, no-branch error):
//!
//!   - `overrideFlow` RETURN type: BRANCHIND->RETURN and CALLIND->RETURN (the
//!     two opcodes the RETURN arm rewrites) — funcdata_op.cc:1013-1019.
//!   - `overrideFlow` RETURN on a CALL/BRANCH/CBRANCH: the "Do not currently
//!     support complex overrides" error (funcdata_op.cc:1014-1015).
//!   - `overrideFlow` CALL on a CBRANCH: the "Do not currently support CBRANCH
//!     overrides" error (funcdata_op.cc:1002-1003).
//!   - `overrideFlow` CALL_RETURN rewrites BRANCHIND->CALLIND AND inserts a
//!     RETURN (the combined arm, distinct from the porter's CALL-stays-CALL case).
//!   - `Override::stringToType` exhaustive incl. the unknown-token -> NONE path
//!     (the no-special-casing guard: any unmatched byte string is NONE, not a
//!     hardcoded fallback to a real type) — override.cc:421-433.
//!   - the per-function `Override` store round-trip used by `FlowInfo` at flow
//!     time: insert_flow_override / get_flow_override / has_flow_override and the
//!     "no override at this addr -> NONE" miss — override.cc:148,233,84.
//!   - call-fixup registration is NAME-keyed, not address/value-hardcoded:
//!     a fixup registered under a name resolves by that name via getPayloadId,
//!     an UNKNOWN name returns -1, and a different type's namespace does not
//!     leak — pcodeinject.cc:285-309.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::inject_sleigh::PcodeInjectLibrarySleigh;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::overrides::{flow_type, Override};
use kuna_decomp::pcodeinject::{CALLFIXUP_TYPE, CALLOTHERFIXUP_TYPE};
use kuna_decomp::seams::{Architecture, OpId, TypeOp, VarnodeId};

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
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn opcode_flags(opc: OpCode) -> uint4 {
    match opc {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(4, Address::new(ram, off), dt(4))
}

fn code_of(fd: &Funcdata, op: OpId) -> OpCode {
    fd.obank().get(op).unwrap().code()
}

// -----------------------------------------------------------------------------
// 1. overrideFlow RETURN: BRANCHIND -> RETURN and CALLIND -> RETURN.
//    (funcdata_op.cc:1013-1019 — the two opcodes the RETURN arm rewrites)
// -----------------------------------------------------------------------------
#[test]
fn w10_override_flow_return_rewrites_branchind() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x400);
    // A dead BRANCHIND at 0x400 (in0 = the indirect target varnode, non-const).
    let bi = mk_op(&mut fd, 1, 0x400, OpCode::CPUI_BRANCHIND);
    let tgt = mk_vn(&mut fd, 0x900);
    fd.op_set_input(bi, tgt, 0).unwrap();
    // RETURN override: findPrimaryBranch(true,true,false) finds the BRANCHIND via
    // the findbranch arm; the RETURN arm rewrites BRANCHIND -> RETURN.
    fd.override_flow(&addr, flow_type::RETURN).unwrap();
    assert_eq!(code_of(&fd, bi), OpCode::CPUI_RETURN);
}

#[test]
fn w10_override_flow_return_rewrites_callind() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x400);
    let ci = mk_op(&mut fd, 1, 0x400, OpCode::CPUI_CALLIND);
    let tgt = mk_vn(&mut fd, 0x900);
    fd.op_set_input(ci, tgt, 0).unwrap();
    // RETURN override finds the CALLIND via the findcall arm; RETURN arm rewrites
    // CALLIND -> RETURN (funcdata_op.cc:1018-1019).
    fd.override_flow(&addr, flow_type::RETURN).unwrap();
    assert_eq!(code_of(&fd, ci), OpCode::CPUI_RETURN);
}

// -----------------------------------------------------------------------------
// 2. overrideFlow RETURN on a CALL: "Do not currently support complex overrides".
//    (funcdata_op.cc:1014-1015) — and the symmetric CBRANCH guard for CALL.
// -----------------------------------------------------------------------------
#[test]
fn w10_override_flow_return_on_call_is_complex_error() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x400);
    // A dead CALL: RETURN override finds it (findcall=true) but the RETURN arm
    // refuses a CALL/BRANCH/CBRANCH with the "complex overrides" error.
    let call = mk_op(&mut fd, 1, 0x400, OpCode::CPUI_CALL);
    let tgt = mk_vn(&mut fd, 0x900);
    fd.op_set_input(call, tgt, 0).unwrap();
    let err = fd.override_flow(&addr, flow_type::RETURN).unwrap_err();
    assert!(
        err.explain().contains("Do not currently support complex overrides"),
        "got: {}",
        err.explain()
    );
    // The op is left unchanged (still a CALL): the error fires before any rewrite.
    assert_eq!(code_of(&fd, call), OpCode::CPUI_CALL);
}

#[test]
fn w10_override_flow_call_on_cbranch_is_cbranch_error() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x400);
    // A dead CBRANCH with a NON-constant in0 (findPrimaryBranch skips internal
    // branches whose in0 is constant; a real machine CBRANCH target is non-const).
    let cb = mk_op(&mut fd, 2, 0x400, OpCode::CPUI_CBRANCH);
    let tgt = mk_vn(&mut fd, 0x900);
    let cond = mk_vn(&mut fd, 0x910);
    fd.op_set_input(cb, tgt, 0).unwrap();
    fd.op_set_input(cb, cond, 1).unwrap();
    // CALL override on a CBRANCH: the CALL arm explicitly rejects CBRANCH.
    let err = fd.override_flow(&addr, flow_type::CALL).unwrap_err();
    assert!(
        err.explain().contains("Do not currently support CBRANCH overrides"),
        "got: {}",
        err.explain()
    );
    assert_eq!(code_of(&fd, cb), OpCode::CPUI_CBRANCH);
}

// -----------------------------------------------------------------------------
// 3. overrideFlow CALL_RETURN rewrites BRANCHIND -> CALLIND AND inserts a RETURN.
//    (funcdata_op.cc:1000-1011 — distinct from the porter's CALL-stays-CALL case)
// -----------------------------------------------------------------------------
#[test]
fn w10_override_flow_call_return_branchind_becomes_callind_plus_return() {
    use std::collections::BTreeSet;
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x400);
    let bi = mk_op(&mut fd, 1, 0x400, OpCode::CPUI_BRANCHIND);
    let tgt = mk_vn(&mut fd, 0x900);
    fd.op_set_input(bi, tgt, 0).unwrap();
    // Snapshot the dead ops present before the override (a TEST bookkeeping set
    // for "which op id is new" — not ported logic).
    let before: BTreeSet<OpId> = fd.obank().iter_dead().collect();
    fd.override_flow(&addr, flow_type::CALL_RETURN).unwrap();
    // BRANCHIND was rewritten to CALLIND (the CALL/CALL_RETURN arm).
    assert_eq!(code_of(&fd, bi), OpCode::CPUI_CALLIND);
    // Exactly one new op was inserted, and it is a RETURN.
    let new_ops: Vec<OpId> =
        fd.obank().iter_dead().filter(|o| !before.contains(o)).collect();
    assert_eq!(new_ops.len(), 1, "exactly one RETURN op inserted");
    assert_eq!(code_of(&fd, new_ops[0]), OpCode::CPUI_RETURN);
}

// -----------------------------------------------------------------------------
// 4. Override::stringToType exhaustive + the unknown-token -> NONE guard.
//    (override.cc:421-433) — the no-special-casing check: an arbitrary token
//    must map to NONE, not silently to a real override type.
// -----------------------------------------------------------------------------
#[test]
fn w10_string_to_type_exhaustive_and_unknown_is_none() {
    assert_eq!(Override::string_to_type(b"branch"), flow_type::BRANCH);
    assert_eq!(Override::string_to_type(b"call"), flow_type::CALL);
    assert_eq!(Override::string_to_type(b"callreturn"), flow_type::CALL_RETURN);
    assert_eq!(Override::string_to_type(b"return"), flow_type::RETURN);
    // Anything else is NONE (the C++ default-return). Includes case mismatch,
    // empty, near-misses, and a function-name-shaped token (no special-casing).
    for tok in [
        &b""[..],
        &b"Branch"[..],
        &b"BRANCH"[..],
        &b"call_return"[..],
        &b"returns"[..],
        &b"main"[..],
        &b"none"[..],
        &b"guard_dispatch_icall"[..],
    ] {
        assert_eq!(
            Override::string_to_type(tok),
            flow_type::NONE,
            "token {:?} must map to NONE",
            String::from_utf8_lossy(tok)
        );
    }
    // NONE is 0 and the four real types are the distinct 1..=4 the C++ enum fixes.
    assert_eq!(flow_type::NONE, 0);
    let mut seen = std::collections::BTreeSet::new();
    for t in [flow_type::BRANCH, flow_type::CALL, flow_type::CALL_RETURN, flow_type::RETURN] {
        assert!((1..=4).contains(&t));
        assert!(seen.insert(t), "flow types must be distinct");
    }
}

// -----------------------------------------------------------------------------
// 5. The per-function Override store round-trip FlowInfo reads at flow time.
//    insert_flow_override / get_flow_override / has_flow_override + the miss path.
//    (override.cc:148,233; override.hh:84)
// -----------------------------------------------------------------------------
#[test]
fn w10_override_store_flow_roundtrip_and_miss() {
    let m = build_manager();
    let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
    let a1 = Address::new(Rc::clone(&ram), 0x10007d);
    let a2 = Address::new(Rc::clone(&ram), 0x100099);
    let mut ov = Override::new();
    // Empty store: hasFlowOverride is false, any lookup is NONE (FlowInfo's
    // flow.cc:43 `flowoverride_present` guard depends on exactly this).
    assert!(!ov.has_flow_override());
    assert_eq!(ov.get_flow_override(&a1), flow_type::NONE);
    // Insert one override; the presence flag flips and the value round-trips.
    ov.insert_flow_override(a1.clone(), flow_type::CALL_RETURN);
    assert!(ov.has_flow_override());
    assert_eq!(ov.get_flow_override(&a1), flow_type::CALL_RETURN);
    // A DIFFERENT address still misses (NONE), proving the map is keyed by the
    // exact address, not a blanket "any override present -> apply everywhere".
    assert_eq!(ov.get_flow_override(&a2), flow_type::NONE);
    // Re-insert at the same address overwrites (map insert), like the C++.
    ov.insert_flow_override(a1.clone(), flow_type::RETURN);
    assert_eq!(ov.get_flow_override(&a1), flow_type::RETURN);
}

// A flow override re-seeded onto a FRESH Funcdata (the console rebuild model:
// build_and_follow_flow_with_override re-inserts the stashed facts) is visible
// through the Funcdata's own getOverride — the exact handle FlowInfo reads.
#[test]
fn w10_funcdata_override_reseed_visible_through_getoverride() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x10007d);
    assert!(!fd.get_override().has_flow_override());
    // Mirror the build_and_follow_flow_with_override re-seed.
    fd.get_override_mut().insert_flow_override(addr.clone(), flow_type::CALL_RETURN);
    assert!(fd.get_override().has_flow_override());
    assert_eq!(fd.get_override().get_flow_override(&addr), flow_type::CALL_RETURN);
}

// -----------------------------------------------------------------------------
// 6. Call-fixup registration is NAME-keyed (no address/value/function hardcode).
//    A fixup registered under a name resolves by THAT name; an unknown name is
//    -1; the CALLOTHERFIXUP namespace does not leak into the CALLFIXUP lookup.
//    (pcodeinject.cc:285-309; inject_sleigh.cc manual_call_fixup/register_inject)
// -----------------------------------------------------------------------------
#[test]
fn w10_callfixup_getpayloadid_is_name_keyed_no_special_casing() {
    // Exercise the EXACT map the cspec decode populates: register_inject (which
    // `decode_call_fixups` reaches) calls `base.register_call_fixup(name, id)`
    // with the name decoded from `<callfixup name="...">`, and `IfcFixupApply`
    // reads it back via `base.get_payload_id(CALLFIXUP_TYPE, name)`.
    let mut lib = PcodeInjectLibrarySleigh::new(0);
    // Before registration, the name is unknown -> -1 (the IfcFixupApply
    // "Unknown fixup" path).
    assert_eq!(lib.base.get_payload_id(CALLFIXUP_TYPE, b"guard_dispatch_icall"), -1);
    // Register two DISTINCT call-fixup names (the ids are the allocate_inject
    // indices the real decode path hands in).
    lib.base.register_call_fixup(b"guard_dispatch_icall", 0).unwrap();
    lib.base.register_call_fixup(b"alloca_probe", 1).unwrap();
    // Each name resolves to ITS OWN id — the lookup keys on the name, not a fixed
    // slot or a hardcoded function string.
    assert_eq!(lib.base.get_payload_id(CALLFIXUP_TYPE, b"guard_dispatch_icall"), 0);
    assert_eq!(lib.base.get_payload_id(CALLFIXUP_TYPE, b"alloca_probe"), 1);
    // A name that was never registered is still -1 (not silently the last id).
    assert_eq!(lib.base.get_payload_id(CALLFIXUP_TYPE, b"never_registered"), -1);
    // The same name under the CALLOTHERFIXUP namespace is -1: namespaces are
    // separate maps (pcodeinject.cc:289-308), no cross-leak.
    assert_eq!(lib.base.get_payload_id(CALLOTHERFIXUP_TYPE, b"guard_dispatch_icall"), -1);
    // The round-trip name read back from the id matches (getCallFixupName).
    assert_eq!(lib.base.get_call_fixup_name(0), b"guard_dispatch_icall".to_vec());
    // Re-registering the SAME name is a hard error (C++ would assert a duplicate);
    // the recovery never silently overwrites a fixup name -> id mapping.
    assert!(lib.base.register_call_fixup(b"guard_dispatch_icall", 2).is_err());
}
