//! (kuna `poolextents`) ARM32 literal-pool extents from trusted references.
//!
//! On ARM a function's PC-relative constants sit in `.text` right after the
//! code that loads them, and a function whose last act is a call kuna cannot
//! prove no-return is followed by its own pool. The decompiler's flow walk
//! takes that call's fall-through and decodes the pool as instructions; the
//! `funcboundflow` bound (`kuna_decomp::kuna_funcboundflow`) needs to know where
//! the pools are before it walks, from evidence the walk itself cannot taint.
//!
//! The Listing walk takes the same fall-throughs, so neither "this word was
//! decoded" nor "some instruction reads this word" is evidence on its own: a
//! word after a non-returning `svc` decodes as whatever it is, and if that is
//! a PC-relative load it names a live instruction as its literal. The witness
//! has to be an instruction that is itself code. An instruction is **trusted**
//! when it is reachable from a function entry over static branch and call
//! targets and over the fall-through of instructions that are not calls, or
//! are conditional calls, or are direct calls to a function some path of which
//! provably reaches a `RETURN`. A computed jump, an indirect call or a user
//! operation never proves a return (a PLT stub ends in `ldr pc,[..]` and must
//! stay unproven), so everything reached only by falling through one of those
//! is provisional and neither witnesses a pool nor labels a word as code.
//!
//! A pool word is then a 4-aligned executable word some trusted whole-word
//! load reads at a fixed address (the resolved `[0x…]` and `[pc,#imm]` forms
//! `aif::kuna_poolentry` parses, both words of a 64-bit literal), covered by no
//! trusted instruction and the static target of none. That judgement is made
//! twice, under two notions of trust, and a word that passes either is a pool
//! word: the one above, and a strict one whose trust crosses no call
//! fall-through at all. The strict witness is what sees a pool after a call to
//! a function that does return — gcc places one there when
//! `__builtin_unreachable()` follows the call — since the word is then reached
//! under the first notion and so excluded by it. Each judgement alone never
//! names live code (compiled code reads no instruction word as a whole-word
//! literal from code that is certainly code), so neither does their union.
//! Maximal stride-4 runs of the union are the extents.
//!
//! Shapes this answers, from the reproducers the earlier walk-side evidence
//! failed: a data word after a non-returning `svc` that decodes as a load of a
//! returning call's fall-through (the word is provisional, its load is
//! ignored, nothing is bounded); a pool constant that decodes as a PC-relative
//! load of live code after a returning call (the load is at best reached past
//! a call, so it is no strict witness, and the live code is covered under the
//! first notion); a pool after a returning call whose only load sits past
//! another call (no strict witness, and the word is reached: not a pool word).
//! In the reported shape the pool's loads sit before the first import call and
//! are trusted under both notions, while the junk at the fall-through of the
//! final call is provisional and so does not shield the words it covers.

use std::collections::{BTreeSet, HashMap};

use super::model::{FlowKind, Insn};
use super::{CodeUnit, Listing};

/// The condition suffixes the ARM SLEIGH appends to a mnemonic.
const CONDS: [&str; 17] = [
    "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al",
];

/// A call, indirect call or user operation: flow past it depends on whether
/// what it invokes comes back.
fn suspect(insn: &Insn) -> bool {
    insn.flow.is_call || insn.flow.kind == FlowKind::CallOther
}

/// The direct callee of a call instruction.
fn direct_callee(insn: &Insn) -> Option<u64> {
    (insn.flow.is_call && !insn.flow.is_computed).then(|| insn.flows.first().copied()).flatten()
}

/// Does flow provably continue from `insn` into its fall-through?
fn falls_through(insn: &Insn, returns: &BTreeSet<u64>) -> bool {
    if insn.fall_through.is_none() {
        return false;
    }
    if !suspect(insn) {
        return true;
    }
    if insn.flow.is_conditional && insn.flow.is_call {
        return true;
    }
    direct_callee(insn).is_some_and(|callee| returns.contains(&callee))
}

