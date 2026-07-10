//! Independent verifier adversarial tests for item `w3-ir-funcdata-varnode`
//! (`decompiler/cpp/funcdata_varnode.cc`).
//!
//! Re-derived from the C++ oracle (NOT the Rust port). Targets the hunt-list
//! spots flagged most fragile for this item:
//!
//!   - `find_input_overlap` / `last_input_before` — the C++ `setInputVarnode`
//!     overlap pre-check (`funcdata_varnode.cc:349-363`):
//!         iter = vbank.beginDef(input, vn->getAddr()+vn->getSize());
//!         if (iter != vbank.beginDef()) { --iter; invn = *iter; ... }
//!     The `--iter` step-back idiom over the def tree, where inputs are the
//!     contiguous prefix (def-tree class order input<written<free). Boundary
//!     cases: probe at exactly an input start, multiple inputs at distinct
//!     addresses, inputs present + a written varnode interleaved, the
//!     overlapping-but-not-identical error, and the no-input early return.
//!
//!   - `total_replace` with ONE op reading `vn` in TWO slots
//!     (`funcdata_varnode.cc:1495`): the descend list holds the op twice (one
//!     entry per read, `addDescend` pushes once per `opSetInput`), `getSlot`
//!     returns the FIRST still-matching slot, and `eraseDescend` removes the
//!     FIRST matching entry. Two iterations must repoint slot 0 then slot 1.
//!
//!   - `destroy_varnode` (`funcdata_varnode.cc:274`) on a WRITTEN varnode (def
//!     teardown) and on an INPUT varnode (no def): every reader's slot becomes
//!     null, the def op's output is cleared, the varnode is freed.
//!
//! Each assertion cites the C++ line it is derived from.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace,
    UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

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

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
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

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
}

/// Promote a fresh free varnode at (off,size) to a formal input (bypasses the
/// xref split-borrow `setInputVarnode` needs — see funcdata_varnode.rs module doc).
fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), None);
    fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
}

/// A written varnode (output of a fresh COPY op) so it can carry multiple
/// descendants (free non-spacebase varnodes are limited to one).
fn make_written(fd: &mut Funcdata, off: u64, size: int4) -> (VarnodeId, OpId) {
    let r = ram(fd);
    let pc = Address::new(Rc::clone(&r), off);
    let defop = fd.obank_mut().create_at(1, pc);
    fd.obank_mut().change_opcode(defop, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
    let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: defop, seqnum: seq };
    let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
    fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
    (vn, defop)
}

/// Wire `vn` as input `slot` of `op` (descend link + op input), as the op-graph
/// would maintain.
fn wire_read(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
    fd.vbank_mut().add_descend(vn, op).unwrap();
    fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
}

// ===========================================================================
// V1: find_input_overlap / last_input_before --iter step-back boundaries.
//
// C++ setInputVarnode overlap pre-check (funcdata_varnode.cc:349-363):
//   iter = vbank.beginDef(input, vn->getAddr()+vn->getSize()); // lower_bound
//   if (iter != vbank.beginDef()) {  // beginDef() == start of WHOLE def tree
//     --iter; invn = *iter;          // previous input
//     if (invn->isInput()) {
//       if (overlap) { if (size==size && addr==addr) return invn; throw; }
//     }
//   }
// Inputs are the def-tree prefix; --iter from a probe in the input region
// yields the input with the greatest (addr,size) strictly below the probe.
// ===========================================================================

#[test]
fn find_input_overlap_picks_immediately_preceding_input() {
    // Three inputs at distinct addresses. A candidate exactly matching the
    // MIDDLE input must resolve to that middle input (the immediately preceding
    // element of beginDef(input, mid+size)), not the first or last.
    let mut fd = build_fd();
    let r = ram(&fd);
    let _lo = make_input(&mut fd, 0x100, 4);
    let mid = make_input(&mut fd, 0x200, 4);
    let _hi = make_input(&mut fd, 0x300, 4);

    let cand = fd.new_varnode(4, &Address::new(r, 0x200), None);
    // C++: --iter lands on the input at 0x200 (same addr+size) -> identical ->
    // return invn (funcdata_varnode.cc:358-359).
    assert_eq!(
        fd.find_input_overlap(cand).unwrap(),
        Some(mid),
        "candidate must resolve to the exactly-matching middle input"
    );
}

