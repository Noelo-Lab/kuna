//! The Kotlin call-form fixes of ktfacade (`kt_facade_call`, `kt_extension_call`,
//! `kt_jvm_name_call`, `kt_synthetic_accessor_call`), one at a time on their kuna-fixes
//! cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kotlin::kuna_fix_kt_facade_call::{default_import, java_property, mangled};
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
fn top_level_calls_are_written_by_name_with_their_imports() {
    let (fixed, without) = fixed_and_without("pkg/TestFacadeCall", KunaFix::KtFacadeCall);
    for written in [
        "import pkg.lib.LIMIT",
        "import pkg.lib.lonely",
        "return pkg.lib.twice(this.base) + lonely()",
        "return pkg.lib.counter + LIMIT + names.size",
        "return \"${greet()}${greet(\"y\")}\"",
        "return counter * counter + pkg.lib.counter",
    ] {
        assert!(fixed.contains(written), "{written}: {fixed}");
    }
    assert!(!fixed.contains("FacadeLib"), "{fixed}");
    assert!(without.contains("FacadeLib.twice(this.base) + FacadeMulti.lonely()"), "{without}");
    assert!(without.contains("return counter * counter + counter"), "{without}");
}

#[test]
fn extension_calls_are_written_on_their_receivers() {
    let (fixed, without) = fixed_and_without("pkg/TestExtensionCall", KunaFix::KtExtensionCall);
    for written in [
        "this.s.shout().length + this.s.wrap(2).length",
        "this.s.tag = 4",
        "return this.s.firstChar.code + this.s.tag + this.s.length.half",
        "return this.s.toBox().v + this.s.twiceOf().length",
        "import pkg.lib.ExtBox.Companion.toBox",
        "(var10001 as CharSequence).describe()",
        "(b as Any).label()",
    ] {
        assert!(fixed.contains(written), "{written}: {fixed}");
    }
    assert!(without.contains("tag = this.s\n"), "{without}");
    assert!(without.contains("return firstChar.code + tag"), "{without}");
}

#[test]
fn member_calls_use_their_kotlin_names() {
    let (fixed, without) = fixed_and_without("pkg/TestJvmNameCall", KunaFix::KtJvmNameCall);
    assert!(fixed.contains("return this.c.bump() + this.s.bump()"), "{fixed}");
    assert!(fixed.contains("return this.c.size() + this.c.get(1)"), "{fixed}");
    assert!(fixed.contains("this.s.closed = true"), "{fixed}");
    assert!(fixed.contains("take_dXQi75k"), "{fixed}");
    assert!(without.contains("this.c.sizeOf() + this.c.getByte(1)"), "{without}");
    assert!(without.contains("this.s.setClosed(true)"), "{without}");
}

#[test]
fn accessor_calls_are_the_accesses_they_make() {
    let (fixed, without) =
        fixed_and_without("pkg/TestSyntheticAccessorCall", KunaFix::KtSyntheticAccessorCall);
    assert!(fixed.contains("return \"${label(this.base.doubled())}${PREFIX}\""), "{fixed}");
    assert!(fixed.contains("hits = hits + this.base"), "{fixed}");
    assert!(without.contains("access$getHits$p"), "{without}");
}

#[test]
fn names_kotlin_mangles_or_imports_by_default() {
    assert!(mangled("take-dXQi75k", "take"));
    assert!(!mangled("bump$main", "bump"));
    assert!(!mangled("-deprecated_size", "size"));
    assert_eq!(java_property("getClosed").as_deref(), Some("closed"));
    assert_eq!(java_property("isEmpty").as_deref(), Some("isEmpty"));
    assert_eq!(java_property("get"), None);
    assert!(default_import("kotlin.collections"));
    assert!(default_import("kotlin"));
    assert!(!default_import("kotlin.math"));
    assert!(!default_import("okio"));
}
