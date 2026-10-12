//! (kuna, ida) Reject a return register that carries no value the function ever
//! computed — the `undefined16 main(...)` / `return v4;` phantom.
//!
//! # The symptom
//!
//! An x86-64 SysV function with no known prototype was modelled as returning a
//! **16-byte** value, materialized by writing the genuine result to byte 0 and an
//! unrelated leftover to byte 8:
//!
//! ```text
//! char v4 [16];
//! v4[0] = v16 ^ 1;   // the real int result
//! v4[8] = v22;       // an uninitialized stack slot
//! return v4;
//! ```
//!
//! That output is not merely unreadable, it is wrong: it reads memory the
//! function never wrote. IDA Pro recovers `return (unsigned __int8)v16 ^ 1;`.
//!
//! # Why it happens
//!
//! Return recovery registers one trial per output register the prototype model
//! characterizes — for x86-64 gcc that is `RAX` *and* `RDX` — and marks a trial
//! active when its value survives ancestor-realism and is used only at the
//! RETURN. The compiler spec's output rule (`join_dual_class`) then accepts two
//! consecutive active trials as one 16-byte return, and the RETURN is rewritten
//! to `PIECE(RDX,RAX)`.
//!
//! Ancestor realism asks whether a value could *legitimately reach* the RETURN —
//! it is not asking whether the function meant to return it. Both of the shapes
//! that produce the phantom pass it:
//!
//! * a **callee-saved register restore**: the epilogue's `RDX` is a copy of an
//!   input Varnode for a stack slot the function never stores to, so the value is
//!   whatever the caller's frame happened to hold;
//! * a **clobber at a no-return call**: where the flow model turns a call that
//!   never returns into a return, `RDX` is the INDIRECT-creation standing for
//!   "the callee wrote something here", which is a statement about the callee,
//!   not a value.
//!
//! The upstream port already rejects the second shape *when it sees it* (a trial
//! formed from an INDIRECT creation is dropped unless it is first in its storage
//! class), but a trial is only checked once, at the first live RETURN — so a
//! function that reaches the restore shape first keeps the trial and joins.
//!
//! # When the decision is made
//!
//! Not at recovery time. There, the epilogue's restore is still
//! `COPY(LOAD(sp - k))` — indistinguishable from `return *p` — so there is
//! nothing to decide on. By the time the prototype is fixated, heritage has
//! resolved that load into a bare unwritten Varnode for a frame slot the function
//! never stores to, and the difference is plain. So this runs late, on the
//! already-built concatenation, and rewrites the RETURN to the half that carries
//! a value.
//!
//! # The rule
//!
//! A return trial whose value, at **every** live RETURN, traces back only to
//! things the function did not compute is not a return value.
//!
//! An unwritten Varnode that is a **formal input parameter** is the one exception
//! — the function was handed that value, so handing it back is a real return.
//! That carve-out is `option retinputhalf` and lives in
//! [`crate::kuna_retinputhalf`]; without it a returned pair whose high half is a
//! passthrough argument loses the half *and* the argument.
//!
//! "Did not compute" is decided by a bounded walk back through the operations
//! that only *move* a value — copies, phis, indirects, and piece/subpiece
//! reshaping — stopping at the first operation that produces one. A terminal is
//! uncomputed when it is an unwritten (input or free) Varnode or an INDIRECT
//! creation; a constant counts as computed, because returning a literal is a real
//! return. Anything the walk cannot classify is treated as computed, so an
//! unfamiliar shape keeps today's answer.
//!
//! # A value built in one return register
//!
//! The same `PIECE` appears when the function assembles a value in its single
//! return register: `((u64)hi << 32) | lo` folds into `RAX = PIECE(ESI, EDI)`,
//! and heritage splits a partly written register into `RAX = PIECE(RAX[4:4],
//! EAX)`. Each half is therefore judged against the bytes it occupies in the
//! returned storage (its register of a join, or its offset in the one register),
//! never against its own address -- an argument folded straight into the value
//! sits at its own address by definition and would read as leftover, dropping
//! the half, narrowing the return and orphaning the argument. And the high half
//! of one register is never kept alone: that would return its bits as the whole
//! value.
//!
//! # Why this cannot break a genuine multi-register return
//!
//! Two independent guards. First, the rule only ever edits a value concatenated
//! from two halves, and never deactivates the last survivor — so a function with
//! a single recovered return register is untouched, whatever its value looks like.
//! Second, a real 16-byte struct return *computes* both halves: it builds them
//! from constants, arithmetic, or loads through a pointer, and a LOAD is not in
//! the move-only set, so the walk stops there and reports computed. Only a half
//! that is pure leftover — never written, or a callee's clobber — is dropped.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How far back the walk chases move-only operations before giving up and
/// calling the value computed. Deep enough for the epilogue chains that produce
/// the phantom (a restore is a copy or two; a clobber is one indirect), shallow
/// enough that this never shows up in a profile.
const MAX_DEPTH: u32 = 24;

