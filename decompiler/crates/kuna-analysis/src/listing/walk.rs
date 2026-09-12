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
use super::model::{DiscoveredFunction, Insn, InsnLite, Reference, RefKind};
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
}

/// Where [`step`] files a decoded instruction, and the visit dedup it asks.
///
/// The serial walk's sink is its global `BTreeMap<u64, Insn>`; a sink that
/// collects [`InsnLite`] records for a later merge is the other shape, which is
/// why the record handed over is the `Send` one and the conversion to [`Insn`]
/// belongs to the sink.
pub(super) trait InsnSink {
    /// Has this address already been decoded?
    fn decoded(&self, vma: u64) -> bool;
    /// File the decoded record.
    fn record(&mut self, insn: InsnLite);
}

impl InsnSink for BTreeMap<u64, Insn> {
    fn decoded(&self, vma: u64) -> bool {
        // INVARIANT (first-writer-wins): whoever decodes an address first owns
        // the record, and every later arrival at that address is dropped here.
        // That is order-independent only because `decode_one` at a given address
        // is a pure function of (address, image bytes, context values) and the
        // record carries no trace of who paid for the decode -- `Insn` has no
        // owning-function field. A walk that splits this map across workers must
        // preserve exactly that: one decoder per address, and a record that does
        // not depend on which worker produced it.
        self.contains_key(&vma)
    }

    fn record(&mut self, insn: InsnLite) {
        self.insert(insn.addr, Insn::from(insn));
    }
}

/// Where [`step`] claims a function entry discovered at a direct CALL target.
pub(super) trait FuncSink {
    /// Record a function entry at `entry` unless one is already recorded.
    fn claim(&mut self, entry: u64);
}

impl FuncSink for BTreeMap<u64, DiscoveredFunction> {
    fn claim(&mut self, entry: u64) {
        // INVARIANT (first-writer-wins): the seeds are pre-inserted before the
        // walk starts, so this can never overwrite a seed's name/from_symbol,
        // and `discovered(entry)` is a constant function of `entry`. Both halves
        // are what make the claim order-independent: which visit reaches a CALL
        // target first, and how many do, cannot change what is recorded.
        self.entry(entry).or_insert_with(|| discovered(entry));
    }
}

/// Where [`step`] files a cross-reference edge.
pub(super) trait RefSink {
    /// File a reference into both directions.
    fn file(&mut self, from: u64, to: u64, kind: RefKind);
}

/// The serial walk's two reference maps behind a [`RefSink`].
///
/// A no-op when the caller asked for no reference model, so the walk's edge
/// sites read the same either way and a new one cannot miss the gate.
pub(super) struct RefBuckets {
    want: bool,
    to: BTreeMap<u64, Vec<Reference>>,
    from: BTreeMap<u64, Vec<Reference>>,
}

impl RefBuckets {
    fn new(want: bool) -> RefBuckets {
        RefBuckets { want, to: BTreeMap::new(), from: BTreeMap::new() }
    }
}

impl RefSink for RefBuckets {
    fn file(&mut self, from: u64, to: u64, kind: RefKind) {
        if !self.want {
            return;
        }
        let r = Reference { from, to, kind, op_index: None };
        self.to.entry(to).or_default().push(r.clone());
        self.from.entry(from).or_default().push(r);
    }
}

/// Where [`step`] files evidence that a target address is a stack-passed
/// callback -- an already-proven x86 `PUSH imm` of an in-range address.
pub(super) trait CallbackSink {
    /// Record one `PUSH imm` site.
    fn file(&mut self, source: u64, target: u64);
}

/// The bounded, deduplicated callback-evidence model behind a [`CallbackSink`].
///
/// Keeping the lowest source per target deduplicates before storage. A stable
/// hash rank samples across the address space instead of systematically starving
/// high-address sections, and makes the cap deterministic regardless of the
/// order the walk reached the sites in -- the same property the parallel walk
/// needs, since a schedule that changes which site arrives first must not change
/// which targets survive the cap.
#[derive(Default)]
pub(super) struct CallbackEvidence {
    refs: BTreeMap<u64, u64>,
    /// Stable sampling rank for each retained target.
    ranks: BTreeSet<(u64, u64)>,
}

