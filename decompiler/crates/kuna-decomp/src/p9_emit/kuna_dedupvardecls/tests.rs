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

#[test]
fn declaration_names_are_unique_without_stealing_later_names() {
    let locals = ["word", "word", "word_1", "word"];
    let mut names = DeclNameUniquifier::new(locals, std::iter::empty());
    assert_eq!(names.unique("word"), "word");
    assert_eq!(names.unique("word"), "word_2");
    assert_eq!(names.unique("word_1"), "word_1");
    assert_eq!(names.unique("word"), "word_3");
}

#[test]
fn parameter_and_user_name_collisions_preserve_the_existing_owner() {
    let locals = ["value", "value", "debug_name", "debug_name"];
    let mut names = DeclNameUniquifier::new(locals, ["value"]);
    assert_eq!(names.unique("value"), "value_1");
    assert_eq!(names.unique("value"), "value_2");
    assert_eq!(names.unique("debug_name"), "debug_name");
    assert_eq!(names.unique("debug_name"), "debug_name_1");
}

#[test]
fn global_and_direct_callee_suffixes_remain_owned_by_their_references() {
    // `value_1` models a referenced global and `callee_1` a named direct call.
    // The second local of each repeated spelling must skip those identifiers,
    // leaving the printed references bound to their original non-local owners.
    let locals = ["value", "value", "callee", "callee"];
    let mut names = DeclNameUniquifier::new(locals, ["value_1", "callee_1"]);
    assert_eq!(names.unique("value"), "value");
    assert_eq!(names.unique("value"), "value_2");
    assert_eq!(names.unique("callee"), "callee");
    assert_eq!(names.unique("callee"), "callee_2");
}

#[test]
fn same_sequence_is_deterministic() {
    fn allocate() -> Vec<String> {
        let locals = ["v2", "v2", "v2", "v2_1"];
        let mut names = DeclNameUniquifier::new(locals, ["a0"]);
        locals.into_iter().map(|name| names.unique(name)).collect()
    }
    assert_eq!(allocate(), allocate());
    assert_eq!(allocate(), ["v2", "v2_2", "v2_3", "v2_1"]);
}

#[test]
fn ghidra_style_prefixes_are_preserved_when_suffixing() {
    let locals = ["uVar1", "uVar1", "Var2", "Var2"];
    let mut names = DeclNameUniquifier::new(locals, std::iter::empty());
    assert_eq!(names.unique("uVar1"), "uVar1");
    assert_eq!(names.unique("uVar1"), "uVar1_1");
    assert_eq!(names.unique("Var2"), "Var2");
    assert_eq!(names.unique("Var2"), "Var2_1");
}
