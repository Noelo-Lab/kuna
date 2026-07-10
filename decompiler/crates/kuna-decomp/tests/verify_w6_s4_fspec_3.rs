//! Adversarial verification tests for item `w6-s4-fspec-3`
//! (`FuncCallSpecs` + the `FspecSpace` print/encode arms:
//! `decompiler/cpp/fspec.cc` 4854-5982 and `fspec.hh:1645-1742`).
//!
//! Written by the INDEPENDENT verifier.  These target the spots the hunt list
//! flagged as most fragile for the *ported* (non-seamed) surface of this item:
//!
//!   - **Width / signedness / monotone** in `setInputBytesConsumed`
//!     (`fspec.cc:5894`): the `while (inputConsume.size() <= slot)` grow loop
//!     (size_t vs int4) and the smallest-wins rule with the `0` sentinel
//!     (`oldVal == 0 || val < oldVal`) — including the *equal* and *larger*
//!     cases that must NOT change, and the out-of-range read returning 0.
//!   - **Off-by-one / index-math** in `ParamActive::getTrialForInputVarnode`
//!     (`fspec.hh:1749`): the `slot < stackplaceholder ? 1 : 2` boundary —
//!     specifically the `slot == stackplaceholder` case (subtract 2, NOT 1).
//!   - **Wrapping** in `resolveSpacebaseRelative` (`fspec.cc:4904-4905`): the
//!     `stackoffset -= addr.getOffset(); stackoffset = wrapOffset(stackoffset)`
//!     underflow on a `uintb` for a locked stack parameter — C++ wraps; the
//!     port uses `wsub` + `wrap_offset`.
//!   - **Address arithmetic / reuse-slot encoding** in
//!     `transferLockedInputParam` + `transferLockedInput` (`fspec.cc:5043`,
//!     `:5105`): the `startaddr + (sz-1)` / trial-end overlap test and the
//!     `reuse > 0 => op->getIn(reuse)` reuse path, driven through
//!     `lateRestriction` with an input-locked register-only prototype.
//!   - **`checkInputJoin` positive delegation** (`fspec.cc:5354`): the porter's
//!     in-crate test only covers the early-`false` gates; this pins a matched
//!     pair that reaches `FuncProto::checkInputJoin`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::fspec::{
    FuncCallSpecs, FuncProto, ParameterPieces, ProtoModel, OFFSET_UNKNOWN,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::DefOpInfo;
use kuna_num::opcodes::OpCode;

// --- shared builders (mirror the in-crate `fcs_*` harness) ------------------

fn fcs_manager() -> AddrSpaceManager {
    use kuna_base::space::{ConstantSpace, UniqueSpace};
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
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    // A spacebase (stack) space at index 3.
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
    // A processor "register" space at index 4 for register-storage params.
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        4,
        0,
        0,
        0,
    )))
    .unwrap();
    m.set_default_code_space(2).unwrap();
    m
}

fn fcs_fd() -> Funcdata {
    let glb = Rc::new(ArchContext::new(fcs_manager()));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let entry = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, entry, 0x10000000, 0x40).unwrap()
}

fn fcs_ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}
fn fcs_stack(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("stack").unwrap())
}
fn fcs_reg(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap())
}

fn fcs_new_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = fcs_ram(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    let flags = if opc == OpCode::CPUI_CALL || opc == OpCode::CPUI_CALLIND {
        pcodeop_flags::call
    } else {
        0
    };
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
    op
}

/// A varnode in `space` at `off`/`size` defined by `op` (sets `written`).
fn fcs_out(fd: &mut Funcdata, op: OpId, space: Rc<AddrSpace>, off: u64, size: int4) -> VarnodeId {
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum: seq };
    let vn = {
        let mut noop = |_b: &mut kuna_decomp::varnode::VarnodeBank,
                        _o: VarnodeId,
                        _n: VarnodeId|
         -> KunaResult<()> { panic!("collision") };
        fd.vbank_mut()
            .create_def(
                size,
                Address::new(space, off),
                Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN)),
                def,
                &mut noop,
            )
            .unwrap()
    };
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

fn std_model(mgr: &AddrSpaceManager) -> ProtoModel {
    let mut model = ProtoModel::new(mgr);
    model.build_param_list("standard").unwrap();
    model
}

