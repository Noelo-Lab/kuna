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
        // (kuna, decbench F2) The `error(status,…)`-conditional recognizer:
        // glibc `error`/`error_at_line` never return WHEN the `status` argument is
        // a nonzero constant (they call `exit(status)`), so a wrapper whose tail is
        // `call error(2,…)` is itself no-return — but `error` is NOT on the Known
        // no-return list (it *returns* for `error(0,…)`). The recognizer is a
        // per-call-site value check (arg0 is a nonzero literal), gated by its own
        // `--option noreturn_error` flag (default-on, DIV-16). When on, compute the
        // `error`/`error_at_line` entry addresses (PLT stubs + any defined funcsym)
        // so the propagation rule can treat a `call error(nonzero)` tail as terminal.
        let error_recog = if ctx.arch.analysis_noreturn_error {
            ErrorRecognizer::from_object(ctx.file, ctx.bytes, listing)
        } else {
            ErrorRecognizer::disabled()
        };
        for fact in propagate_noreturn(listing, &error_recog) {
            out.noreturn.push(fact);
        }
        out
    }
}

/// The `error(status,…)`-conditional no-return recognizer (decbench F2).
///
/// glibc `error(int status, int errnum, const char *fmt, …)` and
/// `error_at_line(int status, …)` call `exit(status)` and **never return** when
/// `status != 0` — but *do* return for `status == 0`. So `error` cannot be a plain
/// Known no-return, yet a wrapper whose tail is `call error(2,…)` (e.g. GNU
/// `pfatal_with_name`) is unconditionally no-return. This recognizer holds the
/// `error`/`error_at_line` entry addresses and answers, per call site, whether the
/// `status` argument (x86-64 SysV first int arg = `EDI`/`RDI`) is a nonzero
/// constant, so [`function_is_no_return`] can treat that tail call as terminal.
///
/// This is exactly the value-conditional slice of Ghidra's discovered-no-return
/// analyzer (which flags a `call error` with a nonzero status "Subroutine does not
/// return"); it reuses the *same* propagation fixpoint and the same `NoReturnFact`
/// commit seam — no parallel machinery.
struct ErrorRecognizer {
    /// Entry addresses (PLT stub + any defined funcsym) of `error`/`error_at_line`.
    /// Empty ⇒ the recognizer is inert (disabled, or no `error` import present).
    error_addrs: BTreeSet<u64>,
}

impl ErrorRecognizer {
    /// A disabled recognizer (no addresses ⇒ never matches).
    fn disabled() -> Self {
        ErrorRecognizer { error_addrs: BTreeSet::new() }
    }

    /// Collect the `error`/`error_at_line` entry addresses from the object: the
    /// dynamic-import PLT stubs (`resolve_imports`, the same stream the Known
    /// no-return pass scans) plus any defined/named funcsym the Listing carries
    /// (the unstripped-static case). Version-suffix stripped like the Known pass.
    fn from_object(file: &object::File, bytes: &[u8], listing: &Listing) -> Self {
        let mut error_addrs = BTreeSet::new();
        let is_error_name = |n: &str| n == "error" || n == "error_at_line";
        // Dynamic imports (PLT stubs): the common dynamically-linked case.
        for p in crate::s1_loader::format::resolve_imports(file, bytes) {
            if let Ok(n) =
                String::from_utf8(crate::s1_loader::elf_plt::strip_version(&p.name))
            {
                if is_error_name(n.trim_start_matches('_')) {
                    error_addrs.insert(p.addr);
                }
            }
        }
        // Named functions in the Listing (a defined/statically-linked `error`).
        for (&entry, f) in listing.functions() {
            if let Some(n) = f.name.as_deref() {
                if is_error_name(n.trim_start_matches('_')) {
                    error_addrs.insert(entry);
                }
            }
        }
        ErrorRecognizer { error_addrs }
    }

    /// Is `entry` the `error`/`error_at_line` function?
    fn is_error(&self, entry: u64) -> bool {
        self.error_addrs.contains(&entry)
    }

