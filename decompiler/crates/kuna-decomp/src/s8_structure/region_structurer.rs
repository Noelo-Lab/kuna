//! S8 region-based (Phoenix/SAILR) structurer — Increment 1 PoC (port of angr's
//! `analyses/decompiler/structuring/{recursive_structurer,phoenix,sailr}.py`).
//!
//! # What this is
//!
//! kuna's default structuring engine is Ghidra's
//! [`CollapseStructure`](crate::blockaction::CollapseStructure) (the verbatim
//! `CollapseStructure`/`TraceDAG` port), which emits an unstructured `goto` for
//! every control-flow edge it cannot fold with one of its schemas.  angr's
//! Phoenix/SAILR structurer recovers far more structure (sequences, if/else,
//! loops, switches) and emits dramatically fewer gotos.  This module is the
//! **foundation** of porting that structurer to kuna, wired behind the master
//! `--option regionstructure` (default **OFF**, parity-safe).
//!
//! # Increment 1 scope (this file)
//!
//! A correctness-over-completeness PoC that proves the whole seam end-to-end:
//!
//! 1. **Region post-order walk + replace** (angr `recursive_structurer._analyze`):
//!    the [`KunaRegionIdentifier`](crate::s7_regions::kuna_regionid) (the ported
//!    angr `RegionIdentifier`) runs over the real CFG via
//!    [`build_from_block_graph`](crate::s7_regions::kuna_regionid::KunaRegionIdentifier::build_from_block_graph),
//!    proving the W7 adapter on a live function.  The structuring itself then runs
//!    on the seeded `sblocks` graph (a `BlockCopy` mirror of `bblocks`, same
//!    topology) with the region tree as guidance.
//! 2. **Acyclic sequence schema** (angr `phoenix._match_acyclic_sequence`):
//!    collapse a chain of single-predecessor/single-successor blocks into one
//!    [`BlockList`](crate::block::BlockKind::Ls) via
//!    [`BlockGraph::new_block_list`](crate::block::BlockGraph::new_block_list).
//! 3. **Edge virtualization fallback** (angr `phoenix._virtualize_edge` +
//!    `sailr._order_virtualizable_edges`): when no schema matches and more than one
//!    component remains, mark the "best" remaining edge as a `goto` (SAILR H1
//!    sibling-count, H3 return-edge heuristics) and wrap it into a
//!    [`BlockGoto`](crate::block::BlockKind::Goto) /
//!    [`BlockIf`](crate::block::BlockKind::If)-goto / `BlockMultiGoto`, exactly the
//!    flags Ghidra's `set_goto_branch` sets so the renderer
//!    ([`s9_emit::printc`](crate::printc)) and `ActionFinalStructure`'s
//!    `mark_unstructured` are happy.
//!
//! Loops, if/else (ITE), switches and short-circuit folding are **later
//! increments**; in Inc 1 they fall back to virtualized gotos (honest-partial,
//! never a panic).  Because the virtualize fallback always removes one edge, the
//! loop always converges to a single structured root — there is no "stuck" state.
//!
//! # Honest-partial safety
//!
//! The structurer never aborts the run.  If, against expectation, it cannot reduce
//! the graph to a single node (e.g. an un-virtualizable irreducible knot), the
//! caller ([`ActionBlockStructure`](crate::blockaction::ActionBlockStructure))
//! detects the non-collapsed graph and the structurer reports failure so the
//! caller can re-seed and fall back to `CollapseStructure`.  Default-OFF the whole
//! module is dead code and output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};

use crate::block::BlockGraph;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::seams::BlockId;

use kuna_base::marshal::ElementId;

/// Marshaling element `<regionstructure>` (kuna).  ElementIds live in the 4000+
/// range (the next free id after `tailcalljump` = 4101).
pub const ELEM_REGIONSTRUCTURE: ElementId = ElementId::new("regionstructure", 4102);

/// Guard cap on structuring rounds: `2*n^2 + 64`, computed in 64-bit (mirrors the
/// [`kuna_guard_cap`](crate::s7_regions::kuna_regionid) caps).  The virtualize
/// fallback removes one edge per round so the real bound is `O(edges)`; the cap is
/// only a hang-guard turning a mis-port into a clean failure instead of a loop.
fn round_cap(num_nodes: int4) -> i64 {
    let n = num_nodes as i64;
    2 * n * n + 64
}

/// The `--option regionstructure on|off` parser (kuna control surface).
///
/// Mirrors [`OptionGotoReduce`](crate::s8_structure::kuna_gotoreduce::OptionGotoReduce):
/// parse `on`/`off`, returning the bool + a confirmation message.  The flag is
/// stored as `Architecture::region_structure` (and copied to the seam in
/// `build_arch_handle`).
pub struct OptionRegionStructure;

