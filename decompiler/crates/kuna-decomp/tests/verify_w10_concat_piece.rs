//! INDEPENDENT VERIFIER adversarial tests for item `w10-concat-piece`
//! (`RulePieceStructure` + `PieceNode`/`gatherPieces`/`isLeaf`, op.cc:822-906,
//! ruleaction.cc:7496-7733).
//!
//! These drive only the PUBLIC `op`/`ruleaction_6` API and target the
//! hunt-list spots the review flagged as most fragile for this port:
//!
//!  - `PieceNode::gatherPieces` byte-offset math (op.cc:900):
//!    `(rootVn->getSpace()->isBigEndian() == (i==1)) ? base+in1.size : base`.
//!    The porter's in-module test only covers LITTLE-endian; the BIG-endian arm
//!    is the mirror image and the single most error-prone line in the port.
//!  - `gatherPieces` recursion: a non-leaf intermediate must be expanded AFTER
//!    both of its parent's slot-nodes are pushed (DFS order), with the child
//!    offsets accumulated relative to the parent's offset (not the root's).
//!  - `PieceNode::isLeaf` (op.cc:831-847): the `loneDescend()==0` arm (a
//!    PIECE-defined varnode read by TWO ops is a leaf) and the `isAddrTied()`
//!    address-mismatch arm.
//!  - `RulePieceStructure::spanningRange` (ruleaction.cc:7534): the
//!    `offset+size > ct->getSize()` early-`false` boundary.
//!
//! Construction note: the only public opcode-setting path is
//! `PcodeOpBank::change_opcode`; a written (non-leaf) varnode is produced with
//! the public `VarnodeBank::set_def` + `DefOpInfo`.  No private API is used.

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::op::{gather_pieces, piece_is_leaf, PcodeOpBank, PieceNode};
use kuna_decomp::ruleaction_6::RulePieceStructure;
use kuna_decomp::context::{OpId, TypeOp, VarnodeId};
use kuna_decomp::varnode::{DefOpInfo, VarnodeBank};

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// const(0), unique(1), ram(2) — `big_end` selects the ram endianness so the
/// big-endian `gatherPieces` arm can be exercised.
fn manager(big_end: bool) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big_end,
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

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(2).unwrap()), off)
}

/// Create a 2-input PIECE op with output `out`, slot-0 input `hi`, slot-1 input
/// `lo`, link the input descend lists, and return the op id.
fn piece_op(
    obank: &mut PcodeOpBank,
    vbank: &mut VarnodeBank,
    m: &AddrSpaceManager,
    pc: u64,
    out: VarnodeId,
    hi: VarnodeId,
    lo: VarnodeId,
) -> OpId {
    let op = obank.create_at(2, ram(m, pc));
    obank.change_opcode(op, TypeOp::new(OpCode::CPUI_PIECE, 0, "CONCAT"));
    {
        let o = obank.get_mut(op).unwrap();
        o.set_input(Some(hi), 0);
        o.set_input(Some(lo), 1);
        o.set_output(Some(out));
    }
    vbank.add_descend(hi, op).unwrap();
    vbank.add_descend(lo, op).unwrap();
    op
}

/// Make `vn` a written varnode whose def is `op` (so `is_written()` and
/// `get_def()` line up with the bank), via the public `set_def` path.
fn mark_written(obank: &PcodeOpBank, vbank: &mut VarnodeBank, vn: VarnodeId, op: OpId) {
    let seqnum: SeqNum = obank.get(op).unwrap().get_seq_num().clone();
    let mut noop = |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| Ok(());
    vbank.set_def(vn, DefOpInfo { id: op, seqnum }, &mut noop).unwrap();
}

// ---------------------------------------------------------------------------
// AT1 — gatherPieces byte-offset math is MIRRORED in big-endian.
// ---------------------------------------------------------------------------
//
// op.cc:900: offset = (rootVn->isBigEndian() == (i==1)) ? base + in(1-i).size : base.
// Little-endian (porter's test): slot 0 (hi) -> in1.size, slot 1 (lo) -> 0.
// Big-endian: the predicate flips, so slot 0 -> base (==0) and slot 1 -> in0.size.
// A coincidental "always slot-0-gets-the-size" shortcut would FAIL this test.
#[test]
fn at1_gather_pieces_big_endian_offsets_are_mirrored() {
    let m = manager(true); // BIG-endian ram
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut obank = PcodeOpBank::new();

    let hi = vbank.create(4, ram(&m, 0x0), dt(4)); // slot 0, 4 bytes
    let lo = vbank.create(4, ram(&m, 0x4), dt(4)); // slot 1, 4 bytes
    let out = vbank.create(8, ram(&m, 0x0), dt(8));
    let op = piece_op(&mut obank, &mut vbank, &m, 0x100, out, hi, lo);

    let mut stack: Vec<PieceNode> = Vec::new();
    gather_pieces(&mut stack, &obank, &vbank, out, op, 0, 0);

    assert_eq!(stack.len(), 2);
    // slot 0: predicate (BE == (0==1)) = (true == false) = false -> base = 0.
    assert_eq!(stack[0].get_slot(), 0);
    assert_eq!(stack[0].get_type_offset(), 0);
    assert!(stack[0].is_leaf());
    // slot 1: predicate (BE == (1==1)) = (true == true) = true -> base + in(0).size = 4.
    assert_eq!(stack[1].get_slot(), 1);
    assert_eq!(stack[1].get_type_offset(), 4);
    assert!(stack[1].is_leaf());
}

