//! Independent verifier adversarial tests for item `w7-s7-kuna-regiongraph`
//! (`decompiler/cpp/kuna_regiongraph.{cc,hh}`): the kuna-owned region digraph
//! substrate for the angr `RegionIdentifier` port (S7).
//!
//! These target the hunt-list spots flagged as most fragile for this item:
//!   - `KunaEdgeSum`/`_sort_edge` 2^64 carry: address sums straddling 2^64
//!     must order by (carry, low64), not by the wrapped low64;
//!   - `_append_scc` loop-head selection on a pairwise-distance tie: the LAST
//!     successor in `KunaNodeOrder` wins (angr's `{v:k}` dict-inversion
//!     overwrite-on-collision = last-wins);
//!   - quasi-topo self-loop handling (a real self-loop is a `src==dst` edge
//!     that must collapse to a present-but-edgeless id, never a phantom edge);
//!   - postorder/`getSortedSuccs` ordering with addresses near `u64::MAX`
//!     (ascending push => largest-addr successor explored first), proving the
//!     `KunaNodeOrder` is unsigned;
//!   - `graph_updated` incremental dominator patching with MULTIPLE dominatees
//!     re-pointed at once: the C++ iterates `doms`/`invertedDomTree` in raw
//!     pointer order while the Rust port iterates in `KunaNodeOrder`; the
//!     observable result must still equal a from-scratch recomputation
//!     (order-insensitive consumers — LOSS-recorded divergence);
//!   - `subgraph_between_nodes` pruning of an interior cycle that cannot reach
//!     the frontier (erase-while-iterating / dangling-node fixpoint).
//!
//! Each assertion is re-derived from the C++ oracle in
//! `decompiler/cpp/kuna_regiongraph.cc`, not from the Rust port.

use std::collections::BTreeMap;

use kuna_base::types::{uint4, uintb};
use kuna_decomp::kuna_regiongraph::{
    kuna_dfs_postorder_deterministic, kuna_immediate_dominators, kuna_quasi_topo_sort,
    kuna_subgraph_between_nodes, KunaIncrementalDominators, KunaNodeId, KunaNodeSet, KunaNodePool,
    KunaRegionGraph, NodeKind,
};

/// Test harness mirroring the in-crate `Builder`: a pool plus auto-incrementing
/// creation idents (the deterministic `(addr, ident)` tiebreaker).
struct B {
    pool: KunaNodePool,
    next_ident: uint4,
}

impl B {
    fn new() -> B {
        B { pool: KunaNodePool::new(), next_ident: 0 }
    }
    fn node(&mut self, addr: uintb) -> KunaNodeId {
        let id = self.next_ident;
        self.next_ident += 1;
        self.pool.make(NodeKind::Block, addr, id)
    }
    fn graph(&self, edges: &[(KunaNodeId, KunaNodeId)], extra: &[KunaNodeId]) -> KunaRegionGraph {
        let mut g = KunaRegionGraph::new();
        for &n in extra {
            g.add_node(&self.pool, n);
        }
        for &(a, b) in edges {
            g.add_edge(&self.pool, a, b);
        }
        g
    }
    fn set(&self, ids: &[KunaNodeId]) -> KunaNodeSet {
        let mut s = KunaNodeSet::new();
        for &i in ids {
            s.insert(self.pool.key(i));
        }
        s
    }
}

