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

    // ---- 2. Structure the seeded sblocks graph -------------------------------
    let sroot = data.sblocks_root();
    let graph = data.sblocks_mut();
    let mut st = RegionStructurer::new(graph, sroot);
    st.structure()
}

/// The acyclic sequence + virtualize structuring engine, operating on the
/// seeded `sblocks` [`BlockGraph`] (the `BlockCopy` mirror of `bblocks`).
struct RegionStructurer<'a> {
    /// The structuring graph (the `sblocks` root graph) — same surface
    /// `CollapseStructure` drives.
    graph: &'a mut BlockGraph,
    /// The root BlockGraph node id (its `list` holds the live components).
    graph_id: BlockId,
}

impl<'a> RegionStructurer<'a> {
    fn new(graph: &'a mut BlockGraph, graph_id: BlockId) -> RegionStructurer<'a> {
        RegionStructurer { graph, graph_id }
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
