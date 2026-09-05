//! (kuna) `calleearityfwd` — reconcile a call's argument list against a sibling
//! that finalizes *later*.
//!
//! # The gap
//!
//! [`calleearity`](crate::p4_calls::kuna_calleearity) reconciles a call site
//! that recovered nothing against another call to the same callee whose argument
//! list is already final, and its own documentation names the case it cannot
//! reach: `ActionActiveParam` finalizes each call spec the moment that spec is
//! fully checked, in `qlst` order, so a site is only ever reconciled against the
//! sites *before* it.  When the site that lost its argument is the one that
//! finalizes first, the witness does not exist yet and the rescue declines.
//!
//! That is not a corner case — it is the MSVC aligned-`operator new` idiom, both
//! halves of which call the same allocator:
//!
//! ```text
//!   cmp  rcx,0x1000
//!   jc   small                 ; rcx = the requested size
//!   lea  rax,[rcx+0x27]        ; large: the over-aligned size
//!   cmp  rax,rcx
//!   jbe  __std_abort           ; the overflow guard
//!   mov  rcx,rax
//!   call operator_new          ; <- rcx written here; recovered
//! small:
//!   test rcx,rcx
//!   jz   nothing
//!   call operator_new          ; <- rcx live-in; `onlyOpUse` sees the CBRANCH
//! ```
//!
//! The small arm's `rcx` is never rewritten before the call, so the Varnode
//! reaching it is the same one the `test` branches on and `Funcdata::onlyOpUse`
//! (`funcdata_varnode.cc:1851`) marks the trial inactive.  The large arm gets a
//! fresh `rcx` from the `mov` and keeps its argument.  Measured on
//! `trappy attack.exe` @0x1400011c0: the losing site 0x1400016f7 finalizes with
//! an empty witness list while its sibling 0x1400016d1 — which recovers exactly
//! `RCX` — is still `input_active` and has not been finalized yet.  The result is
//! `sub_140002c90(v11 + 0x27)` on one branch and `sub_140002c90()` on the other.
//!
//! # The mechanism
//!
//! Symmetry, without moving anything.  Reordering the finalization would be the
//! obvious fix and it is the wrong one: `Funcdata::checkCallDoubleUse`
//! (`funcdata_varnode.cc:2121`) reads whether *another* call spec is still
//! `input_active` while scoring a trial, so deferring a spec's finalization past
//! its neighbours' `checkInputTrialUse` silently changes argument recovery
//! everywhere, on every binary.
//!
//! Instead the call site that finalized with an **empty** argument list is set
//! aside — with the Varnodes its promotable trials pointed at, captured before
//! `opSetAllInput` drops them — and retried once, at the end of the same
//! `ActionActiveParam::apply`, when every spec in the pass is final.  The witness
//! search, the promotion plan and every one of its refusals are
//! [`calleearity`](crate::p4_calls::kuna_calleearity)'s, unchanged: register
//! storage only, real Varnodes only, all-or-nothing, never subtractive, and only
//! a call that recovered nothing at all.  This option adds no new way to promote
//! a trial; it only lets the existing one see the sites that come after.
//!
//! Two further limits are this module's own:
//!
//! * **The captured Varnode must still be the trial's own width.**  The normal
//!   path inserts a truncating `SUBPIECE` for an oversized Varnode; the retry
//!   runs after the trials are gone, so rather than rebuild that it declines.
//! * **Nothing crosses an `apply`.**  A rescue candidate is retried only within
//!   the pass that produced it, because its trials — and the slot numbering the
//!   captured Varnodes came from — do not survive `deleteUnusedTrials`.
//!
//! Inert unless both `calleearity` and `calleearityfwd` are on: this completes
//! that rule rather than adding a second one, so `calleearity off` still turns
//! all sibling reconciliation off.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleearity::{best_witness_for, plan_promotions, TrialView};

/// Marshaling element `<calleearityfwd>` (kuna 4000+ range; 4133 = calleearity).
pub const ELEM_CALLEEARITYFWD: ElementId = ElementId::new("calleearityfwd", 4137);

/// (kuna) Retry the `calleearity` reconciliation against sibling calls that
/// finalize later in the same pass: `calleearityfwd on|off`.
pub struct OptionCalleeArityFwd;

impl OptionCalleeArityFwd {
    /// The option name.
    pub const NAME: &'static str = "calleearityfwd";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity_fwd`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Forward call-site argument-list reconciliation turned {prop}")))
    }
}

