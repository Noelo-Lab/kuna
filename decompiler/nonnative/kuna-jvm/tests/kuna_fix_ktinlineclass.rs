//! The fix `kt_inline_class_value` on its kuna-fixes case, off
//! (`KunaFixes::without`) and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASE: &str = "pkg/TestKtInlineClassValue";

fn text_with(fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{CASE}"))
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn inline_class_values_are_written_as_the_classes() {
    let fixed = text_with(KunaFixes::DEFAULT);
    for written in [
        "return a.plus(b)\n",
        "return a.isZero()\n",
        "var sum: TestKtInlineClassValue.Meters = TestKtInlineClassValue.Meters(0.0)\n",
        "sum = sum.plus(TestKtInlineClassValue.Meters(x))\n",
        "this.total = this.total.plus(m.half)\n",
        "public fun boxed(m: pkg.TestKtInlineClassValue.Meters): Any {\n      return m\n",
        "return a == b\n",
        "val var10001: String = o.greet()\n",
        "return \"${m.toString()}${m.hashCode()}\"\n",
        "return m.scaled(3.0).v + m.v\n",
    ] {
        assert!(fixed.contains(written), "{written}\n{fixed}");
    }
    let outer = &fixed[..fixed.find("@JvmInline").expect("the value class")];
    for plumbing in ["constructor_impl", "box_impl", "unbox_impl", "equals_impl0", "_impl/*", "plus_HWDY4ac"] {
        assert!(!outer.contains(plumbing), "{plumbing}\n{outer}");
    }
}

#[test]
fn an_underlying_value_is_read_from_its_property() {
    let fixed = text_with(KunaFixes::DEFAULT);
    for read in [
        "return m.v * 2\n",
        "System.out.println(m.v)\n",
        "return \"len ${m.v}\"\n",
        "return (if (c) a else b).v\n",
    ] {
        assert!(fixed.contains(read), "{read}\n{fixed}");
    }
    let broken = text_with(KunaFixes::DEFAULT.without(KunaFix::KtInlineClassValue));
    for wrong in ["return m * 2\n", "System.out.println(m)\n", "return \"len $m\"\n", "return if (c) a else b\n"] {
        assert!(broken.contains(wrong), "{wrong}\n{broken}");
    }
    assert!(broken.contains("Meters.constructor_impl/* $VF was: constructor-impl */(0.0)"), "{broken}");
}
