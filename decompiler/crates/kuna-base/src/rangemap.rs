//! Port of `decompiler/cpp/rangemap.hh` (W1): an interval map container.
//!
//! A container for records occupying (possibly overlapping) intervals, i.e.
//! a map from a linear ordered domain to (multiple) records.  This is the
//! interval tree the symbol database (`database.hh` `EntryMap`) sits on.
//!
//! The main interval map is implemented as a *multiset* of disjoint
//! sub-ranges mapping to the record objects.  After deduping, the sub-ranges
//! form the common refinement of all the possibly overlapping record ranges.
//! A sub-range is duplicated for each distinct record that overlaps that
//! sub-range.  The sub-range multiset is updated with every insertion or
//! deletion of records, which may insert new or delete existing boundary
//! points separating the disjoint sub-ranges.
//!
//! ## Transcription notes (ADR 0001/0002)
//!
//! - The C++ `std::multiset<AddrRange>` (ordered by ending boundary `last`,
//!   then `subsort`) becomes a `BTreeMap` keyed by `(last, subsort, seq)`
//!   where `seq` is a gap-allocated tiebreak that permits duplicates AND
//!   places a new element before/after/between its `(last, subsort)`-equals
//!   exactly where the C++ oracle's `std::multiset::insert` (libstdc++
//!   `_M_get_insert_hint_equal_pos`, hinted and unhinted) would put it —
//!   that relative order is observable through `find()`/iteration and was
//!   verified element-for-element against the C++ (see the differential
//!   digest test).  Real elements use `0 < seq < u64::MAX`, so `seq == 0` /
//!   `seq == u64::MAX` serve as exact `lower_bound` / `upper_bound` search
//!   sentinels.  The mutable, non-ordering fields of `AddrRange` (`first`,
//!   `a`, `b`, `value`) live in the map's value half.  (A build against a
//!   different C++ standard library could legally order exact-duplicate
//!   keys differently; kuna's oracle is libstdc++.)
//! - The C++ `std::list<_recordtype>` (whose order is observable through
//!   `begin_list`, and which `insert` splices into (b, subsort) position)
//!   becomes a slab (`Vec<Option<...>>` + free list) threaded with an
//!   explicit doubly-linked list, so splice semantics and [`RecordIdx`]
//!   handle stability match C++ `list` iterators (an erased handle's slot
//!   may be reused, exactly as an erased C++ iterator must not be reused).
//! - C++ iterator pairs become [`SubRangeIter`] (a `DoubleEndedIterator`,
//!   since callers walk `--res.second`) yielding [`RecordIdx`]; the
//!   `find_begin`/`find_end` positional protocol is kept via
//!   [`RangeMap::find_begin`]/[`RangeMap::find_end`] returning opaque
//!   positions consumed by [`RangeMap::iter_between`].

use std::collections::btree_map;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

/// Contract of the C++ `linetype`: elements of the linear domain.
/// Must support comparison, equality, and +/- of small constants;
/// arithmetic wraps (the C++ relies on defined unsigned wraparound).
pub trait Line: Copy + Ord {
    /// `self + 1` (wrapping).
    fn plus1(self) -> Self;
    /// `self - 1` (wrapping).
    fn minus1(self) -> Self;
}

macro_rules! impl_line {
    ($($t:ty),*) => {$(
        impl Line for $t {
            #[inline]
            fn plus1(self) -> Self { self.wrapping_add(1) }
            #[inline]
            fn minus1(self) -> Self { self.wrapping_sub(1) }
        }
    )*};
}
impl_line!(u8, u16, u32, u64, usize, i8, i16, i32, i64);

/// Contract of the C++ `subsorttype`: how overlapping intervals are
/// sub-sorted.  `minimal()` is the C++ `subsorttype(false)` constructor,
/// `maximal()` is `subsorttype(true)`.
pub trait Subsort: Clone + Ord {
    /// `subsorttype(false)` — a minimal value.
    fn minimal() -> Self;
    /// `subsorttype(true)` — a maximal value.
    fn maximal() -> Self;
}

macro_rules! impl_subsort_uint {
    ($($t:ty),*) => {$(
        impl Subsort for $t {
            #[inline]
            fn minimal() -> Self { <$t>::MIN }
            #[inline]
            fn maximal() -> Self { <$t>::MAX }
        }
    )*};
}
impl_subsort_uint!(u8, u16, u32, u64, usize, i8, i16, i32, i64);

impl Subsort for bool {
    #[inline]
    fn minimal() -> Self {
        false
    }
    #[inline]
    fn maximal() -> Self {
        true
    }
}

impl Subsort for () {
    #[inline]
    fn minimal() -> Self {}
    #[inline]
    fn maximal() -> Self {}
}

/// Contract of the C++ `_recordtype`: the main object in the container.
pub trait RangeRecord: Sized {
    /// Integer data-type defining the linear domain (C++ `linetype`).
    type LineType: Line;
    /// The data-type used for subsorting (C++ `subsorttype`).
    type SubsortType: Subsort;
    /// The data-type containing initialization data (C++ `inittype`).
    type InitType;

    /// The C++ `recordtype(inittype,linetype,linetype)` constructor.
    fn create(data: Self::InitType, a: Self::LineType, b: Self::LineType) -> Self;
    /// Beginning of range (C++ `getFirst`).
    fn get_first(&self) -> Self::LineType;
    /// End of range, inclusive (C++ `getLast`).
    fn get_last(&self) -> Self::LineType;
    /// Retrieve the subsorttype object (C++ `getSubsort`).
    fn get_subsort(&self) -> Self::SubsortType;
}

/// Shorthand for a record's line type.
pub type LineOf<R> = <R as RangeRecord>::LineType;
/// Shorthand for a record's subsort type.
pub type SubsortOf<R> = <R as RangeRecord>::SubsortType;

/// Stable handle to a record in the container — the analogue of a C++
/// `std::list<_recordtype>::iterator`.  Invalidated by
/// [`RangeMap::erase`]/[`RangeMap::clear`] of that record (the slot may be
/// reused), exactly as the C++ iterator would be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordIdx(usize);

/// Ordering key of a sub-range in the multiset — the C++ `AddrRange`
/// comparison fields plus the duplicate-permitting insertion counter.
/// The derived lexicographic `Ord` transcribes `AddrRange::operator<`
/// (compare `last`, then `subsort`) with `seq` as the final tiebreak.
///
/// Publicly this is an opaque position token (see [`RangeMap::find_begin`]).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubRangeKey<L, S> {
    last: L,     // End of the disjoint sub-range
    subsort: S,  // How this should be sub-sorted
    seq: u64, // kuna: multiset duplicate tiebreak (gap-allocated, see hinted_seq)
}

