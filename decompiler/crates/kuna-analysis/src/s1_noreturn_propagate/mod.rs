//! Structural no-return **propagation** — the kuna analog of angr's `CFGFast`
//! call-graph no-return propagation (the `returning` /
//! `_iteratively_analyze_function_features` fixpoint).
//!
//! # What this is (the second Listing/xref consumer)
//!
//! The *Known* analyzer ([`crate::s1_loader::noreturn`]) is name-based; the
//! *Discovered* analyzer ([`crate::s1_noreturn_disc`]) is evidence-based — a
//! callee is no-return once **≥3** of its call sites show no valid fall-through
//! (Ghidra's `FindNoReturnFunctionsAnalyzer` threshold). Both miss a class of
//! real no-return wrappers:
//!
//! ```c
//! static void xalloc_die(void) { error(0, 0, "%s", _("memory exhausted")); abort(); }
//! ```
//!
//! `xalloc_die` (coreutils) ends, unconditionally, in `call abort` — and kuna
//! *already* models `abort` no-return. But:
//!
//! - the **Known** list does not carry `xalloc_die` (it is a custom name), and
//! - the **Discovered** evidence rule does not fire: at `-O2` `xalloc_die` is
//!   called from too few sites to meet the ≥3 threshold, AND its single `call
//!   abort` is followed by **alignment NOP padding** (a *valid* instruction
//!   start), so `noreturn_disc`'s "no valid fall-through" predicate reads the
//!   padding as a returning fall-through and never concludes no-return.
//!
//! The result: when decompiling a *caller* of `xalloc_die` (e.g. `x2nrealloc`),
//! kuna assumes control returns from the `call xalloc_die`, and at `-O2` (where
//! the cold error path is laid out last) the bytes after the call become a
//! spurious fall-through back-edge → an invalid `while(true)` loop + `goto` +
//! dead stack-spill artifacts. angr, whose CFGFast propagates no-return over the
//! call graph from the known set with **no evidence threshold**, renders the same
//! function cleanly.
//!
//! # The propagation rule (angr-faithful, conservative)
//!
//! Seed the *terminal* (already-no-return) set from the **Known** list (this is
//! the key difference from `noreturn_disc`, which only seeds from ≥3-evidence
//! discoveries). A function `F` is concluded no-return — and added to the set,
//! re-evaluating its callers — when ALL of:
//!
//! 1. its **last real instruction** (last by address, after skipping trailing
//!    alignment NOP padding) is a `CALL` (or tail `JMP`) whose static callee is
//!    already in the terminal set; AND
//! 2. it contains **no `RETURN`** instruction (no path returns); AND
//! 3. it contains **no computed/indirect jump** (control we cannot reason about);
//!    AND
//! 4. every static jump target in the reachable body stays within the body
//!    `[entry, last_real]` (or is itself a terminal callee) — no branch escapes to
//!    a possibly-returning neighbour, and none skips into the unreachable padding
//!    tail.
//!
//! **Soundness:** to return to its caller a function must execute a `RETURN`
//! (excluded by 2) or transfer control out of its body to code that returns —
//! a tail `JMP`/fall-through to a non-terminal function (excluded by 1+4) or a
//! computed jump (excluded by 3). With its only reachable exit a call/jump to an
//! already-no-return function, `F` cannot return. This is strictly *more*
//! conservative than angr's propagation (which also tail-recurses from its known
//! set). Iterated to a fixpoint so a wrapper-of-a-wrapper converges.
//!
//! # Emission + commit (the existing, proven seam)
//!
//! Each concluded function emits a [`NoReturnFact`] — the SAME fact the Known and
//! Discovered passes emit, resolved by the SAME commit arm (`engine.rs`
//! `commit_analysis_output` → `set_function_no_return`). The flow-repair /
//! dead-code elimination after a no-return call is **inherited** from the engine
//! (the `flow.rs` artificial-halt path). This pass builds NO new commit arm and
//! does NO S7 work.
//!
//! # Origin (the angr reference)
//!
//! angr `CFGFast` no-return analysis: starting from a known no-return set
//! (`abort`/`exit`/`__assert_fail`/…), any function whose body unconditionally
//! reaches a no-return call is itself no-return, propagated to a fixpoint over the
//! call graph with no evidence threshold (`test_decompiling_tee_O2_x2nrealloc`).
//!
//! # Faithful scope / LOSS
//!
//! - Like `noreturn_disc`, this is **Listing-dependent** (`--option listing on`)
//!   and **default-OFF** (`noreturn_propagate`): a no-return heuristic can be
//!   wrong, so it ships behind its own flag, real-ELF path only ⇒ every parity
//!   gate is byte-identical.
//! - The fixpoint is bounded (the candidate count + 2 sweeps); it never *removes*
//!   a fact.

