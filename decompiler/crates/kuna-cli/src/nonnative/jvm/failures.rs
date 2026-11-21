//! What the CLI reports about a method or class besides its text: the bodies
//! written inside it that the engine could not decompile, the engine's own
//! warnings, and whether a class was decompiled, failed or is hidden.
//!
//! Code printed as part of a method is that method's: a lambda, an anonymous
//! or local class, or one of that class's methods, written inside method M as
//! an error comment, fails M (`nested_errors`, and M's `error` names it) as
//! well as being its own record's failure when it has one. A failure outside
//! every method of a class (a field initializer's lambda, a member class the
//! engine could not write) is the class's. `decompile` prints each failure
//! once, under the innermost selected record that holds it. A method whose
//! `$VF:` comments the engine marks as errors (an assert it could not
//! resugar, a switch on an enum it could not simplify, ...) has complete text
//! that may be wrong: those comments are its `warnings`, which fail nothing.

use std::collections::{BTreeMap, BTreeSet};

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::kuna_driver::ClassResult;
use kuna_jvm::kuna_spans::BodyFailure;

use super::render::Record;
use super::run::Outcome;
use super::select::{java_name, Universe};
use crate::jsonfmt::Json;

/// A body written inside a method or class that could not be decompiled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Nested {
    /// `com.ex.Main.lambda$main$0(I)V`, or a class name.
    pub(crate) body: String,
    pub(crate) error: String,
}

impl Nested {
    fn from(failure: &BodyFailure) -> Nested {
        let class = java_name(&failure.class);
        let body = if failure.name.is_empty() {
            class
        } else {
            format!("{class}.{}{}", failure.name, failure.descriptor)
        };
        Nested {
            body,
            error: failure.error.clone(),
        }
    }

    pub(crate) fn all(failures: &[BodyFailure]) -> Vec<Nested> {
        failures.iter().map(Nested::from).collect()
    }

    pub(crate) fn json(list: &[Nested]) -> Json {
        Json::Array(
            list.iter()
                .map(|n| {
                    Json::Object(vec![
                        ("body".into(), Json::Str(n.body.clone())),
                        ("error".into(), Json::Str(n.error.clone())),
                    ])
                })
                .collect(),
        )
    }
}

/// A method's `error`: its own failure, else the bodies inside it that failed.
pub(crate) fn method_error(own: Option<&str>, nested: &[Nested]) -> Option<String> {
    if let Some(own) = own {
        return Some(own.to_string());
    }
    (!nested.is_empty()).then(|| {
        nested
            .iter()
            .map(|n| format!("in {}: {}", n.body, n.error))
            .collect::<Vec<_>>()
            .join("; ")
    })
}

/// Why `class`, written in `unit`, is an error comment instead of its source.
pub(crate) fn class_failure(unit: &DecompiledClass, class: &str) -> Option<String> {
    let span = unit
        .classes
        .iter()
        .find(|c| c.internal == class)
        .and_then(|c| c.error.clone());
    if unit.qualified_name == class {
        unit.error.clone().or(span)
    } else {
        span
    }
}

/// Why the unit as a whole is an error comment: its top-level class failed.
pub(crate) fn unit_failure(unit: &DecompiledClass) -> Option<String> {
    class_failure(unit, &unit.qualified_name)
}

/// The failures in `class`'s text outside its methods.
pub(crate) fn outside_methods(unit: &DecompiledClass, class: &str) -> Vec<Nested> {
    unit.classes
        .iter()
        .find(|c| c.internal == class)
        .map(|c| Nested::all(&c.nested))
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Status {
    /// The engine's source for the class (methods inside may still have failed).
    Decompiled,
    /// The engine wrote an error comment in its place, or nothing.
    Failed,
    /// Not in the source: the engine folds it away (a synthetic class).
    Hidden,
}

impl Status {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Status::Decompiled => "decompiled",
            Status::Failed => "failed",
            Status::Hidden => "hidden",
        }
    }
}

/// The delivered units, for finding the one a class is written in.
pub(crate) struct Units<'a> {
    outcome: &'a Outcome,
    by_name: BTreeMap<&'a str, &'a ClassResult>,
    missing: BTreeMap<&'a str, &'a str>,
}

