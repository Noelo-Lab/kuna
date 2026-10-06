//! P4 output-trial scoring across a re-tested condition — the `condexeretuse`
//! decision point.
//!
//! # The gap
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! walks every descendant of a value reaching a RETURN and rejects the output
//! trial when one of them is a competing use: another CALL, a branch, a
//! dereference. ARM conditional execution builds a control-flow graph in which
//! some of those descendants sit on paths that cannot run:
//!
//! ```text
//!   cmp   r0,#0
//!   moveq r0,#7     ; A: skip B unless Z           B: r0 = 7
//!   bxeq  lr        ; C: r0' = MULTIEQUAL(r0, 7); return (D) if Z
//!   b     getv      ; E: call getv(r0'); return
//! ```
//!
//! `C` merges the two arms of `A` and then branches on the same flag again, so
//! the execution that came through `B` always continues to `D`, and the one that
//! skipped `B` always continues to `E`. `ActionConditionalExe` threads `C` away,
//! but later in the same `mainloop` iteration than `ActionReturnRecovery`, which
//! scores output trials once. At that point the `7` flows through `C` into both
//! the RETURN in `D` and the CALL in `E`, the CALL rejects the trial, and
//! `int pick(int a) { if (a) return getv(); return 7; }` prints as
//! `void pick(int a0)`.
//!
//! # The rule
//!
//! When the walk for a RETURN match fails and it passed through such a merge, it
//! is repeated once with the merge's control flow taken into account. A value
//! entering a MULTIEQUAL in a block that re-tests its init block's condition
//! ([`ConditionalExecution::forced_out_block`](crate::condexe::ConditionalExecution::forced_out_block):
//! two in-edges leading back through straight-line blocks to one CBRANCH block,
//! two out-edges, a CBRANCH on the same condition or its complement) through
//! exactly one in-edge determines the out-edge that execution takes. The output
//! carries that pair, and so does every value computed from it inside the merge
//! block. A use of such a value outside the merge block that the forced out block
//! cannot reach is skipped: on any execution that reaches it, the merge was
//! entered through the other in-edge and the value there is not the one being
//! scored. Reachability follows only the forced out-edge of any other such
//! merge it enters through one in-edge, so a 64-bit
//! `moveq r0,#7; moveq r1,#0; bxeq lr`, whose `r0` passes two merges that
//! re-test Z, is covered. Everything else keeps the upstream treatment.
//!
//! The repeated walk passes only if it also reaches the RETURN slot being
//! matched through a use it did not skip. A value whose forced branch leads away
//! from the RETURN (into a spin loop, or into a call that does not return) never
//! gets there, and skipping every use it cannot reach would otherwise pass the
//! walk with nothing checked. It also gives up (keeps the upstream rejection)
//! when a value is reached both through such a merge and along another route,
//! because the uses it skipped are then real for the other route. Call-site
//! input trials are left alone.

use std::collections::{HashMap, HashSet};

use kuna_num::opcodes::OpCode;

use crate::condexe::ConditionalExecution;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Cap on blocks visited by one reachability walk; exceeding it treats every
/// block as reachable, which keeps the upstream rejection.
const MAX_VISIT: usize = 4096;

/// A merge block that re-tests its init block's condition, and the out block
/// it branches to for the value carrying this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forced {
    pub merge: BlockId,
    pub next: BlockId,
}

/// Whether the walk matched at `opmatch` may take re-tested merges into
/// account: the option is on and the match is a RETURN.
pub fn applies(data: &Funcdata, opmatch: OpId) -> bool {
    data.get_arch().cond_exe_ret_use
        && data.obank().get(opmatch).map(|o| o.code()) == Some(OpCode::CPUI_RETURN)
}

/// Whether any MULTIEQUAL `op` a walk reached through `vn` sits in a merge
/// block that re-tests its condition and was entered through one in-edge.
pub fn any_retest_merge(data: &Funcdata, entered: &[(OpId, VarnodeId)]) -> bool {
    entered.iter().any(|&(op, vn)| carried(data, op, vn, None).is_some())
}

/// The [`Forced`] pair the output of `op` carries when the walk reaches it
/// through `vn`, which already carries `from`: a fresh pair for a MULTIEQUAL in a
/// re-testing merge block entered through exactly one in-edge, `from` for an op
/// inside `from`'s merge block, nothing otherwise.
pub fn carried(data: &Funcdata, op: OpId, vn: VarnodeId, from: Option<Forced>) -> Option<Forced> {
    let o = data.obank().get(op)?;
    let bl = o.get_parent()?;
    if o.code() == OpCode::CPUI_MULTIEQUAL {
        return entered_merge(data, op, vn, bl);
    }
    from.filter(|f| f.merge == bl)
}

fn entered_merge(data: &Funcdata, op: OpId, vn: VarnodeId, bl: BlockId) -> Option<Forced> {
    let o = data.obank().get(op)?;
    if o.num_input() != 2 {
        return None;
    }
    let slot = match (o.get_in(0) == Some(vn), o.get_in(1) == Some(vn)) {
        (true, false) => 0,
        (false, true) => 1,
        _ => return None,
    };
    let next = ConditionalExecution::forced_out_block(data, bl, slot)?;
    Some(Forced { merge: bl, next })
}

/// Blocks reachable from each forced out block, computed once per walk.
#[derive(Default)]
pub struct Reach {
    sets: HashMap<BlockId, Option<HashSet<BlockId>>>,
}

/// Whether `op` is a use the value carrying `f` can never arrive at: it sits
/// outside `f.merge`, in a block `f.next` cannot reach.
pub fn cannot_arrive(data: &Funcdata, f: Forced, op: OpId, reach: &mut Reach) -> bool {
    let Some(bl) = data.obank().get(op).and_then(|o| o.get_parent()) else {
        return false;
    };
    if bl == f.merge {
        return false;
    }
    let set = reach.sets.entry(f.next).or_insert_with(|| reachable_from(data, f.next));
    set.as_ref().is_some_and(|s| !s.contains(&bl))
}

/// Every block reachable from `from`, itself included, where an execution that
/// arrives at another re-testing merge block through one in-edge follows only
/// the out-edge that merge forces; `None` past [`MAX_VISIT`].
fn reachable_from(data: &Funcdata, from: BlockId) -> Option<HashSet<BlockId>> {
    let graph = data.bblocks_ref();
    let mut blocks: HashSet<BlockId> = HashSet::new();
    let mut seen: HashSet<(BlockId, i32)> = HashSet::new();
    let mut stack = vec![(from, -1)];
    while let Some((bl, inslot)) = stack.pop() {
        if !seen.insert((bl, inslot)) {
            continue;
        }
        blocks.insert(bl);
        if seen.len() > MAX_VISIT {
            return None;
        }
        let b = graph.block(bl);
        let forced = (inslot >= 0)
            .then(|| ConditionalExecution::forced_out_block(data, bl, inslot))
            .flatten();
        for i in 0..b.size_out() {
            let next = b.get_out(i);
            if forced.is_some_and(|f| f != next) {
                continue;
            }
            let nb = graph.block(next);
            let slot = if nb.size_in() == 2 && nb.size_out() == 2 { b.get_out_rev_index(i) } else { -1 };
            stack.push((next, slot));
        }
    }
    Some(blocks)
}

#[cfg(test)]
#[path = "kuna_condexeretuse/tests.rs"]
mod tests;