/// How many Varnodes the whole-function walk ([`computes_everywhere`]) visits
/// before giving up and answering `false` -- "not computed", which is the
/// refusal: the callee's recovered return is not stated, and every caller reads
/// the call exactly as it does with `passthrough` off. Exhausting it takes a
/// move-only closure of 4,096 Varnodes reachable from ONE returned value, which
/// is two orders of magnitude past the deepest copy/phi chain measured over the
/// decbench corpus, so the cap is a guard against a pathological body rather
/// than a budget the walk is expected to spend.
const MAX_NODES: u32 = 4096;

/// [`computes_from`], asked of every byte and every path instead of any one of
/// them: a value is computed only when NO terminal reachable through the
/// move-only operations is one the function never produced.
///
/// The relaxed question is the right one for the pair repair, which asks it of
/// one half at a time and has to keep a genuine wide return. A caller reading a
/// callee's recovered return type needs the strict one: a value pieced together
/// from a call's narrow result and a leftover, or merged from one path that
/// computes it and one that does not, is not a return value the caller can hand
/// on.
///
/// Written as a worklist over the reachable move-only closure rather than the
/// recursion [`computes_from`] uses: "every input" over a phi-rich -O0 body
/// revisits the same Varnodes exponentially, and this runs once per decompiled
/// function instead of once per returned register pair.
///
/// Running out of budget answers `false`, not `true`: the answer is read by
/// [`crate::p4_calls::kuna_protoorder`]'s `recovered_output`, where `true` states
/// the callee's return to every caller ahead of it and `false` states nothing at
/// all, so the unfinished walk has to take the second.
fn computes_everywhere(data: &Funcdata, vn: VarnodeId, placed_at: Option<&Address>, globals: bool) -> bool {
    let mut seen: std::collections::HashSet<VarnodeId> = std::collections::HashSet::new();
    let mut work: Vec<VarnodeId> = vec![vn];
    let mut budget = MAX_NODES;
    while let Some(cur) = work.pop() {
        if budget == 0 {
            // `true` here would STATE the callee's return on a body the walk
            // never finished reading, which is the direction that invents one.
            return false;
        }
        budget -= 1;
        if !seen.insert(cur) {
            continue;
        }
        let Some(v) = data.vbank().get(cur) else { continue };
        if v.is_constant() {
            continue;
        }
        let Some(def) = v.get_def() else {
            if placed_at.is_some_and(|a| a == v.get_addr()) {
                return false;
            }
            if globals && v.is_persist() {
                continue;
            }
            if !crate::kuna_retinputhalf::is_input_parameter(data, cur) {
                return false;
            }
            continue;
        };
        let Some(op) = data.obank().get(def) else { continue };
        if op.code() == OpCode::CPUI_INDIRECT && op.is_indirect_creation() {
            return false;
        }
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_SUBPIECE => {
                work.extend(op.get_in(0));
            }
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => {
                work.extend((0..op.num_input()).filter_map(|i| op.get_in(i)));
            }
            // Anything else produces a value; the walk stops here.
            _ => continue,
        }
    }
    true
}

