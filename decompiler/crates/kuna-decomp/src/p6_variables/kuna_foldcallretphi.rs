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
//! exactly as they do in the spilled form.  Four further conditions bound it:
//!
//!   * the call's output must carry the callee's whole return value
//!     ([`call_output_is_full_width`]): a locked output, or an unlocked one
//!     that no wider return storage contains, up to `long long`.  An `eax` read
//!     out of `rax`, or out of `edx:eax` on i386, is narrowed only by the
//!     spill's declaration, and a fold would drop that,
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
//! changes what it returns.
//! [`op_writes_tied_storage`](crate::p6_variables::kuna_callretfold::op_writes_tied_storage)
//! is the missing barrier, and every span this module clears is tested with it —
//! for a frame slot as well, since the callee can reach one whenever the frame
//! address escaped into the call.  (The same hole is reachable through
//! `foldcallret` alone, without this option, when the call takes no global
//! operand; that is GH-657, fixed in `foldcallret`'s own two spans.  This
//! module's guard is deliberately left as it shipped: it is the wider of the
//! two — every opcode, the storage test unsoftened, and the INDIRECT question
//! over the whole distance — so the folds it discounts are exactly the ones it
//! discounted before, and the only thing this option inherits from the GH-657
//! fix is what that fix changes in `foldcallret`'s own rendering.)
//!
//! # Where the folded call is actually printed
//!
//! `call_output_foldable`'s span guard ends at the single use, which is the
//! call's textual home only when that use op is itself a statement.  If the use
//! op's own output is *implied*, the expression keeps travelling: it is printed
//! wherever that implied value is finally consumed, and that can be a later
//! block behind a branch.  The rejection this module discounts is sometimes the
//! only thing holding such a call in place, so the discount re-derives the real
//! print point (the end of
//! [`print_chain`](crate::p6_variables::kuna_callretfold::print_chain)) and
//! re-runs the span guard over the whole distance the call would move.  A print
//! point outside the call's own block, or one with a barrier or a global write
//! in between, declines.  So does a chain into the right-hand operand of a
//! `BOOL_AND`/`BOOL_OR`: all of those ops share one block, but C prints them as
//! `&&`/`||`, and a call in the operand they skip would stop being made.

use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::type_metatype;
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
    if !call_output_is_full_width(data, call) {
        return false;
    }
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

/// (kuna) Does the call survive the move all the way to its print point?
///
/// `call_output_foldable` has already cleared the span from the call to
/// `use_op`; this re-runs the guard over the span from the call to the statement
/// the folded expression actually lands in, which is where it is evaluated at
/// run time, and adds the memory-write barrier
/// [`op_writes_tied_storage`](crate::p6_variables::kuna_callretfold::op_writes_tied_storage)
/// that the opcode test misses.  The print point itself is not a barrier — the
/// folded expression is evaluated as its operand, before it — but its *other*
/// operands must not read an effect of the call, so the INDIRECT test covers it
/// too.  A chain into the right-hand operand of a short-circuit operator
/// declines: see [`chain_reaches_short_circuit_rhs`].
///
/// This is wider than either span `foldcallret` clears — every opcode, the
/// storage test with no self-copy exemption, no exemption for the ops the
/// expression travels through, and the INDIRECT question over the whole distance
/// — and stays that way on purpose: the folds it releases are ones the merge
/// machinery was holding, so they are the ones with the furthest to travel.
fn print_point_is_order_safe(data: &Funcdata, call: OpId, use_op: OpId) -> bool {
    let Some(chain) = crate::kuna_callretfold::print_chain(data, use_op) else {
        return false;
    };
    if chain_reaches_short_circuit_rhs(data, call, &chain) {
        return false;
    }
    let point = *chain.last().expect("print_chain: non-empty");
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
            || crate::kuna_callretfold::op_writes_tied_storage(data, mid)
            || crate::kuna_callretfold::op_reads_indirect_output_of(data, mid, call)
    });
    span_clear && !crate::kuna_callretfold::op_reads_indirect_output_of(data, point, call)
}

