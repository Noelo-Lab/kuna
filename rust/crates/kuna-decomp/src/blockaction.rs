//! Port of `decompiler/cpp/blockaction.{cc,hh}` (W7, item `w7-s7-blockaction`)
//! — the control-flow *structuring* engine: turning the basic-block CFG into a
//! tree of high-level code constructs (if/else, while/do, do/while, switch,
//! goto) by repeated schema-matching collapse.
//!
//! # What this is
//!
//! The structuring algorithm ([`CollapseStructure`]) follows `blockaction.hh`:
//!   - Start with a control-flow graph of basic blocks (the `sblocks`
//!     [`BlockGraph`](crate::block::BlockGraph), a `BlockCopy` mirror of
//!     `bblocks`).
//!   - Repeatedly: search for a sub-graph matching a specific code-structure
//!     element ([`CollapseStructure::collapse_internal`]'s rule cascade) and
//!     collapse the component nodes into a single structured node.
//!   - When the process gets stuck, remove ("goto") an edge chosen by a
//!     directed-acyclic-graph trace ([`TraceDAG`]) restricted to the innermost
//!     loop, marking it unstructured.
//!
//! The **schema-precedence order** in [`CollapseStructure::collapse_internal`]
//! (goto → cat → properIf → ifElse → whileDo → doWhile → infLoop → switch, then
//! the deferred ifNoExit / caseFallthru) is *output-determining*: it decides
//! which gotos get emitted when several structurings are possible, so it is
//! transcribed in exactly the C++ order.  Likewise [`LoopBody`] ordering (by
//! head/tail index, then nesting depth) and the [`TraceDAG`] bad-edge scoring
//! (`BadEdgeScore::operator<` / `compareFinal`) are tie-breakers that decide
//! *which* edge becomes a goto — transcribed verbatim.
//!
//! # Faithfulness — fully ported vs. seam-deferred
//!
//! Everything that is **CFG topology** — [`FloatingEdge`], [`LoopBody`],
//! [`TraceDAG`], and the whole [`CollapseStructure`] collapse loop and its rule
//! cascade — is fully ported and tested; it operates on the
//! [`BlockGraph`](crate::block::BlockGraph) edge/label API that W3 supplies.
//!
//! Two data-flow surfaces that `block.cc` itself left as `// SEAM(W7)` in
//! `block.rs` are threaded here as faithful-topology + seam-noted-dataflow:
//!
//!   - **`FlowBlock::negateCondition`** ([`negate_condition`]): the *edge swap*
//!     (`swapEdges`) is fully ported (it is pure topology and is what determines
//!     the true/false ordering the collapse rules depend on); the **op-flag flip**
//!     on the underlying `BlockBasic`'s CBRANCH (`boolean_flip`/`fallthru_true`)
//!     is the data-flow half.  Standalone (the test engine) only swaps edges;
//!     the [`ActionBlockStructure`] wiring threads the op-flag flip through the
//!     op bank.  The return value (whether a *data-flow* change was made) is the
//!     `dataflow_changecount` driver, transcribed exactly.
//!   - **`FlowBlock::isComplex`** ([`is_complex`]): the C++ base default returns
//!     `true`; `BlockBasic::isComplex` counts statements against
//!     `max_implied_ref` (data-flow).  The port returns the base default (`true`)
//!     and seam-notes the statement count.  This only affects whether a whileDo
//!     uses *overflow syntax* and whether `ruleBlockOr` fires — recorded as a loss.
//!
//! The structuring [`Action`]s whose body is a single call into a `BlockGraph`
//! method that `block.cc` defers to W7/W8 ([`ActionFinalStructure`] →
//! `orderBlocks`/`finalizePrinting`/`scopeBreak`/`markUnstructured`/
//! `markLabelBumpUp`; [`ActionPreferComplement`] → `preferComplement`;
//! [`ActionStructureTransform`] → `finalTransform`; [`ActionNormalizeBranches`]
//! → `flipInPlace`) reproduce the C++ control structure and surface the unported
//! `BlockGraph` method as a seam — recorded as losses.  [`ActionBlockStructure`]
//! (the collapse driver) and [`ActionNodeJoin`]/[`ConditionalJoin`] (split-
//! condition rejoin) are fully ported against the available Funcdata API.

#![allow(clippy::needless_range_loop)] // C++ indexes by edge/path slot; preserve it

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::block::{BlockGraph, BlockKind, BlockType};
use crate::seams::BlockId;

// ===========================================================================
// FloatingEdge (blockaction.hh:31-39, blockaction.cc:27-38)
// ===========================================================================

/// \brief Class for holding an edge while the underlying graph is being
/// manipulated (C++ `FloatingEdge`).
///
/// The original FlowBlock nodes that define the end-points of the edge may get
/// collapsed, but the edge may still exist between higher-level components.  The
/// edge can still be retrieved via [`get_current_edge`](FloatingEdge::get_current_edge).
#[derive(Debug, Clone, Copy)]
pub struct FloatingEdge {
    /// Starting FlowBlock of the edge (C++ `FlowBlock *top`)
    top: BlockId,
    /// Ending FlowBlock of the edge (C++ `FlowBlock *bottom`)
    bottom: BlockId,
}

impl FloatingEdge {
    /// Construct given end points (C++ `FloatingEdge(FlowBlock*,FlowBlock*)`).
    pub fn new(t: BlockId, b: BlockId) -> FloatingEdge {
        FloatingEdge { top: t, bottom: b }
    }

    /// Get the starting FlowBlock (C++ `getTop`).
    pub fn get_top(&self) -> BlockId {
        self.top
    }
    /// Get the ending FlowBlock (C++ `getBottom`).
    pub fn get_bottom(&self) -> BlockId {
        self.bottom
    }

    /// Retrieve the current edge as a \e top FlowBlock and the index of the
    /// outgoing edge (C++ `FloatingEdge::getCurrentEdge`, `blockaction.cc:27`).
    ///
    /// If the end-points have been collapsed together, returns `(None, _)`.  The
    /// `top`/`bottom` nodes are updated to FlowBlocks in the current collapsed
    /// graph (`graph_id` is the containing BlockGraph node).  On success returns
    /// `(Some(top), outedge)` where `outedge` is the outgoing-edge index.
    pub fn get_current_edge(&mut self, graph: &BlockGraph, graph_id: BlockId) -> (Option<BlockId>, int4) {
        while graph.block(self.top).get_parent() != Some(graph_id) {
            // Move up through collapse hierarchy to current graph
            self.top = graph.block(self.top).get_parent().expect("getCurrentEdge: top parent");
        }
        while graph.block(self.bottom).get_parent() != Some(graph_id) {
            self.bottom = graph.block(self.bottom).get_parent().expect("getCurrentEdge: bottom parent");
        }
        let outedge = graph.block(self.top).get_out_index(self.bottom);
        if outedge < 0 {
            return (None, outedge); // Edge does not exist (any longer)
        }
        (Some(self.top), outedge)
    }
}

// ===========================================================================
// LoopBody (blockaction.hh:46-76, blockaction.cc:46-497)
// ===========================================================================

/// \brief A description of the body of a loop (C++ `LoopBody`).
///
/// Following Tarjan, assuming there are no \e irreducible edges, a loop body is
/// defined by the \e head (entry-point) and 1 or more tails, which each have a
/// \e back \e edge into the head.
///
/// Stored in a `Vec` (the C++ `list<LoopBody>`); ordering across LoopBody
/// records (by [`compare_ends`](LoopBody::compare_ends), then by `depth` for the
/// nesting sort) is output-determining and transcribed exactly.  The `head`
/// field is set to `None` when a LoopBody is subsumed by `mergeIdenticalHeads`
/// (the C++ sets `head = (FlowBlock *)0`).
#[derive(Debug, Clone)]
pub struct LoopBody {
    /// head of the loop (C++ `FlowBlock *head`); `None` once subsumed
    head: Option<BlockId>,
    /// (Possibly multiple) nodes with back edge returning to the head
    tails: Vec<BlockId>,
    /// Nested depth of this loop (C++ `int4 depth`)
    depth: int4,
    /// Total number of unique head and tail nodes (C++ `int4 uniquecount`)
    uniquecount: int4,
    /// Official exit block from loop, or `None` (C++ `FlowBlock *exitblock`)
    exitblock: Option<BlockId>,
    /// Edges that exit to the formal exit block (C++ `list<FloatingEdge> exitedges`)
    exitedges: Vec<FloatingEdge>,
    /// Immediately containing loop body index into the looporder vector, or `None`
    /// (C++ `LoopBody *immed_container`).  We use a `Vec` index because the C++
    /// pointer aliases a record in the same `looporder`/`loopbody` collection.
    immed_container: Option<usize>,
}

impl LoopBody {
    /// Construct with a loop head (C++ `LoopBody(FlowBlock *h)`).
    pub fn new(h: BlockId) -> LoopBody {
        LoopBody {
            head: Some(h),
            tails: Vec::new(),
            depth: 0,
            uniquecount: 0,
            exitblock: None,
            exitedges: Vec::new(),
            immed_container: None,
        }
    }

    /// Return the head FlowBlock of the loop (C++ `getHead`).  `None` if subsumed.
    pub fn get_head(&self) -> Option<BlockId> {
        self.head
    }
    /// Add a \e tail to the loop (C++ `addTail`).
    pub fn add_tail(&mut self, bl: BlockId) {
        self.tails.push(bl);
    }
    /// Get the exit FlowBlock or `None` (C++ `getExitBlock`).
    pub fn get_exit_block(&self) -> Option<BlockId> {
        self.exitblock
    }

    /// \brief Find blocks in containing loop that aren't in \b this (C++
    /// `LoopBody::extendToContainer`, `blockaction.cc:46`).
    ///
    /// Assuming \b this has all of its nodes marked, find all additional nodes
    /// that create the body of the `container` loop.  Mark these and put them in
    /// the `body` list.
    fn extend_to_container(&self, container: &LoopBody, body: &mut Vec<BlockId>, graph: &mut BlockGraph) {
        let mut i = 0;
        let container_head = container.head.expect("extendToContainer: container head");
        if !graph.block(container_head).is_mark() {
            // container head may already be in subloop, if not
            graph.block_mut(container_head).set_mark(); // add it to new body
            body.push(container_head);
            i = 1; // make sure we don't traverse back from it
        }
        for j in 0..container.tails.len() {
            let tail = container.tails[j];
            if !graph.block(tail).is_mark() {
                // container tail may already be in subloop, if not
                graph.block_mut(tail).set_mark();
                body.push(tail); // add to body, make sure we DO traverse back from it
            }
        }
        // -this- head is already marked, but hasn't been traversed
        let head = self.head.expect("extendToContainer: head");
        if head != container_head {
            // Unless the container has the same head, traverse the contained head
            let sizein = graph.block(head).size_in();
            for k in 0..sizein {
                if graph.block(head).is_goto_in(k) {
                    continue; // Don't trace back through irreducible edges
                }
                let bl = graph.block(head).get_in(k);
                if graph.block(bl).is_mark() {
                    continue; // Already in list
                }
                graph.block_mut(bl).set_mark();
                body.push(bl);
            }
        }

        while i < body.len() {
            let curblock = body[i];
            i += 1;
            let sizein = graph.block(curblock).size_in();
            for k in 0..sizein {
                if graph.block(curblock).is_goto_in(k) {
                    continue; // Don't trace back through irreducible edges
                }
                let bl = graph.block(curblock).get_in(k);
                if graph.block(bl).is_mark() {
                    continue; // Already in list
                }
                graph.block_mut(bl).set_mark();
                body.push(bl);
            }
        }
    }

    /// Update loop body to current view (C++ `LoopBody::update`, `blockaction.cc:94`).
    ///
    /// Updates the \b head node to the FlowBlock in the current collapsed graph.
    /// The \b tail nodes are also updated until one is found that has not
    /// collapsed into \b head; this first updated \b tail is returned.  Returns
    /// `None` if the loop has been completely collapsed.
    pub fn update(&mut self, graph: &BlockGraph, graph_id: BlockId) -> Option<BlockId> {
        let mut head = self.head.expect("LoopBody::update: head");
        while graph.block(head).get_parent() != Some(graph_id) {
            head = graph.block(head).get_parent().expect("update: head parent");
        }
        self.head = Some(head);
        for i in 0..self.tails.len() {
            let mut bottom = self.tails[i];
            while graph.block(bottom).get_parent() != Some(graph_id) {
                bottom = graph.block(bottom).get_parent().expect("update: tail parent");
            }
            self.tails[i] = bottom;
            if bottom != head {
                // If the loop hasn't been fully collapsed yet
                return Some(bottom);
            }
        }
        let mut i = graph.block(head).size_out() - 1;
        while i >= 0 {
            if graph.block(head).get_out(i) == head {
                // Check for head looping with itself
                return Some(head);
            }
            i -= 1;
        }
        None
    }

    /// Mark the body FlowBlocks of \b this loop (C++ `LoopBody::findBase`,
    /// `blockaction.cc:119`).  Collect all FlowBlock nodes that reach a \b tail
    /// without going through \b head, into `body`, and mark them.
    pub fn find_base(&mut self, body: &mut Vec<BlockId>, graph: &mut BlockGraph) {
        let head = self.head.expect("findBase: head");
        graph.block_mut(head).set_mark();
        body.push(head);
        for j in 0..self.tails.len() {
            let tail = self.tails[j];
            if !graph.block(tail).is_mark() {
                graph.block_mut(tail).set_mark();
                body.push(tail);
            }
        }
        self.uniquecount = body.len() as int4; // Number of nodes that either head or tail
        let mut i = 1;
        while i < body.len() {
            let curblock = body[i];
            i += 1;
            let sizein = graph.block(curblock).size_in();
            for k in 0..sizein {
                if graph.block(curblock).is_goto_in(k) {
                    continue; // Don't trace back through irreducible edges
                }
                let bl = graph.block(curblock).get_in(k);
                if graph.block(bl).is_mark() {
                    continue; // Already in list
                }
                graph.block_mut(bl).set_mark();
                body.push(bl);
            }
        }
    }

    /// Extend body (to blocks that never exit) (C++ `LoopBody::extend`,
    /// `blockaction.cc:150`).  Extend the \b body to every FlowBlock reachable
    /// \b only from \b head without hitting the \b exitblock.
    pub fn extend(&self, body: &mut Vec<BlockId>, graph: &mut BlockGraph) {
        let mut trial: Vec<BlockId> = Vec::new();
        let mut i = 0;
        while i < body.len() {
            let bl = body[i];
            i += 1;
            let sizeout = graph.block(bl).size_out();
            for j in 0..sizeout {
                if graph.block(bl).is_goto_out(j) {
                    continue; // Don't extend through goto edge
                }
                let curbl = graph.block(bl).get_out(j);
                if graph.block(curbl).is_mark() {
                    continue;
                }
                if Some(curbl) == self.exitblock {
                    continue;
                }
                let mut count = graph.block(curbl).get_visit_count();
                if count == 0 {
                    trial.push(curbl); // New possible extension
                }
                count += 1;
                graph.block_mut(curbl).set_visit_count(count);
                if count == graph.block(curbl).size_in() {
                    graph.block_mut(curbl).set_mark();
                    body.push(curbl);
                }
            }
        }
        for i in 0..trial.len() {
            graph.block_mut(trial[i]).set_visit_count(0); // Make sure to clear the count
        }
    }

