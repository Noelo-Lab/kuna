//! S8 **short-circuit operand-chain protection** for
//! [`returndup`](crate::p8_structure::kuna_returndup) — decline the one split that
//! permanently destroys a `||`/`&&` fold.
//!
//! # The structural gap
//!
//! [`ActionReturnDup`](crate::p8_structure::kuna_returndup::ActionReturnDup) gives
//! every predecessor of a shared bare-epilogue RETURN block its own private return,
//! which is exactly how the source's early-return guards are recovered.  But the
//! operand blocks of a short-circuit expression are *also* predecessors of a shared
//! return, and for them the private return is the wrong shape: it is precisely the
//! shared out-target that
//! [`CollapseStructure::rule_block_or`](crate::blockaction) needs in order to fuse
//! two conditionals into one `BlockCondition`.  `ActionReturnDup` runs in the
//! `returnsplit` group and `rule_block_or` runs later, in `collapse_conditions`, so
//! the loss is permanent: the guards can never share an out-target again, and a
//! source that wrote `return a || (b && (c || d));` comes back as a cascade of
//! constant-return guards.
//!
//! Measured over the whole decbench corpus (85,195 functions, three optimisation
//! levels; `docs/decbench/returndup-o0-measurement.md`), this sub-shape carries
//! essentially all of `returndup`'s harm at `-O0` (-665 GED-perfect) and almost none
//! of its benefit at the optimized levels (+15 / +20 of +832).
//!
//! # What this option does
//!
//! With `option orchain on` (DIV-69, the default),
//! [`Funcdata::returndup_apply`](crate::funcdata::Funcdata) first asks
//! [`shortcircuit_shared_targets`] which blocks are the *shared out-target* of a fold
//! `rule_block_or` would perform, and declines the whole function's splits when one of
//! its own candidates is in that set.  It is one-directional -- it can only make
//! `returndup` fire less -- and inert with `option returndup off`.
//!
//! # The predicate
//!
//! [`shortcircuit_shared_targets`] is a miniature, read-only replay of
//! `CollapseStructure::collapse_conditions` on the **bblocks** CFG — the pass whose
//! precondition is being protected, so the admission test is mirrored rather than
//! approximated.  A *condition node* starts as one basic block with two out-edges;
//! two nodes `p`, `q` fuse when
//!
//! * `q` is one of `p`'s two out-targets, and `q` has exactly one in-edge (`p`'s) —
//!   `rule_block_or`'s `orblock->sizeIn() != 1` test;
//! * `q` also has two out-edges and is not a switch block;
//! * `q` is not [complex](crate::funcdata::Funcdata::bb_is_complex) — the same
//!   `BlockBasic::isComplex` verdict `rule_block_or` reads, computed from the same
//!   op lists;
//! * `p`'s *other* out-target, the **clause block**, is also one of `q`'s out-targets
//!   (the shared out-target), and is neither `p` nor `q`;
//! * `q`'s other out-target is not `p` (upstream's "no looping" test).
//!
//! Each fuse records its clause block and replaces `p` with a node whose out-targets
//! are the clause block and `q`'s other target, so a chain folds inside-out exactly
//! as the fixpoint in `collapse_conditions` does.  The two upstream tests that are
//! omitted are the ones that cannot hold here: `bl->isGotoOut` /
//! `orblock->isInteriorGotoTarget` (no goto has been elected yet — `select_goto` runs
//! after `collapse_conditions`), and `bl->isBackEdgeOut` (a back edge into a block
//! with a single in-edge would make that block unreachable from the entry).
//!
//! Because the replay runs before the rest of P8 rather than on the final structuring
//! tree, it is an over-approximation: it can protect a return whose fold another
//! collapse rule would have consumed first.  It never under-approximates the
//! single-pass `rule_block_or` shape, which is what the split destroys.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Marshaling element `<orchain>` (kuna).  ElementIds live in the 4000+ range
/// (4121 itecondlist was the previous high-water mark; this is 4122).
pub const ELEM_ORCHAIN: ElementId = ElementId::new("orchain", 4122);

