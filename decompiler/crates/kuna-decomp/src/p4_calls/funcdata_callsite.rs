//! Call-site parameter/return recovery over the live IR (the `FuncCallSpecs`
//! trial machinery, `fspec.cc:5543-5860`).
//!
//! In C++ these are methods on `FuncCallSpecs` taking `Funcdata &data`:
//!   - [`check_input_trial_use`] — `FuncCallSpecs::checkInputTrialUse`
//!     (`fspec.cc:5592`): for each input trial, decide \e active / \e inactive /
//!     \e no-use from ancestor-realism + `ancestorOpUse`, freeing definitely-
//!     unused dataflow.
//!   - [`final_input_check`] — `FuncCallSpecs::finalInputCheck` (`fspec.cc:5571`):
//!     re-check cond-exe-affected trials after conditional execution analysis.
//!   - [`build_input_from_trials`] — `FuncCallSpecs::buildInputFromTrials`
//!     (`fspec.cc:5692`): set the CALL op's final input list from the used trials.
//!   - [`collect_output_trial_varnodes`] / [`check_output_trial_use`] /
//!     [`build_output_from_trials`] — the return-value analogues
//!     (`fspec.cc:5543`/`5668`/`5777`).
//!
//! Here they are free functions taking `(&mut FuncCallSpecs, &mut Funcdata, …)`:
//! the C++ `FuncCallSpecs` *is-a* `FuncProto` member of the `qlst` and mutates
//! `data` through it; the borrow checker forces the call spec to be lifted out of
//! `Funcdata::qlst` first (see [`Funcdata::take_call_specs`]) so the two `&mut`
//! borrows do not overlap.  Driven by `ActionActiveParam`/`ActionActiveReturn`
//! (`coreaction.cc:1769`/`1817`).
//!
//! `uintb` is `u64` with wrapping ops; the `AliasChecker` graph walk routes
//! through the live-IR [`AliasGatherAccess`](crate::varmap::AliasGatherAccess) already
//! built in `funcdata_spacebase.rs`.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::funcdata_varnode::AncestorRealistic;
use crate::context::{OpId, VarnodeId};
use crate::varmap::AliasChecker;

/// (kuna) The deferred argument-list retry one finalized call leaves behind.
///
/// At most one of the two is ever set: a call whose list came out EMPTY is
/// [`crate::p4_calls::kuna_calleearityfwd`]'s, one whose list came out SHORT is
/// [`crate::p4_calls::kuna_calleearitylive`]'s. Both are replayed at the end of
/// `ActionActiveParam::apply`, when every spec in the pass is final.
#[derive(Default)]
pub struct PendingCallFixup {
    /// The empty-list rescue candidate (`calleearityfwd`).
    pub rescue: Option<crate::p4_calls::kuna_calleearityfwd::PendingRescue>,
    /// The short-list extension candidate (`calleearitylive`).
    pub extend: Option<crate::p4_calls::kuna_calleearitylive::PendingExtend>,
}

