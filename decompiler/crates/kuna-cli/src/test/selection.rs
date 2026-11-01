//! What a `kuna test` run was asked to execute versus what the harness could run.
//!
//! The harness takes a datatest name only when it is the file name of an `*.xml`
//! directly in the corpus directory and drops any other without a word, and its
//! output names the tests inside each file rather than the files, so the
//! file-level selection is resolved here by the same rule before the run.  Unit
//! tests are named in the harness's own output, so their selection is read back
//! from it afterwards.  Every selection error makes the run exit 2.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::jsonfmt::Json;

pub(super) struct Data {
    corpus: PathBuf,
    requested: Vec<String>,
    matched: Vec<String>,
    missing: Vec<String>,
    unreadable: Option<String>,
}

pub(super) struct Unit {
    requested: Vec<String>,
    matched: Vec<String>,
    missing: Vec<String>,
}

/// Resolve the datatest names (already `.xml`-suffixed) against `corpus` the way
/// `decomp_test_dbg`'s `gather_data_tests` does.  No names means the whole corpus.
pub(super) fn data(corpus: &Path, requested: &[String]) -> Data {
    let (files, unreadable) = match std::fs::read_dir(corpus) {
        Ok(entries) => (corpus_files(entries), None),
        Err(e) => (BTreeSet::new(), Some(e.to_string())),
    };
    let wanted: BTreeSet<&String> = requested.iter().collect();
    let matched = files
        .iter()
        .filter(|f| wanted.is_empty() || wanted.contains(f))
        .cloned()
        .collect();
    Data {
        corpus: corpus.to_path_buf(),
        requested: requested.to_vec(),
        matched,
        missing: first_seen(requested.iter().filter(|n| !files.contains(*n))),
        unreadable,
    }
}

fn corpus_files(entries: std::fs::ReadDir) -> BTreeSet<String> {
    entries
        .flatten()
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("xml"))
        .filter_map(|e| e.file_name().to_str().map(str::to_string))
        .collect()
}

/// The unit selection, from the names the harness reported running.  Without a
/// unit footer the run did not finish, so nothing is called missing.
pub(super) fn unit(requested: &[String], ran: &[String], finished: bool) -> Unit {
    let ran_set: BTreeSet<&String> = ran.iter().collect();
    let missing = if finished {
        first_seen(requested.iter().filter(|n| !ran_set.contains(n)))
    } else {
        Vec::new()
    };
    Unit {
        requested: requested.to_vec(),
        matched: first_seen(ran.iter()),
        missing,
    }
}

fn first_seen<'a>(names: impl Iterator<Item = &'a String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    names.filter(|n| seen.insert(*n)).cloned().collect()
}

impl Data {
    pub(super) fn errors(&self) -> Vec<String> {
        let dir = self.corpus.display();
        if let Some(e) = &self.unreadable {
            return vec![format!("cannot read datatest directory {dir}: {e}")];
        }
        if self.requested.is_empty() && self.matched.is_empty() {
            return vec![format!("no datatests (*.xml) in {dir}")];
        }
        self.missing
            .iter()
            .map(|n| format!("datatest '{n}' not found in {dir}"))
            .collect()
    }

    fn json(&self) -> Json {
        Json::Object(vec![
            (
                "corpus".into(),
                Json::Str(self.corpus.to_string_lossy().into_owned()),
            ),
            ("requested".into(), strings(&self.requested)),
            ("matched".into(), strings(&self.matched)),
            ("missing".into(), strings(&self.missing)),
        ])
    }
}

impl Unit {
    pub(super) fn errors(&self) -> Vec<String> {
        self.missing
            .iter()
            .map(|n| format!("unit test '{n}' not found in the harness registry"))
            .collect()
    }

    fn json(&self) -> Json {
        Json::Object(vec![
            ("requested".into(), strings(&self.requested)),
            ("matched".into(), strings(&self.matched)),
            ("missing".into(), strings(&self.missing)),
        ])
    }
}

/// The `selection` object of `kuna test --json`; `errors` empty <=> `complete`.
pub(super) fn json(data: Option<&Data>, unit: Option<&Unit>, errors: &[String]) -> Json {
    Json::Object(vec![
        ("complete".into(), Json::Bool(errors.is_empty())),
        ("errors".into(), strings(errors)),
        ("data".into(), data.map_or(Json::Null, Data::json)),
        ("unit".into(), unit.map_or(Json::Null, Unit::json)),
    ])
}

fn strings(v: &[String]) -> Json {
    Json::Array(v.iter().map(|s| Json::Str(s.clone())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus(files: &[&str]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "kuna-selection-{}-{}",
            std::process::id(),
            files.join("+").replace('/', "_")
        ));
        std::fs::create_dir_all(&dir).unwrap();
        for f in files {
            std::fs::write(dir.join(f), "").unwrap();
        }
        dir
    }

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_mixed_request_matches_what_exists_and_names_the_rest() {
        let dir = corpus(&["a.xml", "b.xml", "notes.txt"]);
        let sel = data(
            &dir,
            &names(&["b.xml", "gone.xml", "gone.xml", "notes.txt"]),
        );
        assert_eq!(sel.matched, names(&["b.xml"]));
        assert_eq!(sel.missing, names(&["gone.xml", "notes.txt"]));
        assert_eq!(sel.errors().len(), 2);
    }

    #[test]
    fn no_names_selects_the_whole_corpus_and_an_empty_one_is_an_error() {
        let dir = corpus(&["b.xml", "a.xml"]);
        let all = data(&dir, &[]);
        assert_eq!(all.matched, names(&["a.xml", "b.xml"]));
        assert!(all.errors().is_empty());

        let empty = corpus(&["readme.txt"]);
        assert_eq!(data(&empty, &[]).errors().len(), 1);
        let gone = data(&empty.join("absent"), &[]);
        assert!(gone.errors()[0].starts_with("cannot read datatest directory"));
    }

    #[test]
    fn unit_names_are_missing_only_once_the_run_finished() {
        let ran = names(&["alpha"]);
        let sel = unit(&names(&["alpha", "beta"]), &ran, true);
        assert_eq!(sel.matched, ran);
        assert_eq!(sel.missing, names(&["beta"]));
        assert!(unit(&names(&["beta"]), &[], false).errors().is_empty());
        assert!(unit(&[], &[], true).errors().is_empty());
    }
}
