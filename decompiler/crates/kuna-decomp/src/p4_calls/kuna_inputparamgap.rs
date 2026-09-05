//! (kuna) `inputparamgap` — a run of unused argument registers must not veto a
//! LATER register the function's own body reads before it writes.
//!
//! # The symptom
//!
//! A Wayland `wl_keyboard_listener` key handler, decompiled by kuna, reads two
//! values it never defines:
//!
//! ```text
//! void sub_6500(long a0)
//! {
//!   ...
//!   int v7; // r8d
//!   int v8; // r9d
//!
//!   if (v8 != 1 || !*(long *)(a0 + 0x68))
//!     return;
//!   v2 = xkb_state_key_get_one_sym(*(long *)(a0 + 0x68),v7 + 8);
//! ```
//!
//! `v7` and `v8` are declared and then read with nothing ever assigning them.
//! The disassembly says where they come from: the function opens `cmp $0x1,%r9d`
//! and later `lea 0x8(%r8),%edi`, so both are incoming argument registers of the
//! callback's ABI signature
//! `key(void *data, struct wl_keyboard *, uint32_t serial, uint32_t time,
//! uint32_t key, uint32_t state)` — `r8d` is `key`, `r9d` is `state`. An agent
//! reading kuna's output cannot tell that, and the C does not recompile.
//!
//! # Why the incoming registers are dropped
//!
//! Not for want of evidence, and not because the callback is unprototyped. The
//! function's own input recovery (`ActionInputPrototype`, `coreaction.cc:4941`)
//! registers a trial for every input Varnode the heritage produced and marks it
//! ACTIVE when it has descendants — i.e. when the body genuinely reads the
//! register before writing it. `rdi`, `r8d` and `r9d` all reach
//! `ParamListStandard::fillinMap` as active trials, and `buildTrialMap` even
//! synthesizes unreferenced filler trials for the three registers the callback
//! never touches (`rsi`, `rdx`, `rcx`).
//!
//! They are then thrown away by `forceInactiveChain` (`fspec.cc:1519`), called
//! with `maxchain = 2`. That rule walks the section counting consecutive unused
//! parameter slots, and once the run exceeds two it sets `seenchain` and marks
//! every remaining trial inactive — including trials that already scored active.
//! `fillinMap` then only marks the still-active trials used, so `r8d`/`r9d`
//! never become parameters and are left to render as undefined locals.
//!
//! The witness's gap is exactly three registers wide, one past the limit. That
//! the limit, and not the missing prototype, is the whole story is visible in a
//! 30-line fixture: a callback with the same shape but a TWO-register hole
//! (`rdi` and `rcx` read, `rsi`/`rdx` dead) recovers cleanly today, filler
//! parameters and all —
//!
//! ```text
//! void on_two(long a0,unsigned long a1,unsigned long a2,int a3)   // 2-slot gap: recovered
//! void on_key(long a0) { int v1; // r8d ... }                     // 3-slot gap: dropped
//! ```
//!
//! Both functions are reached only through a function-pointer table and neither
//! has a prototype, so prototype evidence cannot be what separates them.
//!
//! # Why the chain rule is right at a call site and wrong here
//!
//! `forceInactiveChain` is shared between two very different questions, and its
//! premise only holds for one of them.
//!
//! At a CALL SITE, `checkInputTrialUse` scores a trial active from the CALLER's
//! data flow: the argument register holds a value the caller wrote and does not
//! otherwise use. That is genuinely ambiguous — a register can hold a live value
//! across a call for reasons that have nothing to do with the callee — so a long
//! run of empty slots really is evidence that the recovery has walked past the
//! end of the argument list, and the rule keeps it from fabricating six
//! arguments out of incidental liveness. `calleedeadarg`'s record names the same
//! rule from the other side: punching a hole in a call's register argument list
//! makes `fillinMap` read the hole as the end of the list.
//!
//! For the function's OWN inputs the evidence is not a heuristic. An active
//! trial there means the function reads that register on some path before any
//! definition of it, which on a caller-saved argument register has exactly one
//! explanation: the value arrived in it. The gap slots carry no counter-evidence
//! at all — an unused argument register is what an ignored parameter looks like,
//! and a callback whose signature is fixed by the API it is registered with
//! ignores parameters as a matter of course. So the run-length veto is trading a
//! fact for a heuristic, and the callbacks that most need a signature (they have
//! no call site anywhere in the image) are exactly the ones it fires on.
//!
//! # What this option changes
//!
//! With `inputparamgap` on, a chain of unused REGISTER slots inside the
//! function's OWN input recovery no longer sets `seenchain`, so an active trial
//! after the gap survives and `forceInactiveChain`'s existing tail loop fills the
//! hole with the filler parameters it already synthesized. The witness becomes
//!
//! ```text
//! void sub_6500(long a0,unsigned long a1,unsigned long a2,unsigned long a3,int a4,int a5)
//!   if (a5 != 1 || !*(long *)(a0 + 0x68)) return;
//!   v2 = xkb_state_key_get_one_sym(*(long *)(a0 + 0x68),a4 + 8);
//! ```
//!
//! which is the Wayland prototype, positions and all.
//!
//! Three narrowing clauses keep the relaxation off everything else:
//!
//! 1. **The function's own input recovery only.** The exemption is carried on the
//!    [`ParamActive`](crate::fspec::ParamActive) that `ActionInputPrototype`
//!    builds, and nothing sets it on a call site's trials. Every call in every
//!    binary scores exactly as before.
//! 2. **It protects REGISTER trials only.** The evidence this option rests on —
//!    read-before-written — is a register's. A caller-saved argument register the
//!    body reads live-in can only be carrying a value the caller placed there; a
//!    positive-offset stack slot read live-in is much weaker, since the Win64
//!    home area is routinely used as scratch and an over-wide or aliased read
//!    looks the same. So a stack trial's fate is left exactly as upstream decides
//!    it, `seenchain` and all. This was measured, not assumed: an earlier draft
//!    that suppressed the veto for the whole section let one Win64
//!    `sub_140010a57` span its four-register hole and promote eleven scratch
//!    slots of the caller's argument area into a fifteen-parameter signature.
//!    Because trials sort into formal parameter order, protecting only register
//!    trials also keeps the existing hole-filling tail loop inside the register
//!    file, so the reach of this option is bounded by the ABI — six parameters on
//!    x86-64 SysV, four on Win64.
//! 3. **No new way to score a trial active.** This never marks a trial active
//!    that was not already active; it only stops an unused run from cancelling
//!    one. A register the body does not read before writing is still not a
//!    parameter.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::fspec::{ParamActive, ParamEntry, ParamTrial};
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<inputparamgap>` (kuna 4000+ range; 4140 was the previous
/// high-water mark).
pub const ELEM_INPUTPARAMGAP: ElementId = ElementId::new("inputparamgap", 4141);

/// The `inputparamgap` option surface.
pub struct OptionInputParamGap;

impl OptionInputParamGap {
    /// The option name.
    pub const NAME: &'static str = "inputparamgap";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::input_param_gap`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Own-input register-gap tolerance turned {prop}")))
    }
}

/// Is `trial` a REGISTER input that `forceInactiveChain` must not demote?
///
/// True only when all three narrowing clauses hold: `active` belongs to the
/// function's own input recovery with the option on
/// ([`ParamActive::is_own_input_gap`]), the trial is already ACTIVE (this never
/// makes one active), and its `ParamEntry` is an exclusion — i.e. a register —
/// group. A stack trial, an entry-less trial, an inactive trial, or any call
/// site answers false and takes the upstream veto unchanged.
pub fn trial_is_protected(
    active: &ParamActive,
    trial: &ParamTrial,
    entries: &[ParamEntry],
) -> bool {
    if !active.is_own_input_gap() || !trial.is_active() {
        return false;
    }
    match trial.get_entry() {
        Some(e) => entries[e].is_exclusion(),
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_inputparamgap/tests.rs"]
mod tests;
