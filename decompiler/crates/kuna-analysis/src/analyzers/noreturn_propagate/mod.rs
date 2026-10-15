//! Structural no-return **propagation** — the kuna analog of angr's `CFGFast`
//! call-graph no-return propagation (the `returning` /
//! `_iteratively_analyze_function_features` fixpoint).
//!
//! # What this is (the second Listing/xref consumer)
//!
//! The *Known* analyzer ([`crate::loader::noreturn`]) is name-based; the
//! *Discovered* analyzer ([`crate::noreturn_disc`]) is evidence-based — a
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
//!   and **default-ON since DIV-14** (`noreturn_propagate`; the Listing itself
//!   defaults off, so the datatest parity path never fires it — decompile-all
//!   builds the Listing per DIV-15). A no-return heuristic can be wrong, so it
//!   keeps its own flag: `option noreturn_propagate off` restores upstream.
//! - The fixpoint is bounded (the candidate count + 2 sweeps); it never *removes*
//!   a fact.

use std::collections::{BTreeMap, BTreeSet};

use crate::listing::model::Insn;
use crate::listing::Listing;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, NoReturnFact, Phase};

/// The structural no-return propagation consumer pass (the second Listing/xref
/// consumer).
///
/// Option-gated (gate id `noreturn_propagate`, default on); short-circuits to an
/// empty output when no Listing is built (`ctx.listing.is_none()`).
#[derive(Default)]
pub struct NoReturnPropagatePass;

impl AnalysisPass for NoReturnPropagatePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "noreturn_propagate"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Listing-dependent: a no-op when the Listing is absent (the `--option
        // listing on` flag is off).
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
        // (kuna, ghidra port) The CFG-reachability rule (Ghidra's
        // `targetOnlyCallsNoReturn`) generalizes the tail-call rule to mid-body
        // no-return calls, dead returns, and switch-of-no-return. Its own flag
        // (`--option noreturn_reach`, default-on, DIV-19); Listing-gated like the rest,
        // so every parity gate stays byte-identical.
        let reach = ctx.arch.analysis_noreturn_reach;
        for fact in propagate_noreturn(listing, &error_recog, reach) {
            out.noreturn.push(fact);
        }
        // (kuna, Ghidra-gap) Emit every `call error(nonzero,…)` site so the decompile
        // seam prunes its fall-through (CALL_RETURN override). Gated by the same
        // `noreturn_error` flag as the recognizer; the recognizer is `disabled()` (no
        // addresses) when the flag is off, so this yields an empty set then.
        out.no_fallthru_calls = collect_error_noreturn_callsites(listing, &error_recog);
        out
    }
}

/// (kuna, Ghidra-gap) Every `call error(nonzero,…)` / `error_at_line(nonzero,…)`
/// call-site address in the program. glibc `error()` with a nonzero constant status
/// calls `exit(status)` and never returns, so the CALL's fall-through must be pruned
/// exactly as Ghidra does ("Subroutine does not return"). Unlike [`function_is_no_return`]
/// (which only inspects a *tail* call to conclude a wrapper no-return, entry-keyed),
/// this collects EVERY such call site — the fall-through prune is what stops the
/// flow-follower from walking past the call into the next function (the docolon→eval6
/// boundary-overrun class). Reuses the recognizer's per-call-site value check.
fn collect_error_noreturn_callsites(
    listing: &Listing,
    error_recog: &ErrorRecognizer,
) -> Vec<u64> {
    let mut sites = Vec::new();
    if error_recog.error_addrs.is_empty() {
        return sites;
    }
    for (&entry, _) in listing.functions() {
        let next = listing.next_function_after(entry).map(|f| f.entry);
        let body = ListingInstructionBody::new(listing, entry, next);
        for (&vma, insn) in listing.instructions_in_range(entry, next) {
            // `call_is_terminal_error` = is_call && target is error && arg0 nonzero const
            // (its name is "terminal" because such a call never returns; it is valid at
            // any body position, not only the last).
            if error_recog.call_is_terminal_error(&body, vma, insn, false) {
                sites.push(vma);
            }
        }
    }
    sites.sort_unstable();
    sites.dedup();
    sites
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

trait InstructionBody {
    fn instruction_at(&self, vma: u64) -> Option<&Insn>;
    fn instruction_before(&self, vma: u64) -> Option<(u64, &Insn)>;
}

struct ListingInstructionBody<'a> {
    listing: &'a Listing,
    entry: u64,
    next: Option<u64>,
}

