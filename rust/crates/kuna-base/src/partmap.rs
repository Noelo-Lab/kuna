//! Port of `decompiler/cpp/partmap.hh` (W1): the `partmap<>` template mapping
//! a linear space to value objects.
//!
//! Let R be the linear space with an ordering, and let { a_i } be a finite
//! set of points in R.  The a_i partition R into a finite number of disjoint
//! sets
//! `{ x : x < a_0 }, { x : x>=a_0 && x < a_1 }, ... { x : x>=a_n }`.
//!
//! A partmap maps elements of this partition to value objects.  A value is
//! associated with any element x in R by looking up the value associated
//! with the partition element containing x.
//!
//! The map is defined by starting with a *default* value object that applies
//! to the whole linear space.  Then *split* points are introduced, one at a
//! time, in the linear space.  At each split point, the associated value
//! object is split into two objects.  At any point the value object
//! describing some part of the linear space can be changed.
//!
//! C++ method-name mapping: `getValue` (non-const) → [`PartMap::get_value_mut`],
//! `getValue` (const) → [`PartMap::get_value`], `begin()` → [`PartMap::iter`],
//! `begin(pnt)` (a `lower_bound`) → [`PartMap::iter_from`].

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

/// A map from a linear space to value objects (C++ `partmap<_linetype,_valuetype>`).
#[derive(Debug, Clone)]
pub struct PartMap<K, V> {
    /// Map from linear split points to the value objects
    database: BTreeMap<K, V>,
    /// The value object *before* the first split point
    defaultvalue: V,
}

impl<K: Ord + Clone, V: Clone + Default> Default for PartMap<K, V> {
    /// C++ default-constructs `defaultvalue`.
    fn default() -> Self {
        PartMap::new(V::default())
    }
}

impl<K: Ord + Clone, V: Clone> PartMap<K, V> {
    /// Create an empty partition with the given default value object.
    pub fn new(defaultvalue: V) -> Self {
        PartMap { database: BTreeMap::new(), defaultvalue }
    }

    /// Look up the first split point coming before the given point and return
    /// the value object it maps to.  If there is no earlier split point
    /// return the default value.  (C++ `getValue`, const flavor.)
    pub fn get_value(&self, pnt: &K) -> &V {
        // C++: iter = database.upper_bound(pnt); if begin -> default; --iter.
        match self.database.range((Unbounded, Included(pnt))).next_back() {
            Some((_, v)) => v,
            None => &self.defaultvalue,
        }
    }

    /// Mutable flavor of [`PartMap::get_value`] (C++ `getValue`, non-const).
    /// Note that, as in C++, this can hand back a mutable reference to the
    /// default value object.
    pub fn get_value_mut(&mut self, pnt: &K) -> &mut V {
        let key = self
            .database
            .range((Unbounded, Included(pnt)))
            .next_back()
            .map(|(k, _)| k.clone());
        match key {
            Some(k) => self
                .database
                .get_mut(&k)
                .expect("partmap: split point vanished"),
            None => &mut self.defaultvalue,
        }
    }

    /// Add (if not already present) a point to the linear partition.
    /// Returns the (possibly) new value object for the range starting at the
    /// point.  (C++ `split`.)
    pub fn split(&mut self, pnt: &K) -> &mut V {
        // C++: iter = database.upper_bound(pnt);
        //      if (iter != begin) { --iter;
        //        if ((*iter).first == pnt) return (*iter).second;  // point matches exactly
        //        newref = database[pnt] = (*iter).second; }        // copy of original partition value
        //      else newref = database[pnt] = defaultvalue;         // copy of defaultvalue
        let newval = match self.database.range((Unbounded, Included(pnt))).next_back() {
            Some((k, v)) => {
                if k == pnt {
                    None // point matches exactly: return old ref
                } else {
                    Some(v.clone())
                }
            }
            None => Some(self.defaultvalue.clone()),
        };
        match newval {
            None => self
                .database
                .get_mut(pnt)
                .expect("partmap: exact split point vanished"),
            Some(v) => self.database.entry(pnt.clone()).or_insert(v),
        }
    }

    /// Get the default value object (C++ `defaultValue`, const flavor).
    pub fn default_value(&self) -> &V {
        &self.defaultvalue
    }

    /// Get the default value object (C++ `defaultValue`, non-const flavor).
    pub fn default_value_mut(&mut self) -> &mut V {
        &mut self.defaultvalue
    }

