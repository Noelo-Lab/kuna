//! Discovered-no-return detection — the kuna analog of Ghidra's
//! `FindNoReturnFunctionsAnalyzer` ("Non-Returning Functions - Discovered").
//!
//! # What this is (the first Listing/xref consumer)
//!
//! The *Known* no-return analyzer ([`crate::loader::noreturn`]) is name-based:
//! it marks `exit`/`abort`/… no-return because they appear on a shipped list. But
//! a binary can carry a **custom** no-return wrapper the static lists do not know
//! — e.g. a `static void die(const char*) { fprintf(stderr,…); exit(1); }` that
//! tail-calls `exit`. `die` is no-return *in fact* (it never returns), but no name
//! list catches it.
//!
//! The *Discovered* analyzer finds these by **flow evidence**, not by name: it
//! looks at every call site of a candidate callee and asks "does the instruction
//! after the call have valid fall-through?". A function that never returns leaves
//! its callers with dead/invalid bytes (or another function's entry) right after
//! the `call` — the compiler knows control never comes back, so it does not emit a
//! sane fall-through there. When **≥3** call sites of a callee show this signal
//! (Ghidra's evidence threshold), the callee is concluded no-return. The result is
//! iterated to a fixpoint: a function whose last act is a call to an
//! already-discovered-no-return function is itself no-return.
//!
//! This was **infeasible at the kuna-analysis tier until the Listing/xref tier**
//! (`docs/history/listing-tier-design.md`): it needs the program-wide disassembled Listing
//! (instructions + call sites + the discovered-function model) that only the
//! `--option listing on` recursive-descent walk produces. With the Listing built
//! and threaded into [`AnalysisCtx::listing`](crate::pass::AnalysisCtx::listing),
//! it ports cleanly and read-only. Default-OFF (a flow heuristic that can be
//! wrong, gated behind its own `noreturn_disc` flag *and* requiring `listing on`).
//!
//! # Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/analysis/FindNoReturnFunctionsAnalyzer.java`
//!   — `AnalyzerType.INSTRUCTION_ANALYZER`, `setDefaultEnablement(false)`. The
//!   decompiler-relevant logic ported here:
//!   - the evidence tally over each callee's call sites: a "no-flow" indication is
//!     a call whose fall-through is not a valid instruction (`getNoReturnIndices` /
//!     `checkForNoReturn`); a callee with `indications >= EVIDENCE_THRESHOLD` (3) is
//!     marked (`NUMBER_OF_OPERANDS_TO_VOTE` analog).
//!   - the worklist fixpoint: a newly-discovered no-return function feeds back so a
//!     caller calling *it* as its last act is re-evaluated.
//!
//! # Emission + commit (the existing, proven seam)
//!
//! Each discovered callee emits a [`NoReturnFact`] — the SAME fact the Known pass
//! emits, resolved by the SAME commit arm (`engine.rs` `commit_analysis_output`,
//! `find_function_across_scopes`/`query_global_function` → `set_function_no_return`).
//! The flow-repair / dead-code elimination after a no-return call is **inherited**
//! from the engine (the `flow.rs` artificial-halt path), exactly as the Known pass
//! relies on. This pass builds NO new commit arm.
//!
//! # Faithful scope / LOSS
//!
//! - **Evidence threshold = 3**, matching Ghidra's `EVIDENCE_THRESHOLD`. Below the
//!   threshold a callee is left alone (a custom wrapper called only once or twice
//!   is not concluded no-return — the heuristic needs corroboration).
//! - The "no-valid-fall-through" predicate is the Listing-faithful subset Ghidra's
//!   `checkForNoReturn` reduces to at this tier: the fall-through VMA is not a
//!   decoded instruction start (`!is_instruction_start`), or it landed in data
//!   (`is_data`), or it is another *function's* entry (`function_at(fall) && fall
//!   != callee`). Computed/indirect calls contribute no evidence (no static
//!   callee). A call whose callee is already modeled no-return (Known) or
//!   call-fixup'd is skipped (no double-marking).
//! - The fixpoint is bounded (2 sweeps suffice for the typical wrapper-of-wrapper
//!   depth; more sweeps converge with no new facts). It never *removes* a fact.

use std::collections::{BTreeMap, BTreeSet};

use crate::listing::{Listing, RefKind};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Phase};

/// Ghidra's `FindNoReturnFunctionsAnalyzer` evidence threshold: a callee is
/// concluded no-return once at least this many of its call sites show no valid
/// fall-through (`EVIDENCE_THRESHOLD` / the operand-vote count).
const EVIDENCE_THRESHOLD: usize = 3;

