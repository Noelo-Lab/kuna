//! (kuna) `aifbracket` — refuse an AIF gap candidate that is a fragment of the
//! known function around it (P1 code/data partition).
//!
//! # The defect (GH-299)
//!
//! [`super::run_aif`] probes the bytes the recursive-descent walk left undecoded.
//! Some of those holes are not gaps between functions at all: they sit inside a
//! function the walk already knows, behind a dispatch it could not follow (an
//! unresolved jump table, a computed `jmp`), an undecodable instruction, or a call
//! it believed does not return. A block in such a hole that opens with a common
//! two-instruction prologue and branches back into the function's decoded code
//! passes both acceptance tests — a fingerprint shared by four discovered
//! functions, and a valid-subroutine walk, which counts the branch into decoded
//! code as "adds information" — and becomes a `sub_<addr>` in the middle of a body.
//! On the GH-299 witness, a 3.4 MB stripped i386 PE, 737 of the 2,067 mid-body AIF
//! entries sit inside a function kuna has the entry of.
//!
//! # The rule
//!
//! A kuna function is an entry with no extent, so "the hole lies inside a known
//! function" cannot be read off the function list: on a sparsely discovered image
//! nearly every hole lies between one known entry and the next. The candidate's own
//! speculative body supplies the missing proof instead. [`rejects`] refuses an
//! accept when that body falls through or jumps (not calls) into an instruction the
//! walk decoded that is not a function entry and lies in the same entry interval as
//! the candidate: the candidate's code rejoins the function that encloses it.
//!
//! A real function can do that too (a shared tail, a tail jump into code the walk
//! absorbed), so a joined candidate is still accepted on any boundary evidence that
//! a function starts there ([`boundary_evidence`]):
//!
//!  1. an inbound reference — the walk filed one, or an aligned pointer-sized word
//!     in an allocated section holds the candidate's address ([`BracketEvidence`]);
//!  2. it lies within the first word of a hole that a terminal opens: the decoded
//!     instruction before the hole is a return, a direct unconditional jump, or a
//!     call the walk did not fall through (a no-return call), or nothing decoded
//!     precedes it. The word allowance admits the function behind a one-word
//!     literal pool, which A32 code puts right after a return;
//!  3. the instruction that ends exactly at the candidate is alignment padding
//!     (`nop`, `int3`, a self-`mov` or self-`lea`), a return or an unconditional
//!     jump.
//!
//! A refused candidate still consumes its body, as an `aifcorroborate` refusal
//! does: otherwise the cursor resumes inside the fragment it just refused.
//!
//! # Measured
//!
//! Each stripped image scored against its unstripped twin's symbol table, AIF on
//! (`--mode aggressive` / `--mode reliable`), `aifbracket` off -> on:
//!
//! ```text
//! 98 ARM ELFs (49,530 functions)     aggressive  mid-body 7,132 -> 5,551   true starts lost 0, gained 0
//!                                    reliable    mid-body 12,637 -> 10,947  lost 0, gained 0
//! 43 i386 PEs (85,484 functions)     aggressive  mid-body 9,156 -> 7,865   lost 0, gained 0
//!                                    reliable    mid-body 13,027 -> 10,153  lost 0, gained 0
//! 696 x86-64 ELFs (146,571)          aggressive  unchanged
//! ```
//!
//! Most of the mid-body entries that remain never touch decoded code (a hole that
//! is the tail of a function, ending in its own return), which this rule cannot see.

use std::collections::{BTreeSet, HashSet};

use crate::listing::{FlowKind, Listing};

use super::GapDecoder;

/// The longest x86 instruction; the backward search for the instruction ending at
/// a candidate tries every length up to it.
const MAX_INSN_LEN: u64 = 15;

/// How far into a terminal-opened hole a candidate may start and still count as
/// starting at its boundary: one 32-bit literal-pool word.
const HOLE_SLOT: u64 = 4;

/// The aligned pointer-sized words in the image's allocated sections whose value
/// (with or without the Thumb bit) is an executable address.
#[derive(Default)]
pub struct BracketEvidence {
    pointers: HashSet<u64>,
}

impl BracketEvidence {
    pub fn from_object(file: &object::File, exec: &[(u64, u64)]) -> Self {
        use object::read::{Object, ObjectSection};
        use object::SectionKind;
        let width = if file.is_64() { 8 } else { 4 };
        let little = file.is_little_endian();
        let in_exec = |v: u64| exec.iter().any(|&(lo, hi)| lo <= v && v < hi);
        let mut pointers = HashSet::new();
        for sec in file.sections() {
            let allocated = match sec.flags() {
                object::SectionFlags::Elf { sh_flags } => {
                    sh_flags & u64::from(object::elf::SHF_ALLOC) != 0
                }
                object::SectionFlags::Coff { characteristics } => {
                    characteristics & object::pe::IMAGE_SCN_MEM_DISCARDABLE == 0
                }
                _ => true,
            };
            let skipped = matches!(
                sec.kind(),
                SectionKind::Debug
                    | SectionKind::DebugString
                    | SectionKind::Metadata
                    | SectionKind::Note
                    | SectionKind::Linker
                    | SectionKind::UninitializedData
                    | SectionKind::UninitializedTls
                    | SectionKind::Common
            );
            if !allocated || skipped || sec.address() == 0 {
                continue;
            }
            let Ok(data) = sec.data() else { continue };
            let skew = ((width - sec.address() % width) % width) as usize;
            for word in data.get(skew..).unwrap_or(&[]).chunks_exact(width as usize) {
                let value = match (width, little) {
                    (8, true) => u64::from_le_bytes(word.try_into().unwrap()),
                    (8, false) => u64::from_be_bytes(word.try_into().unwrap()),
                    (_, true) => u64::from(u32::from_le_bytes(word.try_into().unwrap())),
                    (_, false) => u64::from(u32::from_be_bytes(word.try_into().unwrap())),
                };
                for v in [value, value & !1] {
                    if in_exec(v) {
                        pointers.insert(v);
                    }
                }
            }
        }
        BracketEvidence { pointers }
    }
}

