//! P4 caller-save spill tolerance in active input-trial scoring — the
//! `spillargtrial` decision point.
//!
//! # The gap
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1891`)
//! decides whether the Varnode reaching a CALL in a candidate parameter register
//! is used *only* as that call's argument.  Any `CPUI_STORE` among the Varnode's
//! descendants makes the answer `false`; `FuncCallSpecs::check_input_trial_use`
//! then calls `mark_inactive`, which also sets CHECKED, so the trial is never
//! re-scored and the argument-producing chain dies to dead-code elimination.
//!
//! The blanket STORE rejection is deliberate upstream: a callee-saved value that
//! the caller happens to write to its frame just before a call must not become a
//! bogus extra argument.  But it also rejects the *opposite* idiom.  When a value
//! is both an argument and live across the call, the register allocator emits a
//! caller-save spill of that same value:
//!
//! ```text
//!   movapd XMMWORD PTR [rsp+0x20],xmm1   ; caller-save spill of the argument
//!   movapd xmm0,xmm3
//!   call   atan2                          ; xmm1 is argument 1
//!   ...
//!   movaps xmm1,XMMWORD PTR [rsp+0x20]   ; reload for the second call
//! ```
//!
//! The spill is a second descendant of the very Varnode the trial is scoring, so
//! the trial goes inactive and the call renders as `atan2(one_arg)` — GH-275.  The
//! same shape appears whenever a value is passed to one call and reused after it,
//! which is why the issue needs *more than one* call to reproduce: a single call
//! leaves the value dead afterwards, so no spill is emitted.
//!
//! # The levels
//!
//! Relaxing the rule admits non-arguments — the failure mode is a *spurious*
//! trailing argument — so the predicate strength is settable, ordered by
//! permissiveness:
//!
//! * [`LEVEL_OFF`] — upstream-faithful; every STORE rejects.
//! * [`LEVEL_RELOAD`] — reject unless the store is one half of a genuine
//!   caller-save **spill/reload pair**: a later `CPUI_LOAD` reads the same frame
//!   slot at the same width.  A value that is stored and never read back is a
//!   real store, not a spill.
//! * [`LEVEL_SPILL`] — reject unless the store writes the walked Varnode's *value*
//!   to a caller-frame slot at all, with no reload required.
//!
//! # Timing
//!
//! `ActionActiveParam` runs before `ActionStackPtrFlow` in the first `mainloop`
//! iteration (`infra/universalaction.rs`), so `RuleStoreVarnode` has not yet
//! folded the frame `CPUI_STORE` into a direct stack-space write and
//! `RuleLoadVarnode::check_spacebase` still returns `None` for it — the pointer is
//! the raw `INT_ADD(<stack pointer register>, #const)`.  By the second iteration
//! the trial is CHECKED and no longer re-scored, so the predicate here has to read
//! that raw form directly.
//!
//! Two frame references denote the same slot only relative to the *same* stack
//! pointer value: different SSA versions of the stack pointer sit at different
//! actual offsets, so comparing their raw constants would be meaningless.  A
//! caller-save reload always straddles the call it was spilled for, and the call
//! re-defines the stack pointer, so the reload search cannot simply match
//! constants -- it walks forward from the store's own base Varnode carrying the
//! running offset delta.  That is also why it never scans the whole function.

use kuna_base::error::KunaResult;
use kuna_base::space::VarnodeStorage;
use kuna_base::types::{int4, uintb};

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// `option spillargtrial off` — upstream-faithful; every STORE descendant makes
/// the trial inactive.
pub const LEVEL_OFF: int4 = 0;

/// `option spillargtrial reload` — tolerate a frame store only when a later LOAD
/// reads the same slot at the same width (a genuine caller-save spill/reload
/// pair).
pub const LEVEL_RELOAD: int4 = 1;

/// `option spillargtrial spill` — tolerate any store of the walked Varnode's
/// value into a caller-frame slot, reload or not.
pub const LEVEL_SPILL: int4 = 2;

/// `option spillargtrial off|reload|spill` (C++ has no equivalent — upstream is
/// unconditionally `off`).
pub struct OptionSpillArgTrial;

impl OptionSpillArgTrial {
    /// The option name.
    pub const NAME: &'static str = "spillargtrial";