/// The mutable (non-ordering) half of the C++ `AddrRange`.
#[derive(Debug, Clone)]
struct SubRange<L> {
    first: L,         // Start of the disjoint sub-range
    a: L,             // Start of full range occupied by the entire recordtype
    b: L,             // End of full range occupied by the entire recordtype
    value: RecordIdx, // "Iterator" pointing at the actual recordtype
}

/// A node of the record list (C++ `std::list<_recordtype>` element).
#[derive(Debug, Clone)]
struct RecordNode<R> {
    rec: R,
    prev: Option<usize>,
    next: Option<usize>,
}

/// A position among the sub-ranges: `Some(key)` for a real element, `None`
/// for the C++ `tree.end()`.
pub type Position<L, S> = Option<SubRangeKey<L, S>>;

// Internal shorthands for the multiset representation.
type KeyOf<R> = SubRangeKey<LineOf<R>, SubsortOf<R>>;
type TreeOf<R> = BTreeMap<KeyOf<R>, SubRange<LineOf<R>>>;
type TreeRangeOf<'a, R> = btree_map::Range<'a, KeyOf<R>, SubRange<LineOf<R>>>;

/// Gap between freshly spaced seqs; a new element can be placed between two
/// equals 32 times before its group is renumbered.
const SEQ_GAP: u64 = 1 << 32;
/// First seq assigned in an empty equal range (leaves room below for
/// lower-bound inserts and above for upper-bound inserts).
const SEQ_START: u64 = 1 << 63;

/// An interval map container (C++ `rangemap<_recordtype>`).
pub struct RangeMap<R: RangeRecord> {
    /// The underlying multiset of sub-ranges.
    tree: TreeOf<R>,
    /// Storage for the actual record objects (slab + linked list order).
    records: Vec<Option<RecordNode<R>>>,
    free: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

/// Iterator over sub-ranges, dereferencing to the record handle.  The same
/// record may be visited multiple times, depending on how much it overlaps
/// other records.  Sub-ranges are sorted in linear order (by ending
/// boundary), then by subsort (C++ `PartIterator`).
pub struct SubRangeIter<'a, R: RangeRecord> {
    inner: Option<TreeRangeOf<'a, R>>,
}

impl<R: RangeRecord> Iterator for SubRangeIter<'_, R> {
    type Item = RecordIdx;
    fn next(&mut self) -> Option<RecordIdx> {
        self.inner.as_mut()?.next().map(|(_, d)| d.value)
    }
}

impl<R: RangeRecord> DoubleEndedIterator for SubRangeIter<'_, R> {
    fn next_back(&mut self) -> Option<RecordIdx> {
        self.inner.as_mut()?.next_back().map(|(_, d)| d.value)
    }
}

/// Iterator over records in list order (C++ `begin_list()`/`end_list()`).
pub struct RecordListIter<'a, R: RangeRecord> {
    map: &'a RangeMap<R>,
    cur: Option<usize>,
}

impl<'a, R: RangeRecord> Iterator for RecordListIter<'a, R> {
    type Item = (RecordIdx, &'a R);
    fn next(&mut self) -> Option<(RecordIdx, &'a R)> {
        let i = self.cur?;
        let node = self.map.records[i]
            .as_ref()
            .expect("rangemap: record list links to a freed slot");
        self.cur = node.next;
        Some((RecordIdx(i), &node.rec))
    }
}

impl<R: RangeRecord> Default for RangeMap<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: RangeRecord> RangeMap<R> {
    /// Create an empty container.
    pub fn new() -> Self {
        RangeMap {
            tree: BTreeMap::new(),
            records: Vec::new(),
            free: Vec::new(),
            head: None,
            tail: None,
        }
    }

    /// Return `true` if the container is empty (C++ `empty`, which checks
    /// the record list).
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }

    /// Clear all records from the container (C++ `clear`).
    pub fn clear(&mut self) {
        self.tree.clear();
        self.records.clear();
        self.free.clear();
        self.head = None;
        self.tail = None;
    }

    /// Access a record by handle (C++ `*iter` on a list iterator).
    /// Panics on a stale/erased handle (internal invariant, ADR 0004).
    pub fn record(&self, idx: RecordIdx) -> &R {
        &self.records[idx.0]
            .as_ref()
            .expect("rangemap: stale RecordIdx")
            .rec
    }

    /// Mutable access to a record.  The caller must not alter the values
    /// returned by `get_first`/`get_last`/`get_subsort` while the record is
    /// in the container (same unstated invariant as mutating through a C++
    /// list iterator).
    pub fn record_mut(&mut self, idx: RecordIdx) -> &mut R {
        &mut self.records[idx.0]
            .as_mut()
            .expect("rangemap: stale RecordIdx")
            .rec
    }