// ---------------------------------------------------------------------------
// AT2 — nested CONCAT tree: parent slots pushed before children (DFS), and the
// child offsets accumulate from the PARENT offset, not the root.
// ---------------------------------------------------------------------------
//
// Tree (little-endian):
//   out(8) = PIECE( mid(4) , c(4) )          [top op]
//   mid(4) = PIECE( a(2)   , b(2) )          [inner op]   (mid is a non-leaf)
// gatherPieces order must be: [top.slot0(mid), inner.slot0(a), inner.slot1(b),
//                              top.slot1(c)].
// Offsets (LE):
//   top.slot0 (mid): base(0) + in1(c).size(4) = 4  -> recurse with base=4
//     inner.slot0 (a): 4 + in1(b).size(2) = 6
//     inner.slot1 (b): 4
//   top.slot1 (c): 0
#[test]
fn at2_gather_pieces_nested_tree_dfs_order_and_accumulated_offsets() {
    let m = manager(false); // little-endian
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut obank = PcodeOpBank::new();

    let a = vbank.create(2, ram(&m, 0x10), dt(2));
    let b = vbank.create(2, ram(&m, 0x12), dt(2));
    let c = vbank.create(4, ram(&m, 0x20), dt(4));
    let mid = vbank.create(4, ram(&m, 0x30), dt(4));
    let out = vbank.create(8, ram(&m, 0x40), dt(8));

    // inner: mid = PIECE(a, b)
    let inner = piece_op(&mut obank, &mut vbank, &m, 0x200, mid, a, b);
    // mid must be WRITTEN by `inner` (and have exactly one descendant) to be a
    // non-leaf — otherwise isLeaf short-circuits on `!isWritten`.
    mark_written(&obank, &mut vbank, mid, inner);

    // top: out = PIECE(mid, c)
    let top = piece_op(&mut obank, &mut vbank, &m, 0x100, out, mid, c);

    let mut stack: Vec<PieceNode> = Vec::new();
    gather_pieces(&mut stack, &obank, &vbank, out, top, 0, 0);

    // 4 nodes: top.slot0(mid, non-leaf), inner.slot0(a), inner.slot1(b), top.slot1(c)
    assert_eq!(stack.len(), 4, "expected DFS to visit mid then a,b then c");

    // [0] top slot0 = mid, offset 4, NON-leaf (expanded).
    assert_eq!(stack[0].get_varnode(&obank), mid);
    assert_eq!(stack[0].get_type_offset(), 4);
    assert!(!stack[0].is_leaf());
    // [1] inner slot0 = a, offset 6 (accumulated from parent base 4).
    assert_eq!(stack[1].get_varnode(&obank), a);
    assert_eq!(stack[1].get_type_offset(), 6);
    assert!(stack[1].is_leaf());
    // [2] inner slot1 = b, offset 4.
    assert_eq!(stack[2].get_varnode(&obank), b);
    assert_eq!(stack[2].get_type_offset(), 4);
    assert!(stack[2].is_leaf());
    // [3] top slot1 = c, offset 0.
    assert_eq!(stack[3].get_varnode(&obank), c);
    assert_eq!(stack[3].get_type_offset(), 0);
    assert!(stack[3].is_leaf());
}

