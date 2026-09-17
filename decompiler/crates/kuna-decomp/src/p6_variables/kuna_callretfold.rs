//! (kuna) Call-return variable folding — inline a single-use call return into
//! its use site (port of angr's "call return variable folding"; test
//! `test_call_return_variable_folding`, S6).
//!
//! # The presentation gap
//!
//! Ghidra/kuna force **every** call output to be *explicit* in
//! [`ActionMarkExplicit::base_explicit`](crate::p6_variables::coreaction_cleanup)
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
//! Gated by the `foldcallret` option (default-ON per DIV-14), [`call_output_foldable`]
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
//!      barrier ([`op_is_barrier`]), and none **reads a value the call
//!      indirectly writes** — an input defined by a `CPUI_INDIRECT` whose effect
//!      op (the iop encoding in input 1) is this call.  Sinking the call past
//!      such a read (e.g. the out-parameter copy `Merge::snipReads` places right
//!      after the call, GH-181) would hand the read the *pre*-call value;
//!   4. the statement the folded expression is finally printed in is still
//!      reachable under (3) — [`fold_print_point_is_order_safe`], checked once
//!      the implied chain downstream of the use is classified.
//!
//! Keeping `LOAD` in the forbidden set is necessary, not redundant: the call may
//! `STORE` memory that an intervening `LOAD` reads, so sinking the call past that
//! `LOAD` would change the value the load observes (a read-after-write hazard not
//! covered by the `STORE`/`CALL` checks alone).  The predicate strictly *tightens*
//! the printer's pre-existing single-use inlining with a same-block,
//! side-effect-free guard, preferring false negatives (stay explicit) over
//! reordering bugs.
//!
//! An opcode set alone is not enough to say "nothing in between writes memory
//! the callee reads": heritage promotes a write to a fixed address into a plain
//! `CPUI_COPY`, so `v = f(); glob = 42; use(v)` used to fold and evaluate `f()`
//! after the write to `glob` (GH-657) — the same goes for a frame slot whose
//! address escaped into the call.  [`op_is_barrier`] therefore also stops at any
//! op whose output varnode is persistent or address-tied
//! ([`op_writes_tied_storage`]).
//!
//! The direct call output can still have one descendant while a derived
//! truncation or arithmetic result fans out later. If that derived expression
//! has multiple uses, [`expression_contains_foldable_call`] makes it explicit
//! before multiplier analysis can duplicate the implied call expression at
//! each sink.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::context::{OpId, VarnodeId};

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
    let def_op = match data.obank().get(def) {
        Some(op) => op,
        None => return false,
    };
    if !def_op.is_call() || (def_op.code() == OpCode::CPUI_NEW && def_op.num_input() == 1) {
        return false;
    }

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

    // (3) no barrier strictly between the call and its use, and nothing there
    // reading a value this call indirectly writes.
    !ops[def_idx + 1..use_idx]
        .iter()
        .any(|&mid| op_is_barrier(data, mid) || op_reads_indirect_output_of(data, mid, def))
}

/// Does a derived value's printable implied expression contain a foldable call?
///
/// Explicit inputs stop the walk because they already provide one textual
/// evaluation point. A match means `root` must become that evaluation point.
pub fn expression_contains_foldable_call(data: &Funcdata, root: VarnodeId) -> bool {
    fn walk(data: &Funcdata, vn: VarnodeId, root: VarnodeId, seen: &mut Vec<VarnodeId>) -> bool {
        if seen.contains(&vn) {
            return false;
        }
        seen.push(vn);
        let Some(varnode) = data.vbank().get(vn) else {
            return false;
        };
        if vn != root && varnode.is_explicit() {
            return false;
        }
        let Some(def) = varnode.get_def() else {
            return false;
        };
        let Some(op) = data.obank().get(def) else {
            return false;
        };
        if op.is_call() {
            return call_output_foldable(data, vn);
        }

        let (slot, slotback) = match op.code() {
            OpCode::CPUI_LOAD => (1, 2),
            OpCode::CPUI_PTRADD => (0, 1),
            OpCode::CPUI_SEGMENTOP => (2, 3),
            _ => (0, op.num_input()),
        };
        (slot..slotback).any(|slot| {
            op.get_in(slot)
                .map(|input| walk(data, input, root, seen))
                .unwrap_or(false)
        })
    }

    walk(data, root, root, &mut Vec::new())
}

/// Does `op` read a varnode defined by a `CPUI_INDIRECT` attached to `call`
/// (its input 1 iop-encodes `call`)?  Such a read observes the call's indirect
/// effect, so the call expression must not be sunk past it (GH-181).  Marker
/// ops (a later call's own INDIRECTs chain the earlier call's versions as
/// inputs) have no textual evaluation point and are skipped.
pub(crate) fn op_reads_indirect_output_of(data: &Funcdata, op: OpId, call: OpId) -> bool {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return true, // stale: be conservative
    };
    if o.is_marker() {
        return false;
    }
    for slot in 0..o.num_input() {
        let def = o
            .get_in(slot)
            .and_then(|vn| data.vbank().get(vn))
            .filter(|v| !v.is_annotation())
            .and_then(|v| v.get_def());
        let ind = match def {
            Some(d) => d,
            None => continue,
        };
        let indop = match data.obank().get(ind) {
            Some(io) => io,
            None => continue,
        };
        if indop.code() != OpCode::CPUI_INDIRECT {
            continue;
        }
        let effect = indop
            .get_in(1)
            .and_then(|iv| data.vbank().get(iv))
            .map(|iv| crate::funcdata_varnode::op_iop_decode(iv.get_addr().get_offset()));
        if effect == Some(call) {
            return true;
        }
    }
    false
}

