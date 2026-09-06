//! (kuna) `calleearitylive` — extend a PARTIALLY recovered argument list, with
//! the callee's own body as the discriminator.
//!
//! # The gap
//!
//! [`calleearity`](crate::p4_calls::kuna_calleearity) and
//! [`calleearityfwd`](crate::p4_calls::kuna_calleearityfwd) reconcile a call
//! site against a sibling call to the same callee, and both refuse a site that
//! recovered *any* argument. That limit is not arbitrary — it is what the
//! whole-corpus sweep bought, and the module that carries it records the two
//! shapes it bought it from: `Sleep(200)` became `Sleep(200,0)` from a sibling
//! that had over-recovered `rdx`, and the internal *variadic* logger
//! `sub_1b11c(5,0,"Zip: empty archive?")` gained two arguments its format string
//! has no conversions for.
//!
//! But a partial list is not always self-consistent. `graphy` @0x1002c90 calls
//! the same helper fifteen times; eleven render with five arguments and four
//! with three:
//!
//! ```text
//!   sub_1005250(&v24,v22 & 0xffffffff,v8 & 0xff);                    // 0x10033c7
//!   sub_1005250(&v24,v22 & 0xffffffff,v8 & 0xff,v38,sub_1004ef0(..)); // 0x1003470
//! ```
//!
//! The two sites are instruction-for-instruction the same shape — `lea
//! -0x40(%rbp),%rdi; mov %r15d,%esi; movzbl ..,%edx; mov %r12d,%ecx; movzbl
//! %al,%r8d; call` — so `ecx` and `r8d` are written by dedicated `mov`s
//! immediately before the call at both. What sinks the short ones is
//! `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`) rejecting the `ecx` trial
//! because a `CBRANCH` also reads that value, and `fillinMap`'s positional rule
//! then drops `r8d` behind the hole it leaves.
//!
//! # The discriminator
//!
//! The sibling alone cannot settle this: it is exactly the evidence the sweep
//! showed was not enough. What settles it is the **callee's own body**, read
//! with the same bounded decode
//! [`calleedeadarg`](crate::p4_calls::kuna_calleedeadarg) already takes for the
//! subtractive direction — and read for two things at once:
//!
//! * every register the witness claims **beyond** this site's own list is read
//!   before it is written by the callee, so it genuinely carries an input; and
//! * **no other argument register of the model is**. This second half is what
//!   refuses the shapes the sweep found. `Sleep` is an import with no body to
//!   decode; `sub_1b11c` opens with an AArch64 variadic register-save area
//!   (`str x3,[sp,#136]; stp x4,x5,[sp,#144]; stp x6,x7,[sp,#160]`), so it reads
//!   `x5`, `x6` and `x7` too — registers a five-argument witness does not claim
//!   — and the extension is declined. A fixed-arity callee reads exactly the
//!   argument registers its prototype names and no more.
//!
//! Everything else is [`calleearity`](crate::p4_calls::kuna_calleearity)'s,
//! unchanged: register storage only, real Varnodes only, all-or-nothing, and
//! never subtractive. Two further limits are this module's own:
//!
//! * **Prefix only.** The site's own recovered list must be exactly the leading
//!   run of the witness's. Parameters are positional; a site whose arguments
//!   disagree with the witness *in place* is a different call, not a shorter one.
//! * **Deferred, always.** The four short sites in the witness above are the
//!   FIRST four and the first five-argument site is the fifth, so an in-order
//!   rule would see no witness at any of them. Like `calleearityfwd` this runs
//!   once at the end of `ActionActiveParam::apply`, when every spec in the pass
//!   is final, rather than reordering finalization — which would change what
//!   `Funcdata::checkCallDoubleUse` sees while scoring, on every binary.
//!
//! Inert unless both `calleearity` and `calleearitylive` are on: this completes
//! that rule rather than adding a second one.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, ParamEntry};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleearity::best_witness_for;

/// Marshaling element `<calleearitylive>` (kuna 4000+ range; 4145 = arraycoverwidth).
pub const ELEM_CALLEEARITYLIVE: ElementId = ElementId::new("calleearitylive", 4146);

