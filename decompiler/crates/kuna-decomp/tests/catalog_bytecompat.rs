//! Catalog API checks: valid JSON, complete registry coverage, and live values.

use std::collections::BTreeSet;

use kuna_decomp::kuna_phases::{
    emit_catalog_json, emit_catalog_json_one, kuna_num_settables, kuna_settable_by_index,
    lookup_settable,
};
use serde_json::Value;

fn catalog() -> Value {
    serde_json::from_str(&emit_catalog_json(|_| None)).expect("catalog must be valid JSON")
}

#[test]
fn full_catalog_lists_every_registered_settable_once() {
    let catalog = catalog();
    let rows = catalog.as_array().expect("catalog must be an array");
    let actual: BTreeSet<_> = rows
        .iter()
        .map(|row| row["option"].as_str().expect("option must be a string"))
        .collect();
    let expected: BTreeSet<_> = (0..kuna_num_settables())
        .map(|index| kuna_settable_by_index(index).option)
        .collect();
    assert_eq!(actual.len(), rows.len(), "duplicate catalog option");
    assert_eq!(actual, expected);
}

#[test]
fn static_catalog_rows_have_schema_without_current_values() {
    let catalog = catalog();
    for row in catalog.as_array().unwrap() {
        assert!(row["values"].is_array());
        assert!(row["default"].is_string());
        assert!(row["tier"].is_string());
        assert!(row["symptoms"].is_array());
        assert!(row["destructive_as_default"].is_boolean());
        assert!(row.get("current").is_none());
    }
}

#[test]
fn single_settable_rows_match_the_full_catalog() {
    let catalog = catalog();
    for row in catalog.as_array().unwrap() {
        let option = row["option"].as_str().unwrap();
        let one = emit_catalog_json_one(option, None).expect("registered option");
        let one: Value = serde_json::from_str(&one).expect("single row must be valid JSON");
        assert_eq!(&one, row, "different catalog data for {option}");
    }
}

#[test]
fn live_current_value_is_separate_from_the_shipped_default() {
    let one = emit_catalog_json_one("stackalloc", Some("on")).unwrap();
    let one: Value = serde_json::from_str(&one).unwrap();
    assert_eq!(one["current"], "on");
    assert_eq!(one["default"], "off");
}

#[test]
fn unknown_single_settable_is_none() {
    assert!(emit_catalog_json_one("not-a-real-option", None).is_none());
    assert!(lookup_settable("not-a-real-option").is_none());
}
