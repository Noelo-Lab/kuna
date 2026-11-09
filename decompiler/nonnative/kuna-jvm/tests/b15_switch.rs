//! `SwitchHelper` / `SwitchExpressionHelper` on hand-built statement trees: the
//! breadth-first exprent search, the Java 21 `ordinal()` enum switch resugaring
//! (against the `CfgCases$Color` fixture enum) and the early exits.

use std::path::PathBuf;

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::invocation_exprent::InvocationExprent;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::JList;
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::switch_expression_helper::SwitchExpressionHelper;
use kuna_jvm::switch_head_exprent::SwitchHeadExprent;
use kuna_jvm::switch_helper::SwitchHelper;
use kuna_jvm::switch_statement::SwitchStatement;
use kuna_jvm::var_exprent::VarExprent;

use crate::common::{self, ContextGuard};

const COLOR: &str = "CfgCases$Color";

fn fixture() -> (ContextGuard, StructMethod) {
    let guard = common::install_default_context();
    let sc = guard.struct_context();
    sc.add_space_file(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/cfg/classes/r8"),
        true,
    );
    let mt = sc
        .get_class(COLOR)
        .expect("fixture enum")
        .get_method("values", "()[LCfgCases$Color;")
        .expect("fixture method");
    (guard, mt)
}

fn bb_with(values: &[i32]) -> Statement {
    let stat = BasicBlockStatement::create().0;
    for &v in values {
        stat.get_exprents()
            .unwrap()
            .add(ConstExprent::new_int(v, false, None).0);
    }
    stat
}

/// `switch (var.ordinal())` over `labels` (`None` = `default`), as the only child of a root.
fn ordinal_switch(
    classname: &str,
    labels: &[Option<i32>],
    mt: StructMethod,
) -> (RootStatement, SwitchStatement, VarExprent) {
    let var = VarExprent::new(1, None, None);
    let ordinal = InvocationExprent::new();
    ordinal.set_name("ordinal".to_string());
    ordinal.set_classname(classname.to_string());
    ordinal.set_instance(Some(var.0));

    let sw = SwitchStatement::new_empty();
    let head = SwitchHeadExprent::new(ordinal.0, None);
    head.set_case_values(sw.get_case_values());
    sw.get_headexprent_list().add(head.0);
    for label in labels {
        sw.get_case_values().add(JList::from(vec![
            label.map(|v| ConstExprent::new_int(v, false, None).0)
        ]));
    }

    let root = RootStatement::new(sw.0, DummyExitStatement::new(), mt);
    (root, sw, var)
}

fn comments(root: RootStatement) -> Vec<String> {
    root.d(|d| d.comment_lines.as_ref().map(|c| c.to_vec()))
        .unwrap_or_default()
}

#[test]
fn find_exprents_is_breadth_first_and_can_stop_at_the_first_statement() {
    let _guard = common::install_default_context();
    let (b, c, d) = (bb_with(&[2, 1]), bb_with(&[3]), bb_with(&[4]));
    let inner = SequenceStatement::new_varargs(&[b, c]);
    let outer = SequenceStatement::new_varargs(&[inner.0, d]);

    let mut all = Vec::new();
    SwitchHelper::find_exprents(
        outer.0,
        Exprent::try_as_const,
        |k| k.get_int_value() > 1,
        false,
        |stat, k| all.push((stat, k.get_int_value())),
    );
    assert_eq!(all, vec![(d, 4), (b, 2), (c, 3)]);

    let mut first = Vec::new();
    SwitchHelper::find_exprents(
        outer.0,
        Some,
        |e: Exprent| e.try_as_const().is_some_and(|k| k.get_int_value() < 4),
        true,
        |stat, e| first.push((stat, e.as_const().get_int_value())),
    );
    assert_eq!(first, vec![(b, 2), (b, 1)]);
}

#[test]
fn java21_ordinal_switch_becomes_a_switch_on_the_enum() {
    let (_guard, mt) = fixture();
    let (root, sw, var) = ordinal_switch(COLOR, &[Some(0), Some(2), None], mt);

    assert!(SwitchHelper::simplify_switches(root.0, mt, root));

    let head = sw.get_headexprent().as_switch_head();
    assert_eq!(head.get_value(), var.0);
    assert!(head.get_case_values().ptr_eq(&sw.get_case_values()));

    let labels: Vec<Option<(String, String, bool, String)>> = sw
        .get_case_values()
        .iter()
        .map(|l| {
            assert_eq!(l.len(), 1);
            l.get(0).map(|e| {
                let f = e.as_field();
                (
                    f.get_name(),
                    f.get_classname(),
                    f.is_static(),
                    f.get_descriptor().descriptor_string.clone(),
                )
            })
        })
        .collect();
    let field = |name: &str| {
        Some((
            name.to_string(),
            COLOR.to_string(),
            true,
            format!("L{COLOR};"),
        ))
    };
    assert_eq!(labels, vec![field("RED"), field("BLUE"), None]);
    assert!(comments(root).is_empty());
}

#[test]
fn ordinal_switch_on_an_unknown_class_is_left_alone_with_a_comment() {
    let (_guard, mt) = fixture();
    let (root, sw, var) = ordinal_switch("p/Missing", &[Some(0), None], mt);
    let before = sw.get_headexprent().as_switch_head().get_value();

    assert!(!SwitchHelper::simplify_switches(root.0, mt, root));

    assert_eq!(sw.get_headexprent().as_switch_head().get_value(), before);
    assert_ne!(before, var.0);
    assert!(sw.get_case_values().get(0).get(0).unwrap().is_const());
    assert_eq!(
        comments(root),
        vec![
            "$VF: Unable to simplify switch-on-enum, as the enum class was not able to be found."
                .to_string()
        ]
    );
    assert!(root.d(|d| d.add_error_comment));
}

#[test]
fn switch_expressions_skip_phantom_string_hash_and_successorless_switches() {
    let (_guard, mt) = fixture();

    let (root, sw, _) = ordinal_switch(COLOR, &[Some(0), None], mt);
    sw.set_phantom(true);
    assert!(!SwitchExpressionHelper::process_switch_expressions(root.0));

    let (root, sw, _) = ordinal_switch(COLOR, &[Some(0), None], mt);
    let value = sw.get_headexprent().as_switch_head().get_value();
    value.as_invocation().set_name("hashCode".to_string());
    value
        .as_invocation()
        .set_classname("java/lang/String".to_string());
    assert!(!SwitchExpressionHelper::process_switch_expressions(root.0));

    let (root, sw, _) = ordinal_switch(COLOR, &[Some(0), None], mt);
    assert!(!SwitchExpressionHelper::process_switch_expressions(root.0));
    assert!(!sw.is_phantom());
}
