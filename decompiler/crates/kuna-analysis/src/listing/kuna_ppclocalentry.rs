//! (kuna) `ppclocalentry` — a PPC64 ELFv2 **local entry point** is not a
//! function (P1 code/data partition).
//!
//! # The defect
//!
//! The OpenPOWER ELFv2 ABI gives a function two entry points. The *global* entry
//! is the symbol's `st_value`; it materialises the TOC pointer `r2` from `r12`
//! (`addis r2,r12,hi; addi r2,r2,lo`) and falls into the *local* entry, which is
//! where a caller that already holds the right `r2` — anything in the same
//! module — branches instead. The distance between the two lives in the symbol's
//! `st_other` field (`readelf -sW` prints it as `[<localentry>: 8]`).
//!
//! Nothing in kuna read that field, so the recursive-descent walk saw an
//! intra-module `bl` land eight bytes past a function symbol and did what it does
//! for every other CALL target: it minted a function there. On an ordinary
//! `gcc`-built ppc64le image that splits **every locally-called function into
//! two** — the named symbol truncated to its 8-byte TOC prologue, and the whole
//! real body filed under an anonymous `sub_<hex>`:
//!
//! ```text
//! {"name": "__do_global_ctors_aux", "address_hex": "0x940", "size": 8}
//! {"name": "sub_948",               "address_hex": "0x948", "size": 112}
//! ```
//!
//! The husk is what an agent actually gets back. `p2_lift`'s `funcboundflow`
//! then truncates the flow at the first *known function entry* it falls through
//! into — which is the split it was handed — so `kuna decompile
//! __do_global_ctors_aux` emits an empty body with a `funcboundflow:
//! fall-through reached the next function entry` warning, and the constructor
//! loop is nowhere in the output under the name it belongs to.
//!
//! # The rule
//!
//! An address that some defined `STT_FUNC` symbol *declares* to be its own local
//! entry is, on the ABI's own authority, a point inside that function. It is
//! never a function of its own — the two entries are the same routine by
//! construction, which is why this is a strict correction rather than a
//! heuristic. So the walk withholds the function claim there, exactly as
//! [`super::kuna_unmappedentry`] does at the same seam.
//!
//! The instruction closure is unchanged, and provably so: the fold is only
//! applied when the **global** entry is itself a walk seed, so the bytes at the
//! local entry are reached as that function's fall-through either way. This
//! option can therefore never lose a body — it only ever removes the second,
//! duplicate entry over one.
//!
//! # Guards
//!
//! Four, all cheap, all readable off the symbol table:
//!
//! 1. `st_other`'s local-entry field must decode to a real offset. The ABI packs
//!    it in bits 5–7 as `(1 << n) >> 2 << 2`; `n` of 0 or 1 means *no* separate
//!    local entry and 7 is reserved, so only `n ∈ 2..=6` (4/8/16/32/64 bytes)
//!    folds.
//! 2. A sized symbol must actually contain its local entry (`offset < st_size`).
//!    Unsized symbols — what `gcc` emits for the `crtstuff.c` routines this fires
//!    on — skip the test and lean on guard 3.
//! 3. The local entry must not be the address of any other defined text symbol.
//!    That is the case where the "interior" address is really the next function,
//!    and it is the only way an unsized symbol could mislead us.
//! 4. The global entry must be a walk seed, and no other seed may lie between the
//!    two. This is what makes the closure argument above hold.
//!
//! PPC64-only (the ABI feature is), and inert on an image whose symbols carry no
//! local-entry annotation — a stripped one included, where there is no symbol to
//! read and therefore nothing to fold.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::architecture::Architecture;
use object::read::{Object, ObjectSymbol};
use object::{SymbolFlags, SymbolKind};

/// Decode the ELFv2 local-entry offset out of a symbol's `st_other`.
///
/// The ABI stores it in bits 5–7 as `(1 << n) >> 2 << 2`: `n` of 0 or 1 means the
/// local and global entries coincide (no fold), 2–6 mean 4/8/16/32/64 bytes, and
/// 7 is reserved. Returns `None` for every value that does not name a distinct
/// local entry.
pub(super) fn local_entry_offset(st_other: u8) -> Option<u64> {
    match (st_other >> 5) & 7 {
        n @ 2..=6 => Some(((1u64 << n) >> 2) << 2),
        _ => None,
    }
}

