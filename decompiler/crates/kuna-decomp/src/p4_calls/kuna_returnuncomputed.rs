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
//! # Why this cannot break a genuine multi-register return
//!
//! Two independent guards. First, the rule only ever runs with **more than one**
//! active trial, and never deactivates the last survivor — so a function with a
//! single recovered return register is untouched, whatever its value looks like.
//! Second, a real 16-byte struct return *computes* both halves: it builds them
//! from constants, arithmetic, or loads through a pointer, and a LOAD is not in
//! the move-only set, so the walk stops there and reports computed. Only a half
//! that is pure leftover — never written, or a callee's clobber — is dropped.

use kuna_base::address::Address;
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

/// Does `vn` carry a value the function actually computed?
///
/// Walks back through move-only operations; see the module docs for the
/// classification. Errs toward `true` (computed), which is the no-change answer.
fn computes_a_value(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    computes_from(data, vn, depth, None)
}

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
/// `placed_at` is the storage of the return half the walk started from, which
/// turns the carve-out into "did the function PUT an argument here": a terminal at
/// a different address was moved into the return register by an instruction the
/// function executed, while a terminal at the same address is the caller's
/// register passing straight through untouched -- leftover, and exactly what this
/// module exists to drop. `None` drops the placement test and is the shape-only
/// question the unit tests ask. See [`crate::kuna_retinputhalf`].
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
/// Returns `true` when a RETURN was rewritten.
pub fn strip_uncomputed_return_piece(data: &mut Funcdata) -> bool {
    // Collect first: the rewrite mutates the op bank.
    let mut fixes: Vec<(OpId, VarnodeId, OpId)> = Vec::new();
    for retop in data.obank().iter_code(OpCode::CPUI_RETURN).collect::<Vec<_>>() {
        let Some(o) = data.obank().get(retop) else { continue };
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() < 2 {
            continue;
        }
        let Some(joined) = o.get_in(1) else { continue };
        let Some(def) = data.vbank().get(joined).and_then(|v| v.get_def()) else { continue };
        let Some(piece) = data.obank().get(def) else { continue };
        // Only the two-register join return recovery builds; anything else is
        // someone else's op and stays.
        if piece.code() != OpCode::CPUI_PIECE || piece.num_input() != 2 {
            continue;
        }
        let (Some(hi), Some(lo)) = (piece.get_in(0), piece.get_in(1)) else { continue };
        let (hi_addr, lo_addr) = match (data.vbank().get(hi), data.vbank().get(lo)) {
            (Some(h), Some(l)) => (h.get_addr().clone(), l.get_addr().clone()),
            _ => continue,
        };
        let hi_real = computes_from(data, hi, 0, Some(&hi_addr));
        let lo_real = computes_from(data, lo, 0, Some(&lo_addr));
        let keep = match (hi_real, lo_real) {
            // Both halves carry a value: a genuine wide return. Leave it alone.
            (true, true) => continue,
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

    if fixes.is_empty() {
        return false;
    }
    let mut scratch: Vec<OpId> = Vec::new();
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
