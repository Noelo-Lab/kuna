//! (kuna) `calleepopframe` — read a callee's pop off a fixed-frame caller that
//! re-reserves the bytes the callee popped.
//!
//! # The gap
//!
//! GCC and Clang on i386 Windows reserve the outgoing-argument area once, in the
//! prologue, and write each call's arguments with `mov [esp+k]`. A `__stdcall`
//! callee pops that area, so the caller lowers the stack pointer again right
//! after the call to keep its frame fixed:
//!
//! ```text
//!   mov  [esp+0x10], eax
//!   ...
//!   mov  [esp], 0x80000001
//!   call [RegOpenKeyExA]   ; ret 0x14
//!   sub  esp, 0x14         ; the frame is back where it was
//! ```
//!
//! [`crate::p6_variables::kuna_calleepop`] reads a pop off the push run in
//! front of the call, and there is none here, so the guess stays at the return
//! address alone. The `sub` then lowers the solved stack pointer by the popped
//! bytes, and every later frame offset in the function is off by them: an
//! incoming argument reads as a local, and a local whose address one call
//! filled is read back from a different slot.
//!
//! # The mechanism
//!
//! For a call whose model leaves its extrapop unknown, the guess is `4 + n`
//! when both hold:
//!
//! * the first write to the stack pointer after the call, in its block, is
//!   `call result - n` with `0 < n <= 256`, a multiple of the pointer size
//!   (when that write is the next call's return-address push, the lowering
//!   folded into it, less the push; a push of an argument is no
//!   re-reservation);
//! * every pointer-sized slot of `[esp, esp + n)` at the call instruction is
//!   stored to between the previous call in the block (or its start) and this
//!   one, so the `n` bytes are the arguments the caller just wrote.
//!
//! The caller's frame is the evidence, not the callee's identity, so this also
//! covers a call through a register the target was spilled to and reloaded
//! from, which is still unresolved when the solver runs. A `__cdecl` callee in
//! such a frame is followed by no adjustment at all, and one in a push-style
//! frame by a raise. A push-style caller that defers a cleanup and lowers the
//! stack for the next call's alignment fails the second test, since the
//! alignment slots are never stored.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p6_variables::kuna_calleepop::{
    at_address, const_signed, decompose, MAX_ARG_BYTES, RETURN_ADDRESS_ONLY,
};

/// `option calleepopframe on|off`.
pub struct OptionCalleePopFrame;

impl OptionCalleePopFrame {
    /// The option name.
    pub const NAME: &'static str = "calleepopframe";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_pop_frame`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Fixed-frame callee-pop reading turned {prop}")))
    }
}

/// The extrapop a fixed-frame caller states for the callee of `call`, or `None`
/// when its frame says nothing.
///
/// `call_sp` and `call_out` are the call's stack-pointer INDIRECT input and
/// output, as for [`crate::kuna_calleepop::guess_extra_pop`].
pub fn guess_extra_pop(
    data: &Funcdata,
    spacebase: &Address,
    call: OpId,
    call_sp: VarnodeId,
    call_out: VarnodeId,
) -> Option<int4> {
    let step = data.vbank().get(call_sp)?.get_size();
    if step <= 0 {
        return None;
    }
    let n = rereserved(data, spacebase, call, call_out, step)?;
    if n <= 0 || n > MAX_ARG_BYTES || n % step != 0 {
        return None;
    }
    let (base, delta) = decompose(data, spacebase, call_sp)?;
    let stored = stored_offsets(data, spacebase, call, base, step);
    let first = delta + step;
    let staged = (0..n / step).all(|i| stored.contains(&(first + i * step)));
    staged.then_some(RETURN_ADDRESS_ONLY + n)
}