/// True iff the accepted gap candidate at `entry`, whose speculative body is
/// `body`, is a fragment of its enclosing known function with no boundary evidence.
pub(super) fn rejects(
    listing: &Listing,
    decoder: &mut GapDecoder,
    evidence: &BracketEvidence,
    entry: u64,
    body: &BTreeSet<u64>,
) -> bool {
    joins_enclosing_interior(listing, decoder, entry, body)
        && !boundary_evidence(listing, decoder, evidence, entry)
}

/// Whether `body` falls through or jumps to a decoded non-entry instruction in the
/// same entry interval as `entry`.
fn joins_enclosing_interior(
    listing: &Listing,
    decoder: &GapDecoder,
    entry: u64,
    body: &BTreeSet<u64>,
) -> bool {
    let Some(owner) = listing.function_containing(entry).map(|f| f.entry) else {
        return false;
    };
    body.iter().any(|&at| {
        let insn = decoder.recorded(at);
        let jumps = if insn.is_call { &[][..] } else { &insn.flows[..] };
        insn.fall_through.iter().chain(jumps).any(|&target| {
            !body.contains(&target)
                && listing.is_instruction_start(target)
                && listing.function_at(target).is_none()
                && listing.function_containing(target).map(|f| f.entry) == Some(owner)
        })
    })
}

fn boundary_evidence(
    listing: &Listing,
    decoder: &mut GapDecoder,
    evidence: &BracketEvidence,
    entry: u64,
) -> bool {
    if listing.has_refs_to(entry) || evidence.pointers.contains(&entry) {
        return true;
    }
    if let Some(lo) = hole_start_within(listing, entry, HOLE_SLOT) {
        if opens_hole(listing, lo) {
            return true;
        }
        if lo == entry {
            return false;
        }
    }
    follows_boundary_instruction(decoder, entry)
}

/// The first byte of the undefined hole containing `entry`, when it lies at most
/// `slot` bytes before `entry`.
fn hole_start_within(listing: &Listing, entry: u64, slot: u64) -> Option<u64> {
    let mut lo = entry;
    while lo > entry.saturating_sub(slot) && listing.is_undefined(lo - 1) {
        lo -= 1;
    }
    (lo == 0 || !listing.is_undefined(lo - 1)).then_some(lo)
}

/// Whether the walk stopped at `lo` because flow ended there: the decoded
/// instruction before it has no fall-through and is not a computed jump, or no
/// decoded instruction ends right before it.
fn opens_hole(listing: &Listing, lo: u64) -> bool {
    let Some(prev) = lo.checked_sub(1) else { return true };
    let Some(insn) = listing.instruction_before(lo) else { return true };
    if insn.addr.saturating_add(u64::from(insn.len)) <= prev {
        return true;
    }
    insn.fall_through.is_none() && insn.flow.kind != FlowKind::ComputedJump
}

/// Whether some speculative instruction ending exactly at `entry` is padding, a
/// return or an unconditional jump.
fn follows_boundary_instruction(decoder: &mut GapDecoder, entry: u64) -> bool {
    (1..=MAX_INSN_LEN).any(|len| {
        let Some(at) = entry.checked_sub(len) else { return false };
        decoder.probe(at).is_some_and(|insn| {
            u64::from(insn.len) == len
                && (matches!(insn.kind, FlowKind::Return | FlowKind::UnconditionalBranch)
                    || is_padding(&insn.mnemonic, &insn.operands))
        })
    })
}

/// Whether a decoded instruction is alignment padding: `nop`, `int3`, or a move or
/// address computation that leaves its register unchanged.
fn is_padding(mnemonic: &str, operands: &str) -> bool {
    let mnemonic = mnemonic.to_ascii_lowercase();
    if mnemonic == "nop" || mnemonic == "int3" {
        return true;
    }
    let operands: String = operands
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_ascii_lowercase();
    let Some((dst, src)) = operands.split_once(',') else { return false };
    match mnemonic.as_str() {
        "mov" | "cpy" | "xchg" => dst == src,
        "lea" => {
            let inner = src.strip_prefix('[').and_then(|s| s.strip_suffix(']')).unwrap_or("");
            inner.strip_suffix("+0x0").unwrap_or(inner) == dst
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_forms() {
        assert!(is_padding("NOP", ""));
        assert!(is_padding("NOP", "dword ptr [EAX + EAX*0x1]"));
        assert!(is_padding("INT3", ""));
        assert!(is_padding("MOV", "ESI,ESI"));
        assert!(is_padding("MOV", "EDI,EDI"));
        assert!(is_padding("LEA", "ESI,[ESI]"));
        assert!(is_padding("LEA", "ESI,[ESI + 0x0]"));
        assert!(is_padding("mov", "r8,r8"));
        assert!(is_padding("nop", ""));
        assert!(!is_padding("MOV", "EBP,ESP"));
        assert!(!is_padding("LEA", "ECX,[ESP + 0x4]"));
        assert!(!is_padding("LEA", "ESI,[ESI + 0x8]"));
        assert!(!is_padding("PUSH", "EBP"));
        assert!(!is_padding("movs", "r0,r0"));
    }
}
