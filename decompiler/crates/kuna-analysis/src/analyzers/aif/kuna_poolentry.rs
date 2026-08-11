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
//! the cursor jumps past the body, so the real entry is never probed. In A32 the
//! whole word decodes and the phantom lands on the word itself.
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
//! absolute `[0x…]` target (the form the ARM SLEIGH prints for a resolved
//! `ldr rN,[pc,#imm]`) or a `[pc,#imm]` literal (the form `vldr`/`ldrd` print,
//! resolving the target in the semantic body instead — `ARMneon.sinc` `vldrRn`)
//! contributes one referenced, word-aligned literal word inside the executable
//! image, plus the second word of a 64-bit literal. Maximal stride-4 runs of those
//! words are the inferred pools. This is strictly more conservative than an ELF
//! `$d` mapping-symbol oracle: a pool word nothing loads is not part of a run.
//!
//! Completing the `[pc,#imm]` half matters more than it looks: without it every
//! pool holding a float or a 64-bit constant under-runs and the additive half
//! plants its entry ON a pool word. Measured over the 98-image ARM corpus that
//! difference is 19 split bodies / 89.7% ground truth versus 1 / 98.4%.
//!
//! The scan reads the decoded Listing **and** the speculatively-decoded bodies of
//! the gap-discovered routines (AIF's accepts, `ptrentry`'s targets). Those bodies
//! are not in the Listing, and a pool sandwiched between two gap-discovered
//! functions — the exact shape that produces the phantom — is referenced only from
//! inside one.
//!
//! Two consumers, with different warrants:
//!
//! * **RECALL, additive** — emit an entry fact at `pool_end`, the first address
//!   after a pool that abuts a return-class terminal, when it is undefined and
//!   passes AIF's own fingerprint + valid-subroutine acceptance. Purely additive,
//!   so "never removes an entry" is a property of the wiring, exactly as in
//!   `ptrentry` / `tailcallentry`.
//! * **PRECISION, subtractive** — drop an AIF-accepted entry that lies inside an
//!   inferred pool, but ONLY when that pool's end carries a replacement entry. The
//!   pairing is what makes the removal a MOVE rather than a delete: unpaired it
//!   destroys 531 real bodies on the corpus, paired it destroys zero.

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
/// the AIF accept list with the paired pool interiors dropped.
pub struct PoolResult {
    pub pools: Vec<Pool>,
    pub added: Vec<u64>,
    pub kept_aif: Vec<u64>,
    pub suppressed: Vec<u64>,
}

/// The absolute literal target of one instruction, if its operand body carries a
/// bare `[0xHEX]` (no register, no shift) — the form the ARM SLEIGH prints for a
/// resolved PC-relative literal load.
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

/// The `[pc,#imm]` literal form — what the ARM SLEIGH prints for `vldr`/`ldrd`
/// (`ARMneon.sinc` `vldrRn`), which resolve the target in the semantic body rather
/// than in the display. `pc_delta` yields 4 in Thumb, 8 in A32, and `None` where
/// there is no decode mode at all; the base is the word-aligned PC. The delta is a
/// closure because resolving it is a context-database lookup and all but a handful
/// of instructions never reach it.
fn pcrel_literal_target(
    addr: u64,
    operands: &str,
    pc_delta: impl FnOnce() -> Option<u64>,
) -> Option<u64> {
    let open = operands.find("[pc")?;
    let pc_delta = pc_delta()?;
    let rest = &operands[open + 3..];
    let close = rest.find(']')?;
    let inner = &rest[..close];
    let base = (addr + pc_delta) & !3u64;
    if inner.is_empty() {
        return Some(base);
    }
    // Only the immediate form is a literal reference. `tbb [pc,r3]` / `tbh
    // [pc,r3,lsl #1]` also print `[pc`, and must not be read as one.
    let mut num = inner.strip_prefix(",#")?;
    let neg = num.starts_with('-');
    if neg {
        num = &num[1..];
    }
    if !num.bytes().all(|b| b.is_ascii_alphanumeric()) {
        return None;
    }
    let imm = if let Some(h) = num.strip_prefix("0x") {
        u64::from_str_radix(h, 16).ok()?
    } else {
        num.parse::<u64>().ok()?
    };
    if neg { base.checked_sub(imm) } else { base.checked_add(imm) }
}

/// One instruction's literal target under both display forms.
fn literal_target(addr: u64, operands: &str, mode: &DecodeMode) -> Option<u64> {
    absolute_literal_target(operands)
        .or_else(|| pcrel_literal_target(addr, operands, || mode.pc_delta(addr)))
}

