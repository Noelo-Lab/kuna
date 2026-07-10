//! Independent adversarial verification tests for `w7-s7-kuna-regionid`
//! (port of `decompiler/cpp/kuna_regionid.{cc,hh}`).
//!
//! These tests were written by the VERIFIER, not the porter.  They re-derive
//! expected behavior directly from the C++ algorithm (angr `RegionIdentifier`)
//! and target the structuring-schema-precedence / iteration-order hunt items:
//!   - the multi-exit cyclic region normal/abnormal-exit postorder split
//!     (C++ `makeCyclicRegion`, l.756-775);
//!   - the supergraph single-in/single-out chain collapse (`makeSupergraph`);
//!   - nested loop headers (a loop containing another header returns `None`,
//!     C++ `makeCyclicRegion` l.713-719);
//!   - `regions_by_block_addrs` whole-tree coverage and the recursive walker
//!     visiting every leaf block exactly once;
//!   - the weakly-connected-component pick with an explicit entry address
//!     (C++ `pickConnectedComponent`).
//!
//! The structural invariants (every block visited once; the union of
//! `regions_by_block_addrs` equals the input block set) must hold for ANY valid
//! region tree, so they catch merge/ordering/off-by-one regressions without
//! pinning a fragile exact tree shape.

use kuna_decomp::kuna_regionid::{KunaGraphRegion, KunaRegionIdentifier, KunaRegionVisitor};
use kuna_decomp::context::BlockId;

/// Visitor that records the leaf-block addresses (in walk order) and, for every
/// cyclic region, its body block addresses (sorted).  Mirrors the in-module
/// `Collector` but lives entirely in the external test crate (public API only).
#[derive(Default)]
struct Collector {
    addrs: Vec<u64>,
    /// Sorted body block addrs for each cyclic region (in exit order).
    cyclic_bodies: Vec<Vec<u64>>,
    /// Open-region stack: (is_cyclic, body accumulator).
    stack: Vec<(bool, Vec<u64>)>,
}

impl KunaRegionVisitor for Collector {
    fn enter_region(&mut self, region: &KunaGraphRegion) {
        self.stack.push((region.is_cyclic(), Vec::new()));
    }
    fn exit_region(&mut self, _region: &KunaGraphRegion) {
        if let Some((cyclic, mut body)) = self.stack.pop() {
            if cyclic {
                body.sort_unstable();
                self.cyclic_bodies.push(body);
            }
        }
    }
    fn visit_block(&mut self, _block: Option<BlockId>, addr: u64) {
        self.addrs.push(addr);
        for frame in self.stack.iter_mut() {
            if frame.0 {
                frame.1.push(addr);
            }
        }
    }
}

fn run(edges: &[(u64, u64)], addrs: &[u64]) -> (KunaRegionIdentifier, Collector) {
    let mut ri = KunaRegionIdentifier::new();
    let mut id = std::collections::BTreeMap::new();
    for &a in addrs {
        id.insert(a, ri.add_synthetic_block(a));
    }
    for &(a, b) in edges {
        ri.add_synthetic_edge(id[&a], id[&b]);
    }
    ri.compute().expect("compute failed");
    let mut col = Collector::default();
    ri.walk_blocks(&mut col).expect("walk failed");
    (ri, col)
}

fn sorted(v: &[u64]) -> Vec<u64> {
    let mut r = v.to_vec();
    r.sort_unstable();
    r
}

