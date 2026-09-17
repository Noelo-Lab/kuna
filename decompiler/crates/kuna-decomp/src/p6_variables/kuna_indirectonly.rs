//! (kuna) Mark the function inputs that are read only through INDIRECT ops —
//! the `indirectonly` option.
//!
//! # The gap
//!
//! `varnode_flags::indirectonly` ([`crate::varnode`]) has two readers and, on
//! the merged tree, no writer at all: `ActionMarkIndirectOnly` was scheduled but
//! its body was left a no-op, so every test of `is_indirect_only` answers
//! `false`.  Both readers take the wrong branch when it does:
//!
//!   * [`HighVariable::has_name`](crate::p6_variables::variable::HighVariable::has_name)
//!     refuses to name an unaffected input whose every instance is
//!     indirect-only.  With the flag dead, such an input is named, and the name
//!     is what `emit_local_var_decls` needs to print a declaration — so a
//!     register the function never really uses gets its own `vN;` line.
//!   * [`Merge::merge_test_adjacent`](crate::p6_variables::merge::Merge) refuses
//!     a speculative merge involving an *illegal* input, "UNLESS the illegal
//!     input is only used indirectly".  With the flag dead the exception never
//!     applies and kuna rejects strictly more merges than upstream.
//!
//! # What this does
//!
//! [`mark_indirect_only`] is `Funcdata::markIndirectOnly`
//! (`funcdata_varnode.cc:845-858`): walk the input def-set, and for every
//! *illegal* input — an input Varnode that `ActionDirectWrite` did not reach, so
//! nothing a real parameter feeds can be affected by it — ask
//! [`check_indirect_use`] whether all of its flow ends in INDIRECT ops.  If so,
//! set `indirectonly`.
//!
//! [`check_indirect_use`] is `Funcdata::checkIndirectUse`
//! (`funcdata_varnode.cc:801-844`): a worklist over the descendants.  A reader
//! that is a plain `CPUI_INDIRECT` is the accepting case and the walk stops
//! there; an INDIRECT *from a STORE* is accepted but the walk continues through
//! its output (the value survives the store, so where it ends up still matters);
//! a `CPUI_MULTIEQUAL` is transparent and the walk continues through its output.
//! **Any other reader at all — a real arithmetic use, a compare, a CALL
//! argument, a COPY — fails the whole test**, which is what keeps an input that
//! is genuinely read out of the flag.
//!
//! Upstream uses the Varnode `mark` bit as its visited set and clears it on the
//! way out; this port keeps the visited set local so a bailout cannot leave
//! stale marks behind for a later pass.  The traversal order and the accepted
//! opcode set are unchanged.
//!
//! # Why it is an option, and why it is off
//!
//! Both consumers change what is printed — a declaration disappears, and a
//! speculative merge that was refused can now happen — so the change ships
//! behind `indirectonly`, and `off` (the default) restores the inert stub
//! byte-for-byte.
//!
//! It defaults to *off* although the body is upstream verbatim.  The merge the
//! flag unlocks is sound in one direction and not in the other:
//!
//!   * The illegal input is the copy's **destination** — the machine really
//!     does store into the frame slot — and the merge only moves where the
//!     value is computed.  Safe, and the reason to turn the option on.
//!   * The illegal input is the copy's **source** — the machine loads the slot
//!     into a register and mutates the register — and the merge makes the
//!     emitted C mutate the slot.  If the slot's address escaped, a later call
//!     reads a value the machine never wrote there.
//!
//! The cover machinery cannot separate the two: a CPUI_INDIRECT is only
//! attached where the storage is still live in the SSA, so a slot whose last
//! read is before the loop carries no INDIRECT at any call after it and there
//! is nothing for the cover-intersection test to intersect.  Stock Ghidra
//! 12.1.2 emits the same fabricated store on the same input, so this is an
//! upstream defect the port inherits faithfully rather than a porting error —
//! see `docs/features/indirectonly/counterexample.md`.

use std::collections::HashSet;

use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::varnode::varnode_flags;

/// Check whether the given Varnode only flows into call-based INDIRECT ops
/// (C++ `Funcdata::checkIndirectUse`, `funcdata_varnode.cc:801-844`).
///
/// Flow is only followed through MULTIEQUAL ops and through the output of an
/// INDIRECT that came from a STORE.  Returns `true` when every flow hits an
/// INDIRECT op.
pub fn check_indirect_use(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut vlist: Vec<VarnodeId> = vec![vn];
    let mut mark: HashSet<VarnodeId> = HashSet::new();
    mark.insert(vn);

    let mut i = 0usize;
    let mut result = true;
    while i < vlist.len() && result {
        let cur = vlist[i];
        i += 1;
        let descend: Vec<_> = match data.vbank().get(cur) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for op in descend {
            let (opc, indirect_store, outvn) = match data.obank().get(op) {
                Some(o) => (o.code(), o.is_indirect_store(), o.get_out()),
                None => continue,
            };
            if opc == OpCode::CPUI_INDIRECT {
                if indirect_store {
                    // INDIRECT from a STORE is not a negative result but
                    // continue to follow data-flow.
                    if let Some(out) = outvn {
                        if mark.insert(out) {
                            vlist.push(out);
                        }
                    }
                }
            } else if opc == OpCode::CPUI_MULTIEQUAL {
                if let Some(out) = outvn {
                    if mark.insert(out) {
                        vlist.push(out);
                    }
                }
            } else {
                result = false;
                break;
            }
        }
    }
    result
}

/// Mark the illegal input Varnodes that are used only in INDIRECTs
/// (C++ `Funcdata::markIndirectOnly`, `funcdata_varnode.cc:845-858`).
///
/// Returns the number of Varnodes newly flagged, for the action's change count.
pub fn mark_indirect_only(data: &mut Funcdata) -> u32 {
    let inputs: Vec<VarnodeId> = data.vbank().iter_def_flag(varnode_flags::input).collect();
    let mut hits: Vec<VarnodeId> = Vec::new();
    for vn in inputs {
        // Only check illegal inputs.
        let illegal = data.vbank().get(vn).map(|v| v.is_illegal_input()).unwrap_or(false);
        if !illegal {
            continue;
        }
        if check_indirect_use(data, vn) {
            hits.push(vn);
        }
    }
    let count = hits.len() as u32;
    for vn in hits {
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.set_flags_pub(varnode_flags::indirectonly);
        }
    }
    count
}

/// The `indirectonly` option (`on|off`).
pub struct OptionIndirectOnly;

impl OptionIndirectOnly {
    /// The option name.
    pub const NAME: &'static str = "indirectonly";

    /// Parse `on|off` and return the resolved flag + confirmation message.  The
    /// caller writes the flag into `Architecture::mark_indirect_only`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Marking of indirect-only inputs turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_indirectonly/tests.rs"]
mod tests;