/// The per-address ARM decode mode, read straight out of the engine's
/// `ContextDatabase` — the SAME `TMode` the bytes at that address were decoded
/// under, whichever pass painted it (`arm_markers`' `$t`/`$a` mapping symbols and
/// STT_FUNC LSB, the Cortex-M vector-table region paint at either signature width,
/// the libc-start `main` pointer paint). Reading the database rather than
/// re-deriving the paints is what keeps this correct when the paint that matters
/// came from an option: on betaflight the shipped vector-table signature does not
/// match (its initial SP is in CCM RAM, not SRAM) and only `cortexmvectors`'
/// widened scan paints the image Thumb.
///
/// A language with no `TMode` register — x86-64, i386, everything that is not ARM
/// — makes the lookup fail, and [`Self::pc_delta`] then answers `None`, which
/// disables the `[pc,#imm]` reference form outright. That is a structural brake on
/// top of the ISA fact that x86-64 RIP-relative loads target `.rodata` rather than
/// `.text` interstices and that i386 has no PC-relative addressing at all.
struct DecodeMode<'a> {
    arch: &'a kuna_decomp::architecture::Architecture,
    code_space: Rc<AddrSpace>,
}

impl DecodeMode<'_> {
    /// PC bias of a `[pc,#imm]` literal at `addr`: Thumb +4, A32 +8, `None` where
    /// the language has no decode-mode context.
    fn pc_delta(&self, addr: u64) -> Option<u64> {
        let at = kuna_base::address::Address::new(Rc::clone(&self.code_space), addr);
        let tmode = self
            .arch
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &at))
            .ok()?;
        Some(if tmode == 1 { 4 } else { 8 })
    }
}

/// Where a known instruction ENDS, and whether it was a return-class terminal.
/// Built over the Listing plus the speculatively-decoded gap routines; used by the
/// "this pool follows a body" gate.
type CodeEnds = std::collections::BTreeMap<u64, bool>;

/// Every referenced literal word: 4-aligned, inside an executable range, and not
/// the start of a decoded instruction. `speculative` names the gap-discovered
/// routine entries whose bodies the Listing never decoded.
fn referenced_words(
    listing: &Listing,
    decoder: &mut GapDecoder,
    speculative: &[u64],
    ends: &mut CodeEnds,
    mode: &DecodeMode,
) -> BTreeSet<u64> {
    let exec = listing.exec_ranges().to_vec();
    let in_exec = |vma: u64| exec.iter().any(|&(lo, hi)| vma >= lo && vma < hi);
    let mut out: BTreeSet<u64> = BTreeSet::new();

    for (&vma, insn) in listing.instructions() {
        let e = ends.entry(vma + insn.len as u64).or_insert(false);
        *e |= insn.flow.is_terminal;
        if let Some(t) = literal_target(vma, &insn.operands, mode) {
            insert_literal(&mut out, listing, &in_exec, t, wide(&insn.mnemonic));
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
            if let Some(t) = literal_target(vma, &insn.operands, mode) {
                insert_literal(&mut out, listing, &in_exec, t, wide(&insn.mnemonic));
            }
        }
    }
    out
}

/// A 64-bit literal load covers TWO words; the second is loaded by nothing.
fn wide(mnemonic: &str) -> bool {
    let m = mnemonic.to_ascii_lowercase();
    m.starts_with("vldr.64") || m.starts_with("ldrd")
}

fn insert_literal(
    out: &mut BTreeSet<u64>,
    listing: &Listing,
    in_exec: &dyn Fn(u64) -> bool,
    t: u64,
    wide: bool,
) {
    let n = if wide { 2 } else { 1 };
    for k in 0..n {
        let w = t + k * 4;
        if w % 4 == 0 && in_exec(w) && !listing.is_instruction_start(w) {
            out.insert(w);
        }
    }
}

/// Does a body end at `lo` (allowing up to 2 bytes of Thumb alignment padding), and
/// does it end with a return-class terminal? An inter-function pool follows a
/// `bx lr` / `pop {..pc}`, while an INTRA-function pool follows the unconditional
/// `b` the compiler emits to jump over it — which is the split shape, so the
/// terminal class is the gate that separates them.
fn follows_return(ends: &CodeEnds, lo: u64) -> bool {
    [0u64, 2].iter().any(|&back| lo >= back && ends.get(&(lo - back)) == Some(&true))
}

/// Maximal stride-4 runs over the referenced words.
///
/// Adjacency is exact: an unreferenced word BREAKS the run. Bridging one or more
/// unreferenced words was measured and is dominated — the run then swallows short
/// real functions, and even in paired mode 62 (1-word) to 119 (combined) real
/// bodies stop being decompiled at all.
fn runs(words: &BTreeSet<u64>) -> Vec<Pool> {
    let mut pools: Vec<Pool> = Vec::new();
    for &w in words {
        match pools.last_mut() {
            Some(p) if w == p.hi => p.hi = w + 4,
            _ => pools.push(Pool { lo: w, hi: w + 4 }),
        }
    }
    pools
}

