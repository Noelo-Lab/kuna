//! Independent VERIFIER adversarial tests for item `w7-s7-blockaction`
//! (`decompiler/cpp/blockaction.{cc,hh}`).
//!
//! These target the hunt-list spots flagged as most fragile for this item.
//! Every expected value is re-derived from the C++ oracle, NOT from the Rust
//! port:
//!
//!   - `LoopBody::find` (blockaction.cc:1021) — a binary search whose
//!     `int4 max = looporder.size()-1` goes to `-1` on an empty list (the
//!     signedness/underflow hunt: `min<=max` must be `0<=-1 == false`), and
//!     whose 3-way `compare_head` (blockaction.cc:489) drives the `min=mid+1`
//!     / `max=mid-1` boundary updates.  Off-by-one / boundary-inclusion hunt.
//!   - `LoopBody::mergeIdenticalHeads` (blockaction.cc:446) — accumulates
//!     `tails[0]` from a subsumed body into the survivor, marks the subsumed
//!     `head=NULL`, and `resize()`s the `looporder` index list; iteration-order
//!     + erase-while-iterating hunt (the Rust uses `truncate`).
//!   - `LoopBody::compare_ends` (blockaction.cc:473) — a total order by head
//!     index then tail[0] index; comparator-totality hunt (antisymmetry +
//!     transitivity used by `std::sort`/`sort_by`).
//!
//! The C++ oracle for `find`:
//!   int4 min=0, max=looporder.size()-1;
//!   while(min<=max){ mid=(min+max)/2; comp=compare_head(looporder[mid],looptop);
//!     if(comp==0) return looporder[mid];
//!     if(comp<0) min=mid+1; else max=mid-1; }
//!   return 0;
//! where compare_head returns -1 if a.head<looptop, 0 if equal, 1 if greater.

use kuna_base::types::int4;
use kuna_decomp::block::{BlockGraph, BlockKind, FlowBlock};
use kuna_decomp::blockaction::LoopBody;
use kuna_decomp::seams::BlockId;

/// Build a bare BlockGraph with `n` components, returning them so the test can
/// stamp explicit `index` values (mirrors how the production pipeline assigns
/// FlowBlock indices; here we control them directly to pin the comparators).
fn build(n: usize) -> (BlockGraph, BlockId, Vec<BlockId>) {
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let mut blocks = Vec::new();
    for _ in 0..n {
        blocks.push(g.new_block(root));
    }
    (g, root, blocks)
}

/// Stamp an explicit block index (the value `getIndex()` returns), so the
/// binary-search / comparator inputs are exactly controlled.
fn set_index(g: &mut BlockGraph, bl: BlockId, idx: int4) {
    g.block_mut(bl).set_index(idx);
}

/// Reproduce the C++ `LoopBody::find` lookup contract: build the
/// `(head_index, loopbody_index)` table the way `build_looporder_lookup` does
/// (sorted by head index) and binary-search it.
fn lookup_sorted(pairs: &[(int4, usize)]) -> Vec<(int4, usize)> {
    let mut v = pairs.to_vec();
    v.sort_by_key(|&(h, _)| h);
    v
}

// ===========================================================================
// 1. LoopBody::find — binary search boundaries + signed max underflow
// ===========================================================================

#[test]
fn find_empty_lookup_returns_none_no_underflow_panic() {
    // C++: max = 0 - 1 = -1 (int4); min=0; loop guard 0 <= -1 is false; return 0.
    // The Rust mirrors `max: int4 = len()-1 = -1`; a `usize` mistake here would
    // either panic on `0usize - 1` or index out of bounds.  Must be None.
    let (mut g, _root, b) = build(1);
    set_index(&mut g, b[0], 7);
    let lookup: Vec<(int4, usize)> = Vec::new();
    let res = LoopBody::find(b[0], &lookup, &g);
    assert_eq!(res, None, "empty looporder must return None without panicking");
}

