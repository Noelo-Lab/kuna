//! (kuna) `calleepop` — recover how many argument bytes a callee pops when the
//! compiler spec cannot say.
//!
//! # The gap
//!
//! `StackSolver` (`coreaction.cc:48`) solves for the stack pointer's value at
//! every spacebase reference in a function.  A call whose prototype model
//! declares `extrapop="unknown"` contributes no equation, only a *guess*, and
//! upstream's guess is the constant `4` — the return address and nothing else,
//! i.e. "the callee pops none of its arguments".
//!
//! On x86 Windows that guess is wrong for the majority of calls.
//! `x86win.cspec`'s `<default_proto>` is `__stdcall` with `extrapop="unknown"`
//! precisely because the convention is decided per callee, and a `__stdcall`
//! callee pops `4 + <argument bytes>`.  The error is not local: the solver
//! latches its solution into `INT_ADD sp, #c` at every reference, so every
//! stack slot after the first call is displaced by the running total of the
//! argument bytes nobody popped.  Two consequences follow, and both are worse
//! than a cosmetic offset:
//!
//! * A slot written through a pointer taken *before* a call and read *after* it
//!   lands at two different offsets, so one stack object splits into two
//!   independent variables.  The read half has no reaching definition, so the
//!   decompiler assigns it a constant — the emitted C then computes over that
//!   constant instead of over the bytes the callee wrote.
//! * The displaced locals drift down onto the outgoing-argument slots of an
//!   earlier call.  Once a declared prototype claims those slots, the local can
//!   no longer be mapped and the reference prints as a raw `&Stackffffffe0`.
//!
//! # The mechanism
//!
//! The guess is refined for the one family whose convention the ABI fixes:
//! **an imported function**.  A Win32 API entry point is `__stdcall` by the
//! platform's own rule (`WINAPI`), and kuna already knows which callees are
//! imports — `peimportcall` paints `Varnode::externref` over the Import Address
//! Table, which is what lets `ActionDeindirect` resolve `call dword ptr [IAT]`
//! to a name in the first place.  A call to an *internal* function keeps
//! upstream's guess: MSVC compiles those `__cdecl` by default and nothing local
//! contradicts it.
//!
//! For an imported callee the guess is read off the caller's own stack
//! discipline, which is available at the point the solver runs (the first
//! heritage pass, before any argument recovery):
//!
//! * **Caller cleanup vetoes the guess.** If the stack pointer coming out of
//!   the call is consumed by an `INT_ADD sp, #+k`, the *caller* is popping the
//!   arguments — the callee is `__cdecl` and pops nothing.  The guess stays at
//!   upstream's `4`.
//! * **Otherwise the argument pushes are counted.** The call's return-address
//!   push sits at a known constant offset `d` from some base (the function
//!   input, an earlier call's result, or a merge).  Walking back up from `d+4`
//!   in pointer-sized steps, each step that is itself a spacebase reference
//!   with a STORE through it is one pushed argument.
//! * **The walk stops at the prologue.** A push whose stored value is the
//!   *input* Varnode of a register is a callee-save, not an argument, and ends
//!   the run — which is what separates `push ebx; push esi; push 0xa; push
//!   0x40; call` into two saves and two arguments.  A step with no spacebase
//!   reference at all (a bare `sub esp,n` frame allocation, whose Varnode is
//!   dead) ends it too.
//!
//! The result is `4 + <argument bytes>`, and it is still a *guess*: it is only
//! consulted where the exact equations leave the variable undetermined, so a
//! call the surrounding code already pins is unaffected.  Where the reading is
//! wrong the error is the same kind upstream already makes, and the option
//! restores the constant `4`.
//!
//! Inert wherever the compiler spec knows its own extrapop — `x86gcc.cspec`
//! (`__cdecl`, `extrapop="4"`), every RISC spec, and x86-64 — because those
//! calls never reach the guess at all.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleepop>` (kuna 4000+ range; 4150 = the previous high
/// water mark).
pub const ELEM_CALLEEPOP: ElementId = ElementId::new("calleepop", 4151);

/// (kuna) Recover a callee-pops-arguments purge for an unknown extrapop:
/// `calleepop on|off`.
pub struct OptionCalleePop;

impl OptionCalleePop {
    /// The option name.
    pub const NAME: &'static str = "calleepop";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_pop`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-pop argument recovery turned {prop}")))
    }
}

