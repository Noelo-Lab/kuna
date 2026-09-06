//! The inventory-time function extent — the `size` every whole-binary surface
//! reports per entry.
//!
//! # What this is
//!
//! kuna's model of a function is its **entry**: the Listing is keyed by entry
//! VMA and [`ConsoleProgram::function_entries_canonical`] yields one record per
//! entry address. Nothing in that model carries a body, so an inventory record
//! had no extent at all and a caller could not tell a PLT thunk from `main`
//! without decompiling the whole binary (`docs/re-needs/functions-json-size.md`).
//!
//! This module supplies the extent as the **address-contiguous clip**
//! `[entry, min(next_entry, end_of_containing_code_section))` — the same
//! reconstruction `kuna_analysis`'s FID extent generator
//! (`analyzers/fid/extent.rs`) and `noreturn_disc` already use where they need a
//! body from an entry-keyed model. It reuses the entry list and the loader's
//! section table, both already in hand, so the cheap inventory call stays cheap:
//! no decode, no Listing, no per-function work beyond a binary search.
//!
//! # What the number means
//!
//! An **upper bound** on the function's byte extent, not its exact body. The
//! clip runs to the next entry, so inter-function alignment padding is counted
//! in. Measured against ELF `st_size` over the 41 symbolized fixtures in
//! `kuna-analysis/tests/fixtures` (1428 functions with ground truth): never
//! short, exact for 231, median overshoot +8 bytes, worst +52 — i.e. padding.
//! That is the right shape for what the field is for (ordering an inventory by
//! weight); a caller needing the exact body must still decompile.
//!
//! An entry outside every CODE section reports `0` — an import pointer slot or
//! an undefined external is an address, not a body, and `0` is the established
//! "no extent recovered" answer on the decompile path.
//!
//! That sentinel only means what it says while a section table exists. An image
//! carrying none — a sectionless ELF, or one whose section headers are corrupt
//! enough that the loader continues from the program headers — publishes no CODE
//! span anywhere, so *every* entry took the sentinel and the whole binary
//! reported `0` (`docs/re-needs/zero-function-sizes-make.md`: size-based triage
//! then discards all of it, `--min-size 1` answering count 0 of total 12 with no
//! error). When the section table yields no CODE span at all, [`spans`] clips
//! against the executable load segments instead — coarser, but the same kind of
//! answer, since the number was always an upper bound clipped at the next entry.
//! The fallback is whole-table, never per-entry: an entry that misses the CODE
//! spans an image *does* publish is the import pointer slot the sentinel exists
//! for, and widening it there would hand a body to exactly those.
//!
//! # LOSS
//!
//! Address-contiguous, not flow-reachable: an outlined or interleaved body (a
//! `.part.0` cold half living past the next entry) is clipped at the neighbour,
//! and code physically between two entries is attributed to the first. This is
//! the approximation the FID generator documents and accepts for the same
//! reason — the compiler lays a function down contiguously in the common case.

/// One CODE section as `(vma, end)`, ascending by `vma`.
type CodeSpan = (u64, u64);

/// The extent of the function entered at `entry`.
///
/// `next` is the next entry address strictly after `entry` (`None` for the last
/// one), and `code` the ascending `(vma, end)` CODE spans. Returns `0` when
/// `entry` lies in no CODE section.
///
/// Split out from the bulk pass so the clip rule is unit-testable without a
/// loaded [`crate::engine::ConsoleProgram`] — the shape `analyzers/fid/extent.rs`
/// uses for the same reason.
fn clip(entry: u64, next: Option<u64>, code: &[CodeSpan]) -> u64 {
    let Some(&(_, end)) = code.iter().find(|&&(vma, end)| vma <= entry && entry < end) else {
        return 0;
    };
    // The neighbour only tightens the bound when it really is ahead of this
    // entry and inside the same section; a duplicate or out-of-section next
    // entry leaves the section end as the stop.
    let stop = match next {
        Some(n) if n > entry && n < end => n,
        _ => end,
    };
    stop - entry
}

/// The ascending `(vma, end)` CODE spans of a loader section table
/// (`ConsoleProgram::sections`, `(vma, size, flags)`).
///
/// Zero-length and non-CODE sections are dropped; a section whose `vma + size`
/// wraps is dropped rather than clamped, so a malformed table yields `0` extents
/// instead of nonsense ones.
pub(crate) fn code_spans(sections: &[(u64, u64, u32)]) -> Vec<CodeSpan> {
    let mut spans: Vec<CodeSpan> = sections
        .iter()
        .filter(|(_, _, flags)| flags & kuna_sleigh::loadimage::section_flags::CODE != 0)
        .filter_map(|&(vma, size, _)| vma.checked_add(size).map(|end| (vma, end)))
        .filter(|&(vma, end)| end > vma)
        .collect();
    spans.sort_unstable();
    spans
}

/// The CODE spans to clip against: the section table's, or the executable load
/// segments' when the table publishes none.
///
/// Both arguments are `(vma, size, flags)` in the loader's own vocabulary
/// (`ConsoleProgram::sections` and `ConsoleProgram::segments`), which is why one
/// filter reads both — a segment's CODE bit is its execute permission.
pub(crate) fn spans(sections: &[(u64, u64, u32)], segments: &[(u64, u64, u32)]) -> Vec<CodeSpan> {
    let from_sections = code_spans(sections);
    if !from_sections.is_empty() {
        return from_sections;
    }
    code_spans(segments)
}

