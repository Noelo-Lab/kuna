//! (kuna `calleeargs`) Bound an x86-32 call's stack arguments by what its
//! callee's own code touches.
//!
//! An argument pushed for an outer call before an inner call sits on the stack
//! across the inner call, and nothing at the inner call site tells the two
//! apart: a `__cdecl` callee also returns with `ret`, and MSVC may defer its
//! cleanup past the next call. The callee does tell: it never touches the stack
//! above its last argument.
//!
//! The extent comes from the same walk that proves a callee's pop
//! ([`crate::p4_calls::kuna_calleepurge`]): the highest byte at or above the
//! first argument slot that a load or store touches, through the stack pointer
//! or any register holding a stack address the walk placed exactly, rounded up
//! to a whole slot. A nested direct call whose own extent is known touches its
//! arguments in the callee's frame. It is stated only when the walk saw every
//! path and placed every access: an indirect jump, a budget overrun, an access
//! through an unplaced stack address (the frame pointer the caller left, one a
//! helper such as `__SEH_prolog4` set, a stack pointer moved by a call the walk
//! cannot read, an argument address indexed at run time), or an address at or
//! above the return address that is stored, returned or live in a register at
//! a call (`va_start`, `lea ecx,[esp+8]` for a by-value `this`) states nothing.
//! A pointer into the callee's locals, however it is indexed, is taken to stay
//! among them, and a callee the walk cannot read is taken to read only the
//! arguments pushed for it, unless it is one that moves its caller's stack.
//!
//! The bound a call site uses is the extent widened to what the callee's
//! returns pop (`ret 8` takes two slots whether or not both are read) and to
//! what the caller takes off the stack right after the call (`pop ecx`,
//! `add esp,8`: a `__cdecl` cleanup covers the arguments pushed for this call).
//! At the call site (`check_input_trial_use`), a stack trial starting at or
//! past the bound (or past what the callee pops, if that is more) is no
//! argument, and one within the touched extent skips the flow-insensitive
//! local-alias test. Neither applies in a function that calls a stack-moving
//! helper, whose stack offsets the solver can only guess. The first is a judgement:
//! an unread trailing parameter of a `__cdecl` callee whose cleanup the
//! compiler deferred past a later call is dropped with an outer call's pushes,
//! so it ships as an option.

use kuna_base::address::Address;

use crate::architecture::Architecture;

/// `option calleeargs on|off` (default on).
pub struct OptionCalleeArgs;

impl OptionCalleeArgs {
    /// The option name.
    pub const NAME: &'static str = "calleeargs";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::kuna_calleeargs`.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(bool, String)> {
        let val = crate::p0_knowledge::options::on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee stack-argument extent turned {prop}")))
    }
}

/// The stack-argument bytes the callee `entry` of the direct call at `call_at`
/// can take, when the option is on and the walk can account for every access.
pub fn callee_stack_arg_bytes(
    arch: &Architecture,
    entry: &Address,
    call_at: &Address,
) -> Option<crate::p4_calls::kuna_calleepurge::StackArgs> {
    if !arch.kuna_calleeargs {
        return None;
    }
    crate::p4_calls::kuna_calleepurge::callee_arg_bytes(arch, entry, call_at)
}

/// Does the direct callee `entry` move its caller's stack pointer? The solver
/// cannot follow such a call, so the stack offsets of every call in its caller
/// are guesses, and no extent is applied there.
pub fn callee_moves_stack(arch: &Architecture, entry: &Address) -> bool {
    arch.kuna_calleeargs && crate::p4_calls::kuna_calleepurge::moves_stack(arch, entry)
}

#[cfg(test)]
mod tests;
