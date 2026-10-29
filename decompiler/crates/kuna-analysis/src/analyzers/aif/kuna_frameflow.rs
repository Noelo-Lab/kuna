//! A recovered frame root that the frame-expanded Listing enters by
//! straight-line flow is the continuation of the code before it, not a function.
//! A shrink-wrapped routine saves LR a few instructions after its real entry;
//! once a caller reaches that entry, its fall-through runs into the `push`.

use std::collections::BTreeSet;

use crate::listing::{FlowKind, Listing, RefKind};

/// How far back a straight-line run is traced before the root is kept.
const MAX_RUN_INSNS: usize = 256;

/// The `roots` the instruction immediately before enters by fall-through or a
/// conditional skip, on a run that begins at a function entry or a branch
/// target. A run that begins at a call's fall-through is speculative: the callee
/// may not return, and a literal pool after a no-return call decodes as such a
/// run into the next routine.
pub(crate) fn entered(listing: &Listing, roots: &BTreeSet<u64>) -> BTreeSet<u64> {
    roots
        .iter()
        .copied()
        .filter(|&root| {
            listing.instruction_before(root).is_some_and(|insn| {
                insn.addr.saturating_add(u64::from(insn.len)) == root
                    && !insn.flow.is_call
                    && (insn.fall_through == Some(root)
                        || (insn.flows.contains(&root) && insn.flow.kind != FlowKind::UnconditionalBranch))
                    && reached(listing, insn.addr)
            })
        })
        .collect()
}

/// True iff the straight-line run ending at `at` starts at a function entry or
/// a branch target rather than after a call.
fn reached(listing: &Listing, mut at: u64) -> bool {
    for _ in 0..MAX_RUN_INSNS {
        if listing.function_at(at).is_some() || branched_to(listing, at) {
            return true;
        }
        let Some(prev) = listing.instruction_before(at).filter(|prev| {
            prev.addr.saturating_add(u64::from(prev.len)) == at && prev.fall_through == Some(at)
        }) else {
            return true;
        };
        if prev.flow.is_call {
            return false;
        }
        at = prev.addr;
    }
    false
}

/// True iff code other than the fall-through before `at` flows to it.
fn branched_to(listing: &Listing, at: u64) -> bool {
    listing.refs_to(at).iter().any(|r| {
        r.kind == RefKind::Code
            && listing.instruction_at(r.from).is_some_and(|insn| insn.fall_through != Some(at))
    })
}