/// Upstream's constant guess (`coreaction.cc:238`): the return address only.
pub const RETURN_ADDRESS_ONLY: int4 = 4;

/// Most argument bytes a single call is credited with, so a malformed frame
/// cannot walk the whole stack.
const MAX_ARG_BYTES: int4 = 256;

/// The guessed extrapop for a call whose model leaves it unknown.
///
/// `call_sp` is the stack-pointer Varnode entering the call (the INDIRECT's
/// `in[0]`, i.e. the slot the return address was just pushed into) and
/// `call_out` is the INDIRECT's output.  Returns [`RETURN_ADDRESS_ONLY`]
/// whenever the caller's discipline does not say otherwise, which is exactly
/// upstream's answer.
pub fn guess_extra_pop(
    data: &Funcdata,
    spacebase: &Address,
    entry: &Address,
    call_sp: VarnodeId,
    call_out: VarnodeId,
) -> int4 {
    if !is_imported(data, entry) {
        return RETURN_ADDRESS_ONLY;
    }
    let (base, delta) = match decompose(data, spacebase, call_sp) {
        Some(bd) => bd,
        None => return RETURN_ADDRESS_ONLY,
    };
    let step = match data.vbank().get(call_sp).map(|v| v.get_size()) {
        Some(s) if s > 0 => s,
        _ => return RETURN_ADDRESS_ONLY,
    };
    // Count the argument pushes back from the return-address slot.  `delta` is
    // measured against whatever base the additive normalization settled on, so
    // the run can straddle offset zero -- the base Varnode itself is a pushed
    // slot whenever `INT_ADD(base,#0)` folded away.
    let mut bytes: int4 = 0;
    let mut off = delta + step;
    while bytes < MAX_ARG_BYTES {
        let slot = if off == 0 { Some(base) } else { child_at(data, spacebase, base, off) };
        match slot {
            Some(v) if is_argument_push(data, v) => {
                bytes += step;
                off += step;
            }
            _ => break,
        }
    }
    // A caller-side `add esp,#k` that covers the whole argument run is a
    // `__cdecl` cleanup, so the callee popped nothing.  The bound matters: a
    // mid-function `pop reg` also raises the stack pointer, and restoring one
    // saved register does not account for a multi-argument run.
    if bytes > 0 && caller_cleans_up(data, spacebase, call_out, bytes) {
        return RETURN_ADDRESS_ONLY;
    }
    RETURN_ADDRESS_ONLY + bytes
}

/// Is the callee an imported function?
///
/// `peimportcall` paints `Varnode::externref` over the Import Address Table, and
/// `ActionDeindirect` resolves a `call dword ptr [IAT slot]` through the
/// FunctionSymbol registered at that slot — so the call spec's entry address IS
/// the slot, and the property is readable straight off it.  A binary whose
/// imports were never marked up (a raw `.text` blob, or `peimportcall` off) has
/// no imported callees by this test, which leaves the whole function at
/// upstream's answer.
fn is_imported(data: &Funcdata, entry: &Address) -> bool {
    // A call whose target was never resolved carries the invalid Address, whose
    // space pointer is the sentinel — the property query would dereference it.
    let size = match entry.get_space() {
        Some(s) => s.get_addr_size() as int4,
        None => return false,
    };
    let props = data.get_arch().query_global_properties(entry, size, entry);
    (props & crate::varnode::varnode_flags::externref) != 0
}

/// Did the *caller* raise the stack pointer past the argument run after the
/// call?  That is a `__cdecl` cleanup, so the callee popped nothing.
///
/// The measurement is the highest stack-pointer value reachable from the call's
/// result, not the presence of an `add esp,#k` op: additive normalization
/// re-bases a following push run onto the call's own result and leaves the
/// cleanup Varnode itself dead, so the `add` is often not there to find.  What
/// survives is the *topmost pushed slot*, which sits one slot below the value
/// the cleanup produced.
///
/// The bound matters in the other direction too: a mid-function `pop reg` also
/// raises the stack pointer, and restoring one saved register does not account
/// for a multi-argument run.
fn caller_cleans_up(
    data: &Funcdata,
    spacebase: &Address,
    call_out: VarnodeId,
    argbytes: int4,
) -> bool {
    let step = match data.vbank().get(call_out).map(|v| v.get_size()) {
        Some(s) if s > 0 => s,
        _ => return false,
    };
    let mut high: int4 = if is_argument_push(data, call_out) { step } else { 0 };
    let descend: Vec<_> = match data.vbank().get(call_out) {
        Some(v) => v.descend_iter().collect(),
        None => return false,
    };
    for op in descend {
        let o = match data.obank().get(op) {
            Some(o) => o,
            None => continue,
        };
        if o.code() != OpCode::CPUI_INT_ADD || !at_address(data, o.get_out(), spacebase) {
            continue;
        }
        let other = if o.get_in(0) == Some(call_out) { o.get_in(1) } else { o.get_in(0) };
        let c = match const_signed(data, other) {
            Some(c) => c,
            None => continue,
        };
        let reached = match o.get_out() {
            Some(out) if is_argument_push(data, out) => c + step,
            _ => c,
        };
        if reached > high {
            high = reached;
        }
    }
    high >= argbytes
}

