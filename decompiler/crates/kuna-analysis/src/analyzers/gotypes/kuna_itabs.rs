//! (kuna `gotypeglobals`) The second `gotypes` CONSUMER — the runtime's
//! interface-dispatch tables (`itab`s) become NAMED data globals.
//!
//! Every interface value a Go program builds carries an `itab` — a record
//! binding one interface type to one concrete type, holding the method table
//! the dispatch jumps through. Go's linker names each static one
//! `go:itab.<concrete>,<interface>` (`go:itab.main.Point,fmt.Stringer`), a
//! name a stripped image loses while the records themselves stay reachable
//! through the moduledata. Reinstating the names is what turns an interface
//! conversion's `dat_<addr>` operand into the pair it actually encodes, and
//! makes `runtime.getitab` / assertion-helper arguments legible.
//!
//! ## The record, and where the two names come from
//!
//! An `itab` is `internal/abi.ITab`: `Inter *InterfaceType` at offset 0,
//! `Type *Type` at offset 8, `Hash uint32` at 16, then a variable `Fun`
//! method array at 24. Both `Inter` and `Type` are ordinary `_type`
//! descriptors in the `[types, etypes)` blob the harvest already decodes, so
//! the itab's name is built from the two descriptor ADDRESSES the record
//! points at — never from a spelled type name (the #1542 F1 lesson: a Go
//! reflection name is not unique, so binding by address is the only sound
//! key).
//!
//! ## Where the itabs are listed, across releases
//!
//! - **go1.18..go1.25:** moduledata carries `itablinks []*itab`, a slice of
//!   pointers (the `.itablink` section); each pointer is followed to its
//!   record. No per-record size is needed.
//! - **go1.26+:** the slice is gone. moduledata carries `itaboffset` /
//!   `itabsize` and the records sit consecutively in
//!   `[types + itaboffset, types + itaboffset + itabsize)`, enumerated by
//!   advancing each record's own `ITab.Size()` — `sizeof(ITab{})` for a
//!   "does not implement" placeholder (`Fun[0] == 0`), else
//!   `sizeof(ITab{}) + (len(Inter.Methods) - 1) * PtrSize`. This mirrors the
//!   typelinks → descriptor-region change `locate_type_meta_descriptors`
//!   already handles for `_type` records.
//!
//! Both are located by cross-validation inside the moduledata window: a
//! candidate list/region is accepted only when its sampled records decode as
//! itabs (both pointers land on valid descriptors, `Inter`'s is interface
//! kind). Rides the `gotypeglobals` gate — the judgment is identical to the
//! `_type` descriptor globals: trust the moduledata binding, name a data
//! symbol.

use std::collections::{BTreeMap, HashSet};

use super::*;
use crate::pass::{SymFact, SymKind};
use super::kuna_typeglobals::global_name_ok;

/// The linker-style itab symbol prefix (go1.21+ colon spelling, matching the
/// `type:` form the descriptor globals use).
const NAME_PREFIX: &str = "go:itab.";

/// `sizeof(internal/abi.ITab{})` on a 64/32-bit target: `Inter` + `Type`
/// (`2*word`), `Hash` (4), 4 bytes alignment padding before the word-aligned
/// `Fun[1]`, then one `Fun` word — `3*word + 8` (32 on 64-bit, 20 → padded to
/// 24 on 32-bit, where `word`=4 gives `3*4+8 = 20`; the runtime's `Sizeof`
/// rounds to the 4-byte word, so 20 is exact there).
pub(super) fn itab_header_size(word: u64) -> u64 {
    3 * word + 8
}

