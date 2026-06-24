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

use kuna_base::address::RangeList;
use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use super::classify::classify;
use super::decode::decode_one;
use super::model::{DiscoveredFunction, Insn, Reference, RefKind};

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
    /// Instruction-byte coverage (`[vma, vma+len-1]` per decoded insn).
    pub covered: RangeList,
}

impl WalkState {
    fn new() -> Self {
        WalkState {
            insns: BTreeMap::new(),
            refs_to: BTreeMap::new(),
            refs_from: BTreeMap::new(),
            funcs: BTreeMap::new(),
            covered: RangeList::new(),
        }
    }

    /// File a reference into both directions (`refs_to[to]` and `refs_from[from]`).
    fn file_ref(&mut self, from: u64, to: u64, kind: RefKind) {
        let r = Reference { from, to, kind, op_index: None };
        self.refs_to.entry(to).or_default().push(r.clone());
        self.refs_from.entry(from).or_default().push(r);
    }
}

/// True if `vma` lands inside any executable range `[lo, hi)`.
fn in_exec(exec_ranges: &[(u64, u64)], vma: u64) -> bool {
    exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}

/// Run the two-level recursive-descent walk.
///
/// `seeds` is the function-entry root set the caller supplies (already
/// exec-filtered/deduped — design §3.1). `seed_funcs` carries the
/// [`DiscoveredFunction`] metadata for each seed (name / from_symbol); seeds
/// without an entry there get a generic discovered record. `exec_ranges` bounds
/// every decode (out-of-bounds = stop-this-path). `code_space` is the space the
/// `Address`es are built in.
pub(super) fn walk(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    seeds: &[u64],
    seed_funcs: &BTreeMap<u64, DiscoveredFunction>,
) -> WalkState {
    let mut st = WalkState::new();

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

            let decoded = match decode_one(translate, vma, code_space) {
                Ok(d) => d,
                Err(_) => continue, // decode error: stop this path (mark gap)
            };
            if decoded.len == 0 {
                continue; // zero-length decode would not advance; stop this path
            }

            let c = classify(&decoded.ops, vma, decoded.len);

            st.insns.insert(
                vma,
                Insn {
                    addr: vma,
                    len: decoded.len,
                    fall_through: c.fall_through,
                    flow: c.flow,
                    flows: c.flows.clone(),
                    mnemonic: decoded.mnemonic,
                    pcode: None,
                },
            );
            st.covered.insert_range(Rc::clone(code_space), vma, vma + decoded.len as u64 - 1);

            // Successor edges.
            for &t in &c.flows {
                if c.flow.is_call {
                    // CALL/CALLIND direct target → a NEW function entry.
                    st.funcs.entry(t).or_insert_with(|| discovered(t));
                    func_worklist.push(t);
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
