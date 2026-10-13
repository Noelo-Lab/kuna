//! (kuna) `option bejoin evidence`: join a two-register value in the ABI's
//! order only on proof that it is one value twice a register's width.
//!
//! An `int` worked out from the high half of a 64-bit temporary can leave the
//! temporary's low half in the second register, and then compiles to the same
//! instructions as the `long long` that returns the temporary whole (ARM's
//! `umull r1,r0` is both `((u64)a * b) >> 32` and `(u64)a * b`; PowerPC's
//! `addc 4,4,6; addze 3,3; add 3,3,5` is both `(a + b) >> 32` and `a + b`).
//! No reading of such bytes can tell the two apart, so they keep the
//! first-register-low join, which C's truncation reads as the right `int`.
//!
//! The ABI's order is taken only where every live RETURN holds one of these,
//! each of which an `int` function has no reason to compute:
//!
//! * a sign extension: the first register is the second shifted right
//!   arithmetically by all but one of its bits (`mr 4,3; srawi 3,3,31`);
//! * a zero extension: the first register is a literal zero beside a value;
//! * the two halves of one product, the low half made by an instruction of
//!   its own (PowerPC's `mullw` beside `mulhw`, MIPS's `mflo` beside `mfhi`),
//!   not by the instruction that also makes the high half (ARM's `umull`,
//!   SPARC's `umul`, which write it whether or not anything reads it).
//!
//! A call's result handed back whole is no proof: `return g() >> 32` of a
//! `long long g()` leaves `g`'s low word in the second register, the code of
//! `return g()`.
//!
//! The extended value must be made on purpose: a value the function computes
//! or another register's argument, not the second register's own entry value
//! (`srawi 3,4,31` is both `(long long)b` and `b >> 31`), not a load (a
//! volatile read leaves what it read wherever the register allocator put it),
//! not a call's or another instruction's by-product. Nothing but the RETURNs
//! (and the shift a sign extension makes from it) may read it. A RETURN never
//! reached, and the RETURN of a `restore` in a SPARC call's delay slot, are
//! not counted; a function whose code return recovery cannot all see (an
//! unfollowed indirect jump, an operation that may read any register) proves
//! nothing.
//!
//! A call's own two-register result joins in the ABI's order when the
//! callee's recovery returned its pair that way ([`callee_high_first`]), which
//! `decompile-all` makes available by decompiling callees first. A caller of
//! such a callee that reads only the first register reads the value's HIGH
//! word, not the whole of a narrower result, so the call is given the pair
//! and the read becomes its high piece ([`build_high_read`]).

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, ParamActive};
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;

use super::{entry_value_of, hidden_readers, literal_value, moved, moves_register_window, never_reached, undecided_input};

/// How many Varnodes one walk visits before it gives up (and proves nothing).
const MAX_NODES: usize = 1024;

/// How many moves [`unmove`] looks through.
const MAX_MOVES: u32 = 16;

/// Does every live RETURN of `data` hold the two used trials of `active`, in
/// the ABI's order, as one value twice a register's width (see the module
/// header)?
pub(super) fn proven_wide(active: &ParamActive, data: &Funcdata, return_ops: &[OpId]) -> bool {
    let used = (0..active.get_num_trials()).take_while(|&i| active.get_trial(i).is_used()).count();
    if used != 2 {
        return false;
    }
    let (lo, hi) = active.join_pair_order();
    let (lo_t, hi_t) = (active.get_trial(lo), active.get_trial(hi));
    if lo_t.get_size() != hi_t.get_size() {
        return false;
    }
    let pair = Pair {
        lo_slot: lo_t.get_slot(),
        hi_slot: hi_t.get_slot(),
        size: lo_t.get_size(),
        lo_reg: lo_t.get_address().clone(),
    };
    let live: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0) && !never_reached(data, r))
        .collect();
    if live.is_empty() || !hidden_readers(data).is_empty() {
        return false;
    }
    live.iter().all(|&r| wide_at(data, r, &pair))
}

/// The pair a RETURN carries, in the ABI's order.
struct Pair {
    lo_slot: int4,
    hi_slot: int4,
    size: int4,
    /// The second register, which holds the low word.
    lo_reg: Address,
}

