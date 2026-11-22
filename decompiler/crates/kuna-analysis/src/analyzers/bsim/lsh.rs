//! The sparse weighted feature vector and its comparison — the LSH core of BSim.
//!
//! A decompiled function's feature list (the sorted 32-bit hashes produced by
//! [`kuna_decomp::signature`]) becomes an [`LshCosineVector`]: one
//! [`HashEntry`] per **distinct** hash, carrying how often it occurred (term
//! frequency) and the coefficient the weight tables assign. Two vectors are
//! compared with a cosine similarity; [`VectorCompare`] collects the counts the
//! significance formula needs.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Generic/src/main/java/generic/lsh/vector/LSHCosineVector.java` —
//!   `installFeatures`, `calcLength`, `compare`, `compareCounts`,
//!   `calcUniqueHash`.
//! - `.../vector/HashEntry.java` — the per-feature record; `tf` is stored
//!   **minus one** and `getTF()` adds it back.
//! - `.../vector/VectorCompare.java` — `fillOut`.
//! - `.../vector/LSHVectorFactory.java` — `calculateSignificance` and
//!   `getSelfSignificance`.
//!
//! # Faithfulness
//!
//! The hashes are **signed** `i32` on both sides of the merge comparison: Ghidra
//! compares `hash1 + 0x80000000 < hash2 + 0x80000000` to get an unsigned order
//! over signed values, which is reproduced with `u32` casts rather than a plain
//! signed compare. Term frequency is clamped at 63 (`HashEntry` stores it in a
//! `short` minus one), and the IDF index at 511.
//!
//! `calcUniqueHash` is a two-register CRC mix whose **shift amounts read the
//! other register's current value** (`reg1` mixes `reg2 >>> 24` and vice versa);
//! getting that order wrong changes every vector identity.

use super::lshweights::{IDFLookup, WeightFactory};

/// One distinct feature in a vector.
///
/// Java stores `tf` as `short (count - 1)` so a count of 1 costs one `short`
/// value of 0; [`HashEntry::tf`] adds it back. `idf` is an index into the IDF
/// weight table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HashEntry {
    /// The 32-bit feature hash (signed, as Java's `int`).
    pub hash: i32,
    /// The stored term frequency minus one (`HashEntry.tf`).
    tf: i16,
    /// The normalised IDF index (`HashEntry.idf`).
    idf: i16,
    /// The feature's weight.
    pub coeff: f64,
}

impl HashEntry {
    /// A feature with an explicit weight (`HashEntry(int,int,double)`), for
    /// restoring a stored vector.
    pub fn with_weight(hash: i32, term_count: i32, weight: f64) -> Self {
        HashEntry {
            hash,
            tf: if term_count > 63 { 63 } else { (term_count - 1) as i16 },
            idf: 1,
            coeff: weight,
        }
    }

    /// A feature weighted from its term frequency and corpus frequency
    /// (`HashEntry(int,int,int,WeightFactory)`).
    pub fn weighted(hash: i32, term_count: i32, doc_count: i32, factory: &WeightFactory) -> Self {
        let tf = if term_count > 63 { 63 } else { (term_count - 1) as i16 };
        let idf = if doc_count > 511 { 511 } else { doc_count as i16 };
        HashEntry { hash, tf, idf, coeff: factory.get_coeff(idf, tf) }
    }

    /// The term frequency, restored (`getTF`).
    pub fn term_frequency(&self) -> i16 {
        self.tf + 1
    }

    /// The IDF index (`getIDF`).
    pub fn idf_index(&self) -> i16 {
        self.idf
    }
}

/// The intermediate result of comparing two vectors (`VectorCompare`).
///
/// `fill_out` derives `min`, `max`, `num_flip` and `diff` from the four counts.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VectorCompare {
    /// The dot product of the two vectors.
    pub dotproduct: f64,
    /// Total hashes in the first vector, counting multiplicity.
    pub acount: i32,
    /// Total hashes in the second vector, counting multiplicity.
    pub bcount: i32,
    /// Hashes the two vectors share, counting multiplicity.
    pub intersectcount: i32,
    /// The smaller vector count.
    pub min: i32,
    /// The larger vector count.
    pub max: i32,
    /// Hashes in the smaller vector that are not shared.
    pub numflip: i32,
    /// The difference in vector size.
    pub diff: i32,
}

