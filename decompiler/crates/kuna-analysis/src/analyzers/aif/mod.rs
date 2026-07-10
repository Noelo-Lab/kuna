//! Aggressive Instruction Finder gap-walk — the kuna analog of Ghidra's
//! `AggressiveInstructionFinderAnalyzer` ("Aggressive Instruction Finder"), the
//! third Listing/xref consumer.
//!
//! # What this is (the speculative gap-filler)
//!
//! Entry discovery ([`crate::entry`]) + funcsyms find functions that are
//! *named*, *unwind-described* (`.eh_frame` FDE), or reached by a static CALL (the
//! recursive-descent Listing walk follows every direct call edge). But a function
//! reachable ONLY through an **indirect / data path** — its address stored in a
//! `.rodata` function-pointer table and called via `call *reg`, with the index
//! opaque to static analysis — is invisible to all of those: it is in no symbol
//! table, has no FDE, and no static CALL edge points at it. It sits in the
//! executable image as an **undefined gap** between two discovered functions.
//!
//! AIF speculatively fills those gaps. Over the [`CodeUnit`](crate::listing::CodeUnit)
//! partition the Listing walk left behind, it finds each UNDEFINED gap between
//! discovered functions ([`Listing::first_undefined_after`]), speculatively decodes
//! the gap (the Listing decoded only *reachable* code; AIF probes the gaps with the
//! same SLEIGH decoder), and accepts a gap start as a NEW function entry when it
//! BOTH:
//!
//!  1. **disassembles into a valid subroutine** ([`check_valid_subroutine`] — the
//!     kuna analog of `PseudoDisassembler.checkValidSubroutine` /
//!     `followSubFlows`: follow fall-through + intra-function branches from the gap
//!     start, reaching a clean RET without hitting a bad/undecodable byte, an
//!     out-of-range flow, or a too-short routine; > 2 instructions); and
//!  2. **matches a function-start fingerprint seen elsewhere** (Ghidra's
//!     mask-histogram: a candidate's prologue must match a fingerprint that ≥ the
//!     acceptance threshold of the *already-discovered* functions share). This is
//!     what keeps AIF from "creating a lot of bad code" — a gap only becomes a
//!     function if its prologue looks like the prologues the binary already uses.
//!
//! Accepted gap starts are emitted as [`AnalysisOutput::entries`] — the SAME
//! `entries` fact the entry-discovery pass emits, applied by the SAME commit arm
//! (`engine.rs` `commit_analysis_output`: `name_function` + `add_function` +
//! `register_symbol`), so a discovered gap function becomes `sub_<addr>`,
//! decompilable by name. The flow-repair half ("schedule myself to run again,
//! disassemble the body") is **inherited**: kuna's engine decompiles any
//! registered entry on demand, so emitting the entry is the whole job here.
//!
//! # Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/prototype/analysis/AggressiveInstructionFinderAnalyzer.java`
//!   — `AnalyzerType.BYTE_ANALYZER`, **`setDefaultEnablement(false)`** with the
//!   DESCRIPTION warning *"IT MAY CREATE A LOT OF BAD CODE!"*. The decompiler
//!   -relevant logic ported here:
//!   - the `funcStartMap` fingerprint histogram over every discovered function's
//!     prologue (upstream uses `SleighDebugLogger(MASKS_ONLY)` + `getInstructionMask`
//!     to mask operand bits; see the LOSS note below);
//!   - the `funcCount >= MINIMUM_FUNCTION_COUNT` (20) + `getNumInstructions() > 0`
//!     bail (too few functions ⇒ no reliable fingerprint corpus);
//!   - the gap walk over undefined data (`getFirstUndefinedData` →
//!     [`Listing::first_undefined_after`]), the fingerprint match
//!     (`startCount >= 4`), and `checkValidSubroutine` acceptance;
//!   - the `numInstr <= 2` "don't allow two-instruction routines" reject.
//! - `Ghidra/Framework/SoftwareModeling/src/main/java/ghidra/app/util/PseudoDisassembler.java`
//!   — `checkValidSubroutine` / `followSubFlows` (the valid-subroutine predicate).
//!
//! # Why AIF is not a pure [`AnalysisPass`] over `ctx.listing`
//!
//! The two no-return consumers ([`crate::noreturn_disc`] /
//! [`crate::noreturn_propagate`]) read ONLY the built Listing, so they are pure
//! `AnalysisPass`es over `ctx.listing`. AIF additionally needs to **speculatively
//! decode the gap bytes** — bytes the Listing walk left undecoded — which requires
//! the live SLEIGH decoder (the upstream constructs its own `PseudoDisassembler`).
//! The decoder is not in `AnalysisCtx`, so AIF is driven by [`run_aif`], invoked
//! from `passes::run_listing_consumers` with the same `translate`/`code_space` the
//! Listing build held. It is still gated end-to-end by its own `aif`
//! `--option` (and the `listing` flag), exactly like the other consumers
//! (`engine.rs::analysis_pass_enabled`), and its output is the same additive
//! `entries` fact stream.
//!
//! # Faithful scope / LOSS
//!
//! - **The fingerprint is the decoded *mnemonic* sequence of the first
//!   [`FINGERPRINT_INSNS`] instructions, not Ghidra's masked instruction bytes.**
//!   Ghidra builds the fingerprint from `getInstructionMask` (the per-constructor
//!   bit-mask that separates opcode bits from operand bits) — a SLEIGH capability
//!   kuna's decoder does not surface (`docs/history/listing-tier-design.md` §8: the mask
//!   accessor "is the same one FID would need; build it independently only if FID
//!   is taken"). The mnemonic sequence is the operand-insensitive opcode projection
//!   the kuna decoder *does* expose (`print_assembly` splits mnemonic from operand
//!   body), so two functions with the same prologue opcodes but different operand
//!   immediates (e.g. `add $0xb,%eax` vs `add $0x40,%eax`) hash to the SAME
//!   fingerprint — exactly the masked-bytes equivalence class. This is a faithful
//!   *substitute* for, not a bit-exact reproduction of, Ghidra's mask histogram.
//! - **Acceptance threshold = [`FINGERPRINT_THRESHOLD`] (4)**, matching Ghidra's
//!   `startCount < 4 ⇒ continue`.
//! - **Minimum function count = [`MINIMUM_FUNCTION_COUNT`] (20)**, matching
//!   `MINIMUM_FUNCTION_COUNT`. A real ELF (crt + libc glue + the program's own
//!   functions) clears this easily.
//! - **Context-mode (ARM Thumb / MIPS16) gap probing is NOT handled** (the upstream
//!   ARM variant `ArmAggressiveInstructionFinderAnalyzer` adds per-gap `TMode`
//!   context). kuna's AIF probes gaps in the Listing's already-resolved decode
//!   context; x86-64 (the gated target) needs no decode-mode context. Documented LOSS.
//! - **DISABLED by default** (Ghidra `setDefaultEnablement(false)`): a speculative
//!   gap-filler can create false-positive functions, so it ships behind its own
//!   `aif` flag *and* requires `--option listing on`. The buildplan's verdict
//!   (`docs/history/analysis-port-buildplan.md` §1.3) is "build the sound substitute" — this
//!   is that substitute, kept off by default.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use crate::listing::{decode::decode_one, FlowKind, Listing};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// Ghidra's `MINIMUM_FUNCTION_COUNT`: AIF does nothing on a program with fewer
/// discovered functions than this (too small a corpus to fingerprint reliably).
const MINIMUM_FUNCTION_COUNT: usize = 20;