/// F-A: `KunaNodeOrder` / `getSortedSuccs` must be an UNSIGNED order even with
/// addresses near `u64::MAX`.  Deterministic postorder pushes successors in
/// ascending order so the LARGEST-addr successor is explored first.  If the
/// comparator were signed, `0xFFFF...` would sort as a negative (smallest) and
/// the postorder would flip.  Re-derived from C++ `getSortedSuccs`
/// (`sort(..., KunaNodeOrder())`) over `uintb addr`.
#[test]
fn sorted_succs_is_unsigned_near_u64_max() {
    let mut b = B::new();
    let s = b.node(0x10);
    let lo = b.node(0x20);
    let hi = b.node(u64::MAX); // would be "negative" under a signed compare
    let mid = b.node(0x8000_0000_0000_0000); // high bit set: also "negative" signed
    // Insert edges out of address order; getSortedSuccs imposes KunaNodeOrder.
    let g = b.graph(&[(s, hi), (s, lo), (s, mid)], &[]);
    let sorted = g.get_sorted_succs(&b.pool, s).unwrap();
    // Unsigned ascending: 0x20 < 0x8000.. < 0xFFFF..
    assert_eq!(sorted, vec![lo, mid, hi]);

    // Postorder pushes ascending => hi (largest) explored first, finishes first
    // among the leaves; s finishes last.  All three succs are sinks.
    let mut po = Vec::new();
    kuna_dfs_postorder_deterministic(&b.pool, &g, s, &mut po).unwrap();
    // hi pushed last => popped/explored first => finishes first.
    assert_eq!(po[0], hi);
    assert_eq!(po[1], mid);
    assert_eq!(po[2], lo);
    assert_eq!(*po.last().unwrap(), s); // root finishes last
}

/// F-B: `_append_scc` loop-head tie-break = LAST successor in `KunaNodeOrder`.
///
/// Build a 3-node strongly connected component {x,y,z} fully symmetric so that
/// every member has equal total pairwise shortest-path distance to the others.
/// The single entry into the SCC (from `a`) reaches ALL THREE members, so
/// `_append_scc` falls into the `sccSuccs.size() > 1` branch and must pick the
/// loop head by minimum total distance; on the (here total) tie angr's
/// `{v: k}` inversion overwrites on equal key, so the LAST inserted (largest in
/// `KunaNodeOrder`, since `sccSuccs` is pre-sorted ascending) wins.  In the
/// fully-symmetric K3 cycle the chosen head is therefore the highest-addr
/// member `z`, and the expansion lists it first inside the SCC.
/// Re-derived from C++ `kunaAppendScc` (`distToNode[total]=...; loopHead =
/// (*distToNode.begin()).second`).
#[test]
fn append_scc_loop_head_distance_tie_last_wins() {
    let mut b = B::new();
    let a = b.node(0x10);
    // Fully connected 3-cycle (each -> the other two): all pairwise dist == 1,
    // so every candidate has identical total distance (2).
    let x = b.node(0x20);
    let y = b.node(0x30);
    let z = b.node(0x40);
    let g = b.graph(
        &[
            (a, x),
            (a, y),
            (a, z), // a enters every SCC member => >1 sccSuccs
            (x, y),
            (x, z),
            (y, x),
            (y, z),
            (z, x),
            (z, y),
        ],
        &[],
    );
    let mut order = Vec::new();
    kuna_quasi_topo_sort(&b.pool, &g, &mut order).unwrap();
    assert_eq!(order.len(), 4);
    assert_eq!(order[0], a, "a precedes the SCC");
    // sccSuccs sorted ascending [x,y,z]; equal totals; map overwrites => z (last)
    // is the loop head and therefore the first SCC member emitted.
    let scc_part = &order[1..];
    assert_eq!(scc_part[0], z, "tie => last-in-order (z) is the loop head");
    // All three present exactly once.
    let mut sorted = scc_part.to_vec();
    sorted.sort_by_key(|n| n.0);
    assert_eq!(sorted, {
        let mut v = vec![x, y, z];
        v.sort_by_key(|n| n.0);
        v
    });
}