impl OptionRegionStructure {
    /// The option name.
    pub const NAME: &'static str = "regionstructure";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Region-based (Phoenix/SAILR) structurer turned {prop}")))
    }
}

/// Run the region-based structurer over a function's seeded `sblocks` graph.
///
/// Pre-conditions (established by [`ActionBlockStructure::apply`](crate::blockaction::ActionBlockStructure)):
/// `data.install_switch_defaults()` and `data.seed_sblocks_copy()` have run, so
/// `sblocks` is a `BlockCopy` mirror of `bblocks` with the same edges.
///
/// 1. Runs the [`KunaRegionIdentifier`](crate::s7_regions::kuna_regionid) over the
///    real CFG (proves the W7 adapter; its node order guides the virtualize
///    fallback).
/// 2. Collapses the `sblocks` graph to a single structured root using the acyclic
///    sequence schema + the SAILR-ordered virtualize-to-goto fallback.
///
/// Returns `Ok(true)` if the graph collapsed to a single structured root (the
/// caller proceeds to `ActionFinalStructure`), `Ok(false)` if it could not (the
/// caller re-seeds and falls back to `CollapseStructure`).  Never panics.
pub fn run_region_structurer(data: &mut Funcdata) -> KunaResult<bool> {
    // ---- 1. Region identification over the real CFG (W7 adapter proof) --------
    // recursive_structurer._analyze runs the RegionIdentifier first; here it both
    // proves build_from_block_graph().compute() on a live function and supplies a
    // deterministic node order (by block start address) the virtualize fallback
    // uses for tie-breaking.  A failure to identify regions is non-fatal: the
    // structurer still runs on the sblocks topology (honest-partial).
    let mut ri = crate::s7_regions::kuna_regionid::KunaRegionIdentifier::new();
    let _ = ri.build_from_block_graph(data).and_then(|()| ri.compute().map(|_| ()));

    // ---- 1b. Precompute the switch/jump-table maps over bblocks (Inc 4) -------
    // The switch-case schema (`match_acyclic_switch_cases`) needs the same two maps
    // `ActionBlockStructure::apply` builds for `CollapseStructure`: `switch_blocks`
    // (the bblocks switch BlockBasic → `Funcdata::jumpvec` slot, C++ `BlockSwitch(ind)`'s
    // `ind->getJumptable()`) and `switch_case_edges` (the per-switch case-edge topology
    // `(switch_bb, target_bb) → (outindex, isdefault)`, C++ `BlockSwitch::addCase`'s
    // `getInIndex`/`isDefaultBranch`).  Built here against `data` (whose switch
    // in/out edges structuring never severs) before the `sblocks_mut` borrow.
    let (switch_blocks, switch_case_edges) = compute_switch_maps(data);

    // ---- 2. Structure the seeded sblocks graph -------------------------------
    let sroot = data.sblocks_root();
    let graph = data.sblocks_mut();
    let mut st = RegionStructurer::new(graph, sroot)
        .with_switch_maps(switch_blocks, switch_case_edges);
    st.structure()
}

/// Precompute the bblocks switch-block → jumpvec-slot map and the per-switch
/// case-edge topology map over `data` (Inc 4).
///
/// A mirror of the precomputation in
/// [`ActionBlockStructure::apply`](crate::blockaction::ActionBlockStructure) so the
/// region structurer's switch schema feeds [`BlockGraph::new_block_switch`] exactly
/// the maps Ghidra's `CollapseStructure::ruleBlockSwitch` does.  A bblocks
/// `BlockBasic` is a switch block when it `is_switch_out` and its tail BRANCHIND
/// has a recovered `JumpTable` (`find_jump_table_index`); the case-edge map records
/// each switch's first out-edge to every target as `(outindex, isdefault)` (C++
/// `getInIndex` returns the first matching in-edge).
#[allow(clippy::type_complexity)]
fn compute_switch_maps(
    data: &Funcdata,
) -> (
    std::collections::BTreeMap<BlockId, usize>,
    std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
) {
    let mut switch_blocks: std::collections::BTreeMap<BlockId, usize> =
        std::collections::BTreeMap::new();
    let nbb = data.bblocks_get_size();
    for i in 0..nbb {
        let bb = data.bblocks_get_block(i);
        if data.bblocks_ref().block(bb).is_switch_out() {
            if let Some(indop) = data.bb_op_tail(bb) {
                if let Some(jt_idx) = data.find_jump_table_index(indop) {
                    switch_blocks.insert(bb, jt_idx);
                }
            }
        }
    }
    let mut switch_case_edges: std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)> =
        std::collections::BTreeMap::new();
    for &sbb in switch_blocks.keys() {
        let nout = data.bblocks_ref().block(sbb).size_out();
        for j in 0..nout {
            let target = data.bblocks_ref().block(sbb).get_out(j);
            let isdef = data.bblocks_ref().block(sbb).is_default_branch(j);
            switch_case_edges.entry((sbb, target)).or_insert((j, isdef));
        }
    }
    (switch_blocks, switch_case_edges)
}

