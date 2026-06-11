//! Independent verifier adversarial tests for item `w3-ir-funcdata`
//! (`decompiler/cpp/funcdata.{hh,cc}`, `funcdata_block.cc`).
//!
//! Targets the hunt-list spots flagged most fragile for this item:
//!   - `replace_reads_thunk` vs the C++ `VarnodeBank::replace` (`varnode.cc:1351`)
//!     — the read-repointing the varnode wave delegated here.  The C++ has an
//!     `if (op->output == newvn) continue;` skip (an op cannot become an input
//!     to its own definition).  The repaired thunk reproduces it; the
//!     `thunk_repoints_self_def_*` tests pin that the self-def slot stays on
//!     oldvn and that an ordinary reader still repoints, against the C++ oracle.
//!   - `BlockBasic::insert` SeqNum order assignment (`block.cc:2262`): the
//!     midpoint computation, the `~0` overflow clamp when appending at the end,
//!     and the `ordafter-ordbefore<=1` fallback to a full `setOrder()`.
//!   - `BlockBasic::setOrder` (`block.cc:2686`) step = `~0/size - 1`: strictly
//!     increasing, gap-spread orders across a large block.
//!   - `spliceBlockBasic` op-list merge order + final `setOrder()`
//!     (`funcdata_block.cc:927`).
//!
//! Each assertion is re-derived from the C++ oracle, not from the Rust port.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uintm};
use kuna_num::opcodes::OpCode;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::{pcodeop_flags, PcodeOpBank};
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp};
use kuna_decomp::varnode::VarnodeBank;

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

fn unk_type() -> Rc<Datatype> {
    Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN))
}

fn copy_op(fd: &mut Funcdata, inputs: int4, off: u64) -> OpId {
    let ram = ram_space(fd);
    let op = fd.obank_mut().create_at(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    op
}

fn new_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.unwrap();
    fd.bblocks_mut().new_block_basic(root)
}

fn order_of(fd: &Funcdata, op: OpId) -> uintm {
    fd.obank().get(op).unwrap().get_seq_num().get_order()
}

// ---------------------------------------------------------------------------
// F1: replace_reads_thunk omits the C++ `op->output == newvn` skip.
//
// C++ VarnodeBank::replace(oldvn,newvn) (varnode.cc:1351):
//   while(iter!=oldvn->descend.end()) {
//     op = *iter; ...
//     if (op->output == newvn) continue;   // <-- THE SKIP under test
//     i = op->getSlot(oldvn); ... op->setInput(newvn,i);
//   }
//
// Scenario: op DEFINES newvn (op->output == newvn) and READS oldvn in slot 0.
// C++ leaves slot 0 reading oldvn (self-definition avoided).  The thunk
// repoints slot 0 to newvn, making op read its own output.
// ---------------------------------------------------------------------------
#[test]
fn thunk_repoints_self_def_input_cpp_skips_it() {
    let manage = build_manager();
    let ram = Rc::clone(manage.get_space_by_name("ram").unwrap());
    let ct = unk_type();
    let mut bank = VarnodeBank::new(&manage, 0x10000000).unwrap();
    let mut obank = PcodeOpBank::new();

    // oldvn (to be replaced) and newvn (the survivor / op's own output).
    let oldvn = bank.create(4, Address::new(Rc::clone(&ram), 0x100), Rc::clone(&ct));
    let newvn = bank.create(4, Address::new(Rc::clone(&ram), 0x200), Rc::clone(&ct));

    // op: output == newvn, input[0] == oldvn.
    let op = obank.create_at(1, Address::new(Rc::clone(&ram), 0x1000));
    obank.change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    obank.get_mut(op).unwrap().set_output(Some(newvn));
    obank.get_mut(op).unwrap().set_input(Some(oldvn), 0);
    // oldvn is read by op (descend link, as the op-graph would maintain).
    bank.add_descend(oldvn, op).unwrap();

    // Run the production thunk exactly as xref would.
    {
        let mut thunk = Funcdata::replace_reads_thunk(&mut obank);
        thunk(&mut bank, oldvn, newvn).unwrap();
    }

    let in0 = obank.get(op).unwrap().get_in(0);
    // C++ oracle: `if (op->output == newvn) continue;` (varnode.cc:1362) skips
    // this op, leaving input[0] == oldvn — an op never becomes an input to its
    // own definition.  The repaired thunk now restores that skip.
    assert_eq!(
        in0,
        Some(oldvn),
        "self-definition input must stay reading oldvn (C++ replace skips op->output==newvn)"
    );
    assert_ne!(in0, Some(newvn), "self-def slot must NOT be repointed to newvn (would self-reference)");
    // C++ leaves the skipped op's descend link on oldvn intact (the erase only
    // runs for repointed entries); the blanket destroy is gone.
    assert_eq!(
        bank.get(oldvn).unwrap().descend_iter().filter(|&o| o == op).count(),
        1,
        "skipped self-def op keeps its descend link on oldvn"
    );
    // newvn gained no descend entry for the skipped op.
    assert_eq!(
        bank.get(newvn).unwrap().descend_iter().filter(|&o| o == op).count(),
        0,
        "self-def op is not added to newvn's descend"
    );
}

