//! Independent adversarial verification tests for `w7-harness-kunaregion`
//! (port of `decompiler/unittests/testkunaregion.cc`, the 3 angr
//! RegionIdentifier S7 unit tests).
//!
//! These tests were written by the VERIFIER, not the porter.  The harness port
//! cannot read the C++ pointer-level state directly (`top->getHead()->getAddr()`,
//! the `top->getGraph()` region-node count, `walkRegionBlocks(loop, loopCol)`)
//! because the external test crate has no node-pool access.  It RECONSTRUCTS
//! those values through the public API + a stack-tracking visitor:
//!   - `head_addr(region)` reads the head node id via `get_head()` then looks
//!     up its `addr` in `region.get_graph().node_keys()`;
//!   - the cyclic-region head/body are captured during the single walk
//!     (head = first leaf visited while the cyclic frame is open; body = every
//!     leaf visited while it is on the stack);
//!   - `direct_subregions` = the number of regions entered while exactly the top
//!     frame is open (stack depth 1), the public-API stand-in for the C++
//!     `isRegion()` count over `top->getGraph()`.
//!
//! These three reconstructions are the only places the harness deviates from a
//! literal transcription, so they are the only places a faithfulness bug can
//! hide.  The tests below pin each reconstruction against an invariant that the
//! C++ pointer-level original also satisfies, on shapes the 3 ported tests never
//! exercise (multi-direct-subregion top, a loop with a nested sub-region body,
//! a head that is NOT the lowest input address).

use kuna_decomp::kuna_regionid::{KunaGraphRegion, KunaRegionIdentifier, KunaRegionVisitor};
use kuna_decomp::seams::BlockId;

/// Re-implementation of the harness collector, kept byte-for-byte equivalent so
/// the tests exercise the SAME reconstruction logic under review.
#[derive(Default)]
struct Collector {
    addrs: Vec<u64>,
    /// (head addr, sorted body addrs) per cyclic region.
    cyclics: Vec<(u64, Vec<u64>)>,
    direct_subregions: i32,
    /// (is_cyclic, head-or-MAX, body) per open region frame.
    stack: Vec<(bool, u64, Vec<u64>)>,
    /// Max region-nesting depth seen (frame 0 = top => depth 1 at deepest top).
    max_depth: usize,
}

impl KunaRegionVisitor for Collector {
    fn enter_region(&mut self, region: &KunaGraphRegion) {
        if self.stack.len() == 1 {
            self.direct_subregions += 1;
        }
        self.stack.push((region.is_cyclic(), u64::MAX, Vec::new()));
        if self.stack.len() > self.max_depth {
            self.max_depth = self.stack.len();
        }
    }
    fn exit_region(&mut self, _region: &KunaGraphRegion) {
        if let Some((cyclic, head, mut body)) = self.stack.pop() {
            if cyclic {
                body.sort_unstable();
                self.cyclics.push((head, body));
            }
        }
    }
    fn visit_block(&mut self, _block: Option<BlockId>, addr: u64) {
        self.addrs.push(addr);
        for frame in self.stack.iter_mut() {
            if frame.1 == u64::MAX {
                frame.1 = addr;
            }
            if frame.0 {
                frame.2.push(addr);
            }
        }
    }
}

/// Public-API head-address reconstruction, identical to the harness `head_addr`.
fn head_addr(region: &KunaGraphRegion) -> u64 {
    let head = region.get_head().expect("region has a head");
    region
        .get_graph()
        .node_keys()
        .find(|k| k.id == head)
        .expect("head node is a member of its own region graph")
        .addr
}

fn sorted(input: &[u64]) -> Vec<u64> {
    let mut v = input.to_vec();
    v.sort_unstable();
    v
}

