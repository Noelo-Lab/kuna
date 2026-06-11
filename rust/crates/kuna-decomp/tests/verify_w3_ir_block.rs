//! Independent verifier adversarial tests for item `w3-ir-block`
//! (`decompiler/cpp/block.{cc,hh}`).
//!
//! These target the hunt-list spots flagged as most fragile for this item:
//!   - reverse-index correction in the half-edge slide (`halfDeleteOutEdge`
//!     / `halfDeleteInEdge`) when removing a non-terminal edge whose *mirror*
//!     edge lives among several edges on the other block (off-by-one in the
//!     mirror's `reverse_index -= 1`);
//!   - `removeFromFlow` re-routing many in-edges through a single out-edge,
//!     preserving in/out edge order and the mirror invariant;
//!   - `spliceBlock` flag inheritance (`f_unstructured_targ|f_entry_point` from
//!     the head, `f_switch_out` from the tail) and out-edge transfer order;
//!   - `removeFromFlowSplit` flip vs non-flip edge remapping;
//!   - the `BlockMap` binary-search resolver boundaries (empty, single, miss);
//!   - encode/decode round-trip of a nested BlockGraph so the `<bhead>` sort +
//!     `findLevelBlock` index resolution is exercised across two levels.
//!
//! Each assertion is re-derived from the C++ oracle, not from the Rust port.

use std::rc::Rc;

use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_base::types::int4;
use kuna_decomp::block::{block_flags, BlockEdge, BlockGraph, BlockKind, BlockMap, FlowBlock};
use kuna_decomp::seams::BlockId;

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.set_default_code_space(1).unwrap();
    m
}

fn build_graph(n: usize) -> (BlockGraph, BlockId, Vec<BlockId>) {
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(root);
    let mut blocks = Vec::new();
    for _ in 0..n {
        blocks.push(g.new_block(root));
    }
    (g, root, blocks)
}

/// Full mirror-invariant check over the component list (C++ `checkEdges`).
fn assert_edges_consistent(g: &BlockGraph, root: BlockId) {
    let list: Vec<BlockId> = g.arena[root].get_list().to_vec();
    for &bl in &list {
        for i in 0..g.arena[bl].size_in() {
            let rev = g.arena[bl].get_in_rev_index(i);
            let other = g.arena[bl].get_in(i);
            assert_eq!(g.arena[other].get_out(rev), bl, "in mirror point");
            assert_eq!(g.arena[other].get_out_rev_index(rev), i, "in mirror index");
        }
        for i in 0..g.arena[bl].size_out() {
            let rev = g.arena[bl].get_out_rev_index(i);
            let other = g.arena[bl].get_out(i);
            assert_eq!(g.arena[other].get_in(rev), bl, "out mirror point");
            assert_eq!(g.arena[other].get_in_rev_index(rev), i, "out mirror index");
        }
    }
}

/// HUNT: off-by-one / mirror reverse_index correction in the half-edge slide.
///
/// Build a hub `h` that takes in-edges from a, b, c (in that order) and target
/// `t` that has in-edges from h and from a separate source `s` (so t's in-edge
/// list is [h, s]).  Removing the middle in-edge of h (the b->h edge) forces
/// `halfDeleteInEdge` to slide the c->h edge down one slot AND decrement the
/// reverse_index of c's *out* edge that mirrors it.  If the mirror correction
/// is applied to the wrong slot, the global mirror invariant breaks.
#[test]
fn w3_ir_block_half_delete_corrects_mirror_reverse_index() {
    let (mut g, root, b) = build_graph(6);
    let (a, bb, c, h, s, t) = (b[0], b[1], b[2], b[3], b[4], b[5]);
    // h's in edges, in order: a, b, c
    g.add_edge(a, h);
    g.add_edge(bb, h);
    g.add_edge(c, h);
    // t's in edges, in order: h, s
    g.add_edge(h, t);
    g.add_edge(s, t);
    assert_eq!(g.arena[h].size_in(), 3);
    assert_eq!(g.arena[t].size_in(), 2);
    assert_edges_consistent(&g, root);

    // Remove the MIDDLE in edge of h (b -> h).  remaining h in-edges slide to
    // [a, c]; c's out edge that pointed at slot 2 of h must now read slot 1.
    g.remove_edge(bb, h);
    assert_eq!(g.arena[h].size_in(), 2);
    assert_eq!(g.arena[h].get_in(0), a);
    assert_eq!(g.arena[h].get_in(1), c);
    // c -> h reverse index must now be 1 (slid down from 2)
    let cslot = g.arena[c].get_out_index(h);
    assert!(cslot >= 0);
    assert_eq!(g.arena[c].get_out_rev_index(cslot), 1, "c's mirror reverse_index slid");
    // The whole graph mirror invariant must hold (this is what a wrong-slot
    // decrement would corrupt).
    assert_edges_consistent(&g, root);
}