// ---------------------------------------------------------------------------
// F1 (companion): when op->output != newvn the thunk and C++ AGREE.
// This pins that the divergence is specifically the self-def case, not a
// blanket mis-port of replace.
// ---------------------------------------------------------------------------
#[test]
fn thunk_repoints_ordinary_reader_matches_cpp() {
    let manage = build_manager();
    let ram = Rc::clone(manage.get_space_by_name("ram").unwrap());
    let ct = unk_type();
    let mut bank = VarnodeBank::new(&manage, 0x10000000).unwrap();
    let mut obank = PcodeOpBank::new();

    let oldvn = bank.create(4, Address::new(Rc::clone(&ram), 0x100), Rc::clone(&ct));
    let newvn = bank.create(4, Address::new(Rc::clone(&ram), 0x200), Rc::clone(&ct));
    let other = bank.create(4, Address::new(Rc::clone(&ram), 0x300), Rc::clone(&ct));

    // op reads oldvn in slot 0, slot 1 holds `other`; output is `other`
    // (!= newvn), so the self-def skip does NOT apply.
    let op = obank.create_at(2, Address::new(Rc::clone(&ram), 0x1000));
    obank.change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    obank.get_mut(op).unwrap().set_output(Some(other));
    obank.get_mut(op).unwrap().set_input(Some(oldvn), 0);
    obank.get_mut(op).unwrap().set_input(Some(other), 1);
    // oldvn is free; a single descendant is permitted.
    bank.add_descend(oldvn, op).unwrap();

    {
        let mut thunk = Funcdata::replace_reads_thunk(&mut obank);
        thunk(&mut bank, oldvn, newvn).unwrap();
    }

    // The oldvn slot repointed to newvn; slot 1 (other) untouched.
    assert_eq!(obank.get(op).unwrap().get_in(0), Some(newvn));
    assert_eq!(obank.get(op).unwrap().get_in(1), Some(other));
    // newvn now reads op once (the single repointed slot).
    let cnt = bank.get(newvn).unwrap().descend_iter().filter(|&o| o == op).count();
    assert_eq!(cnt, 1, "descend gains exactly one entry for the repointed slot");
    // oldvn fully detached.
    assert_eq!(bank.get(oldvn).unwrap().num_descend(), 0);
}

// ---------------------------------------------------------------------------
// F2: BlockBasic::insert order assignment — append fills with the +0x1000000
// step until it saturates, then the <=1 gap forces setOrder(); a mid-insert
// always lands strictly between its neighbours.  (block.cc:2262)
// ---------------------------------------------------------------------------
#[test]
fn bb_insert_orders_strictly_increasing_and_midpoint() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);

    // Append 5 ops; each gets ordbefore+0x800000 (midpoint of the +0x1000000 gap)
    // off the prior, all strictly increasing.
    let mut ops = Vec::new();
    for i in 0..5 {
        let op = copy_op(&mut fd, 1, 0x1000 + i * 4);
        fd.bb_insert_op(op, bl, None);
        ops.push(op);
    }
    let orders: Vec<uintm> = ops.iter().map(|&o| order_of(&fd, o)).collect();
    for w in orders.windows(2) {
        assert!(w[0] < w[1], "append orders must strictly increase: {orders:?}");
    }
    // First appended op: prev=None -> ordbefore=2, ordafter=2+0x1000000=0x1000002,
    // mid = 0x1000002/2 + 2/2 = 0x800001 + 1 = 0x800002.
    assert_eq!(orders[0], 0x800002, "first-append midpoint (C++ ordafter/2 + ordbefore/2)");

    // Insert before ops[2]; new order must be strictly between ops[1] and ops[2].
    let mid = copy_op(&mut fd, 1, 0x9000);
    fd.bb_insert_op(mid, bl, Some(ops[2]));
    let om = order_of(&fd, mid);
    let o1 = order_of(&fd, ops[1]);
    let o2 = order_of(&fd, ops[2]);
    assert!(o1 < om && om < o2, "mid-insert must land strictly between neighbours: {o1} < {om} < {o2}");
    assert_eq!(fd.bb_ops(bl), vec![ops[0], ops[1], mid, ops[2], ops[3], ops[4]]);
}

