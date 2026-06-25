//! Port of Ghidra `BlockGraph::scopeBreak` (`block.cc:1271`) and the per-block
//! `scopeBreak` overrides — loop-exit `goto` -> `break;` recovery (the angr
//! Phoenix/SAILR "loop-successor refinement"; Basque et al., USENIX Security
//! 2024 "SAILR", S8 structuring).
//!
//! # The structural gap
//!
//! After the structurer collapses the CFG into a structured AST, an edge that
//! leaves a loop to the loop's *single successor* is the semantic `break;`.
//! kuna's port of `ActionFinalStructure` ran `finalizePrinting` and
//! `markUnstructured` but left `scopeBreak` an explicit SEAM stub
//! (`s8_structure/blockaction.rs`, `docs/rust-port/losses.md`): every such
//! loop-exit edge therefore rendered as a raw `goto <successor-label>;` plus a
//! synthesized label on the successor.  angr recovers the structured `break;`.
//!
//! On `1after909::doit` the command-processing `while` loop has **nine**
//! error/`quit` paths that exit to the shared cleanup block; kuna emitted nine
//! `goto label_239f;` + the label, where angr emits nine `break;`.
//!
//! # What this pass does
//!
//! [`kuna_scope_break`] is the faithful transcription of `BlockGraph::scopeBreak`
//! and the overrides for every structured block type.  It is a single recursive
//! walk of the already-collapsed structured tree, carrying:
//!
//!   * `curexit` — the structured block that executes next when `this` falls
//!     through (the C++ `getIndex()` of the next sibling in the parent list, or
//!     the parent's own exit at the end of a list);
//!   * `curloopexit` — the *innermost enclosing loop's* successor block (set by
//!     the loop nodes: `BlockWhileDo`/`BlockDoWhile`/`BlockInfLoop`/`BlockSwitch`
//!     rebind it to their own `curexit`).
//!
//! For each `BlockGoto` / goto-carrying `BlockIf` / `BlockSwitch` case whose goto
//! target equals `curloopexit`, the `gototype` flag is rewritten
//! `f_goto_goto -> f_break_goto`.  The printer
//! ([`PrintC::emit_goto_statement`](crate::printc)) already emits `break;` for
//! `f_break_goto` and `markUnstructured` (which runs *after* this pass) then skips
//! marking the now-`break` target's label, so the synthesized label disappears.
//!
//! Block identity: the C++ compares `getIndex()` values; in the kuna arena each
//! structured block has a unique [`BlockId`](crate::block) (1:1 with the C++
//! `index`), so [`BlockId`] equality is the faithful, re-index-stable analog.
//!
//! `scopeBreak` sets only `f_break_goto`; `f_continue_goto` is established earlier
//! during loop construction (the loop back-edge goto), so this pass needs no
//! continue handling.
//!
//! # Gating
//!
//! The decision is a P0 assertion: `option loopbreak_recovery on|off`
//! (`Architecture::recover_loop_break`).  `ActionFinalStructure` calls
//! [`kuna_scope_break`] between `finalizePrinting` and `markUnstructured` only
//! when the flag is set.  kuna ships it **DIV-7 default-on** (the ablation over
//! the 675-assertion datatest corpus plus the cargo golden suite is clean, and
//! it converges kuna toward upstream Ghidra, which runs `scopeBreak`
//! unconditionally); `option loopbreak_recovery off` restores the byte-identical
//! raw-`goto` rendering.
//!
//! ## SEAM(W9) — `ArchOption` wiring
//!
//! Upstream `scopeBreak` is not optional (it always runs).  kuna gates it behind
//! [`OptionLoopBreakRecovery`] so the rendering stays flippable per
//! decompilation; the option parse is exposed here and the flag is threaded by
//! the W9 option dispatch (`architecture.rs`).

use kuna_base::error::KunaResult;
use kuna_base::types::uint4;

use crate::block::{block_flags, BlockGraph, BlockKind, BlockType};
use crate::options::on_or_off;
use crate::seams::BlockId;