impl CallbackSink for CallbackEvidence {
    fn file(&mut self, source: u64, target: u64) {
        if let Some(prior) = self.refs.get_mut(&target) {
            *prior = (*prior).min(source);
            return;
        }
        let rank = callback_rank(target);
        self.refs.insert(target, source);
        self.ranks.insert((rank, target));
        if self.refs.len() > super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE {
            if let Some(worst) = self.ranks.pop_last() {
                self.refs.remove(&worst.1);
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

/// Where [`step`] offers the successors of the instruction it just decoded.
///
/// The serial walk pushes them onto its two worklists; a walk that partitions
/// the address space routes each one to whoever owns it.
pub(super) trait Successors {
    /// A same-path successor (branch target or fall-through).
    fn insn(&mut self, vma: u64);
    /// A newly claimed function entry.
    fn func(&mut self, entry: u64);
}

/// The per-instruction decisions that come off the [`Architecture`] — captured
/// once, before the walk, so [`step`] reads no engine state but the decoder.
#[derive(Debug, Clone, Copy)]
pub(super) struct WalkPolicy {
    /// `analysis_unmappedentry`: refuse a function claim the walk would not
    /// decode (see [`super::kuna_unmappedentry`]).
    pub unmappedentry: bool,
    /// Harvest x86 `PUSH imm` callback evidence (see
    /// [`super::kuna_callbackentry`]). Decided by the caller, which alone sees
    /// the parsed object's architecture.
    pub stack_callbacks: bool,
}

impl WalkPolicy {
    /// The one place the walk reads a decision off the [`Architecture`].
    pub(super) fn from_arch(arch: &Architecture, stack_callbacks: bool) -> WalkPolicy {
        WalkPolicy { unmappedentry: arch.analysis_unmappedentry, stack_callbacks }
    }
}

/// Everything [`step`] reads that does not change from one instruction to the
/// next.
pub(super) struct StepCtx<'a> {
    /// The decoder.
    pub translate: &'a dyn Translate,
    /// The space every `Address` is built in.
    pub code_space: &'a Rc<AddrSpace>,
    /// The executable-range universe (sorted, the out-of-bounds gate).
    pub exec_ranges: &'a [(u64, u64)],
    /// The PPC64 ELFv2 local-entry fold, keyed by local entry VMA.
    pub local_entries: &'a BTreeMap<u64, u64>,
    /// The option bits the loop consults.
    pub policy: WalkPolicy,
    /// What the build captures beyond the partition.
    pub detail: ListingDetail,
}

/// True if `vma` lands inside any executable range `[lo, hi)`.
pub(super) fn in_exec(exec_ranges: &[(u64, u64)], vma: u64) -> bool {
    exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}

/// Decode `vma`, file its record, and offer its successors to `out`.
///
/// The whole per-instruction body of the walk, and the only copy of it: the
/// serial [`walk`] drives it through a sink that pushes onto its own worklists.
/// Returns `false` when the path stopped here (already decoded, outside every
/// executable range, undecodable, or zero-length) and nothing was recorded.
pub(super) fn step<I, F, R, C, S>(
    ctx: &StepCtx<'_>,
    vma: u64,
    insns: &mut I,
    funcs: &mut F,
    refs: &mut R,
    callbacks: &mut C,
    out: &mut S,
) -> bool
where
    I: InsnSink + ?Sized,
    F: FuncSink + ?Sized,
    R: RefSink + ?Sized,
    C: CallbackSink + ?Sized,
    S: Successors + ?Sized,
{
    if insns.decoded(vma) {
        return false; // the VisitStat dedup (overlap detection free)
    }
    if !in_exec(ctx.exec_ranges, vma) {
        return false; // out-of-bounds gate (flow.rs:891 analog)
    }

    let decoded = match decode_one(
        ctx.translate,
        vma,
        ctx.code_space,
        ctx.detail.assembly,
        ctx.policy.stack_callbacks,
    ) {
        Ok(d) => d,
        Err(_) => return false, // decode error: stop this path (mark gap)
    };
    if decoded.len == 0 {
        return false; // zero-length decode would not advance; stop this path
    }

    let c = classify(&decoded.ops, vma, decoded.len);

    let fall_through = vma.wrapping_add(u64::from(decoded.len));
    let stack_values: Vec<u64> = if ctx.policy.stack_callbacks {
        decoded
            .stored_scalar_values
            .iter()
            .copied()
            .filter(|&target| target != fall_through && in_exec(ctx.exec_ranges, target))
            .collect()
    } else {
        Vec::new()
    };
    // A constant STORE value is the narrow p-code shape shared by
    // `PUSH imm` and `MOV [mem],imm`. Only that shape earns the on-demand
    // assembly parse; register immediates and absolute LOAD addresses do
    // not trigger a second decode.
    let mnemonic = if !stack_values.is_empty() && decoded.mnemonic.is_empty() {
        super::decode::mnemonic_at(ctx.translate, vma, ctx.code_space)
    } else {
        decoded.mnemonic
    };
    let is_stack_callback_push = mnemonic.eq_ignore_ascii_case("PUSH");

    insns.record(InsnLite {
        addr: vma,
        len: decoded.len,
        fall_through: c.fall_through,
        flow: c.flow,
        flows: c.flows.clone(),
        mnemonic,
        operands: decoded.operands,
    });

    if is_stack_callback_push {
        for target in stack_values {
            callbacks.file(vma, target);
        }
    }

    // Successor edges.
    for &t in &c.flows {
        if c.flow.is_call {
            // CALL/CALLIND direct target -> a NEW function entry, but only
            // where the instruction worklist would agree to decode
            // (`unmappedentry`) and where the target is not the callee's own
            // PPC64 ELFv2 local entry (`ppclocalentry` -- a point inside a
            // function whose global entry is already a seed, so the bytes
            // here are walked either way). The reference is filed whatever
            // the claim decides (where there is a reference model at all);
            // only the function claim is withheld.
            if !ctx.local_entries.contains_key(&t)
                && super::kuna_unmappedentry::admits_call_entry(
                    ctx.policy.unmappedentry,
                    ctx.exec_ranges,
                    t,
                )
            {
                funcs.claim(t);
                out.func(t);
            }
            refs.file(vma, t, RefKind::Call);
        } else {
            // Branch target -> same-function successor.
            out.insn(t);
            refs.file(vma, t, RefKind::Code);
        }
    }
    if let Some(fall) = c.fall_through {
        out.insn(fall);
        refs.file(vma, fall, RefKind::Code);
    }
    true
}

/// The serial walk's two worklists behind a [`Successors`] sink.
struct Worklists<'a> {
    insns: Vec<u64>,
    funcs: &'a mut Vec<u64>,
}

impl Successors for Worklists<'_> {
    fn insn(&mut self, vma: u64) {
        self.insns.push(vma);
    }
    fn func(&mut self, entry: u64) {
        self.funcs.push(entry);
    }
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
/// (the reference sink becomes a no-op). Every instruction contributes an
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

    let ctx = StepCtx {
        translate,
        code_space,
        exec_ranges,
        local_entries,
        policy: WalkPolicy::from_arch(arch, want_stack_callbacks),
        detail,
    };

    let mut insns: BTreeMap<u64, Insn> = BTreeMap::new();
    let mut funcs: BTreeMap<u64, DiscoveredFunction> = BTreeMap::new();
    let mut refs = RefBuckets::new(detail.refs);
    let mut callbacks = CallbackEvidence::default();

    // Function-entry worklist, seeded from the root set.
    let mut func_worklist: Vec<u64> = seeds.to_vec();
    let mut visited_funcs: BTreeSet<u64> = BTreeSet::new();

    // Pre-populate the function model with the seed metadata so a seeded entry
    // keeps its name/from_symbol even if a later CALL also targets it.
    for &entry in seeds {
        let df = seed_funcs.get(&entry).cloned().unwrap_or_else(|| discovered(entry));
        funcs.entry(entry).or_insert(df);
    }

    while let Some(entry) = func_worklist.pop() {
        if !visited_funcs.insert(entry) {
            continue; // already walked this function
        }

        // Per-function instruction worklist.
        let mut work = Worklists { insns: vec![entry], funcs: &mut func_worklist };
        while let Some(vma) = work.insns.pop() {
            step(&ctx, vma, &mut insns, &mut funcs, &mut refs, &mut callbacks, &mut work);
        }
    }

    WalkState {
        insns,
        refs_to: refs.to,
        refs_from: refs.from,
        funcs,
        stack_callback_refs: callbacks.refs,
    }
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
        let mut forward = CallbackEvidence::default();
        let mut reverse = CallbackEvidence::default();
        for target in 0..count as u64 {
            forward.file(0x2000 + target, 0x1000 + target);
        }
        for target in (0..count as u64).rev() {
            reverse.file(0x3000 + target, 0x1000 + target);
        }
        assert_eq!(
            forward.refs.len(),
            super::super::kuna_callbackentry::MAX_CALLBACK_EVIDENCE
        );
        assert_eq!(
            forward.refs.keys().collect::<Vec<_>>(),
            reverse.refs.keys().collect::<Vec<_>>()
        );

        let retained = *forward.refs.keys().next().unwrap();
        forward.file(1, retained);
        assert_eq!(forward.refs[&retained], 1);
    }
}