    /// Choose the exit block for \b this loop (C++ `LoopBody::findExit`,
    /// `blockaction.cc:182`).  A structured loop is allowed at most one exit
    /// block; pick it.  `looporder` is needed only to resolve `immed_container`.
    pub fn find_exit(&mut self, body: &[BlockId], graph: &mut BlockGraph, looporder: &[LoopBody]) {
        let mut trialexit: Vec<BlockId> = Vec::new();

        for j in 0..self.tails.len() {
            let tail = self.tails[j];
            let sizeout = graph.block(tail).size_out();
            for i in 0..sizeout {
                if graph.block(tail).is_goto_out(i) {
                    continue; // Don't use goto as exit edge
                }
                let curbl = graph.block(tail).get_out(i);
                if !graph.block(curbl).is_mark() {
                    if self.immed_container.is_none() {
                        self.exitblock = Some(curbl);
                        return;
                    }
                    trialexit.push(curbl);
                }
            }
        }

        for i in 0..body.len() {
            let bl = body[i];
            if (i > 0) && ((i as int4) < self.uniquecount) {
                continue; // Filter out tails (processed previously)
            }
            let sizeout = graph.block(bl).size_out();
            for j in 0..sizeout {
                if graph.block(bl).is_goto_out(j) {
                    continue; // Don't use goto as exit edge
                }
                let curbl = graph.block(bl).get_out(j);
                if !graph.block(curbl).is_mark() {
                    if self.immed_container.is_none() {
                        self.exitblock = Some(curbl);
                        return;
                    }
                    trialexit.push(curbl);
                }
            }
        }

        self.exitblock = None; // Default exit is null, if no block meeting condition can be found
        if trialexit.is_empty() {
            return;
        }

        // If there is a containing loop, force exitblock to be in the containing loop
        if let Some(container_idx) = self.immed_container {
            let mut extension: Vec<BlockId> = Vec::new();
            // C++ aliases *immed_container; here it is a record in looporder.
            self.extend_to_container(&looporder[container_idx], &mut extension, graph);
            for i in 0..trialexit.len() {
                let bl = trialexit[i];
                if graph.block(bl).is_mark() {
                    self.exitblock = Some(bl);
                    break;
                }
            }
            LoopBody::clear_marks(&extension, graph);
        }
    }

    /// Find preferred \b tail (C++ `LoopBody::orderTails`, `blockaction.cc:245`).
    /// If there is a single \b tail that has an outgoing edge to the \b exitblock,
    /// make sure it is the first tail.
    pub fn order_tails(&mut self, graph: &BlockGraph) {
        if self.tails.len() <= 1 {
            return;
        }
        let exitblock = match self.exitblock {
            Some(e) => e,
            None => return,
        };
        let mut prefindex = 0;
        let mut trial = self.tails[0];
        while prefindex < self.tails.len() {
            trial = self.tails[prefindex];
            let sizeout = graph.block(trial).size_out();
            let mut j = 0;
            while j < sizeout {
                if graph.block(trial).get_out(j) == exitblock {
                    break;
                }
                j += 1;
            }
            if j < sizeout {
                break;
            }
            prefindex += 1;
        }
        if prefindex >= self.tails.len() {
            return;
        }
        if prefindex == 0 {
            return;
        }
        self.tails[prefindex] = self.tails[0]; // Swap preferred tail into the first position
        self.tails[0] = trial;
    }

    /// Label edges that exit the loop (C++ `LoopBody::labelExitEdges`,
    /// `blockaction.cc:270`).  Put the edges in priority for removal: middle exit
    /// at front, \e head exit, then \e tail exit.
    pub fn label_exit_edges(&mut self, body: &[BlockId], graph: &BlockGraph) {
        let mut toexitblock: Vec<BlockId> = Vec::new();
        let mut i = self.uniquecount as usize;
        while i < body.len() {
            // For non-head/tail nodes of graph
            let curblock = body[i];
            i += 1;
            let sizeout = graph.block(curblock).size_out();
            for k in 0..sizeout {
                if graph.block(curblock).is_goto_out(k) {
                    continue; // Don't exit through goto edges
                }
                let bl = graph.block(curblock).get_out(k);
                if Some(bl) == self.exitblock {
                    toexitblock.push(curblock);
                    continue; // Postpone exit to exitblock
                }
                if !graph.block(bl).is_mark() {
                    self.exitedges.push(FloatingEdge::new(curblock, bl));
                }
            }
        }
        if let Some(head) = self.head {
            let sizeout = graph.block(head).size_out();
            for k in 0..sizeout {
                if graph.block(head).is_goto_out(k) {
                    continue; // Don't exit through goto edges
                }
                let bl = graph.block(head).get_out(k);
                if Some(bl) == self.exitblock {
                    toexitblock.push(head);
                    continue; // Postpone exit to exitblock
                }
                if !graph.block(bl).is_mark() {
                    self.exitedges.push(FloatingEdge::new(head, bl));
                }
            }
        }
        let mut i = self.tails.len() as int4 - 1;
        while i >= 0 {
            // Put exits from more preferred tails later
            let curblock = self.tails[i as usize];
            i -= 1;
            if Some(curblock) == self.head {
                continue;
            }
            let sizeout = graph.block(curblock).size_out();
            for k in 0..sizeout {
                if graph.block(curblock).is_goto_out(k) {
                    continue; // Don't exit through goto edges
                }
                let bl = graph.block(curblock).get_out(k);
                if Some(bl) == self.exitblock {
                    toexitblock.push(curblock);
                    continue; // Postpone exit to exitblock
                }
                if !graph.block(bl).is_mark() {
                    self.exitedges.push(FloatingEdge::new(curblock, bl));
                }
            }
        }
        for i in 0..toexitblock.len() {
            // Now we do exits to exitblock
            let bl = toexitblock[i];
            self.exitedges
                .push(FloatingEdge::new(bl, self.exitblock.expect("labelExitEdges: exitblock")));
        }
    }

    /// Record any loops that \b body contains (C++ `LoopBody::labelContainments`,
    /// `blockaction.cc:327`).  Searches for any loop contained by \b this and
    /// updates its `depth` and `immed_container` field.
    ///
    /// `self_idx` is the index of \b this within `looporder` (the C++ uses the
    /// `this` pointer; here we pass the index because `immed_container` is an
    /// index into the same vector).  Operates on the full `looporder` slice in
    /// place; mutates `depth`/`immed_container` of contained loops.
    pub fn label_containments(
        body: &[BlockId],
        looporder_indices: &[usize],
        loopbody: &mut [LoopBody],
        self_idx: usize,
        graph: &BlockGraph,
        looporder_lookup: &[(int4, usize)],
    ) {
        let mut containlist: Vec<usize> = Vec::new();

        let head = loopbody[self_idx].head;
        for i in 0..body.len() {
            let curblock = body[i];
            if Some(curblock) != head {
                if let Some(subloop) = LoopBody::find(curblock, looporder_lookup, graph) {
                    // subloop is an index into loopbody (the looporder record).
                    containlist.push(subloop);
                    loopbody[subloop].depth += 1;
                }
            }
        }
        // See C++ comment: depth ordering means immed_container is settled here.
        let depth = loopbody[self_idx].depth;
        for i in 0..containlist.len() {
            let lb = containlist[i];
            let take = match loopbody[lb].immed_container {
                None => true,
                Some(ic) => loopbody[ic].depth < depth,
            };
            if take {
                loopbody[lb].immed_container = Some(self_idx);
            }
        }
        let _ = looporder_indices;
    }

    /// Collect likely \e unstructured edges (C++ `LoopBody::emitLikelyEdges`,
    /// `blockaction.cc:364`).  Add edges that exit from \b this loop body to the
    /// list of likely \e gotos, giving them the proper priority (`likely` holds
    /// the exit edges in reverse priority order).
    pub fn emit_likely_edges(&mut self, likely: &mut Vec<FloatingEdge>, graph: &BlockGraph, graph_id: BlockId) {
        let mut head = self.head.expect("emitLikelyEdges: head");
        while graph.block(head).get_parent() != Some(graph_id) {
            head = graph.block(head).get_parent().expect("emitLikelyEdges: head parent");
        }
        self.head = Some(head);
        if let Some(mut exitblock) = self.exitblock {
            while graph.block(exitblock).get_parent() != Some(graph_id) {
                exitblock = graph.block(exitblock).get_parent().expect("emitLikelyEdges: exit parent");
            }
            self.exitblock = Some(exitblock);
        }
        for i in 0..self.tails.len() {
            let mut tail = self.tails[i];
            while graph.block(tail).get_parent() != Some(graph_id) {
                tail = graph.block(tail).get_parent().expect("emitLikelyEdges: tail parent");
            }
            self.tails[i] = tail;
            if Some(tail) == self.exitblock {
                // If the exitblock was collapsed into the tail, we no longer really have an exit
                self.exitblock = None;
            }
        }
        let enditer = self.exitedges.len();
        let mut iter = 0;
        let mut holdin: Option<BlockId> = None;
        let mut holdout: Option<BlockId> = None;
        while iter != enditer {
            let (inbl, outedge) = self.exitedges[iter].get_current_edge(graph, graph_id);
            iter += 1;
            let inbl = match inbl {
                Some(b) => b,
                None => continue,
            };
            let outbl = graph.block(inbl).get_out(outedge);
            if iter == enditer && Some(outbl) == self.exitblock {
                // If this is the official exit edge
                holdin = Some(inbl); // Hold off putting the edge in list
                holdout = Some(outbl);
                break;
            }
            likely.push(FloatingEdge::new(inbl, outbl));
        }
        let mut i = self.tails.len() as int4 - 1;
        while i >= 0 {
            // Go in reverse order, to put out less preferred back-edges first
            if let (Some(hin), Some(hout)) = (holdin, holdout) {
                if i == 0 {
                    likely.push(FloatingEdge::new(hin, hout));
                }
            }
            let tail = self.tails[i as usize];
            let sizeout = graph.block(tail).size_out();
            for j in 0..sizeout {
                let bl = graph.block(tail).get_out(j);
                if Some(bl) == self.head {
                    // If out edge to head (back-edge for this loop)
                    likely.push(FloatingEdge::new(tail, head));
                }
            }
            i -= 1;
        }
    }

    /// Mark all the exits to this loop (C++ `LoopBody::setExitMarks`,
    /// `blockaction.cc:416`).  Exit edges have their `f_loop_exit_edge` set.
    pub fn set_exit_marks(&mut self, graph: &mut BlockGraph, graph_id: BlockId) {
        for idx in 0..self.exitedges.len() {
            let (inloop, outedge) = self.exitedges[idx].get_current_edge(graph, graph_id);
            if let Some(inloop) = inloop {
                graph.set_loop_exit(inloop, outedge);
            }
        }
    }

    /// Clear the mark on all the exits to this loop (C++ `LoopBody::clearExitMarks`,
    /// `blockaction.cc:430`).
    pub fn clear_exit_marks(&mut self, graph: &mut BlockGraph, graph_id: BlockId) {
        for idx in 0..self.exitedges.len() {
            let (inloop, outedge) = self.exitedges[idx].get_current_edge(graph, graph_id);
            if let Some(inloop) = inloop {
                graph.clear_loop_exit(inloop, outedge);
            }
        }
    }

    /// Merge loop bodies that share the same \e head (C++
    /// `LoopBody::mergeIdenticalHeads`, `blockaction.cc:446`).  Merge each \b tail
    /// from one into the other and set the merged LoopBody \b head to `None`.
    /// Operates on `looporder` (a list of indices into `loopbody`); resizes the
    /// `looporder` index list in place.
    pub fn merge_identical_heads(looporder: &mut Vec<usize>, loopbody: &mut [LoopBody]) {
        let mut i = 0;
        let mut j = i + 1;

        let mut curbody = looporder[i];
        while j < looporder.len() {
            let nextbody = looporder[j];
            j += 1;
            if loopbody[nextbody].head == loopbody[curbody].head {
                let tail0 = loopbody[nextbody].tails[0];
                loopbody[curbody].add_tail(tail0);
                loopbody[nextbody].head = None; // Mark this LoopBody as subsumed
            } else {
                i += 1;
                looporder[i] = nextbody;
                curbody = nextbody;
            }
        }
        i += 1; // Total size of merged array
        looporder.truncate(i);
    }

    /// Compare the \b head then \b tail (C++ `LoopBody::compare_ends`,
    /// `blockaction.cc:473`).  Returns `true` if `a` comes before `b`.
    pub fn compare_ends(a: &LoopBody, b: &LoopBody, graph: &BlockGraph) -> std::cmp::Ordering {
        let aindex = graph.block(a.head.expect("compare_ends a")).get_index();
        let bindex = graph.block(b.head.expect("compare_ends b")).get_index();
        if aindex != bindex {
            return aindex.cmp(&bindex);
        }
        let aindex = graph.block(a.tails[0]).get_index(); // Only compare the first tail
        let bindex = graph.block(b.tails[0]).get_index();
        aindex.cmp(&bindex)
    }