impl VectorCompare {
    /// Derive `min`, `max`, `num_flip` and `diff` (`VectorCompare.fillOut`).
    pub fn fill_out(&mut self) {
        if self.acount < self.bcount {
            self.min = self.acount;
            self.max = self.bcount;
        } else {
            self.min = self.bcount;
            self.max = self.acount;
        }
        self.diff = self.max - self.min;
        self.numflip = self.min - self.intersectcount;
    }
}

/// A weighted sparse feature vector (`LSHCosineVector`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LshCosineVector {
    /// Entries sorted by hash (Ghidra's `hash` array).
    entries: Vec<HashEntry>,
    /// The vector's euclidean length (`length`).
    length: f64,
    /// Total hashes counting multiplicity (`hashcount`).
    hashcount: i32,
}

impl LshCosineVector {
    /// An empty vector (the Java template constructor).
    pub fn new() -> Self {
        LshCosineVector::default()
    }

    /// Build from a feature list that is already sorted ascending
    /// (`LSHCosineVector(int[], WeightFactory, IDFLookup)`).
    ///
    /// The same hash may repeat; that is its term frequency.
    pub fn from_features(feature: &[u32], factory: &WeightFactory, idf: &IDFLookup) -> Self {
        let entries = feature
            .chunk_by(|a, b| a == b)
            .map(|group| {
                let hash = group[0] as i32;
                HashEntry::weighted(hash, group.len().min(64) as i32, idf.get_count(hash), factory)
            })
            .collect();
        Self::from_entries(entries)
    }

    /// Install already-sorted hashes and weights (`setHashEntries`).
    pub fn from_entries(entries: Vec<HashEntry>) -> Self {
        let mut vector = LshCosineVector { entries, ..Default::default() };
        vector.calc_length();
        vector
    }

    /// The entries, sorted by hash.
    pub fn entries(&self) -> &[HashEntry] {
        &self.entries
    }

    /// `numEntries`.
    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }

    /// The euclidean length (`getLength`).
    pub fn length(&self) -> f64 {
        self.length
    }

    /// Total hashes counting multiplicity.
    pub fn hashcount(&self) -> i32 {
        self.hashcount
    }

    /// `calcLength`: the euclidean norm over the coefficients, and the summed
    /// term frequency.
    fn calc_length(&mut self) {
        self.length = 0.0;
        self.hashcount = 0;
        for entry in &self.entries {
            let coeff = entry.coeff;
            self.length += coeff * coeff;
            self.hashcount += i32::from(entry.term_frequency());
        }
        self.length = self.length.sqrt();
    }

    /// Cosine similarity against `other`, filling `data` (`compare`).
    ///
    /// Returns 0 when the combined norm is zero or non-finite. The merge walks both arrays
    /// once; the shared part of a hash contributes the **smaller** term frequency
    /// weighted by that side's coefficient, exactly as Ghidra accumulates it.
    pub fn compare(&self, other: &LshCosineVector, data: &mut VectorCompare) -> f64 {
        let mut res = 0.0;
        let mut intersectcount = 0;
        let mut iter = 0usize;
        let mut iter2 = 0usize;
        let norm = self.length * other.length;
        if norm.is_finite() && norm > 0.0 {
            while iter < self.entries.len() && iter2 < other.entries.len() {
                let hash1 = self.entries[iter].hash;
                let hash2 = other.entries[iter2].hash;
                if hash1 == hash2 {
                    let t1 = i32::from(self.entries[iter].term_frequency());
                    let t2 = i32::from(other.entries[iter2].term_frequency());
                    if t1 < t2 {
                        let w1 = self.entries[iter].coeff;
                        res += w1 * w1;
                        intersectcount += t1;
                    } else {
                        let w2 = other.entries[iter2].coeff;
                        res += w2 * w2;
                        intersectcount += t2;
                    }
                    iter += 1;
                    iter2 += 1;
                } else if (hash1 as u32) < (hash2 as u32) {
                    iter += 1;
                } else {
                    iter2 += 1;
                }
            }
            data.dotproduct = res;
            res /= norm;
        } else {
            data.dotproduct = res;
        }
        data.intersectcount = intersectcount;
        data.acount = self.hashcount;
        data.bcount = other.hashcount;
        res
    }

    /// The counts-only comparison (`compareCounts`), without the dot product.
    pub fn compare_counts(&self, other: &LshCosineVector, data: &mut VectorCompare) {
        let mut intersectcount = 0;
        let mut iter = 0usize;
        let mut iter2 = 0usize;
        while iter < self.entries.len() && iter2 < other.entries.len() {
            let hash1 = self.entries[iter].hash;
            let hash2 = other.entries[iter2].hash;
            if hash1 == hash2 {
                let t1 = i32::from(self.entries[iter].term_frequency());
                let t2 = i32::from(other.entries[iter2].term_frequency());
                intersectcount += t1.min(t2);
                iter += 1;
                iter2 += 1;
            } else if (hash1 as u32) < (hash2 as u32) {
                iter += 1;
            } else {
                iter2 += 1;
            }
        }
        data.intersectcount = intersectcount;
        data.acount = self.hashcount;
        data.bcount = other.hashcount;
    }
}

