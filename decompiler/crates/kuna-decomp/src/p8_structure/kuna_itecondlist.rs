//! S8 **ITE condition-list tolerance** — let the `iteregion` / `iteboolean` diamond
//! matchers see through a multi-component `BlockList` in the *condition* position.
//!
//! # The structural gap
//!
//! [`ActionIteRegion`](crate::p8_structure::kuna_iteregion::ActionIteRegion) and
//! [`ActionIteBoolean`](crate::p8_structure::kuna_iteboolean::ActionIteBoolean) both
//! require the condition component of the 3-component
//! [`BlockIf`](crate::block::BlockType::If) to bottom out in a single leaf — a lone
//! `CBRANCH`-terminated basic block, or (for `iteboolean`) a folded
//! [`BlockCondition`](crate::block::BlockType::Condition).  Their shared helper
//! [`leaf_bblock`](crate::p8_structure::kuna_iteregion::leaf_bblock) descends a
//! `BlockList` of **one** component but returns `None` for a list of two or more.
//!
//! That bail is what makes kuna re-roll only **every other** diamond in a run of
//! structurally identical ones.  When a diamond's predecessor is itself a
//! just-collapsed `if`, the block-concatenation rule folds the predecessor together
//! with the following condition block into a two-component `BlockList`, and *that
//! list* becomes component 0 of the next `BlockIf`.  A chain of N identical diamonds
//! alternates between the two shapes, so exactly `ceil(N/2)` of them are re-rolled
//! (measured, N = 1..8: 1 1 2 2 3 3 4 4) and the rest print as
//! `if (c) v = A; else { v = B; }` — three extra CFG blocks and four extra edges
//! each, against a source that wrote one `?:` or one boolean assignment.  On bash
//! `shell_initialize` three source-identical `x = a||b||c||d;` call arguments come
//! out as one re-rolled assignment plus two explicit diamonds.
//!
//! # What this option does
//!
//! With `option itecondlist on`, both matchers descend a condition-position
//! `BlockList` to its **last** component before applying their usual leaf/condition
//! test ([`cond_list_tail`]).  Nothing else changes: the arms keep the strict
//! single-statement `leaf_bblock` test, the labelled-goto-target guard still applies
//! to the descended leaf, and the marked op is still the tail's terminal `CBRANCH`.
//!
//! # Why descending the tail is sound
//!
//! The rewrite is print-only, and the printer already renders exactly this shape.
//! `PrintC::emit_block_if` emits the condition component **twice**: once under
//! `NO_BRANCH` (its statements) and once under `ONLY_BRANCH` (its branch condition).
//! For a `BlockList`, `PrintC::emit_block_ls` emits every component in order under
//! `NO_BRANCH` but **only the last** under `ONLY_BRANCH`.  So the leading components
//! are already emitted once, as ordinary statements, *before* the `if` header — and
//! the ternary/boolean emitters
//! ([`emit_block_if_ite`](crate::printc) / `emit_block_if_bool`) perform the same two
//! emissions on the same block.  Folding only the tail therefore leaves everything
//! that precedes it byte-identical; it is the same tolerance `cond_cbranch` already
//! grants leading statements *inside* one basic block, spelled across a block
//! boundary.
//!
//! The descent is restricted to [`BlockType::Ls`](crate::block::BlockType::Ls) for
//! that reason.  A [`BlockGraph`](crate::block::BlockType::Graph) is **not**
//! included: `PrintC::emit_block_graph` ignores `ONLY_BRANCH` and emits every
//! component, so descending one would make the emitters print the whole graph twice.

use kuna_base::error::KunaResult;

use crate::block::BlockType;
use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

use kuna_base::marshal::ElementId;

/// Marshaling element `<itecondlist>` (kuna).  ElementIds live in the 4000+ range
/// (4120 paramcopyhoist was the previous high-water mark; this is 4121).
pub const ELEM_ITECONDLIST: ElementId = ElementId::new("itecondlist", 4121);

/// How many nested `BlockList` levels the descent walks before giving up.  The
/// structured tree is a tree, so this is only a sanity bound.
const MAX_LIST_DEPTH: u32 = 16;

/// The block the diamond matchers should test as the condition leaf of `id`.
///
/// With `option itecondlist off` this is `id` itself (byte-identical to the shipped
/// `iteregion`/`iteboolean` behaviour).  With it on, a condition-position
/// [`BlockType::Ls`] is descended to its **last** component — the one the printer
/// renders as the `if` header — so a diamond whose condition got concatenated with a
/// preceding statement block still matches.
pub(crate) fn cond_list_tail(data: &Funcdata, id: BlockId) -> BlockId {
    if !data.get_arch().itecondlist {
        return id;
    }
    let mut cur = id;
    for _ in 0..MAX_LIST_DEPTH {
        let g = data.sblocks_ref();
        let blk = g.block(cur);
        if blk.get_type() != BlockType::Ls {
            break;
        }
        let n = blk.get_size();
        if n == 0 {
            break;
        }
        cur = blk.get_block(n - 1);
    }
    cur
}

//===========================================================================
// Option parse (`itecondlist on|off`).
//===========================================================================

/// The `itecondlist on|off` ArchOption.
pub struct OptionIteCondList;

impl OptionIteCondList {
    /// The option name.
    pub const NAME: &'static str = "itecondlist";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((
            val,
            format!("ITE diamond matching through a concatenated condition list turned {prop}"),
        ))
    }
}
