//! (kuna) `condexeret` -- a return register that fails recovery only on a path
//! `ActionConditionalExe` removes gets one more look after it.
//!
//! `ActionReturnRecovery` (coreaction.cc:1954) walks each output trial with
//! `AncestorRealistic`, and one path that reaches the function's own input
//! (not directwrite) fails the whole trial.  A register written under a
//! condition, followed by a branch on the same condition again, has such a
//! path -- taken the first time, taken again the second -- which cannot run,
//! but stays in the graph until `ActionConditionalExe` threads the merge block
//! away, later in the same `mainloop` iteration.  Output trials get one pass
//! (`maxpass` is 0 unless the model has a delayed heritage space), so the trial
//! is never looked at again and the function prints `void f(void)`.
//!
//! When the walk fails at an input read by a MULTIEQUAL in a block of the
//! shape `ActionConditionalExe` threads ([`threadable_block`]), the trial and
//! that block are remembered, and if any remembered trial is still unchecked
//! after the last normal pass, the container stays open for one more pass on
//! the next iteration.  That pass re-checks a remembered trial only if its
//! block is gone from the graph; every other trial keeps its verdict.  The
//! extra pass happens at most once per recovery, and where the path survives
//! the threading the trial fails again exactly as before.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId};
use crate::expression::BooleanExpressionMatch;
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// Where a function's output trials stand with respect to the retry pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CondExeRetry {
    /// No retry scheduled.
    #[default]
    Idle,
    /// The next pass re-checks a listed trial once its listed block is gone.
    Pending(Vec<(int4, BlockId)>),
    /// The retry pass has run.
    Done,
}

/// The block of `reader` when it is a MULTIEQUAL in a block
/// `ActionConditionalExe` could thread: two in-edges that lead back through
/// straight-line blocks to one block ending in a CBRANCH, two out-edges, and a
/// CBRANCH on the same condition or its complement (`testIBlock`,
/// `findInitPre` and `verifySameCondition` of `ConditionalExecution::verify`,
/// condexe.cc:401; the removability of the block's ops is left to it).
pub fn threadable_block(fd: &Funcdata, reader: OpId) -> Option<BlockId> {
    let o = fd.obank().get(reader)?;
    if o.code() != OpCode::CPUI_MULTIEQUAL {
        return None;
    }
    let bl = o.get_parent()?;
    let g = fd.bblocks_ref();
    if g.block(bl).size_in() != 2 || g.block(bl).size_out() != 2 || !ends_in_cbranch(fd, bl) {
        return None;
    }
    let bound = fd.bblocks_get_size();
    let head = |mut b: BlockId| {
        let mut steps = 0;
        while g.block(b).size_out() == 1 && g.block(b).size_in() == 1 && steps < bound {
            b = g.block(b).get_in(0);
            steps += 1;
        }
        b
    };
    let init = head(g.block(bl).get_in(0));
    let shaped = init != bl
        && init == head(g.block(bl).get_in(1))
        && g.block(init).size_out() == 2
        && ends_in_cbranch(fd, init);
    let same = shaped
        && match (fd.bb_op_tail(bl), fd.bb_op_tail(init)) {
            (Some(cb), Some(icb)) => {
                BooleanExpressionMatch::new().verify_condition(cb, icb, fd.vbank(), fd.obank())
            }
            _ => false,
        };
    same.then_some(bl)
}

fn ends_in_cbranch(fd: &Funcdata, bl: BlockId) -> bool {
    fd.bb_op_tail(bl).and_then(|op| fd.obank().get(op)).map(|o| o.code())
        == Some(OpCode::CPUI_CBRANCH)
}

/// Does the current pass leave trial `i` alone?  Only the retry pass does, for
/// a trial it did not remember or whose remembered block is still in the graph.
pub fn skips(active: &ParamActive, i: int4, fd: &Funcdata) -> bool {
    skips_with(active, i, |b| !fd.bblocks_ref().arena.contains_key(b))
}

fn skips_with(active: &ParamActive, i: int4, gone: impl Fn(BlockId) -> bool) -> bool {
    match active.cond_exe_retry() {
        CondExeRetry::Pending(failed) => !failed.iter().any(|&(t, b)| t == i && gone(b)),
        _ => false,
    }
}

/// End-of-pass bookkeeping, before `ParamActive::finishPass`.  `failed` holds
/// the trials this pass saw fail at an input read by a MULTIEQUAL in a
/// [`threadable_block`], with that block.  After the last normal pass, schedule
/// the retry if any of them is still unchecked; after the retry pass, close it.
pub fn end_pass(active: &mut ParamActive, enabled: bool, mut failed: Vec<(int4, BlockId)>) {
    match active.cond_exe_retry() {
        CondExeRetry::Pending(_) => active.set_cond_exe_retry(CondExeRetry::Done),
        CondExeRetry::Idle if enabled && active.get_num_passes() == active.get_max_pass() => {
            failed.retain(|&(i, _)| !active.get_trial(i).is_checked());
            if failed.is_empty() {
                return;
            }
            failed.sort_unstable();
            failed.dedup();
            active.set_max_pass(active.get_max_pass() + 1);
            active.set_cond_exe_retry(CondExeRetry::Pending(failed));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests;
