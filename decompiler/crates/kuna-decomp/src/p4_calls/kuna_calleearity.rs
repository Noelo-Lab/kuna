//! (kuna) `calleearity` — one callee, one argument list.
//!
//! # The gap
//!
//! When a callee's prototype is unlocked, kuna recovers the argument list
//! **independently at every call site** (`ActionActiveParam` ->
//! `checkInputTrialUse` -> `ParamListStandard::fillinMap`).  Nothing reconciles
//! the results, so one function can be called four times in one caller and
//! render with four different arities:
//!
//! ```text
//!   v13 = sub_140008160(0x28);          // 0x14000252c
//!   v13 = sub_140008160();              // 0x1400026b0   <-- the size is gone
//!   v39 = (char *)sub_140008160(v33);   // 0x1400026cf
//!   v16 = (long long *)sub_140008160(0x28);
//! ```
//!
//! The odd one out is MSVC's aligned `operator new[]`, where the requested size
//! is also the operand of an overflow check:
//!
//! ```text
//!   lea  rcx, [rbx+0x27]
//!   cmp  rcx, rbx
//!   jbe  __std_abort            ; the overflow guard
//!   call sub_140008160
//! ```
//!
//! `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`) asks whether the Varnode
//! reaching the CALL in `rcx` is used *only* as this call's argument, and a
//! `CPUI_CBRANCH` among its descendants answers no — so `checkInputTrialUse`
//! marks the trial inactive and the size disappears from the call.  The rule is
//! upstream's and it is not obviously wrong on its own terms: a register that is
//! tested and branched on genuinely might be live caller state rather than an
//! argument.  Relaxing it directly would fabricate an argument at every
//! `test rcx,rcx; jz; call` in the corpus.
//!
//! What settles the question is not local dataflow at all.  It is that the
//! *same callee* is called elsewhere in this function with that register
//! carrying an argument.
//!
//! # The mechanism
//!
//! At `buildInputFromTrials` time, look for another call spec in this function
//! with the same callee entry address whose argument list is already final, and
//! read its **register** argument storage off its CALL op.  Any of those
//! locations that this call site has a live, not-definitely-unused trial for is
//! promoted to used, so the two sites render with the same arity.
//!
//! Deliberate limits, each of which is what keeps a rescue from inventing an
//! argument:
//!
//! * **Only a call that recovered NOTHING.** This is the limit the whole-corpus
//!   sweep bought.  Without it the rule reads "same callee, same arity", which
//!   is false for a variadic callee and for an over-recovered witness, and it
//!   showed both: `Sleep(200)` became `Sleep(200,0)` from a sibling that had
//!   over-recovered `rdx`, and an internal logging function
//!   `sub_1b11c(5,0,"Zip: empty archive?")` gained two arguments its format
//!   string has no conversions for.  A call site with *some* arguments has a
//!   self-consistent list that may genuinely be shorter than another site's; a
//!   call site with **none** has failed outright, because a function other sites
//!   pass arguments to does not take zero here.  Only that second shape is
//!   rescued.
//! * **Register storage only.** A finalized call's stack arguments sit at
//!   *caller*-relative addresses that differ per call site by the site's own
//!   stack offset, so they are not comparable between two calls the way a
//!   register is.
//! * **A real Varnode only.** A synthetic *unreferenced* trial (slot < 0) has no
//!   value at this site; materializing one would print a read of an undefined
//!   register, so a witness location that maps to one aborts the whole rescue.
//! * **All or nothing.** Parameters are positional: adding the second argument
//!   while the first stays missing would be worse than adding neither.
//! * **Never subtractive.** A witness with *fewer* arguments proves nothing —
//!   its own site may be the one that lost them.
//!
//! # What it cannot do
//!
//! The witness has to be final already, and `ActionActiveParam` finalizes each
//! call spec as soon as that spec is fully checked, in `qlst` order.  So this
//! rule reconciles a call site against the sites *before* it, not after: when the
//! very first call to a callee is the one that lost an argument, nothing here
//! rescues it.  Deferring every finalization until every spec is checked would
//! make the direction symmetric and is the wrong way to get it — it changes what
//! `checkCallDoubleUse` sees while scoring, on every binary.  The other direction
//! is [`kuna_calleearityfwd`](crate::p4_calls::kuna_calleearityfwd), which retries
//! this same plan at the end of the pass instead of moving anything.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<calleearity>` (kuna 4000+ range; 4132 = varargstackargs).
pub const ELEM_CALLEEARITY: ElementId = ElementId::new("calleearity", 4133);

/// (kuna) Reconcile the recovered argument list across call sites of the same
/// callee: `calleearity on|off`.
pub struct OptionCalleeArity;

impl OptionCalleeArity {
    /// The option name.
    pub const NAME: &'static str = "calleearity";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Call-site argument-list reconciliation turned {prop}")))
    }
}

/// The register argument storage of an already-final direct call, in prototype
/// order.
///
/// Read off the call spec's recorded `final_input_storage`, not off the CALL
/// op's inputs: those carry the argument *values* (`sub_140008160(0x28)` passes
/// a constant Varnode whose address is in the constant space), and the location
/// an argument occupied is exactly what a sibling call needs.
///
/// `None` when the call is not usable as a witness — not a live direct CALL, no
/// recovered arguments, or carrying a stack argument, whose caller-relative
/// address means nothing at another call site.
fn witness_storage(data: &Funcdata, fc: &FuncCallSpecs) -> Option<Vec<(Address, int4)>> {
    let o = data.obank().get(fc.get_op())?;
    if o.is_dead() || o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let storage = fc.final_input_storage();
    if storage.is_empty() {
        return None;
    }
    if storage.iter().any(|(a, _)| {
        a.get_space().map(|s| s.get_type() == spacetype::IPTR_SPACEBASE).unwrap_or(true)
    }) {
        return None;
    }
    Some(storage.to_vec())
}

