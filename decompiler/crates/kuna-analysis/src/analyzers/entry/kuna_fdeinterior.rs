//! (kuna) `fdeinterior` — never start a function inside another function's
//! `.eh_frame` FDE (P1 code/data partition).
//!
//! kuna's function symbols carry no extent: a `FunctionSymbol` is an entry
//! address, so the commit boundary's "is this candidate already inside a known
//! function?" question has no answer and every discovery oracle is free to plant
//! a `sub_<addr>` in the middle of a body it cannot see. Three of them do it on
//! ordinary compiler output:
//!
//! * `eh_frame_full` promotes every `.gcc_except_table` **landing pad** to a
//!   top-level function. A landing pad is the block the unwinder jumps to
//!   mid-frame; it inherits the parent's live `rbp`, so on its own it decompiles
//!   with an uninitialised frame pointer and every local becomes a garbage
//!   dereference (`g++ -O0`, `probe_try_catch` at `[0x2acf, 0x2ba7)`: `sub_2b0b`,
//!   `sub_2b8a`).
//! * `aif`, the aggressive gap walk, starts a "function" at the first undecoded
//!   byte of a region the walk could not reach — which for an exception region is
//!   in the middle of an instruction (`sub_2b1a`, the second byte of a `je`).
//! * the prologue-pattern oracles match a 16-byte-aligned `push rbp; mov rbp,rsp`
//!   that happens to sit inside a larger body.
//!
//! `.eh_frame` supplies exactly the extent kuna lacks: every FDE records its
//! function's `pcBegin` and `pcRange`, and the FDE is per-function by
//! construction (one `.cfi_startproc`/`.cfi_endproc` pair). So a discovered entry
//! that falls **strictly inside** an FDE body is, on the unwinder's own
//! authority, not a function — it is a point inside one. That is IDA's model:
//! `get_func(0x2b0b).start_ea == 0x2acf`, the landing pad taken from the FDE.
//!
//! ## What makes a range eligible
//!
//! Not every FDE describes one function. The linker emits a SINGLE FDE covering
//! the whole PLT, and every stub inside it is a real function — suppressing
//! interiors there would delete every import. A range is therefore used only when
//! it is a single-function frame:
//!
//! 1. no already-named function start (`.symtab`/`.dynsym` FUNC + PLT stubs) lies
//!    strictly inside it,
//! 2. no other FDE's `pcBegin` lies strictly inside it, and
//! 3. it does not overlap a stub section (`.plt`, `.plt.sec`, `.plt.got`,
//!    `.iplt`, `.MIPS.stubs`).
//!
//! Guard 1 alone rejects the PLT on every binary measured; 2 and 3 are the cheap
//! belt-and-suspenders for a stripped image whose stubs carry no name.
//!
//! ## Scope
//!
//! ELF-only, and inert on any image without `.eh_frame` FDEs — which includes
//! essentially the whole bare-metal ARM Cortex-M population (3 of 98 decbench
//! images carry any FDE at all; the rest unwind through `.ARM.exidx`), so the
//! ARM entry-recall work this composes with (`cortexmvectors`, `ptrentry`,
//! `tailcallentry`) is structurally untouched. An entry AT an FDE `pcBegin` is
//! never suppressed, so the `.eh_frame` oracle's own product is preserved.

use object::read::{Object, ObjectSection};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{
    decode_eh_pointer, encoded_size, existing_function_addrs, parse_cie_fde_encoding, read_u32,
    read_u64, DW_EH_PE_ABSPTR,
};

/// Section names whose contents are linker-generated stubs sharing one FDE.
const STUB_SECTIONS: &[&str] = &[".plt", ".plt.sec", ".plt.got", ".iplt", ".MIPS.stubs"];