/// The method count of an interface descriptor: `InterfaceType.Methods` is the
/// trailing slice, `Type` base (`type_base`) + `PkgPath` name pointer (`word`)
/// in front of it, so its length word sits at `inter + type_base + 2*word`.
/// `None` if unreadable.
pub(super) fn interface_method_count(img: &GoImage, inter_vma: u64) -> Option<u64> {
    let methods_len = inter_vma.checked_add(img.type_base())?.checked_add(2 * img.word_size)?;
    let n = word(img, methods_len)?;
    // An interface a static itab names has at least one method (empty-interface
    // values use `eface`, not `itab`); bound a corrupt read.
    if n == 0 || n > 4096 {
        return None;
    }
    Some(n)
}

/// The in-image size of the itab record at `vma`, for the go1.26+ contiguous
/// walk. `Fun[0] == 0` is the "does not implement" placeholder (header only);
/// otherwise the `Fun` array holds one word per interface method.
pub(super) fn itab_size(img: &GoImage, vma: u64, inter_vma: u64) -> Option<u64> {
    let header = itab_header_size(img.word_size);
    let fun0 = word(img, vma.checked_add(2 * img.word_size + 8)?)?;
    if fun0 == 0 {
        return Some(header);
    }
    let n = interface_method_count(img, inter_vma)?;
    Some(header + (n - 1) * img.word_size)
}

/// Decode one itab record at `vma` into its (concrete, interface) type names,
/// or `None` if it is not a well-formed itab (either pointer missing, out of
/// the type blob, or `Inter` not an interface descriptor).
pub(super) fn read_itab(img: &GoImage, types: u64, etypes: u64, vma: u64) -> Option<(GoType, GoType)> {
    let inter_ptr = word(img, vma)?;
    let type_ptr = word(img, vma + img.word_size)?;
    let in_blob = |p: u64| {
        p >= types && p.checked_add(img.type_base()).is_some_and(|end| end <= etypes)
    };
    if !in_blob(inter_ptr) || !in_blob(type_ptr) {
        return None;
    }
    let inter = read_type(img, types, etypes, (inter_ptr - types) as u32)?;
    if inter.kind != KIND_INTERFACE {
        return None;
    }
    let concrete = read_type(img, types, etypes, (type_ptr - types) as u32)?;
    Some((concrete, inter))
}

/// The located itab records' VMAs, from whichever representation this release
/// uses. Tries the go≤1.25 `itablinks` pointer slice first, then the go1.26+
/// `[types + itaboffset, …]` contiguous region; `None` if neither validates.
fn locate_itabs(img: &GoImage, anchor: u64, types: u64, etypes: u64) -> Option<Vec<u64>> {
    locate_itablinks_slice(img, anchor, types, etypes)
        .or_else(|| locate_itab_region(img, anchor, types, etypes))
}

/// go1.18..go1.25: an `itablinks []*itab` slice in the moduledata window whose
/// sampled entries are pointers to valid itab records.
pub(super) fn locate_itablinks_slice(img: &GoImage, anchor: u64, types: u64, etypes: u64) -> Option<Vec<u64>> {
    for j in 0..MODULEDATA_WINDOW_WORDS {
        let Some((ptr, len)) = plausible_slice(img, anchor + j * img.word_size, 1_000_000) else {
            continue;
        };
        let n = len as usize;
        let step = (n / 16).max(1);
        let (mut ok, mut tried) = (0usize, 0usize);
        for k in (0..n).step_by(step).take(16) {
            tried += 1;
            let Some(it) = word(img, ptr + (k as u64) * img.word_size) else { break };
            if read_itab(img, types, etypes, it).is_some() {
                ok += 1;
            }
        }
        if tried >= 1 && ok == tried {
            let mut out = Vec::with_capacity(n.min(GOTYPES_MAX_TYPELINKS));
            for k in 0..n.min(GOTYPES_MAX_TYPELINKS) {
                if let Some(it) = word(img, ptr + (k as u64) * img.word_size) {
                    if read_itab(img, types, etypes, it).is_some() {
                        out.push(it);
                    }
                }
            }
            if !out.is_empty() {
                return Some(out);
            }
        }
    }
    None
}