/// Entries of functions some path provably returns from.
///
/// An instruction-level least fixpoint, propagated backwards from every
/// `RETURN`: an instruction reaches a return when a static branch target of
/// its does (a call's target does not count: the callee's return is not the
/// caller's), or when its fall-through does and flow provably continues into
/// that fall-through ([`falls_through`]). An entry that reaches a return makes
/// every direct call to it fall through, which can mark the call sites in
/// turn. A function the known no-return list names never returns.
pub(crate) fn returning_entries(listing: &Listing) -> BTreeSet<u64> {
    let insns: Vec<&Insn> = listing.instructions().map(|(_, i)| i).collect();
    let index: HashMap<u64, usize> = insns.iter().enumerate().map(|(i, insn)| (insn.addr, i)).collect();
    let mut branch_preds: Vec<Vec<usize>> = vec![Vec::new(); insns.len()];
    let mut fall_preds: Vec<Vec<usize>> = vec![Vec::new(); insns.len()];
    let mut call_sites: HashMap<u64, Vec<usize>> = HashMap::new();
    for (p, insn) in insns.iter().enumerate() {
        if let Some(callee) = direct_callee(insn) {
            call_sites.entry(callee).or_default().push(p);
        } else {
            for t in &insn.flows {
                if let Some(&s) = index.get(t) {
                    branch_preds[s].push(p);
                }
            }
        }
        if let Some(&s) = insn.fall_through.as_ref().and_then(|f| index.get(f)) {
            fall_preds[s].push(p);
        }
    }

    let mut reaches = vec![false; insns.len()];
    let mut returns: BTreeSet<u64> = BTreeSet::new();
    let mut work: Vec<usize> = Vec::new();
    for (i, insn) in insns.iter().enumerate() {
        if insn.flow.kind == FlowKind::Return {
            reaches[i] = true;
            work.push(i);
        }
    }
    while let Some(s) = work.pop() {
        if let Some(f) = listing.function_at(insns[s].addr) {
            if !f.has_no_return && returns.insert(f.entry) {
                for &c in call_sites.get(&f.entry).map(Vec::as_slice).unwrap_or(&[]) {
                    let back = insns[c].fall_through.as_ref().and_then(|f| index.get(f));
                    if !reaches[c] && back.is_some_and(|&b| reaches[b]) {
                        reaches[c] = true;
                        work.push(c);
                    }
                }
            }
        }
        for &p in &branch_preds[s] {
            if !reaches[p] {
                reaches[p] = true;
                work.push(p);
            }
        }
        for &p in &fall_preds[s] {
            if !reaches[p] && falls_through(insns[p], &returns) {
                reaches[p] = true;
                work.push(p);
            }
        }
    }
    returns
}

/// Instructions reachable from a function entry without falling through an
/// unproven call or user operation. With an empty `returns` no call's
/// fall-through is crossed at all: the strict notion.
pub(crate) fn trusted_instructions(listing: &Listing, returns: &BTreeSet<u64>) -> BTreeSet<u64> {
    let mut trusted: BTreeSet<u64> = BTreeSet::new();
    let mut work: Vec<u64> = listing.functions().map(|(&entry, _)| entry).collect();
    while let Some(at) = work.pop() {
        let Some(insn) = listing.instruction_at(at) else { continue };
        if !trusted.insert(at) {
            continue;
        }
        work.extend(insn.flows.iter().copied());
        if falls_through(insn, returns) {
            work.extend(insn.fall_through);
        }
    }
    trusted
}

/// Is `mnemonic` a load of a whole 32- or 64-bit word: `ldr`, `ldrd` or `vldr`
/// with any condition suffix or width? A byte or halfword read proves nothing
/// about the word it sits in.
fn is_word_load(mnemonic: &str) -> bool {
    let m = mnemonic.to_ascii_lowercase();
    let m = m.split('.').next().unwrap_or("");
    if m.starts_with("vldr") {
        return true;
    }
    let Some(rest) = m.strip_prefix("ldr") else { return false };
    let rest = rest.strip_prefix('d').unwrap_or(rest);
    rest.is_empty() || CONDS.contains(&rest)
}

/// Maximal stride-4 runs, half-open `[lo, hi)`.
fn runs(words: &BTreeSet<u64>) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    for &w in words {
        match out.last_mut() {
            Some(run) if w == run.1 => run.1 = w + 4,
            _ => out.push((w, w + 4)),
        }
    }
    out
}

/// The literal-pool extents the trusted instructions of `listing` prove.
///
/// `literal` resolves one instruction's address and operand text to the word
/// it loads at a fixed address, or `None`; `wide` says whether a mnemonic loads
/// two words.
pub fn pool_extents(
    listing: &Listing,
    literal: &dyn Fn(u64, &str) -> Option<u64>,
    wide: &dyn Fn(&str) -> bool,
) -> Vec<(u64, u64)> {
    let returns = returning_entries(listing);
    let mut words = pool_words(listing, &trusted_instructions(listing, &returns), literal, wide);
    words.append(&mut pool_words(listing, &trusted_instructions(listing, &BTreeSet::new()), literal, wide));
    runs(&words)
}

/// The words the `trusted` instructions prove to be literal-pool data.
fn pool_words(
    listing: &Listing,
    trusted: &BTreeSet<u64>,
    literal: &dyn Fn(u64, &str) -> Option<u64>,
    wide: &dyn Fn(&str) -> bool,
) -> BTreeSet<u64> {
    let labels: BTreeSet<u64> = trusted
        .iter()
        .filter_map(|at| listing.instruction_at(*at))
        .flat_map(|insn| insn.flows.iter().copied())
        .collect();
    let exec = listing.exec_ranges();
    let in_exec = |w: u64| exec.iter().any(|&(lo, hi)| w >= lo && w < hi);
    let covered = |w: u64| match listing.code_unit_at(w) {
        CodeUnit::Instruction(start) => trusted.contains(&start),
        _ => false,
    };
    let mut words: BTreeSet<u64> = BTreeSet::new();
    for &at in trusted {
        let Some(insn) = listing.instruction_at(at) else { continue };
        if !is_word_load(&insn.mnemonic) {
            continue;
        }
        let Some(t) = literal(at, &insn.operands) else { continue };
        let n = if wide(&insn.mnemonic) { 2 } else { 1 };
        for k in 0..n {
            let w = t.wrapping_add(k * 4);
            if w % 4 == 0 && in_exec(w) && !labels.contains(&w) && !covered(w) {
                words.insert(w);
            }
        }
    }
    words
}

#[cfg(test)]
#[path = "kuna_poolextents/tests.rs"]
mod tests;