/// The acyclic sequence + virtualize structuring engine, operating on the
/// seeded `sblocks` [`BlockGraph`] (the `BlockCopy` mirror of `bblocks`).
struct RegionStructurer<'a> {
    /// The structuring graph (the `sblocks` root graph) — same surface
    /// `CollapseStructure` drives.
    graph: &'a mut BlockGraph,
    /// The root BlockGraph node id (its `list` holds the live components).
    graph_id: BlockId,
    /// The bblocks switch BlockBasic → `Funcdata::jumpvec` slot map (Inc 4), keyed
    /// by the bblocks id each `sblocks` `BlockCopy` exit-leaf's `copy` references
    /// (C++ `BlockSwitch(ind)`'s `ind->getJumptable()`).  Empty ⇒ no switch schema.
    switch_blocks: std::collections::BTreeMap<BlockId, usize>,
    /// The per-switch case-edge topology `(switch_bb, target_bb) → (outindex,
    /// isdefault)` over bblocks (Inc 4), fed to [`BlockGraph::new_block_switch`]
    /// (C++ `BlockSwitch::addCase`).
    switch_case_edges: std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
}

impl<'a> RegionStructurer<'a> {
    fn new(graph: &'a mut BlockGraph, graph_id: BlockId) -> RegionStructurer<'a> {
        RegionStructurer {
            graph,
            graph_id,
            switch_blocks: std::collections::BTreeMap::new(),
            switch_case_edges: std::collections::BTreeMap::new(),
        }
    }

    /// Attach the precomputed switch/jump-table maps (Inc 4 switch schema).
    fn with_switch_maps(
        mut self,
        switch_blocks: std::collections::BTreeMap<BlockId, usize>,
        switch_case_edges: std::collections::BTreeMap<(BlockId, BlockId), (int4, bool)>,
    ) -> Self {
        self.switch_blocks = switch_blocks;
        self.switch_case_edges = switch_case_edges;
        self
    }

    /// Number of live top-level components in the structuring graph
    /// (C++ `BlockGraph::getSize`).
    fn size(&self) -> int4 {
        self.graph.block(self.graph_id).get_size()
    }

    /// The i-th live top-level component.
    fn component(&self, i: int4) -> BlockId {
        self.graph.block(self.graph_id).get_block(i)
    }

    /// Drive the structuring to a single root.  Returns `Ok(true)` on success
    /// (one component remains), `Ok(false)` if it could not converge.
    fn structure(&mut self) -> KunaResult<bool> {
        // angr phoenix._analyze: iterate until one node remains.
        self.graph.clear_visit_count(self.graph_id);

        let cap = round_cap(self.size());
        let mut rounds: i64 = 0;
        while self.size() > 1 {
            rounds += 1;
            if rounds > cap {
                // Hang-guard: report non-convergence so the caller falls back.
                return Ok(false);
            }

            // (a0) acyclic switch-case schema (phoenix._match_acyclic_switch_cases*,
            //      Inc 4).  Must run before the sequence/ITE schemas (which already
            //      refuse `is_switch_out` nodes), mirroring Ghidra's pass order where
            //      a nested switch resolves before its surrounding structure.  Emits
            //      a `BlockSwitch` carrying the `CaseOrder` descriptors
            //      `ActionFinalStructure::finalize_switch_printing` expects, so the
            //      switch-finalization/rendering path is unchanged.
            if self.match_acyclic_switch_cases()? {
                continue;
            }
            // (a) acyclic sequence schema (phoenix._match_acyclic_sequence).
            if self.match_acyclic_sequence()? {
                continue;
            }
            // (a2) structural ITE schema (phoenix._match_acyclic_ite): fold a
            //      2-out condition whose true/false clauses reconverge into a
            //      BlockIf.  kuna's structural form (the condition lives on the
            //      block's terminal CBRANCH) needs no claripy — only the
            //      true-edge orientation Ghidra's `negateCondition` would supply,
            //      so the false-clause-only if-then case is left to virtualize
            //      (honest-partial; the full negate path is a later increment).
            if self.match_acyclic_ite()? {
                continue;
            }
            // (b) wrap any already-marked goto edges (phoenix: BlockGoto/IfGoto).
            if self.rule_block_goto()? {
                continue;
            }
            // (c) last-resort edge virtualization (phoenix._last_resort_refinement
            //     + sailr._order_virtualizable_edges).  Always removes one edge.
            if self.virtualize_one_edge()? {
                continue;
            }
            // Nothing matched and >1 node remains and no edge to virtualize:
            // an un-virtualizable knot.  Report failure (caller falls back).
            return Ok(false);
        }
        Ok(self.size() == 1)
    }

    // -----------------------------------------------------------------------
    // (a) acyclic sequence schema — phoenix._match_acyclic_sequence
    // -----------------------------------------------------------------------

    /// Find a chain of single-pred/single-succ components and collapse it into a
    /// [`BlockList`](crate::block::BlockKind::Ls).
    ///
    /// Port of angr `phoenix._match_acyclic_sequence`: a node with a single
    /// successor `end`, where `end` has a single predecessor and the edge is not a
    /// back-edge, merges into a sequence.  The chain-extension mirrors Ghidra's
    /// `ruleBlockCat` (extend while the tail stays single-in/single-out).
    fn match_acyclic_sequence(&mut self) -> KunaResult<bool> {
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            if let Some(nodes) = self.sequence_chain_from(bl) {
                let graph_id = self.graph_id;
                self.graph.new_block_list(graph_id, &nodes)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// If `bl` starts a foldable single-in/single-out sequence chain, return the
    /// chain (≥2 blocks) to concatenate; else `None`.  Mirrors `ruleBlockCat`'s
    /// start-of-chain + extension guards (which encode angr's `out_degree==1 &&
    /// in_degree(end)==1 && not back-edge` test over the working graph).
    fn sequence_chain_from(&self, bl: BlockId) -> Option<Vec<BlockId>> {
        let b = self.graph.block(bl);
        if b.size_out() != 1 || b.is_switch_out() {
            return None;
        }
        // Must be the START of the chain (angr only merges at the head of a run).
        if b.size_in() == 1 && self.graph.block(b.get_in(0)).size_out() == 1 {
            return None;
        }
        let out0 = b.get_out(0);
        if out0 == bl {
            return None; // self-loop / back edge
        }
        if self.graph.block(out0).size_in() != 1 {
            return None; // `end` must have a single predecessor (angr in_degree==1)
        }
        if !b.is_decision_out(0) {
            return None; // not a structured (non-goto, non-back) edge
        }
        if self.graph.block(out0).is_switch_out() {
            return None; // switch resolves first
        }

        let mut nodes = vec![bl, out0];
        let mut outblock = out0;
        // Extend the chain greedily (ruleBlockCat extension).
        loop {
            let ob = self.graph.block(outblock);
            if ob.size_out() != 1 {
                break;
            }
            let nxt = ob.get_out(0);
            if nxt == bl {
                break; // no looping back to head
            }
            if self.graph.block(nxt).size_in() != 1 {
                break;
            }
            if !ob.is_decision_out(0) {
                break;
            }
            if self.graph.block(nxt).is_switch_out() {
                break;
            }
            outblock = nxt;
            nodes.push(outblock);
        }
        Some(nodes)
    }

    // -----------------------------------------------------------------------
    // (a2) structural ITE schema — phoenix._match_acyclic_ite (kuna structural
    //      form: the condition is the block's terminal CBRANCH; no claripy)
    // -----------------------------------------------------------------------

    /// Fold a 2-out condition node whose true/false clauses reconverge into a
    /// [`BlockIf`](crate::block::BlockKind::If).
    ///
    /// Mirrors the structural detection of Ghidra `CollapseStructure::ruleBlockIfElse`
    /// (the full if/else, both clauses present) and `ruleBlockIf` (if-then, one
    /// clause).  This is the kuna analog of angr `phoenix._match_acyclic_ite`,
    /// realized over kuna's `BlockIf` builders — the edge condition lives on the
    /// block's terminal CBRANCH so no claripy/condition-processor is needed.  The
    /// if-then case is only taken when the clause is on the **true** edge (the
    /// false-clause if-then needs Ghidra's `negateCondition` data-flow flip — a
    /// later increment); otherwise the node is left to virtualize (honest-partial).
    fn match_acyclic_ite(&mut self) -> KunaResult<bool> {
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            if self.try_if_else(bl)? {
                return Ok(true);
            }
            if self.try_if_then_true_clause(bl)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// If/else: a 2-out condition whose true and false clauses each have a single
    /// in-edge and a single out-edge, both exiting to the same `outblock`, with no
    /// goto/switch on any edge.  Mirrors Ghidra `ruleBlockIfElse`.  No condition
    /// negation needed — the true/false clauses go to `new_block_if_else` in
    /// edge-true/edge-false order, exactly as Ghidra passes them.
    fn try_if_else(&mut self, bl: BlockId) -> KunaResult<bool> {
        let b = self.graph.block(bl);
        if b.size_out() != 2 || b.is_switch_out() {
            return Ok(false);
        }
        if !b.is_decision_out(0) || !b.is_decision_out(1) {
            return Ok(false);
        }
        let tc = b.get_true_out();
        let fc = b.get_false_out();
        if tc == bl || fc == bl {
            return Ok(false); // no loops
        }
        let tcb = self.graph.block(tc);
        let fcb = self.graph.block(fc);
        if tcb.size_in() != 1 || fcb.size_in() != 1 {
            return Ok(false); // nothing else may hit a clause
        }
        if tcb.size_out() != 1 || fcb.size_out() != 1 {
            return Ok(false); // single exit from each clause
        }
        if tcb.is_switch_out() || fcb.is_switch_out() {
            return Ok(false);
        }
        if tcb.is_goto_out(0) || fcb.is_goto_out(0) {
            return Ok(false); // clauses must exit structurally
        }
        let out_t = tcb.get_out(0);
        let out_f = fcb.get_out(0);
        if out_t == bl || out_t != out_f {
            return Ok(false); // clauses must reconverge to the same block
        }
        let graph_id = self.graph_id;
        self.graph.new_block_if_else(graph_id, bl, tc, fc);
        Ok(true)
    }

    /// If-then (true-clause only): a 2-out condition where the **true** clause has
    /// a single in/out edge and exits to the **false** successor (so the false edge
    /// is the after-if path).  Mirrors Ghidra `ruleBlockIf`'s `i == 1` (true-edge)
    /// arm, which needs no `negateCondition`.  The false-clause arm (`i == 0`,
    /// needing the data-flow condition flip) is deferred to a later increment.
    fn try_if_then_true_clause(&mut self, bl: BlockId) -> KunaResult<bool> {
        let b = self.graph.block(bl);
        if b.size_out() != 2 || b.is_switch_out() {
            return Ok(false);
        }
        if b.get_out(0) == bl || b.get_out(1) == bl {
            return Ok(false); // no loops
        }
        if b.is_goto_out(0) || b.is_goto_out(1) || !b.is_decision_out(1) {
            return Ok(false);
        }
        // Only the true-edge clause (i == 1): the clause is the true successor,
        // and after the clause control reaches the false successor.
        let clause = b.get_true_out();
        let after = b.get_false_out();
        let cb = self.graph.block(clause);
        if cb.size_in() != 1 || cb.size_out() != 1 {
            return Ok(false);
        }
        if cb.is_switch_out() || cb.is_goto_out(0) {
            return Ok(false);
        }
        if cb.get_out(0) != after {
            return Ok(false); // path after the clause must be the other branch
        }
        let graph_id = self.graph_id;
        self.graph.new_block_if(graph_id, bl, clause);
        Ok(true)
    }

    // -----------------------------------------------------------------------
    // (a0) acyclic switch-case schema — phoenix._match_acyclic_switch_cases*
    //      (Inc 4).  kuna structural form: the switch is a node flagged
    //      `f_switch_out` whose out-edges (case + default + exit) carry the jump
    //      table's edge labels (built by `seed_sblocks_copy`); the recovered
    //      `JumpTable` lives in `Funcdata::jumpvec`, resolved via the precomputed
    //      `switch_blocks`/`switch_case_edges` maps.  No claripy / AIL
    //      condition-processor is needed — the case topology *is* the edge set.
    //      This is the kuna analog of angr
    //      `phoenix._match_acyclic_switch_cases_address_computed` →
    //      `_switch_build_cases` → `_make_switch_cases_core`, realized over kuna's
    //      `BlockSwitch` builder (the same path Ghidra's `ruleBlockSwitch` drives).
    // -----------------------------------------------------------------------

    /// Find a structured switch region and fold it into a
    /// [`BlockSwitch`](crate::block::BlockKind::Switch).
    ///
    /// Port of angr `phoenix._match_acyclic_switch_cases` (the
    /// `*_address_computed` resolved-jump-table arm kuna already has the data for).
    /// The topological match mirrors Ghidra `CollapseStructure::ruleBlockSwitch`:
    /// a `f_switch_out` head whose cases each have a single in-edge from the head
    /// and at most one out-edge to a common exit block.  Returns `Ok(true)` once a
    /// switch is folded (or a skip-edge is virtualized to a goto, which makes
    /// progress); honest-partial: a switch we cannot match falls through to the
    /// virtualize fallback (never a panic, never a whole-function abort).
    fn match_acyclic_switch_cases(&mut self) -> KunaResult<bool> {
        if self.switch_blocks.is_empty() {
            return Ok(false); // no recovered jump table ⇒ no switch schema
        }
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            if self.try_switch_cases(bl)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Attempt to fold the switch rooted at `bl` (port of Ghidra
    /// `CollapseStructure::ruleBlockSwitch`, `blockaction.cc:1649`, realized in the
    /// region structurer).  Mirrors the exact case/exit/skip topology checks; on a
    /// clean match emits `BlockSwitch` via [`BlockGraph::new_block_switch`].
    fn try_switch_cases(&mut self, bl: BlockId) -> KunaResult<bool> {
        if !self.graph.block(bl).is_switch_out() {
            return Ok(false);
        }
        // Only proceed if this switch has a recovered jump table (`getJumptable()`
        // ≠ 0); else there is no case-label source and `new_block_switch` would be
        // meaningless — leave it to virtualize (honest partial).
        let jt_index = match self.switch_jt_index(bl) {
            Some(j) => j,
            None => return Ok(false),
        };

        let sizeout = self.graph.block(bl).size_out();
        let mut exitblock: Option<BlockId> = None;

        // --- Find the "obvious" exit block (ruleBlockSwitch first loop) ---------
        for i in 0..sizeout {
            let curbl = self.graph.block(bl).get_out(i);
            if curbl == bl {
                exitblock = Some(curbl); // exit back to top of switch (loop)
                break;
            }
            if self.graph.block(curbl).size_out() > 1 {
                exitblock = Some(curbl);
                break;
            }
            if self.graph.block(curbl).size_in() > 1 {
                exitblock = Some(curbl);
                break;
            }
        }

        if exitblock.is_none() {
            // Every immediate block has sizeIn==1 and sizeOut<=1.
            for i in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(i);
                if self.graph.block(curbl).is_goto_in(0) {
                    return Ok(false); // in cannot be a goto
                }
                if self.graph.block(curbl).is_switch_out() {
                    return Ok(false); // resolve nested switch first
                }
                if self.graph.block(curbl).size_out() == 1 {
                    if self.graph.block(curbl).is_goto_out(0) {
                        return Ok(false); // out cannot be goto
                    }
                    let curout = self.graph.block(curbl).get_out(0);
                    if let Some(e) = exitblock {
                        if e != curout {
                            return Ok(false);
                        }
                    } else {
                        exitblock = Some(curout);
                    }
                }
            }
        } else if let Some(e) = exitblock {
            // A determined exit block: no in/out gotos on it, and every case must
            // fall through only to it.
            for i in 0..self.graph.block(e).size_in() {
                if self.graph.block(e).is_goto_in(i) {
                    return Ok(false);
                }
            }
            for i in 0..self.graph.block(e).size_out() {
                if self.graph.block(e).is_goto_out(i) {
                    return Ok(false);
                }
            }
            for i in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(i);
                if curbl == e {
                    continue; // switch can go straight to the exit
                }
                if self.graph.block(curbl).size_in() > 1 {
                    return Ok(false); // only the switch may fall into a case
                }
                if self.graph.block(curbl).is_goto_in(0) {
                    return Ok(false);
                }
                if self.graph.block(curbl).size_out() > 1 {
                    return Ok(false); // at most one exit from a case
                }
                if self.graph.block(curbl).size_out() == 1 {
                    if self.graph.block(curbl).is_goto_out(0) {
                        return Ok(false);
                    }
                    if self.graph.block(curbl).get_out(0) != e {
                        return Ok(false); // which must be the exit block
                    }
                }
                if self.graph.block(curbl).is_switch_out() {
                    return Ok(false); // nested switch first
                }
            }
        }

        // --- Skip-to-exit handling (checkSwitchSkips) ---------------------------
        // If a non-default case edge goes straight to the exit while the default
        // does not, virtualize those skip edges to gotos (progress) and bail this
        // round; the next round re-matches.  Returns Ok(true) (progress made).
        if !self.check_switch_skips(bl, exitblock)? {
            return Ok(true);
        }

        // --- Collect the case components (ruleBlockSwitch tail) ----------------
        let mut cases: Vec<BlockId> = vec![bl];
        for i in 0..sizeout {
            let curbl = self.graph.block(bl).get_out(i);
            if Some(curbl) == exitblock {
                continue; // the exit is not a case
            }
            cases.push(curbl);
        }

        let graph_id = self.graph_id;
        let switch_case_edges = std::mem::take(&mut self.switch_case_edges);
        let res = self.graph.new_block_switch(
            graph_id,
            &cases,
            exitblock.is_some(),
            jt_index,
            &switch_case_edges,
        );
        self.switch_case_edges = switch_case_edges;
        // new_block_switch can fail if the exit-leaf is not a BlockCopy (should not
        // happen for a real switch head, but keep it honest-partial: report no
        // match so the fallback virtualizes rather than aborting the run).
        match res {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Resolve this switch head's `Funcdata::jumpvec` slot via its exit-leaf
    /// `BlockCopy`'s `copy` (the bblocks BlockBasic carrying the BRANCHIND) against
    /// the precomputed `switch_blocks` map (C++ `BlockSwitch(ind)`'s
    /// `ind->getJumptable()`).  `None` ⇒ no recovered table for this head.
    fn switch_jt_index(&self, bl: BlockId) -> Option<usize> {
        let leaf = self.graph.get_exit_leaf(bl)?;
        let bb = self.graph.block(leaf).get_copy()?;
        self.switch_blocks.get(&bb).copied()
    }

    /// Convert any non-default switch edge that skips straight to the exit into a
    /// goto (port of Ghidra `CollapseStructure::checkSwitchSkips`,
    /// `blockaction.cc:1607`).  Returns `Ok(false)` (and marks the gotos) when such
    /// skip edges exist alongside a default that does not go to the exit — the
    /// switch then re-matches next round; otherwise `Ok(true)` (clean to fold).
    fn check_switch_skips(
        &mut self,
        switchbl: BlockId,
        exitblock: Option<BlockId>,
    ) -> KunaResult<bool> {
        let exitblock = match exitblock {
            Some(e) => e,
            None => return Ok(true),
        };
        let sizeout = self.graph.block(switchbl).size_out();
        let mut defaultnottoexit = false;
        let mut anyskiptoexit = false;
        for edgenum in 0..sizeout {
            if self.graph.block(switchbl).get_out(edgenum) == exitblock {
                if !self.graph.block(switchbl).is_default_branch(edgenum) {
                    anyskiptoexit = true;
                }
            } else if self.graph.block(switchbl).is_default_branch(edgenum) {
                defaultnottoexit = true;
            }
        }
        if !anyskiptoexit {
            return Ok(true);
        }
        if !defaultnottoexit
            && self.graph.block(switchbl).get_type() == crate::block::BlockType::MultiGoto
        {
            if let crate::block::BlockKind::MultiGoto { defaultswitch, .. } =
                self.graph.block(switchbl).kind()
            {
                if *defaultswitch {
                    defaultnottoexit = true;
                }
            }
        }
        if !defaultnottoexit {
            return Ok(true);
        }
        for edgenum in 0..sizeout {
            if self.graph.block(switchbl).get_out(edgenum) == exitblock
                && !self.graph.block(switchbl).is_default_branch(edgenum)
            {
                self.graph.set_goto_branch(switchbl, edgenum)?;
            }
        }
        Ok(false)
    }

    // -----------------------------------------------------------------------
    // (b) wrap an already-goto-marked edge — phoenix BlockGoto path
    // -----------------------------------------------------------------------

    /// If any component has an out-edge already flagged `f_goto_edge` (by a prior
    /// virtualization round), wrap it into the matching goto block.  Mirrors
    /// Ghidra `CollapseStructure::ruleBlockGoto`.
    fn rule_block_goto(&mut self) -> KunaResult<bool> {
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            let sizeout = self.graph.block(bl).size_out();
            for e in 0..sizeout {
                if self.graph.block(bl).is_goto_out(e) {
                    let graph_id = self.graph_id;
                    if self.graph.block(bl).is_switch_out() {
                        self.graph.new_block_multi_goto(graph_id, bl, e);
                        return Ok(true);
                    }
                    if sizeout == 2 {
                        // The TRUE (out-edge 1) branch must be the goto for
                        // new_block_if_goto; if the goto is the false branch,
                        // virtualize as a single goto would have flipped it.  For
                        // the PoC, only wrap when the true edge is the goto;
                        // otherwise virtualize the whole 2-out node as a goto by
                        // marking the other edge too is wrong — instead fall through
                        // and let the sequence/virtualize logic handle it.  Here we
                        // require edge 1 to be the goto (matching ruleBlockGoto's
                        // precondition; the SAILR virtualizer marks the true edge).
                        if self.graph.block(bl).is_goto_out(1) {
                            self.graph.new_block_if_goto(graph_id, bl)?;
                            return Ok(true);
                        }
                        // goto is on edge 0 (false): swap which edge is the goto by
                        // re-marking is unsafe; skip and let virtualize pick again.
                        continue;
                    }
                    if sizeout == 1 {
                        self.graph.new_block_goto(graph_id, bl);
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    // -----------------------------------------------------------------------
    // (c) edge virtualization — phoenix._last_resort_refinement +
    //     sailr._order_virtualizable_edges
    // -----------------------------------------------------------------------

    /// Pick the "best" remaining structured edge and mark it a `goto`
    /// (`set_goto_branch`), so the next `rule_block_goto` round wraps it.
    ///
    /// Port of `phoenix._last_resort_refinement` ordered by
    /// `sailr._order_virtualizable_edges` (H1 sibling-count, H3 return-edge), with
    /// the base `_chick_order_edges` tie-break.  Returns `false` only if no
    /// structured (non-goto, non-self) out-edge exists at all.
    fn virtualize_one_edge(&mut self) -> KunaResult<bool> {
        // Collect candidate edges: structured (non-goto) out-edges of components,
        // excluding self-loops and edges to a single-out chain that the sequence
        // schema would otherwise fold (so we never goto a foldable sequence).
        let mut candidates: Vec<Edge> = Vec::new();
        let n = self.size();
        for i in 0..n {
            let src = self.component(i);
            let sb = self.graph.block(src);
            let sout = sb.size_out();
            for e in 0..sout {
                if !sb.is_decision_out(e) {
                    continue; // already a goto / back edge
                }
                let dst = sb.get_out(e);
                if dst == src {
                    continue; // self-loop (back edge handled elsewhere)
                }
                candidates.push(Edge { src, edge: e, dst });
            }
        }
        if candidates.is_empty() {
            return Ok(false);
        }

        // Order by the SAILR heuristic and take the best.
        let best = self.order_virtualizable_edges(&candidates);
        // Mark the chosen out-edge as an unstructured goto (sets f_goto_edge +
        // f_interior_gotoout/in — exactly Ghidra's set_goto_branch).
        self.graph.set_goto_branch(best.src, best.edge)?;
        Ok(true)
    }

    /// Order virtualizable edges, best first (port of
    /// `sailr._order_virtualizable_edges` → `_chick_order_edges`).
    ///
    /// H1: prefer the edge whose destination has the fewest sibling in-edges
    /// (`in_degree(dst) - 1`).  H3: among ties, prefer an edge whose destination
    /// is a simple `return` block.  Final tie-break: the base `_chick_order_edges`
    /// ordering by destination address then source address (deterministic).
    fn order_virtualizable_edges<'e>(&self, edges: &'e [Edge]) -> &'e Edge {
        // H1: minimum sibling count.
        let sibling_count = |e: &Edge| -> int4 { self.graph.block(e.dst).size_in() - 1 };
        let min_siblings = edges.iter().map(sibling_count).min().unwrap_or(0);
        let mut best: Vec<&Edge> =
            edges.iter().filter(|e| sibling_count(e) == min_siblings).collect();
        if best.len() == 1 {
            return best[0];
        }

        // H3: prefer an edge to a simple return block.
        let returns: Vec<&Edge> =
            best.iter().copied().filter(|e| self.is_simple_return(e.dst)).collect();
        if returns.len() == 1 {
            return returns[0];
        }
        if !returns.is_empty() {
            best = returns;
        }

        // Base ordering (_chick_order_edges): by (-node_seq[dst], in_degree(dst),
        // out_degree(src), -src.addr, -dst.addr).  Without the region node_seq on
        // the sblocks ids, we approximate the deterministic base by (dst in-degree,
        // src out-degree, dst addr, src addr) — a stable, address-keyed order.
        best.sort_by(|a, b| {
            let ka = (
                self.graph.block(a.dst).size_in(),
                self.graph.block(a.src).size_out(),
                self.block_addr(a.dst),
                self.block_addr(a.src),
            );
            let kb = (
                self.graph.block(b.dst).size_in(),
                self.graph.block(b.src).size_out(),
                self.block_addr(b.dst),
                self.block_addr(b.src),
            );
            ka.cmp(&kb)
        });
        best[0]
    }

    /// Is `bl` a "simple return" block (angr `structured_node_is_simple_return`)?
    /// Approximated structurally: a leaf with no structured successors (a sink) —
    /// the destination of a return/exit edge.  Used by the H3 heuristic.
    fn is_simple_return(&self, bl: BlockId) -> bool {
        self.graph.block(bl).size_out() == 0
    }

    /// The start address of a component, resolving through its front-leaf
    /// `BlockCopy` to the underlying `bblocks` block range (deterministic key for
    /// edge ordering; falls back to `0` for a node with no resolvable cover).
    fn block_addr(&self, bl: BlockId) -> uintb {
        crate::block::block_get_start(&self.graph.arena, bl).get_offset()
    }
}

/// A candidate edge for virtualization: `src --edge--> dst` (`edge` is the
/// out-edge index on `src`).
struct Edge {
    src: BlockId,
    edge: int4,
    dst: BlockId,
}