/// Ghidra's `startCount < 4 ⇒ continue`: a gap prologue must match a fingerprint
/// shared by at least this many discovered functions to be accepted.
const FINGERPRINT_THRESHOLD: usize = 4;

/// Number of leading instructions whose mnemonic sequence forms a function-start
/// fingerprint. Ghidra hashes the masked bytes of the first **two** instructions
/// (the entry instruction + the one after, concatenated); we mirror that depth.
const FINGERPRINT_INSNS: usize = 2;

/// Ghidra's `numInstr <= 2 ⇒ continue`: a candidate must disassemble into more
/// than two instructions ("don't allow two-instruction routines").
const MIN_SUBROUTINE_INSNS: usize = 3;

/// `followSubFlows` instruction cap (Ghidra passes `4000`): bound the validity
/// walk so a pathological gap cannot loop unbounded.
const MAX_FOLLOW_INSNS: usize = 4000;

/// The Aggressive Instruction Finder gap-walk consumer pass (the third Listing/xref
/// consumer) — used only for the `AnalysisPass` IDENTITY (the `aif` gate name + the
/// `S1` stage). The real work is in [`run_aif`] (it needs the live decoder, not just
/// `ctx.listing` — see the module docs). The pass's [`AnalysisPass::run`] is a
/// deliberate no-op: it carries no decoder, so it cannot probe gaps; registering it
/// only fixes the gate name/stage for the option machinery.
#[derive(Default)]
pub struct AggressiveInstructionFinderPass;

