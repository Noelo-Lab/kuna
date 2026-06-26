//! Porter in-module tests for `block.rs`.
//!
//! Exercise: graph construction + edge ops preserving C++ ordering and the
//! reverse-index mirror invariant; the Cooper/Harvey/Kennedy dominator tree on
//! straight-line / diamond / loop / irreducible graphs; the Tarjan
//! spanning-tree edge labels; and the FlowBlock encode/decode round-trip via
//! `PackedEncode`/`PackedDecode`.

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace};
use kuna_base::types::int4;

use super::*;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Build a manager with const(0) and ram(1) so basic-block covers have a space.
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

fn ram(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space(1).unwrap())
}

/// A graph + a root BlockGraph node holding `n` plain components.
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

/// Verify the in/out edge mirror invariant of the whole graph: every out edge
/// has a matching in edge at its reverse_index that points back and vice versa
/// (the C++ `checkEdges` invariant).
fn assert_edges_consistent(g: &BlockGraph, root: BlockId) {
    let list: Vec<BlockId> = g.arena[root].get_list().to_vec();
    for &bl in &list {
        for i in 0..g.arena[bl].intothis.len() {
            let edge = &g.arena[bl].intothis[i];
            let rev = edge.reverse_index as usize;
            let other = edge.point.unwrap();
            let edger = &g.arena[other].outofthis[rev];
            assert_eq!(edger.point, Some(bl), "intothis mirror point");
            assert_eq!(edger.reverse_index as usize, i, "intothis mirror index");
        }
        for i in 0..g.arena[bl].outofthis.len() {
            let edge = &g.arena[bl].outofthis[i];
            let rev = edge.reverse_index as usize;
            let other = edge.point.unwrap();
            let edger = &g.arena[other].intothis[rev];
            assert_eq!(edger.point, Some(bl), "outofthis mirror point");
            assert_eq!(edger.reverse_index as usize, i, "outofthis mirror index");
        }
    }
}

// ---------------------------------------------------------------------------
// edge construction / ordering
// ---------------------------------------------------------------------------

#[test]
fn add_edge_orders_in_out_and_sets_reverse_index() {
    let (mut g, root, b) = build_graph(3);
    // 0 -> 1, 0 -> 2 : out edges of 0 are [1,2] in insertion order
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    assert_eq!(g.arena[b[0]].size_out(), 2);
    assert_eq!(g.arena[b[0]].get_out(0), b[1]);
    assert_eq!(g.arena[b[0]].get_out(1), b[2]);
    // 1 and 2 each have one in edge from 0
    assert_eq!(g.arena[b[1]].size_in(), 1);
    assert_eq!(g.arena[b[1]].get_in(0), b[0]);
    // reverse indices: 0's out[0] mirrors 1's in[0]
    assert_eq!(g.arena[b[0]].get_out_rev_index(0), 0);
    assert_eq!(g.arena[b[0]].get_out_rev_index(1), 0);
    assert_edges_consistent(&g, root);
}

#[test]
fn remove_edge_shifts_and_corrects_reverse_indices() {
    let (mut g, root, b) = build_graph(4);
    // 0 has three out edges to 1,2,3
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[0], b[3]);
    // remove the middle edge 0->2; remaining out edges slide to [1,3]
    g.remove_edge(b[0], b[2]);
    assert_eq!(g.arena[b[0]].size_out(), 2);
    assert_eq!(g.arena[b[0]].get_out(0), b[1]);
    assert_eq!(g.arena[b[0]].get_out(1), b[3]);
    assert_eq!(g.arena[b[2]].size_in(), 0);
    assert_edges_consistent(&g, root);
}