/// The extent of a single address against an ascending entry list — the
/// single-target paths (`--addr` on an address the enumeration does not know),
/// where there is no bulk pass to piggyback on.
pub(crate) fn extent_at(entry: u64, ascending_entries: &[u64], code: &[CodeSpan]) -> u64 {
    let next = ascending_entries.iter().copied().find(|&e| e > entry);
    clip(entry, next, code)
}

/// Fill each entry's extent in one pass over an **address-ascending** entry list
/// (what `function_entries_canonical` builds from its `BTreeMap`).
pub(crate) fn assign_extents(entries: &mut [crate::engine::FunctionEntry], code: &[CodeSpan]) {
    for i in 0..entries.len() {
        let entry = entries[i].addr.get_offset();
        let next = entries.get(i + 1).map(|e| e.addr.get_offset());
        entries[i].size = clip(entry, next, code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &[CodeSpan] = &[(0x1000, 0x1100)];

    #[test]
    fn clips_at_the_next_entry() {
        assert_eq!(clip(0x1000, Some(0x1020), TEXT), 0x20);
    }

    #[test]
    fn last_entry_runs_to_the_section_end() {
        assert_eq!(clip(0x10c0, None, TEXT), 0x40);
    }

    #[test]
    fn next_entry_outside_the_section_does_not_extend_past_it() {
        // The neighbour is the first entry of the NEXT code section, so the clip
        // must stop at this section's end rather than swallow the gap between
        // them (the `.init` → `.plt` shape: 0x1000+0x1b then 0x1020).
        assert_eq!(clip(0x10c0, Some(0x2000), TEXT), 0x40);
    }

    #[test]
    fn entry_outside_every_code_section_has_no_extent() {
        // A `.got` import slot or an undefined external: an address, not a body.
        assert_eq!(clip(0x5000, Some(0x5008), TEXT), 0);
    }

    #[test]
    fn picks_the_containing_section_not_the_first() {
        let code: &[CodeSpan] = &[(0x1000, 0x101b), (0x1020, 0x1030), (0x1040, 0x1673)];
        // `.plt` stub: bounded by its own section, not by `.text`'s end.
        assert_eq!(clip(0x1020, Some(0x1040), code), 0x10);
        // `.init`: the next entry is in another section, so `.init`'s end wins.
        assert_eq!(clip(0x1000, Some(0x1020), code), 0x1b);
    }

    #[test]
    fn a_duplicate_or_backward_neighbour_is_ignored() {
        assert_eq!(clip(0x1040, Some(0x1040), TEXT), 0x1100 - 0x1040);
        assert_eq!(clip(0x1040, Some(0x1000), TEXT), 0x1100 - 0x1040);
    }

    #[test]
    fn code_spans_keeps_only_sized_code_sections() {
        use kuna_sleigh::loadimage::section_flags;
        let sections = vec![
            (0x1000, 0x100, section_flags::CODE),
            (0x2000, 0x100, section_flags::DATA),
            (0x3000, 0, section_flags::CODE),          // zero length
            (u64::MAX, 0x10, section_flags::CODE),     // wraps
            (0x0500, 0x080, section_flags::CODE | section_flags::READONLY),
        ];
        assert_eq!(code_spans(&sections), vec![(0x500, 0x580), (0x1000, 0x1100)]);
    }

    #[test]
    fn spans_prefer_the_section_table() {
        use kuna_sleigh::loadimage::section_flags;
        let sections = vec![(0x1000, 0x100, section_flags::CODE)];
        let segments = vec![(0x0, 0x8000, section_flags::CODE)];
        assert_eq!(spans(&sections, &segments), vec![(0x1000, 0x1100)]);
    }

    #[test]
    fn spans_fall_back_to_the_executable_segments() {
        use kuna_sleigh::loadimage::section_flags;
        // A sectionless ELF: no section table at all, four PT_LOADs, one of them
        // executable. Only the executable one can contain a body.
        let segments = vec![
            (0x0000, 0x810, section_flags::DATA | section_flags::READONLY),
            (0x1000, 0x12d5, section_flags::CODE | section_flags::READONLY),
            (0x3000, 0xd20, section_flags::DATA | section_flags::READONLY),
            (0x7d78, 0x2a8, section_flags::DATA),
        ];
        assert_eq!(spans(&[], &segments), vec![(0x1000, 0x22d5)]);
        // The last entry of that segment runs to its end, where before every
        // entry in the image reported 0.
        assert_eq!(clip(0x12d0, None, &spans(&[], &segments)), 0x1005);
    }

    #[test]
    fn a_section_table_with_no_code_still_falls_back() {
        use kuna_sleigh::loadimage::section_flags;
        // The blindness is "no CODE span", not "no sections": a table of pure
        // data sections leaves the same empty world an absent one does.
        let sections = vec![(0x3000, 0x100, section_flags::DATA)];
        let segments = vec![(0x1000, 0x100, section_flags::CODE)];
        assert_eq!(spans(&sections, &segments), vec![(0x1000, 0x1100)]);
    }

    #[test]
    fn no_segments_either_leaves_the_extent_unrecovered() {
        assert_eq!(spans(&[], &[]), Vec::<CodeSpan>::new());
    }

    #[test]
    fn extent_at_finds_the_next_entry_in_the_list() {
        let entries = [0x1000, 0x1020, 0x1080];
        assert_eq!(extent_at(0x1020, &entries, TEXT), 0x60);
        assert_eq!(extent_at(0x1080, &entries, TEXT), 0x80);
        // An address the enumeration does not know still clips against its
        // neighbours (`--addr` on an undiscovered function).
        assert_eq!(extent_at(0x1040, &entries, TEXT), 0x40);
    }
}
