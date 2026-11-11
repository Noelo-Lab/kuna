//! `--omit KEY[,KEY]...`: per-function record collections a `--json` caller
//! asked not to receive.
//!
//! An omitted collection is absent from every record and named in the document's
//! top-level `omitted` array, so a consumer can tell "not asked for" from an
//! examined collection that came back empty. Nothing is omitted by default, and
//! a document that omits nothing carries no `omitted` key.

use crate::jsonfmt::Json;

/// The collections a caller can withhold, spelled as their record keys.
pub(crate) const KEYS: [&str; 3] = ["line_mappings", "variables", "types"];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Omit {
    pub(crate) line_mappings: bool,
    /// Also withholds `stack_objects`, which `--no-vars` ties to the variables.
    pub(crate) variables: bool,
    pub(crate) types: bool,
}

impl Omit {
    /// Add one `--omit` value (comma-separated record keys) to this selection.
    pub(crate) fn add(&mut self, value: &str) -> Result<(), String> {
        let mut named = false;
        for key in value.split(',').map(str::trim).filter(|key| !key.is_empty()) {
            match key {
                "line_mappings" => self.line_mappings = true,
                "variables" => self.variables = true,
                "types" => self.types = true,
                other => {
                    return Err(format!(
                        "--omit {other:?}: expected a comma-separated list of {}",
                        KEYS.join(", ")
                    ))
                }
            }
            named = true;
        }
        if named {
            Ok(())
        } else {
            Err(format!("--omit needs at least one of {}", KEYS.join(", ")))
        }
    }

    pub(crate) fn is_empty(self) -> bool {
        self == Self::default()
    }

    /// The record keys this selection withholds, in record order.
    pub(crate) fn withheld(self) -> Vec<&'static str> {
        [
            (self.line_mappings, "line_mappings"),
            (self.variables, "variables"),
            (self.types, "types"),
            (self.variables, "stack_objects"),
        ]
        .into_iter()
        .filter_map(|(omitted, key)| omitted.then_some(key))
        .collect()
    }

    /// The document's top-level `omitted` array, `None` when nothing is withheld.
    pub(crate) fn json(self) -> Option<Json> {
        (!self.is_empty()).then(|| {
            Json::Array(self.withheld().into_iter().map(|key| Json::Str(key.to_string())).collect())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Omit;

    #[test]
    fn keys_accumulate_across_values_and_report_in_record_order() {
        let mut omit = Omit::default();
        omit.add("types").unwrap();
        omit.add(" variables , line_mappings").unwrap();
        assert_eq!(omit.withheld(), ["line_mappings", "variables", "types", "stack_objects"]);
    }

    #[test]
    fn unknown_and_empty_values_are_refused() {
        let err = Omit::default().add("variables,lines").unwrap_err();
        assert!(err.contains("\"lines\"") && err.contains("line_mappings"), "{err}");
        assert!(Omit::default().add(" , ").is_err());
    }

    #[test]
    fn nothing_omitted_means_no_marker() {
        assert!(Omit::default().json().is_none());
        let mut omit = Omit::default();
        omit.add("line_mappings").unwrap();
        assert_eq!(omit.withheld(), ["line_mappings"]);
    }
}