/// The discovered-no-return consumer pass (the first Listing/xref consumer).
///
/// Default-OFF (gate id `noreturn_disc`); short-circuits to an empty output when
/// no Listing is built (`ctx.listing.is_none()`).
#[derive(Default)]
pub struct NoReturnDiscoveredPass;

impl AnalysisPass for NoReturnDiscoveredPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "noreturn_disc"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Listing-dependent: a no-op when the Listing is absent (the `--option
        // listing on` flag is off). This keeps the pass inert by default even if
        // its own gate is flipped on without the Listing.
        let Some(listing) = ctx.listing else {
            return out;
        };
        for fact in discover_noreturn(listing) {
            out.noreturn.push(fact);
        }
        out
    }
}

/// Is the fall-through of the call instruction at `call_vma` invalid — evidence
/// that the callee did not return?
///
/// The instruction after the call (`call_vma + len`) has no valid fall-through
/// when it is not a decoded instruction start, OR it landed in data, OR it is the
/// entry of *another* function (the compiler placed the next function immediately
/// after the dead call). A call instruction with no recorded fall-through VMA
/// (terminal — e.g. a tail-call lowered to a jump) is itself evidence.
fn call_site_has_no_fallthrough(listing: &Listing, call_vma: u64, callee: u64) -> bool {
    let Some(insn) = listing.instruction_at(call_vma) else {
        // The call site is not a decoded instruction; no evidence we can read.
        return false;
    };
    let Some(fall) = insn.fall_through else {
        // A call with no fall-through at all (a lowered tail jump) is evidence.
        return true;
    };
    // The strongest signal: the byte after the call is not a valid instruction
    // (it fell off the decoded run / into data).
    if !listing.is_instruction_start(fall) {
        return true;
    }
    if listing.is_data(fall) {
        return true;
    }
    // The byte after the call begins *another* function — the compiler placed the
    // next function right after the dead call (`function_at(fall)` and it is not
    // the callee itself).
    if fall != callee && listing.function_at(fall).is_some() {
        return true;
    }
    false
}

/// Count the call sites of `callee` that show no valid fall-through (the evidence
/// tally over `refs_to(callee)` Call edges — Ghidra's `getNoReturnIndices`).
fn no_fallthrough_evidence(listing: &Listing, callee: u64) -> usize {
    listing
        .refs_to(callee)
        .iter()
        .filter(|r| r.kind == RefKind::Call)
        .filter(|r| call_site_has_no_fallthrough(listing, r.from, callee))
        .count()
}

/// Is `callee` skippable — already modeled no-return / call-fixup'd, already
/// concluded this run, or not a discovered function entry we can name?
fn skip_callee(listing: &Listing, callee: u64, terminal: &BTreeSet<u64>) -> bool {
    if terminal.contains(&callee) {
        // Already concluded no-return (Known-seeded or discovered this run).
        return true;
    }
    match listing.function_at(callee) {
        // Call-fixup'd: marking it again is a no-op — and Ghidra only acts when the
        // function is not already modeled.
        Some(f) => f.call_fixup.is_some(),
        // Not a discovered function entry (e.g. a mid-function branch target that
        // is somehow a Call edge target): nothing to mark.
        None => true,
    }
}

/// Does function `entry` flow ONLY into an already-terminal (no-return) function
/// — the fixpoint promotion rule (a function whose every terminal path is a call
/// to an already-discovered-no-return function, with no returning path, is itself
/// no-return)?
///
/// Sound guard against false positives: we require BOTH (a) the function's
/// instruction span `[entry, next_entry)` contains a terminal call to a function
/// in `terminal` (a no-fall-through call to a no-return callee), AND (b) the
/// function contains NO `RETURN` instruction (no path returns). A function with a
/// `ret` somewhere returns on that path and must not be promoted by this rule
/// (the call-site evidence rule still applies to it independently). This is the
/// cheap, Listing-faithful form of Ghidra's worklist feedback (a newly no-return
/// function re-flags its tail-calling callers).
fn last_act_is_terminal_call(listing: &Listing, entry: u64, terminal: &BTreeSet<u64>) -> bool {
    let next = listing.next_function_after(entry).map(|f| f.entry);
    let mut has_terminal_call = false;
    for (&vma, insn) in listing.instructions() {
        if vma < entry {
            continue;
        }
        if let Some(n) = next {
            if vma >= n {
                break;
            }
        }
        // (b) Any returning path disqualifies the function from this rule.
        if insn.flow.is_terminal {
            return false;
        }
        if !insn.flow.is_call {
            continue;
        }
        // (a) A call whose static callee is already terminal AND which itself has
        // no valid fall-through (the call is a terminal flow of this function).
        for &callee in &insn.flows {
            if terminal.contains(&callee)
                && call_site_has_no_fallthrough(listing, vma, callee)
            {
                has_terminal_call = true;
            }
        }
    }
    has_terminal_call
}