/// (kuna) Extend a partially recovered argument list to a sibling call's, when
/// the callee's own body agrees: `calleearitylive on|off`.
pub struct OptionCalleeArityLive;

impl OptionCalleeArityLive {
    /// The option name.
    pub const NAME: &'static str = "calleearitylive";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_arity_live`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-body argument-list extension turned {prop}")))
    }
}

/// A call site that finalized with a SHORT but non-empty argument list, held
/// until every spec in the pass is final.
///
/// `recovered` is the storage of the arguments it did keep, in prototype order;
/// `candidates` are the register trials it dropped, each paired with the Varnode
/// the CALL carried for it — read before `opSetAllInput` removes it, which is
/// the only moment it is reachable.
pub struct PendingExtend {
    op: OpId,
    entry: Address,
    recovered: Vec<(Address, int4)>,
    candidates: Vec<(Address, int4, VarnodeId)>,
}

/// Is `addr` in a stack (spacebase) space? A missing space answers `true`, so an
/// unresolvable location is treated as the un-comparable case.
fn is_stack(addr: &Address) -> bool {
    addr.get_space().map(|s| s.get_type() == spacetype::IPTR_SPACEBASE).unwrap_or(true)
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

/// Capture a call site that is about to render with a short argument list.
///
/// Called from `build_input_from_trials` with the trials still intact and the
/// CALL op's pre-rewrite inputs still attached. `None` whenever the extension
/// could not apply anyway: option off, locked prototype, not a live direct CALL,
/// nothing recovered at all (that is `calleearityfwd`'s case), a stack argument
/// among the recovered ones, or no promotable register trial left behind them.
pub fn capture_partial_call(fc: &FuncCallSpecs, data: &Funcdata) -> Option<PendingExtend> {
    let arch = data.get_arch();
    if !arch.callee_arity || !arch.callee_arity_live || fc.is_input_locked() {
        return None;
    }
    let op = fc.get_op();
    let o = data.obank().get(op)?;
    if o.is_dead() || o.code() != OpCode::CPUI_CALL {
        return None;
    }
    let active = fc.active_input();
    let mut recovered: Vec<(Address, int4)> = Vec::new();
    let mut candidates: Vec<(Address, int4, VarnodeId)> = Vec::new();
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        let addr = t.get_address();
        if t.is_used() {
            // A stack argument means the register section already ended, and its
            // caller-relative address is not comparable across call sites.
            if is_stack(addr) || !candidates.is_empty() {
                return None;
            }
            recovered.push((addr.clone(), t.get_size()));
            continue;
        }
        if t.is_definitely_not_used() || t.is_unref() || is_stack(addr) {
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
    if recovered.is_empty() || candidates.is_empty() {
        return None;
    }
    Some(PendingExtend { op, entry: fc.get_entry_address().clone(), recovered, candidates })
}

/// Retry every captured call site now that the pass has finalized all of them.
///
/// Applied in capture order, so a site extended here can itself witness a later
/// one — the same chaining the in-order rule already has.
pub fn extend_pending(data: &mut Funcdata, pending: &[PendingExtend]) -> int4 {
    let mut extended = 0;
    for p in pending {
        if extend_one(data, p) {
            extended += 1;
        }
    }
    extended
}

/// The model's argument locations that `witness` does NOT claim.
///
/// The callee reading one of these before writing it is what says the witness is
/// not its whole argument list — a variadic register-save prologue reads every
/// argument register there is.
pub fn uncovered_argument_locations(
    entries: &[(Address, int4)],
    witness: &[(Address, int4)],
) -> Vec<(Address, int4)> {
    entries
        .iter()
        .filter(|(ea, esz)| !witness.iter().any(|(a, sz)| overlaps(ea, *esz, a, *sz)))
        .cloned()
        .collect()
}

/// The model's argument locations as plain `(address, size)` pairs.
fn entry_locations(entries: &[ParamEntry]) -> Vec<(Address, int4)> {
    entries
        .iter()
        .map(|e| (Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect()
}

/// Which candidate trials cover the witness's `tail`, in prototype order, or
/// `None` when this site must not be touched.
///
/// All or nothing: parameters are positional, so covering the second missing
/// location while the first stays absent would be worse than covering neither.
/// The candidates are consumed left to right, so a location out of prototype
/// order does not match.
pub fn plan_tail(
    tail: &[(Address, int4)],
    candidates: &[(Address, int4)],
) -> Option<Vec<int4>> {
    let mut picked = Vec::new();
    let mut next = 0usize;
    for (wa, wsz) in tail {
        let k = candidates[next..].iter().position(|(ca, csz)| ca == wa && csz == wsz)?;
        picked.push((next + k) as int4);
        next += k + 1;
    }
    Some(picked)
}

/// Does the callee's own body agree that `witness` is its COMPLETE register
/// argument list?
///
/// Two halves, and the second is the one that refuses a variadic callee and an
/// import: every location in `tail` — the part of the witness this site is
/// missing — must be read before written by the callee, and no argument location
/// of the model outside `witness` may be.
fn callee_body_agrees(
    data: &Funcdata,
    entry: &Address,
    entries: &[ParamEntry],
    witness: &[(Address, int4)],
    tail: &[(Address, int4)],
) -> bool {
    let Some(live) = data.kuna_callee_entry_dead(entry) else { return false };
    if !live.is_complete() {
        return false;
    }
    if !tail.iter().all(|(a, sz)| live.proves_read(a, *sz)) {
        return false;
    }
    uncovered_argument_locations(&entry_locations(entries), witness)
        .iter()
        .all(|(a, sz)| !live.proves_read(a, *sz))
}

/// Extend one captured call's input list from a now-final sibling, or leave it
/// alone. Returns whether the call gained arguments.
fn extend_one(data: &mut Funcdata, p: &PendingExtend) -> bool {
    let keep = p.recovered.len() + 1;
    match data.obank().get(p.op) {
        Some(o)
            if !o.is_dead()
                && o.code() == OpCode::CPUI_CALL
                && o.num_input() == keep as int4 => {}
        _ => return false,
    }
    let witness = best_witness_for(&p.entry, p.op, data);
    if witness.len() <= p.recovered.len() {
        return false;
    }
    // Positional: this site's own list has to be the witness's leading run.
    if !witness
        .iter()
        .zip(p.recovered.iter())
        .all(|((wa, wsz), (ra, rsz))| wa == ra && wsz == rsz)
    {
        return false;
    }
    let tail = &witness[p.recovered.len()..];
    // All or nothing: every missing location needs a live, right-width Varnode
    // still standing at this site, in prototype order.
    let locs: Vec<(Address, int4)> =
        p.candidates.iter().map(|(a, sz, _)| (a.clone(), *sz)).collect();
    let Some(picked) = plan_tail(tail, &locs) else { return false };
    let promoted: Vec<VarnodeId> =
        picked.iter().map(|i| p.candidates[*i as usize].2).collect();
    let Some(idx) = data.get_call_specs_index(p.op) else { return false };
    let entries = {
        let fc = data.get_call_specs(idx);
        if !fc.proto().has_model() {
            return false;
        }
        fc.proto().input_param_entries()
    };
    if entries.is_empty() {
        return false;
    }
    if !callee_body_agrees(data, &p.entry, &entries, &witness, tail) {
        return false;
    }
    let mut newparam: Vec<VarnodeId> = Vec::new();
    for i in 0..keep as int4 {
        match data.obank().get(p.op).and_then(|o| o.get_in(i)) {
            Some(v) => newparam.push(v),
            None => return false,
        }
    }
    for vn in &promoted {
        if data.vbank().get(*vn).is_none() {
            return false;
        }
        newparam.push(*vn);
    }
    if data.op_set_all_input(p.op, &newparam).is_err() {
        return false;
    }
    // Record where the arguments lived so a later extension can witness this site.
    data.get_call_specs_mut(idx).set_final_input_storage(witness);
    true
}

#[cfg(test)]
#[path = "kuna_calleearitylive/tests.rs"]
mod tests;