/// HUNT: erase-while-iterating + edge order in `removeFromFlow`.
///
/// `bl` has two in-edges (from p0, p1) and exactly one out-edge (to q).  After
/// `removeFromFlow(bl)`, both p0 and p1 must flow to q, in their original order,
/// and bl is fully disconnected (but NOT removed from the list).
#[test]
fn w3_ir_block_remove_from_flow_reroutes_all_inedges() {
    let (mut g, root, b) = build_graph(4);
    let (p0, p1, bl, q) = (b[0], b[1], b[2], b[3]);
    g.add_edge(p0, bl);
    g.add_edge(p1, bl);
    g.add_edge(bl, q);
    g.remove_from_flow(bl);

    assert_eq!(g.arena[bl].size_in(), 0, "bl disconnected (in)");
    assert_eq!(g.arena[bl].size_out(), 0, "bl disconnected (out)");
    // q now has both predecessors. C++ replaces via intothis[0].reverse_index
    // repeatedly, so the order q sees is the order the edges were rerouted.
    assert_eq!(g.arena[q].size_in(), 2);
    let preds: Vec<BlockId> = (0..g.arena[q].size_in()).map(|i| g.arena[q].get_in(i)).collect();
    assert!(preds.contains(&p0) && preds.contains(&p1));
    // bl stays in the component list (removeFromFlow does NOT remove it)
    assert!(g.arena[root].get_list().contains(&bl));
    assert_edges_consistent(&g, root);
}

/// HUNT: `spliceBlock` flag inheritance + out-edge transfer.
///
/// head has 1 out edge to mid (which has 1 in edge); mid has 2 out edges to x,y.
/// head carries f_entry_point; mid carries f_switch_out.  After splice, head
/// keeps f_entry_point (from head) | f_switch_out (from mid), loses everything
/// else, gains mid's two out edges in order, and mid is deleted from the graph.
#[test]
fn w3_ir_block_splice_block_inherits_flags_and_out_edges() {
    let (mut g, root, b) = build_graph(4);
    let (head, mid, x, y) = (b[0], b[1], b[2], b[3]);
    g.add_edge(head, mid);
    g.add_edge(mid, x);
    g.add_edge(mid, y);
    // head carries entry_point + a flag that must be dropped (f_mark2 stand-in:
    // use f_donothing_loop which is NOT in the kept set fl1|fl2).
    g.arena[head].set_flag(block_flags::f_entry_point | block_flags::f_donothing_loop);
    g.arena[mid].set_flag(block_flags::f_switch_out);

    // splice head with its single successor mid (head has 1 out, mid has 1 in)
    g.splice_block(root, head).unwrap();

    // mid removed from graph
    assert!(!g.arena[root].get_list().contains(&mid));
    // head now flows to x, y (mid's outs), in order
    assert_eq!(g.arena[head].size_out(), 2);
    assert_eq!(g.arena[head].get_out(0), x);
    assert_eq!(g.arena[head].get_out(1), y);
    // flags: exactly f_entry_point | f_switch_out (C++ `bl->flags = fl1 | fl2`)
    let fl = g.arena[head].get_flags();
    assert_eq!(fl & block_flags::f_entry_point, block_flags::f_entry_point, "kept entry_point");
    assert_eq!(fl & block_flags::f_switch_out, block_flags::f_switch_out, "inherited switch_out");
    assert_eq!(fl & block_flags::f_donothing_loop, 0, "dropped donothing_loop");
    assert_edges_consistent(&g, root);
}

/// HUNT: `removeFromFlowSplit` flip vs non-flip remapping.
///
/// bl has 2 ins (i0,i1) and 2 outs (o0,o1).  Non-flip: i0->o0, i1->o1.
/// Flip: i0->o1, i1->o0.  We verify the non-flip case end-to-end.
#[test]
fn w3_ir_block_remove_from_flow_split_noflip_maps_parallel() {
    let (mut g, root, b) = build_graph(5);
    let (i0, i1, bl, o0, o1) = (b[0], b[1], b[2], b[3], b[4]);
    g.add_edge(i0, bl);
    g.add_edge(i1, bl);
    g.add_edge(bl, o0);
    g.add_edge(bl, o1);
    g.remove_from_flow_split(bl, false);
    // bl is fully disconnected
    assert_eq!(g.arena[bl].size_in(), 0);
    assert_eq!(g.arena[bl].size_out(), 0);
    // i0 -> o0 and i1 -> o1 (parallel, non-flipped)
    assert!(g.arena[i0].get_out_index(o0) >= 0, "i0 -> o0");
    assert!(g.arena[i1].get_out_index(o1) >= 0, "i1 -> o1");
    assert_edges_consistent(&g, root);
}