#[test]
fn swap_edges_flips_out_order_and_flip_path_flag() {
    let (mut g, root, b) = build_graph(3);
    g.add_edge(b[0], b[1]); // false out
    g.add_edge(b[0], b[2]); // true out
    assert!(!g.arena[b[0]].get_flip_path());
    g.swap_edges(b[0]);
    assert_eq!(g.arena[b[0]].get_out(0), b[2]);
    assert_eq!(g.arena[b[0]].get_out(1), b[1]);
    assert!(g.arena[b[0]].get_flip_path());
    assert_edges_consistent(&g, root);
}

#[test]
fn switch_edge_replaces_target_in_place() {
    let (mut g, root, b) = build_graph(4);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    // move the 0->1 edge to 0->3, preserving slot 0
    g.switch_edge(b[0], b[1], b[3]);
    assert_eq!(g.arena[b[0]].get_out(0), b[3]);
    assert_eq!(g.arena[b[0]].get_out(1), b[2]);
    assert_eq!(g.arena[b[1]].size_in(), 0);
    assert_eq!(g.arena[b[3]].size_in(), 1);
    assert_edges_consistent(&g, root);
}

#[test]
fn out_edge_flag_mirrors_to_in_edge() {
    let (mut g, _root, b) = build_graph(2);
    g.add_edge(b[0], b[1]);
    g.set_out_edge_flag(b[0], 0, edge_flags::f_loop_edge);
    assert!(g.arena[b[0]].is_loop_out(0));
    assert!(g.arena[b[1]].is_loop_in(0));
    g.clear_out_edge_flag(b[0], 0, edge_flags::f_loop_edge);
    assert!(!g.arena[b[0]].is_loop_out(0));
    assert!(!g.arena[b[1]].is_loop_in(0));
}

// ---------------------------------------------------------------------------
// spanning tree (Tarjan)
// ---------------------------------------------------------------------------

#[test]
fn spanning_tree_labels_back_edge_on_simple_loop() {
    // entry(0) -> 1 -> 2 -> 1(back) ; 1 -> 3 (exit)
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[1]); // back edge
    g.add_edge(b[1], b[3]);

    let mut preorder = Vec::new();
    let mut rootlist = Vec::new();
    g.find_spanning_tree(root, &mut preorder, &mut rootlist);

    // the 2 -> 1 edge must be labeled a back edge (and loop edge)
    // find block 2 in the (now reordered) graph and check its out edge to 1
    let two = b[2];
    let one = b[1];
    let slot = g.arena[two].get_out_index(one);
    assert!(slot >= 0, "2 must still have an out edge to 1");
    assert!(g.arena[two].is_back_edge_out(slot), "2->1 is a back edge");
    assert!(g.arena[two].is_loop_out(slot), "2->1 is also a loop edge");
}

// ---------------------------------------------------------------------------
// dominators (Cooper/Harvey/Kennedy)
// ---------------------------------------------------------------------------

/// Run spanning tree + forward dominators on a graph already wired with edges,
/// returning the immediate-dominator index of each original block.
fn run_dominators(g: &mut BlockGraph, root: BlockId, b: &[BlockId]) -> Vec<Option<int4>> {
    let mut rootlist = Vec::new();
    g.structure_loops(root, &mut rootlist);
    g.calc_forward_dominator(root, &rootlist);
    b.iter()
        .map(|&blk| g.arena[blk].get_immed_dom().map(|d| g.arena[d].get_index()))
        .collect()
}

#[test]
fn dominator_straight_line() {
    // 0 -> 1 -> 2
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[2]);
    let _ = run_dominators(&mut g, root, &b);
    // entry's idom is null; each other block's idom is its unique predecessor.
    assert_eq!(g.arena[b[0]].get_immed_dom(), None);
    assert_eq!(g.arena[b[1]].get_immed_dom(), Some(b[0]));
    assert_eq!(g.arena[b[2]].get_immed_dom(), Some(b[1]));
    // reverse-post-order: entry index 0, strictly increasing along flow.
    assert_eq!(g.arena[b[0]].get_index(), 0);
    assert!(g.arena[b[1]].get_index() < g.arena[b[2]].get_index());
}