    /// Is the last (tail) instruction of a body a `call error`/`call error_at_line`
    /// whose `status` argument is a **nonzero constant** — i.e. the call (and so the
    /// whole no-returning-path body) never returns?
    ///
    /// `call_idx` indexes the tail CALL in `body`. Returns true only when (a) the
    /// call target is an `error` address and (b) the last write to the first int-arg
    /// register (x86-64 SysV `EDI`/`RDI`, the `int status`) before the call is a
    /// `MOV reg, imm` with a nonzero literal. A zero status (`XOR EDI,EDI` /
    /// `MOV EDI,0x0` — `error` *returns*) or any non-constant / unprovable definition
    /// is rejected (conservative: a false positive would drop live caller code).
    fn call_is_terminal_error(&self, body: &[(u64, &Insn)], call_idx: usize) -> bool {
        let (_, call) = body[call_idx];
        if !call.flow.is_call {
            return false;
        }
        if !call.flows.iter().any(|t| self.is_error(*t)) {
            return false;
        }
        arg0_is_nonzero_const(body, call_idx)
    }
}

/// Uppercase register token of the first integer argument on x86-64 SysV — the
/// `int status` slot of `error()`/`error_at_line()`. The 64-/32-/16-/8-bit views
/// of `rdi`; on any non-x86 target none of these appear as a destination operand,
/// so the whole recognizer degrades to a clean no-op (no false positives).
fn is_arg0_reg(reg: &str) -> bool {
    matches!(reg.to_ascii_uppercase().as_str(), "RDI" | "EDI" | "DI" | "DIL")
}

/// Parse an operand token as an integer literal (SLEIGH x86 emits hex `0x2` /
/// `-0x1`; decimal is accepted defensively). `None` ⇒ not a literal.
fn parse_imm(tok: &str) -> Option<i64> {
    let t = tok.trim();
    let (neg, t) = t.strip_prefix('-').map_or((false, t), |r| (true, r));
    let v = if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        i64::from_str_radix(h, 16).ok()?
    } else {
        t.parse::<i64>().ok()?
    };
    Some(if neg { -v } else { v })
}