    /// Iterate records in list order (C++ `begin_list()`/`end_list()`).
    pub fn records(&self) -> RecordListIter<'_, R> {
        RecordListIter { map: self, cur: self.head }
    }

    /// First record in list order (C++ `begin_list()`), or `None`.
    pub fn list_begin(&self) -> Option<RecordIdx> {
        self.head.map(RecordIdx)
    }

    /// Successor in list order (C++ `++` on a list iterator), or `None` at
    /// the end of the list.
    pub fn list_next(&self, idx: RecordIdx) -> Option<RecordIdx> {
        self.records[idx.0]
            .as_ref()
            .expect("rangemap: stale RecordIdx")
            .next
            .map(RecordIdx)
    }

    /// Iterate over all sub-ranges (C++ `begin()`/`end()`).
    pub fn iter(&self) -> SubRangeIter<'_, R> {
        SubRangeIter { inner: Some(self.tree.range(..)) }
    }

    // ---- internal helpers -------------------------------------------------

    /// `AddrRange(l)` search key: lower_bound sentinel `(l, minimal, 0)`.
    fn min_key(l: LineOf<R>) -> SubRangeKey<LineOf<R>, SubsortOf<R>> {
        SubRangeKey { last: l, subsort: SubsortOf::<R>::minimal(), seq: 0 }
    }

    /// `AddrRange(l,s)` upper_bound sentinel `(l, s, u64::MAX)`: every real
    /// element comparing equal on `(last, subsort)` sorts strictly below it.
    fn max_key(l: LineOf<R>, s: SubsortOf<R>) -> SubRangeKey<LineOf<R>, SubsortOf<R>> {
        SubRangeKey { last: l, subsort: s, seq: u64::MAX }
    }

    /// First element with key >= `k` (C++ `tree.lower_bound`).
    fn lower_bound_key(
        &self,
        k: SubRangeKey<LineOf<R>, SubsortOf<R>>,
    ) -> Option<SubRangeKey<LineOf<R>, SubsortOf<R>>> {
        self.tree
            .range((Included(k), Unbounded))
            .next()
            .map(|(key, _)| key.clone())
    }

    /// Successor element (C++ `++iter`).
    fn next_key(
        &self,
        k: &SubRangeKey<LineOf<R>, SubsortOf<R>>,
    ) -> Option<SubRangeKey<LineOf<R>, SubsortOf<R>>> {
        self.tree
            .range((Excluded(k.clone()), Unbounded))
            .next()
            .map(|(key, _)| key.clone())
    }

    // ---- multiset insertion-position machinery ----------------------------
    //
    // The C++ container is a std::multiset, and the relative order of
    // elements with EQUAL (last, subsort) keys is observable (find() visits
    // them in tree order).  That order is determined by where
    // `tree.insert(...)` / `tree.insert(hint, ...)` place a new element
    // within its equal range, which for the oracle binary is libstdc++'s
    // `_M_get_insert_hint_equal_pos` logic.  We reproduce it exactly: the
    // `seq` component of the key is allocated from gaps so an element can be
    // placed before/after/between its equals.

    /// Smallest seq of the `(last, subsort)` equal range, if any element.
    fn group_min(&self, last: LineOf<R>, subsort: &SubsortOf<R>) -> Option<u64> {
        self.tree
            .range((
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: 0 }),
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: u64::MAX }),
            ))
            .next()
            .map(|(k, _)| k.seq)
    }

    /// Largest seq of the `(last, subsort)` equal range, if any element.
    fn group_max(&self, last: LineOf<R>, subsort: &SubsortOf<R>) -> Option<u64> {
        self.tree
            .range((
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: 0 }),
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: u64::MAX }),
            ))
            .next_back()
            .map(|(k, _)| k.seq)
    }

    /// Re-space the seqs of an equal range (rare; only when a gap is
    /// exhausted).  Returns old-seq -> new-seq.  NB: this rewrites the keys
    /// of that equal range, so any outstanding [`Position`] pointing into it
    /// goes stale — acceptable because exhausting a 2^32 gap requires a
    /// pathological insertion pattern no real workload reaches.
    fn renumber_group(&mut self, last: LineOf<R>, subsort: &SubsortOf<R>) -> BTreeMap<u64, u64> {
        let old: Vec<(KeyOf<R>, SubRange<LineOf<R>>)> = self
            .tree
            .range((
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: 0 }),
                Included(SubRangeKey { last, subsort: subsort.clone(), seq: u64::MAX }),
            ))
            .map(|(k, d)| (k.clone(), d.clone()))
            .collect();
        let mut map = BTreeMap::new();
        for (i, (k, d)) in old.into_iter().enumerate() {
            self.tree.remove(&k);
            let new_seq = SEQ_START + (i as u64) * SEQ_GAP;
            map.insert(k.seq, new_seq);
            self.tree
                .insert(SubRangeKey { last: k.last, subsort: k.subsort, seq: new_seq }, d);
        }
        map
    }

    /// Allocate a seq placing a new element after the whole equal range
    /// (`insert_equal` without a useful hint — the upper bound).
    fn seq_append(&mut self, last: LineOf<R>, subsort: &SubsortOf<R>) -> u64 {
        match self.group_max(last, subsort) {
            None => SEQ_START,
            Some(m) => {
                if m < u64::MAX - SEQ_GAP {
                    m + SEQ_GAP
                } else {
                    let map = self.renumber_group(last, subsort);
                    map[&m] + SEQ_GAP
                }
            }
        }
    }

    /// Allocate a seq placing a new element before the whole equal range
    /// (`_M_get_insert_equal_lower_pos` — the lower bound).
    fn seq_prepend(&mut self, last: LineOf<R>, subsort: &SubsortOf<R>) -> u64 {
        match self.group_min(last, subsort) {
            None => SEQ_START,
            Some(m) => Self::seq_below(m).unwrap_or_else(|| {
                let map = self.renumber_group(last, subsort);
                Self::seq_below(map[&m]).expect("rangemap: renumber left no gap")
            }),
        }
    }

    /// A seq strictly between 0 (exclusive) and `hi`, or `None` if no gap.
    fn seq_below(hi: u64) -> Option<u64> {
        if hi > SEQ_GAP {
            Some(hi - SEQ_GAP)
        } else if hi >= 2 {
            Some(hi / 2)
        } else {
            None
        }
    }

    /// Allocate a seq strictly between two group members' seqs.
    fn seq_mid(&mut self, last: LineOf<R>, subsort: &SubsortOf<R>, lo: u64, hi: u64) -> u64 {
        debug_assert!(lo < hi);
        let mid = lo + (hi - lo) / 2;
        if mid > lo {
            mid
        } else {
            let map = self.renumber_group(last, subsort);
            let (nlo, nhi) = (map[&lo], map[&hi]);
            nlo + (nhi - nlo) / 2
        }
    }

    /// Position a new `(last, subsort)` element exactly where
    /// `tree.insert(hint, AddrRange(last, subsort))` would land in the C++
    /// (libstdc++ `_M_get_insert_hint_equal_pos`), returning its seq.
    /// `hint == None` is `tree.end()`.
    fn hinted_seq(
        &mut self,
        hint: Option<&SubRangeKey<LineOf<R>, SubsortOf<R>>>,
        last: LineOf<R>,
        subsort: &SubsortOf<R>,
    ) -> u64 {
        use std::cmp::Ordering;
        // Compare the new key against an existing element on the C++
        // AddrRange comparator fields (last, then subsort) only.
        let cmp_k_to = |key: &SubRangeKey<LineOf<R>, SubsortOf<R>>| -> Ordering {
            match last.cmp(&key.last) {
                Ordering::Equal => subsort.cmp(&key.subsort),
                o => o,
            }
        };
        let in_group = |key: &SubRangeKey<LineOf<R>, SubsortOf<R>>| {
            key.last == last && key.subsort == *subsort
        };
        match hint {
            // hint == end(): insert after rightmost if k >= rightmost, else
            // generic insert_equal — both append at the equal range's end.
            None => self.seq_append(last, subsort),
            Some(h) => {
                if cmp_k_to(h) != Ordering::Greater {
                    // k <= *hint: "first, try before"
                    let before = self
                        .tree
                        .range((Unbounded, Excluded(h.clone())))
                        .next_back()
                        .map(|(k, _)| k.clone());
                    match before {
                        // hint == leftmost: insert before it
                        None => self.seq_prepend(last, subsort),
                        Some(bk) => {
                            if cmp_k_to(&bk) != Ordering::Less {
                                // *before <= k <= *hint: insert between them.
                                // before/hint are tree-adjacent, so the slot
                                // is bounded by whichever of them is in k's
                                // equal range.
                                match (in_group(&bk), in_group(h)) {
                                    (true, true) => self.seq_mid(last, subsort, bk.seq, h.seq),
                                    (true, false) => self.seq_append(last, subsort),
                                    (false, true) => {
                                        Self::seq_below(h.seq).unwrap_or_else(|| {
                                            let map = self.renumber_group(last, subsort);
                                            Self::seq_below(map[&h.seq])
                                                .expect("rangemap: renumber left no gap")
                                        })
                                    }
                                    (false, false) => SEQ_START,
                                }
                            } else {
                                // fallback: _M_get_insert_equal_pos (upper bound)
                                self.seq_append(last, subsort)
                            }
                        }
                    }
                } else {
                    // *hint < k: "... then try after"
                    let after = self
                        .tree
                        .range((Excluded(h.clone()), Unbounded))
                        .next()
                        .map(|(k, _)| k.clone());
                    match after {
                        // hint == rightmost: insert after it
                        None => self.seq_append(last, subsort),
                        Some(ak) => {
                            if cmp_k_to(&ak) != Ordering::Greater {
                                // *hint < k <= *after: insert between them;
                                // only `after` can be in k's equal range.
                                if in_group(&ak) {
                                    Self::seq_below(ak.seq).unwrap_or_else(|| {
                                        let map = self.renumber_group(last, subsort);
                                        Self::seq_below(map[&ak.seq])
                                            .expect("rangemap: renumber left no gap")
                                    })
                                } else {
                                    SEQ_START
                                }
                            } else {
                                // fallback: _M_get_insert_equal_lower_pos (lower bound)
                                self.seq_prepend(last, subsort)
                            }
                        }
                    }
                }
            }
        }
    }

    /// `tree.insert(hint, AddrRange(...))` — hinted multiset insert with
    /// libstdc++ placement semantics.
    fn tree_insert_hinted(
        &mut self,
        hint: Option<&SubRangeKey<LineOf<R>, SubsortOf<R>>>,
        last: LineOf<R>,
        subsort: SubsortOf<R>,
        data: SubRange<LineOf<R>>,
    ) {
        let seq = self.hinted_seq(hint, last, &subsort);
        let prev = self.tree.insert(SubRangeKey { last, subsort, seq }, data);
        debug_assert!(prev.is_none(), "rangemap: multiset seq collision");
    }

    /// Unhinted `tree.insert(AddrRange(...))` — upper bound of equal range.
    fn tree_insert(&mut self, last: LineOf<R>, subsort: SubsortOf<R>, data: SubRange<LineOf<R>>) {
        let seq = self.seq_append(last, &subsort);
        let prev = self.tree.insert(SubRangeKey { last, subsort, seq }, data);
        debug_assert!(prev.is_none(), "rangemap: multiset seq collision");
    }

    fn alloc_record(&mut self, rec: R) -> RecordIdx {
        let node = RecordNode { rec, prev: None, next: None };
        match self.free.pop() {
            Some(i) => {
                debug_assert!(self.records[i].is_none());
                self.records[i] = Some(node);
                RecordIdx(i)
            }
            None => {
                self.records.push(Some(node));
                RecordIdx(self.records.len() - 1)
            }
        }
    }

    /// Splice `idx` into the record list before `target` (`None` = at the
    /// end) — the C++ `record.splice(pos, record, liter)`.
    fn list_insert_before(&mut self, target: Option<RecordIdx>, idx: RecordIdx) {
        let i = idx.0;
        match target {
            None => {
                let old_tail = self.tail;
                {
                    let node = self.records[i].as_mut().expect("rangemap: stale RecordIdx");
                    node.prev = old_tail;
                    node.next = None;
                }
                match old_tail {
                    Some(t) => {
                        self.records[t]
                            .as_mut()
                            .expect("rangemap: broken record list")
                            .next = Some(i)
                    }
                    None => self.head = Some(i),
                }
                self.tail = Some(i);
            }
            Some(t) => {
                let tprev = self.records[t.0]
                    .as_ref()
                    .expect("rangemap: stale splice target")
                    .prev;
                {
                    let node = self.records[i].as_mut().expect("rangemap: stale RecordIdx");
                    node.prev = tprev;
                    node.next = Some(t.0);
                }
                self.records[t.0]
                    .as_mut()
                    .expect("rangemap: stale splice target")
                    .prev = Some(i);
                match tprev {
                    Some(p) => {
                        self.records[p]
                            .as_mut()
                            .expect("rangemap: broken record list")
                            .next = Some(i)
                    }
                    None => self.head = Some(i),
                }
            }
        }
    }

    /// Unlink and free a record slot (C++ `record.erase(v)`).
    fn free_record(&mut self, idx: RecordIdx) {
        let node = self.records[idx.0]
            .take()
            .expect("rangemap: erasing a stale RecordIdx");
        match node.prev {
            Some(p) => {
                self.records[p]
                    .as_mut()
                    .expect("rangemap: broken record list")
                    .next = node.next
            }
            None => self.head = node.next,
        }
        match node.next {
            Some(n) => {
                self.records[n]
                    .as_mut()
                    .expect("rangemap: broken record list")
                    .prev = node.prev
            }
            None => self.tail = node.prev,
        }
        self.free.push(idx.0);
    }

    // ---- zip / unzip ------------------------------------------------------

    /// Remove the given partition boundary (C++ `zip`).
    ///
    /// All sub-ranges that end with the given boundary point are deleted, and
    /// all sub-ranges that begin with the given boundary point (+1) are
    /// extended to cover the deleted sub-range.
    /// `start` is the first sub-range that ends with the boundary point
    /// (the C++ callers pass `tree.lower_bound(AddrRange(i))`).
    fn zip(&mut self, i: LineOf<R>, start: SubRangeKey<LineOf<R>, SubsortOf<R>>) {
        let f = self
            .tree
            .get(&start)
            .expect("rangemap: zip start position vanished")
            .first;
        let mut cur = Some(start);
        // (the C++ loop has no end-of-tree guard; a None cursor just stops)
        while let Some(k) = cur.take() {
            if k.last != i {
                cur = Some(k);
                break;
            }
            let nxt = self.next_key(&k);
            self.tree.remove(&k);
            cur = nxt;
        }
        let i1 = i.plus1();
        while let Some(k) = cur.take() {
            let d = self
                .tree
                .get_mut(&k)
                .expect("rangemap: zip cursor vanished");
            if d.first != i1 {
                break;
            }
            d.first = f;
            cur = self.next_key(&k);
        }
    }

    /// Insert the given partition boundary (C++ `unzip`).
    ///
    /// All sub-ranges that contain the boundary point are split into a
    /// sub-range that ends at the boundary point and a sub-range that begins
    /// with the boundary point (+1).
    /// `start` is the first sub-range containing the boundary point.
    fn unzip(&mut self, i: LineOf<R>, start: SubRangeKey<LineOf<R>, SubsortOf<R>>) {
        if start.last == i {
            return; // Can't split size 1 (i.e. split already present)
        }
        // C++ keeps `hint` fixed at the entry iterator for every insert
        let hint = start.clone();
        let plus1 = i.plus1();
        let mut cur = Some(start);
        while let Some(k) = cur.take() {
            let d = self
                .tree
                .get_mut(&k)
                .expect("rangemap: unzip cursor vanished");
            if d.first > i {
                // C++ condition `(*iter).first <= i` failed
                break;
            }
            let f = d.first;
            d.first = plus1;
            let copy = SubRange { first: f, a: d.a, b: d.b, value: d.value };
            let subsort = k.subsort.clone();
            // new sub-range [f, i] sorts strictly before the cursor (last = i
            // < k.last), so forward iteration never revisits it
            self.tree_insert_hinted(Some(&hint), i, subsort, copy);
            cur = self.next_key(&k);
        }
    }

    // ---- main operations --------------------------------------------------

    /// Insert a new record into the container (C++ `insert`).
    ///
    /// `data` is the extra initialization for the record, `a` is the start
    /// of the range occupied by the new record and `b` the (inclusive) end.
    /// Returns the handle to the new record.
    pub fn insert(&mut self, data: R::InitType, a: LineOf<R>, b: LineOf<R>) -> RecordIdx {
        let mut f = a;
        let low0 = self.lower_bound_key(Self::min_key(f));
        if let Some(lk) = &low0 {
            // Check if left boundary refines existing partition
            if self.tree[lk].first < f {
                self.unzip(f.minus1(), lk.clone()); // If so do the refinement
            }
        }

        // Splice the new record before the sub-range at
        // tree.lower_bound(AddrRange(b, subsort)).
        let rec = R::create(data, a, b);
        let subsort = rec.get_subsort();
        let idx = self.alloc_record(rec);
        let spot_value = self
            .tree
            .range((
                Included(SubRangeKey { last: b, subsort: subsort.clone(), seq: 0 }),
                Unbounded,
            ))
            .next()
            .map(|(_, d)| d.value);
        self.list_insert_before(spot_value, idx);

        let mut low = low0;
        while let Some(lk) = low.take() {
            let (lfirst, llast) = {
                let d = &self.tree[&lk];
                (d.first, lk.last)
            };
            if lfirst > b {
                // C++ condition `(*low).first <= b` failed
                break;
            }
            if f <= llast {
                // Do we overlap at all
                if f < lfirst {
                    // Assume the hint makes this insert an O(1) op
                    self.tree_insert_hinted(
                        Some(&lk),
                        lfirst.minus1(),
                        subsort.clone(),
                        SubRange { first: f, a, b, value: idx },
                    );
                    f = lfirst;
                }
                if llast <= b {
                    // Insert as much of interval as we can
                    self.tree_insert_hinted(
                        Some(&lk),
                        llast,
                        subsort.clone(),
                        SubRange { first: f, a, b, value: idx },
                    );
                    if llast == b {
                        // Did we manage to insert it all
                        // (NB: like the C++, this break still falls into the
                        // trailing `if f <= b` below, inserting a DUPLICATE
                        // [f,b] sub-range for the record.  Verified against
                        // the C++ oracle: a record whose right boundary lands
                        // exactly on an existing cell boundary is visited
                        // twice on that cell by find()/iteration.)
                        break;
                    }
                    f = llast.plus1();
                } else if b < llast {
                    // We can insert everything left, but must refine
                    self.unzip(b, lk);
                    break;
                }
            }
            low = self.next_key(&lk);
        }
        if f <= b {
            self.tree_insert(b, subsort, SubRange { first: f, a, b, value: idx });
        }
        idx
    }

    /// Erase a given record from the container (C++ `erase`).
    pub fn erase(&mut self, v: RecordIdx) {
        let (a, b) = {
            let r = self.record(v);
            (r.get_first(), r.get_last())
        };
        let mut leftsew = true;
        let mut rightsew = true;
        let mut rightoverlap = false;
        let mut leftoverlap = false;
        let mut low = self.lower_bound_key(Self::min_key(a));

        let aminus1 = a.minus1();
        // scan backward over sub-ranges ending exactly at a-1
        let mut uplow = low.clone();
        loop {
            let prev = match &uplow {
                Some(k) => self
                    .tree
                    .range((Unbounded, Excluded(k.clone())))
                    .next_back(),
                None => self.tree.iter().next_back(),
            };
            let Some((pk, pd)) = prev else {
                break; // uplow == tree.begin()
            };
            if pk.last != aminus1 {
                break;
            }
            if pd.b == aminus1 {
                leftsew = false; // Still a split between a-1 and a
                break;
            }
            uplow = Some(pk.clone());
        }

        loop {
            // The record's own sub-ranges guarantee low != end on entry
            // (dereferencing end here is C++ UB; we panic per ADR 0004).
            let k = low.expect("rangemap: erased record has no sub-range in tree");
            let nxt = self.next_key(&k);
            let d = self
                .tree
                .get(&k)
                .expect("rangemap: erase cursor vanished");
            if d.value == v {
                self.tree.remove(&k);
            } else {
                match d.a.cmp(&a) {
                    std::cmp::Ordering::Less => leftoverlap = true, // a splits somebody else
                    std::cmp::Ordering::Equal => leftsew = false, // Somebody else splits at a (in addition to v)
                    std::cmp::Ordering::Greater => {}
                }
                match b.cmp(&d.b) {
                    std::cmp::Ordering::Less => rightoverlap = true, // b splits somebody else
                    std::cmp::Ordering::Equal => rightsew = false, // Somebody else splits at b (in addition to v)
                    std::cmp::Ordering::Greater => {}
                }
            }
            low = nxt;
            match &low {
                Some(k2) if self.tree[k2].first <= b => {}
                _ => break,
            }
        }
        if let Some(k2) = &low {
            if self.tree[k2].a.minus1() == b {
                rightsew = false;
            }
        }
        if leftsew && leftoverlap {
            let start = self
                .lower_bound_key(Self::min_key(aminus1))
                .expect("rangemap: zip(a-1) start missing");
            self.zip(aminus1, start);
        }
        if rightsew && rightoverlap {
            let start = self
                .lower_bound_key(Self::min_key(b))
                .expect("rangemap: zip(b) start missing");
            self.zip(b, start);
        }
        self.free_record(v);
    }

    // ---- queries ----------------------------------------------------------

    /// Find sub-ranges intersecting the given boundary point (C++
    /// `find(linetype)`), i.e. every duplicate of the refinement cell
    /// containing `point`, in subsort order.
    pub fn find(&self, point: LineOf<R>) -> SubRangeIter<'_, R> {
        let first = self
            .tree
            .range((Included(Self::min_key(point)), Unbounded))
            .next();
        match first {
            // Check for no intersection
            None => SubRangeIter { inner: None },
            Some((_, d)) if point < d.first => SubRangeIter { inner: None },
            Some((k, _)) => {
                let lo = Included(k.clone());
                let hi = Included(Self::max_key(k.last, SubsortOf::<R>::maximal()));
                SubRangeIter { inner: Some(self.tree.range((lo, hi))) }
            }
        }
    }

    /// Find sub-ranges intersecting the given boundary point, bounded
    /// between the given subsorts (C++ `find(linetype, subsorttype, subsorttype)`).
    pub fn find_subsorts(
        &self,
        point: LineOf<R>,
        sub1: SubsortOf<R>,
        sub2: SubsortOf<R>,
    ) -> SubRangeIter<'_, R> {
        let first = self
            .tree
            .range((
                Included(SubRangeKey { last: point, subsort: sub1, seq: 0 }),
                Unbounded,
            ))
            .next();
        match first {
            None => SubRangeIter { inner: None },
            Some((_, d)) if point < d.first => SubRangeIter { inner: None },
            Some((k, _)) => {
                let lo = Included(k.clone());
                let hi = Included(Self::max_key(k.last, sub2));
                if Self::bound_inverted(&lo, &hi) {
                    return SubRangeIter { inner: None };
                }
                SubRangeIter { inner: Some(self.tree.range((lo, hi))) }
            }
        }
    }

    /// Find the beginning of sub-ranges that contain the given boundary point
    /// (C++ `find_begin`: `tree.lower_bound(AddrRange(point))`).
    pub fn find_begin(&self, point: LineOf<R>) -> Position<LineOf<R>, SubsortOf<R>> {
        self.lower_bound_key(Self::min_key(point))
    }

    /// Find the ending of sub-ranges that contain the given boundary point
    /// (C++ `find_end`): the first sub-range after those containing `point`.
    pub fn find_end(&self, point: LineOf<R>) -> Position<LineOf<R>, SubsortOf<R>> {
        let upper = self
            .tree
            .range((
                Excluded(Self::max_key(point, SubsortOf::<R>::maximal())),
                Unbounded,
            ))
            .next();
        let (k, d) = upper?;
        if point < d.first {
            return Some(k.clone());
        }
        // If we reach here, (*iter).last is bigger than point (as per
        // upper_bound) but point >= (*iter).first, i.e. point is contained in
        // the sub-range.  So we have to do one more search for the first
        // sub-range after the containing sub-range.
        self.tree
            .range((
                Excluded(Self::max_key(k.last, SubsortOf::<R>::maximal())),
                Unbounded,
            ))
            .next()
            .map(|(k2, _)| k2.clone())
    }

    /// Find the first record overlapping the given interval (C++
    /// `find_overlap`); `None` plays the role of `tree.end()`.
    pub fn find_overlap(&self, point: LineOf<R>, end: LineOf<R>) -> Option<RecordIdx> {
        // First range where right boundary is equal to or past point
        let (_, d) = self
            .tree
            .range((Included(Self::min_key(point)), Unbounded))
            .next()?;
        if d.first <= end {
            Some(d.value)
        } else {
            None
        }
    }

    /// Iterate the sub-ranges in `[begin, end)` — the C++
    /// `while (iter != enditer) ... ++iter` protocol over positions from
    /// [`RangeMap::find_begin`]/[`RangeMap::find_end`].
    pub fn iter_between(
        &self,
        begin: &Position<LineOf<R>, SubsortOf<R>>,
        end: &Position<LineOf<R>, SubsortOf<R>>,
    ) -> SubRangeIter<'_, R> {
        let lo = match begin {
            Some(k) => Included(k.clone()),
            None => return SubRangeIter { inner: None },
        };
        let hi = match end {
            Some(k) => Excluded(k.clone()),
            None => Unbounded,
        };
        if Self::bound_inverted(&lo, &hi) {
            return SubRangeIter { inner: None };
        }
        SubRangeIter { inner: Some(self.tree.range((lo, hi))) }
    }

    /// `true` when a (lo, hi) bound pair would panic `BTreeMap::range`
    /// (an inverted/empty range — corresponds to C++ iterator-pair UB; we
    /// treat it as an empty range).
    fn bound_inverted(
        lo: &std::ops::Bound<SubRangeKey<LineOf<R>, SubsortOf<R>>>,
        hi: &std::ops::Bound<SubRangeKey<LineOf<R>, SubsortOf<R>>>,
    ) -> bool {
        match (lo, hi) {
            (Included(l), Excluded(h)) => h <= l,
            (Included(l), Included(h)) => h < l,
            (Excluded(l), Excluded(h)) => h <= l,
            (Excluded(l), Included(h)) => h < l,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test analogue of database.hh's SymbolEntry: u64 line domain, u64
    /// subsort, and a tag identifying the record.  All expected outputs
    /// below were pinned by running the identical scenario against the C++
    /// rangemap.hh (g++ -std=c++11).
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestRecord {
        a: u64,
        b: u64,
        subsort: u64,
        tag: u32,
    }

    impl RangeRecord for TestRecord {
        type LineType = u64;
        type SubsortType = u64;
        type InitType = (u32, u64); // (tag, subsort)

        fn create(data: (u32, u64), a: u64, b: u64) -> Self {
            TestRecord { a, b, subsort: data.1, tag: data.0 }
        }
        fn get_first(&self) -> u64 {
            self.a
        }
        fn get_last(&self) -> u64 {
            self.b
        }
        fn get_subsort(&self) -> u64 {
            self.subsort
        }
    }

    type RM = RangeMap<TestRecord>;

    fn tags(rm: &RM, it: SubRangeIter<'_, TestRecord>) -> Vec<u32> {
        it.map(|idx| rm.record(idx).tag).collect()
    }

    #[test]
    fn test_rangemap_empty_container() {
        let rm = RM::new();
        assert!(rm.empty());
        assert_eq!(rm.iter().count(), 0);
        assert_eq!(rm.records().count(), 0);
        assert!(rm.find(0).next().is_none());
        assert!(rm.find(u64::MAX).next().is_none());
        assert!(rm.find_overlap(0, u64::MAX).is_none());
        assert_eq!(rm.find_begin(5), None);
        assert_eq!(rm.find_end(5), None);
        assert!(rm.list_begin().is_none());
    }

    #[test]
    fn test_rangemap_single_record() {
        let mut rm = RM::new();
        let r = rm.insert((1, 10), 5, 10);
        assert!(!rm.empty());
        assert_eq!(rm.record(r).a, 5);
        assert_eq!(rm.record(r).b, 10);
        // inside / boundary / outside
        assert_eq!(tags(&rm, rm.find(5)), vec![1]);
        assert_eq!(tags(&rm, rm.find(7)), vec![1]);
        assert_eq!(tags(&rm, rm.find(10)), vec![1]);
        assert!(rm.find(4).next().is_none());
        assert!(rm.find(11).next().is_none());
        // single cell in the refinement
        assert_eq!(rm.iter().count(), 1);
        rm.erase(r);
        assert!(rm.empty());
        assert_eq!(rm.iter().count(), 0);
    }

    #[test]
    fn test_rangemap_right_boundary_duplicate_matches_cpp() {
        // C++ oracle: A[0,10] then B[5,10] (llast == b path) gives cells
        // A,A,B,B and find(7) = [1, 2, 2] — B's last cell is DUPLICATED by
        // the trailing insert after the loop break.
        let mut rm = RM::new();
        let a = rm.insert((1, 10), 0, 10);
        let b = rm.insert((2, 20), 5, 10);
        assert_eq!(tags(&rm, rm.iter()), vec![1, 1, 2, 2]);
        assert_eq!(tags(&rm, rm.find(7)), vec![1, 2, 2]);
        let order: Vec<u32> = rm.records().map(|(_, r)| r.tag).collect();
        assert_eq!(order, vec![1, 2]);
        // erase removes every duplicate and sews the partition back together
        rm.erase(b);
        assert_eq!(tags(&rm, rm.iter()), vec![1]);
        assert_eq!(tags(&rm, rm.find(7)), vec![1]);
        rm.erase(a);
        assert!(rm.empty());
    }

    #[test]
    fn test_rangemap_overlap_refinement() {
        // C++ oracle: A[0,10], B[5,15] refine into cells
        // [0,4]A (4), [5,10]A (10), [5,10]B (10), [11,15]B (15).
        let mut rm = RM::new();
        rm.insert((1, 10), 0, 10);
        rm.insert((2, 20), 5, 15);
        assert_eq!(tags(&rm, rm.iter()), vec![1, 1, 2, 2]);
        assert_eq!(tags(&rm, rm.find(0)), vec![1]);
        assert_eq!(tags(&rm, rm.find(7)), vec![1, 2]);
        assert_eq!(tags(&rm, rm.find(12)), vec![2]);
        assert!(rm.find(16).next().is_none());
        // internal cell boundaries (private in C++, checked via the key/data)
        let cells: Vec<(u64, u64, u32)> = rm
            .tree
            .iter()
            .map(|(k, d)| (d.first, k.last, rm.record(d.value).tag))
            .collect();
        assert_eq!(
            cells,
            vec![(0, 4, 1), (5, 10, 1), (5, 10, 2), (11, 15, 2)]
        );
    }

    #[test]
    fn test_rangemap_erase_interior_record_rezips() {
        // C[0,30] then D[10,20] (both-sides refinement); erasing D must zip
        // both boundaries back, leaving C as the single cell [0,30].
        let mut rm = RM::new();
        let c = rm.insert((1, 10), 0, 30);
        let d = rm.insert((2, 20), 10, 20);
        assert_eq!(tags(&rm, rm.find(15)), vec![1, 2]);
        assert_eq!(rm.iter().count(), 4); // [0,9]C [10,20]C [10,20]D [21,30]C
        rm.erase(d);
        assert_eq!(rm.iter().count(), 1);
        let cells: Vec<(u64, u64, u32)> = rm
            .tree
            .iter()
            .map(|(k, dd)| (dd.first, k.last, rm.record(dd.value).tag))
            .collect();
        assert_eq!(cells, vec![(0, 30, 1)]);
        assert_eq!(tags(&rm, rm.find(15)), vec![1]);
        assert_eq!(tags(&rm, rm.find(25)), vec![1]);
        rm.erase(c);
        assert!(rm.empty());
    }

    #[test]
    fn test_rangemap_identical_ranges_subsort_order() {
        // C++ oracle: identical [5,10] with subsorts 10, 20, 5 (tags 1,2,3):
        // sub-ranges iterate [3,3,1,2,2] (subsort order, insert duplicates
        // in insertion order) and the record list order is [3,1,2].
        let mut rm = RM::new();
        rm.insert((1, 10), 5, 10);
        rm.insert((2, 20), 5, 10);
        rm.insert((3, 5), 5, 10);
        assert_eq!(tags(&rm, rm.find(5)), vec![3, 3, 1, 2, 2]);
        let order: Vec<u32> = rm.records().map(|(_, r)| r.tag).collect();
        assert_eq!(order, vec![3, 1, 2]);
        // list cursor protocol agrees with the iterator
        let mut cur = rm.list_begin();
        let mut walked = Vec::new();
        while let Some(idx) = cur {
            walked.push(rm.record(idx).tag);
            cur = rm.list_next(idx);
        }
        assert_eq!(walked, vec![3, 1, 2]);
    }

    #[test]
    fn test_rangemap_find_subsorts_matches_cpp() {
        // C++ oracle outputs for the identical-ranges trio:
        //   find(5,10,10)  -> 3 3 1
        //   find(10,0,20)  -> 3 3 1 2 2
        //   find(10,6,15)  -> 1
        let mut rm = RM::new();
        rm.insert((1, 10), 5, 10);
        rm.insert((2, 20), 5, 10);
        rm.insert((3, 5), 5, 10);
        assert_eq!(tags(&rm, rm.find_subsorts(5, 10, 10)), vec![3, 3, 1]);
        assert_eq!(tags(&rm, rm.find_subsorts(10, 0, 20)), vec![3, 3, 1, 2, 2]);
        assert_eq!(tags(&rm, rm.find_subsorts(10, 6, 15)), vec![1]);
        assert!(rm.find_subsorts(11, 0, u64::MAX).next().is_none());
    }

    #[test]
    fn test_rangemap_find_begin_end_iteration() {
        // C++ oracle: begin..end walk at 7 visits [3,3,1,2,2]; at 11 the
        // positions are equal (empty walk).
        let mut rm = RM::new();
        rm.insert((1, 10), 5, 10);
        rm.insert((2, 20), 5, 10);
        rm.insert((3, 5), 5, 10);
        let fb = rm.find_begin(7);
        let fe = rm.find_end(7);
        assert_eq!(tags(&rm, rm.iter_between(&fb, &fe)), vec![3, 3, 1, 2, 2]);
        assert_eq!(rm.find_begin(11), rm.find_end(11));
        assert!(rm
            .iter_between(&rm.find_begin(11), &rm.find_end(11))
            .next()
            .is_none());
        // reverse walk (`--res.second` in database.cc) via DoubleEnded
        let walked_back: Vec<u32> = rm
            .iter_between(&fb, &fe)
            .rev()
            .map(|i| rm.record(i).tag)
            .collect();
        assert_eq!(walked_back, vec![2, 2, 1, 3, 3]);
    }

    #[test]
    fn test_rangemap_find_overlap_matches_cpp() {
        // C++ oracle: overlap(0,4) = end; overlap(0,5) -> tag 3;
        // overlap(8,30) -> tag 3 (first sub-range in (last,subsort) order).
        let mut rm = RM::new();
        rm.insert((1, 10), 5, 10);
        rm.insert((2, 20), 5, 10);
        rm.insert((3, 5), 5, 10);
        assert!(rm.find_overlap(0, 4).is_none());
        assert_eq!(rm.record(rm.find_overlap(0, 5).unwrap()).tag, 3);
        assert_eq!(rm.record(rm.find_overlap(8, 30).unwrap()).tag, 3);
        assert!(rm.find_overlap(11, 20).is_none());
    }

    #[test]
    fn test_rangemap_boundaries_at_u64_max() {
        let mut rm = RM::new();
        let a = rm.insert((1, 10), u64::MAX - 5, u64::MAX);
        let b = rm.insert((2, 20), 0, u64::MAX);
        // B refines around A: [0,MAX-6]B, [MAX-5,MAX]A, [MAX-5,MAX]B x2
        // (right-boundary duplicate, as in the C++).
        assert_eq!(tags(&rm, rm.find(u64::MAX)), vec![1, 2, 2]);
        assert_eq!(tags(&rm, rm.find(0)), vec![2]);
        assert_eq!(tags(&rm, rm.find(u64::MAX - 6)), vec![2]);
        assert_eq!(rm.record(rm.find_overlap(u64::MAX, u64::MAX).unwrap()).tag, 1);
        rm.erase(b); // exercises aminus1 wraparound (a == 0) in erase
        assert_eq!(tags(&rm, rm.iter()), vec![1]);
        assert_eq!(tags(&rm, rm.find(u64::MAX)), vec![1]);
        assert!(rm.find(u64::MAX - 6).next().is_none());
        rm.erase(a);
        assert!(rm.empty());
    }

    #[test]
    fn test_rangemap_record_list_splice_order() {
        // Insert C[0,10] sub 1, A[0,5] sub 0, B[0,20] sub 0: the list is
        // spliced by (b, subsort) position -> [A, C, B].
        let mut rm = RM::new();
        rm.insert((3, 1), 0, 10); // C
        rm.insert((1, 0), 0, 5); // A
        rm.insert((2, 0), 0, 20); // B
        let order: Vec<u32> = rm.records().map(|(_, r)| r.tag).collect();
        assert_eq!(order, vec![1, 3, 2]);
    }

    #[test]
    fn test_rangemap_clear_and_handle_reuse() {
        let mut rm = RM::new();
        let a = rm.insert((1, 1), 0, 4);
        rm.insert((2, 2), 2, 8);
        rm.erase(a);
        // slot reuse after erase, like a C++ list node reallocation
        let c = rm.insert((3, 3), 100, 200);
        assert_eq!(rm.record(c).tag, 3);
        assert_eq!(tags(&rm, rm.find(150)), vec![3]);
        assert_eq!(tags(&rm, rm.find(3)), vec![2]);
        rm.clear();
        assert!(rm.empty());
        assert_eq!(rm.iter().count(), 0);
        assert!(rm.find(150).next().is_none());
    }

    #[test]
    fn test_rangemap_differential_digest_vs_cpp_oracle() {
        // 58 randomized insert/erase ops (LCG-driven), digesting after each
        // op: find() tags at every point 0..=31, find_overlap(p, p+3),
        // record-list order, and full sub-range iteration order.  The
        // expected digest and live-record count were produced by running the
        // byte-identical op sequence against the C++ rangemap.hh
        // (g++ -std=c++11 against libstdc++; FNV-1a over little-endian u64s;
        // PCG-style LCG: state*6364136223846793005+1442695040888963407,
        // output state>>33).  This pins refinement, multiset equal-key
        // ordering (including the hinted-insert placements), splice order
        // and zip/unzip behavior at scale.  The per-op traces of the two
        // implementations were verified line-identical over this prefix.
        //
        // Why 58 ops and not more: at op 58 of this sequence the UPSTREAM
        // container corrupts itself — erase() only walks sub-ranges whose
        // key (ending boundary) is >= getFirst(), but an earlier zip() had
        // legitimately extended one of the record's sub-ranges below its
        // own range (visible as find(1) returning a [3,5] record from op 21
        // on, which the Rust port reproduces exactly).  Erasing that record
        // strands the extended sub-range, whose record pointer then dangles:
        // C++ silently reads freed list memory (UB), the Rust port panics
        // ("stale RecordIdx") per ADR 0004.  Beyond this point the C++
        // output is not a sound oracle.
        struct Lcg(u64);
        impl Lcg {
            fn next(&mut self) -> u64 {
                self.0 = self
                    .0
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                self.0 >> 33
            }
        }
        struct Fnv(u64);
        impl Fnv {
            fn mix(&mut self, v: u64) {
                for byte in v.to_le_bytes() {
                    self.0 ^= byte as u64;
                    self.0 = self.0.wrapping_mul(0x100000001b3);
                }
            }
        }
        const SEP: u64 = u64::MAX;
        const NONE: u64 = u64::MAX - 1;

        let mut rng = Lcg(0x853c49e6748fea9b);
        let mut fnv = Fnv(0xcbf29ce484222325);
        let mut rm = RM::new();
        let mut live: Vec<RecordIdx> = Vec::new();
        for op in 0..58u32 {
            let r = rng.next() % 3;
            if live.is_empty() || r != 0 {
                let a = rng.next() % 24;
                let len = rng.next() % 8;
                let ss = rng.next() % 4;
                live.push(rm.insert((op, ss), a, a + len));
            } else {
                let i = (rng.next() as usize) % live.len();
                rm.erase(live[i]);
                live.remove(i);
            }
            for p in 0..=31u64 {
                for idx in rm.find(p) {
                    fnv.mix(rm.record(idx).tag as u64);
                }
                fnv.mix(SEP);
            }
            for p in 0..=31u64 {
                match rm.find_overlap(p, p + 3) {
                    Some(idx) => fnv.mix(rm.record(idx).tag as u64),
                    None => fnv.mix(NONE),
                }
            }
            for (_, rec) in rm.records() {
                fnv.mix(rec.tag as u64);
            }
            fnv.mix(SEP);
            for idx in rm.iter() {
                fnv.mix(rm.record(idx).tag as u64);
            }
            fnv.mix(SEP);
        }
        assert_eq!(live.len(), 26);
        assert_eq!(fnv.0, 0xa774de5500ebcb54);
    }

    #[test]
    fn test_rangemap_full_range_record() {
        // A record spanning the whole u64 domain.
        let mut rm = RM::new();
        let a = rm.insert((1, 0), 0, u64::MAX);
        assert_eq!(tags(&rm, rm.find(0)), vec![1]);
        assert_eq!(tags(&rm, rm.find(u64::MAX)), vec![1]);
        assert_eq!(tags(&rm, rm.find(1 << 63)), vec![1]);
        let d = rm.insert((2, 1), 5, 5); // single-point record inside
        assert_eq!(tags(&rm, rm.find(5)), vec![1, 2]); // cell [5,5], subsort order A(0) then D(1)
        rm.erase(d);
        assert_eq!(tags(&rm, rm.iter()), vec![1]);
        rm.erase(a);
        assert!(rm.empty());
    }
}