/// (kuna) foldcallret: does the fold of call output `vn` survive all the way to
/// the statement it is actually printed in?
///
/// [`call_output_foldable`] clears the span from the call to its single use, but
/// that use is the call's textual home only when the use op is itself a
/// statement.  If the use op's own output is *implied*, the expression keeps
/// travelling and is evaluated wherever that value is finally consumed, which
/// can be several statements later — past a write the callee reads (GH-657) or
/// in another block.  Called from
/// [`check_implied_cover`](crate::p6_variables::coreaction_cleanup), where
/// `ActionMarkImplied`'s descendants-first walk has already classified the
/// downstream values, so the landing statement is derivable.
pub fn fold_print_point_is_order_safe(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(def) = vn_def(data, vn) else {
        return false;
    };
    let Some(use_op) = data.lone_descend(vn) else {
        return false;
    };
    print_point_is_order_safe(data, def, use_op)
}

/// Longest implied chain [`print_point`] will chase before giving up.
const MAX_IMPLIED_CHAIN: usize = 8;

/// (kuna) The op at whose statement the expression rooted at `use_op` is printed.
///
/// An op is its own statement when it has no output (STORE, CBRANCH, a void
/// call, RETURN) or when its output is explicit.  Otherwise the output is
/// implied and the expression migrates into that value's own consumer, so the
/// walk follows the implied chain.  `None` means there is no provable single
/// print point: a marker, a fan-out, or a value not classified yet.
fn print_point(data: &Funcdata, use_op: OpId) -> Option<OpId> {
    let mut op = use_op;
    for _ in 0..MAX_IMPLIED_CHAIN {
        if op_is_marker(data, op) {
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
/// Runs the barrier test over the span from the call to the statement the folded
/// expression lands in, which is where it is evaluated at run time.  Only the
/// barrier test: the INDIRECT question — whether a read of the call's own effect
/// gets separated from it — is decided over the span to the use, where it always
/// was, and widening it here would *lift* declines as often as it adds them (the
/// collapsed INDIRECT of the call itself reads that effect by construction).  The
/// print point is not itself in the span: the folded expression is evaluated as
/// its operand, before it.
fn print_point_is_order_safe(data: &Funcdata, call: OpId, use_op: OpId) -> bool {
    let Some(point) = print_point(data, use_op) else {
        return false;
    };
    let Some(blk) = op_parent(data, call) else {
        return false;
    };
    if op_parent(data, point) != Some(blk) {
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
    !ops[ci + 1..pi].iter().any(|&mid| op_is_barrier(data, mid))
}

/// Is `op` a `CPUI_COPY` back into the storage its input already occupies?
///
/// `RuleIndirectCollapse` rewrites the INDIRECT a call attaches to a global it
/// turns out not to write as `glob = COPY glob`, and the same shape survives at
/// a `return`.  Such an op writes the value that is already there and reads only
/// to do so, so no callee can observe it on either side of the move; counting it
/// as a write would decline folds over a write that is not one.  A volatile
/// location is excluded: there the access itself is the effect.
fn op_is_self_copy(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if o.code() != OpCode::CPUI_COPY {
        return false;
    }
    let (Some(out), Some(inp)) = (o.get_out(), o.get_in(0)) else {
        return false;
    };
    let (Some(ov), Some(iv)) = (data.vbank().get(out), data.vbank().get(inp)) else {
        return false;
    };
    !ov.is_volatile()
        && !iv.is_volatile()
        && ov.get_size() == iv.get_size()
        && ov.get_offset() == iv.get_offset()
        && Rc::ptr_eq(ov.get_space(), iv.get_space())
}

/// An op whose relative order with the moved call is observable: any call, a
/// memory-touching op (LOAD/STORE/CALLOTHER), or a write to storage the callee
/// may read ([`op_writes_tied_storage`]).
fn op_is_barrier(data: &Funcdata, op: OpId) -> bool {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return true, // stale: be conservative
    };
    if o.is_call() {
        return true;
    }
    if matches!(
        o.code(),
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER
    ) {
        return true;
    }
    op_writes_tied_storage(data, op)
}

/// (kuna) Does `op` write storage the callee could read — memory, not a register?
///
/// Heritage promotes a write to a fixed address into a plain `CPUI_COPY` (or any
/// arithmetic op) whose output varnode is address-tied: a global is persistent,
/// a frame slot is tied to its stack address.  Neither is an opcode, so an
/// opcode-only barrier test waves both through, and folding the call past one
/// hands the callee the new value (GH-657).  For a global the callee needs
/// nothing but the address; for a frame slot it needs a pointer into the frame,
/// which it has whenever the frame address escaped, and an escaped slot is *not*
/// kept as a `CPUI_STORE` — heritage promotes it like any other.  So both are
/// barriers.  The cost is the false positives: a frame slot the callee cannot
/// reach also declines.
///
/// Marker ops are skipped: an INDIRECT/MULTIEQUAL performs no write of its own,
/// it records one its effect op performs, and that op is either the call being
/// folded or a barrier in its own right.
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
        && !op_is_self_copy(data, op)
}

pub(crate) fn op_is_marker(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).map(|o| o.is_marker()).unwrap_or(true)
}

pub(crate) fn op_parent(data: &Funcdata, op: OpId) -> Option<crate::context::BlockId> {
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
