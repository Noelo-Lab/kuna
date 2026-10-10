//! (kuna) `calleepopslot` — keep [`crate::p6_variables::kuna_calleepop`]'s
//! guess right when a stack slot is pushed in front of one call and popped by a
//! later one.
//!
//! # The gap
//!
//! MSVC builds a class returned by value directly in the slot that is then the
//! by-value argument of the next call:
//!
//! ```text
//!   push ecx            ; the slot for Load's result
//!   mov  ecx, esp
//!   push 0x2a0017
//!   push ecx            ; hidden return pointer
//!   call [Load]         ; __cdecl
//!   add  esp, 8         ; the slot stays on the stack
//!   push eax            ; hidden return pointer
//!   lea  ecx, [ebp-8]
//!   call [Arr::operator=] ; __thiscall, ret 8: pops the pointer AND the slot
//! ```
//!
//! `calleepop` reads both cleanups wrong, and each error moves every later
//! stack reference in the function:
//!
//! * **`Load`.** The push run in front of the call is three slots, because the
//!   slot sits right above the two arguments. `add esp,8` covers two of them, so
//!   the bounded caller-cleanup veto does not fire and `Load` is credited with
//!   popping all three. The next push then lands on a local, and every later
//!   call loses its stack arguments.
//! * **`operator=`.** `add esp,8` and the push cancel, so the return-address
//!   slot is `Load`'s own result and the guess gives up at `4`. Walked, the run
//!   would still stop at the first slot not pushed since `Load` returned, so
//!   the slot pushed before `Load` would not count either.
//!
//! The class size is not in an MSVC mangled name, so a demangled prototype does
//! not settle `operator=`'s cleanup either; the caller's stack is the only
//! evidence.
//!
//! # The mechanism
//!
//! Both halves read the caller's stack depth for consistency instead of the push
//! run alone.
//!
//! * **A push above the call's own result vetoes a callee pop.** If the caller,
//!   after the call, raises the stack pointer and pushes at or above the slot
//!   the return address left (a store through `call_out + c`, `c > 0`), then the
//!   callee popped nothing: had it popped the run, that push would land above
//!   the run, on the caller's own frame. The push must come before any other
//!   call, whose own pop would move it. An epilogue raises the stack pointer
//!   too, so the veto is declined when the run ended at a saved register, when a
//!   slot up to the push is popped into a register first, and when the store is
//!   only a later call's return address.
//! * **A return-address slot that is an earlier reference is walked** from that
//!   reference, but only when the caller's own stack activity continues from
//!   the call's result, so that a cleanup after the call could be seen, and
//!   only when no cleanup is deferred past the next call: MSVC cleans two
//!   `__cdecl` calls with one `add esp,8` after the second, so a raise after
//!   the next call that (taking that call to pop nothing) reaches above this
//!   call's result keeps `calleepop`'s `4`.
//! * **The run continues past the call right before it.** When the walk
//!   reaches a slot that nothing pushed since that call `P` returned, and the
//!   caller never raised the stack pointer to the slot after `P`, the slot is
//!   still the one that was there while `P` ran. If `P`'s pop is settled, the
//!   walk continues in `P`'s frame, and a slot there counts only when its push
//!   precedes `P` (same block, or a dominating one). Settled means an exact
//!   extrapop, or the return-address-only guess of a call with nothing pushed
//!   for it or whose pushes the caller cleans up; a guessed callee pop is not
//!   settled, so a wrong guess does not carry into the next call.
//!
//! Only consulted where `calleepop` already is: a call whose model leaves its
//! extrapop unknown, and only an imported callee is credited with a pop, so
//! inert on every spec that states its own.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p6_variables::kuna_calleepop::{
    at_address, caller_cleans_up, caller_raise, child_at, const_signed, decompose,
    is_argument_push, is_imported, MAX_ARG_BYTES, RETURN_ADDRESS_ONLY,
};

/// `option calleepopslot on|off`.
pub struct OptionCalleePopSlot;

