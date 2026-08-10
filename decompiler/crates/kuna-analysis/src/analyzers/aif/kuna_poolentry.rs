//! (kuna) `poolentry` — reference-driven ARM literal-pool inference, and the two
//! entry facts it supports (P1 code/data partition, ARM discovery).
//!
//! # The defect
//!
//! [`super::run_aif`] slides its gap cursor ONE BYTE at a time (`mod.rs`
//! `advanced = gap_start.saturating_add(1)`) with no instruction-alignment filter,
//! because [`Listing::first_undefined_after`] is byte-granular by construction. A
//! PC-relative literal pool is data, so it is an undefined gap; the cursor probes
//! every byte of it and accepts the first one whose 2-mnemonic fingerprint clears
//! the histogram threshold. On STM32 Thumb the pool words are SRAM addresses
//! `0x2000_xxxx` whose HIGH halfword decodes as `movs r0,#0`, so the accepted entry
//! lands at `pool_word + 2` — one halfword before the real function — and on accept
//! the cursor jumps past the body, so the real entry is never probed.
//!
//! Upstream Ghidra does not have this: its reference analyzer defines pc-relative
//! literal targets as DATA before AIF runs, so those bytes are not an undefined gap
//! at all. kuna's Listing has no literal-pool data-definition step; the module LOSS
//! note in `mod.rs` ("context-mode ARM gap probing is NOT handled") is the same gap
//! seen from the other side.
//!
//! # What this module computes
//!
//! [`run_pool_pass`] reconstructs the missing data definition from the references
//! that are actually there: every instruction whose operand body carries a bare
//! absolute `[0x…]` target (the form SLEIGH's ARM disassembly prints for
//! `ldr rN,[pc,#imm]` / `ldr.w` / `vldr` / `ldrd`) contributes one referenced,
//! word-aligned literal word inside the executable image. Maximal stride-4 runs of
//! those words are the inferred pools. This is strictly more conservative than an
//! ELF `$d` mapping-symbol oracle: a pool word nothing loads is not part of a run.
//!
//! The scan reads the decoded Listing **and** the speculatively-decoded bodies of
//! the gap-discovered routines (AIF's accepts, `ptrentry`'s targets). Those bodies
//! are not in the Listing, and a pool sandwiched between two gap-discovered
//! functions — the exact shape that produces the phantom — is referenced only from
//! inside one.
//!
//! Two consumers, with different risk profiles and separate gates:
//!
//! * **RECALL, additive** — emit an entry fact at `pool_end`, the first address
//!   after a pool run, when it is undefined and passes AIF's own fingerprint +
//!   valid-subroutine acceptance. Purely additive, so "never removes an entry" is a
//!   property of the wiring, exactly as in `ptrentry` / `tailcallentry`.
//! * **PRECISION, subtractive** — drop an AIF-accepted entry that lies inside an
//!   inferred pool. This one REMOVES entries, so no wiring can make it safe; its
//!   safety has to come from the predicate.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;

use crate::listing::Listing;

use super::GapDecoder;

/// A maximal run of word-aligned literal words, `[lo, hi)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pool {
    pub lo: u64,
    pub hi: u64,
}

/// What the pass produced: the inferred pools, the additive `pool_end` entries, and
/// the AIF accept list with the pool interiors dropped.
pub struct PoolResult {
    pub pools: Vec<Pool>,
    pub added: Vec<u64>,
    pub kept_aif: Vec<u64>,
    pub suppressed: Vec<u64>,
}

