//! Loop shaping (`MergeHelper`): direct paths, empty-statement removal and the
//! while-header match, on hand-built statement graphs.

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::do_statement::{DoStatement, Type as LoopType};
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::if_exprent::{IfExprent, Type as IfType};
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::merge_helper::MergeHelper;
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{EdgeDirection, Statement};
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common::ContextGuard;
use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};

fn setup() -> ContextGuard {
    let context = oracle_context();
    start_method(fixture_method());
    context
}

fn bb() -> Statement {
    BasicBlockStatement::create().0
}

fn edge(type_: i32, source: Statement, destination: Statement) -> StatEdge {
    let e = StatEdge::new(type_, source, destination);
    source.add_successor(e);
    e
}

fn labeled(type_: i32, source: Statement, destination: Statement, closure: Statement) -> StatEdge {
    let e = StatEdge::new_closure(type_, source, destination, Some(closure));
    source.add_successor(e);
    e
}

fn var(index: i32) -> Exprent {
    VarExprent::new(index, Some(VarType::VARTYPE_BOOLEAN()), None).0
}

/// `do { if (<var1>) <ifedge>; <elseedge> } <out>` inside `{ do; after }`; the
/// if head is an empty basic block.
struct HeadIf {
    dostat: DoStatement,
    ifs: IfStatement,
    head: Statement,
    after: Statement,
    out: StatEdge,
    cond: Exprent,
}

fn head_if_loop() -> HeadIf {
    let (head, after) = (bb(), bb());
    let cond = var(1);
    let mut stack = ListStack::new();
    stack.push(cond);
    let ifexpr = IfExprent::new(IfType::Value, &mut stack, None);

    let ifs = IfStatement::new_empty();
    ifs.set_first(head);
    ifs.get_stats().borrow_mut().add_with_key(head, head.id());
    ifs.get_headexprent_list().add(ifexpr.0);
    ifs.set_all_parent();

    let dostat = DoStatement::new(ifs.0);
    dostat.set_all_parent();
    let seq = SequenceStatement::new(vec![dostat.0, after]);
    seq.set_all_parent();
    let out = edge(StatEdge::TYPE_REGULAR, dostat.0, after);

    HeadIf {
        dostat,
        ifs,
        head,
        after,
        out,
        cond,
    }
}

#[test]
fn direct_path_follows_edges_then_parents() {
    let _context = setup();
    let (a, b, c) = (bb(), bb(), bb());
    edge(StatEdge::TYPE_REGULAR, a, b);
    assert!(MergeHelper::is_direct_path(a, b));
    assert!(!MergeHelper::is_direct_path(a, c));

    let exit = DummyExitStatement::new();
    assert!(MergeHelper::is_direct_path(c, exit.0));
    assert!(!MergeHelper::is_direct_path(c, a));

    let (body, other) = (bb(), bb());
    let dostat = DoStatement::new(body);
    dostat.set_all_parent();
    assert!(MergeHelper::is_direct_path(body, dostat.0));
    assert!(!MergeHelper::is_direct_path(body, other));

    let (p, q, r) = (bb(), bb(), bb());
    let seq = SequenceStatement::new(vec![p, q]);
    seq.set_all_parent();
    edge(StatEdge::TYPE_CONTINUE, seq.0, r);
    assert!(MergeHelper::is_direct_path(q, r));
    edge(StatEdge::TYPE_EXCEPTION, q, p);
    assert!(
        MergeHelper::is_direct_path(q, r),
        "exception edges are not direct"
    );
}

#[test]
fn removing_the_loop_head_leaves_an_empty_block() {
    let _context = setup();
    let body = bb();
    let dostat = DoStatement::new(body);
    dostat.set_all_parent();
    let cont = labeled(StatEdge::TYPE_CONTINUE, body, dostat.0, dostat.0);

    MergeHelper::remove_last_empty_statement(dostat, body);

    let first = dostat.get_first().unwrap();
    assert_ne!(first, body);
    assert!(first.is_basic_block());
    assert_eq!(first.get_parent(), Some(dostat.0));
    assert_eq!(dostat.get_stats().borrow().to_vec(), vec![first]);
    assert_eq!(cont.get_source(), first);
    assert_eq!(first.get_all_successor_edges(), vec![cont]);
    assert!(body.get_all_successor_edges().is_empty());
}