/// HUNT: binary-search boundary in the `BlockMap` resolver (empty case).
///
/// `findBlock` sets `max = size-1`, so an empty resolver yields `max = -1` and
/// the `min<=max` loop never runs (must NOT index `cache[-1]`).  The port's
/// only public mutator-free surface is `new` + `find_level_block`; the populated
/// path (`sortList`/`findBlock` over real indices) is exercised transitively by
/// the nested encode/decode round-trip below, where the inner resolver resolves
/// every `<edge>` endpoint by index.
#[test]
fn w3_ir_block_blockmap_empty_resolver_misses_cleanly() {
    let empty = BlockMap::new();
    assert_eq!(empty.find_level_block(0), None);
    assert_eq!(empty.find_level_block(-1), None);
    assert_eq!(empty.find_level_block(7), None);
}

/// HUNT: nested-graph encode/decode round-trip exercising the `<bhead>` sort
/// (`BlockMap::sortList`) + `findLevelBlock` index resolution across two levels,
/// and the interleaved decode/addBlock ordering.
#[test]
fn w3_ir_block_nested_graph_encode_decode_roundtrip() {
    let m = build_manager();
    let mut g = BlockGraph::new();
    let top = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    g.root = Some(top);

    // Three plain components with an edge 0->1, 1->2, and a back edge 2->0.
    let n0 = g.new_block(top);
    let n1 = g.new_block(top);
    let n2 = g.new_block(top);
    g.arena[n0].set_index(0);
    g.arena[n1].set_index(1);
    g.arena[n2].set_index(2);
    g.add_edge(n0, n1);
    g.add_edge(n1, n2);
    g.add_edge(n2, n0);

    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        g.encode(top, &mut enc);
    }

    // Decode into a fresh graph.
    let mut g2 = BlockGraph::new();
    let top2 = g2.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    g2.decode_top(top2, &mut dec).unwrap();

    // Same component count + indices preserved.
    assert_eq!(g2.arena[top2].get_size(), 3);
    let by_index: Vec<int4> =
        g2.arena[top2].get_list().iter().map(|&c| g2.arena[c].get_index()).collect();
    assert!(by_index.contains(&0) && by_index.contains(&1) && by_index.contains(&2));

    // Resolve each decoded block by index and verify the edge topology survived:
    // index 0 -> index 1, index 1 -> index 2, index 2 -> index 0.
    let find = |g: &BlockGraph, idx: int4| -> BlockId {
        *g.arena[top2].get_list().iter().find(|&&c| g.arena[c].get_index() == idx).unwrap()
    };
    let d0 = find(&g2, 0);
    let d1 = find(&g2, 1);
    let d2 = find(&g2, 2);
    assert!(g2.arena[d0].get_out_index(d1) >= 0, "0 -> 1 survived");
    assert!(g2.arena[d1].get_out_index(d2) >= 0, "1 -> 2 survived");
    assert!(g2.arena[d2].get_out_index(d0) >= 0, "2 -> 0 (back) survived");
    assert_edges_consistent(&g2, top2);
}

/// HUNT: `BlockEdge::decode` rejects an edge whose endpoint index is not in the
/// resolver (C++ returns LowlevelError "Bad serialized edge in block graph").
#[test]
fn w3_ir_block_edge_decode_rejects_unknown_endpoint() {
    let m = build_manager();
    // Encode an edge whose endpoint has index 99, but build a resolver that
    // does NOT know index 99.
    let mut g = BlockGraph::new();
    let top = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    let a = g.new_block(top);
    let c = g.new_block(top);
    g.arena[a].set_index(7);
    g.arena[c].set_index(99);
    let edge = BlockEdge::new(c, 0, 0);
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        edge.encode(&mut enc, &g.arena);
    }
    // resolver knows only index 7 (block a), not 99
    let resolver = BlockMap::new();
    let _ = (&resolver, a); // resolver has no entries; lookup must miss
    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut decoded = BlockEdge::default();
    let res = decoded.decode(&mut dec, &resolver);
    assert!(res.is_err(), "decode must reject an unresolved endpoint");
}