/// go1.26+: `itaboffset` / `itabsize` — a pair of adjacent words in the
/// moduledata window where `[types + itaboffset, + itabsize)` walks as
/// consecutive itab records by `ITab.Size()`, every one decoding.
pub(super) fn locate_itab_region(img: &GoImage, anchor: u64, types: u64, etypes: u64) -> Option<Vec<u64>> {
    for i in 0..MODULEDATA_WINDOW_WORDS {
        let Some(off) = word(img, anchor + i * img.word_size) else { break };
        let Some(sz) = word(img, anchor + (i + 1) * img.word_size) else { break };
        // itaboffset is an offset from `types`; the region must be non-empty,
        // bounded, and land inside the mapped blob neighbourhood.
        if off == 0 || sz == 0 || sz > 0x100_0000 {
            continue;
        }
        let Some(start) = types.checked_add(off) else { continue };
        let Some(end) = start.checked_add(sz) else { continue };
        if !mapped(img, start) || !mapped(img, end - 1) {
            continue;
        }
        if let Some(out) = walk_itab_region(img, types, etypes, start, end) {
            if !out.is_empty() {
                return Some(out);
            }
        }
    }
    None
}

/// Walk `[start, end)` as consecutive itab records, advancing by each one's
/// `ITab.Size()`. Returns the VMAs only if the walk lands EXACTLY on `end`
/// with every record a valid itab — the cross-validation that rejects a wrong
/// `(itaboffset, itabsize)` candidate.
pub(super) fn walk_itab_region(img: &GoImage, types: u64, etypes: u64, start: u64, end: u64) -> Option<Vec<u64>> {
    let mut out = Vec::new();
    let mut p = start;
    // Bound the record count so a pathological region cannot loop unboundedly.
    for _ in 0..(GOTYPES_MAX_TYPELINKS * 2) {
        if p == end {
            return Some(out);
        }
        if p > end {
            return None;
        }
        let (_concrete, inter) = read_itab(img, types, etypes, p)?;
        let size = itab_size(img, p, inter.vma)?;
        if size == 0 {
            return None;
        }
        out.push(p);
        p = p.checked_add(size)?;
    }
    None
}

/// Produce one name-only [`SymFact`] per located itab: its VMA and the
/// linker-style `go:itab.<concrete>,<interface>` data symbol, built from the
/// two descriptor names. Deliberately a NAME, not a typed-data fact: an itab
/// address is used polymorphically (passed to `getitab`, compared, stored into
/// an interface word), so locking its global to a struct type demotes those
/// uses and HIDES the symbol a reference would otherwise render — exactly how
/// Go's own symtab names an itab (an untyped data symbol). Both names must be
/// printable ([`global_name_ok`]); deduplicated by VMA, in address order. An
/// itab VMA the image ALREADY names (`occupied` — an unstripped binary's own
/// symbol table) is skipped: the existing name is authoritative (and a
/// sub-package type's short record-name would otherwise collide with the
/// linker's full-path spelling), so the pass stays purely additive — it names
/// only the itabs stripping took.
pub(super) fn collect(
    img: &GoImage,
    meta: &TypeMeta,
    anchor: u64,
    occupied: &HashSet<u64>,
) -> Vec<SymFact> {
    let Some(itab_vmas) = locate_itabs(img, anchor, meta.types, meta.etypes) else {
        return Vec::new();
    };
    let mut by_vma: BTreeMap<u64, String> = BTreeMap::new();
    for vma in itab_vmas {
        if occupied.contains(&vma) {
            continue;
        }
        let Some((concrete, inter)) = read_itab(img, meta.types, meta.etypes, vma) else { continue };
        if !global_name_ok(&concrete.name) || !global_name_ok(&inter.name) {
            continue;
        }
        by_vma
            .entry(vma)
            .or_insert_with(|| format!("{NAME_PREFIX}{},{}", concrete.name, inter.name));
    }
    by_vma
        .into_iter()
        .map(|(addr, name)| SymFact { addr, name, kind: SymKind::Data })
        .collect()
}
