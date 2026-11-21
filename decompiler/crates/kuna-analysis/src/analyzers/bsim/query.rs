//! Searching a corpus: which stored functions look like this one?
//!
//! This is BSim's query half. Given a query function's feature list, it builds a
//! weighted vector and scores it against every candidate in the corpus, returning
//! the matches above a similarity floor ordered best-first.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Features/BSim/src/main/java/ghidra/features/bsim/query/CompareSignatures.java`
//!   — the ordering and the match record BSim reports.
//! - `.../query/client/AbstractSQLFunctionDatabase.java` — the similarity floor
//!   is applied **before** the match limit, so a strong match from a later
//!   candidate is never crowded out by a weak one taken first.
//!
//! # What a match does and does not say
//!
//! Similarity is a measure of **shape**, not proof of the same source. A 1.0
//! match is the same vector, and two hand-written functions that compile alike
//! score high without being the same code. Every result therefore carries both
//! numbers — similarity and significance — so a reader can judge the evidence
//! rather than a verdict.

use super::corpus::BsimCorpus;
use super::lsh::{score_candidate, BsimMatch, LshCosineVector};
use super::lshweights::LshWeights;
use super::signature::BsimSignature;

/// BSim's default similarity floor for "this is a match".
pub const DEFAULT_SIMILARITY_FLOOR: f64 = 0.7;

/// The default number of matches a query returns.
pub const DEFAULT_MAX_MATCHES: usize = 10;

/// Filters that narrow the search before scoring.
///
/// They match **exact** metadata values and combine with AND, as BSim's native
/// filters do: a filter is a restriction of the scope, never a broadening of it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MatchFilters {
    /// Keep only candidates from this executable name, when set.
    pub executable: Option<String>,
    /// Keep only candidates with this architecture, when set.
    pub architecture: Option<String>,
    /// Keep only candidates with this compiler, when set.
    pub compiler: Option<String>,
}

impl MatchFilters {
    /// True when no filter is set.
    pub fn is_empty(&self) -> bool {
        self.executable.is_none() && self.architecture.is_none() && self.compiler.is_none()
    }

    /// True when the executable record passes these filters.
    pub fn admits(&self, exec: &super::corpus::CorpusExecutable) -> bool {
        self.executable.as_ref().is_none_or(|v| &exec.name == v)
            && self.architecture.as_ref().is_none_or(|v| &exec.architecture == v)
            && self.compiler.as_ref().is_none_or(|v| &exec.compiler == v)
    }
}

/// A query: the function to look for, and the bounds of the search.
#[derive(Clone, Debug)]
pub struct QueryOptions {
    /// Minimum cosine similarity for a match.
    pub min_similarity: f64,
    /// Maximum matches to return.
    pub max_matches: usize,
    /// Metadata scope.
    pub filters: MatchFilters,
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions {
            min_similarity: DEFAULT_SIMILARITY_FLOOR,
            max_matches: DEFAULT_MAX_MATCHES,
            filters: MatchFilters::default(),
        }
    }
}

/// What a query found, with the scope it searched.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryResult {
    /// The matches, best first.
    pub matches: Vec<BsimMatch>,
    /// How many corpus functions were **considered** (passed the filters).
    pub considered: usize,
    /// How many were skipped because their stored settings differ.
    pub skipped_settings: usize,
    /// The floor that was applied.
    pub min_similarity: f64,
    /// True when more matches qualified than `max_matches` returned.
    pub truncated: bool,
}

impl QueryResult {
    /// One sentence when nothing matched, naming the floor so "not in the
    /// corpus" and "searched with a high floor" are never confused.
    pub fn empty_reason(&self, min_similarity: f64) -> Option<String> {
        if !self.matches.is_empty() {
            return None;
        }
        Some(format!(
            "no stored function is at least {min_similarity:.2} similar (threshold used); \
             {0} function(s) were considered",
            self.considered
        ))
    }
}

