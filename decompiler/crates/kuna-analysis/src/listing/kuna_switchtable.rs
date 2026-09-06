//! (kuna) `switchtable` — read a computed jump's target table out of the image,
//! so the case bodies of a switch belong to the function that dispatches to them.
//!
//! # The gap
//!
//! [`super::xrefs`]'s walk is a recursive descent over [`super::classify`]'s
//! static successors, and a `BRANCHIND` contributes none (`classify.rs`: "no
//! static target, deferred jump-table resolution"). So the walk dies at every
//! switch dispatch: on an MSVC i386 window procedure whose handler is
//!
//! ```text
//! 0x401435  JA   0x40172a                        ; default
//! 0x40143b  JMP  dword ptr [EAX*0x4 + 0x4017c4]  ; ten cases
//! ...
//! 0x4016ef  PUSH 0x403288                        ; "Product Already Registered"
//! ```
//!
//! the whole case-body region between the dispatch and the epilogue is
//! undecoded, every reference it forms is missing, and `kuna strings` reports a
//! literal the disassembly plainly pushes with `xrefs_count 0` and no owning
//! function. The engine's own switch recovery (`p2_lift/jumptable.rs`) does
//! reach that body, but it runs over a decompiled function; this tier has only
//! the bytes.
//!
//! # The rule
//!
//! One table read, of content that cannot change, bounded by the image's own
//! partition. The instruction must be a computed **jump** with no static target,
//! and the table base is a constant the instruction itself materializes — the
//! [`XrefKind::Data`](super::xrefs::XrefKind::Data) reference the constant scan
//! already files for it. From that base the entries are read in order while each
//! one is
//!
//!  * a pointer-sized, pointer-aligned word of **allocated, non-writable** memory
//!    with file content (the same [`PoolImage`] dereference `poolref` uses), and
//!  * an address inside the **same executable section** as the dispatching
//!    instruction — a case body is code, and it is the dispatcher's own code.
//!
//! The first word that is not both ends the table, which is what stops the scan
//! at the `cc cc cc cc` padding after the last case on the example above. A run
//! of fewer than [`MIN_ENTRIES`] is not a table: one plausible word after a
//! constant is a coincidence, a switch is not.
//!
//! A `ram` *varnode* base is deliberately not a candidate. `jmp dword ptr
//! [__imp_X]` — a PE import veneer, an ELF PLT entry — encodes its slot as a
//! direct data-space operand, which the constant scan files as a
//! [`Read`](super::xrefs::XrefKind::Read) and not a `Data`, so a veneer is never
//! read as a one-entry table of whatever its unrelocated slot happens to hold.

use super::kuna_poolref::PoolImage;
use super::xrefs::in_range;

/// Below this an accepted run is a coincidence rather than a table: a compiler
/// does not lower a one-case switch through a jump table.
const MIN_ENTRIES: usize = 2;

/// The largest table read, the same ceiling the engine's own switch recovery
/// takes (`Architecture::max_jumptable_size`, 1024). A scan that runs this far
/// has stopped describing a switch and is walking whatever follows it.
const MAX_ENTRIES: usize = 1024;

/// The case bodies the computed jump at `from` dispatches to through a table at
/// `base`, or empty when `base` does not front one.
///
/// `exec` is the executable partition; the entries are confined to the range of
/// it that contains `from`, so a table cannot carry the walk into a different
/// section's code.
pub(super) fn targets(base: u64, from: u64, pool: &PoolImage, exec: &[(u64, u64)]) -> Vec<u64> {
    let Some(&section) = exec.iter().find(|&&(lo, hi)| from >= lo && from < hi) else {
        return Vec::new();
    };
    let stride = u64::from(pool.ptr_width());
    let mut out = Vec::new();
    for i in 0..MAX_ENTRIES as u64 {
        let Some(at) = base.checked_add(i * stride) else { break };
        let Some(word) = pool.word_at(at) else { break };
        if !in_range(&[section], word) {
            break;
        }
        out.push(word);
    }
    if out.len() < MIN_ENTRIES {
        out.clear();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A three-entry table at 0x2000 into code at 0x1000..0x1100, then a word of
    /// `0xcccccccc` padding — the shape MSVC emits.
    const TABLE: [u8; 20] = [
        0x10, 0x10, 0x00, 0x00, // 0x2000 -> 0x1010
        0x20, 0x10, 0x00, 0x00, // 0x2004 -> 0x1020
        0x30, 0x10, 0x00, 0x00, // 0x2008 -> 0x1030
        0xcc, 0xcc, 0xcc, 0xcc, // 0x200c -> padding
        0x40, 0x10, 0x00, 0x00, // 0x2010 -> 0x1040 (past the padding)
    ];
    const EXEC: [(u64, u64); 1] = [(0x1000, 0x1100)];

    fn image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x2000, 0x2014, &TABLE[..])], 4, true).unwrap()
    }

    #[test]
    fn a_table_of_code_addresses_yields_every_case_body() {
        assert_eq!(targets(0x2000, 0x1008, &image(), &EXEC), vec![0x1010, 0x1020, 0x1030]);
    }

    /// The stop rule: the first word that is not a code address ends the table,
    /// so the entry past the padding is not taken up.
    #[test]
    fn the_scan_stops_at_the_first_word_that_is_not_code() {
        let t = targets(0x2000, 0x1008, &image(), &EXEC);
        assert!(!t.contains(&0x1040));
    }

    /// A single plausible word after a constant is a coincidence, not a switch.
    #[test]
    fn a_run_shorter_than_two_entries_is_not_a_table() {
        assert!(targets(0x2008, 0x1008, &image(), &EXEC).is_empty());
    }

    /// A case body is the dispatcher's own code: a word landing in another
    /// executable range ends the table rather than extending it.
    #[test]
    fn a_word_outside_the_dispatchers_own_section_ends_the_table() {
        let exec = [(0x1000, 0x1015), (0x1020, 0x1100)];
        assert!(targets(0x2000, 0x1008, &image(), &exec).is_empty());
    }

    /// Nothing is read for an instruction the executable partition does not
    /// contain (a relocatable object, whose sections are not the runtime ones).
    #[test]
    fn a_dispatch_in_no_executable_section_reads_no_table() {
        assert!(targets(0x2000, 0x9000, &image(), &EXEC).is_empty());
    }

    /// The table must be read-only image content: an unaligned or unmapped base
    /// is not one.
    #[test]
    fn a_base_that_is_not_a_readable_word_reads_no_table() {
        assert!(targets(0x2001, 0x1008, &image(), &EXEC).is_empty());
        assert!(targets(0x8000, 0x1008, &image(), &EXEC).is_empty());
    }
}
