//! (kuna) `condexeret` -- a return register that fails recovery only on paths
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
//! When a trial's walk at a RETURN fails at such an input, a second walk from
//! the same RETURN passes every non-directwrite input and records it
//! ([`remember`]).  The (trial, RETURN) pair is remembered only if that walk
//! meets nothing else that fails and every input it recorded is read by a
//! MULTIEQUAL in a block `ActionConditionalExe` could thread
//! ([`threadable_block`]).  If a remembered trial is still unchecked after the
//! last normal pass, the container stays open for one more pass on the next
//! iteration, which re-walks a remembered pair only once all of its blocks are
//! gone from the graph, and with every function input overlapping the recorded
//! inputs failing whether or not it has become directwrite since ([`check`]).
//! Everything else the first walk met passed it, so the re-walk can pass only
//! where threading left no path from the register's entry value to the RETURN.
//! The extra pass happens at most once per recovery.

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId};
use crate::expression::BooleanExpressionMatch;
use crate::fspec::{ParamActive, ParamTrial};
use crate::funcdata::Funcdata;
use crate::funcdata_varnode::AncestorRealistic;

/// A trial whose walk at one RETURN failed only at inputs read in threadable
/// merge blocks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Remembered {
    pub trial: int4,
    pub ret: OpId,
    /// The merge blocks that read the failing inputs.
    pub merges: Vec<BlockId>,
    /// The storage of the failing inputs.
    pub inputs: Vec<(Address, int4)>,
}

/// Where a function's output trials stand with respect to the retry pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CondExeRetry {
    /// No retry scheduled.
    #[default]
    Idle,
    /// The next pass re-walks only these (trial, RETURN) pairs.
    Pending(Vec<Remembered>),
    /// The retry pass has run.
    Done,
}

/// How the current pass treats one trial at one RETURN.
#[derive(Debug, PartialEq, Eq)]
pub enum Check {
    /// The upstream walk.
    Normal,
    /// Leave the trial's verdict alone.
    Skip,
    /// Walk with inputs overlapping these ranges failing.
    Strict(Vec<(Address, int4)>),
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

/// Is this pass the one that remembers failures: the option is on, no retry
/// is scheduled yet, and it is the last normal pass?
pub fn remembering(active: &ParamActive, enabled: bool) -> bool {
    enabled
        && *active.cond_exe_retry() == CondExeRetry::Idle
        && active.get_num_passes() == active.get_max_pass()
}

/// After `trial`'s upstream walk from slot `slot` of RETURN `ret` failed at an
/// input read by `reader`: walk again past every non-directwrite input, and
/// remember the pair if nothing else failed and every such input was read in a
/// [`threadable_block`].
pub fn remember(
    fd: &mut Funcdata,
    i: int4,
    trial: &ParamTrial,
    ret: OpId,
    slot: int4,
    reader: OpId,
) -> Option<Remembered> {
    threadable_block(fd, reader)?;
    let mut walk = AncestorRealistic::new();
    walk.collect_inputs();
    let (realistic, _) = walk.execute(
        fd,
        ret,
        slot,
        trial.get_size(),
        trial.has_cond_exe_effect(),
        trial.is_killed_by_call(),
        false,
    );
    if !realistic || walk.collected().is_empty() {
        return None;
    }
    let mut merges = Vec::new();
    let mut inputs = Vec::new();
    for &(rd, vn) in walk.collected() {
        merges.push(threadable_block(fd, rd)?);
        let v = fd.vbank().get(vn)?;
        inputs.push((v.get_addr().clone(), v.get_size()));
    }
    merges.sort_unstable();
    merges.dedup();
    Some(Remembered { trial: i, ret, merges, inputs })
}

/// How the current pass treats trial `i` at RETURN `ret`: normally outside the
/// retry pass; in it, strictly for a remembered pair whose blocks are all gone,
/// and not at all otherwise.
pub fn check(active: &ParamActive, i: int4, ret: OpId, fd: &Funcdata) -> Check {
    check_with(active, i, ret, |b| !fd.bblocks_ref().arena.contains_key(b))
}

fn check_with(active: &ParamActive, i: int4, ret: OpId, gone: impl Fn(BlockId) -> bool) -> Check {
    let CondExeRetry::Pending(list) = active.cond_exe_retry() else {
        return Check::Normal;
    };
    match list.iter().find(|r| r.trial == i && r.ret == ret) {
        Some(r) if r.merges.iter().all(|&b| gone(b)) => Check::Strict(r.inputs.clone()),
        _ => Check::Skip,
    }
}

/// End-of-pass bookkeeping, before `ParamActive::finishPass`.  After the last
/// normal pass, schedule the retry if a remembered trial is still unchecked;
/// after the retry pass, close it.
pub fn end_pass(active: &mut ParamActive, enabled: bool, mut remembered: Vec<Remembered>) {
    match active.cond_exe_retry() {
        CondExeRetry::Pending(_) => active.set_cond_exe_retry(CondExeRetry::Done),
        CondExeRetry::Idle if remembering(active, enabled) => {
            remembered.retain(|r| !active.get_trial(r.trial).is_checked());
            if remembered.is_empty() {
                return;
            }
            active.set_max_pass(active.get_max_pass() + 1);
            active.set_cond_exe_retry(CondExeRetry::Pending(remembered));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests;
