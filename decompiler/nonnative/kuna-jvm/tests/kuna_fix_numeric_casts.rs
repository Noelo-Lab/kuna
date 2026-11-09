//! The fixes `var_merge_narrowing`, `switch_expression_poly_cast` and
//! `ternary_numeric_arms`: their type tests in isolation, and their
//! kuna-fixes cases in both modes.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack};
use kuna_jvm::p6_variables::kuna_fix_var_merge_narrowing::{narrows, strictly_below};
use kuna_jvm::substrate::kuna_fix_ternary_numeric_arms::promotes;
use kuna_jvm::var_type::VarType;

use crate::common;

/// The text the `<set>/<id>` kuna-fixes case prints.
fn decompile(set: &str, id: &str) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let id = format!("{set}/{id}");
    let case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("no case {id}"));
    with_engine_stack(move || run_case(&case))
        .actual
        .expect("output")
}

fn both(id: &str) -> (String, String) {
    let compat = decompile("kuna-fixes-compat", id);
    let fixed = decompile("kuna-fixes", id);
    assert_eq!(count_failed_methods(&compat), 0);
    assert_eq!(count_failed_methods(&fixed), 0);
    (compat, fixed)
}

#[test]
fn strictly_below_is_a_proper_reference_subtype() {
    let _guard = common::install_default_context();
    let object = VarType::VARTYPE_OBJECT();
    let integer = VarType::VARTYPE_INTEGER();
    let strings = VarType::VARTYPE_STRING().resize_array_dim(1);
    assert!(strictly_below(&integer, &object));
    assert!(strictly_below(&strings, &object));
    assert!(!strictly_below(&object, &integer));
    assert!(!strictly_below(&integer, &integer));
    assert!(!strictly_below(&VarType::VARTYPE_INT(), &object));
    assert!(!strictly_below(&integer, &VarType::VARTYPE_NULL()));
    assert!(!strictly_below(&integer, &VarType::VARTYPE_STRING()));
    assert!(narrows(&integer, &integer, &object));
    assert!(narrows(&integer, &object, &integer));
    assert!(!narrows(&object, &integer, &object));
}

#[test]
fn promotes_is_javac_typing_operands_together() {
    let int = Some(VarType::VARTYPE_INT());
    let long = Some(VarType::VARTYPE_LONG());
    let integer = Some(VarType::VARTYPE_INTEGER());
    let boolean = Some(VarType::VARTYPE_BOOLEAN());
    let boxed_boolean = Some(VarType::VARTYPE_BOOLEAN_OBJ());
    let object = Some(VarType::VARTYPE_OBJECT());
    assert!(promotes(&[int.clone(), long.clone()]));
    assert!(promotes(&[int.clone(), integer.clone()]));
    assert!(promotes(&[
        integer.clone(),
        Some(VarType::VARTYPE_LONG_OBJ())
    ]));
    assert!(promotes(&[boolean.clone(), boxed_boolean.clone()]));
    assert!(promotes(&[
        int.clone(),
        Some(VarType::VARTYPE_CHAR()),
        long.clone()
    ]));
    assert!(!promotes(&[int.clone(), int.clone()]));
    assert!(!promotes(&[integer.clone(), integer.clone()]));
    assert!(!promotes(&[int.clone(), object.clone()]));
    assert!(!promotes(&[boolean.clone(), int.clone()]));
    assert!(!promotes(&[int.clone(), None]));
    assert!(!promotes(&[]));
}

#[test]
fn a_cast_stays_after_the_check_its_value_passes() {
    let (compat, fixed) = both("pkg/TestSuspendResultCast");
    assert!(
        compat.contains("Integer var10 = (Integer)src.next($continuation);"),
        "{compat}"
    );
    assert!(
        fixed.contains("Object var10 = src.next($continuation);"),
        "{fixed}"
    );
    assert!(fixed.contains("Integer var11 = (Integer)var10;"), "{fixed}");
    assert!(!fixed.contains(")src.next("), "{fixed}");

    let (compat, fixed) = both("pkg/TestVarMergeNarrowing");
    assert!(
        compat.contains("String var2 = (String)var1.next();"),
        "{compat}"
    );
    assert!(fixed.contains("Object var2 = var1.next();"), "{fixed}");
    assert!(
        fixed.contains("String var3 = (String)var1.next();"),
        "{fixed}"
    );
}

#[test]
fn a_switch_expression_with_promoted_results_keeps_its_context() {
    let (compat, fixed) = both("pkg/TestSwitchExprPolyCast");
    assert_eq!(compat.matches("(switch (").count(), 3, "{compat}");
    assert!(!fixed.contains(")(switch ("), "{fixed}");
    assert!(fixed.contains("take(switch (x) {"), "{fixed}");
}

#[test]
fn a_reference_conditional_keeps_an_arm_cast() {
    let (compat, fixed) = both("pkg/TestTernaryNumericArms");
    for (old, new) in [
        ("return c ? 1 : 2L;", "return c ? (Object)1 : 2L;"),
        ("return c ? 1 : a;", "return c ? (Integer)1 : a;"),
        ("return c ? true : b;", "return c ? (Boolean)true : b;"),
        ("return c ? d : 0.5;", "return c ? d : (Double)0.5;"),
    ] {
        assert!(compat.contains(old), "{old}: {compat}");
        assert!(fixed.contains(new), "{new}: {fixed}");
    }
    assert!(fixed.contains("return c ? i : l;"), "{fixed}");
}
