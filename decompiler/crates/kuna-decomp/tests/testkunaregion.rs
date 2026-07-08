//! Port of `decompiler/unittests/testkunaregion.cc` (kuna-owned unit tests for
//! the angr `RegionIdentifier` port, S7).
//!
//! These are NOT upstream Ghidra tests.  `regionid_diamond0` / `regionid_diamond1`
//! are direct ports of angr's `test_region_identifier_0` / `test_region_identifier_1`
//! (angr/tests/analyses/decompiler/test_structurer.py:50,80): synthetic DummyNode
//! graphs, asserting the final top-level region graph has exactly two nodes.
//! `regionid_loop` is kuna-added cyclic coverage (angr's two synthetic tests are
//! both acyclic).
//!
//! Ported 1:1 from the C++ `TEST(...)` macros, assertion-for-assertion, against
//! the merged `kuna_regionid` public API.  The C++ collector keeps
//! `const KunaGraphRegion *` pointers and re-walks one region later; the external
//! Rust test crate has no node-pool access, so the visitor captures the head and
//! body of every cyclic region during the single tree walk — equivalent for the
//! asserted shapes (the loop head is the first block visited in a cyclic region,
//! and its body is exactly the blocks visited while that region is open).

use kuna_decomp::kuna_regionid::{KunaGraphRegion, KunaRegionIdentifier, KunaRegionVisitor};
use kuna_decomp::context::BlockId;

/// Collect every leaf block address (in walk order) and every cyclic region.
///
/// Port of the C++ `KunaTestCollector`.  For each cyclic region the collector
/// records `(head addr, sorted body block addrs)`: the head is the first block
/// visited while the region is open (its `node_keys()` iterate in
/// `KunaNodeOrder`, so the lowest-address member — the loop head — comes first),
/// and the body is every block visited while the region is on the stack.
///
/// `direct_subregions` additionally counts the sub-regions entered as direct
/// children of the top region (walk-stack depth 1).  This is the public-API
/// equivalent of the C++ `top->getGraph()` region-node count: every region node
/// in the top graph wraps exactly one such direct-child region.
#[derive(Default)]
struct KunaTestCollector {
    /// Every leaf block address, in walk order (C++ `vector<uintb> addrs`).
    addrs: Vec<u64>,
    /// `(head addr, sorted body addrs)` for each cyclic region (C++
    /// `vector<const KunaGraphRegion *> cyclics`, dereferenced eagerly here).
    cyclics: Vec<(u64, Vec<u64>)>,
    /// Count of direct-child sub-regions of the top region (stack depth 1).
    direct_subregions: i32,
    /// Open-region stack: `(is_cyclic, head addr or u64::MAX, body accumulator)`.
    /// The top region occupies frame 0; depth-1 frames are its direct children.
    stack: Vec<(bool, u64, Vec<u64>)>,
}

impl KunaRegionVisitor for KunaTestCollector {
    fn enter_region(&mut self, region: &KunaGraphRegion) {
        // A direct child of the top region is entered while exactly the top frame
        // (depth 0) is open, i.e. the stack has length 1 at entry time.
        if self.stack.len() == 1 {
            self.direct_subregions += 1;
        }
        // C++ records the region pointer if isCyclic(); we record cyclic regions
        // by pushing an open frame whose head is filled from the first block.
        self.stack.push((region.is_cyclic(), u64::MAX, Vec::new()));
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
                frame.1 = addr; // first block of an open region = its head (walk order)
            }
            if frame.0 {
                frame.2.push(addr);
            }
        }
    }
}

/// Sorted copy of the collected addresses (C++ `sortedAddrs`).
fn sorted_addrs(input: &[u64]) -> Vec<u64> {
    let mut res = input.to_vec();
    res.sort_unstable();
    res
}

/// The address of a region's head, read through the public API: the head node
/// lives inside the region's own graph, so its `KunaNodeKey` (carrying the addr)
/// appears in `node_keys()`.  This is the public-API equivalent of the C++
/// `region->getHead()->getAddr()`.
fn head_addr(region: &KunaGraphRegion) -> u64 {
    let head = region.get_head().expect("region has a head");
    region
        .get_graph()
        .node_keys()
        .find(|k| k.id == head)
        .expect("head node is a member of its own region graph")
        .addr
}