/// (kuna) The FDE-interior entry-suppression pass (`fdeinterior`).
///
/// Runs at load like every other analysis pass and reports the eligible FDE
/// bodies in [`AnalysisOutput::fde_bodies`]; the **commit** applies them
/// ([`suppress_interior_entries`]) only when `--option fdeinterior on` let this
/// pass's output through the gate, so `off` restores the previous discovery set
/// exactly.
pub struct FdeInteriorPass;

impl AnalysisPass for FdeInteriorPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "fdeinterior"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        AnalysisOutput {
            fde_bodies: eligible_fde_bodies(ctx.file, ctx.bytes),
            ..AnalysisOutput::default()
        }
    }
}

/// Drop every discovered entry that falls strictly inside one of `bodies`.
///
/// Entries at a body's first address are kept (that address IS the function), so
/// this can only ever remove an entry no `.eh_frame` FDE endorses as a start.
/// With `bodies` empty (the gate off, a non-ELF image, or no `.eh_frame`) it is a
/// no-op.
pub fn suppress_interior_entries(entries: &mut Vec<u64>, bodies: &[(u64, u64)]) -> Vec<u64> {
    if bodies.is_empty() || entries.is_empty() {
        return Vec::new();
    }
    let mut dropped = Vec::new();
    entries.retain(|&vma| {
        if is_interior(bodies, vma) {
            dropped.push(vma);
            false
        } else {
            true
        }
    });
    dropped
}

/// Is `vma` strictly inside one of `bodies`? The ranges are sorted and disjoint
/// (an overlapping set is rejected wholesale by [`scan_fde_bodies`]), so only the
/// nearest range starting at or below `vma` can contain it.
pub fn is_interior(bodies: &[(u64, u64)], vma: u64) -> bool {
    let idx = bodies.partition_point(|&(start, _)| start <= vma);
    idx > 0 && {
        let (start, end) = bodies[idx - 1];
        start < vma && vma < end
    }
}

/// The `.eh_frame` FDE bodies that describe exactly one function — the ranges
/// whose interior may be used to reject a discovered entry.
pub fn eligible_fde_bodies(file: &object::File, bytes: &[u8]) -> Vec<(u64, u64)> {
    let all = scan_fde_bodies(file);
    if all.is_empty() {
        return all;
    }
    let named = existing_function_addrs(file, bytes);
    let starts: Vec<u64> = all.iter().map(|&(start, _)| start).collect();
    let stubs = stub_ranges(file);
    all.iter()
        .copied()
        .filter(|&(start, end)| {
            let contains_named = contains_strictly(&named, start, end);
            let contains_start = contains_strictly(&starts, start, end);
            let overlaps_stub = stubs.iter().any(|&(lo, hi)| start < hi && lo < end);
            !contains_named && !contains_start && !overlaps_stub
        })
        .collect()
}

/// Does the sorted `addrs` hold any value strictly between `start` and `end`?
fn contains_strictly(addrs: &[u64], start: u64, end: u64) -> bool {
    addrs.partition_point(|&a| a <= start) < addrs.partition_point(|&a| a < end)
}

/// `[address, address + size)` of every linker-stub section.
fn stub_ranges(file: &object::File) -> Vec<(u64, u64)> {
    file.sections()
        .filter(|s| s.name().map(|n| STUB_SECTIONS.contains(&n)).unwrap_or(false))
        .map(|s| (s.address(), s.address() + s.size()))
        .filter(|&(lo, hi)| hi > lo)
        .collect()
}

/// Every `.eh_frame` FDE's `[pcBegin, pcBegin + pcRange)`, sorted by start.
///
/// The record walk is [`super::scan_eh_frame_starts`]'s, extended by the one
/// field it skips: `pcRange` sits immediately after `pcBegin` in the same CIE
/// `R` encoding, but is a LENGTH — so it is decoded with the encoding's format
/// bits only, never its pc-relative application.
pub fn scan_fde_bodies(file: &object::File) -> Vec<(u64, u64)> {
    scan_fde_bodies_sized(file, 8)
}