/// C++ `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5592`).
///
/// Run through each input trial and decide whether it is \e active (a write
/// reached the call with no intervening read), \e inactive, or \e no-use.  A
/// definitely-unused trial has its dataflow freed (`opSetInput` with a zero
/// constant).  The `aliascheck` is the deferred-gather local alias checker
/// (`AliasChecker::gather(..., defer=true)`), supplied by the caller.
pub fn check_input_trial_use(idx: int4, data: &mut Funcdata, aliascheck: &mut AliasChecker) {
    // INDEX-BASED (CORRECTION-7 #3): the call spec stays in `data.qlst` so the
    // ancestor walk (`ancestor_op_use` -> `only_op_use` -> `check_call_double_use`)
    // can look up the *other* call's spec via `get_call_specs_index(op)`.  We
    // access this call's spec through `data.get_call_specs[_mut](idx)`, dropping
    // that borrow before any `&mut data` op; the trial mutated by
    // `ancestor_op_use` is cloned out, passed, then written back.
    let op = data.get_call_specs(idx).get_op();
    if data.obank().get(op).map(|o| o.is_dead()).unwrap_or(true) {
        // C++ throw LowlevelError("Function call in dead code"); the action
        // wrapper would surface it — here a dead call spec is simply skipped (the
        // recovery cannot mutate a dead op).  (Faithful: this op never reaches
        // here on the live path, as setupCallSpecs only runs on live CALLs.)
        return;
    }

    let maxancestor = data.get_arch().trim_recurse_max;
    // C++ callee_pop / expop: hard evidence about active trials when the callee
    // pops its own parameters and the extrapop is recovered (>4).
    let mut callee_pop = false;
    let mut expop = 0i32;
    if data.get_call_specs(idx).proto().has_model() {
        callee_pop = data.get_call_specs(idx).get_model_extra_pop() == crate::fspec::EXTRAPOP_UNKNOWN;
        if callee_pop {
            expop = data.get_call_specs(idx).get_extra_pop();
            if expop == crate::fspec::EXTRAPOP_UNKNOWN || expop <= 4 {
                callee_pop = false;
            }
        }
    }

    let num_trials = data.get_call_specs_mut(idx).get_active_input().get_num_trials();
    for i in 0..num_trials {
        if data.get_call_specs_mut(idx).get_active_input().get_trial(i).is_checked() {
            continue;
        }
        let slot = data.get_call_specs_mut(idx).get_active_input().get_trial(i).get_slot();
        let vn = match data.obank().get(op).and_then(|o| o.get_in(slot)) {
            Some(v) => v,
            None => continue,
        };
        let (vn_space, vn_offset, vn_size, vn_is_input) = {
            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            (
                v.get_space().clone(),
                v.get_offset(),
                v.get_size(),
                v.is_input(),
            )
        };
        let is_spacebase = vn_space.get_type() == spacetype::IPTR_SPACEBASE;

        if is_spacebase {
            // hasLocalAlias(vn): build the alias info lazily on first use.
            let has_alias = {
                let mut access = data.alias_gather_access();
                aliascheck.has_local_alias(Some((vn_space.clone(), vn_offset)), &mut access)
            };
            let trial_addr =
                data.get_call_specs_mut(idx).get_active_input().get_trial(i).get_address().clone();
            // C++ keeps these two no-use conditions as separate branches
            // (fspec.cc:5616-5619): a stack alias, OR a stack location outside the
            // CALLER's local range, both mean "not a parameter".  The range test is
            // on the argument Varnode's caller-relative address, not the trial's
            // callee-relative one — (kuna) `callsitestackargs` owns that choice.
            #[allow(clippy::if_same_then_else)]
            if has_alias {
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
            } else if crate::p4_calls::kuna_callsitestackargs::outside_caller_local_range(
                data.get_arch().callsite_stack_args,
                data.get_func_proto().get_local_range(),
                &vn_space,
                vn_offset,
                &trial_addr,
            ) {
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
            } else if callee_pop {
                let off = trial_addr.get_offset();
                let sz = data.get_call_specs_mut(idx).get_active_input().get_trial(i).get_size();
                if (off as i64 + (sz as i64 - 1)) < expop as i64 {
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_active();
                } else {
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
                }
            } else {
                let (trial_size, trial_cond, trial_killed) = {
                    let t = data.get_call_specs_mut(idx).get_active_input().get_trial(i);
                    (t.get_size(), t.has_cond_exe_effect(), t.is_killed_by_call())
                };
                let mut ancestor = AncestorRealistic::new();
                let (realistic, solid) =
                    ancestor.execute(data, op, slot, trial_size, trial_cond, trial_killed, false);
                ancestor.apply_trial(
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i),
                    realistic,
                    solid,
                );
                if realistic || solid {
                    // Clone the trial across `ancestor_op_use` (it needs `&mut
                    // data`, and its walk reads the populated qlst); write the
                    // mutated trial back afterward.
                    let mut trial =
                        data.get_call_specs(idx).active_input().get_trial(i).clone();
                    let only = data.ancestor_op_use(maxancestor, vn, op, &mut trial, 0, 0);
                    *data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i) = trial;
                    if only {
                        data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_active();
                    } else {
                        data.get_call_specs_mut(idx)
                            .get_active_input()
                            .get_trial_mut(i)
                            .mark_inactive();
                    }
                } else {
                    // Stackvar for unrealistic ancestor is definitely not a parameter
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
                }
            }
        } else if {
            // (kuna) `calleedeadarg`: the callee's OWN body can settle a register
            // trial the caller's data flow cannot.  A register the callee
            // overwrites (or returns without touching) on every path from its
            // entry is dead there and cannot be carrying an argument, however
            // live it looks on this side of the call.
            let (trial_addr, trial_size) = {
                let t = data.get_call_specs(idx).active_input().get_trial(i);
                (t.get_address().clone(), t.get_size())
            };
            crate::p4_calls::kuna_calleedeadarg::trial_is_dead_in_callee(
                data, idx, slot, &trial_addr, trial_size,
            )
        } {
            data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
        } else {
            let (trial_size, trial_cond, trial_killed) = {
                let t = data.get_call_specs_mut(idx).get_active_input().get_trial(i);
                (t.get_size(), t.has_cond_exe_effect(), t.is_killed_by_call())
            };
            let mut ancestor = AncestorRealistic::new();
            let (realistic, solid) =
                ancestor.execute(data, op, slot, trial_size, trial_cond, trial_killed, true);
            ancestor.apply_trial(
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i),
                realistic,
                solid,
            );
            if realistic || solid {
                let mut trial = data.get_call_specs(idx).active_input().get_trial(i).clone();
                let only = data.ancestor_op_use(maxancestor, vn, op, &mut trial, 0, 0);
                *data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i) = trial;
                if only {
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_active();
                    if data
                        .get_call_specs_mut(idx)
                        .get_active_input()
                        .get_trial(i)
                        .has_cond_exe_effect()
                    {
                        data.get_call_specs_mut(idx).get_active_input().mark_needs_final_check();
                    }
                } else {
                    data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_inactive();
                }
            } else if vn_is_input {
                // Not likely a parameter but maybe
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_inactive();
            } else {
                // An ancestor is unaffected, an unusual input, or killed by a call
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
            }
        }

        // If definitely not used, free up the dataflow.
        if data.get_call_specs_mut(idx).get_active_input().get_trial(i).is_definitely_not_used() {
            let c = data.new_constant(vn_size, 0);
            let _ = data.op_set_input(op, c, slot);
        }
    }
}