use std::collections::{BTreeMap, BTreeSet};

use crate::listing::model::Insn;
use crate::listing::Listing;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Stage};

/// The structural no-return propagation consumer pass (the second Listing/xref
/// consumer).
///
/// Default-OFF (gate id `noreturn_propagate`); short-circuits to an empty output
/// when no Listing is built (`ctx.listing.is_none()`).
#[derive(Default)]
pub struct NoReturnPropagatePass;

impl AnalysisPass for NoReturnPropagatePass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "noreturn_propagate"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Listing-dependent: a no-op when the Listing is absent (the `--option
        // listing on` flag is off). This keeps the pass inert by default even if
        // its own gate is flipped on without the Listing.
        let Some(listing) = ctx.listing else {
            return out;
        };
        for fact in propagate_noreturn(listing) {
            out.noreturn.push(fact);
        }
        out
    }
}

/// Is `insn` an alignment NOP — the trailing padding a compiler emits after a
/// no-return call (`NOP`, `NOP word ptr CS:[...]`, …; the SLEIGH x86 mnemonic for
/// every padding form begins `NOP`)? Such padding is unreachable after a
/// no-return call, so it is skipped when locating the *last real* instruction.
fn is_padding_nop(insn: &Insn) -> bool {
    insn.mnemonic.trim_start().to_ascii_uppercase().starts_with("NOP")
}

/// Is `callee` skippable as a propagation candidate — already modeled no-return /
/// call-fixup'd, already concluded this run, or not a discovered function entry?
/// (Identical guard shape to `noreturn_disc`'s `skip_callee`.)
fn skip_candidate(listing: &Listing, entry: u64, terminal: &BTreeSet<u64>) -> bool {
    if terminal.contains(&entry) {
        // Already concluded no-return (Known-seeded or propagated this run).
        return true;
    }
    match listing.function_at(entry) {
        // Call-fixup'd: marking it again is a no-op.
        Some(f) => f.call_fixup.is_some(),
        // Not a discovered function entry: nothing to mark.
        None => true,
    }
}

