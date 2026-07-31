//! The function-extent generator — the code units the FID hasher fingerprints.
//!
//! # What this is
//!
//! Ghidra's `FunctionExtentGenerator` returns the ordered list of code units that
//! make up a function's body. The concrete strategy FID uses is
//! `FunctionBodyFunctionExtentGenerator`: it walks `func.getBody()` (the address
//! set the FunctionManager recorded as the body) and collects the instructions in
//! ascending address order.
//!
//! kuna's Listing tracks function **entries**, not bodies (`listing/mod.rs` — the
//! `DiscoveredFunction` model is keyed by entry VMA; there is no per-function
//! body address set). So this generator reconstructs the body as the
//! **address-contiguous clip** `[entry, next_function_after(entry))` over the
//! flat, address-ordered instruction map — the exact idiom
//! [`crate::noreturn_disc`] already uses for its call-site / fall-through
//! reasoning.
//!
//! # LOSS (documented, §1 of `docs/history/fid-design.md`)
//!
//! This is **address-contiguous, not flow-reachable**: an interleaved or outlined
//! body (code from another function physically sitting between `entry` and the
//! next function entry, or this function's own code living after the next entry)
//! is mis-clipped. This is the same approximation `noreturn_disc` accepts and
//! the one place kuna's FID is *less* faithful than Ghidra's. For the common case
//! (the compiler lays a function down contiguously) it is exact.
//!
//! # Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/hash/FunctionBodyFunctionExtentGenerator.java`
//!   — `calculateExtent(func)`: iterate `listing.getInstructions(body, true)`
//!   (ascending) and collect them. kuna substitutes the `[entry, next)` clip for
//!   `func.getBody()`.

use crate::listing::{Insn, Listing};

/// Reconstruct the extent of the function entered at `entry`: the decoded
/// instructions in `[entry, next_function_after(entry))`, in ascending address
/// order.
///
/// Returns the instructions in body order — the input to
/// [`crate::fid::hash::FidHasher`] (PR4 turns each `&Insn` + its SLEIGH mask
/// into an [`crate::fid::hash::InsnFingerprint`]).
///
/// The hasher itself rejects extents below
/// [`SHORT_HASH_CODE_UNIT_LENGTH`](crate::fid::hash::SHORT_HASH_CODE_UNIT_LENGTH);
/// this generator returns whatever the clip contains (possibly empty).
pub fn calculate_extent<'a>(listing: &'a Listing, entry: u64) -> Vec<&'a Insn> {
    // The exclusive upper bound: the next function entry strictly after `entry`
    // (`mod.rs::next_function_after`). `None` ⇒ the clip runs to the end of the
    // instruction map.
    let next = listing.next_function_after(entry).map(|f| f.entry);
    clip_extent(listing.instructions_in_range(entry, next), entry, next)
}

/// The pure clip: given the address-ordered `(vma, insn)` stream, keep the
/// instructions in `[entry, next)` (with `next = None` meaning "to the end").
///
/// Split out from [`calculate_extent`] so the clip rule is unit-testable without
/// constructing a full [`Listing`] (which needs an `object::File` + the SLEIGH
/// decoder). The production path supplies the Listing's range-bounded iterator.
fn clip_extent<'a, I>(instructions: I, entry: u64, next: Option<u64>) -> Vec<&'a Insn>
where
    I: IntoIterator<Item = (&'a u64, &'a Insn)>,
{
    let mut extent: Vec<&Insn> = Vec::new();
    for (&vma, insn) in instructions {
        if vma < entry {
            // Before this function's entry — `getInstructions(body, true)` would
            // not include it (the body starts at `entry`).
            continue;
        }
        if matches!(next, Some(n) if vma >= n) {
            // Reached the next function's entry — the clip is exclusive of it.
            // The map is address-ordered, so everything past here is out of body.
            break;
        }
        extent.push(insn);
    }
    extent
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn insn_at(addr: u64, len: u32) -> Insn {
        Insn {
            addr,
            len,
            fall_through: Some(addr + len as u64),
            flow: Default::default(),
            flows: vec![],
            mnemonic: String::new(),
            operands: String::new(),
            pcode: None,
        }
    }

    /// Build an address-ordered instruction map (the shape `instructions()`
    /// yields) for the clip tests.
    fn map_of(insns: Vec<Insn>) -> BTreeMap<u64, Insn> {
        insns.into_iter().map(|i| (i.addr, i)).collect()
    }

    #[test]
    fn clips_between_consecutive_function_entries() {
        // Two functions: f@0x1000 (3 insns) and g@0x1010 (2 insns), laid out
        // contiguously. The extent of f is exactly its three instructions.
        let map = map_of(vec![
            insn_at(0x1000, 4),
            insn_at(0x1004, 4),
            insn_at(0x1008, 4),
            insn_at(0x1010, 4),
            insn_at(0x1014, 4),
        ]);

        let ext_f: Vec<u64> = clip_extent(map.iter(), 0x1000, Some(0x1010))
            .iter()
            .map(|i| i.addr)
            .collect();
        assert_eq!(ext_f, vec![0x1000, 0x1004, 0x1008]);

        let ext_g: Vec<u64> = clip_extent(map.iter(), 0x1010, None)
            .iter()
            .map(|i| i.addr)
            .collect();
        assert_eq!(ext_g, vec![0x1010, 0x1014]);
    }

    #[test]
    fn last_function_runs_to_end_of_map() {
        let map = map_of(vec![
            insn_at(0x2000, 2),
            insn_at(0x2002, 2),
            insn_at(0x2004, 2),
        ]);
        let ext: Vec<u64> = clip_extent(map.iter(), 0x2000, None)
            .iter()
            .map(|i| i.addr)
            .collect();
        assert_eq!(ext, vec![0x2000, 0x2002, 0x2004]);
    }

    #[test]
    fn instructions_before_entry_excluded() {
        // A function entry that is not the first instruction in the map: anything
        // before the entry is not part of the body.
        let map = map_of(vec![
            insn_at(0x3000, 4), // before entry
            insn_at(0x3004, 4), // entry
            insn_at(0x3008, 4),
        ]);
        let ext: Vec<u64> = clip_extent(map.iter(), 0x3004, None)
            .iter()
            .map(|i| i.addr)
            .collect();
        assert_eq!(ext, vec![0x3004, 0x3008]);
    }

    #[test]
    fn empty_clip_when_no_instructions_in_range() {
        // An entry past every instruction yields an empty extent (the hasher then
        // rejects it as < 4 code units).
        let map = map_of(vec![insn_at(0x4000, 4), insn_at(0x4004, 4)]);
        let ext = clip_extent(map.iter(), 0x5000, None);
        assert!(ext.is_empty());
    }
}