/// Does `retop` hold one wide value in the pair's two slots?
fn wide_at(data: &Funcdata, retop: OpId, pair: &Pair) -> bool {
    let Some(o) = data.obank().get(retop) else { return false };
    let (Some(lo), Some(hi)) = (o.get_in(pair.lo_slot), o.get_in(pair.hi_slot)) else { return false };
    sign_extended(data, hi, lo, pair)
        || zero_extended(data, hi, lo, pair)
        || product_halves(data, hi, lo, pair)
}

/// The defining operation of `vn`, if any.
fn def(data: &Funcdata, vn: VarnodeId) -> Option<&PcodeOp> {
    data.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d))
}

/// `vn` before the moves that only carry it ([`moved`]).
fn unmove(data: &Funcdata, vn: VarnodeId) -> VarnodeId {
    let mut cur = vn;
    for _ in 0..MAX_MOVES {
        match moved(data, cur) {
            Some(x) => cur = x,
            None => break,
        }
    }
    cur
}

/// Is `hi` the low word `lo` shifted right arithmetically by all but one of
/// its bits, `lo` a value made on purpose and read by nothing but that shift
/// and the RETURNs?
fn sign_extended(data: &Funcdata, hi: VarnodeId, lo: VarnodeId, pair: &Pair) -> bool {
    let Some(shift_id) = data.vbank().get(unmove(data, hi)).and_then(|v| v.get_def()) else { return false };
    let Some(shift) = data.obank().get(shift_id).filter(|op| op.code() == OpCode::CPUI_INT_SRIGHT) else { return false };
    let (Some(src), Some(by)) = (shift.get_in(0), shift.get_in(1)) else { return false };
    let x = unmove(data, lo);
    literal_value(data, by, 1) == Some(8 * pair.size as u64 - 1)
        && unmove(data, src) == x
        && on_purpose(data, lo, pair)
        && only_returned(data, x, pair, Some(shift_id))
}

/// Is `hi` a literal zero beside a low word `lo` made on purpose and read by
/// nothing but the RETURNs?
fn zero_extended(data: &Funcdata, hi: VarnodeId, lo: VarnodeId, pair: &Pair) -> bool {
    literal_value(data, hi, 4) == Some(0) && on_purpose(data, lo, pair) && only_returned(data, unmove(data, lo), pair, None)
}

/// Is the low word `lo` (through moves, choices of such values, and the
/// extensions a byte or halfword load makes) a value the function computed or
/// another register's argument: not a literal, not the second register's own
/// entry value, not a load, not a call's result or clobber, not a piece of a
/// wider value?
fn on_purpose(data: &Funcdata, lo: VarnodeId, pair: &Pair) -> bool {
    if entry_value_of(data, lo, &pair.lo_reg) {
        return false;
    }
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![lo];
    while let Some(cur) = work.pop() {
        let cur = unmove(data, cur);
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_NODES || literal_value(data, cur, 4).is_some() {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        let Some(op) = def(data, cur) else {
            if !v.is_input() || v.get_addr() == &pair.lo_reg {
                return false;
            }
            continue;
        };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => work.extend((0..op.num_input()).filter_map(|i| op.get_in(i))),
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => work.extend(op.get_in(0)),
            OpCode::CPUI_LOAD
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_NEW
            | OpCode::CPUI_SEGMENTOP
            | OpCode::CPUI_CPOOLREF => return false,
            _ => {}
        }
    }
    true
}