impl OptionCalleePopSlot {
    /// The option name.
    pub const NAME: &'static str = "calleepopslot";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_pop_slot`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-pop slot tracking turned {prop}")))
    }
}

/// The settled pop of each earlier call, by the stack-pointer Varnode its
/// INDIRECT produces.
pub type SettledPops = BTreeMap<VarnodeId, int4>;

/// The push run in front of a call.
struct Run {
    /// Argument bytes counted.
    bytes: int4,
    /// The walk ended at a slot that stores a register's input value.
    at_save: bool,
    /// The return-address slot is an earlier reference itself, which
    /// `calleepop` does not walk.
    folded: bool,
}

/// The guessed extrapop for a call whose model leaves it unknown, and whether
/// it is settled enough for a later call's walk to continue past it.
///
/// Same inputs as [`crate::kuna_calleepop::guess_extra_pop`], plus the pops
/// settled for earlier calls.
pub fn guess_extra_pop(
    data: &Funcdata,
    spacebase: &Address,
    entry: &Address,
    call_sp: VarnodeId,
    call_out: VarnodeId,
    settled: &SettledPops,
) -> (int4, bool) {
    let run = push_run(data, spacebase, call_sp, call_out, settled);
    let cleaned = |run: &Run| {
        run.bytes > 0
            && (caller_cleans_up(data, spacebase, call_out, run.bytes)
                || (!run.at_save && pushes_above(data, spacebase, call_out)))
    };
    let run = match run {
        Some(r) => r,
        None => return (RETURN_ADDRESS_ONLY, false),
    };
    if run.bytes == 0 || cleaned(&run) {
        return (RETURN_ADDRESS_ONLY, true);
    }
    if !is_imported(data, entry) || (run.folded && deferred_cleanup(data, spacebase, call_out)) {
        return (RETURN_ADDRESS_ONLY, false);
    }
    (RETURN_ADDRESS_ONLY + run.bytes, false)
}

/// Count the argument pushes back from the return-address slot `call_sp`,
/// continuing past earlier calls whose pop is settled.
///
/// A return-address slot that is an earlier reference itself (the pushes
/// exactly undo a cleanup in front of them) is walked only when the caller's
/// stack after the call is visible, so a cleanup there could be seen. A slot
/// reached past an earlier call counts only when its push precedes that call.
fn push_run(
    data: &Funcdata,
    spacebase: &Address,
    call_sp: VarnodeId,
    call_out: VarnodeId,
    settled: &SettledPops,
) -> Option<Run> {
    let step = match data.vbank().get(call_sp).map(|v| v.get_size()) {
        Some(s) if s > 0 => s,
        _ => return None,
    };
    let (mut base, delta, folded) = match decompose(data, spacebase, call_sp) {
        Some((b, d)) => (b, d, false),
        None if continues(data, spacebase, call_out) => (call_sp, 0, true),
        None => return None,
    };
    let mut bytes: int4 = 0;
    let mut off = delta + step;
    let mut crossed: Option<OpId> = None;
    while bytes < MAX_ARG_BYTES {
        let slot = if off == 0 { Some(base) } else { child_at(data, spacebase, base, off) };
        if let Some(v) = slot {
            let before = crossed.is_none_or(|at| pushed_before(data, v, at));
            if before && is_argument_push(data, v) {
                bytes += step;
                off += step;
                continue;
            }
            if is_stored(data, v) {
                return Some(Run { bytes, at_save: before, folded });
            }
        }
        if crossed.is_some() {
            break;
        }
        match cross(data, spacebase, base, off, settled) {
            Some((b, o, at)) => {
                base = b;
                off = o;
                crossed = Some(at);
            }
            None => break,
        }
    }
    Some(Run { bytes, at_save: false, folded })
}