/// F-C: quasi-topo with a REAL self-loop edge.  In `kunaQuasiTopoSort` a
/// `src==dst` edge (a real self-loop, since a 1-node SCC is "trivial" and never
/// collapsed) hits the `if (src == dst) { idPresent[src]=true; continue; }`
/// branch: the node is present but contributes no condensation edge and no
/// in-degree.  It must still appear exactly once in the output and in the right
/// topological slot.  Re-derived from C++ `kunaQuasiTopoSort` self-loop branch.
#[test]
fn quasi_topo_real_self_loop() {
    let mut b = B::new();
    let a = b.node(0x10);
    let s = b.node(0x20); // self-looping middle node
    let c = b.node(0x30);
    // a -> s -> c, plus s -> s (self loop; {s} stays a trivial SCC).
    let g = b.graph(&[(a, s), (s, s), (s, c)], &[]);
    let mut order = Vec::new();
    kuna_quasi_topo_sort(&b.pool, &g, &mut order).unwrap();
    // Each node once; topological a, s, c (self loop does not duplicate s).
    assert_eq!(order, vec![a, s, c]);
}

/// F-D: `graph_updated` re-points MULTIPLE dominatees at once and must match a
/// from-scratch recomputation, exercising the documented pointer-order vs
/// `KunaNodeOrder` iteration divergence (the C++ `doms`/`invertedDomTree` use
/// the default `std::less<ptr>` comparator; the Rust port iterates in
/// `KunaNodeOrder`).  The result is order-insensitive, so `verify()` (recompute
/// from scratch + compare) must pass after the patch.
///
/// Shape: a diamond region {b,c,m_inner} collapsed.  Concretely:
///   a -> h -> b -> t -> z ; h -> c -> t ; a -> w  (w stays outside region)
/// Collapse the single-entry/single-exit region rooted at h (nodes h,b,c) into
/// one node `r`; `t` and everything past it keep their dominators, and the
/// frontier patching path is exercised by touching `df` first.
/// Re-derived from C++ `graphUpdated` + `verify`.
#[test]
fn graph_updated_multi_dominatee_matches_scratch() {
    let mut b = B::new();
    let a = b.node(0x10);
    let h = b.node(0x20); // region entry (replaced_head)
    let bb = b.node(0x30); // region member
    let c = b.node(0x40); // region member
    let t = b.node(0x50); // region exit successor (dominatee of region)
    let z = b.node(0x60); // past the region
    let w = b.node(0x70); // sibling outside the region, dominated by a
    // a dominates everything; h dominates b,c,t (the diamond head); t dom z.
    let g = b.graph(
        &[(a, h), (h, bb), (h, c), (bb, t), (c, t), (t, z), (a, w)],
        &[],
    );
    let mut dom = KunaIncrementalDominators::new(&b.pool, &g, a, false).unwrap();
    // Sanity: from-scratch dominators before the merge.
    assert_eq!(dom.idom(h), Some(a));
    assert_eq!(dom.idom(bb), Some(h));
    assert_eq!(dom.idom(c), Some(h));
    assert_eq!(dom.idom(t), Some(h)); // t dominated by the diamond head h
    assert_eq!(dom.idom(z), Some(t));
    assert_eq!(dom.idom(w), Some(a));
    // Touch frontiers so the dfs-patch path runs too.
    let _ = dom.df(&b.pool, &g, t).unwrap();

    // Collapse {h,b,c} into a single region node r (same addr as the head h).
    let r = b.node(0x20);
    // Updated graph: a -> r -> t -> z ; a -> w.
    let g2 = b.graph(&[(a, r), (r, t), (t, z), (a, w)], &[]);
    let replaced = b.set(&[h, bb, c]);
    dom.graph_updated(&b.pool, r, &replaced, h).unwrap();

    // Patched dominators must equal a from-scratch computation on g2.
    assert_eq!(dom.idom(a), Some(a));
    assert_eq!(dom.idom(r), Some(a));
    assert_eq!(dom.idom(t), Some(r)); // t now dominated by the region node
    assert_eq!(dom.idom(z), Some(t));
    assert_eq!(dom.idom(w), Some(a)); // unaffected sibling
    assert_eq!(dom.idom(h), None); // folded
    assert_eq!(dom.idom(bb), None); // folded
    assert_eq!(dom.idom(c), None); // folded

    // The hard invariant: incremental == from-scratch (order-insensitive patch).
    dom.verify(&b.pool, &g2).unwrap();

    // Cross-check against an independent from-scratch idom computation on g2.
    let mut scratch: BTreeMap<KunaNodeId, KunaNodeId> = BTreeMap::new();
    kuna_immediate_dominators(&b.pool, &g2, a, &mut scratch).unwrap();
    assert_eq!(scratch.get(&t), Some(&r));
    assert_eq!(scratch.get(&z), Some(&t));
    assert_eq!(scratch.get(&w), Some(&a));
}

