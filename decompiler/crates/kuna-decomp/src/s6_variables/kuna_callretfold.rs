//! (kuna) Call-return variable folding — inline a single-use call return into
//! its use site (port of angr's "call return variable folding"; test
//! `test_call_return_variable_folding`, S6).
//!
//! # The presentation gap
//!
//! Ghidra/kuna force **every** call output to be *explicit* in
//! [`ActionMarkExplicit::base_explicit`](crate::s6_variables::coreaction_cleanup)
//! (`coreaction.cc:3105`, the `if (op->isCall()) return -1;` arm), so a call
//! result always spills to its own named local:
//!
//! ```c
//! v5 = timespec_cmp(...);
//! if (v5 <= -1) { ... }
//! ```
//!
//! angr instead folds a call's return value into its single use, emitting the
//! call expression where it is consumed:
//!
//! ```c
//! if (timespec_cmp(...) <= -1) { ... }
//! ```
//!
//! Forcing call outputs explicit is conservative-by-design: marking a call
//! output *implied* makes the printer drop its standalone statement and emit the
//! call expression recursively at the point of use — which **moves the call's
//! evaluation** down to that use site.  If side-effecting statements sit between
//! the call and its use, that reorders observable behaviour.  angr performs the
//! same fold but only where it is order-safe.
//!
//! # What this does
//!
//! Gated by the `foldcallret` option (default-OFF), [`call_output_foldable`]
//! relaxes the forced-explicit rule for call outputs that are provably order-safe
//! to fold.  `base_explicit` consults it inside the `is_call()` arm; when it
//! returns `true` the call output falls through to the normal descendant-count
//! path (so the existing implied-marking machinery inlines it).  When the option
//! is off the predicate is never consulted and output is byte-identical.
//!
//! # The order-safety predicate (decider-refined)
//!
//! A call output `vn` may be folded iff **all** hold:
//!
//!   1. `vn` has **exactly one** descendant op ([`Funcdata::lone_descend`]) and
//!      that use op is **not** an SSA marker (MULTIEQUAL/INDIRECT phi — a phi has
//!      no single textual eval point);
//!   2. the use op lives in the **same basic block** as the call op (a cross-block
//!      move would cross merge/branch edges and conditional execution), and is
//!      ordered **after** the call in that block;
//!   3. **no** op strictly between the call and its use (in block op order) is a
//!      call (`is_call`) or a memory op (`CPUI_LOAD`/`CPUI_STORE`/`CPUI_CALLOTHER`).
//!
//! Keeping `LOAD` in the forbidden set is necessary, not redundant: the call may
//! `STORE` memory that an intervening `LOAD` reads, so sinking the call past that
//! `LOAD` would change the value the load observes (a read-after-write hazard not
//! covered by the `STORE`/`CALL` checks alone).  The predicate strictly *tightens*
//! the printer's pre-existing single-use inlining with a same-block,
//! side-effect-free guard, preferring false negatives (stay explicit) over
//! reordering bugs.

use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::seams::{OpId, VarnodeId};

/// (kuna) Is the call output `vn` order-safe to fold into its single use?
///
/// Called from `base_explicit` only when `glb->fold_call_returns` is on and the
/// defining op of `vn` is a (non-`NEW`) call.  Read-only over the merged tree.
pub fn call_output_foldable(data: &Funcdata, vn: VarnodeId) -> bool {
    // (1) exactly one descendant, and it is not an SSA marker.
    let use_op = match data.lone_descend(vn) {
        Some(o) => o,
        None => return false, // zero or >1 uses: not a single-use fold
    };
    if op_is_marker(data, use_op) {
        return false; // a phi has no single textual evaluation point
    }

    // The call op defining vn.
    let def = match vn_def(data, vn) {
        Some(d) => d,
        None => return false,
    };

    // (2) same basic block, use ordered after the call.
    let def_blk = match op_parent(data, def) {
        Some(b) => b,
        None => return false,
    };
    let use_blk = match op_parent(data, use_op) {
        Some(b) => b,
        None => return false,
    };
    if def_blk != use_blk {
        return false; // cross-block move crosses edges / conditional execution
    }

    let ops = data.bb_ops(def_blk); // block op order (seqnum)
    let def_idx = match ops.iter().position(|&o| o == def) {
        Some(i) => i,
        None => return false,
    };
    let use_idx = match ops.iter().position(|&o| o == use_op) {
        Some(i) => i,
        None => return false,
    };
    if use_idx <= def_idx {
        return false; // use must come strictly after the call in this block
    }

    // (3) no side-effecting / memory op strictly between the call and its use.
    for &mid in &ops[def_idx + 1..use_idx] {
        if op_is_barrier(data, mid) {
            return false;
        }
    }
    true
}

/// An op whose relative order with the moved call is observable: any call, or a
/// memory-touching op (LOAD/STORE/CALLOTHER).
fn op_is_barrier(data: &Funcdata, op: OpId) -> bool {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return true, // stale: be conservative
    };
    if o.is_call() {
        return true;
    }
    matches!(
        o.code(),
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER
    )
}

fn op_is_marker(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).map(|o| o.is_marker()).unwrap_or(true)
}

fn op_parent(data: &Funcdata, op: OpId) -> Option<crate::seams::BlockId> {
    data.obank().get(op).and_then(|o| o.get_parent())
}

fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).and_then(|v| v.get_def())
}

//===========================================================================
// Option gate
//===========================================================================

/// (kuna) Toggle call-return variable folding: `foldcallret on|off`.
///
/// Ships default-**OFF** (opt-in): it changes call-output rendering broadly, so
/// it is not byte-identical to upstream.  "on" lets order-safe single-use call
/// outputs inline into their use site.
pub struct OptionFoldCallRet;

impl OptionFoldCallRet {
    /// The option name.
    pub const NAME: &'static str = "foldcallret";

    /// Parse `on|off` and return the resolved flag + confirmation message.  The
    /// caller writes the flag into `Architecture::fold_call_returns`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Call-return variable folding turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_callretfold/tests.rs"]
mod tests;
