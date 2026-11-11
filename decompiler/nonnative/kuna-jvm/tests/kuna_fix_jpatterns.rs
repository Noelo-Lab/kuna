//! `instanceof` patterns where the class file shows the source had one: the
//! fix `pattern_binding_shape` alone off (`KunaFixes::without`), the option
//! `jvm_source_patterns` (its row, the compat switch, its case with it off),
//! and the pattern-subject parts of `jvm_source_locals` and
//! `jvm_javac_layout`, each on its kuna-fixes case with the option off.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};
use kuna_jvm::kuna_options::{
    find, resolve, take_form_options, JAVAC_LAYOUT, KUNA_FORM, SOURCE_LOCALS, SOURCE_PATTERNS,
};

fn text_with(id: &str, options: &[(&str, &str)], fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"));
    case.options
        .extend(options.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

fn assert_has(text: &str, lines: &[&str]) {
    for line in lines {
        assert!(text.contains(line), "{line}\n{text}");
    }
}

#[test]
fn a_test_and_cast_javac_did_not_compile_as_a_pattern_stays() {
    let id = "pkg/TestPatternBindingShape";
    let fixed = text_with(id, &[], KunaFixes::DEFAULT);
    assert_has(
        &fixed,
        &[
            "if (this.head instanceof TestPatternBindingShape.Leaf) {\n         TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)this.head;",
            "TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)shared;",
            "TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)this.slots[i];",
            "TestPatternBindingShape.Leaf leaf = (TestPatternBindingShape.Leaf)node.next;",
            "if (!(o instanceof TestPatternBindingShape)) {\n         return false;\n      }\n\n      TestPatternBindingShape other = (TestPatternBindingShape)o;",
            "if (this.head instanceof TestPatternBindingShape.Leaf leaf) {",
            "if (!(node instanceof TestPatternBindingShape.Leaf leaf)) {",
            "if (node instanceof TestPatternBindingShape.Leaf leaf) {",
        ],
    );

    let vineflower = text_with(
        id,
        &[(SOURCE_PATTERNS, "0")],
        KunaFixes::DEFAULT.without(KunaFix::PatternBindingShape),
    );
    assert_has(
        &vineflower,
        &[
            "if (shared instanceof TestPatternBindingShape.Leaf leaf) {",
            "if (this.slots[i] instanceof TestPatternBindingShape.Leaf leaf) {",
            "if (node.next instanceof TestPatternBindingShape.Leaf leaf) {",
            "if (!(o instanceof TestPatternBindingShape other)) {",
        ],
    );
}

#[test]
fn the_source_patterns_row_is_a_kuna_form_option_on_by_default() {
    let row = find(SOURCE_PATTERNS).expect("jvm_source_patterns row");
    assert!(row.is_kuna());
    assert_eq!((row.values, row.default), ("on|off", "on"));
    assert_eq!(row.change_kind, KUNA_FORM);
    assert_eq!(
        resolve(SOURCE_PATTERNS, "off").unwrap(),
        (SOURCE_PATTERNS, "0".to_string())
    );
    let pairs = |items: &[(&str, &str)]| -> Vec<(String, String)> {
        items
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    };
    let (forms, rest) = take_form_options(&pairs(&[(SOURCE_PATTERNS, "off"), ("a", "1")]), false);
    assert!(!forms.contains(SOURCE_PATTERNS));
    assert_eq!(rest, pairs(&[("a", "1")]));
    assert!(take_form_options(&[], false).0.contains(SOURCE_PATTERNS));
    assert!(!take_form_options(&[], true).0.contains(SOURCE_PATTERNS));
}

#[test]
fn a_cast_the_line_table_shows_as_a_statement_stays() {
    let id = "pkg/TestSourcePatterns";
    let kept = text_with(id, &[], KunaFixes::DEFAULT);
    assert_has(
        &kept,
        &[
            "if (shape instanceof TestSourcePatterns.Square) {\n         TestSourcePatterns.Square square = (TestSourcePatterns.Square)shape;\n         return square.side * square.side;",
            "} else if (shape instanceof TestSourcePatterns.Circle) {\n         TestSourcePatterns.Circle circle = (TestSourcePatterns.Circle)shape;",
            "if (shape instanceof TestSourcePatterns.Square square) {\n         return square.side;",
            "if (round && shape instanceof TestSourcePatterns.Circle circle && circle.radius > 0) {",
        ],
    );

    let patterns = text_with(id, &[(SOURCE_PATTERNS, "0")], KunaFixes::DEFAULT);
    assert!(
        !patterns.contains("(TestSourcePatterns.Square)shape;"),
        "{patterns}"
    );
    assert_has(
        &patterns,
        &[
            "} else if (shape instanceof TestSourcePatterns.Circle circle) {",
            "if (round && shape instanceof TestSourcePatterns.Circle circle && circle.radius > 0) {",
        ],
    );
}

#[test]
fn a_named_local_stays_the_subject_of_a_pattern() {
    let id = "pkg/TestPatternSubjectLocal";
    let kept = text_with(id, &[], KunaFixes::DEFAULT);
    assert_has(
        &kept,
        &[
            "TestPatternSubjectLocal.Entry entry = this.root.lookup(key);\n      if (entry instanceof TestPatternSubjectLocal.Group group) {",
            "TestPatternSubjectLocal.Entry entry = this.root.lookup(key);\n      if (!(entry instanceof TestPatternSubjectLocal.Group group)) {",
            "TestPatternSubjectLocal.Entry entry = this.root.lookup(key);\n         if (entry instanceof TestPatternSubjectLocal.Group group && group.count > 0) {",
            "if (this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group) {",
            "if (this.root.parent instanceof TestPatternSubjectLocal.Group group) {",
        ],
    );

    let folded = text_with(id, &[(SOURCE_LOCALS, "0")], KunaFixes::DEFAULT);
    assert!(!folded.contains("Entry entry ="), "{folded}");
    assert_has(
        &folded,
        &["if (!(this.root.lookup(key) instanceof TestPatternSubjectLocal.Group group)) {"],
    );
}

#[test]
fn a_negated_pattern_keeps_its_guard() {
    let id = "pkg/TestNegatedPatternLayout";
    let guard = text_with(id, &[], KunaFixes::DEFAULT);
    assert_has(
        &guard,
        &[
            "if (!(item instanceof TestNegatedPatternLayout.Box box)) {\n         return -1;\n      }\n\n      box.grow();",
            "if (!(item instanceof TestNegatedPatternLayout.Box box)) {\n         throw new IllegalStateException();\n      }\n\n      this.total += box.size;",
        ],
    );

    let nested = text_with(id, &[(JAVAC_LAYOUT, "0")], KunaFixes::DEFAULT);
    assert_has(
        &nested,
        &["if (item instanceof TestNegatedPatternLayout.Box box) {\n         box.grow();"],
    );
}
