//! Aggressive Instruction Finder gap-walk — the kuna analog of Ghidra's
//! `AggressiveInstructionFinderAnalyzer` ("Aggressive Instruction Finder"), a
//! Listing/xref consumer.
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
//! Listing build held. It is gated before invocation by its own `aif` `--option`
//! (and the `listing` flag), exactly like the other consumers; the downstream
//! `engine.rs::analysis_pass_enabled` check remains as a defensive commit gate.
//! Its output is the same additive `entries` fact stream.
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

pub mod kuna_aifcorroborate;
pub mod kuna_aifstrict;
pub mod kuna_poolentry;
pub mod kuna_ptrentry;
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

/// The Aggressive Instruction Finder gap-walk consumer pass — used only for the
/// `AnalysisPass` IDENTITY (the `aif` gate name + the `S1` stage). The real work is
/// in [`run_aif`] (it needs the live decoder, not just
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
    operands: String,
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
            operands: decoded.operands,
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
///  - a `CALL` or `JUMP` into already-discovered code "adds info"
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
    check_valid_subroutine_with_policy(decoder, listing, entry, gap_lo, gap_hi, false)
        .map(|(body, _adds_info)| body)
}

fn check_valid_subroutine_strict(
    decoder: &mut GapDecoder,
    listing: &Listing,
    entry: u64,
    gap_lo: u64,
    gap_hi: u64,
) -> Option<BTreeSet<u64>> {
    check_valid_subroutine_with_policy(decoder, listing, entry, gap_lo, gap_hi, true)
        .map(|(body, _adds_info)| body)
}