#[test]
fn removing_a_trailing_statement_turns_its_entries_into_continues() {
    let _context = setup();
    let (a, e) = (bb(), bb());
    let seq = SequenceStatement::new(vec![a, e]);
    seq.set_all_parent();
    let dostat = DoStatement::new(seq.0);
    dostat.set_all_parent();
    let ae = edge(StatEdge::TYPE_REGULAR, a, e);

    MergeHelper::clean_empty_statements(dostat, Some(e));

    assert_eq!(ae.get_type(), StatEdge::TYPE_CONTINUE);
    assert_eq!(ae.get_source(), a);
    assert_eq!(ae.get_destination(), dostat.0);
    assert_eq!(ae.closure(), Some(dostat.0));
    assert!(dostat.get_label_edges().borrow().contains(&ae));
    assert_eq!(
        dostat.get_predecessor_edges(StatEdge::TYPE_CONTINUE),
        vec![ae]
    );
    assert_eq!(a.get_successor_edges(StatEdge::TYPE_CONTINUE), vec![ae]);
    assert!(a.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty());
    assert!(e.get_all_predecessor_edges().is_empty());
    assert_eq!(seq.get_stats().borrow().to_vec(), vec![a]);
}

#[test]
fn removing_an_if_body_clears_the_if_slot() {
    let _context = setup();
    let (h, t) = (bb(), bb());
    let ifs = IfStatement::new_empty();
    ifs.set_first(h);
    ifs.get_stats().borrow_mut().add_with_key(h, h.id());
    ifs.get_stats().borrow_mut().add_with_key(t, t.id());
    ifs.set_ifstat(Some(t));
    ifs.set_all_parent();
    let dostat = DoStatement::new(ifs.0);
    dostat.set_all_parent();
    let ht = edge(StatEdge::TYPE_REGULAR, h, t);

    MergeHelper::remove_last_empty_statement(dostat, t);

    assert_eq!(ifs.get_ifstat(), None);
    assert_eq!(ifs.get_stats().borrow().to_vec(), vec![h]);
    assert_eq!(ht.get_type(), StatEdge::TYPE_CONTINUE);
    assert_eq!(ht.get_destination(), dostat.0);
}

#[test]
fn cleaning_an_empty_statement_drops_its_first_successor() {
    let _context = setup();
    let (a, e) = (bb(), bb());
    let seq = SequenceStatement::new(vec![a, e]);
    seq.set_all_parent();
    let dostat = DoStatement::new(seq.0);
    dostat.set_all_parent();
    let ae = edge(StatEdge::TYPE_REGULAR, a, e);
    let x = bb();
    edge(StatEdge::TYPE_REGULAR, e, x);

    MergeHelper::clean_empty_statements(dostat, Some(e));

    assert!(e.get_all_successor_edges().is_empty());
    assert!(x.get_all_predecessor_edges().is_empty());
    assert_eq!(ae.get_type(), StatEdge::TYPE_CONTINUE);
    assert_eq!(seq.get_stats().borrow().to_vec(), vec![a]);

    let full = bb();
    full.get_exprents().unwrap().add(var(2));
    let fe = edge(StatEdge::TYPE_REGULAR, full, a);
    MergeHelper::clean_empty_statements(dostat, Some(full));
    assert_eq!(
        full.get_all_successor_edges(),
        vec![fe],
        "non-empty: untouched"
    );
}

#[test]
fn head_if_with_break_becomes_a_negated_while() {
    let _context = setup();
    let HeadIf {
        dostat,
        ifs,
        head,
        after,
        out,
        cond,
    } = head_if_loop();
    let ifedge = labeled(StatEdge::TYPE_BREAK, head, after, dostat.0);
    ifs.set_if_edge(Some(ifedge));
    let cont = labeled(StatEdge::TYPE_CONTINUE, ifs.0, dostat.0, dostat.0);

    assert!(MergeHelper::match_while(dostat));

    assert_eq!(dostat.get_looptype(), LoopType::While);
    let condition = dostat.get_condition_exprent().unwrap();
    let not = condition.as_function();
    assert_eq!(not.get_func_type(), FunctionType::BoolNot);
    let operand = not.get_lst_operands().get(0);
    assert_ne!(operand, cond, "the header is a copy");
    assert_eq!(operand.as_var().get_index(), 1);

    assert!(head.get_all_successor_edges().is_empty());
    assert!(ifs.get_all_successor_edges().is_empty());
    assert!(!dostat.get_label_edges().borrow().contains(&ifedge));
    assert!(!dostat.get_label_edges().borrow().contains(&cont));
    assert_eq!(after.get_all_predecessor_edges(), vec![out]);
    assert_eq!(dostat.get_all_successor_edges(), vec![out]);

    let body = dostat.get_first().unwrap();
    assert!(body.is_basic_block());
    assert_eq!(body.get_parent(), Some(dostat.0));
    let succs = body.get_all_successor_edges();
    assert_eq!(succs.len(), 1);
    assert_eq!(succs[0].get_type(), StatEdge::TYPE_CONTINUE);
    assert_eq!(succs[0].get_destination(), dostat.0);
    assert_eq!(succs[0].closure(), Some(dostat.0));
    assert!(dostat.get_label_edges().borrow().contains(&succs[0]));
}

