//! Unit tests for `condexe.rs`.
//!
//! These build hand-wired basic-block CFGs on a real [`Funcdata`] and exercise:
//!   - the verify-path rejection conditions (`testIBlock`, `findInitPre`,
//!     `verifySameCondition`) — the read-only matrix that decides whether a
//!     CONDEXE configuration is removable;
//!   - a positive `trial` on a minimal same-condition diamond;
//!   - `RuleOrPredicate::MultiPredicate` discovery (`discoverZeroSlot`,
//!     `discoverCbranch`, `discoverPathIsTrue`, `discoverConditionalZero`);
//!   - `specs()` registration and the `Rule`/`Action` group-clone contract.
//!
//! The mutation path (`execute`/`doReplacement`/`RuleOrPredicate::applyOp`'s
//! op-creation tail) routes through the `opSetOutput` seam (LOSS-035/036) and is
//! covered by a seam-pinning test that asserts the documented `Err`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionGroupList, Rule};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

use super::{ActionConditionalExe, ConditionalExecution, MultiPredicate, RuleOrPredicate};

// -----------------------------------------------------------------------------
// Fixture
// -----------------------------------------------------------------------------

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
    m.set_default_code_space(2).unwrap();
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

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space(0).unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn opcode_flags(opc: OpCode) -> kuna_base::types::uint4 {
    match opc {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS => pcodeop_flags::booloutput,
        _ => 0,
    }
}

fn new_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// A formal input varnode (multiple descendants allowed).
fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(size, Address::new(ram, off), dt(size));
    let mut noop_thunk = |_b: &mut crate::varnode::VarnodeBank,
                          _o: VarnodeId,
                          _n: VarnodeId|
     -> kuna_base::error::KunaResult<()> { panic!("mk_input: collision") };
    fd.vbank_mut().set_input(vn, &mut noop_thunk).unwrap()
}

fn mk_const(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    let c = const_space(fd);
    fd.vbank_mut().create(size, Address::new(c, val), dt(size))
}

/// Output varnode defined by `op` (sets the `written` flag).  Distinct address
/// => no xref collision, so the no-op replace thunk is never invoked.
fn mk_out(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum: seq };
    let vn = {
        let mut noop_thunk = |_b: &mut crate::varnode::VarnodeBank,
                              _o: VarnodeId,
                              _n: VarnodeId|
         -> kuna_base::error::KunaResult<()> { panic!("mk_out: collision") };
        fd.vbank_mut()
            .create_def(size, Address::new(ram, off), dt(size), def, &mut noop_thunk)
            .unwrap()
    };
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn wire_read(fd: &mut Funcdata, reader: OpId, vn: VarnodeId, slot: int4) {
    fd.obank_mut().get_mut(reader).unwrap().set_input(Some(vn), slot);
    fd.vbank_mut().add_descend(vn, reader).unwrap();
}

fn new_bb(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().new_block_basic(root)
}

/// Set the entry block (hoists `bblocks_root_pub()` out of the `bblocks_mut()`
/// borrow to satisfy the borrow checker).
fn set_start(fd: &mut Funcdata, entry: BlockId) {
    let root = fd.bblocks_root_pub();
    fd.bblocks_mut().set_start_block(root, entry);
}

/// Set a basic block's cover so `block_get_start` has an address.
fn set_cover(fd: &mut Funcdata, bl: BlockId, start: u64) {
    let rs = ram_space(fd);
    let beg = Address::new(Rc::clone(&rs), start);
    let end = Address::new(rs, start + 0xf);
    fd.set_basic_block_range(bl, &beg, &end);
}

/// Place `op` at the end of basic block `bl` (alive + inserted at end).
fn place(fd: &mut Funcdata, op: OpId, bl: BlockId) {
    fd.op_insert_end(op, bl);
}

// =============================================================================
// testIBlock rejection matrix (verify-path read-only conditions)
// =============================================================================

