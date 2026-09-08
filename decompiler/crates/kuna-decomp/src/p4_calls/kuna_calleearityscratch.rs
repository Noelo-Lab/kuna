//! (kuna) `calleearityscratch` — let a boundary register the caller only used as
//! SCRATCH end a [`calleearitycut`](crate::p4_calls::kuna_calleearitycut) run.
//!
//! # The gap
//!
//! `calleearitycut` bounds a callee-body argument run by the register the run
//! stops at, and requires that register to be QUIET: the caller must have placed
//! nothing in it.  The reason is stated in that module and it is a real one — a
//! caller that computes a value into the next argument register right before the
//! CALL is passing a further argument whatever the callee decode can see, and
//! truncating there would hide it.
//!
//! But "the caller wrote it" and "the caller is passing it" are not the same
//! claim, and a register allocator will happily spend an argument register on a
//! throwaway.  An x86-64 ELF crackme decodes a string with an unrolled XOR loop
//! and then hands the buffer to its printer:
//!
//! ```text
//!   0x400f58  lea   rdi,[rbp-0x30]        ; the buffer -- the argument
//!   ...
//!   0x400fc9  movzx esi,byte ptr [rbp-0x1a]  ; the XOR key, in an argument register
//!   0x400fcd  xor   byte ptr [r8+0x6],sil    ; ...and the caller consumes it here
//!   ...
//!   0x401007  call  0x417f80
//! ```
//!
//! `rdi` is live into the CALL and the callee reads it at its first instruction,
//! so `calleearitybody` picks the run `{rdi}`; the callee's decode is cut three
//! instructions later at a nested call, so nothing proves `rsi` dead and
//! `calleearitycut` is asked for the bound.  `rsi` is the register the run stops
//! at, the caller wrote it, and the run is refused — the whole one-argument list
//! is dropped and the call renders `sub_417f80();`.
//!
//! # What tells scratch from an argument
//!
//! Argument setup is a value the caller computes and then uses for NOTHING but
//! the call.  That is `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`), and
//! upstream has already asked it: `FuncCallSpecs::checkInputTrialUse` runs
//! `ancestorOpUse` over every trial and marks the trial ACTIVE when the answer is
//! yes, INACTIVE when the value has some other use in the caller.  Here the XOR
//! at `0x400fcd` is that other use and `rsi`'s trial scores inactive.
//!
//! So the boundary is quiet enough when the caller's own trial scoring already
//! says the register is not carrying a parameter.  Two restrictions keep that
//! narrow:
//!
//! * **The trial must have been checked and scored inactive.**  An unchecked
//!   trial has no verdict to borrow, and an ACTIVE one is upstream saying the
//!   value reaches the call and nothing else — exactly the further argument
//!   `calleearitycut` refuses to hide.
//! * **A constant is never scratch.**  A caller that materializes a constant into
//!   the next argument register right before the CALL is setting up an argument
//!   in the most literal way there is, and it stays refused however the trial
//!   scored.
//!
//! The reachable error is unchanged from `calleearitycut`'s: the run is still
//! required to be contiguous and to stop short, so the worst outcome is a MISSING
//! trailing argument, never one printed in the wrong position — and the site this
//! rule fires on rendered no arguments at all, so it trades an empty list for a
//! prefix the callee's own body proves it reads.
//!
//! Inert unless `calleearitycut` is also on: it widens that rule's boundary test
//! rather than adding a second rule.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleearitybody::BodyTrial;

/// Marshaling element `<calleearityscratch>` (kuna 4000+ range; 4155 = indirectanchor).
pub const ELEM_CALLEEARITYSCRATCH: ElementId = ElementId::new("calleearityscratch", 4156);

/// (kuna) Let a caller-scratch register bound a cut callee-body argument run:
/// `calleearityscratch on|off`.
pub struct OptionCalleeArityScratch;

impl OptionCalleeArityScratch {
    /// The option name.
    pub const NAME: &'static str = "calleearityscratch";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity_scratch`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Caller-scratch bound for a cut callee-body run turned {prop}")))
    }
}

/// Is the register this run stops at one the caller only used as scratch?
///
/// `bound` is the trial immediately past the run, exactly as
/// [`crate::p4_calls::kuna_calleearitycut::accepts_cut_run`] found it.
pub fn boundary_is_caller_scratch(bound: &BodyTrial) -> bool {
    bound.caller_checked && !bound.caller_active && !bound.caller_constant
}

#[cfg(test)]
#[path = "kuna_calleearityscratch/tests.rs"]
mod tests;