fn scan_fde_bodies_sized(file: &object::File, ptr_size: usize) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }
    let Some(sec) = file.section_by_name(".eh_frame") else {
        return out;
    };
    let sec_vma = sec.address();
    let Ok(data) = sec.data() else {
        return out;
    };
    if data.is_empty() {
        return out;
    }
    let le = file.is_little_endian();

    let mut cie_enc: Vec<(usize, u8)> = Vec::new();
    let mut o = 0usize;
    while o + 4 <= data.len() {
        let length = read_u32(&data[o..], le) as usize;
        if length == 0 {
            break;
        }
        if length == 0xffff_ffff {
            if o + 12 > data.len() {
                break;
            }
            let ext = read_u64(&data[o + 4..], le) as usize;
            o = match o.checked_add(12).and_then(|v| v.checked_add(ext)) {
                Some(v) => v,
                None => break,
            };
            continue;
        }
        let next = o + 4 + length;
        if next > data.len() || o + 8 > data.len() {
            break;
        }
        let cie_id = read_u32(&data[o + 4..], le);
        if cie_id == 0 {
            let enc = parse_cie_fde_encoding(&data[o..next], ptr_size).unwrap_or(DW_EH_PE_ABSPTR);
            cie_enc.push((o, enc));
        } else {
            let cie_ptr_field = o + 4;
            if (cie_id as usize) <= cie_ptr_field {
                let cie_off = cie_ptr_field - cie_id as usize;
                if let Some(enc) = cie_enc.iter().find(|&&(co, _)| co == cie_off).map(|&(_, e)| e) {
                    let pc_begin_off = o + 8;
                    let field_vma = sec_vma + pc_begin_off as u64;
                    let pc_begin =
                        decode_eh_pointer(enc, field_vma, sec_vma, data, pc_begin_off, ptr_size);
                    let range_off =
                        pc_begin_off + encoded_size(enc, ptr_size, &data[pc_begin_off..]);
                    // Length field: the CIE's format, absolute application.
                    let pc_range =
                        decode_eh_pointer(enc & 0x0f, 0, 0, data, range_off, ptr_size);
                    if let (Some(begin), Some(range)) = (pc_begin, pc_range) {
                        if range > 0 {
                            out.push((begin, begin.wrapping_add(range)));
                        }
                    }
                }
            }
        }
        o = next;
    }

    // A `pcRange` read is only trusted when the record walk stayed self-consistent;
    // an out-of-order or overlapping set is dropped wholesale rather than guessed at.
    out.sort_unstable();
    out.dedup();
    if out.windows(2).any(|w| w[0].1 > w[1].0) {
        return Vec::new();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interior_is_strict() {
        let bodies = vec![(0x1000, 0x1100), (0x2000, 0x2010)];
        assert!(!is_interior(&bodies, 0x1000), "an FDE start is a function, not an interior");
        assert!(is_interior(&bodies, 0x1080));
        assert!(!is_interior(&bodies, 0x1100), "the open end belongs to the next function");
        assert!(!is_interior(&bodies, 0x1200));
        assert!(is_interior(&bodies, 0x2008));
    }

    #[test]
    fn suppress_keeps_starts_and_gaps() {
        let bodies = vec![(0x1000, 0x1100)];
        let mut entries = vec![0x1000, 0x1010, 0x10f0, 0x1100, 0x2000];
        let dropped = suppress_interior_entries(&mut entries, &bodies);
        assert_eq!(entries, vec![0x1000, 0x1100, 0x2000]);
        assert_eq!(dropped, vec![0x1010, 0x10f0]);
    }

    #[test]
    fn no_bodies_is_a_no_op() {
        let mut entries = vec![0x1010, 0x1020];
        let dropped = suppress_interior_entries(&mut entries, &[]);
        assert_eq!(entries, vec![0x1010, 0x1020]);
        assert!(dropped.is_empty());
    }
}