// ---------------------------------------------------------------------------
// F2b: forcing the `ordafter-ordbefore<=1` fallback. Insert repeatedly before
// the SAME op (front of a tight pair) until the gap collapses and setOrder()
// re-spreads every op; the list must still be totally ordered and intact.
// ---------------------------------------------------------------------------
#[test]
fn bb_insert_gap_collapse_triggers_full_reorder() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let anchor = copy_op(&mut fd, 1, 0x1000);
    fd.bb_insert_op(anchor, bl, None);

    // Insert 64 ops all before `anchor`. The midpoints keep halving the
    // [2, anchor.order] interval; once ordafter-ordbefore<=1, setOrder() fires
    // and re-spreads. Either way every order stays distinct and increasing.
    let mut inserted = Vec::new();
    for i in 0..64 {
        let op = copy_op(&mut fd, 1, 0x2000 + i * 4);
        fd.bb_insert_op(op, bl, Some(anchor));
        inserted.push(op);
    }
    let listed = fd.bb_ops(bl);
    assert_eq!(listed.len(), 65);
    assert_eq!(*listed.last().unwrap(), anchor, "anchor stays at the tail");
    // Orders strictly increase across the whole block (totality after any
    // setOrder fallback).
    let orders: Vec<uintm> = listed.iter().map(|&o| order_of(&fd, o)).collect();
    for w in orders.windows(2) {
        assert!(w[0] < w[1], "post-collapse orders must be strictly increasing");
    }
    // All distinct.
    let mut sorted = orders.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), orders.len(), "no duplicate SeqNum orders after reorder");
}

// ---------------------------------------------------------------------------
// F3: spliceBlockBasic merges outbl's ops to the tail of bl in order, then
// runs a clean setOrder().  (funcdata_block.cc:927)  Empty bl + non-empty
// outbl is the boundary the per-insert path must still reorder correctly.
// ---------------------------------------------------------------------------
#[test]
fn splice_empty_bl_inherits_outbl_ops_in_order() {
    let mut fd = build_fd();
    let rs = ram_space(&fd);
    let root = fd.bblocks_ref().root.unwrap();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let outbl = fd.bblocks_mut().new_block_basic(root);
    let succ = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(bl, outbl);
    fd.bblocks_mut().add_edge(outbl, succ);
    fd.set_basic_block_range(bl, &Address::new(Rc::clone(&rs), 0x1000), &Address::new(Rc::clone(&rs), 0x1004));
    fd.set_basic_block_range(outbl, &Address::new(Rc::clone(&rs), 0x2000), &Address::new(Rc::clone(&rs), 0x2008));

    // bl empty; outbl holds 3 plain COPYs.
    let mut outops = Vec::new();
    for i in 0..3 {
        let op = copy_op(&mut fd, 1, 0x2000 + i * 4);
        fd.bb_insert_op(op, outbl, None);
        outops.push(op);
    }

    fd.splice_block_basic(bl).unwrap();

    // bl now holds outbl's ops in their original order.
    assert_eq!(fd.bb_ops(bl), outops);
    // Orders strictly increasing after the trailing setOrder().
    let orders: Vec<uintm> = outops.iter().map(|&o| order_of(&fd, o)).collect();
    for w in orders.windows(2) {
        assert!(w[0] < w[1], "spliced op orders must be strictly increasing");
    }
    // bl inherited outbl's out-edge to succ; cover spans both ranges.
    assert_eq!(fd.bblocks_ref().block(bl).get_out(0), succ);
    if let kuna_decomp::block::BlockKind::Basic(b) = fd.bblocks_ref().block(bl).kind() {
        assert_eq!(b.cover.num_ranges(), 2);
    } else {
        panic!("bl not basic");
    }
}

