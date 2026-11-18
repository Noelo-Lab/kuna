//! The fixes of ktsuspend2 (suspend lambdas, suspend functions without
//! metadata, `$$forInline` copies, `$suspendImpl` bodies, the `Nothing`
//! throw) on their kuna-fixes cases, each with only its own fix turned off.

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

/// The case's default text and its text with `fix` alone off; each `fixed`
/// is only in the first, each `lost` only in the second.
fn check(id: &str, fix: KunaFix, fixed: &[&str], lost: &[&str]) -> (String, String) {
    let text = |fixes| {
        let mut case = fix_case(id);
        case.fixes = Some(fixes);
        with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
    };
    let (on, off) = (text(KunaFixes::DEFAULT), text(KunaFixes::DEFAULT.without(fix)));
    for f in fixed {
        assert!(on.contains(f) && !off.contains(f), "{f}\n{on}\n----\n{off}");
    }
    for l in lost {
        assert!(off.contains(l) && !on.contains(l), "{l}\n{on}\n----\n{off}");
    }
    (on, off)
}

#[test]
fn nothing_call_ends_the_branch() {
    let (on, off) = check(
        "pkg/TestNothingValueThrowKt",
        KunaFix::KtNothingValueThrow,
        &["   if (x < 0) {\n      fail(\"negative\")\n   } else {"],
        &["throw KotlinNothingValueException()"],
    );
    assert_eq!(off.matches("throw KotlinNothingValueException()").count(), 4, "{off}");
    assert!(on.contains("   if (v < 0) {\n      src.never()\n   } else {"), "{on}");
}

#[test]
fn sam_emit_is_a_suspend_function() {
    let (on, _) = check(
        "pkg/TestSuspendSamKt",
        KunaFix::KtSuspendWithoutMetadata,
        &[
            "override final suspend fun emit(v: Int) {\n         out.emit(v)\n         out.emit(v * 2)",
            "override final suspend fun emit(s: String) {",
        ],
        &["`$completion`: Continuation<in Unit>): Any"],
    );
    assert!(!on.contains("label"), "{on}");
}

#[test]
fn for_inline_copies_are_not_written() {
    check(
        "pkg/TestForInlineCopyKt",
        KunaFix::KtForInlineCopy,
        &[],
        &["fun `collect$$forInline`(", "fun `emit$$forInline`("],
    );
}

#[test]
fn suspend_lambdas_are_rebuilt() {
    let (on, off) = check(
        "pkg/TestSuspendLambdaKt",
        KunaFix::KtSuspendLambda,
        &[
            "   return { s: String ->\n      s.length + src.next()\n   }",
            "   return pump_1@{ n: Int ->\n      repeat(n) { i ->\n         this@pump_1.emit(src.next())",
            "      runTwice({\n         this@nested_1.emit(src.next())\n      })",
            "      helper(src) * 2\n",
            "            return@lambda_1 v\n",
        ],
        &["Could not decompile lambda"],
    );
    assert!(!on.contains(" as (") && off.contains(" as ("), "{on}");
    assert_eq!(off.matches("Could not decompile lambda").count(), 10, "{off}");
}

#[test]
fn suspend_impl_bodies_are_written() {
    check(
        "pkg/TestSuspendImpl",
        KunaFix::KtSuspendImpl,
        &["   public open suspend fun first(): Int {\n      return this.src.next() + this.offset()\n   }"],
        &["`sum$suspendImpl`(this, n)", "`first$suspendImpl`(this)"],
    );
}