#[test]
fn find_input_overlap_no_inputs_but_written_present_returns_none() {
    // No inputs exist, but a written varnode does. C++: beginDef(input, probe)
    // lower_bound lands at the start of the written region == beginDef() (whole
    // tree start), so `iter == beginDef()` and the overlap branch is skipped ->
    // the candidate is free to become the input (Ok(None)).
    let mut fd = build_fd();
    let r = ram(&fd);
    let (_w, _d) = make_written(&mut fd, 0x40, 4); // a written varnode, no inputs
    let cand = fd.new_varnode(4, &Address::new(r, 0x40), None);
    assert_eq!(
        fd.find_input_overlap(cand).unwrap(),
        None,
        "with no inputs the --iter guard (iter != beginDef) is false -> no overlap"
    );
}

#[test]
fn find_input_overlap_probe_at_first_input_start_is_none() {
    // Candidate strictly BELOW the only input. probe = cand.addr+size lands at
    // or below the first input start, so beginDef(input, probe) == beginDef()
    // and the step-back is guarded out -> Ok(None).
    let mut fd = build_fd();
    let r = ram(&fd);
    let _inp = make_input(&mut fd, 0x200, 4);
    // candidate [0x100,0x104) ; probe = 0x104 <= 0x200 (first input start).
    let cand = fd.new_varnode(4, &Address::new(r, 0x100), None);
    assert_eq!(
        fd.find_input_overlap(cand).unwrap(),
        None,
        "probe at/below first input start: --iter guarded out"
    );
}

#[test]
fn find_input_overlap_overlapping_not_identical_errors() {
    // An input [0x100,0x104). A candidate [0x101,0x103) overlaps it but differs
    // in (addr,size). C++: overlap true, size/addr mismatch -> throw
    // LowlevelError("Overlapping input varnodes") (funcdata_varnode.cc:360).
    let mut fd = build_fd();
    let r = ram(&fd);
    let _inp = make_input(&mut fd, 0x100, 4);
    let cand = fd.new_varnode(2, &Address::new(r, 0x101), None);
    assert!(
        fd.find_input_overlap(cand).is_err(),
        "overlapping-but-not-identical candidate must error"
    );
}

#[test]
fn find_input_overlap_multiple_inputs_same_addr_picks_largest() {
    // Two inputs at the SAME address, sizes 2 and 4. The def tree (input class)
    // sorts by (addr, size); --iter from a probe above both picks the LAST in
    // iteration order == the largest (size 4). A size-4 candidate at that addr
    // resolves to the size-4 input (identical), not the size-2 one.
    let mut fd = build_fd();
    let r = ram(&fd);
    let small = make_input(&mut fd, 0x100, 2);
    let big = make_input(&mut fd, 0x100, 4);
    // Candidate matching the size-4 input. probe = 0x100 + 4 = 0x104. --iter
    // steps back over {(0x100,2),(0x100,4)} to (0x100,4) == big.
    let cand = fd.new_varnode(4, &Address::new(r, 0x100), None);
    assert_eq!(
        fd.find_input_overlap(cand).unwrap(),
        Some(big),
        "--iter must land on the largest (addr,size) input below the probe"
    );
    assert_ne!(fd.find_input_overlap(cand).unwrap(), Some(small));
}

// ===========================================================================
// V2: total_replace with ONE op reading vn in TWO slots.
//
// C++ totalReplace (funcdata_varnode.cc:1495):
//   iter = vn->beginDescend();
//   while (iter != vn->endDescend()) { op = *iter++; i = op->getSlot(vn);
//                                      opSetInput(op,newvn,i); }
// The descend list holds the op twice (one entry per read). getSlot returns the
// FIRST slot still reading vn; eraseDescend removes the FIRST matching entry.
// After both iterations BOTH slots must read newvn, vn loses both descends, and
// newvn gains both.
// ===========================================================================

