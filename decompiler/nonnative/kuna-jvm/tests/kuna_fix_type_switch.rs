//! The pattern-switch label fixes in isolation: `replace_labels` on hand-built
//! case lists, the multi-pattern guard strip and the unnamed-pattern types.

use kuna_jvm::code_constants::{CONSTANT_CLASS, CONSTANT_INTEGER};
use kuna_jvm::code_type::CodeType;
use kuna_jvm::const_exprent::{ConstExprent, ConstValue};
use kuna_jvm::exprent::Exprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::p8_structure::kuna_fix_type_switch_label_lookup::{
    replace_labels, strip_multi_pattern_guards, unnamed_pattern_types,
};
use kuna_jvm::pooled_constant::PooledConstant;
use kuna_jvm::primitive_constant::{ConstValue as PoolValue, PrimitiveConstant};
use kuna_jvm::rt::JList;
use kuna_jvm::switch_statement::SwitchStatement;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common;
use crate::k2_dom_helper::{fixture_method, start_method};

fn index(i: i32) -> Option<Exprent> {
    Some(ConstExprent::new_int(i, false, None).0)
}

fn int_arg(v: i32) -> Option<PooledConstant> {
    Some(PooledConstant::Primitive(PrimitiveConstant::new(
        CONSTANT_INTEGER,
        PoolValue::Integer(v),
    )))
}

fn class_arg(name: &str) -> Option<PooledConstant> {
    Some(PooledConstant::Primitive(PrimitiveConstant::new(
        CONSTANT_CLASS,
        PoolValue::String(name.into()),
    )))
}

fn switch_with(cases: Vec<Vec<Option<Exprent>>>) -> SwitchStatement {
    let sw = SwitchStatement::new_empty();
    for case in cases {
        sw.get_case_values().add(JList::from(case));
    }
    sw
}

fn labels(sw: SwitchStatement) -> Vec<Vec<String>> {
    sw.get_case_values()
        .iter()
        .map(|l| {
            l.iter()
                .map(|e| match e {
                    None => "default".to_string(),
                    Some(e) => match e.try_as_const().and_then(|c| c.get_value()) {
                        Some(ConstValue::Int(v)) => v.to_string(),
                        _ => match e.try_as_function() {
                            Some(f) if f.get_func_type() == FunctionType::Instanceof => format!(
                                "instanceof {}",
                                f.get_lst_operands().get(1).get_expr_type().value_str()
                            ),
                            _ => "other".to_string(),
                        },
                    },
                })
                .collect()
        })
        .collect()
}

fn selector() -> Exprent {
    VarExprent::new(1, Some(VarType::VARTYPE_OBJECT()), None).0
}

fn class_type(name: &str) -> VarType {
    VarType::new_value(CodeType::Object, 0, name)
}

fn type_test(name: &str) -> Exprent {
    FunctionExprent::new(
        FunctionType::Instanceof,
        vec![
            selector(),
            ConstExprent::new(class_type(name), None, None).0,
        ],
        None,
    )
    .0
}

fn not(e: Exprent) -> Exprent {
    FunctionExprent::new_single(FunctionType::BoolNot, e, None).0
}

fn and(a: Exprent, b: Exprent) -> Exprent {
    FunctionExprent::new(FunctionType::BooleanAnd, vec![a, b], None).0
}

#[test]
fn every_label_of_a_multi_label_case_is_replaced() {
    let _context = common::install_default_context();
    start_method(fixture_method());
    // case 5, 7 -> ...; case 1 -> ...; default
    let sw = switch_with(vec![vec![index(0), index(1)], vec![index(2)], vec![None]]);
    replace_labels(sw, &[int_arg(5), int_arg(7), int_arg(1)], selector(), false);
    assert_eq!(labels(sw), vec![vec!["5", "7"], vec!["1"], vec!["default"]]);
}

#[test]
fn a_substituted_integer_label_is_not_taken_for_an_index() {
    let _context = common::install_default_context();
    start_method(fixture_method());
    // case 1 -> ...; case Integer i -> ... (index 1, no cast); default
    let sw = switch_with(vec![vec![index(0)], vec![index(1)], vec![None]]);
    replace_labels(
        sw,
        &[int_arg(1), class_arg("java/lang/Integer")],
        selector(),
        false,
    );
    let got = labels(sw);
    assert_eq!(got[0], vec!["1"]);
    assert_eq!(got[1], vec!["instanceof java/lang/Integer"]);
}

#[test]
fn multi_pattern_case_loses_javacs_retest_and_prints_unnamed() {
    let _context = common::install_default_context();
    start_method(fixture_method());
    let sw = switch_with(vec![vec![index(0), index(1)], vec![None]]);
    replace_labels(
        sw,
        &[
            class_arg("java/lang/Integer"),
            class_arg("java/lang/String"),
        ],
        selector(),
        false,
    );
    let retest = not(and(
        not(type_test("java/lang/Integer")),
        not(type_test("java/lang/String")),
    ));
    sw.get_case_guards().add(Some(retest));
    strip_multi_pattern_guards(sw);
    assert!(sw.get_case_guards().get(0).is_none());

    let types = unnamed_pattern_types(&sw.get_case_values().get(0)).expect("two type patterns");
    let names: Vec<&str> = types.iter().map(|t| t.value_str()).collect();
    assert_eq!(names, vec!["java/lang/Integer", "java/lang/String"]);
    assert!(unnamed_pattern_types(&sw.get_case_values().get(1)).is_none());
}

#[test]
fn a_source_guard_survives_the_strip() {
    let _context = common::install_default_context();
    start_method(fixture_method());
    let sw = switch_with(vec![vec![index(0), index(1)], vec![None]]);
    replace_labels(
        sw,
        &[
            class_arg("java/lang/Integer"),
            class_arg("java/lang/String"),
        ],
        selector(),
        false,
    );
    let flag = VarExprent::new(2, Some(VarType::VARTYPE_BOOLEAN()), None).0;
    // !(!(o instanceof Integer || o instanceof String) || !flag): javac's test && flag
    let retest_or = FunctionExprent::new(
        FunctionType::BooleanOr,
        vec![
            type_test("java/lang/Integer"),
            type_test("java/lang/String"),
        ],
        None,
    )
    .0;
    let guard = not(FunctionExprent::new(
        FunctionType::BooleanOr,
        vec![not(retest_or), not(flag)],
        None,
    )
    .0);
    sw.get_case_guards().add(Some(guard));
    strip_multi_pattern_guards(sw);
    let kept = sw.get_case_guards().get(0).expect("source guard");
    assert!(kept.equals(flag));

    // a guard that is not javac's re-test is left alone
    let sw2 = switch_with(vec![vec![index(0), index(1)], vec![None]]);
    replace_labels(
        sw2,
        &[
            class_arg("java/lang/Integer"),
            class_arg("java/lang/String"),
        ],
        selector(),
        false,
    );
    sw2.get_case_guards().add(Some(type_test("java/lang/Long")));
    strip_multi_pattern_guards(sw2);
    assert!(sw2.get_case_guards().get(0).is_some());
}
