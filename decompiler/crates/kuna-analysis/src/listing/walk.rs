//! The recursive-descent disassembly driver (design §3).
//!
//! Two-level worklist, mirroring `FlowInfo`'s `VisitStat`/`addrlist` *design*
//! (`flow.rs`) but lightweight (no `Funcdata`, no banks, no blocks): an outer
//! **function worklist** (CALL targets become new function entries) and an inner
//! per-function **instruction worklist** (branch/fall-through successors).
//!
//! The walk is the program-wide extension `FlowInfo` lacks: `FlowInfo` stops at
//! RETURN, treats CALL as fall-through, and never recurses into callees; here a
//! CALL/CALLIND *direct* target seeds a new function entry. Indirect targets
//! (BRANCHIND/CALLIND) are recorded with the computed/indirect predicates but
//! contribute NO static successor (deferred jump-table resolution, design §8).
//!
//! Termination: `visited_funcs` bounds the function worklist and `insns`
//! membership bounds the instruction worklist; both are monotonic over a finite
//! address universe.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

use super::classify::classify;
use super::context::ContextPainter;
use super::decode::decode_one;
use super::model::{DiscoveredFunction, Insn, Reference, RefKind};
use super::ListingDetail;

/// The accumulating maps the walk fills. Lifted into [`super::Listing`] by
/// [`super::Listing::build`].
pub(super) struct WalkState {
    /// Instruction model, keyed by VMA.
    pub insns: BTreeMap<u64, Insn>,
    /// Incoming xref edges (callers / branch sources), keyed by target VMA.
    pub refs_to: BTreeMap<u64, Vec<Reference>>,
    /// Outgoing xref edges, keyed by source VMA.
    pub refs_from: BTreeMap<u64, Vec<Reference>>,
    /// Discovered/seeded functions, keyed by entry VMA (ordered).
    pub funcs: BTreeMap<u64, DiscoveredFunction>,
    /// Plausible x86 `PUSH imm` callback evidence, keyed by target and bounded
    /// during the walk. The value is the lowest source address for that target.
    pub stack_callback_refs: BTreeMap<u64, u64>,
    /// Stable sampling rank for each retained target. This prevents the bounded
    /// model from systematically starving callbacks in a high-address section.
    stack_callback_ranks: BTreeSet<(u64, u64)>,
    /// Whether [`WalkState::file_ref`] records anything at all.
    want_refs: bool,
}

impl WalkState {
    fn new(want_refs: bool) -> Self {
        WalkState {
            insns: BTreeMap::new(),
            refs_to: BTreeMap::new(),
            refs_from: BTreeMap::new(),
            funcs: BTreeMap::new(),
            stack_callback_refs: BTreeMap::new(),
            stack_callback_ranks: BTreeSet::new(),
            want_refs,
        }
    }

    /// File a reference into both directions (`refs_to[to]` and `refs_from[from]`).
    ///
    /// A no-op when the caller asked for no reference model, so the walk's edge
    /// sites read the same either way and a new one cannot miss the gate.
    fn file_ref(&mut self, from: u64, to: u64, kind: RefKind) {
        if !self.want_refs {
            return;
        }
        let r = Reference { from, to, kind, op_index: None };
        self.refs_to.entry(to).or_default().push(r.clone());
        self.refs_from.entry(from).or_default().push(r);
    }

    /// Record one already-proven `PUSH imm` site without letting a large image
    /// accumulate an unbounded side model. Keeping the lowest source per target
    /// deduplicates before storage. A stable hash rank samples across the address
    /// space instead of systematically starving high-address sections, and makes
    /// the cap deterministic regardless of recursive-descent worklist order.
    fn file_stack_callback_ref(&mut self, source: u64, target: u64) {
        if let Some(prior) = self.stack_callback_refs.get_mut(&target) {
            *prior = (*prior).min(source);
            return;
        }
        let rank = callback_rank(target);
        self.stack_callback_refs.insert(target, source);
        self.stack_callback_ranks.insert((rank, target));
        if self.stack_callback_refs.len() > super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE {
            if let Some(worst) = self.stack_callback_ranks.pop_last() {
                self.stack_callback_refs.remove(&worst.1);
            }
        }
    }
}