#[test]
fn dominator_diamond() {
    // 0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3.  idom(1)=idom(2)=0, idom(3)=0.
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[3]);
    g.add_edge(b[2], b[3]);
    let _ = run_dominators(&mut g, root, &b);
    assert_eq!(g.arena[b[0]].get_immed_dom(), None);
    assert_eq!(g.arena[b[1]].get_immed_dom(), Some(b[0]));
    assert_eq!(g.arena[b[2]].get_immed_dom(), Some(b[0]));
    // the merge point is dominated by the split, not by either arm
    assert_eq!(g.arena[b[3]].get_immed_dom(), Some(b[0]));
    // and dominates() agrees: 0 dominates everything, 1 does not dominate 3
    assert!(g.dominates(b[0], Some(b[3])));
    assert!(!g.dominates(b[1], Some(b[3])));
}

#[test]
fn dominator_self_loop_and_exit() {
    // 0 -> 1 ; 1 -> 1 (self loop) ; 1 -> 2 (exit)
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[1], b[1]);
    g.add_edge(b[1], b[2]);
    let _ = run_dominators(&mut g, root, &b);
    assert_eq!(g.arena[b[0]].get_immed_dom(), None);
    assert_eq!(g.arena[b[1]].get_immed_dom(), Some(b[0]));
    assert_eq!(g.arena[b[2]].get_immed_dom(), Some(b[1]));
    // the self edge is a back/loop edge
    let slot = g.arena[b[1]].get_out_index(b[1]);
    assert!(slot >= 0);
    assert!(g.arena[b[1]].is_loop_out(slot));
}

#[test]
fn dominator_irreducible_two_entry_loop() {
    // Classic irreducible graph: 0 -> 1, 0 -> 2, 1 -> 2, 2 -> 1.
    // The 1<->2 loop has two entries (from 0 directly to each), so it is
    // irreducible; findIrreducible must label an irreducible edge and the run
    // must still terminate with a valid dominator tree.
    let (mut g, root, b) = build_graph(3);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[2]);
    g.add_edge(b[2], b[1]);
    let _ = run_dominators(&mut g, root, &b);
    // 0 dominates all; 1 and 2 are each dominated directly by 0 (neither
    // dominates the other, since each can be entered from 0).
    assert_eq!(g.arena[b[0]].get_immed_dom(), None);
    assert_eq!(g.arena[b[1]].get_immed_dom(), Some(b[0]));
    assert_eq!(g.arena[b[2]].get_immed_dom(), Some(b[0]));
    // The graph is irreducible (the 1<->2 loop is entered both at its header
    // and at its body), so structureLoops must have labeled at least one
    // irreducible edge somewhere in the graph.
    let mut has_irr = false;
    for &blk in &b {
        for i in 0..g.arena[blk].size_out() {
            if g.arena[blk].is_irreducible_out(i) {
                has_irr = true;
            }
        }
    }
    assert!(has_irr, "an irreducible edge should be labeled");
}

#[test]
fn dom_tree_and_depth_diamond() {
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[3]);
    g.add_edge(b[2], b[3]);
    let _ = run_dominators(&mut g, root, &b);

    // buildDomTree: child[index_of(0)] should contain 1, 2, and 3 (all
    // immediately dominated by entry).
    let child = g.build_dom_tree(root);
    let entry_idx = g.arena[b[0]].get_index() as usize;
    let mut kids: Vec<int4> =
        child[entry_idx].iter().map(|&c| g.arena[c].get_index()).collect();
    kids.sort_unstable();
    let mut expect: Vec<int4> =
        [b[1], b[2], b[3]].iter().map(|&c| g.arena[c].get_index()).collect();
    expect.sort_unstable();
    assert_eq!(kids, expect);

    // buildDomDepth: entry depth 1, its children depth 2, max depth 2.
    let (depth, max) = g.build_dom_depth(root);
    assert_eq!(max, 2);
    assert_eq!(depth[g.arena[b[0]].get_index() as usize], 1);
    assert_eq!(depth[g.arena[b[3]].get_index() as usize], 2);
}