/// C++ `FuncCallSpecs::finalInputCheck` (`fspec.cc:5571`).
///
/// Re-check trials that might have been converted to \e not-used by conditional
/// execution analysis.  Each active trial with a cond-exe effect is re-run
/// through ancestor-realism; a now-unrealistic ancestor marks it no-use.
pub fn final_input_check(fc: &mut FuncCallSpecs, data: &mut Funcdata) {
    let op = fc.get_op();
    let num_trials = fc.get_active_input().get_num_trials();
    for i in 0..num_trials {
        if !fc.get_active_input().get_trial(i).is_active() {
            continue;
        }
        if !fc.get_active_input().get_trial(i).has_cond_exe_effect() {
            continue;
        }
        let (slot, trial_size, trial_cond, trial_killed) = {
            let t = fc.get_active_input().get_trial(i);
            (t.get_slot(), t.get_size(), t.has_cond_exe_effect(), t.is_killed_by_call())
        };
        let mut ancestor = AncestorRealistic::new();
        let (realistic, solid) =
            ancestor.execute(data, op, slot, trial_size, trial_cond, trial_killed, false);
        ancestor.apply_trial(fc.get_active_input().get_trial_mut(i), realistic, solid);
        if !(realistic || solid) {
            fc.get_active_input().get_trial_mut(i).mark_no_use();
        }
    }
}

