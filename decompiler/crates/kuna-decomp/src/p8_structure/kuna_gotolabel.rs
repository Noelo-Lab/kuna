//! S8 label bookkeeping shared by the goto-reduction passes (kuna).
//!
//! # The defect this closes
//!
//! Every emitted `goto label_X;` needs a `label_X:` in the same function, or the
//! output is not C.  The printer emits the label from the `f_unstructured_targ`
//! flag [`BlockGraph::mark_unstructured`](crate::block::BlockGraph::mark_unstructured)
//! sets on each goto target, and the three goto-reduction passes that convert a
//! `goto` into a duplicated tail
//! ([`kuna_gotoreduce`](crate::p8_structure::kuna_gotoreduce),
//! [`kuna_taildup`](crate::p8_structure::kuna_taildup),
//! [`kuna_crossjumpreverter`](crate::p8_structure::kuna_crossjumpreverter)) clear
//! that flag once the converted target is no longer reached by an unstructured
//! edge.  Each pass carried its own copy of the "still reached" test, and all
//! three copies looked at `BlockGoto`/`BlockIf` goto targets only — so a target
//! still reached by a **switch case** goto or a `BlockMultiGoto` virtual edge read
//! as unreferenced.  On `coreutils/sort` (O2, stripped) `main` lost the label of
//! the block its switch default arm jumps to and printed
//!
//! ```c
//!     switch(v52) {
//!       default:
//!         goto label_661d;      // no `label_661d:` anywhere in the function
//! ```
//!
//! [`referenced_goto_targets`] is the one complete test: it walks the structured
//! tree and collects **every** carrier of an unstructured edge — `BlockGoto`,
//! `BlockIf`-goto, `BlockSwitch` case edges (`CaseOrder::gototype`, what
//! `BlockSwitch::markUnstructured` marks) and `BlockMultiGoto::gotoedges`.  It is
//! only ever used to *decline* a label clear, so it can only keep a label the
//! printer would otherwise drop.

use crate::block::{block_flags, BlockGraph, BlockKind};
use crate::context::BlockId;
use crate::funcdata::Funcdata;

/// Every sblock still targeted by an unstructured edge somewhere in the
/// structured tree rooted at `root`.
///
/// Mirrors the carriers [`BlockGraph::mark_unstructured`](crate::block::BlockGraph::mark_unstructured)
/// marks: a `BlockGoto` target, a `BlockIf` goto target, a `BlockSwitch` case
/// whose edge is a real goto, and every `BlockMultiGoto` virtual edge.  Targets
/// are recorded as given (the block the carrier names), matching how the passes
/// address the flag they clear.
pub(crate) fn referenced_goto_targets(graph: &BlockGraph, root: BlockId) -> std::collections::BTreeSet<BlockId> {
    let mut set = std::collections::BTreeSet::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let blk = graph.block(id);
        for i in 0..blk.get_size() {
            stack.push(blk.get_block(i));
        }
        if let Some(t) = blk.get_if_goto_target() {
            set.insert(t);
        }
        if let Some(t) = blk.get_goto_target() {
            set.insert(t);
        }
        // BlockSwitch: a case arm whose edge stayed unstructured prints its own
        // `goto` (printc `emit_block_switch_c`), so its target still needs a label.
        for case in blk.switch_caseblocks() {
            if case.gototype == block_flags::f_goto_goto {
                set.insert(case.block);
            }
        }
        // BlockMultiGoto: the virtual edges a switch reconversion reads back.
        if let BlockKind::MultiGoto { gotoedges, .. } = blk.kind() {
            for &t in gotoedges {
                set.insert(t);
            }
        }
    }
    set
}

/// Clear `f_unstructured_targ` on each converted goto target that no carrier in
/// the (already mutated) structured tree still points at — the shared tail of the
/// three goto-reduction passes.
///
/// The flag lives on the target's **front leaf**
/// ([`BlockGraph::mark_copy_block`](crate::block::BlockGraph)), and two carriers
/// can name different blocks — a `BlockList` and the `BlockCopy` it opens with —
/// that resolve to that same leaf, so the surviving-carrier test compares front
/// leaves.  Comparing the named blocks dropped the label of a leaf a surviving
/// `goto` reached through its enclosing list.
///
/// The clear itself addresses the block the pass names, not that block's front
/// leaf, so a release whose target is composite leaves the flag set and the
/// label printed.  That asymmetry is deliberate: a label no `goto` reaches is
/// legal C, a `goto` with no label is not, and tightening the clear would
/// remove statements rather than fix a label.
pub(crate) fn release_converted_labels(data: &mut Funcdata, converted: &[BlockId]) {
    if converted.is_empty() {
        return;
    }
    let root = data.sblocks_root();
    let graph = data.sblocks_ref();
    let still: std::collections::BTreeSet<BlockId> = referenced_goto_targets(graph, root)
        .into_iter()
        .filter_map(|t| graph.get_front_leaf(t))
        .collect();
    let clears: Vec<BlockId> = converted
        .iter()
        .copied()
        .filter(|&t| graph.get_front_leaf(t).map(|l| !still.contains(&l)).unwrap_or(false))
        .collect();
    for t in clears {
        data.sblocks_mut().block_mut(t).clear_flag(block_flags::f_unstructured_targ);
    }
}