#[test]
fn head_if_with_exiting_else_becomes_a_plain_while() {
    let _context = setup();
    let HeadIf {
        dostat,
        ifs,
        head,
        after,
        out,
        cond,
    } = head_if_loop();
    let ifedge = labeled(StatEdge::TYPE_CONTINUE, head, dostat.0, dostat.0);
    ifs.set_if_edge(Some(ifedge));
    let elseedge = labeled(StatEdge::TYPE_BREAK, ifs.0, after, dostat.0);

    assert!(MergeHelper::match_while(dostat));

    assert_eq!(dostat.get_looptype(), LoopType::While);
    let condition = dostat.get_condition_exprent().unwrap();
    assert_ne!(condition, cond);
    assert_eq!(condition.as_var().get_index(), 1, "not negated");

    assert!(ifs.get_all_successor_edges().is_empty());
    assert!(after.get_all_predecessor_edges() == vec![out]);
    assert!(!dostat.get_label_edges().borrow().contains(&elseedge));

    let body = dostat.get_first().unwrap();
    assert!(body.is_basic_block());
    assert_ne!(body, head);
    assert_eq!(ifedge.get_source(), body);
    assert_eq!(body.get_all_successor_edges(), vec![ifedge]);
    assert!(head.get_all_successor_edges().is_empty());
    assert_eq!(
        dostat.get_predecessor_edges(StatEdge::TYPE_CONTINUE),
        vec![ifedge]
    );
}

#[test]
fn non_if_heads_and_non_empty_if_heads_do_not_match() {
    let _context = setup();
    let body = bb();
    let dostat = DoStatement::new(body);
    dostat.set_all_parent();
    assert!(!MergeHelper::match_while(dostat));
    assert_eq!(dostat.get_looptype(), LoopType::Infinite);

    let HeadIf {
        dostat,
        ifs,
        head,
        after,
        ..
    } = head_if_loop();
    head.get_exprents().unwrap().add(var(3));
    let ifedge = labeled(StatEdge::TYPE_BREAK, head, after, dostat.0);
    ifs.set_if_edge(Some(ifedge));
    assert!(!MergeHelper::match_while(dostat));
    assert_eq!(dostat.get_looptype(), LoopType::Infinite);
    assert_eq!(head.get_all_successor_edges(), vec![ifedge]);
    assert_eq!(
        dostat
            .get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward)
            .len(),
        1
    );
}

#[test]
fn not_and_cast_wrappers_are_peeled() {
    let _context = setup();
    let v = var(4);
    let not = FunctionExprent::new_single(FunctionType::BoolNot, v, None).0;
    let not_not = FunctionExprent::new_single(FunctionType::BoolNot, not, None).0;
    assert_eq!(MergeHelper::drill_nots(not_not), v);
    assert_eq!(MergeHelper::drill_nots(v), v);

    let w = var(5);
    let eq = FunctionExprent::new(FunctionType::Eq, vec![w, v], None).0;
    let not_eq = FunctionExprent::new_single(FunctionType::BoolNot, eq, None).0;
    assert_eq!(MergeHelper::drill_nots(not_eq), w);
    let lt = FunctionExprent::new(FunctionType::Lt, vec![w, v], None).0;
    assert_eq!(MergeHelper::drill_nots(lt), lt);

    let cast = FunctionExprent::new_single(FunctionType::Cast, v, None).0;
    let cast_cast = FunctionExprent::new_single(FunctionType::Cast, cast, None).0;
    assert_eq!(MergeHelper::get_uncast(cast_cast), v);
    assert_eq!(MergeHelper::as_invocation_exprent(cast_cast), None);
    assert!(!MergeHelper::is_iterator_call(cast_cast));
    assert!(!MergeHelper::is_has_next_call(v));
    assert!(!MergeHelper::is_next_call(v));
    assert!(!MergeHelper::is_next_unboxing(v));
}
