//! (kuna `bejoinpass`) A big-endian register pair that a call hands back and
//! the function returns untouched is one value, joined in one order.
//!
//! `option bejoin` decides the order of a function's returned pair from that
//! function's own verdict, and the order of a call's output pair from its
//! callee's verdict ([`crate::kuna_bejoin::call_join_order`]). A pair that
//! passes from one to the other unchanged is judged twice, and the two
//! verdicts can disagree:
//!
//! * `u64 w(u64 a, u64 b) { return add64(a, b); }` on MIPS o32 big-endian is
//!   `jal add64; nop; lw $31,..; jr $31`. `add64` is proven, so the call reads
//!   `v0:v1` high word first. `w` itself is not (nothing in it, and no caller,
//!   proves a 64-bit value), and `passthrough`'s tail-return claim gave it
//!   the callee's two registers, so its RETURN joined them first register low:
//!   `return CONCAT44((int)v1,(int)(v1 >> 0x20));`, the callee's value with
//!   its halves swapped.
//! * The other way round, a function proven by its callers, or declared, that
//!   returns what an unproven or unknown callee left in the pair (a MIPS PIC
//!   call through the GOT, a PowerPC tail call to an unproven callee) joined
//!   the call's pair first register low and its own RETURN high word first,
//!   which prints the same swap.
//!
//! The pair is the same two registers on both sides, so it is one value. A
//! side that is proven fixes the order of both:
//!
//! * [`settle_return`]: when every live RETURN hands back the two registers a
//!   call to a proven callee left, untouched (through moves and phis, before
//!   or after the call's output is built), the function's pair is that
//!   callee's value. `whole` (the default) joins it in the ABI's order, so the
//!   wrapper prints `return add64(a,b);`. The same bytes are also `u32 hi(u64
//!   a, u64 b) { return add64(a, b) >> 32; }`, which compiles to the identical
//!   instructions; nothing in the function tells the two apart. `high` returns
//!   the first register alone instead, the callee's high word, `return
//!   (int)(add64(a,b) >> 0x20);`. A function proven itself (by its callers
//!   reading the second register, or a declared or DWARF 8-byte return) keeps
//!   the pair whole in either mode.
//! * [`call_order`]: a call whose callee is not proven joins its pair in the
//!   ABI's order when the function's RETURN, already joined high word first,
//!   reads that pair's two registers untouched in the same places.
//!
//! Only a pair of two four-byte registers is judged, and only under a proof
//! mode of `bejoin` (`evidence`, `callers`); `off` leaves both sides to their
//! own verdicts.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// How many moves and phis the lockstep walks follow.
const MAX_DEPTH: u32 = 8;

/// What `option bejoinpass` does with a pair a call hands back to the RETURN.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BeJoinPassMode {
    /// Each side joins by its own verdict.
    Off,
    /// One value, in the order of its proven side; an unproven function
    /// returning a proven callee's pair returns the whole value.
    #[default]
    Whole,
    /// [`Self::Whole`], except that an unproven function returning a proven
    /// callee's pair returns its first register alone.
    High,
}

impl BeJoinPassMode {
    pub fn as_str(self) -> &'static str {
        match self {
            BeJoinPassMode::Off => "off",
            BeJoinPassMode::Whole => "whole",
            BeJoinPassMode::High => "high",
        }
    }

    /// Parse an `option bejoinpass` value.
    pub fn parse(p1: &str) -> KunaResult<(BeJoinPassMode, String)> {
        let mode = match p1 {
            "off" => BeJoinPassMode::Off,
            "whole" => BeJoinPassMode::Whole,
            "high" => BeJoinPassMode::High,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown bejoinpass value: {other} (expected off|whole|high)"
                )))
            }
        };
        Ok((mode, format!("Handed-back register pair join set to {}", mode.as_str())))
    }
}

fn mode(data: &Funcdata) -> BeJoinPassMode {
    data.get_arch().be_join_pass
}

/// The model's pair, when it is two four-byte registers the ABI returns high
/// word first and both options are live.
fn word_pair(data: &Funcdata) -> Option<crate::kuna_bejoin::WidePair> {
    if mode(data) == BeJoinPassMode::Off || !crate::kuna_bejoin::mode(data).proves() {
        return None;
    }
    let pair = data.kuna_wide()?.pair.clone();
    (pair.hi.1 == 4 && pair.lo.1 == 4).then_some(pair)
}