/// The significance of a match, normalised for these weights
/// (`LSHVectorFactory.calculateSignificance`).
///
/// Similarity says how alike two vectors are; significance says how unlikely that
/// agreement is by chance. It starts from the dot product and subtracts two
/// penalties: every hash that had to **flip** (present on one side only) costs
/// `flip_norm0 + flip_norm1 / max`, and every hash of **size difference** costs
/// `diff_norm0 + diff_norm1 / max`; `addend` corrects the final scale.
///
/// `data.fill_out()` is applied here so a caller may pass the raw comparison.
pub fn calculate_significance(factory: &WeightFactory, data: &mut VectorCompare) -> f64 {
    data.fill_out();
    let max = f64::from(data.max.max(1));
    data.dotproduct
        - f64::from(data.numflip) * (factory.flip_norm0() + factory.flip_norm1() / max)
        - f64::from(data.diff) * (factory.diff_norm0() + factory.diff_norm1() / max)
        + factory.addend()
}

/// A vector's significance against itself (`getSelfSignificance`).
pub fn self_significance(factory: &WeightFactory, vector: &LshCosineVector) -> f64 {
    let length = vector.length();
    length * length + factory.addend()
}

/// One scored candidate: which corpus function matched, how similar, and how
/// significant.
#[derive(Clone, Debug, PartialEq)]
pub struct BsimMatch {
    /// The candidate's name in the corpus.
    pub name: String,
    /// The candidate's address in its own executable.
    pub address: u64,
    /// The executable the candidate came from.
    pub executable: String,
    /// Cosine similarity in `0..=1`.
    pub similarity: f64,
    /// BSim's significance for this pair.
    pub significance: f64,
}