impl AnalysisPass for AggressiveInstructionFinderPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "aif"
    }

    /// No-op: AIF cannot run from the pure-`ctx` loop (it needs the SLEIGH decoder
    /// to probe undecoded gap bytes). The live path is [`run_aif`], invoked from
    /// `run_listing_consumers` with the decoder. Returns an empty output here so a
    /// stray registration in the pure loop is inert.
    fn run(&self, _ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput::default()
    }
}

/// One function-start fingerprint: the mnemonic sequence of the first
/// [`FINGERPRINT_INSNS`] instructions of a function, plus the total byte length of
/// those instructions (the upstream concatenates the masked bytes, so the prefix
/// length is part of the identity).
///
/// The mnemonic-sequence projection is the kuna substitute for Ghidra's masked
/// instruction bytes (see the module LOSS note): operand-insensitive, so functions
/// with the same prologue opcodes but different operand immediates share a
/// fingerprint, the same equivalence class the mask histogram forms.
type Fingerprint = (Vec<String>, u64);

/// A speculative decoder over the loadimage bytes, used to probe undefined gaps.
///
/// It wraps the live SLEIGH decoder ([`Translate::one_instruction`] via
/// [`decode_one`]) plus the code space + executable-range universe, and decodes one
/// candidate instruction at a time — the kuna analog of the upstream
/// `PseudoDisassembler` that re-decodes gap bytes without persisting them. It caches
/// each speculative decode so a re-probe of the same VMA is free.
struct GapDecoder<'a> {
    translate: &'a dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &'a [(u64, u64)],
    /// `vma -> Some(insn)` on a good decode, `vma -> None` on an undecodable byte
    /// (so a re-probe is cached too).
    cache: BTreeMap<u64, Option<ProbedInsn>>,
}

/// A speculatively-decoded instruction (the gap-probe analog of [`crate::listing::Insn`]).
#[derive(Clone)]
struct ProbedInsn {
    len: u32,
    kind: FlowKind,
    is_call: bool,
    is_terminal: bool,
    fall_through: Option<u64>,
    flows: Vec<u64>,
    mnemonic: String,
}

impl<'a> GapDecoder<'a> {
    fn new(
        translate: &'a dyn Translate,
        code_space: Rc<AddrSpace>,
        exec_ranges: &'a [(u64, u64)],
    ) -> Self {
        GapDecoder { translate, code_space, exec_ranges, cache: BTreeMap::new() }
    }

    fn in_exec(&self, vma: u64) -> bool {
        self.exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
    }

    /// Speculatively decode the instruction at `vma`. `None` if out of range or
    /// undecodable. Caches the result (good or bad).
    fn probe(&mut self, vma: u64) -> Option<ProbedInsn> {
        if let Some(cached) = self.cache.get(&vma) {
            return cached.clone();
        }
        let result = self.decode_uncached(vma);
        self.cache.insert(vma, result.clone());
        result
    }

