//! (kuna) `calleearitybody` — recover an argument list from the callee's own
//! body when the call has no sibling to compare against.
//!
//! # The gap
//!
//! [`calleearity`](crate::p4_calls::kuna_calleearity),
//! [`calleearityfwd`](crate::p4_calls::kuna_calleearityfwd) and
//! [`calleearitylive`](crate::p4_calls::kuna_calleearitylive) are all *sibling*
//! reconciliation: each needs another call to the same entry address in the same
//! function whose argument list is already final and longer.  That leaves one
//! quadrant of the family empty — the callee that is called exactly **once**:
//!
//! ```text
//!   0x14000b649  mov  rbx,rcx                 ; rcx = the caller's own a0
//!   0x14000b64c  cmp  dword ptr [rcx+0x238],edi
//!   0x14000b652  je   0x14000b726
//!   ...
//!   0x14000b676  lea  r8,[0x14000ca0c]        ; payload's ADDRESS, for CreateThread
//!   ...
//!   0x14000b726  call 0x14000ca0c             ; the only call to payload
//! ```
//!
//! `rcx` is untouched between the entry and the `CALL`, so the argument really is
//! the caller's own first parameter, and kuna's own recovery of the callee agrees
//! — `payload` on its own comes out `unsigned long long payload(unsigned int *)`.
//! But `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`) rejects the `rcx` trial
//! because a `CBRANCH` also reads that value, and the address-taken `lea` is not
//! a call site, so the sibling search has nothing to find.  The call renders
//! `payload();` and the incoming context is gone.
//!
//! # The discriminator
//!
//! Relaxing `onlyOpUse` is the wrong fix and the family already says so: it would
//! fabricate an argument at every `test rcx,rcx; jz; call` there is.  What is
//! left is the evidence `calleearitylive` already reads — the bounded decode of
//! the callee body [`calleedeadarg`](crate::p4_calls::kuna_calleedeadarg) takes
//! (`probe_callee_entry_dead`) — standing **alone** as the witness rather than
//! only extending a sibling's list.
//!
//! Walk this call's trials in prototype order and read the callee's answer for
//! each argument register:
//!
//! * the recovered list is the leading run the callee `proves_input` — some path
//!   reads that register before writing it, for a value that can actually reach
//!   something, so it genuinely carries an input; and
//! * the register right after that run must be `proves_dead` — the callee
//!   overwrites it before ever reading it, on *every* path.
//!
//! The second half is the whole safety of the rule.  `proves_input` is an
//! existential and would happily run to the end of the argument registers; what
//! bounds the list is a register the callee is *proven* not to consume.  It is
//! also what refuses the shape the family's corpus sweep found: an AArch64
//! variadic register-save prologue (`str x3,[sp,#136]; stp x4,x5,[sp,#144];
//! stp x6,x7,[sp,#160]`) reads every argument register there is, so no dead one
//! follows the run and the rule declines rather than inventing eight arguments.
//! An import, a thunk and a body past the decode budget all answer neither, and
//! decline the same way.
//!
//! Everything else is [`calleearity`](crate::p4_calls::kuna_calleearity)'s,
//! unchanged: register storage only, real Varnodes only, all-or-nothing, never
//! subtractive, and only a call that recovered **nothing at all** — a site with a
//! self-consistent partial list is `calleearitylive`'s, and is left alone.
//! Like both deferred rules it runs once at the end of `ActionActiveParam::apply`
//! rather than reordering finalization, and it runs *after* them, so wherever a
//! sibling can speak the sibling still wins.
//!
//! Inert unless `calleearity` is also on: this completes that rule rather than
//! adding a second one.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, ParamEntry};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;

/// Marshaling element `<calleearitybody>` (kuna 4000+ range; 4151 = calleepop).
pub const ELEM_CALLEEARITYBODY: ElementId = ElementId::new("calleearitybody", 4152);

/// (kuna) Recover a lone call's argument list from the callee's own body:
/// `calleearitybody on|off`.
pub struct OptionCalleeArityBody;

impl OptionCalleeArityBody {
    /// The option name.
    pub const NAME: &'static str = "calleearitybody";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity_body`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body argument-list recovery turned {prop}")))
    }
}