/// Query `corpus` for functions resembling `query`.
///
/// `weights` must be the table matching the corpus's settings and language; a
/// corpus whose settings differ from `weights` cannot be compared, and every such
/// candidate is counted in [`QueryResult::skipped_settings`] rather than scored
/// against the wrong scale.
pub fn query(
    corpus: &BsimCorpus,
    weights: &LshWeights,
    query: &BsimSignature,
    options: &QueryOptions,
) -> QueryResult {
    let query_features = as_signed(&query.features);
    let query_vector =
        LshCosineVector::from_features(&query_features, &weights.factory, &weights.idf);
    let mut considered = 0usize;
    let mut skipped_settings = 0usize;
    let mut scored: Vec<BsimMatch> = Vec::new();
    let settings_match = corpus.settings() == weights.settings as u32;

    for (exec, candidate) in corpus.functions() {
        if !options.filters.admits(exec) {
            continue;
        }
        considered += 1;
        if !settings_match {
            skipped_settings += 1;
            continue;
        }
        let candidate_features = as_signed(&candidate.features);
        let candidate_vector = LshCosineVector::from_features(
            &candidate_features,
            &weights.factory,
            &weights.idf,
        );
        let Some((similarity, significance)) =
            score_candidate(&weights.factory, &query_vector, &candidate_vector, options.min_similarity)
        else {
            continue;
        };
        scored.push(BsimMatch {
            name: candidate.name.clone(),
            address: candidate.address,
            executable: exec.name.clone(),
            similarity,
            significance,
        });
    }

    // Best first: similarity, then significance, then the identity fields so the
    // order is deterministic when two candidates tie on both numbers.
    scored.sort_by(|a, b| {
        b.similarity
            .total_cmp(&a.similarity)
            .then_with(|| b.significance.total_cmp(&a.significance))
            .then_with(|| a.executable.cmp(&b.executable))
            .then_with(|| a.address.cmp(&b.address))
    });
    let truncated = scored.len() > options.max_matches;
    scored.truncate(options.max_matches);
    QueryResult {
        matches: scored,
        considered,
        skipped_settings,
        min_similarity: options.min_similarity,
        truncated,
    }
}

