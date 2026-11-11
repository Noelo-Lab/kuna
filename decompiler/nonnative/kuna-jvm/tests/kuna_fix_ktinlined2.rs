//! The fixes `unlisted_anonymous_class`, `kt_companion_nameless_class` and
//! `refresh_printed_var_names`, each with only its own fix turned off
//! (`KunaFixes::without`).

use std::path::{Path, PathBuf};

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

fn run(mut case: Case, fixes: KunaFixes) -> Option<String> {
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual)
}

fn text_with(id: &str, fixes: KunaFixes) -> String {
    run(fix_case(id), fixes).expect("the compared unit")
}

#[test]
fn inlined_objects_are_written_where_they_are_created() {
    let fixed = text_with("pkg/TestUnlistedAnonymousClass", KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    for text in [
        "this.queue.schedule(object : UnlistedTask(label, `cancelable$iv`) {",
        "this@TestUnlistedAnonymousClass.sent = this@TestUnlistedAnonymousClass.sent + id + label.length",
        "return object : UnlistedHandler {",
        "return code * base + this@TestUnlistedAnonymousClass.sent + 1",
        "val var10000: Any? = `$this$unlistedTransform$iv`.drain(object : UnlistedSink<Any> {",
        "if (valuex < limit) {",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("$$inlined"), "{fixed}");

    let without = text_with(
        "pkg/TestUnlistedAnonymousClass",
        KunaFixes::DEFAULT.without(KunaFix::UnlistedAnonymousClass),
    );
    for text in [
        "this.queue.schedule(TestUnlistedAnonymousClass$send$$inlined$execute$default$1(label, true, this, id, label), 5L)",
        "return TestUnlistedAnonymousClass$offset$$inlined$unlistedHandler$1(base, this)",
    ] {
        assert!(without.contains(text), "{text}\n{without}");
    }
}

/// The corpus case of the kuna-fixes table run on the test's own classes:
/// Vineflower writes no unit for a class it fails whole.
fn companion_case() -> Case {
    let mut case = fix_case("pkg/TestRefreshPrintedVarNames");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/ktinlined2/pkg");
    case.class_files = [
        "TestCompanionNamelessClass.class",
        "TestCompanionNamelessClass$1.class",
        "TestCompanionNamelessClass$Companion.class",
        "CompanionAction.class",
    ]
    .iter()
    .map(|f| dir.join(f))
    .collect();
    case.compare = vec!["TestCompanionNamelessClass.kt".to_string()];
    case
}

#[test]
fn a_companion_beside_an_anonymous_class_is_found() {
    let fixed = run(companion_case(), KunaFixes::DEFAULT).expect("the class's unit");
    for text in [
        "public class TestCompanionNamelessClass(start: Int) {",
        "this.apply(object : CompanionAction {",
        "public companion object {",
        "return TestCompanionNamelessClass(start).count * 2",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }

    let without = run(
        companion_case(),
        KunaFixes::DEFAULT.without(KunaFix::KtCompanionNamelessClass),
    );
    assert!(without.is_none(), "{without:?}");
}

#[test]
fn a_refreshed_parameter_keeps_its_name() {
    let fixed = text_with("pkg/TestRefreshPrintedVarNames", KunaFixes::DEFAULT);
    assert!(fixed.contains("            var countx: Long = count\n"), "{fixed}");
    assert!(fixed.contains("               countx -= this@TestRefreshPrintedVarNames.step\n"), "{fixed}");

    let without = text_with(
        "pkg/TestRefreshPrintedVarNames",
        KunaFixes::DEFAULT.without(KunaFix::RefreshPrintedVarNames),
    );
    assert!(without.contains("            var countx: Long = countx\n"), "{without}");
}

#[test]
fn a_nested_copy_reads_its_creators_captured_values() {
    let id = "pkg/TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    for text in [
        "public fun Source.dropFirst(count: Int, fallback: Int): Source {",
        "if (skipped.element >= count) {",
        "`$this$dropFirst_u24lambda_u240`.put(it + fallback, `$completion`)",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("$count$inlined"), "{fixed}");

    let without = text_with(
        id,
        KunaFixes::DEFAULT.without(KunaFix::UnlistedAnonymousClass),
    );
    assert!(without.contains(".`$count$inlined`) {"), "{without}");
}