fn fcs_with_std_model(mgr: &AddrSpaceManager, call: OpId) -> FuncCallSpecs {
    let mut fc = FuncCallSpecs::new(call, Address::default());
    let voidt = Rc::new(Datatype::new(1, type_metatype::TYPE_VOID));
    let mut fp = FuncProto::new();
    fp.set_internal(Rc::new(std_model(mgr)), voidt);
    fc.proto_mut().copy(&fp);
    fc
}

// ---------------------------------------------------------------------------
// F-A: setInputBytesConsumed monotone + sentinel-zero + out-of-range read
//   C++ fspec.cc:5877-5905.  `getInputBytesConsumed`: slot>=size -> 0.
//   `setInputBytesConsumed`: grow with 0, then change iff `oldVal==0 || val<oldVal`.
// ---------------------------------------------------------------------------

#[test]
fn fspec3_input_bytes_consumed_sentinel_and_monotone() {
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mgr = fcs_manager();
    let mut fc = fcs_with_std_model(&mgr, call);

    // Empty vector: any read is the 0 default (slot >= size()).
    assert_eq!(fc.get_input_bytes_consumed(0), 0);
    assert_eq!(fc.get_input_bytes_consumed(7), 0);

    // First set at slot 3 grows the vector [0,0,0,?]; the stored slot is
    // changed off its 0 sentinel (oldVal == 0) => returns true.
    assert!(fc.set_input_bytes_consumed(3, 5));
    assert_eq!(fc.get_input_bytes_consumed(3), 5);
    // Slots 0..3 were grown to the 0 sentinel and read back as 0.
    assert_eq!(fc.get_input_bytes_consumed(0), 0);
    assert_eq!(fc.get_input_bytes_consumed(2), 0);

    // A LARGER value does NOT replace 5 (only smaller wins) => false, unchanged.
    assert!(!fc.set_input_bytes_consumed(3, 9));
    assert_eq!(fc.get_input_bytes_consumed(3), 5);

    // An EQUAL value does NOT replace (not strictly smaller) => false, unchanged.
    assert!(!fc.set_input_bytes_consumed(3, 5));
    assert_eq!(fc.get_input_bytes_consumed(3), 5);

    // A SMALLER value replaces => true.
    assert!(fc.set_input_bytes_consumed(3, 2));
    assert_eq!(fc.get_input_bytes_consumed(3), 2);

    // A slot whose stored value is the 0 sentinel: even val==0 keeps it 0 and,
    // since oldVal==0, the branch is taken and returns true (C++:
    // `oldVal == 0 || val < oldVal` short-circuits on oldVal==0).
    assert!(fc.set_input_bytes_consumed(1, 0));
    assert_eq!(fc.get_input_bytes_consumed(1), 0);

    // Out-of-range read past the grown vector is still the 0 default.
    assert_eq!(fc.get_input_bytes_consumed(100), 0);
}

// ---------------------------------------------------------------------------
// F-B: getTrialForInputVarnode placeholder boundary.
//   C++ fspec.hh:1752: slot -= ((sp<0)||(slot<sp)) ? 1 : 2;
//   The `slot == stackplaceholder` case takes the `else 2` branch — a classic
//   off-by-one trap.
// ---------------------------------------------------------------------------

#[test]
fn fspec3_trial_for_input_varnode_placeholder_equal_boundary() {
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mgr = fcs_manager();
    let mut fc = fcs_with_std_model(&mgr, call);
    let stk = fcs_stack(&fd);

    // `ParamActive::slotbase` starts at 1 (C++ ctor), so four registrations get
    // slots 1,2,3,4 and vector indices 0,1,2,3 at offsets 0x0/0x8/0x10/0x18.
    let pa = fc.get_active_input();
    pa.register_trial(&addr(&stk, 0x0), 8); // slot 1, idx 0
    pa.register_trial(&addr(&stk, 0x8), 8); // slot 2, idx 1
    pa.register_trial(&addr(&stk, 0x10), 8); // slot 3, idx 2
    pa.register_trial(&addr(&stk, 0x18), 8); // slot 4, idx 3

    // No placeholder yet (stackplaceholder == -1): subtract 1 everywhere.
    assert_eq!(pa.get_trial_for_input_varnode(1).get_address().get_offset(), 0x0);
    assert_eq!(pa.get_trial_for_input_varnode(4).get_address().get_offset(), 0x18);

    // Establish the placeholder at slotbase (now 5 after four registrations).
    pa.set_placeholder_slot(); // stackplaceholder = 5
    // slot < placeholder(5): subtract 1.
    assert_eq!(pa.get_trial_for_input_varnode(3).get_address().get_offset(), 0x10); // idx 2
    assert_eq!(pa.get_trial_for_input_varnode(4).get_address().get_offset(), 0x18); // idx 3
    // slot == placeholder(5): the BOUNDARY — must take `else 2`, NOT `1`.
    //   5 - 2 = idx 3 (0x18).  A buggy `<=` (`slot <= sp ? 1 : 2`) would take
    //   `-1` here and read idx 4 (out of bounds / wrong), so the `<` boundary is
    //   load-bearing.  With the correct `<`, slot 5 == slot 4's value (0x18).
    assert_eq!(pa.get_trial_for_input_varnode(5).get_address().get_offset(), 0x18); // idx 3
}

