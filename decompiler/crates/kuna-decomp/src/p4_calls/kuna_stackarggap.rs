//! (kuna) `stackarggap` — a stack argument cannot sit behind an argument
//! register the caller never wrote.
//!
//! # The symptom
//!
//! A Windows PE that hashes a buffer with OpenSSL calls `EVP_DigestFinal_ex`,
//! whose signature is `int EVP_DigestFinal_ex(EVP_MD_CTX *, unsigned char *,
//! unsigned int *)`. kuna renders the call with five arguments:
//!
//! ```text
//! v2 = EVP_DigestInit_ex(v3,EVP_sha256(),0,a3,v10);
//! ...
//! if (EVP_DigestFinal_ex(v3,v9,v7,a3,1) == 1) {
//! ```
//!
//! `a3` is the enclosing function's own fourth parameter, forwarded to nothing:
//! the disassembly never writes `R9` between the entry and either call. The
//! fifth argument is a leftover in the outgoing-argument area. The reader is
//! told the digest context takes two more operands than it does, and the two
//! invented ones are the reader's own incoming state, so they look load-bearing.
//!
//! # Where the two arguments come from
//!
//! The call site is
//!
//! ```text
//! 0x14000ba52   c744242001000000      MOV dword ptr [RSP + 0x20],0x1
//! 0x14000ba5a   4489742448            MOV dword ptr [RSP + 0x48],R14D
//! 0x14000ba5f   4c8d442448            LEA R8,[RSP + 0x48]
//! 0x14000ba64   488b17                MOV RDX,qword ptr [RDI]
//! 0x14000ba67   488bce                MOV RCX,RSI
//! 0x14000ba6a   ff15f88a0000          CALL qword ptr [0x140014568]
//! ```
//!
//! Three argument registers are loaded, `R9` is not, and `[RSP+0x20]` — the
//! Win64 fifth-argument slot — is left holding a `1` that belongs to the frame's
//! own bookkeeping. The trials `ParamListStandard::fillinMap` scores are
//!
//! ```text
//!   [1] register RCX   active
//!   [2] register RDX   active
//!   [3] register R8    active
//!   [4] register R9    inactive, UNREFERENCED   <- no Varnode: never written
//!   [5] stack    +0x20 active
//! ```
//!
//! `forceInactiveChain` (`fspec.cc:1519`) then runs its hole-filling tail loop.
//! It records the highest active trial — the stack slot at `[5]` — and marks
//! every inactive trial below it active, so that the recovered argument list has
//! no holes in it. That promotes `[4]`, and `buildInputFromTrials` materializes
//! the Varnode the trial never had by reading `R9` at the call, which resolves to
//! the caller's untouched incoming `R9`. Both invented arguments are that one
//! decision: the fifth is the stack slot the rule believed, the fourth is the
//! hole it filled to reach it.
//!
//! # Why the hole is not fillable here
//!
//! The hole-filling rule is right about registers. A caller can pass an argument
//! in `RDX` and leave `RCX` looking empty (it already held the value, or the
//! trial scoring lost it), and reading that hole as the end of the argument list
//! is how a call loses arguments it plainly passes — `calleedeadarg`'s record
//! names the same failure from the other side.
//!
//! It is not right across the register/stack boundary, and not because the
//! evidence is weak but because the ABI forbids the shape it is reconstructing.
//! Every `ParamListStandard` model assigns argument slots in resource order: the
//! stack is the last resource, reached only once the register file in front of it
//! is exhausted. A Win64 call has a fifth argument at `[RSP+0x20]` **only if** it
//! has a fourth in `R9`; an x86-64 SysV call spills to the stack only past
//! `R9`/`XMM7`. So an argument-register slot with no Varnode at all — nothing in
//! the caller ever wrote that register — is not a hole in the middle of an
//! argument list. It is the end of one, and whatever occupies the outgoing-
//! argument area past it is the frame's scratch, not an operand.
//!
//! Upstream already draws exactly this conclusion one slot to the right.
//! `forceInactiveChain` ends a call site's argument list at the first
//! *unreferenced stack* trial (`isUnref() && isRecoverSubcall() &&
//! isSpacebase()` sets `seenchain`), on the reasoning that a call-site stack slot
//! nothing wrote cannot be an argument. `stackarggap` is that same sentence about
//! the register slot immediately in front of the stack.
//!
//! # What this option changes
//!
//! With `stackarggap` on, `forceInactiveChain` ends a CALL SITE's argument list
//! at an UNREFERENCED argument-REGISTER trial whose next credible slot is on the
//! STACK. The witness becomes
//!
//! ```text
//! v2 = EVP_DigestInit_ex(v3,EVP_sha256(),0);
//! ...
//! if (EVP_DigestFinal_ex(v3,v9,v7) == 1) {
//! ```
//!
//! which is the OpenSSL prototype, arity and all, with no signature supplied.
//!
//! Four clauses keep the veto off everything else:
//!
//! 1. **Call sites only.** The exemption reads `ParamActive::isRecoverSubcall`,
//!    so the function's OWN input recovery — where an untouched argument register
//!    is an ignored parameter rather than the end of a list, which is the whole
//!    premise of `inputparamgap` — is untouched.
//! 2. **Unreferenced registers only.** An INACTIVE register trial still has a
//!    Varnode: the caller put something there, and trial scoring merely could not
//!    prove it was for the callee. That is the ambiguous case the hole-filling
//!    rule exists for, and it still fills. Only a trial `buildTrialMap`
//!    synthesized for a `ParamEntry` with no Varnode at all — the caller never
//!    wrote the register — ends the list. A wrapper that forwards its own fourth
//!    parameter is referenced and unaffected.
//! 3. **Only at the register/stack boundary.** The veto fires only when the next
//!    non-eliminated trial in the section is a stack slot, so a hole in the
//!    middle of the register file with live registers behind it keeps upstream's
//!    fill. The ABI argument is about the boundary and the rule is too.
//! 4. **Nothing new becomes an argument.** This only stops trials from being
//!    marked active; it never marks one. An argument list can lose its invented
//!    tail, never gain a member.
//!
//! A variadic call site reaches none of this: `varargstackargs` has already split
//! its stack tail into its own `fillinMap` section, so the register prefix and
//! the stack slots are never scored against each other and clause 3 cannot hold.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use crate::fspec::ParamActive;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<stackarggap>` (kuna 4000+ range; 4156 was the previous
/// high-water mark).
pub const ELEM_STACKARGGAP: ElementId = ElementId::new("stackarggap", 4157);