// ---------------------------------------------------------------------------
// F4: bb_set_order step = ~0/size - 1 spreads orders with the expected gap and
// stays strictly increasing for a large block (block.cc:2686).
// ---------------------------------------------------------------------------
#[test]
fn bb_set_order_spreads_with_uniform_step() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let n: u64 = 100;
    let mut ops = Vec::new();
    for i in 0..n {
        let op = copy_op(&mut fd, 1, 0x1000 + i * 4);
        fd.bb_insert_op(op, bl, None);
        ops.push(op);
    }
    fd.bb_set_order(bl);
    // step = u32::MAX / 100 - 1 = 42949671; count_i = step*(i+1).
    let step: uintm = (u32::MAX / (n as u32)).wrapping_sub(1);
    for (i, &op) in ops.iter().enumerate() {
        let expected = step.wrapping_mul((i as u32) + 1);
        assert_eq!(order_of(&fd, op), expected, "setOrder count mismatch at index {i}");
    }
    // Strictly increasing across the whole block.
    let orders: Vec<uintm> = ops.iter().map(|&o| order_of(&fd, o)).collect();
    for w in orders.windows(2) {
        assert!(w[0] < w[1]);
    }
}

// ---------------------------------------------------------------------------
// F5: bb_remove_op then re-insert round-trips head/tail/len for every position
// (head, middle, tail) — the intrusive-list splice fidelity of ADR 0001.
// ---------------------------------------------------------------------------
#[test]
fn bb_remove_reinsert_roundtrips_links() {
    let mut fd = build_fd();
    let bl = new_block(&mut fd);
    let a = copy_op(&mut fd, 1, 0x1000);
    let b = copy_op(&mut fd, 1, 0x1004);
    let c = copy_op(&mut fd, 1, 0x1008);
    fd.bb_insert_op(a, bl, None);
    fd.bb_insert_op(b, bl, None);
    fd.bb_insert_op(c, bl, None);

    // Remove middle, re-insert before c -> same list.
    fd.bb_remove_op(bl, b);
    assert_eq!(fd.bb_ops(bl), vec![a, c]);
    fd.bb_insert_op(b, bl, Some(c));
    assert_eq!(fd.bb_ops(bl), vec![a, b, c]);
    assert_eq!(fd.bb_op_head(bl), Some(a));
    assert_eq!(fd.bb_op_tail(bl), Some(c));
    assert_eq!(fd.bb_op_len(bl), 3);

    // Remove head and tail, leaving the middle as both head and tail.
    fd.bb_remove_op(bl, a);
    fd.bb_remove_op(bl, c);
    assert_eq!(fd.bb_op_head(bl), Some(b));
    assert_eq!(fd.bb_op_tail(bl), Some(b));
    assert_eq!(fd.bb_op_len(bl), 1);
    // Removed ops carry no parent.
    assert_eq!(fd.obank().get(a).unwrap().get_parent(), None);
    assert_eq!(fd.obank().get(c).unwrap().get_parent(), None);
}

// A startbasic flag is cleared on the spliced head op (funcdata_block.cc:949).
#[test]
fn splice_clears_startbasic_on_merged_head() {
    let mut fd = build_fd();
    let rs = ram_space(&fd);
    let root = fd.bblocks_ref().root.unwrap();
    let bl = fd.bblocks_mut().new_block_basic(root);
    let outbl = fd.bblocks_mut().new_block_basic(root);
    let succ = fd.bblocks_mut().new_block_basic(root);
    fd.bblocks_mut().add_edge(bl, outbl);
    fd.bblocks_mut().add_edge(outbl, succ);
    fd.set_basic_block_range(bl, &Address::new(Rc::clone(&rs), 0x1000), &Address::new(Rc::clone(&rs), 0x1000));
    fd.set_basic_block_range(outbl, &Address::new(Rc::clone(&rs), 0x2000), &Address::new(Rc::clone(&rs), 0x2000));

    let head = copy_op(&mut fd, 1, 0x2000);
    // Mark the outbl head op startbasic, as the flow builder would.
    fd.obank_mut().get_mut(head).unwrap().set_flag(pcodeop_flags::startbasic);
    fd.bb_insert_op(head, outbl, None);

    fd.splice_block_basic(bl).unwrap();
    let flags = fd.obank().get(head).unwrap().get_flags();
    assert_eq!(flags & pcodeop_flags::startbasic, 0, "startbasic must be cleared on the merged head");
}
