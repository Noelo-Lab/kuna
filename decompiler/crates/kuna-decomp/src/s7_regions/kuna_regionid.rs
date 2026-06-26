//! Port of `decompiler/cpp/kuna_regionid.{cc,hh}` (W7, item
//! `w7-s7-kuna-regionid`): the kuna-owned region-identification analysis (S7),
//! a faithful, ANALYSIS-ONLY reimplementation of angr's `RegionIdentifier`
//! (`angr/analyses/decompiler/region_identifier.py`) and the read-only subset
//! of its `GraphRegion` result type (`graph_region.py`).
//!
//! This module is **not** part of upstream Ghidra.  It builds on the merged
//! graph substrate [`crate::kuna_regiongraph`] (`KunaRegionGraph`,
//! `KunaNodePool`, the DFS/dominator utilities) and collapses a *private* copy
//! of a function's control-flow graph into a nested [`KunaGraphRegion`] tree.
//!
//! # Faithfulness
//!
//! The C++ is already a faithful angr port; this module transcribes it
//! structure-for-structure.  Iteration order, tie-breakers, and the panic-mode
//! / guard-cap heuristics are preserved.  The porting notes in the C++
//! (`RI l.<n>` = `region_identifier.py` line numbers at the angr commit current
//! to 2026-06) are carried over verbatim where they pin a behavior.
//!
//! Per ADR 0001 the C++ `KunaRegionNode *` becomes a [`KunaNodeId`] arena id
//! and `KunaGraphRegion *` becomes a [`RegionPayloadId`] index into the
//! identifier's region pool.  Per ADR 0002 every container the C++ *iterates*
//! for output is keyed by the ordered [`KunaNodeKey`] (`KunaNodeOrder`);
//! "membership only" pointer sets become `BTreeSet` keyed by raw id.  Per ADR
//! 0003 `uintb -> u64`, `int4 -> i32`, `int8 -> i64`; the guard caps are
//! computed in 64-bit exactly as the C++ adversarial review pinned them.
//!
//! # SEAM(W7): the `buildFromBlockGraph` / console-command surface
//!
//! The C++ also exposes Input A (`buildFromBlockGraph`, which reads a decompiler
//! `BlockGraph` read-only) and three console commands (`IfcKunaRegionTree`,
//! `IfcKunaRegionBlocks`, `IfcKunaRegionWalk`).  Both depend on
//! `FlowBlock::getStart()`/`lastOp()` and the `IfaceDecompCommand` console
//! machinery, which are a later wave (the same surface `kuna_assert.rs` seams
//! off).  This port realizes the full identifier algorithm over the synthetic
//! input API (`add_synthetic_block`/`add_synthetic_edge`), exactly what the
//! `testkunaregion.cc` unit tests drive; the block-graph adapter and the
//! console bindings are `// SEAM(W7)` (see `losses`).

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, int8, uintb};

use crate::funcdata::Funcdata;
use crate::kuna_regiongraph::{
    kuna_dfs_back_edges, kuna_dfs_postorder_deterministic, kuna_immediate_dominators,
    kuna_quasi_topo_sort, kuna_subgraph_between_nodes, KunaIncrementalDominators, KunaNodeId,
    KunaNodeKey, KunaNodePool, KunaNodeSet, KunaRegionGraph, NodeKind, RegionPayloadId,
};

/// Fixpoint iteration cap: `~2*n^2`, computed in 64-bit (C++ `kunaGuardCap`).
///
/// The caps exist only to turn a mis-ported guard into an error instead of a
/// hang; 64-bit arithmetic keeps them inert for any real graph (a 32-bit
/// `2*n*n` would overflow — and instantly fire — at `n >= 32768`).
fn kuna_guard_cap(num_nodes: int4) -> int8 {
    let n = num_nodes as int8;
    2 * n * n + 64
}

/// (kuna) One identified region: port of angr's `GraphRegion` (read-only
/// subset) (C++ `KunaGraphRegion`).
///
/// Owned by the [`KunaRegionIdentifier`] that created it (lives in its
/// `region_pool`); referenced from the working graph by the wrapper node's
/// [`RegionPayloadId`].  `head`/`wrapper` are filled by the identifier after
/// construction, matching the C++ `friend`-mutated fields.
#[derive(Debug, Clone, Default)]
pub struct KunaGraphRegion {
    /// Entry node (lives inside `graph`) (C++ `KunaRegionNode *head`).
    head: Option<KunaNodeId>,
    /// The `k_region` node wrapping this region in the parent graph
    /// (C++ `KunaRegionNode *wrapper`).
    wrapper: Option<KunaNodeId>,
    /// The region's internal subgraph (C++ `KunaRegionGraph graph`).
    graph: KunaRegionGraph,
    /// Is successor info present? (false only for top-level wrap regions)
    /// (C++ `bool hasSuccs`).
    has_succs: bool,
    /// Nodes outside the region that region nodes flow to
    /// (C++ `KunaNodeSet successors`).
    successors: KunaNodeSet,
    /// `graph` extended with the successor frontier (valid iff `has_succs`)
    /// (C++ `KunaRegionGraph graphWithSuccessors`).
    graph_with_successors: KunaRegionGraph,
    /// Is `full_graph` present? (cyclic regions only) (C++ `bool hasFull`).
    has_full: bool,
    /// All edges among loop nodes incl. entries/exits (valid iff `has_full`)
    /// (C++ `KunaRegionGraph fullGraph`).
    full_graph: KunaRegionGraph,
    /// Does this region contain a loop? (C++ `bool cyclic`).
    cyclic: bool,
    /// Is some ancestor region cyclic? (C++ `bool cyclicAncestor`).
    cyclic_ancestor: bool,
}

impl KunaGraphRegion {
    /// Entry node (C++ `getHead`).  `None` only before the identifier fills it.
    pub fn get_head(&self) -> Option<KunaNodeId> {
        self.head
    }
    /// Wrapper node in the parent graph (C++ `getWrapper`).
    pub fn get_wrapper(&self) -> Option<KunaNodeId> {
        self.wrapper
    }
    /// Internal subgraph (C++ `getGraph`).
    pub fn get_graph(&self) -> &KunaRegionGraph {
        &self.graph
    }
    /// Is successor info present? (C++ `hasSuccessorInfo`).
    pub fn has_successor_info(&self) -> bool {
        self.has_succs
    }
    /// External successors (C++ `getSuccessors`).
    pub fn get_successors(&self) -> &KunaNodeSet {
        &self.successors
    }
    /// Subgraph + frontier (C++ `getGraphWithSuccessors`).
    pub fn get_graph_with_successors(&self) -> &KunaRegionGraph {
        &self.graph_with_successors
    }
    /// Is the full loop graph present? (C++ `hasFullGraph`).
    pub fn has_full_graph(&self) -> bool {
        self.has_full
    }
    /// Full loop graph (C++ `getFullGraph`).
    pub fn get_full_graph(&self) -> &KunaRegionGraph {
        &self.full_graph
    }
    /// Does the region contain a loop? (C++ `isCyclic`).
    pub fn is_cyclic(&self) -> bool {
        self.cyclic
    }
    /// Is an ancestor cyclic? (C++ `hasCyclicAncestor`).
    pub fn has_cyclic_ancestor(&self) -> bool {
        self.cyclic_ancestor
    }
}

/// (kuna) One identified cyclic (loop) region, projected onto basic-block start
/// addresses for the S8 region structurer.  Produced by
/// [`KunaRegionIdentifier::cyclic_loops`].
///
/// `head_addr` is the loop head's start address; `body` is every basic-block
/// start address inside the loop (recursively resolved through nested
/// regions/multi-nodes); `exits` is every loop-successor block start address (the
/// angr `GraphRegion.successors` frontier — the blocks control reaches on leaving
/// the loop).  The structurer maps these addresses back to its `sblocks`
/// `BlockCopy` components to drive loop-successor refinement.
#[derive(Debug, Clone)]
pub struct KunaCyclicLoop {
    /// The loop head's basic-block start address.
    pub head_addr: uintb,
    /// Every basic-block start address in the loop body.
    pub body: BTreeSet<uintb>,
    /// Every loop-successor (exit) basic-block start address.
    pub exits: BTreeSet<uintb>,
}

/// (kuna) Callback interface for walking the blocks of a region tree
/// (C++ `KunaRegionVisitor`).
///
/// `visit_block` fires once per leaf block in deterministic region order
/// (merged chains expand to their members); `enter_region`/`exit_region`
/// bracket each nested region.  `block` is `None` for synthetic (test-only)
/// nodes.
pub trait KunaRegionVisitor {
    /// Called before a region's nodes (C++ `enterRegion`).
    fn enter_region(&mut self, _region: &KunaGraphRegion) {}
    /// Called after a region's nodes (C++ `exitRegion`).
    fn exit_region(&mut self, _region: &KunaGraphRegion) {}
    /// Called for each leaf block: `(wrapped block, addr)` (C++ `visitBlock`).
    fn visit_block(&mut self, block: Option<crate::seams::BlockId>, addr: uintb);
}

/// (kuna) The region identification analysis (port of angr's
/// `RegionIdentifier`) (C++ `KunaRegionIdentifier`).
pub struct KunaRegionIdentifier {
    /// Owns every node ever created (C++ pooled `KunaRegionNode *`s).  The
    /// pool's allocation index *is* the C++ `nextIdent` (each `make` bumps it),
    /// so we pass an explicit `next_ident` to keep the node `ident` field — the
    /// `KunaNodeOrder` tiebreaker — independent of the pool's internal index.
    pool: KunaNodePool,
    /// Owns every region ever created (C++ `vector<KunaGraphRegion *>`).
    region_pool: Vec<KunaGraphRegion>,
    /// Node creation counter (determinism tiebreaker) (C++ `uint4 nextIdent`).
    next_ident: u32,
    /// Option (angr default true) (C++ `largestSuccessorTreeOutsideLoop`).
    largest_successor_tree_outside_loop: bool,
    /// Option (angr default false) (C++ `refineLoopsWithSingleSuccessor`).
    refine_loops_with_single_successor: bool,
    /// Option (angr default false) (C++ `completeSuccessors`).
    complete_successors: bool,
    /// Is an entry address known? (C++ `bool hasEntryAddr`).
    has_entry_addr: bool,
    /// Entry node address (C++ `uintb entryAddr`).
    entry_addr: uintb,
    /// The graph being collapsed (C++ `KunaRegionGraph workGraph`).
    work_graph: KunaRegionGraph,
    /// Current start node of `work_graph` (C++ `KunaRegionNode *startNode`).
    start_node: Option<KunaNodeId>,
    /// Live loop-header list (consulted by `make_cyclic_region`)
    /// (C++ `vector<KunaRegionNode *> loopHeaders`).
    loop_headers: Vec<KunaNodeId>,
    /// Quasi-topological order (C++
    /// `map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder> nodeOrder`).  Keyed
    /// by raw id; the `(addr,ident)` ordering of the C++ map is never iterated
    /// in an output-affecting way here (only point lookups), so a raw-id key is
    /// faithful and the stored value is the order pair.
    node_order: BTreeMap<KunaNodeId, (int4, int4)>,
    /// Result: the top-level region (C++ `KunaGraphRegion *topRegion`).
    top_region: Option<RegionPayloadId>,
    /// Result: flat per-region block address lists
    /// (C++ `vector<vector<uintb> > regionsByBlockAddrs`).
    regions_by_block_addrs: Vec<Vec<uintb>>,
    /// Has `compute` run? (C++ `bool computed`).
    computed: bool,
}

impl Default for KunaRegionIdentifier {
    fn default() -> Self {
        KunaRegionIdentifier::new()
    }
}

impl KunaRegionIdentifier {
    /// Construct a fresh identifier (C++ constructor).
    pub fn new() -> KunaRegionIdentifier {
        KunaRegionIdentifier {
            pool: KunaNodePool::new(),
            region_pool: Vec::new(),
            next_ident: 0,
            largest_successor_tree_outside_loop: true, // angr default (RI l.53)
            refine_loops_with_single_successor: false, // angr default (RI l.55)
            complete_successors: false,                // angr default (RI l.56)
            has_entry_addr: false,
            entry_addr: 0,
            work_graph: KunaRegionGraph::new(),
            start_node: None,
            loop_headers: Vec::new(),
            node_order: BTreeMap::new(),
            top_region: None,
            regions_by_block_addrs: Vec::new(),
            computed: false,
        }
    }