/// (kuna) Does the folded call's value reach the right-hand operand of an `&&`
/// or `||` on its way to the print point?
///
/// P-code's `BOOL_AND`/`BOOL_OR` evaluate both operands, and C's `&&`/`||` skip
/// the right-hand one when the left one decides.  A call folded there could
/// stop being made, so it keeps its own statement.  The left-hand operand is
/// always evaluated, and the printer emits input 0 on the left.
fn chain_reaches_short_circuit_rhs(data: &Funcdata, call: OpId, chain: &[OpId]) -> bool {
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

/// (kuna) Does `call`'s output carry the whole value its callee returns?
///
/// A locked output is the callee's declared type.  An unlocked one is only as
/// wide as this function reads it, while the callee may return more: `eax` out
/// of `rax`, or `eax` out of `edx:eax` on i386.  The spilled local's declaration
/// is then the only place the narrowing is written down, and folding drops it:
/// `int v = f(); v == -1` against an `unsigned long long f()` that returns
/// `0xffffffff` would become an always-false `f() == -1`.  Such a call keeps its
/// spill.  The output is narrowed when a wider return register holds it, or
/// when the model would return a wider integer, up to `long long`, in storage
/// that contains it ([`wider_int_return_contains`]).
fn call_output_is_full_width(data: &Funcdata, call: OpId) -> bool {
    let Some(out) = data.obank().get(call).and_then(|o| o.get_out()) else {
        return false;
    };
    let Some(v) = data.vbank().get(out) else {
        return false;
    };
    let Some(idx) = data.get_call_specs_index(call) else {
        return false;
    };
    let proto = data.get_call_specs(idx).proto();
    if proto.is_output_locked() {
        return true;
    }
    if !proto.has_model() {
        return false;
    }
    let Some(outlist) = proto.model().output_list() else {
        return false;
    };
    let (addr, size) = (v.get_addr(), v.get_size());
    let in_wider_register = outlist.get_entry().iter().any(|e| {
        e.get_min_size() <= size && e.get_size() > size && e.justified_contain(addr, size) >= 0
    });
    !in_wider_register && !wider_int_return_contains(data, outlist, addr, size)
}

/// (kuna) Would the model return an integer wider than `size` bytes, up to
/// `long long`, in storage that contains `(addr, size)`?
///
/// This is the joined case the per-register entries cannot see: `edx:eax` on
/// i386 is a `join` entry for 5..8 bytes, and on ARM `r0:r1` is not an entry at
/// all but a rule.  Asking the model where an `N`-byte `int` goes covers both.
/// Pairs wider than `long long` (`rdx:rax`, `x0:x1`) carry `__int128` or a
/// two-word struct and are left out, or every 64-bit output would count as
/// narrowed.  A model that cannot answer counts as narrowed.
fn wider_int_return_contains(
    data: &Funcdata,
    outlist: &crate::fspec::ParamListStandard,
    addr: &kuna_base::address::Address,
    size: i32,
) -> bool {
    let arch = data.get_arch();
    let Some(types) = arch.types() else {
        return true;
    };
    let widest = types.get_size_of_long_long();
    let mut wide = 2;
    while wide <= widest {
        if wide > size {
            let Ok(dt) = types.get_base(wide, type_metatype::TYPE_INT) else {
                return true;
            };
            let pieces = crate::fspec::PrototypePieces {
                outtype: Some(dt),
                first_var_arg_slot: -1,
                ..Default::default()
            };
            let mut res = Vec::new();
            match outlist.assign_map(&pieces, types, &mut res, arch.manage()) {
                Ok(()) => {}
                Err(KunaError::ParamUnassigned { .. }) => {
                    wide *= 2;
                    continue;
                }
                Err(_) => return true,
            }
            let Some(stored) = res.first() else {
                return true;
            };
            let stored_size = stored.type_.as_ref().map(|t| t.get_size()).unwrap_or(0);
            if !stored.addr.is_invalid()
                && stored_size > size
                && addr.overlap_join(0, &stored.addr, stored_size).unwrap_or(-1) >= 0
                && addr.overlap_join(size - 1, &stored.addr, stored_size).unwrap_or(-1) >= 0
            {
                return true;
            }
        }
        wide *= 2;
    }
    false
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
