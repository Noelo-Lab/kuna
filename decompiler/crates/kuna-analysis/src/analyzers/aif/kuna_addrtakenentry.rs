//! (kuna) `addrtakenentry` — code whose address the program takes, and that no
//! flow reaches, is a function entry (P1 code/data partition, x86-64).
//!
//! The Listing walk makes a function only at a CALL target. A function whose
//! address is taken — `lea rax,[rip+handler]` stored to a global, `mov
//! edi,offset handler` passed to a registration call — and that is only ever
//! called through that pointer is reached by no flow at all: the walk never
//! decodes it, and the extent bounder folds its bytes into the function before
//! it. On an x64 PE a frame-less leaf also has no `.pdata` record, so no
//! load-time oracle names it either.
//!
//! Two kinds of evidence name such an address:
//!
//! 1. an instruction the walk decoded materializes it: a RIP-relative `lea`, a
//!    `mov` of an immediate into a register or memory, or a `push` of an
//!    immediate;
//! 2. on a PE, a base-relocated word holds it and it lies before the first
//!    `RUNTIME_FUNCTION` of its section — the span the relocation oracle's
//!    `.pdata` hull guard leaves out (see `entry::kuna_pereloccode`).
//!
//! Either way the address may still be a label or data, so a candidate becomes
//! an entry only when all of these hold:
//!
//! - it is undecoded executable memory: no decoded instruction covers it, so no
//!   flow the walk followed reaches it;
//! - it starts where a function starts: the bytes in front of it, back to the
//!   last decoded instruction (or one this pass accepted, or the start of its
//!   section), are padding or zero fill, and that instruction does not fall
//!   through;
//! - it is code that ends like a function: decoded inside its gap, every path
//!   ends in a return, a computed jump, a trap, a call followed by padding, or a
//!   jump to a known function entry; no path falls out of the gap into other
//!   code or jumps into the middle of decoded code; and it does not open with
//!   zero fill or `int3`.
//!
//! The accepted entries re-seed the walk, so their callees and the addresses
//! their own code takes are found too; acceptance repeats over the rebuilt
//! Listing for at most [`MAX_ROUNDS`] rounds.
//!
//! x86-64 only. On i386 every address is an absolute immediate, and the same
//! shape names non-functions: a MinGW SJLJ landing pad sits after its
//! function's `ret` and alignment padding, and the function stores its address
//! with `mov [ebp-0x1c],imm` — measured, 40 i386 PEs gained 210 true starts,
//! lost 574 and grew 4,974 mid-body entries.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;
use object::read::Object;

use crate::listing::{FlowKind, Listing};

use super::kuna_aifnoppad::{is_padding, is_zero_fill};
use super::{GapDecoder, ProbedInsn, MAX_FOLLOW_INSNS};

/// How many re-walks the accepted entries may trigger.
pub(crate) const MAX_ROUNDS: usize = 4;

/// The longest filler run read in front of a candidate.
const MAX_PADDING_INSNS: usize = 64;

/// The evidence that does not change between rounds: the executable bytes the
/// instruction scan reads, and a PE's pre-record relocated code pointers.
pub(crate) struct AddrTaken {
    texts: Vec<(u64, u64, Vec<u8>)>,
    relocated: Vec<u64>,
}

impl AddrTaken {
    /// `None` off x86-64.
    pub(crate) fn new(file: &object::File, bytes: &[u8]) -> Option<AddrTaken> {
        if file.architecture() != object::Architecture::X86_64 {
            return None;
        }
        let texts = crate::entry::executable_sections(file);
        let relocated = crate::entry::kuna_pereloccode::pre_record_code_pointers(bytes, &texts);
        Some(AddrTaken { texts, relocated })
    }

    /// The candidates over `listing`, address-sorted: every undecoded executable
    /// address a decoded instruction materializes, and every pre-record relocated
    /// code pointer.
    fn candidates(&self, listing: &Listing) -> Vec<u64> {
        let mut out: Vec<u64> = self.relocated.clone();
        for (&at, insn) in listing.instructions() {
            let Some(bytes) = self.bytes(at, insn.len) else { continue };
            if let Some(target) = materialized(at, bytes) {
                if target != at.wrapping_add(u64::from(insn.len)) {
                    out.push(target);
                }
            }
        }
        out.retain(|&t| listing.is_undefined(t));
        out.sort_unstable();
        out.dedup();
        out
    }

    fn bytes(&self, at: u64, len: u32) -> Option<&[u8]> {
        let (lo, _, data) = self.texts.iter().find(|&&(lo, hi, _)| at >= lo && at < hi)?;
        let off = usize::try_from(at - lo).ok()?;
        data.get(off..off.checked_add(len as usize)?)
    }