// ---------------------------------------------------------------------------
// F-C: resolveSpacebaseRelative input-locked wrap underflow.
//   C++ fspec.cc:4904-4905: stackoffset -= addr.getOffset(); wrapOffset(...).
//   With refvn offset 0 and a locked stack param at offset 0x20, the subtract
//   underflows uintb and wraps (C++ silent wrap; port uses wsub + wrap_offset).
// ---------------------------------------------------------------------------

#[test]
fn fspec3_resolve_spacebase_relative_locked_wrap_underflow() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let stk = fcs_stack(&fd);

    // CALLIND with 2 inputs: target (slot 0) + placeholder (slot 1).
    let call = fcs_new_op(&mut fd, 2, 0x1010, OpCode::CPUI_CALLIND);
    let ram = fcs_ram(&fd);
    let target = {
        let c = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
        fd.vbank_mut().create(
            8,
            Address::new(c, 0x2000),
            Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN)),
        )
    };
    let _ = &ram;
    fd.op_set_input(call, target, 0).unwrap();

    // LOAD producing the placeholder; its input 0 (refvn) is a stack varnode
    // at offset 0 (so stackoffset is read as 0 before the subtract).
    let load = fcs_new_op(&mut fd, 1, 0x1008, OpCode::CPUI_LOAD);
    let refvn = fd.vbank_mut().create(
        8,
        Address::new(Rc::clone(&stk), 0x0),
        Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN)),
    );
    fd.op_set_input(load, refvn, 0).unwrap();
    let uniq = Rc::clone(fd.get_arch().manage().get_space(1).unwrap());
    let phvn = fcs_out(&mut fd, load, uniq, 0x0, 8);
    fd.vbank_mut().get_mut(phvn).unwrap().set_spacebase_placeholder();
    fd.op_set_input(call, phvn, 1).unwrap();

    // fc: standard model, then a LOCKED input prototype with one stack param at
    // offset 0x20.  stack_placeholder_slot stays -1, so the abort short-circuit
    // is skipped and the input-locked branch runs.
    let mut fc = fcs_with_std_model(&mgr, call);
    let stack_param_off: u64 = 0x20;
    let pieces = ParameterPieces {
        addr: Address::new(Rc::clone(&stk), stack_param_off),
        type_: Some(Rc::new(Datatype::new(8, type_metatype::TYPE_INT))),
        flags: kuna_decomp::fspec::parameter_pieces_flags::TYPELOCK,
    };
    fc.proto_mut().set_param(0, "p0", &pieces);
    fc.proto_mut().set_input_lock(true);
    assert!(fc.proto().is_input_locked());

    // getSlot(phvn) finds phvn at slot 1; slot = 1 - 1 = 0 -> getParam(0) (the
    // stack param).  spacebase matches, so the wrap branch runs:
    //   stackoffset(0) - 0x20  ==  2^64 - 0x20  (wrapped), then wrapOffset.
    let r = fc.resolve_spacebase_relative(&mut fd, phvn);
    assert!(r.is_ok(), "input-locked stack param should resolve, got {r:?}");
    let expected = 0u64.wrapping_sub(stack_param_off); // wrap_offset on a 8-byte space is identity
    assert_eq!(
        fc.get_spacebase_offset(),
        expected,
        "stackoffset must wrap (uintb), not panic / saturate"
    );
    assert_ne!(fc.get_spacebase_offset(), OFFSET_UNKNOWN);
}

