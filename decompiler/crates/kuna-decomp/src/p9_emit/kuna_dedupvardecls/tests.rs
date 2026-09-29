//! Declaration-option, signature-deduplication and name-allocation tests.

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
        (t.to_string(), String::new(), n.to_string(), None, Some(("stack".to_string(), off)))
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
        (t.to_string(), String::new(), n.to_string(), None, Some(("stack".to_string(), off)))
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
    let s1: DeclSignature =
        ("int2".into(), String::new(), "arr".into(), Some(("int2".into(), 32)), None);
    let s2: DeclSignature =
        ("int2".into(), String::new(), "arr".into(), Some(("int2".into(), 32)), None);
    let s3: DeclSignature =
        ("int2".into(), String::new(), "arr".into(), Some(("int2".into(), 16)), None);
    assert!(!d.is_duplicate(s1));
    assert!(d.is_duplicate(s2)); // identical array -> suppress
    assert!(!d.is_duplicate(s3)); // different count -> keep
}

#[test]
fn dedup_keeps_distinct_declarator_suffixes() {
    let mut d = DeclDedup::new();
    let sig = |back: &str| -> DeclSignature {
        ("char (*".into(), back.into(), "p".into(), None, Some(("rax".into(), 0)))
    };
    assert!(!d.is_duplicate(sig(")[16]")));
    assert!(d.is_duplicate(sig(")[16]")));
    assert!(!d.is_duplicate(sig(")[8]")));
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

#[test]
fn default_allocator_accepts_unreserved_and_generated_base_names() {
    let mut names = DeclNameUniquifier::default();
    for (base, expected) in [
        ("x", "x"), ("x", "x_1"), ("x_1", "x_1_1"),
        ("", ""), ("", "_1"), ("_1", "_1_1"),
        ("α", "α"), ("α", "α_1"),
    ] {
        assert_eq!(names.unique(base), expected);
    }
}

#[test]
fn allocation_matches_a_first_free_name_reference() {
    use std::collections::BTreeSet;

    let pool = ["", "x", "x_1", "x_2", "x_1_1", "tmp", "tmp_9", "α", "α_1", "_1"];
    let mut random = 1u64;
    let mut choose = || {
        random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
        pool[((random >> 32) as usize) % pool.len()]
    };
    for case in 0..1000 {
        let locals: Vec<_> = (0..case % 31).map(|_| choose()).collect();
        let occupied: Vec<_> = (0..case % 7).map(|_| choose()).collect();
        let reserved: BTreeSet<_> = locals.iter().chain(&occupied).copied().collect();
        let mut used: BTreeSet<String> = occupied.iter().map(|name| (*name).to_owned()).collect();
        let mut names = DeclNameUniquifier::new(locals.iter().copied(), occupied.iter().copied());
        for _ in 0..64 {
            let base = choose();
            let expected = if !used.contains(base) {
                base.to_owned()
            } else {
                (1u32..)
                    .map(|suffix| format!("{base}_{suffix}"))
                    .find(|name| !reserved.contains(name.as_str()) && !used.contains(name))
                    .unwrap()
            };
            assert_eq!(names.unique(base), expected, "case {case}, base {base:?}");
            assert!(used.insert(expected));
        }
    }
}