/// A block with a non-CBRANCH terminator is rejected by testIBlock.
#[test]
fn trial_rejects_iblock_without_cbranch() {
    // initblock(CBRANCH) -> {a, b} ; a,b -> iblock(BRANCH, not CBRANCH) -> {x,y}
    let mut fd = build_fd();
    let init = new_bb(&mut fd);
    let a = new_bb(&mut fd);
    let b = new_bb(&mut fd);
    let iblock = new_bb(&mut fd);
    let x = new_bb(&mut fd);
    let y = new_bb(&mut fd);
    for (bl, s) in [(init, 0x1000), (a, 0x2000), (b, 0x3000), (iblock, 0x4000), (x, 0x5000), (y, 0x6000)] {
        set_cover(&mut fd, bl, s);
    }
    // init two-way; a/b funnel into iblock; iblock two-way out.
    fd.bblocks_mut().add_edge(init, a);
    fd.bblocks_mut().add_edge(init, b);
    fd.bblocks_mut().add_edge(a, iblock);
    fd.bblocks_mut().add_edge(b, iblock);
    fd.bblocks_mut().add_edge(iblock, x);
    fd.bblocks_mut().add_edge(iblock, y);
    set_start(&mut fd, init);

    // iblock terminator is a BRANCH, NOT a CBRANCH.
    let br = new_op(&mut fd, 1, 0x4001, OpCode::CPUI_BRANCH);
    let tgt = mk_input(&mut fd, 0x10, 8);
    wire_read(&mut fd, br, tgt, 0);
    place(&mut fd, br, iblock);

    fd.structure_reset();
    let mut ce = ConditionalExecution::new(&fd);
    assert!(!ce.trial(iblock, &fd), "iblock without final CBRANCH must be rejected");
}

/// A block with the wrong in/out arity is rejected by testIBlock.
#[test]
fn trial_rejects_wrong_arity_iblock() {
    // A simple straight-line block (1 in, 1 out) cannot be an iblock.
    let mut fd = build_fd();
    let entry = new_bb(&mut fd);
    let mid = new_bb(&mut fd);
    let exit = new_bb(&mut fd);
    for (bl, s) in [(entry, 0x1000), (mid, 0x2000), (exit, 0x3000)] {
        set_cover(&mut fd, bl, s);
    }
    fd.bblocks_mut().add_edge(entry, mid);
    fd.bblocks_mut().add_edge(mid, exit);
    set_start(&mut fd, entry);
    fd.structure_reset();

    let mut ce = ConditionalExecution::new(&fd);
    assert!(!ce.trial(mid, &fd), "1-in/1-out block cannot be an iblock");
}

/// findInitPre fails when the two in-paths do not converge on a single 2-out
/// initblock (here the two predecessors have distinct 2-out heads).
#[test]
fn trial_rejects_when_inpaths_diverge() {
    // Two SEPARATE 2-out predecessors p1,p2 feed iblock — they don't share an
    // initblock, so findInitPre returns false.
    let mut fd = build_fd();
    let p1 = new_bb(&mut fd);
    let p1b = new_bb(&mut fd); // p1's other successor
    let p2 = new_bb(&mut fd);
    let p2b = new_bb(&mut fd);
    let iblock = new_bb(&mut fd);
    let x = new_bb(&mut fd);
    let y = new_bb(&mut fd);
    let entry = new_bb(&mut fd);
    for (bl, s) in [
        (entry, 0x900), (p1, 0x1000), (p1b, 0x1100), (p2, 0x2000), (p2b, 0x2100),
        (iblock, 0x4000), (x, 0x5000), (y, 0x6000),
    ] {
        set_cover(&mut fd, bl, s);
    }
    fd.bblocks_mut().add_edge(entry, p1);
    fd.bblocks_mut().add_edge(entry, p2);
    fd.bblocks_mut().add_edge(p1, iblock);
    fd.bblocks_mut().add_edge(p1, p1b);
    fd.bblocks_mut().add_edge(p2, iblock);
    fd.bblocks_mut().add_edge(p2, p2b);
    fd.bblocks_mut().add_edge(iblock, x);
    fd.bblocks_mut().add_edge(iblock, y);
    set_start(&mut fd, entry);

    // Give iblock a real CBRANCH so testIBlock passes and we reach findInitPre.
    let cmp = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_INT_EQUAL);
    let a = mk_input(&mut fd, 0x10, 4);
    let k = mk_const(&mut fd, 0, 4);
    let cond = mk_out(&mut fd, cmp, 0x80, 1);
    wire_read(&mut fd, cmp, a, 0);
    wire_read(&mut fd, cmp, k, 1);
    place(&mut fd, cmp, iblock);
    let cb = new_op(&mut fd, 2, 0x4001, OpCode::CPUI_CBRANCH);
    let tgt = mk_input(&mut fd, 0x18, 8);
    wire_read(&mut fd, cb, tgt, 0);
    wire_read(&mut fd, cb, cond, 1);
    place(&mut fd, cb, iblock);

    fd.structure_reset();
    let mut ce = ConditionalExecution::new(&fd);
    // testIBlock passes (2-in/2-out + CBRANCH) but findInitPre must reject: the
    // two predecessors p1,p2 are distinct initblocks.
    assert!(!ce.trial(iblock, &fd), "divergent in-paths => no common initblock");
}