    /// The candidates of `listing` that pass every guard, address-sorted.
    pub(crate) fn accept(
        &self,
        listing: &Listing,
        translate: &dyn Translate,
        code_space: Rc<AddrSpace>,
    ) -> Vec<u64> {
        let candidates = self.candidates(listing);
        if candidates.is_empty() {
            return Vec::new();
        }
        let ranges = listing.exec_ranges();
        let mut decoder = GapDecoder::new(translate, code_space, ranges);
        let mut claimed: BTreeMap<u64, Claimed> = BTreeMap::new();
        let mut accepted: BTreeSet<u64> = BTreeSet::new();
        for target in candidates {
            let Some(&(lo, hi)) = ranges.iter().find(|&&(lo, hi)| target >= lo && target < hi) else {
                continue;
            };
            if splits_claimed(&claimed, target) {
                continue;
            }
            let Some(padded) = isolated(listing, &mut decoder, &claimed, lo, target) else {
                continue;
            };
            let gap_hi = listing.next_instruction_start_after(target).map_or(hi, |n| n.min(hi));
            if let Some(body) = routine(listing, &mut decoder, &accepted, target, gap_hi, padded) {
                accepted.insert(target);
                claimed.extend(body);
            }
        }
        accepted.into_iter().collect()
    }
}

/// One instruction of an accepted routine: where it ends, and whether it falls
/// through.
#[derive(Clone, Copy)]
struct Claimed {
    end: u64,
    falls: bool,
}

/// The address the decoded x86-64 instruction `bytes` at `at` materializes as a
/// value, when it is a RIP-relative `lea`, a `mov` of an immediate, or a `push`
/// of an immediate. Operand-size overrides narrow the value below an address and
/// are refused.
fn materialized(at: u64, bytes: &[u8]) -> Option<u64> {
    let mut i = 0;
    while let Some(&prefix) = bytes.get(i) {
        match prefix {
            0x66 | 0x67 => return None,
            0xF0 | 0xF2 | 0xF3 | 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 => i += 1,
            _ => break,
        }
    }
    let mut rex_w = false;
    if bytes.get(i).is_some_and(|&rex| rex & 0xF0 == 0x40) {
        rex_w = bytes[i] & 0x08 != 0;
        i += 1;
    }
    let len = bytes.len();
    let tail32 = || Some(u32::from_le_bytes(bytes.get(len.checked_sub(4)?..)?.try_into().ok()?));
    match *bytes.get(i)? {
        0x8D => {
            if bytes.get(i + 1)? & 0xC7 != 0x05 || len != i + 6 {
                return None;
            }
            let disp = tail32()? as i32 as i64 as u64;
            let target = at.wrapping_add(len as u64).wrapping_add(disp);
            Some(if rex_w { target } else { target & 0xFFFF_FFFF })
        }
        0xB8..=0xBF if rex_w => {
            let imm = bytes.get(i + 1..)?;
            (imm.len() == 8).then(|| u64::from_le_bytes(imm.try_into().unwrap()))
        }
        0xB8..=0xBF => (len == i + 5).then(|| tail32().map(u64::from)).flatten(),
        0xC7 => {
            if (bytes.get(i + 1)? >> 3) & 7 != 0 {
                return None;
            }
            let imm = tail32()?;
            Some(if rex_w { imm as i32 as i64 as u64 } else { u64::from(imm) })
        }
        0x68 => {
            if len != i + 5 {
                return None;
            }
            let imm = tail32()?;
            Some(imm as i32 as i64 as u64)
        }
        _ => None,
    }
}

/// Whether `target` lies strictly inside an instruction a routine accepted this
/// round.
fn splits_claimed(claimed: &BTreeMap<u64, Claimed>, target: u64) -> bool {
    claimed.range(..target).next_back().is_some_and(|(_, c)| c.end > target)
}

/// Whether `target` starts where a function starts: the instruction before it
/// (decoded, or accepted this round, inside its range) does not fall through
/// and only filler lies between the two; with no instruction before it, only
/// filler lies between the range start and `target`. Returns whether any filler
/// was read.
fn isolated(
    listing: &Listing,
    decoder: &mut GapDecoder,
    claimed: &BTreeMap<u64, Claimed>,
    lo: u64,
    target: u64,
) -> Option<bool> {
    let decoded = listing
        .instruction_before(target)
        .filter(|insn| insn.addr >= lo)
        .map(|insn| (insn.addr, Claimed {
            end: insn.addr.wrapping_add(u64::from(insn.len)),
            falls: insn.fall_through.is_some(),
        }));
    let overlay = claimed.range(lo..target).next_back().map(|(&at, &c)| (at, c));
    let mut at = match decoded.into_iter().chain(overlay).max_by_key(|&(at, _)| at) {
        Some((_, prev)) if prev.end > target || prev.falls => return None,
        Some((_, prev)) => prev.end,
        None => lo,
    };
    let start = at;
    for _ in 0..=MAX_PADDING_INSNS {
        if at == target {
            return Some(at != start);
        }
        let insn = decoder.probe(at)?;
        if !is_padding(&insn, true) && !is_zero_fill(&insn) {
            return None;
        }
        at = at.checked_add(u64::from(insn.len)).filter(|&next| next <= target)?;
    }
    None
}