/// Map every PPC64 ELFv2 local entry VMA to the global entry that declares it,
/// applying guards 1–3. Pure over the object; `seeds` (guard 4) is applied by
/// [`fold_map`].
pub(super) fn declared_local_entries(file: &object::File) -> BTreeMap<u64, u64> {
    let mut folds = BTreeMap::new();
    if file.architecture() != object::Architecture::PowerPc64 {
        return folds;
    }
    // Guard 3's universe: every defined text symbol address in the image.
    let defined: BTreeSet<u64> = file
        .symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.kind() == SymbolKind::Text && s.address() != 0)
        .map(|s| s.address())
        .collect();

    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text || sym.address() == 0 {
            continue;
        }
        let SymbolFlags::Elf { st_other, .. } = sym.flags() else { continue };
        let Some(offset) = local_entry_offset(st_other) else { continue };
        if sym.size() != 0 && offset >= sym.size() {
            continue;
        }
        let local = sym.address().saturating_add(offset);
        if defined.contains(&local) {
            continue;
        }
        folds.insert(local, sym.address());
    }
    folds
}

/// The local-entry fold the walk consults: `local entry VMA -> global entry VMA`,
/// restricted to the folds whose global entry is a seed with no other seed in
/// between (guard 4). Empty whenever `--option ppclocalentry off`, which restores
/// the previous, husk-producing discovery set exactly.
pub(super) fn fold_map(
    arch: &Architecture,
    file: &object::File,
    seeds: &[u64],
) -> BTreeMap<u64, u64> {
    if !arch.analysis_ppclocalentry {
        return BTreeMap::new();
    }
    // Architecture-gated and cheap in that order: `declared_local_entries` answers
    // empty on the first check for every non-PPC64 image, so no seed set is ever
    // built there and this costs nothing on the arches that cannot need it.
    let declared = declared_local_entries(file);
    if declared.is_empty() {
        return declared;
    }
    let seeded: BTreeSet<u64> = seeds.iter().copied().collect();
    declared
        .into_iter()
        .filter(|&(local, global)| {
            seeded.contains(&global) && seeded.range((global + 1)..=local).next().is_none()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ABI's `(1 << n) >> 2 << 2` packing, including the three values that
    /// name no distinct local entry (0 and 1 mean the entries coincide, 7 is
    /// reserved).
    #[test]
    fn local_entry_offset_decodes_the_st_other_field() {
        let at = |n: u8| local_entry_offset(n << 5);
        assert_eq!(at(0), None);
        assert_eq!(at(1), None);
        assert_eq!(at(2), Some(4));
        assert_eq!(at(3), Some(8));
        assert_eq!(at(4), Some(16));
        assert_eq!(at(5), Some(32));
        assert_eq!(at(6), Some(64));
        assert_eq!(at(7), None);
        // The low five bits are visibility/reserved and must not leak in.
        assert_eq!(local_entry_offset((3 << 5) | 0x1f), Some(8));
    }

    /// The repo's ppc64le fixture: every `crtstuff.c` routine carries
    /// `[<localentry>: 8]`, and each fold points 8 bytes back at its own symbol.
    #[test]
    fn fixture_declares_a_local_entry_for_every_crtstuff_routine() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/plt_ppc64le");
        let bytes = std::fs::read(path).expect("fixture readable");
        let file = object::File::parse(&*bytes).expect("fixture parses");
        let folds = declared_local_entries(&file);
        for (global, local) in [(0x6f0, 0x6f8), (0x750, 0x758), (0x7c0, 0x7c8), (0x8b0, 0x8b8), (0x940, 0x948)] {
            assert_eq!(
                folds.get(&local),
                Some(&global),
                "0x{local:x} is 0x{global:x}'s ELFv2 local entry"
            );
        }
        // Every fold is 8 bytes past its global entry and nothing else folds.
        for (&local, &global) in &folds {
            assert_eq!(local - global, 8, "fixture is built with an 8-byte TOC prologue");
        }
    }

    /// A non-PPC64 image folds nothing at all, whatever its symbols look like.
    #[test]
    fn non_ppc64_images_fold_nothing() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("fixture readable");
        let file = object::File::parse(&*bytes).expect("fixture parses");
        assert!(declared_local_entries(&file).is_empty());
    }
}