// =============================================================================
// verifySameCondition (integration: BooleanExpressionMatch through condexe)
// =============================================================================

/// Build the canonical CONDEXE diamond where init and iblock branch on the SAME
/// condition; verify the full positive `trial`.
#[test]
fn trial_accepts_same_condition_diamond() {
    // initblock(CBRANCH cond) -> {prea, preb} ; both -> iblock(CBRANCH cond) ->
    // {posta, postb}.  cond is shared (an INT_EQUAL output read by both CBRANCHs).
    let mut fd = build_fd();
    let init = new_bb(&mut fd);
    let prea = new_bb(&mut fd);
    let preb = new_bb(&mut fd);
    let iblock = new_bb(&mut fd);
    let posta = new_bb(&mut fd);
    let postb = new_bb(&mut fd);
    for (bl, s) in [
        (init, 0x1000), (prea, 0x2000), (preb, 0x3000),
        (iblock, 0x4000), (posta, 0x5000), (postb, 0x6000),
    ] {
        set_cover(&mut fd, bl, s);
    }
    // init: out0=false=>preb, out1=true=>prea (slot order picks true/false).
    fd.bblocks_mut().add_edge(init, preb); // out slot 0 (false)
    fd.bblocks_mut().add_edge(init, prea); // out slot 1 (true)
    fd.bblocks_mut().add_edge(prea, iblock);
    fd.bblocks_mut().add_edge(preb, iblock);
    fd.bblocks_mut().add_edge(iblock, postb); // out slot 0 (false)
    fd.bblocks_mut().add_edge(iblock, posta); // out slot 1 (true)
    set_start(&mut fd, init);

    // Shared condition: cond = INT_EQUAL(a, #0), placed in init; read by both
    // CBRANCHs (cond is a written output => multiple descendants allowed).
    let a = mk_input(&mut fd, 0x10, 4);
    let k = mk_const(&mut fd, 0, 4);
    let cmp = new_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_EQUAL);
    let cond = mk_out(&mut fd, cmp, 0x80, 1);
    wire_read(&mut fd, cmp, a, 0);
    wire_read(&mut fd, cmp, k, 1);
    place(&mut fd, cmp, init);
    let init_cb = new_op(&mut fd, 2, 0x1001, OpCode::CPUI_CBRANCH);
    let tgt0 = mk_input(&mut fd, 0x100, 8);
    wire_read(&mut fd, init_cb, tgt0, 0);
    wire_read(&mut fd, init_cb, cond, 1);
    place(&mut fd, init_cb, init);

    let iblock_cb = new_op(&mut fd, 2, 0x4001, OpCode::CPUI_CBRANCH);
    let tgt1 = mk_input(&mut fd, 0x108, 8);
    wire_read(&mut fd, iblock_cb, tgt1, 0);
    wire_read(&mut fd, iblock_cb, cond, 1);
    place(&mut fd, iblock_cb, iblock);

    fd.structure_reset();
    let mut ce = ConditionalExecution::new(&fd);
    assert!(
        ce.trial(iblock, &fd),
        "init and iblock branch on the same condition with the canonical diamond => removable"
    );
}

