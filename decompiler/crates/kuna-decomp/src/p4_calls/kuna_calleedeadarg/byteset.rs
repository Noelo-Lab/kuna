//! A set of register-space bytes stored as the runs it covers.
//!
//! The callee probe records, at every path terminator, which register bytes the
//! path wrote, and the run keeps each probed summary for every later caller. A
//! register write covers whole contiguous bytes, so a path that wrote a few
//! hundred bytes covers a few dozen runs; storing the runs instead of one tree
//! entry per byte is what keeps those summaries from growing the batch's heap
//! by megabytes per probed callee. The set semantics, and the byte order of
//! iteration, are those of a `BTreeSet<(int4, u64)>`.

use kuna_base::types::int4;

/// Bytes keyed by `(space index, offset)`, held as sorted, disjoint,
/// non-adjacent inclusive runs `(space index, first, last)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct ByteSet {
    runs: Vec<(int4, u64, u64)>,
    count: usize,
}

impl ByteSet {
    pub(super) fn new() -> Self {
        ByteSet::default()
    }

    /// How many bytes the set holds.
    pub(super) fn len(&self) -> usize {
        self.count
    }

    pub(super) fn clear(&mut self) {
        self.runs.clear();
        self.count = 0;
    }

    /// Drop the spare capacity, for a set kept for the rest of the run.
    pub(super) fn shrink_to_fit(&mut self) {
        self.runs.shrink_to_fit();
    }

    /// The index of the first run that starts after `(idx, b)`.
    fn after(&self, idx: int4, b: u64) -> usize {
        self.runs.partition_point(|&(i, first, _)| (i, first) <= (idx, b))
    }

    pub(super) fn contains(&self, &(idx, b): &(int4, u64)) -> bool {
        let at = self.after(idx, b);
        at > 0 && {
            let (i, _, last) = self.runs[at - 1];
            i == idx && b <= last
        }
    }

    /// Add one byte; `true` when it was not already present.
    pub(super) fn insert(&mut self, (idx, b): (int4, u64)) -> bool {
        let at = self.after(idx, b);
        let joins_prev = at > 0 && {
            let (i, _, last) = self.runs[at - 1];
            if i == idx && b <= last {
                return false;
            }
            i == idx && last.checked_add(1) == Some(b)
        };
        let joins_next = at < self.runs.len() && {
            let (i, first, _) = self.runs[at];
            i == idx && b.checked_add(1) == Some(first)
        };
        match (joins_prev, joins_next) {
            (true, true) => {
                self.runs[at - 1].2 = self.runs[at].2;
                self.runs.remove(at);
            }
            (true, false) => self.runs[at - 1].2 = b,
            (false, true) => self.runs[at].1 = b,
            (false, false) => self.runs.insert(at, (idx, b, b)),
        }
        self.count += 1;
        true
    }

    /// Drop one byte; `true` when it was present.
    #[cfg(test)]
    pub(super) fn remove(&mut self, &(idx, b): &(int4, u64)) -> bool {
        if !self.contains(&(idx, b)) {
            return false;
        }
        let at = self.after(idx, b) - 1;
        let (i, first, last) = self.runs[at];
        match (first == b, last == b) {
            (true, true) => {
                self.runs.remove(at);
            }
            (true, false) => self.runs[at].1 = b + 1,
            (false, true) => self.runs[at].2 = b - 1,
            (false, false) => {
                self.runs[at].2 = b - 1;
                self.runs.insert(at + 1, (i, b + 1, last));
            }
        }
        self.count -= 1;
        true
    }

    /// Is every byte of `self` in `other`? A run is covered only by one run
    /// of `other`, since `other`'s runs are maximal.
    pub(super) fn is_subset(&self, other: &ByteSet) -> bool {
        if self.count > other.count {
            return false;
        }
        let mut j = 0;
        for &(idx, first, last) in &self.runs {
            while j < other.runs.len() && (other.runs[j].0, other.runs[j].2) < (idx, first) {
                j += 1;
            }
            match other.runs.get(j) {
                Some(&(i, f, l)) if i == idx && f <= first && last <= l => {}
                _ => return false,
            }
        }
        true
    }