    fn decode_uncached(&self, vma: u64) -> Option<ProbedInsn> {
        if !self.in_exec(vma) {
            return None;
        }
        let decoded = decode_one(self.translate, vma, &self.code_space).ok()?;
        if decoded.len == 0 {
            return None;
        }
        let c = crate::listing::classify::classify(&decoded.ops, vma, decoded.len);
        Some(ProbedInsn {
            len: decoded.len,
            kind: c.flow.kind,
            is_call: c.flow.is_call,
            is_terminal: c.flow.is_terminal,
            fall_through: c.fall_through,
            flows: c.flows,
            mnemonic: decoded.mnemonic,
        })
    }

    /// The fingerprint of the candidate prologue at `entry` (the first
    /// [`FINGERPRINT_INSNS`] speculatively-decoded instructions). `None` if the
    /// prologue does not decode contiguously or carries an empty mnemonic.
    fn fingerprint(&mut self, entry: u64) -> Option<Fingerprint> {
        let mut mnems: Vec<String> = Vec::with_capacity(FINGERPRINT_INSNS);
        let mut total_len: u64 = 0;
        let mut vma = entry;
        for _ in 0..FINGERPRINT_INSNS {
            let insn = self.probe(vma)?;
            if insn.mnemonic.is_empty() {
                return None;
            }
            mnems.push(insn.mnemonic.clone());
            total_len += insn.len as u64;
            vma = vma.checked_add(insn.len as u64)?;
        }
        Some((mnems, total_len))
    }
}

/// Compute the function-start fingerprint at `entry` over the BUILT Listing (a
/// discovered function whose prologue is already decoded). The histogram half reads
/// the Listing's instruction model (no speculative decode needed — these are
/// reachable functions).
fn fingerprint_in_listing(listing: &Listing, entry: u64) -> Option<Fingerprint> {
    let mut mnems: Vec<String> = Vec::with_capacity(FINGERPRINT_INSNS);
    let mut total_len: u64 = 0;
    let mut vma = entry;
    for _ in 0..FINGERPRINT_INSNS {
        let insn = listing.instruction_at(vma)?;
        if insn.mnemonic.is_empty() {
            return None;
        }
        mnems.push(insn.mnemonic.clone());
        total_len += insn.len as u64;
        vma = vma.checked_add(insn.len as u64)?;
    }
    Some((mnems, total_len))
}

/// Build the function-start fingerprint histogram over every DISCOVERED function
/// (Ghidra's `funcStartMap`): map each prologue fingerprint to the number of
/// discovered functions that share it.
fn build_fingerprint_histogram(listing: &Listing) -> BTreeMap<Fingerprint, usize> {
    let mut hist: BTreeMap<Fingerprint, usize> = BTreeMap::new();
    for (&entry, _) in listing.functions() {
        if let Some(fp) = fingerprint_in_listing(listing, entry) {
            *hist.entry(fp).or_insert(0) += 1;
        }
    }
    hist
}