/// Same diamond but the two CBRANCHs key on UNRELATED conditions =>
/// verifySameCondition fails => trial rejects.
#[test]
fn trial_rejects_unrelated_conditions() {
    let mut fd = build_fd();
    let init = new_bb(&mut fd);
    let prea = new_bb(&mut fd);
    let preb = new_bb(&mut fd);
    let iblock = new_bb(&mut fd);
    let posta = new_bb(&mut fd);
    let postb = new_bb(&mut fd);
    for (bl, s) in [
        (init, 0x1000), (prea, 0x2000), (preb, 0x3000),
        (iblock, 0x4000), (posta, 0x5000), (postb, 0x6000),
    ] {
        set_cover(&mut fd, bl, s);
    }
    fd.bblocks_mut().add_edge(init, preb);
    fd.bblocks_mut().add_edge(init, prea);
    fd.bblocks_mut().add_edge(prea, iblock);
    fd.bblocks_mut().add_edge(preb, iblock);
    fd.bblocks_mut().add_edge(iblock, postb);
    fd.bblocks_mut().add_edge(iblock, posta);
    set_start(&mut fd, init);

    // init keys on INT_EQUAL(a,#0); iblock keys on a DIFFERENT free var (b!=0).
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x18, 4);
    let k = mk_const(&mut fd, 0, 4);
    let cmp0 = new_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_EQUAL);
    let cond0 = mk_out(&mut fd, cmp0, 0x80, 1);
    wire_read(&mut fd, cmp0, a, 0);
    wire_read(&mut fd, cmp0, k, 1);
    place(&mut fd, cmp0, init);
    let init_cb = new_op(&mut fd, 2, 0x1001, OpCode::CPUI_CBRANCH);
    let tgt0 = mk_input(&mut fd, 0x100, 8);
    wire_read(&mut fd, init_cb, tgt0, 0);
    wire_read(&mut fd, init_cb, cond0, 1);
    place(&mut fd, init_cb, init);

    let k2 = mk_const(&mut fd, 0, 4);
    let cmp1 = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_INT_EQUAL);
    let cond1 = mk_out(&mut fd, cmp1, 0x88, 1);
    wire_read(&mut fd, cmp1, b, 0); // DIFFERENT variable
    wire_read(&mut fd, cmp1, k2, 1);
    place(&mut fd, cmp1, iblock);
    let iblock_cb = new_op(&mut fd, 2, 0x4001, OpCode::CPUI_CBRANCH);
    let tgt1 = mk_input(&mut fd, 0x108, 8);
    wire_read(&mut fd, iblock_cb, tgt1, 0);
    wire_read(&mut fd, iblock_cb, cond1, 1);
    place(&mut fd, iblock_cb, iblock);

    fd.structure_reset();
    let mut ce = ConditionalExecution::new(&fd);
    assert!(!ce.trial(iblock, &fd), "unrelated conditions => not removable");
}

// =============================================================================
// MultiPredicate discovery (RuleOrPredicate building blocks)
// =============================================================================