/// Score `query` against one candidate vector, returning `None` when the
/// candidate fails `min_similarity`.
///
/// A candidate scoring below the floor is not a match, and Ghidra applies the
/// floor before the match limit so a better match from elsewhere is not crowded
/// out by a worse one.
pub fn score_candidate(
    factory: &WeightFactory,
    query: &LshCosineVector,
    candidate: &LshCosineVector,
    min_similarity: f64,
) -> Option<(f64, f64)> {
    let norm = query.length * candidate.length;
    if !min_similarity.is_finite() || !norm.is_finite() || norm <= 0.0 {
        return None;
    }
    let mut data = VectorCompare::default();
    let similarity = query.compare(candidate, &mut data);
    if !similarity.is_finite() {
        return None;
    }
    // Rounded norms can put an exact match just below 1. Check entry equality
    // only if the merge found every occurrence on both sides.
    let similarity = if similarity < 1.0
        && data.intersectcount == data.acount
        && data.intersectcount == data.bcount
        && query.entries == candidate.entries
    {
        1.0
    } else {
        similarity.clamp(0.0, 1.0)
    };
    if similarity < min_similarity {
        return None;
    }
    let significance = calculate_significance(factory, &mut data);
    significance.is_finite().then_some((similarity, significance))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A factory whose weights are all 1, so coefficients equal term frequency
    /// handling and the arithmetic is checkable by hand.
    fn unit_factory() -> WeightFactory {
        WeightFactory::with_unit_weights_for_tests()
    }

    fn empty_idf() -> IDFLookup {
        IDFLookup::default()
    }

    #[test]
    fn repeated_hashes_become_one_entry_with_their_term_frequency() {
        let f = unit_factory();
        let v = LshCosineVector::from_features(&[1, 1, 1, 2], &f, &empty_idf());
        assert_eq!(v.num_entries(), 2);
        assert_eq!(v.entries()[0].hash, 1);
        assert_eq!(v.entries()[0].term_frequency(), 3);
        assert_eq!(v.entries()[1].term_frequency(), 1);
        assert_eq!(v.hashcount(), 4);
    }

    #[test]
    fn an_identical_vector_compares_to_one() {
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[1, 2, 3], &f, &empty_idf());
        let b = a.clone();
        let mut data = VectorCompare::default();
        let sim = a.compare(&b, &mut data);
        assert!((sim - 1.0).abs() < 1e-12, "self similarity {sim}");
        assert_eq!(data.intersectcount, 3);
        data.fill_out();
        assert_eq!(data.numflip, 0);
        assert_eq!(data.diff, 0);
    }

    #[test]
    fn disjoint_vectors_share_nothing() {
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[1, 2], &f, &empty_idf());
        let b = LshCosineVector::from_features(&[3, 4], &f, &empty_idf());
        let mut data = VectorCompare::default();
        let sim = a.compare(&b, &mut data);
        assert!(sim.abs() < 1e-12, "disjoint similarity {sim}");
        assert_eq!(data.intersectcount, 0);
        data.fill_out();
        // Both vectors have two hashes, so `max - min` is 0; every hash in the
        // smaller vector is unshared, so all of them "flip".
        assert_eq!(data.diff, 0);
        assert_eq!(data.numflip, 2);
    }

    #[test]
    fn an_empty_vector_compares_to_zero() {
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[1], &f, &empty_idf());
        let empty = LshCosineVector::new();
        let mut data = VectorCompare::default();
        assert_eq!(a.compare(&empty, &mut data), 0.0);
        assert_eq!(empty.compare(&a, &mut data), 0.0);
        assert_eq!(empty.num_entries(), 0);
    }

    #[test]
    fn zero_and_underflowed_norms_are_not_matches_even_at_zero_floor() {
        let f = unit_factory();
        let normal = LshCosineVector::from_features(&[1], &f, &empty_idf());
        for coefficient in [0.0, 1e-200] {
            let degenerate = LshCosineVector::from_entries(vec![HashEntry::with_weight(1, 1, coefficient)]);
            assert!(score_candidate(&f, &degenerate, &normal, 0.0).is_none());
            assert!(score_candidate(&f, &normal, &degenerate, 0.0).is_none());
        }
        assert!(score_candidate(&f, &normal, &normal, f64::NAN).is_none());
    }

    #[test]
    fn the_merge_orders_hashes_as_unsigned() {
        // `-1` is the LARGEST 32-bit value unsigned, though the smallest signed.
        // A feature list is therefore expected sorted by UNSIGNED hash, which is
        // how Ghidra stores it (`getSignatureVector` sorts the u32 values).
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[5, u32::MAX], &f, &empty_idf());
        let b = LshCosineVector::from_features(&[5], &f, &empty_idf());
        let mut data = VectorCompare::default();
        a.compare(&b, &mut data);
        // Only 5 intersects: -1 is above 5 in unsigned order, so the walk passes
        // it without a spurious match.
        assert_eq!(data.intersectcount, 1);
    }

    #[test]
    fn term_frequency_is_clamped_at_sixty_three() {
        let f = unit_factory();
        let features = vec![7u32; 100];
        let v = LshCosineVector::from_features(&features, &f, &empty_idf());
        assert_eq!(v.num_entries(), 1);
        // Java stores `min(63, count-1)` and adds one back.
        assert_eq!(v.entries()[0].term_frequency(), 64);
    }

    #[test]
    fn compare_counts_agrees_with_compare_on_the_intersection() {
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[1, 1, 2], &f, &empty_idf());
        let b = LshCosineVector::from_features(&[1, 2, 2], &f, &empty_idf());
        let mut with_dot = VectorCompare::default();
        a.compare(&b, &mut with_dot);
        let mut counts = VectorCompare::default();
        a.compare_counts(&b, &mut counts);
        // Intersection counts multiplicity: min(2,1) + min(1,2) = 2.
        assert_eq!(counts.intersectcount, 2);
        assert_eq!(with_dot.intersectcount, counts.intersectcount);
        assert_eq!(counts.acount, with_dot.acount);
        assert_eq!(counts.bcount, with_dot.bcount);
    }

    #[test]
    fn an_identical_pair_is_maximally_significant() {
        let f = unit_factory();
        let idf = empty_idf();
        for features in [&[1, 2][..], &[1, 2, 3][..]] {
            let a = LshCosineVector::from_features(features, &f, &idf);
            let b = a.clone();
            let (sim, sig) = score_candidate(&f, &a, &b, 1.0).expect("exact match");
            assert_eq!(sim, 1.0);
            assert!(score_candidate(&f, &a, &b, 1.0 + f64::EPSILON).is_none());
            // No flips or size difference: significance is the dot product plus the addend.
            assert_eq!(sig, features.len() as f64 + f.addend());
        }

        let a = LshCosineVector::from_features(&[1, 2, 3], &f, &idf);
        let b = LshCosineVector::from_features(&[1, 2, 4], &f, &idf);
        assert!(score_candidate(&f, &a, &b, 1.0).is_none());
    }

    #[test]
    fn a_low_similarity_candidate_is_refused_by_the_floor() {
        let f = unit_factory();
        let a = LshCosineVector::from_features(&[1, 2, 3], &f, &empty_idf());
        let b = LshCosineVector::from_features(&[100, 200, 300], &f, &empty_idf());
        assert!(score_candidate(&f, &a, &b, 0.7).is_none());
        // At a zero floor the pair still scores, with a finite similarity.
        let (sim, _) = score_candidate(&f, &a, &b, 0.0).expect("zero floor accepts");
        assert!(sim.abs() < 1e-12);
    }

    #[test]
    fn flips_and_size_difference_both_reduce_significance() {
        // Penalties that dominate: with a non-zero addend and non-zero flip cost,
        // a partial match must score below an identical one.
        let mut f = unit_factory();
        f.set_penalties_for_tests(1.0, 1.0, 1.0, 1.0, 0.0);
        let base = LshCosineVector::from_features(&[1, 2, 3, 4, 5, 6], &f, &empty_idf());
        let near = LshCosineVector::from_features(&[1, 2, 3, 4, 5, 7], &f, &empty_idf());
        let exact = self_significance(&f, &base);
        let (_, partial) = score_candidate(&f, &base, &near, 0.0).expect("partial scores");
        assert!(partial < exact, "partial {partial} should be < exact {exact}");
    }

    #[test]
    fn significance_uses_the_smaller_of_the_two_counts_for_the_penalty_denominator() {
        let mut f = unit_factory();
        f.set_penalties_for_tests(0.0, 0.0, 0.5, 0.0, 0.0);
        let big = LshCosineVector::from_features(&[1, 2, 3, 4], &f, &empty_idf());
        let small = LshCosineVector::from_features(&[1, 2], &f, &empty_idf());
        let mut data = VectorCompare::default();
        big.compare(&small, &mut data);
        data.fill_out();
        // `diff` is max - min = 2, and the penalty divides by max = 4.
        assert_eq!(data.diff, 2);
        assert_eq!(data.max, 4);
        let sig = calculate_significance(&f, &mut data);
        assert!((sig - (data.dotproduct - 2.0 * 0.5)).abs() < 1e-12, "sig {sig}");
    }
}