/// The richest usable witness for `fc`: another call spec to the same entry
/// address whose input recovery is already finished.
fn best_witness(fc: &FuncCallSpecs, data: &Funcdata) -> Vec<(Address, int4)> {
    best_witness_for(fc.get_entry_address(), fc.get_op(), data)
}

/// [`best_witness`] addressed by callee entry and calling op rather than by the
/// spec itself, so the deferred retry in
/// [`kuna_calleearityfwd`](crate::p4_calls::kuna_calleearityfwd) — which has no
/// `FuncCallSpecs` left to ask — runs the same search.
pub fn best_witness_for(entry: &Address, op: OpId, data: &Funcdata) -> Vec<(Address, int4)> {
    let mut best: Vec<(Address, int4)> = Vec::new();
    for j in 0..data.num_calls() {
        let sib = data.get_call_specs(j);
        if sib.is_input_active() || sib.get_op() == op {
            continue;
        }
        if sib.get_entry_address() != entry {
            continue;
        }
        if let Some(args) = witness_storage(data, sib) {
            if args.len() > best.len() {
                best = args;
            }
        }
    }
    best
}

/// What `plan_promotions` needs to know about one of this call's trials.
#[derive(Clone, Debug)]
pub struct TrialView {
    /// The trial's storage address.
    pub addr: Address,
    /// The trial's size in bytes.
    pub size: int4,
    /// Already a parameter (`ParamTrial::is_used`).
    pub used: bool,
    /// Scored definitely-not-used, so the CALL input is a zero constant now.
    pub definitely_not_used: bool,
    /// Synthetic hole-filling trial with no Varnode at this call site.
    pub unref: bool,
    /// The CALL op still carries a value in this trial's input slot.
    ///
    /// Deliberately NOT "the input Varnode lives at the trial's storage
    /// address": a CALL input carries the argument's *value*, and after
    /// constant propagation and Varnode merging that value is routinely a
    /// constant or a temporary rather than the register the ABI passes it in.
    /// The storage lives in the trial; only the presence of a value is asked of
    /// the op.
    pub has_value: bool,
}

/// Which trial indices to promote so this call's argument list covers
/// `witness`, or `None` when it must not be touched.
///
/// Refuses outright when the call already recovered *any* argument: a
/// self-consistent shorter list may be a genuinely shorter call (a variadic
/// callee, or a witness that itself over-recovered), and the sweep measured both
/// shapes.  Only an argument list that came out **empty** is a recovery failure
/// this can speak to.
///
/// All or nothing beyond that: parameters are positional, so covering the second
/// witness location while the first stays missing would be worse than covering
/// neither.  An empty `witness` yields an empty promotion list, the same no-op.
pub fn plan_promotions(witness: &[(Address, int4)], trials: &[TrialView]) -> Option<Vec<int4>> {
    if trials.iter().any(|t| t.used) {
        return None;
    }
    let mut promote = Vec::new();
    for (addr, size) in witness {
        let Some((i, t)) = trials
            .iter()
            .enumerate()
            .find(|(_, t)| t.addr == *addr && t.size == *size)
        else {
            return None;
        };
        if t.definitely_not_used || t.unref || !t.has_value {
            return None;
        }
        promote.push(i as int4);
    }
    Some(promote)
}

/// Promote this call's trials so its argument list matches a sibling call to the
/// same callee.  Returns the number of trials promoted (0 = nothing changed).
///
/// Called from `build_input_from_trials` after `deriveInputMap` has set the
/// `used` flags and before the CALL op's final input list is written.
pub fn unify_with_sibling_call(fc: &mut FuncCallSpecs, data: &Funcdata) -> int4 {
    if !data.get_arch().callee_arity || fc.is_input_locked() {
        return 0;
    }
    let op = fc.get_op();
    match data.obank().get(op) {
        Some(o) if !o.is_dead() && o.code() == OpCode::CPUI_CALL => {}
        _ => return 0,
    }
    let witness = best_witness(fc, data);
    if witness.is_empty() {
        return 0;
    }

    let trials: Vec<TrialView> = (0..fc.get_active_input().get_num_trials())
        .map(|i| {
            let t = fc.get_active_input().get_trial(i);
            let slot = t.get_slot();
            let has_value =
                slot >= 1 && data.obank().get(op).and_then(|o| o.get_in(slot)).is_some();
            TrialView {
                addr: t.get_address().clone(),
                size: t.get_size(),
                used: t.is_used(),
                definitely_not_used: t.is_definitely_not_used(),
                unref: t.is_unref(),
                has_value,
            }
        })
        .collect();

    let Some(promote) = plan_promotions(&witness, &trials) else { return 0 };
    for i in &promote {
        fc.get_active_input().get_trial_mut(*i).mark_used();
    }
    promote.len() as int4
}

#[cfg(test)]
#[path = "kuna_calleearity/tests.rs"]
mod tests;