    /// Compare just the \b head (C++ `LoopBody::compare_head`, `blockaction.cc:489`).
    /// Returns -1, 0, or 1.
    pub fn compare_head(a_head_index: int4, looptop_index: int4) -> int4 {
        if a_head_index != looptop_index {
            if a_head_index < looptop_index {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    /// Find a LoopBody (C++ `LoopBody::find`, `blockaction.cc:1021`).  Given the
    /// top FlowBlock of a loop, find the corresponding LoopBody index from the
    /// ordered lookup `(head_index, loopbody_index)` pairs (sorted by head index;
    /// the C++ binary-searches the sorted `looporder` vector).  Returns the
    /// `loopbody` index or `None`.
    pub fn find(looptop: BlockId, looporder_lookup: &[(int4, usize)], graph: &BlockGraph) -> Option<usize> {
        let looptop_index = graph.block(looptop).get_index();
        let mut min: int4 = 0;
        let mut max: int4 = looporder_lookup.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let comp = LoopBody::compare_head(looporder_lookup[mid as usize].0, looptop_index);
            if comp == 0 {
                return Some(looporder_lookup[mid as usize].1);
            }
            if comp < 0 {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }
        None
    }

    /// Clear the body marks (C++ `LoopBody::clearMarks`, `blockaction.cc:1039`).
    pub fn clear_marks(body: &[BlockId], graph: &mut BlockGraph) {
        for i in 0..body.len() {
            graph.block_mut(body[i]).clear_mark();
        }
    }
}

// ===========================================================================
// TraceDAG (blockaction.hh:96-181, blockaction.cc:499-1014)
// ===========================================================================

/// f_active: this BlockTrace is \e active (C++ `BlockTrace::f_active`).
const F_ACTIVE: uint4 = 1;
/// f_terminal: all paths from this point exit (C++ `BlockTrace::f_terminal`).
const F_TERMINAL: uint4 = 2;

/// A node in the control-flow graph with multiple outgoing edges in the DAG
/// (C++ `TraceDAG::BranchPoint`).  Identified by its arena index in
/// [`TraceDAG::branchlist`]; child traces are indices into
/// [`TraceDAG::tracelist`].
#[derive(Debug, Clone)]
struct BranchPoint {
    /// The parent BranchPoint index along which \b this is only one path; `None`
    /// for the virtual root (C++ `BranchPoint *parent`)
    parent: Option<usize>,
    /// Index (of the out edge from the parent) of the path along which \b this
    /// lies (C++ `int4 pathout`)
    pathout: int4,
    /// FlowBlock that embodies the branch point; `None` for the virtual root
    /// (C++ `FlowBlock *top`)
    top: Option<BlockId>,
    /// BlockTrace indices for each possible path out of \b this BranchPoint
    /// (C++ `vector<BlockTrace *> paths`)
    paths: Vec<usize>,
    /// Depth of BranchPoints from the root (C++ `int4 depth`)
    depth: int4,
    /// Possible mark (C++ `bool ismark`)
    ismark: bool,
}

/// A trace of a single path out of a BranchPoint (C++ `TraceDAG::BlockTrace`).
/// Identified by its arena index in [`TraceDAG::tracelist`].
#[derive(Debug, Clone)]
struct BlockTrace {
    /// Properties of the BlockTrace (C++ `uint4 flags`)
    flags: uint4,
    /// Parent BranchPoint index for which this is a path (C++ `BranchPoint *top`)
    top: usize,
    /// Index of the out-edge for this path relative to the parent BranchPoint
    /// (C++ `int4 pathout`)
    pathout: int4,
    /// Current node being traversed along 1 path from the decision point; `None`
    /// for a virtual-root child before its first push, or once terminal
    /// (C++ `FlowBlock *bottom`)
    bottom: Option<BlockId>,
    /// Next FlowBlock node \b this BlockTrace will try to push into; `None` once
    /// terminal (C++ `FlowBlock *destnode`)
    destnode: Option<BlockId>,
    /// If >1, edge to \b destnode is "virtual" representing multiple edges coming
    /// together (C++ `int4 edgelump`)
    edgelump: int4,
    /// BranchPoint blocker \b this traces into; `None` if none
    /// (C++ `BranchPoint *derivedbp`)
    derivedbp: Option<usize>,
}

impl BlockTrace {
    /// Return `true` if \b this is active (C++ `isActive`).
    fn is_active(&self) -> bool {
        (self.flags & F_ACTIVE) != 0
    }
    /// Return `true` if \b this terminates (C++ `isTerminal`).
    fn is_terminal(&self) -> bool {
        (self.flags & F_TERMINAL) != 0
    }
}

/// \brief Record for scoring a BlockTrace for suitability as an unstructured
/// branch (C++ `TraceDAG::BadEdgeScore`).
#[derive(Debug, Clone)]
struct BadEdgeScore {
    /// Putative exit block for the BlockTrace (C++ `FlowBlock *exitproto`)
    exitproto: BlockId,
    /// The active BlockTrace being considered (index into tracelist)
    trace: usize,
    /// Minimum distance crossed by \b this and any other BlockTrace sharing the
    /// same exit block (C++ `int4 distance`)
    distance: int4,
    /// 1 if BlockTrace destination has no exit, 0 otherwise (C++ `int4 terminal`)
    terminal: int4,
    /// Number of active BlockTraces with same BranchPoint and exit as \b this
    /// (C++ `int4 siblingedge`)
    siblingedge: int4,
}

/// \brief Algorithm for selecting unstructured edges based on Directed Acyclic
/// Graphs (DAG) (C++ `TraceDAG`).
///
/// Traces edges with single-entry/at-most-one-exit structure from given root
/// points, retiring [`BranchPoint`]s as their paths merge, and on a stall
/// selecting the least-structurable edge ([`select_bad_edge`](TraceDAG::select_bad_edge))
/// as a likely goto.  Produces the `likelygoto` list used whenever
/// [`CollapseStructure`] gets stuck.
struct TraceDAG<'a> {
    /// A reference to the list of likely goto edges being produced
    likelygoto: &'a mut Vec<FloatingEdge>,
    /// List of root FlowBlocks to trace from (C++ `vector<FlowBlock *> rootlist`)
    rootlist: Vec<BlockId>,
    /// Current set of BranchPoints that have been traced (the C++ `branchlist`
    /// owns them; here they live in this arena indexed by `usize`)
    branchlist: Vec<BranchPoint>,
    /// All BlockTrace objects ever created (the C++ `BranchPoint` owns its
    /// BlockTraces; here they live in this arena indexed by `usize`)
    tracelist: Vec<BlockTrace>,
    /// Number of active BlockTrace objects (C++ `int4 activecount`)
    activecount: int4,
    /// Current number of active BlockTraces that can't be pushed further
    missedactivecount: int4,
    /// The list of \e active BlockTrace objects, in order (C++
    /// `list<BlockTrace *> activetrace`).  Holds tracelist indices; the C++
    /// `activeiter` is realized by searching this list (the trace stores no
    /// iterator — see [`remove_active`]).
    activetrace: Vec<usize>,
    /// Position of the current \e active BlockTrace being pushed (index into
    /// `activetrace`) (C++ `list<...>::iterator current_activeiter`)
    current_activeiter: usize,
    /// Designated exit block for the DAG (or `None`) (C++ `FlowBlock *finishblock`)
    finishblock: Option<BlockId>,
}

impl<'a> TraceDAG<'a> {
    /// Construct given the container for likely unstructured edges (C++
    /// `TraceDAG::TraceDAG`, `blockaction.cc:951`).
    fn new(lg: &'a mut Vec<FloatingEdge>) -> TraceDAG<'a> {
        TraceDAG {
            likelygoto: lg,
            rootlist: Vec::new(),
            branchlist: Vec::new(),
            tracelist: Vec::new(),
            activecount: 0,
            missedactivecount: 0,
            activetrace: Vec::new(),
            current_activeiter: 0,
            finishblock: None,
        }
    }

    /// Add a root FlowBlock to the trace (C++ `addRoot`).
    fn add_root(&mut self, root: BlockId) {
        self.rootlist.push(root);
    }
    /// Mark an exit point not to trace beyond (C++ `setFinishBlock`).
    fn set_finish_block(&mut self, bl: BlockId) {
        self.finishblock = Some(bl);
    }

    // --- BranchPoint helpers (members of BranchPoint in C++) ---------------

    /// Given the BlockTrace objects, create them for a new BranchPoint (C++
    /// `BranchPoint::createTraces`, `blockaction.cc:499`).  `bp` is the index of
    /// the branch point being populated.
    fn create_traces(&mut self, bp: usize, graph: &BlockGraph) {
        let top = self.branchlist[bp].top.expect("createTraces: top");
        let sizeout = graph.block(top).size_out();
        for i in 0..sizeout {
            if !graph.block(top).is_loop_dag_out(i) {
                continue;
            }
            let po = self.branchlist[bp].paths.len() as int4;
            // BlockTrace(this,paths.size(),i): bottom = top->top; destnode = bottom->getOut(eo)
            let bottom = top;
            let destnode = graph.block(bottom).get_out(i);
            let trace = BlockTrace {
                flags: 0,
                top: bp,
                pathout: po,
                bottom: Some(bottom),
                destnode: Some(destnode),
                edgelump: 1,
                derivedbp: None,
            };
            let tidx = self.tracelist.len();
            self.tracelist.push(trace);
            self.branchlist[bp].paths.push(tidx);
        }
    }

    /// Mark a path from `bp` up to the root BranchPoint (C++ `BranchPoint::markPath`,
    /// `blockaction.cc:509`).
    fn mark_path(&mut self, bp: usize) {
        let mut cur = Some(bp);
        while let Some(c) = cur {
            self.branchlist[c].ismark = !self.branchlist[c].ismark;
            cur = self.branchlist[c].parent;
        }
    }

    /// Calculate distance between two BranchPoints (C++ `BranchPoint::distance`,
    /// `blockaction.cc:524`).  Assumes `bp` has had its path up to the root marked.
    fn distance(&self, bp: usize, op2: usize) -> int4 {
        // find the common ancestor
        let mut cur = Some(op2);
        let depth = self.branchlist[bp].depth;
        let op2_depth = self.branchlist[op2].depth;
        while let Some(c) = cur {
            if self.branchlist[c].ismark {
                // Found the common ancestor
                return (depth - self.branchlist[c].depth) + (op2_depth - self.branchlist[c].depth);
            }
            cur = self.branchlist[c].parent;
        }
        depth + op2_depth + 1
    }

    /// Create a child BranchPoint from a parent BlockTrace (C++
    /// `BranchPoint::BranchPoint(BlockTrace *parenttrace)`, `blockaction.cc:565`).
    /// Returns the new branch point index.
    fn new_branch_point_from_trace(&mut self, parenttrace: usize, graph: &BlockGraph) -> usize {
        let parent = self.tracelist[parenttrace].top;
        let depth = self.branchlist[parent].depth + 1;
        let pathout = self.tracelist[parenttrace].pathout;
        let top = self.tracelist[parenttrace].destnode;
        let bp = BranchPoint {
            parent: Some(parent),
            depth,
            pathout,
            ismark: false,
            top,
            paths: Vec::new(),
        };
        let bpidx = self.branchlist.len();
        self.branchlist.push(bp);
        self.create_traces(bpidx, graph);
        bpidx
    }

    // --- BadEdgeScore comparators (C++ BadEdgeScore) -----------------------

    /// Compare BadEdgeScore for unstructured suitability (C++
    /// `BadEdgeScore::compareFinal`, `blockaction.cc:617`).  Returns `true` if
    /// \b this is LESS likely to be the bad edge than `op2`.
    fn compare_final(&self, this: &BadEdgeScore, op2: &BadEdgeScore) -> bool {
        if this.siblingedge != op2.siblingedge {
            return op2.siblingedge < this.siblingedge; // bigger sibling edge -> less likely
        }
        if this.terminal != op2.terminal {
            return this.terminal < op2.terminal;
        }
        if this.distance != op2.distance {
            return this.distance < op2.distance; // Less distance -> less likely
        }
        // Less depth means less likely to be bad
        let this_depth = self.branchlist[self.tracelist[this.trace].top].depth;
        let op2_depth = self.branchlist[self.tracelist[op2.trace].top].depth;
        this_depth < op2_depth
    }

    /// Comparator for grouping BlockTraces with the same exit block and parent
    /// BranchPoint (C++ `BadEdgeScore::operator<`, `blockaction.cc:635`).
    fn bad_edge_less(&self, this: &BadEdgeScore, op2: &BadEdgeScore, graph: &BlockGraph) -> std::cmp::Ordering {
        let thisind = graph.block(this.exitproto).get_index();
        let op2ind = graph.block(op2.exitproto).get_index();
        if thisind != op2ind {
            return thisind.cmp(&op2ind); // Sort on exit block being traced to
        }
        let tmpbl = self.branchlist[self.tracelist[this.trace].top].top;
        let thisind = tmpbl.map(|b| graph.block(b).get_index()).unwrap_or(-1);
        let tmpbl = self.branchlist[self.tracelist[op2.trace].top].top;
        let op2ind = tmpbl.map(|b| graph.block(b).get_index()).unwrap_or(-1);
        if thisind != op2ind {
            return thisind.cmp(&op2ind); // sort on branch point being traced from
        }
        let thisind = self.tracelist[this.trace].pathout;
        let op2ind = self.tracelist[op2.trace].pathout; // Then on the branch being taken
        thisind.cmp(&op2ind)
    }

    // --- TraceDAG members --------------------------------------------------

    /// Remove the indicated BlockTrace (C++ `TraceDAG::removeTrace`,
    /// `blockaction.cc:656`).  Records the edge as a likely goto and patches the
    /// BranchPoint/BlockTrace/pathout hierarchy.
    fn remove_trace(&mut self, trace: usize, graph: &mut BlockGraph) {
        // Record that we should now treat this edge like goto
        let bottom = self.tracelist[trace].bottom.expect("removeTrace: bottom");
        let destnode = self.tracelist[trace].destnode.expect("removeTrace: destnode");
        self.likelygoto.push(FloatingEdge::new(bottom, destnode)); // Create goto record
        let newcount = graph.block(destnode).get_visit_count() + self.tracelist[trace].edgelump;
        graph.block_mut(destnode).set_visit_count(newcount); // Ignore edge(s)

        let parentbp = self.tracelist[trace].top;

        if self.tracelist[trace].bottom != self.branchlist[parentbp].top {
            // If trace has moved past the root branch, we can treat trace as terminal
            self.tracelist[trace].flags |= F_TERMINAL;
            self.tracelist[trace].bottom = None;
            self.tracelist[trace].destnode = None;
            self.tracelist[trace].edgelump = 0;
            // Do NOT remove from active list
            return;
        }
        // Otherwise actually remove the path from the BranchPoint
        self.remove_active(trace);
        let size = self.branchlist[parentbp].paths.len();
        let trace_pathout = self.tracelist[trace].pathout;
        for i in (trace_pathout + 1) as usize..size {
            // Move every trace above -trace-'s pathout down one slot
            let movedtrace = self.branchlist[parentbp].paths[i];
            self.tracelist[movedtrace].pathout -= 1; // Correct the trace's pathout
            if let Some(derivedbp) = self.tracelist[movedtrace].derivedbp {
                self.branchlist[derivedbp].pathout -= 1; // Correct derived BranchPoint's pathout
            }
            self.branchlist[parentbp].paths[i - 1] = movedtrace;
        }
        self.branchlist[parentbp].paths.pop(); // Remove the vacated slot
        // The C++ deletes the trace record; our arena keeps it (no longer referenced).
    }

    /// Process a set of conflicting BlockTrace objects that go to the same exit
    /// point (C++ `TraceDAG::processExitConflict`, `blockaction.cc:694`).
    /// `scores` is the working list; `start`/`end` are indices into it.
    fn process_exit_conflict(&mut self, scores: &mut [BadEdgeScore], start: usize, end: usize) {
        let mut start = start;
        while start != end {
            let mut iter = start + 1;
            let startbp = self.tracelist[scores[start].trace].top;
            if iter != end {
                self.mark_path(startbp); // Mark path to root, find common ancestors easily
                loop {
                    if startbp == self.tracelist[scores[iter].trace].top {
                        // Edge coming from same BranchPoint
                        scores[start].siblingedge += 1;
                        scores[iter].siblingedge += 1;
                    }
                    let dist = self.distance(startbp, self.tracelist[scores[iter].trace].top);
                    // Distance is symmetric; update minimum for both traces
                    if scores[start].distance == -1 || scores[start].distance > dist {
                        scores[start].distance = dist;
                    }
                    if scores[iter].distance == -1 || scores[iter].distance > dist {
                        scores[iter].distance = dist;
                    }
                    iter += 1;
                    if iter == end {
                        break;
                    }
                }
                self.mark_path(startbp); // Unmark the path
            }
            start += 1;
        }
    }

    /// Select the most likely unstructured edge from active BlockTraces (C++
    /// `TraceDAG::selectBadEdge`, `blockaction.cc:730`).  Returns the tracelist
    /// index of the chosen BlockTrace.
    fn select_bad_edge(&mut self, graph: &BlockGraph) -> usize {
        let mut badedgelist: Vec<BadEdgeScore> = Vec::new();
        for &aiter in &self.activetrace {
            if self.tracelist[aiter].is_terminal() {
                continue;
            }
            let top_top = self.branchlist[self.tracelist[aiter].top].top;
            if top_top.is_none() && self.tracelist[aiter].bottom.is_none() {
                continue; // Never remove virtual edges
            }
            let destnode = self.tracelist[aiter].destnode.expect("selectBadEdge: destnode");
            let terminal = if graph.block(destnode).size_out() == 0 { 1 } else { 0 };
            badedgelist.push(BadEdgeScore {
                trace: aiter,
                exitproto: destnode,
                distance: -1,
                siblingedge: 0,
                terminal,
            });
        }
        // badedgelist.sort();
        badedgelist.sort_by(|a, b| self.bad_edge_less(a, b, graph));

        // Find traces to the same exitblock and process conflicts
        let mut startidx = 0usize;
        let mut curbl = badedgelist[0].exitproto;
        let mut samenodecount = 1;
        let mut iter = 1usize;
        while iter != badedgelist.len() {
            if curbl == badedgelist[iter].exitproto {
                samenodecount += 1; // Count another trace to the same exit
                iter += 1;
            } else {
                // A new exit node
                if samenodecount > 1 {
                    self.process_exit_conflict(&mut badedgelist, startidx, iter);
                }
                curbl = badedgelist[iter].exitproto;
                startidx = iter;
                samenodecount = 1;
                iter += 1;
            }
        }
        if samenodecount > 1 {
            // Process possible final group of traces exiting to same block
            self.process_exit_conflict(&mut badedgelist, startidx, iter);
        }

        let mut maxidx = 0usize;
        let mut iter = 1usize;
        while iter != badedgelist.len() {
            if self.compare_final(&badedgelist[maxidx], &badedgelist[iter]) {
                maxidx = iter;
            }
            iter += 1;
        }
        badedgelist[maxidx].trace
    }

    /// Move a BlockTrace into the \e active category (C++ `TraceDAG::insertActive`,
    /// `blockaction.cc:786`).
    fn insert_active(&mut self, trace: usize) {
        self.activetrace.push(trace);
        self.tracelist[trace].flags |= F_ACTIVE;
        self.activecount += 1;
    }

    /// Remove a BlockTrace from the \e active category (C++
    /// `TraceDAG::removeActive`, `blockaction.cc:798`).
    fn remove_active(&mut self, trace: usize) {
        if let Some(pos) = self.activetrace.iter().position(|&t| t == trace) {
            self.activetrace.remove(pos);
        }
        self.tracelist[trace].flags &= !F_ACTIVE;
        self.activecount -= 1;
    }

    /// Check if we can push the given BlockTrace into its next node (C++
    /// `TraceDAG::checkOpen`, `blockaction.cc:810`).
    fn check_open(&self, trace: usize, graph: &BlockGraph) -> bool {
        if self.tracelist[trace].is_terminal() {
            return false; // Already been opened
        }
        let mut isroot = false;
        if self.branchlist[self.tracelist[trace].top].depth == 0 {
            if self.tracelist[trace].bottom.is_none() {
                return true; // Artificial root can always open its first level
            }
            isroot = true;
        }

        let bl = self.tracelist[trace].destnode.expect("checkOpen: destnode");
        if Some(bl) == self.finishblock && !isroot {
            return false; // Only the root can open the designated exit
        }
        let ignore = self.tracelist[trace].edgelump + graph.block(bl).get_visit_count();
        let mut count = 0;
        for i in 0..graph.block(bl).size_in() {
            if graph.block(bl).is_loop_dag_in(i) {
                count += 1;
                if count > ignore {
                    return false;
                }
            }
        }
        true
    }

    /// Open a new BranchPoint along a given BlockTrace (C++ `TraceDAG::openBranch`,
    /// `blockaction.cc:839`).  Returns the new `current_activeiter` position (index
    /// into `activetrace`).
    fn open_branch(&mut self, parent: usize, graph: &BlockGraph) -> usize {
        let newbranch = self.new_branch_point_from_trace(parent, graph);
        self.tracelist[parent].derivedbp = Some(newbranch);
        if self.branchlist[newbranch].paths.is_empty() {
            // No new traces, return immediately to parent trace (mark terminal)
            self.tracelist[parent].derivedbp = None;
            self.tracelist[parent].flags |= F_TERMINAL;
            self.tracelist[parent].bottom = None;
            self.tracelist[parent].destnode = None;
            self.tracelist[parent].edgelump = 0;
            // The C++ deletes the empty newbranch; our arena keeps it unreferenced.
            return self.active_pos(parent);
        }
        self.remove_active(parent);
        for i in 0..self.branchlist[newbranch].paths.len() {
            let p = self.branchlist[newbranch].paths[i];
            self.insert_active(p);
        }
        let first = self.branchlist[newbranch].paths[0];
        self.active_pos(first)
    }

    /// Position of a trace within the active list (the C++ trace `activeiter`).
    fn active_pos(&self, trace: usize) -> usize {
        self.activetrace.iter().position(|&t| t == trace).unwrap_or(self.activetrace.len())
    }

    /// Check if a given BlockTrace can be retired (C++ `TraceDAG::checkRetirement`,
    /// `blockaction.cc:866`).  On success returns `(true, Some(exitblock))` (the
    /// exit block may itself be `None` if all sibling paths are terminal).
    fn check_retirement(&self, trace: usize) -> (bool, Option<BlockId>) {
        if self.tracelist[trace].pathout != 0 {
            return (false, None); // Only check, if this is the first sibling
        }
        let bp = self.tracelist[trace].top;
        if self.branchlist[bp].depth == 0 {
            // Special conditions for retirement of root branch point
            for i in 0..self.branchlist[bp].paths.len() {
                let curtrace = self.branchlist[bp].paths[i];
                if !self.tracelist[curtrace].is_active() {
                    return (false, None);
                }
                if !self.tracelist[curtrace].is_terminal() {
                    return (false, None); // All root paths must be terminal
                }
            }
            return (true, None);
        }
        let mut outblock: Option<BlockId> = None;
        for i in 0..self.branchlist[bp].paths.len() {
            let curtrace = self.branchlist[bp].paths[i];
            if !self.tracelist[curtrace].is_active() {
                return (false, None);
            }
            if self.tracelist[curtrace].is_terminal() {
                continue;
            }
            if outblock == self.tracelist[curtrace].destnode {
                continue;
            }
            if outblock.is_some() {
                return (false, None);
            }
            outblock = self.tracelist[curtrace].destnode;
        }
        (true, outblock)
    }

    /// Retire a BranchPoint, updating its parent BlockTrace (C++
    /// `TraceDAG::retireBranch`, `blockaction.cc:900`).  Returns the next
    /// `current_activeiter` position (index into `activetrace`).
    fn retire_branch(&mut self, bp: usize, exitblock: Option<BlockId>) -> usize {
        let mut edgeout_bl: Option<BlockId> = None;
        let mut edgelump_sum = 0;

        for i in 0..self.branchlist[bp].paths.len() {
            let curtrace = self.branchlist[bp].paths[i];
            if !self.tracelist[curtrace].is_terminal() {
                edgelump_sum += self.tracelist[curtrace].edgelump;
                if edgeout_bl.is_none() {
                    edgeout_bl = self.tracelist[curtrace].bottom;
                }
            }
            self.remove_active(curtrace); // Child traces are complete and no longer active
        }
        if self.branchlist[bp].depth == 0 {
            // If this is the root block
            return 0; // activetrace.begin()
        }

        if let Some(parent) = self.branchlist[bp].parent {
            let parenttrace = self.branchlist[parent].paths[self.branchlist[bp].pathout as usize];
            self.tracelist[parenttrace].derivedbp = None; // Derived branchpoint is gone
            if edgeout_bl.is_none() {
                // If all traces were terminal
                self.tracelist[parenttrace].flags |= F_TERMINAL;
                self.tracelist[parenttrace].bottom = None;
                self.tracelist[parenttrace].destnode = None;
                self.tracelist[parenttrace].edgelump = 0;
            } else {
                self.tracelist[parenttrace].bottom = edgeout_bl;
                self.tracelist[parenttrace].destnode = exitblock;
                self.tracelist[parenttrace].edgelump = edgelump_sum;
            }
            self.insert_active(parenttrace); // Parent trace gets re-activated
            return self.active_pos(parenttrace);
        }
        0
    }

    /// Clear the \b visitcount field of any FlowBlock we have modified (C++
    /// `TraceDAG::clearVisitCount`, `blockaction.cc:940`).
    fn clear_visit_count(&self, graph: &mut BlockGraph) {
        for fe in self.likelygoto.iter() {
            graph.block_mut(fe.get_bottom()).set_visit_count(0);
        }
    }

    /// Create the initial (virtual) BranchPoint and BlockTrace objects (C++
    /// `TraceDAG::initialize`, `blockaction.cc:967`).
    fn initialize(&mut self) {
        let rootbranch = BranchPoint {
            parent: None,
            depth: 0,
            pathout: -1,
            ismark: false,
            top: None,
            paths: Vec::new(),
        };
        self.branchlist.push(rootbranch); // index 0
        let rootidx = 0;

        for i in 0..self.rootlist.len() {
            // BlockTrace(rootBranch, paths.size(), rootlist[i]): bottom=null, destnode=bl
            let po = self.branchlist[rootidx].paths.len() as int4;
            let bl = self.rootlist[i];
            let trace = BlockTrace {
                flags: 0,
                top: rootidx,
                pathout: po,
                bottom: None,
                destnode: Some(bl),
                edgelump: 1,
                derivedbp: None,
            };
            let tidx = self.tracelist.len();
            self.tracelist.push(trace);
            self.branchlist[rootidx].paths.push(tidx);
            self.insert_active(tidx);
        }
    }

    /// Push the trace through, removing edges as necessary (C++
    /// `TraceDAG::pushBranches`, `blockaction.cc:983`).
    fn push_branches(&mut self, graph: &mut BlockGraph) {
        self.current_activeiter = 0; // activetrace.begin()
        self.missedactivecount = 0;
        while self.activecount > 0 {
            if self.current_activeiter >= self.activetrace.len() {
                self.current_activeiter = 0; // activetrace.begin()
            }
            let curtrace = self.activetrace[self.current_activeiter];
            if self.missedactivecount >= self.activecount {
                // Could not push any trace further
                let badtrace = self.select_bad_edge(graph);
                self.remove_trace(badtrace, graph);
                self.current_activeiter = 0;
                self.missedactivecount = 0;
            } else {
                let (can_retire, exitblock) = self.check_retirement(curtrace);
                if can_retire {
                    let bp = self.tracelist[curtrace].top;
                    self.current_activeiter = self.retire_branch(bp, exitblock);
                    self.missedactivecount = 0;
                } else if self.check_open(curtrace, graph) {
                    self.current_activeiter = self.open_branch(curtrace, graph);
                    self.missedactivecount = 0;
                } else {
                    self.missedactivecount += 1;
                    self.current_activeiter += 1;
                }
            }
        }
        self.clear_visit_count(graph);
    }
}

// ===========================================================================
// negateCondition / isComplex seams (block.cc virtuals; SEAM(W7) in block.rs)
// ===========================================================================

// `FlowBlock::negateCondition` and its subtype overrides (C++
// `block.cc:294/2355/3015/3071`) are realized as the method
// [`CollapseStructure::negate_condition_rec`] above: the topology half
// (`swapEdges`) runs in place on the structured graph, and the data-flow half
// (the leaf `BlockBasic` CBRANCH `boolean_flip`/`fallthru_true` op flags +
// the `BlockCondition` opcode) is recorded into `pending_flips` (op flags) and
// realized against the obank by `ActionBlockStructure::apply` after the collapse.

/// Determine whether a FlowBlock is too complex to be a condition clause (C++
/// `FlowBlock::isComplex` base + `BlockBasic::isComplex`, `block.hh:254`/`block.cc:2403`;
/// left as `// SEAM(W7)` in `block.rs`).
///
/// The C++ base default returns `true`; `BlockBasic::isComplex` counts statements
/// against `glb->max_implied_ref` (data-flow).  The port returns the base default
/// (`true`), seam-noting the statement count.  This affects only whether a
/// whileDo uses *overflow syntax* and whether `ruleBlockOr` fires (both fall back
/// to the conservative branch).  See losses.
fn is_complex(_graph: &BlockGraph, _bl: BlockId) -> bool {
    // BlockBasic::isComplex statement counting needs the op list + max_implied_ref -- SEAM(W7)
    true
}

/// Surface the unported `BlockGraph::newBlockSwitch` (C++ `block.cc:1907`).
///
/// SEAM(W7/W4): the switch *factory* needs `FlowBlock::getExitLeaf` and
/// `BlockSwitch::grabCaseBasic` (the latter reads `JumpTable` labels, W4) — both
/// left unported in `block.rs`, and the `BlockGraph` switch factory is not yet
/// public.  [`CollapseStructure::rule_block_switch`] performs the full match /
/// exit-block / skip-edge decision (the structuring novelty) but cannot build
/// the node here; this returns an `Err` so the seam is visible.  See losses.
fn new_block_switch_seam(_has_exit: bool) -> KunaResult<()> {
    Err(KunaError::lowlevel(
        "kuna rust port: BlockGraph::newBlockSwitch needs getExitLeaf + grabCaseBasic \
         (JumpTable labels, W4) which block.rs leaves as SEAM(W7); not available at this \
         item's boundary",
    ))
}

// ===========================================================================
// CollapseStructure (blockaction.hh:192-226, blockaction.cc:1046-1893)
// ===========================================================================

/// \brief Build a code structure from a control-flow graph (C++ `CollapseStructure`).
///
/// Manages the main control-flow structuring algorithm: repeatedly search for
/// sub-graphs matching code-structure elements and collapse them; when stuck,
/// remove an edge (chosen by [`TraceDAG`]) and mark it unstructured.  Operates on
/// a [`BlockGraph`] (the `sblocks` graph) identified by its root [`BlockId`].
pub struct CollapseStructure<'a> {
    /// Have we made a search for unstructured edges in the final DAG (C++ `finaltrace`)
    finaltrace: bool,
    /// Have we generated a \e likely \e goto list for the current innermost loop
    /// (C++ `likelylistfull`)
    likelylistfull: bool,
    /// The current \e likely \e goto list (C++ `list<FloatingEdge> likelygoto`)
    likelygoto: Vec<FloatingEdge>,
    /// Index of the next most \e likely \e goto edge (C++ `likelyiter`)
    likelyiter: usize,
    /// The list of loop bodies (C++ `list<LoopBody> loopbody`)
    loopbody: Vec<LoopBody>,
    /// Current (innermost) loop being structured (index into `loopbody`)
    /// (C++ `loopbodyiter`)
    loopbodyiter: usize,
    /// The control-flow graph (C++ `BlockGraph &graph`)
    graph: &'a mut BlockGraph,
    /// The root BlockGraph node id within `graph`
    graph_id: BlockId,
    /// Number of data-flow changes made during structuring (C++ `dataflow_changecount`)
    dataflow_changecount: int4,
    /// bblocks `BlockBasic` ids whose underlying CBRANCH op needs its
    /// `boolean_flip`/`fallthru_true` flipped (the data-flow half of
    /// `BlockBasic::negateCondition`).  In C++ this flip happens in-place on the
    /// live `BlockBasic` (the `BlockCopy::copy` pointer); in the dual-arena port
    /// the op flags live in the `Funcdata` obank, so we record each flip here and
    /// apply (XOR-reduced — even flips cancel) after `collapseAll` returns, when
    /// `ActionBlockStructure::apply` holds `&mut Funcdata` again.
    pending_flips: Vec<BlockId>,
}

impl<'a> CollapseStructure<'a> {
    /// Construct given a control-flow graph (C++ `CollapseStructure::CollapseStructure`,
    /// `blockaction.cc:1870`).  `graph_id` is the root BlockGraph node.
    pub fn new(graph: &'a mut BlockGraph, graph_id: BlockId) -> CollapseStructure<'a> {
        CollapseStructure {
            finaltrace: false,
            likelylistfull: false,
            likelygoto: Vec::new(),
            likelyiter: 0,
            loopbody: Vec::new(),
            loopbodyiter: 0,
            graph,
            graph_id,
            dataflow_changecount: 0,
            pending_flips: Vec::new(),
        }
    }

    /// Get the number of data-flow changes (C++ `getChangeCount`).
    pub fn get_change_count(&self) -> int4 {
        self.dataflow_changecount
    }

    /// The XOR-reduced set of bblocks `BlockBasic` ids whose CBRANCH op must have
    /// its `boolean_flip`/`fallthru_true` toggled (the deferred data-flow half of
    /// `BlockBasic::negateCondition`).  Even numbers of flips on the same block
    /// cancel out (a double-negation), so we reduce to parity here.
    pub fn take_pending_flips(&mut self) -> Vec<BlockId> {
        let mut parity: std::collections::BTreeMap<BlockId, bool> = std::collections::BTreeMap::new();
        for &b in &self.pending_flips {
            let e = parity.entry(b).or_insert(false);
            *e = !*e;
        }
        parity.into_iter().filter(|&(_, v)| v).map(|(b, _)| b).collect()
    }

    /// The data-flow half of `FlowBlock::negateCondition` and its subtype
    /// overrides (C++ `block.cc:294/2355` + the `BlockCopy`/`BlockList`/
    /// `BlockCondition` virtuals in `block.hh:53` etc.).
    ///
    /// The **topology** half (`swapEdges` on every node visited) runs here exactly
    /// as in C++.  The **data-flow** half — flipping the `boolean_flip`/
    /// `fallthru_true` op flags on the leaf `BlockBasic`'s CBRANCH (reached via the
    /// `BlockCopy::copy` pointer) and flipping a `BlockCondition`'s opcode — is
    /// recorded into `pending_flips` (op flags) / applied to the structured node
    /// (opcode) so it can be realized against the obank after the collapse.
    /// Returns `true` if a data-flow change was made (the C++ return that drives
    /// `dataflow_changecount`).
    fn negate_condition_rec(&mut self, bl: BlockId, toporbottom: bool) -> bool {
        let bt = self.graph.block(bl).get_type();
        let res = match bt {
            // BlockCopy::negateCondition: copy->negateCondition(true); then swap.
            // `copy` is the bblocks BlockBasic — record its op-flag flip.
            BlockType::Copy => {
                if let Some(copy) = self.graph.block(bl).get_copy() {
                    self.pending_flips.push(copy);
                }
                true
            }
            // BlockList::negateCondition: getBlock(size-1)->negateCondition(false)
            // then swap this node's edges.
            BlockType::Ls => {
                let sz = self.graph.block(bl).get_size();
                if sz > 0 {
                    let last = self.graph.block(bl).get_block(sz - 1);
                    self.negate_condition_rec(last, false)
                } else {
                    false
                }
            }
            // BlockCondition::negateCondition: distribute the NOT to both sides,
            // flip the AND/OR opcode, then swap this node's edges.
            BlockType::Condition => {
                let b0 = self.graph.block(bl).get_block(0);
                let b1 = self.graph.block(bl).get_block(1);
                let r0 = self.negate_condition_rec(b0, false);
                let r1 = self.negate_condition_rec(b1, false);
                self.graph.block_mut(bl).flip_condition_opcode();
                r0 || r1
            }
            // base FlowBlock::negateCondition: edge swap only, no data-flow change.
            _ => false,
        };
        // FlowBlock::negateCondition(toporbottom): if (!toporbottom) return; swapEdges();
        if toporbottom {
            self.graph.swap_edges(bl);
        }
        res
    }

    /// Number of components in the structuring graph (C++ `graph.getSize()`).
    fn size(&self) -> int4 {
        self.graph.block(self.graph_id).get_size()
    }
    /// The i-th component (C++ `graph.getBlock(i)`).
    fn block_at(&self, i: int4) -> BlockId {
        self.graph.block(self.graph_id).get_block(i)
    }

    /// \brief Mark FlowBlocks \b only reachable from a given root (C++
    /// `CollapseStructure::onlyReachableFromRoot`, `blockaction.cc:1052`).
    fn only_reachable_from_root(&mut self, root: BlockId, body: &mut Vec<BlockId>) {
        let mut trial: Vec<BlockId> = Vec::new();
        let mut i = 0;
        self.graph.block_mut(root).set_mark();
        body.push(root);
        while i < body.len() {
            let bl = body[i];
            i += 1;
            let sizeout = self.graph.block(bl).size_out();
            for j in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(j);
                if self.graph.block(curbl).is_mark() {
                    continue;
                }
                let mut count = self.graph.block(curbl).get_visit_count();
                if count == 0 {
                    trial.push(curbl); // New possible extension
                }
                count += 1;
                self.graph.block_mut(curbl).set_visit_count(count);
                if count == self.graph.block(curbl).size_in() {
                    self.graph.block_mut(curbl).set_mark();
                    body.push(curbl);
                }
            }
        }
        for i in 0..trial.len() {
            self.graph.block_mut(trial[i]).set_visit_count(0); // Make sure to clear the count
        }
    }

    /// Mark edges exiting the body as \e unstructured gotos (C++
    /// `CollapseStructure::markExitsAsGotos`, `blockaction.cc:1083`).  Returns the
    /// number of edges marked.
    fn mark_exits_as_gotos(&mut self, body: &[BlockId]) -> KunaResult<int4> {
        let mut changecount = 0;
        for i in 0..body.len() {
            let bl = body[i];
            let sizeout = self.graph.block(bl).size_out();
            for j in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(j);
                if !self.graph.block(curbl).is_mark() {
                    self.graph.set_goto_branch(bl, j)?; // mark edge as goto
                    changecount += 1;
                }
            }
        }
        Ok(changecount)
    }

    /// Mark edges between root components as \e unstructured gotos (C++
    /// `CollapseStructure::clipExtraRoots`, `blockaction.cc:1108`).  Returns
    /// `true` if any cross-over edges were found (and marked).
    fn clip_extra_roots(&mut self) -> KunaResult<bool> {
        for i in 1..self.size() {
            // Skip the canonical root
            let bl = self.block_at(i);
            if self.graph.block(bl).size_in() != 0 {
                continue;
            }
            let mut body: Vec<BlockId> = Vec::new();
            self.only_reachable_from_root(bl, &mut body);
            let count = self.mark_exits_as_gotos(&body)?;
            LoopBody::clear_marks(&body, self.graph);
            if count != 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Identify all the loops in this graph (C++ `CollapseStructure::labelLoops`,
    /// `blockaction.cc:1126`).  Creates a [`LoopBody`] for each back-edge; returns
    /// the `looporder` index list (into `self.loopbody`) sorted by head/tail.
    fn label_loops(&mut self) -> Vec<usize> {
        let mut looporder: Vec<usize> = Vec::new();
        for i in 0..self.size() {
            let bl = self.block_at(i);
            let sizein = self.graph.block(bl).size_in();
            for j in 0..sizein {
                if self.graph.block(bl).is_back_edge_in(j) {
                    // back-edge coming in must be from the bottom of a loop
                    let loopbottom = self.graph.block(bl).get_in(j);
                    let mut lb = LoopBody::new(bl);
                    lb.add_tail(loopbottom);
                    looporder.push(self.loopbody.len());
                    self.loopbody.push(lb);
                }
            }
        }
        // sort(looporder, LoopBody::compare_ends);
        // Split-borrow `graph` (read-only here) and `loopbody` immutably so the
        // sort comparator can read both without re-borrowing `self`.
        let graph: &BlockGraph = self.graph;
        let loopbody = &self.loopbody;
        looporder.sort_by(|&a, &b| LoopBody::compare_ends(&loopbody[a], &loopbody[b], graph));
        looporder
    }

    /// Identify and label all loop structure for this graph (C++
    /// `CollapseStructure::orderLoopBodies`, `blockaction.cc:1148`).
    fn order_loop_bodies(&mut self) {
        let mut looporder = self.label_loops();
        if !self.loopbody.is_empty() {
            let oldsize = looporder.len();
            LoopBody::merge_identical_heads(&mut looporder, &mut self.loopbody);
            if oldsize != looporder.len() {
                // If there was merging, drop subsumed loop bodies (head == None).
                // Keep the looporder indices valid by remapping after compaction.
                let mut keep: Vec<usize> = Vec::new();
                for idx in 0..self.loopbody.len() {
                    if self.loopbody[idx].get_head().is_some() {
                        keep.push(idx);
                    }
                }
                // Build remap old-index -> new-index
                let mut remap = vec![usize::MAX; self.loopbody.len()];
                for (newi, &oldi) in keep.iter().enumerate() {
                    remap[oldi] = newi;
                }
                let mut compacted: Vec<LoopBody> = Vec::with_capacity(keep.len());
                for &oldi in &keep {
                    let mut lb = self.loopbody[oldi].clone();
                    if let Some(ic) = lb.immed_container {
                        lb.immed_container = if remap[ic] == usize::MAX { None } else { Some(remap[ic]) };
                    }
                    compacted.push(lb);
                }
                self.loopbody = compacted;
                looporder = looporder.iter().map(|&o| remap[o]).filter(|&n| n != usize::MAX).collect();
            }
            // Build the sorted head-index lookup for LoopBody::find.
            let lookup = self.build_looporder_lookup(&looporder);
            for idx in 0..self.loopbody.len() {
                let mut body: Vec<BlockId> = Vec::new();
                self.loopbody[idx].find_base(&mut body, self.graph);
                {
                    let looporder_indices = looporder.clone();
                    LoopBody::label_containments(
                        &body,
                        &looporder_indices,
                        &mut self.loopbody,
                        idx,
                        self.graph,
                        &lookup,
                    );
                }
                LoopBody::clear_marks(&body, self.graph);
            }
            // loopbody.sort(): by depth (deepest first), stable.
            self.stable_sort_loopbody_by_depth();
            for idx in 0..self.loopbody.len() {
                let mut body: Vec<BlockId> = Vec::new();
                self.loopbody[idx].find_base(&mut body, self.graph);
                // findExit needs immed_container resolved against the full loopbody.
                let looporder_snapshot = self.loopbody.clone();
                self.loopbody[idx].find_exit(&body, self.graph, &looporder_snapshot);
                self.loopbody[idx].order_tails(self.graph);
                let lb = self.loopbody[idx].clone();
                lb.extend(&mut body, self.graph);
                self.loopbody[idx].label_exit_edges(&body, self.graph);
                LoopBody::clear_marks(&body, self.graph);
            }
        }
        self.likelylistfull = false;
        self.loopbodyiter = 0;
    }

    /// Stable-sort `loopbody` by depth, deepest first (C++ `loopbody.sort()` with
    /// `operator<` ordering by `depth > op2.depth`).  Re-maps `immed_container`
    /// indices through the permutation.
    fn stable_sort_loopbody_by_depth(&mut self) {
        let n = self.loopbody.len();
        let mut order: Vec<usize> = (0..n).collect();
        // Stable sort by depth descending (operator<: a < b iff a.depth > b.depth).
        order.sort_by(|&a, &b| self.loopbody[b].depth.cmp(&self.loopbody[a].depth));
        let mut remap = vec![usize::MAX; n];
        for (newi, &oldi) in order.iter().enumerate() {
            remap[oldi] = newi;
        }
        let mut sorted: Vec<LoopBody> = Vec::with_capacity(n);
        for &oldi in &order {
            let mut lb = self.loopbody[oldi].clone();
            if let Some(ic) = lb.immed_container {
                lb.immed_container = Some(remap[ic]);
            }
            sorted.push(lb);
        }
        self.loopbody = sorted;
    }

    /// Build the sorted `(head_index, loopbody_index)` lookup that
    /// `LoopBody::find` binary-searches (the C++ `looporder` vector is already
    /// sorted by head index after `mergeIdenticalHeads`).
    fn build_looporder_lookup(&self, looporder: &[usize]) -> Vec<(int4, usize)> {
        let mut lookup: Vec<(int4, usize)> = looporder
            .iter()
            .map(|&idx| {
                let head = self.loopbody[idx].get_head().expect("looporder head");
                (self.graph.block(head).get_index(), idx)
            })
            .collect();
        lookup.sort_by_key(|&(h, _)| h);
        lookup
    }

    /// Find likely \e unstructured edges within the innermost loop body (C++
    /// `CollapseStructure::updateLoopBody`, `blockaction.cc:1193`).  Returns
    /// `true` if there are likely unstructured edges left to provide.
    fn update_loop_body(&mut self) -> bool {
        if self.finaltrace {
            // If we've already performed trace on DAG with no likely goto edges
            return false; // don't repeat the trace
        }
        let mut loopbottom: Option<BlockId> = None;
        let mut looptop: Option<BlockId> = None;
        while self.loopbodyiter < self.loopbody.len() {
            loopbottom = self.loopbody[self.loopbodyiter].update(self.graph, self.graph_id);
            if loopbottom.is_some() {
                looptop = self.loopbody[self.loopbodyiter].get_head();
                if loopbottom == looptop {
                    // Check for single node looping back to itself
                    self.likelygoto.clear();
                    self.likelygoto
                        .push(FloatingEdge::new(looptop.unwrap(), looptop.unwrap()));
                    self.likelyiter = 0;
                    self.likelylistfull = true;
                    return true;
                }
                if !self.likelylistfull || self.likelyiter != self.likelygoto.len() {
                    break; // Loop still exists
                }
            }
            self.loopbodyiter += 1;
            self.likelylistfull = false; // Need to generate likely list for new loopbody (or no loopbody)
            loopbottom = None;
        }
        if self.likelylistfull && self.likelyiter != self.likelygoto.len() {
            return true;
        }

        // Need to generate likely gotos for a new inner loop or DAG
        self.likelygoto.clear();
        {
            let graph_id = self.graph_id;
            let mut likely = std::mem::take(&mut self.likelygoto);
            {
                let mut tracer = TraceDAG::new(&mut likely);
                if let Some(lb) = loopbottom {
                    tracer.add_root(looptop.unwrap()); // Trace from the top of the loop
                    tracer.set_finish_block(lb);
                    self.loopbody[self.loopbodyiter].set_exit_marks(self.graph, graph_id);
                } else {
                    for i in 0..self.size() {
                        let bl = self.block_at(i);
                        if self.graph.block(bl).size_in() == 0 {
                            tracer.add_root(bl);
                        }
                    }
                }
                tracer.initialize();
                tracer.push_branches(self.graph);
            }
            self.likelygoto = likely;
        }
        self.likelylistfull = true; // Mark likelygoto generation complete
        if let Some(_lb) = loopbottom {
            let graph_id = self.graph_id;
            let mut likely = std::mem::take(&mut self.likelygoto);
            self.loopbody[self.loopbodyiter].emit_likely_edges(&mut likely, self.graph, graph_id);
            self.likelygoto = likely;
            self.loopbody[self.loopbodyiter].clear_exit_marks(self.graph, graph_id);
        } else if self.likelygoto.is_empty() {
            self.finaltrace = true; // No loops left and trace didn't find gotos
            return false;
        }
        self.likelyiter = 0;
        true
    }

    /// Select an edge to mark as \e unstructured (C++ `CollapseStructure::selectGoto`,
    /// `blockaction.cc:1260`).  Returns the FlowBlock whose outgoing edge was
    /// marked, or `None`.  Errors (the C++ `throw LowlevelError`) if it cannot
    /// finish collapsing.
    fn select_goto(&mut self) -> KunaResult<Option<BlockId>> {
        while self.update_loop_body() {
            while self.likelyiter != self.likelygoto.len() {
                let graph_id = self.graph_id;
                let (startbl, outedge) =
                    self.likelygoto[self.likelyiter].get_current_edge(self.graph, graph_id);
                self.likelyiter += 1;
                if let Some(startbl) = startbl {
                    self.graph.set_goto_branch(startbl, outedge)?; // Mark selected branch as goto
                    return Ok(Some(startbl));
                }
            }
        }
        if !self.clip_extra_roots()? {
            return Err(KunaError::lowlevel("Could not finish collapsing block structure"));
        }
        Ok(None)
    }

    /// Attempt to apply a BlockList structure (C++ `CollapseStructure::ruleBlockCat`,
    /// `blockaction.cc:1284`).
    fn rule_block_cat(&mut self, bl: BlockId) -> KunaResult<bool> {
        if self.graph.block(bl).size_out() != 1 {
            return Ok(false);
        }
        if self.graph.block(bl).is_switch_out() {
            return Ok(false);
        }
        if self.graph.block(bl).size_in() == 1
            && self.graph.block(self.graph.block(bl).get_in(0)).size_out() == 1
        {
            return Ok(false); // Must be start of chain
        }
        let mut outblock = self.graph.block(bl).get_out(0);
        if outblock == bl {
            return Ok(false); // No looping
        }
        if self.graph.block(outblock).size_in() != 1 {
            return Ok(false); // Nothing else can hit outblock
        }
        if !self.graph.block(bl).is_decision_out(0) {
            return Ok(false); // Not a goto or a loopbottom
        }
        if self.graph.block(outblock).is_switch_out() {
            return Ok(false); // Switch must be resolved first
        }

        let mut nodes: Vec<BlockId> = Vec::new();
        nodes.push(bl); // The first two blocks being concatenated
        nodes.push(outblock);

        while self.graph.block(outblock).size_out() == 1 {
            let outbl2 = self.graph.block(outblock).get_out(0);
            if outbl2 == bl {
                break; // No looping
            }
            if self.graph.block(outbl2).size_in() != 1 {
                break; // Nothing else can hit outblock
            }
            if !self.graph.block(outblock).is_decision_out(0) {
                break; // Don't use loop bottom
            }
            if self.graph.block(outbl2).is_switch_out() {
                break; // Switch must be resolved first
            }
            outblock = outbl2;
            nodes.push(outblock); // Extend the cat chain
        }

        let graph_id = self.graph_id;
        self.graph.new_block_list(graph_id, &nodes)?; // Concatenate into a single block
        Ok(true)
    }

    /// Attempt to apply a BlockCondition structure (C++ `CollapseStructure::ruleBlockOr`,
    /// `blockaction.cc:1321`).
    fn rule_block_or(&mut self, bl: BlockId) -> KunaResult<bool> {
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
            if is_complex(self.graph, orblock) {
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
                if self.negate_condition_rec(bl, true) {
                    self.dataflow_changecount += 1;
                }
            }
            if j == 0 {
                // clauseblock needs to be true out of orblock
                if self.negate_condition_rec(orblock, true) {
                    self.dataflow_changecount += 1;
                }
            }

            let graph_id = self.graph_id;
            self.graph.new_block_condition(graph_id, bl, orblock)?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Attempt to apply a 2-component form of BlockIf (C++
    /// `CollapseStructure::ruleBlockProperIf`, `blockaction.cc:1378`).
    fn rule_block_proper_if(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 2 {
            return false; // Must be binary condition
        }
        if self.graph.block(bl).is_switch_out() {
            return false;
        }
        if self.graph.block(bl).get_out(0) == bl {
            return false; // No loops
        }
        if self.graph.block(bl).get_out(1) == bl {
            return false;
        }
        if self.graph.block(bl).is_goto_out(0) {
            return false; // Neither branch must be unstructured
        }
        if self.graph.block(bl).is_goto_out(1) {
            return false;
        }
        for i in 0..2 {
            let clauseblock = self.graph.block(bl).get_out(i);
            if self.graph.block(clauseblock).size_in() != 1 {
                continue; // Nothing else can hit clauseblock
            }
            if self.graph.block(clauseblock).size_out() != 1 {
                continue; // Only one way out of clause
            }
            if self.graph.block(clauseblock).is_switch_out() {
                continue; // Don't use switch
            }
            if !self.graph.block(bl).is_decision_out(i) {
                continue; // Don't use loopbottom or exit
            }
            if self.graph.block(clauseblock).is_goto_out(0) {
                continue; // No unstructured jumps out of clause
            }
            let outblock = self.graph.block(clauseblock).get_out(0);
            if outblock != self.graph.block(bl).get_out(1 - i) {
                continue; // Path after clause must be the same
            }

            if i == 0 {
                // Clause must be true
                if self.negate_condition_rec(bl, true) {
                    self.dataflow_changecount += 1;
                }
            }

            let graph_id = self.graph_id;
            self.graph.new_block_if(graph_id, bl, clauseblock);
            return true;
        }
        false
    }

    /// Attempt to apply a 3-component form of BlockIf (C++
    /// `CollapseStructure::ruleBlockIfElse`, `blockaction.cc:1416`).
    fn rule_block_if_else(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 2 {
            return false; // Must be binary condition
        }
        if self.graph.block(bl).is_switch_out() {
            return false;
        }
        if !self.graph.block(bl).is_decision_out(0) {
            return false;
        }
        if !self.graph.block(bl).is_decision_out(1) {
            return false;
        }

        let tc = self.graph.block(bl).get_true_out();
        let fc = self.graph.block(bl).get_false_out();
        if self.graph.block(tc).size_in() != 1 {
            return false; // Nothing else must hit true clause
        }
        if self.graph.block(fc).size_in() != 1 {
            return false; // Nothing else must hit false clause
        }

        if self.graph.block(tc).size_out() != 1 {
            return false; // Only one exit from clause
        }
        if self.graph.block(fc).size_out() != 1 {
            return false; // Only one exit from clause
        }
        let outblock = self.graph.block(tc).get_out(0);
        if outblock == bl {
            return false; // No loops
        }
        if outblock != self.graph.block(fc).get_out(0) {
            return false; // Clauses must exit to same place
        }

        if self.graph.block(tc).is_switch_out() {
            return false;
        }
        if self.graph.block(fc).is_switch_out() {
            return false;
        }
        if self.graph.block(tc).is_goto_out(0) {
            return false;
        }
        if self.graph.block(fc).is_goto_out(0) {
            return false;
        }

        let graph_id = self.graph_id;
        self.graph.new_block_if_else(graph_id, bl, tc, fc);
        true
    }

    /// Attempt to apply the BlockGoto or BlockMultiGoto structure (C++
    /// `CollapseStructure::ruleBlockGoto`, `blockaction.cc:1450`).
    fn rule_block_goto(&mut self, bl: BlockId) -> KunaResult<bool> {
        let sizeout = self.graph.block(bl).size_out();
        for i in 0..sizeout {
            if self.graph.block(bl).is_goto_out(i) {
                if self.graph.block(bl).is_switch_out() {
                    let graph_id = self.graph_id;
                    self.graph.new_block_multi_goto(graph_id, bl, i);
                    return Ok(true);
                }
                if sizeout == 2 {
                    if !self.graph.block(bl).is_goto_out(1) {
                        // True branch must be goto
                        if self.negate_condition_rec(bl, true) {
                            self.dataflow_changecount += 1;
                        }
                    }
                    let graph_id = self.graph_id;
                    self.graph.new_block_if_goto(graph_id, bl)?;
                    return Ok(true);
                }
                if sizeout == 1 {
                    let graph_id = self.graph_id;
                    self.graph.new_block_goto(graph_id, bl);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Attempt to apply BlockIf where the body does not exit (C++
    /// `CollapseStructure::ruleBlockIfNoExit`, `blockaction.cc:1481`).
    fn rule_block_if_no_exit(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 2 {
            return false; // Must be binary condition
        }
        if self.graph.block(bl).is_switch_out() {
            return false;
        }
        if self.graph.block(bl).get_out(0) == bl {
            return false; // No loops
        }
        if self.graph.block(bl).get_out(1) == bl {
            return false;
        }
        if self.graph.block(bl).is_goto_out(0) {
            return false;
        }
        if self.graph.block(bl).is_goto_out(1) {
            return false;
        }
        for i in 0..2 {
            let clauseblock = self.graph.block(bl).get_out(i);
            if self.graph.block(clauseblock).size_in() != 1 {
                continue; // Nothing else must hit clause
            }
            if self.graph.block(clauseblock).size_out() != 0 {
                continue; // Must be no way out of clause
            }
            if self.graph.block(clauseblock).is_switch_out() {
                continue;
            }
            if !self.graph.block(bl).is_decision_out(i) {
                continue;
            }

            if i == 0 {
                // clause must be true out of bl
                if self.negate_condition_rec(bl, true) {
                    self.dataflow_changecount += 1;
                }
            }
            let graph_id = self.graph_id;
            self.graph.new_block_if(graph_id, bl, clauseblock);
            return true;
        }
        false
    }

    /// Attempt to apply the BlockWhileDo structure (C++
    /// `CollapseStructure::ruleBlockWhileDo`, `blockaction.cc:1518`).
    fn rule_block_while_do(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 2 {
            return false; // Must be binary condition
        }
        if self.graph.block(bl).is_switch_out() {
            return false;
        }
        if self.graph.block(bl).get_out(0) == bl {
            return false; // No loops at this point
        }
        if self.graph.block(bl).get_out(1) == bl {
            return false;
        }
        if self.graph.block(bl).is_interior_goto_target() {
            return false;
        }
        if self.graph.block(bl).is_goto_out(0) {
            return false;
        }
        if self.graph.block(bl).is_goto_out(1) {
            return false;
        }
        for i in 0..2 {
            let clauseblock = self.graph.block(bl).get_out(i);
            if self.graph.block(clauseblock).size_in() != 1 {
                continue; // Nothing else must hit clause
            }
            if self.graph.block(clauseblock).size_out() != 1 {
                continue; // Only one way out of clause
            }
            if self.graph.block(clauseblock).is_switch_out() {
                continue;
            }
            if self.graph.block(clauseblock).get_out(0) != bl {
                continue; // Clause must loop back to bl
            }

            let overflow = is_complex(self.graph, bl); // Check if we need overflow syntax
            if (i == 0) != overflow {
                // clause must be true out of bl unless we use overflow syntax
                if self.negate_condition_rec(bl, true) {
                    self.dataflow_changecount += 1;
                }
            }
            let graph_id = self.graph_id;
            let newbl = self.graph.new_block_while_do(graph_id, bl, clauseblock);
            if overflow {
                self.graph.block_mut(newbl).set_overflow_syntax();
            }
            return true;
        }
        false
    }

    /// Attempt to apply the BlockDoWhile structure (C++
    /// `CollapseStructure::ruleBlockDoWhile`, `blockaction.cc:1555`).
    fn rule_block_do_while(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 2 {
            return false; // Must be binary condition
        }
        if self.graph.block(bl).is_switch_out() {
            return false;
        }
        if self.graph.block(bl).is_goto_out(0) {
            return false;
        }
        if self.graph.block(bl).is_goto_out(1) {
            return false;
        }
        for i in 0..2 {
            if self.graph.block(bl).get_out(i) != bl {
                continue; // Must loop back on itself
            }
            if i == 0 {
                // must loop on true condition
                if self.negate_condition_rec(bl, true) {
                    self.dataflow_changecount += 1;
                }
            }
            let graph_id = self.graph_id;
            self.graph.new_block_do_while(graph_id, bl);
            return true;
        }
        false
    }

    /// Attempt to apply the BlockInfLoop structure (C++
    /// `CollapseStructure::ruleBlockInfLoop`, `blockaction.cc:1579`).
    fn rule_block_inf_loop(&mut self, bl: BlockId) -> bool {
        if self.graph.block(bl).size_out() != 1 {
            return false; // Must only be one way out
        }
        // (see C++ comment: we intentionally do NOT check isSwitchOut here)
        if self.graph.block(bl).is_goto_out(0) {
            return false;
        }
        if self.graph.block(bl).get_out(0) != bl {
            return false; // Must fall into itself
        }
        let graph_id = self.graph_id;
        self.graph.new_block_inf_loop(graph_id, bl);
        true
    }

    /// \brief Check for switch edges that go straight to the exit block (C++
    /// `CollapseStructure::checkSwitchSkips`, `blockaction.cc:1607`).  If such skip
    /// edges exist, they are converted to gotos and `false` is returned.
    fn check_switch_skips(&mut self, switchbl: BlockId, exitblock: Option<BlockId>) -> KunaResult<bool> {
        let exitblock = match exitblock {
            Some(e) => e,
            None => return Ok(true),
        };

        // Is there a "default" edge that goes straight to the exitblock
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

        if !defaultnottoexit && self.graph.block(switchbl).get_type() == BlockType::MultiGoto {
            if let BlockKind::MultiGoto { defaultswitch, .. } = self.graph.block(switchbl).kind() {
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

    /// Attempt to apply the BlockSwitch structure (C++
    /// `CollapseStructure::ruleBlockSwitch`, `blockaction.cc:1649`).
    fn rule_block_switch(&mut self, bl: BlockId) -> KunaResult<bool> {
        if !self.graph.block(bl).is_switch_out() {
            return Ok(false);
        }
        let mut exitblock: Option<BlockId> = None;
        let sizeout = self.graph.block(bl).size_out();

        // Find "obvious" exitblock, is sizeIn>1 or sizeOut>1
        for i in 0..sizeout {
            let curbl = self.graph.block(bl).get_out(i);
            if curbl == bl {
                exitblock = Some(curbl); // Exit back to top of switch (loop)
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
            // Every immediate block has sizeIn==1 and sizeOut<=1
            for i in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(i);
                if self.graph.block(curbl).is_goto_in(0) {
                    return Ok(false); // In cannot be a goto
                }
                if self.graph.block(curbl).is_switch_out() {
                    return Ok(false); // Must resolve nested switch first
                }
                if self.graph.block(curbl).size_out() == 1 {
                    if self.graph.block(curbl).is_goto_out(0) {
                        return Ok(false); // Out cannot be goto
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
            // From here we have a determined exitblock
            for i in 0..self.graph.block(e).size_in() {
                // No in gotos to exitblock
                if self.graph.block(e).is_goto_in(i) {
                    return Ok(false);
                }
            }
            for i in 0..self.graph.block(e).size_out() {
                // No out gotos from exitblock
                if self.graph.block(e).is_goto_out(i) {
                    return Ok(false);
                }
            }
            for i in 0..sizeout {
                let curbl = self.graph.block(bl).get_out(i);
                if curbl == e {
                    continue; // The switch can go straight to the exit block
                }
                if self.graph.block(curbl).size_in() > 1 {
                    return Ok(false); // A case can only have the switch fall into it
                }
                if self.graph.block(curbl).is_goto_in(0) {
                    return Ok(false); // In cannot be a goto
                }
                if self.graph.block(curbl).size_out() > 1 {
                    return Ok(false); // There can be at most 1 exit from a case
                }
                if self.graph.block(curbl).size_out() == 1 {
                    if self.graph.block(curbl).is_goto_out(0) {
                        return Ok(false); // Out cannot be goto
                    }
                    if self.graph.block(curbl).get_out(0) != e {
                        return Ok(false); // which must be to the exitblock
                    }
                }
                if self.graph.block(curbl).is_switch_out() {
                    return Ok(false); // Nested switch must be resolved first
                }
            }
        }

        if !self.check_switch_skips(bl, exitblock)? {
            return Ok(true); // We match, but special condition that adds gotos
        }

        let mut cases: Vec<BlockId> = Vec::new();
        cases.push(bl);
        for i in 0..sizeout {
            let curbl = self.graph.block(bl).get_out(i);
            if Some(curbl) == exitblock {
                continue; // Don't include exit as a case
            }
            cases.push(curbl);
        }
        // graph.newBlockSwitch(cases, hasExit) -- SEAM(W7/W4): `BlockGraph::
        // newBlockSwitch` needs `getExitLeaf` and `grabCaseBasic` (the latter
        // reads the JumpTable labels, W4); `block.rs` left both as W7 (see its
        // `markCopyBlock` note) and the BlockGraph switch factory is not yet
        // public.  The match/exit/skip *decision* above is fully ported; only
        // the node construction surfaces the seam.  See losses.
        let _ = cases;
        new_block_switch_seam(exitblock.is_some())?;
        Ok(true)
    }

    /// Attempt to mark switch cases that fall through to another case (C++
    /// `CollapseStructure::ruleCaseFallthru`, `blockaction.cc:1729`).
    fn rule_case_fallthru(&mut self, bl: BlockId) -> KunaResult<bool> {
        if !self.graph.block(bl).is_switch_out() {
            return Ok(false);
        }
        let sizeout = self.graph.block(bl).size_out();
        let mut nonfallthru = 0; // Count of exits that are not fallthru
        let mut fallthru: Vec<BlockId> = Vec::new();

        for i in 0..sizeout {
            let curbl = self.graph.block(bl).get_out(i);
            if curbl == bl {
                return Ok(false); // Cannot exit to itself
            }
            if self.graph.block(curbl).size_in() > 2 || self.graph.block(curbl).size_out() > 1 {
                nonfallthru += 1;
            } else if self.graph.block(curbl).size_out() == 1 {
                let target = self.graph.block(curbl).get_out(0);
                if self.graph.block(target).size_in() == 2 && self.graph.block(target).size_out() <= 1 {
                    let inslot = self.graph.block(curbl).get_out_rev_index(0);
                    if self.graph.block(target).get_in(1 - inslot) == bl {
                        fallthru.push(curbl);
                    }
                }
            }
            if nonfallthru > 1 {
                return Ok(false); // Can have at most 1 other exit block
            }
        }
        if fallthru.is_empty() {
            return Ok(false); // No fall thru candidates
        }

        // Mark the fallthru edges as gotos
        for i in 0..fallthru.len() {
            let curbl = fallthru[i];
            self.graph.set_goto_branch(curbl, 0)?;
        }

        Ok(true)
    }

    /// The main collapsing loop (C++ `CollapseStructure::collapseInternal`,
    /// `blockaction.cc:1768`).  Collapse everything until no additional rules
    /// apply; if handed a particular `targetbl`, try simplifying from it first.
    /// Returns the count of \e isolated FlowBlocks.
    fn collapse_internal(&mut self, mut targetbl: Option<BlockId>) -> KunaResult<int4> {
        let mut isolated_count;
        let mut fullchange;

        loop {
            let mut change;
            loop {
                change = false;
                let mut index = 0;
                isolated_count = 0;
                while index < self.size() {
                    let bl;
                    if targetbl.is_none() {
                        bl = self.block_at(index);
                        index += 1;
                    } else {
                        bl = targetbl.unwrap(); // Pick out targeted block
                        change = true; // force a change so we still go through all blocks
                        targetbl = None; // Only target the block once
                        index = self.size();
                    }
                    if self.graph.block(bl).size_in() == 0 && self.graph.block(bl).size_out() == 0 {
                        // A completely collapsed block
                        isolated_count += 1;
                        continue; // This does not constitute a change
                    }
                    // Try each rule on the block, in C++ schema-precedence order
                    if self.rule_block_goto(bl)? {
                        change = true;
                        continue;
                    }
                    if self.rule_block_cat(bl)? {
                        change = true;
                        continue;
                    }
                    if self.rule_block_proper_if(bl) {
                        change = true;
                        continue;
                    }
                    if self.rule_block_if_else(bl) {
                        change = true;
                        continue;
                    }
                    if self.rule_block_while_do(bl) {
                        change = true;
                        continue;
                    }
                    if self.rule_block_do_while(bl) {
                        change = true;
                        continue;
                    }
                    if self.rule_block_inf_loop(bl) {
                        change = true;
                        continue;
                    }
                    if self.rule_block_switch(bl)? {
                        change = true;
                        continue;
                    }
                    // ruleBlockOr is intentionally NOT in this cascade (see C++ comment)
                }
                if !change {
                    break;
                }
            }
            // Applying IfNoExit too early can cause other (preferable) rules to miss.
            fullchange = false;
            let mut index = 0;
            while index < self.size() {
                let bl = self.block_at(index);
                if self.rule_block_if_no_exit(bl) {
                    fullchange = true;
                    break;
                }
                if self.rule_case_fallthru(bl)? {
                    fullchange = true;
                    break;
                }
                index += 1;
            }
            if !fullchange {
                break;
            }
        }
        Ok(isolated_count)
    }

    /// Simplify just the conditional AND/OR constructions (C++
    /// `CollapseStructure::collapseConditions`, `blockaction.cc:1854`).
    fn collapse_conditions(&mut self) -> KunaResult<()> {
        let mut change = true;
        while change {
            change = false;
            for i in 0..self.size() {
                let bl = self.block_at(i);
                if self.rule_block_or(bl)? {
                    change = true;
                }
            }
        }
        Ok(())
    }

    /// Run the whole algorithm (C++ `CollapseStructure::collapseAll`,
    /// `blockaction.cc:1877`).  Collapse everything to isolated blocks.
    pub fn collapse_all(&mut self) -> KunaResult<()> {
        self.finaltrace = false;
        let graph_id = self.graph_id;
        self.graph.clear_visit_count(graph_id);
        self.order_loop_bodies();

        self.collapse_conditions()?;

        let mut isolated_count = self.collapse_internal(None)?;
        while isolated_count < self.size() {
            let targetbl = self.select_goto()?;
            isolated_count = self.collapse_internal(targetbl)?;
        }
        Ok(())
    }
}

// ===========================================================================
// ConditionalJoin (blockaction.hh:234-265, blockaction.cc:1898-2108)
// ===========================================================================

/// Last op of a basic block (C++ `BlockBasic::lastOp` = `op.back()`).  Reads the
/// [`BasicData::op_tail`](crate::block::BasicData) of the block in `bblocks`.
fn last_op_of(data: &Funcdata, bl: BlockId) -> Option<crate::seams::OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_tail,
        _ => None,
    }
}

/// First op of a basic block (C++ `BlockBasic::beginOp` = `op.front()`).
fn first_op_of(data: &Funcdata, bl: BlockId) -> Option<crate::seams::OpId> {
    match data.bblocks_ref().block(bl).kind() {
        crate::block::BlockKind::Basic(bd) => bd.op_head,
        _ => None,
    }
}

/// The next op in the same basic block (C++ `++iter` over `op` list).
fn next_op_in_block(data: &Funcdata, op: crate::seams::OpId) -> Option<crate::seams::OpId> {
    let (_prev, next) = data.obank().get(op).expect("next_op_in_block: stale op").basic_neighbours();
    next
}

/// \brief A pair of Varnode objects that have been split (and should be merged)
/// (C++ `ConditionalJoin::MergePair`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergePair {
    /// Varnode coming from block1 (C++ `Varnode *side1`)
    pub side1: VarnodeId,
    /// Varnode coming from block2 (C++ `Varnode *side2`)
    pub side2: VarnodeId,
}

/// \brief Discover and eliminate \e split conditions (C++ `ConditionalJoin`).
///
/// A \b split condition is when a conditional expression (a CBRANCH) is
/// duplicated across two blocks that would otherwise merge.  This recognizes the
/// pattern ([`match`](ConditionalJoin::match_blocks)) and rejoins it
/// ([`execute`](ConditionalJoin::execute)).
///
/// # Faithfulness
///
/// The **match path** ([`match_blocks`](ConditionalJoin::match_blocks),
/// [`find_dups`](ConditionalJoin::find_dups), [`check_exit_block`]) is read-only
/// topology + the [`functional_equality_level`](crate::expression::functional_equality_level)
/// probe, fully ported.  The **execute path** (`nodeJoinCreateBlock` +
/// MULTIEQUAL creation/cutdown + CBRANCH move) reproduces the C++ mutation
/// sequence but threads the `Funcdata::opSetOutput` (LOSS shared with condexe)
/// and `opSetOpcode` (W6 op-info) seams already documented in the rule wave — so
/// [`execute`](ConditionalJoin::execute) returns a `KunaResult` and the seam is
/// visible.  See losses.
pub struct ConditionalJoin {
    block1: Option<BlockId>,
    block2: Option<BlockId>,
    exita: Option<BlockId>,
    exitb: Option<BlockId>,
    a_in1: int4,
    a_in2: int4,
    b_in1: int4,
    b_in2: int4,
    cbranch1: Option<crate::seams::OpId>,
    cbranch2: Option<crate::seams::OpId>,
    /// The new joined condition block (C++ `BlockBasic *joinblock`).  Written by
    /// the seamed `execute` (see [`ConditionalJoin::execute`]); carried for layout
    /// fidelity until the op-creation seam lands.
    #[allow(dead_code)]
    joinblock: Option<BlockId>,
    /// Map from the MergePair of Varnodes to the merged Varnode (C++
    /// `map<MergePair,Varnode *> mergeneed`).  The C++ keys on a lexicographic
    /// compare of the create-indices; we carry the same order via an ordered
    /// `Vec` of `(MergePair, Option<VarnodeId>)` (the `BTree`-only convention with
    /// a non-`Ord` key resolved by the explicit ordered insert in
    /// [`merge_need_get_or_insert`](ConditionalJoin::merge_need_get_or_insert)).
    mergeneed: Vec<(MergePair, Option<VarnodeId>)>,
}

impl Default for ConditionalJoin {
    fn default() -> Self {
        ConditionalJoin::new()
    }
}

impl ConditionalJoin {
    /// Constructor (C++ `ConditionalJoin::ConditionalJoin`).
    pub fn new() -> ConditionalJoin {
        ConditionalJoin {
            block1: None,
            block2: None,
            exita: None,
            exitb: None,
            a_in1: 0,
            a_in2: 0,
            b_in1: 0,
            b_in2: 0,
            cbranch1: None,
            cbranch2: None,
            joinblock: None,
            mergeneed: Vec::new(),
        }
    }

    /// Compare two MergePairs lexicographically by create-index of `side1` then
    /// `side2` (C++ `MergePair::operator<`, `blockaction.cc:1898`).
    fn merge_pair_less(a: &MergePair, b: &MergePair, data: &Funcdata) -> std::cmp::Ordering {
        let s1 = data.vbank().get(a.side1).expect("mergePair: stale side1").get_create_index();
        let s2 = data.vbank().get(b.side1).expect("mergePair: stale side1").get_create_index();
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        let a2 = data.vbank().get(a.side2).expect("mergePair: stale side2").get_create_index();
        let b2 = data.vbank().get(b.side2).expect("mergePair: stale side2").get_create_index();
        a2.cmp(&b2)
    }

    /// Insert a MergePair into the ordered `mergeneed` map (or find it), keeping
    /// the C++ `map<MergePair,...>` create-index ordering.  Returns the index in
    /// `mergeneed`.
    fn merge_need_get_or_insert(&mut self, mp: MergePair, data: &Funcdata) -> usize {
        // Find existing
        for (i, (k, _)) in self.mergeneed.iter().enumerate() {
            if *k == mp {
                return i;
            }
        }
        // Insert preserving order
        let pos = self
            .mergeneed
            .iter()
            .position(|(k, _)| ConditionalJoin::merge_pair_less(&mp, k, data) == std::cmp::Ordering::Less)
            .unwrap_or(self.mergeneed.len());
        self.mergeneed.insert(pos, (mp, None));
        pos
    }

    /// Search for duplicate conditional expressions (C++ `ConditionalJoin::findDups`,
    /// `blockaction.cc:1912`).
    fn find_dups(&mut self, data: &Funcdata) -> bool {
        let b1 = self.block1.expect("findDups: block1");
        let b2 = self.block2.expect("findDups: block2");
        let cbranch1 = match last_op_of(data, b1) {
            Some(op) => op,
            None => return false,
        };
        if data.obank().get(cbranch1).expect("findDups: cbranch1").code() != OpCode::CPUI_CBRANCH {
            return false;
        }
        let cbranch2 = match last_op_of(data, b2) {
            Some(op) => op,
            None => return false,
        };
        if data.obank().get(cbranch2).expect("findDups: cbranch2").code() != OpCode::CPUI_CBRANCH {
            return false;
        }
        self.cbranch1 = Some(cbranch1);
        self.cbranch2 = Some(cbranch2);

        if data.obank().get(cbranch1).unwrap().is_boolean_flip() {
            return false; // flip hasn't propagated through yet
        }
        if data.obank().get(cbranch2).unwrap().is_boolean_flip() {
            return false;
        }

        let vn1 = data.obank().get(cbranch1).unwrap().get_in(1).expect("findDups: cbranch1 in1");
        let vn2 = data.obank().get(cbranch2).unwrap().get_in(1).expect("findDups: cbranch2 in1");

        if vn1 == vn2 {
            return true;
        }

        // Parallel RulePushMulti, so we know it will apply if we do the join
        if !data.vbank().get(vn1).unwrap().is_written() {
            return false;
        }
        if !data.vbank().get(vn2).unwrap().is_written() {
            return false;
        }
        if data.vbank().get(vn1).unwrap().is_spacebase() {
            return false;
        }
        if data.vbank().get(vn2).unwrap().is_spacebase() {
            return false;
        }
        let mut buf1 = [vn1, vn1];
        let mut buf2 = [vn2, vn2];
        let res = crate::expression::functional_equality_level(
            vn1,
            vn2,
            &mut buf1,
            &mut buf2,
            data.vbank(),
            data.obank(),
        );
        if res < 0 {
            return false;
        }
        if res > 1 {
            return false;
        }
        let op1 = data.vbank().get(vn1).unwrap().get_def().expect("findDups: vn1 def");
        let op1c = data.obank().get(op1).unwrap().code();
        if op1c == OpCode::CPUI_SUBPIECE {
            return false;
        }
        if op1c == OpCode::CPUI_COPY {
            return false;
        }

        self.merge_need_get_or_insert(MergePair { side1: vn1, side2: vn2 }, data);
        true
    }

    /// Test blocks for the merge condition (C++ `ConditionalJoin::match`,
    /// `blockaction.cc:2065`).
    pub fn match_blocks(&mut self, b1: BlockId, b2: BlockId, data: &Funcdata) -> bool {
        self.block1 = Some(b1);
        self.block2 = Some(b2);
        // Check for the ConditionalJoin block pattern
        if b2 == b1 {
            return false;
        }
        let bb = data.bblocks_ref();
        if bb.block(b1).size_out() != 2 {
            return false;
        }
        if bb.block(b2).size_out() != 2 {
            return false;
        }
        let exita = bb.block(b1).get_out(0);
        let exitb = bb.block(b1).get_out(1);
        if exita == exitb {
            return false;
        }
        if bb.block(b2).get_out(0) != exita {
            return false; // False exits must match
        }
        if bb.block(b2).get_out(1) != exitb {
            return false; // True exits must match
        }
        self.exita = Some(exita);
        self.exitb = Some(exitb);
        self.a_in2 = bb.block(b2).get_out_rev_index(0);
        self.b_in2 = bb.block(b2).get_out_rev_index(1);
        self.a_in1 = bb.block(b1).get_out_rev_index(0);
        self.b_in1 = bb.block(b1).get_out_rev_index(1);

        if !self.find_dups(data) {
            self.clear();
            return false;
        }
        self.check_exit_block(exita, self.a_in1, self.a_in2, data);
        self.check_exit_block(exitb, self.b_in1, self.b_in2, data);
        true
    }

    /// \brief Look for additional Varnode pairs in an exit block that need to be
    /// merged (C++ `ConditionalJoin::checkExitBlock`, `blockaction.cc:1954`).
    fn check_exit_block(&mut self, exit: BlockId, in1: int4, in2: int4, data: &Funcdata) {
        let mut iter = first_op_of(data, exit);
        while let Some(op) = iter {
            let next = next_op_in_block(data, op);
            let code = data.obank().get(op).unwrap().code();
            if code == OpCode::CPUI_MULTIEQUAL {
                // Anything merging from our two root blocks
                let vn1 = data.obank().get(op).unwrap().get_in(in1).expect("checkExitBlock: in1");
                let vn2 = data.obank().get(op).unwrap().get_in(in2).expect("checkExitBlock: in2");
                if vn1 != vn2 {
                    self.merge_need_get_or_insert(MergePair { side1: vn1, side2: vn2 }, data);
                }
            } else if code != OpCode::CPUI_COPY {
                break;
            }
            iter = next;
        }
    }

    /// Execute the merge (C++ `ConditionalJoin::execute`, `blockaction.cc:2094`).
    ///
    /// SEAM: the mutation sequence (`nodeJoinCreateBlock`, `setupMultiequals` →
    /// `newOp`/`newUniqueOut`/`opSetOutput`, `moveCbranch`, `cutDownMultiequals` →
    /// `opSetOpcode`) needs the `opSetOutput` (LOSS, shared with condexe) and
    /// `opSetOpcode` W6 op-info seams.  The block-creation half
    /// (`nodeJoinCreateBlock`) is available; the op-creation/opcode half is not,
    /// so this surfaces the seam.  See losses.
    pub fn execute(&mut self, _data: &mut Funcdata) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "kuna rust port: ConditionalJoin::execute needs Funcdata::opSetOutput (newUniqueOut) \
             and opSetOpcode (W6 op-info table) to build/rewrite the join MULTIEQUALs; not \
             available at this item's boundary",
        ))
    }

    /// Clear for a new test (C++ `ConditionalJoin::clear`, `blockaction.cc:2104`).
    pub fn clear(&mut self) {
        self.mergeneed.clear();
    }
}

// ===========================================================================
// Structuring Actions (blockaction.hh:270-360, blockaction.cc:2110-2365)
// ===========================================================================

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::seams::VarnodeId;

/// \brief Give each control-flow structure an opportunity to make a final
/// transform (C++ `ActionStructureTransform`).  Currently sets up \e for loops
/// via BlockWhileDo.
pub struct ActionStructureTransform {
    base: ActionBase,
    /// Are p-code ops allowed to be moved by \b this action (C++ `allowOpMoves`)
    allow_op_moves: bool,
}

impl ActionStructureTransform {
    /// Construct in group `g` (C++ `ActionStructureTransform::ActionStructureTransform`).
    pub fn boxed(g: impl Into<String>, allow_moves: bool) -> Box<dyn Action> {
        Box::new(ActionStructureTransform {
            base: ActionBase::new(0, "structuretransform", g),
            allow_op_moves: allow_moves,
        })
    }
}

impl Action for ActionStructureTransform {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionStructureTransform {
            base: self.base.clone(),
            allow_op_moves: self.allow_op_moves,
        }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: data.getStructure().finalTransform(data,allowOpMoves); return 0;
        // SEAM(W7/W8): `BlockGraph::finalTransform` (the for-loop / op-move setup
        // over BlockWhileDo nodes) is not ported in `block.rs`.  See losses.
        let _ = self.allow_op_moves;
        0
    }
}

/// \brief Flip conditional control-flow so that \e preferred comparison
/// operators are used (C++ `ActionNormalizeBranches`).
pub struct ActionNormalizeBranches {
    base: ActionBase,
}

impl ActionNormalizeBranches {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNormalizeBranches { base: ActionBase::new(0, "normalizebranches", g) })
    }
}

impl Action for ActionNormalizeBranches {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionNormalizeBranches { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ iterates the basic blocks, for each 2-out block whose last op is a
        // CBRANCH it runs opFlipInPlaceTest and, on a clean (==0) test,
        // opFlipInPlaceExecute + bb->flipInPlaceExecute().
        //
        // SEAM(W7): `Funcdata::opFlipInPlaceExecute` is itself seamed
        // (funcdata_op.rs) — it needs `replaceLessequal`/`newConstant`; and
        // `BlockBasic::flipInPlaceExecute` (the op-flag flip + swapEdges) is the
        // block-side half left as W7 in `block.rs`.  The control structure is
        // transcribed; the mutation is the seam.  See losses.
        let graph = data.bblocks_ref();
        let root = graph.root.expect("ActionNormalizeBranches: bblocks root");
        let n = graph.block(root).get_size();
        for i in 0..n {
            let bb = data.bblocks_ref().block(root).get_block(i);
            if data.bblocks_ref().block(bb).size_out() != 2 {
                continue;
            }
            let cbranch = match last_op_of(data, bb) {
                Some(op) => op,
                None => continue,
            };
            if data.obank().get(cbranch).unwrap().code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let mut fliplist: Vec<crate::seams::OpId> = Vec::new();
            if data.op_flip_in_place_test(cbranch, &mut fliplist, true) != 0 {
                continue;
            }
            // opFlipInPlaceExecute + bb->flipInPlaceExecute -- SEAM(W7) (see above)
            // (skipped: the executes are seamed; no change recorded)
        }
        // data.clearDeadOps();  -- SEAM(W7): clearDeadOps not in merged tree.
        0
    }
}

/// \brief Attempt to normalize symmetric block structures (C++
/// `ActionPreferComplement`).  Uses `preferComplement()` on structured FlowBlocks
/// to choose between symmetric structurings.
pub struct ActionPreferComplement {
    base: ActionBase,
    /// Are p-code ops allowed to be modified by \b this action (C++ `allowOpMods`)
    allow_op_mods: bool,
}

impl ActionPreferComplement {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>, allow_mods: bool) -> Box<dyn Action> {
        Box::new(ActionPreferComplement {
            base: ActionBase::new(0, "prefercomplement", g),
            allow_op_mods: allow_mods,
        })
    }
}

impl Action for ActionPreferComplement {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionPreferComplement {
            base: self.base.clone(),
            allow_op_mods: self.allow_op_mods,
        }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ walks the structured-block tree and calls
        // curbl->preferComplement(data,allowOpMods) on each BlockGraph node.
        //
        // SEAM(W7/W8): `FlowBlock::preferComplement` (the symmetric-structuring
        // choice via flipInPlaceTest/Execute) is left as W7 in `block.rs`; not
        // available at this item's boundary.  See losses.
        let _ = self.allow_op_mods;
        0
    }
}

/// \brief Structure control-flow using standard high-level code constructs (C++
/// `ActionBlockStructure`).  Drives [`CollapseStructure`].
pub struct ActionBlockStructure {
    base: ActionBase,
}

impl ActionBlockStructure {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionBlockStructure { base: ActionBase::new(0, "blockstructure", g) })
    }
}

impl Action for ActionBlockStructure {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionBlockStructure { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ (blockaction.cc:2170):
        //   BlockGraph &graph(data.getStructure());
        //   if (graph.getSize() != 0) return 0;           // already structured
        //   data.installSwitchDefaults();
        //   graph.buildCopy(data.getBasicBlocks());
        //   CollapseStructure collapse(graph); collapse.collapseAll();
        //   count += collapse.getChangeCount();
        if data.sblocks_get_size() != 0 {
            return 0; // Already structured
        }
        data.install_switch_defaults();
        // graph.buildCopy(data.getBasicBlocks());  -- cross-arena copy: mirror
        // every bblocks BlockBasic into sblocks as a BlockCopy leaf (the copy's
        // `copy` field points back at the bblocks block so the printer can walk
        // its op list).
        data.seed_sblocks_copy();
        // CollapseStructure collapse(graph); collapse.collapseAll();
        let sroot = data.sblocks_root();
        let mut collapse = CollapseStructure::new(data.sblocks_mut(), sroot);
        let collapse_res = collapse.collapse_all();
        let cc = collapse.get_change_count();
        // Realize the deferred data-flow half of BlockBasic::negateCondition: each
        // recorded bblocks block has its trailing CBRANCH `boolean_flip`/
        // `fallthru_true` toggled (XOR-reduced — even flips cancel).  This must run
        // whether or not `collapse_all` Err'd, because the flips that were already
        // recorded reflect topology changes the collapse committed before aborting.
        let flips = collapse.take_pending_flips();
        drop(collapse); // release the sblocks borrow before touching the obank
        for bb in flips {
            data.block_basic_negate_lastop(bb);
        }
        if let Err(e) = collapse_res {
            // A still-un-ported collapse sub-seam (switch/node-creation) aborts;
            // surface it as "no change made" rather than taking down the run, the
            // documented honest-partial-parity degradation.  See losses.
            let _ = e;
            return 0;
        }
        self.base_mut().count += cc;
        0
    }
}

/// \brief Perform final organization of the control-flow structure (C++
/// `ActionFinalStructure`).  Label unstructured edges, order switch cases, and
/// order disjoint components.
pub struct ActionFinalStructure {
    base: ActionBase,
}

impl ActionFinalStructure {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionFinalStructure { base: ActionBase::new(0, "finalstructure", g) })
    }
}

impl Action for ActionFinalStructure {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionFinalStructure { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++:
        //   graph.orderBlocks();
        //   graph.finalizePrinting(data);
        //   graph.scopeBreak(-1,-1);     // break statements
        //   graph.markUnstructured();    // gotos
        //   graph.markLabelBumpUp(false);// label fixup
        //
        // SEAM(W7/W8): all five `BlockGraph` methods are left unported in
        // `block.rs` (`orderBlocks`/`finalizePrinting`/`scopeBreak`/
        // `markUnstructured`/`markLabelBumpUp` — the print-prep + goto/break
        // labeling pass).  Recorded as losses; the action shell is in place so the
        // schedule can run it as a no-op until block.rs supplies them.
        0
    }
}

/// \brief Look for conditional branch expressions that have been split and
/// rejoin them (C++ `ActionNodeJoin`).  Drives [`ConditionalJoin`].
pub struct ActionNodeJoin {
    base: ActionBase,
}

impl ActionNodeJoin {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNodeJoin { base: ActionBase::new(0, "nodejoin", g) })
    }
}