/// Re-express the slot `base + off` in the frame of the earlier call whose
/// INDIRECT produced `base`, when that call's pop is settled and the caller
/// never raised the stack pointer past the slot after it. Also returns that
/// INDIRECT, which sits right in front of the call.
fn cross(
    data: &Funcdata,
    spacebase: &Address,
    base: VarnodeId,
    off: int4,
    settled: &SettledPops,
) -> Option<(VarnodeId, int4, OpId)> {
    let pop = *settled.get(&base)?;
    let def = data.vbank().get(base)?.get_def()?;
    let o = data.obank().get(def)?;
    if o.code() != OpCode::CPUI_INDIRECT || off < caller_raise(data, spacebase, base) {
        return None;
    }
    let call_sp = o.get_in(0)?;
    if !at_address(data, Some(call_sp), spacebase) {
        return None;
    }
    let total = off + pop;
    Some(match decompose(data, spacebase, call_sp) {
        Some((b, d)) => (b, d + total, def),
        None => (call_sp, total, def),
    })
}

/// Is some STORE through `slot` ahead of `at` on every path to it: earlier in
/// the same block, or in a block that dominates `at`'s?
fn pushed_before(data: &Funcdata, slot: VarnodeId, at: OpId) -> bool {
    let Some(ref_op) = data.obank().get(at) else { return false };
    let Some(ref_block) = ref_op.get_parent() else { return false };
    let ref_order = ref_op.get_seq_num().get_order();
    let Some(v) = data.vbank().get(slot) else { return false };
    v.descend_iter().any(|op| {
        let Some(o) = data.obank().get(op) else { return false };
        if o.code() != OpCode::CPUI_STORE || o.get_in(1) != Some(slot) {
            return false;
        }
        match o.get_parent() {
            Some(b) if b == ref_block => o.get_seq_num().get_order() < ref_order,
            Some(b) => data.bblocks_ref().dominates(b, Some(ref_block)),
            None => false,
        }
    })
}

/// Does the caller's own stack activity continue from the call's result: a
/// stack-pointer reference derived from it, or a store through it?
fn continues(data: &Funcdata, spacebase: &Address, call_out: VarnodeId) -> bool {
    data.vbank().get(call_out).is_some_and(|v| {
        v.descend_iter().any(|op| {
            data.obank().get(op).is_some_and(|o| {
                at_address(data, o.get_out(), spacebase)
                    || (o.code() == OpCode::CPUI_STORE && o.get_in(1) == Some(call_out))
            })
        })
    })
}

/// Does the caller push through a stack pointer strictly above the call's
/// result, which only a raise followed by a push produces, before any other
/// call and without popping any slot up to that push first? A later call's own
/// pop would move the push, and an epilogue restores saved registers with
/// `pop reg` and may then call, so neither a pop nor a return-address store
/// counts.
fn pushes_above(data: &Funcdata, spacebase: &Address, call_out: VarnodeId) -> bool {
    let Some(from) = data.vbank().get(call_out).and_then(|v| v.get_def()) else { return false };
    let refs = offsets_from(data, spacebase, call_out);
    let Some(top) =
        refs.iter().filter(|(c, v)| *c > 0 && is_pushed(data, *v, from)).map(|(c, _)| *c).min()
    else {
        return false;
    };
    !refs.iter().any(|(c, v)| (0..=top).contains(c) && is_popped(data, *v))
}

/// Does the caller's cleanup after the NEXT call reach above this call's
/// result? That is a cleanup deferred across both calls (`call f; push x;
/// call g; add esp,8`), so this callee may have popped nothing. The next call
/// is taken to pop nothing itself, which only lowers the reach.
fn deferred_cleanup(data: &Funcdata, spacebase: &Address, call_out: VarnodeId) -> bool {
    offsets_from(data, spacebase, call_out).into_iter().filter(|(d, _)| *d <= 0).any(|(d, sp)| {
        let Some(v) = data.vbank().get(sp) else { return false };
        v.descend_iter().any(|op| {
            let Some(o) = data.obank().get(op) else { return false };
            if o.code() != OpCode::CPUI_INDIRECT
                || o.get_in(0) != Some(sp)
                || !is_call_effect(data, o.get_in(1))
            {
                return false;
            }
            let Some(next_out) = o.get_out() else { return false };
            let reach = d + RETURN_ADDRESS_ONLY + caller_raise(data, spacebase, next_out);
            reach > 0
        })
    })
}