// ---------------------------------------------------------------------------
// AT3 — isLeaf loneDescend()==0 arm: a PIECE-defined varnode read by TWO ops is
// a leaf (op.cc:840-841), even though its def IS a PIECE.
// ---------------------------------------------------------------------------
#[test]
fn at3_piece_is_leaf_when_two_descendants_force_lone_descend_null() {
    let m = manager(false);
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut obank = PcodeOpBank::new();

    // mid = PIECE(a, b); mid is read by the top PIECE AND by a second consumer.
    let a = vbank.create(2, ram(&m, 0x10), dt(2));
    let b = vbank.create(2, ram(&m, 0x12), dt(2));
    let mid = vbank.create(4, ram(&m, 0x30), dt(4));
    let inner = piece_op(&mut obank, &mut vbank, &m, 0x200, mid, a, b);
    mark_written(&obank, &mut vbank, mid, inner);

    // First reader of mid (a COPY) and a SECOND reader (another COPY) so that
    // mid->loneDescend() == 0 (numDescend == 2).
    let r1 = obank.create_at(1, ram(&m, 0x300));
    obank.change_opcode(r1, TypeOp::new(OpCode::CPUI_COPY, 0, "copy"));
    obank.get_mut(r1).unwrap().set_input(Some(mid), 0);
    vbank.add_descend(mid, r1).unwrap();

    let r2 = obank.create_at(1, ram(&m, 0x310));
    obank.change_opcode(r2, TypeOp::new(OpCode::CPUI_COPY, 0, "copy"));
    obank.get_mut(r2).unwrap().set_input(Some(mid), 0);
    vbank.add_descend(mid, r2).unwrap();

    // mid is written by a PIECE but has 2 descendants -> loneDescend null -> leaf.
    assert!(
        piece_is_leaf(&obank, &vbank, /*root*/ mid, /*vn*/ mid, 0),
        "two readers must force the loneDescend()==0 leaf arm"
    );
}

// ---------------------------------------------------------------------------
// AT4 — isLeaf isAddrTied address-mismatch arm (op.cc:842-845): an addr-tied,
// PIECE-defined, lone-descended varnode whose address != root.getAddr()+relOff
// is a leaf; at the MATCHING address it is NOT.
// ---------------------------------------------------------------------------
#[test]
fn at4_piece_is_leaf_addrtied_address_mismatch() {
    let m = manager(false);
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut obank = PcodeOpBank::new();

    // root at ram:0x100 (8 bytes); the candidate `mid` is PIECE-defined,
    // addr-tied, lone-descended.
    let a = vbank.create(2, ram(&m, 0x10), dt(2));
    let b = vbank.create(2, ram(&m, 0x12), dt(2));
    let root = vbank.create(8, ram(&m, 0x100), dt(8));
    let mid = vbank.create(4, ram(&m, 0x104), dt(4)); // sits at root+4
    let inner = piece_op(&mut obank, &mut vbank, &m, 0x200, mid, a, b);
    mark_written(&obank, &mut vbank, mid, inner);
    // exactly one descendant (loneDescend != null)
    let r1 = obank.create_at(1, ram(&m, 0x300));
    obank.change_opcode(r1, TypeOp::new(OpCode::CPUI_COPY, 0, "copy"));
    obank.get_mut(r1).unwrap().set_input(Some(mid), 0);
    vbank.add_descend(mid, r1).unwrap();
    // make `mid` addr-tied so the isAddrTied arm is taken.
    vbank
        .get_mut(mid)
        .unwrap()
        .set_flags_pub(kuna_decomp::varnode::varnode_flags::addrtied);

    // relOffset == 4: root.getAddr()+4 == ram:0x104 == mid.getAddr() -> NOT leaf
    // (all guards pass through to `return false`).
    assert!(
        !piece_is_leaf(&obank, &vbank, root, mid, 4),
        "addr-tied vn at the matching address must NOT be a leaf"
    );
    // relOffset == 0: root.getAddr()+0 == ram:0x100 != ram:0x104 -> leaf.
    assert!(
        piece_is_leaf(&obank, &vbank, root, mid, 0),
        "addr-tied vn whose address != root+relOffset must be a leaf"
    );
}

// ---------------------------------------------------------------------------
// AT5 — spanningRange early-false boundary (ruleaction.cc:7537): a range that
// runs past the end of the structured type returns false (does NOT span), and
// never touches getSubType.
// ---------------------------------------------------------------------------
#[test]
fn at5_spanning_range_overrun_returns_false() {
    // A bare 8-byte STRUCT (metatype TYPE_STRUCT == 4 <= TYPE_ARRAY -> piece-structured).
    let ct = Rc::new(Datatype::new(8, type_metatype::TYPE_STRUCT));
    // offset 6 + size 4 = 10 > 8 -> the `offset+size > ct->getSize()` guard fires.
    assert!(
        !RulePieceStructure::spanning_range(&ct, 6, 4).unwrap(),
        "a range overrunning the struct must return false (not-spanning)"
    );
    // A range fully inside (offset 0 + size 8 == 8, not >) does NOT take the
    // early-false; with no concrete subtype factory the getSubType walk returns
    // None -> true ("don't know, assume multiple").
    assert!(
        RulePieceStructure::spanning_range(&ct, 0, 8).unwrap(),
        "an in-bounds range with no resolvable subtype assumes multiple (true)"
    );
}