/// Split a spacebase reference into `(base, constant offset)` when it is
/// defined as `INT_ADD(base, #c)` with `base` another reference to the same
/// stack pointer.  `c` is not necessarily negative: the additive normalization
/// re-bases a run of pushes onto whichever earlier Varnode survived, so a call
/// preceded by an `add esp,#k` cleanup lands at a positive offset from it.
fn decompose(
    data: &Funcdata,
    spacebase: &Address,
    vn: VarnodeId,
) -> Option<(VarnodeId, int4)> {
    let op = data.vbank().get(vn)?.get_def()?;
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let (base, cvn) = match const_signed(data, o.get_in(1)) {
        Some(_) => (o.get_in(0)?, o.get_in(1)),
        None => (o.get_in(1)?, o.get_in(0)),
    };
    let c = const_signed(data, cvn)?;
    if !at_address(data, Some(base), spacebase) {
        return None;
    }
    Some((base, c))
}

/// The spacebase reference defined as `INT_ADD(base, #off)`, if the function
/// has one.
fn child_at(
    data: &Funcdata,
    spacebase: &Address,
    base: VarnodeId,
    off: int4,
) -> Option<VarnodeId> {
    let descend: Vec<_> = data.vbank().get(base)?.descend_iter().collect();
    for op in descend {
        let o = data.obank().get(op)?;
        if o.code() != OpCode::CPUI_INT_ADD {
            continue;
        }
        let other = if o.get_in(0) == Some(base) { o.get_in(1) } else { o.get_in(0) };
        if const_signed(data, other) != Some(off) {
            continue;
        }
        let out = o.get_out();
        if at_address(data, out, spacebase) {
            return out;
        }
    }
    None
}

/// Is this stack slot an argument push rather than a prologue register save?
///
/// A slot with no STORE through it is neither (a bare frame allocation), and
/// ends the run the same way a save does.
fn is_argument_push(data: &Funcdata, slot: VarnodeId) -> bool {
    let descend: Vec<_> = match data.vbank().get(slot) {
        Some(v) => v.descend_iter().collect(),
        None => return false,
    };
    let mut stored = false;
    for op in descend {
        let o = match data.obank().get(op) {
            Some(o) => o,
            None => continue,
        };
        if o.code() != OpCode::CPUI_STORE || o.get_in(1) != Some(slot) {
            continue;
        }
        stored = true;
        // The saved value of a callee-saved register is the register's own
        // input Varnode; nothing else the caller pushes can be.
        if let Some(v) = o.get_in(2).and_then(|v| data.vbank().get(v)) {
            if v.is_input() && v.get_space().get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR {
                return false;
            }
        }
    }
    stored
}

/// Is `vn` a reference to the stack pointer itself?
fn at_address(data: &Funcdata, vn: Option<VarnodeId>, spacebase: &Address) -> bool {
    vn.and_then(|v| data.vbank().get(v))
        .map(|v| crate::coreaction_stackptr::addr_eq(v.get_addr(), spacebase))
        .unwrap_or(false)
}

/// The signed value of a constant Varnode, sign-extended from its own size.
fn const_signed(data: &Funcdata, vn: Option<VarnodeId>) -> Option<int4> {
    let v = vn.and_then(|v| data.vbank().get(v))?;
    if !v.is_constant() {
        return None;
    }
    let size = v.get_size();
    if size <= 0 || size > 8 {
        return None;
    }
    let raw = v.get_offset();
    let shift = 64 - (size as u32) * 8;
    Some((((raw << shift) as i64) >> shift) as int4)
}

#[cfg(test)]
#[path = "kuna_calleepop/tests.rs"]
mod tests;