    /// Parse `off`/`reload`/`spill` into the predicate level + a confirmation
    /// message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((
                LEVEL_OFF,
                "Caller-save spill tolerance in input-trial scoring turned off".to_string(),
            )),
            // Empty parameter reads as the conservative level, not the permissive
            // one: `option spillargtrial` alone must not silently pick the arm
            // with the larger spurious-argument surface.
            "reload" | "on" | "" => Ok((
                LEVEL_RELOAD,
                "Caller-save spill tolerance in input-trial scoring turned on, \
                 spill/reload pairs only"
                    .to_string(),
            )),
            "spill" => Ok((
                LEVEL_SPILL,
                "Caller-save spill tolerance in input-trial scoring turned on, \
                 any caller-frame store of the argument value"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of off, reload, spill",
            )),
        }
    }
}

/// The stack-pointer register storage this architecture's stack space is
/// relative to (C++ `SpacebaseSpace::getSpacebase(0)`).
fn stack_pointer_storage(data: &Funcdata) -> Option<VarnodeStorage> {
    let stackspc = data.get_arch().manage().get_stack_space()?;
    if stackspc.num_spacebase() < 1 {
        return None;
    }
    stackspc.get_spacebase(0).ok()
}

/// Is `vn` the stack-pointer register itself (any SSA version of it)?
fn is_stack_pointer(data: &Funcdata, vn: VarnodeId, sp: &VarnodeStorage) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return false;
    }
    let Some(spspace) = sp.space.as_ref() else { return false };
    v.get_space().get_index() == spspace.get_index()
        && v.get_addr().get_offset() == sp.offset
        && v.get_size() as u32 == sp.size
}

/// Resolve a LOAD/STORE pointer Varnode to a caller-frame slot, as the pair
/// `(base stack-pointer Varnode, byte offset from it)`.
///
/// Accepts the stack pointer itself (offset 0) and `INT_ADD(sp, #const)` in
/// either operand order — the raw pre-`ActionStackPtrFlow` form.
fn frame_slot(data: &Funcdata, ptr: VarnodeId, sp: &VarnodeStorage) -> Option<(VarnodeId, uintb)> {
    if is_stack_pointer(data, ptr, sp) {
        return Some((ptr, 0));
    }
    let v = data.vbank().get(ptr)?;
    if !v.is_written() {
        return None;
    }
    let def = v.get_def()?;
    let d = data.obank().get(def)?;
    if d.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let in0 = d.get_in(0)?;
    let in1 = d.get_in(1)?;
    let const_of = |c: VarnodeId| -> Option<uintb> {
        let cv = data.vbank().get(c)?;
        cv.is_constant().then(|| cv.get_offset())
    };
    if is_stack_pointer(data, in0, sp) {
        return const_of(in1).map(|k| (in0, k));
    }
    if is_stack_pointer(data, in1, sp) {
        return const_of(in0).map(|k| (in1, k));
    }
    None
}