/// C++ `FuncCallSpecs::buildInputFromTrials` (`fspec.cc:5692`).
///
/// Set the final input list of the CALL op from the used input trials, in
/// prototype order.  `op->getIn(0)` (the fspec annotation) is preserved.  A
/// trial whose Varnode is bigger than its recovered type is truncated with a
/// `SUBPIECE`.  Spacebase parameters mark their stack range as unmapped.
///
/// (kuna) Returns the deferred-retry candidate this call leaves behind — the
/// `calleearityfwd` rescue when its argument list comes out EMPTY, the
/// `calleearitylive` extension when it comes out SHORT. In both cases the
/// sibling that could speak for it may not be final yet, and the Varnodes its
/// dropped trials point at are reachable only here, before `opSetAllInput`
/// drops them.
pub fn build_input_from_trials(
    fc: &mut FuncCallSpecs,
    data: &mut Funcdata,
) -> PendingCallFixup {
    let op = fc.get_op();
    let mut newparam: Vec<VarnodeId> = Vec::new();
    // Preserve the fspec parameter (in0).
    if let Some(in0) = data.obank().get(op).and_then(|o| o.get_in(0)) {
        newparam.push(in0);
    }

    // varargs + locked: move the fixed args to the front so the relative order of
    // variable args is preserved.
    if fc.is_dotdotdot() && fc.is_input_locked() {
        let entries = fc.proto().input_param_entries();
        fc.get_active_input().sort_fixed_position(&entries);
    }

    // C++ data.getArch()->translate->isBigEndian(): the program's byte order,
    // read here off the default code space (the lift target's endianness).
    let big_endian = data
        .get_arch()
        .manage()
        .get_default_code_space()
        .map(|s| s.is_big_endian())
        .unwrap_or(false);
    // (kuna) `calleearity`: before the argument list is written, reconcile it
    // with a sibling call to the same callee whose list is already final, so one
    // callee does not render with two arities in one function.  Inert with the
    // option off.  See [`crate::p4_calls::kuna_calleearity`].
    crate::p4_calls::kuna_calleearity::unify_with_sibling_call(fc, data);

    let stackoffset = fc.get_stackoffset();
    let num_trials = fc.get_active_input().get_num_trials();
    for i in 0..num_trials {
        if !fc.get_active_input().get_trial(i).is_used() {
            continue; // Don't keep unused parameters
        }
        let (sz, addr, slot, is_unref) = {
            let t = fc.get_active_input().get_trial(i);
            (t.get_size(), t.get_address().clone(), t.get_slot(), t.is_unref())
        };
        let spc = match addr.get_space() {
            Some(s) => s.clone(),
            None => continue,
        };
        let mut off = addr.get_offset();
        let isspacebase = spc.get_type() == spacetype::IPTR_SPACEBASE;
        if isspacebase {
            // Translate the parameter address relative to the caller's spacebase.
            off = spc.wrap_offset(stackoffset.wrapping_add(off));
        }
        let vn: VarnodeId = if is_unref {
            // recovered unreferenced address as part of prototype: create the vn.
            data.new_varnode(sz, &Address::new(spc.clone(), off), None)
        } else {
            let cur = match data.obank().get(op).and_then(|o| o.get_in(slot)) {
                Some(v) => v,
                None => continue,
            };
            let cur_size = data.vbank().get(cur).map(|v| v.get_size()).unwrap_or(sz);
            if cur_size > sz {
                // Varnode bigger than type: create a truncating SUBPIECE.
                let cur_addr = data.vbank().get(cur).map(|v| v.get_addr().clone());
                let cur_addr = match cur_addr {
                    Some(a) => a,
                    None => continue,
                };
                let opaddr = data.obank().get(op).expect("buildInput: stale op").get_addr().clone();
                let newop = data.new_op(2, opaddr);
                let outaddr = if big_endian {
                    &cur_addr + ((cur_size - sz) as i64)
                } else {
                    cur_addr
                };
                let outvn = match data.new_varnode_out(sz, &outaddr, newop) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                data.op_set_opcode_code(newop, OpCode::CPUI_SUBPIECE);
                let _ = data.op_set_input(newop, cur, 0);
                let zero = data.new_constant(1, 0);
                let _ = data.op_set_input(newop, zero, 1);
                data.op_insert_before(newop, op);
                outvn
            } else {
                cur
            }
        };
        newparam.push(vn);
        // Mark the stack range used to pass this parameter as unmapped.
        if isspacebase {
            data.scope_local_mark_not_mapped(&spc, off, sz, true);
        }
    }
    // (kuna) `calleearityfwd` / `calleearitylive`: capture the retry candidate
    // while the trials and the CALL's pre-rewrite inputs are both still there.
    let pending = if newparam.len() < 2 {
        PendingCallFixup {
            rescue: crate::p4_calls::kuna_calleearityfwd::capture_empty_call(fc, data),
            extend: None,
        }
    } else {
        PendingCallFixup {
            rescue: None,
            extend: crate::p4_calls::kuna_calleearitylive::capture_partial_call(fc, data),
        }
    };
    let _ = data.op_set_all_input(op, &newparam);
    // (kuna) `calleearity`: remember WHERE each recovered argument lived before
    // the trials are dropped -- `newparam` holds values, not locations.
    let storage: Vec<(Address, int4)> = (0..fc.get_active_input().get_num_trials())
        .filter_map(|i| {
            let t = fc.get_active_input().get_trial(i);
            t.is_used().then(|| (t.get_address().clone(), t.get_size()))
        })
        .collect();
    fc.set_final_input_storage(storage);
    fc.get_active_input().delete_unused_trials();
    pending
}

/// C++ `FuncCallSpecs::collectOutputTrialVarnodes` (`fspec.cc:5543`).
///
/// Walk the INDIRECT ops immediately before the CALL collecting the Varnodes
/// that match the output trials (one per trial slot).
fn collect_output_trial_varnodes(fc: &mut FuncCallSpecs, data: &mut Funcdata) -> Vec<Option<VarnodeId>> {
    let op = fc.get_op();
    let num_trials = fc.get_active_output().get_num_trials();
    let mut trialvn: Vec<Option<VarnodeId>> = vec![None; num_trials as usize];

    let mut indop = data.op_previous_op(op);
    while let Some(io) = indop {
        let code = match data.obank().get(io) {
            Some(o) => o.code(),
            None => break,
        };
        if code != OpCode::CPUI_INDIRECT {
            break;
        }
        let is_creation = data.obank().get(io).map(|o| o.is_indirect_creation()).unwrap_or(false);
        if is_creation {
            if let Some(vn) = data.obank().get(io).and_then(|o| o.get_out()) {
                let (vaddr, vsize) = {
                    let v = data.vbank().get(vn);
                    match v {
                        Some(v) => (v.get_addr().clone(), v.get_size()),
                        None => {
                            indop = data.op_previous_op(io);
                            continue;
                        }
                    }
                };
                let index = fc.get_active_output().which_trial(&vaddr, vsize);
                if index >= 0 {
                    trialvn[index as usize] = Some(vn);
                    // the exact varnode may have changed, reset the trial address
                    fc.get_active_output().get_trial_mut(index).set_address(vaddr, vsize);
                }
            }
        }
        indop = data.op_previous_op(io);
    }
    trialvn
}

