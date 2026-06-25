//! Unit tests for the `dedupvardecls` option parser and the `DeclDedup` helper.

use super::*;

#[test]
fn apply_parses_on_off() {
    let (val, msg) = OptionDedupVarDecls.apply("on").unwrap();
    assert!(val);
    assert!(msg.contains("on"));
    let (val, msg) = OptionDedupVarDecls.apply("off").unwrap();
    assert!(!val);
    assert!(msg.contains("off"));
}

#[test]
fn apply_rejects_garbage() {
    assert!(OptionDedupVarDecls.apply("maybe").is_err());
}

#[test]
fn dedup_suppresses_only_repeats() {
    let mut d = DeclDedup::new();
    let sig = |t: &str, n: &str, off: u64| -> DeclSignature {
        (t.to_string(), n.to_string(), None, Some(("stack".to_string(), off)))
    };
    // First occurrence: emit (not a duplicate).
    assert!(!d.is_duplicate(sig("int4", "option_index", 0x3c)));
    // Identical signature: suppress.
    assert!(d.is_duplicate(sig("int4", "option_index", 0x3c)));
    assert!(d.is_duplicate(sig("int4", "option_index", 0x3c)));
}

#[test]
fn dedup_keeps_distinct_signatures() {
    let mut d = DeclDedup::new();
    let sig = |t: &str, n: &str, off: u64| -> DeclSignature {
        (t.to_string(), n.to_string(), None, Some(("stack".to_string(), off)))
    };
    // Same name + type, DIFFERENT storage slot -> distinct -> both emit.
    assert!(!d.is_duplicate(sig("int4", "v1", 0x10)));
    assert!(!d.is_duplicate(sig("int4", "v1", 0x20)));
    // Same name + slot, DIFFERENT type -> distinct -> both emit.
    assert!(!d.is_duplicate(sig("char *", "v2", 0x30)));
    assert!(!d.is_duplicate(sig("int8", "v2", 0x30)));
}

#[test]
fn dedup_handles_array_adornment() {
    let mut d = DeclDedup::new();
    let s1: DeclSignature = ("int2".into(), "arr".into(), Some(("int2".into(), 32)), None);
    let s2: DeclSignature = ("int2".into(), "arr".into(), Some(("int2".into(), 32)), None);
    let s3: DeclSignature = ("int2".into(), "arr".into(), Some(("int2".into(), 16)), None);
    assert!(!d.is_duplicate(s1));
    assert!(d.is_duplicate(s2)); // identical array -> suppress
    assert!(!d.is_duplicate(s3)); // different count -> keep
}