/// The one trial of `active` on `loc`.
fn trial_on(active: &ParamActive, loc: &(kuna_base::address::Address, int4)) -> Option<int4> {
    let found: Vec<int4> = (0..active.get_num_trials())
        .filter(|&i| {
            let t = active.get_trial(i);
            t.get_address() == &loc.0 && t.get_size() == loc.1 && t.get_slot() > 0 && !t.is_unref()
        })
        .collect();
    match found[..] {
        [i] => Some(i),
        _ => None,
    }
}

/// `vn` with moves stripped: COPY (an injected `v0 = v0` included) and an
/// INDIRECT that is not a call's creation.
fn strip(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..data.vbank().num_varnodes() {
        let Some(op) = data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            return vn;
        };
        match op.code() {
            OpCode::CPUI_COPY => {}
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => {}
            _ => return vn,
        }
        match op.get_in(0) {
            Some(next) => vn = next,
            None => return vn,
        }
    }
    vn
}

fn phi(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    let def = data.vbank().get(vn)?.get_def()?;
    (data.obank().get(def)?.code() == OpCode::CPUI_MULTIEQUAL).then_some(def)
}

/// Walk `hi` and `lo` back together through moves and phis, calling `end` on
/// each pair of sources. `false` when a walk cannot be followed or `end`
/// refuses a pair.
fn lockstep(data: &Funcdata, hi: VarnodeId, lo: VarnodeId, depth: u32, end: &mut dyn FnMut(VarnodeId, VarnodeId) -> bool) -> bool {
    if depth == 0 {
        return false;
    }
    let (hi, lo) = (strip(data, hi), strip(data, lo));
    let input = |op: OpId, slot: int4| data.obank().get(op).and_then(|o| o.get_in(slot));
    let arity = |op: OpId| data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
    let parent = |op: OpId| data.obank().get(op).and_then(|o| o.get_parent());
    match (phi(data, hi), phi(data, lo)) {
        (Some(h), Some(l)) if parent(h) == parent(l) && arity(h) == arity(l) => (0..arity(h)).all(|slot| {
            matches!((input(h, slot), input(l, slot)), (Some(a), Some(b)) if lockstep(data, a, b, depth - 1, end))
        }),
        (None, None) => end(hi, lo),
        _ => false,
    }
}

