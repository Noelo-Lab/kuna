//! Tail-call function-entry recovery (`tailcallentry`) — the Listing walk's
//! missing notion of a tail call.
//!
//! # The gap
//!
//! [`super::walk`] makes a new function entry at a CALL target and treats every
//! other flow target as a *same-function successor* (`walk.rs` "Branch target →
//! same-function successor"). A routine reached only by a tail `B` is therefore
//! absorbed into whichever function branched to it and never becomes a function
//! of its own: on the ARM Cortex-M corpus that is 513 missed entries, 17.4% of
//! the whole ARM entry-recall gap (`docs/features/arm-entry-granularity/`).
//!
//! # Why this is a post-walk pass and not a change to the walk
//!
//! Splitting at a tail call **cannot change which instructions the walk
//! decodes**. Write the walk as its closure: `I` is the decoded-instruction set,
//! `F` the function-entry set; every `e ∈ F` is decoded, every non-call flow
//! target of an `i ∈ I` joins `I`, every call target joins `F`. Moving a
//! tail-call target from `I` to `F` leaves the closure fixed, because `F ⊆ I` —
//! a function entry is walked, so it is decoded either way. The split is
//! therefore purely additive on `F`, and computing it *after* the walk gives the
//! model complete predecessor and region information instead of whatever the
//! worklist order happened to expose.
//!
//! That is what makes "this option can never remove an entry" a property of the
//! wiring rather than of a heuristic: `passes.rs` never rebuilds the Listing, so
//! `funcdisc_recursive`, `aif` and every other consumer see byte-identical
//! input.
//!
//! # The precision model: region containment
//!
//! The naive rule — split at every unconditional-branch target whose predecessor
//! ends the flow — measures 39% precision and splits 55% of the time, because it
//! cannot tell a tail call from a rotated loop head. Four guards, each measured
//! on the corpus (`docs/features/tailcallentry/`), take that to 94.6% with zero
//! split bodies:
//!
//! 1. [`Guard::ImpurePredecessor`] — every predecessor of the target must be an
//!    unconditional branch. A fall-through or conditional-branch predecessor
//!    means the caller's straight-line code runs into the target, which is
//!    ordinary intra-function flow.
//! 2. [`Guard::InCallerRegion`] — the branch must leave the caller's
//!    entry-ordered region ([`Listing::function_containing`]), i.e. at least one
//!    *other* discovered function entry lies between the branch and its target.
//!    This is the containment fact, and the ablation says it is the load-bearing
//!    one: dropping it costs 43 points of precision (94.6% → 51.5%) and splits
//!    514 real bodies.
//! 3. [`Guard::NotTerminating`] — the target's flow region must reach a `RETURN`
//!    or a computed jump. Terminating-routine validity, without the length floor
//!    a one-instruction `bx lr` handler cannot pass.
//! 4. [`Guard::EpilogueStart`] — the target must not open with a stack restore.
//!    A function does not begin by tearing down a frame it never built; such a
//!    target is the caller's shared epilogue.
//!
//! ARM-gated like the sibling Stage-2/3 seed scans, so every non-ARM binary is a
//! strict no-op (the mechanism is architecture-neutral; the evidence is not).

use std::collections::{BTreeMap, BTreeSet};

use super::{FlowKind, Insn, Listing, RefKind};

/// Instruction cap on a speculative region closure (`followSubFlows`' 4000).
const MAX_REGION_INSNS: usize = 4000;

/// Why a candidate tail-call target was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
    /// Some predecessor of the target is not an unconditional branch.
    ImpurePredecessor,
    /// Some branch site does not leave its own entry-ordered function region.
    InCallerRegion,
    /// The target's flow region never reaches a `RETURN` / computed jump.
    NotTerminating,
    /// The target opens with a stack restore (the caller's shared epilogue).
    EpilogueStart,
}

/// The accepted tail-call function entries, address-sorted.
///
/// ARM-only (the corpus the containment model was measured on); a strict no-op
/// on every other architecture.
pub fn tail_call_entries(file: &object::File, listing: &Listing) -> Vec<u64> {
    use object::read::Object;
    if file.architecture() != object::Architecture::Arm {
        return Vec::new();
    }
    let entries: Vec<u64> = listing.functions().map(|(&a, _)| a).collect();
    branch_targets(listing, &entries)
        .into_iter()
        .filter(|(target, srcs)| probe(listing, &entries, *target, srcs).is_ok())
        .map(|(target, _)| target)
        .collect()
}

/// Every unconditional-branch target that is a decoded instruction and not
/// already a function entry, keyed by target with all of its branch sites.
fn branch_targets(listing: &Listing, entries: &[u64]) -> BTreeMap<u64, Vec<u64>> {
    let known: BTreeSet<u64> = entries.iter().copied().collect();
    let mut out: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for (&vma, insn) in listing.instructions() {
        if insn.flow.kind != FlowKind::UnconditionalBranch {
            continue;
        }
        for &t in &insn.flows {
            if t != vma && !known.contains(&t) && listing.is_instruction_start(t) {
                out.entry(t).or_default().push(vma);
            }
        }
    }
    out
}