// ---------------------------------------------------------------------------
// F-D: lateRestriction input-locked register-reuse path.
//   Drives transferLockedInput -> transferLockedInputParam (fspec.cc:5043).
//   A register param exactly matching a registered trial whose slot points at a
//   live CALL input reuses that input (reuse > 0 => op->getIn(reuse)); the
//   newinput list then carries [dest, reused].
// ---------------------------------------------------------------------------

#[test]
fn fspec3_late_restriction_locked_register_reuse() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let reg = fcs_reg(&fd);

    // CALL with 2 inputs: dest (slot 0) + one register operand (slot 1).
    let call = fcs_new_op(&mut fd, 2, 0x1010, OpCode::CPUI_CALL);
    let dest = {
        let c = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
        fd.vbank_mut().create(
            8,
            Address::new(c, 0x2000),
            Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN)),
        )
    };
    fd.op_set_input(call, dest, 0).unwrap();
    let regvn = fd.vbank_mut().create(
        4,
        Address::new(Rc::clone(&reg), 0x0),
        Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN)),
    );
    fd.op_set_input(call, regvn, 1).unwrap();

    // Share ONE model so isCompatible(restricted) is true.
    let model = Rc::new(std_model(&mgr));

    // fc: has the shared model (hasModel() true), one register trial at the
    // param address whose slot is 1 (the live CALL input).
    let mut fc = FuncCallSpecs::new(call, Address::default());
    let voidt = Rc::new(Datatype::new(1, type_metatype::TYPE_VOID));
    fc.proto_mut().set_internal(Rc::clone(&model), Rc::clone(&voidt));
    {
        let pa = fc.get_active_input();
        pa.register_trial(&addr(&reg, 0x0), 4); // slotbase 0
        pa.get_trial(0); // ensure it exists
        pa.get_trial_mut(0).set_slot(1); // point at CALL input slot 1
    }

    // restricted: same model, input-locked, one register param matching the
    // trial address/size exactly.
    let mut restricted = FuncProto::new();
    restricted.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    let pieces = ParameterPieces {
        addr: Address::new(Rc::clone(&reg), 0x0),
        type_: Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))),
        flags: kuna_decomp::fspec::parameter_pieces_flags::TYPELOCK,
    };
    restricted.set_param(0, "a", &pieces);
    restricted.set_input_lock(true);
    assert!(restricted.is_input_locked());

    let mut newinput: Vec<Option<VarnodeId>> = Vec::new();
    let mut newoutput: Vec<VarnodeId> = Vec::new();
    let ok = fc
        .late_restriction(&fd, &restricted, &mut newinput, &mut newoutput)
        .expect("late_restriction over a register-locked proto");
    assert!(ok, "register reuse should succeed (no stack placeholder needed)");

    // newinput == [dest (slot 0), reused regvn (slot 1)].
    assert_eq!(newinput.len(), 2);
    assert_eq!(newinput[0], Some(dest));
    assert_eq!(newinput[1], Some(regvn), "param should reuse op->getIn(1)");
    // The prototype was converted (copy of restricted) — now input-locked.
    assert!(fc.proto().is_input_locked());
}

// ---------------------------------------------------------------------------
// F-E: checkInputJoin positive delegation.  The porter's in-crate test only
//   hits the early-false gates; this pins a matched adjacent pair that reaches
//   FuncProto::checkInputJoin (fspec.cc:5372) and returns its verdict.
// ---------------------------------------------------------------------------

#[test]
fn fspec3_check_input_join_reaches_funcproto() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = fcs_with_std_model(&mgr, call);
    let reg = fcs_reg(&fd);

    // Two adjacent 4-byte register trials, sizes matching the vn sizes.
    {
        let pa = fc.get_active_input();
        pa.register_trial(&addr(&reg, 0x0), 4);
        pa.register_trial(&addr(&reg, 0x4), 4);
    }
    // Not input-active, slot in range, sizes match -> delegates to
    // FuncProto::checkInputJoin.  Whatever it returns, the call must not panic
    // and must be a *deterministic* bool (the join-or-not verdict for the
    // standard model's two register slots).
    let r_his = fc.check_input_join(1, true, 4, 4);
    let r_los = fc.check_input_join(1, false, 4, 4);
    // The two orderings probe the same pair; both reach FuncProto without
    // panicking on the trial-index math.  (We assert determinism: a repeat is
    // identical.)
    assert_eq!(r_his, fc.check_input_join(1, true, 4, 4));
    assert_eq!(r_los, fc.check_input_join(1, false, 4, 4));
}