/// discoverZeroSlot recognizes a 2-input MULTIEQUAL one of whose inputs is a
/// COPY of constant 0.
#[test]
fn multipredicate_discovers_zero_slot() {
    let mut fd = build_fd();
    let join = new_bb(&mut fd);
    set_cover(&mut fd, join, 0x4000);

    // tmp0 = COPY(#0) ; otherVn (defined) ; me = MULTIEQUAL(tmp0, otherVn).
    let zero = mk_const(&mut fd, 0, 4);
    let copy = new_op(&mut fd, 1, 0x3000, OpCode::CPUI_COPY);
    let tmp0 = mk_out(&mut fd, copy, 0x70, 4);
    wire_read(&mut fd, copy, zero, 0);
    // otherVn: output of some op so it isn't free.
    let defop = new_op(&mut fd, 1, 0x3100, OpCode::CPUI_COPY);
    let src = mk_input(&mut fd, 0x20, 4);
    let other = mk_out(&mut fd, defop, 0x78, 4);
    wire_read(&mut fd, defop, src, 0);

    let me = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_MULTIEQUAL);
    fd.obank_mut().get_mut(me).unwrap().set_flag(pcodeop_flags::marker);
    let me_out = mk_out(&mut fd, me, 0x80, 4);
    wire_read(&mut fd, me, tmp0, 0); // zero slot = 0
    wire_read(&mut fd, me, other, 1);
    place(&mut fd, me, join);

    let mut mp = MultiPredicate::new();
    assert!(mp.discover_zero_slot(me_out, &fd), "2-input MULTIEQUAL with COPY(#0) input recognized");
    assert_eq!(mp.zero_slot, 0, "slot 0 is the zero path");
    assert_eq!(mp.other_vn, Some(other), "otherVn is the non-zero input");
}

/// discoverZeroSlot rejects a MULTIEQUAL whose 'zero' input copies a NON-zero
/// constant.
#[test]
fn multipredicate_rejects_nonzero_copy() {
    let mut fd = build_fd();
    let join = new_bb(&mut fd);
    set_cover(&mut fd, join, 0x4000);
    let five = mk_const(&mut fd, 5, 4); // NOT zero
    let copy = new_op(&mut fd, 1, 0x3000, OpCode::CPUI_COPY);
    let tmp = mk_out(&mut fd, copy, 0x70, 4);
    wire_read(&mut fd, copy, five, 0);
    let defop = new_op(&mut fd, 1, 0x3100, OpCode::CPUI_COPY);
    let src = mk_input(&mut fd, 0x20, 4);
    let other = mk_out(&mut fd, defop, 0x78, 4);
    wire_read(&mut fd, defop, src, 0);
    let me = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_MULTIEQUAL);
    let me_out = mk_out(&mut fd, me, 0x80, 4);
    wire_read(&mut fd, me, tmp, 0);
    wire_read(&mut fd, me, other, 1);

    let mut mp = MultiPredicate::new();
    assert!(!mp.discover_zero_slot(me_out, &fd), "copy of #5 is not a zero set");
}