/// The walk, carrying the input-parameter carve-out's **placement** test.
///
/// `placed_at` is the storage the half occupies in the returned value, which
/// turns the carve-out into "did the function PUT an argument here": a terminal at
/// a different address was moved into the return register by an instruction the
/// function executed, while a terminal at the same address is the caller's
/// register passing straight through untouched -- leftover, and exactly what this
/// module exists to drop. `None` drops the placement test: the pair repair asks
/// it of a half the function moved out of its register and back
/// ([`crate::kuna_retinputhalf::is_moved_back`]) and of the low half beside one,
/// and the unit tests ask the shape-only question with it. See
/// [`crate::kuna_retinputhalf`].
fn computes_from(data: &Funcdata, vn: VarnodeId, depth: u32, placed_at: Option<&Address>) -> bool {
    if depth >= MAX_DEPTH {
        return true;
    }
    let Some(v) = data.vbank().get(vn) else { return true };
    // A literal IS a computed return: `return 0;` is a return value.
    if v.is_constant() {
        return true;
    }
    // Never written: a function input, or a free Varnode standing for a location
    // the function only ever reads (the callee-saved restore shape). An input
    // parameter the function PLACED in the return register is a value it was
    // handed and is handing back, which is a real return.
    let Some(def) = v.get_def() else {
        if placed_at.is_some_and(|a| a == v.get_addr()) {
            return false;
        }
        return crate::kuna_retinputhalf::is_input_parameter(data, vn);
    };
    let Some(op) = data.obank().get(def) else { return true };
    // The callee wrote something here; that is a fact about the callee.
    if op.code() == OpCode::CPUI_INDIRECT && op.is_indirect_creation() {
        return false;
    }
    let inputs: Vec<VarnodeId> = match op.code() {
        // Pure moves: chase the source.
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_SUBPIECE => {
            op.get_in(0).into_iter().collect()
        }
        // Reshaping and phis: computed if ANY input is.
        OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_PIECE => {
            (0..op.num_input()).filter_map(|i| op.get_in(i)).collect()
        }
        // Anything else produces a value.
        _ => return true,
    };
    if inputs.is_empty() {
        return true;
    }
    inputs.into_iter().any(|i| computes_from(data, i, depth + 1, placed_at))
}

/// Does every live RETURN of `data` hand back a value the function computed?
///
/// The same walk the pair repair runs, asked of the whole function and of a
/// single return register, where the repair cannot act: it only ever chooses
/// between two active trials and never deactivates the last survivor, so a
/// function recovered with ONE return register keeps whatever reached the
/// RETURN. gnulib's `version_etc_arn` is `void`, ends its fallthrough path in a
/// `__fprintf_chk` and keeps that call's `RAX` clobber -- an INDIRECT creation --
/// as its "result", so kuna recovers it as returning `long`. A caller has no way
/// to see that from the recovered prototype alone, which is why `passthrough`
/// asks this question of the callee (see [`crate::kuna_passthrough`]).
///
/// `false` means at least one RETURN hands back a terminal the function never
/// computed. A function with no live RETURN answers `true`, the no-change answer.
pub fn every_return_computes(data: &Funcdata) -> bool {
    every_return_computes_with(data, false)
}

/// [`every_return_computes`], with a value read out of global memory
/// (`return stdout;`) counted as computed when `globals`: the program's own
/// data, not a register or frame slot the caller left behind.  `callrettype`
/// asks it this way ([`crate::p4_calls::kuna_callrettype`]).
pub fn every_return_computes_with(data: &Funcdata, globals: bool) -> bool {
    for retop in data.obank().iter_code(OpCode::CPUI_RETURN).collect::<Vec<_>>() {
        let Some(o) = data.obank().get(retop) else { continue };
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() < 2 {
            continue;
        }
        let Some(value) = o.get_in(1) else { continue };
        let placed = data.vbank().get(value).map(|v| v.get_addr().clone());
        if !computes_everywhere(data, value, placed.as_ref(), globals) {
            return false;
        }
    }
    true
}

/// The storage locations `vn` occupies, most significant first: the pieces of
/// a join, or `vn`'s own storage.
pub(crate) fn storage_pieces(data: &Funcdata, vn: VarnodeId) -> Option<Vec<(Rc<AddrSpace>, u64, i32)>> {
    let v = data.vbank().get(vn)?;
    let addr = v.get_addr();
    let space = addr.get_space()?;
    if space.get_type() != spacetype::IPTR_JOIN {
        return Some(vec![(Rc::clone(space), addr.get_offset(), v.get_size())]);
    }
    let rec = data.get_arch().manage().find_join(addr.get_offset()).ok()?;
    (0..rec.num_pieces())
        .map(|i| {
            let p = rec.get_piece(i);
            p.space.clone().map(|s| (s, p.offset, p.size as i32))
        })
        .collect()
}

/// Is `vn` stored across two or more locations (a register pair's join)?
fn spans_two_locations(data: &Funcdata, vn: VarnodeId) -> bool {
    storage_pieces(data, vn).is_some_and(|p| p.len() > 1)
}