/// Run both consumers over a completed Listing + the gap-discovered entry sets.
///
/// `aif_entries` is AIF's accept list (the only list the precision half filters);
/// `other_speculative` names further gap-discovered routines (`ptrentry`) whose
/// bodies carry literal references but which are not filtered.
pub fn run_pool_pass(
    arch: &kuna_decomp::architecture::Architecture,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
    aif_entries: &[u64],
    other_speculative: &[u64],
) -> PoolResult {
    let mode = DecodeMode { arch, code_space: Rc::clone(&code_space) };
    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut spec: Vec<u64> = aif_entries.to_vec();
    spec.extend_from_slice(other_speculative);
    let mut ends: CodeEnds = CodeEnds::new();
    let words = referenced_words(listing, &mut decoder, &spec, &mut ends, &mode);
    let pools = runs(&words);
    // The two consumers take DIFFERENT pool sets. Suppression wants every inferred
    // pool (an AIF accept inside any pool is a phantom); the additive half wants
    // only pools that abut the end of a returning body, since a pool the scan found
    // in the middle of nowhere has no reason to be followed by a function.
    let add_pools: Vec<Pool> =
        pools.iter().copied().filter(|p| follows_return(&ends, p.lo)).collect();

    let mut added = Vec::new();
    let hist = super::build_fingerprint_histogram(listing);
    if hist.values().any(|&c| c >= super::FINGERPRINT_THRESHOLD) {
        let mut accepted: BTreeSet<u64> = BTreeSet::new();
        for p in &add_pools {
            let entry = p.hi;
            if !listing.is_undefined(entry) {
                continue;
            }
            let Some(fp) = decoder.fingerprint(entry) else { continue };
            if hist.get(&fp).copied().unwrap_or(0) < super::FINGERPRINT_THRESHOLD {
                continue;
            }
            let gap_hi = listing.next_instruction_start_after(entry).unwrap_or(u64::MAX);
            if super::check_valid_subroutine(&mut decoder, listing, entry, entry, gap_hi).is_some()
            {
                accepted.insert(entry);
            }
        }
        added = accepted.into_iter().collect();
    }

    // PAIRED: only drop a phantom when the pool it sits in has a REPLACEMENT entry
    // at its end — either one this pass just added, or one some other stage already
    // found. The body the phantom was decompiling (at a 2-byte-early address) is
    // then still emitted, at the right address, which makes the removal a MOVE and
    // "no body stops being decompiled" a property of the wiring again.
    let replaced: BTreeSet<u64> = pools
        .iter()
        .filter(|p| {
            added.binary_search(&p.hi).is_ok()
                || listing.function_at(p.hi).is_some()
                || listing.is_instruction_start(p.hi)
        })
        .map(|p| p.hi)
        .collect();
    let (kept_aif, suppressed): (Vec<u64>, Vec<u64>) = aif_entries
        .iter()
        .copied()
        .partition(|&e| !pools.iter().any(|p| e >= p.lo && e < p.hi && replaced.contains(&p.hi)));

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

    /// The `[pc,#imm]` form, and the two register-indexed `[pc` shapes that print
    /// the same prefix and must NOT be read as literal references — `tbb`/`tbh`
    /// jump tables, which would otherwise be inferred as pools.
    #[test]
    fn pcrel_reads_the_immediate_form_only() {
        // Thumb: base is the word-aligned PC (addr + 4).
        assert_eq!(pcrel_literal_target(0x8000102, "d0,[pc,#0x18]", || Some(4)), Some(0x800011c));
        // A32: addr + 8, already word-aligned.
        assert_eq!(pcrel_literal_target(0x60800100, "d0,[pc,#0x20]", || Some(8)), Some(0x60800128));
        assert_eq!(pcrel_literal_target(0x8000100, "r0,r1,[pc,#-0x10]", || Some(4)), Some(0x80000f4));
        assert_eq!(pcrel_literal_target(0x8000100, "[pc,r3]", || Some(4)), None);
        assert_eq!(pcrel_literal_target(0x8000100, "[pc,r3,lsl #1]", || Some(4)), None);
        // Off ARM there is no decode mode, so the form is disabled outright.
        assert_eq!(pcrel_literal_target(0x8000100, "d0,[pc,#0x18]", || None), None);
    }

    /// A run breaks at the first unreferenced word — no bridging (measured: any
    /// bridging swallows short real functions).
    #[test]
    fn runs_are_exactly_adjacent() {
        let words: BTreeSet<u64> = [0x1000, 0x1004, 0x1008, 0x1010, 0x1014].into_iter().collect();
        assert_eq!(
            runs(&words),
            vec![Pool { lo: 0x1000, hi: 0x100c }, Pool { lo: 0x1010, hi: 0x1018 }]
        );
    }

    /// The additive gate reads the return class, and tolerates the two bytes of
    /// Thumb alignment padding between a `bx lr` and a word-aligned pool.
    #[test]
    fn follows_return_wants_a_return_class_terminal() {
        let mut ends = CodeEnds::new();
        ends.insert(0x1000, true); // a `bx lr` ends here
        ends.insert(0x2000, false); // an unconditional `b` over an intra-function pool
        assert!(follows_return(&ends, 0x1000));
        assert!(follows_return(&ends, 0x1002), "2 bytes of Thumb alignment padding");
        assert!(!follows_return(&ends, 0x2000), "a `b` over the pool is the split shape");
        assert!(!follows_return(&ends, 0x3000));
    }
}