/// discoverCbranch + discoverPathIsTrue + discoverConditionalZero on a small
/// predication CFG: condBlock(CBRANCH a==0) -> {zeroBlock, otherBlock} -> join.
#[test]
fn multipredicate_discovers_cbranch_and_path() {
    let mut fd = build_fd();
    let cond = new_bb(&mut fd);
    let zero_bl = new_bb(&mut fd);
    let other_bl = new_bb(&mut fd);
    let join = new_bb(&mut fd);
    for (bl, s) in [(cond, 0x1000), (zero_bl, 0x2000), (other_bl, 0x3000), (join, 0x4000)] {
        set_cover(&mut fd, bl, s);
    }
    // condBlock: out0=false=>other_bl, out1=true=>zero_bl (true path sets zero).
    fd.bblocks_mut().add_edge(cond, other_bl); // slot 0 (false)
    fd.bblocks_mut().add_edge(cond, zero_bl); // slot 1 (true)
    fd.bblocks_mut().add_edge(zero_bl, join); // join in slot 0
    fd.bblocks_mut().add_edge(other_bl, join); // join in slot 1
    set_start(&mut fd, cond);

    // a==0 condition in condBlock.
    let a = mk_input(&mut fd, 0x10, 4);
    let k = mk_const(&mut fd, 0, 4);
    let eq = new_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_EQUAL);
    let c = mk_out(&mut fd, eq, 0x80, 1);
    wire_read(&mut fd, eq, a, 0);
    wire_read(&mut fd, eq, k, 1);
    place(&mut fd, eq, cond);
    let cb = new_op(&mut fd, 2, 0x1001, OpCode::CPUI_CBRANCH);
    let tgt = mk_input(&mut fd, 0x100, 8);
    wire_read(&mut fd, cb, tgt, 0);
    wire_read(&mut fd, cb, c, 1);
    place(&mut fd, cb, cond);

    // zero set: COPY(#0) in zero_bl (join slot 0).
    let zero = mk_const(&mut fd, 0, 4);
    let cp0 = new_op(&mut fd, 1, 0x2000, OpCode::CPUI_COPY);
    let tmp0 = mk_out(&mut fd, cp0, 0x70, 4);
    wire_read(&mut fd, cp0, zero, 0);
    place(&mut fd, cp0, zero_bl);
    // other set in other_bl (join slot 1).
    let cp1 = new_op(&mut fd, 1, 0x3000, OpCode::CPUI_COPY);
    let src = mk_input(&mut fd, 0x20, 4);
    let other = mk_out(&mut fd, cp1, 0x78, 4);
    wire_read(&mut fd, cp1, src, 0);
    place(&mut fd, cp1, other_bl);

    // me = MULTIEQUAL(tmp0[from zero_bl, slot0], other[from other_bl, slot1]).
    let me = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_MULTIEQUAL);
    fd.obank_mut().get_mut(me).unwrap().set_flag(pcodeop_flags::marker);
    let me_out = mk_out(&mut fd, me, 0x88, 4);
    wire_read(&mut fd, me, tmp0, 0);
    wire_read(&mut fd, me, other, 1);
    place(&mut fd, me, join);

    fd.structure_reset();
    let mut mp = MultiPredicate::new();
    assert!(mp.discover_zero_slot(me_out, &fd));
    assert_eq!(mp.zero_slot, 0);
    assert!(mp.discover_cbranch(&fd), "single controlling CBRANCH found");
    assert_eq!(mp.cond_block, Some(cond));
    assert_eq!(mp.zero_block, Some(zero_bl));
    mp.discover_path_is_true(&fd);
    // condBlock true-out is zero_bl => zeroPathIsTrue = true.
    assert!(mp.zero_path_is_true, "true path of condBlock flows to the zero set");
    // a==0 via INT_EQUAL, no boolean flip => zeroPathIsTrue stays true.
    assert!(mp.discover_conditional_zero(a, &fd));
    assert!(mp.zero_path_is_true);
}

// =============================================================================
// Rule / Action registration contracts
// =============================================================================

#[test]
fn specs_lists_orpredicate_in_condexe_group() {
    let specs = super::specs();
    assert_eq!(specs.len(), 1, "condexe.cc defines exactly one Rule (RuleOrPredicate)");
    assert_eq!(specs[0].group, "condexe");
    // The ctor builds a working RuleOrPredicate offered on INT_OR/INT_XOR only.
    let rule = (specs[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]);
}

#[test]
fn orpredicate_clone_filters_by_group() {
    let rule = RuleOrPredicate::new("condexe");
    // In-group => clones.
    let mut gl = ActionGroupList::new();
    gl.insert("condexe");
    assert!(rule.clone_rule(&gl).is_some());
    // Out-of-group => filtered out.
    let other = ActionGroupList::from_names(["analysis"]);
    assert!(rule.clone_rule(&other).is_none());
}

#[test]
fn action_conditionalexe_clone_filters_by_group() {
    let act = ActionConditionalExe::new("condexe");
    assert_eq!(act.get_name(), "conditionalexe");
    assert_eq!(act.get_group(), "condexe");
    let gl = ActionGroupList::from_names(["condexe"]);
    assert!(act.clone_filtered(&gl).is_some());
    let none = ActionGroupList::from_names(["analysis"]);
    assert!(act.clone_filtered(&none).is_none());
}