/// C++ `FuncCallSpecs::checkOutputTrialUse` (`fspec.cc:5668`).
///
/// Collect the output-trial Varnodes, then mark each trial active iff its
/// Varnode is the first occurrence read after the call (`ancestorOpUse`-free
/// here: the C++ uses the loneDescend/forward chain — transcribed below).
pub fn check_output_trial_use(fc: &mut FuncCallSpecs, data: &mut Funcdata) -> Vec<Option<VarnodeId>> {
    let trialvn = collect_output_trial_varnodes(fc, data);
    // The location is either used or not.  If it is used it can be the official
    // output or a copy — mark it active.  A trial Varnode that was found is used.
    let num_trials = fc.get_active_output().get_num_trials();
    for i in 0..num_trials {
        if trialvn[i as usize].is_some() {
            fc.get_active_output().get_trial_mut(i).mark_active();
        } else {
            fc.get_active_output().get_trial_mut(i).mark_no_use();
        }
    }
    trialvn
}

/// C++ `FuncCallSpecs::buildOutputFromTrials` (`fspec.cc:5777`).
///
/// Move the single active output trial to be the CALL's output Varnode (the
/// two-piece concat and join-space cases are the multi-register return stub —
/// see the inline note); destroy the INDIRECT ops that were holding the trials.
pub fn build_output_from_trials(
    fc: &mut FuncCallSpecs,
    data: &mut Funcdata,
    trialvn: &[Option<VarnodeId>],
) {
    let op = fc.get_op();
    let mut finalvn: Vec<VarnodeId> = Vec::new();
    let num_trials = fc.get_active_output().get_num_trials();
    for i in 0..num_trials {
        if !fc.get_active_output().get_trial(i).is_used() {
            break;
        }
        let slot = fc.get_active_output().get_trial(i).get_slot();
        if let Some(Some(vn)) = trialvn.get((slot - 1) as usize) {
            finalvn.push(*vn);
        }
    }
    fc.get_active_output().delete_unused_trials();
    let n = fc.get_active_output().get_num_trials();
    if n == 0 {
        return; // Nothing is a formal output
    }

    let mut deletedops: Vec<OpId> = Vec::new();
    if n == 1 {
        // A single, properly justified output: move it to the CALL's output.
        let finaloutvn = finalvn[0];
        if let Some(indop) = data.vbank().get(finaloutvn).and_then(|v| v.get_def()) {
            deletedops.push(indop);
        }
        let _ = data.op_set_output(op, finaloutvn);
    } else {
        // (kuna `rustabi`) The two-piece concat (fspec.cc:5813-5853). The blocker
        // the stub below records is stale -- `constructJoinAddress` IS reachable
        // off the merged handle, and the sibling
        // `ActionReturnRecovery::build_return_output` calls it -- but completing
        // the branch changes which calls have an output, so it ships gated. When
        // it fires the CALL gains the join-space pair the model asked for and both
        // halves become SUBPIECEs of it, instead of staying INDIRECT creations
        // that render as locals the function never assigns.
        let entry = fc.get_entry_address().clone();
        if crate::kuna_rustabi::build_call_output_pair(op, data, &finalvn, Some(&entry)) {
            return;
        }
        // STUB(W4 translate-on-handle): leave the trials in place rather than
        // fabricate a malformed concat (mirrors the same stub in
        // ActionReturnRecovery::build_return_output).
        return;
    }

    for dop in deletedops {
        // Destroy the original INDIRECT ops and free their now-orphan inputs
        // (fspec.cc:5857-5866).
        let in0 = data.obank().get(dop).and_then(|o| o.get_in(0));
        let in1 = data.obank().get(dop).and_then(|o| o.get_in(1));
        data.op_destroy(dop);
        if let Some(v) = in0 {
            let _ = data.delete_varnode(v);
        }
        if let Some(v) = in1 {
            let _ = data.delete_varnode(v);
        }
    }
}
