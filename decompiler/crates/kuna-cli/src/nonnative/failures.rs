//! What the CLI reports about a member or type besides its text: the bodies
//! written inside it that the engine could not decompile, the engine's own
//! warnings, and whether a type was decompiled, failed or is hidden.
//!
//! Code printed as part of a member is that member's: a lambda, an anonymous
//! or local class, or one of that class's methods, written inside method M as
//! an error comment, fails M (`nested_errors`, and M's `error` names it) as
//! well as being its own record's failure when it has one. A failure outside
//! every member of a type (a field initializer's lambda, a member class the
//! engine could not write) is the type's. `decompile` prints each failure
//! once, under the innermost selected record that holds it. A member whose
//! text the engine flags (an assert it could not resugar, a switch on an enum
//! it could not simplify, ...) has complete text that may be wrong: those
//! comments are its `warnings`, which fail nothing.

use std::collections::{BTreeMap, BTreeSet};

use kuna_managed::{Failure, Inventory, Unit};

use super::render::Record;
use super::run::Outcome;
use crate::jsonfmt::Json;

pub(crate) fn nested_json(list: &[Failure]) -> Json {
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

/// A member's `error`: its own failure, else the bodies inside it that failed.
pub(crate) fn member_error(own: Option<&str>, nested: &[Failure]) -> Option<String> {
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

/// Why `name`, written in `unit`, is an error comment instead of its source.
pub(crate) fn type_failure(unit: &Unit, name: &str) -> Option<String> {
    let span = unit.types.iter().find(|t| t.name == name).and_then(|t| t.error.clone());
    if unit.name == name {
        unit.error.clone().or(span)
    } else {
        span
    }
}

/// Why `unit` has no decompiled source: no text, or its root type failed.
pub(crate) fn unit_failure(unit: &Unit) -> Option<String> {
    match unit.text {
        Some(_) => type_failure(unit, &unit.name),
        None => Some(unit.error.clone().unwrap_or_else(|| "no output".into())),
    }
}

/// The failures in `name`'s text outside its members.
pub(crate) fn outside_members(unit: &Unit, name: &str) -> Vec<Failure> {
    unit.types.iter().find(|t| t.name == name).map(|t| t.nested.clone()).unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Status {
    /// The engine's source for the type (members inside may still have failed).
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

/// The delivered units, for finding the one a type is written in.
pub(crate) struct Units<'a> {
    outcome: &'a Outcome,
    by_name: BTreeMap<&'a str, &'a Unit>,
    missing: BTreeMap<&'a str, &'a str>,
}

impl<'a> Units<'a> {
    pub(crate) fn new(outcome: &'a Outcome) -> Units<'a> {
        Units {
            outcome,
            by_name: outcome.results.iter().map(|r| (r.name.as_str(), r)).collect(),
            missing: outcome.missing.iter().map(|(n, why)| (n.as_str(), why.as_str())).collect(),
        }
    }

    /// The delivered unit `name` is written in (the nearest type of its
    /// chain the engine took for a root), or why there is none.
    pub(crate) fn find(&self, inventory: &Inventory, name: &str) -> Result<&'a Unit, String> {
        let chain = inventory.chain(name);
        if let Some(found) = chain.iter().find_map(|name| self.by_name.get(name.as_str())) {
            return Ok(found);
        }
        let root = chain.last().map(String::as_str).unwrap_or(name);
        Err(self
            .missing
            .get(root)
            .map(|why| why.to_string())
            .or_else(|| self.outcome.error.clone())
            .unwrap_or_else(|| "no output".into()))
    }

    /// The name of the unit `name` is written in, delivered or not.
    pub(crate) fn unit_name(&self, inventory: &Inventory, name: &str) -> String {
        match self.find(inventory, name) {
            Ok(unit) => unit.name.clone(),
            Err(_) => inventory.root_of(name),
        }
    }

    /// The bytes of the type definitions each unit is written from.
    pub(crate) fn sizes(&self, inventory: &Inventory) -> BTreeMap<String, u64> {
        let mut sizes = BTreeMap::new();
        for t in &inventory.types {
            *sizes.entry(self.unit_name(inventory, &t.name)).or_insert(0) += t.size;
        }
        sizes
    }

    /// The delivered units holding one of `names`; every one for `None`.
    pub(crate) fn holding(&self, inventory: &Inventory, names: Option<&BTreeSet<String>>) -> Vec<&'a Unit> {
        match names {
            None => self.outcome.results.iter().collect(),
            Some(names) => {
                let wanted: BTreeSet<String> = names
                    .iter()
                    .filter_map(|c| self.find(inventory, c).ok())
                    .map(|r| r.name.clone())
                    .collect();
                self.outcome.results.iter().filter(|r| wanted.contains(&r.name)).collect()
            }
        }
    }
}

/// `warning:` lines for `records` and for type `outside`'s text outside its
/// members: each failed body once, under its own record when that is
/// selected, else under the first record that holds it; then each record's
/// engine warnings.
pub(crate) fn warn(records: &[Record], outside: Option<(String, &[Failure])>) {
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
    if let Some((name, nested)) = outside {
        for n in nested {
            if !own.contains(&n.body) && told.insert(n.body.clone()) {
                eprintln!("warning: {name}: in {}: {}", n.body, n.error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_member_error_is_its_own_else_the_nested_ones() {
        let failure = |body: &str| Failure { body: body.into(), error: "boom".into() };
        let nested = [failure("a.B.f()V"), failure("a.B$1")];
        assert_eq!(member_error(Some("own"), &nested).as_deref(), Some("own"));
        assert_eq!(member_error(None, &nested).as_deref(), Some("in a.B.f()V: boom; in a.B$1: boom"));
        assert_eq!(member_error(None, &[]), None);
    }
}