/// `ActionConditionalExe::apply` short-circuits to 0 when there are unreachable
/// blocks (the C++ `hasUnreachableBlocks` guard).
#[test]
fn action_apply_bails_on_unreachable_blocks() {
    // entry -> a ; orphan (no in-edge) => has_unreachable_blocks() == true.
    let mut fd = build_fd();
    let entry = new_bb(&mut fd);
    let a = new_bb(&mut fd);
    let _orphan = new_bb(&mut fd);
    for (bl, s) in [(entry, 0x1000), (a, 0x2000), (_orphan, 0x3000)] {
        set_cover(&mut fd, bl, s);
    }
    fd.bblocks_mut().add_edge(entry, a);
    set_start(&mut fd, entry);
    fd.structure_reset();
    assert!(fd.has_unreachable_blocks());

    let mut act = ActionConditionalExe::new("condexe");
    let mut ctx = crate::action::ActionContext::new();
    let res = act.apply(&mut fd, &mut ctx);
    assert_eq!(res, 0, "apply returns 0 immediately on unreachable blocks");
    assert_eq!(act.base().count, 0, "no hits recorded");
}

// =============================================================================
// Seam pins (opSetOutput / W6 opSetOpcode)
// =============================================================================

/// `RuleOrPredicate::applyOp` returns 0 (no change) when neither input is a
/// zero-predicated MULTIEQUAL — the early `(test0==false)&&(test1==false)`
/// return, reached without touching any seam.
#[test]
fn orpredicate_noop_on_nonpredicate_inputs() {
    let mut fd = build_fd();
    let bl = new_bb(&mut fd);
    set_cover(&mut fd, bl, 0x1000);
    // result = a | b with a,b plain inputs (no MULTIEQUAL/zero form).
    let a = mk_input(&mut fd, 0x10, 4);
    let b = mk_input(&mut fd, 0x18, 4);
    let or = new_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_OR);
    let _o = mk_out(&mut fd, or, 0x80, 4);
    wire_read(&mut fd, or, a, 0);
    wire_read(&mut fd, or, b, 1);
    place(&mut fd, or, bl);

    let mut rule = RuleOrPredicate::new("condexe");
    assert_eq!(rule.apply_op(or, &mut fd), 0, "no predicate form => no change");
}

