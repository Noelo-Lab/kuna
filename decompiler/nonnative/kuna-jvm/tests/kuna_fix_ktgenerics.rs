//! The fixes of ktgenerics on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`), and the widening conversion table.

use std::path::Path;

use crate::common;

use kuna_jvm::code_type::CodeType;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fix_kt_implicit_widening::conversion;
use kuna_jvm::kuna_fix_kt_java_cast_syntax::KotlinCast;
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

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

fn check(id: &str, fix: KunaFix, pairs: &[(&str, &str)]) {
    let (on, off) = on_off(id, fix);
    for (fixed, lost) in pairs {
        assert!(on.contains(fixed) && !on.contains(lost), "{fixed}\n{on}");
        assert!(off.contains(lost) && !off.contains(fixed), "{lost}\n{off}");
    }
}

#[test]
fn implicit_widening_writes_the_conversion() {
    check(
        "pkg/TestKtImplicitWidening",
        KunaFix::KtImplicitWidening,
        &[
            (
                "val b0: Int = data[i++].toInt()",
                "val b0: Int = data[i++]\n",
            ),
            ("this.takeInt(b.toInt())", "this.takeInt(b)"),
            ("return s.toInt()", "return s\n"),
            ("this.count = data[0].toInt()", "this.count = data[0]\n"),
            ("return c.code.toDouble()", "return c\n"),
        ],
    );
}

#[test]
fn monitor_calls_cast_the_receiver() {
    check(
        "pkg/TestKtObjectMonitorCall",
        KunaFix::KtObjectMonitorCall,
        &[
            (
                "(this as java.lang.Object).wait(ms, ns)",
                "this.wait(ms, ns)",
            ),
            ("(this as java.lang.Object).notify()", "this.notify()"),
            (
                "(var10000 as java.lang.Object).notifyAll()",
                "var10000.notifyAll()",
            ),
        ],
    );
}

#[test]
fn generic_locals_get_type_arguments() {
    check(
        "pkg/TestKtRawGenericType",
        KunaFix::KtRawGenericType,
        &[
            (
                "val result: MutableList<String> = ArrayList()\n",
                "val result: java.util.List = ArrayList() as java.util.List",
            ),
            (
                "val matches: List<String> = this.findMatching(prefix)",
                "val matches: java.util.List =",
            ),
            (
                "val i: MutableIterator<String> = this.names.iterator()",
                "val i: java.util.Iterator = this.names.iterator()",
            ),
            (
                "is Collection<*> && (`$this$none$iv` as Collection<String>)",
                "is java.util.Collection",
            ),
            (
                "val last: ObjectRef<Any?> = ObjectRef()",
                "val last: ObjectRef = ObjectRef()",
            ),
            ("var c: Class<*> =", "var c: Class ="),
            (
                "total += i.next().length",
                "total += (i.next() as String).length",
            ),
        ],
    );
}

#[test]
fn widening_conversions_follow_java() {
    let _guard = common::install_default_context();
    let suffix = |to: CodeType, from: CodeType| {
        let value = VarExprent::new(1, Some(VarType::new(from)), None).0;
        match conversion(value, &VarType::new(to), &VarType::new(from)) {
            Some(KotlinCast::Convert(s)) => Some(s),
            _ => None,
        }
    };
    assert_eq!(suffix(CodeType::Int, CodeType::Byte), Some(".toInt()"));
    assert_eq!(suffix(CodeType::Int, CodeType::Char), Some(".code"));
    assert_eq!(suffix(CodeType::Long, CodeType::Short), Some(".toLong()"));
    assert_eq!(
        suffix(CodeType::Double, CodeType::Char),
        Some(".code.toDouble()")
    );
    assert_eq!(suffix(CodeType::Short, CodeType::Byte), Some(".toShort()"));
    assert_eq!(suffix(CodeType::Char, CodeType::Byte), None);
    assert_eq!(suffix(CodeType::Short, CodeType::Char), None);
    assert_eq!(suffix(CodeType::Int, CodeType::Long), None);
    assert_eq!(suffix(CodeType::Int, CodeType::Int), None);
    let constant = ConstExprent::new_int(7, false, None).0;
    assert!(conversion(
        constant,
        &VarType::new(CodeType::Long),
        &VarType::new(CodeType::Int)
    )
    .is_none());
}