fn env_usize(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_on(name: &str) -> bool {
    std::env::var(name).is_ok()
}

/// The absolute literal target of one instruction, if its operand body carries a
/// bare `[0xHEX]` (no register, no shift) — the form SLEIGH's ARM disassembly
/// prints for a resolved PC-relative literal load.
fn absolute_literal_target(operands: &str) -> Option<u64> {
    let open = operands.find("[0x")?;
    let rest = &operands[open + 3..];
    let close = rest.find(']')?;
    let hex = &rest[..close];
    if hex.is_empty() || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(hex, 16).ok()
}

/// Where a known instruction ENDS, and whether it was a return-class terminal.
/// Built over the Listing plus the speculatively-decoded gap routines; used by the
/// "this pool follows a body" gates.
type CodeEnds = std::collections::BTreeMap<u64, bool>;

/// Every referenced literal word: 4-aligned, inside an executable range, and not
/// the start of a decoded instruction. `speculative` names the gap-discovered
/// routine entries whose bodies the Listing never decoded.
fn referenced_words(
    listing: &Listing,
    decoder: &mut GapDecoder,
    speculative: &[u64],
    ends: &mut CodeEnds,
    branch_targets: &mut BTreeSet<u64>,
) -> BTreeSet<u64> {
    let exec = listing.exec_ranges().to_vec();
    let in_exec = |vma: u64| exec.iter().any(|&(lo, hi)| vma >= lo && vma < hi);
    let mut out: BTreeSet<u64> = BTreeSet::new();

    for (&vma, insn) in listing.instructions() {
        let e = ends.entry(vma + insn.len as u64).or_insert(false);
        *e |= insn.flow.is_terminal;
        if !insn.flow.is_call {
            branch_targets.extend(insn.flows.iter().copied());
        }
        if let Some(t) = absolute_literal_target(&insn.operands) {
            if t % 4 == 0 && in_exec(t) && !listing.is_instruction_start(t) {
                out.insert(t);
            }
        }
    }
    for &entry in speculative {
        let gap_hi = listing.next_instruction_start_after(entry).unwrap_or(u64::MAX);
        let body = super::check_valid_subroutine(decoder, listing, entry, entry, gap_hi)
            .or_else(|| {
                super::kuna_ptrentry::check_terminating_routine(decoder, listing, entry, gap_hi)
            });
        let Some(body) = body else { continue };
        for vma in body {
            let Some(insn) = decoder.probe(vma) else { continue };
            let e = ends.entry(vma + insn.len as u64).or_insert(false);
            *e |= insn.is_terminal;
            if !insn.is_call {
                branch_targets.extend(insn.flows.iter().copied());
            }
            if let Some(t) = absolute_literal_target(&insn.operands) {
                if t % 4 == 0 && in_exec(t) && !listing.is_instruction_start(t) {
                    out.insert(t);
                }
            }
        }
    }
    out
}

/// Does a body end at `lo` (allowing up to 2 bytes of Thumb alignment padding)?
/// `need_return` additionally demands that the instruction it ends with is a
/// return-class terminal — an inter-function pool follows a `bx lr` / `pop {..pc}`,
/// while an INTRA-function pool follows the unconditional `b` the compiler emits to
/// jump over it, which is the split shape.
fn follows_body(ends: &CodeEnds, lo: u64, need_return: bool) -> bool {
    for back in [0u64, 2] {
        if lo < back {
            continue;
        }
        if let Some(&term) = ends.get(&(lo - back)) {
            if !need_return || term {
                return true;
            }
        }
    }
    false
}

/// Maximal stride-4 runs over the referenced words.
///
/// `KUNA_POOL_MERGE_GAP` (default 0) allows that many UNREFERENCED words inside a
/// run — the `vldr`/`ldrd` shape, where a 64-bit literal's second word is loaded by
/// nothing. A gap word is bridged only when it holds no decoded instruction.
fn runs(listing: &Listing, words: &BTreeSet<u64>) -> Vec<Pool> {
    let merge_gap = env_usize("KUNA_POOL_MERGE_GAP", 0);
    let mut pools: Vec<Pool> = Vec::new();
    for &w in words {
        let extend = match pools.last() {
            Some(p) if w >= p.hi && w - p.hi <= merge_gap * 4 => {
                (p.hi..w).step_by(4).all(|g| !listing.is_instruction_start(g))
            }
            _ => false,
        };
        if extend {
            pools.last_mut().unwrap().hi = w + 4;
        } else {
            pools.push(Pool { lo: w, hi: w + 4 });
        }
    }
    pools
}

/// Run both halves over a completed Listing + the gap-discovered entry sets.
///
/// `aif_entries` is AIF's accept list (the only list the precision half filters);
/// `other_speculative` names further gap-discovered routines (`ptrentry`) whose
/// bodies carry literal references but which are not filtered.
#[allow(clippy::too_many_arguments)]
pub fn run_pool_pass(
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    aif_entries: &[u64],
    other_speculative: &[u64],
    want_add: bool,
    want_suppress: bool,
) -> PoolResult {
    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut spec: Vec<u64> = aif_entries.to_vec();
    spec.extend_from_slice(other_speculative);
    let mut ends: CodeEnds = CodeEnds::new();
    let mut branch_targets: BTreeSet<u64> = BTreeSet::new();
    let words = referenced_words(listing, &mut decoder, &spec, &mut ends, &mut branch_targets);
    let mut pools = runs(listing, &words);
    let after_code = env_on("KUNA_POOL_AFTER_CODE");
    let after_ret = env_on("KUNA_POOL_AFTER_RET");
    // The two halves take DIFFERENT pool sets. Suppression wants every inferred
    // pool (an AIF accept inside any pool is a phantom); the additive half wants
    // only pools that abut the end of a body, since a pool the scan found in the
    // middle of nowhere has no reason to be followed by a function.
    let add_pools: Vec<Pool> = if after_code || after_ret {
        pools.iter().copied().filter(|p| follows_body(&ends, p.lo, after_ret)).collect()
    } else {
        pools.clone()
    };
    if env_on("KUNA_POOL_GATE_BOTH") {
        pools = add_pools.clone();
    }

    let mut added = Vec::new();
    if want_add {
        let hist = super::build_fingerprint_histogram(listing);
        if hist.values().any(|&c| c >= super::FINGERPRINT_THRESHOLD) {
            let relaxed = env_on("KUNA_POOL_TERM"); // drop MIN_SUBROUTINE_INSNS
            let no_branch_target = env_on("KUNA_POOL_NO_BRANCHTGT");
            let mut accepted: BTreeSet<u64> = BTreeSet::new();
            for p in &add_pools {
                let entry = p.hi;
                if !listing.is_undefined(entry) {
                    continue;
                }
                // A pool_end that some known instruction BRANCHES to is the
                // continuation of the function whose early return the pool follows
                // (the intra-function pool shape), not a new function.
                if no_branch_target && branch_targets.contains(&entry) {
                    continue;
                }
                let Some(fp) = decoder.fingerprint(entry) else { continue };
                if hist.get(&fp).copied().unwrap_or(0) < super::FINGERPRINT_THRESHOLD {
                    continue;
                }
                let gap_hi = listing.next_instruction_start_after(entry).unwrap_or(u64::MAX);
                let ok = if relaxed {
                    super::kuna_ptrentry::check_terminating_routine(
                        &mut decoder,
                        listing,
                        entry,
                        gap_hi,
                    )
                    .is_some()
                } else {
                    super::check_valid_subroutine(&mut decoder, listing, entry, entry, gap_hi)
                        .is_some()
                };
                if ok {
                    accepted.insert(entry);
                }
            }
            added = accepted.into_iter().collect();
        }
    }

    // PAIRED mode: only drop a phantom when the pool it sits in has a REPLACEMENT
    // entry at its end — either one this pass just added, or one some other stage
    // already found. Then the body the phantom was decompiling (at a 2-byte-early
    // address) is still emitted, at the right address.
    let paired = env_on("KUNA_POOL_PAIRED");
    let replaced: BTreeSet<u64> = pools
        .iter()
        .filter(|p| {
            added.binary_search(&p.hi).is_ok()
                || listing.function_at(p.hi).is_some()
                || listing.is_instruction_start(p.hi)
        })
        .map(|p| p.hi)
        .collect();
    let mut kept_aif = aif_entries.to_vec();
    let mut suppressed = Vec::new();
    if want_suppress {
        let (keep, drop): (Vec<u64>, Vec<u64>) = aif_entries.iter().copied().partition(|&e| {
            !pools
                .iter()
                .any(|p| e >= p.lo && e < p.hi && (!paired || replaced.contains(&p.hi)))
        });
        kept_aif = keep;
        suppressed = drop;
    }

    PoolResult { pools, added, kept_aif, suppressed }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_bare_absolute_bracket_is_a_literal_reference() {
        assert_eq!(absolute_literal_target("r0,[0x80001c8]"), Some(0x80001c8));
        assert_eq!(absolute_literal_target("r3,[r7,#0x4]"), None);
        assert_eq!(absolute_literal_target("r3,[r3,#0x0]"), None);
        assert_eq!(absolute_literal_target("r0,r1,[0x8000200]"), Some(0x8000200));
        assert_eq!(absolute_literal_target("r0,[0x]"), None);
    }
}
