//! (kuna `--assert callee`) A caller-stated destination for an indirect call.
//!
//! The front-ends park `call site -> callee entry` facts on the
//! [`Architecture`], keyed by the entry of the function holding the call.
//! [`seed`] copies a function's facts into its fresh `Funcdata`'s `Override`
//! before flow follows, where `FlowInfo::setup_callind_specs` turns each named
//! `CALLIND` into a direct `CALL` to the callee and copies the callee's
//! prototype onto it. A named indirect jump is a tail call: the flow skips
//! jump-table recovery for it ([`is_stated`]) and builds the call followed by a
//! return. [`audit`] then records every fact the followed flow did
//! not turn into such a call, so a front-end can report it as refused.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

/// Why [`audit`] found no direct call to the stated callee at a call site.
pub const NOT_APPLIED: &str = "the function's flow makes no indirect call at this address";

/// Seed `fd`'s indirect-call overrides from the facts parked for its entry.
pub fn seed(arch: &Architecture, fd: &mut Funcdata) {
    let Some(sites) = arch.kuna_callee_overrides.get(fd.get_address()) else {
        return;
    };
    for (site, callee) in sites {
        fd.get_override_mut().insert_indirect_override(site.clone(), callee.clone());
    }
}

/// Is a fact parked for the instruction at `site` of `fd`?
pub fn is_stated(arch: &Architecture, fd: &Funcdata, site: &Address) -> bool {
    arch.kuna_callee_overrides
        .get(fd.get_address())
        .is_some_and(|sites| sites.contains_key(site))
}

/// Record on `fd` each parked fact its followed flow did not apply.
pub fn audit(arch: &Architecture, fd: &mut Funcdata) {
    let Some(sites) = arch.kuna_callee_overrides.get(fd.get_address()) else {
        return;
    };
    let unapplied: Vec<(Address, Address)> = sites
        .iter()
        .filter(|(site, callee)| !calls_directly(fd, site, callee))
        .map(|(site, callee)| (site.clone(), callee.clone()))
        .collect();
    for (site, callee) in unapplied {
        fd.note_rejected_callee_override(site, callee, NOT_APPLIED);
    }
}

fn calls_directly(fd: &Funcdata, site: &Address, callee: &Address) -> bool {
    (0..fd.num_calls()).any(|i| {
        let spec = fd.get_call_specs(i);
        (spec.get_entry_address() == callee || spec.plt_stub() == Some(callee))
            && fd.obank().get(spec.get_op()).is_some_and(|op| op.get_addr() == site)
    })
}