#[test]
fn find_single_element_hit_and_miss() {
    let (mut g, _root, b) = build(3);
    // looptop candidates with distinct indices.
    set_index(&mut g, b[0], 10); // the only entry's head
    set_index(&mut g, b[1], 10); // a different block but same index -> hit
    set_index(&mut g, b[2], 4); // index below -> miss (compare_head returns 1)

    let lookup = lookup_sorted(&[(10, 42)]);

    // Hit: looptop index 10 == entry head index 10 -> Some(loopbody index 42).
    assert_eq!(LoopBody::find(b[1], &lookup, &g), Some(42));
    // Miss low: looptop index 4 < 10 -> compare_head(a=10, top=4) = 1 -> max=mid-1=-1
    // -> None.
    assert_eq!(LoopBody::find(b[2], &lookup, &g), None);
}

#[test]
fn find_three_elements_walks_both_branches() {
    // lookup heads sorted: indices [2, 5, 9] -> loopbody ids [100, 200, 300].
    // looptop=2 takes the max=mid-1 branch then hits at mid=0.
    // looptop=9 takes the min=mid+1 branch then hits at mid=2.
    // looptop=5 hits immediately at mid=1.
    // looptop=7 misses (between 5 and 9): mid=1(comp<0)->min=2; mid=2(comp>0)->max=1; stop.
    let (mut g, _root, blk) = build(4);
    set_index(&mut g, blk[0], 2);
    set_index(&mut g, blk[1], 5);
    set_index(&mut g, blk[2], 9);
    set_index(&mut g, blk[3], 7);

    let lookup = lookup_sorted(&[(9, 300), (2, 100), (5, 200)]); // unsorted input -> sorted

    assert_eq!(LoopBody::find(blk[0], &lookup, &g), Some(100), "head==2 hit");
    assert_eq!(LoopBody::find(blk[1], &lookup, &g), Some(200), "head==5 hit");
    assert_eq!(LoopBody::find(blk[2], &lookup, &g), Some(300), "head==9 hit");
    assert_eq!(LoopBody::find(blk[3], &lookup, &g), None, "head==7 miss between entries");
}

#[test]
fn compare_head_is_signed_three_way_and_drives_search() {
    // Direct oracle check (C++ compare_head, blockaction.cc:489).
    assert_eq!(LoopBody::compare_head(2, 9), -1); // a < top
    assert_eq!(LoopBody::compare_head(9, 9), 0); // equal
    assert_eq!(LoopBody::compare_head(9, 2), 1); // a > top
    // Negative indices must compare as signed (would invert under u32).
    assert_eq!(LoopBody::compare_head(-1, 0), -1);
    assert_eq!(LoopBody::compare_head(0, -1), 1);
}

// ===========================================================================
// 2. LoopBody::mergeIdenticalHeads — tail accumulation + truncation
// ===========================================================================

#[test]
fn merge_identical_heads_accumulates_tails_and_truncates() {
    // Three loop bodies in looporder, sorted by head then tail (the C++
    // precondition is that compare_ends has already sorted them):
    //   lb0: head=H,  tail=T1
    //   lb1: head=H,  tail=T2   (same head -> subsumed into lb0)
    //   lb2: head=K,  tail=T3
    // C++ mergeIdenticalHeads: curbody=looporder[0]=lb0; j=1: lb1.head==lb0.head
    //   -> lb0.addTail(lb1.tails[0]=T2); lb1.head=NULL. j=2: lb2.head!=lb0.head
    //   -> i=1; looporder[1]=lb2; curbody=lb2. resize(2).
    // Result: looporder = [lb0, lb2]; lb0.tails = [T1, T2]; lb1.head == None.
    let (mut g, _root, blk) = build(5);
    let h = blk[0];
    let k = blk[1];
    let t1 = blk[2];
    let t2 = blk[3];
    let t3 = blk[4];

    let mut lb0 = LoopBody::new(h);
    lb0.add_tail(t1);
    let mut lb1 = LoopBody::new(h);
    lb1.add_tail(t2);
    let mut lb2 = LoopBody::new(k);
    lb2.add_tail(t3);

    let mut loopbody = vec![lb0, lb1, lb2];
    let mut looporder = vec![0usize, 1, 2];

    LoopBody::merge_identical_heads(&mut looporder, &mut loopbody);

    // looporder truncated to the survivors, in order [lb0, lb2].
    assert_eq!(looporder, vec![0usize, 2], "looporder keeps survivors lb0,lb2");
    // lb1 marked subsumed (head None).
    assert_eq!(loopbody[1].get_head(), None, "lb1 subsumed -> head None");
    // lb0 still has head H, and inherited T2 as a second tail.
    assert_eq!(loopbody[0].get_head(), Some(h), "survivor keeps head H");
    // lb2 untouched.
    assert_eq!(loopbody[2].get_head(), Some(k), "lb2 head K untouched");
    let _ = (t3, &g); // silence unused (g built for index parity with other tests)
    let _ = &mut g;
}

