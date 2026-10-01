//! Logic-level tests for `kuna_is_tail_call_branch`, exercising the tail-call
//! recognition decision on hand-built IR.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::context::{ArchContext, OpId, TypeOp};

use super::*;

/// const(0), unique(1), ram(2, IPTR_PROCESSOR).
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
    let glb = Rc::new(ArchContext::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn unk_type(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn proc_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

/// Build an op of `opc` with input-0 set to a code-ref-like varnode in the
/// processor space at `off`.  Returns the op id.
fn build_branch_op(fd: &mut Funcdata, opc: OpCode, off: u64) -> OpId {
    let pc = Address::new(proc_space(fd), 0x1000);
    let op = fd.obank_mut().create_at(1, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, "op"));
    let ram = proc_space(fd);
    let vn = fd
        .vbank_mut()
        .create(8, Address::new(ram, off), unk_type(8));
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
    op
}

#[test]
fn gate_off_returns_false() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    // gate=false: default-pipeline byte-identical, never fires.
    assert!(!kuna_is_tail_call_branch(&fd, op, false, true, false));
}

#[test]
fn direct_branch_to_known_function_fires() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    // The canonical `jmp setlocale@plt` tail-jump shape: a direct BRANCH whose
    // target is another known function's entry.
    assert!(kuna_is_tail_call_branch(&fd, op, true, true, false));
}

/// Without an applied `flow ... branch` override the selection order is
/// unchanged: `tailcalljump` claims first and `tailcallframe` is not consulted.
/// The flow-level near misses (a refused branch fact, an applied one at another
/// instruction) are end-to-end in `kuna-cli/tests/explicit_branch_assertion_cli.rs`.
#[test]
fn without_a_branch_override_tailcalljump_claims_first() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    let frame_consulted = std::cell::Cell::new(false);
    assert_eq!(
        crate::flow::select_inferred_tail_call(
            false,
            || kuna_is_tail_call_branch(&fd, op, true, true, false),
            || {
                frame_consulted.set(true);
                true
            },
        ),
        Some("tailcalljump")
    );
    assert!(!frame_consulted.get(), "tailcallframe was consulted after tailcalljump claimed");
}

#[test]
fn target_not_a_known_function_returns_false() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x1100);
    // An ordinary intraprocedural jump targets a mid-function address (no
    // function entry there) -> not a tail call.
    assert!(!kuna_is_tail_call_branch(&fd, op, true, false, false));
}

#[test]
fn self_tail_recursion_excluded() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x1000);
    // dest == the current function's own entry: left as an ordinary back-edge.
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, true));
}

#[test]
fn indirect_branch_returns_false() {
    let mut fd = build_fd();
    // A BRANCHIND (indirect jump) is not a direct tail-call jmp.
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x18d0);
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, false));
}

#[test]
fn indirect_branch_with_a_sole_foreign_destination_is_a_tail_call() {
    let mut fd = build_fd();
    let op = build_branch_op(&mut fd, OpCode::CPUI_BRANCHIND, 0x18d0);
    let yes = || true;
    let no = || false;
    assert!(kuna_is_tail_call_table(&fd, op, true, true, false, false, yes), "locked");
    assert!(kuna_is_tail_call_table(&fd, op, true, true, false, true, no), "out of extent");
    assert!(
        !kuna_is_tail_call_table(&fd, op, true, true, false, false, no),
        "no prototype, inside the extent"
    );
    assert!(!kuna_is_tail_call_table(&fd, op, false, true, false, true, yes), "gate off");
    assert!(!kuna_is_tail_call_table(&fd, op, true, false, false, true, yes), "not a function");
    assert!(!kuna_is_tail_call_table(&fd, op, true, true, true, true, yes), "its own entry");
    let direct = build_branch_op(&mut fd, OpCode::CPUI_BRANCH, 0x18d0);
    assert!(!kuna_is_tail_call_table(&fd, direct, true, true, false, true, yes), "direct jump");
    let asked = std::cell::Cell::new(false);
    kuna_is_tail_call_table(&fd, op, false, true, false, false, || {
        asked.set(true);
        true
    });
    assert!(!asked.get(), "a closed gate never looks the callee up");
}

#[test]
fn parked_pieces_state_the_call_unless_they_only_map_the_return() {
    use crate::fspec::{ParameterPieces, PrototypePieces};
    let int = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let void = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let stated = |returns_value| Some(StatedCallee { returns_value });
    assert_eq!(stated_by_pieces(None), None);
    assert_eq!(stated_by_pieces(Some(&PrototypePieces::default())), stated(false), "f(void)");
    let int_f = PrototypePieces { outtype: Some(int.clone()), ..Default::default() };
    assert_eq!(stated_by_pieces(Some(&int_f)), stated(true), "int f(void)");
    let void_f = PrototypePieces { outtype: Some(void), intypes: vec![int], ..Default::default() };
    assert_eq!(stated_by_pieces(Some(&void_f)), stated(false), "void f(int)");
    let return_only =
        PrototypePieces { output_storage: Some(ParameterPieces::default()), ..Default::default() };
    assert_eq!(stated_by_pieces(Some(&return_only)), None, "map return");
}

#[test]
fn a_tail_call_returning_a_value_needs_the_callers_output_stated() {
    let stated = |returns_value| Some(StatedCallee { returns_value });
    assert!(tail_call_states_the_body(stated(true), true));
    assert!(!tail_call_states_the_body(stated(true), false), "would print void");
    assert!(tail_call_states_the_body(stated(false), false), "returns nothing");
    assert!(!tail_call_states_the_body(None, true), "no callee prototype");
}

#[test]
fn tailcalljump_values_split_direct_from_computed_jumps() {
    let gates = |v| tail_call_mode(v).map(|(jumps, tables, _)| (jumps, tables)).unwrap();
    assert_eq!(gates("on"), (true, true));
    assert_eq!(gates("direct"), (true, false));
    assert_eq!(gates("off"), (false, false));
    assert!(tail_call_mode("tables").is_err());
}

#[test]
fn a_table_has_a_sole_destination_only_when_every_entry_agrees() {
    let fd = build_fd();
    let at = |off| Address::new(proc_space(&fd), off);
    assert_eq!(kuna_sole_destination(false, [at(0x18d0)]), Some(at(0x18d0)));
    assert_eq!(kuna_sole_destination(false, [at(0x18d0), at(0x18d0)]), Some(at(0x18d0)));
    assert_eq!(kuna_sole_destination(false, [at(0x18d0), at(0x18e0)]), None);
    assert_eq!(kuna_sole_destination(false, []), None);
    assert_eq!(kuna_sole_destination(true, [at(0x18d0)]), None, "an override stays a table");
}

#[test]
fn call_returns_false() {
    let mut fd = build_fd();
    // A real CALL is already a call, not a jump to recover.
    let op = build_branch_op(&mut fd, OpCode::CPUI_CALL, 0x18d0);
    assert!(!kuna_is_tail_call_branch(&fd, op, true, true, false));
}

#[test]
fn option_default_is_off_and_apply_flips() {
    // Shipped default: option tailcalljump off (kept opt-in; default-on regresses
    // 2 datatests, Long double #1/#2).
    let mut opt = TailCallJumpOption::default();
    assert!(!opt.is_enabled());
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}
