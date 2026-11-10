//! The fixes `kt_inline_lock_local` and `synchronized_lock_assignment` on
//! their kuna-fixes cases, each alone off (`KunaFixes::without`) and on, in
//! the output each is for and in the other language's.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes, java: Option<bool>) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    if let Some(java) = java {
        let name = id.rsplit('/').next().unwrap();
        case.options.retain(|(k, _)| k != "jvm_plugins");
        case.options
            .push(("jvm_plugins".into(), if java { "0" } else { "1" }.into()));
        case.compare = vec![format!("{name}.{}", if java { "java" } else { "kt" })];
    }
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const LOCK_LOCAL: &str = "pkg/TestKtInlineLockLocal";
const LOCK_ASSIGNMENT: &str = "pkg/TestSynchronizedLockAssignment";
const BREAK_EXIT: &str = "pkg/TestSynchronizedBreakExit";

#[test]
fn a_lock_local_is_not_read_again() {
    let fixed = text_with(LOCK_LOCAL, KunaFixes::DEFAULT, None);
    assert_eq!(fixed.matches("synchronized (l) {").count(), 2, "{fixed}");
    assert!(fixed.contains("synchronized (e) {"), "{fixed}");
    assert!(!fixed.contains("synchronized (this.slots[i])"), "{fixed}");
    assert!(fixed.contains("synchronized (this.fixed) {"), "{fixed}");
    assert_eq!(
        fixed.matches("synchronized (this.lock) {").count(),
        1,
        "{fixed}"
    );

    let broken = text_with(
        LOCK_LOCAL,
        KunaFixes::DEFAULT.without(KunaFix::KtInlineLockLocal),
        None,
    );
    assert_eq!(
        broken.matches("synchronized (this.lock) {").count(),
        3,
        "{broken}"
    );
    assert!(
        broken.contains("synchronized (this.slots[i]) {"),
        "{broken}"
    );
    assert_eq!(
        broken
            .replace("synchronized (this.slots[i])", "synchronized (e)")
            .replacen("synchronized (this.lock)", "synchronized (l)", 2),
        fixed,
        "only the heads of field, element and usedAfter differ"
    );
}

#[test]
fn a_lock_local_keeps_its_other_uses() {
    let fixed = text_with(BREAK_EXIT, KunaFixes::DEFAULT, None);
    assert!(fixed.contains("synchronized (state) {"), "{fixed}");
    assert!(
        fixed.contains("&& state is TestSynchronizedBreakExit.Finishing"),
        "{fixed}"
    );
    let broken = text_with(
        BREAK_EXIT,
        KunaFixes::DEFAULT.without(KunaFix::KtInlineLockLocal),
        None,
    );
    assert!(
        broken.contains("synchronized (`this_$iv`.state) {"),
        "{broken}"
    );
}

#[test]
fn java_output_keeps_the_kotlin_lock_locals() {
    for fixes in [KunaFixes::DEFAULT, KunaFixes::COMPAT] {
        let java = text_with(LOCK_LOCAL, fixes, Some(true));
        assert_eq!(java.matches("synchronized (l) {").count(), 4, "{java}");
        assert!(java.contains("synchronized (e) {"), "{java}");
    }
}

#[test]
fn only_the_lock_variable_assignment_folds_into_the_head() {
    let fixed = text_with(LOCK_ASSIGNMENT, KunaFixes::DEFAULT, None);
    for kept in [
        "Object l = this.lock;\n      this.lock = new Object();\n      synchronized (l) {",
        "Object l = this.lock;\n      this.probe.outside();\n      synchronized (l) {",
        "Object e = this.slots[i];\n      this.slots[i] = new Object();\n      synchronized (e) {",
        "Object l = this.next();\n      synchronized (this.next()) {",
        "synchronized (this.lock) {\n         this.lock = new Object();",
    ] {
        assert!(fixed.contains(kept), "{kept}: {fixed}");
    }
    let broken = text_with(
        LOCK_ASSIGNMENT,
        KunaFixes::DEFAULT.without(KunaFix::SynchronizedLockAssignment),
        None,
    );
    assert!(
        broken.contains("this.lock = new Object();\n      synchronized (this.lock) {"),
        "{broken}"
    );
    assert!(
        broken.contains("synchronized (this.slots[i]) {"),
        "{broken}"
    );
    assert!(!broken.contains("Object l = this.next();"), "{broken}");
}