/// One of a captured call's trials, in prototype order.
///
/// `vn` is the Varnode the CALL carried in that trial's slot — read before
/// `opSetAllInput` drops it, which is the only moment it is reachable — and is
/// `None` for a trial that could not be promoted anyway (definitely-not-used,
/// unreferenced, no value, or the wrong width for the trial).
#[derive(Clone, Debug)]
pub struct BodyTrial {
    /// The trial's storage address.
    pub addr: Address,
    /// The trial's size in bytes.
    pub size: int4,
    /// The Varnode standing at this trial, when it is promotable.
    pub vn: Option<VarnodeId>,
}

/// A call site that finalized with an empty argument list and no sibling to
/// speak for it, held until the sibling rules have had their turn.
pub struct PendingBodyArgs {
    op: OpId,
    entry: Address,
    trials: Vec<BodyTrial>,
}

/// Is `addr` in the `register` space the callee-body summary answers for?
///
/// By name, not by `spacetype`: a processor's register file and its ram are both
/// `IPTR_PROCESSOR`, and `register` is the name
/// [`probe_callee_entry_dead`](crate::p4_calls::kuna_calleedeadarg::probe_callee_entry_dead)
/// resolves its own index from.
fn is_register(addr: &Address) -> bool {
    addr.get_space().map(|s| s.get_name() == "register").unwrap_or(false)
}

/// Capture a call site that is about to render with an empty argument list.
///
/// Called from `build_input_from_trials` with the trials still intact and the
/// CALL op's pre-rewrite inputs still attached.  `None` whenever the recovery
/// could not apply anyway: option off, locked prototype, not a live direct CALL
/// to a known entry, or no register trial at all.
pub fn capture_lone_call(fc: &FuncCallSpecs, data: &Funcdata) -> Option<PendingBodyArgs> {
    let arch = data.get_arch();
    if !arch.callee_arity || !arch.callee_arity_body || fc.is_input_locked() {
        return None;
    }
    let op = fc.get_op();
    let o = data.obank().get(op)?;
    if o.is_dead() || o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let entry = fc.get_entry_address().clone();
    if entry.is_invalid() {
        return None;
    }
    let active = fc.active_input();
    let mut trials: Vec<BodyTrial> = Vec::new();
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if t.is_used() {
            return None; // this site recovered something; not ours to speak to
        }
        let addr = t.get_address().clone();
        let size = t.get_size();
        let slot = t.get_slot();
        let mut vn = None;
        if !t.is_definitely_not_used() && !t.is_unref() && slot >= 1 {
            if let Some(v) = data.obank().get(op).and_then(|o| o.get_in(slot)) {
                // The normal path would insert a truncating SUBPIECE here; the
                // retry runs after the trials are gone, so an oversized Varnode
                // is declined instead.
                if data.vbank().get(v).map(|x| x.get_size()) == Some(size) {
                    vn = Some(v);
                }
            }
        }
        trials.push(BodyTrial { addr, size, vn });
    }
    if !trials.first().map(|t| is_register(&t.addr)).unwrap_or(false) {
        return None;
    }
    Some(PendingBodyArgs { op, entry, trials })
}