#[test]
fn regionid_diamond0() {
    //       1
    //       |
    //       2
    //      / .
    //     3   4
    //      \ /
    //       5
    //       |
    //       6
    let mut ri = KunaRegionIdentifier::new();
    let n1 = ri.add_synthetic_block(1);
    let n2 = ri.add_synthetic_block(2);
    let n3 = ri.add_synthetic_block(3);
    let n4 = ri.add_synthetic_block(4);
    let n5 = ri.add_synthetic_block(5);
    let n6 = ri.add_synthetic_block(6);
    ri.add_synthetic_edge(n1, n2);
    ri.add_synthetic_edge(n2, n3);
    ri.add_synthetic_edge(n2, n4);
    ri.add_synthetic_edge(n3, n5);
    ri.add_synthetic_edge(n4, n5);
    ri.add_synthetic_edge(n5, n6);

    let top_id = ri.compute().expect("compute returns a top region");
    {
        let top = ri.region(top_id);
        assert_eq!(top.get_graph().num_nodes(), 2); // The angr assertion
        assert_eq!(head_addr(top), 1_u64);
        assert!(!top.is_cyclic());
    }

    // The recursive walker covers every input block exactly once.
    let mut col = KunaTestCollector::default();
    ri.walk_blocks(&mut col).expect("walk succeeds");

    // Exactly one of the two top-level nodes is a sub-region (the public-API
    // equivalent of the C++ region-node count: the walk enters exactly one
    // direct-child region of the top).
    assert_eq!(col.direct_subregions, 1);

    assert_eq!(col.addrs.len(), 6_usize);
    let got = sorted_addrs(&col.addrs);
    for (i, &g) in got.iter().enumerate().take(6) {
        assert_eq!(g, (i + 1) as u64);
    }
    assert_eq!(col.cyclics.len(), 0_usize);
}

#[test]
fn regionid_diamond1() {
    //        1
    //        |
    //        2
    //        | .
    //        | 3
    //        | /
    //        4
    //        |
    //        5
    //        | .
    //        | 6
    //        | /
    //        7
    //        |
    //        8
    let mut ri = KunaRegionIdentifier::new();
    // nodes[1..=8] hold the synthetic blocks (index 0 is an unused placeholder,
    // mirroring the C++ `KunaRegionNode *nodes[9]` 1-based layout).
    let n1 = ri.add_synthetic_block(1);
    let nodes = {
        let mut v = vec![n1, n1];
        for i in 2..=8u64 {
            v.push(ri.add_synthetic_block(i));
        }
        v
    };
    ri.add_synthetic_edge(nodes[1], nodes[2]);
    ri.add_synthetic_edge(nodes[2], nodes[3]);
    ri.add_synthetic_edge(nodes[3], nodes[4]);
    ri.add_synthetic_edge(nodes[2], nodes[4]);
    ri.add_synthetic_edge(nodes[4], nodes[5]);
    ri.add_synthetic_edge(nodes[5], nodes[6]);
    ri.add_synthetic_edge(nodes[6], nodes[7]);
    ri.add_synthetic_edge(nodes[5], nodes[7]);
    ri.add_synthetic_edge(nodes[7], nodes[8]);

    let top_id = ri.compute().expect("compute returns a top region");
    assert_eq!(ri.region(top_id).get_graph().num_nodes(), 2); // The angr assertion

    let mut col = KunaTestCollector::default();
    ri.walk_blocks(&mut col).expect("walk succeeds");
    assert_eq!(col.addrs.len(), 8_usize);
    let got = sorted_addrs(&col.addrs);
    for (i, &g) in got.iter().enumerate().take(8) {
        assert_eq!(g, (i + 1) as u64);
    }
}

#[test]
fn regionid_loop() {
    //   1 -> 2 -> 3 -> 4
    //        ^    |
    //        +----+        (3 loops back to 2)
    let mut ri = KunaRegionIdentifier::new();
    let n1 = ri.add_synthetic_block(1);
    let n2 = ri.add_synthetic_block(2);
    let n3 = ri.add_synthetic_block(3);
    let n4 = ri.add_synthetic_block(4);
    ri.add_synthetic_edge(n1, n2);
    ri.add_synthetic_edge(n2, n3);
    ri.add_synthetic_edge(n3, n2);
    ri.add_synthetic_edge(n3, n4);

    let top_id = ri.compute().expect("compute returns a top region");
    assert!(!ri.region(top_id).is_cyclic());

    // Exactly one cyclic region, headed at address 2, containing blocks {2,3}.
    let mut col = KunaTestCollector::default();
    ri.walk_blocks(&mut col).expect("walk succeeds");
    assert_eq!(col.cyclics.len(), 1_usize);
    let (loop_head, loop_addrs) = &col.cyclics[0];
    assert_eq!(*loop_head, 2_u64);
    // The C++ re-walks the loop via walkRegionBlocks(loop, loopCol); here the body
    // captured while the region was open is the same set, already sorted.
    assert_eq!(loop_addrs.len(), 2_usize);
    assert_eq!(loop_addrs[0], 2_u64);
    assert_eq!(loop_addrs[1], 3_u64);

    // regions_by_block_addrs contains the loop body as its own region.
    let rbba = ri.get_regions_by_block_addrs();
    let mut found = false;
    for region in rbba.iter() {
        let sorted_region = sorted_addrs(region);
        if sorted_region.len() == 2 && sorted_region[0] == 2 && sorted_region[1] == 3 {
            found = true;
            break;
        }
    }
    assert!(found);

    // Whole-tree walk covers every block exactly once.
    assert_eq!(col.addrs.len(), 4_usize);
    let got = sorted_addrs(&col.addrs);
    for (i, &g) in got.iter().enumerate().take(4) {
        assert_eq!(g, (i + 1) as u64);
    }
}