impl<'a> Units<'a> {
    pub(crate) fn new(outcome: &'a Outcome) -> Units<'a> {
        Units {
            outcome,
            by_name: outcome.results.iter().map(|r| (r.qualified_name.as_str(), r)).collect(),
            missing: outcome.missing.iter().map(|(n, why)| (n.as_str(), why.as_str())).collect(),
        }
    }

    /// The delivered unit `class` is written in (the nearest class of its
    /// chain the engine took for a top-level class), or why there is none.
    pub(crate) fn find(&self, universe: &Universe, class: &str) -> Result<&'a ClassResult, String> {
        let chain = universe.chain(class);
        if let Some(found) = chain.iter().find_map(|name| self.by_name.get(name.as_str())) {
            return Ok(found);
        }
        let root = chain.last().map(String::as_str).unwrap_or(class);
        Err(self
            .missing
            .get(root)
            .map(|why| why.to_string())
            .or_else(|| self.outcome.error.clone())
            .unwrap_or_else(|| "no output".into()))
    }

    /// The name of the unit `class` is written in, delivered or not.
    pub(crate) fn unit_name(&self, universe: &Universe, class: &str) -> String {
        match self.find(universe, class) {
            Ok(result) => result.qualified_name.clone(),
            Err(_) => universe.root_of(class),
        }
    }

    /// The bytes of the class files each unit is written from.
    pub(crate) fn sizes(&self, universe: &Universe) -> BTreeMap<String, u64> {
        let mut sizes = BTreeMap::new();
        for class in &universe.classes {
            *sizes.entry(self.unit_name(universe, &class.internal)).or_insert(0) += class.size as u64;
        }
        sizes
    }

    /// The delivered units holding one of `classes`; every one for `None`.
    pub(crate) fn holding(&self, universe: &Universe, classes: Option<&BTreeSet<String>>) -> Vec<&'a ClassResult> {
        match classes {
            None => self.outcome.results.iter().collect(),
            Some(classes) => {
                let wanted: BTreeSet<String> = classes
                    .iter()
                    .filter_map(|c| self.find(universe, c).ok())
                    .map(|r| r.qualified_name.clone())
                    .collect();
                self.outcome
                    .results
                    .iter()
                    .filter(|r| wanted.contains(&r.qualified_name))
                    .collect()
            }
        }
    }
}

/// Why `result` has no decompiled source, if it has none.
pub(crate) fn result_failure(result: &ClassResult) -> Option<String> {
    match &result.unit {
        Some(unit) => unit_failure(unit),
        None => Some(result.error.clone().unwrap_or_else(|| "no output".into())),
    }
}

/// `warning:` lines for `records` and for `class`'s text outside its
/// methods: each failed body once, under its own record when that is
/// selected, else under the first record that holds it; then each record's
/// engine warnings.
pub(crate) fn warn(records: &[Record], class: Option<(&str, &[Nested])>) {
    let own: BTreeSet<String> = records
        .iter()
        .filter(|r| r.own_error.is_some())
        .map(Record::signature)
        .collect();
    let mut told: BTreeSet<String> = BTreeSet::new();
    for record in records {
        let signature = record.signature();
        if let Some(error) = &record.own_error {
            eprintln!("warning: {signature}: {error}");
        }
        for nested in &record.nested {
            if !own.contains(&nested.body) && told.insert(nested.body.clone()) {
                eprintln!("warning: {signature}: in {}: {}", nested.body, nested.error);
            }
        }
        for warning in &record.warnings {
            eprintln!("warning: {signature}: {warning}");
        }
    }
    if let Some((name, nested)) = class {
        for n in nested {
            if !own.contains(&n.body) && told.insert(n.body.clone()) {
                eprintln!("warning: {}: in {}: {}", java_name(name), n.body, n.error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn failure(class: &str, name: &str, descriptor: &str) -> BodyFailure {
        BodyFailure {
            class: class.into(),
            name: name.into(),
            descriptor: descriptor.into(),
            error: "boom".into(),
        }
    }

    #[test]
    fn nested_bodies_are_named_like_methods_or_classes() {
        let lambda = Nested::from(&failure("com/ex/Main", "lambda$main$0", "(I)V"));
        assert_eq!(lambda.body, "com.ex.Main.lambda$main$0(I)V");
        assert_eq!(Nested::from(&failure("com/ex/Main$1", "", "")).body, "com.ex.Main$1");
    }

    #[test]
    fn a_method_error_is_its_own_else_the_nested_ones() {
        let nested = Nested::all(&[failure("a/B", "f", "()V"), failure("a/B$1", "", "")]);
        assert_eq!(method_error(Some("own"), &nested).as_deref(), Some("own"));
        assert_eq!(
            method_error(None, &nested).as_deref(),
            Some("in a.B.f()V: boom; in a.B$1: boom")
        );
        assert_eq!(method_error(None, &[]), None);
    }
}