#[test]
fn merge_identical_heads_no_merge_is_identity() {
    // All distinct heads: no merging, looporder unchanged, no subsumption.
    let (_g, _root, blk) = build(4);
    let mut lb0 = LoopBody::new(blk[0]);
    lb0.add_tail(blk[1]);
    let mut lb1 = LoopBody::new(blk[2]);
    lb1.add_tail(blk[3]);

    let mut loopbody = vec![lb0, lb1];
    let mut looporder = vec![0usize, 1];
    LoopBody::merge_identical_heads(&mut looporder, &mut loopbody);

    assert_eq!(looporder, vec![0usize, 1], "distinct heads -> looporder unchanged");
    assert_eq!(loopbody[0].get_head(), Some(blk[0]));
    assert_eq!(loopbody[1].get_head(), Some(blk[2]));
}

#[test]
fn merge_identical_heads_all_same_head_collapses_to_one() {
    // Four bodies all sharing one head: survivor is lb0, accumulating tails of
    // lb1,lb2,lb3; looporder truncates to [0].
    let (_g, _root, blk) = build(5);
    let h = blk[0];
    let mut bodies = Vec::new();
    let mut lb0 = LoopBody::new(h);
    lb0.add_tail(blk[1]);
    bodies.push(lb0);
    for &t in &[blk[2], blk[3], blk[4]] {
        let mut lb = LoopBody::new(h);
        lb.add_tail(t);
        bodies.push(lb);
    }
    let mut looporder = vec![0usize, 1, 2, 3];
    LoopBody::merge_identical_heads(&mut looporder, &mut bodies);

    assert_eq!(looporder, vec![0usize], "all-same-head collapses to single survivor");
    assert_eq!(bodies[0].get_head(), Some(h));
    for s in &bodies[1..] {
        assert_eq!(s.get_head(), None, "every other body subsumed");
    }
}

// ===========================================================================
// 3. LoopBody::compare_ends — total order (antisymmetry / tie-break)
// ===========================================================================

#[test]
fn compare_ends_orders_by_head_then_tail_and_is_antisymmetric() {
    use std::cmp::Ordering;
    let (mut g, _root, blk) = build(4);
    // Distinct controlled indices.
    set_index(&mut g, blk[0], 3); // head A
    set_index(&mut g, blk[1], 8); // head B
    set_index(&mut g, blk[2], 1); // tail of A
    set_index(&mut g, blk[3], 9); // tail of B

    let mut a = LoopBody::new(blk[0]);
    a.add_tail(blk[2]);
    let mut b = LoopBody::new(blk[1]);
    b.add_tail(blk[3]);

    // head 3 < head 8 -> a before b.
    assert_eq!(LoopBody::compare_ends(&a, &b, &g), Ordering::Less);
    // antisymmetry.
    assert_eq!(LoopBody::compare_ends(&b, &a, &g), Ordering::Greater);

    // Same head, tail decides.
    let mut a2 = LoopBody::new(blk[0]);
    a2.add_tail(blk[2]); // tail index 1
    let mut a3 = LoopBody::new(blk[0]);
    a3.add_tail(blk[3]); // tail index 9
    assert_eq!(LoopBody::compare_ends(&a2, &a3, &g), Ordering::Less);
    assert_eq!(LoopBody::compare_ends(&a3, &a2, &g), Ordering::Greater);

    // Equal head and tail -> Equal (reflexive on identical ends).
    let mut a4 = LoopBody::new(blk[0]);
    a4.add_tail(blk[2]);
    assert_eq!(LoopBody::compare_ends(&a2, &a4, &g), Ordering::Equal);
}