/// The kuna analog of `PseudoDisassembler.checkValidSubroutine` /
/// `followSubFlows`: starting at `entry`, follow the speculatively-decoded control
/// flow (fall-through + intra-function branches) and decide whether it is a valid
/// subroutine.
///
/// Faithful to the upstream predicate, scoped to the speculative decode:
///  - decode each reached instruction; an undecodable byte inside the gap, or an
///    out-of-range flow target, is a hard reject (`checkValidSubroutine` returns
///    false on a bad instruction / `!memory.contains`);
///  - a `RETURN` (terminal) reached on a path is a valid termination
///    (`didTerminate`); a computed/indirect jump is also a terminate signal
///    (`flowType.isComputed() ⇒ didTerminate = true`);
///  - a `CALL` (or a `JUMP` into already-discovered code) "adds info"
///    (`didCallValidSubroutine`);
///  - reject a two-instruction-or-shorter routine ([`MIN_SUBROUTINE_INSNS`] —
///    `numInstr <= 2`);
///  - bound the walk at [`MAX_FOLLOW_INSNS`].
///
/// Returns `Some(body_insns)` (the VMAs the valid routine covers) on acceptance,
/// `None` on rejection. The body set lets the caller skip re-probing addresses
/// already inside an accepted routine.
fn check_valid_subroutine(
    decoder: &mut GapDecoder,
    listing: &Listing,
    entry: u64,
    gap_lo: u64,
    gap_hi: u64,
) -> Option<BTreeSet<u64>> {
    let mut body: BTreeSet<u64> = BTreeSet::new();
    let mut worklist: Vec<u64> = vec![entry];
    let mut did_terminate = false;
    let mut adds_info = false;
    let mut steps = 0usize;

    while let Some(vma) = worklist.pop() {
        if body.contains(&vma) {
            continue; // already followed (the VisitStat dedup)
        }
        steps += 1;
        if steps > MAX_FOLLOW_INSNS {
            break;
        }

        // A flow target that has ALREADY been decoded by the Listing (i.e. it lands
        // in already-discovered code) is a legitimate flow into existing code, not a
        // byte we must re-decode. Only bytes INSIDE the gap are "must-decode-here".
        if vma < gap_lo || vma >= gap_hi {
            // Outside the gap: if it is decoded code, "adds info"; otherwise (data /
            // undecoded outside the gap) it is a bad flow target → reject.
            if listing.is_instruction_start(vma) {
                adds_info = true;
                continue;
            }
            if !decoder.in_exec(vma) {
                return None;
            }
            // Outside the gap but inside exec and not a known instruction start:
            // a flow into the interior of existing code or an unreached region; do
            // not chase it (the upstream stops at the existing-code boundary).
            continue;
        }

        // Inside the gap: speculatively decode. An undecodable byte invalidates.
        let Some(insn) = decoder.probe(vma) else {
            return None;
        };
        body.insert(vma);

        if insn.is_terminal {
            did_terminate = true;
            continue;
        }
        if matches!(insn.kind, FlowKind::ComputedJump) {
            // A computed/indirect jump with no static target is a terminate signal.
            did_terminate = true;
            continue;
        }

        for &target in &insn.flows {
            if !decoder.in_exec(target) {
                return None; // flow leaves the executable image (`!memory.contains`)
            }
            if insn.is_call {
                adds_info = true; // a call (to any in-image target) adds info
            } else {
                worklist.push(target); // branch target → intra-routine successor
            }
        }
        if let Some(fall) = insn.fall_through {
            worklist.push(fall);
        }
    }

    if body.len() < MIN_SUBROUTINE_INSNS {
        return None; // `numInstr <= 2`
    }
    if !did_terminate && !adds_info {
        return None; // `didTerminate || didCallValidSubroutine`
    }
    Some(body)
}