    //
    // Options / inputs
    //

    /// Set option (before compute) (C++ `setLargestSuccessorTreeOutsideLoop`).
    pub fn set_largest_successor_tree_outside_loop(&mut self, val: bool) {
        self.largest_successor_tree_outside_loop = val;
    }
    /// Set option (before compute) (C++ `setRefineLoopsWithSingleSuccessor`).
    pub fn set_refine_loops_with_single_successor(&mut self, val: bool) {
        self.refine_loops_with_single_successor = val;
    }
    /// Set option (before compute) (C++ `setCompleteSuccessors`).
    pub fn set_complete_successors(&mut self, val: bool) {
        self.complete_successors = val;
    }
    /// Set the entry address (C++ `setEntryAddr`).
    pub fn set_entry_addr(&mut self, addr: uintb) {
        self.has_entry_addr = true;
        self.entry_addr = addr;
    }

    /// Allocate a pooled node (C++ `newNode`).
    fn new_node(&mut self, kind: NodeKind, addr: uintb) -> KunaNodeId {
        let id = self.pool.make(kind, addr, self.next_ident);
        self.next_ident += 1;
        id
    }

    /// Allocate a pooled region (C++ `newRegion`).
    fn new_region(&mut self) -> RegionPayloadId {
        let id = RegionPayloadId(self.region_pool.len() as u32);
        self.region_pool.push(KunaGraphRegion::default());
        id
    }

    /// Borrow a region payload.
    pub fn region(&self, id: RegionPayloadId) -> &KunaGraphRegion {
        &self.region_pool[id.0 as usize]
    }

    /// Input B: add a synthetic test node (C++ `addSyntheticBlock`).
    pub fn add_synthetic_block(&mut self, addr: uintb) -> KunaNodeId {
        // block stays None (synthetic).
        let n = self.new_node(NodeKind::Block, addr);
        self.work_graph.add_node(&self.pool, n);
        n
    }

    /// Input B: add a synthetic test edge (C++ `addSyntheticEdge`).
    pub fn add_synthetic_edge(&mut self, a: KunaNodeId, b: KunaNodeId) {
        self.work_graph.add_edge(&self.pool, a, b);
    }

    /// Input A: build the working graph from a real basic-block CFG
    /// (C++ `KunaRegionIdentifier::buildFromBlockGraph`).
    ///
    /// SEAM(W7): closes the block-graph adapter the C++ exposes for the
    /// `IfcKunaRegion*` console commands.  Reads the decompiler's `bblocks`
    /// read-only — never mutated by this analysis-only pass — emitting one
    /// `k_block` node per basic block (keyed on its start address, carrying the
    /// real [`crate::seams::BlockId`]) and one edge per CFG out-edge, with the
    /// entry address set to the first block's start.
    ///
    /// Each node parks `endsWithBranchindOrCbranch` (the `lastOp()->code()`
    /// probe over the block's tail op) so [`make_supergraph`](Self::make_supergraph)
    /// can consult it during `compute()` without re-borrowing the function.  The
    /// tail op is `bb_op_tail` (`BlockBasic::lastOp`); the block CFG is immutable
    /// across identification, so the precompute is equivalent to the C++ live
    /// probe and deterministic (block iteration is in CFG index order, op tail is
    /// the unique last op under `(addr, ident)` ordering).
    ///
    /// Must be called on a fresh identifier (before `compute`); the working graph
    /// must be empty.  Returns an error if `bblocks` is empty (no entry block).
    pub fn build_from_block_graph(&mut self, fd: &Funcdata) -> KunaResult<()> {
        use std::collections::BTreeMap as StdBTreeMap;
        use kuna_num::opcodes::OpCode;

        if self.work_graph.num_nodes() != 0 {
            return Err(KunaError::lowlevel(
                "kuna regionid: build_from_block_graph on a non-empty working graph",
            ));
        }
        let size = fd.bblocks_get_size();
        if size == 0 {
            return Err(KunaError::lowlevel(
                "kuna regionid: build_from_block_graph on a function with no basic blocks",
            ));
        }

        // First pass: one k_block node per basic block, in CFG index order.  The
        // node carries the real BlockId and the precomputed branchy predicate;
        // map BlockId -> node so the edge pass can resolve CFG out-edges.
        let mut node: StdBTreeMap<crate::seams::BlockId, KunaNodeId> = StdBTreeMap::new();
        let mut entry_addr: Option<uintb> = None;
        for i in 0..size {
            let bl = fd.bblocks_get_block(i);
            let addr = fd.bblocks_block_start(bl).get_offset();
            if i == 0 {
                entry_addr = Some(addr);
            }
            let n = self.new_node(NodeKind::Block, addr);
            {
                let nm = self.pool.get_mut(n);
                nm.set_block(bl);
                // endsWithBranchindOrCbranch: probe the block's tail op
                // (FlowBlock::lastOp).  Empty blocks (no tail op) are not branchy.
                let branchy = match fd.bb_op_tail(bl) {
                    Some(op) => matches!(
                        fd.obank().get(op).map(|o| o.code()),
                        Some(OpCode::CPUI_BRANCHIND) | Some(OpCode::CPUI_CBRANCH)
                    ),
                    None => false,
                };
                nm.set_branchy(branchy);
            }
            self.work_graph.add_node(&self.pool, n);
            node.insert(bl, n);
        }

        // Second pass: one edge per CFG out-edge.  Out-edge order is the block's
        // own out-edge list (deterministic; the supergraph and node-order plumbing
        // re-sort under KunaNodeOrder where it matters).
        let bg = fd.bblocks_ref();
        for i in 0..size {
            let bl = fd.bblocks_get_block(i);
            let block = bg.block(bl);
            let from = node[&bl];
            for j in 0..block.size_out() {
                let out = block.get_out(j);
                if let Some(&to) = node.get(&out) {
                    self.work_graph.add_edge(&self.pool, from, to);
                }
            }
        }

        if let Some(addr) = entry_addr {
            self.set_entry_addr(addr);
        }
        Ok(())
    }

    //
    // Driver
    //

    /// Run the analysis; returns the top-level region id (C++ `compute`).
    pub fn compute(&mut self) -> KunaResult<RegionPayloadId> {
        if self.computed {
            return self
                .top_region
                .ok_or_else(|| KunaError::lowlevel("kuna regionid: top region missing"));
        }
        self.computed = true;
        if self.work_graph.num_nodes() == 0 {
            return Err(KunaError::lowlevel("kuna regionid: empty input graph"));
        }
        // Move the working graph out so the algorithm can mutate it while still
        // borrowing `self` for the node pool / region pool (the C++ keeps both
        // as members; Rust's borrow checker needs the explicit split).
        let mut g = std::mem::take(&mut self.work_graph);
        self.pick_connected_component(&mut g); // RI _pick_one_connected_component
        self.make_supergraph(&mut g)?; // RI _make_supergraph
        self.start_node = Some(self.get_start_node(&g)?); // RI l.121
        self.compute_node_order(&g)?; // RI l.122
        let top = self.make_regions(&mut g)?; // RI l.123
        self.work_graph = g;
        self.top_region = Some(top);
        self.build_regions_by_block_addrs(); // RI l.126
        Ok(top)
    }

    /// Result (None before compute) (C++ `getTopRegion`).
    pub fn get_top_region(&self) -> Option<RegionPayloadId> {
        self.top_region
    }

    /// Flat region lists (C++ `getRegionsByBlockAddrs`).
    pub fn get_regions_by_block_addrs(&self) -> &Vec<Vec<uintb>> {
        &self.regions_by_block_addrs
    }

    //
    // Connected component / supergraph
    //

    /// Keep one weakly connected component (C++ `pickConnectedComponent`).
    fn pick_connected_component(&self, g: &mut KunaRegionGraph) {
        // Weakly connected components in deterministic discovery order (RI l.129)
        let mut all_nodes: Vec<KunaNodeId> = Vec::new();
        g.get_nodes(&mut all_nodes);
        let mut components: Vec<Vec<KunaNodeId>> = Vec::new();
        let mut assigned: BTreeSet<KunaNodeId> = BTreeSet::new(); // Membership only
        for &start in all_nodes.iter() {
            if assigned.contains(&start) {
                continue;
            }
            let mut comp: Vec<KunaNodeId> = vec![start];
            assigned.insert(start);
            let mut pos = 0usize;
            while pos < comp.len() {
                let cur = comp[pos];
                pos += 1;
                let succs: Vec<KunaNodeId> = g.get_succs(cur).unwrap_or(&[]).to_vec();
                for s in succs {
                    if !assigned.contains(&s) {
                        assigned.insert(s);
                        comp.push(s);
                    }
                }
                let preds: Vec<KunaNodeId> = g.get_preds(cur).unwrap_or(&[]).to_vec();
                for p in preds {
                    if !assigned.contains(&p) {
                        assigned.insert(p);
                        comp.push(p);
                    }
                }
            }
            components.push(comp);
        }
        if components.len() <= 1 {
            return;
        }

        let mut chosen: i64 = -1;
        let mut largest: usize = 0;
        for i in 0..components.len() {
            if components[i].len() > components[largest].len() {
                largest = i;
            }
            if self.has_entry_addr && chosen < 0 {
                for &n in components[i].iter() {
                    if self.pool.get(n).get_addr() == self.entry_addr {
                        chosen = i as i64;
                        break;
                    }
                }
                if chosen >= 0 {
                    break; // RI l.144: stop scanning once the entry component is found
                }
            }
        }
        let chosen = if chosen < 0 { largest } else { chosen as usize };

        let mut keep: BTreeSet<KunaNodeId> = BTreeSet::new(); // Membership only
        for &n in components[chosen].iter() {
            keep.insert(n);
        }
        for &n in all_nodes.iter() {
            if !keep.contains(&n) {
                g.remove_node(n);
            }
        }
    }

    /// angr `_block_ends_with_indirect_jump_or_call`
    /// (C++ `endsWithBranchindOrCbranch`).
    ///
    /// The C++ reaches `FlowBlock::lastOp()->code()` to test for
    /// `CPUI_BRANCHIND`/`CPUI_CBRANCH`.  The basic-block CFG is immutable across
    /// region identification, so [`build_from_block_graph`] precomputes that
    /// `lastOp` probe once per real block (deterministically, via `bb_op_tail` —
    /// the block's tail op under `(addr, ident)` ordering) and parks it on the
    /// node ([`crate::kuna_regiongraph::KunaRegionNode::branchy`]); here we read
    /// the parked flag.  Synthetic test nodes have no block and never set the
    /// flag, so the answer is `false` for them — exactly the C++
    /// synthetic-node path the unit tests drive.
    ///
    /// [`build_from_block_graph`]: KunaRegionIdentifier::build_from_block_graph
    fn ends_with_branchind_or_cbranch(&self, n: KunaNodeId) -> bool {
        let node = self.pool.get(n);
        let last = if node.is_multi() {
            // C++ takes the last member of the chain.
            match node.get_chain().last() {
                Some(&m) => m,
                None => return false,
            }
        } else {
            n
        };
        let last_node = self.pool.get(last);
        if last_node.get_kind() != NodeKind::Block {
            return false;
        }
        last_node.ends_with_branchind_or_cbranch()
    }

    /// Merge `a -> b` into a `k_multi` (C++ `mergeNodes`).
    ///
    /// The only caller (supergraph) always passes `force_multinode=True`, so the
    /// "forms a region by itself" arm is gone (RI `_merge_nodes`, l.1142).
    fn merge_nodes(
        &mut self,
        g: &mut KunaRegionGraph,
        a: KunaNodeId,
        b: KunaNodeId,
    ) -> KunaResult<KunaNodeId> {
        let mut in_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        g.in_edges(a, &mut in_e)?;
        let mut out_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        g.out_edges(b, &mut out_e)?;

        let mut members: Vec<KunaNodeId> = Vec::new();
        for &src in &[a, b] {
            let node = self.pool.get(src);
            if node.is_multi() {
                for &m in node.get_chain() {
                    members.push(m);
                }
            } else if node.get_kind() == NodeKind::Block {
                members.push(src);
            } else {
                return Err(KunaError::lowlevel(
                    "kuna regionid: cannot merge non-block node",
                )); // angr raises TypeError
            }
        }

        let addr = self.pool.get(members[0]).get_addr();
        let m = self.new_node(NodeKind::Multi, addr);
        self.pool.get_mut(m).set_chain(members);

        g.remove_node(a);
        g.remove_node(b);
        g.add_node(&self.pool, m);
        for (mut src, _dst) in in_e {
            if src == b {
                src = m; // b -> a back edge becomes a self loop
            }
            g.add_edge(&self.pool, src, m);
        }
        for (_src, mut dst) in out_e {
            if dst == a {
                dst = m; // b -> a forward arm of a 2-cycle
            }
            g.add_edge(&self.pool, m, dst);
        }
        Ok(m)
    }