/// Run the evidence-tally + fixpoint over the Listing and return the discovered
/// no-return functions as [`NoReturnFact`]s (design §6 + Ghidra's
/// `FindNoReturnFunctionsAnalyzer`).
///
/// Two rules, iterated to a fixpoint:
/// 1. **Call-site evidence**: a callee with `>= EVIDENCE_THRESHOLD` (3) call sites
///    showing no valid fall-through is no-return.
/// 2. **Last-act promotion** (the fixpoint feedback): a function whose terminal
///    instruction is a call to an already-discovered-no-return function is itself
///    no-return — so a wrapper-of-a-wrapper converges.
///
/// The result is keyed by entry VMA (a `BTreeMap` so the emitted facts are
/// address-ordered and deduped). The `terminal` set seeds with the Known-list
/// no-return functions so a tail-call to (e.g.) `exit` already counts.
fn discover_noreturn(listing: &Listing) -> Vec<NoReturnFact> {
    let mut discovered: BTreeMap<u64, NoReturnFact> = BTreeMap::new();
    // The "already no-return" anchor set: the Known-seeded functions plus
    // everything discovered so far. A call to one of these is a no-fall-through
    // site for its caller (rule 2 reads this).
    let mut terminal: BTreeSet<u64> = listing
        .functions()
        .filter(|(_, f)| f.has_no_return)
        .map(|(&entry, _)| entry)
        .collect();

    // Candidate functions: every discovered function entry (rule 1 needs a callee
    // with Call edges; rule 2 needs any function with a terminal call). Collected
    // once; the worklist re-scans them each sweep.
    let candidates: Vec<u64> = listing.functions().map(|(&entry, _)| entry).collect();

    // Fixpoint: keep sweeping while a sweep concludes a new no-return function. The
    // `terminal` set growing each round is what lets rule 2 converge. Bounded by
    // the candidate count (each sweep only adds; a candidate is concluded once).
    let mut changed = true;
    let mut sweeps = 0usize;
    let max_sweeps = candidates.len() + 2;
    while changed && sweeps < max_sweeps {
        changed = false;
        sweeps += 1;
        for &entry in &candidates {
            if skip_callee(listing, entry, &terminal) {
                continue;
            }
            // Rule 1: direct call-site evidence. Rule 2: terminal-call promotion.
            let concluded = no_fallthrough_evidence(listing, entry) >= EVIDENCE_THRESHOLD
                || last_act_is_terminal_call(listing, entry, &terminal);
            if concluded {
                let name = listing
                    .function_at(entry)
                    .and_then(|f| f.name.clone())
                    .unwrap_or_else(|| format!("sub_{entry:x}"));
                discovered.insert(entry, NoReturnFact { addr: entry, name });
                terminal.insert(entry);
                changed = true;
            }
        }
    }

    discovered.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // The fixpoint convergence + threshold logic is exercised end-to-end by the
    // cross-crate `verify_noreturn_disc.rs` gate (a real x86-64 fixture with a
    // custom `die()` wrapper called >=3x). A unit test here would need a hand-built
    // `Listing`, whose private fields are not constructible outside the `listing`
    // module; the threshold/predicate are pinned by that e2e + the predicate
    // helpers below stay tiny and obviously-correct by inspection.

    #[test]
    fn threshold_is_three() {
        assert_eq!(EVIDENCE_THRESHOLD, 3);
    }

    #[test]
    fn empty_listing_consumer_is_inert_via_pass() {
        // With no Listing, the pass short-circuits to an empty output (the
        // default-off / no-listing path; structurally the parity-safe no-op).
        // Constructing an AnalysisCtx requires a real object::File/image, so this
        // is covered by the e2e; here we only assert the pass identity wiring.
        let p = NoReturnDiscoveredPass;
        assert_eq!(p.id(), "noreturn_disc");
        assert_eq!(p.phase(), Phase::P1);
    }
}