#[test]
fn find_common_block_walks_dom_tree() {
    // diamond: common dominator of the two arms is the entry.
    let (mut g, root, b) = build_graph(4);
    g.set_start_block(root, b[0]);
    g.add_edge(b[0], b[1]);
    g.add_edge(b[0], b[2]);
    g.add_edge(b[1], b[3]);
    g.add_edge(b[2], b[3]);
    let _ = run_dominators(&mut g, root, &b);
    assert_eq!(g.find_common_block(b[1], b[2]), Some(b[0]));
    assert_eq!(g.find_common_block_set(&[b[1], b[2], b[3]]), b[0]);
    // marks must be cleared after the walk
    for &blk in &b {
        assert!(!g.arena[blk].is_mark());
    }
}

// ---------------------------------------------------------------------------
// basic block cover (getStart / getStop)
// ---------------------------------------------------------------------------

#[test]
fn basic_block_get_start_stop_from_cover() {
    let m = build_manager();
    let mut g = BlockGraph::new();
    let root = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    let bb = g.new_block_basic(root);
    let mut cover = RangeList::new();
    cover.insert_range(ram(&m), 0x1000, 0x100f);
    if let BlockKind::Basic(bd) = g.arena[bb].kind_mut() {
        bd.cover = cover;
    }
    let start = block_get_start(&g.arena, bb);
    let stop = block_get_stop(&g.arena, bb);
    assert_eq!(start, Address::new(ram(&m), 0x1000));
    assert_eq!(stop, Address::new(ram(&m), 0x100f));
    // a non-basic (graph) block yields invalid start
    assert!(block_get_start(&g.arena, root).is_invalid());
}

// ---------------------------------------------------------------------------
// type name <-> block_type
// ---------------------------------------------------------------------------

#[test]
fn type_name_roundtrip_for_decode_factory_types() {
    // Only plain/copy/graph round-trip through nameToType (per the C++ TODO).
    assert_eq!(name_to_type("graph"), BlockType::Graph);
    assert_eq!(name_to_type("copy"), BlockType::Copy);
    assert_eq!(name_to_type("plain"), BlockType::Plain);
    assert_eq!(name_to_type("anythingelse"), BlockType::Plain);
    assert_eq!(type_to_name(BlockType::If), "properif");
    assert_eq!(type_to_name(BlockType::Ls), "list");
    assert_eq!(type_to_name(BlockType::Basic), "basic");
}

// ---------------------------------------------------------------------------
// encode / decode round-trip (PackedEncode/PackedDecode)
// ---------------------------------------------------------------------------

#[test]
fn encode_decode_graph_roundtrip() {
    let m = build_manager();

    // Build a small graph: entry(0) -> 1, 0 -> 2, 1 -> 2.  Two of the three
    // blocks are plain, one is a copy (so altindex is exercised on encode).
    let mut g = BlockGraph::new();
    let top = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    let b0 = g.new_block(top);
    let b1 = g.new_block(top);
    let b2 = g.new_block(top);
    // assign indices (decode keys cross-references on index; encode writes them)
    g.arena[b0].set_index(0);
    g.arena[b1].set_index(1);
    g.arena[b2].set_index(2);
    g.add_edge(b0, b1);
    g.add_edge(b0, b2);
    g.add_edge(b1, b2);

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

    // The decoded graph must mirror the original: 3 components, same indices,
    // and the same edge structure.
    assert_eq!(g2.arena[top2].get_size(), 3);
    // map decoded blocks by index
    let find = |g: &BlockGraph, idx: int4| -> BlockId {
        *g.arena[top2]
            .get_list()
            .iter()
            .find(|&&bl| g.arena[bl].get_index() == idx)
            .expect("decoded block by index")
    };
    let d0 = find(&g2, 0);
    let d1 = find(&g2, 1);
    let d2 = find(&g2, 2);
    // 0 -> 1 and 0 -> 2
    assert!(g2.arena[d0].get_out_index(d1) >= 0);
    assert!(g2.arena[d0].get_out_index(d2) >= 0);
    // 1 -> 2
    assert!(g2.arena[d1].get_out_index(d2) >= 0);
    // 2's in edges come from 0 and 1
    assert_eq!(g2.arena[d2].size_in(), 2);
    assert_edges_consistent(&g2, top2);
}

