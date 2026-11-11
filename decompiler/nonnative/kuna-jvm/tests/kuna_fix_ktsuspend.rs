//! The suspend-function fixes of ktsuspend on their kuna-fixes cases, each with only its own fix
//! turned off (`KunaFixes::without`), and the function-type `invoke` predicate.

use std::path::Path;

use kuna_jvm::kotlin::kuna_fix_kt_suspend_state_machine::is_function_invoke;
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

/// The case's default text, and its text with `fix` alone off.
fn on_off(id: &str, fix: KunaFix) -> (String, String) {
    let text = |fixes| {
        let mut case = fix_case(id);
        case.fixes = Some(fixes);
        with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
    };
    (
        text(KunaFixes::DEFAULT),
        text(KunaFixes::DEFAULT.without(fix)),
    )
}

fn check(id: &str, fix: KunaFix, pairs: &[(&str, &str)]) -> (String, String) {
    let (on, off) = on_off(id, fix);
    for (fixed, lost) in pairs {
        assert!(on.contains(fixed) && !on.contains(lost), "{fixed}\n{on}");
        assert!(off.contains(lost) && !off.contains(fixed), "{lost}\n{off}");
    }
    (on, off)
}

const MACHINE: &str = "object : ContinuationImpl(`$completion`)";

#[test]
fn state_machines_become_direct_bodies() {
    let (on, off) = check(
        "pkg/TestSuspendStateMachineKt",
        KunaFix::KtSuspendStateMachine,
        &[
            (
                "   val a: Int = src.next()\n   val d: Double = a * 1.5\n",
                "`$continuation`.`D$0` = d",
            ),
            (
                "   try {\n      val t: String = src.text(src.next())\n",
                "return closing(null, null, this",
            ),
        ],
    );
    assert_eq!(on.matches(MACHINE).count(), 1, "only handOff keeps its machine\n{on}");
    assert_eq!(off.matches(MACHINE).count(), 10, "{off}");
    assert!(on.contains("public suspend fun handOff(x: Int): Int {\n   var `$continuation`"));
    assert!(on.contains("return \"${src.next()}:${kotlin.coroutines.coroutineContext"));
}

#[test]
fn member_machines_become_direct_bodies() {
    let (on, off) = check(
        "pkg/TestSuspendMemberMachine",
        KunaFix::KtSuspendStateMachine,
        &[(
            "this.total = this.total + (this.src.next() as Number).toLong()",
            "`$continuation`.`J$0` = ",
        )],
    );
    assert!(!on.contains(MACHINE));
    assert_eq!(off.matches(MACHINE).count(), 2);
}

#[test]
fn unit_tail_call_loses_its_check() {
    check(
        "pkg/TestSuspendTailCallKt",
        KunaFix::KtSuspendStateMachine,
        &[(
            "   src.done()\n   return Unit.INSTANCE\n",
            "=== IntrinsicsKt.getCOROUTINE_SUSPENDED()",
        )],
    );
}

#[test]
fn suspend_calls_leave_out_the_continuation() {
    check(
        "pkg/TestSuspendTailCallKt",
        KunaFix::KtSuspendCallContinuation,
        &[
            ("return src.next()\n", "return src.next(`$completion`)"),
            ("return f(src)\n", "return f(src, `$completion`)"),
            ("return kotlin.coroutines.coroutineContext", "`$completion`.getContext()"),
        ],
    );
    check(
        "pkg/TestSuspendStateMachineKt",
        KunaFix::KtSuspendCallContinuation,
        &[(
            "val s: String = src.text(a + b)\n",
            "src.text(a + b, `$completion`)",
        )],
    );
    check(
        "pkg/TestSuspendMemberMachine",
        KunaFix::KtSuspendCallContinuation,
        &[(
            "val a: Int = this.src.next()\n",
            "val a: Int = (this.src.next(`$completion`) as Number).toInt()",
        )],
    );
}

#[test]
fn coroutine_boxing_is_implicit() {
    check(
        "pkg/TestCoroutineBoxingKt",
        KunaFix::KtCoroutineBoxing,
        &[
            ("return items.size\n", "Boxing.boxInt(items.size)"),
            ("return items.isEmpty()\n", "Boxing.boxBoolean(items.isEmpty())"),
        ],
    );
}

#[test]
fn suspend_function_types_are_written_suspend() {
    check(
        "pkg/TestSuspendFunctionType",
        KunaFix::KtSuspendFunctionType,
        &[
            ("val action: suspend (String) -> Int", "val action: (String, Continuation<Int>) -> Any?"),
            ("var hook: (suspend () -> Unit)?", "var hook: ((Continuation<Unit>) -> Any?)?"),
            (
                "b: suspend (Int, String) -> Boolean",
                "b: (Int, String, Continuation<Boolean>) -> Any?",
            ),
        ],
    );
}

#[test]
fn function_type_invoke_is_recognised() {
    let obj = "Ljava/lang/Object;";
    assert!(is_function_invoke("kotlin/jvm/functions/Function1", "invoke", &format!("({obj}){obj}")));
    assert!(is_function_invoke(
        "kotlin/jvm/functions/Function3",
        "invoke",
        &format!("({obj}{obj}{obj}){obj}")
    ));
    assert!(!is_function_invoke("kotlin/jvm/functions/Function0", "invoke", &format!("(){obj}")));
    assert!(!is_function_invoke("kotlin/jvm/functions/Function2", "invoke", &format!("({obj}){obj}")));
    assert!(!is_function_invoke("kotlin/jvm/functions/Function1", "apply", &format!("({obj}){obj}")));
    assert!(!is_function_invoke("pkg/Function1", "invoke", &format!("({obj}){obj}")));
}