/// `call_out` and every stack-pointer reference defined as `call_out + #c`, with
/// their offsets `c`.
fn offsets_from(
    data: &Funcdata,
    spacebase: &Address,
    call_out: VarnodeId,
) -> Vec<(int4, VarnodeId)> {
    let mut refs = vec![(0, call_out)];
    let Some(v) = data.vbank().get(call_out) else { return refs };
    for op in v.descend_iter() {
        let Some(o) = data.obank().get(op) else { continue };
        if o.code() != OpCode::CPUI_INT_ADD || !at_address(data, o.get_out(), spacebase) {
            continue;
        }
        let other = if o.get_in(0) == Some(call_out) { o.get_in(1) } else { o.get_in(0) };
        if let (Some(c), Some(out)) = (const_signed(data, other), o.get_out()) {
            refs.push((c, out));
        }
    }
    refs
}

/// Is `vn` the IOP annotation that ties an INDIRECT to a call?
fn is_call_effect(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
    vn.and_then(|v| data.vbank().get(v))
        .and_then(|v| v.get_addr().get_space())
        .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_IOP)
}

/// Is something other than a call's return address stored through this slot,
/// in the block of `from` (the call's INDIRECT) and ahead of the next call?
fn is_pushed(data: &Funcdata, slot: VarnodeId, from: OpId) -> bool {
    let Some(v) = data.vbank().get(slot) else { return false };
    v.descend_iter().any(|op| {
        data.obank().get(op).is_some_and(|o| {
            o.code() == OpCode::CPUI_STORE
                && o.get_in(1) == Some(slot)
                && !stores_return_address(data, op)
                && before_next_call(data, from, op)
        })
    })
}

/// Is `op` reached from `from` within its block, past `from`'s own call but no
/// other?
fn before_next_call(data: &Funcdata, from: OpId, op: OpId) -> bool {
    let Some(at) = data.obank().get(from).map(|o| o.get_addr().clone()) else { return false };
    let mut cur = from;
    loop {
        let Some(next) = data.obank().get(cur).and_then(|o| o.basic_neighbours().1) else {
            return false;
        };
        if next == op {
            return true;
        }
        let Some(o) = data.obank().get(next) else { return false };
        if o.is_call() && o.get_addr() != &at {
            return false;
        }
        cur = next;
    }
}

/// Is `store` part of a call instruction, i.e. the return-address push?
fn stores_return_address(data: &Funcdata, store: OpId) -> bool {
    let Some(at) = data.obank().get(store).map(|o| o.get_addr().clone()) else { return false };
    let mut cur = store;
    for _ in 0..64 {
        let Some(next) = data.obank().get(cur).and_then(|o| o.basic_neighbours().1) else {
            return false;
        };
        let Some(o) = data.obank().get(next) else { return false };
        if o.get_addr() != &at {
            return false;
        }
        if o.is_call() {
            return true;
        }
        cur = next;
    }
    false
}

/// Is this stack slot loaded whole into a register, as `pop reg` does?
fn is_popped(data: &Funcdata, slot: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(slot) else { return false };
    let size = v.get_size();
    v.descend_iter().any(|op| {
        let Some(o) = data.obank().get(op) else { return false };
        o.code() == OpCode::CPUI_LOAD
            && o.get_in(1) == Some(slot)
            && o.get_out().and_then(|out| data.vbank().get(out)).is_some_and(|out| {
                out.get_size() == size
                    && out.get_space().get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR
            })
    })
}

/// Is anything stored through this stack slot?
fn is_stored(data: &Funcdata, slot: VarnodeId) -> bool {
    data.vbank().get(slot).is_some_and(|v| {
        v.descend_iter().any(|op| {
            data.obank()
                .get(op)
                .is_some_and(|o| o.code() == OpCode::CPUI_STORE && o.get_in(1) == Some(slot))
        })
    })
}

#[cfg(test)]
#[path = "kuna_calleepopslot/tests.rs"]
mod tests;
