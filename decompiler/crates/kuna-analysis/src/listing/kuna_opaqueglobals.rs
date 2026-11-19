//! (kuna `opaquepredicate globals`) Which writable data the program may
//! write: the whole-program scan behind `kuna_decomp::kuna_opaqueglobals`.
//!
//! It is the walk `kuna_floatglobals` takes (every function from its entry,
//! registers and stack slots followed as constants), filing three more
//! things:
//!
//! - every store whose address is known, with its width;
//! - every address in writable data the code holds as a value rather than
//!   dereferences on the spot: moved into a register or a stack slot, stored
//!   to memory, or used in arithmetic whose result the walk does not know
//!   (`arr[i]` adds an index to `arr`);
//! - every pointer-sized word of loaded data that holds such an address
//!   (`int *p = &g;`), unless it is the address of a slot the run-time loader
//!   fills, which the relocation table itself spells.
//!
//! A held address may be stored through later, at an offset the scan cannot
//! see. It dirties the named object it lies in. Outside every named object
//! it dirties the [`ESCAPE_REACH`] bytes after it that no named object
//! claims, the reach `kuna_floatglobals` gives an escaped address. A named
//! object wider than a scalar that ends at the address, or starts within a
//! pointer width after it, is dirty as well: an end pointer walks down into
//! the first, and a biased base (`arr - 1`) indexes up into the second. The
//! run-time loader's slots (`LoadImage::kuna_runtime_written`) are dirty too.
//!
//! Read-only data counts as data nothing writes: its sections are listed
//! with the writable ones.
//!
//! Nothing is clean when a function was too long to walk to its end, or when
//! the image cannot list its loader slots.

use std::collections::BTreeSet;

use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::kuna_floatglobals::FloatScan;
use kuna_decomp::kuna_opaqueglobals::UnwrittenGlobals;
use kuna_sleigh::loadimage::section_flags;

use super::kuna_floatglobals::{walk, Evidence, Image};

/// How far past a held address outside every named object a store through
/// it may reach.
const ESCAPE_REACH: u64 = 256;
/// The widest object taken for a scalar: a pointer to its neighbour is not a
/// pointer one element off its end.
const SCALAR: u64 = 8;
/// How many named objects starting at or before an address are checked for
/// containing it.
const NESTED: usize = 8;

/// The stores and held addresses the walk has filed.
#[derive(Default)]
pub(super) struct Writes {
    stores: BTreeSet<(u64, u64)>,
    taken: BTreeSet<u64>,
    /// A function's walk stopped at its instruction limit.
    pub(super) incomplete: bool,
}

impl Writes {
    pub(super) fn store(&mut self, at: u64, size: u32) {
        self.stores.insert((at, at.saturating_add(u64::from(size))));
    }

    pub(super) fn take(&mut self, at: u64) {
        self.taken.insert(at);
    }
}

/// The writable data of the loaded program and the part of it that may be
/// written. Empty, so that nothing reads as constant, when the scan cannot
/// vouch for the whole program.
pub fn scan(arch: &Architecture, input: &FloatScan) -> UnwrittenGlobals {
    let loader = arch.translate().loader_rc();
    let Some(external) = loader.try_borrow().ok().and_then(|l| l.kuna_runtime_written()) else {
        return UnwrittenGlobals::default();
    };
    let objects = named_objects(arch);
    walk(arch, input, |image, evidence, ptr| decide(image, evidence, ptr, &objects, external))
        .flatten()
        .unwrap_or_default()
}

/// `[lo, hi)` of every sized data symbol in the default data space, sorted.
fn named_objects(arch: &Architecture) -> Vec<(u64, u64)> {
    let (Some(scope), Some(space)) = (arch.symboltab.get_global_scope(), arch.manage().get_default_data_space())
    else {
        return Vec::new();
    };
    let mut objects: Vec<(u64, u64)> = arch
        .symboltab
        .scope_space_symbol_specs(scope, space.get_index() as usize)
        .into_iter()
        .filter(|(_, ct, _, _)| ct.get_metatype() != type_metatype::TYPE_CODE && ct.get_size() > 0)
        .map(|(_, ct, addr, _)| (addr.get_offset(), addr.get_offset().saturating_add(ct.get_size() as u64)))
        .collect();
    objects.sort_unstable();
    objects
}

fn decide(
    image: &Image,
    evidence: &Evidence,
    ptr: u32,
    objects: &[(u64, u64)],
    mut external: Vec<(u64, u64)>,
) -> Option<UnwrittenGlobals> {
    let writes = &evidence.writes;
    if writes.incomplete || !matches!(ptr, 4 | 8) {
        return None;
    }
    let mut taken: BTreeSet<u64> = writes.taken.union(&evidence.escapes).copied().collect();
    external.sort_unstable();
    let width = ptr as usize;
    for section in image.sections.iter().filter(|s| s.flags & section_flags::CODE == 0) {
        let Some(bytes) = section.bytes.as_deref() else { continue };
        let skip = (section.lo.wrapping_neg() % u64::from(ptr)) as usize;
        for word in bytes.get(skip..).unwrap_or_default().chunks_exact(width) {
            let mut raw = [0u8; 8];
            let value = if image.big_endian {
                raw[8 - width..].copy_from_slice(word);
                u64::from_be_bytes(raw)
            } else {
                raw[..width].copy_from_slice(word);
                u64::from_le_bytes(raw)
            };
            let slot = external.partition_point(|&(lo, _)| lo <= value);
            let filled = slot > 0 && value < external[slot - 1].1;
            if image.is_writable_data(value) && !filled {
                taken.insert(value);
            }
        }
    }
    let mut dirty = external;
    dirty.extend(writes.stores.iter().copied());
    let reach = u64::from(ptr);
    for &at in &taken {
        let after = objects.partition_point(|&(start, _)| start <= at);
        let around: Vec<(u64, u64)> = objects[..after].iter().rev().take(NESTED).filter(|o| o.1 > at).copied().collect();
        if around.is_empty() {
            let floor = objects[..after].iter().rev().take(NESTED).map(|o| o.1).max().unwrap_or(0);
            let ceiling = objects.get(after).map_or(u64::MAX, |o| o.0);
            dirty.push((at.saturating_sub(reach).max(floor.min(at)), at.saturating_add(ESCAPE_REACH).min(ceiling)));
        }
        dirty.extend(around);
        let before = objects[..after].iter().rev().take(NESTED).filter(|o| o.1 == at);
        let ahead = objects[after..].iter().take_while(|o| o.0 <= at.saturating_add(reach));
        dirty.extend(before.chain(ahead).filter(|o| o.1 - o.0 > SCALAR).copied());
    }
    let data = image
        .sections
        .iter()
        .filter(|s| s.flags & section_flags::CODE == 0)
        .map(|s| (s.lo, s.hi, s.bytes.is_some()))
        .collect();
    Some(UnwrittenGlobals::new(data, dirty))
}