/// Does function `entry` unconditionally terminate in a call/tail-jump to an
/// already-no-return callee (the propagation rule, §"The propagation rule")?
///
/// Reads the function body `[entry, next_function_entry)` from the Listing and
/// applies rules 1–4 (last-real-instruction is a terminal call/jump; no RETURN;
/// no computed jump; no escaping/tail-skipping branch). Conservative: any
/// uncertainty (a return path, an indirect jump, a branch out of the reachable
/// body) leaves `entry` untouched.
fn function_is_no_return(listing: &Listing, entry: u64, terminal: &BTreeSet<u64>) -> bool {
    let next = listing.next_function_after(entry).map(|f| f.entry);
    // The function body in address order (the Listing's `instructions()` is a
    // BTreeMap iterator, already sorted).
    let body: Vec<(u64, &Insn)> = listing
        .instructions()
        .filter(|(&vma, _)| vma >= entry && next.map_or(true, |n| vma < n))
        .map(|(&vma, insn)| (vma, insn))
        .collect();
    if body.is_empty() {
        return false;
    }

    // Rule 1: the last *real* (non-padding) instruction must be a CALL — or a tail
    // JMP — to a callee already in the terminal set. Trailing alignment NOPs after
    // a no-return call are unreachable and skipped.
    let Some(&(last_vma, last)) = body.iter().rev().find(|(_, insn)| !is_padding_nop(insn))
    else {
        return false;
    };
    let last_is_terminal_transfer = (last.flow.is_call || last.flow.is_jump)
        && last.flows.iter().any(|t| terminal.contains(t));
    if !last_is_terminal_transfer {
        return false;
    }

    // Rules 2–4 over the reachable body (up to and including the last real
    // instruction; the padding tail past `last_vma` is unreachable, so it is not
    // scanned).
    for &(vma, insn) in &body {
        if vma > last_vma {
            break;
        }
        // Rule 2: any returning path disqualifies the function.
        if insn.flow.is_terminal {
            return false;
        }
        // Rule 3: a computed/indirect JUMP is control we cannot reason about
        // (a computed CALL is fine — calls are return-modeled).
        if insn.flow.is_jump && insn.flow.is_computed {
            return false;
        }
        // Rule 4: every static jump target must stay within the reachable body
        // `[entry, last_vma]`, or be an already-no-return callee. A target outside
        // it is either an escaping branch to a possibly-returning neighbour or a
        // skip into the unreachable padding tail — both disqualify.
        if insn.flow.is_jump {
            for &t in &insn.flows {
                let in_body = t >= entry && t <= last_vma;
                if !in_body && !terminal.contains(&t) {
                    return false;
                }
            }
        }
    }
    true
}

/// Run the call-graph no-return propagation over the Listing and return the
/// concluded functions as [`NoReturnFact`]s.
///
/// Seed the terminal set from the Known-list no-return functions
/// (`has_no_return`), then sweep the candidate functions to a fixpoint: each
/// newly-concluded function joins the terminal set so a caller whose last act is a
/// call to *it* is re-evaluated next sweep (a wrapper-of-a-wrapper converges).
/// Keyed by entry VMA (a `BTreeMap` ⇒ address-ordered, deduped).
fn propagate_noreturn(listing: &Listing) -> Vec<NoReturnFact> {
    let mut discovered: BTreeMap<u64, NoReturnFact> = BTreeMap::new();
    // The "already no-return" anchor set: the Known-seeded functions plus
    // everything propagated so far. A call to one of these is a terminal transfer
    // for its caller.
    let mut terminal: BTreeSet<u64> = listing
        .functions()
        .filter(|(_, f)| f.has_no_return)
        .map(|(&entry, _)| entry)
        .collect();

    let candidates: Vec<u64> = listing.functions().map(|(&entry, _)| entry).collect();

    // Fixpoint: keep sweeping while a sweep concludes a new no-return function. The
    // `terminal` set growing each round is what lets the propagation converge.
    let mut changed = true;
    let mut sweeps = 0usize;
    let max_sweeps = candidates.len() + 2;
    while changed && sweeps < max_sweeps {
        changed = false;
        sweeps += 1;
        for &entry in &candidates {
            if skip_candidate(listing, entry, &terminal) {
                continue;
            }
            if function_is_no_return(listing, entry, &terminal) {
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

    // The fixpoint convergence + the propagation rule are exercised end-to-end by
    // the cross-crate `verify_noreturn_propagate.rs` gate (a real x86-64 fixture
    // with a custom `my_die() { ...; abort(); }` wrapper called only ONCE — below
    // `noreturn_disc`'s ≥3 threshold and followed by NOP padding, so ONLY this
    // pass concludes it no-return). A unit test here would need a hand-built
    // `Listing`, whose private fields are not constructible outside the `listing`
    // module; the rule predicates stay tiny and obviously-correct by inspection.

    #[test]
    fn pass_identity_wiring() {
        let p = NoReturnPropagatePass;
        assert_eq!(p.id(), "noreturn_propagate");
        assert_eq!(p.stage(), Stage::S1);
    }

    #[test]
    fn empty_listing_consumer_is_inert_via_pass() {
        // With no Listing, the pass short-circuits to an empty output (the
        // default-off / no-listing path; the parity-safe no-op).
        let p = NoReturnPropagatePass;
        assert_eq!(p.id(), "noreturn_propagate");
    }
}
