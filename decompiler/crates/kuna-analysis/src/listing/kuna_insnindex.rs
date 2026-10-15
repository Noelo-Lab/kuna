//! Ordered instruction-address index grouped into 4 KiB address pages.
//!
//! The outer tree stores one row vector per populated page; each page's rows
//! remain sorted by the complete address. This keeps sparse address ranges
//! valid while amortizing tree-node overhead across the instructions in a page.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};

const PAGE_SHIFT: u32 = 12;
const PAGE_MASK: u64 = (1 << PAGE_SHIFT) - 1;

#[derive(Clone, Default)]
pub(super) struct InstructionIndex {
    pages: BTreeMap<u64, Vec<(u64, usize)>>,
    len: usize,
}

impl InstructionIndex {
    /// Add `address` with a lazily supplied row slot, or return the existing
    /// slot. The closure runs only for a new address, so callers can append the
    /// payload once without a separate lookup/insert race or orphaned row.
    pub fn insert_with(
        &mut self,
        address: u64,
        make_slot: impl FnOnce() -> usize,
    ) -> Option<usize> {
        let Self { pages, len } = self;
        match pages.entry(address >> PAGE_SHIFT) {
            Entry::Occupied(mut page) => {
                let rows = page.get_mut();
                if rows.last().is_some_and(|(key, _)| address > *key) {
                    let slot = make_slot();
                    rows.push((address, slot));
                    *len += 1;
                    return None;
                }
                match rows.binary_search_by_key(&address, |(key, _)| *key) {
                    Ok(i) => Some(rows[i].1),
                    Err(i) => {
                        let slot = make_slot();
                        rows.insert(i, (address, slot));
                        *len += 1;
                        None
                    }
                }
            }
            Entry::Vacant(page) => {
                let slot = make_slot();
                page.insert(vec![(address, slot)]);
                *len += 1;
                None
            }
        }
    }

    pub fn get(&self, address: u64) -> Option<usize> {
        let rows = self.pages.get(&(address >> PAGE_SHIFT))?;
        let i = rows.binary_search_by_key(&address, |(key, _)| *key).ok()?;
        Some(rows[i].1)
    }

    pub fn contains_key(&self, address: u64) -> bool {
        self.get(address).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn keys(&self) -> impl DoubleEndedIterator<Item = &u64> {
        self.pages
            .values()
            .flat_map(|rows| rows.iter().map(|(address, _)| address))
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&u64, &usize)> {
        self.pages
            .values()
            .flat_map(|rows| rows.iter().map(|(address, slot)| (address, slot)))
    }

    /// Return an address-ordered, double-ended view with the same bound
    /// semantics as `BTreeMap::range`. The page bounds restrict traversal to
    /// relevant buckets; binary searches trim only the boundary pages.
    pub fn range(
        &self,
        bounds: impl RangeBounds<u64>,
    ) -> impl DoubleEndedIterator<Item = (&u64, &usize)> {
        let start = owned_bound(bounds.start_bound());
        let end = owned_bound(bounds.end_bound());
        // `BTreeMap::range` validates bounds while constructing its range
        // only when the map has a root. An empty map therefore returns an
        // empty iterator even for an otherwise-invalid range.
        if !self.pages.is_empty() {
            assert_valid_range(&start, &end);
        }
        let page_bounds = page_span(&start, &end)
            .map_or((Bound::Included(0), Bound::Excluded(0)), |(first, last)| {
                (Bound::Included(first), Bound::Included(last))
            });
        self.pages
            .range(page_bounds)
            .flat_map(move |(&page, rows)| {
                let first = match start {
                    Bound::Included(address) if address >> PAGE_SHIFT == page => {
                        rows.partition_point(|(key, _)| *key < address)
                    }
                    Bound::Excluded(address) if address >> PAGE_SHIFT == page => {
                        rows.partition_point(|(key, _)| *key <= address)
                    }
                    _ => 0,
                };
                let last = match end {
                    Bound::Included(address) if address >> PAGE_SHIFT == page => {
                        rows.partition_point(|(key, _)| *key <= address)
                    }
                    Bound::Excluded(address) if address >> PAGE_SHIFT == page => {
                        rows.partition_point(|(key, _)| *key < address)
                    }
                    _ => rows.len(),
                };
                rows[first..last]
                    .iter()
                    .map(|(address, slot)| (address, slot))
            })
    }
}

fn owned_bound(bound: Bound<&u64>) -> Bound<u64> {
    match bound {
        Bound::Included(address) => Bound::Included(*address),
        Bound::Excluded(address) => Bound::Excluded(*address),
        Bound::Unbounded => Bound::Unbounded,
    }
}

fn assert_valid_range(start: &Bound<u64>, end: &Bound<u64>) {
    let invalid = match (start, end) {
        (Bound::Included(a), Bound::Included(b)) => a > b,
        (Bound::Included(a), Bound::Excluded(b)) => a > b,
        (Bound::Excluded(a), Bound::Included(b)) => a > b,
        (Bound::Excluded(a), Bound::Excluded(b)) => a >= b,
        _ => false,
    };
    assert!(!invalid, "range start is greater than range end");
}

fn page_span(start: &Bound<u64>, end: &Bound<u64>) -> Option<(u64, u64)> {
    let first = match start {
        Bound::Unbounded => 0,
        Bound::Included(address) => *address >> PAGE_SHIFT,
        Bound::Excluded(address) => {
            let page = *address >> PAGE_SHIFT;
            if *address & PAGE_MASK == PAGE_MASK {
                page + 1
            } else {
                page
            }
        }
    };
    let last = match end {
        Bound::Unbounded => u64::MAX >> PAGE_SHIFT,
        Bound::Included(address) => *address >> PAGE_SHIFT,
        Bound::Excluded(address) => {
            let page = *address >> PAGE_SHIFT;
            if *address & PAGE_MASK == 0 {
                page.checked_sub(1)?
            } else {
                page
            }
        }
    };
    (first <= last).then_some((first, last))
}