    /// Add every run of `other`.
    fn add_runs(&mut self, other: &ByteSet) {
        if other.runs.is_empty() {
            return;
        }
        if self.runs.is_empty() {
            *self = other.clone();
            return;
        }
        let mut merged: Vec<(int4, u64, u64)> = Vec::with_capacity(self.runs.len() + other.runs.len());
        let (mut a, mut b) = (self.runs.iter().peekable(), other.runs.iter().peekable());
        loop {
            let next = match (a.peek(), b.peek()) {
                (Some(&&x), Some(&&y)) => {
                    if (x.0, x.1) <= (y.0, y.1) {
                        a.next();
                        x
                    } else {
                        b.next();
                        y
                    }
                }
                (Some(&&x), None) => {
                    a.next();
                    x
                }
                (None, Some(&&y)) => {
                    b.next();
                    y
                }
                (None, None) => break,
            };
            match merged.last_mut() {
                Some(last) if last.0 == next.0 && last.2.checked_add(1).is_none_or(|end| next.1 <= end) => {
                    last.2 = last.2.max(next.2);
                }
                _ => merged.push(next),
            }
        }
        self.count = merged.iter().map(|&(_, f, l)| (l - f) as usize + 1).sum();
        self.runs = merged;
    }

    /// Move every byte of `other` into `self`, leaving `other` empty.
    pub(super) fn append(&mut self, other: &mut ByteSet) {
        let taken = std::mem::take(other);
        self.add_runs(&taken);
    }

    /// The bytes in `self` or `other`.
    pub(super) fn union(&self, other: &ByteSet) -> ByteSet {
        let mut out = self.clone();
        out.add_runs(other);
        out
    }

    /// The bytes in `self` and not in `other`.
    pub(super) fn difference(&self, other: &ByteSet) -> ByteSet {
        let mut runs = Vec::with_capacity(self.runs.len());
        let mut j = 0;
        for &(idx, first, last) in &self.runs {
            while j < other.runs.len() && (other.runs[j].0, other.runs[j].2) < (idx, first) {
                j += 1;
            }
            let mut from = Some(first);
            let mut k = j;
            while let (Some(start), Some(&(i, f, l))) = (from, other.runs.get(k)) {
                if i != idx || f > last {
                    break;
                }
                if f > start {
                    runs.push((idx, start, f - 1));
                }
                from = l.checked_add(1).filter(|&n| n <= last);
                k += 1;
            }
            if let Some(start) = from {
                runs.push((idx, start, last));
            }
        }
        let count = runs.iter().map(|&(_, f, l)| (l - f) as usize + 1).sum();
        ByteSet { runs, count }
    }

    /// The bytes in both `self` and `other`.
    pub(super) fn intersection(&self, other: &ByteSet) -> ByteSet {
        let mut runs = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < self.runs.len() && j < other.runs.len() {
            let (ai, af, al) = self.runs[i];
            let (bi, bf, bl) = other.runs[j];
            if (ai, al) < (bi, bf) {
                i += 1;
                continue;
            }
            if (bi, bl) < (ai, af) {
                j += 1;
                continue;
            }
            runs.push((ai, af.max(bf), al.min(bl)));
            if (ai, al) < (bi, bl) {
                i += 1;
            } else {
                j += 1;
            }
        }
        let count = runs.iter().map(|&(_, f, l)| (l - f) as usize + 1).sum();
        ByteSet { runs, count }
    }

    /// Every byte, in `(space index, offset)` order.
    pub(super) fn iter(&self) -> impl Iterator<Item = (int4, u64)> + '_ {
        self.runs.iter().flat_map(|&(idx, first, last)| (first..=last).map(move |b| (idx, b)))
    }
}

impl Extend<(int4, u64)> for ByteSet {
    fn extend<I: IntoIterator<Item = (int4, u64)>>(&mut self, iter: I) {
        for byte in iter {
            self.insert(byte);
        }
    }
}

impl FromIterator<(int4, u64)> for ByteSet {
    fn from_iter<I: IntoIterator<Item = (int4, u64)>>(iter: I) -> Self {
        let mut out = ByteSet::new();
        out.extend(iter);
        out
    }
}

impl IntoIterator for ByteSet {
    type Item = (int4, u64);
    type IntoIter = std::vec::IntoIter<(int4, u64)>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}
