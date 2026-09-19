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
//!   4. nothing that **writes state the callee may read** sits between the call
//!      and the statement the folded expression is finally printed in, which is
//!      further than the use whenever the use op's own output is implied
//!      ([`fold_print_point_is_order_safe`], asked once the implied chain below
//!      the use is classified).  The ops that chain travels through are not
//!      barriers to it: each consumes the previous one's value, so the call is
//!      evaluated before them either way;
//!   5. the call's value does not reach the **right-hand operand** of a
//!      `BOOL_AND`/`BOOL_OR` anywhere along that chain
//!      ([`chain_reaches_short_circuit_rhs`]).  P-code evaluates both operands,
//!      but C prints them as `&&`/`||`, which skip the right-hand one, so a
//!      call folded there stops being made whenever the left-hand side decides
//!      (GH-684).  Input 0 is printed on the left and always evaluated, so a
//!      call reaching it still folds.
//!
//! (4) asks a narrower question than (3) ([`op_is_write_barrier`] against
//! [`op_is_barrier`]: no `LOAD`), and deliberately.  Up to the use a read is
//! still a hazard this pass can decide — the call may `STORE` what the `LOAD`
//! reads, and the use op is the fold's textual home, so the `LOAD` is something
//! the call is moved across.  Past the use, the reads left in the span are the
//! ones the folded expression ends up printed *beside*, inside the one statement
//! the whole chain collapses into, and de-folding over those moves no statement:
//! it costs 117 further functions over the 37-binary corpus in
//! `docs/features/gh657/`, dominated by `__ctype_b_loc()` inlined into the same
//! conditional as the loads it now sits next to.  The same reasoning keeps the
//! INDIRECT half of (3) on the span to the use: `dat_33798 =
//! *__errno_location()` is one call, one load and one store in exactly that
//! order in the binary, and asking past the use de-folds
//! 27 more functions and corrects none.  `foldcallretphi` — the option that lets
//! a fold travel past a merge conflict at all — is what otherwise holds such
//! calls in place, so it re-asks both questions in full over the whole distance
//! for its own folds, exactly as it shipped.
//!
//! Keeping `LOAD` in (3)'s forbidden set is necessary, not redundant: the call
//! may `STORE` memory that an intervening `LOAD` reads, so sinking the call past
//! that `LOAD` would change the value the load observes (a read-after-write
//! hazard not covered by the `STORE`/`CALL` checks alone).  The predicate
//! strictly *tightens* the printer's pre-existing single-use inlining with a
//! same-block, side-effect-free guard, preferring false negatives (stay
//! explicit) over reordering bugs.
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

/// Longest implied chain [`print_chain`] will chase before giving up.
const MAX_IMPLIED_CHAIN: usize = 8;

