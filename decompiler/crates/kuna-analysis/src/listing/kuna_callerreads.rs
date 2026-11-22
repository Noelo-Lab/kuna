//! (kuna `callerreads`) Where each direct call returns to.
//!
//! A function decompiled alone may return what its callers read after calling
//! it (`kuna_decomp::kuna_callerreads`). The walk already files every direct
//! call as a [`RefKind::Call`] edge, so this reads the completed Listing and
//! pairs each edge's target with the address the calling instruction falls
//! through to. The decompile step decodes the callers from those addresses only
//! for a function it recovered `void`.

use super::model::RefKind;
use super::Listing;

/// Every direct call of the walk as `(callee entry, return address)`, sorted
/// and de-duplicated.
pub fn call_returns(listing: &Listing) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    for from in listing.ref_source_iter() {
        let Some(back) = listing.instruction_at(from).and_then(|i| i.fall_through) else { continue };
        for r in listing.refs_from(from) {
            if r.kind == RefKind::Call {
                out.push((r.to, back));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Direct calls reached by the raw Thumb partition walk. `PARTITION_ONLY`
/// retains control targets and fall-throughs, but does not build references.
/// A conditional call's continuation can also be reached without the call.
/// Shared code cannot establish which caller's byte bound applies.
pub(super) fn raw_call_returns(listing: &Listing) -> Vec<(u64, u64, u64)> {
    use super::model::FlowKind;

    let owners = raw_owners(listing);

    let mut out = Vec::new();
    for (_, insn) in listing.instructions() {
        if insn.flow.kind != FlowKind::Call || insn.flow.is_conditional {
            continue;
        }
        if let (Some(back), [callee]) = (insn.fall_through, insn.flows.as_slice()) {
            if listing.function_at(*callee).is_none() {
                continue;
            }
            if let Some(&Some(caller)) = owners.get(&insn.addr) {
                if owners.get(&back) == Some(&Some(caller)) {
                    out.push((*callee, back, caller));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn raw_owners(listing: &Listing) -> kuna_decomp::kuna_fasthash::FxHashMap<u64, Option<u64>> {
    use std::collections::hash_map::Entry;

    let mut owners = kuna_decomp::kuna_fasthash::FxHashMap::default();
    let mut pending = Vec::new();
    for (&entry, _) in listing.functions() {
        pending.push((entry, Some(entry)));
        while let Some((vma, owner)) = pending.pop() {
            let Some(insn) = listing.instruction_at(vma) else { continue };
            let owner = match owners.entry(vma) {
                Entry::Vacant(slot) => {
                    slot.insert(owner);
                    owner
                }
                Entry::Occupied(mut slot) => {
                    if slot.get().is_none() || *slot.get() == owner {
                        continue;
                    }
                    // After a second owner reaches a node, all its successors
                    // are ambiguous too. Each instruction is visited at most twice.
                    slot.insert(None);
                    None
                }
            };
            pending.extend(insn.fall_through.map(|next| (next, owner)));
            if !insn.flow.is_call {
                pending.extend(insn.flows.iter().map(|&next| (next, owner)));
            }
        }
    }
    owners
}