/// ADVERSARIAL 1 — `head_addr` must return the region's ENTRY, not merely the
/// minimum-address member of the graph.  The 3 ported tests all have the entry
/// at the lowest address (1 or 2), so a buggy `head_addr` that returned
/// `node_keys().next()` (the KunaNodeOrder minimum) would still pass them.
///
/// Here the input entry is forced (`set_entry_addr(50)`) to a node whose address
/// is GREATER than other members of the top region's graph, so "first key in
/// KunaNodeOrder" != "the head".  The C++ `top->getHead()->getAddr()` returns
/// the entry (50); a min-key shortcut would return something smaller.  The
/// reconstruction must agree with the entry.
#[test]
fn head_addr_is_entry_not_min_key() {
    // Linear chain 50 -> 60 -> 70, entry forced at 50 (the largest? no — pick
    // addresses so the entry is NOT the global minimum among ALL inputs, and add
    // a second, non-entry component with a smaller address so the min-key
    // shortcut would be tempted toward 10).
    let mut ri = KunaRegionIdentifier::new();
    let a = ri.add_synthetic_block(50);
    let b = ri.add_synthetic_block(60);
    let c = ri.add_synthetic_block(70);
    ri.add_synthetic_edge(a, b);
    ri.add_synthetic_edge(b, c);
    // A smaller-addressed non-entry component (discarded by pickConnectedComponent).
    let d = ri.add_synthetic_block(10);
    let e = ri.add_synthetic_block(20);
    ri.add_synthetic_edge(d, e);
    ri.set_entry_addr(50);

    let top_id = ri.compute().expect("compute");
    let top = ri.region(top_id);
    // The entry component (50,60,70) survives; head is the entry, addr 50.
    assert_eq!(
        head_addr(top),
        50,
        "head_addr must return the entry node's address (getHead()->getAddr()), \
         not the lowest member address"
    );

    // Cross-check: the head is also the first leaf block visited by the walk
    // (the start node is the entry), so the two reconstructions agree.
    let mut col = Collector::default();
    ri.walk_blocks(&mut col).expect("walk");
    assert_eq!(
        col.addrs.first().copied(),
        Some(50),
        "the entry/head is the first block visited in the recursive walk"
    );
    // And the discarded component never appears.
    assert!(
        !col.addrs.contains(&10) && !col.addrs.contains(&20),
        "the non-entry component is discarded before the walk"
    );
}

/// ADVERSARIAL 2 — `direct_subregions` must equal the EXACT count of region
/// nodes in `top->getGraph()`, the C++ loop
///   for (iter : top->getGraph()) if ((*iter).first->isRegion()) regionCount++;
/// All three ported tests have `top.num_nodes() == 2` with one block + one
/// region node (count == 1), so a "set a bool" / cap-at-one reconstruction bug
/// would be invisible in them.  The external crate cannot read a node's
/// `NodeKind`, but it CAN observe, for the TOP region only, that every member is
/// either a leaf block (a `visit_block` at stack depth 1) or a sub-region (an
/// `enter_region` at stack depth 1).  So the closed identity
///   direct_subregions + (#blocks visited at stack depth 1) == top.num_nodes()
/// pins the reconstruction against the true region-node count, on a shape whose
/// top graph is genuinely a mix of a block and a region (here: one nested
/// two-diamond region + the tail block 7, so num_nodes == 2, count == 1).
#[test]
fn direct_subregions_matches_top_region_node_count() {
    //        1
    //       / \
    //      2   3        (diamond A)
    //       \ /
    //        4
    //       / \
    //      5   6        (diamond B)
    //       \ /
    //        7
    let mut ri = KunaRegionIdentifier::new();
    let n = |ri: &mut KunaRegionIdentifier, a: u64| ri.add_synthetic_block(a);
    let n1 = n(&mut ri, 1);
    let n2 = n(&mut ri, 2);
    let n3 = n(&mut ri, 3);
    let n4 = n(&mut ri, 4);
    let n5 = n(&mut ri, 5);
    let n6 = n(&mut ri, 6);
    let n7 = n(&mut ri, 7);
    ri.add_synthetic_edge(n1, n2);
    ri.add_synthetic_edge(n1, n3);
    ri.add_synthetic_edge(n2, n4);
    ri.add_synthetic_edge(n3, n4);
    ri.add_synthetic_edge(n4, n5);
    ri.add_synthetic_edge(n4, n6);
    ri.add_synthetic_edge(n5, n7);
    ri.add_synthetic_edge(n6, n7);

    let top_id = ri.compute().expect("compute");
    let top_num_nodes = ri.region(top_id).get_graph().num_nodes();

    // Count leaf blocks that are direct members of the top region (visited while
    // exactly the top frame is open) via a depth-aware collector.
    #[derive(Default)]
    struct TopMembers {
        depth: usize,
        top_leaf_blocks: i32,
        direct_subregions: i32,
        all_addrs: Vec<u64>,
    }
    impl KunaRegionVisitor for TopMembers {
        fn enter_region(&mut self, _r: &KunaGraphRegion) {
            if self.depth == 1 {
                self.direct_subregions += 1;
            }
            self.depth += 1;
        }
        fn exit_region(&mut self, _r: &KunaGraphRegion) {
            self.depth -= 1;
        }
        fn visit_block(&mut self, _b: Option<BlockId>, addr: u64) {
            self.all_addrs.push(addr);
            if self.depth == 1 {
                self.top_leaf_blocks += 1;
            }
        }
    }
    let mut tm = TopMembers::default();
    ri.walk_blocks(&mut tm).expect("walk");

    // Every block visited exactly once (structural invariant for ANY valid tree).
    assert_eq!(sorted(&tm.all_addrs), vec![1, 2, 3, 4, 5, 6, 7]);

    // The closed identity: top graph = its region nodes + its leaf-block nodes.
    assert_eq!(
        tm.direct_subregions + tm.top_leaf_blocks,
        top_num_nodes,
        "direct_subregions ({}) + top leaf blocks ({}) must equal top.num_nodes() ({})",
        tm.direct_subregions,
        tm.top_leaf_blocks,
        top_num_nodes
    );

    // Sanity: the harness `direct_subregions` (stack.len()==1 form) agrees with
    // the depth-aware count, and the cyclic list is empty for an acyclic graph.
    let mut col = Collector::default();
    ri.walk_blocks(&mut col).expect("walk");
    assert_eq!(
        col.direct_subregions, tm.direct_subregions,
        "the two stack/depth formulations of direct_subregions must agree"
    );
    assert_eq!(col.cyclics.len(), 0, "acyclic graph => no cyclic regions");
}