impl<'a> ListingInstructionBody<'a> {
    fn new(listing: &'a Listing, entry: u64, next: Option<u64>) -> Self {
        Self {
            listing,
            entry,
            next,
        }
    }

    fn contains(&self, vma: u64) -> bool {
        vma >= self.entry && self.next.map_or(true, |next| vma < next)
    }
}

impl<'a> InstructionBody for ListingInstructionBody<'a> {
    fn instruction_at(&self, vma: u64) -> Option<&Insn> {
        if self.contains(vma) {
            self.listing.instruction_at(vma)
        } else {
            None
        }
    }

    fn instruction_before(&self, vma: u64) -> Option<(u64, &Insn)> {
        if !self.contains(vma) {
            return None;
        }
        let (address, insn) = self.listing.instruction_entry_before(vma)?;
        self.contains(address).then_some((address, insn))
    }
}

#[cfg(test)]
impl<'a> InstructionBody for BTreeMap<u64, &'a Insn> {
    fn instruction_at(&self, vma: u64) -> Option<&Insn> {
        self.get(&vma).copied()
    }

    fn instruction_before(&self, vma: u64) -> Option<(u64, &Insn)> {
        self.range(..vma)
            .next_back()
            .map(|(&address, &insn)| (address, insn))
    }
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
        for p in crate::loader::format::resolve_imports(file, bytes) {
            if let Ok(n) =
                String::from_utf8(crate::loader::elf_plt::strip_version(&p.name))
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

    /// Is the instruction at `call_vma` a `call error`/`call error_at_line`
    /// whose `status` argument is a **nonzero constant** — i.e. the call (and so the
    /// whole no-returning-path body) never returns?
    ///
    /// Returns true only when (a) the call target is an `error` address and (b) the
    /// last write to the first int-arg register (x86-64 SysV `EDI`/`RDI`, the `int
    /// status`) before the call is a `MOV reg, imm` with a nonzero literal. A zero
    /// status (`XOR EDI,EDI` / `MOV EDI,0x0` — `error` *returns*) or any non-constant /
    /// unprovable definition is rejected (conservative: a false positive would drop
    /// live caller code). `require_contiguous` preserves the reachability walk's
    /// stricter predecessor rule; the strict tail and call-site scans intentionally
    /// retain their address-order semantics.
    fn call_is_terminal_error<B: InstructionBody + ?Sized>(
        &self,
        body: &B,
        call_vma: u64,
        call: &Insn,
        require_contiguous: bool,
    ) -> bool {
        if !call.flow.is_call {
            return false;
        }
        if !call.flows.iter().any(|t| self.is_error(*t)) {
            return false;
        }
        arg0_is_nonzero_const_before(body, call_vma, require_contiguous)
    }
}

/// Uppercase register token of the first integer argument on x86-64 SysV — the
/// `int status` slot of `error()`/`error_at_line()`. The 64-/32-/16-/8-bit views
/// of `rdi`; on any non-x86 target none of these appear as a destination operand,
/// so the whole recognizer degrades to a clean no-op (no false positives).
fn is_arg0_reg(reg: &str) -> bool {
    ["RDI", "EDI", "DI", "DIL"]
        .iter()
        .any(|candidate| reg.eq_ignore_ascii_case(candidate))
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

/// Backward-scan the straight-line predecessors of the call at `call_vma` for the
/// defining write of the first int-arg register, returning true iff that write is
/// a nonzero **constant** load.
///
/// Conservative by construction: it stops (and returns false) at the first
/// control-flow instruction or intervening `call` (beyond which the straight-line
/// definition we depend on no longer dominates the call and `edi`/`rdi` may be
/// clobbered), and returns false for a zero literal (`error(0,…)` *returns*) or any
/// non-literal / unrecognised definition. A false positive would mark a returning
/// function no-return and drop live caller code, so every uncertain case is a no.
fn arg0_is_nonzero_const_before<B: InstructionBody + ?Sized>(
    body: &B,
    call_vma: u64,
    require_contiguous: bool,
) -> bool {
    let mut vma = call_vma;
    while let Some((previous_vma, insn)) = body.instruction_before(vma) {
        if require_contiguous && insn.fall_through != Some(vma) {
            return false;
        }
        // A branch/return between here and the call breaks straight-line dominance.
        if insn.flow.is_jump || insn.flow.is_terminal {
            return false;
        }
        if let Some(result) = arg0_scan_result(insn) {
            return result;
        }
        vma = previous_vma;
    }
    false
}

fn arg0_scan_result(insn: &Insn) -> Option<bool> {
    let ops = insn.operands.trim();
    let mut parts = ops.splitn(2, ',');
    let dest = parts.next().unwrap_or("").trim();
    let src = parts.next().unwrap_or("").trim();
    if is_arg0_reg(dest) {
        // The nearest definition of the status register: decide here.
        // `MOV EDI, <imm>`: nonzero literal ⇒ error() never returns.
        return Some(
            insn.mnemonic.as_str().trim().eq_ignore_ascii_case("MOV")
                && parse_imm(src).is_some_and(|v| v != 0),
        );
    }
    // A call before the definition clobbers the caller-saved arg register.
    insn.flow.is_call.then_some(false)
}

/// Is `insn` an alignment NOP — the trailing padding a compiler emits after a
/// no-return call (`NOP`, `NOP word ptr CS:[...]`, …; the SLEIGH x86 mnemonic for
/// every padding form begins `NOP`)? Such padding is unreachable after a
/// no-return call, so it is skipped when locating the *last real* instruction.
fn is_padding_nop(insn: &Insn) -> bool {
    insn.mnemonic
        .as_str()
        .trim_start()
        .get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("NOP"))
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
    let body = ListingInstructionBody::new(listing, entry, next);

    // Rule 1: the last *real* (non-padding) instruction must be a CALL — or a tail
    // JMP — to a callee already in the terminal set. Trailing alignment NOPs after
    // a no-return call are unreachable and skipped.
    let Some((last_vma, last)) = listing
        .instructions_in_range(entry, next)
        .rev()
        .find(|(_, insn)| !is_padding_nop(insn))
        .map(|(&vma, insn)| (vma, insn))
    else {
        return false;
    };
    let last_is_terminal_transfer = (last.flow.is_call || last.flow.is_jump)
        && last.flows.iter().any(|t| terminal.contains(t));
    // (kuna, decbench F2) …or a tail `call error(nonzero,…)`: glibc error()/
    // error_at_line() with a nonzero constant status calls exit() and never
    // returns, so the wrapper is no-return even though `error` itself is NOT on the
    // Known no-return list (it returns for error(0,…)). Value-conditional, checked
    // per call site by the recognizer (arg0 is a nonzero literal).
    let last_is_terminal_error = error_recog.call_is_terminal_error(&body, last_vma, last, false);
    if !last_is_terminal_transfer && !last_is_terminal_error {
        return false;
    }

    // Rules 2–4 over the reachable body (up to and including the last real
    // instruction; the padding tail past `last_vma` is unreachable, so it is not
    // scanned).
    for (&vma, insn) in listing.instructions_in_range(entry, next) {
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

/// **CFG-reachability no-return** — the port of Ghidra's
/// `FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn` (the "Non-Returning
/// Functions - Discovered" analyzer's reachability rule), the generalization the
/// strict tail-call rule ([`function_is_no_return`], rules 1–4) misses.
///
/// The strict rule requires the **last real instruction** to be a terminal
/// call/jump, no `RETURN` anywhere, and no computed jump. That is a subset: it
/// cannot conclude a wrapper whose no-return call is **mid-body** (with a dead
/// tail after it), whose `RETURN` is present-but-**unreachable**, or that routes
/// through a **switch** (indirect jump) whose every arm is no-return. Ghidra
/// concludes all three by walking the CFG: a call to an already-no-return callee
/// ends that path (its fall-through is dead), and the function is no-return iff
/// **no `RETURN` is reachable from entry** and at least one reachable path ends at
/// a no-return transfer. Iterated to a call-graph fixpoint by
/// [`propagate_noreturn`], this converts e.g. openssh `sshpkt_vfatal` (a switch
/// whose arms all `call` the log-die) → then `sshpkt_fatal` (whose *first* stmt is
/// `call sshpkt_vfatal`, the write/return tail dead).
///
/// Instruction-level reachability over the Listing body (kuna has no separate
/// block CFG in this tier, but `Insn` carries `fall_through` + resolved `flows` +
/// the flow flags — enough). **Conservative by construction:** every uncertainty
/// (a reachable `RETURN`/terminal, an unresolved indirect jump, a branch escaping
/// the body to a possibly-returning neighbour, a call with no modelled
/// fall-through that is not itself terminal) returns `false` — a false positive
/// would drop live caller code, so every doubt is a "returns".
fn function_reaches_only_noreturn(
    listing: &Listing,
    entry: u64,
    terminal: &BTreeSet<u64>,
    error_recog: &ErrorRecognizer,
) -> bool {
    let next = listing.next_function_after(entry).map(|f| f.entry);
    let body = ListingInstructionBody::new(listing, entry, next);
    reaches_only_noreturn_walk(&body, entry, next, terminal, error_recog)
}

/// The reachability walk core (extracted for unit testing): from `entry`, walk the
/// instruction-level reachable graph of `body` (bounded `[entry, next)`), treating
/// a call/jump to a callee in `terminal` (or a value-conditional `error(nonzero,…)`
/// tail) as a dead transfer, and return true iff **no `RETURN` is reachable** and at
/// least one reachable path ends at such a transfer. Conservative: every uncertainty
/// returns false. See [`function_reaches_only_noreturn`].
fn reaches_only_noreturn_walk<B: InstructionBody + ?Sized>(
    body: &B,
    entry: u64,
    next: Option<u64>,
    terminal: &BTreeSet<u64>,
    error_recog: &ErrorRecognizer,
) -> bool {
    let in_body = |vma: u64| vma >= entry && next.map_or(true, |n| vma < n);
    if body.instruction_at(entry).is_none() {
        return false;
    }

    let mut visited: BTreeSet<u64> = BTreeSet::new();
    let mut todo: Vec<u64> = vec![entry];
    let mut hit_noreturn = false;

    // Push a control successor, resolving whether it stays in the body, lands on an
    // already-no-return callee (a dead transfer), or escapes to a neighbour.
    // Returns false iff the successor escapes to a possibly-returning neighbour
    // (which disqualifies the whole function).
    enum Succ {
        Follow(u64),
        NoReturn,
        Escape,
    }
    let classify = |vma: u64| -> Succ {
        if in_body(vma) {
            Succ::Follow(vma)
        } else if terminal.contains(&vma) {
            Succ::NoReturn
        } else {
            Succ::Escape
        }
    };

    while let Some(vma) = todo.pop() {
        if !visited.insert(vma) {
            continue;
        }
        let Some(insn) = body.instruction_at(vma) else {
            // Fell out of the body without a modelled transfer (should be caught by
            // the successor classifier, but stay safe).
            return false;
        };

        // A reachable RETURN (terminal that is not a call) ⇒ the function returns.
        if insn.flow.is_terminal && !insn.flow.is_call {
            return false;
        }

        // A call/jump whose static target is already no-return (or a value-conditional
        // `error(nonzero,…)` tail) ends this path: its fall-through is dead code.
        let transfers_to_noreturn = (insn.flow.is_call || insn.flow.is_jump)
            && insn.flows.iter().any(|t| terminal.contains(t));
        // error(nonzero,…): use the same contiguous predecessor chain that the old
        // temporary straight-line window supplied to the argument back-scan.
        let transfers_to_error =
            insn.flow.is_call && error_recog.call_is_terminal_error(body, vma, insn, true);
        // A CONDITIONAL jump (`ja`/`jne`/… — a jump that also FALLS THROUGH) to a
        // no-return target ends only its TAKEN arm; the fall-through (the normal,
        // non-taken path) must still be walked. This is the GCC -O2 hot/cold-split
        // shape: an assertion / stack-canary check `jcc <.cold fragment>` where the
        // `.cold` fragment is `call abort`/`__stack_chk_fail` (no-return), while the
        // function returns on the fall-through. Short-circuiting here (as an
        // unconditional transfer does) skips that returning path and wrongly
        // concludes the whole function no-return — which then propagates up every
        // caller (the coreutils `quotearg_*` family → `fmt`/main's dropped file-open
        // `error()` path). IDA Pro / Ghidra both treat such a function as returning.
        // So only short-circuit an UNCONDITIONAL transfer (a call whose no-return
        // callee makes its fall-through dead, or a jump with no fall-through); a
        // conditional jump falls into the `is_jump` handler below, which walks BOTH
        // its taken (→ terminal) and fall-through arms.
        let conditional_jump = insn.flow.is_jump && insn.flow.has_fallthrough;
        if (transfers_to_noreturn || transfers_to_error) && !conditional_jump {
            hit_noreturn = true;
            continue;
        }

        if insn.flow.is_call {
            // A returning call: control continues at the fall-through.
            match insn.fall_through {
                Some(ft) => match classify(ft) {
                    Succ::Follow(v) => todo.push(v),
                    Succ::NoReturn => hit_noreturn = true,
                    Succ::Escape => return false,
                },
                // A call with no modelled fall-through that is not to a terminal
                // callee: we cannot reason about where control goes → returns.
                None => return false,
            }
            continue;
        }

        if insn.flow.is_jump {
            if insn.flow.is_computed || insn.flow.is_indirect {
                // Resolved switch: follow every case target. Unresolved ⇒ cannot prove.
                if insn.flows.is_empty() {
                    return false;
                }
                for &t in &insn.flows {
                    match classify(t) {
                        Succ::Follow(v) => todo.push(v),
                        Succ::NoReturn => hit_noreturn = true,
                        Succ::Escape => return false,
                    }
                }
            } else {
                for &t in &insn.flows {
                    match classify(t) {
                        Succ::Follow(v) => todo.push(v),
                        Succ::NoReturn => hit_noreturn = true,
                        Succ::Escape => return false,
                    }
                }
                // A conditional jump also falls through.
                if insn.flow.has_fallthrough {
                    if let Some(ft) = insn.fall_through {
                        match classify(ft) {
                            Succ::Follow(v) => todo.push(v),
                            Succ::NoReturn => hit_noreturn = true,
                            Succ::Escape => return false,
                        }
                    }
                }
            }
            continue;
        }

        // A straight-line instruction: follow its fall-through.
        match insn.fall_through {
            Some(ft) => match classify(ft) {
                Succ::Follow(v) => todo.push(v),
                Succ::NoReturn => hit_noreturn = true,
                Succ::Escape => return false,
            },
            // No fall-through and not terminal: end of the reachable stream with no
            // return and no transfer — cannot conclude.
            None => return false,
        }
    }

    hit_noreturn
}

/// Run the call-graph no-return propagation over the Listing and return the
/// concluded functions as [`NoReturnFact`]s.
///
/// Seed the terminal set from the Known-list no-return functions
/// (`has_no_return`), then sweep the candidate functions to a fixpoint: each
/// newly-concluded function joins the terminal set so a caller whose last act is a
/// call to *it* is re-evaluated next sweep (a wrapper-of-a-wrapper converges).
/// Keyed by entry VMA (a `BTreeMap` ⇒ address-ordered, deduped).
fn propagate_noreturn(
    listing: &Listing,
    error_recog: &ErrorRecognizer,
    reach: bool,
) -> Vec<NoReturnFact> {
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
            // The strict tail-call rule first (cheap, proven); then, if enabled, the
            // CFG-reachability generalization (Ghidra's `targetOnlyCallsNoReturn`).
            let concluded = function_is_no_return(listing, entry, &terminal, error_recog)
                || (reach && function_reaches_only_noreturn(listing, entry, &terminal, error_recog));
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

    // The fixpoint convergence + the propagation rule are exercised end-to-end by
    // the cross-crate `verify_noreturn_propagate.rs` gate (a real x86-64 fixture
    // with a custom `my_die() { ...; abort(); }` wrapper called only ONCE — below
    // `noreturn_disc`'s ≥3 threshold and followed by NOP padding, so ONLY this
    // pass concludes it no-return). The test-only Listing constructor also lets the
    // borrowed body view exercise real address-index boundaries without a decoder.

    #[test]
    fn pass_identity_wiring() {
        let p = NoReturnPropagatePass;
        assert_eq!(p.id(), "noreturn_propagate");
        assert_eq!(p.phase(), Phase::P1);
    }

    // --- reach walk: a conditional jump to a no-return target still returns via its
    //     fall-through (the GCC hot/cold-split regression) --------------------------

    fn mk_insn(addr: u64, ft: Option<u64>, flow: crate::listing::model::FlowType, flows: Vec<u64>) -> crate::listing::model::Insn {
        crate::listing::model::Insn {
            addr,
            len: 2,
            fall_through: ft,
            flow,
            flows,
            mnemonic: "".into(),
            operands: String::new(),
            pcode: None,
        }
    }

    /// A function guarded by an assertion / stack-canary branch to a no-return
    /// `.cold` fragment — `jcc <terminal>; …; RET` — RETURNS on the fall-through.
    /// The reach walk must follow that fall-through and conclude it returns. Before
    /// the fix the `transfers_to_noreturn` fast-path `continue`d past the conditional
    /// jump, skipping the returning arm, and wrongly marked the whole coreutils
    /// `quotearg_*` family no-return (dropping `fmt`/main's file-open `error()` path).
    #[test]
    fn reach_walk_follows_conditional_jump_fallthrough_to_ret() {
        use std::collections::{BTreeMap, BTreeSet};
        let straight = crate::listing::model::FlowType { has_fallthrough: true, ..Default::default() };
        let cjump = crate::listing::model::FlowType {
            is_jump: true,
            is_conditional: true,
            has_fallthrough: true,
            ..Default::default()
        };
        let ret = crate::listing::model::FlowType { is_terminal: true, ..Default::default() };
        // entry → cjump[→ 0x100 terminal] (fall-through) → RET
        let insns = vec![
            mk_insn(0x0, Some(0x2), straight, vec![]),
            mk_insn(0x2, Some(0x4), cjump, vec![0x100]),
            mk_insn(0x4, Some(0x6), straight, vec![]),
            mk_insn(0x6, None, ret, vec![]),
        ];
        let body: BTreeMap<u64, &crate::listing::model::Insn> = insns.iter().map(|i| (i.addr, i)).collect();
        let terminal: BTreeSet<u64> = [0x100].into_iter().collect();
        let recog = super::ErrorRecognizer::disabled();
        assert!(
            !super::reaches_only_noreturn_walk(&body, 0x0, Some(0x100), &terminal, &recog),
            "a function that returns on the conditional-jump fall-through must NOT be no-return"
        );

        // Control: an UNCONDITIONAL jump to the terminal (no fall-through) IS no-return.
        let ujump = crate::listing::model::FlowType { is_jump: true, ..Default::default() };
        let only = vec![mk_insn(0x0, None, ujump, vec![0x100])];
        let body2: BTreeMap<u64, &crate::listing::model::Insn> = only.iter().map(|i| (i.addr, i)).collect();
        assert!(
            super::reaches_only_noreturn_walk(&body2, 0x0, Some(0x100), &terminal, &recog),
            "an unconditional jump whose only target is no-return IS no-return"
        );
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

    fn arg0_is_nonzero_const(body: &[(u64, &Insn)], call_idx: usize) -> bool {
        let index: BTreeMap<u64, &Insn> = body.iter().copied().collect();
        arg0_is_nonzero_const_before(&index, body[call_idx].0, false)
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
    fn listing_instruction_body_bounds_exact_and_predecessor_queries() {
        let mut previous_function = insn("MOV", "EDI,0x2", false, false);
        previous_function.addr = 0x80;
        let mut call = insn("CALL", "0x500", true, false);
        call.addr = 0x100;
        call.flows = vec![0x500];
        let next_function = mk_insn(
            0x200,
            None,
            crate::listing::model::FlowType {
                is_terminal: true,
                ..Default::default()
            },
            vec![],
        );
        let listing = Listing::from_insns_for_test(
            vec![previous_function, call.clone(), next_function],
            true,
        );
        let body = ListingInstructionBody::new(&listing, 0x100, Some(0x200));

        assert!(body.instruction_at(0x80).is_none());
        assert!(body.instruction_at(0x100).is_some());
        assert!(body.instruction_at(0x200).is_none());
        assert!(body.instruction_before(0x100).is_none());
        assert!(body.instruction_before(0x200).is_none());

        let recog = ErrorRecognizer {
            error_addrs: [0x500].into_iter().collect(),
        };
        assert!(
            !recog.call_is_terminal_error(&body, 0x100, &call, false),
            "a definition in the preceding function must not classify this call"
        );
    }

    #[test]
    fn error_argument_scans_preserve_strict_and_reach_gap_rules() {
        let mut definition = insn("MOV", "EDI,0x2", false, false);
        definition.addr = 0x100;
        definition.fall_through = Some(0x105);
        let mut call = insn("CALL", "0x500", true, false);
        call.addr = 0x110;
        call.flows = vec![0x500];
        let listing = Listing::from_insns_for_test(vec![definition, call.clone()], true);
        let body = ListingInstructionBody::new(&listing, 0x100, None);
        let recog = ErrorRecognizer {
            error_addrs: [0x500].into_iter().collect(),
        };

        assert!(
            recog.call_is_terminal_error(&body, 0x110, &call, false),
            "the strict scan keeps its legacy address-order behavior across a gap"
        );
        assert!(
            !recog.call_is_terminal_error(&body, 0x110, &call, true),
            "the reach scan requires an exact fall-through predecessor chain"
        );

        let mut definition = insn("MOV", "EDI,0x2", false, false);
        definition.addr = 0x100;
        definition.fall_through = Some(0x105);
        let mut clobber = insn("CALL", "0x600", true, false);
        clobber.addr = 0x105;
        clobber.fall_through = Some(0x110);
        let mut call = insn("CALL", "0x500", true, false);
        call.addr = 0x110;
        call.flows = vec![0x500];
        let listing = Listing::from_insns_for_test(vec![definition, clobber, call.clone()], true);
        let body = ListingInstructionBody::new(&listing, 0x100, None);
        assert!(
            !recog.call_is_terminal_error(&body, 0x110, &call, false),
            "an intervening call still clobbers the argument register"
        );
    }

    // --- CFG-reachability rule (Ghidra targetOnlyCallsNoReturn port, DIV-19) --------

    /// Build an `Insn` with explicit flow shape for the reachability walk tests.
    #[allow(clippy::too_many_arguments)]
    fn rinsn(
        addr: u64,
        fall_through: Option<u64>,
        flows: Vec<u64>,
        is_call: bool,
        is_jump: bool,
        is_terminal: bool,
        is_computed: bool,
        is_indirect: bool,
        has_fallthrough: bool,
    ) -> Insn {
        let mut flow = crate::listing::model::FlowType::default();
        flow.is_call = is_call;
        flow.is_jump = is_jump;
        flow.is_terminal = is_terminal;
        flow.is_computed = is_computed;
        flow.is_indirect = is_indirect;
        flow.has_fallthrough = has_fallthrough;
        Insn {
            addr,
            len: 1,
            fall_through,
            flow,
            flows,
            mnemonic: "".into(),
            operands: String::new(),
            pcode: None,
        }
    }

    /// Run [`reaches_only_noreturn_walk`] over a hand-built body. `entry`=0x100,
    /// `next`=0x200; `ABORT`=0x50 is the seeded terminal (no-return) callee.
    fn walk(insns: &[Insn]) -> bool {
        const NEXT: u64 = 0x200;
        let body: BTreeMap<u64, &Insn> = insns.iter().map(|i| (i.addr, i)).collect();
        let terminal: BTreeSet<u64> = [0x50u64].into_iter().collect();
        reaches_only_noreturn_walk(&body, 0x100, Some(NEXT), &terminal, &ErrorRecognizer::disabled())
    }
    const ABORT: u64 = 0x50;

    #[test]
    fn reach_tail_noreturn_call_is_noreturn() {
        // 0x100: call ABORT  (terminal callee, no return anywhere)
        let b = [rinsn(0x100, Some(0x105), vec![ABORT], true, false, false, false, false, true)];
        assert!(walk(&b), "a body whose only path ends in call abort is no-return");
    }

    #[test]
    fn reach_midbody_noreturn_with_dead_tail_is_noreturn() {
        // The reachability-ONLY case the strict rule misses: the no-return call is NOT
        // the last instruction; a `ret` sits after it but is unreachable (the call's
        // fall-through is never followed).
        // 0x100: call ABORT (ft->0x105)   0x105: ret
        let b = [
            rinsn(0x100, Some(0x105), vec![ABORT], true, false, false, false, false, true),
            rinsn(0x105, None, vec![], false, false, true, false, false, false),
        ];
        assert!(walk(&b), "mid-body call abort with an unreachable ret tail is no-return");
    }

    #[test]
    fn reach_switch_of_noreturn_is_noreturn() {
        // A switch (computed jump) whose every arm calls a no-return function.
        // 0x100: jmp [table]  (computed, flows=[0x110,0x120])
        // 0x110: call ABORT   0x120: call ABORT
        let b = [
            rinsn(0x100, None, vec![0x110, 0x120], false, true, false, true, true, false),
            rinsn(0x110, Some(0x115), vec![ABORT], true, false, false, false, false, true),
            rinsn(0x120, Some(0x125), vec![ABORT], true, false, false, false, false, true),
        ];
        assert!(walk(&b), "a switch whose every arm ends in call abort is no-return");
    }

    #[test]
    fn reach_returning_function_is_not_noreturn() {
        // 0x100: call FOO (returning, ft->0x105)  0x105: ret  → a RETURN is reachable.
        let foo = 0x60; // NOT in the terminal set
        let b = [
            rinsn(0x100, Some(0x105), vec![foo], true, false, false, false, false, true),
            rinsn(0x105, None, vec![], false, false, true, false, false, false),
        ];
        assert!(!walk(&b), "a reachable ret after a returning call ⇒ returns");
    }

    #[test]
    fn reach_live_return_on_one_branch_is_not_noreturn() {
        // 0x100: jz 0x120 (cond, flows=[0x120], ft->0x110)
        // 0x110: call ABORT (dies)   0x120: ret (REACHABLE via the branch)
        let b = [
            rinsn(0x100, Some(0x110), vec![0x120], false, true, false, false, false, true),
            rinsn(0x110, Some(0x115), vec![ABORT], true, false, false, false, false, true),
            rinsn(0x120, None, vec![], false, false, true, false, false, false),
        ];
        assert!(!walk(&b), "a reachable ret on the taken branch ⇒ returns (conservative)");
    }

    #[test]
    fn reach_unresolved_indirect_jump_is_rejected() {
        // 0x100: jmp rax (indirect, NO resolved flows) → cannot prove.
        let b = [rinsn(0x100, None, vec![], false, true, false, true, true, false)];
        assert!(!walk(&b), "an unresolved indirect jump ⇒ cannot conclude no-return");
    }

    #[test]
    fn reach_escape_to_returning_neighbour_is_rejected() {
        // 0x100: jmp 0x9999 (direct, target OUTSIDE body, not terminal) → escape.
        let b = [rinsn(0x100, None, vec![0x9999], false, true, false, false, false, false)];
        assert!(!walk(&b), "a tail jump escaping the body to a neighbour ⇒ cannot conclude");
    }

    #[test]
    fn reach_call_with_no_fallthrough_to_returning_callee_is_rejected() {
        // 0x100: call FOO with no modelled fall-through, FOO not terminal → uncertain.
        let foo = 0x60;
        let b = [rinsn(0x100, None, vec![foo], true, false, false, false, false, false)];
        assert!(!walk(&b), "a call with no fall-through to a non-terminal callee ⇒ reject");
    }

    #[test]
    fn empty_listing_consumer_is_inert_via_pass() {
        // With no Listing, the pass short-circuits to an empty output (the
        // default-off / no-listing path; the parity-safe no-op).
        let p = NoReturnPropagatePass;
        assert_eq!(p.id(), "noreturn_propagate");
    }
}