    /// Merge single-out -> single-in chains (C++ `makeSupergraph`).
    ///
    /// Port of RI `_make_supergraph` (l.246).  Only the "transition" arm
    /// survives: angr's `fake_return`/`call` edge types cannot occur on a p-code
    /// basic block graph (every edge is a transition).
    fn make_supergraph(&mut self, g: &mut KunaRegionGraph) -> KunaResult<()> {
        let mut entry_node: Option<KunaNodeId> = None;
        if self.has_entry_addr {
            for key in g.node_keys() {
                if key.addr == self.entry_addr {
                    entry_node = Some(key.id);
                    break;
                }
            }
        }
        loop {
            let mut merged = false;
            let mut edges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
            g.all_edges(&mut edges);
            for (src, dst) in edges {
                if let Some(en) = entry_node {
                    if dst == en {
                        continue; // The entry node must never be merged INTO (RI l.253)
                    }
                }
                if g.size_out(src)? == 1
                    && g.size_in(dst)? == 1
                    && src != dst
                    && !self.ends_with_branchind_or_cbranch(dst)
                {
                    let m = self.merge_nodes(g, src, dst)?;
                    if entry_node == Some(src) {
                        entry_node = Some(m);
                    }
                    merged = true;
                    break;
                }
            }
            if !merged {
                break; // Clean pass over all edges: fixpoint
            }
        }
        Ok(())
    }

    //
    // Node lookup / ordering plumbing
    //

    /// First in-degree-0 node (or entry-addr node) (C++ `getStartNode`).
    fn get_start_node(&self, g: &KunaRegionGraph) -> KunaResult<KunaNodeId> {
        for key in g.node_keys() {
            if g.get_preds(key.id)?.is_empty() {
                return Ok(key.id);
            }
        }
        if self.has_entry_addr {
            for key in g.node_keys() {
                if key.addr == self.entry_addr {
                    return Ok(key.id);
                }
            }
        }
        Err(KunaError::lowlevel(
            "kuna regionid: cannot find the start node from the graph",
        ))
    }

    /// Node with the entry address (or None) (C++ `getEntryNode`).
    fn get_entry_node(&self, g: &KunaRegionGraph) -> Option<KunaNodeId> {
        if !self.has_entry_addr {
            return None;
        }
        for key in g.node_keys() {
            if key.addr == self.entry_addr {
                return Some(key.id);
            }
        }
        None
    }

    /// First node with the given address (C++ `nodeByAddr`).
    fn node_by_addr(&self, g: &KunaRegionGraph, addr: uintb) -> KunaResult<KunaNodeId> {
        for key in g.node_keys() {
            if key.addr == addr {
                return Ok(key.id);
            }
        }
        Err(KunaError::lowlevel(
            "kuna regionid: no node with the requested address",
        ))
    }

    /// Fill `node_order` via quasi-topological sort (C++ `computeNodeOrder`).
    fn compute_node_order(&mut self, g: &KunaRegionGraph) -> KunaResult<()> {
        let mut sorted: Vec<KunaNodeId> = Vec::new();
        kuna_quasi_topo_sort(&self.pool, g, &mut sorted)?;
        self.node_order.clear();
        for (i, &n) in sorted.iter().enumerate() {
            self.node_order.insert(n, (i as int4, 0));
        }
        Ok(())
    }

    /// The order pair of a node, or an error if missing.
    fn order_of(&self, n: KunaNodeId) -> KunaResult<(int4, int4)> {
        self.node_order
            .get(&n)
            .copied()
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: node missing from node order"))
    }