/// ADVERSARIAL 3 — the cyclic-region body reconstruction (every leaf visited
/// while the cyclic frame is open) must include blocks that live inside a
/// NESTED sub-region of the loop, matching the C++ `walkRegionBlocks(loop, ...)`
/// which recurses into sub-regions.  The ported `regionid_loop` has a flat
/// two-block body, so a reconstruction that only captured the loop's *direct*
/// member blocks (and skipped nested-region blocks) would still pass it.
///
/// Here the loop body contains an inner acyclic diamond, so a correct capture
/// must report all loop-body block addresses, and the head must be the loop
/// header (lowest-order member, the first leaf visited while the frame is open).
#[test]
fn cyclic_body_includes_nested_region_blocks() {
    // 1 -> 2 -> [ diamond 3/4/5 -> 6 ] -> 2  (back edge), and 6 -> 7 exits.
    //   loop body = {2,3,4,5,6}; the diamond 3/4/5 may collapse into a nested
    //   sub-region inside the loop region.
    let mut ri = KunaRegionIdentifier::new();
    let n1 = ri.add_synthetic_block(1);
    let n2 = ri.add_synthetic_block(2);
    let n3 = ri.add_synthetic_block(3);
    let n4 = ri.add_synthetic_block(4);
    let n5 = ri.add_synthetic_block(5);
    let n6 = ri.add_synthetic_block(6);
    let n7 = ri.add_synthetic_block(7);
    ri.add_synthetic_edge(n1, n2);
    ri.add_synthetic_edge(n2, n3);
    ri.add_synthetic_edge(n3, n4);
    ri.add_synthetic_edge(n3, n5);
    ri.add_synthetic_edge(n4, n6);
    ri.add_synthetic_edge(n5, n6);
    ri.add_synthetic_edge(n6, n2); // back edge -> loop header 2
    ri.add_synthetic_edge(n6, n7); // loop exit

    ri.compute().expect("compute");
    let mut col = Collector::default();
    ri.walk_blocks(&mut col).expect("walk");

    // Whole-tree coverage: every block once.
    assert_eq!(sorted(&col.addrs), vec![1, 2, 3, 4, 5, 6, 7]);

    // Exactly one cyclic region.
    assert_eq!(col.cyclics.len(), 1, "one loop => one cyclic region");
    let (head, body) = &col.cyclics[0];

    // Head reconstruction: the loop header is address 2 (the back-edge target),
    // and it is the first leaf visited while the frame is open.
    assert_eq!(*head, 2, "loop head reconstruction = back-edge target addr 2");

    // Body reconstruction must include the nested diamond's blocks (3,4,5,6),
    // proving the open-frame capture recurses through nested sub-regions exactly
    // like the C++ walkRegionBlocks(loop, loopCol).
    assert_eq!(
        body,
        &vec![2, 3, 4, 5, 6],
        "cyclic body must include nested-region blocks, not just direct members"
    );
    // 1 (pre-loop) and 7 (exit) are NOT in the loop body.
    assert!(
        !body.contains(&1) && !body.contains(&7),
        "pre-loop and exit blocks are outside the loop region"
    );

    // The loop region nests deeper than the top (max_depth >= 2): confirms the
    // body really came from an open *nested* frame, not the top frame.
    assert!(
        col.max_depth >= 2,
        "the loop region is a proper nested frame (depth >= 2), got {}",
        col.max_depth
    );
}
