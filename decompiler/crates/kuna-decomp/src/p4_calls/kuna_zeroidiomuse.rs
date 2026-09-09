//! P4 self-cancelling-operand tolerance in input-trial scoring — the
//! `zeroidiomuse` decision point.
//!
//! # The gap
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! walks every descendant of the Varnode reaching a CALL in a candidate parameter
//! register and asks whether the value is used *only* as that call's argument.
//! Another CALL that consumes the same value rejects the trial, which
//! `FuncCallSpecs::check_input_trial_use` turns into `mark_inactive` — and an
//! inactive trial is CHECKED, so it is never re-scored and the argument
//! disappears.
//!
//! The walk follows the value through arbitrary arithmetic, and `x ^ x` is
//! arithmetic. That is a problem on x86, where clearing a register is spelled as
//! a self-XOR (and, less often, a self-subtract):
//!
//! ```text
//!   .Lloop:
//!     xor    esi,esi              ; <- consumes the previous iteration's rsi
//!     xor    edi,edi
//!     call   read_one
//!     ...
//!     mov    esi,0x2
//!     mov    edi,0x13050
//!     call   watchdog             ; <- the trials being scored
//!     jmp    .Lloop
//! ```
//!
//! The `INT_XOR` at the loop head is a live descendant of the value passed to
//! `watchdog`: it arrives there via the killed-by-call `INDIRECT`, the loop-head
//! `MULTIEQUAL` and the `SUBPIECE` that slices `esi` out of `rsi`. So the walk
//! reaches `call read_one`, whose own trial for that register is active, and both
//! of `watchdog`'s arguments go inactive — the call renders `watchdog()`.
//!
//! The competing use is fake. `INT_XOR(v, v)` is `0` whatever `v` is, so nothing
//! downstream of it can observe the value being scored; the second call is not
//! competing for the same parameter at all. The idiom survives to this point
//! because `ActionActiveParam` runs before the operands are common-subexpression
//! eliminated: the two `esi` reads are two distinct `SUBPIECE` Varnodes of one
//! `MULTIEQUAL`, so the constant fold that eventually produces `read_one(0, 0,
//! ...)` has not fired yet and the equality test has to be structural.
//!
//! # The rule
//!
//! Skip a descendant op entirely — neither reject on it nor continue the walk
//! through its output — when it is an `INT_XOR` or `INT_SUB` whose two operands
//! are the same value. Everything else keeps the upstream treatment. The test is
//! deliberately narrow: identical Varnodes, equal constants, or one of a short
//! whitelist of pure value-reshaping ops (`COPY`, `SUBPIECE`, `PIECE`,
//! `INT_ZEXT`, `INT_SEXT`) applied to operands that are themselves the same
//! value, to a bounded depth. It admits no new argument on its own; it only
//! declines to let a provably constant result veto one.

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How far [`same_value`] recurses through pure reshaping ops. Two is what the
/// register-slice idiom needs (`SUBPIECE` of a shared `MULTIEQUAL`); more only
/// buys cost.
const MAX_DEPTH: u32 = 2;

/// Whether the descendant op at `op` throws its operand away — an `INT_XOR` or
/// `INT_SUB` of a value with itself, whose result is `0` regardless. Off
/// restores the upstream treatment, where such an op is walked through like any
/// other arithmetic.
pub fn op_discards_operand(data: &Funcdata, op: OpId) -> bool {
    if !data.get_arch().zero_idiom_use {
        return false;
    }
    let Some(o) = data.obank().get(op) else {
        return false;
    };
    if !matches!(o.code(), OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB) {
        return false;
    }
    let (Some(a), Some(b)) = (o.get_in(0), o.get_in(1)) else {
        return false;
    };
    same_value(data, a, b, MAX_DEPTH)
}

/// Can `a` and `b` be proven to hold the same value without evaluating
/// anything? Identity, equal constants, or the same pure reshaping op over
/// operands that are themselves the same value.
fn same_value(data: &Funcdata, a: VarnodeId, b: VarnodeId, depth: u32) -> bool {
    if a == b {
        return true;
    }
    let (Some(va), Some(vb)) = (data.vbank().get(a), data.vbank().get(b)) else {
        return false;
    };
    if va.get_size() != vb.get_size() {
        return false;
    }
    if va.is_constant() || vb.is_constant() {
        return va.is_constant() && vb.is_constant() && va.get_offset() == vb.get_offset();
    }
    if depth == 0 {
        return false;
    }
    let (Some(da), Some(db)) = (va.get_def(), vb.get_def()) else {
        return false;
    };
    let (Some(oa), Some(ob)) = (data.obank().get(da), data.obank().get(db)) else {
        return false;
    };
    if oa.code() != ob.code() || !is_pure_reshape(oa.code()) {
        return false;
    }
    if oa.num_input() != ob.num_input() {
        return false;
    }
    (0..oa.num_input()).all(|i| match (oa.get_in(i), ob.get_in(i)) {
        (Some(x), Some(y)) => same_value(data, x, y, depth - 1),
        _ => false,
    })
}

/// The ops [`same_value`] will look through: pure, total, and a function of
/// their inputs alone.
fn is_pure_reshape(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
    )
}

#[cfg(test)]
#[path = "kuna_zeroidiomuse/tests.rs"]
mod tests;