/// ADVERSARIAL 1 — a cyclic region with TWO non-absorbable exits.
///
/// Graph: entry 1 -> head 2; loop body {2,3} via back edge 3->2; two distinct
/// loop exits 4 (from 2) and 5 (from 3) that each ALSO have a second predecessor
/// from 1, so `refineLoop` stage 1 cannot absorb them (in-degree 2 != 1).  Both
/// exits join at sink 6.  This drives the `refinedExitNodes.size() > 1` branch
/// of `makeCyclicRegion` (the deterministic post-order normal/abnormal split,
/// C++ l.756-775) — the highest-risk ordering path in the port.
///
/// Re-derived from the C++: loop body stays {2,3}; every block is visited once;
/// there is exactly one cyclic region, headed at 2, with body {2,3}.
#[test]
fn cyclic_region_two_exits_postorder_split() {
    let edges = [
        (1, 2),
        (2, 3),
        (3, 2), // back edge -> loop {2,3}
        (2, 4), // exit A
        (3, 5), // exit B
        (1, 4), // second pred of 4 (blocks stage-1 absorption)
        (1, 5), // second pred of 5
        (4, 6),
        (5, 6),
    ];
    let (ri, col) = run(&edges, &[1, 2, 3, 4, 5, 6]);

    // Every input block is visited exactly once.
    assert_eq!(col.addrs.len(), 6, "every block visited once");
    assert_eq!(sorted(&col.addrs), vec![1, 2, 3, 4, 5, 6]);

    // Exactly one cyclic region, body {2,3}.
    assert_eq!(col.cyclic_bodies.len(), 1, "one loop region");
    assert_eq!(col.cyclic_bodies[0], vec![2, 3], "loop body is {{2,3}}");

    // The loop body {2,3} appears as a region in regions_by_block_addrs.
    let rbba = ri.get_regions_by_block_addrs();
    assert!(
        rbba.iter().any(|r| sorted(r) == vec![2, 3]),
        "loop body {{2,3}} is its own region in regions_by_block_addrs: {:?}",
        rbba
    );

    // Whole-tree block coverage: the union of all region block lists equals the
    // 6 input blocks (no block dropped or double-counted across the BFS levels
    // of buildRegionsByBlockAddrs — a sub-region head appears once as a marker).
    let mut all: Vec<u64> = rbba.iter().flatten().copied().collect();
    all.sort_unstable();
    all.dedup();
    for a in [1, 2, 3, 4, 5, 6] {
        assert!(all.contains(&a), "block {} present somewhere in rbba", a);
    }
}

/// ADVERSARIAL 2 — supergraph chain collapse.
///
/// A pure single-in/single-out chain 1->2->3->4->5 must collapse, in
/// `makeSupergraph`, into a single multi-node (every interior edge has
/// sizeOut(src)==1 && sizeIn(dst)==1).  After identification the whole function
/// is one region whose graph has a single node, yet the recursive walker still
/// expands the merged chain to all five leaf blocks in execution order.
///
/// Re-derived from `mergeNodes`/`makeSupergraph`: chain order is preserved, so
/// the walk order is exactly 1,2,3,4,5.
#[test]
fn supergraph_chain_collapses_but_walk_expands_all() {
    let edges = [(1, 2), (2, 3), (3, 4), (4, 5)];
    let (ri, col) = run(&edges, &[1, 2, 3, 4, 5]);

    // The merged chain expands to all five blocks IN EXECUTION ORDER (the
    // k_multi chain preserves order; this is NOT address-sorted by accident
    // because the addresses already ascend, so also assert order explicitly).
    assert_eq!(col.addrs, vec![1, 2, 3, 4, 5], "chain walked in execution order");

    // No cyclic regions in an acyclic chain.
    assert_eq!(col.cyclic_bodies.len(), 0);

    // The top region's internal graph is a single (merged) node.
    let top = ri.get_top_region().unwrap();
    assert_eq!(
        ri.region(top).get_graph().num_nodes(),
        1,
        "the entire chain collapses to one supergraph node"
    );
}

/// ADVERSARIAL 3 — nested loops: an outer loop whose body contains a second
/// loop header.  `makeCyclicRegion` must refuse (return `None`) for the outer
/// header while its body still contains the inner, unstructured header
/// (C++ l.713-719), structure the inner loop first, then the outer.  The result
/// must contain TWO cyclic regions and visit every block once.
///
/// Graph: 1 -> 2; outer loop head 2; inner loop head 3 with self-ish body
/// {3,4} (4->3 back edge); inner exits to 5; 5->2 closes the outer loop; 2->6
/// exits.  Outer body {2,3,4,5}, inner body {3,4}.
#[test]
fn nested_loops_inner_structured_first() {
    let edges = [
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 3), // inner back edge -> inner loop {3,4}
        (4, 5),
        (5, 2), // outer back edge -> outer loop
        (2, 6), // outer exit
    ];
    let (_ri, col) = run(&edges, &[1, 2, 3, 4, 5, 6]);

    // Every block visited exactly once.
    assert_eq!(col.addrs.len(), 6, "every block visited once");
    assert_eq!(sorted(&col.addrs), vec![1, 2, 3, 4, 5, 6]);

    // Two cyclic regions get formed (inner {3,4}, outer containing it).
    assert_eq!(
        col.cyclic_bodies.len(),
        2,
        "inner and outer loops both structured: {:?}",
        col.cyclic_bodies
    );
    // The inner loop body {3,4} is one of them.
    assert!(
        col.cyclic_bodies.iter().any(|b| b == &vec![3, 4]),
        "inner loop body {{3,4}} present: {:?}",
        col.cyclic_bodies
    );
    // The outer body is a strict superset that contains 2 and the inner blocks.
    assert!(
        col.cyclic_bodies
            .iter()
            .any(|b| b.contains(&2) && b.contains(&3) && b.contains(&4) && b.len() > 2),
        "outer loop body contains the head 2 and the inner blocks: {:?}",
        col.cyclic_bodies
    );
}