/// PIN the W6 opcode-resolution seam: building the predicate form and driving
/// `applyOp` to the `opSetOpcode(op,CPUI_COPY)` step surfaces the documented
/// `Err` from `resolve_typeop`; the public `apply_op` degrades it to 0.
///
/// We reach the single-form (`checkSingle`) path: `result = tmp1 | val2` where
/// tmp1 = MULTIEQUAL(COPY(#0), val1) and val2 plays the (val2==0)?val1:0 role.
/// The discovery succeeds up to `opSetInput`/`opSetOpcode`; `opSetOpcode` then
/// hits the W6 seam.  `apply_op_inner` returns the seam `Err`.
#[test]
fn orpredicate_single_form_surfaces_w6_seam() {
    let mut fd = build_fd();
    let condb = new_bb(&mut fd);
    let zero_bl = new_bb(&mut fd);
    let other_bl = new_bb(&mut fd);
    let join = new_bb(&mut fd);
    for (bl, s) in [(condb, 0x1000), (zero_bl, 0x2000), (other_bl, 0x3000), (join, 0x4000)] {
        set_cover(&mut fd, bl, s);
    }
    // condBlock: out0=false=>other_bl, out1=true=>zero_bl.  (val2==0) is true on
    // the FALSE path here, so zeroPathIsTrue must be false for checkSingle to fire.
    fd.bblocks_mut().add_edge(condb, zero_bl); // slot 0 (false) => zero set
    fd.bblocks_mut().add_edge(condb, other_bl); // slot 1 (true)
    fd.bblocks_mut().add_edge(zero_bl, join);
    fd.bblocks_mut().add_edge(other_bl, join);
    set_start(&mut fd, condb);

    // val2 (the `other` input to the OR) and val1 (the non-zero MULTIEQUAL side).
    let val2 = mk_input(&mut fd, 0x10, 4);
    let val1 = mk_input(&mut fd, 0x18, 4);
    // cond = INT_EQUAL(val2, #0) in condBlock.
    let k = mk_const(&mut fd, 0, 4);
    let eq = new_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_EQUAL);
    let c = mk_out(&mut fd, eq, 0x80, 1);
    wire_read(&mut fd, eq, val2, 0);
    wire_read(&mut fd, eq, k, 1);
    place(&mut fd, eq, condb);
    let cb = new_op(&mut fd, 2, 0x1001, OpCode::CPUI_CBRANCH);
    let tgt = mk_input(&mut fd, 0x100, 8);
    wire_read(&mut fd, cb, tgt, 0);
    wire_read(&mut fd, cb, c, 1);
    place(&mut fd, cb, condb);

    // zero set: tmp_zero = COPY(#0) in zero_bl (join slot 0).
    let zero = mk_const(&mut fd, 0, 4);
    let cp0 = new_op(&mut fd, 1, 0x2000, OpCode::CPUI_COPY);
    let tmp0 = mk_out(&mut fd, cp0, 0x70, 4);
    wire_read(&mut fd, cp0, zero, 0);
    place(&mut fd, cp0, zero_bl);

    // tmp1 = MULTIEQUAL(tmp0 [zero, slot 0], val1 [other path, slot 1]).
    let me = new_op(&mut fd, 2, 0x4000, OpCode::CPUI_MULTIEQUAL);
    fd.obank_mut().get_mut(me).unwrap().set_flag(pcodeop_flags::marker);
    let tmp1 = mk_out(&mut fd, me, 0x88, 4);
    wire_read(&mut fd, me, tmp0, 0);
    wire_read(&mut fd, me, val1, 1);
    place(&mut fd, me, join);

    // result = tmp1 | val2  (the INT_OR rule target).
    let or = new_op(&mut fd, 2, 0x4001, OpCode::CPUI_INT_OR);
    let _o = mk_out(&mut fd, or, 0x90, 4);
    wire_read(&mut fd, or, tmp1, 0); // branch0 = MULTIEQUAL form
    wire_read(&mut fd, or, val2, 1); // branch1 not a predicate (val2 plain) => checkSingle path
    place(&mut fd, or, join);

    fd.structure_reset();

    let rule = RuleOrPredicate::new("condexe");
    // apply_op_inner returns the W6 seam Err once discovery reaches opSetOpcode.
    let res = rule.apply_op_inner(or, &mut fd);
    assert!(res.is_err(), "discovery reaches opSetOpcode(CPUI_COPY) => W6 seam Err");
    let msg = format!("{}", res.unwrap_err());
    assert!(msg.contains("glb->inst") || msg.contains("W6"), "Err names the W6 op-info seam: {msg}");
}

/// `new_unique_out` (the `newUniqueOut`→`opSetOutput` helper): the
/// w4x-flow-linkage wave filled `Funcdata::op_set_output`, so this now creates
/// a real unique-space output linked to the op (formerly the pinned
/// LOSS-035/036 seam Err).
#[test]
fn condexe_new_unique_out_links_unique_output() {
    let mut fd = build_fd();
    let bl = new_bb(&mut fd);
    set_cover(&mut fd, bl, 0x1000);
    set_start(&mut fd, bl);
    fd.structure_reset();
    let ce = ConditionalExecution::new(&fd);
    let op = new_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let outvn = ce.new_unique_out(4, op, &mut fd).expect("newUniqueOut real since w4x-flow-linkage");
    let v = fd.vbank().get(outvn).expect("fresh output varnode");
    assert_eq!(v.get_size(), 4);
    assert_eq!(v.get_space().get_name(), "unique", "output lives in the unique space");
    assert_eq!(fd.obank().get(op).expect("op").get_out(), Some(outvn), "op output linked");
    assert_eq!(v.get_def(), Some(op), "output defined by the op");
}
