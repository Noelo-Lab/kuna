//! Kotlin fixes on classes outside the corpus, which Vineflower fails whole: kotlinc 2.0.0 classes
//! in `data/ktinlined2` and `data/kt_anonymous_class_failure_reason` (`-jvm-target 17`), run on a
//! kuna-fixes case's options.

use std::path::{Path, PathBuf};

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../decompiler/managed/kuna-jvm/corpus/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

fn run(case: Case) -> Option<String> {
    with_engine_stack(move || run_case(&case).actual)
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
    let fixed = run(companion_case()).expect("the class's unit");
    for text in [
        "public class TestCompanionNamelessClass(start: Int) {",
        "this.apply(object : CompanionAction {",
        "public companion object {",
        "return TestCompanionNamelessClass(start).count * 2",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
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
    let fixed = run(own_case(NAMELESS, "TestNamelessCapture.kt")).expect("unit");
    for text in [
        "      val var1: NamelessContext = NamelessEmpty\n      return object : NamelessSink {",
        "               return var1\n",
        "return value * this@TestNamelessCapture.base + var1.tag.length",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("$context"), "{fixed}");
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
    let site = run(own_case(GUARDS, "TestUnlistedGuards.kt")).expect("unit");
    assert!(
        site.contains("return TestUnlistedGuards$later$$inlined$guardSuspend$1(null, this, step)"),
        "{site}"
    );
    let unit = "TestUnlistedGuardsKt$typedSource$$inlined$guardSource$1.kt";
    assert!(run(own_case(GUARDS, unit)).is_some());
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
    let fixed = run(own_case(CAPTURED_ARRAY, "TestCapturedArrayNameKt.kt")).expect("unit");
    for text in [
        "   val var6: Array<Int> = arrayOf(a, b, c)\n   return object : ArraySource {",
        "         val var2: Array<Int> = var6\n",
        "            s += var2[var3] * k\n",
    ] {
        assert!(fixed.contains(text), "{text}\n{fixed}");
    }
    assert!(!fixed.contains("values$iv"), "{fixed}");
}

/// `tests/data/kt_anonymous_class_failure_reason/` (kotlinc 2.0.0, `-jvm-target 17`) run as a
/// kuna-fixes Kotlin case: it has no case of its own, because what its fix changes is printed
/// only with `dump-exception-on-error` on.
fn unwritable_case() -> Case {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/kt_anonymous_class_failure_reason/pkg");
    let mut class_files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "class"))
        .collect();
    class_files.sort();
    let main = class_files
        .iter()
        .position(|p| p.ends_with("TestUnwritableObjectsKt.class"))
        .unwrap();
    let main = class_files.remove(main);
    class_files.insert(0, main);
    let mut case = fix_case("pkg/TestReferenceClassesKt");
    case.id = "kuna-fixes/pkg/TestUnwritableObjectsKt".to_string();
    case.class_files = class_files;
    case.compare = vec![
        "TestUnwritableObjectsKt.kt".into(),
        "TestUnwritableObjectsKt.java".into(),
    ];
    case
}

/// The unwritable classes' text with the exception in failure comments.
fn unwritable_text() -> String {
    let mut case = unwritable_case();
    case.options.retain(|(k, _)| k != "dump-exception-on-error");
    case.options
        .push(("dump-exception-on-error".into(), "1".into()));
    run(case).expect("the compared unit")
}

#[test]
fn an_unwritable_anonymous_class_says_what_it_is() {
    let fixed = unwritable_text();
    assert_eq!(count_failed_methods(&fixed), 1, "{fixed}");
    assert!(
        fixed.contains(
            "Anonymous class does not have Class Kotlin metadata: pkg.TestUnwritableObjectsKt$localReference$1 \
             is a reference to a local function that captures variables\n"
        ),
        "{fixed}"
    );
}