use kuna_base::marshal::ElementId;

/// Marshaling element `<loopbreakrecovery>` (kuna).  ElementIds live in the
/// 4000+ range.
pub const ELEM_LOOPBREAK_RECOVERY: ElementId = ElementId::new("loopbreakrecovery", 4091);

//===========================================================================
// scopeBreak: the structured-tree walk that lowers loop-exit gotos to breaks.
//===========================================================================

/// (kuna) Entry point: run `scopeBreak(-1,-1)` over the structured tree rooted
/// at `root` (C++ `BlockGraph::scopeBreak(-1,-1)` as called from
/// `ActionFinalStructure`).
///
/// `None` is the faithful analog of the C++ sentinel index `-1` ("no fixed
/// exit").
pub fn kuna_scope_break(bg: &mut BlockGraph, root: BlockId) {
    scope_break(bg, root, None, None);
}

/// The per-block recursion (C++ virtual `FlowBlock::scopeBreak` dispatched by
/// block type).  `curexit` is the block reached on fall-through; `curloopexit`
/// is the innermost enclosing loop's successor.  A goto whose target equals
/// `curloopexit` is a `break`.
fn scope_break(
    bg: &mut BlockGraph,
    this_id: BlockId,
    curexit: Option<BlockId>,
    curloopexit: Option<BlockId>,
) {
    match bg.block(this_id).get_type() {
        // BlockGraph::scopeBreak (block.cc:1271): each component exits either to
        // the next component in the list, or (at the end) to the parent exit.
        BlockType::Graph | BlockType::Ls => {
            let n = bg.block(this_id).get_size();
            for i in 0..n {
                let curbl = bg.block(this_id).get_block(i);
                let ind = if i + 1 < n {
                    Some(bg.block(this_id).get_block(i + 1))
                } else {
                    curexit
                };
                scope_break(bg, curbl, ind, curloopexit);
            }
        }
        // BlockGoto::scopeBreak (block.cc:2914):
        //   getBlock(0)->scopeBreak(gototarget->getIndex(),curloopexit);
        //   if (curloopexit == gototarget->getIndex()) gototype = f_break_goto;
        BlockType::Goto => {
            let b0 = bg.block(this_id).get_block(0);
            let target = bg.block(this_id).get_goto_target();
            scope_break(bg, b0, target, curloopexit);
            if target.is_some() && target == curloopexit {
                set_goto_type(bg, this_id, block_flags::f_break_goto);
            }
        }
        // BlockMultiGoto::scopeBreak (block.cc:2966): getBlock(0)->scopeBreak(-1,curloopexit).
        BlockType::MultiGoto => {
            let b0 = bg.block(this_id).get_block(0);
            scope_break(bg, b0, None, curloopexit);
        }
        // BlockCondition::scopeBreak (block.cc:3082): both halves, no fixed exit.
        BlockType::Condition => {
            let b0 = bg.block(this_id).get_block(0);
            let b1 = bg.block(this_id).get_block(1);
            scope_break(bg, b0, None, curloopexit);
            scope_break(bg, b1, None, curloopexit);
        }
        // BlockIf::scopeBreak (block.cc:3123):
        //   getBlock(0)->scopeBreak(-1,curloopexit);            // condition
        //   for(i=1;i<size;++i) getBlock(i)->scopeBreak(curexit,curloopexit);
        //   if (gototarget && gototarget->getIndex()==curloopexit) gototype=f_break_goto;
        BlockType::If => {
            let b0 = bg.block(this_id).get_block(0);
            scope_break(bg, b0, None, curloopexit);
            let n = bg.block(this_id).get_size();
            for i in 1..n {
                let bi = bg.block(this_id).get_block(i);
                scope_break(bg, bi, curexit, curloopexit);
            }
            let target = bg.block(this_id).get_if_goto_target();
            if target.is_some() && target == curloopexit {
                set_if_goto_type(bg, this_id, block_flags::f_break_goto);
            }
        }
        // BlockWhileDo::scopeBreak (block.cc:3372): a new loop scope -- the loop's
        // own curexit becomes the body's curloopexit.
        //   getBlock(0)->scopeBreak(-1,curexit);                 // top (condition)
        //   getBlock(1)->scopeBreak(getBlock(0)->getIndex(),curexit); // body
        BlockType::WhileDo => {
            let b0 = bg.block(this_id).get_block(0);
            let b1 = bg.block(this_id).get_block(1);
            scope_break(bg, b0, None, curexit);
            scope_break(bg, b1, Some(b0), curexit);
        }
        // BlockDoWhile::scopeBreak (block.cc:3485): getBlock(0)->scopeBreak(-1,curexit).
        BlockType::DoWhile => {
            let b0 = bg.block(this_id).get_block(0);
            scope_break(bg, b0, None, curexit);
        }
        // BlockInfLoop::scopeBreak (block.cc:3513):
        //   getBlock(0)->scopeBreak(getBlock(0)->getIndex(),curexit); // exits into itself
        BlockType::InfLoop => {
            let b0 = bg.block(this_id).get_block(0);
            scope_break(bg, b0, Some(b0), curexit);
        }
        // BlockSwitch::scopeBreak (block.cc:3664): a new scope, curloopexit=curexit.
        //   getBlock(0)->scopeBreak(-1,curexit);                 // top has multiple exits
        //   for each case: if gototype!=0 { if bl->getIndex()==curexit gototype=f_break_goto }
        //                  else bl->scopeBreak(curexit,curexit);
        BlockType::Switch => {
            let b0 = bg.block(this_id).get_block(0);
            scope_break(bg, b0, None, curexit);
            let cases: Vec<(usize, BlockId, uint4)> = bg
                .block(this_id)
                .switch_caseblocks()
                .iter()
                .enumerate()
                .map(|(i, c)| (i, c.block, c.gototype))
                .collect();
            for (i, bl, gototype) in cases {
                if gototype != 0 {
                    // A goto that goes straight to the switch exit is an (empty) break.
                    if Some(bl) == curexit {
                        if let Some(cbs) = bg.block_mut(this_id).switch_caseblocks_mut() {
                            cbs[i].gototype = block_flags::f_break_goto;
                        }
                    }
                } else {
                    scope_break(bg, bl, curexit, curexit);
                }
            }
        }
        // Base FlowBlock::scopeBreak (block.hh:205) does nothing: t_basic/t_copy/t_plain.
        _ => {}
    }
}

