//! Validate the passing-set used by the parity gate; other record metadata is optional.

use std::collections::BTreeSet;

#[derive(serde::Deserialize)]
struct Baseline {
    passing: BTreeSet<String>,
}

pub(super) fn passing(text: &str) -> serde_json::Result<BTreeSet<String>> {
    serde_json::from_str::<Baseline>(text).map(|baseline| baseline.passing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_set_semantics_and_ignores_metadata() {
        let actual = passing(r#"{"passing":["data:b","unit:a","data:b"],"returncode":0}"#).unwrap();
        assert_eq!(actual, BTreeSet::from(["data:b".into(), "unit:a".into()]));
        assert!(passing(r#"{"passing":[]}"#).unwrap().is_empty());
    }

    #[test]
    fn a_broken_record_cannot_become_an_empty_baseline() {
        for text in [
            "null",
            "[]",
            "{}",
            r#"{"passing":null}"#,
            r#"{"passing":"data:a"}"#,
            r#"{"passing":["data:a",1]}"#,
            r#"{"passing":[],"passing":["data:a"]}"#,
            r#"{"passing":[]} false"#,
        ] {
            assert!(passing(text).is_err(), "accepted {text}");
        }
    }
}
