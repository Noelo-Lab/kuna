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
//! exactly as they do in the spilled form.  Three further conditions bound it:
//!
//!   * the operand's high must not belong to a `VariableGroup` (the piece
//!     intersection loop of `inflate_test` reasons about overlapping storage,
//!     not versions, so its rejections are never discounted),
//!   * the use op must not itself read an INDIRECT effect of the call — there
//!     the folded text would name the operand's high both as the call's
//!     argument (pre-call) and as an operand of the use (post-call), and
//!   * the whole distance the call moves has to be order-safe
//!     ([`print_point_is_order_safe`]).
//!
//! # What order-safe means here, and why it is not `foldcallret`'s span
//!
//! `foldcallret`'s guard tests *opcodes*: no CALL, LOAD, STORE or CALLOTHER
//! between the call and its use.  That set is not the whole of "writes something
//! the callee can see".  A write to a fixed global address is heritaged into a
//! plain `CPUI_COPY`, which the opcode test waves through, so on
//!
//! ```text
//! v2 = helper(g);   // helper returns k
//! k = 42;           // a CPUI_COPY: no STORE, no LOAD, no call
//! ok = v1 & v2;
//! ```
//!
//! folding `helper(g)` into the last statement evaluates it *after* `k = 42` and
//! changes what it returns.  [`op_writes_tied_storage`] is the missing
//! barrier, and every span this module clears is tested with it — for a frame
//! slot as well, since the callee can reach one whenever the frame address
//! escaped into the call.  (The same hole
//! is reachable through `foldcallret` alone, without this option, when the call
//! takes no global operand; that is GH-657 and is fixed separately, since
//! `foldcallret` is default-on and this option is not.)
//!
//! # Where the folded call is actually printed
//!
//! `call_output_foldable`'s span guard ends at the single use, which is the
//! call's textual home only when that use op is itself a statement.  If the use
//! op's own output is *implied*, the expression keeps travelling: it is printed
//! wherever that implied value is finally consumed, and that can be a later
//! block behind a branch.  The rejection this module discounts is sometimes the
//! only thing holding such a call in place, so the discount re-derives the real
//! print point ([`print_point`]) and re-runs the span guard over the whole
//! distance the call would move.  A print point outside the call's own block, or
//! one with a barrier or a global write in between, declines.

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
    if !print_point_is_order_safe(data, call, use_op) {
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

/// Longest implied chain [`print_point`] will chase before giving up.
const MAX_IMPLIED_CHAIN: usize = 8;

/// (kuna) The op at whose statement the expression rooted at `use_op` is printed.
///
/// An op is its own statement when it has no output (STORE, CBRANCH, a void
/// call, RETURN) or when its output is explicit.  Otherwise the output is
/// implied and the expression migrates into that value's own consumer, so the
/// walk follows the implied chain.  `None` means there is no provable single
/// print point: a marker, a fan-out, or a value this pass has not classified yet.
fn print_point(data: &Funcdata, use_op: OpId) -> Option<OpId> {
    let mut op = use_op;
    for _ in 0..MAX_IMPLIED_CHAIN {
        if crate::kuna_callretfold::op_is_marker(data, op) {
            return None;
        }
        let out = data.obank().get(op)?.get_out();
        let Some(out) = out else {
            return Some(op);
        };
        let v = data.vbank().get(out)?;
        if v.is_explicit() {
            return Some(op);
        }
        if !v.is_implied() {
            return None;
        }
        op = data.lone_descend(out)?;
    }
    None
}

/// (kuna) Does the call survive the move all the way to its print point?
///
/// `call_output_foldable` has already cleared the span from the call to
/// `use_op`; this re-runs the same guard over the span from the call to the
/// statement the folded expression actually lands in, which is where it is
/// evaluated at run time, and adds the memory-write barrier
/// [`op_writes_tied_storage`] that the opcode test misses.  The print point
/// itself is not a barrier — the folded expression is evaluated as its operand,
/// before it — but its *other* operands must not read an effect of the call,
/// so the INDIRECT test covers it too.
fn print_point_is_order_safe(data: &Funcdata, call: OpId, use_op: OpId) -> bool {
    let Some(point) = print_point(data, use_op) else {
        return false;
    };
    let Some(blk) = crate::kuna_callretfold::op_parent(data, call) else {
        return false;
    };
    if crate::kuna_callretfold::op_parent(data, point) != Some(blk) {
        return false;
    }
    let ops = data.bb_ops(blk);
    let (Some(ci), Some(pi)) = (
        ops.iter().position(|&o| o == call),
        ops.iter().position(|&o| o == point),
    ) else {
        return false;
    };
    if pi <= ci {
        return false;
    }
    let span_clear = !ops[ci + 1..pi].iter().any(|&mid| {
        crate::kuna_callretfold::op_is_barrier(data, mid)
            || op_writes_tied_storage(data, mid)
            || crate::kuna_callretfold::op_reads_indirect_output_of(data, mid, call)
    });
    span_clear && !crate::kuna_callretfold::op_reads_indirect_output_of(data, point, call)
}

/// (kuna) Does `op` write storage the callee could read — memory, not a register?
///
/// Heritage promotes a write to a fixed address into a plain `CPUI_COPY` (or any
/// arithmetic op) whose output varnode is address-tied: a global is persistent,
/// a frame slot is tied to its stack address.  Neither is an opcode, so
/// [`op_is_barrier`](crate::p6_variables::kuna_callretfold) — which tests for
/// CALL/LOAD/STORE/CALLOTHER — waves both through, and folding the call past one
/// hands the callee the new value.  For a global the callee needs nothing but
/// the address (the shape in the module header); for a frame slot it needs a
/// pointer into the frame, which it has whenever the frame address escaped, and
/// an escaped slot is *not* kept as a `CPUI_STORE` — heritage promotes it like
/// any other.  So both are barriers here.  The cost is the false positives: a
/// frame slot the callee cannot reach also declines.
///
/// Marker ops are skipped: an INDIRECT/MULTIEQUAL performs no write of its own,
/// it records one its effect op performs, and that op is either the call being
/// folded (its own INDIRECTs are what this module exists to discount) or a
/// barrier in its own right.
fn op_writes_tied_storage(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return true; // stale: be conservative
    };
    if o.is_marker() {
        return false;
    }
    let Some(out) = o.get_out() else {
        return false;
    };
    data.vbank()
        .get(out)
        .map(|v| v.is_persist() || v.is_addr_tied())
        .unwrap_or(true)
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
