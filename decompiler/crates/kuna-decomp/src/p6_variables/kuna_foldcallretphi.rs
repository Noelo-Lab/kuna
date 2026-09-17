//! (kuna) Fold a single-use call result past the merge phalanx — the
//! `foldcallretphi` option.
//!
//! # The gap
//!
//! `foldcallret` (§6.4) lets an order-safe single-use call output fall through
//! `base_explicit`'s forced-explicit call arm, but that is only the first of two
//! gates.  `ActionMarkImplied` then runs
//! [`check_implied_cover`](crate::p6_variables::coreaction_cleanup), whose third
//! arm forces a Varnode explicit when one of its operands cannot have its Cover
//! inflated to the print site without colliding with another SSA version of the
//! same `HighVariable` (`Merge::inflate_test`).  Upstream never reaches that arm
//! with a call output — Ghidra marks every call output explicit one pass
//! earlier — so with `foldcallret` on it fires on a configuration it was never
//! written for:
//!
//! ```text
//! v3 = sub_3700(stdin,v7);   // the call reads the global `stdin`
//! v1 = 1;
//! v12 &= v3;                 // the single use
//! ```
//!
//! The operand `stdin` is an address-tied global with dozens of SSA versions,
//! and the version that collides with the call output's Cover is the output of
//! the `CPUI_INDIRECT` the **call itself** attaches to `stdin` (a call may write
//! any global).  The conflict is self-inflicted: the write that makes the
//! operand's other version live at the print site is performed by the very call
//! being moved there.
//!
//! # What this does
//!
//! [`conflict_is_self_call_effect`] re-walks the instances that `inflate_test`
//! just rejected on and answers one question: is *every* colliding version an
//! INDIRECT effect of this call?  If so, the rejection is discounted and the
//! call output stays an implied candidate.
//!
//! The discount is about *versions*, not order: the collision it forgives is
//! one the call itself creates, and in the folded rendering the operand read and
//! that write still happen at the same point — inside the call expression —
//! exactly as they do in the spilled form.  Two further conditions bound it:
//!
//!   * the operand's high must not belong to a `VariableGroup` (the piece
//!     intersection loop of `inflate_test` reasons about overlapping storage,
//!     not versions, so its rejections are never discounted), and
//!   * nothing from the call up to and including the statement the expression
//!     lands in may read an INDIRECT effect of the call — there the folded text
//!     would name the operand's high both as the call's argument (pre-call) and
//!     beside it (post-call).
//!
//! # Why this option carries its own span guard
//!
//! `call_output_foldable`'s span ends at the single use, which is the call's
//! textual home only when that use op is itself a statement; when the use op's
//! own output is implied the expression keeps travelling, and the rejection this
//! module discounts is sometimes the only thing that was holding such a call in
//! place.  The distance it then travels is therefore this option's to clear, and
//! it is cleared twice over:
//!
//!   * against barriers — a call, a memory opcode, or a write to persistent or
//!     address-tied storage — by
//!     [`fold_print_point_is_order_safe`](crate::p6_variables::kuna_callretfold),
//!     one arm earlier in the same
//!     [`check_implied_cover`](crate::p6_variables::coreaction_cleanup) call.
//!     That arm runs for every folded call output, this option on or off; it is
//!     where GH-657's tied-write barrier reaches the default fold.
//!   * against reads of the call's own INDIRECT effects, by
//!     [`landing_span_reads_call_effect`](crate::p6_variables::kuna_callretfold).
//!     `foldcallret` asks that question only as far as the use, where asking it
//!     further declines faithful folds; here it is asked over the whole
//!     distance, because the folds that reach this far are exactly the ones the
//!     merge machinery was holding.  Dropping it folds `ssh` O2 `sub_4fd30`'s
//!     `sub_3fa80` past a copy of an escaped stack slot the call reloads —
//!     GH-181's shape.
//!
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// (kuna) Is `inflate_test`'s rejection of `operand` purely an effect of `call`?
///
/// `high` is the HighVariable of the call output whose Cover the operand would
/// have to be inflated over.  Returns `true` only when at least one instance of
/// the operand's high collides with that Cover and *every* colliding instance is
/// the output of a `CPUI_INDIRECT` whose effect op is `call`.  Read-only; the
/// Covers it reads are the ones `Merge::inflate_test` has just refreshed.
pub fn conflict_is_self_call_effect(
    data: &Funcdata,
    call: OpId,
    operand: VarnodeId,
    high: HighVariableId,
    use_op: OpId,
) -> bool {
    let Some(ahigh) = data.vbank().get(operand).and_then(|v| v.get_high()) else {
        return false;
    };
    // A grouped high's rejection can come from the piece-intersection loop,
    // which is about overlapping storage rather than versions: never discount it.
    if data.high_bank().high_piece_id(ahigh).is_some() {
        return false;
    }
    if op_reads_indirect_effect_of(data, use_op, call) {
        return false;
    }
    if crate::kuna_callretfold::landing_span_reads_call_effect(data, call, use_op) {
        return false;
    }
    let Some(high_cover) = data.high_bank().internal_cover(high).cloned() else {
        return false;
    };
    let n = data.high_bank().get(ahigh).map(|h| h.num_instances()).unwrap_or(0);
    let mut saw_conflict = false;
    for i in 0..n {
        let b = match data.high_bank().get(ahigh) {
            Some(h) => h.get_instance(i),
            None => return false,
        };
        if data.varnode_copy_shadow(b, operand) {
            continue;
        }
        let collides = data
            .vbank()
            .get(b)
            .and_then(|v| v.cover())
            .map(|c| c.intersect(&high_cover) == 2)
            .unwrap_or(false);
        if !collides {
            continue;
        }
        saw_conflict = true;
        if !is_indirect_effect_of(data, b, call) {
            return false;
        }
    }
    saw_conflict
}

/// Is `vn` the output of a `CPUI_INDIRECT` whose effect op (the iop encoded in
/// input 1) is `call`?
fn is_indirect_effect_of(data: &Funcdata, vn: VarnodeId, call: OpId) -> bool {
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else {
        return false;
    };
    let Some(op) = data.obank().get(def) else {
        return false;
    };
    if op.code() != OpCode::CPUI_INDIRECT {
        return false;
    }
    op.get_in(1)
        .and_then(|iv| data.vbank().get(iv))
        .map(|iv| crate::funcdata_varnode::op_iop_decode(iv.get_addr().get_offset()))
        == Some(call)
}

/// Does `op` read a value `call` writes indirectly?
fn op_reads_indirect_effect_of(data: &Funcdata, op: OpId, call: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return true;
    };
    (0..o.num_input()).any(|slot| {
        o.get_in(slot)
            .filter(|&vn| {
                data.vbank().get(vn).map(|v| !v.is_annotation()).unwrap_or(false)
            })
            .map(|vn| is_indirect_effect_of(data, vn, call))
            .unwrap_or(false)
    })
}

//===========================================================================
// Option gate
//===========================================================================

/// (kuna) Toggle the self-effect discount: `foldcallretphi on|off`.
pub struct OptionFoldCallRetPhi;

impl OptionFoldCallRetPhi {
    /// The option name.
    pub const NAME: &'static str = "foldcallretphi";

    /// Parse `on|off` and return the resolved flag + confirmation message.  The
    /// caller writes the flag into `Architecture::fold_call_ret_phi`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Call-return folding past the merge phalanx turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_foldcallretphi/tests.rs"]
mod tests;
