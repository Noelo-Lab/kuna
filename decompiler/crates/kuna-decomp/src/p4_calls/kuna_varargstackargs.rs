//! (kuna) `varargstackargs` — the stack resource is its own section at a
//! variadic call site.
//!
//! # The gap
//!
//! `ParamListStandard::fillinMap` (`fspec.cc:1560`) scores a call's recovered
//! input trials one *resource section* at a time.  Within a section the trials
//! are positional, so two rules police the gaps between them:
//!
//! * `forceNoUse` — once a whole exclusion group in the section is
//!   *definitely not used*, everything after it in that section is inactive.
//! * `forceInactiveChain` — a run of more than `maxchain` (2) consecutive
//!   unused slots ends the section: nothing after the run is a parameter.
//!
//! Both encode the same assumption: parameters fill the resource in order, so a
//! long hole means the recovery has run off the end of the real argument list.
//!
//! On AArch64 the general-purpose registers `x0`–`x7` and the outgoing stack
//! area are ONE section (`AARCH64.cspec`/`AARCH64_apple.cspec` open a new
//! resource only for the hidden-return and floating-point pentries), and that
//! assumption is exactly wrong for a **variadic** call on an ABI that passes the
//! variable arguments on the stack.  Apple's arm64 ABI does: clang emits
//!
//! ```text
//!   add  x8, sp, #0x14        ; &passcode
//!   mov  x9, sp
//!   str  x8, [x9]             ; the variadic argument, at [sp+0]
//!   adrp x0, "%d"             ; the one fixed parameter
//!   bl   _scanf
//! ```
//!
//! so `x1`–`x7` are structurally empty between the last fixed register
//! parameter and the first stack argument.  Seven empty slots is a longer hole
//! than either rule tolerates, so the stack trial — already scored ACTIVE by
//! `checkInputTrialUse`, having passed ancestor-realism and the local-alias
//! test — is forced inactive again, and the call renders as `scanf("%d")` with
//! its destination dropped.  `forceInactiveChain`'s trailing "force the gaps
//! below the last active slot active" loop is why the hole cannot simply be
//! ignored in place: reactivating the stack trial where it sits would also
//! fabricate `x1`–`x7` as six invented register arguments.
//!
//! # The mechanism
//!
//! Split the section instead.  When the call's prototype is variadic, the
//! register prefix and the stack tail of a section are scored independently:
//! neither rule can carry a verdict across the boundary, the register gap stops
//! being evidence about the stack argument, and the gap-filling loop stays
//! inside the half that produced its `max`.
//!
//! Nothing else about trial scoring changes.  A stack trial still has to be
//! ACTIVE on its own evidence to survive — `checkInputTrialUse`'s local-alias
//! and caller-local-range tests and `ancestorOpUse` all run first, unchanged —
//! so the option can only ever keep an argument the recovery had already
//! believed in, never invent one from an untouched slot.
//!
//! The variadic requirement is what keeps the divergence honest: for a call
//! with a fully known prototype a hole in the register file IS evidence, and a
//! non-variadic callee's arguments really do fill the resource in order.  Only
//! `...` makes the hole a property of the ABI rather than of the recovery.
//!
//! # The second condition: the stack area must start at the stack pointer
//!
//! The whole-corpus sweep found the counter-example that shapes the rest of the
//! rule.  MIPS o32 PIC saves `$gp` across calls with `sw gp,16(sp)` /
//! `lw gp,16(s8)`, and `mips32be.cspec` puts the first stack parameter at
//! **offset 16** -- exactly that slot.  Discounting the `a1`-`a3` hole there
//! turns the caller's own register save into a second argument to a `printf`
//! whose format string has no conversion at all.
//!
//! So the cut also requires the section's stack area to begin at callee-relative
//! offset **0**.  Everything an ABI puts between the outgoing stack pointer and
//! its first stack argument says the stack area sits *behind* the register file:
//! MIPS o32's 16 bytes are, in that cspec's own comment, "backup storage space
//! for register params"; x86-64's 8 and i386's 4 are the pushed return address;
//! Win64's 32 are the home area.  An ABI whose first stack argument is the word
//! at `[sp]` reserves nothing in front of it, which is what lets Apple's arm64
//! ABI hand the entire variadic tail to it.
//!
//! That leaves ARM32 AAPCS (stack pentry at 0, but variadic arguments do fill
//! `r0`-`r3` first) as the residual over-recovery risk; it did not fire anywhere
//! in the sweep, and it is why the option is opt-in rather than a default.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use crate::fspec::ParamActive;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<varargstackargs>` (kuna 4000+ range; 4131 = cleanupcode).
pub const ELEM_VARARGSTACKARGS: ElementId = ElementId::new("varargstackargs", 4132);

/// (kuna) Score a variadic call's stack arguments as their own resource
/// section: `varargstackargs on|off`.
pub struct OptionVarargStackArgs;

impl OptionVarargStackArgs {
    /// The option name.
    pub const NAME: &'static str = "varargstackargs";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::vararg_stack_args`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Variadic stack-argument sectioning turned {prop}")))
    }
}

/// Where a `fillinMap` resource section should be cut in two, or `None` to
/// score it whole.
///
/// The cut is the first stack (spacebase) trial of the section, subject to three
/// conditions:
///
/// * the section opens with at least one register trial — a section that is
///   entirely stack (i386 `__cdecl`, where every argument including the fixed
///   ones is already on the stack) has no boundary to draw and no register hole
///   to discount;
/// * that trial sits at callee-relative offset 0, so the ABI reserves neither a
///   return-address slot nor a register home area in front of its stack
///   arguments (see the module header);
/// * it carries a `ParamEntry`, because it becomes the new section's
///   `groupstart` through `ParamTrial::slot_group`.
pub fn stack_section_split(active: &ParamActive, start: int4, stop: int4) -> Option<int4> {
    if !active.is_vararg_stack_split() {
        return None;
    }
    let mut seen_register = false;
    for i in start..stop {
        let trial = active.get_trial(i);
        let on_stack = trial
            .get_address()
            .get_space()
            .map(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
            .unwrap_or(false);
        if !on_stack {
            seen_register = true;
            continue;
        }
        if !seen_register
            || trial.get_entry().is_none()
            || trial.get_address().get_offset() != 0
        {
            return None;
        }
        return Some(i);
    }
    None
}

#[cfg(test)]
#[path = "kuna_varargstackargs/tests.rs"]
mod tests;