/// The blocks that are the **shared out-target** of at least one short-circuit fold
/// `CollapseStructure::rule_block_or` would perform on this CFG — i.e. the blocks a
/// `returndup` split must leave shared.  See the module header for the predicate.
pub(crate) fn shortcircuit_shared_targets(data: &Funcdata) -> BTreeSet<BlockId> {
    let mut shared: BTreeSet<BlockId> = BTreeSet::new();
    let nbb = data.bblocks_get_size();
    if nbb < 3 {
        return shared;
    }

    // Condition nodes, keyed by the block that heads them: a fold rewrites the head's
    // out-targets in place and kills the absorbed node, which is how a whole chain
    // collapses onto its leftmost operand.
    let mut outs: BTreeMap<BlockId, [BlockId; 2]> = BTreeMap::new();
    let mut indeg: BTreeMap<BlockId, int4> = BTreeMap::new();
    let mut dead: BTreeSet<BlockId> = BTreeSet::new();
    for i in 0..nbb {
        let b = data.bblocks_get_block(i);
        let blk = data.bblocks_ref().block(b);
        indeg.insert(b, blk.size_in());
        if blk.size_out() != 2 || blk.is_switch_out() {
            continue;
        }
        let (o0, o1) = (blk.get_out(0), blk.get_out(1));
        if o0 == o1 {
            continue; // degenerate two-way branch: no distinct clause block
        }
        outs.insert(b, [o0, o1]);
    }

    let mut queue: VecDeque<BlockId> = outs.keys().copied().collect();
    let mut queued: BTreeSet<BlockId> = outs.keys().copied().collect();
    while let Some(p) = queue.pop_front() {
        queued.remove(&p);
        if dead.contains(&p) {
            continue;
        }
        let pouts = match outs.get(&p) {
            Some(o) => *o,
            None => continue,
        };
        for i in 0..2usize {
            let q = pouts[i];
            let clause = pouts[1 - i];
            if q == p || clause == p || clause == q || dead.contains(&q) {
                continue;
            }
            if indeg.get(&q).copied().unwrap_or(0) != 1 {
                continue;
            }
            let qouts = match outs.get(&q) {
                Some(o) => *o,
                None => continue,
            };
            let j = if qouts[0] == clause {
                0
            } else if qouts[1] == clause {
                1
            } else {
                continue;
            };
            if qouts[1 - j] == p {
                continue; // upstream "no looping"
            }
            if data.bb_is_complex(q) {
                continue; // BlockBasic::isComplex — the sibling is too wide to fold
            }

            shared.insert(clause);
            outs.insert(p, [clause, qouts[1 - j]]);
            dead.insert(q);
            outs.remove(&q);
            *indeg.entry(q).or_insert(0) -= 1;
            *indeg.entry(clause).or_insert(0) -= 1;

            // The fold changed `p`'s out-targets, dropped the clause block's in-degree
            // (which can newly admit it as somebody's `orblock`) and left `q`'s other
            // target with a new predecessor: re-queue everything that can now match.
            for b in [p, clause, qouts[1 - j]] {
                for r in std::iter::once(b).chain(preds_of(data, b)) {
                    if outs.contains_key(&r) && !dead.contains(&r) && queued.insert(r) {
                        queue.push_back(r);
                    }
                }
            }
            break;
        }
    }

    // The value-materialisation join.  Before the constants are propagated into the
    // epilogue phi, a chain's two terminal arms are not the RETURN block itself but
    // one bare `v = K` block each (`COPY` + `BRANCH`, one out-edge), and the RETURN
    // block sits one edge further on.  It is the *first* `returndup` invocation of the
    // schedule's fullloop that sees that graph, and splitting there separates the two
    // arms of the 0/1 select the source wrote as a single expression — the diamond
    // `iteboolean`/`iteregion` re-roll — before the second invocation can decline
    // anything.  So a block joining two or more single-exit arms that are themselves
    // shared fold targets is protected as well.
    let joins: Vec<BlockId> = (0..nbb)
        .map(|i| data.bblocks_get_block(i))
        .filter(|&u| {
            let blk = data.bblocks_ref().block(u);
            blk.size_in() > 1
                && (0..blk.size_in())
                    .map(|k| blk.get_in(k))
                    .filter(|t| {
                        shared.contains(t) && data.bblocks_ref().block(*t).size_out() == 1
                    })
                    .count()
                    > 1
        })
        .collect();
    shared.extend(joins);
    shared
}

/// The CFG predecessors of `b` (the raw bblocks in-edges — a superset of the
/// predecessors in the partially folded graph, which is all the worklist needs).
fn preds_of(data: &Funcdata, b: BlockId) -> Vec<BlockId> {
    let blk = data.bblocks_ref().block(b);
    (0..blk.size_in()).map(|i| blk.get_in(i)).collect()
}

//===========================================================================
// Option parse (`orchain on|off`).
//===========================================================================

/// The `orchain on|off` ArchOption (decline a `returndup` split that would break a
/// short-circuit operand chain).
pub struct OptionOrChain;

impl OptionOrChain {
    /// The option name.
    pub const NAME: &'static str = "orchain";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Short-circuit operand-chain protection for returndup turned {prop}")))
    }
}