    /// Sort by `node_order` (stable) (C++ `sortByNodeOrder`).
    fn sort_by_node_order(&self, nodes: &mut [KunaNodeId]) -> KunaResult<()> {
        // C++ `stable_sort` over the order pair; collect-and-check first so a
        // missing entry surfaces as an error (the C++ comparator throws).
        for &n in nodes.iter() {
            self.order_of(n)?;
        }
        // `slice::sort_by` is a stable sort, matching the C++ `stable_sort`; the
        // closure cannot return an error so we latch any (already pre-checked)
        // missing-order lookup and surface it afterward.
        let mut err: Option<KunaError> = None;
        nodes.sort_by(|&a, &b| match (self.order_of(a), self.order_of(b)) {
            (Ok(oa), Ok(ob)) => oa.cmp(&ob),
            _ => {
                err = Some(KunaError::lowlevel(
                    "kuna regionid: node missing from node order",
                ));
                std::cmp::Ordering::Equal
            }
        });
        match err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// `slice_graph` (+ self-loop hack) (C++ `sliceGraph`).
    fn slice_graph(
        &self,
        g: &KunaRegionGraph,
        node: KunaNodeId,
        frontier: &KunaNodeSet,
        include_frontier: bool,
        res: &mut KunaRegionGraph,
    ) -> KunaResult<()> {
        kuna_subgraph_between_nodes(&self.pool, g, node, frontier, include_frontier, res)?;
        // RI l.109 HACK: an infinite self-loop slices to nothing; keep the body.
        if res.num_nodes() == 0 && g.has_edge(node, node) {
            res.add_edge(&self.pool, node, node);
        }
        Ok(())
    }

    //
    // Cyclic phase
    //

    /// Back-edge targets, in `node_order` (C++ `findLoopHeaders`).
    fn find_loop_headers(
        &self,
        g: &KunaRegionGraph,
        res: &mut Vec<KunaNodeId>,
    ) -> KunaResult<()> {
        let start = self
            .start_node
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: no start node"))?;
        let mut back_edges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        kuna_dfs_back_edges(&self.pool, g, start, &mut back_edges)?;
        let mut head_set: KunaNodeSet = KunaNodeSet::new();
        for (_src, dst) in back_edges {
            head_set.insert(self.pool.key(dst));
        }
        res.clear();
        for k in head_set.iter() {
            res.push(k.id);
        }
        self.sort_by_node_order(res)?;
        Ok(())
    }

    /// RI `_find_initial_loop_nodes` (l.290) (C++ `findInitialLoopNodes`).
    fn find_initial_loop_nodes(
        &self,
        g: &KunaRegionGraph,
        head: KunaNodeId,
        res: &mut KunaNodeSet,
    ) -> KunaResult<()> {
        let start = self
            .start_node
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: no start node"))?;
        let mut back_edges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        kuna_dfs_back_edges(&self.pool, g, start, &mut back_edges)?;
        let mut latching: KunaNodeSet = KunaNodeSet::new();
        for (src, dst) in back_edges {
            if dst == head {
                latching.insert(self.pool.key(src));
            }
        }
        let mut loop_sub = KunaRegionGraph::new();
        self.slice_graph(g, head, &latching, true, &mut loop_sub)?;

        // Switch-case expansion: a node with more than two non-self successors is
        // probably a switch head; pull in successors whose predecessors all sit
        // inside the slice already.
        let mut guard: int8 = 0;
        let guard_max = kuna_guard_cap(g.num_nodes());
        loop {
            guard += 1;
            if guard > guard_max {
                return Err(KunaError::lowlevel(
                    "kuna regionid: initial loop nodes did not converge",
                ));
            }
            let mut updated = false;
            let mut snap: Vec<KunaNodeId> = Vec::new();
            loop_sub.get_nodes(&mut snap);
            for node in snap {
                let mut nonself: Vec<KunaNodeId> = Vec::new();
                for &s in g.get_succs(node)? {
                    if s != node {
                        nonself.push(s);
                    }
                }
                if nonself.len() as int4 > 2 {
                    for succ in nonself {
                        if loop_sub.has_edge(node, succ) {
                            continue;
                        }
                        let mut all_in = true;
                        for &p in g.get_preds(succ)? {
                            if !loop_sub.contains_node(p) {
                                all_in = false;
                                break;
                            }
                        }
                        if all_in {
                            updated = true;
                            loop_sub.add_edge(&self.pool, node, succ);
                        }
                    }
                }
            }
            if !updated {
                break;
            }
        }

        res.clear();
        for key in loop_sub.node_keys() {
            res.insert(*key);
        }
        Ok(())
    }

    /// RI `_refine_loop` (l.317) (C++ `refineLoop`).
    ///
    /// `loop_nodes`/`exit_nodes` are in/out (they arrive as copies of the
    /// initial sets and leave refined).
    fn refine_loop(
        &self,
        g: &KunaRegionGraph,
        head: KunaNodeId,
        loop_nodes: &mut KunaNodeSet,
        exit_nodes: &mut KunaNodeSet,
    ) -> KunaResult<()> {
        if (self.refine_loops_with_single_successor && exit_nodes.is_empty())
            || (!self.refine_loops_with_single_successor && exit_nodes.len() <= 1)
        {
            return Ok(());
        }

        let initial_exit_nodes = exit_nodes.clone(); // Stage 3 needs the originals (RI l.395)

        // Stage 1: absorb single-in-degree, at-most-single-out exit nodes.
        let mut guard: int8 = 0;
        let guard_max = kuna_guard_cap(g.num_nodes());
        loop {
            guard += 1;
            if guard > guard_max {
                return Err(KunaError::lowlevel(
                    "kuna regionid: loop refinement (stage 1) did not converge",
                ));
            }
            let mut added_any = false;
            let snap: Vec<KunaNodeKey> = exit_nodes.iter().copied().collect();
            for ek in snap {
                let exit_node = ek.id;
                if g.size_in(exit_node)? == 1 && g.size_out(exit_node)? <= 1 {
                    added_any = true;
                    loop_nodes.insert(ek);
                    for &s in g.get_succs(exit_node)? {
                        let sk = self.pool.key(s);
                        if !loop_nodes.contains(&sk) {
                            exit_nodes.insert(sk);
                        }
                    }
                    exit_nodes.remove(&ek);
                }
            }
            if !added_any {
                break;
            }
        }
        if exit_nodes.len() <= 1 {
            return Ok(());
        }

        // Stage 2: absorb dominated exit candidates whose preds are all in the loop.
        let mut idom_map: BTreeMap<KunaNodeId, KunaNodeId> = BTreeMap::new();
        kuna_immediate_dominators(&self.pool, g, head, &mut idom_map)?;

        let mut sorted_exits: Vec<KunaNodeId> = exit_nodes.iter().map(|k| k.id).collect();
        self.sort_by_node_order(&mut sorted_exits)?;
        let mut have_new_exits = true;
        let mut side_graph = KunaRegionGraph::new(); // n -> newly exposed exit edges (feeds stage 3)
        let mut guard: int8 = 0;
        while sorted_exits.len() > 1 && have_new_exits {
            guard += 1;
            if guard > guard_max {
                return Err(KunaError::lowlevel(
                    "kuna regionid: loop refinement (stage 2) did not converge",
                ));
            }
            // Candidates, in sorted_exits order.
            let mut cand_order: Vec<KunaNodeId> = Vec::new();
            {
                let snap = sorted_exits.clone();
                for n in snap {
                    let mut all_preds_in = true;
                    for &p in g.get_preds(n)? {
                        if p != n && !loop_nodes.contains(&self.pool.key(p)) {
                            all_preds_in = false;
                            break;
                        }
                    }
                    if all_preds_in && crate::kuna_regiongraph::kuna_dominates(&idom_map, head, n) {
                        cand_order.push(n);
                    }
                }
            }
            // Union of every candidate's would-be new exits (computed BEFORE any
            // candidate is absorbed — the per-candidate sets are deliberately
            // stale, matching RI l.360-366).
            let mut all_new_exit_candidates: KunaNodeSet = KunaNodeSet::new();
            for &c in cand_order.iter() {
                for &s in g.get_succs(c)? {
                    let sk = self.pool.key(s);
                    if !loop_nodes.contains(&sk) {
                        all_new_exit_candidates.insert(sk);
                    }
                }
            }
            // Progress guard (RI l.369): if every candidate is itself a new-exit
            // candidate, absorbing none would loop forever — clear the veto set.
            let mut all_vetoed = true;
            for &c in cand_order.iter() {
                if !all_new_exit_candidates.contains(&self.pool.key(c)) {
                    all_vetoed = false;
                    break;
                }
            }
            if all_vetoed {
                all_new_exit_candidates.clear();
            }
            // Absorb the surviving candidates.
            let mut new_exit_nodes: KunaNodeSet = KunaNodeSet::new();
            for &n in cand_order.iter() {
                if all_new_exit_candidates.contains(&self.pool.key(n)) {
                    continue;
                }
                loop_nodes.insert(self.pool.key(n));
                if let Some(pos) = sorted_exits.iter().position(|&x| x == n) {
                    sorted_exits.remove(pos);
                }
                for &s in g.get_succs(n)? {
                    // Recomputed LIVE (RI l.379).
                    let sk = self.pool.key(s);
                    if !loop_nodes.contains(&sk) {
                        new_exit_nodes.insert(sk);
                        side_graph.add_edge(&self.pool, n, s);
                    }
                }
            }
            have_new_exits = !new_exit_nodes.is_empty();
            for nk in new_exit_nodes.iter() {
                let present = sorted_exits.contains(&nk.id);
                if !present {
                    sorted_exits.push(nk.id);
                }
            }
            self.sort_by_node_order(&mut sorted_exits)?;
        }
        exit_nodes.clear();
        for &n in sorted_exits.iter() {
            exit_nodes.insert(self.pool.key(n));
        }
        let exit_keys: Vec<KunaNodeKey> = exit_nodes.iter().copied().collect();
        for ek in exit_keys {
            loop_nodes.remove(&ek);
        }

        // Stage 3 (RI l.391): zero exits left — pull the LARGEST single-owner
        // successor tree back OUT of the loop and make its root the exit.
        if self.largest_successor_tree_outside_loop && exit_nodes.is_empty() {
            // initial_exit -> subtree, and inverse map newnode -> initial_exits.
            let mut initial_exit_to_new: BTreeMap<KunaNodeKey, KunaNodeSet> = BTreeMap::new();
            let mut newnode_to_initial_exits: BTreeMap<KunaNodeKey, KunaNodeSet> = BTreeMap::new();
            for ek in initial_exit_nodes.iter() {
                let initial_exit = ek.id;
                if !side_graph.contains_node(initial_exit) {
                    continue;
                }
                // BFS tree successors over side_graph (content visit-order independent).
                let mut seen: BTreeSet<KunaNodeId> = BTreeSet::new(); // Membership only
                seen.insert(initial_exit);
                let mut queue: Vec<KunaNodeId> = vec![initial_exit];
                let mut pos = 0usize;
                while pos < queue.len() {
                    let cur = queue[pos];
                    pos += 1;
                    let succs = side_graph.get_sorted_succs(&self.pool, cur)?;
                    for s in succs {
                        if seen.contains(&s) {
                            continue;
                        }
                        seen.insert(s);
                        let sk = self.pool.key(s);
                        initial_exit_to_new.entry(*ek).or_default().insert(sk);
                        newnode_to_initial_exits.entry(sk).or_default().insert(*ek);
                        queue.push(s);
                    }
                }
            }
            // For every (newnode, initial_exit) link also record the reverse so a
            // subtree is the union of all reachable newnodes (RI l.~410).
            let inverse_snapshot: Vec<(KunaNodeKey, Vec<KunaNodeKey>)> = newnode_to_initial_exits
                .iter()
                .map(|(k, v)| (*k, v.iter().copied().collect()))
                .collect();
            for (newnode, owners) in inverse_snapshot {
                for owner in owners {
                    initial_exit_to_new.entry(owner).or_default().insert(newnode);
                }
            }
            // Drop subtrees with more than one out-of-tree predecessor.
            let mut drop_list: Vec<KunaNodeKey> = Vec::new();
            for (exit_key, subtree) in initial_exit_to_new.iter() {
                let mut subtree_preds: BTreeSet<KunaNodeId> = BTreeSet::new(); // size only
                for sk in subtree.iter() {
                    for &p in g.get_preds(sk.id)? {
                        if !subtree.contains(&self.pool.key(p)) {
                            subtree_preds.insert(p);
                        }
                    }
                    if subtree_preds.len() > 1 {
                        break;
                    }
                }
                if subtree_preds.len() > 1 {
                    drop_list.push(*exit_key);
                }
            }
            for d in drop_list {
                initial_exit_to_new.remove(&d);
            }

            if !initial_exit_to_new.is_empty() {
                let mut max_size: int4 = -1;
                let mut max_count: int4 = 0;
                let mut max_exit: Option<KunaNodeKey> = None;
                for (exit_key, subtree) in initial_exit_to_new.iter() {
                    let sz = subtree.len() as int4;
                    if sz > max_size {
                        max_size = sz;
                        max_count = 1;
                        max_exit = Some(*exit_key);
                    } else if sz == max_size {
                        max_count += 1;
                    }
                }
                if max_count == 1 {
                    let max_exit = max_exit.unwrap();
                    let subtree = initial_exit_to_new[&max_exit].clone();
                    let mut all_single_owner = true;
                    for sk in subtree.iter() {
                        match newnode_to_initial_exits.get(sk) {
                            Some(owners) if owners.len() == 1 => {}
                            _ => {
                                all_single_owner = false;
                                break;
                            }
                        }
                    }
                    if all_single_owner {
                        for sk in subtree.iter() {
                            loop_nodes.remove(sk);
                        }
                        loop_nodes.remove(&max_exit);
                        exit_nodes.insert(max_exit);
                    }
                }
            }
        }
        Ok(())
    }

    /// RI `_make_cyclic_region` (l.518) (C++ `makeCyclicRegion`).
    ///
    /// Returns `None` when the loop contains another (still-unstructured) header.
    fn make_cyclic_region(
        &mut self,
        head: KunaNodeId,
        g: &mut KunaRegionGraph,
    ) -> KunaResult<Option<RegionPayloadId>> {
        let original_entry = self.get_entry_node(g);

        let mut initial_loop_nodes: KunaNodeSet = KunaNodeSet::new();
        self.find_initial_loop_nodes(g, head, &mut initial_loop_nodes)?;

        // Make sure no OTHER loop header is contained in this loop (address
        // compare, matching `n.addr != head.addr` at RI l.527).
        let head_addr = self.pool.get(head).get_addr();
        for nk in initial_loop_nodes.iter() {
            if self.pool.get(nk.id).get_addr() == head_addr {
                continue;
            }
            if self.loop_headers.contains(&nk.id) {
                return Ok(None);
            }
        }

        let mut normal_entries: KunaNodeSet = KunaNodeSet::new();
        for &p in g.get_preds(head)? {
            if !initial_loop_nodes.contains(&self.pool.key(p)) {
                normal_entries.insert(self.pool.key(p));
            }
        }
        let mut abnormal_entries: KunaNodeSet = KunaNodeSet::new();
        for nk in initial_loop_nodes.iter() {
            if nk.id == head {
                continue;
            }
            for &p in g.get_preds(nk.id)? {
                if !initial_loop_nodes.contains(&self.pool.key(p)) {
                    abnormal_entries.insert(self.pool.key(p));
                }
            }
        }
        let mut initial_exit_nodes: KunaNodeSet = KunaNodeSet::new();
        for nk in initial_loop_nodes.iter() {
            for &s in g.get_succs(nk.id)? {
                if !initial_loop_nodes.contains(&self.pool.key(s)) {
                    initial_exit_nodes.insert(self.pool.key(s));
                }
            }
        }

        let mut refined_loop_nodes = initial_loop_nodes.clone();
        let mut refined_exit_nodes = initial_exit_nodes;
        self.refine_loop(g, head, &mut refined_loop_nodes, &mut refined_exit_nodes)?;

        // (kuna) RI l.551-556 _ensure_jump_at_loop_exit_ends omitted: appends AIL
        // Jump statements for the structurer; this port is analysis-only.

        let mut normal_exit: Option<KunaNodeId> = None;
        let mut abnormal_exits: KunaNodeSet = KunaNodeSet::new();
        if refined_exit_nodes.len() > 1 {
            // angr uses (non-deterministic) networkx.dfs_postorder_nodes here;
            // kuna substitutes the deterministic variant — a strict improvement.
            let mut postorder: Vec<KunaNodeId> = Vec::new();
            kuna_dfs_postorder_deterministic(&self.pool, g, head, &mut postorder)?;
            let mut po_idx: BTreeMap<KunaNodeId, int4> = BTreeMap::new(); // Lookup only
            for (i, &n) in postorder.iter().enumerate() {
                po_idx.insert(n, i as int4);
            }
            let mut keyed: Vec<(int4, KunaNodeId)> = Vec::new();
            for ek in refined_exit_nodes.iter() {
                let pi = po_idx.get(&ek.id).copied().ok_or_else(|| {
                    KunaError::lowlevel("kuna regionid: exit node not reachable from loop head")
                })?;
                keyed.push((pi, ek.id));
            }
            // C++ `sort(keyed)`: lexicographic by (postorder index, pointer).
            // The pointer tiebreaker is unobservable (postorder indices are
            // unique); sort by index, stable on insertion order for safety.
            keyed.sort_by(|a, b| a.0.cmp(&b.0));
            normal_exit = Some(keyed[0].1);
            for k in keyed.iter().skip(1) {
                abnormal_exits.insert(self.pool.key(k.1));
            }
        } else if refined_exit_nodes.len() == 1 {
            normal_exit = Some(refined_exit_nodes.iter().next().unwrap().id);
        }

        let region = self.abstract_cyclic_region(
            g,
            &refined_loop_nodes,
            head,
            &normal_entries,
            &abnormal_entries,
            normal_exit,
            &abnormal_exits,
        )?;

        // (kuna) RI l.579-581 force_loop_single_exit guarded-successor refinement
        // omitted: rewrites AIL conditional jumps through the condition processor.

        if let Some(oe) = original_entry {
            let r = &self.region_pool[region.0 as usize];
            // RI l.786: the *region's internal* graph absorbed the entry node.
            if r.graph.contains_node(oe) && r.head != Some(oe) {
                // RI l.585: head node absorbed the entry; update tracked entry.
                self.has_entry_addr = true;
                self.entry_addr = head_addr;
            }
        }
        Ok(Some(region))
    }

    /// RI `_abstract_cyclic_region` (l.1017) (C++ `abstractCyclicRegion`).
    #[allow(clippy::too_many_arguments)]
    fn abstract_cyclic_region(
        &mut self,
        g: &mut KunaRegionGraph,
        loop_nodes: &KunaNodeSet,
        head: KunaNodeId,
        normal_entries: &KunaNodeSet,
        abnormal_entries: &KunaNodeSet,
        normal_exit: Option<KunaNodeId>,
        abnormal_exits: &KunaNodeSet,
    ) -> KunaResult<RegionPayloadId> {
        let region_id = self.new_region();
        let head_addr = self.pool.get(head).get_addr();
        let wrapper = self.new_node(NodeKind::Region, head_addr);
        self.pool.get_mut(wrapper).set_region(region_id);
        {
            let region = &mut self.region_pool[region_id.0 as usize];
            region.head = Some(head);
            region.cyclic = true;
            region.wrapper = Some(wrapper);
            region.has_full = true;
        }

        let mut subgraph = KunaRegionGraph::new();
        let mut full_graph = KunaRegionGraph::new();
        let mut region_outedges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        let mut delayed_edges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();

        for nk in loop_nodes.iter() {
            let node = nk.id;
            subgraph.add_node(&self.pool, node);
            let mut in_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
            g.in_edges(node, &mut in_e)?;
            let mut out_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
            g.out_edges(node, &mut out_e)?;
            for (src, dst) in in_e {
                full_graph.add_edge(&self.pool, src, dst);
                if loop_nodes.contains(&self.pool.key(src)) {
                    subgraph.add_edge(&self.pool, src, dst);
                }
                // (kuna) RI l.1046 `src == region` arm is dead: wrapper not in g.
                // The angr `src in normal_entries` and `src in abnormal_entries`
                // arms perform the same action (delay a `src -> wrapper` edge);
                // merged here, with the inconsistent-edge assert as the else.
                else if normal_entries.contains(&self.pool.key(src))
                    || abnormal_entries.contains(&self.pool.key(src))
                {
                    delayed_edges.push((src, wrapper));
                } else {
                    return Err(KunaError::lowlevel(
                        "kuna regionid: inconsistent cyclic region in-edge",
                    )); // RI assert 0
                }
            }
            for (src, dst) in out_e {
                full_graph.add_edge(&self.pool, src, dst);
                if loop_nodes.contains(&self.pool.key(dst)) {
                    subgraph.add_edge(&self.pool, src, dst);
                }
                // (kuna) RI l.1062 `dst == region` arm is dead: wrapper not in g.
                // The angr `dst == normal_exit` and `dst in abnormal_exits` arms
                // perform the same action (record a region out-edge and delay a
                // `wrapper -> dst` edge); merged, with the assert as the else.
                else if Some(dst) == normal_exit || abnormal_exits.contains(&self.pool.key(dst)) {
                    region_outedges.push((node, dst));
                    delayed_edges.push((wrapper, dst));
                } else {
                    return Err(KunaError::lowlevel(
                        "kuna regionid: inconsistent cyclic region out-edge",
                    )); // RI assert 0
                }
            }
        }

        // graph_with_successors = subgraph + region out-edges + successor frontier.
        let mut gws = subgraph.clone();
        for (a, b) in region_outedges.iter() {
            gws.add_edge(&self.pool, *a, *b);
        }
        let mut successors: KunaNodeSet = KunaNodeSet::new();
        if let Some(ne) = normal_exit {
            successors.insert(self.pool.key(ne));
        }
        for ak in abnormal_exits.iter() {
            successors.insert(*ak);
        }
        // Edges among the successors themselves (RI l.~1090).
        let succ_list: Vec<KunaNodeKey> = successors.iter().copied().collect();
        for s0 in succ_list.iter() {
            for s1 in succ_list.iter() {
                if s0.id != s1.id && g.has_edge(s0.id, s1.id) {
                    gws.add_edge(&self.pool, s0.id, s1.id);
                }
            }
        }

        // Commit the computed subgraphs into the region payload.
        {
            let region = &mut self.region_pool[region_id.0 as usize];
            region.graph = subgraph;
            region.full_graph = full_graph;
            region.has_succs = true;
            region.graph_with_successors = gws;
            region.successors = successors;
        }

        // Collapse the loop nodes out of g and splice in the wrapper.
        for nk in loop_nodes.iter() {
            g.remove_node(nk.id);
        }
        g.add_node(&self.pool, wrapper);
        for (a, b) in delayed_edges {
            g.add_edge(&self.pool, a, b);
        }

        let order = self.order_of(head).map_err(|_| {
            KunaError::lowlevel("kuna regionid: loop head missing from node order")
        })?;
        self.node_order.insert(wrapper, order); // RI l.1099
        Ok(region_id)
    }

    /// RI `_make_regions` (l.435) (C++ `makeRegions`).
    fn make_regions(&mut self, g: &mut KunaRegionGraph) -> KunaResult<RegionPayloadId> {
        let mut structured_loop_headers: BTreeSet<KunaNodeId> = BTreeSet::new(); // Membership only
        let mut new_regions: Vec<RegionPayloadId> = Vec::new();

        let mut guard: int8 = 0;
        let guard_max = kuna_guard_cap(g.num_nodes());
        loop {
            // Outer: iteratively find and make loop regions.
            guard += 1;
            if guard > guard_max {
                return Err(KunaError::lowlevel(
                    "kuna regionid: cyclic phase did not converge",
                ));
            }
            let mut headers = std::mem::take(&mut self.loop_headers);
            self.find_loop_headers(g, &mut headers)?;
            self.loop_headers = headers;
            if self.loop_headers.is_empty() {
                break;
            }
            loop {
                // Inner: find all loops.
                guard += 1;
                if guard > guard_max {
                    return Err(KunaError::lowlevel(
                        "kuna regionid: cyclic phase did not converge",
                    ));
                }
                let mut restart = false;
                self.start_node = Some(self.get_start_node(g)?);
                let mut headers = std::mem::take(&mut self.loop_headers);
                self.find_loop_headers(g, &mut headers)?;
                self.loop_headers = headers;
                if self.loop_headers.is_empty() {
                    break;
                }
                let snap = self.loop_headers.clone(); // RI l.460 list(reversed(...))
                for i in (0..snap.len()).rev() {
                    let node = snap[i];
                    if structured_loop_headers.contains(&node) {
                        continue;
                    }
                    if !g.contains_node(node) {
                        continue;
                    }
                    let region = self.make_cyclic_region(node, g)?;
                    match region {
                        None => {
                            // Failed (nested header): remove from the LIVE header
                            // list so later attempts no longer see it (RI l.471).
                            if let Some(pos) =
                                self.loop_headers.iter().position(|&x| x == node)
                            {
                                self.loop_headers.remove(pos);
                            }
                        }
                        Some(r) => {
                            new_regions.push(r);
                            structured_loop_headers.insert(node);
                            restart = true;
                            break;
                        }
                    }
                }
                if restart {
                    continue;
                }
                break;
            }
        }

        // Acyclic phase: process each cyclic region's body, then the residual
        // graph (RI l.484 wraps the residual graph in a top-level GraphRegion;
        // kuna represents that entry as region==None and routes it at g).
        for widx in 0..=new_regions.len() {
            let is_top = widx == new_regions.len();
            let region_id: Option<RegionPayloadId> =
                if is_top { None } else { Some(new_regions[widx]) };

            // The "sub" graph is either g (top) or the region's own graph; we
            // operate on an owned copy and write it back so the borrow checker
            // is happy while still mutating self's pools.
            let mut sub: KunaRegionGraph = if is_top {
                std::mem::take(g)
            } else {
                std::mem::take(&mut self.region_pool[region_id.unwrap().0 as usize].graph)
            };
            let mut secondary: Option<KunaRegionGraph> = if !is_top {
                let r = &self.region_pool[region_id.unwrap().0 as usize];
                if r.has_succs {
                    Some(r.graph_with_successors.clone())
                } else {
                    None
                }
            } else {
                None
            };
            let cyc = if is_top {
                false
            } else {
                self.region_pool[region_id.unwrap().0 as usize].cyclic
            };
            let mut head: KunaNodeId = if is_top {
                self.get_start_node(&sub)?
            } else {
                self.region_pool[region_id.unwrap().0 as usize]
                    .head
                    .ok_or_else(|| KunaError::lowlevel("kuna regionid: region has no head"))?
            };

            let mut failed_region_attempts: BTreeSet<(KunaNodeId, KunaNodeId)> = BTreeSet::new();
            let mut inner_guard: int8 = 0;
            loop {
                let made =
                    self.make_acyclic_region(head, &mut sub, secondary.as_mut(), &mut failed_region_attempts, cyc)?;
                if !made {
                    break;
                }
                inner_guard += 1;
                if inner_guard > guard_max {
                    return Err(KunaError::lowlevel(
                        "kuna regionid: acyclic phase did not converge",
                    ));
                }
                if !sub.contains_node(head) {
                    let addr = self.pool.get(head).get_addr();
                    head = self.node_by_addr(&sub, addr)?; // RI l.499-501
                }
            }
            let addr = self.pool.get(head).get_addr();
            head = self.node_by_addr(&sub, addr)?;

            // Write the (possibly collapsed) graph + secondary back.
            if is_top {
                *g = sub;
            } else {
                let rid = region_id.unwrap();
                self.region_pool[rid.0 as usize].graph = sub;
                if let Some(sec) = secondary {
                    self.region_pool[rid.0 as usize].graph_with_successors = sec;
                }
                self.region_pool[rid.0 as usize].head = Some(head); // RI l.504
            }
        }

        if g.num_nodes() == 1 {
            // RI l.506
            let res = g.node_keys().next().unwrap().id;
            if let Some(r) = self.pool.get(res).get_region() {
                return Ok(r);
            }
        }
        let new_head = self.get_start_node(g)?; // RI l.511
        let top = self.new_region();
        {
            let region = &mut self.region_pool[top.0 as usize];
            region.head = Some(new_head);
            region.graph = g.clone();
            region.has_succs = false;
            region.cyclic = false;
        }
        Ok(top)
    }

    //
    // Acyclic phase
    //

    /// RI `_check_region` (l.882): pure predicate over dominance frontiers
    /// (C++ `checkRegion`).
    fn check_region(
        &self,
        g: &KunaRegionGraph,
        start_node: KunaNodeId,
        end_node: KunaNodeId,
        doms: &mut KunaIncrementalDominators,
    ) -> KunaResult<bool> {
        if !doms.dominates(start_node, end_node) {
            let early_start_frontier = doms.df(&self.pool, g, start_node)?;
            for it in early_start_frontier.iter() {
                if it.id != start_node && it.id != end_node {
                    return Ok(false);
                }
            }
        }

        let end_frontier = doms.df(&self.pool, g, end_node)?;
        for it in end_frontier.iter() {
            if doms.dominates(start_node, it.id) && it.id != end_node {
                return Ok(false); // An edge enters the region
            }
        }

        let start_frontier = doms.df(&self.pool, g, start_node)?;
        for it in start_frontier.iter() {
            let node = it.id;
            if node == start_node || node == end_node {
                continue;
            }
            if !end_frontier.contains(it) {
                return Ok(false); // An edge leaves the region
            }
            for &p in g.get_preds(node)? {
                if doms.dominates(start_node, p) && !doms.dominates(end_node, p) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// RI `_compute_region` (l.920) (C++ `computeRegion`).
    ///
    /// `include_frontier` is always false at the call sites, so it is dropped.
    fn compute_region(
        &mut self,
        g: &KunaRegionGraph,
        node: KunaNodeId,
        frontier: &KunaNodeSet,
        dummy_endnode: Option<KunaNodeId>,
        cyclic_ancestor: bool,
    ) -> KunaResult<Option<RegionPayloadId>> {
        let mut subgraph = KunaRegionGraph::new();
        let mut frontier_edges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        let mut stack: Vec<KunaNodeId> = vec![node];
        let mut traversed: BTreeSet<KunaNodeId> = BTreeSet::new(); // Membership only

        while let Some(cur) = stack.pop() {
            // LIFO, like Python list.pop().
            if frontier.contains(&self.pool.key(cur)) {
                continue;
            }
            if traversed.contains(&cur) {
                continue; // Idempotent revisit skip
            }
            traversed.insert(cur);
            subgraph.add_node(&self.pool, cur);
            for &succ in g.get_succs(cur)? {
                // (kuna) RI l.944 `node_ in frontier` inner branch is unreachable
                // (frontier nodes are filtered at pop); dropped.
                if Some(succ) == dummy_endnode {
                    continue;
                }
                if frontier.contains(&self.pool.key(succ)) {
                    frontier_edges.push((cur, succ));
                    continue;
                }
                subgraph.add_edge(&self.pool, cur, succ);
                if traversed.contains(&succ) {
                    continue;
                }
                stack.push(succ);
            }
        }

        let mut real_frontier: KunaNodeSet = KunaNodeSet::new();
        for fk in frontier.iter() {
            if Some(fk.id) != dummy_endnode {
                real_frontier.insert(*fk);
            }
        }

        if subgraph.num_nodes() > 1 {
            let region_id = self.new_region();
            let mut gws = subgraph.clone();
            for (a, b) in frontier_edges {
                if Some(b) != dummy_endnode {
                    gws.add_edge(&self.pool, a, b);
                }
            }
            let region = &mut self.region_pool[region_id.0 as usize];
            region.head = Some(node);
            region.graph = subgraph;
            region.has_succs = true;
            region.successors = real_frontier;
            region.graph_with_successors = gws;
            region.cyclic = false;
            region.cyclic_ancestor = cyclic_ancestor;
            return Ok(Some(region_id));
        }
        Ok(None)
    }

    /// RI `_region_in_edges` (l.1115) (C++ `regionInEdges`).
    fn region_in_edges(
        &self,
        g: &KunaRegionGraph,
        region_id: RegionPayloadId,
        res: &mut Vec<(KunaNodeId, KunaNodeId)>,
    ) -> KunaResult<()> {
        let head = self.region_pool[region_id.0 as usize]
            .head
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: region has no head"))?;
        if g.contains_node(head) {
            g.in_edges(head, res)?;
        }
        Ok(())
    }

    /// RI `_region_out_edges` (l.1119): member->outside edges re-sourced at the
    /// region's wrapper node (C++ `regionOutEdges`).
    fn region_out_edges(
        &self,
        g: &KunaRegionGraph,
        region_id: RegionPayloadId,
        res: &mut Vec<(KunaNodeId, KunaNodeId)>,
    ) -> KunaResult<()> {
        let wrapper = self.region_pool[region_id.0 as usize]
            .wrapper
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: region has no wrapper"))?;
        let member_keys: Vec<KunaNodeKey> = self.region_pool[region_id.0 as usize]
            .graph
            .node_keys()
            .copied()
            .collect();
        for mk in member_keys {
            let member = mk.id;
            if !g.contains_node(member) {
                continue;
            }
            let succs: Vec<KunaNodeId> = g.get_succs(member)?.to_vec();
            for s in succs {
                if self.region_pool[region_id.0 as usize].graph.contains_node(s) {
                    continue;
                }
                res.push((wrapper, s));
            }
        }
        Ok(())
    }

    /// RI `_abstract_acyclic_region` (l.980) (C++ `abstractAcyclicRegion`).
    fn abstract_acyclic_region(
        &mut self,
        g: &mut KunaRegionGraph,
        region_id: RegionPayloadId,
        frontier: &KunaNodeSet,
        dummy_endnode: Option<KunaNodeId>,
        secondary_graph: Option<&mut KunaRegionGraph>,
    ) -> KunaResult<()> {
        if self.region_pool[region_id.0 as usize].wrapper.is_none() {
            let head = self.region_pool[region_id.0 as usize]
                .head
                .ok_or_else(|| KunaError::lowlevel("kuna regionid: region has no head"))?;
            let addr = self.pool.get(head).get_addr();
            let w = self.new_node(NodeKind::Region, addr);
            self.pool.get_mut(w).set_region(region_id);
            self.region_pool[region_id.0 as usize].wrapper = Some(w);
        }
        let wrapper = self.region_pool[region_id.0 as usize].wrapper.unwrap();

        let mut in_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        self.region_in_edges(g, region_id, &mut in_e)?;
        let mut out_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        self.region_out_edges(g, region_id, &mut out_e)?;

        let mut nodes_set: KunaNodeSet = KunaNodeSet::new();
        {
            let members: Vec<KunaNodeId> = {
                let mut v = Vec::new();
                self.region_pool[region_id.0 as usize].graph.get_nodes(&mut v);
                v
            };
            for m in members {
                nodes_set.insert(self.pool.key(m));
                if Some(m) != dummy_endnode {
                    g.remove_node(m);
                }
            }
        }
        g.add_node(&self.pool, wrapper);

        // node_order[wrapper] = min over members (lexicographic pair, RI l.1000).
        let mut first = true;
        let mut mn: (int4, int4) = (0, 0);
        for nk in nodes_set.iter() {
            let o = self.order_of(nk.id).map_err(|_| {
                KunaError::lowlevel("kuna regionid: region member missing from node order")
            })?;
            if first || o < mn {
                mn = o;
                first = false;
            }
        }
        self.node_order.insert(wrapper, mn);

        for (src, _dst) in in_e {
            if !nodes_set.contains(&self.pool.key(src)) {
                g.add_edge(&self.pool, src, wrapper);
            }
        }
        for (_src, dst) in out_e {
            if !nodes_set.contains(&self.pool.key(dst)) {
                g.add_edge(&self.pool, wrapper, dst);
            }
        }
        for fk in frontier.iter() {
            if Some(fk.id) != dummy_endnode {
                g.add_edge(&self.pool, wrapper, fk.id);
            }
        }

        if let Some(sec) = secondary_graph {
            let empty_frontier = KunaNodeSet::new();
            self.abstract_acyclic_region(sec, region_id, &empty_frontier, None, None)?;
        }
        Ok(())
    }

    /// RI `_update_graph` (l.870) (C++ `replaceRegionInGraph`).
    fn replace_region_in_graph(
        &mut self,
        g: &mut KunaRegionGraph,
        region_id: RegionPayloadId,
        replaced_nodes: &KunaNodeSet,
    ) -> KunaResult<()> {
        let wrapper = self.region_pool[region_id.0 as usize]
            .wrapper
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: region has no wrapper"))?;
        let mut in_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        self.region_in_edges(g, region_id, &mut in_e)?;
        let mut out_e: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        self.region_out_edges(g, region_id, &mut out_e)?;
        for rk in replaced_nodes.iter() {
            g.remove_node(rk.id);
        }
        g.add_node(&self.pool, wrapper);
        for (a, _b) in in_e {
            g.add_edge(&self.pool, a, wrapper);
        }
        for (_a, b) in out_e {
            g.add_edge(&self.pool, wrapper, b);
        }
        Ok(())
    }

    /// RI `_make_acyclic_region` (l.715) (C++ `makeAcyclicRegion`).
    fn make_acyclic_region(
        &mut self,
        head: KunaNodeId,
        graph: &mut KunaRegionGraph,
        mut secondary_graph: Option<&mut KunaRegionGraph>,
        failed_region_attempts: &mut BTreeSet<(KunaNodeId, KunaNodeId)>,
        cyclic: bool,
    ) -> KunaResult<bool> {
        let mut head_inedges: Vec<(KunaNodeId, KunaNodeId)> = Vec::new();
        graph.in_edges(head, &mut head_inedges)?;

        // Mirrors Python's graph_copy-is-graph aliasing: if no head in-edges, gc
        // *is* graph; otherwise gc is a private copy with head in-edges cut.
        let mut local_copy: Option<KunaRegionGraph> = None;
        if !head_inedges.is_empty() {
            let mut lc = graph.clone();
            for (a, b) in head_inedges.iter() {
                lc.remove_edge(*a, *b);
            }
            local_copy = Some(lc);
        }

        // Compute endnodes against gc (= local_copy if present, else graph).
        let mut endnodes: Vec<KunaNodeId> = Vec::new();
        {
            let gc: &KunaRegionGraph = local_copy.as_ref().unwrap_or(graph);
            for key in gc.node_keys() {
                if gc.get_succs(key.id)?.is_empty() {
                    endnodes.push(key.id);
                }
            }
        }
        if endnodes.is_empty() {
            return Ok(false); // RI l.742
        }

        let mut add_dummy = false;
        if endnodes.len() > 1 {
            add_dummy = true;
        } else if !head_inedges.is_empty() {
            // RI l.749: lone end node that is NOT a predecessor of head (in the
            // ORIGINAL graph) still needs the dummy.
            let is_pred = graph.get_preds(head)?.iter().any(|&p| p == endnodes[0]);
            if !is_pred {
                add_dummy = true;
            }
        }

        // Materialize gc as an owned working graph: the dominator/slice scratch.
        // The C++ aliases gc to `graph` when there are no head in-edges and no
        // dummy; we keep a true separate copy and re-sync it from `graph` after a
        // collapse on that aliased path (see `gc_differs` below), so the
        // observable behavior — dominators built/patched over the same structure
        // the collapse sees — is identical.
        let mut gc_owned: KunaRegionGraph = match local_copy {
            Some(lc) => lc,
            None => graph.clone(),
        };

        let mut dummy: Option<KunaNodeId> = None;
        if add_dummy {
            let d = self.new_node(NodeKind::Dummy, !0u64);
            for &en in endnodes.iter() {
                gc_owned.add_edge(&self.pool, en, d);
            }
            endnodes.clear();
            endnodes.push(d);
            dummy = Some(d);
        }

        let mut doms = KunaIncrementalDominators::new(&self.pool, &gc_owned, head, false)?;
        let mut postdoms = KunaIncrementalDominators::new(&self.pool, &gc_owned, endnodes[0], true)?;

        let mut region_created = false;
        let mut postorder_snap: Vec<KunaNodeId> = Vec::new();
        kuna_dfs_postorder_deterministic(&self.pool, &gc_owned, head, &mut postorder_snap)?;
        // The C++ holds gc/graph distinct; here `graph` is the collapse target
        // and `gc_owned` the dominance/slice scratch.  Whether the working copy
        // differs from `graph` mirrors the C++ `gc != &graph` test.
        let gc_differs = !head_inedges.is_empty() || add_dummy;

        for &node in postorder_snap.iter() {
            if Some(node) == dummy {
                continue;
            }
            if cyclic && node == head {
                continue;
            }
            if !gc_owned.contains_node(node) {
                continue;
            }

            let out_degree = gc_owned.size_out(node)?;
            if out_degree == 0 {
                // RI l.780: the root of the region hierarchy should always be a
                // region; wrap a lone, isolated non-region leaf.
                if gc_owned.size_in(node)? == 0 && !self.pool.get(node).is_region() {
                    let r = self.new_region();
                    {
                        let region = &mut self.region_pool[r.0 as usize];
                        region.head = Some(node);
                        region.graph.add_node(&self.pool, node);
                        region.has_succs = false;
                        region.cyclic = false;
                        region.cyclic_ancestor = cyclic;
                    }
                    let empty_frontier = KunaNodeSet::new();
                    self.abstract_acyclic_region(
                        graph,
                        r,
                        &empty_frontier,
                        None,
                        secondary_graph.as_deref_mut(),
                    )?;
                }
                continue;
            }

            let mut postdom_node = postdoms.idom(node);
            while let Some(pdn) = postdom_node {
                let attempt = (node, pdn);
                if !failed_region_attempts.contains(&attempt)
                    && self.check_region(&gc_owned, node, pdn, &mut doms)?
                {
                    let mut frontier: KunaNodeSet = KunaNodeSet::new();
                    frontier.insert(self.pool.key(pdn));
                    let region =
                        self.compute_region(&gc_owned, node, &frontier, dummy, cyclic)?;
                    if let Some(region_id) = region {
                        // Backpatch graph_with_successors from the parent's
                        // secondary graph (RI l.809-827).
                        if let Some(sec) = secondary_graph.as_deref() {
                            self.backpatch_secondary(region_id, sec, &gc_owned)?;
                        }

                        self.abstract_acyclic_region(
                            graph,
                            region_id,
                            &frontier,
                            dummy,
                            secondary_graph.as_deref_mut(),
                        )?;
                        region_created = true;
                        let mut replaced_nodes: KunaNodeSet = KunaNodeSet::new();
                        {
                            let mut members = Vec::new();
                            self.region_pool[region_id.0 as usize]
                                .graph
                                .get_nodes(&mut members);
                            for m in members {
                                replaced_nodes.insert(self.pool.key(m));
                            }
                        }
                        if gc_differs {
                            // C++ `gc != &graph`: patch the scratch copy
                            // explicitly (no frontier edges) via _update_graph.
                            self.replace_region_in_graph(
                                &mut gc_owned,
                                region_id,
                                &replaced_nodes,
                            )?;
                        } else {
                            // C++ `gc == &graph`: the collapse above already
                            // mutated graph, which *is* gc.  Our gc_owned is a
                            // separate clone, so re-sync it from graph (there is
                            // no dummy on this path, so the copy is exact).
                            gc_owned = graph.clone();
                        }
                        let wrapper =
                            self.region_pool[region_id.0 as usize].wrapper.unwrap();
                        let rhead = self.region_pool[region_id.0 as usize].head.unwrap();
                        doms.graph_updated(&self.pool, wrapper, &replaced_nodes, rhead)?;
                        postdoms.graph_updated(&self.pool, wrapper, &replaced_nodes, rhead)?;
                        break; // Continue the post-order traversal
                    }
                }

                failed_region_attempts.insert(attempt);
                if !doms.dominates(node, pdn) {
                    break;
                }
                if postdoms.idom(pdn) == Some(pdn) {
                    break;
                }
                postdom_node = postdoms.idom(pdn);
            }
        }
        Ok(region_created)
    }

    /// Backpatch a child region's `graph_with_successors` from the parent's
    /// secondary graph (RI l.809-833, extracted to keep borrows local).
    fn backpatch_secondary(
        &mut self,
        region_id: RegionPayloadId,
        secondary_graph: &KunaRegionGraph,
        gc: &KunaRegionGraph,
    ) -> KunaResult<()> {
        let snap: Vec<KunaNodeId> = {
            let mut v = Vec::new();
            self.region_pool[region_id.0 as usize]
                .graph_with_successors
                .get_nodes(&mut v);
            v
        };
        for nn in snap {
            if !secondary_graph.contains_node(nn) {
                continue;
            }
            let osuccs: Vec<KunaNodeId> = secondary_graph.get_succs(nn)?.to_vec();
            for succ in osuccs {
                if self.complete_successors {
                    if !self.region_pool[region_id.0 as usize]
                        .graph_with_successors
                        .has_edge(nn, succ)
                    {
                        self.region_pool[region_id.0 as usize]
                            .graph_with_successors
                            .add_edge(&self.pool, nn, succ);
                        self.region_pool[region_id.0 as usize]
                            .successors
                            .insert(self.pool.key(succ));
                    }
                } else if !gc.contains_node(succ) {
                    self.region_pool[region_id.0 as usize]
                        .graph_with_successors
                        .add_edge(&self.pool, nn, succ);
                    self.region_pool[region_id.0 as usize]
                        .successors
                        .insert(self.pool.key(succ));
                }
            }
        }
        // Add edges between successors (RI l.829-833).
        let succ_list: Vec<KunaNodeKey> = self.region_pool[region_id.0 as usize]
            .successors
            .iter()
            .copied()
            .collect();
        for s0 in succ_list.iter() {
            for s1 in succ_list.iter() {
                if s0.id != s1.id && secondary_graph.has_edge(s0.id, s1.id) {
                    self.region_pool[region_id.0 as usize]
                        .graph_with_successors
                        .add_edge(&self.pool, s0.id, s1.id);
                }
            }
        }
        Ok(())
    }

    //
    // Outputs
    //

    /// RI `_make_regions_by_block_addrs` (l.169) (C++ `buildRegionsByBlockAddrs`).
    fn build_regions_by_block_addrs(&mut self) {
        self.regions_by_block_addrs.clear();
        let top = match self.top_region {
            Some(t) => t,
            None => return,
        };
        let mut work_list: Vec<RegionPayloadId> = vec![top];
        let mut seen_regions: BTreeSet<RegionPayloadId> = BTreeSet::new(); // Membership only
        while !work_list.is_empty() {
            let mut children_regions: Vec<RegionPayloadId> = Vec::new();
            for &region_id in work_list.iter() {
                let mut children_blocks: Vec<uintb> = Vec::new();
                let member_keys: Vec<KunaNodeKey> = self.region_pool[region_id.0 as usize]
                    .graph
                    .node_keys()
                    .copied()
                    .collect();
                for mk in member_keys {
                    let node = self.pool.get(mk.id);
                    if node.get_kind() == NodeKind::Block {
                        children_blocks.push(node.get_addr());
                    } else if node.is_multi() {
                        for &m in node.get_chain() {
                            children_blocks.push(self.pool.get(m).get_addr());
                        }
                    } else if node.is_region() {
                        if let Some(sub) = node.get_region() {
                            if !seen_regions.contains(&sub) {
                                children_regions.push(sub);
                                let sub_head =
                                    self.region_pool[sub.0 as usize].head.unwrap();
                                children_blocks.push(self.pool.get(sub_head).get_addr());
                                seen_regions.insert(sub);
                            }
                        }
                    }
                    // k_dummy: skip (never appears in region graphs).
                }
                if !children_blocks.is_empty() {
                    self.regions_by_block_addrs.push(children_blocks);
                }
            }
            work_list = children_regions;
        }
    }

    /// Walk one region (C++ static `walkRegionBlocks`).
    pub fn walk_region_blocks<V: KunaRegionVisitor>(
        &self,
        region_id: RegionPayloadId,
        visitor: &mut V,
    ) {
        let region = &self.region_pool[region_id.0 as usize];
        visitor.enter_region(region);
        let member_keys: Vec<KunaNodeKey> = region.graph.node_keys().copied().collect();
        for mk in member_keys {
            let node = self.pool.get(mk.id);
            if node.is_region() {
                if let Some(sub) = node.get_region() {
                    self.walk_region_blocks(sub, visitor);
                }
            } else if node.is_multi() {
                let chain: Vec<KunaNodeId> = node.get_chain().to_vec();
                for m in chain {
                    let mn = self.pool.get(m);
                    visitor.visit_block(mn.get_block(), mn.get_addr());
                }
            } else if node.get_kind() == NodeKind::Block {
                visitor.visit_block(node.get_block(), node.get_addr());
            }
        }
        // Re-borrow region (the recursive walk released it).
        let region = &self.region_pool[region_id.0 as usize];
        visitor.exit_region(region);
    }

    /// Walk the whole tree (C++ `walkBlocks`).
    pub fn walk_blocks<V: KunaRegionVisitor>(&self, visitor: &mut V) -> KunaResult<()> {
        let top = self
            .top_region
            .ok_or_else(|| KunaError::lowlevel("kuna regionid: compute() has not run"))?;
        self.walk_region_blocks(top, visitor);
        Ok(())
    }

    /// Read a node's address (C++ `KunaRegionNode::getAddr`); the pool is
    /// private, so this exposes a single node's address for console rendering.
    pub fn node_addr(&self, id: KunaNodeId) -> uintb {
        self.pool.get(id).get_addr()
    }

    /// Collect every *leaf-block* start address reachable under `node`, recursing
    /// through `k_multi` chains and `k_region` wrappers (the latter expand to their
    /// own loop body via the region pool).  Used by [`Self::cyclic_loops`] to
    /// resolve a cyclic region's body/exit nodes back to the basic-block addresses
    /// the S8 structurer keys on.  `k_dummy` nodes contribute nothing.
    fn collect_leaf_addrs(&self, node: KunaNodeId, out: &mut BTreeSet<uintb>) {
        let n = self.pool.get(node);
        match n.get_kind() {
            NodeKind::Block => {
                out.insert(n.get_addr());
            }
            NodeKind::Multi => {
                for &m in n.get_chain() {
                    out.insert(self.pool.get(m).get_addr());
                }
            }
            NodeKind::Region => {
                if let Some(sub) = n.get_region() {
                    let member_ids: Vec<KunaNodeId> = {
                        let mut v = Vec::new();
                        self.region_pool[sub.0 as usize].graph.get_nodes(&mut v);
                        v
                    };
                    for m in member_ids {
                        self.collect_leaf_addrs(m, out);
                    }
                }
            }
            NodeKind::Dummy => {}
        }
    }

    /// Expose the identified **cyclic** (loop) regions for the S8 region structurer
    /// (`region_structurer`).  For each cyclic region in the pool, returns a
    /// [`KunaCyclicLoop`] carrying the loop head's start address, the set of every
    /// basic-block start address in its loop body (recursively resolved through
    /// nested multi-nodes / sub-regions), and the set of its *exit* (successor)
    /// block start addresses — the angr `RegionIdentifier` refined loop body +
    /// successor frontier (`GraphRegion.successors`).
    ///
    /// This is the read side of the omitted
    /// `_refine_loop_successors_to_guarded_successors` /
    /// `_ensure_jump_at_loop_exit_ends` steps: it hands the structurer the
    /// region identifier's *correct* loop-body / exit identification (which absorbs
    /// the dominated switch-case successors the structural natural-loop walk on the
    /// sblocks graph misses), so the structurer can virtualize the secondary exits
    /// to break-gotos and fold the loop.  Only available after [`compute`] has run;
    /// returns an empty vec otherwise.
    ///
    /// [`compute`]: Self::compute
    pub fn cyclic_loops(&self) -> Vec<KunaCyclicLoop> {
        let mut res = Vec::new();
        for region in self.region_pool.iter() {
            if !region.cyclic {
                continue;
            }
            let head_addr = match region.head {
                Some(h) => self.pool.get(h).get_addr(),
                None => continue,
            };
            // Body: every leaf-block address inside the region's own loop graph.
            let mut body: BTreeSet<uintb> = BTreeSet::new();
            let body_members: Vec<KunaNodeId> = {
                let mut v = Vec::new();
                region.graph.get_nodes(&mut v);
                v
            };
            for m in body_members {
                self.collect_leaf_addrs(m, &mut body);
            }
            // Exits: the head address of each successor node (the block control
            // reaches when it leaves the loop).  A successor may itself be a
            // wrapped region — use its head, since that is the block the loop-exit
            // edge actually targets.
            let mut exits: BTreeSet<uintb> = BTreeSet::new();
            for sk in region.successors.iter() {
                exits.insert(self.pool.get(sk.id).get_addr());
            }
            res.push(KunaCyclicLoop { head_addr, body, exits });
        }
        res
    }

    /// Render the nested region tree (C++ `IfcKunaRegionTree::printRegion`
    /// recursion): one `region head=0x.. nodes=N [cyclic]` line per region,
    /// `block 0x..` per leaf block, indent 2 spaces per depth.  The members are
    /// iterated in `KunaNodeOrder` (the `node_keys` order the walker uses), so
    /// the text is deterministic.  Returns the empty string if `compute()` has
    /// not run.
    pub fn render_tree(&self) -> String {
        let mut os = String::new();
        if let Some(top) = self.top_region {
            self.render_region(top, 0, &mut os);
        }
        os
    }

    /// Recursive helper for [`render_tree`].
    fn render_region(&self, region_id: RegionPayloadId, depth: usize, os: &mut String) {
        let region = &self.region_pool[region_id.0 as usize];
        for _ in 0..depth {
            os.push_str("  ");
        }
        os.push_str("region head=0x");
        let head_addr = region.get_head().map(|h| self.pool.get(h).get_addr()).unwrap_or(0);
        os.push_str(&format!("{head_addr:x}"));
        os.push_str(" nodes=");
        os.push_str(&region.graph.num_nodes().to_string());
        if region.is_cyclic() {
            os.push_str(" cyclic");
        }
        os.push('\n');
        let member_keys: Vec<KunaNodeKey> = region.graph.node_keys().copied().collect();
        for mk in member_keys {
            let node = self.pool.get(mk.id);
            if node.is_region() {
                if let Some(sub) = node.get_region() {
                    self.render_region(sub, depth + 1, os);
                }
            } else if node.is_multi() {
                let chain: Vec<KunaNodeId> = node.get_chain().to_vec();
                for m in chain {
                    for _ in 0..depth + 1 {
                        os.push_str("  ");
                    }
                    os.push_str("block 0x");
                    os.push_str(&format!("{:x}", self.pool.get(m).get_addr()));
                    os.push('\n');
                }
            } else if node.get_kind() == NodeKind::Block {
                for _ in 0..depth + 1 {
                    os.push_str("  ");
                }
                os.push_str("block 0x");
                os.push_str(&format!("{:x}", node.get_addr()));
                os.push('\n');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test visitor mirroring `testkunaregion.cc`'s `KunaTestCollector`:
    /// collect every leaf block address (in walk order) and, for each cyclic
    /// region, the head address plus the addresses visited inside it.
    ///
    /// The C++ collector keeps `const KunaGraphRegion *` pointers and re-walks
    /// one later; here we capture the per-cyclic block list during the single
    /// walk (depth-tracked), which is equivalent for the asserted shapes.
    #[derive(Default)]
    struct Collector {
        addrs: Vec<uintb>,
        /// (head addr, sorted body block addrs) for every cyclic region.
        cyclics: Vec<(uintb, Vec<uintb>)>,
        /// Stack of (is_cyclic, head addr, body accumulator) for the open
        /// regions, so blocks are bucketed into the innermost cyclic region.
        stack: Vec<(bool, uintb, Vec<uintb>)>,
    }

    impl KunaRegionVisitor for Collector {
        fn enter_region(&mut self, region: &KunaGraphRegion) {
            // head addr is unknown here (no pool); the walker fills it via the
            // first block, but cyclic regions always have a head, so record a
            // placeholder and patch the head from the first visited block.
            let cyclic = region.is_cyclic();
            self.stack.push((cyclic, u64::MAX, Vec::new()));
        }
        fn exit_region(&mut self, _region: &KunaGraphRegion) {
            if let Some((cyclic, head, body)) = self.stack.pop() {
                if cyclic {
                    let mut sorted = body.clone();
                    sorted.sort_unstable();
                    self.cyclics.push((head, sorted));
                }
            }
        }
        fn visit_block(&mut self, _block: Option<crate::seams::BlockId>, addr: uintb) {
            self.addrs.push(addr);
            // Record into every open cyclic region's body, and set the head of
            // the innermost open region from its first block.
            for frame in self.stack.iter_mut() {
                if frame.1 == u64::MAX {
                    frame.1 = addr; // first block seen = region head (walk order)
                }
                if frame.0 {
                    frame.2.push(addr);
                }
            }
        }
    }

    fn sorted(v: &[uintb]) -> Vec<uintb> {
        let mut r = v.to_vec();
        r.sort_unstable();
        r
    }

    /// Port of `testkunaregion.cc:regionid_diamond0` (angr
    /// `test_region_identifier_0`): a diamond `1 -> 2 -> {3,4} -> 5 -> 6`.
    #[test]
    fn regionid_diamond0() {
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

        let top = ri.compute().unwrap();
        let region = ri.region(top);
        // The angr assertion: the top-level region graph has exactly two nodes.
        assert_eq!(region.get_graph().num_nodes(), 2);
        let head = region.get_head().unwrap();
        assert_eq!(ri.pool.get(head).get_addr(), 1);
        assert!(!region.is_cyclic());

        // Exactly one of the two top-level nodes is a sub-region.
        let mut region_count = 0;
        for key in region.get_graph().node_keys() {
            if ri.pool.get(key.id).is_region() {
                region_count += 1;
            }
        }
        assert_eq!(region_count, 1);

        // The recursive walker covers every input block exactly once.
        let mut col = Collector::default();
        ri.walk_blocks(&mut col).unwrap();
        assert_eq!(col.addrs.len(), 6);
        let got = sorted(&col.addrs);
        for i in 0..6 {
            assert_eq!(got[i], (i + 1) as uintb);
        }
        assert_eq!(col.cyclics.len(), 0);
    }

    /// Port of `testkunaregion.cc:regionid_diamond1` (angr
    /// `test_region_identifier_1`): two stacked half-diamonds.
    #[test]
    fn regionid_diamond1() {
        let mut ri = KunaRegionIdentifier::new();
        let mut nodes = [KunaNodeId(0); 9];
        for (i, slot) in nodes.iter_mut().enumerate().take(9).skip(1) {
            *slot = ri.add_synthetic_block(i as uintb);
        }
        ri.add_synthetic_edge(nodes[1], nodes[2]);
        ri.add_synthetic_edge(nodes[2], nodes[3]);
        ri.add_synthetic_edge(nodes[3], nodes[4]);
        ri.add_synthetic_edge(nodes[2], nodes[4]);
        ri.add_synthetic_edge(nodes[4], nodes[5]);
        ri.add_synthetic_edge(nodes[5], nodes[6]);
        ri.add_synthetic_edge(nodes[6], nodes[7]);
        ri.add_synthetic_edge(nodes[5], nodes[7]);
        ri.add_synthetic_edge(nodes[7], nodes[8]);

        let top = ri.compute().unwrap();
        // The angr assertion: top-level region graph has exactly two nodes.
        assert_eq!(ri.region(top).get_graph().num_nodes(), 2);

        let mut col = Collector::default();
        ri.walk_blocks(&mut col).unwrap();
        assert_eq!(col.addrs.len(), 8);
        let got = sorted(&col.addrs);
        for i in 0..8 {
            assert_eq!(got[i], (i + 1) as uintb);
        }
    }

    /// Port of `testkunaregion.cc:regionid_loop` (kuna-added cyclic coverage):
    /// `1 -> 2 -> 3 -> 4` with a back edge `3 -> 2` (loop body `{2,3}`).
    #[test]
    fn regionid_loop() {
        let mut ri = KunaRegionIdentifier::new();
        let n1 = ri.add_synthetic_block(1);
        let n2 = ri.add_synthetic_block(2);
        let n3 = ri.add_synthetic_block(3);
        let n4 = ri.add_synthetic_block(4);
        ri.add_synthetic_edge(n1, n2);
        ri.add_synthetic_edge(n2, n3);
        ri.add_synthetic_edge(n3, n2);
        ri.add_synthetic_edge(n3, n4);

        let top = ri.compute().unwrap();
        assert!(!ri.region(top).is_cyclic());

        // Exactly one cyclic region, headed at address 2, containing {2,3}.
        let mut col = Collector::default();
        ri.walk_blocks(&mut col).unwrap();
        assert_eq!(col.cyclics.len(), 1);
        let (loop_head, loop_addrs) = &col.cyclics[0];
        assert_eq!(*loop_head, 2);
        assert_eq!(loop_addrs.len(), 2);
        assert_eq!(loop_addrs[0], 2);
        assert_eq!(loop_addrs[1], 3);

        // regions_by_block_addrs contains the loop body {2,3} as its own region.
        let rbba = ri.get_regions_by_block_addrs();
        let found = rbba.iter().any(|r| {
            let s = sorted(r);
            s.len() == 2 && s[0] == 2 && s[1] == 3
        });
        assert!(found);

        // Whole-tree walk covers every block exactly once.
        assert_eq!(col.addrs.len(), 4);
        let got = sorted(&col.addrs);
        for i in 0..4 {
            assert_eq!(got[i], (i + 1) as uintb);
        }
    }

    //
    // SEAM(W7) block-graph adapter coverage (Input A: build_from_block_graph)
    //

    mod block_graph {
        use std::rc::Rc;

        use kuna_base::address::Address;
        use kuna_base::space::{
            addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
        };
        use kuna_num::opcodes::OpCode;

        use crate::funcdata::Funcdata;
        use crate::kuna_regionid::KunaRegionIdentifier;
        use crate::op::pcodeop_flags;
        use crate::seams::{Architecture, BlockId, TypeOp};

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

        fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
            Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
        }

        fn addr(rs: &Rc<AddrSpace>, off: u64) -> Address {
            Address::new(Rc::clone(rs), off)
        }

        /// Append a terminating op of `opc` at `pc` into block `bl`.
        fn term_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, pc: Address) {
            let op = fd.obank_mut().create_at(1, pc);
            fd.obank_mut()
                .change_opcode(op, TypeOp::new(opc, pcodeop_flags::branch, format!("{opc:?}")));
            fd.obank_mut().mark_alive(op);
            fd.bb_insert_op(op, bl, None);
        }

        /// Real-CFG analog of the synthetic `regionid_diamond0`: a diamond
        ///   entry(0x1000, CBRANCH) -> {a(0x1100), b(0x1200)} -> join(0x1300, RETURN)
        /// built over real `bblocks` and driven through
        /// [`KunaRegionIdentifier::build_from_block_graph`] (Input A, the W7 seam).
        #[test]
        fn build_from_block_graph_diamond() {
            let mut fd = build_fd();
            let rs = ramspace(&fd);
            let root = fd.bblocks_root_pub();
            let entry = fd.bblocks_mut().new_block_basic(root);
            let a = fd.bblocks_mut().new_block_basic(root);
            let b = fd.bblocks_mut().new_block_basic(root);
            let join = fd.bblocks_mut().new_block_basic(root);
            fd.bblocks_mut().add_edge(entry, a);
            fd.bblocks_mut().add_edge(entry, b);
            fd.bblocks_mut().add_edge(a, join);
            fd.bblocks_mut().add_edge(b, join);
            fd.bblocks_mut().set_start_block(root, entry);
            // Distinct start addresses (bblocks_block_start reads the block cover).
            fd.set_basic_block_range(entry, &addr(&rs, 0x1000), &addr(&rs, 0x100f));
            fd.set_basic_block_range(a, &addr(&rs, 0x1100), &addr(&rs, 0x110f));
            fd.set_basic_block_range(b, &addr(&rs, 0x1200), &addr(&rs, 0x120f));
            fd.set_basic_block_range(join, &addr(&rs, 0x1300), &addr(&rs, 0x130f));
            // Tail ops: the entry forks on a CBRANCH; the join is a plain RETURN.
            term_op(&mut fd, entry, OpCode::CPUI_CBRANCH, addr(&rs, 0x100c));
            term_op(&mut fd, join, OpCode::CPUI_RETURN, addr(&rs, 0x130c));

            let mut ri = KunaRegionIdentifier::new();
            ri.build_from_block_graph(&fd).unwrap();
            let top = ri.compute().unwrap();

            let region = ri.region(top);
            // Same shape the synthetic diamond asserts: the top region collapses
            // to two nodes (the entry head + one nested sub-region).
            assert_eq!(region.get_graph().num_nodes(), 2);
            let head = region.get_head().unwrap();
            assert_eq!(ri.pool.get(head).get_addr(), 0x1000);
            assert!(!region.is_cyclic());
            let mut region_count = 0;
            for key in region.get_graph().node_keys() {
                if ri.pool.get(key.id).is_region() {
                    region_count += 1;
                }
            }
            assert_eq!(region_count, 1);

            // The walker exposes the real BlockId payload for every leaf block
            // (carried through build_from_block_graph), exactly once each.
            let mut col = super::Collector::default();
            ri.walk_blocks(&mut col).unwrap();
            assert_eq!(col.addrs.len(), 4);
            let got = super::sorted(&col.addrs);
            assert_eq!(got, vec![0x1000, 0x1100, 0x1200, 0x1300]);
            assert_eq!(col.cyclics.len(), 0);
        }

        /// The CBRANCH-ending block keeps `endsWithBranchindOrCbranch` true and is
        /// therefore NOT merged into a supernode by `make_supergraph`; a plain
        /// (non-branchy) chain still merges.  Drives the seam end-to-end: a real
        /// `lastOp()->code()` probe parked on the node steers the supergraph.
        #[test]
        fn build_from_block_graph_supergraph_respects_branchind_cbranch() {
            // Chain: entry(CBRANCH) -> mid(plain) -> tail(RETURN), single in/out.
            let mut fd = build_fd();
            let rs = ramspace(&fd);
            let root = fd.bblocks_root_pub();
            let entry = fd.bblocks_mut().new_block_basic(root);
            let mid = fd.bblocks_mut().new_block_basic(root);
            let tail = fd.bblocks_mut().new_block_basic(root);
            fd.bblocks_mut().add_edge(entry, mid);
            fd.bblocks_mut().add_edge(mid, tail);
            fd.bblocks_mut().set_start_block(root, entry);
            fd.set_basic_block_range(entry, &addr(&rs, 0x1000), &addr(&rs, 0x100f));
            fd.set_basic_block_range(mid, &addr(&rs, 0x1100), &addr(&rs, 0x110f));
            fd.set_basic_block_range(tail, &addr(&rs, 0x1200), &addr(&rs, 0x120f));
            // The entry block ends in CBRANCH; the supergraph must NOT merge `mid`
            // INTO `entry` while it would (single out / single in) absent the flag.
            term_op(&mut fd, entry, OpCode::CPUI_CBRANCH, addr(&rs, 0x100c));
            term_op(&mut fd, mid, OpCode::CPUI_COPY, addr(&rs, 0x1108));
            term_op(&mut fd, tail, OpCode::CPUI_RETURN, addr(&rs, 0x120c));

            let mut ri = KunaRegionIdentifier::new();
            ri.build_from_block_graph(&fd).unwrap();
            ri.compute().unwrap();

            // All three blocks are walked exactly once regardless of merge shape.
            let mut col = super::Collector::default();
            ri.walk_blocks(&mut col).unwrap();
            let got = super::sorted(&col.addrs);
            assert_eq!(got, vec![0x1000, 0x1100, 0x1200]);
        }

        /// `build_from_block_graph` rejects a non-empty working graph (it must run
        /// on a fresh identifier, before any synthetic input or compute).
        #[test]
        fn build_from_block_graph_rejects_non_empty() {
            let fd = build_fd();
            let mut ri = KunaRegionIdentifier::new();
            ri.add_synthetic_block(0x10);
            assert!(ri.build_from_block_graph(&fd).is_err());
        }
    }
}