/// Returns `Some((body, corroborated))`. `corroborated` is Ghidra's `addsInfo`
/// computed the upstream way — set by a CALL, or by a jump whose target is already a
/// decoded instruction, and by nothing else. It is deliberately NOT the local
/// `adds_info` that feeds the `didTerminate || didCallValidSubroutine` gate below:
/// that one also fires on a plain fall-through out of the gap into decoded code,
/// which is the signature of a mid-body phantom rather than evidence against one
/// (see [`kuna_aifcorroborate`]). Only the gap-walk accept ([`probe_gap_start`])
/// reads it; the two wrappers above drop it, because their callers carry their own
/// corroboration.
fn check_valid_subroutine_with_policy(
    decoder: &mut GapDecoder,
    listing: &Listing,
    entry: u64,
    gap_lo: u64,
    gap_hi: u64,
    strict: bool,
) -> Option<(BTreeSet<u64>, bool)> {
    let mut body: BTreeSet<u64> = BTreeSet::new();
    let mut worklist: Vec<u64> = vec![entry];
    let mut did_terminate = false;
    let mut adds_info = false;
    let mut corroborated = false;
    let mut steps = 0usize;

    while let Some(vma) = worklist.pop() {
        if body.contains(&vma) {
            continue; // already followed (the VisitStat dedup)
        }
        steps += 1;
        if steps > MAX_FOLLOW_INSNS {
            if strict {
                return None;
            }
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
            // Outside the gap but inside exec and not a known instruction start.
            // The established AIF/ARM oracles stop at this boundary; the fast
            // pointer-root validator rejects the uncorroborated escape.
            if strict {
                return None;
            }
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
                // Upstream: "calls always add info".
                corroborated = true;
                if !strict || listing.is_instruction_start(target) {
                    adds_info = true;
                }
            } else {
                // Upstream: "jumps must jump to existing code".
                if listing.is_instruction_start(target) {
                    corroborated = true;
                }
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
    Some((body, corroborated))
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
/// `aifstrict` (GH-299) adds the structural rejects in [`kuna_aifstrict`]: the gap
/// cursor slides to the next 4-byte boundary instead of the next byte, probing only
/// an aligned address or a hole's first byte. `aifcorroborate` (GH-313) restores
/// upstream's second fingerprint test in [`kuna_aifcorroborate`]: an accept must
/// either add information or match a prologue 50 discovered functions share.
pub fn run_aif(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    aifstrict: bool,
    aifcorroborate: bool,
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
        let next_code = listing.next_instruction_start_after(gap_start);
        let gap_hi = next_code.unwrap_or(u64::MAX);

        // (kuna, `aifstrict`) The cursor slides to the next instruction-alignment
        // boundary rather than the next byte, and only an aligned address or a
        // hole's first byte is a candidate at all.
        let mut advanced = if aifstrict {
            kuna_aifstrict::next_probe_after(gap_start, gap_hi)
        } else {
            gap_start.saturating_add(1)
        };
        let probe_here =
            !aifstrict || kuna_aifstrict::probe_allowed(listing, gap_start);
        if probe_here && !claimed.contains(&gap_start) {
            match probe_gap_start(&mut decoder, listing, &hist, gap_start, gap_hi, aifcorroborate)
            {
                Probe::Accept(body) => {
                    accepted.insert(gap_start);
                    let body_max = body.iter().copied().max().unwrap_or(gap_start);
                    advanced = body_max.saturating_add(1);
                    claimed.extend(body);
                }
                // (kuna, `aifcorroborate`) An uncorroborated candidate is refused
                // as an ENTRY but still consumes its body. Without that, refusing
                // an accept hands the cursor back to the interior of the same hole
                // and it accepts a worse candidate deeper inside — measured on the
                // 3.4 MB PE witness as mid-body entries RISING by 222 while 1,100
                // real entries were lost. See `kuna_aifcorroborate`.
                Probe::Uncorroborated(body) => {
                    let body_max = body.iter().copied().max().unwrap_or(gap_start);
                    advanced = body_max.saturating_add(1);
                    claimed.extend(body);
                }
                Probe::Reject => {}
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

/// The outcome of one gap-start probe. `Uncorroborated` is `aifcorroborate`'s
/// refusal and exists as its own variant because it is not a plain `Reject`: the
/// candidate DID disassemble into a valid subroutine, so the cursor must still
/// consume its body rather than fall back into it.
enum Probe {
    Accept(BTreeSet<u64>),
    Uncorroborated(BTreeSet<u64>),
    Reject,
}

/// Probe a single gap start: speculatively decode its prologue, require its
/// fingerprint to match a ≥-threshold histogram entry, and run the valid-subroutine
/// check.
///
/// `aifcorroborate` (GH-313) additionally applies upstream's SECOND fingerprint
/// test here, which is the only frame where both of its inputs exist: the
/// `startCount` histogram hit computed just above, and the corroboration flag the
/// validity walk returns. See [`kuna_aifcorroborate`].
fn probe_gap_start(
    decoder: &mut GapDecoder,
    listing: &Listing,
    hist: &BTreeMap<Fingerprint, usize>,
    gap_start: u64,
    gap_hi: u64,
    aifcorroborate: bool,
) -> Probe {
    let Some(fp) = decoder.fingerprint(gap_start) else { return Probe::Reject };
    let count = hist.get(&fp).copied().unwrap_or(0);
    if count < FINGERPRINT_THRESHOLD {
        return Probe::Reject;
    }
    let Some((body, corroborated)) =
        check_valid_subroutine_with_policy(decoder, listing, gap_start, gap_start, gap_hi, false)
    else {
        return Probe::Reject;
    };
    if aifcorroborate && !kuna_aifcorroborate::accepts(corroborated, count) {
        return Probe::Uncorroborated(body);
    }
    Probe::Accept(body)
}

pub(crate) fn validate_pointer_targets(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    candidates: impl IntoIterator<Item = u64>,
) -> Vec<u64> {
    let hist = build_fingerprint_histogram(listing);
    if !hist.values().any(|&count| count >= FINGERPRINT_THRESHOLD) {
        return Vec::new();
    }

    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted = BTreeSet::new();
    let mut claimed = BTreeSet::new();
    for target in candidates {
        if !listing.is_undefined(target) || claimed.contains(&target) {
            continue;
        }
        let Some(fp) = decoder.fingerprint(target) else { continue };
        if hist.get(&fp).copied().unwrap_or(0) < FINGERPRINT_THRESHOLD {
            continue;
        }
        let gap_hi = listing.next_instruction_start_after(target).unwrap_or(u64::MAX);
        if let Some(body) =
            check_valid_subroutine_strict(&mut decoder, listing, target, target, gap_hi)
        {
            accepted.insert(target);
            claimed.extend(body);
        }
    }
    accepted.into_iter().collect()
}

// ===========================================================================
// Stage 2: raw, UNPAIRED Thumb-prologue gap seeding (angr-style)
// ===========================================================================

/// (kuna, Stage-2 ARM discovery) The **raw, UNPAIRED Thumb-prologue seed** scan —
/// the kuna analog of angr's `CFGFast._func_addrs_from_prologues()`
/// (`angr/analyses/cfg/cfg_fast.py:2607`) over `ArchARMCortexM.thumb_prologs`
/// (`archinfo/arch_arm.py:401`: `{rb"[\x00-\xff]\xb5", rb"\x2d\xe9[\x00-\xff][\x00-\xff]"}`).
///
/// # Why this exists (the dense-binary residual after AIF)
///
/// `funcstart_patterns` seeds a candidate only when a Ghidra `<patternpairs>`
/// EPILOGUE prepattern sits immediately before it, so a function preceded by a
/// literal pool / data / padding is never seeded; the recursive-descent walk
/// (direct `BL` only) cannot reach a function in a call-graph component reachable
/// only through indirect calls / pointer tables; and AIF's fingerprint gap-walk,
/// which advances its cursor past each accepted body, skips dense back-to-back
/// prologue clusters. The residual — measured on betaflight STM32F405 as ~483
/// ground-truth functions that START WITH A CANONICAL THUMB PUSH — all begin with
/// `PUSH {..,lr}` (`0xB5xx`) or `PUSH.W {..,lr}` (`0xE92D..`).
///
/// angr recovers exactly these by seeding EVERY prologue byte-pattern directly,
/// with NO epilogue/prepattern/fingerprint requirement (`finditer` over exec
/// memory at 2-byte alignment, seeds `position | 1`). This mirrors that: scan
/// every executable section at 2-byte alignment for the two canonical LR-saving
/// Thumb prologues and seed each match that survives the precision guards.
///
/// # Precision (angr measured the raw prologues at ~93%; the guards close the gap)
///
///  1. **gap-only** — a candidate already covered by the walk
///     ([`Listing::is_undefined`] is false — it is an instruction start OR an
///     instruction interior) is skipped, so a prologue-shaped byte-pair inside a
///     discovered body never splits it. This IS the reuse of the walk's `covered`
///     RangeList (via the Listing's code-unit partition).
///  2. **`check_valid_subroutine`** — the SAME validity predicate AIF uses
///     ([`check_valid_subroutine`]): the candidate must speculatively decode (in
///     the already-painted Thumb `TMode`, `cortexm_thumb_paints`) into a valid
///     subroutine (> 2 instructions, reaches a clean RET / computed jump /
///     adds-info call, no undecodable byte, no out-of-image flow).
///  3. **body-claim dedup** — candidates are processed in ascending address order
///     and each accepted routine's body is `claimed` (the same `claimed` guard
///     [`run_aif`] uses), so a prologue-shaped byte-pair in the interior of an
///     already-accepted lower-address routine is skipped.
///
/// ARM-gated (`0xB5xx`/`0xE92D` are Thumb encodings): a strict no-op on every
/// non-ARM object. The result is the accepted prologue VMAs, address-sorted, ready
/// to feed the recursive-descent walk as ADDITIONAL seeds (parallel to
/// `full_pattern_starts`), so the walk expands each into a full function and
/// discovers its callees. Gated end-to-end by the same `funcstart_patterns`
/// (`analysis_funcstart_patterns`) discovery flag its caller checks — no new
/// stage-model option — so x86-64 / console / datatest are byte-identical.
pub fn raw_thumb_prologue_seeds(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    use object::read::Object;
    // `0xB5xx` / `0xE92D` are Thumb (16-/32-bit) encodings; meaningless on any
    // other architecture. On a confirmed Cortex-M image the whole exec region is
    // painted `TMode=1` (`cortexm_thumb_paints`), so the speculative decode below
    // is Thumb.
    if file.architecture() != object::Architecture::Arm {
        return Vec::new();
    }

    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted: BTreeSet<u64> = BTreeSet::new();
    let mut claimed: BTreeSet<u64> = BTreeSet::new();

    // Address-ordered scan (executable_sections is address-sorted upstream), so the
    // `claimed` body-dedup sees a real function's entry before any prologue-shaped
    // byte-pair in its interior.
    for (sec_addr, _sec_hi, data) in crate::entry::executable_sections(file) {
        // Snap to the first even (2-byte-aligned) VMA in the section, then stride
        // by two — the Thumb instruction alignment (and angr's finditer stride).
        let mut off = (sec_addr as usize) & 1;
        while off + 1 < data.len() {
            if is_thumb_lr_prologue(&data, off) {
                let vma = sec_addr + off as u64;
                // gap-only + not already inside an accepted routine.
                if listing.is_undefined(vma) && !claimed.contains(&vma) {
                    let gap_hi = listing.next_instruction_start_after(vma).unwrap_or(u64::MAX);
                    if let Some(body) =
                        check_valid_subroutine(&mut decoder, listing, vma, vma, gap_hi)
                    {
                        accepted.insert(vma);
                        claimed.extend(body);
                    }
                }
            }
            off += 2;
        }
    }

    accepted.into_iter().collect()
}

/// True iff the 2 (or 4) bytes at `data[off..]` are a canonical **LR-saving Thumb
/// prologue** — `PUSH {..,lr}` (16-bit `0xB5xx`) or `PUSH.W {..,lr}` (32-bit
/// `0xE92D..` with the LR bit). Little-endian (Cortex-M).
///
/// - `PUSH {registers, lr}` T1: `1011 0101 <register_list:8>` = halfword `0xB5xx`,
///   stored little-endian as bytes `[xx, 0xB5]` — so `data[off+1] == 0xB5`
///   (angr's `rb"[\x00-\xff]\xb5"`).
/// - `PUSH.W {registers, lr}` T2 (`STMDB SP!`): first halfword `0xE92D` stored as
///   `[0x2D, 0xE9]`; the second halfword's bit 14 is `M` (LR). Requiring the LR bit
///   (`data[off+3] & 0x40`, bit 14 lives in the high byte of the 2nd halfword)
///   keeps a non-LR `STMDB SP!` (a mid-function register spill, not a prologue) out
///   — the precision refinement over angr's unconditional
///   `rb"\x2d\xe9[\x00-\xff][\x00-\xff]"`.
fn is_thumb_lr_prologue(data: &[u8], off: usize) -> bool {
    if off + 1 >= data.len() {
        return false;
    }
    // PUSH {..,lr} — 16-bit.
    if data[off + 1] == 0xB5 {
        return true;
    }
    // PUSH.W {..,lr} — 32-bit, require the second halfword's LR bit.
    data[off] == 0x2D
        && data[off + 1] == 0xE9
        && off + 3 < data.len()
        && (data[off + 3] & 0x40) != 0
}

// ===========================================================================
// Stage 3: code-pointer-table (vtable / callback / ISR-handler) gap seeding
// ===========================================================================

/// (kuna, Stage-3 ARM discovery) The **code-pointer-table seed** scan — the kuna
/// analog of Ghidra's reference-based function discovery (`ArmThumbFunctionTableScript`
/// / the loader's data-pointer markup) and angr's `_seg_list` pointer scanning over
/// `.data`/`.rodata`/`.text`.
///
/// # Why this exists (the frameless-callback residual after Stages 1-2)
///
/// Stages 1-2 seed the recursive-descent walk from `<patternpairs>` epilogue-paired
/// starts and from canonical LR-saving Thumb prologues (`PUSH {..,lr}` /
/// `PUSH.W {..,lr}`), and the walk follows every *direct* `BL`/`BLX <imm>`. That
/// covers the whole statically-reachable + canonical-prologue population. The residual
/// is functions reachable ONLY through an indirect / data path — their address is
/// stored (Thumb-bit set) in a function-pointer **table** (an RTOS command/vtable, a
/// driver op-struct, a Cortex-M exception vector) and reached via `BLX <reg>` /
/// `LDR pc,[..]`, so no static CALL edge and no epilogue-pair points at them, and
/// their prologue may be a bare `SUB SP,#imm` / `PUSH {..}` (no LR) / `VPUSH` that
/// the LR-only Stage-2 matcher structurally skips.
///
/// # Mechanism (measured across cf2 / usart-stdio / betaflight: +12 real, 0 false)
///
/// Scan every allocated section at 4-byte alignment for a word whose value, with the
/// Thumb bit cleared, points into an executable range (a code pointer). For each such
/// target that survives the precision guards, seed it as a function. The corroborating
/// **code-pointer reference** is what lets this safely admit a non-LR-push prologue
/// that Stage-2 conservatively rejects.
///
/// # Precision — the triple signal (why this stays at ~100% and never chases the
/// angr-over-discovery trap, e.g. crazyflie's descriptor table @0x8075xxx that
/// force-complete-scan misreads as functions)
///
///  1. **gap-only** ([`Listing::is_undefined`]) + **body-claim dedup** — a target
///     already covered by the walk, or inside an already-accepted Stage-3 body, is
///     skipped (never split a discovered function). Same guards as Stages 1-2.
///  2. **stack-frame prologue** ([`is_thumb_function_prologue`]) — the target's first
///     bytes must be a frame-establishing Thumb prologue (`PUSH`/`PUSH.W`/`SUB SP`/
///     `SUB.W SP`/`VPUSH`/`STMDB SP!`). A data word that coincidentally points into a
///     code gap almost never lands on a prologue; the pointed-to bytes of a real
///     function entry (nearly) always do.
///  3. **`check_valid_subroutine`** — the SAME validity predicate Stages 1-2 use: the
///     candidate must speculatively decode (in the painted Thumb `TMode`) into a valid
///     subroutine (> 2 instructions, reaches a clean RET / computed jump / adds-info
///     call, no undecodable byte, no out-of-image flow).
///
/// ARM-gated (Thumb code pointers): a strict no-op on every non-ARM object. The result
/// is the accepted target VMAs, address-sorted, fed to the recursive-descent walk as
/// ADDITIONAL seeds (parallel to Stages 1-2), so the walk expands each into a full
/// function and discovers its callees. Gated end-to-end by the same
/// `funcstart_patterns` discovery flag its caller checks — no new stage-model option —
/// so x86-64 / console / datatest are byte-identical.
pub fn code_pointer_table_seeds(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    use object::read::Object;
    use object::read::ObjectSection;
    // Thumb code pointers are an ARM-only notion; a strict no-op elsewhere.
    if file.architecture() != object::Architecture::Arm {
        return Vec::new();
    }
    let little = file.is_little_endian();

    // Executable section bytes, for the prologue byte-check of a pointer target.
    let execs = crate::entry::executable_sections(file);
    let exec_bytes_at = |vma: u64, n: usize| -> Option<Vec<u8>> {
        for &(lo, hi, ref data) in &execs {
            if vma >= lo && vma < hi {
                let off = (vma - lo) as usize;
                if off + n <= data.len() {
                    return Some(data[off..off + n].to_vec());
                }
                return None;
            }
        }
        None
    };
    let in_exec = |vma: u64| exec_ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi);

    // Collect candidate targets: 4-byte-aligned code pointers (Thumb bit set) in any
    // allocated section, address-sorted + deduped. Address order lets the `claimed`
    // body-dedup see a real entry before a pointer into its interior.
    let mut candidates: BTreeSet<u64> = BTreeSet::new();
    for sec in file.sections() {
        let sec_addr = sec.address();
        if sec_addr == 0 {
            continue;
        }
        let Ok(data) = sec.data() else { continue };
        // Snap to the first 4-aligned VMA in the section, then stride by four.
        let mut off = (4 - (sec_addr as usize & 3)) & 3;
        while off + 4 <= data.len() {
            let w = if little {
                u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            } else {
                u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            } as u64;
            // A Thumb function pointer has bit 0 set and points into executable memory.
            if (w & 1) != 0 && in_exec(w & !1) {
                candidates.insert(w & !1);
            }
            off += 4;
        }
    }

    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted: BTreeSet<u64> = BTreeSet::new();
    let mut claimed: BTreeSet<u64> = BTreeSet::new();

    for target in candidates {
        // gap-only + not already inside an accepted Stage-3 routine.
        if !listing.is_undefined(target) || claimed.contains(&target) {
            continue;
        }
        // The pointed-to entry must begin with a stack-frame prologue (the precision
        // signal that separates a real function pointer from a coincidental code
        // pointer / mid-function label / misread data descriptor).
        let Some(bytes) = exec_bytes_at(target, 4) else { continue };
        if !is_thumb_function_prologue(&bytes) {
            continue;
        }
        let gap_hi = listing.next_instruction_start_after(target).unwrap_or(u64::MAX);
        if let Some(body) = check_valid_subroutine(&mut decoder, listing, target, target, gap_hi) {
            accepted.insert(target);
            claimed.extend(body);
        }
    }

    accepted.into_iter().collect()
}

/// True iff the bytes at `data[0..]` are a canonical **stack-frame-establishing Thumb
/// prologue** — a broader set than [`is_thumb_lr_prologue`] (which requires the LR
/// bit), admissible here only because a corroborating code-pointer reference already
/// vouches for the target (see [`code_pointer_table_seeds`]). Little-endian (Cortex-M);
/// big-endian ARM would byte-swap, but Cortex-M is LE and this is the gated target.
///
/// Recognized entry prologues (first halfword):
///  - `PUSH {registers}` T1: `0xB4xx`/`0xB5xx` (with or without LR) — stored LE as
///    `[xx, 0xB4|0xB5]`, so `data[1] ∈ {0xB4, 0xB5}`.
///  - `SUB SP, SP, #imm` T1: `1011 0000 1 imm7` = `0xB0(0x80|imm7)` — `data[1]==0xB0`
///    with `data[0]` bit 7 set (the frame-allocation form, not `ADD SP`).
///  - `PUSH.W {registers}` / `STMDB SP!` T2: `0xE92D..` — LE `[0x2D, 0xE9, ..]`.
///  - `VPUSH {..}` T2: `0xED2D..` — LE `[0x2D, 0xED, ..]`.
///  - `SUB.W SP, SP, #imm` T2/T3: `0xF1AD..`/`0xF2AD..` — LE `[0xAD, 0xF1|0xF2, ..]`.
fn is_thumb_function_prologue(data: &[u8]) -> bool {
    if data.len() < 2 {
        return false;
    }
    // PUSH {..} (16-bit): 0xB4xx / 0xB5xx.
    if data[1] == 0xB4 || data[1] == 0xB5 {
        return true;
    }
    // SUB SP, SP, #imm (16-bit): 0xB0 with the imm7 form bit (bit 7 of the low byte).
    if data[1] == 0xB0 && (data[0] & 0x80) != 0 {
        return true;
    }
    if data.len() < 4 {
        return false;
    }
    // PUSH.W {..} / STMDB SP! (32-bit): first halfword 0xE92D.
    if data[0] == 0x2D && data[1] == 0xE9 {
        return true;
    }
    // VPUSH {..} (32-bit): first halfword 0xED2D.
    if data[0] == 0x2D && data[1] == 0xED {
        return true;
    }
    // SUB.W SP, SP, #imm (32-bit): first halfword 0xF1AD / 0xF2AD.
    if data[0] == 0xAD && (data[1] == 0xF1 || data[1] == 0xF2) {
        return true;
    }
    false
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
    fn thumb_lr_prologue_matcher() {
        // PUSH {r4,lr} = 0xB510 -> LE bytes [0x10, 0xB5]; the second byte 0xB5 is
        // the discriminant (angr's `[\x00-\xff]\xb5`).
        assert!(is_thumb_lr_prologue(&[0x10, 0xB5], 0));
        assert!(is_thumb_lr_prologue(&[0xF0, 0xB5], 0)); // PUSH {r4-r7,lr}
        assert!(is_thumb_lr_prologue(&[0x00, 0xB5], 0)); // PUSH {lr}
        // PUSH.W {..,lr} = 0xE92D 0x4???  -> LE bytes [0x2D,0xE9, lo, hi] with the
        // LR bit (bit 14) set in `hi` (0x40).  0xE92D4800 = PUSH.W {r11,lr}.
        assert!(is_thumb_lr_prologue(&[0x2D, 0xE9, 0x00, 0x48], 0));
        assert!(is_thumb_lr_prologue(&[0x2D, 0xE9, 0xF0, 0x4F], 0)); // {r4-r11,lr}
        // A non-LR STMDB SP! (0xE92D 0x0???) is NOT a prologue — no LR bit.
        assert!(!is_thumb_lr_prologue(&[0x2D, 0xE9, 0x00, 0x0F], 0));
        // Random bytes / not-a-prologue.
        assert!(!is_thumb_lr_prologue(&[0x00, 0x00], 0));
        assert!(!is_thumb_lr_prologue(&[0xB5, 0x10], 0)); // 0xB5 in the wrong (low) byte
        // Bounds: too short for a 32-bit check falls back to the 16-bit test.
        assert!(!is_thumb_lr_prologue(&[0x2D, 0xE9], 0)); // no 2nd halfword => reject
        assert!(!is_thumb_lr_prologue(&[0x2D], 0));
    }

    #[test]
    fn thumb_function_prologue_matcher() {
        // PUSH {r4,lr} = 0xB510 -> LE [0x10,0xB5]; PUSH {r4} (no LR) = 0xB410.
        assert!(is_thumb_function_prologue(&[0x10, 0xB5]));
        assert!(is_thumb_function_prologue(&[0x10, 0xB4])); // push without LR (Stage-2 skips this)
        // SUB SP,SP,#8 = 0xB082 -> LE [0x82,0xB0]; the imm7-form bit (0x80) is set.
        assert!(is_thumb_function_prologue(&[0x82, 0xB0]));
        assert!(is_thumb_function_prologue(&[0x84, 0xB0])); // SUB SP,#0x10
        // ADD SP,SP,#imm = 0xB0(0x00|imm7) has bit7 CLEAR -> not a frame prologue.
        assert!(!is_thumb_function_prologue(&[0x02, 0xB0]));
        // PUSH.W {..} = 0xE92D.. ; VPUSH = 0xED2D.. ; SUB.W SP = 0xF1AD/0xF2AD.
        assert!(is_thumb_function_prologue(&[0x2D, 0xE9, 0x00, 0x48]));
        assert!(is_thumb_function_prologue(&[0x2D, 0xED, 0x04, 0x8B])); // vpush {d8}
        assert!(is_thumb_function_prologue(&[0xAD, 0xF1, 0x08, 0x0D])); // sub.w sp,sp,#8
        assert!(is_thumb_function_prologue(&[0xAD, 0xF2, 0x08, 0x0D]));
        // Non-prologues.
        assert!(!is_thumb_function_prologue(&[0x00, 0x00]));
        assert!(!is_thumb_function_prologue(&[0xB5, 0x10])); // 0xB5 in the wrong byte
        assert!(!is_thumb_function_prologue(&[0x2D, 0xE9])); // truncated 32-bit -> reject
        assert!(!is_thumb_function_prologue(&[0x2D]));
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