/// Does nothing but moves into the low-word slot of the live RETURNs, and
/// the instruction at `except` (a sign extension's shift, with the flags it
/// sets and nothing reads), read `x`? A call whose inputs are still trials
/// does not yet read a register's entry value it finds in place.
fn only_returned(data: &Funcdata, x: VarnodeId, pair: &Pair, except: Option<OpId>) -> bool {
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work = vec![x];
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > MAX_NODES {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        for d in v.descend_iter() {
            let Some(op) = data.obank().get(d) else { continue };
            if op.is_dead() || Some(d) == except || except.is_some_and(|e| same_instruction_unread(data, d, e)) {
                continue;
            }
            match op.code() {
                OpCode::CPUI_RETURN => {
                    if op.get_halt_type() != 0 || never_reached(data, d) {
                        continue;
                    }
                    if (0..op.num_input()).any(|i| i != pair.lo_slot && op.get_in(i) == Some(cur)) {
                        return false;
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
                    if undecided_input(data, d) && (v.is_input() || entry_value_of(data, cur, v.get_addr())) => {}
                OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => work.extend(op.get_out()),
                OpCode::CPUI_INDIRECT if op.get_in(0) == Some(cur) && !op.is_indirect_creation() => {
                    work.extend(op.get_out())
                }
                _ => match op.get_out() {
                    Some(out) if moved(data, out) == Some(cur) => work.push(out),
                    _ => return false,
                },
            }
        }
    }
    true
}

/// Is `op` another p-code operation of the machine instruction `of` belongs
/// to, whose result nothing outside that instruction reads (the carry
/// `srawi` or `asr` computes)?
fn same_instruction_unread(data: &Funcdata, op: OpId, of: OpId) -> bool {
    let (Some(a), Some(b)) = (data.obank().get(op), data.obank().get(of)) else { return false };
    let at = b.get_addr();
    if a.get_addr() != at {
        return false;
    }
    let mut work: Vec<VarnodeId> = a.get_out().into_iter().collect();
    let mut seen: BTreeSet<VarnodeId> = BTreeSet::new();
    while let Some(cur) = work.pop() {
        if !seen.insert(cur) {
            continue;
        }
        if seen.len() > 64 {
            return false;
        }
        let Some(v) = data.vbank().get(cur) else { return false };
        for r in v.descend_iter() {
            let Some(r) = data.obank().get(r) else { continue };
            if r.is_dead() {
                continue;
            }
            if r.get_addr() != at {
                return false;
            }
            work.extend(r.get_out());
        }
    }
    true
}

/// Are `hi` and `lo` the high and low halves of one product of two extended
/// words, the low half made by an instruction of its own (a separate low
/// product, or a move out of the register the product left it in) and read
/// by nothing but the RETURNs?
fn product_halves(data: &Funcdata, hi: VarnodeId, lo: VarnodeId, pair: &Pair) -> bool {
    let Some(wide) = high_half_of(data, unmove(data, hi), pair.size) else { return false };
    let Some(mult) = def(data, wide).filter(|op| op.code() == OpCode::CPUI_INT_MULT) else { return false };
    let factors = |op: &PcodeOp| -> Option<BTreeSet<VarnodeId>> {
        let f = |i: int4| {
            let x = op.get_in(i)?;
            match def(data, x) {
                Some(e) if matches!(e.code(), OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT) => e.get_in(0).map(|y| unmove(data, y)),
                _ => None,
            }
        };
        Some([f(0)?, f(1)?].into_iter().collect())
    };
    let Some(wide_factors) = factors(mult) else { return false };
    let low = unmove(data, lo);
    let Some(op) = def(data, low) else { return false };
    let own = match op.code() {
        OpCode::CPUI_INT_MULT => {
            let both: Option<BTreeSet<VarnodeId>> =
                (0..2).map(|i| op.get_in(i).map(|x| unmove(data, x))).collect::<Option<BTreeSet<_>>>();
            both.as_ref() == Some(&wide_factors) && data.vbank().get(low).is_some_and(|v| v.get_size() == pair.size)
        }
        OpCode::CPUI_SUBPIECE => {
            op.get_in(0) == Some(wide)
                && op.get_in(1).and_then(|k| literal_value(data, k, 1)) == Some(0)
                && moved_out(data, lo, low, op.get_addr())
        }
        _ => false,
    };
    own && only_returned(data, moved_target(data, lo, low), pair, None)
}

/// The wide value `vn` is the high `size` bytes of: `SUBPIECE(w, size)`, or
/// `SUBPIECE(w >> 8*size, 0)`.
fn high_half_of(data: &Funcdata, vn: VarnodeId, size: int4) -> Option<VarnodeId> {
    let op = def(data, vn).filter(|op| op.code() == OpCode::CPUI_SUBPIECE)?;
    let w = op.get_in(0)?;
    if data.vbank().get(w)?.get_size() != 2 * size {
        return None;
    }
    match literal_value(data, op.get_in(1)?, 1)? {
        k if k == size as u64 => Some(w),
        0 => {
            let sh = def(data, w).filter(|s| s.code() == OpCode::CPUI_INT_RIGHT)?;
            (literal_value(data, sh.get_in(1)?, 1)? == 8 * size as u64).then_some(sh.get_in(0)?)
        }
        _ => None,
    }
}

/// Is there, between the RETURN's low word `lo` and `low`, a move of a
/// machine instruction other than the one at `made` and other than a
/// register window's `restore`?
fn moved_out(data: &Funcdata, lo: VarnodeId, low: VarnodeId, made: &Address) -> bool {
    let mut cur = lo;
    for _ in 0..MAX_MOVES {
        if cur == low {
            return false;
        }
        let Some(d) = data.vbank().get(cur).and_then(|v| v.get_def()) else { return false };
        let Some(op) = data.obank().get(d) else { return false };
        if op.get_addr() != made && !moves_register_window(data, d) && op.code() != OpCode::CPUI_INDIRECT {
            return true;
        }
        match moved(data, cur) {
            Some(x) => cur = x,
            None => return false,
        }
    }
    false
}

/// The first Varnode, walking from the RETURN's `lo` back toward `low`, that
/// a move of its own instruction made ([`moved_out`]), or `low`.
fn moved_target(data: &Funcdata, lo: VarnodeId, low: VarnodeId) -> VarnodeId {
    let made = def(data, low).map(|op| op.get_addr().clone());
    let mut cur = lo;
    let mut last = low;
    for _ in 0..MAX_MOVES {
        if cur == low {
            break;
        }
        let Some(op) = def(data, cur) else { break };
        if Some(op.get_addr()) != made.as_ref() && op.code() == OpCode::CPUI_COPY {
            last = cur;
        }
        match moved(data, cur) {
            Some(x) => cur = x,
            None => break,
        }
    }
    last
}

/// Did the callee at `entry` -- in its own recovery, which `decompile-all`
/// records before it reaches the callers -- return the two trials of
/// `active` as one value with the first register's word high, as the ABI
/// returns it?
pub(super) fn callee_high_first(data: &Funcdata, entry: &Address, active: &ParamActive) -> bool {
    let used = (0..active.get_num_trials()).take_while(|&i| active.get_trial(i).is_used()).count();
    if used != 2 {
        return false;
    }
    let (lo, hi) = active.join_pair_order();
    let (lo_t, hi_t) = (active.get_trial(lo), active.get_trial(hi));
    let Some(sp) = entry.get_space() else { return false };
    let Some((addr, size)) = data.kuna_callee_return_storage((sp.get_index(), entry.get_offset())) else { return false };
    if *size != lo_t.get_size() + hi_t.get_size() {
        return false;
    }
    let offset = |reg: &Address, rsize: int4| -> Option<int4> {
        if !addr.is_join() {
            let k = addr.justified_contain(*size, reg, rsize, false);
            return (k >= 0).then_some(k);
        }
        let jr = data.get_arch().manage.find_join(addr.get_offset()).ok()?;
        let mut at = 0;
        for i in (0..jr.num_pieces()).rev() {
            let p = jr.get_piece(i);
            let k = p.get_addr().justified_contain(p.size as int4, reg, rsize, false);
            if k >= 0 {
                return Some(at + k);
            }
            at += p.size as int4;
        }
        None
    };
    offset(hi_t.get_address(), hi_t.get_size()) == Some(lo_t.get_size()) && offset(lo_t.get_address(), lo_t.get_size()) == Some(0)
}

/// The (high, low) register halves of `size` bytes at `addr`: the pieces of a
/// two-piece join, or the two halves of one register by significance.
fn halves(data: &Funcdata, addr: &Address, size: int4) -> Option<((Address, int4), (Address, int4))> {
    if addr.is_join() {
        let jr = data.get_arch().manage.find_join(addr.get_offset()).ok()?;
        if jr.num_pieces() != 2 {
            return None;
        }
        let piece = |i| (jr.get_piece(i).get_addr(), jr.get_piece(i).size as int4);
        return Some((piece(0), piece(1)));
    }
    let half = size / 2;
    if half <= 0 || 2 * half != size {
        return None;
    }
    let space = addr.get_space()?;
    let a = Address::new(space.clone(), addr.get_offset());
    let b = Address::new(space.clone(), addr.get_offset() + half as u64);
    if addr.justified_contain(size, &a, half, false) == half {
        Some(((a, half), (b, half)))
    } else {
        Some(((b, half), (a, half)))
    }
}

/// Give a call whose callee's own recovery returned its pair high word
/// first, as the ABI returns it ([`callee_high_first`]), that pair as its
/// output when the caller reads only the first register: the read becomes
/// the pair's high piece, `(int)(g() >> 32)`, rather than the whole of a
/// narrower result, which C's truncation would read as the low word.
///
/// Runs in `buildOutputFromTrials` while the unused trials are still present;
/// `trialvn` holds each trial's INDIRECT creation, by original slot. Returns
/// `false`, changing nothing, unless the pair is built.
pub(crate) fn build_high_read(fc: &mut FuncCallSpecs, data: &mut Funcdata, trialvn: &[Option<VarnodeId>]) -> bool {
    if super::mode(data) != super::BeJoinMode::Evidence || !fc.proto().has_model() {
        return false;
    }
    let reads: Vec<(Address, int4, VarnodeId)> = {
        let active = fc.get_active_output();
        (0..active.get_num_trials())
            .map(|i| active.get_trial(i))
            .filter(|t| t.is_used())
            .filter_map(|t| trialvn.get((t.get_slot() - 1) as usize).copied().flatten().map(|vn| (t.get_address().clone(), t.get_size(), vn)))
            .collect()
    };
    let [(ref raddr, rsize, vn)] = reads[..] else { return false };
    let entry = fc.get_entry_address().clone();
    let Some(sp) = entry.get_space() else { return false };
    let Some((addr, size)) = data.kuna_callee_return_storage((sp.get_index(), entry.get_offset())).cloned() else {
        return false;
    };
    let Some(((hi_addr, hi_size), (lo_addr, lo_size))) = halves(data, &addr, size) else { return false };
    let within = hi_addr.justified_contain(hi_size, raddr, rsize, false);
    if within < 0 {
        return false;
    }
    let manager = data.get_arch().manage.clone();
    let mut probe = ParamActive::new(false);
    for (a, s) in [(&hi_addr, hi_size), (&lo_addr, lo_size)] {
        probe.register_trial(a, s);
    }
    for i in 0..2 {
        probe.get_trial_mut(i).mark_active();
    }
    if fc.proto().derive_output_map(&mut probe, &manager).is_err()
        || !(0..2).all(|i| probe.get_trial(i).is_used())
        || probe.join_pair_order() == (0, 1)
        || !callee_high_first(data, &entry, &probe)
    {
        return false;
    }
    let op = fc.get_op();
    let Some(call_addr) = data.obank().get(op).map(|o| o.get_addr().clone()) else { return false };
    let Some(joinaddr) = crate::kuna_rustabi::pair_join_address(data, &hi_addr, hi_size, &lo_addr, lo_size, &call_addr)
    else {
        return false;
    };
    let Ok(whole) = data.new_varnode_out(lo_size + hi_size, &joinaddr, op) else { return false };
    if let Some(v) = data.vbank_mut().get_mut(whole) {
        v.set_write_mask();
    }
    let indop = data.vbank().get(vn).and_then(|v| v.get_def());
    let sub = data.new_op(2, call_addr);
    data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
    let c = data.new_constant(4, (lo_size + within) as u64);
    let _ = data.op_set_input(sub, whole, 0);
    let _ = data.op_set_input(sub, c, 1);
    let _ = data.op_set_output(sub, vn);
    data.op_insert_after(sub, op);
    if let Some(indop) = indop {
        data.op_destroy(indop);
    }
    true
}
