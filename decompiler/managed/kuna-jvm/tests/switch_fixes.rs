//! The switch fixes on hand-built input: `type_switch_label_lookup` (`replace_labels` on case
//! lists, the multi-pattern guard strip, the unnamed-pattern types) and
//! `switch_yield_const_retype` (`ConstExprent::can_be_written_as`).

use kuna_jvm::code_constants::{CONSTANT_CLASS, CONSTANT_INTEGER};
use kuna_jvm::code_type::CodeType;
use kuna_jvm::const_exprent::{ConstExprent, ConstValue};
use kuna_jvm::exprent::Exprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::testing::{
    replace_labels, strip_multi_pattern_guards, unnamed_pattern_types,
};
use kuna_jvm::pooled_constant::PooledConstant;
use kuna_jvm::primitive_constant::{ConstValue as PoolValue, PrimitiveConstant};
use kuna_jvm::rt::JList;
use kuna_jvm::switch_statement::SwitchStatement;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common::{self, fixture_method, start_method};

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

/// One literal per type and value kind Vineflower's `ConstExprent.toJava` was run on; `false`
/// where it throws "invalid constant type".
fn literal_forms() -> Vec<(VarType, Option<ConstValue>, bool)> {
    let int = || Some(ConstValue::Int(1));
    let string = || Some(ConstValue::String("java/lang/String".into()));
    vec![
        (VarType::VARTYPE_BOOLEAN(), int(), true),
        (VarType::VARTYPE_BOOLEAN(), Some(ConstValue::Long(1)), true),
        (VarType::VARTYPE_BOOLEAN_OBJ(), int(), true),
        (VarType::VARTYPE_BYTECHAR(), int(), true),
        (VarType::VARTYPE_BYTE(), int(), true),
        (VarType::VARTYPE_BYTE_OBJ(), int(), true),
        (VarType::VARTYPE_CHARACTER(), int(), true),
        (VarType::VARTYPE_CHAR(), int(), true),
        (VarType::VARTYPE_CLASS(), None, true),
        (VarType::VARTYPE_CLASS(), string(), true),
        (
            VarType::VARTYPE_DOUBLE(),
            Some(ConstValue::Double(0.5)),
            true,
        ),
        (
            VarType::VARTYPE_DOUBLE_OBJ(),
            Some(ConstValue::Double(0.5)),
            true,
        ),
        (VarType::VARTYPE_FLOAT(), Some(ConstValue::Float(0.5)), true),
        (VarType::VARTYPE_FLOAT(), int(), true),
        (
            VarType::VARTYPE_FLOAT_OBJ(),
            Some(ConstValue::Float(0.5)),
            true,
        ),
        (VarType::VARTYPE_INTEGER(), int(), true),
        (VarType::VARTYPE_INTEGER(), None, true),
        (VarType::VARTYPE_INT(), int(), true),
        (VarType::VARTYPE_INT(), None, true),
        (VarType::VARTYPE_INT(), string(), true),
        (VarType::VARTYPE_LONG(), Some(ConstValue::Long(1)), true),
        (VarType::VARTYPE_LONG_OBJ(), Some(ConstValue::Long(1)), true),
        (
            VarType::VARTYPE_METHODTYPE(),
            Some(ConstValue::String("()V".into())),
            true,
        ),
        (VarType::VARTYPE_NULL(), int(), true),
        (VarType::VARTYPE_NULL(), None, true),
        (VarType::VARTYPE_OBJECT(), int(), false),
        (VarType::VARTYPE_OBJECT(), None, true),
        (VarType::VARTYPE_SHORTCHAR(), int(), true),
        (VarType::VARTYPE_SHORT(), int(), true),
        (VarType::VARTYPE_SHORT_OBJ(), int(), true),
        (VarType::VARTYPE_STRING(), None, true),
        (VarType::VARTYPE_STRING(), string(), true),
        (VarType::VARTYPE_UNKNOWN(), int(), false),
    ]
}

#[test]
fn writable_types_are_those_vineflower_writes() {
    let _guard = common::install_default_context();
    for (type_, value, written) in literal_forms() {
        let c = ConstExprent::new(type_.clone(), value.clone(), None);
        assert_eq!(c.can_be_written_as(&type_), written, "{type_} {value:?}");
    }
}

#[test]
fn a_literal_is_not_writable_as_a_join_of_unrelated_arms() {
    let _guard = common::install_default_context();
    let string = ConstExprent::new(
        VarType::VARTYPE_STRING(),
        Some(ConstValue::String("s".into())),
        None,
    );
    let class = ConstExprent::new(
        VarType::VARTYPE_CLASS(),
        Some(ConstValue::String("java/lang/String".into())),
        None,
    );
    let int = ConstExprent::new(VarType::VARTYPE_BYTECHAR(), Some(ConstValue::Int(97)), None);
    let joins = [
        VarType::VARTYPE_OBJECT(),
        VarType::new_signature("Ljava/io/Serializable;"),
        VarType::new_signature("Ljava/lang/Comparable;"),
        VarType::new_signature("Ljava/lang/Number;"),
        VarType::new_signature("TT;"),
    ];
    for join in &joins {
        assert!(!string.can_be_written_as(join), "String as {join}");
        assert!(!class.can_be_written_as(join), "Class as {join}");
        assert!(!int.can_be_written_as(join), "int as {join}");
    }
    assert!(string.can_be_written_as(&VarType::VARTYPE_STRING()));
    assert!(class.can_be_written_as(&VarType::VARTYPE_CLASS()));
    for primitive in [
        VarType::VARTYPE_CHAR(),
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_BOOLEAN(),
        VarType::VARTYPE_CHARACTER(),
        VarType::VARTYPE_INTEGER(),
    ] {
        assert!(int.can_be_written_as(&primitive), "int as {primitive}");
    }
    let no_value = ConstExprent::new(VarType::VARTYPE_STRING(), None, None);
    assert!(no_value.can_be_written_as(&VarType::new_signature("Ljava/io/Serializable;")));
    assert!(no_value.can_be_written_as(&VarType::new(CodeType::Null)));
}