/// The `stackarggap` option surface.
pub struct OptionStackArgGap;

impl OptionStackArgGap {
    /// The option name.
    pub const NAME: &'static str = "stackarggap";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::stack_arg_gap`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Call-site stack-argument gap veto turned {prop}")))
    }
}

/// Does the inactive trial at `index` end this call site's argument list?
///
/// True only when all four narrowing clauses hold: the option is on and `active`
/// belongs to a call site's recovery, the trial has no Varnode (`is_unref`), it
/// is an argument REGISTER, and the next trial in `[index+1, stop)` that has not
/// already been eliminated lives in a stack (spacebase) space. `is_spacebase`
/// reports, per trial index, whether that trial's address is in a spacebase — the
/// caller supplies it so this stays a pure read of `ParamActive`.
pub fn ends_argument_list(
    active: &ParamActive,
    index: int4,
    stop: int4,
    trial_is_spacebase: impl Fn(int4) -> bool,
) -> bool {
    if !active.is_stack_arg_gap() || !active.is_recover_subcall() {
        return false;
    }
    let trial = active.get_trial(index);
    if !trial.is_unref() || trial.is_definitely_not_used() || trial_is_spacebase(index) {
        return false;
    }
    for i in (index + 1)..stop {
        let next = active.get_trial(i);
        if next.is_definitely_not_used() || next.get_entry().is_none() {
            continue;
        }
        return trial_is_spacebase(i);
    }
    false
}

/// Is `addr`'s space a stack (spacebase) space?  The one shape test the veto
/// needs, kept here so `force_inactive_chain` reads the same rule this module
/// documents.
pub fn address_is_spacebase(addr: &kuna_base::address::Address) -> bool {
    addr.get_space().map(|s| s.get_type() == spacetype::IPTR_SPACEBASE).unwrap_or(false)
}

#[cfg(test)]
#[path = "kuna_stackarggap/tests.rs"]
mod tests;