/// (kuna) The ops the folded expression travels through, ending at the one whose
/// statement it is printed in.
///
/// An op is its own statement when it has no output (STORE, CBRANCH, a void
/// call, RETURN) or when its output is explicit.  Otherwise the output is
/// implied and the expression migrates into that value's own consumer, so the
/// walk follows the implied chain.  `None` means there is no provable single
/// print point: a marker, a fan-out, or a value not classified yet.
pub(crate) fn print_chain(data: &Funcdata, use_op: OpId) -> Option<Vec<OpId>> {
    let mut chain = Vec::with_capacity(MAX_IMPLIED_CHAIN);
    let mut op = use_op;
    for _ in 0..MAX_IMPLIED_CHAIN {
        if op_is_marker(data, op) {
            return None;
        }
        chain.push(op);
        let Some(out) = data.obank().get(op)?.get_out() else {
            return Some(chain);
        };
        let v = data.vbank().get(out)?;
        if v.is_explicit() {
            return Some(chain);
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
/// Runs [`op_is_write_barrier`] over the span from the call to the statement the
/// folded expression lands in, which is where it is evaluated at run time.  The
/// ops the expression travels *through* are exempt: each consumes the previous
/// one's value, so the call is still evaluated before them in the folded text
/// exactly as it is in the binary — a `LOAD` of the pointer a call just returned
/// is not a load the call was moved past.
///
/// Only writes are asked about here, and only opcodes — the INDIRECT question is
/// decided over the span to the use.  The module header measures what the two
/// wider forms of this test would cost.  A chain that carries the call into the
/// right-hand operand of `&&`/`||` declines before the span is looked at.
fn print_point_is_order_safe(data: &Funcdata, call: OpId, use_op: OpId) -> bool {
    let Some(chain) = print_chain(data, use_op) else {
        return false;
    };
    if chain_reaches_short_circuit_rhs(data, call, &chain) {
        return false;
    }
    let point = *chain.last().expect("print_chain: non-empty");
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
    !ops[ci + 1..pi]
        .iter()
        .any(|&mid| !chain.contains(&mid) && op_is_write_barrier(data, mid))
}

/// (kuna GH-684) Does the folded call's value reach the right-hand operand of an
/// `&&` or `||` on its way to the print point?
///
/// P-code's `BOOL_AND`/`BOOL_OR` evaluate both operands, and C's `&&`/`||` skip
/// the right-hand one when the left one decides.  A call folded there could
/// stop being made, so it keeps its own statement.  The left-hand operand is
/// always evaluated, and the printer emits input 0 on the left.  Both fold
/// predicates, this module's and `foldcallretphi`'s, ask it over the same chain.
pub(crate) fn chain_reaches_short_circuit_rhs(data: &Funcdata, call: OpId, chain: &[OpId]) -> bool {
    let Some(mut val) = data.obank().get(call).and_then(|o| o.get_out()) else {
        return true;
    };
    for &op in chain {
        let Some(o) = data.obank().get(op) else {
            return true;
        };
        if matches!(o.code(), OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR)
            && (1..o.num_input()).any(|slot| o.get_in(slot) == Some(val))
        {
            return true;
        }
        match o.get_out() {
            Some(out) => val = out,
            None => break,
        }
    }
    false
}

/// Is `op` a `CPUI_COPY` back into the storage its input already occupies?
///
/// `RuleIndirectCollapse` rewrites the INDIRECT a call attaches to a global it
/// turns out not to write as `glob = COPY glob`, and the same shape survives at
/// a `return`.  Such an op writes the value that is already there and reads only
/// to do so, so no callee can observe it on either side of the move; counting it
/// as a write would decline folds over a write that is not one.  A volatile
/// location is excluded: there the access itself is the effect.
///
/// The test compares storage — same space, offset and size — and not the SSA
/// version, which is what makes "the value already there" true: heritage links a
/// read of an address-tied location to the definition reaching that point, so
/// input 0 of the copy *is* the current value of the output's address.  A copy
/// that restored an older version would have to read it from somewhere else, and
/// that read is a different address.
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
/// may read ([`op_writes_observable_storage`]).
///
/// This is the set for the span from the call to its single use.  `foldcallretphi`
/// composes it too, over the wider span it clears for its own folds.
pub(crate) fn op_is_barrier(data: &Funcdata, op: OpId) -> bool {
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
    op_writes_observable_storage(data, op)
}

/// The same question past the single use, where only writes are left to ask
/// about: any call, a `STORE` or `CALLOTHER`, or a write to storage the callee
/// may read.  `LOAD` is not in the set — see the module header.
fn op_is_write_barrier(data: &Funcdata, op: OpId) -> bool {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return true, // stale: be conservative
    };
    if o.is_call() {
        return true;
    }
    if matches!(o.code(), OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER) {
        return true;
    }
    op_writes_observable_storage(data, op)
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
///
/// This is the storage test alone; [`op_writes_observable_storage`] is the one
/// the fold predicates use.  `foldcallretphi` composes this one, unsoftened, in
/// the span guard it shipped with.
pub(crate) fn op_writes_tied_storage(data: &Funcdata, op: OpId) -> bool {
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

/// [`op_writes_tied_storage`] minus the copies that store the value already
/// there ([`op_is_self_copy`]), which no callee can observe on either side of
/// the move.
fn op_writes_observable_storage(data: &Funcdata, op: OpId) -> bool {
    op_writes_tied_storage(data, op) && !op_is_self_copy(data, op)
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