/// The whole acceptance predicate, returning the [`Guard`] that rejected.
fn probe(
    listing: &Listing,
    entries: &[u64],
    target: u64,
    srcs: &[u64],
) -> Result<(), Guard> {
    if !predecessors_are_branches(listing, target) {
        return Err(Guard::ImpurePredecessor);
    }
    if srcs.iter().any(|&src| in_caller_region(entries, src, target)) {
        return Err(Guard::InCallerRegion);
    }
    if listing.instruction_at(target).is_some_and(|i| restores_frame(i)) {
        return Err(Guard::EpilogueStart);
    }
    if !terminates(listing, target) {
        return Err(Guard::NotTerminating);
    }
    Ok(())
}

/// True iff every reference into `target` is an unconditional branch — no
/// fall-through and no conditional-branch predecessor.
fn predecessors_are_branches(listing: &Listing, target: u64) -> bool {
    listing.refs_to(target).iter().all(|r| {
        r.kind == RefKind::Code
            && listing.instruction_at(r.from).is_some_and(|i| {
                i.fall_through != Some(target)
                    && (!i.flows.contains(&target)
                        || i.flow.kind == FlowKind::UnconditionalBranch)
            })
    })
}

/// True iff `target` lies inside the entry-ordered region of the function
/// containing `src` — the branch crosses no discovered function boundary.
fn in_caller_region(entries: &[u64], src: u64, target: u64) -> bool {
    let Some(caller) = region_of(entries, src) else { return true };
    target > caller && next_entry(entries, caller).is_none_or(|hi| target < hi)
}

/// The greatest function entry `<= vma` (the entry-ordered region key, the same
/// granularity [`Listing::function_containing`] reads).
fn region_of(entries: &[u64], vma: u64) -> Option<u64> {
    match entries.binary_search(&vma) {
        Ok(i) => Some(entries[i]),
        Err(0) => None,
        Err(i) => Some(entries[i - 1]),
    }
}

fn next_entry(entries: &[u64], entry: u64) -> Option<u64> {
    match entries.binary_search(&entry) {
        Ok(i) => entries.get(i + 1).copied(),
        Err(i) => entries.get(i).copied(),
    }
}

/// True iff the flow region reachable from `target` reaches a `RETURN` or a
/// computed jump — terminating-routine validity, with no length floor.
fn terminates(listing: &Listing, target: u64) -> bool {
    let mut seen = BTreeSet::new();
    let mut work = vec![target];
    while let Some(vma) = work.pop() {
        if !seen.insert(vma) || seen.len() >= MAX_REGION_INSNS {
            continue;
        }
        let Some(i) = listing.instruction_at(vma) else { continue };
        if matches!(i.flow.kind, FlowKind::Return | FlowKind::ComputedJump) {
            return true;
        }
        if !i.flow.is_call {
            work.extend(i.flows.iter().copied());
        }
        work.extend(i.fall_through);
    }
    false
}

/// True iff the instruction restores the stack frame (`POP` / `VPOP` /
/// `LDM SP!` / `ADD SP`) — an epilogue, never a function start.
fn restores_frame(i: &Insn) -> bool {
    let m = i.mnemonic.to_ascii_lowercase();
    let o = i.operands.to_ascii_lowercase();
    match m.split('.').next().unwrap_or("") {
        "pop" | "vpop" => true,
        "ldmia" | "ldm" | "ldmfd" => o.starts_with("sp!"),
        "add" | "adds" => o.starts_with("sp"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entries() -> Vec<u64> {
        vec![0x100, 0x200, 0x300]
    }

    #[test]
    fn region_lookup_is_entry_ordered() {
        let e = entries();
        assert_eq!(region_of(&e, 0x0ff), None);
        assert_eq!(region_of(&e, 0x100), Some(0x100));
        assert_eq!(region_of(&e, 0x1ff), Some(0x100));
        assert_eq!(region_of(&e, 0x300), Some(0x300));
        assert_eq!(next_entry(&e, 0x100), Some(0x200));
        assert_eq!(next_entry(&e, 0x300), None);
    }

    /// The containment guard: a branch that stays inside its own region is
    /// intra-function flow; one that crosses a discovered entry is a tail call.
    #[test]
    fn containment_is_the_region_crossing() {
        let e = entries();
        assert!(in_caller_region(&e, 0x110, 0x180), "same region ⇒ not a tail call");
        assert!(!in_caller_region(&e, 0x110, 0x280), "crosses 0x200 ⇒ tail call");
        assert!(!in_caller_region(&e, 0x310, 0x180), "backward across entries");
        assert!(in_caller_region(&e, 0x310, 0x380), "last region has no upper bound");
        assert!(in_caller_region(&e, 0x010, 0x080), "no containing entry ⇒ reject");
    }

    fn insn(mnemonic: &str, operands: &str) -> Insn {
        Insn {
            addr: 0,
            len: 2,
            fall_through: None,
            flow: Default::default(),
            flows: Vec::new(),
            mnemonic: mnemonic.into(),
            operands: operands.into(),
            pcode: None,
        }
    }

    #[test]
    fn epilogue_starts_are_recognised() {
        assert!(restores_frame(&insn("pop", "{r4,pc}")));
        assert!(restores_frame(&insn("pop.w", "{r4,r5,lr}")));
        assert!(restores_frame(&insn("ldmia.w", "sp!,{r4,pc}")));
        assert!(restores_frame(&insn("add", "sp,#0x14")));
        assert!(!restores_frame(&insn("push", "{r4,lr}")));
        assert!(!restores_frame(&insn("add", "r0,r1")));
        assert!(!restores_frame(&insn("ldr", "r0,[sp,#4]")));
        assert!(!restores_frame(&insn("ldmia", "r3!,{r0,r1}")));
    }
}
