//! Which previous-pass markers a refinement of a revisited range would cut.
//!
//! A range that grows on a later pass (an unaligned access now reaches across
//! two ranges heritaged separately) holds the MULTIEQUAL/INDIRECT markers of
//! the smaller ranges. `collect` sets them aside for `removeRevisitedMarkers`
//! and the refinement partitions the range from the reads, writes and inputs
//! alone, so a partition boundary can fall inside a marker. Each partition is
//! then collected on its own, and a marker that starts in one and ends in
//! another arrives as a write wider than its range. Upstream's
//! `Heritage::refinement` and renaming both assume every Varnode of a range
//! lies inside it: the partition's reads are linked to the whole marker, and a
//! second refinement of the partition indexes past its refinement array.

use kuna_base::types::int4;

/// The partition holding each byte of a refined range.
pub(crate) struct Partitions {
    starts: Vec<usize>,
    ends: Vec<usize>,
}

impl Partitions {
    /// `refine` holds each partition's size at its first byte, as
    /// `Heritage::refinement` leaves it.
    pub(crate) fn new(refine: &[int4]) -> Partitions {
        let mut starts = vec![0; refine.len()];
        let mut ends = vec![0; refine.len()];
        let mut pos = 0;
        while pos < refine.len() {
            let end = (pos + refine[pos].max(1) as usize).min(refine.len());
            starts[pos..end].fill(pos);
            ends[pos..end].fill(end);
            pos = end;
        }
        Partitions { starts, ends }
    }

    /// The partitions `[lo, hi)` that `[start, start + size)` spans when it
    /// crosses a partition boundary, or `None` when one partition holds it.
    pub(crate) fn cut_span(&self, start: usize, size: usize) -> Option<(usize, usize)> {
        let last = (start + size).checked_sub(1)?;
        let (&end, &hi) = (self.ends.get(start)?, self.ends.get(last)?);
        (end < start + size).then(|| (self.starts[start], hi))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spans_inside_one_partition_are_kept() {
        let parts = Partitions::new(&[2, 0, 1, 1, 4, 0, 0, 0]);
        assert_eq!(parts.cut_span(0, 2), None);
        assert_eq!(parts.cut_span(2, 1), None);
        assert_eq!(parts.cut_span(4, 4), None);
        assert_eq!(parts.cut_span(5, 2), None);
    }

    #[test]
    fn a_cut_span_is_widened_to_the_partitions_it_touches() {
        let parts = Partitions::new(&[3, 0, 0, 5, 0, 0, 0, 0, 3, 0, 0, 5, 0, 0, 0, 0]);
        assert_eq!(parts.cut_span(0, 8), Some((0, 8)), "an 8-byte marker over a 3+5 split");
        assert_eq!(parts.cut_span(8, 8), Some((8, 16)));
        assert_eq!(parts.cut_span(2, 4), Some((0, 8)));
        assert_eq!(parts.cut_span(3, 5), None);
    }

    #[test]
    fn a_span_past_the_range_is_not_widened() {
        let parts = Partitions::new(&[2, 0, 1, 1]);
        assert_eq!(parts.cut_span(3, 2), None);
        assert_eq!(parts.cut_span(4, 1), None);
        assert_eq!(parts.cut_span(0, 0), None);
    }

    #[test]
    fn a_healed_one_three_junction_is_one_partition() {
        let parts = Partitions::new(&[4, 3, 0, 0, 4, 0, 0, 0]);
        assert_eq!(parts.cut_span(1, 3), None, "the stale size at the junction is not a cut");
        assert_eq!(parts.cut_span(2, 4), Some((0, 8)));
    }
}