/// SplitMix64's finalizer gives every target a stable, inexpensive sampling
/// rank. This is not randomness: the same evidence set always retains the same
/// targets, independent of discovery order.
fn callback_rank(mut target: u64) -> u64 {
    target = (target ^ (target >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    target = (target ^ (target >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    target ^ (target >> 31)
}

/// True if `vma` lands inside any executable range `[lo, hi)`.
pub(super) fn in_exec(exec_ranges: &[(u64, u64)], vma: u64) -> bool {
    exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}

/// Run the two-level recursive-descent walk.
///
/// `seeds` is the function-entry root set the caller supplies (already
/// exec-filtered/deduped — design §3.1). `seed_funcs` carries the
/// [`DiscoveredFunction`] metadata for each seed (name / from_symbol); seeds
/// without an entry there get a generic discovered record. `exec_ranges` bounds
/// every decode (out-of-bounds = stop-this-path). `code_space` is the space the
/// `Address`es are built in. `arch` exposes the engine's `ContextDatabase`, into
/// which `painter` paints the per-address decode mode (ARM `TMode` / MIPS
/// `ISA_MODE`) **before** any [`decode_one`] runs (design §4.2 / PR5) — without
/// it a Thumb/MIPS16 function misdecodes as A32/MIPS32. On x86-64 (no decode-mode
/// context) the painter is empty and this is a no-op.
///
/// `local_entries` is the PPC64 ELFv2 local-entry fold (`ppclocalentry`), keyed
/// by the local entry VMA: a CALL landing on one of those is a call into the
/// INTERIOR of the function at its value, so no function is claimed there. Empty
/// on every other architecture and whenever the option is off.
///
/// `detail.assembly` is forwarded to [`decode_one`]: `false` leaves every
/// [`Insn::mnemonic`]/[`Insn::operands`] empty and skips the second SLEIGH parse
/// that produces them (see [`decode_one`] for the cost).
///
/// `detail.refs` selects whether the reference model is filed at all
/// ([`WalkState::file_ref`] becomes a no-op). Every instruction contributes an
/// edge per successor, a fall-through included, so on a large program this is
/// the same order of magnitude as the instruction model itself; `false` leaves
/// both maps empty for a caller that reads neither.
pub(super) fn walk(
    translate: &dyn Translate,
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    seeds: &[u64],
    seed_funcs: &BTreeMap<u64, DiscoveredFunction>,
    painter: &ContextPainter,
    local_entries: &BTreeMap<u64, u64>,
    detail: ListingDetail,
    want_stack_callbacks: bool,
) -> WalkState {
    // Paint the decode-mode context (ARM TMode / MIPS ISA_MODE) into the engine's
    // ContextDatabase BEFORE we decode a single instruction — the timing the
    // alternate ISAs require (the same ordering `commit_analysis_output` uses).
    // `set_variable` fills each mode from its marker up to the next change point,
    // so painting once here covers every address the walk visits. A no-op when
    // `painter` is empty (x86-64 / any language with no decode-mode context).
    if !painter.is_empty() {
        painter.paint_all(arch, code_space);
    }

    let mut st = WalkState::new(detail.refs);

    // Function-entry worklist, seeded from the root set.
    let mut func_worklist: Vec<u64> = seeds.to_vec();
    let mut visited_funcs: BTreeSet<u64> = BTreeSet::new();

    // Pre-populate the function model with the seed metadata so a seeded entry
    // keeps its name/from_symbol even if a later CALL also targets it.
    for &entry in seeds {
        let df = seed_funcs.get(&entry).cloned().unwrap_or_else(|| discovered(entry));
        st.funcs.entry(entry).or_insert(df);
    }

    while let Some(entry) = func_worklist.pop() {
        if !visited_funcs.insert(entry) {
            continue; // already walked this function
        }

        // Per-function instruction worklist.
        let mut insn_worklist: Vec<u64> = vec![entry];
        while let Some(vma) = insn_worklist.pop() {
            if st.insns.contains_key(&vma) {
                continue; // the VisitStat dedup (overlap detection free)
            }
            if !in_exec(exec_ranges, vma) {
                continue; // out-of-bounds gate (flow.rs:891 analog)
            }

            let decoded = match decode_one(
                translate,
                vma,
                code_space,
                detail.assembly,
                want_stack_callbacks,
            ) {
                Ok(d) => d,
                Err(_) => continue, // decode error: stop this path (mark gap)
            };
            if decoded.len == 0 {
                continue; // zero-length decode would not advance; stop this path
            }

            let c = classify(&decoded.ops, vma, decoded.len);

            let fall_through = vma.wrapping_add(u64::from(decoded.len));
            let stack_values: Vec<u64> = if want_stack_callbacks {
                decoded
                    .stored_scalar_values
                    .iter()
                    .copied()
                    .filter(|&target| target != fall_through && in_exec(exec_ranges, target))
                    .collect()
            } else {
                Vec::new()
            };
            // A constant STORE value is the narrow p-code shape shared by
            // `PUSH imm` and `MOV [mem],imm`. Only that shape earns the on-demand
            // assembly parse; register immediates and absolute LOAD addresses do
            // not trigger a second decode.
            let mnemonic = if !stack_values.is_empty() && decoded.mnemonic.is_empty() {
                super::decode::mnemonic_at(translate, vma, code_space)
            } else {
                decoded.mnemonic
            };
            let is_stack_callback_push = mnemonic.eq_ignore_ascii_case("PUSH");

            st.insns.insert(
                vma,
                Insn {
                    addr: vma,
                    len: decoded.len,
                    fall_through: c.fall_through,
                    flow: c.flow,
                    flows: c.flows.clone(),
                    mnemonic,
                    operands: decoded.operands,
                    pcode: None,
                },
            );

            if is_stack_callback_push {
                for target in stack_values {
                    st.file_stack_callback_ref(vma, target);
                }
            }

            // Successor edges.
            for &t in &c.flows {
                if c.flow.is_call {
                    // CALL/CALLIND direct target → a NEW function entry, but only
                    // where the instruction worklist would agree to decode
                    // (`unmappedentry`) and where the target is not the callee's own
                    // PPC64 ELFv2 local entry (`ppclocalentry` — a point inside a
                    // function whose global entry is already a seed, so the bytes
                    // here are walked either way). The reference is filed whatever
                    // the claim decides (where there is a reference model at all);
                    // only the function claim is withheld.
                    if !local_entries.contains_key(&t)
                        && super::kuna_unmappedentry::admits_call_entry(arch, exec_ranges, t)
                    {
                        st.funcs.entry(t).or_insert_with(|| discovered(t));
                        func_worklist.push(t);
                    }
                    st.file_ref(vma, t, RefKind::Call);
                } else {
                    // Branch target → same-function successor.
                    insn_worklist.push(t);
                    st.file_ref(vma, t, RefKind::Code);
                }
            }
            if let Some(fall) = c.fall_through {
                insn_worklist.push(fall);
                st.file_ref(vma, fall, RefKind::Code);
            }
        }
    }

    st
}

/// A generic discovered (not symbol-seeded) function record at `entry`.
fn discovered(entry: u64) -> DiscoveredFunction {
    DiscoveredFunction {
        entry,
        name: None,
        from_symbol: false,
        has_no_return: false,
        call_fixup: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_evidence_cap_is_deduplicated_and_order_independent() {
        let count = super::super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE + 32;
        let mut forward = WalkState::new(false);
        let mut reverse = WalkState::new(false);
        for target in 0..count as u64 {
            forward.file_stack_callback_ref(0x2000 + target, 0x1000 + target);
        }
        for target in (0..count as u64).rev() {
            reverse.file_stack_callback_ref(0x3000 + target, 0x1000 + target);
        }
        assert_eq!(
            forward.stack_callback_refs.len(),
            super::super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE
        );
        assert_eq!(
            forward.stack_callback_refs.keys().collect::<Vec<_>>(),
            reverse.stack_callback_refs.keys().collect::<Vec<_>>()
        );

        let retained = *forward.stack_callback_refs.keys().next().unwrap();
        forward.file_stack_callback_ref(1, retained);
        assert_eq!(forward.stack_callback_refs[&retained], 1);
    }
}
