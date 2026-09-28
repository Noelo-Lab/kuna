//! Mark illegal input Varnodes whose uses terminate in INDIRECT operations.
//!
//! Plain INDIRECT readers end a branch. The walk follows MULTIEQUAL and
//! STORE-induced INDIRECT outputs; other readers reject the input. A local
//! visited set handles cycles without leaving Varnode marks on rejection.
//!
//! `indirectonly` remains off by default. The flag can permit a speculative
//! merge that fabricates a store when an escaped frame slot is the copy's
//! source. Destination-side merges can be sound, but this pass does not
//! distinguish those cases. See `docs/features/indirectonly/counterexample.md`.

#[expect(clippy::disallowed_types, reason = "Membership only; the Vec worklist determines traversal order.")]
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
    #[expect(clippy::disallowed_types, reason = "Only insert is used; set iteration cannot affect output.")]
    let mut mark: HashSet<VarnodeId> = HashSet::new();
    mark.insert(vn);

    let mut i = 0usize;
    let mut result = true;
    while i < vlist.len() && result {
        let cur = vlist[i];
        i += 1;
        let Some(v) = data.vbank().get(cur) else { continue };
        for op in v.descend_iter() {
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