    /// Clear a range of split points (C++ `clearRange`).
    ///
    /// Split points are introduced at the two boundary points of the given
    /// range, and all split points in between are removed.  The value object
    /// that was initially present at the left-most boundary point becomes
    /// the value (as a copy) for the whole range.  Requires `pnt1 <= pnt2`
    /// (the C++ erase is undefined otherwise).
    pub fn clear_range(&mut self, pnt1: &K, pnt2: &K) -> &mut V {
        self.split(pnt1);
        self.split(pnt2);
        // C++: beg = lower_bound(pnt1); end = lower_bound(pnt2);
        //      ++beg; database.erase(beg, end);
        // i.e. erase every split point strictly between pnt1 and pnt2.
        // (pnt1 == pnt2 is C++ UB through erase(beg,end) with beg past end;
        // here it simply erases nothing.)
        if pnt1 < pnt2 {
            let doomed: Vec<K> = self
                .database
                .range((Excluded(pnt1), Excluded(pnt2)))
                .map(|(k, _)| k.clone())
                .collect();
            for k in &doomed {
                self.database.remove(k);
            }
        }
        self.database
            .get_mut(pnt1)
            .expect("partmap: clear_range left boundary vanished")
    }

    /// Get the value object for a given point and return the range over
    /// which the value object applies (C++ `bounds`).
    ///
    /// Returns `(value, before, after, valid)` where `before`/`after` define
    /// the maximal range over which the value applies and `valid` describes
    /// which of the bounding points apply:
    ///   - 0 if both bounds apply
    ///   - 1 if there is no lower bound
    ///   - 2 if there is no upper bound
    ///   - 3 if there is neither a lower or upper bound
    ///
    /// (`before`/`after` are `None` exactly where the C++ version leaves the
    /// corresponding out-parameter untouched.)
    pub fn bounds(&self, pnt: &K) -> (&V, Option<&K>, Option<&K>, i32) {
        if self.database.is_empty() {
            return (&self.defaultvalue, None, None, 3);
        }
        // enditer = database.upper_bound(pnt)
        let enditer = self.database.range((Excluded(pnt), Unbounded)).next();
        // iter = enditer - 1, valid only if enditer != begin()
        let previter = self.database.range((Unbounded, Included(pnt))).next_back();
        match previter {
            Some((before, value)) => match enditer {
                None => (value, Some(before), None, 2), // No upperbound
                Some((after, _)) => (value, Some(before), Some(after), 0), // Fully bounded
            },
            None => {
                // No lowerbound; after = first split point (non-empty here)
                let (after, _) = enditer.expect("partmap: non-empty database");
                (&self.defaultvalue, None, Some(after), 1)
            }
        }
    }