/// Does a later `CPUI_LOAD` read the same frame slot at `size` bytes?
///
/// A caller-save reload by construction sits *after* the call the spill was made
/// for, and a CALL re-defines the stack pointer — with the return-address push
/// and the `extrapop` restore both modelled as ordinary p-code arithmetic — so
/// the reload never reads off the store's own base Varnode, and its constant is
/// not directly comparable to the store's.  `ActionStackPtrFlow`, which would
/// normalize all of this, has not run yet at `ActionActiveParam` time.
///
/// So the search carries the *delta*: it walks forward from `base` over the
/// value-preserving and constant-displacing ops, tracking how far each reachable
/// Varnode sits from `base`, and a pointer whose delta equals `offset` addresses
/// the slot.  `MULTIEQUAL` is deliberately not followed — a phi's other arm may
/// carry a different frame.  Capped so a pathological chain cannot make this
/// quadratic.
fn has_later_reload(
    data: &Funcdata,
    base: VarnodeId,
    offset: uintb,
    size: int4,
    store_addr: uintb,
) -> bool {
    const CAP: usize = 64;
    let is_reload = |ptr: VarnodeId, load: OpId| -> bool {
        let Some(l) = data.obank().get(load) else { return false };
        if l.code() != OpCode::CPUI_LOAD || l.get_in(1) != Some(ptr) {
            return false;
        }
        if l.get_addr().get_offset() <= store_addr {
            return false;
        }
        l.get_out()
            .and_then(|ov| data.vbank().get(ov))
            .map(|ov| ov.get_size() == size)
            .unwrap_or(false)
    };
    let const_operand = |op: OpId, vn: VarnodeId| -> Option<uintb> {
        let o = data.obank().get(op)?;
        let other = if o.get_in(0) == Some(vn) { o.get_in(1) } else { o.get_in(0) };
        let c = data.vbank().get(other?)?;
        c.is_constant().then(|| c.get_offset())
    };
    let mut work: Vec<(VarnodeId, uintb)> = vec![(base, 0)];
    let mut i = 0;
    while i < work.len() {
        let (vn, delta) = work[i];
        i += 1;
        let Some(v) = data.vbank().get(vn) else { continue };
        if delta == offset && v.descend_iter().any(|op| is_reload(vn, op)) {
            return true;
        }
        if work.len() >= CAP {
            continue;
        }
        for op in v.descend_iter() {
            let Some(o) = data.obank().get(op) else { continue };
            let next = match o.code() {
                OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY if o.get_in(0) == Some(vn) => {
                    Some(delta)
                }
                OpCode::CPUI_INT_ADD => const_operand(op, vn).map(|k| delta.wrapping_add(k)),
                OpCode::CPUI_INT_SUB if o.get_in(0) == Some(vn) => {
                    const_operand(op, vn).map(|k| delta.wrapping_sub(k))
                }
                _ => None,
            };
            let (Some(d), Some(out)) = (next, o.get_out()) else { continue };
            if !work.iter().any(|(w, _)| *w == out) && work.len() < CAP {
                work.push((out, d));
            }
        }
    }
    false
}

/// Should `only_op_use` tolerate this `CPUI_STORE` descendant of a trial Varnode?
///
/// True only when the store writes `vn`'s own value (operand 2, never the
/// pointer) into a caller-frame slot, and — below [`LEVEL_SPILL`] — a later LOAD
/// reads that slot back at the same width.
pub fn store_is_caller_save_spill(data: &Funcdata, store_op: OpId, vn: VarnodeId) -> bool {
    let level = data.get_arch().spill_arg_trial;
    if level <= LEVEL_OFF {
        return false;
    }
    let Some(o) = data.obank().get(store_op) else { return false };
    if o.get_in(2) != Some(vn) {
        return false;
    }
    let store_addr = o.get_addr().get_offset();
    let Some(ptr) = o.get_in(1) else { return false };
    let Some(sp) = stack_pointer_storage(data) else { return false };
    let Some((base, offset)) = frame_slot(data, ptr, &sp) else { return false };
    if level >= LEVEL_SPILL {
        return true;
    }
    let Some(size) = data.vbank().get(vn).map(|v| v.get_size()) else { return false };
    has_later_reload(data, base, offset, size, store_addr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_ordered_by_permissiveness_so_the_gate_can_use_a_threshold() {
        // `store_is_caller_save_spill` reads `level <= LEVEL_OFF` to opt out and
        // `level >= LEVEL_SPILL` to skip the reload requirement, so the
        // vocabulary must stay monotone in how much it admits.
        assert!(LEVEL_OFF < LEVEL_RELOAD);
        assert!(LEVEL_RELOAD < LEVEL_SPILL);
    }

    #[test]
    fn apply_parses_the_three_spellings() {
        assert_eq!(OptionSpillArgTrial.apply("off").expect("off").0, LEVEL_OFF);
        assert_eq!(OptionSpillArgTrial.apply("reload").expect("reload").0, LEVEL_RELOAD);
        assert_eq!(OptionSpillArgTrial.apply("spill").expect("spill").0, LEVEL_SPILL);
        // `on` and the empty parameter take the CONSERVATIVE level, unlike
        // `calloverlap`, whose bare `on` is its most permissive arm.
        assert_eq!(OptionSpillArgTrial.apply("on").expect("on").0, LEVEL_RELOAD);
        assert_eq!(OptionSpillArgTrial.apply("").expect("empty").0, LEVEL_RELOAD);
        assert!(OptionSpillArgTrial.apply("paired").is_err());
    }
}
