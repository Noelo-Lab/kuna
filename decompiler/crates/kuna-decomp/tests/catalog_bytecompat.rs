//! Catalog lookup error handling. JSON formatting and registry prose are not snapshots.

use kuna_decomp::kuna_phases::{emit_catalog_json_one, lookup_settable};

#[test]
fn unknown_single_settable_is_none() {
    // C++ throws IfaceExecutionError for an unknown option; the port returns None.
    assert!(emit_catalog_json_one("not-a-real-option", None).is_none());
    assert!(lookup_settable("not-a-real-option").is_none());
}
