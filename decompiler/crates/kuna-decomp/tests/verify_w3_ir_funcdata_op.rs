//! Adversarial verification tests for item `w3-ir-funcdata-op`
//! (port of `decompiler/cpp/funcdata_op.cc`).  Written by the INDEPENDENT
//! verifier, targeting the spots the hunt list flagged as most fragile:
//!
//!   - the `opSetInput` constant-reshare guard (formerly a documented loss, now
//!     ported): the C++ re-duplicates a constant that already has a descendant.
//!     This test asserts the dedup happens (a FRESH constant is linked, the
//!     original keeps its single descendant) — no `add_descend` error.
//!   - `opInsertBefore` when EVERY preceding op is an INDIRECT (the backward
//!     walk reaches `beginOp()`).
//!   - `opInsertAfter` when EVERY following op is a MULTIEQUAL (the forward
//!     walk reaches `endOp()` — insert lands at end-of-block).
//!   - `op_target` with a delay-slot layout: two startmark ops in the dead
//!     list; `target()` must stop at the NEAREST preceding startmark, not the
//!     first one.
//!   - `op_flip_in_place_test` BOOL_AND: fliplist order (children before
//!     parent) and the `return subtest1` (front-normalizing) contract.
//!   - `op_next_op` flowing through an EMPTY successor block (single-out flow,
//!     the `while(iter==endOp())` loop must keep advancing through empties).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use kuna_base::error::KunaResult;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, BlockId, OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

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
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space(0).unwrap())
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

/// A constant varnode (free, in the constant space).
fn mk_const(fd: &mut Funcdata, val: u64) -> VarnodeId {
    let c = const_space(fd);
    fd.vbank_mut().create(4, Address::new(c, val), dt(4))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

// -----------------------------------------------------------------------------
// 1. opSetInput constant-reshare guard (ported) — dedup, not error.
// -----------------------------------------------------------------------------
//
// C++ funcdata_op.cc:104-125: when a constant Varnode already has a descendant
// (and is not a spacebase), opSetInput re-duplicates it via newConstant so the
// fresh copy can be linked (constants must have a single descendant).  This is
// now ported (the funcdata_varnode `newConstant` factory has landed): the second
// link gets a FRESH constant, the original keeps its single descendant, and no
// add_descend guard fires.
#[test]
fn w3_ir_funcdata_op_const_reshare_dedups_like_cpp() {
    let mut fd = build_fd();
    let op_a = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let op_b = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_COPY);
    let k = mk_const(&mut fd, 7);
    // First link: the constant gains its single descendant — fine.
    fd.op_set_input(op_a, k, 0).expect("first const link must succeed");
    assert_eq!(fd.vbank().get(k).unwrap().num_descend(), 1);
    // Second link of the SAME constant: the dedup guard re-duplicates it, so the
    // link SUCCEEDS and op_b reads a fresh copy (value 7), not `k` itself.
    fd.op_set_input(op_b, k, 0).expect("re-shared constant dedups and links");
    let dup = fd.obank().get(op_b).unwrap().get_in(0).expect("op_b slot 0 linked");
    assert_ne!(dup, k, "op_b reads a FRESH duplicate, not the shared constant");
    assert!(fd.vbank().get(dup).unwrap().is_constant());
    assert_eq!(fd.vbank().get(dup).unwrap().get_offset(), 7, "duplicate carries the same value");
    // The original constant still has exactly its one (op_a) descendant.
    assert_eq!(fd.vbank().get(k).unwrap().num_descend(), 1);
    assert_eq!(fd.vbank().get(dup).unwrap().num_descend(), 1, "duplicate has its single reader");
}

// A NON-constant free varnode reshare also errors (this is the plain C++
// addDescend invariant, not the constant-dedup path) — proves the seam test
// above is specifically about the missing constant duplication, not a generic
// reshare difference.
#[test]
fn w3_ir_funcdata_op_nonconst_free_reshare_errs_like_cpp() {
    let mut fd = build_fd();
    let op_a = mk_op(&mut fd, 1, 0x100, OpCode::CPUI_COPY);
    let op_b = mk_op(&mut fd, 1, 0x110, OpCode::CPUI_COPY);
    let v = mk_vn(&mut fd, 0x10); // free, non-constant, non-spacebase
    fd.op_set_input(op_a, v, 0).expect("first link ok");
    let res = fd.op_set_input(op_b, v, 0);
    assert!(res.is_err(), "free non-const varnode cannot have two descendants (C++ throws too)");
}

// -----------------------------------------------------------------------------
// 2. opInsertBefore — every preceding op is INDIRECT (walk reaches beginOp).
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_op_insert_before_all_indirect_lands_at_begin() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // block: [ind0, ind1, copy2]
    let ind0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_INDIRECT);
    let ind1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_INDIRECT);
    let copy2 = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_COPY);
    fd.op_insert(ind0, bl, None);
    fd.op_insert(ind1, bl, None);
    fd.op_insert(copy2, bl, None);
    // insert a non-INDIRECT before copy2: it must skip back over ind1 AND ind0
    // (the whole leading INDIRECT run reaches beginOp) and land first.
    let newop = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_INT_ADD);
    fd.op_insert_before(newop, copy2);
    assert_eq!(fd.bb_ops(bl), vec![newop, ind0, ind1, copy2]);
}