/// Set a `BlockGoto`'s `gototype` flag (C++ `BlockGoto::gototype = ...`).
fn set_goto_type(bg: &mut BlockGraph, id: BlockId, ty: uint4) {
    if let BlockKind::Goto { gototype, .. } = bg.block_mut(id).kind_mut() {
        *gototype = ty;
    }
}

/// Set a `BlockIf`'s if-goto `gototype` flag (C++ `BlockIf::gototype = ...`).
fn set_if_goto_type(bg: &mut BlockGraph, id: BlockId, ty: uint4) {
    if let BlockKind::If { gototype, .. } = bg.block_mut(id).kind_mut() {
        *gototype = ty;
    }
}

//===========================================================================
// Option gate
//===========================================================================

/// (kuna) Toggle loop-exit `goto` -> `break;` recovery:
/// `loopbreak_recovery on|off`.
///
/// kuna ships this DIV-7 default-on (it recovers structured `break;` statements
/// out of loops, matching angr and upstream Ghidra's unconditional `scopeBreak`);
/// "off" restores the byte-identical raw-`goto` rendering of kuna's prior port.
pub struct OptionLoopBreakRecovery;

impl OptionLoopBreakRecovery {
    /// The option name.
    pub const NAME: &'static str = "loopbreak_recovery";

    /// Parse `on|off`; returns the resolved flag plus a confirmation message.
    /// The caller writes the flag into `Architecture::recover_loop_break`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Loop break/continue recovery turned {prop}")))
    }
}
