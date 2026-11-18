//! **gowritebarrier, the post-SSA arm** — fold the Go GC write-barrier
//! diamonds the pre-SSA one-hop fold must refuse (`docs/multilang.md` W2/W5).
//!
//! The pre-SSA fold (`p2_lift/kuna_gowritebarrier.rs`) takes the textbook
//! shape: one guard, one-block arm, rejoining in one hop, values dead on
//! exit. Real Go binaries carry two more shapes it rightly refuses:
//!
//! - the barrier call's RESULT feeds the join — the thunk's buffer pointer
//!   and its INDIRECT register effects reach MULTIEQUALs, so the program's
//!   own stores after the join read phi outputs partly defined by the arm;
//! - one arm shared by SEVERAL guards (`runtime.makechan` records three
//!   pointers through one `gcWriteBarrier` block).
//!
//! Those are only safe to fold once SSA has made every read explicit, which
//! is why this action runs in the fullloop tail beside the other edge
//! strippers (`securitycheck`, `stackguard`) and uses the same primitive
//! pair. The matcher ([`crate::idiom::match_guarded_cold_call_phi`]) refuses
//! unless every guard's condition reads a global flag, every op in the arm is
//! GC bookkeeping (allowlisted opcodes; STOREs only through pointers rooted
//! at the barrier call's own output), and every value the arm defines is read
//! only inside the arm or by the join's MULTIEQUALs. Cutting a guard edge
//! then strands nothing: `branch_remove_internal` drops the matching phi
//! inputs, and when the last edge goes the unreachable sweep removes the arm,
//! its join inputs with it — each surviving phi collapses to the hot-path
//! value the program computes.
//!
//! One guard edge is cut per apply (the repeating fullloop re-matches), so a
//! shared arm converges one guard at a time and every cut re-validates the
//! whole shape against the edited graph.
//!
//! Gated by the same `option gowritebarrier off|auto|always` as the pre-SSA
//! arm: the two folds are one judgment — delete GC write-barrier machinery —
//! applied at the window where each shape is provable.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::idiom::match_guarded_cold_call_phi;
use crate::kuna_gowritebarrier::{is_write_barrier_name, live};

/// (kuna) `ActionGoWriteBarrierPhi` — fold shared / phi-feeding Go GC
/// write-barrier arms post-SSA (option `gowritebarrier`).
pub struct ActionGoWriteBarrierPhi {
    base: ActionBase,
    /// Unit-test OR-override of the option gate (the scheduled instance
    /// registers `false` and reads the live mode, as the sibling strippers do).
    enabled: bool,
}

impl ActionGoWriteBarrierPhi {
    /// `enabled` forces the pass on regardless of the option gate (unit
    /// tests); the scheduled instance passes `false`.
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionGoWriteBarrierPhi {
        ActionGoWriteBarrierPhi { base: ActionBase::new(0, "gowbphi", g), enabled }
    }
}

impl Action for ActionGoWriteBarrierPhi {
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
        Some(Box::new(ActionGoWriteBarrierPhi { base: self.base.clone(), enabled: self.enabled }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !self.enabled && !live(data) {
            return 0;
        }
        if data.num_calls() == 0 {
            return 0;
        }
        let Some(m) = match_guarded_cold_call_phi(data, &is_write_barrier_name) else {
            return 0;
        };
        // One edge per apply: the fullloop re-invokes, and every later cut
        // re-matches the whole shape against the already-edited graph.
        let (guard, edge) = m.guard_edges[0];
        data.remove_branch(guard, edge).expect("ActionGoWriteBarrierPhi: removeBranch");
        data.remove_unreachable_blocks(false, true)
            .expect("ActionGoWriteBarrierPhi: removeUnreachableBlocks");
        self.base.count += 1;
        1
    }
}

#[cfg(test)]
#[path = "kuna_gowbphi/tests.rs"]
mod tests;
