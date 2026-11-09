//! `StackVarsProcessor.setVersionsToNull` and `PPandMMHelper.inlinePPIandMMIIf`
//! on hand-built statement trees.

use kuna_jvm::assignment_exprent::AssignmentExprent;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::if_exprent::{self, IfExprent};
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::p_pand_mm_helper::PPandMMHelper;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{try_catch, JList};
use kuna_jvm::stack_vars_processor::StackVarsProcessor;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common::ContextGuard;
use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};

fn setup() -> (ContextGuard, StructMethod) {
    let context = oracle_context();
    let mt = fixture_method();
    start_method(mt);
    (context, mt)
}

fn var(index: i32, version: i32) -> VarExprent {
    let v = VarExprent::new(index, Some(VarType::VARTYPE_INT()), None);
    v.set_version(version);
    v
}

fn if_exprent(condition: Exprent) -> IfExprent {
    let mut stack = ListStack::new();
    stack.push(condition);
    IfExprent::new(if_exprent::Type::Value, &mut stack, None)
}

/// `if (<condition>) {}` whose head block holds `exprents`, under a root.
fn if_tree(exprents: Vec<Exprent>, condition: Exprent) -> (RootStatement, IfStatement, Statement) {
    let bb = BasicBlockStatement::create().0;
    bb.set_exprents(JList::from(exprents));

    let ifstat = IfStatement::new_empty();
    ifstat.get_headexprent_list().add(if_exprent(condition).0);
    ifstat.set_first(bb);
    ifstat.get_stats().borrow_mut().add_with_key(bb, bb.id());
    bb.set_parent(Some(ifstat.0));

    let root = RootStatement::new(ifstat.0, DummyExitStatement::new(), fixture_method());
    (root, ifstat, bb)
}

fn gt_zero(operand: Exprent) -> Exprent {
    let zero = ConstExprent::new_int(0, true, None).0;
    FunctionExprent::new(FunctionType::Gt, vec![operand, zero], None).0
}

fn ppi(index: i32) -> FunctionExprent {
    FunctionExprent::new_single(FunctionType::Ppi, var(index, 0).0, None)
}

/// `inlinePPIandMMIIf` ends in `SequenceHelper.condenseSequences`, which may
/// not be ported yet; the transformation itself happens before it.
fn run_inline(root: RootStatement) -> Option<bool> {
    match try_catch(|| PPandMMHelper::inline_pp_iand_mmi_if(root)) {
        Ok(res) => Some(res),
        Err(caught) => {
            assert!(
                caught.message.contains("SequenceHelper"),
                "unexpected failure: {}",
                caught.message
            );
            None
        }
    }
}

#[test]
fn set_versions_to_null_clears_block_and_head_exprents() {
    let _context = setup();
    let (a, b, c) = (var(1, 3), var(2, 5), var(3, 7));
    let sum = FunctionExprent::new(FunctionType::Add, vec![b.0, c.0], None);
    let assign = AssignmentExprent::new(a.0, sum.0, None);
    let head = var(4, 9);

    let (root, _ifstat, _bb) = if_tree(vec![assign.0], head.0);

    StackVarsProcessor::set_versions_to_null(root.0);

    for v in [a, b, c, head] {
        assert_eq!(v.get_version(), 0, "version of var{}", v.get_index());
    }
}

#[test]
fn inline_ppi_moves_increment_into_if_condition() {
    let _context = setup();
    let inc = ppi(1);
    let use_ = var(1, 0);
    let condition = gt_zero(use_.0);
    let (root, ifstat, bb) = if_tree(vec![inc.0], condition);

    if let Some(res) = run_inline(root) {
        assert!(res);
    }

    let operands = condition.as_function().get_lst_operands().to_vec();
    assert_eq!(operands[0], inc.0);
    assert!(bb.get_exprents().expect("block exprents").is_empty());
    assert!(ifstat.has_ppmm());
}

#[test]
fn inline_ppi_refuses_nested_short_circuit_conditions() {
    let _context = setup();
    let inc = ppi(1);
    let both = FunctionExprent::new(
        FunctionType::BooleanAnd,
        vec![gt_zero(var(1, 0).0), gt_zero(var(2, 0).0)],
        None,
    );
    let either = FunctionExprent::new(
        FunctionType::BooleanOr,
        vec![both.0, gt_zero(var(3, 0).0)],
        None,
    );
    let (root, ifstat, bb) = if_tree(vec![inc.0], either.0);

    assert_eq!(run_inline(root), Some(false));
    assert_eq!(
        bb.get_exprents().expect("block exprents").to_vec(),
        vec![inc.0]
    );
    assert!(!ifstat.has_ppmm());
}

/// Java scans `ifFunc.getAllExprents(true)`, which leaves out `ifFunc` itself:
/// a top-level `&&` is not seen and the increment is inlined into it.
#[test]
fn inline_ppi_does_not_see_a_top_level_short_circuit() {
    let _context = setup();
    let inc = ppi(1);
    let left = gt_zero(var(1, 0).0);
    let both = FunctionExprent::new(
        FunctionType::BooleanAnd,
        vec![left, gt_zero(var(2, 0).0)],
        None,
    );
    let (root, ifstat, bb) = if_tree(vec![inc.0], both.0);

    if let Some(res) = run_inline(root) {
        assert!(res);
    }

    assert_eq!(left.as_function().get_lst_operands().get(0), inc.0);
    assert!(bb.get_exprents().expect("block exprents").is_empty());
    assert!(ifstat.has_ppmm());
}

#[test]
fn inline_ppi_refuses_a_variable_used_twice() {
    let _context = setup();
    let inc = ppi(1);
    let sum = FunctionExprent::new(FunctionType::Add, vec![var(1, 0).0, var(1, 0).0], None);
    let (root, _ifstat, bb) = if_tree(vec![inc.0], gt_zero(sum.0));

    assert_eq!(run_inline(root), Some(false));
    assert_eq!(bb.get_exprents().expect("block exprents").len(), 1);
}

#[test]
fn inline_ppi_ignores_other_last_exprents() {
    let _context = setup();
    let assign = AssignmentExprent::new(var(1, 0).0, var(2, 0).0, None);
    let (root, _ifstat, bb) = if_tree(vec![assign.0], gt_zero(var(1, 0).0));

    assert_eq!(run_inline(root), Some(false));
    assert_eq!(bb.get_exprents().expect("block exprents").len(), 1);
}