#[test]
fn block_edge_encode_decode_roundtrip() {
    let m = build_manager();
    // Two blocks with one edge; encode just the edge and decode it back through
    // a resolver populated with both block indices.
    let mut g = BlockGraph::new();
    let top = g.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    let a = g.new_block(top);
    let c = g.new_block(top);
    g.arena[a].set_index(7);
    g.arena[c].set_index(9);
    let edge = BlockEdge::new(c, 0, 3);

    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        edge.encode(&mut enc, &g.arena);
    }

    // Resolver mapping index 9 -> c (sorted + cached).
    let mut resolver = BlockMap::new();
    resolver.sortlist.push(a);
    resolver.sortlist.push(c);
    resolver.sort_list(&g.arena);

    let mut dec = PackedDecode::new(&m);
    dec.ingest_stream(&buf).unwrap();
    let mut decoded = BlockEdge::default();
    decoded.decode(&mut dec, &resolver).unwrap();
    assert_eq!(decoded.point, Some(c));
    assert_eq!(decoded.reverse_index, 3);
}

// ---------------------------------------------------------------------------
// VERIFIER adversarial tests (item: w10-structure-printbody / Round 1)
//
// Target the cross-arena `build_copy_from` (C++ `BlockGraph::buildCopy`) — the
// most fragile new construct in this item: a slotmap-arena-crossing copy whose
// edge `point` and `immed_dom` remap goes through an explicit BTreeMap instead
// of the C++ `bl->copymap` back-pointer.  The hunt-list flags this as
// iteration-order + erase/remap fragile.
// ---------------------------------------------------------------------------