/// The instructions of the routine at `target`, decoded inside `[target, gap_hi)`,
/// when every path ends like a function's; `None` when one does not.
///
/// `padded` says filler precedes `target`: a candidate that is itself filler is
/// then the middle of that run, not a function opening with a `nop`.
fn routine(
    listing: &Listing,
    decoder: &mut GapDecoder,
    accepted: &BTreeSet<u64>,
    target: u64,
    gap_hi: u64,
    padded: bool,
) -> Option<BTreeMap<u64, Claimed>> {
    let first = decoder.probe(target)?;
    if is_zero_fill(&first) || is_trap(&first) || (padded && is_padding(&first, true)) {
        return None;
    }
    let is_entry = |at: u64| listing.function_at(at).is_some() || accepted.contains(&at);
    let mut body: BTreeMap<u64, Claimed> = BTreeMap::new();
    let mut work: Vec<(u64, bool)> = vec![(target, false)];
    while let Some((at, after_call)) = work.pop() {
        if body.contains_key(&at) {
            continue;
        }
        if body.len() >= MAX_FOLLOW_INSNS {
            return None;
        }
        let insn = decoder.probe(at)?;
        if after_call && (is_padding(&insn, true) || is_trap(&insn)) {
            continue;
        }
        let end = at.checked_add(u64::from(insn.len)).filter(|&end| end <= gap_hi)?;
        body.insert(at, Claimed { end, falls: insn.fall_through.is_some() });
        if insn.is_terminal || insn.kind == FlowKind::ComputedJump || is_trap(&insn) {
            continue;
        }
        for &to in &insn.flows {
            if insn.is_call {
                if !decoder.in_exec(to) {
                    return None;
                }
            } else if to >= target && to < gap_hi {
                work.push((to, false));
            } else if !is_entry(to) {
                return None;
            }
        }
        if let Some(fall) = insn.fall_through {
            if fall >= gap_hi {
                return None;
            }
            work.push((fall, insn.is_call));
        }
    }
    let mut last_end = 0;
    for (&at, c) in &body {
        if at < last_end {
            return None;
        }
        last_end = c.end;
    }
    Some(body)
}

/// `int3`, `ud2` and `hlt`: flow does not continue past them.
fn is_trap(insn: &ProbedInsn) -> bool {
    ["int3", "ud2", "hlt"].iter().any(|m| insn.mnemonic.eq_ignore_ascii_case(m))
}

#[cfg(test)]
mod tests {
    use super::materialized;

    #[test]
    fn rip_relative_lea_names_its_target() {
        // lea rax,[rip+0x9] at 0x140001030 (the GH-1039 registration stub)
        let lea = [0x48, 0x8d, 0x05, 0x09, 0x00, 0x00, 0x00];
        assert_eq!(materialized(0x1_4000_1030, &lea), Some(0x1_4000_1040));
        // lea rcx,[rip-0x10]
        let back = [0x48, 0x8d, 0x0d, 0xf0, 0xff, 0xff, 0xff];
        assert_eq!(materialized(0x1000, &back), Some(0x1007 - 0x10));
    }

    #[test]
    fn register_based_lea_names_nothing() {
        // lea eax,[rcx+rcx*4]
        assert_eq!(materialized(0x1000, &[0x8d, 0x04, 0x89]), None);
        // lea rax,[rbx+0x10]
        assert_eq!(materialized(0x1000, &[0x48, 0x8d, 0x43, 0x10]), None);
    }

    #[test]
    fn immediates_are_read_at_their_width() {
        // mov edi,0x401136
        assert_eq!(materialized(0x1000, &[0xbf, 0x36, 0x11, 0x40, 0x00]), Some(0x401136));
        // movabs rax,0x140001040
        let movabs = [0x48, 0xb8, 0x40, 0x10, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(materialized(0x1000, &movabs), Some(0x1_4000_1040));
        // mov qword [rip+0x100],0x401136
        let store = [0x48, 0xc7, 0x05, 0x00, 0x01, 0x00, 0x00, 0x36, 0x11, 0x40, 0x00];
        assert_eq!(materialized(0x1000, &store), Some(0x401136));
        // push 0x401000
        assert_eq!(materialized(0x1000, &[0x68, 0x00, 0x10, 0x40, 0x00]), Some(0x401000));
        // lea eax,[rip+0x10]: a 32-bit lea keeps the low half
        assert_eq!(materialized(0x1_0000_1000, &[0x8d, 0x05, 0x10, 0x00, 0x00, 0x00]), Some(0x1016));
    }

    #[test]
    fn narrowed_and_unrelated_forms_name_nothing() {
        // mov ax,0x1234
        assert_eq!(materialized(0x1000, &[0x66, 0xb8, 0x34, 0x12]), None);
        // xbegin (c7 f8 rel32)
        assert_eq!(materialized(0x1000, &[0xc7, 0xf8, 0x00, 0x00, 0x00, 0x00]), None);
        // call rel32
        assert_eq!(materialized(0x1000, &[0xe8, 0x00, 0x00, 0x00, 0x00]), None);
    }
}
