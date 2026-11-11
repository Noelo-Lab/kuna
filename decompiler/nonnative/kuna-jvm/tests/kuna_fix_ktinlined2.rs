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
        "`$this$unlistedTransform$iv`.drain(object : UnlistedSink<Any> {",
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

/// A Kotlin case of the kuna-fixes table run on classes of
/// `tests/data/ktinlined2` (kotlinc 2.0.0), comparing the unit `compare`.
fn own_case(classes: &[&str], compare: &str) -> Case {
    let mut case = fix_case("pkg/TestRefreshPrintedVarNames");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/ktinlined2/pkg");
    case.class_files = classes.iter().map(|f| dir.join(f)).collect();
    case.compare = vec![compare.to_string()];
    case
}

/// Vineflower writes no unit for a class it fails whole.
fn companion_case() -> Case {
    own_case(
        &[
            "TestCompanionNamelessClass.class",
            "TestCompanionNamelessClass$1.class",
            "TestCompanionNamelessClass$Companion.class",
            "CompanionAction.class",
        ],
        "TestCompanionNamelessClass.kt",
    )
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
    assert!(
        fixed.contains("            var countx: Long = count\n"),
        "{fixed}"
    );
    assert!(
        fixed.contains("               countx -= this@TestRefreshPrintedVarNames.step\n"),
        "{fixed}"
    );

    let without = text_with(
        "pkg/TestRefreshPrintedVarNames",
        KunaFixes::DEFAULT.without(KunaFix::RefreshPrintedVarNames),
    );
    assert!(
        without.contains("            var countx: Long = countx\n"),
        "{without}"
    );
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

const NAMELESS: &[&str] = &[
    "TestNamelessCapture.class",
    "TestNamelessCapture$sink$$inlined$namelessSink$1.class",
    "NamelessContext.class",
    "NamelessEmpty.class",
    "NamelessSink.class",
];

/// `TestNamelessCapture.class` is kotlinc's with the LocalVariableTable entry
/// of `context$iv` removed, as kotlinc leaves it out in okhttp 4.7.2 and
/// kotlin-stdlib 2.2.20: the local the copy captures stays a local.
#[test]
fn a_nameless_local_a_copy_captures_stays_a_local() {
    let fixed = run(
        own_case(NAMELESS, "TestNamelessCapture.kt"),
        KunaFixes::DEFAULT,
    )
    .expect("unit");
    for text in [
        "      val var1: NamelessContext = NamelessEmpty\n      return object : NamelessSink {",
        "               return var1\n",
        "return value * this@TestNamelessCapture.base + var1.tag.length",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("$context"), "{fixed}");

    let without = run(
        own_case(NAMELESS, "TestNamelessCapture.kt"),
        KunaFixes::DEFAULT.without(KunaFix::UnlistedAnonymousClass),
    )
    .expect("unit");
    assert!(
        without.contains(
            "return TestNamelessCapture$sink$$inlined$namelessSink$1(NamelessEmpty, this)"
        ),
        "{without}"
    );
}

const GUARDS: &[&str] = &[
    "TestUnlistedGuards.class",
    "TestUnlistedGuards$later$$inlined$guardSuspend$1.class",
    "TestUnlistedGuardsKt.class",
    "TestUnlistedGuardsKt$guardSource$1.class",
    "TestUnlistedGuardsKt$guardSuspend$1.class",
    "TestUnlistedGuardsKt$typedSource$$inlined$guardSource$1.class",
    "GuardSource.class",
];

#[test]
fn copies_written_without_their_arguments_stay_classes() {
    let site = run(
        own_case(GUARDS, "TestUnlistedGuards.kt"),
        KunaFixes::DEFAULT,
    )
    .expect("unit");
    assert!(
        site.contains("return TestUnlistedGuards$later$$inlined$guardSuspend$1(null, this, step)"),
        "{site}"
    );
    let unit = "TestUnlistedGuardsKt$typedSource$$inlined$guardSource$1.kt";
    assert!(run(own_case(GUARDS, unit), KunaFixes::DEFAULT).is_some());
}

const CAPTURED_ARRAY: &[&str] = &[
    "TestCapturedArrayNameKt.class",
    "TestCapturedArrayNameKt$arraySource$1.class",
    "TestCapturedArrayNameKt$spread$$inlined$arraySource$1.class",
    "ArraySource.class",
];

/// kotlinc stores `arrayOf(a, b, c)` into a slot it then re-stores under the
/// debug name `values$iv`; the creator declares it by its own name.
#[test]
fn a_captured_variable_is_read_by_the_name_its_creator_declares() {
    let fixed = run(
        own_case(CAPTURED_ARRAY, "TestCapturedArrayNameKt.kt"),
        KunaFixes::DEFAULT,
    )
    .expect("unit");
    for text in [
        "   val var6: Array<Int> = arrayOf(a, b, c)\n   return object : ArraySource {",
        "         val var2: Array<Int> = var6\n",
        "            s += var2[var3] * k\n",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("values$iv"), "{fixed}");

    let without = run(
        own_case(CAPTURED_ARRAY, "TestCapturedArrayNameKt.kt"),
        KunaFixes::DEFAULT.without(KunaFix::UnlistedAnonymousClass),
    )
    .expect("unit");
    assert!(
        without.contains(
            "return TestCapturedArrayNameKt$spread$$inlined$arraySource$1(arrayOf(a, b, c), k)"
        ),
        "{without}"
    );
}