/// A call that finalized with no arguments, held until every spec in the pass is
/// final.
///
/// `candidates` are the trials this site could still have promoted, in prototype
/// order, each paired with the Varnode the CALL carried for it — read before
/// `opSetAllInput` removes it, which is the only moment it is reachable.
pub struct PendingRescue {
    op: OpId,
    entry: Address,
    candidates: Vec<(Address, int4, VarnodeId)>,
}

/// Capture a call site that is about to render with an empty argument list.
///
/// Called from `build_input_from_trials` with the trials still intact and the
/// CALL op's pre-rewrite inputs still attached.  `None` whenever the retry could
/// not apply anyway: option off, locked prototype, not a live direct CALL, some
/// argument already recovered, or no promotable register trial left.
pub fn capture_empty_call(fc: &FuncCallSpecs, data: &Funcdata) -> Option<PendingRescue> {
    let arch = data.get_arch();
    if !arch.callee_arity || !arch.callee_arity_fwd || fc.is_input_locked() {
        return None;
    }
    let op = fc.get_op();
    let o = data.obank().get(op)?;
    if o.is_dead() || o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let active = fc.active_input();
    let mut candidates: Vec<(Address, int4, VarnodeId)> = Vec::new();
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if t.is_used() {
            return None; // this site recovered something; not ours to speak to
        }
        if t.is_definitely_not_used() || t.is_unref() {
            continue;
        }
        let addr = t.get_address();
        let is_stack = addr
            .get_space()
            .map(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
            .unwrap_or(true);
        if is_stack {
            continue;
        }
        let slot = t.get_slot();
        if slot < 1 {
            continue;
        }
        let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { continue };
        // The normal path would insert a truncating SUBPIECE here; the retry runs
        // after the trials are gone, so an oversized Varnode is declined instead.
        if data.vbank().get(vn).map(|v| v.get_size()) != Some(t.get_size()) {
            continue;
        }
        candidates.push((addr.clone(), t.get_size(), vn));
    }
    if candidates.is_empty() {
        return None;
    }
    Some(PendingRescue { op, entry: fc.get_entry_address().clone(), candidates })
}

/// Retry every captured call site now that the pass has finalized all of them.
///
/// Applied in capture order, so a site rescued here can itself witness a later
/// one — the same chaining the in-order rule already has.
pub fn rescue_pending(data: &mut Funcdata, pending: &[PendingRescue]) -> int4 {
    let mut rescued = 0;
    for p in pending {
        if rescue_one(data, p) {
            rescued += 1;
        }
    }
    rescued
}

/// Rewrite one captured call's input list from a now-final sibling, or leave it
/// alone.  Returns whether the call gained arguments.
fn rescue_one(data: &mut Funcdata, p: &PendingRescue) -> bool {
    match data.obank().get(p.op) {
        Some(o) if !o.is_dead() && o.code() == OpCode::CPUI_CALL && o.num_input() == 1 => {}
        _ => return false,
    }
    let witness = best_witness_for(&p.entry, p.op, data);
    if witness.is_empty() {
        return false;
    }
    // Every captured candidate is by construction live, sized and unpromoted, so
    // the shared plan only has to answer whether the witness is covered.
    let views: Vec<TrialView> = p
        .candidates
        .iter()
        .map(|(addr, size, _)| TrialView {
            addr: addr.clone(),
            size: *size,
            used: false,
            definitely_not_used: false,
            unref: false,
            has_value: true,
        })
        .collect();
    let Some(mut promote) = plan_promotions(&witness, &views) else { return false };
    promote.sort_unstable();
    let Some(in0) = data.obank().get(p.op).and_then(|o| o.get_in(0)) else { return false };
    let mut newparam = vec![in0];
    for i in &promote {
        let (_, _, vn) = p.candidates[*i as usize];
        if data.vbank().get(vn).is_none() {
            return false;
        }
        newparam.push(vn);
    }
    if newparam.len() < 2 || data.op_set_all_input(p.op, &newparam).is_err() {
        return false;
    }
    // Record where the arguments lived so a later rescue can witness this site.
    if let Some(idx) = data.get_call_specs_index(p.op) {
        let storage: Vec<(Address, int4)> = promote
            .iter()
            .map(|i| {
                let (addr, size, _) = &p.candidates[*i as usize];
                (addr.clone(), *size)
            })
            .collect();
        data.get_call_specs_mut(idx).set_final_input_storage(storage);
    }
    true
}

#[cfg(test)]
#[path = "kuna_calleearityfwd/tests.rs"]
mod tests;