/// build_copy_from must mirror EVERY source block as a `BlockCopy` whose `copy`
/// field is the *source* BlockId, in source-list order, and remap every edge
/// `point` to the corresponding *dst* BlockId (never leave a source id in a dst
/// edge).  A diamond exercises a join node with two in-edges.
#[test]
fn w10_build_copy_from_remaps_edges_cross_arena() {
    // Source graph: diamond  s0 -> s1, s0 -> s2, s1 -> s3, s2 -> s3.
    let (src, sroot, s) = build_graph(4);
    let mut src = src;
    src.add_edge(s[0], s[1]);
    src.add_edge(s[0], s[2]);
    src.add_edge(s[1], s[3]);
    src.add_edge(s[2], s[3]);
    // Give the source distinct indices + an immediate dominator chain so the
    // remap of `index`/`immed_dom` is observable.
    for (i, &bl) in s.iter().enumerate() {
        src.block_mut(bl).set_index(i as int4);
    }
    src.arena[s[1]].immed_dom = Some(s[0]);
    src.arena[s[2]].immed_dom = Some(s[0]);
    src.arena[s[3]].immed_dom = Some(s[0]);
    src.arena[s[0]].immed_dom = None;

    // Destination: empty graph (the `sblocks` analogue).
    let mut dst = BlockGraph::new();
    let droot = dst.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    dst.root = Some(droot);

    dst.build_copy_from(droot, &src, sroot);

    let dlist: Vec<BlockId> = dst.block(droot).get_list().to_vec();
    assert_eq!(dlist.len(), 4, "one dst BlockCopy per source block");

    // src id -> dst id, in list order (the copy field carries the source id).
    let mut src_to_dst = std::collections::BTreeMap::new();
    for (k, &d) in dlist.iter().enumerate() {
        assert_eq!(
            dst.block(d).get_type(),
            BlockType::Copy,
            "every seeded block is a BlockCopy"
        );
        let copied = dst.block(d).get_copy().expect("BlockCopy.copy set");
        assert_eq!(copied, s[k], "dst order mirrors source list order, copy=src id");
        assert_eq!(dst.block(d).get_index(), k as int4, "index copied verbatim");
        src_to_dst.insert(s[k], d);
    }

    // Every dst edge point is a DST id (cross-arena remap complete); no leaked
    // source ids.  Reconstruct topology and compare to the source.
    for (k, &d) in dlist.iter().enumerate() {
        let outs: Vec<BlockId> = (0..dst.block(d).size_out())
            .map(|i| dst.block(d).get_out(i))
            .collect();
        for &o in &outs {
            assert!(
                dlist.contains(&o),
                "dst out-edge must point into the dst arena (got a leaked id)"
            );
        }
        // Same out-degree as the source.
        assert_eq!(
            outs.len(),
            src.block(s[k]).size_out() as usize,
            "out-degree preserved"
        );
        // immed_dom remapped to the dst copy of s[0] (or None for the root).
        let idom = dst.block(d).get_immed_dom();
        if k == 0 {
            assert_eq!(idom, None, "root idom stays None");
        } else {
            assert_eq!(
                idom,
                Some(src_to_dst[&s[0]]),
                "immed_dom remapped cross-arena to the dst copy of the source idom"
            );
        }
    }

    // The join node (s3) keeps both in-edges, both pointing into dst.
    let djoin = src_to_dst[&s[3]];
    assert_eq!(dst.block(djoin).size_in(), 2, "join keeps 2 in-edges");
    let ins: Vec<BlockId> = (0..dst.block(djoin).size_in())
        .map(|i| dst.block(djoin).get_in(i))
        .collect();
    assert!(
        ins.contains(&src_to_dst[&s[1]]) && ins.contains(&src_to_dst[&s[2]]),
        "join in-edges remapped to the dst copies of s1 and s2"
    );

    // The dst edge mirror invariant must hold after the cross-arena remap.
    assert_edges_consistent(&dst, droot);
}

/// Boundary: build_copy_from on an empty source graph must produce an empty dst
/// (no panic on the empty BTreeMap / empty second pass).
#[test]
fn w10_build_copy_from_empty_source() {
    let src = BlockGraph::new();
    let mut src = src;
    let sroot = src.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    src.root = Some(sroot);

    let mut dst = BlockGraph::new();
    let droot = dst.arena.insert(FlowBlock::new_kind(BlockKind::Graph));
    dst.root = Some(droot);

    dst.build_copy_from(droot, &src, sroot);
    assert_eq!(dst.block(droot).get_list().len(), 0, "empty source -> empty dst");
}

// ---------------------------------------------------------------------------
// kuna_flatten_ifelse — the if-else-flattening print-tree splice
// (angr IfElseFlattener; s8_structure::kuna_ifelseflatten)
// ---------------------------------------------------------------------------

/// Insert a leaf and parent it under `graph`'s `list`.
fn push_leaf(g: &mut BlockGraph, parent: BlockId) -> BlockId {
    let id = g.arena.insert(FlowBlock::new_kind(BlockKind::Plain));
    g.arena[id].parent = Some(parent);
    g.arena[parent].list.push(id);
    id
}