/// Where the `width` bytes of `whole` starting `lsb` bytes above its least
/// significant byte are stored: the return register (or register of a pair) a
/// half of the returned value sits in. `None` when they straddle two pieces.
pub(crate) fn slot_storage(data: &Funcdata, whole: VarnodeId, lsb: i32, width: i32) -> Option<Address> {
    let mut lsb = lsb;
    for (space, off, size) in storage_pieces(data, whole)?.into_iter().rev() {
        if lsb < size {
            if lsb + width > size {
                return None;
            }
            let rel = if space.is_big_endian() { size - lsb - width } else { lsb };
            let at = space.wrap_offset(off.wrapping_add(rel as u64));
            return Some(Address::new(space, at));
        }
        lsb -= size;
    }
    None
}

/// The high half and low-half size of `whole = ZEXT(hi) << 8*k`, the form
/// `RuleConcatZero` rewrites the return pair `PIECE(hi, #0:k)` to.
fn zero_low_pair(data: &Funcdata, def: OpId) -> Option<(VarnodeId, i32, VarnodeId)> {
    let op = data.obank().get(def)?;
    if op.code() != OpCode::CPUI_INT_LEFT || op.num_input() != 2 {
        return None;
    }
    let whole = op.get_out()?;
    let sa = data.vbank().get(op.get_in(1)?)?;
    if !sa.is_constant() {
        return None;
    }
    let ext = data.obank().get(data.vbank().get(op.get_in(0)?)?.get_def()?)?;
    if ext.code() != OpCode::CPUI_INT_ZEXT {
        return None;
    }
    let hi = ext.get_in(0)?;
    let lo_size = data.vbank().get(whole)?.get_size() - data.vbank().get(hi)?.get_size();
    (lo_size > 0 && sa.get_offset() == 8 * lo_size as u64).then_some((hi, lo_size, whole))
}

