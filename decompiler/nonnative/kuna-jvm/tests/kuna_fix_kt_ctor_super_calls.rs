//! The constructor-call, compound-assignment and `when`-head fixes of ktprint6, one at a time on their
//! kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

fn fixed_and_without(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT),
        text_with(id, KunaFixes::DEFAULT.without(fix)),
    )
}

#[test]
fn legacy_parameter_checks_do_not_hide_the_delegation_call() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestLegacyParameterChecks",
        KunaFix::KtLegacyConstructorParameterChecks,
    );
    assert!(!fixed.contains("checkParameterIsNotNull"), "{fixed}");
    assert!(
        !fixed.lines().any(|l| l.trim_start().starts_with("super(")),
        "{fixed}"
    );
    assert!(
        fixed.contains("class Holder(text: String, count: Int) : TestLegacyParameterChecks.Base(text.length) {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("constructor(text: String) : super(text.length) {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("constructor(head: String, tail: String) : this(\"$head$tail\")\n"),
        "{fixed}"
    );
    assert!(fixed.contains("public final val data: ByteArray\n"), "{fixed}");
    assert!(
        without.contains("constructor(text: String) : Intrinsics.checkParameterIsNotNull(text, \"text\") {"),
        "{without}"
    );
    assert!(without.contains("super(text.length)"), "{without}");
    assert!(
        without.contains("public final val data: ByteArray = ByteArray(8)"),
        "{without}"
    );
}

#[test]
fn a_hidden_empty_super_call_is_still_written() {
    let (fixed, without) =
        fixed_and_without("pkg/TestHiddenEmptySuperCall", KunaFix::KtHiddenEmptySuperCall);
    assert!(
        fixed.contains("class Plain(label: String) : TestHiddenEmptySuperCall.Base() {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("object Single : TestHiddenEmptySuperCall.Base() {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("companion object : TestHiddenEmptySuperCall.Base()\n"),
        "{fixed}"
    );
    assert!(
        fixed.contains("public constructor(s: String) {\n         this.remember(s)\n      }"),
        "{fixed}"
    );
    assert!(fixed.contains("public constructor(n: Int)\n"), "{fixed}");
    assert!(
        without.contains("class Plain(label: String) : TestHiddenEmptySuperCall.Base {"),
        "{without}"
    );
    assert!(
        without.contains("public constructor(s: String) : this.remember(s)"),
        "{without}"
    );
    assert!(!without.contains("constructor(n: Int)"), "{without}");
}

#[test]
fn legacy_checks_alone_leave_the_empty_call_to_its_own_fix() {
    let text = text_with(
        "pkg/TestLegacyParameterChecks",
        KunaFixes::DEFAULT.without(KunaFix::KtHiddenEmptySuperCall),
    );
    assert!(!text.contains("checkParameterIsNotNull"), "{text}");
    assert!(text.contains(": TestLegacyParameterChecks.Base(text.length) {"), "{text}");
    assert!(
        text.contains("public constructor(data: ByteArray, owner: Boolean)  {"),
        "{text}"
    );
}

#[test]
fn a_compound_bitwise_assignment_is_an_infix_call() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestCompoundBitwiseAssignment",
        KunaFix::KtCompoundBitwiseAssignment,
    );
    assert!(
        fixed.contains("this.flags = this.flags or (1 shl b)\n"),
        "{fixed}"
    );
    assert!(fixed.contains("this.mask = this.mask ushr 3\n"), "{fixed}");
    assert!(fixed.contains("this.ok = this.ok or (a > b)\n"), "{fixed}");
    for op in ["|=", "^=", "<<=", ">>>="] {
        assert!(!fixed.contains(&format!(" {op} ")), "{fixed}");
    }
    assert!(without.contains("this.flags |= 1 shl b\n"), "{without}");
    assert!(without.contains("this.mask >>>= 3\n"), "{without}");
}

#[test]
fn the_statements_before_a_when_are_indented() {
    let (fixed, without) =
        fixed_and_without("pkg/TestSwitchHeadIndent", KunaFix::KtSwitchHeadIndent);
    assert!(
        fixed.contains("      val b: Int = this.note(s.length + a)\n      var var10000: Int\n      when (n) {"),
        "{fixed}"
    );
    assert!(
        without.contains("\nval b: Int = this.note(s.length + a)\nvar var10000: Int\n"),
        "{without}"
    );
}