/// ADVERSARIAL 4 — self-loop on the head (the `sliceGraph` self-loop HACK,
/// C++ l.416-417: an infinite self-loop slices to nothing, so the body edge is
/// re-added by hand).  An isolated `2 -> 2` self loop reached from 1 must form a
/// cyclic region whose body is exactly {2}.
#[test]
fn self_loop_head_slice_hack() {
    // 1 -> 2, 2 -> 2 (self), 2 -> 3.
    let edges = [(1, 2), (2, 2), (2, 3)];
    let (_ri, col) = run(&edges, &[1, 2, 3]);

    assert_eq!(col.addrs.len(), 3, "every block visited once");
    assert_eq!(sorted(&col.addrs), vec![1, 2, 3]);
    assert_eq!(col.cyclic_bodies.len(), 1, "the self loop is one cyclic region");
    assert_eq!(col.cyclic_bodies[0], vec![2], "self-loop body is exactly {{2}}");
}

/// ADVERSARIAL 5 — weakly-connected-component pick honoring the entry address.
///
/// Two disjoint components: a SMALL one {10,11,12} containing the declared
/// entry, and a LARGER one {1,2,3,4}.  `pickConnectedComponent` must keep the
/// component that holds `entry_addr` (C++ l.212-219: the entry component wins
/// over the largest), so only {10,11,12} survives and exactly those blocks are
/// walked.  This pins the entry-vs-largest precedence (an off-by-one / wrong
/// `chosen` here would silently keep the wrong component).
#[test]
fn connected_component_prefers_entry_over_largest() {
    let mut ri = KunaRegionIdentifier::new();
    // Larger component (4 nodes), no entry.
    let a1 = ri.add_synthetic_block(1);
    let a2 = ri.add_synthetic_block(2);
    let a3 = ri.add_synthetic_block(3);
    let a4 = ri.add_synthetic_block(4);
    ri.add_synthetic_edge(a1, a2);
    ri.add_synthetic_edge(a2, a3);
    ri.add_synthetic_edge(a3, a4);
    // Smaller component (3 nodes), contains the entry at 10.
    let b10 = ri.add_synthetic_block(10);
    let b11 = ri.add_synthetic_block(11);
    let b12 = ri.add_synthetic_block(12);
    ri.add_synthetic_edge(b10, b11);
    ri.add_synthetic_edge(b11, b12);
    ri.set_entry_addr(10);

    ri.compute().unwrap();
    let mut col = Collector::default();
    ri.walk_blocks(&mut col).unwrap();

    // Only the entry component survives.
    assert_eq!(
        sorted(&col.addrs),
        vec![10, 11, 12],
        "the entry component {{10,11,12}} is kept, not the larger {{1,2,3,4}}"
    );
    assert!(
        !col.addrs.contains(&1) && !col.addrs.contains(&4),
        "the larger non-entry component is discarded"
    );
}

/// ADVERSARIAL 6 — `compute()` is idempotent (the `computed` latch, C++ l.158)
/// and an empty graph is a hard error (C++ l.160-161).  A second `compute()`
/// returns the same top-region id without recomputing; an empty identifier
/// errors instead of panicking.
#[test]
fn compute_idempotent_and_empty_errors() {
    // Empty graph -> Err.
    let mut empty = KunaRegionIdentifier::new();
    assert!(empty.compute().is_err(), "empty input graph is an error, not a panic");

    // Idempotent compute on a real graph.
    let mut ri = KunaRegionIdentifier::new();
    let n1 = ri.add_synthetic_block(1);
    let n2 = ri.add_synthetic_block(2);
    ri.add_synthetic_edge(n1, n2);
    let first = ri.compute().unwrap();
    let second = ri.compute().unwrap();
    assert_eq!(first.0, second.0, "second compute returns the cached top region");
}