/// The call whose output `hi` and `lo` are, high word first in the model's
/// pair: either both of its trials' INDIRECT creations, before its output is
/// built, or the most and least significant words of the output it was given.
fn handing_call(data: &Funcdata, hi: VarnodeId, lo: VarnodeId, pair: &crate::kuna_bejoin::WidePair) -> Option<(OpId, bool)> {
    let def = |vn: VarnodeId| data.vbank().get(vn)?.get_def().and_then(|d| data.obank().get(d).map(|o| (d, o)));
    let (_, hop) = def(hi)?;
    let (_, lop) = def(lo)?;
    let at = |vn: VarnodeId, loc: &(kuna_base::address::Address, int4)| {
        data.vbank().get(vn).is_some_and(|v| v.get_addr() == &loc.0 && v.get_size() == loc.1)
    };
    if hop.is_indirect_creation() && lop.is_indirect_creation() {
        let call = |o: &crate::op::PcodeOp| {
            let iop = data.vbank().get(o.get_in(1)?)?;
            let c = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
            matches!(data.obank().get(c)?.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND).then_some(c)
        };
        let c = call(hop)?;
        return (call(lop)? == c && at(hi, &pair.hi) && at(lo, &pair.lo)).then_some((c, false));
    }
    if hop.code() != OpCode::CPUI_SUBPIECE || lop.code() != OpCode::CPUI_SUBPIECE || hop.get_in(0) != lop.get_in(0) {
        return None;
    }
    let whole = hop.get_in(0)?;
    let offset = |o: &crate::op::PcodeOp| data.vbank().get(o.get_in(1)?).filter(|c| c.is_constant()).map(|c| c.get_offset());
    let w = data.vbank().get(whole)?;
    let call = w.get_def()?;
    let code = data.obank().get(call)?.code();
    (w.get_size() == pair.hi.1 + pair.lo.1
        && matches!(code, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
        && offset(hop)? == pair.lo.1 as u64
        && offset(lop)? == 0
        && data.vbank().get(hi).is_some_and(|v| v.get_size() == pair.hi.1)
        && data.vbank().get(lo).is_some_and(|v| v.get_size() == pair.lo.1))
    .then_some((call, true))
}

/// Settle a function's returned pair when every live RETURN hands back, untouched, the
/// pair a call to a proven callee left. Returns `true` when the pair is to join in the
/// ABI's order (`whole`); under `high` an unproven function's second register is made
/// inactive instead. Runs after the trials are scored, before the output map is derived.
pub fn settle_return(data: &Funcdata, active: &mut ParamActive, return_ops: &[OpId]) -> bool {
    let Some(pair) = word_pair(data) else { return false };
    if data.get_func_proto().is_output_locked() || crate::kuna_bejoin::proven_here(data) {
        return false;
    }
    let (Some(hi), Some(lo)) = (trial_on(active, &pair.hi), trial_on(active, &pair.lo)) else { return false };
    if (0..active.get_num_trials()).any(|i| active.get_trial(i).is_active() != (i == hi || i == lo)) {
        return false;
    }
    let (hslot, lslot) = (active.get_trial(hi).get_slot(), active.get_trial(lo).get_slot());
    let mut live = 0;
    for &op in return_ops {
        let Some(ret) = data.obank().get(op) else { return false };
        if ret.is_dead() || ret.get_halt_type() != 0 || crate::kuna_bejoin::never_reached(data, op) {
            continue;
        }
        live += 1;
        let (Some(h), Some(l)) = (ret.get_in(hslot), ret.get_in(lslot)) else { return false };
        let proven_call = &mut |h: VarnodeId, l: VarnodeId| match handing_call(data, h, l, &pair) {
            Some((_, true)) => true,
            Some((call, false)) => data.get_call_specs_index(call).is_some_and(|i| {
                crate::kuna_bejoin::proven_callee(data, data.get_call_specs(i).get_entry_address())
            }),
            None => false,
        };
        if !lockstep(data, h, l, MAX_DEPTH, proven_call) {
            return false;
        }
    }
    if live == 0 {
        return false;
    }
    match mode(data) {
        BeJoinPassMode::Whole => true,
        BeJoinPassMode::High => {
            active.get_trial_mut(lo).mark_inactive();
            false
        }
        BeJoinPassMode::Off => false,
    }
}

/// The order a call's two output trials join in: `order` (the callee's verdict), or the
/// ABI's order when the function's RETURN reads the call's two registers, untouched, as
/// the high and low words of the pair it returns. `trialvn` holds each trial's INDIRECT
/// creation by original slot, as in `buildOutputFromTrials`.
pub fn call_order(data: &Funcdata, active: &ParamActive, trialvn: &[Option<VarnodeId>], order: (int4, int4)) -> (int4, int4) {
    let Some(pair) = word_pair(data) else { return order };
    let abi = active.join_pair_order();
    if order == abi || active.get_num_trials() != 2 || !(0..2).all(|i| active.get_trial(i).is_used()) {
        return order;
    }
    let creation = |i: int4, loc: &(kuna_base::address::Address, int4)| {
        let t = active.get_trial(i);
        if t.get_address() != &loc.0 || t.get_size() != loc.1 {
            return None;
        }
        trialvn.get((t.get_slot() - 1) as usize).copied().flatten()
    };
    let (Some(hv), Some(lv)) = (creation(abi.1, &pair.hi), creation(abi.0, &pair.lo)) else { return order };
    let returned = data.obank().iter_code(OpCode::CPUI_RETURN).any(|op| {
        let Some(ret) = data.obank().get(op) else { return false };
        if ret.is_dead() || ret.get_halt_type() != 0 || ret.num_input() != 2 {
            return false;
        }
        let Some(piece) = ret.get_in(1).and_then(|v| data.vbank().get(v)).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) else {
            return false;
        };
        if piece.code() != OpCode::CPUI_PIECE {
            return false;
        }
        let (Some(h), Some(l)) = (piece.get_in(0), piece.get_in(1)) else { return false };
        let mut found = false;
        lockstep(data, h, l, MAX_DEPTH, &mut |a, b| {
            found |= a == hv && b == lv;
            true
        });
        found
    });
    if returned {
        abi
    } else {
        order
    }
}