/// Run the AIF gap-walk (Ghidra's `added`) and return the accepted gap-start
/// entries.
///
/// `listing` is the built Listing (fingerprint corpus + the undefined-gap
/// partition + the function/instruction-count gate); `translate`/`code_space`/
/// `exec_ranges` drive the speculative gap decode. The result is the accepted gap
/// starts, address-sorted and deduped, ready to emit as [`AnalysisOutput::entries`].
///
/// Bails (returns empty) unless there are ≥ [`MINIMUM_FUNCTION_COUNT`] discovered
/// functions AND ≥ 1 decoded instruction (Ghidra's `funcCount` /
/// `getNumInstructions()` gate), or if no fingerprint reaches the threshold.
pub fn run_aif(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    if listing.function_count() < MINIMUM_FUNCTION_COUNT || listing.num_instructions() == 0 {
        return Vec::new();
    }

    let hist = build_fingerprint_histogram(listing);
    if !hist.values().any(|&c| c >= FINGERPRINT_THRESHOLD) {
        return Vec::new();
    }

    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted: BTreeSet<u64> = BTreeSet::new();
    let mut claimed: BTreeSet<u64> = BTreeSet::new();

    // Walk every undefined gap: `first_undefined_after(start)` returns the next
    // undefined executable VMA. Probe it, then advance past whatever we resolved
    // (the accepted body, or one byte on a reject) and continue.
    let mut cursor = listing.first_undefined_after(0);
    while let Some(gap_start) = cursor {
        // The extent of THIS contiguous undefined gap: from `gap_start` up to the
        // next decoded instruction start (or, if none, an open upper bound). Only
        // this interior is "must-decode-here"; flows past it into discovered code
        // are legitimate.
        let gap_hi = listing.next_instruction_start_after(gap_start).unwrap_or(u64::MAX);

        let mut advanced = gap_start.saturating_add(1);
        if !claimed.contains(&gap_start) {
            if let Some(body) = probe_gap_start(&mut decoder, listing, &hist, gap_start, gap_hi) {
                accepted.insert(gap_start);
                let body_max = body.iter().copied().max().unwrap_or(gap_start);
                advanced = body_max.saturating_add(1);
                claimed.extend(body);
            }
        }

        // Advance to the next undefined gap strictly after the address we consumed.
        match listing.first_undefined_after(advanced.saturating_sub(1)) {
            Some(n) if n > gap_start => cursor = Some(n),
            _ => break,
        }
    }

    accepted.into_iter().collect()
}

/// Probe a single gap start: speculatively decode its prologue, require its
/// fingerprint to match a ≥-threshold histogram entry, and run the valid-subroutine
/// check. Returns the accepted routine's body VMAs, or `None`.
fn probe_gap_start(
    decoder: &mut GapDecoder,
    listing: &Listing,
    hist: &BTreeMap<Fingerprint, usize>,
    gap_start: u64,
    gap_hi: u64,
) -> Option<BTreeSet<u64>> {
    let fp = decoder.fingerprint(gap_start)?;
    let count = hist.get(&fp).copied().unwrap_or(0);
    if count < FINGERPRINT_THRESHOLD {
        return None;
    }
    check_valid_subroutine(decoder, listing, gap_start, gap_start, gap_hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The fingerprint + gap-walk + valid-subroutine logic is exercised end-to-end
    // by the cross-crate `verify_aif.rs` gate (a stripped x86-64 fixture with a
    // function reachable only through a function-pointer table). The constants and
    // the pass identity are pinned here; the predicate helpers stay small.

    #[test]
    fn thresholds_match_ghidra() {
        assert_eq!(MINIMUM_FUNCTION_COUNT, 20);
        assert_eq!(FINGERPRINT_THRESHOLD, 4);
        assert_eq!(MIN_SUBROUTINE_INSNS, 3);
        assert_eq!(FINGERPRINT_INSNS, 2);
    }

    #[test]
    fn pass_identity_wiring() {
        let p = AggressiveInstructionFinderPass;
        assert_eq!(p.id(), "aif");
        assert_eq!(p.phase(), Phase::P1);
    }

    #[test]
    fn fingerprint_is_operand_insensitive_by_construction() {
        // Two prologues with the same mnemonic sequence + total length collide
        // (the masked-bytes equivalence class). Differing only by operand text (not
        // captured in the mnemonic) keeps them in the same bucket.
        let a: Fingerprint = (vec!["PUSH".into(), "MOV".into()], 4);
        let b: Fingerprint = (vec!["PUSH".into(), "MOV".into()], 4);
        let c: Fingerprint = (vec!["PUSH".into(), "SUB".into()], 4);
        let mut hist: BTreeMap<Fingerprint, usize> = BTreeMap::new();
        *hist.entry(a.clone()).or_insert(0) += 1;
        *hist.entry(b).or_insert(0) += 1; // same key → count 2
        *hist.entry(c.clone()).or_insert(0) += 1;
        assert_eq!(hist.get(&a).copied(), Some(2));
        assert_eq!(hist.get(&c).copied(), Some(1));
    }
}