// -----------------------------------------------------------------------------
// 3. opInsertAfter — every following op is MULTIEQUAL (walk reaches endOp).
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_op_insert_after_all_multiequal_lands_at_end() {
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    // block: [copy0, multi1, multi2]
    let copy0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    let multi1 = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_MULTIEQUAL);
    let multi2 = mk_op(&mut fd, 0, 0x120, OpCode::CPUI_MULTIEQUAL);
    fd.op_insert(copy0, bl, None);
    fd.op_insert(multi1, bl, None);
    fd.op_insert(multi2, bl, None);
    // insert a non-MULTIEQUAL after copy0: skip forward over multi1, multi2 (the
    // whole trailing MULTIEQUAL run reaches endOp) and land last.
    let newop = mk_op(&mut fd, 0, 0x130, OpCode::CPUI_INT_ADD);
    fd.op_insert_after(newop, copy0);
    assert_eq!(fd.bb_ops(bl), vec![copy0, multi1, multi2, newop]);
}

// -----------------------------------------------------------------------------
// 4. op_target — delay-slot: two startmark ops; stop at the NEAREST one.
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_op_target_stops_at_nearest_startmark() {
    let mut fd = build_fd();
    // dead-list order: s0(startmark), a, s1(startmark), b, c
    let s0 = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    fd.obank_mut().get_mut(s0).unwrap().set_flag(pcodeop_flags::startmark);
    let a = mk_op(&mut fd, 0, 0x104, OpCode::CPUI_COPY);
    let s1 = mk_op(&mut fd, 0, 0x108, OpCode::CPUI_COPY);
    fd.obank_mut().get_mut(s1).unwrap().set_flag(pcodeop_flags::startmark);
    let b = mk_op(&mut fd, 0, 0x10c, OpCode::CPUI_COPY);
    let c = mk_op(&mut fd, 0, 0x110, OpCode::CPUI_COPY);
    // target of c/b is s1 (nearest preceding startmark), NOT s0.
    assert_eq!(fd.op_target(c), s1);
    assert_eq!(fd.op_target(b), s1);
    // target of s1 is itself.
    assert_eq!(fd.op_target(s1), s1);
    // target of a (before s1) is s0.
    assert_eq!(fd.op_target(a), s0);
}

// -----------------------------------------------------------------------------
// 5. op_flip_in_place_test — BOOL_AND fliplist order + return subtest1.
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_op_flip_bool_and_order_and_return() {
    let mut fd = build_fd();
    // and = BOOL_AND(d0, d1) where:
    //   d0 = INT_LESS(const, x)   -> subtest 0 (in0 constant)  [normalizing front]
    //   d1 = INT_NOTEQUAL(x, y)   -> subtest 0
    // C++ pushes d0, then d1, then `and`; returns subtest1 (== d0's result, 0).
    let and = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_BOOL_AND);
    let d0 = mk_op(&mut fd, 2, 0x110, OpCode::CPUI_INT_LESS);
    let d1 = mk_op(&mut fd, 2, 0x120, OpCode::CPUI_INT_NOTEQUAL);

    // Wire the booleans so each is the lone descendant of its def's output.
    // and.in0 = bvn0 (def d0), and.in1 = bvn1 (def d1).
    let bvn0 = mk_vn(&mut fd, 0x200);
    let bvn1 = mk_vn(&mut fd, 0x210);
    // op_set_output is seam-deferred (the setDef step needs the two-bank
    // split-borrow this item lacks); wire the def/descend shape the read-only
    // flip tracer needs directly through the varnode bank's set_def, with a
    // no-op replace-reads thunk (a fresh varnode has no equivalent to unify).
    wire_def_and_lone_read(&mut fd, d0, bvn0, and, 0);
    wire_def_and_lone_read(&mut fd, d1, bvn1, and, 1);

    // d0 = INT_LESS with a constant in slot 0 -> subtest 0.
    let k = mk_const(&mut fd, 3);
    let x = mk_vn(&mut fd, 0x300);
    fd.op_set_input(d0, k, 0).unwrap();
    fd.op_set_input(d0, x, 1).unwrap();
    // d1 = INT_NOTEQUAL -> subtest 0 (no constant dependence).
    let y = mk_vn(&mut fd, 0x310);
    let z = mk_vn(&mut fd, 0x320);
    fd.op_set_input(d1, y, 0).unwrap();
    fd.op_set_input(d1, z, 1).unwrap();

    let mut fliplist = Vec::new();
    let r = fd.op_flip_in_place_test(and, &mut fliplist, false);
    // children pushed before parent, in0-subtree first.
    assert_eq!(fliplist, vec![d0, d1, and], "fliplist must be d0, d1, then the AND");
    // return value is subtest1 (d0's normalizing result == 0).
    assert_eq!(r, 0, "AND returns its front (in0) subtest, which is normalizing");
}