#[test]
fn total_replace_double_reader_repoints_both_slots_in_order() {
    let mut fd = build_fd();
    let r = ram(&fd);
    let (vn, _d1) = make_written(&mut fd, 0x10, 4);
    let (newvn, _d2) = make_written(&mut fd, 0x20, 4);

    // One op reading vn in BOTH input slots.
    let pc = Address::new(r, 0x1000);
    let op = fd.obank_mut().create_at(2, pc);
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
    wire_read(&mut fd, vn, op, 0);
    wire_read(&mut fd, vn, op, 1);
    assert_eq!(fd.vbank().get(vn).unwrap().num_descend(), 2, "vn read twice (one descend per read)");

    fd.total_replace(vn, newvn).unwrap();

    // Both slots now read newvn (C++ funcdata_varnode.cc:1506 per iteration).
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(newvn), "slot 0 -> newvn");
    assert_eq!(fd.obank().get(op).unwrap().get_in(1), Some(newvn), "slot 1 -> newvn");
    // vn lost both descend links; newvn gained both.
    assert_eq!(fd.vbank().get(vn).unwrap().num_descend(), 0, "vn fully unhooked");
    assert_eq!(fd.vbank().get(newvn).unwrap().num_descend(), 2, "newvn gains both reads");
}

#[test]
fn total_replace_empty_descend_is_noop() {
    // vn with zero descendants: the while loop never executes; newvn unchanged.
    let mut fd = build_fd();
    let (vn, _d1) = make_written(&mut fd, 0x10, 4);
    let (newvn, _d2) = make_written(&mut fd, 0x20, 4);
    assert_eq!(fd.vbank().get(vn).unwrap().num_descend(), 0);
    fd.total_replace(vn, newvn).unwrap();
    assert_eq!(fd.vbank().get(newvn).unwrap().num_descend(), 0, "no readers -> newvn untouched");
}

// ===========================================================================
// V3: destroy_varnode def/descend teardown.
//
// C++ destroyVarnode (funcdata_varnode.cc:274):
//   for (op : vn->descend) op->clearInput(op->getSlot(vn));
//   if (vn->def) { vn->def->setOutput(0); vn->def = 0; }
//   vn->destroyDescend(); vbank.destroy(vn);
// ===========================================================================

#[test]
fn destroy_varnode_written_clears_def_output_and_readers() {
    let mut fd = build_fd();
    let r = ram(&fd);
    // A WRITTEN varnode (output of defop) read by two ops.
    let (vn, defop) = make_written(&mut fd, 0x10, 4);
    let r0 = fd.obank_mut().create_at(1, Address::new(Rc::clone(&r), 0x1000));
    fd.obank_mut().change_opcode(r0, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let r1 = fd.obank_mut().create_at(1, Address::new(r, 0x1004));
    fd.obank_mut().change_opcode(r1, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    wire_read(&mut fd, vn, r0, 0);
    wire_read(&mut fd, vn, r1, 0);
    assert_eq!(fd.obank().get(defop).unwrap().get_out(), Some(vn), "defop output is vn pre-destroy");

    fd.destroy_varnode(vn).unwrap();

    // Both readers' slots null (clearInput), def op output cleared, vn gone.
    assert_eq!(fd.obank().get(r0).unwrap().get_in(0), None, "reader 0 slot nulled");
    assert_eq!(fd.obank().get(r1).unwrap().get_in(0), None, "reader 1 slot nulled");
    assert_eq!(fd.obank().get(defop).unwrap().get_out(), None, "def op output cleared (vn->def->setOutput(0))");
    assert!(fd.vbank().get(vn).is_none(), "varnode freed from the bank");
}

#[test]
fn destroy_varnode_input_no_def_unhooks_and_frees() {
    // An INPUT varnode (no def) read by one op. destroyVarnode must skip the
    // `if (vn->def)` block, null the reader's slot, and free the input directly
    // (vbank.destroy succeeds: def==0 && descend now empty).
    let mut fd = build_fd();
    let r = ram(&fd);
    let inp = make_input(&mut fd, 0x100, 4);
    let op = fd.obank_mut().create_at(1, Address::new(r, 0x1000));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    wire_read(&mut fd, inp, op, 0);

    fd.destroy_varnode(inp).unwrap();

    assert_eq!(fd.obank().get(op).unwrap().get_in(0), None, "reader slot nulled");
    assert!(fd.vbank().get(inp).is_none(), "input varnode freed");
}