/// `n` when the first stack-pointer write after `call` in its block is
/// `call_out - n`, or is the next call's return-address push folded onto
/// `call_out - n`. A push is the next call's argument, not a re-reservation.
fn rereserved(
    data: &Funcdata,
    spacebase: &Address,
    call: OpId,
    call_out: VarnodeId,
    step: int4,
) -> Option<int4> {
    let mut cur = call;
    loop {
        cur = data.obank().get(cur)?.basic_neighbours().1?;
        let o = data.obank().get(cur)?;
        if o.is_call() {
            return None;
        }
        if !at_address(data, o.get_out(), spacebase) {
            continue;
        }
        if o.code() != OpCode::CPUI_INT_ADD {
            return None;
        }
        let other = match (o.get_in(0), o.get_in(1)) {
            (Some(a), b) if a == call_out => b,
            (a, Some(b)) if b == call_out => a,
            _ => return None,
        };
        let lowered = -const_signed(data, other)?;
        return match instruction_kind(data, cur) {
            Instruction::Call => Some(lowered - step),
            Instruction::Push => None,
            Instruction::Other => Some(lowered),
        };
    }
}

enum Instruction {
    Call,
    Push,
    Other,
}

/// What the instruction of the stack-pointer write `op` is: a call (`op` is
/// its return-address push), a push (it stores through `op`'s result), or
/// anything else.
fn instruction_kind(data: &Funcdata, op: OpId) -> Instruction {
    let Some(o) = data.obank().get(op) else { return Instruction::Other };
    let (at, sp) = (o.get_addr().clone(), o.get_out());
    let mut cur = op;
    let mut stored = false;
    while let Some(next) = data.obank().get(cur).and_then(|o| o.basic_neighbours().1) {
        let Some(o) = data.obank().get(next) else { break };
        if o.get_addr() != &at {
            break;
        }
        if o.is_call() {
            return Instruction::Call;
        }
        stored |= o.code() == OpCode::CPUI_STORE && o.get_in(1) == sp;
        cur = next;
    }
    if stored {
        Instruction::Push
    } else {
        Instruction::Other
    }
}

/// The offsets from `base` of the pointer-sized stack slots stored to between
/// the previous call in `call`'s block (or the block start) and `call`.
fn stored_offsets(
    data: &Funcdata,
    spacebase: &Address,
    call: OpId,
    base: VarnodeId,
    step: int4,
) -> Vec<int4> {
    let mut offsets = Vec::new();
    let mut cur = call;
    while let Some(prev) = data.obank().get(cur).and_then(|o| o.basic_neighbours().0) {
        cur = prev;
        let Some(o) = data.obank().get(cur) else { break };
        if o.is_call() {
            break;
        }
        if o.code() != OpCode::CPUI_STORE {
            continue;
        }
        let at = o.get_in(1).and_then(|a| offset_from(data, spacebase, base, a));
        let size = o.get_in(2).and_then(|v| data.vbank().get(v)).map(|v| v.get_size());
        if let (Some(at), Some(size)) = (at, size) {
            offsets.extend((0..size.max(1)).step_by(step as usize).map(|k| at + k));
        }
    }
    offsets
}

/// The offset from `base` of the address `addr`: a stack-pointer reference at
/// `base + #c`, or such a reference plus a constant.
fn offset_from(data: &Funcdata, spacebase: &Address, base: VarnodeId, addr: VarnodeId) -> Option<int4> {
    if at_address(data, Some(addr), spacebase) {
        return sp_offset(data, spacebase, base, addr);
    }
    let def = data.obank().get(data.vbank().get(addr)?.get_def()?)?;
    if def.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let (sp, k) = match (def.get_in(0), def.get_in(1)) {
        (Some(a), b) if at_address(data, Some(a), spacebase) => (a, const_signed(data, b)?),
        (a, Some(b)) if at_address(data, Some(b), spacebase) => (b, const_signed(data, a)?),
        _ => return None,
    };
    Some(sp_offset(data, spacebase, base, sp)? + k)
}

fn sp_offset(data: &Funcdata, spacebase: &Address, base: VarnodeId, sp: VarnodeId) -> Option<int4> {
    if sp == base {
        return Some(0);
    }
    match decompose(data, spacebase, sp)? {
        (b, c) if b == base => Some(c),
        _ => None,
    }
}

#[cfg(test)]
#[path = "kuna_calleepopframe/tests.rs"]
mod tests;
