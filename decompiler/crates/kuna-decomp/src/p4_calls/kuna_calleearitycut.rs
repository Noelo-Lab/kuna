//! (kuna) `calleearitycut` — accept a lone call's callee-body argument run when
//! the decode was CUT before it could prove a dead boundary register.
//!
//! # The gap
//!
//! [`calleearitybody`](crate::p4_calls::kuna_calleearitybody) recovers the
//! argument list of a call that has no sibling from the callee's own body: the
//! list is the leading run of argument registers the callee `proves_input`, and
//! it must END at an argument register the callee `proves_dead`.  That second
//! condition is what bounds an existential, and it is unavailable far more often
//! than the first, because [`probe_callee_entry_dead`] ends every path at the
//! callee's first nested call.  A register the callee only clobbers *after* that
//! call is invisible to the walk, so a perfectly ordinary body proves an input
//! run and no boundary at all:
//!
//! ```text
//!   0x9d9cd  mov  ebx,ecx                  ; the caller's own 4th argument
//!   0x9d9cf  mov  ecx,0x1                  ; the callee's 4th argument
//!   0x9d9d8  call 0x875e0                  ; rdi/rsi/rdx are the caller's own, untouched
//!
//!   0x875e0  ...
//!   0x87601  lea  rax,[rsi+rdx]            ; reads rsi, rdx
//!   0x8760f  mov  eax,ecx                  ; reads rcx
//!   0x87613  add  word ptr [rdi+0x30],0x1  ; reads rdi
//!   0x8769f  call 0x26d50                  ; every path is cut here
//!   ...
//!   0x876d2  mov  r8,qword ptr [rax+0x18]  ; r8 is clobbered only past the cut
//! ```
//!
//! Four registers `proves_input`, `r8` and `r9` prove nothing either way, and
//! `calleearitybody` declines.  The call renders `sub_875e0();` while decompiling
//! that same callee on its own yields `sub_875e0(int8,char *,int8,uint4)`.
//!
//! The caller side cannot rescue it either, and for a reason that is upstream
//! policy rather than a gap: `rdi`, `rsi` and `rdx` are the caller's OWN incoming
//! parameters passed straight through, and `AncestorRealistic::execute`
//! (`funcdata_varnode.cc:2159`) refuses an input Varnode outright — *"if the
//! parameter itself is an input, we don't consider this realistic, we expect to
//! see active movement into the parameter"*.  Only `rcx` is written at the call
//! site, it lands fourth, and `ParamListStandard::fillinMap`'s positional rules
//! read the three-register hole in front of it as the end of the list — so the
//! site recovers nothing at all.
//!
//! # What bounds the run instead
//!
//! A boundary the callee proves dead is not the only way to know a run is the
//! whole list; it is the only way to know it when the run reaches the LAST
//! argument register.  When the run stops short, the register it stops at is
//! itself the boundary, and what has to be ruled out is that the register
//! carries an argument the walk simply could not see.  Three conditions, and the
//! rule is only as safe as their conjunction:
//!
//! * **The run is contiguous.**  `calleearitybody` skips a non-input register
//!   inside the run and keeps going, which can collapse a hole and print a later
//!   argument in an earlier one's position.  Here every picked trial must be
//!   adjacent, so each emitted argument sits at the position the ABI assigns it
//!   and the only reachable error is a MISSING trailing argument — never a
//!   misplaced one.
//! * **The run stops short.**  At least one register argument location must
//!   follow it, unclaimed.  A run that consumes every argument register has no
//!   boundary and is refused, which is the AArch64 variadic register-save
//!   prologue (`str x3,[sp,#136]; stp x4,x5,[sp,#144]; stp x6,x7,[sp,#160]`)
//!   `calleearitybody`'s own corpus sweep found — it reads every argument
//!   register there is.
//! * **The boundary is quiet.**  The caller must not have WRITTEN the register
//!   the run stops at.  A caller that computes a value into it right before the
//!   CALL is passing a further argument whatever the callee decode can see, and
//!   truncating there would hide it; a register holding only the caller's own
//!   untouched incoming value, or nothing at all, carries no such claim.
//!
//! `calleearitybody`'s own guards are unchanged and still apply first: no
//! unclaimed argument location may `proves_input`, the summary must be complete,
//! and only a call that recovered nothing at all is eligible.  This rule adds
//! one alternative to the dead-boundary test and nothing else.
//!
//! Inert unless `calleearitybody` is also on: it widens that rule rather than
//! adding a second one.
//!
//! [`probe_callee_entry_dead`]: crate::p4_calls::kuna_calleedeadarg::probe_callee_entry_dead

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleearitybody::{is_register, BodyTrial};

/// Marshaling element `<calleearitycut>` (kuna 4000+ range; 4153 = calleeretpreserves).
pub const ELEM_CALLEEARITYCUT: ElementId = ElementId::new("calleearitycut", 4154);

/// (kuna) Accept a callee-body argument run bounded by a cut decode:
/// `calleearitycut on|off`.
pub struct OptionCalleeArityCut;

impl OptionCalleeArityCut {
    /// The option name.
    pub const NAME: &'static str = "calleearitycut";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity_cut`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Cut-bounded callee-body argument run turned {prop}")))
    }
}

/// May a run the callee's body proves is an input stand as the whole argument
/// list, although no unclaimed argument location proves dead?
///
/// `picked` are indices into `trials`, in prototype order, and `entries` are the
/// prototype model's own input locations — both exactly as
/// [`crate::p4_calls::kuna_calleearitybody::plan_from_body`] built them.
pub fn accepts_cut_run(
    trials: &[BodyTrial],
    picked: &[int4],
    entries: &[(Address, int4)],
) -> bool {
    let (Some(&first), Some(&last)) = (picked.first(), picked.last()) else { return false };
    if (last - first + 1) as usize != picked.len() {
        return false; // a hole inside the run would move a later argument forward
    }
    let Some(bound) = trials.get(last as usize + 1) else { return false };
    if !is_register(&bound.addr) || !bound.caller_quiet {
        return false;
    }
    // The boundary must be an argument location the model actually names: a
    // register trial past the model's own list bounds nothing.
    entries.iter().any(|(a, sz)| a == &bound.addr && *sz == bound.size)
}

#[cfg(test)]
#[path = "kuna_calleearitycut/tests.rs"]
mod tests;
