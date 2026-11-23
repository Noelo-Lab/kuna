//! S8 print-time goto census (kuna `gotonext`).
//!
//! Upstream `PrintC::emitBlockGoto` prints a `BlockGoto`'s trailing jump only when
//! `BlockGoto::gotoPrints` holds: the goto's target is not the block that runs
//! next anyway (`nextFlowAfter` on the parent chain).  kuna's printer emitted
//! every such jump, so a case arm falling into the next case and an `if` arm
//! ending in a jump to the statement after the `if` printed `goto label_X;`
//! directly above `label_X:`.  The structuring passes that run after
//! `scopeBreak` also leave jumps whose target is the statement after the
//! innermost enclosing loop or switch, which C spells `break;`.
//!
//! [`goto_rewrites`] runs on the final structured tree at print time, so it sees
//! the same layout the printer emits.  It returns the redundant `BlockGoto`
//! jumps, the goto carriers that print as `break;`, and the labels no printed
//! `goto` reaches any more, so every printed `goto` keeps its label.  The label
//! carriers are the ones `BlockGraph::mark_unstructured` marks: a `BlockGoto`,
//! a goto-`BlockIf` and a bare-goto switch arm, each of type `f_goto_goto`, and
//! the edges of a `BlockMultiGoto`, which always keep their labels.

use crate::block::{block_flags, BlockGraph, BlockKind, BlockType};
use crate::p8_structure::kuna_switchmerge::SwitchPlans;
use crate::context::BlockId;
use std::collections::BTreeSet;

#[derive(Default)]
pub(crate) struct GotoRewrites {
    /// `BlockGoto` nodes whose jump is left out.
    pub silent: BTreeSet<BlockId>,
    /// `BlockGoto` and goto-`BlockIf` nodes printed as `break;`.
    pub breaks: BTreeSet<BlockId>,
    /// Switch arms that are a bare goto printed as `break;`: (switch, case block).
    pub case_breaks: BTreeSet<(BlockId, BlockId)>,
    /// Label leaves left unprinted.
    pub labels: BTreeSet<BlockId>,
}

impl GotoRewrites {
    /// The goto type a `BlockGoto` or goto-`BlockIf` node prints with.
    pub(crate) fn gototype(&self, carrier: BlockId, gototype: u32) -> u32 {
        if gototype == block_flags::f_goto_goto && self.breaks.contains(&carrier) {
            block_flags::f_break_goto
        } else {
            gototype
        }
    }

    /// The goto type a bare-goto switch arm prints with.
    pub(crate) fn case_gototype(&self, switch: BlockId, case: BlockId, gototype: u32) -> u32 {
        if gototype == block_flags::f_goto_goto && self.case_breaks.contains(&(switch, case)) {
            block_flags::f_break_goto
        } else {
            gototype
        }
    }
}

/// Where a `break;` at `id` would go: the block after the innermost loop or
/// switch whose body holds `id`; a merged switch breaks out of its host.
fn break_target(graph: &BlockGraph, plans: &SwitchPlans, id: BlockId) -> Option<BlockId> {
    let mut child = id;
    let mut cur = graph.block(id).get_parent()?;
    loop {
        let blk = graph.block(cur);
        // A while-do's condition block holds a goto only in the overflow form,
        // which prints it inside `while( true ) { ... }`; a switch's block 0 prints
        // before the `switch`.
        let body = match blk.get_type() {
            BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => true,
            BlockType::Switch => graph.sub_block(cur, 0) != Some(child),
            _ => false,
        };
        if body {
            let scope = plans.hosts.get(&cur).copied().unwrap_or(cur);
            let parent = graph.block(scope).get_parent()?;
            return graph.next_printed_flow_after(parent, scope);
        }
        child = cur;
        cur = blk.get_parent()?;
    }
}

/// The goto rewrites for the structured tree under `root`; `breaks` is option
/// `loopbreak_recovery`, which also decides whether a jump prints as `break;`.
/// `plans` holds the switch arms the printer moves or leaves out.
pub(crate) fn goto_rewrites(graph: &BlockGraph, root: BlockId, breaks: bool, plans: &SwitchPlans) -> GotoRewrites {
    let mut out = GotoRewrites::default();
    let mut released = BTreeSet::new();
    let mut kept = BTreeSet::new();
    let leaf = |t: BlockId| graph.get_front_leaf(t);
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let blk = graph.block(id);
        for i in 0..blk.get_size() {
            stack.push(blk.get_block(i));
        }
        let goto = match blk.get_type() {
            BlockType::Goto => blk.get_goto_target().map(|t| (t, blk.get_goto_type())),
            _ => blk.get_if_goto_target().map(|t| (t, blk.get_if_goto_type())),
        };
        if let Some((target, gototype)) = goto {
            let t = leaf(target);
            let next = match blk.get_type() {
                BlockType::Goto => blk.get_parent().and_then(|par| graph.next_printed_flow_after(par, id)),
                _ => None,
            };
            if t.is_some() && next == t {
                out.silent.insert(id);
                released.extend(t);
            } else if breaks && t.is_some() && gototype == block_flags::f_goto_goto && break_target(graph, plans, id) == t {
                out.breaks.insert(id);
                released.extend(t);
            } else if gototype == block_flags::f_goto_goto {
                kept.extend(t);
            }
        }
        // A switch's own `BlockMultiGoto` lists the case edges handled below.
        if let BlockKind::MultiGoto { gotoedges, .. } = blk.kind() {
            if blk.get_parent().is_none_or(|p| graph.block(p).get_type() != BlockType::Switch) {
                kept.extend(gotoedges.iter().filter_map(|&t| leaf(t)));
            }
        }
        let cases = blk.switch_caseblocks();
        if cases.iter().any(|c| c.gototype == block_flags::f_goto_goto) {
            let scope = plans.hosts.get(&id).copied().unwrap_or(id);
            let after = graph.block(scope).get_parent().and_then(|par| graph.next_printed_flow_after(par, scope));
            for case in cases.iter().filter(|c| c.gototype == block_flags::f_goto_goto) {
                let t = leaf(case.block);
                if plans.dropped.contains(&(id, case.block)) {
                    released.extend(t);
                } else if breaks && t.is_some() && after == t {
                    out.case_breaks.insert((id, case.block));
                    released.extend(t);
                } else {
                    kept.extend(t);
                }
            }
        }
    }
    out.labels = released.difference(&kept).copied().collect();
    out
}