/// F-E: `subgraph_between_nodes` must prune an interior CYCLE that cannot reach
/// the frontier.  A self-sustaining dead cycle keeps both an in- and out-edge
/// internally, so the dangling-node fixpoint (which only removes nodes missing
/// a pred OR a succ) would never delete it — but the reverse-BFS `canReach`
/// gate stops the forward BFS from ever enqueuing it, so it never enters the
/// slice in the first place.  Re-derived from C++ `kunaSubgraphBetweenNodes`
/// (forward BFS gated on `canReach`, then dangling prune).
#[test]
fn subgraph_between_nodes_prunes_unreachable_cycle() {
    let mut b = B::new();
    let a = b.node(0x10);
    let e = b.node(0x20); // frontier
    let p = b.node(0x30); // dead cycle member
    let q = b.node(0x40); // dead cycle member
    // a -> e (the real path).  a -> p -> q -> p (a 2-cycle that never reaches e).
    let g = b.graph(&[(a, e), (a, p), (p, q), (q, p)], &[]);
    let frontier = b.set(&[e]);
    let mut res = KunaRegionGraph::new();
    kuna_subgraph_between_nodes(&b.pool, &g, a, &frontier, true, &mut res).unwrap();
    // The dead cycle {p,q} cannot reach e => never enters the slice.
    assert!(!res.contains_node(p), "unreachable cycle member p pruned");
    assert!(!res.contains_node(q), "unreachable cycle member q pruned");
    assert!(res.contains_node(a));
    assert!(res.contains_node(e));
    assert!(res.has_edge(a, e));

    // And with includeFrontier=false the frontier itself is dropped.
    let mut res2 = KunaRegionGraph::new();
    kuna_subgraph_between_nodes(&b.pool, &g, a, &frontier, false, &mut res2).unwrap();
    assert!(!res2.contains_node(e));
    assert!(!res2.contains_node(p));
    assert!(!res2.contains_node(q));
}

/// F-F: post-dominators over a node with two reverse-roots merging.  Computing
/// post-dominators reverses edges and roots at the sink; a diamond's post-idom
/// of the entry must be the exit (the merge point post-dominates everything).
/// Exercises `initDoms` post path (`buildReversed` + `immediate_dominators`)
/// and `verify` on the reversed orientation.  Re-derived from C++ `initDoms`
/// (`post` branch).
#[test]
fn post_dominators_diamond_merge() {
    let mut b = B::new();
    let a = b.node(0x10);
    let bb = b.node(0x20);
    let c = b.node(0x30);
    let d = b.node(0x40); // exit / merge
    // Diamond a -> {b,c} -> d.
    let g = b.graph(&[(a, bb), (a, c), (bb, d), (c, d)], &[]);
    // Post-dominators rooted at the exit d.
    let dom = KunaIncrementalDominators::new(&b.pool, &g, d, true).unwrap();
    // d post-dominates all; b and c post-dom only themselves up to d; a's
    // immediate post-dominator is d (both branches reconverge there).
    assert_eq!(dom.idom(d), Some(d));
    assert_eq!(dom.idom(bb), Some(d));
    assert_eq!(dom.idom(c), Some(d));
    assert_eq!(dom.idom(a), Some(d));
    assert!(dom.dominates(d, a), "exit post-dominates entry");
    assert!(!dom.dominates(bb, a), "one branch does not post-dominate entry");
    dom.verify(&b.pool, &g).unwrap();
}