/// Repair a RETURN whose value is a return-recovery register **pair** with an
/// uncomputed half: rewrite it to the half that carries a value, and destroy the
/// now-dead concatenation.
///
/// Runs late, in the one-shot tail, and that timing is the whole point. When
/// return recovery makes the pair decision the epilogue's restore still looks
/// like `COPY(LOAD(sp - k))` — indistinguishable from `return *p` — so there is
/// nothing to decide on. By the time the prototype is fixated, heritage has
/// resolved that load into a bare unwritten Varnode for a frame slot the function
/// never stores to, and the difference is plain.
///
/// A pair joined with its first register as the high half is left alone:
/// return recovery joins that way only a pair whose low word the function
/// returns on purpose ([`crate::kuna_bejoin`]). The second register of an i386
/// `EDX:EAX` join is never kept on its own; that RETURN returns nothing.
///
/// A pair joined first register low on an ABI that returns the first register
/// high ([`Funcdata::kuna_pairs_first_low`]) whose low half is a literal zero
/// is read in its folded form `ZEXT(hi) << 8*k`: the zero is kept when the
/// high half is uncomputed, as it is for any other literal.
///
/// Returns `true` when a RETURN was rewritten.
pub fn strip_uncomputed_return_piece(data: &mut Funcdata) -> bool {
    // Collect first: the rewrite mutates the op bank.
    let mut fixes: Vec<(OpId, VarnodeId, OpId)> = Vec::new();
    let mut own_pairs: Vec<(OpId, VarnodeId, OpId)> = Vec::new();
    let mut drops: Vec<(OpId, OpId)> = Vec::new();
    let mut zeros: Vec<(OpId, i32, OpId)> = Vec::new();
    let moved = |data: &Funcdata, vn: VarnodeId, slot: &Address, size: i32| {
        crate::kuna_retinputhalf::is_moved_back(data, slot, size) && computes_from(data, vn, 0, None)
    };
    for retop in data.obank().iter_code(OpCode::CPUI_RETURN).collect::<Vec<_>>() {
        let Some(o) = data.obank().get(retop) else { continue };
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() < 2 {
            continue;
        }
        let Some(joined) = o.get_in(1) else { continue };
        let Some(def) = data.vbank().get(joined).and_then(|v| v.get_def()) else { continue };
        if let Some((hi, lo_size, whole)) = zero_low_pair(data, def) {
            if !data.kuna_pairs_first_low() || !spans_two_locations(data, whole) {
                continue;
            }
            let hi_size = data.vbank().get(hi).map_or(0, |v| v.get_size());
            let (Some(hi_slot), Some(_)) =
                (slot_storage(data, whole, lo_size, hi_size), slot_storage(data, whole, 0, lo_size))
            else {
                continue;
            };
            if !computes_from(data, hi, 0, Some(&hi_slot)) && !moved(data, hi, &hi_slot, hi_size) {
                zeros.push((retop, lo_size, def));
            }
            continue;
        }
        let Some(piece) = data.obank().get(def) else { continue };
        // Only the two-register join return recovery builds; anything else is
        // someone else's op and stays.
        if piece.code() != OpCode::CPUI_PIECE || piece.num_input() != 2 {
            continue;
        }
        let (Some(hi), Some(lo), Some(whole)) = (piece.get_in(0), piece.get_in(1), piece.get_out())
        else {
            continue;
        };
        if crate::kuna_bejoin::first_register_holds_high(data, whole) {
            continue;
        }
        let (hi_addr, hi_size, lo_addr, lo_size) = match (data.vbank().get(hi), data.vbank().get(lo)) {
            (Some(h), Some(l)) => (h.get_addr().clone(), h.get_size(), l.get_addr().clone(), l.get_size()),
            _ => continue,
        };
        let hi_slot = slot_storage(data, whole, lo_size, hi_size).unwrap_or(hi_addr);
        let lo_slot = slot_storage(data, whole, 0, lo_size).unwrap_or(lo_addr);
        let hi_placed = computes_from(data, hi, 0, Some(&hi_slot));
        let lo_placed = computes_from(data, lo, 0, Some(&lo_slot));
        let hi_moved = !hi_placed && moved(data, hi, &hi_slot, hi_size);
        let lo_moved = !lo_placed && moved(data, lo, &lo_slot, lo_size);
        let keep = match (hi_placed || hi_moved, lo_placed || lo_moved) {
            // Both halves carry a value: a genuine wide return. Leave it alone.
            (true, true) => continue,
            // The high register is real only because the function moved its
            // argument back into it, which says the function returns the pair:
            // a low half that is the function's own argument, untouched, is
            // that argument handed back. Never the high register alone.
            (true, false) if hi_moved => {
                if computes_from(data, lo, 0, None) {
                    continue;
                }
                lo
            }
            // One return register holds both halves, so its high bits are not a
            // return value of their own: handing them back alone would return
            // them in place of the whole register.
            (true, false) if !spans_two_locations(data, whole) => continue,
            // The low register is the function's own argument, left in place or
            // reloaded from its spill, and the high register of a little-endian
            // pair is the second of its class, never a return value alone: the
            // pair stands unless another RETURN returns one register.
            (true, false) if !hi_slot.is_big_endian() && computes_from(data, lo, 0, None) => {
                own_pairs.push((retop, lo, def));
                continue;
            }
            // The high register of an i386 `EDX:EAX` join is never a return
            // value alone: the RETURN returns nothing.
            (true, false) if crate::kuna_retcallhalf::in_pair_second(data, &hi_slot, hi_size) => {
                drops.push((retop, def));
                continue;
            }
            (true, false) => hi,
            // Only the low half is real — the common case, a callee-saved restore
            // in the high register.
            (false, true) => lo,
            // Neither half is real: this is the return the flow model synthesizes
            // where a call that never returns falls through, and both registers
            // hold the callee's clobber. There is no return value to recover, but
            // the function's output storage has to agree across every RETURN, so
            // keep the low (first-in-class) register — what the model would have
            // picked had the join never formed.
            (false, false) => lo,
        };
        fixes.push((retop, keep, def));
    }

    if fixes.is_empty() && drops.is_empty() && zeros.is_empty() {
        return false;
    }
    for (retop, size, piece) in zeros {
        let zero = data.new_constant(size, 0);
        fixes.push((retop, zero, piece));
    }
    fixes.extend(own_pairs);
    let mut scratch: Vec<OpId> = Vec::new();
    for (retop, piece) in drops {
        data.op_remove_input(retop, 1);
        if data.obank().get(piece).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)).is_some_and(|v| v.has_no_descend()) {
            data.op_destroy_recursive(piece, &mut scratch);
        }
    }
    for (retop, keep, piece) in fixes {
        if data.op_set_input(retop, keep, 1).is_err() {
            continue;
        }
        // The concatenation now has no readers. Destroy it so the printer does
        // not emit the phantom `v[8] = <leftover>` write that materialized it.
        let unused = data
            .obank()
            .get(piece)
            .and_then(|o| o.get_out())
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.has_no_descend())
            .unwrap_or(false);
        if unused {
            data.op_destroy_recursive(piece, &mut scratch);
        }
    }
    true
}

#[cfg(test)]
#[path = "kuna_returnuncomputed/tests.rs"]
mod tests;
