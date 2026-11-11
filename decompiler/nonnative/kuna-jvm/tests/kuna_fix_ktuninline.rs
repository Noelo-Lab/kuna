//! The fix `kt_inlined_stdlib_call` on its kuna-fixes case: with it, the
//! inlined copies are the calls; without it, Vineflower's copies.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const CASE: &str = "kuna-fixes/pkg/TestKtInlinedStdlibCall";

#[test]
fn inlined_copies_are_the_calls() {
    let fixed = text_with(CASE, KunaFixes::DEFAULT);
    for call in [
        "return String.format(\"%s-%d\", name, n)",
        "return String.format(Locale.US, \"%08x\", n)",
        "return fmt.format(n)",
        "${i.toString(16)}${l.toString(2)}${b.toString(16)}",
        "return s.toInt(16) + s.toLong(8).toInt()",
        "return a.copyOfRange(from, a.size)",
        "return r.use {",
        "var var12: Int = r.use { res ->",
        "r.use { res ->\n         out.add(res.text())\n         out.add(res.name)\n      }",
        "r.use {\n         return it.text()\n      }",
        "return xs.map { it.name }",
        "return xs.map { s -> s.length + 1 }",
    ] {
        assert!(fixed.contains(call), "{call}\n{fixed}");
    }
    for copy in ["checkRadix", "closeFinally", "Arrays.copyOf", "copyOfRangeImpl", "StringCompanionObject"] {
        assert!(!fixed.contains(copy), "{copy}\n{fixed}");
    }
    assert_eq!(fixed.matches("collectionSizeOrDefault").count(), 1, "{fixed}");
}

#[test]
fn without_the_fix_the_copies_stay() {
    let copies = text_with(CASE, KunaFixes::DEFAULT.without(KunaFix::KtInlinedStdlibCall));
    for copy in [
        "java.lang.String.format(\"%s-%d\", Arrays.copyOf(var5, var5.size))",
        "Integer.toString(i, CharsKt.checkRadix(16))",
        "CloseableKt.closeFinally(var2, var3)",
        "ArraysKt.copyOfRange(a, from, a.size)",
    ] {
        assert!(copies.contains(copy), "{copy}\n{copies}");
    }
    assert!(!copies.contains(".use {") && !copies.contains(".map {"), "{copies}");
}
