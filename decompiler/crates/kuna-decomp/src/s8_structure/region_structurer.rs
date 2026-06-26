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
//! # Increment 4 scope (switch-case recovery)
//!
//! [`match_acyclic_switch_cases`](RegionStructurer::match_acyclic_switch_cases)
//! recovers resolved jump-table switches natively: it matches the `f_switch_out`
//! head (the kuna analog of angr
//! `phoenix._match_acyclic_switch_cases_address_computed` →
//! `_switch_build_cases` → `_make_switch_cases_core`), resolves the recovered
//! [`JumpTable`](crate::jumptable::JumpTable) via the precomputed
//! `switch_blocks`/`switch_case_edges` maps (the same data Ghidra's
//! `CollapseStructure::ruleBlockSwitch` consumes), and folds it into a
//! [`BlockSwitch`](crate::block::BlockKind::Switch) carrying the
//! [`CaseOrder`](crate::block::CaseOrder) descriptors
//! `ActionFinalStructure::finalize_switch_printing` expects.  This is what lets a
//! switch region structure natively instead of forcing the whole function back to
//! `CollapseStructure` — the prerequisite for loop+switch (getopt-style) functions
//! getting the loop win once the cyclic schemas land.
//!
//! Loops (the cyclic schemas) and short-circuit folding are **later increments**;
//! a region the matched schemas cannot fold falls back to virtualized gotos
//! (honest-partial, never a panic).  Because the virtualize fallback always removes
//! one edge, the loop always converges to a single structured root — there is no
//! "stuck" state.
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
/// On success, returns `Ok((true, flips))` where `flips` is the list of `bblocks`
/// `BlockBasic` ids whose trailing CBRANCH must have its `boolean_flip`/
/// `fallthru_true` toggled — the deferred data-flow half of
/// `BlockBasic::negateCondition` (a loop-condition orientation flip), realized by
/// the caller via [`Funcdata::block_basic_negate_lastop`] exactly as the
/// `CollapseStructure` path does.  Returns `Ok((false, _))` if the graph could not
/// be collapsed to a single structured root (the caller re-seeds and falls back to
/// `CollapseStructure` — the flips are then irrelevant and discarded).  Never panics.
pub fn run_region_structurer(data: &mut Funcdata) -> KunaResult<(bool, Vec<BlockId>)> {
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

    // ---- 1c. Precompute BlockBasic::isComplex over bblocks (Inc 5) -----------
    // The short-circuit/condition-fold schema (`match_acyclic_short_circuit_conditions`,
    // a port of `CollapseStructure::ruleBlockOr`) gates the fold on the
    // `next_cond`/`orblock` being *non-complex* (a bare CBRANCH, no real
    // statements) — C++ `BlockBasic::isComplex`.  Mirror `ActionBlockStructure`'s
    // precomputation: a bblocks `BlockBasic` is complex when `bb_is_complex`
    // (more than one non-trivial statement); keyed by bblocks id, the same id each
    // `sblocks` `BlockCopy`'s `copy` references.
    let complex_blocks = compute_complex_blocks(data);

    // ---- 1d. Read the cyclic loop-successor refinement gate (regionlooprefine) -
    // Opt-in (default-OFF).  When ON, a multi-exit / multi-latch / mid-entry loop
    // that the base cyclic schemas cannot fold is refined (its secondary exits and
    // latches virtualized to gotos) so it folds into a structured loop instead of
    // falling back to `CollapseStructure`.  OFF ⇒ byte-identical to the base
    // region structurer (the refinement is dead).  Read before the `sblocks_mut`
    // borrow so we still hold `&Funcdata`.
    let loop_refine = data.get_arch().region_loop_refine;

    // When refinement is ON, project the region identifier's cyclic (loop) regions
    // onto basic-block start addresses: for each loop the head address, the body
    // block addresses, and the loop-successor (exit) block addresses (the angr
    // `GraphRegion.successors` frontier).  This is the read side of the omitted
    // `_refine_loop_successors_to_guarded_successors` — it gives the structurer the
    // region identifier's *correct* loop body (which absorbs the dominated
    // switch-case successors a structural natural-loop walk on the sblocks graph
    // misses) and exit frontier, so the refinement virtualizes exactly the
    // secondary exits.  Empty (and skipped) when refinement is off.
    let cyclic_loops = if loop_refine { ri.cyclic_loops() } else { Vec::new() };

    // The `cyclic_loops` are keyed by basic-block start ADDRESS, but the structuring
    // graph's `BlockCopy` leaves carry only the bblocks `BlockId` they copy (their
    // own `sblocks` cover is empty — `block_get_start` returns invalid through a
    // cross-arena copy).  Precompute the bblocks `BlockId → start offset` map here
    // (while we hold `&Funcdata`) so the structurer can resolve a live component's
    // front-leaf back to its address.  Skipped (empty) when refinement is off.
    let bb_addr: std::collections::BTreeMap<BlockId, uintb> = if loop_refine {
        let nbb = data.bblocks_get_size();
        let mut m = std::collections::BTreeMap::new();
        for i in 0..nbb {
            let bb = data.bblocks_get_block(i);
            m.insert(bb, data.bblocks_block_start(bb).get_offset());
        }
        m
    } else {
        std::collections::BTreeMap::new()
    };

    // ---- 2. Structure the seeded sblocks graph -------------------------------
    let sroot = data.sblocks_root();
    let graph = data.sblocks_mut();
    let mut st = RegionStructurer::new(graph, sroot)
        .with_switch_maps(switch_blocks, switch_case_edges)
        .with_complex_blocks(complex_blocks)
        .with_loop_refine(loop_refine, cyclic_loops, bb_addr);
    let ok = st.structure()?;
    let flips = if ok { st.take_pending_flips() } else { Vec::new() };
    if std::env::var_os("KUNA_RS_DEBUG").is_some() {
        eprintln!("[rs] structure -> ok={ok} flips={}", flips.len());
    }
    Ok((ok, flips))
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

/// Precompute the set of bblocks `BlockBasic` ids that are *complex* (Inc 5).
///
/// A mirror of the `complex_blocks` precomputation in
/// [`ActionBlockStructure::apply`](crate::blockaction::ActionBlockStructure): the
/// structuring graph is a `BlockCopy` mirror without op ownership, so
/// `BlockBasic::isComplex` is read here against the live `bblocks` op lists
/// (`Funcdata::bb_is_complex`) and keyed by the bblocks id each `BlockCopy`'s
/// `copy` pointer references.  The short-circuit/condition-fold schema reads it
/// through [`RegionStructurer::is_complex`] (the kuna analog of
/// `CollapseStructure::is_complex`, gating `ruleBlockOr`).
fn compute_complex_blocks(data: &Funcdata) -> std::collections::BTreeSet<BlockId> {
    let mut complex_blocks: std::collections::BTreeSet<BlockId> =
        std::collections::BTreeSet::new();
    let nbb = data.bblocks_get_size();
    for i in 0..nbb {
        let bb = data.bblocks_get_block(i);
        if data.bb_is_complex(bb) {
            complex_blocks.insert(bb);
        }
    }
    complex_blocks
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
    /// The set of `bblocks` `BlockBasic` ids that are *complex* (C++
    /// `BlockBasic::isComplex` — more than one non-trivial statement), keyed by the
    /// bblocks id each structuring `BlockCopy`'s `copy` references (Inc 5).  Read
    /// through [`RegionStructurer::is_complex`] by the short-circuit/condition-fold
    /// schema (the kuna analog of `CollapseStructure::is_complex`, which gates
    /// `ruleBlockOr`).  Empty ⇒ every block is treated as complex (conservative,
    /// never folds), so an unset map is honest-partial-safe.
    complex_blocks: std::collections::BTreeSet<BlockId>,
    /// `bblocks` `BlockBasic` ids whose CBRANCH op must have its
    /// `boolean_flip`/`fallthru_true` toggled (the deferred data-flow half of
    /// `BlockBasic::negateCondition`) — mirrors `CollapseStructure::pending_flips`.
    pending_flips: Vec<BlockId>,
    /// Cyclic loop-successor refinement gate (`Architecture::region_loop_refine`,
    /// option `regionlooprefine`, default-OFF).  When set, `match_cyclic_schemas`
    /// refines a multi-exit / multi-latch / mid-entry loop that `try_fold_loop`
    /// rejected — virtualizing its secondary exits and latches to gotos so it
    /// folds into a structured loop.  OFF ⇒ the refinement is never attempted, so
    /// output is byte-identical to the base region structurer.
    loop_refine: bool,
    /// The region identifier's cyclic (loop) regions, keyed by loop-head **block
    /// start address** (the angr refined loop body + successor frontier projected
    /// onto basic-block addresses).  Populated only when `loop_refine` is on; used
    /// by `refine_loop_edges` to pick the loop body / single normal exit grounded
    /// in the region identifier's analysis rather than the narrower structural
    /// natural-loop walk.
    cyclic_loops: std::collections::BTreeMap<uintb, crate::s7_regions::kuna_regionid::KunaCyclicLoop>,
    /// bblocks `BlockBasic` id → start offset, so a live `sblocks` component's
    /// front-leaf `BlockCopy` (whose `copy` is a bblocks id) resolves to its
    /// address — the key the `cyclic_loops` body/exit sets use.  Populated only
    /// when `loop_refine` is on.
    bb_addr: std::collections::BTreeMap<BlockId, uintb>,
}

impl<'a> RegionStructurer<'a> {
    fn new(graph: &'a mut BlockGraph, graph_id: BlockId) -> RegionStructurer<'a> {
        RegionStructurer {
            graph,
            graph_id,
            switch_blocks: std::collections::BTreeMap::new(),
            switch_case_edges: std::collections::BTreeMap::new(),
            complex_blocks: std::collections::BTreeSet::new(),
            pending_flips: Vec::new(),
            loop_refine: false,
            cyclic_loops: std::collections::BTreeMap::new(),
            bb_addr: std::collections::BTreeMap::new(),
        }
    }

    /// Attach the cyclic loop-successor refinement gate (`regionlooprefine`) plus
    /// the region identifier's cyclic loop projection (keyed by head address) and
    /// the bblocks-id → address map used to resolve live components to addresses.
    fn with_loop_refine(
        mut self,
        loop_refine: bool,
        cyclic_loops: Vec<crate::s7_regions::kuna_regionid::KunaCyclicLoop>,
        bb_addr: std::collections::BTreeMap<BlockId, uintb>,
    ) -> Self {
        self.loop_refine = loop_refine;
        self.cyclic_loops =
            cyclic_loops.into_iter().map(|l| (l.head_addr, l)).collect();
        self.bb_addr = bb_addr;
        self
    }

    /// Resolve a live structuring component to its front-leaf basic-block start
    /// address (the key the `cyclic_loops` body/exit sets use).  A merged component
    /// (e.g. a `BlockList`) resolves through its front leaf; a leaf with no
    /// resolvable `copy` (or an unmapped id) yields `None`.
    fn component_addr(&self, bl: BlockId) -> Option<uintb> {
        let leaf = self.graph.get_front_leaf(bl)?;
        let bb = self.graph.block(leaf).get_copy()?;
        self.bb_addr.get(&bb).copied()
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

    /// Attach the precomputed `BlockBasic::isComplex` set (Inc 5 short-circuit
    /// schema), keyed by bblocks `BlockBasic` id.  Mirrors
    /// [`CollapseStructure::with_complex_blocks`](crate::blockaction::CollapseStructure).
    fn with_complex_blocks(
        mut self,
        complex_blocks: std::collections::BTreeSet<BlockId>,
    ) -> Self {
        self.complex_blocks = complex_blocks;
        self
    }

    /// The XOR-reduced set of `bblocks` `BlockBasic` ids whose CBRANCH op flags
    /// must flip (even flips cancel) — identical reduction to
    /// `CollapseStructure::take_pending_flips`.
    fn take_pending_flips(&mut self) -> Vec<BlockId> {
        let mut parity: std::collections::BTreeMap<BlockId, bool> =
            std::collections::BTreeMap::new();
        for &b in &self.pending_flips {
            let e = parity.entry(b).or_insert(false);
            *e = !*e;
        }
        parity.into_iter().filter(|&(_, v)| v).map(|(b, _)| b).collect()
    }

    /// Port of `CollapseStructure::negate_condition_rec` (the data-flow + topology
    /// half of `FlowBlock::negateCondition`): swap `bl`'s edges and record the
    /// leaf CBRANCH flip into `pending_flips`.  Returns `true` if a data-flow
    /// change was recorded.
    fn negate_condition_rec(&mut self, bl: BlockId, toporbottom: bool) -> bool {
        use crate::block::BlockType;
        let bt = self.graph.block(bl).get_type();
        let res = match bt {
            BlockType::Copy => {
                if let Some(copy) = self.graph.block(bl).get_copy() {
                    self.pending_flips.push(copy);
                }
                true
            }
            BlockType::Ls => {
                let sz = self.graph.block(bl).get_size();
                if sz > 0 {
                    let last = self.graph.block(bl).get_block(sz - 1);
                    self.negate_condition_rec(last, false)
                } else {
                    false
                }
            }
            BlockType::Condition => {
                let b0 = self.graph.block(bl).get_block(0);
                let b1 = self.graph.block(bl).get_block(1);
                let r0 = self.negate_condition_rec(b0, false);
                let r1 = self.negate_condition_rec(b1, false);
                self.graph.block_mut(bl).flip_condition_opcode();
                r0 || r1
            }
            _ => false,
        };
        if toporbottom {
            self.graph.swap_edges(bl);
        }
        res
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

        // (a1 pre-pass) short-circuit / condition folding (Inc 5).  Ghidra
        // `CollapseStructure::collapseAll` runs `collapseConditions` — which is
        // *only* `ruleBlockOr`, to a fixpoint — BEFORE the if/sequence/loop
        // cascade, so cascading conditions (`if (A) { if (B) … }`) fold to a single
        // compound `A && B` / `A || B` `BlockCondition` first.  We mirror that here:
        // run `match_acyclic_short_circuit_conditions` to a fixpoint up front,
        // before the main schema loop.  Each fold removes one component, so the loop
        // is bounded by `cap`.
        let mut precond_rounds: i64 = 0;
        loop {
            precond_rounds += 1;
            if precond_rounds > cap {
                break; // hang-guard; main loop's fallback still converges
            }
            if !self.match_acyclic_short_circuit_conditions()? {
                break;
            }
        }

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
            // (a3) cyclic loop schemas (Inc 3; phoenix._analyze_cyclic +
            //      _match_cyclic_{while,dowhile,natural_loop}): fold a natural loop
            //      (head with an incoming back-edge) into a
            //      BlockWhileDo/BlockDoWhile/BlockInfLoop, with secondary exits and
            //      secondary latches realized as break/continue gotos (kuna goto
            //      flags).  Lean on the back-edges + dominators already marked on
            //      the seeded sblocks `BlockCopy`s (copied verbatim from `bblocks`
            //      by `build_copy_from`).  Honest-partial: a loop that doesn't match
            //      one of the schemas is left for the virtualize fallback.
            if self.match_cyclic_schemas()? {
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
            if std::env::var_os("KUNA_RS_DEBUG").is_some() {
                let n = self.size();
                eprintln!("[rs] STUCK: {n} components remain (round {rounds})");
                for i in 0..n.min(12) {
                    let c = self.component(i);
                    let b = self.graph.block(c);
                    eprintln!(
                        "[rs]   comp#{} type={:?} sizeout={} sizein={} addr=0x{:x}",
                        b.get_index(),
                        b.get_type(),
                        b.size_out(),
                        b.size_in(),
                        self.component_addr(c).unwrap_or(0),
                    );
                }
            }
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
    // (a1) short-circuit / condition-fold schema — phoenix
    //      ._match_acyclic_short_circuit_conditions (types a–d), realized over
    //      kuna's `BlockCondition` builder exactly as Ghidra
    //      `CollapseStructure::ruleBlockOr` (`blockaction.cc:1321`) does.
    // -----------------------------------------------------------------------

    /// Is `bl` too complex to be folded as a *cascading* condition clause (C++
    /// `FlowBlock::isComplex` virtual dispatch)?
    ///
    /// The base returns `true`; `BlockList`/`BlockCopy` delegate down to the front
    /// `BlockBasic`, whose statement count is the real test.  Resolves `bl` to its
    /// front-leaf `BlockCopy` and reads the precomputed per-BlockBasic complexity
    /// (`complex_blocks`, keyed by the `copy` pointer).  A block that does not
    /// resolve to a `BlockCopy`/`BlockBasic` falls back to the conservative `true`
    /// (so it never participates in a fold — honest-partial).  Identical to
    /// [`CollapseStructure::is_complex`](crate::blockaction::CollapseStructure).
    fn is_complex(&self, bl: BlockId) -> bool {
        let leaf = match self.graph.get_front_leaf(bl) {
            Some(l) => l,
            None => return true, // base FlowBlock::isComplex
        };
        match self.graph.block(leaf).get_copy() {
            Some(basic) => self.complex_blocks.contains(&basic),
            None => true,
        }
    }

    /// Fold cascading short-circuit conditions (`if (A) { if (B) … }` /
    /// `if (A) goto X; if (B) goto X;` and the `||`/`&&`-with-else shapes) into a
    /// single [`BlockCondition`](crate::block::BlockKind::Condition) carrying a
    /// compound `A && B` / `A || B` condition.
    ///
    /// This is the kuna analog of angr `phoenix._match_acyclic_short_circuit_conditions`
    /// (types a–d), realized as a faithful port of Ghidra
    /// `CollapseStructure::ruleBlockOr` (`blockaction.cc:1321`) over kuna's
    /// `BlockCondition` builder — so the folded condition is byte-identical to the
    /// `CollapseStructure` (default-OFF) form.  `new_block_condition` picks
    /// `CPUI_INT_OR`/`CPUI_INT_AND` from the edge orientation (the false-out of `b1`
    /// being `b2` ⇒ `||`, else `&&`), exactly Ghidra's choice; the renderer
    /// lowers those to `||`/`&&`.
    ///
    /// Mirrors Ghidra's pass order: `CollapseStructure::collapseAll` runs
    /// `collapseConditions` (which is *only* `ruleBlockOr`, to a fixpoint) **before**
    /// the if/sequence/loop cascade.  The region structurer therefore runs this
    /// schema before the sequence/ITE schemas (see [`structure`]).
    ///
    /// Honest-partial: a 2-out node whose successor is not a single-in, non-complex,
    /// 2-out condition reconverging through the shared clause is left untouched and
    /// flows to the sequence/ITE schemas (and ultimately the virtualize fallback).
    fn match_acyclic_short_circuit_conditions(&mut self) -> KunaResult<bool> {
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            if self.try_block_or(bl)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Attempt to fold the cascading condition rooted at `bl` (port of Ghidra
    /// `CollapseStructure::ruleBlockOr`, `blockaction.cc:1321`).
    ///
    /// `bl` and its successor `orblock` are both binary (2-out) conditions; `orblock`
    /// is single-in (nothing else reaches it), non-complex (a bare CBRANCH), and not
    /// a goto/switch target; one of `orblock`'s out-edges (the shared `clauseblock`)
    /// is also a direct out-edge of `bl`.  We orient `orblock` onto `bl`'s false out
    /// (and `clauseblock` onto `orblock`'s true out) via `negate_condition_rec`
    /// (recording the deferred data-flow flips), then fold the pair into a
    /// `BlockCondition`.
    fn try_block_or(&mut self, bl: BlockId) -> KunaResult<bool> {
        if self.graph.block(bl).size_out() != 2 {
            return Ok(false);
        }
        if self.graph.block(bl).is_goto_out(0) {
            return Ok(false);
        }
        if self.graph.block(bl).is_goto_out(1) {
            return Ok(false);
        }
        if self.graph.block(bl).is_switch_out() {
            return Ok(false);
        }
        for i in 0..2 {
            let orblock = self.graph.block(bl).get_out(i); // False out is other part of OR
            if orblock == bl {
                continue; // orblock cannot be same block
            }
            if self.graph.block(orblock).size_in() != 1 {
                continue; // Nothing else can hit orblock
            }
            if self.graph.block(orblock).size_out() != 2 {
                continue; // orblock must also be binary condition
            }
            if self.graph.block(orblock).is_interior_goto_target() {
                continue; // No unstructured jumps into or
            }
            if self.graph.block(orblock).is_switch_out() {
                continue;
            }
            if self.graph.block(bl).is_back_edge_out(i) {
                continue; // Don't use loop branch to get to orblock
            }
            if self.is_complex(orblock) {
                continue;
            }
            let clauseblock = self.graph.block(bl).get_out(1 - i);
            if clauseblock == bl {
                continue; // No looping
            }
            if clauseblock == orblock {
                continue;
            }
            let mut j = 0;
            while j < 2 {
                if clauseblock == self.graph.block(orblock).get_out(j) {
                    break;
                }
                j += 1;
            }
            if j == 2 {
                continue;
            }
            if self.graph.block(orblock).get_out(1 - j) == bl {
                continue; // No looping
            }

            if i == 1 {
                // orblock needs to be false out of bl
                self.negate_condition_rec(bl, true);
            }
            if j == 0 {
                // clauseblock needs to be true out of orblock
                self.negate_condition_rec(orblock, true);
            }

            let graph_id = self.graph_id;
            self.graph.new_block_condition(graph_id, bl, orblock)?;
            return Ok(true);
        }
        Ok(false)
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
    // (a3) cyclic loop schemas — Inc 3 (phoenix._analyze_cyclic +
    //      _match_cyclic_{while,dowhile,natural_loop} + the break/continue
    //      rewriters, realized over kuna goto flags)
    // -----------------------------------------------------------------------

    /// Find a natural loop in the live component set and either fold it into a
    /// loop block (`BlockWhileDo`/`BlockDoWhile`/`BlockInfLoop`) or *refine* it
    /// (mark its secondary exits/latches as gotos) so a later round can fold it.
    ///
    /// Port of angr `phoenix._analyze_cyclic` / `_match_cyclic_schemas`, realized
    /// structurally on the seeded `sblocks` graph (the `BlockCopy` mirror of
    /// `bblocks`).  Unlike the `CollapseStructure` `TraceDAG`/`select_goto`
    /// fallback — which virtualizes loop edges by a generic least-structurable
    /// heuristic and therefore emits a goto per non-collapsible latch/exit — this
    /// schema uses the loop's *body and exit* identification (the angr
    /// `RegionIdentifier._refine_loop` view) to pick exactly one normal exit and
    /// one continue target, turning the remaining edges into `break;`/`continue;`
    /// (kuna `f_break_goto` / `f_continue_goto` / `f_goto_edge`).  Returns `true`
    /// if it folded or refined a loop (made progress).
    fn match_cyclic_schemas(&mut self) -> KunaResult<bool> {
        // Locate the innermost natural-loop head: a live component reached by a
        // back-edge whose own loop body contains no *other* (still-live) loop head
        // (so we structure inside-out, matching CollapseStructure's depth order).
        let heads = self.collect_loop_heads();
        let dbg = std::env::var_os("KUNA_RS_DEBUG").is_some();
        if dbg && self.loop_refine {
            // One-shot dump of the region identifier's cyclic loop projection.
            static DUMPED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);
            if !DUMPED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                for (head, l) in self.cyclic_loops.iter() {
                    eprintln!(
                        "[rs] RI loop head=0x{:x} body={} exits=[{}]",
                        head,
                        l.body.len(),
                        l.exits.iter().map(|a| format!("0x{a:x}")).collect::<Vec<_>>().join(",")
                    );
                }
            }
        }
        if heads.is_empty() {
            return Ok(false);
        }
        if dbg {
            eprintln!("[rs] cyclic heads={}", heads.len());
            for &head in heads.iter() {
                let body = self.natural_loop_body(head);
                eprintln!(
                    "[rs]   head#{} sizeout={} sizein={} body={}",
                    self.graph.block(head).get_index(),
                    self.graph.block(head).size_out(),
                    self.graph.block(head).size_in(),
                    body.iter()
                        .map(|&b| format!(
                            "#{}[o{} i{}]",
                            self.graph.block(b).get_index(),
                            self.graph.block(b).size_out(),
                            self.graph.block(b).size_in()
                        ))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
        }
        for &head in heads.iter() {
            // Try to fold a loop that is already in a foldable structural shape —
            // its body has collapsed (via the acyclic sequence/ITE schemas) to a
            // single block that loops to `head` with at most one structural exit.
            if self.try_fold_loop(head)? {
                if dbg {
                    eprintln!("[rs] folded loop head");
                }
                return Ok(true);
            }
        }

        // No directly-foldable loop.  WITHOUT the `regionlooprefine` opt-in
        // (default), we stop here: leaving a multi-exit / multi-latch / irreducible
        // loop unfolded lets the structurer fall through to the virtualize fallback
        // and ultimately back to `CollapseStructure` (proven optimal on reducible
        // loops), guaranteeing the region structurer is never worse than the
        // upstream Ghidra path (the DIV-12 default-on parity invariant).
        //
        // WITH `regionlooprefine` ON, refine the loops the base schemas could not
        // fold: virtualize their SECONDARY exits and latches to gotos so the body
        // collapses to the clean single-exit / single-latch shape `try_fold_loop`
        // accepts — the kuna analog of angr's
        // `RegionIdentifier._refine_loop_successors_to_guarded_successors`
        // (`force_loop_single_exit`).  The virtualized loop-exit gotos are then
        // lowered to `break;` by the existing `scopeBreak` pass and the back-edge
        // gotos to `continue;` by loop construction.  Only loops that would
        // otherwise fall back are touched (`try_fold_loop` already had its turn
        // above), so reducible code is byte-identical regardless of the flag.
        if self.loop_refine {
            // Innermost-first: refine the deepest loop head (largest `index`) whose
            // body contains no other live loop head, so the inside-out fold order
            // matches CollapseStructure.  Refining one head per round (returning
            // true) lets the acyclic schemas re-collapse the freed body before the
            // next loop is considered.
            let ordered = self.order_loop_heads_innermost_first(&heads);
            for &head in ordered.iter() {
                match self.refine_loop_edges(head)? {
                    LoopRefineOutcome::Progressed => {
                        if dbg {
                            eprintln!("[rs]   refined loop head#{}", self.graph.block(head).get_index());
                        }
                        return Ok(true);
                    }
                    LoopRefineOutcome::Irreducible | LoopRefineOutcome::NoChange => {}
                }
            }
        }
        Ok(false)
    }

    /// Order loop heads innermost-first: a head whose natural-loop body contains
    /// *no other* live loop head sorts before an outer head, so refinement (like
    /// `CollapseStructure`'s depth-ordered `loopbody`) reduces the deepest loop
    /// first and its now-collapsed body becomes a single node inside the outer
    /// loop.  Stable within a depth class by `index`.
    fn order_loop_heads_innermost_first(&self, heads: &[BlockId]) -> Vec<BlockId> {
        // For each head, count how many OTHER heads sit inside its body (its
        // "outerness").  Innermost = 0 inner heads; sort ascending by that count,
        // then by descending index (deeper RPO first) as a stable tiebreak.
        let head_set: std::collections::BTreeSet<BlockId> = heads.iter().copied().collect();
        let mut keyed: Vec<(int4, int4, BlockId)> = Vec::with_capacity(heads.len());
        for &head in heads.iter() {
            let body = self.natural_loop_body(head);
            let inner = body
                .iter()
                .filter(|&&b| b != head && head_set.contains(&b))
                .count() as int4;
            // Negated index so a higher index (deeper) sorts earlier within a class.
            keyed.push((inner, -self.graph.block(head).get_index(), head));
        }
        keyed.sort();
        keyed.into_iter().map(|(_, _, h)| h).collect()
    }

    /// Collect every live component that is a natural-loop head: it has at least
    /// one incoming **back-edge** (`f_back_edge`, marked by `structure_loops` on
    /// `bblocks` and copied into the `sblocks` `BlockCopy`s).  Returned in
    /// component order (deterministic).
    fn collect_loop_heads(&self) -> Vec<BlockId> {
        let mut heads = Vec::new();
        let n = self.size();
        for i in 0..n {
            let bl = self.component(i);
            let b = self.graph.block(bl);
            let sizein = b.size_in();
            for j in 0..sizein {
                if b.is_back_edge_in(j) {
                    heads.push(bl);
                    break;
                }
            }
        }
        heads
    }

    /// Is `head` the head of an *innermost* loop — i.e. its natural-loop body (the
    /// nodes that reach a latch of `head` without leaving the dominator subtree of
    /// `head`) contains no *other* live loop head?  Mirrors the inside-out order
    /// `CollapseStructure` gets from its depth-sorted `loopbody`.
    ///
    /// (Retained for a future increment: the speculative `refine_loop_edges`
    /// machinery this supports is not run in Inc 3 — see `match_cyclic_schemas`.)
    #[allow(dead_code)]
    fn is_innermost_loop_head(&self, head: BlockId) -> KunaResult<bool> {
        let body = self.natural_loop_body(head);
        for &bl in body.iter() {
            if bl == head {
                continue;
            }
            // Another live loop head inside the body ⇒ not innermost.
            let b = self.graph.block(bl);
            for j in 0..b.size_in() {
                if b.is_back_edge_in(j) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Compute the natural-loop body of `head`: starting from each latch (a
    /// back-edge source into `head`), walk predecessors that are dominated by
    /// `head`, collecting every node on a path back to `head`.  Classic
    /// natural-loop body, computed over the live `sblocks` graph using the
    /// dominator info (`immed_dom`/`index`) copied from `bblocks`.
    fn natural_loop_body(&self, head: BlockId) -> Vec<BlockId> {
        let mut body: Vec<BlockId> = vec![head];
        let mut in_body = std::collections::BTreeSet::new();
        in_body.insert(head);
        // Seed with latches (back-edge sources into head).
        let hb = self.graph.block(head);
        let mut stack: Vec<BlockId> = Vec::new();
        for j in 0..hb.size_in() {
            if hb.is_back_edge_in(j) {
                let latch = hb.get_in(j);
                if in_body.insert(latch) {
                    body.push(latch);
                    stack.push(latch);
                }
            }
        }
        // Walk predecessors that are dominated by head (so we stay inside the loop).
        while let Some(cur) = stack.pop() {
            let cb = self.graph.block(cur);
            for j in 0..cb.size_in() {
                let p = cb.get_in(j);
                if in_body.contains(&p) {
                    continue;
                }
                // Only nodes dominated by head belong to the natural loop.
                if self.graph.dominates(head, Some(p)) {
                    in_body.insert(p);
                    body.push(p);
                    stack.push(p);
                }
            }
        }
        body
    }

    /// Try to fold a loop whose body has already collapsed so the loop is a clean
    /// structural shape the kuna loop builders accept.  Three cases, in
    /// `CollapseStructure` precedence:
    ///
    ///   * **while-do** — `head` is a 2-out condition, one out-edge is a *clause*
    ///     (single-in/single-out) that loops back to `head`, the other leaves the
    ///     loop (`BlockWhileDo`).
    ///   * **do-while** — `head` is a 2-out condition with one out-edge a self-loop
    ///     to `head` and the other leaving (`BlockDoWhile`).
    ///   * **inf-loop** — `head` is a 1-out block whose single out-edge self-loops
    ///     to `head` (`BlockInfLoop`).
    ///
    /// These mirror `CollapseStructure::ruleBlockWhileDo/DoWhile/InfLoop` exactly,
    /// but are only attempted on a confirmed loop head; the difference vs. Ghidra
    /// is purely *which edges have been virtualized to gotos first* (`refine_loop_edges`).
    fn try_fold_loop(&mut self, head: BlockId) -> KunaResult<bool> {
        let b = self.graph.block(head);
        let sizeout = b.size_out();

        // inf-loop: single out-edge looping back onto head.
        if sizeout == 1 {
            if b.is_goto_out(0) || b.is_switch_out() {
                return Ok(false);
            }
            if b.get_out(0) != head {
                return Ok(false);
            }
            let graph_id = self.graph_id;
            self.graph.new_block_inf_loop(graph_id, head);
            return Ok(true);
        }
        if sizeout != 2 || b.is_switch_out() {
            return Ok(false);
        }
        if b.is_goto_out(0) || b.is_goto_out(1) {
            return Ok(false);
        }

        // do-while: one of head's out-edges self-loops to head
        // (`CollapseStructure::ruleBlockDoWhile`).  The loop must execute on the
        // TRUE condition; if the self-loop is the FALSE edge (i == 0) we flip the
        // condition (recording the deferred data-flow half) — exactly as
        // `ruleBlockDoWhile` calls `negateCondition`.
        for i in 0..2 {
            if self.graph.block(head).get_out(i) == head {
                if i == 0 {
                    self.negate_condition_rec(head, true);
                }
                let graph_id = self.graph_id;
                self.graph.new_block_do_while(graph_id, head);
                return Ok(true);
            }
        }

        // while-do: one out-edge is a clause (single-in/single-out) that loops
        // back to head; the other leaves the loop
        // (`CollapseStructure::ruleBlockWhileDo`).  The clause must be the TRUE
        // exit of head; if it is the FALSE exit (i == 0) we flip the condition.
        //
        // Crucial guard (`ruleBlockWhileDo` `blockaction.cc:1535`): the head must
        // NOT be an interior-goto target.  If a virtualized continue-goto (or any
        // other goto edge) targets the head, folding it as a top-tested while-do
        // would render the head's condition with a `label:` *inside* the
        // `while (...)` expression — malformed C.  Such a loop is really an
        // infinite loop with the head-condition as an `if (...) break;` in the
        // body; rejecting here lets `refine_loop_edges` virtualize the head's exit
        // edge so the loop folds as a `BlockInfLoop` instead (the OFF/Ghidra form).
        if self.graph.block(head).is_interior_goto_target() {
            return Ok(false);
        }
        for i in 0..2 {
            let clause = self.graph.block(head).get_out(i);
            if clause == head {
                continue;
            }
            let cb = self.graph.block(clause);
            if cb.size_in() != 1 || cb.size_out() != 1 {
                continue;
            }
            if cb.is_switch_out() || cb.is_goto_out(0) {
                continue;
            }
            if cb.get_out(0) != head {
                continue; // clause must loop back to head
            }
            if i == 0 {
                self.negate_condition_rec(head, true);
            }
            let graph_id = self.graph_id;
            // After a possible flip the clause is now the TRUE out; re-read it.
            let clause_now = self.graph.block(head).get_true_out();
            self.graph.new_block_while_do(graph_id, head, clause_now);
            return Ok(true);
        }

        Ok(false)
    }

    /// Refine a not-yet-foldable loop by marking its *secondary* control edges as
    /// gotos, so the body collapses to a single self-loop the fold rules accept.
    ///
    /// This is the kuna analog of angr `_refine_cyclic_core` + the
    /// break/continue rewriters: pick ONE normal exit and ONE continue target
    /// (the back-edge to `head`), and convert
    ///
    ///   * every **secondary back-edge** (a latch edge to `head` that is not the
    ///     single primary latch) into a `continue;` (`f_continue_goto` on the
    ///     block + `f_goto_edge` on the edge);
    ///   * every **secondary exit edge** (a body→non-body edge whose target is not
    ///     the chosen normal exit) into a `break;` (`f_break_goto` + `f_goto_edge`)
    ///     when its target is the normal exit, or a plain `goto` otherwise;
    ///   * a **loop-entry edge into the middle of the body** (an edge from outside
    ///     the loop to a body node other than `head` — the head-in-body /
    ///     irreducible case) into a `goto` so the body becomes single-entry.
    ///
    /// Returns [`LoopRefineOutcome::Progressed`] if it marked at least one edge,
    /// [`LoopRefineOutcome::Irreducible`] if the loop is multi-entry at the head
    /// (cannot be refined — caller falls back), or [`LoopRefineOutcome::NoChange`]
    /// if nothing needed refining (already single-exit / single-latch; the base
    /// `try_fold_loop` will fold it, or it is a not-yet-collapsed body that a later
    /// round handles).  Per angr *without* `force_loop_single_exit`, one secondary
    /// exit may legitimately remain a single goto — that's acceptable and still
    /// strictly fewer gotos than Ghidra's per-latch virtualization.
    ///
    /// Gated by `regionlooprefine` (the `self.loop_refine` flag), and only invoked
    /// on a loop the base cyclic schemas already declined to fold, so a reducible
    /// loop never reaches here and output is byte-identical with the flag off.
    fn refine_loop_edges(&mut self, head: BlockId) -> KunaResult<LoopRefineOutcome> {
        // Resolve the loop using the region identifier's projection if we have it
        // (the angr refined loop body + successor frontier, which absorbs the
        // dominated switch-case successors a structural natural-loop walk misses).
        // The loop is keyed by its head's basic-block start address.
        let head_addr = self.component_addr(head);
        let ri_loop = head_addr.and_then(|a| self.cyclic_loops.get(&a).cloned());

        // Build the live loop-body component set + exit-address set, either from the
        // RI projection (preferred) or the structural natural-loop walk (fallback,
        // when the head has no RI loop — e.g. a loop only the structural marker
        // sees).  A live component is "in the body" when its front-leaf address is
        // in the RI body set.
        let dbg = std::env::var_os("KUNA_RS_DEBUG").is_some();
        let (in_body, exit_addrs): (std::collections::BTreeSet<BlockId>, Option<std::collections::BTreeSet<uintb>>) =
            match &ri_loop {
                Some(l) => {
                    let mut s = std::collections::BTreeSet::new();
                    let n = self.size();
                    for i in 0..n {
                        let comp = self.component(i);
                        if let Some(a) = self.component_addr(comp) {
                            if l.body.contains(&a) {
                                s.insert(comp);
                            }
                        }
                    }
                    // `head` must be in the body set (it carries the head address).
                    s.insert(head);
                    (s, Some(l.exits.clone()))
                }
                None => {
                    let body = self.natural_loop_body(head);
                    (body.into_iter().collect(), None)
                }
            };

        // (1) Latch edges: back-edges into `head` from a body component.
        let mut latch_edges: Vec<(BlockId, int4)> = Vec::new();
        {
            let hb = self.graph.block(head);
            for j in 0..hb.size_in() {
                if hb.is_back_edge_in(j) {
                    let latch = hb.get_in(j);
                    let lb = self.graph.block(latch);
                    for e in 0..lb.size_out() {
                        if lb.get_out(e) == head && lb.is_back_edge_out(e) && !lb.is_goto_out(e) {
                            latch_edges.push((latch, e));
                        }
                    }
                }
            }
        }

        // (2) Exit edges (body → non-body) and mid-body entry edges
        //     (non-body → body node ≠ head).  A target is "non-body" when it is not
        //     a body component; with the RI exit set, a body→exit edge is an exit.
        let mut exit_edges: Vec<(BlockId, int4, BlockId)> = Vec::new();
        let mut mid_entry_edges: Vec<(BlockId, int4)> = Vec::new();
        let body_vec: Vec<BlockId> = in_body.iter().copied().collect();
        for &bl in body_vec.iter() {
            let b = self.graph.block(bl);
            for e in 0..b.size_out() {
                let dst = b.get_out(e);
                if dst == bl {
                    continue;
                }
                if !in_body.contains(&dst) && !b.is_goto_out(e) && !b.is_back_edge_out(e) {
                    exit_edges.push((bl, e, dst));
                }
            }
            if bl != head {
                let sin = b.size_in();
                for j in 0..sin {
                    let p = b.get_in(j);
                    if !in_body.contains(&p) {
                        let pb = self.graph.block(p);
                        for e in 0..pb.size_out() {
                            if pb.get_out(e) == bl && !pb.is_goto_out(e) && !pb.is_back_edge_out(e) {
                                mid_entry_edges.push((p, e));
                            }
                        }
                    }
                }
            }
        }

        // (3) Normal exit: prefer the RI successor frontier ordering — the exit
        //     target whose ADDRESS appears earliest in the RI exit set (the angr
        //     normal-exit is the post-order-earliest successor; lowest address is a
        //     deterministic, region-grounded surrogate).  Without RI data, fall
        //     back to the most-targeted exit (ties by index).
        let normal_exit = self.choose_normal_exit_grounded(&exit_edges, &exit_addrs);

        if dbg {
            eprintln!(
                "[rs]     refine head=0x{:x} ri={} body={} latches={} exits={} mid={} head_xin={}",
                head_addr.unwrap_or(0),
                ri_loop.is_some(),
                in_body.len(),
                latch_edges.len(),
                exit_edges.len(),
                mid_entry_edges.len(),
                self.head_extra_entries(head, &in_body),
            );
        }

        // Irreducible at the head (multi-entry that is not a back-edge): when the
        // head is a real RI loop head, virtualize the *extra* head entries (the
        // abnormal-entry edges that make it multi-entry) to gotos so the loop
        // becomes single-entry — the kuna analog of angr's abnormal-entries
        // handling.  When it is NOT a real RI loop head (a structural false
        // positive, e.g. a high-in-degree continuation merge the natural-loop walk
        // mistakes for a loop), refinement must not touch it — bail so the merge is
        // handled by the acyclic schemas / virtualize fallback instead.
        if self.head_extra_entries(head, &in_body) > 0 {
            if ri_loop.is_none() {
                return Ok(LoopRefineOutcome::Irreducible);
            }
            let extra = self.head_extra_entry_edges(head, &in_body);
            let mut any = false;
            for (src, e) in extra {
                self.graph.set_goto_branch(src, e)?;
                any = true;
            }
            return Ok(if any {
                LoopRefineOutcome::Progressed
            } else {
                LoopRefineOutcome::Irreducible
            });
        }

        let mut progressed = false;

        // (3a) Mid-body entries → goto (make the loop single-entry at head).
        for (src, e) in mid_entry_edges.iter() {
            self.graph.set_goto_branch(*src, *e)?;
            progressed = true;
        }
        if progressed {
            return Ok(LoopRefineOutcome::Progressed);
        }

        // (3b) Secondary latches → goto (continue).  Keep ONE primary latch
        //      structural (the deepest, highest-index source) so the fold rules
        //      have the single back-edge they need; virtualize the rest.
        if latch_edges.len() > 1 {
            let primary = latch_edges
                .iter()
                .copied()
                .max_by_key(|&(latch, _)| self.graph.block(latch).get_index())
                .unwrap();
            for &(latch, e) in latch_edges.iter() {
                if (latch, e) == primary {
                    continue;
                }
                self.graph.set_goto_branch(latch, e)?;
                progressed = true;
            }
            if progressed {
                return Ok(LoopRefineOutcome::Progressed);
            }
        }

        // (3c) Secondary exits → goto (break).  Keep ONE structural exit (to the
        //      chosen normal-exit target) and virtualize every other exit edge.
        //      The kept exit lets the loop fold (single structural exit + single
        //      back-edge → while/do-while/inf-loop); `scopeBreak` then lowers the
        //      virtualized loop-exit gotos to `break;`.
        if exit_edges.len() > 1 {
            let keep = self.choose_structural_exit(head, &exit_edges, normal_exit);
            for &(src, e, _dst) in exit_edges.iter() {
                if Some((src, e)) == keep {
                    continue;
                }
                self.graph.set_goto_branch(src, e)?;
                progressed = true;
            }
        }

        Ok(if progressed {
            LoopRefineOutcome::Progressed
        } else {
            LoopRefineOutcome::NoChange
        })
    }

    /// Choose the loop's normal exit target.  With the region identifier's exit
    /// frontier (`exit_addrs`), prefer the live exit edge whose destination address
    /// is the LOWEST in that frontier (a deterministic, region-grounded surrogate
    /// for the angr post-order-earliest normal exit).  Otherwise fall back to
    /// [`Self::choose_normal_exit`] (most-targeted, ties by index).
    fn choose_normal_exit_grounded(
        &self,
        exit_edges: &[(BlockId, int4, BlockId)],
        exit_addrs: &Option<std::collections::BTreeSet<uintb>>,
    ) -> Option<BlockId> {
        if let Some(addrs) = exit_addrs {
            if !addrs.is_empty() {
                // Among the live exit-edge destinations, pick the one whose address
                // is smallest (BTreeSet iterates ascending, so the first matching
                // frontier address that has a live edge wins).
                let mut best: Option<(uintb, BlockId)> = None;
                for &(_, _, dst) in exit_edges.iter() {
                    if let Some(a) = self.component_addr(dst) {
                        if addrs.contains(&a) {
                            match best {
                                None => best = Some((a, dst)),
                                Some((ba, _)) if a < ba => best = Some((a, dst)),
                                _ => {}
                            }
                        }
                    }
                }
                if let Some((_, dst)) = best {
                    return Some(dst);
                }
            }
        }
        self.choose_normal_exit(exit_edges)
    }

    /// Pick the single exit edge to keep structural when refining a multi-exit
    /// loop: prefer an exit whose source is the loop head (so the head's natural
    /// 2-out condition remains the loop test → a clean while/do-while), else the
    /// earliest exit edge by `(src index, dst index)`.  Returns `(src, edge)`.
    fn choose_structural_exit(
        &self,
        head: BlockId,
        exit_edges: &[(BlockId, int4, BlockId)],
        normal_exit: Option<BlockId>,
    ) -> Option<(BlockId, int4)> {
        // 1. An exit straight off the head condition to the normal exit.
        if let Some(ne) = normal_exit {
            for &(src, e, dst) in exit_edges.iter() {
                if src == head && dst == ne {
                    return Some((src, e));
                }
            }
            // 2. Any exit off the head to the normal exit's component.
            for &(src, e, dst) in exit_edges.iter() {
                if src == head && dst == ne {
                    return Some((src, e));
                }
            }
        }
        // 3. Earliest exit edge by (src index, dst index).
        let mut best: Option<(BlockId, int4, BlockId)> = None;
        for &(src, e, dst) in exit_edges.iter() {
            match best {
                None => best = Some((src, e, dst)),
                Some((bs, _, bd)) => {
                    let (si, di) =
                        (self.graph.block(src).get_index(), self.graph.block(dst).get_index());
                    let (bsi, bdi) =
                        (self.graph.block(bs).get_index(), self.graph.block(bd).get_index());
                    if (si, di) < (bsi, bdi) {
                        best = Some((src, e, dst));
                    }
                }
            }
        }
        best.map(|(s, e, _)| (s, e))
    }

    /// Choose the loop's normal exit target from its exit edges: the destination
    /// reached by the most exit edges, ties broken by the EARLIEST node (lowest
    /// reverse-post-order `index`, which the `BlockCopy`s carry verbatim from
    /// `bblocks` — `block_get_start` does not resolve through a cross-arena
    /// `BlockCopy`, so `index` is the reliable deterministic key here).  Mirrors
    /// angr `_refine_cyclic_core`'s successor pick.
    fn choose_normal_exit(&self, exit_edges: &[(BlockId, int4, BlockId)]) -> Option<BlockId> {
        if exit_edges.is_empty() {
            return None;
        }
        let mut counts: std::collections::BTreeMap<BlockId, int4> =
            std::collections::BTreeMap::new();
        for &(_, _, dst) in exit_edges.iter() {
            *counts.entry(dst).or_insert(0) += 1;
        }
        let max = counts.values().copied().max().unwrap_or(0);
        let mut best: Option<BlockId> = None;
        let mut best_idx: int4 = int4::MAX;
        for (&dst, &c) in counts.iter() {
            if c == max {
                let idx = self.graph.block(dst).get_index();
                if idx < best_idx {
                    best_idx = idx;
                    best = Some(dst);
                }
            }
        }
        best
    }

    /// Count the loop head's *extra* entries: in-edges from a node outside the
    /// loop body that are NOT back-edges and NOT already gotos.  A non-zero count
    /// means the loop is multi-entry at the head (irreducible) — these schemas
    /// cannot reduce it and the caller must fall back.  (A normal single-entry
    /// loop has exactly one such edge: the loop preheader; we tolerate up to one.)
    fn head_extra_entries(
        &self,
        head: BlockId,
        in_body: &std::collections::BTreeSet<BlockId>,
    ) -> int4 {
        let hb = self.graph.block(head);
        let mut entries = 0;
        for j in 0..hb.size_in() {
            if hb.is_back_edge_in(j) {
                continue;
            }
            let p = hb.get_in(j);
            if !in_body.contains(&p) && !hb.is_goto_in(j) {
                entries += 1;
            }
        }
        // One preheader entry is normal; more than one is multi-entry/irreducible.
        if entries > 1 {
            entries - 1
        } else {
            0
        }
    }

    /// The loop head's *extra* (abnormal) entry edges to virtualize, KEEPING the
    /// single lowest-index (earliest) non-back-edge predecessor as the structural
    /// preheader and returning every other non-back-edge, non-goto in-edge as a
    /// `(src, out-edge-index)` pair.  Used when an RI loop head is multi-entry: the
    /// kuna analog of angr's abnormal-entries → goto handling, making the loop
    /// single-entry so it can fold.
    fn head_extra_entry_edges(
        &self,
        head: BlockId,
        in_body: &std::collections::BTreeSet<BlockId>,
    ) -> Vec<(BlockId, int4)> {
        // Collect every non-back-edge, non-goto predecessor from outside the body.
        let mut preds: Vec<BlockId> = Vec::new();
        {
            let hb = self.graph.block(head);
            for j in 0..hb.size_in() {
                if hb.is_back_edge_in(j) || hb.is_goto_in(j) {
                    continue;
                }
                let p = hb.get_in(j);
                if !in_body.contains(&p) {
                    preds.push(p);
                }
            }
        }
        if preds.len() <= 1 {
            return Vec::new();
        }
        // Keep the lowest-index predecessor as the structural preheader.
        let keep = preds
            .iter()
            .copied()
            .min_by_key(|&p| self.graph.block(p).get_index());
        let mut res: Vec<(BlockId, int4)> = Vec::new();
        for &p in preds.iter() {
            if Some(p) == keep {
                continue;
            }
            let pb = self.graph.block(p);
            for e in 0..pb.size_out() {
                if pb.get_out(e) == head && !pb.is_goto_out(e) && !pb.is_back_edge_out(e) {
                    res.push((p, e));
                }
            }
        }
        res
    }

    // Note on break/continue: angr's structurer emits `BreakNode`/`ContinueNode`
    // directly (`_rewrite_conditional_jumps_to_breaks`/`_rewrite_jumps_to_continues`).
    // kuna's renderer instead determines break/continue in a SEPARATE, opt-in pass
    // (`kuna_loopbreak_recovery::kuna_scope_break`, gated by the `recover_loop_break`
    // P0 assertion, default-OFF) that runs in `ActionFinalStructure` AFTER
    // structuring.  Setting `f_break_goto`/`f_continue_goto` on a block here (before
    // that pass, and before the loop scope is finalized) mis-renders the loop, so
    // the region structurer marks every virtualized loop edge as a plain goto
    // (`set_goto_branch`); the gated scope-break pass then lowers the loop-exit /
    // loop-back gotos to `break;`/`continue;` exactly as it does for the
    // CollapseStructure path.  This keeps the win (a structured loop with strictly
    // fewer gotos) parity-safe and renderer-correct.

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

/// Outcome of [`RegionStructurer::refine_loop_edges`] on one loop head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopRefineOutcome {
    /// At least one secondary exit / latch / mid-entry edge was virtualized to a
    /// goto — the structurer made progress and should re-run its schema cascade.
    Progressed,
    /// The loop is multi-entry at the head (irreducible); refinement cannot make
    /// it foldable, so the caller leaves it for the virtualize fallback /
    /// `CollapseStructure`.
    Irreducible,
    /// Nothing needed refining for this head (already single-exit/single-latch, or
    /// its body has not yet collapsed enough to expose the secondary edges) — the
    /// caller tries the next head.
    NoChange,
}