/// Wire `vn` to be written by `def` and read (as a lone descendant) by `reader`
/// in `slot`, using only the public bank surfaces this verifier may touch.
/// (op_set_output is seam-deferred for the def link, so set the def flag via the
/// op's output field + the varnode's written flag manually through the bank.)
fn wire_def_and_lone_read(
    fd: &mut Funcdata,
    def: OpId,
    vn: VarnodeId,
    reader: OpId,
    slot: int4,
) {
    // Mark vn as written by `def` via the varnode bank's set_def (the same call
    // op_set_output is built on); a fresh varnode never unifies, so the
    // replace-reads thunk is a no-op here.
    let seqnum = fd.obank().get(def).unwrap().get_seq_num().clone();
    let info = DefOpInfo { id: def, seqnum };
    let mut noop = |_b: &mut VarnodeBank, _o: VarnodeId, _n: VarnodeId| -> KunaResult<()> { Ok(()) };
    let new_vn = fd.vbank_mut().set_def(vn, info, &mut noop).unwrap();
    fd.obank_mut().get_mut(def).unwrap().set_output(Some(new_vn));
    // Link the lone read.
    fd.op_set_input(reader, new_vn, slot).unwrap();
}

// -----------------------------------------------------------------------------
// 6. op_next_op — flow through an EMPTY successor block (single-out).
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_op_next_op_through_empty_block() {
    let mut fd = build_fd();
    let b0 = mk_block(&mut fd);
    let b1 = mk_block(&mut fd); // empty middle block
    let b2 = mk_block(&mut fd);
    // b0 -[single out]-> b1 -[single out]-> b2
    fd.bblocks_mut().add_edge(b0, b1);
    fd.bblocks_mut().add_edge(b1, b2);

    let a = mk_op(&mut fd, 0, 0x100, OpCode::CPUI_COPY);
    fd.op_insert(a, b0, None);
    let c = mk_op(&mut fd, 0, 0x200, OpCode::CPUI_COPY);
    fd.op_insert(c, b2, None);

    // nextOp(a): a is last in b0; flow single-out into b1 (empty) then b2 (begin
    // op is c).  The while(iter==endOp()) loop must skip the empty b1.
    assert_eq!(fd.bblocks_ref().block(b0).size_out(), 1);
    assert_eq!(fd.bblocks_ref().block(b1).size_out(), 1);
    assert_eq!(fd.op_next_op(a), Some(c));
}

// -----------------------------------------------------------------------------
// 7. override_flow — CALL -> BRANCH and CALL -> CALL_RETURN (return-op insert).
//    (C++ Funcdata::overrideFlow, funcdata_op.cc:969)
// -----------------------------------------------------------------------------
#[test]
fn w3_ir_funcdata_override_flow_call_to_branch() {
    use kuna_decomp::overrides::flow_type;
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x300);
    // A dead CALL op at 0x300 (in0 = the call-target coderef varnode).
    let call = mk_op(&mut fd, 1, 0x300, OpCode::CPUI_CALL);
    let tgt = mk_vn(&mut fd, 0x500);
    fd.op_set_input(call, tgt, 0).unwrap();
    // BRANCH override: CALL -> BRANCH (findPrimaryBranch findcall=false,findbranch=false,findreturn=true,
    // here BRANCH wants findcall=true via the BRANCH arm).
    fd.override_flow(&addr, flow_type::BRANCH).unwrap();
    assert_eq!(fd.obank().get(call).unwrap().code(), OpCode::CPUI_BRANCH);
}

#[test]
fn w3_ir_funcdata_override_flow_call_return_inserts_return() {
    use kuna_decomp::overrides::flow_type;
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x300);
    let call = mk_op(&mut fd, 1, 0x300, OpCode::CPUI_CALL);
    let tgt = mk_vn(&mut fd, 0x500);
    fd.op_set_input(call, tgt, 0).unwrap();
    let before = fd.obank().iter_dead().count();
    // CALL_RETURN: CALL stays CALL and a fresh RETURN op is inserted after it.
    fd.override_flow(&addr, flow_type::CALL_RETURN).unwrap();
    assert_eq!(fd.obank().get(call).unwrap().code(), OpCode::CPUI_CALL);
    assert_eq!(fd.obank().iter_dead().count(), before + 1);
    // The inserted op is a RETURN.
    let has_return = fd
        .obank()
        .iter_dead()
        .any(|op| fd.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_RETURN));
    assert!(has_return);
}

#[test]
fn w3_ir_funcdata_override_flow_no_matching_branch_errors() {
    use kuna_decomp::overrides::flow_type;
    let mut fd = build_fd();
    let ram = ram_space(&fd);
    let addr = Address::new(ram, 0x300);
    // Only a COPY at the address: no primary branch -> "Could not apply flowoverride".
    let _copy = mk_op(&mut fd, 0, 0x300, OpCode::CPUI_COPY);
    assert!(fd.override_flow(&addr, flow_type::BRANCH).is_err());
}
