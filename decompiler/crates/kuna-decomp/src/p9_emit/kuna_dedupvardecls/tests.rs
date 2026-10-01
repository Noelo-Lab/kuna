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

fn obj(n: u32) -> DeclIdentity {
    DeclIdentity::High(HighVariableId(n))
}

#[test]
fn dedup_suppresses_only_repeats() {
    let mut d = DeclDedup::new();
    let sig = |t: &str, n: &str, off: u64| -> DeclSignature {
        (t.to_string(), String::new(), n.to_string(), None, Some(("stack".to_string(), off)))
    };
    // First occurrence: emit (not a duplicate).
    assert_eq!(d.earlier(obj(0), sig("int4", "option_index", 0x3c), HighVariableId(99)), None);
    // Identical signature: suppress.
    assert!(d.earlier(obj(0), sig("int4", "option_index", 0x3c), HighVariableId(99)).is_some());
    assert!(d.earlier(obj(0), sig("int4", "option_index", 0x3c), HighVariableId(99)).is_some());
}

#[test]
fn dedup_keeps_distinct_signatures() {
    let mut d = DeclDedup::new();
    let sig = |t: &str, n: &str, off: u64| -> DeclSignature {
        (t.to_string(), String::new(), n.to_string(), None, Some(("stack".to_string(), off)))
    };
    // Same name + type, DIFFERENT storage slot -> distinct -> both emit.
    assert_eq!(d.earlier(obj(0), sig("int4", "v1", 0x10), HighVariableId(99)), None);
    assert_eq!(d.earlier(obj(0), sig("int4", "v1", 0x20), HighVariableId(99)), None);
    // Same name + slot, DIFFERENT type -> distinct -> both emit.
    assert_eq!(d.earlier(obj(0), sig("char *", "v2", 0x30), HighVariableId(99)), None);
    assert_eq!(d.earlier(obj(0), sig("int8", "v2", 0x30), HighVariableId(99)), None);
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
    assert_eq!(d.earlier(obj(0), s1, HighVariableId(99)), None);
    assert!(d.earlier(obj(0), s2, HighVariableId(99)).is_some()); // identical array -> suppress
    assert_eq!(d.earlier(obj(0), s3, HighVariableId(99)), None); // different count -> keep
}

#[test]
fn dedup_keeps_distinct_declarator_suffixes() {
    let mut d = DeclDedup::new();
    let sig = |back: &str| -> DeclSignature {
        ("char (*".into(), back.into(), "p".into(), None, Some(("rax".into(), 0)))
    };
    assert_eq!(d.earlier(obj(0), sig(")[16]"), HighVariableId(99)), None);
    assert!(d.earlier(obj(0), sig(")[16]"), HighVariableId(99)).is_some());
    assert_eq!(d.earlier(obj(0), sig(")[8]"), HighVariableId(99)), None);
}

#[test]
fn dedup_keeps_identical_lines_of_distinct_objects() {
    let mut d = DeclDedup::new();
    let sig = || -> DeclSignature { ("Node *".into(), String::new(), "object".into(), None, None) };
    assert_eq!(d.earlier(obj(1), sig(), HighVariableId(99)), None);
    assert_eq!(d.earlier(obj(2), sig(), HighVariableId(99)), None);
    let slot = DeclIdentity::Symbol(SymbolId::default());
    assert_eq!(d.earlier(slot, sig(), HighVariableId(3)), None);
    assert_eq!(d.earlier(slot, sig(), HighVariableId(4)), Some(HighVariableId(3)));
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

#[test]
fn identifiers_taken_outside_the_function_match_reserving_them_up_front() {
    let cases: [(&[&str], &[&str], &[&str]); 4] = [
        (&["value", "value", "callee", "callee"], &[], &["value_1", "callee_1"]),
        (&["value", "value", "debug_name", "debug_name"], &["value"], &["debug_name"]),
        (&["word", "word", "word_1", "word"], &["a0"], &["word", "word_2"]),
        (&["v2", "v2", "v2", "v2_1"], &[], &["v2_1", "v2_3", "v2_4"]),
    ];
    for (locals, occupied, globals) in cases {
        let mut reserved =
            DeclNameUniquifier::new(locals.iter().copied(), occupied.iter().chain(globals).copied());
        let mut queried = DeclNameUniquifier::new(locals.iter().copied(), occupied.iter().copied());
        let taken = |name: &str| globals.contains(&name);
        for &name in locals {
            assert_eq!(queried.unique_with(name, &taken), reserved.unique(name), "{locals:?} {globals:?}");
        }
    }
}