/// Convert a feature list to the signed `i32` hashes the vector model uses.
///
/// The signature generator sorts `u32` hashes ascending and Ghidra's
/// `LSHCosineVector` compares them as unsigned, so the **bit pattern** is what
/// matters; the value is carried as `i32` because that is what the Java code's
/// `int` arithmetic does. `as i32` is a bit-preserving reinterpretation.
pub fn as_signed(features: &[u32]) -> Vec<i32> {
    features.iter().map(|&f| f as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::bsim::corpus::CorpusExecutable;
    use crate::analyzers::bsim::lshweights::{IDFLookup, LshWeights, WeightFactory};
    use std::rc::Rc;

    fn unit_weights() -> LshWeights {
        LshWeights {
            settings: 0x49,
            factory: Rc::new(WeightFactory::with_unit_weights_for_tests()),
            idf: Rc::new(IDFLookup::default()),
        }
    }

    fn sig(name: &str, address: u64, features: &[u32]) -> BsimSignature {
        BsimSignature {
            name: name.into(),
            address,
            features: features.to_vec(),
            has_unimplemented: false,
            has_bad_data: false,
        }
    }

    fn corpus_with(functions: Vec<BsimSignature>) -> BsimCorpus {
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(CorpusExecutable {
            name: "app".into(),
            architecture: "x86:LE:64:default".into(),
            compiler: "gcc".into(),
            md5: "aa".into(),
            functions,
        });
        corpus
    }

    #[test]
    fn an_identical_function_is_the_best_match() {
        let w = unit_weights();
        let corpus = corpus_with(vec![
            sig("other", 0x10, &[100, 200, 300]),
            sig("target", 0x20, &[1, 2, 3, 4]),
        ]);
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3, 4]), &QueryOptions::default());
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].name, "target");
        assert!((result.matches[0].similarity - 1.0).abs() < 1e-12);
        assert_eq!(result.considered, 2);
    }

    #[test]
    fn a_high_floor_reports_why_nothing_matched() {
        let w = unit_weights();
        let corpus = corpus_with(vec![sig("other", 0x10, &[100, 200, 300])]);
        let options = QueryOptions { min_similarity: 0.99, ..QueryOptions::default() };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options);
        assert!(result.matches.is_empty());
        let reason = result.empty_reason(0.99).expect("a reason");
        assert!(reason.contains("0.99"), "{reason}");
        assert!(reason.contains("considered"), "{reason}");
    }

    #[test]
    fn matches_are_ordered_best_first() {
        let w = unit_weights();
        let corpus = corpus_with(vec![
            sig("exact", 0x10, &[1, 2, 3, 4, 5, 6]),
            sig("close", 0x20, &[1, 2, 3, 4, 5, 7]),
            sig("far", 0x30, &[1, 2, 3, 40, 50, 60]),
        ]);
        let options = QueryOptions { min_similarity: 0.0, ..QueryOptions::default() };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3, 4, 5, 6]), &options);
        let names: Vec<&str> = result.matches.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, ["exact", "close", "far"]);
        assert!(result.matches[0].similarity >= result.matches[1].similarity);
        assert!(result.matches[1].similarity >= result.matches[2].similarity);
    }

    #[test]
    fn the_match_limit_reports_truncation() {
        let w = unit_weights();
        let corpus = corpus_with(
            (0..5u32).map(|i| sig(&format!("f{i}"), 0x10 + u64::from(i), &[1, 2, 3])).collect(),
        );
        let options = QueryOptions { min_similarity: 0.0, max_matches: 2, ..QueryOptions::default() };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options);
        assert_eq!(result.matches.len(), 2);
        assert!(result.truncated);
    }

    #[test]
    fn metadata_filters_narrow_the_scope() {
        let w = unit_weights();
        let mut corpus = BsimCorpus::new(0x49, "x86");
        corpus.insert_executable(CorpusExecutable {
            name: "keep.exe".into(),
            architecture: "x86:LE:64:default".into(),
            compiler: "gcc".into(),
            md5: "aa".into(),
            functions: vec![sig("f", 0x10, &[1, 2, 3])],
        });
        corpus.insert_executable(CorpusExecutable {
            name: "drop.exe".into(),
            architecture: "x86:LE:32:default".into(),
            compiler: "msvc".into(),
            md5: "bb".into(),
            functions: vec![sig("f", 0x20, &[1, 2, 3])],
        });
        let options = QueryOptions {
            min_similarity: 0.0,
            max_matches: 10,
            filters: MatchFilters { executable: Some("keep.exe".into()), ..Default::default() },
        };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options);
        assert_eq!(result.considered, 1);
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].executable, "keep.exe");
    }

    #[test]
    fn a_corpus_built_under_other_settings_is_not_scored() {
        let w = unit_weights();
        let mut corpus = corpus_with(vec![sig("f", 0x10, &[1, 2, 3])]);
        // Rebuild under a different settings value.
        let mut other = BsimCorpus::new(0x4d, "x86");
        for exec in corpus.executables() {
            other.insert_executable(exec.clone());
        }
        corpus = other;
        let options = QueryOptions { min_similarity: 0.0, ..QueryOptions::default() };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options);
        assert!(result.matches.is_empty());
        assert_eq!(result.skipped_settings, 1);
        assert_eq!(result.considered, 1);
    }

    #[test]
    fn features_convert_to_signed_preserving_the_bit_pattern() {
        let unsigned = [0u32, 1, 0x7fff_ffff, 0xffff_ffff];
        assert_eq!(as_signed(&unsigned), vec![0i32, 1, i32::MAX, -1]);
    }
}