impl Action for ActionNodeJoin {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionNodeJoin { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ (blockaction.cc:2327): iterate the basic blocks; for each 2-out
        // block bb, pick the out block with the fewer in-edges (`leastout`), and
        // if it has >1 in-edge, try ConditionalJoin::match(bb, sibling) for every
        // other in-edge.  On a match: count+=1; execute(); clear().
        let graph = data.bblocks_ref();
        if graph.root.is_none() {
            return 0;
        }
        let root = graph.root.unwrap();
        if graph.block(root).get_size() == 0 {
            return 0;
        }
        let mut condjoin = ConditionalJoin::new();
        let n = data.bblocks_ref().block(root).get_size();
        for i in 0..n {
            let bb = data.bblocks_ref().block(root).get_block(i);
            if data.bblocks_ref().block(bb).size_out() != 2 {
                continue;
            }
            let out1 = data.bblocks_ref().block(bb).get_out(0);
            let out2 = data.bblocks_ref().block(bb).get_out(1);
            let (leastout, inslot);
            if data.bblocks_ref().block(out1).size_in() < data.bblocks_ref().block(out2).size_in() {
                leastout = out1;
                inslot = data.bblocks_ref().block(bb).get_out_rev_index(0);
            } else {
                leastout = out2;
                inslot = data.bblocks_ref().block(bb).get_out_rev_index(1);
            }
            if data.bblocks_ref().block(leastout).size_in() == 1 {
                continue;
            }

            let size_in = data.bblocks_ref().block(leastout).size_in();
            for j in 0..size_in {
                if j == inslot {
                    continue;
                }
                let bb2 = data.bblocks_ref().block(leastout).get_in(j);
                if condjoin.match_blocks(bb, bb2, data) {
                    // count += 1; condjoin.execute(); condjoin.clear();
                    // SEAM: execute() surfaces opSetOutput/opSetOpcode (see
                    // ConditionalJoin::execute).  The match is fully ported.
                    match condjoin.execute(data) {
                        Ok(()) => {
                            self.base_mut().count += 1;
                            condjoin.clear();
                            break;
                        }
                        Err(_) => {
                            // The execute seam is not available; do not mutate.
                            condjoin.clear();
                            break;
                        }
                    }
                }
            }
        }
        0
    }
}

/// \brief Split the epilog code of the function (C++ `ActionReturnSplit`).
/// Introduce RETURN operations corresponding to individual branches flowing to
/// the epilog.
pub struct ActionReturnSplit {
    base: ActionBase,
}

impl ActionReturnSplit {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionReturnSplit { base: ActionBase::new(0, "returnsplit", g) })
    }
}

impl Action for ActionReturnSplit {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionReturnSplit { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ (blockaction.cc:2265): iterate `data.beginOp(CPUI_RETURN)`, gather
        // goto-return blocks (`gatherReturnGotos`), pick splittable RETURN blocks
        // (`isSplittable`), and `data.nodeSplit(retnode[i],splitedge[i])`.
        //
        // SEAM(W7/W4): the op-by-opcode iteration (`beginOp(CPUI_RETURN)`), the
        // copy-map traversal `getCopyMap()`/`getGotoTarget()`/`subBlock` chain in
        // `gatherReturnGotos`, and `Funcdata::nodeSplit` (p-code cloning across an
        // edge) are not available in the merged tree.  Recorded as losses; the
        // action shell is in place.
        let _ = data;
        0
    }
}

#[cfg(test)]
mod tests;
