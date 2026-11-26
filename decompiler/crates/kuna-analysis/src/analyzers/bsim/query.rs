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

use kuna_base::error::{KunaError, KunaResult};

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
/// Both the weights and query signature must agree with the corpus settings.
pub fn query(
    corpus: &BsimCorpus,
    weights: &LshWeights,
    query: &BsimSignature,
    options: &QueryOptions,
) -> KunaResult<QueryResult> {
    if !options.min_similarity.is_finite() {
        return Err(KunaError::bad_data("bsim query: similarity floor must be finite"));
    }
    corpus.check_weights(weights)?;
    if query.settings != corpus.settings() {
        return Err(KunaError::bad_data(format!(
            "bsim query: signature settings {:#x} do not match corpus settings {:#x}",
            query.settings, corpus.settings(),
        )));
    }
    let query_vector = LshCosineVector::from_features(&query.features, &weights.factory, &weights.idf);
    if !query_vector.length().is_finite() || query_vector.length() == 0.0 {
        return Err(KunaError::bad_data("bsim query: selected function has no scoreable weight"));
    }
    let mut considered = 0usize;
    let mut scored: Vec<BsimMatch> = Vec::new();

    for (exec, candidate) in corpus.functions() {
        if !options.filters.admits(exec) {
            continue;
        }
        considered += 1;
        let candidate_vector = LshCosineVector::from_features(
            &candidate.features,
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
    Ok(QueryResult {
        matches: scored,
        considered,
        min_similarity: options.min_similarity,
        truncated,
    })
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
            settings: 0x49,
            name: name.into(),
            address,
            features: features.to_vec(),
            has_unimplemented: false,
            has_bad_data: false,
        }
    }

    fn corpus_with(functions: Vec<BsimSignature>) -> BsimCorpus {
        let mut corpus = BsimCorpus::new(0x49, super::super::lshweights::LshProfile::Size64).unwrap();
        corpus.insert_executable(CorpusExecutable {
            name: "app".into(),
            architecture: "x86:LE:64:default".into(),
            compiler: "gcc".into(),
            md5: "aa".into(),
            complete: true,
            functions,
        }).unwrap();
        corpus
    }

    #[test]
    fn an_identical_function_is_the_best_match() {
        let w = unit_weights();
        let corpus = corpus_with(vec![
            sig("other", 0x10, &[100, 200, 300]),
            sig("target", 0x20, &[1, 2, 3, 4]),
        ]);
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3, 4]), &QueryOptions::default()).unwrap();
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
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options).unwrap();
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
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3, 4, 5, 6]), &options).unwrap();
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
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options).unwrap();
        assert_eq!(result.matches.len(), 2);
        assert!(result.truncated);
    }

    #[test]
    fn metadata_filters_narrow_the_scope() {
        let w = unit_weights();
        let mut corpus = BsimCorpus::new(0x49, super::super::lshweights::LshProfile::Size64).unwrap();
        corpus.insert_executable(CorpusExecutable {
            name: "keep.exe".into(),
            architecture: "x86:LE:64:default".into(),
            compiler: "gcc".into(),
            md5: "aa".into(),
            complete: true,
            functions: vec![sig("f", 0x10, &[1, 2, 3])],
        }).unwrap();
        corpus.insert_executable(CorpusExecutable {
            name: "drop.exe".into(),
            architecture: "AARCH64:LE:64:v8A".into(),
            compiler: "msvc".into(),
            md5: "bb".into(),
            complete: true,
            functions: vec![sig("f", 0x20, &[1, 2, 3])],
        }).unwrap();
        let options = QueryOptions {
            min_similarity: 0.0,
            max_matches: 10,
            filters: MatchFilters { executable: Some("keep.exe".into()), ..Default::default() },
        };
        let result = query(&corpus, &w, &sig("q", 0, &[1, 2, 3]), &options).unwrap();
        assert_eq!(result.considered, 1);
        assert_eq!(result.matches.len(), 1);
        assert_eq!(result.matches[0].executable, "keep.exe");
    }

    #[test]
    fn settings_mismatches_are_errors_not_empty_searches() {
        let mut w = unit_weights();
        let corpus = corpus_with(vec![sig("f", 0x10, &[1, 2, 3])]);
        let mut q = sig("q", 0, &[1, 2, 3]);
        let options = QueryOptions::default();
        w.settings = 0x4d;
        let err = query(&corpus, &w, &q, &options).unwrap_err();
        assert!(err.explain().contains("stored settings 0x49"));
        w.settings = 0x49;
        q.settings = 0x4d;
        let err = query(&corpus, &w, &q, &options).unwrap_err();
        assert!(err.explain().contains("signature settings 0x4d"));
    }
}
