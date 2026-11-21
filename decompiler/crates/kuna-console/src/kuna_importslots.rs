//! Indexed half-open import storage, independent of whether a slot lies in code.

#[derive(Default)]
pub(crate) struct ImportSlots(Vec<(u64, u64)>);

impl ImportSlots {
    pub(crate) fn new(mut ranges: Vec<(u64, u64)>) -> Self {
        ranges.retain(|&(lo, hi)| lo < hi);
        ranges.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
        for (lo, hi) in ranges {
            if let Some(last) = merged.last_mut().filter(|last| lo <= last.1) {
                last.1 = last.1.max(hi);
            } else {
                merged.push((lo, hi));
            }
        }
        debug_assert!(merged.windows(2).all(|pair| pair[0].1 < pair[1].0));
        Self(merged)
    }

    pub(crate) fn insert(&mut self, lo: u64, hi: u64) {
        let mut ranges = std::mem::take(&mut self.0);
        ranges.push((lo, hi));
        *self = Self::new(ranges);
    }

    pub(crate) fn contains(&self, address: u64) -> bool {
        let end = self.0.partition_point(|&(lo, _)| {
            #[cfg(test)]
            COMPARISONS.with(|count| count.set(count.get() + 1));
            lo <= address
        });
        end != 0 && address < self.0[end - 1].1
    }
}

#[cfg(test)]
thread_local! {
    static COMPARISONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_preserves_half_open_membership() {
        let ranges = vec![
            (80, 90),
            (12, 16),
            (8, 12),
            (10, 15),
            (9, 10),
            (7, 7),
            (30, 20),
            (0, 1),
            (u64::MAX - 3, u64::MAX),
        ];
        let slots = ImportSlots::new(ranges.clone());
        for address in (0..100).chain([u64::MAX - 4, u64::MAX - 3, u64::MAX - 1, u64::MAX]) {
            assert_eq!(
                slots.contains(address),
                ranges.iter().any(|&(lo, hi)| lo <= address && address < hi),
                "{address:#x}"
            );
        }
        assert!(!ImportSlots::default().contains(0));
    }

    #[test]
    fn large_positive_and_negative_queries_use_logarithmic_comparisons() {
        let slots = ImportSlots::new((0..10_000).map(|i| (4 * i + 1, 4 * i + 3)).collect());
        for i in 0..10_000 {
            for (address, expected) in [(4 * i, false), (4 * i + 2, true)] {
                COMPARISONS.with(|count| count.set(0));
                assert_eq!(slots.contains(address), expected);
                COMPARISONS.with(|count| {
                    assert!(
                        (1..=16).contains(&count.get()),
                        "{} comparisons",
                        count.get()
                    )
                });
            }
        }
    }
}