/// Build a 3-component `BlockIf` `[cond, true, else]` parented under a
/// `BlockList` that also holds a trailing sibling, returning
/// `(graph, parent, if_id, else_clause, tail)`.  This is the exact print-tree
/// shape `ActionIfElseFlatten` edits.
fn build_ifelse_in_list() -> (BlockGraph, BlockId, BlockId, BlockId, BlockId) {
    let mut g = BlockGraph::new();
    let parent = g.arena.insert(FlowBlock::new_kind(BlockKind::Ls));
    g.root = Some(parent);

    // The 3-component BlockIf, parented under the list.
    let if_id = g.arena.insert(FlowBlock::new_kind(BlockKind::If { gototype: 0, gototarget: None }));
    g.arena[if_id].parent = Some(parent);
    g.arena[parent].list.push(if_id);

    let cond = push_leaf(&mut g, if_id);
    let true_clause = push_leaf(&mut g, if_id);
    let else_clause = push_leaf(&mut g, if_id);
    let _ = (cond, true_clause);

    // The trailing sibling after the if in the parent list.
    let tail = push_leaf(&mut g, parent);
    (g, parent, if_id, else_clause, tail)
}

#[test]
fn flatten_ifelse_drops_else_and_splices_sibling() {
    let (mut g, parent, if_id, else_clause, tail) = build_ifelse_in_list();
    assert_eq!(g.block(if_id).get_size(), 3, "starts as a 3-component if/else");
    assert_eq!(g.block(parent).get_size(), 2, "parent holds [if, tail]");

    let ok = g.kuna_flatten_ifelse(if_id);
    assert!(ok, "flatten should succeed for a 3-comp if/else under a list");

    // The `if` is now a 2-component if/then (cond + true-clause).
    assert_eq!(g.block(if_id).get_size(), 2, "if demoted to if/then");
    assert_eq!(g.block(if_id).get_type(), BlockType::If);

    // The else-clause is now the `if`'s immediate successor in the parent.
    assert_eq!(g.block(parent).get_size(), 3, "parent now holds [if, else, tail]");
    assert_eq!(g.block(parent).get_block(0), if_id);
    assert_eq!(g.block(parent).get_block(1), else_clause, "else spliced right after if");
    assert_eq!(g.block(parent).get_block(2), tail);
    assert_eq!(
        g.block(else_clause).get_parent(),
        Some(parent),
        "moved else is re-parented to the list"
    );
}

#[test]
fn flatten_ifelse_refuses_two_component_if() {
    // An if/then (2-component) is not flattenable — there is no else to drop.
    let mut g = BlockGraph::new();
    let parent = g.arena.insert(FlowBlock::new_kind(BlockKind::Ls));
    g.root = Some(parent);
    let if_id = g.arena.insert(FlowBlock::new_kind(BlockKind::If { gototype: 0, gototarget: None }));
    g.arena[if_id].parent = Some(parent);
    g.arena[parent].list.push(if_id);
    let _cond = push_leaf(&mut g, if_id);
    let _tc = push_leaf(&mut g, if_id);

    assert!(!g.kuna_flatten_ifelse(if_id), "2-component if is not flattenable");
    assert_eq!(g.block(if_id).get_size(), 2, "unchanged");
}

#[test]
fn flatten_ifelse_refuses_non_list_parent() {
    // A BlockIf whose parent is itself a BlockIf (a clause, not a list) cannot
    // host a spliced sibling, so the splice is refused (honest-partial).
    let mut g = BlockGraph::new();
    let outer = g.arena.insert(FlowBlock::new_kind(BlockKind::If { gototype: 0, gototarget: None }));
    g.root = Some(outer);
    let inner = g.arena.insert(FlowBlock::new_kind(BlockKind::If { gototype: 0, gototarget: None }));
    g.arena[inner].parent = Some(outer);
    g.arena[outer].list.push(inner);
    let _c = push_leaf(&mut g, inner);
    let _t = push_leaf(&mut g, inner);
    let _e = push_leaf(&mut g, inner);

    assert!(!g.kuna_flatten_ifelse(inner), "non-list parent cannot host the sibling");
    assert_eq!(g.block(inner).get_size(), 3, "unchanged");
}
