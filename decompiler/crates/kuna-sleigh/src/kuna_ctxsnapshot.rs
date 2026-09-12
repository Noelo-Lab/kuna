//! (kuna) A value-exact, `Send` copy of a [`ContextDatabase`]'s read surface.
//!
//! A `ContextDatabase` cannot be cloned faithfully: `FreeArray`'s `Clone`
//! deliberately zeroes the explicit-set mask, and `encode`/`decode` carries
//! neither the mask nor the default blob. What a second decoder actually needs
//! is narrower than either -- it only ever READS context -- so what is copied
//! here is exactly the read surface: the default blob plus the blob at every
//! split point, replayed by value into a fresh database.
//!
//! The masks are deliberately not carried. They steer future WRITES (a paint
//! stops at the first region where the variable was already explicitly set), and
//! a database restored from a snapshot is one nothing writes to: the only reader
//! of a restored database is a decoder built with `allow_context_set(false)` on a
//! language whose constructors carry no `globalset`
//! (`SleighBase::has_context_commits`).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;

use crate::globalcontext::ContextDatabase;

/// The value map of a [`ContextDatabase`] over one address space.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContextValueSnapshot {
    /// Words per context blob (`get_context_size`).
    pub words: usize,
    /// The default blob every other value is overlaid on.
    pub default_blob: Vec<u32>,
    /// `(offset, blob)` per split point, strictly ascending by offset.
    pub points: Vec<(u64, Vec<u32>)>,
}

/// Read `db`'s value map over `space`.
///
/// Walks the split structure through the public API alone: the default blob,
/// then `get_context_bounds` from offset 0, recording each region's start and
/// blob and jumping past its end.
pub fn snapshot_context(db: &dyn ContextDatabase, space: &Rc<AddrSpace>) -> ContextValueSnapshot {
    let default_blob = db.get_default_value().to_vec();
    let words = default_blob.len();
    let mut points: Vec<(u64, Vec<u32>)> = Vec::new();

    let highest = space.get_highest();
    let mut off: u64 = 0;
    loop {
        let addr = Address::new(Rc::clone(space), off);
        let (blob, first, last) = db.get_context_bounds(&addr);
        points.push((first, blob.to_vec()));
        if last >= highest {
            break;
        }
        // A region that does not advance past where the probe started would
        // loop forever; the split structure cannot produce one, so this is a
        // guard rather than a case.
        if last < off {
            break;
        }
        off = last + 1;
    }

    ContextValueSnapshot { words, default_blob, points }
}

/// Write `snap`'s value map into `db` over `space`.
///
/// The default blob is copied in place; then each point is painted, ascending,
/// with an invalid second address -- `get_region_for_set` with an invalid
/// `addr2` paints from the point to the end of the map, so a later point
/// overwrites the suffix of an earlier one and the final value map is the
/// snapshot's. This is what `ContextInternal::decode` does, without the XML.
pub fn restore_context(
    db: &mut dyn ContextDatabase,
    snap: &ContextValueSnapshot,
    space: &Rc<AddrSpace>,
) {
    let n = snap.default_blob.len().min(db.get_default_value().len());
    db.get_default_value_mut()[..n].copy_from_slice(&snap.default_blob[..n]);

    let to_end = Address::new_invalid();
    for (off, blob) in &snap.points {
        let addr = Address::new(Rc::clone(space), *off);
        for (w, value) in blob.iter().enumerate().take(n) {
            db.set_context_region(&addr, &to_end, w as i32, u32::MAX, *value); // cast: word index
        }
    }
}