/// Backward-scan the straight-line predecessors of the tail `call` (`call_idx`)
/// for the defining write of the first int-arg register, returning true iff that
/// write is a nonzero **constant** load.
///
/// Conservative by construction: it stops (and returns false) at the first
/// control-flow instruction or intervening `call` (beyond which the straight-line
/// definition we depend on no longer dominates the call and `edi`/`rdi` may be
/// clobbered), and returns false for a zero literal (`error(0,…)` *returns*) or any
/// non-literal / unrecognised definition. A false positive would mark a returning
/// function no-return and drop live caller code, so every uncertain case is a no.
fn arg0_is_nonzero_const(body: &[(u64, &Insn)], call_idx: usize) -> bool {
    let mut i = call_idx;
    while i > 0 {
        i -= 1;
        let insn = body[i].1;
        // A branch/return between here and the call breaks straight-line dominance.
        if insn.flow.is_jump || insn.flow.is_terminal {
            return false;
        }
        let ops = insn.operands.trim();
        let mut parts = ops.splitn(2, ',');
        let dest = parts.next().unwrap_or("").trim();
        let src = parts.next().unwrap_or("").trim();
        let mnem = insn.mnemonic.trim().to_ascii_uppercase();
        if is_arg0_reg(dest) {
            // The nearest definition of the status register: decide here.
            return match mnem.as_str() {
                // `MOV EDI, <imm>`: nonzero literal ⇒ error() never returns.
                "MOV" => parse_imm(src).is_some_and(|v| v != 0),
                // `XOR EDI,EDI` / `SUB EDI,EDI`: sets zero ⇒ error() *returns*.
                _ => false,
            };
        }
        // A call before the definition clobbers the caller-saved arg register.
        if insn.flow.is_call {
            return false;
        }
    }
    false
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
fn function_is_no_return(
    listing: &Listing,
    entry: u64,
    terminal: &BTreeSet<u64>,
    error_recog: &ErrorRecognizer,
) -> bool {
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
    let Some(last_idx) = body.iter().rposition(|(_, insn)| !is_padding_nop(insn)) else {
        return false;
    };
    let (last_vma, last) = body[last_idx];
    let last_is_terminal_transfer = (last.flow.is_call || last.flow.is_jump)
        && last.flows.iter().any(|t| terminal.contains(t));
    // (kuna, decbench F2) …or a tail `call error(nonzero,…)`: glibc error()/
    // error_at_line() with a nonzero constant status calls exit() and never
    // returns, so the wrapper is no-return even though `error` itself is NOT on the
    // Known no-return list (it returns for error(0,…)). Value-conditional, checked
    // per call site by the recognizer (arg0 is a nonzero literal).
    let last_is_terminal_error = error_recog.call_is_terminal_error(&body, last_idx);
    if !last_is_terminal_transfer && !last_is_terminal_error {
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
fn propagate_noreturn(listing: &Listing, error_recog: &ErrorRecognizer) -> Vec<NoReturnFact> {
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
            if function_is_no_return(listing, entry, &terminal, error_recog) {
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

    // --- error(nonzero,…) recognizer predicates (decbench F2) ------------------

    #[test]
    fn arg0_reg_is_the_x86_64_first_int_arg() {
        for r in ["EDI", "RDI", "edi", "rdi", "DI", "DIL"] {
            assert!(is_arg0_reg(r), "{r} is the x86-64 SysV first int arg");
        }
        // Other regs / a target with no `EDI` are a clean no-op.
        for r in ["EAX", "ESI", "RSI", "R0", "W0", ""] {
            assert!(!is_arg0_reg(r), "{r} is not the first int arg");
        }
    }

    #[test]
    fn parse_imm_reads_hex_and_decimal() {
        assert_eq!(parse_imm("0x2"), Some(2));
        assert_eq!(parse_imm("0x0"), Some(0));
        assert_eq!(parse_imm(" 0xff "), Some(255));
        assert_eq!(parse_imm("-0x1"), Some(-1));
        assert_eq!(parse_imm("7"), Some(7));
        // Not a literal (a register / memory operand).
        assert_eq!(parse_imm("EAX"), None);
        assert_eq!(parse_imm("[rax]"), None);
    }

    /// Build a minimal straight-line `Insn` (fields the recognizer reads).
    fn insn(mnemonic: &str, operands: &str, is_call: bool, is_jump: bool) -> Insn {
        let mut flow = crate::listing::model::FlowType::default();
        flow.is_call = is_call;
        flow.is_jump = is_jump;
        Insn {
            addr: 0,
            len: 1,
            fall_through: Some(1),
            flow,
            flows: vec![],
            mnemonic: mnemonic.into(),
            operands: operands.into(),
            pcode: None,
        }
    }

    #[test]
    fn arg0_nonzero_const_recognizes_mov_edi_imm() {
        // …; MOV EDI,0x2 ; XOR EAX,EAX ; CALL error  → status 2 (nonzero).
        let a = insn("MOV", "EDI,0x2", false, false);
        let b = insn("XOR", "EAX,EAX", false, false);
        let c = insn("CALL", "0x4de0", true, false);
        let body: Vec<(u64, &Insn)> = vec![(0, &a), (1, &b), (2, &c)];
        assert!(arg0_is_nonzero_const(&body, 2), "MOV EDI,0x2 is a nonzero status");
    }

    #[test]
    fn arg0_nonzero_const_rejects_zero_and_nonconst_and_intervening_call() {
        let call = insn("CALL", "0x4de0", true, false);
        // XOR EDI,EDI → 0 (error() returns).
        let z = insn("XOR", "EDI,EDI", false, false);
        assert!(!arg0_is_nonzero_const(&vec![(0, &z), (1, &call)], 1));
        // MOV EDI,0x0 → 0.
        let m0 = insn("MOV", "EDI,0x0", false, false);
        assert!(!arg0_is_nonzero_const(&vec![(0, &m0), (1, &call)], 1));
        // MOV EDI,EAX → not a literal (unprovable) → conservative reject.
        let mr = insn("MOV", "EDI,EAX", false, false);
        assert!(!arg0_is_nonzero_const(&vec![(0, &mr), (1, &call)], 1));
        // MOV EDI,0x2 then an intervening CALL clobbers edi → reject.
        let m2 = insn("MOV", "EDI,0x2", false, false);
        let clobber = insn("CALL", "0x1234", true, false);
        assert!(!arg0_is_nonzero_const(&vec![(0, &m2), (1, &clobber), (2, &call)], 2));
        // A branch between the def and the call breaks dominance → reject.
        let br = insn("JMP", "0x9", false, true);
        assert!(!arg0_is_nonzero_const(&vec![(0, &m2), (1, &br), (2, &call)], 2));
        // No writer at all → reject.
        assert!(!arg0_is_nonzero_const(&vec![(0, &call)], 0));
    }

    #[test]
    fn empty_listing_consumer_is_inert_via_pass() {
        // With no Listing, the pass short-circuits to an empty output (the
        // default-off / no-listing path; the parity-safe no-op).
        let p = NoReturnPropagatePass;
        assert_eq!(p.id(), "noreturn_propagate");
    }
}
