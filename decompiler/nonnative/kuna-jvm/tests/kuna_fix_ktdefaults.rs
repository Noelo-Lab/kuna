//! The fixes `kt_default_args_bridge_lookup`, `kt_default_value_names`,
//! `kt_default_value_coercion_cast` and `kt_delegated_member_default` on
//! their kuna-fixes cases, each alone off (`KunaFixes::without`) and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const LOOKUP: &str = "pkg/TestDefaultArgsBridgeLookup";
const NAMES: &str = "pkg/TestDefaultValueNames";
const CAST: &str = "pkg/TestDefaultValueCoercionCast";
const DELEGATED: &str = "pkg/TestDelegatedMemberDefault";

#[test]
fn a_default_is_read_from_the_method_of_the_jvm_name() {
    let fixed = text_with(LOOKUP, KunaFixes::DEFAULT);
    for header in [
        "fun scale(x: Int, factor: Int = this.base): Int",
        "fun shift(x: Int, by: Int = x + 1): Int",
        "fun walk(d: pkg.TestDefaultArgsBridgeLookup.Meters, steps: Int = 2): Int",
        "suspend fun load(key: String = \"k\", times: Int = key.length): String",
        "fun pick(xs: List<Int>, n: Int = 1): Int",
        "fun pick(xs: List<String>, n: Int = 2): Int",
        "A.emit(text: String, sep: String = \", \"): A",
        "fun make(size: Int = 4, label: String = \"box\")",
        "fun area(scale: Int = 2): Int",
        "fun describe(name: String = \"shape\", width: Int = 3): String",
    ] {
        assert!(fixed.contains(header), "{header}\n{fixed}");
    }
    assert!(!fixed.contains("= ..."), "{fixed}");
    let broken = text_with(
        LOOKUP,
        KunaFixes::DEFAULT.without(KunaFix::KtDefaultArgsBridgeLookup),
    );
    assert_eq!(broken.matches("= ...").count(), 11, "{broken}");
    assert!(
        broken.contains("fun pick(xs: List<Int>, n: Int = 2): Int"),
        "{broken}"
    );
}

#[test]
fn a_default_names_its_receivers_and_parameters() {
    let fixed = text_with(NAMES, KunaFixes::DEFAULT);
    for header in [
        "fun String.pad(width: Int = this@TestDefaultValueNames.length): String",
        "fun String.cut(n: Int = this.length - 1, keep: Boolean = n > 0): String",
        "fun twice(a: Int, b: Int = a * 2): Int",
        "fun scaled(base: Int, by: Int = base * 2): Int",
    ] {
        assert!(fixed.contains(header), "{header}\n{fixed}");
    }
    let broken = text_with(
        NAMES,
        KunaFixes::DEFAULT.without(KunaFix::KtDefaultValueNames),
    );
    for header in [
        "fun String.pad(width: Int = this.length): String",
        "fun String.cut(n: Int = `$this$cut`.length - 1, keep: Boolean = n > 0): String",
        "fun scaled(base: Int, by: Int = var1 * 2): Int",
    ] {
        assert!(broken.contains(header), "{header}\n{broken}");
    }
}

#[test]
fn a_default_loses_the_cast_to_its_parameter_type() {
    let fixed = text_with(CAST, KunaFixes::DEFAULT);
    assert!(fixed.contains("tag: CharSequence = \"tag\",\n"), "{fixed}");
    assert!(
        fixed.contains(
            "base: pkg.TestDefaultValueCoercionCast.Base = TestDefaultValueCoercionCast.Derived(),\n"
        ),
        "{fixed}"
    );
    let broken = text_with(
        CAST,
        KunaFixes::DEFAULT.without(KunaFix::KtDefaultValueCoercionCast),
    );
    assert!(
        broken.contains("tag: CharSequence = \"tag\" as CharSequence,\n"),
        "{broken}"
    );
    assert!(
        broken.contains("Derived() as TestDefaultValueCoercionCast.Base,\n"),
        "{broken}"
    );
}

#[test]
fn a_delegated_member_declares_no_defaults() {
    let fixed = text_with(DELEGATED, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("public override fun area(scale: Int, label: String?): Int"),
        "{fixed}"
    );
    assert!(
        fixed.contains("public abstract fun area(scale: Int = 2, label: String? = null): Int"),
        "{fixed}"
    );
    let broken = text_with(
        DELEGATED,
        KunaFixes::DEFAULT.without(KunaFix::KtDelegatedMemberDefault),
    );
    assert!(
        broken.contains("public override fun area(scale: Int = ..., label: String? = ...): Int"),
        "{broken}"
    );
}