    /// Iterate over all split points (C++ `begin()`/`end()`).
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.database.iter()
    }

    /// Iterate over all split points with mutable values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.database.iter_mut()
    }

    /// Get first split point at-or-after the given point — a `lower_bound`
    /// (C++ `begin(pnt)`).
    pub fn iter_from<'a>(&'a self, pnt: &K) -> impl Iterator<Item = (&'a K, &'a V)> {
        self.database.range((Included(pnt), Unbounded))
    }

    /// Mutable flavor of [`PartMap::iter_from`].
    pub fn iter_from_mut<'a>(&'a mut self, pnt: &K) -> impl Iterator<Item = (&'a K, &'a mut V)> {
        self.database.range_mut((Included(pnt), Unbounded))
    }

    /// Clear all split points (C++ `clear`).
    pub fn clear(&mut self) {
        self.database.clear();
    }

    /// Return `true` if there are no split points (C++ `empty`).
    pub fn empty(&self) -> bool {
        self.database.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partmap_upstream_doc_example() {
        // Transcribes the `#if 0` main() at the bottom of partmap.hh.
        let mut data: PartMap<i32, u32> = PartMap::new(0);
        *data.default_value_mut() = 0;
        *data.split(&5) = 5;
        *data.split(&2) = 2;
        *data.split(&3) = 4;
        *data.split(&3) = 3; // exact re-split overwrites the same slot
        assert_eq!(*data.get_value(&6), 5);
        assert_eq!(*data.get_value(&8), 5);
        assert_eq!(*data.get_value(&4), 3);
        assert_eq!(*data.get_value(&1), 0);
        // iter = data.begin(3); walk to end
        let walked: Vec<u32> = data.iter_from(&3).map(|(_, v)| *v).collect();
        assert_eq!(walked, vec![3, 5]);
    }

    #[test]
    fn test_partmap_get_value_default_when_empty() {
        let mut pm: PartMap<u64, String> = PartMap::new("dflt".to_string());
        assert!(pm.empty());
        assert_eq!(pm.get_value(&0), "dflt");
        assert_eq!(pm.get_value(&u64::MAX), "dflt");
        // non-const getValue can hand back the default value object itself
        pm.get_value_mut(&42).push('!');
        assert_eq!(pm.default_value(), "dflt!");
    }

    #[test]
    fn test_partmap_split_copies_previous_value() {
        let mut pm: PartMap<u64, u32> = PartMap::new(7);
        *pm.split(&10) = 100;
        // split point after 10 copies the value of the partition it lands in
        assert_eq!(*pm.split(&20), 100);
        // split point before the first split copies defaultvalue
        assert_eq!(*pm.split(&5), 7);
        // exact match returns the existing object, does not reset it
        *pm.split(&20) = 200;
        assert_eq!(*pm.split(&20), 200);
        assert_eq!(*pm.get_value(&25), 200);
        assert_eq!(*pm.get_value(&12), 100);
        assert_eq!(*pm.get_value(&7), 7);
        assert_eq!(*pm.get_value(&0), 7);
        assert_eq!(pm.iter().count(), 3);
    }

    #[test]
    fn test_partmap_clear_range() {
        let mut pm: PartMap<u64, u32> = PartMap::new(0);
        *pm.split(&10) = 1;
        *pm.split(&20) = 2;
        *pm.split(&30) = 3;
        *pm.split(&40) = 4;
        // Clearing [10,40) keeps boundary splits at 10 and 40, removes 20/30,
        // and the left boundary's value covers the whole range.
        let v = pm.clear_range(&10, &40);
        assert_eq!(*v, 1);
        assert_eq!(*pm.get_value(&25), 1);
        assert_eq!(*pm.get_value(&39), 1);
        assert_eq!(*pm.get_value(&40), 4);
        let keys: Vec<u64> = pm.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![10, 40]);
        // Degenerate range: boundary split introduced, nothing erased.
        let mut pm2: PartMap<u64, u32> = PartMap::new(9);
        *pm2.clear_range(&5, &5) = 55;
        assert_eq!(*pm2.get_value(&5), 55);
        assert_eq!(*pm2.get_value(&4), 9);
    }

    #[test]
    fn test_partmap_bounds_valid_codes() {
        let mut pm: PartMap<u64, u32> = PartMap::new(0);
        // empty: neither bound
        let (v, before, after, valid) = pm.bounds(&50);
        assert_eq!((*v, before, after, valid), (0, None, None, 3));
        *pm.split(&10) = 1;
        *pm.split(&20) = 2;
        // before the first split point: no lower bound, after = first split
        let (v, before, after, valid) = pm.bounds(&5);
        assert_eq!((*v, before.copied(), after.copied(), valid), (0, None, Some(10), 1));
        // fully bounded
        let (v, before, after, valid) = pm.bounds(&15);
        assert_eq!((*v, before.copied(), after.copied(), valid), (1, Some(10), Some(20), 0));
        // past the last split point: no upper bound
        let (v, before, after, valid) = pm.bounds(&25);
        assert_eq!((*v, before.copied(), after.copied(), valid), (2, Some(20), None, 2));
        // a point sitting exactly on a split belongs to the range it starts
        let (v, before, after, valid) = pm.bounds(&10);
        assert_eq!((*v, before.copied(), after.copied(), valid), (1, Some(10), Some(20), 0));
    }

    #[test]
    fn test_partmap_boundaries_at_u64_max() {
        let mut pm: PartMap<u64, u32> = PartMap::new(1);
        *pm.split(&u64::MAX) = 99;
        *pm.split(&0) = 5;
        assert_eq!(*pm.get_value(&u64::MAX), 99);
        assert_eq!(*pm.get_value(&(u64::MAX - 1)), 5);
        assert_eq!(*pm.get_value(&0), 5);
        let (v, before, after, valid) = pm.bounds(&u64::MAX);
        assert_eq!(
            (*v, before.copied(), after.copied(), valid),
            (99, Some(u64::MAX), None, 2)
        );
        let cleared = pm.clear_range(&0, &u64::MAX);
        assert_eq!(*cleared, 5);
        let keys: Vec<u64> = pm.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![0, u64::MAX]);
    }

    #[test]
    fn test_partmap_iter_from_is_lower_bound() {
        let mut pm: PartMap<u64, u32> = PartMap::new(0);
        *pm.split(&10) = 1;
        *pm.split(&20) = 2;
        // at-or-after semantics: starting exactly on a split point includes it
        let keys: Vec<u64> = pm.iter_from(&10).map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![10, 20]);
        let keys: Vec<u64> = pm.iter_from(&11).map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![20]);
        let keys: Vec<u64> = pm.iter_from(&21).map(|(k, _)| *k).collect();
        assert!(keys.is_empty());
        // mutable flavor
        for (_, v) in pm.iter_from_mut(&10) {
            *v += 100;
        }
        assert_eq!(*pm.get_value(&15), 101);
        pm.clear();
        assert!(pm.empty());
    }
}