/// Which of this call's trials the callee's body claims as arguments, or `None`
/// when the body does not settle the list.
///
/// `entries` are the prototype model's own input locations, and they are what
/// bound the claim.  Two guards, and the rule is only as safe as their
/// conjunction:
///
/// * **no argument location outside the run may be read** by the callee, so the
///   run really is its whole register argument list rather than a prefix of it —
///   this is what refuses a variadic register-save prologue, which reads every
///   argument register there is; and
/// * **some argument location outside the run must be provably dead** — the
///   callee overwrites it before ever reading it, on every path.  "Some path
///   reads this register" is an existential, and on its own it would happily
///   claim a run and say nothing about where the list ends; a register the
///   callee is PROVEN not to consume is what ends it.  A body that forwards its
///   own arguments to a nested call is cut at that call and proves neither, so it
///   declines.
pub fn plan_from_body(
    trials: &[BodyTrial],
    entries: &[(Address, int4)],
    live: &CalleeEntryDead,
) -> Option<Vec<int4>> {
    if !live.is_complete() || entries.is_empty() {
        return None;
    }
    let mut picked: Vec<int4> = Vec::new();
    let mut claimed: Vec<(Address, int4)> = Vec::new();
    for (i, t) in trials.iter().enumerate() {
        // The register section is what the callee-body decode answers for; a
        // caller-relative stack slot ends the walk.
        if !is_register(&t.addr) {
            break;
        }
        if !live.proves_input(&t.addr, t.size) {
            // A register the callee does not consume is not an argument, so it
            // does not break the run.  Before the run it is different: skipping
            // a register the caller DID write would print the next argument in
            // its position, and nothing in the output would say so.  An EMPTY
            // one is fine -- the unreferenced float-register fillers
            // `build_trial_map` synthesizes on SysV sit in front of the first
            // integer argument and there is nothing to print at them.
            if picked.is_empty() && t.vn.is_some() {
                return None;
            }
            continue;
        }
        t.vn?; // all or nothing: a claimed register with no value here aborts
        picked.push(i as int4);
        claimed.push((t.addr.clone(), t.size));
    }
    if picked.is_empty() {
        return None;
    }
    let left: Vec<(Address, int4)> = entries
        .iter()
        .filter(|(ea, esz)| !claimed.iter().any(|(a, sz)| overlaps(ea, *esz, a, *sz)))
        .cloned()
        .collect();
    if left.iter().any(|(a, sz)| live.proves_input(a, *sz)) {
        return None;
    }
    if !left.iter().any(|(a, sz)| live.proves_dead(a, *sz)) {
        return None;
    }
    Some(picked)
}

/// Do two storage locations share a byte?
fn overlaps(a: &Address, asz: int4, b: &Address, bsz: int4) -> bool {
    let (Some(sa), Some(sb)) = (a.get_space(), b.get_space()) else { return false };
    if sa.get_index() != sb.get_index() {
        return false;
    }
    let (ao, bo) = (a.get_offset(), b.get_offset());
    ao < bo.wrapping_add(bsz as u64) && bo < ao.wrapping_add(asz as u64)
}

/// The prototype model's input locations as plain `(address, size)` pairs.
fn entry_locations(entries: &[ParamEntry]) -> Vec<(Address, int4)> {
    entries
        .iter()
        .map(|e| (Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect()
}

/// Retry every captured call site, after the sibling rules have had their turn.
pub fn recover_pending(data: &mut Funcdata, pending: &[PendingBodyArgs]) -> int4 {
    let mut recovered = 0;
    for p in pending {
        if recover_one(data, p) {
            recovered += 1;
        }
    }
    recovered
}

/// Give one captured call the argument list its callee's body implies, or leave
/// it alone.  Returns whether the call gained arguments.
fn recover_one(data: &mut Funcdata, p: &PendingBodyArgs) -> bool {
    // `num_input() == 1` is also what defers to the sibling rules: a call
    // `calleearityfwd` already rescued is no longer empty and is not touched.
    match data.obank().get(p.op) {
        Some(o) if !o.is_dead() && o.code() == OpCode::CPUI_CALL && o.num_input() == 1 => {}
        _ => return false,
    }
    let Some(idx) = data.get_call_specs_index(p.op) else { return false };
    let entries = {
        let fc = data.get_call_specs(idx);
        if !fc.proto().has_model() {
            return false;
        }
        entry_locations(&fc.proto().input_param_entries())
    };
    let picked = {
        let Some(live) = data.kuna_callee_entry_dead(&p.entry) else { return false };
        match plan_from_body(&p.trials, &entries, live) {
            Some(v) => v,
            None => return false,
        }
    };
    let Some(in0) = data.obank().get(p.op).and_then(|o| o.get_in(0)) else { return false };
    let mut newparam = vec![in0];
    for i in &picked {
        let Some(vn) = p.trials[*i as usize].vn else { return false };
        if data.vbank().get(vn).is_none() {
            return false;
        }
        newparam.push(vn);
    }
    if data.op_set_all_input(p.op, &newparam).is_err() {
        return false;
    }
    // Record where the arguments lived so a later sibling rescue can witness
    // this site.
    let storage: Vec<(Address, int4)> = picked
        .iter()
        .map(|i| {
            let t = &p.trials[*i as usize];
            (t.addr.clone(), t.size)
        })
        .collect();
    data.get_call_specs_mut(idx).set_final_input_storage(storage);
    true
}

#[cfg(test)]
#[path = "kuna_calleearitybody/tests.rs"]
mod tests;
