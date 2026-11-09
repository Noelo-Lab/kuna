//! IfHelper / LabelHelper on small hand-built statement graphs, replayed
//! against `tests/data/b13_oracle/B13Oracle.java` (Vineflower 1.12.0).
//!
//! The graphs avoid every path that reaches `MergeHelper.isDirectPath`,
//! `SequenceHelper.condenseSequences` or `AssertProcessor`.

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exit_exprent::{ExitExprent, Type as ExitType};
use kuna_jvm::exprent::Exprent;
use kuna_jvm::if_exprent::{IfExprent, Type as IfType};
use kuna_jvm::if_helper::IfHelper;
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::label_helper::LabelHelper;
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, JHashSet};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_type::VarType;

use crate::common::ContextGuard;
use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};

const REG: i32 = StatEdge::TYPE_REGULAR;
const BREAK: i32 = StatEdge::TYPE_BREAK;
const FINALLYEXIT: i32 = StatEdge::TYPE_FINALLYEXIT;

fn setup() -> (ContextGuard, StructMethod) {
    let context = oracle_context();
    let mt = fixture_method();
    start_method(mt);
    (context, mt)
}

fn reset(mt: StructMethod) {
    reset_class_scope();
    start_method(mt);
}

fn bb() -> Statement {
    BasicBlockStatement::create().0
}

fn edge(type_: i32, source: Statement, destination: Statement) -> StatEdge {
    let e = StatEdge::new(type_, source, destination);
    source.add_successor(e);
    e
}

fn edge_closure(
    type_: i32,
    source: Statement,
    destination: Statement,
    closure: Statement,
) -> StatEdge {
    let e = StatEdge::new_closure(type_, source, destination, Some(closure));
    source.add_successor(e);
    e
}

fn cond() -> IfExprent {
    let mut stack = ListStack::new();
    stack.push(ConstExprent::new_int(1, true, None).0);
    IfExprent::new(IfType::Value, &mut stack, None)
}

fn ret() -> Exprent {
    ExitExprent::new(
        ExitType::Return,
        None,
        Some(VarType::VARTYPE_VOID()),
        None,
        None,
    )
    .0
}

fn if_then(head: Statement, body: Option<Statement>) -> IfStatement {
    let ifs = IfStatement::new(head, 0, None);
    if let Some(body) = body {
        ifs.set_ifstat(Some(body));
        ifs.get_stats().borrow_mut().add_with_key(body, body.id());
    }
    ifs.set_all_parent();
    ifs.get_headexprent_list().add(cond().0);
    ifs
}

fn if_else(head: Statement, if_body: Statement, else_body: Statement) -> IfStatement {
    edge(REG, head, else_body);
    edge(REG, head, if_body);
    let join = bb();
    let a = edge(REG, if_body, join);
    let b = edge(REG, else_body, join);
    let ifs = IfStatement::new(head, 2, None);
    if_body.remove_successor(a);
    else_body.remove_successor(b);
    ifs.set_all_parent();
    ifs.get_headexprent_list().add(cond().0);
    ifs
}

fn expr(e: Exprent) -> String {
    match e.try_as_function() {
        Some(f) => {
            let operands: Vec<String> = f.get_lst_operands().iter().map(expr).collect();
            format!(
                "{}#{}({})",
                f.get_func_type().name(),
                f.get_id(),
                operands.join(", ")
            )
        }
        None => format!("#{}", e.get_id()),
    }
}

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn opt<T: std::fmt::Display>(item: Option<T>) -> String {
    item.map_or_else(|| "null".to_string(), |x| x.to_string())
}

fn flags(edges: Vec<StatEdge>) -> String {
    list(
        edges
            .into_iter()
            .map(|e| format!("{} explicit={} labeled={}", e, e.explicit(), e.labeled())),
    )
}

fn labels(stat: Statement) -> String {
    let mut out: Vec<String> = stat
        .get_label_edges()
        .borrow()
        .to_vec()
        .iter()
        .map(|e| e.to_string())
        .collect();
    out.sort();
    list(out)
}

fn stats_of(stat: Statement) -> Vec<Statement> {
    stat.get_stats().borrow().to_vec()
}

#[test]
fn matches_the_vineflower_oracle() {
    let mut out = String::new();
    let mut p = |key: &str, value: String| out.push_str(&format!("{key} = {value}\n"));

    let (_context, mt) = setup();
    {
        let (h, i, e) = (bb(), bb(), bb());
        let exit = DummyExitStatement::new();
        edge(BREAK, i, exit.0);
        i.get_exprents().unwrap().add(ret());
        e.get_exprents().unwrap().add(ret());
        e.get_exprents().unwrap().add(ret());
        let ifs = if_else(h, i, e);
        let root = RootStatement::new(ifs.0, exit, mt);
        p("pretty.iftype", ifs.iftype().to_string());
        p("pretty.changed", IfHelper::prettify_ifs(root.0).to_string());
        p("pretty.iftype2", ifs.iftype().to_string());
        p("pretty.root", list(stats_of(root.0)));
        let seq = root.get_first().unwrap();
        p("pretty.seq", list(stats_of(seq)));
        p("pretty.seq.parent", opt(seq.get_parent()));
        p("pretty.ifs", list(stats_of(ifs.0)));
        p("pretty.ifs.parent", opt(ifs.get_parent()));
        p("pretty.e.parent", opt(e.get_parent()));
        p("pretty.h.succs", list(h.get_all_successor_edges()));
        p("pretty.ifs.succs", list(ifs.get_all_successor_edges()));
        p("pretty.e.preds", list(e.get_all_predecessor_edges()));
        p("pretty.elsestat", opt(ifs.get_elsestat()));
        p("pretty.elseedge", opt(ifs.get_else_edge()));
    }

    reset(mt);
    {
        let c = bb();
        c.get_exprents().unwrap().add(ret());
        let (h0, h1, t, e) = (bb(), bb(), bb(), bb());
        let inner = if_else(h1, t, e);
        edge(REG, h0, c);
        edge(REG, h0, inner.0);
        let outer = if_then(h0, Some(inner.0));
        let seq = SequenceStatement::new(vec![outer.0, c]);
        seq.set_all_parent();
        edge_closure(BREAK, t, c, seq.0);
        edge_closure(BREAK, e, c, seq.0);
        let exit = DummyExitStatement::new();
        let root = RootStatement::new(seq.0, exit, mt);
        edge_closure(BREAK, c, exit.0, root.0);
        LabelHelper::identify_labels(root);
        p("labels.h0", flags(h0.get_all_successor_edges()));
        p("labels.outer", flags(outer.get_all_successor_edges()));
        p("labels.h1", flags(h1.get_all_successor_edges()));
        p("labels.t", flags(t.get_all_successor_edges()));
        p("labels.e", flags(e.get_all_successor_edges()));
        p("labels.inner", flags(inner.get_all_successor_edges()));
        p("labels.c", flags(c.get_all_successor_edges()));
        p("labels.c.preds", list(c.get_all_predecessor_edges()));
        p("labels.seq.labels", labels(seq.0));
        p("labels.root.labels", labels(root.0));
        p("labels.outer.labels", labels(outer.0));
    }

    reset(mt);
    {
        let (h1, h2, a, b) = (bb(), bb(), bb(), bb());
        edge(FINALLYEXIT, h2, b);
        edge(BREAK, h2, a);
        let child = if_then(h2, None);
        edge(FINALLYEXIT, h1, a);
        edge(REG, h1, child.0);
        let parent = if_then(h1, Some(child.0));
        let exit = DummyExitStatement::new();
        let root = RootStatement::new(parent.0, exit, mt);
        p(
            "merge.before",
            format!(
                "{} {}",
                expr(parent.get_headexprent().0),
                expr(child.get_headexprent().0)
            ),
        );
        p(
            "merge.changed",
            IfHelper::merge_ifs(parent.0, &mut JHashSet::new()).to_string(),
        );
        p("merge.stats", list(stats_of(parent.0)));
        p("merge.ifstat", opt(parent.get_ifstat()));
        p("merge.ifedge", opt(parent.get_if_edge()));
        p("merge.iftype", parent.iftype().to_string());
        p("merge.h1.succs", list(h1.get_all_successor_edges()));
        p("merge.parent.succs", list(parent.get_all_successor_edges()));
        p("merge.h2.succs", list(h2.get_all_successor_edges()));
        p("merge.child.succs", list(child.get_all_successor_edges()));
        p("merge.a.preds", list(a.get_all_predecessor_edges()));
        p("merge.b.preds", list(b.get_all_predecessor_edges()));
        p("merge.cond", expr(parent.get_headexprent().get_condition()));
        p("merge.root", list(stats_of(root.0)));
    }

    reset(mt);
    {
        let (h1, h2, a, b) = (bb(), bb(), bb(), bb());
        h1.get_exprents()
            .unwrap()
            .add(ConstExprent::new_int(7, true, None).0);
        edge(FINALLYEXIT, h2, b);
        edge(FINALLYEXIT, h2, a);
        let second = if_then(h2, None);
        edge(REG, h1, second.0);
        edge(FINALLYEXIT, h1, a);
        let first = if_then(h1, None);
        let seq = SequenceStatement::new(vec![first.0, second.0]);
        seq.set_all_parent();
        let exit = DummyExitStatement::new();
        let root = RootStatement::new(seq.0, exit, mt);
        p(
            "or.changed",
            IfHelper::merge_ifs(seq.0, &mut JHashSet::new()).to_string(),
        );
        p("or.seq", list(stats_of(seq.0)));
        p("or.seq.first", opt(seq.get_first()));
        p("or.second", list(stats_of(second.0)));
        p("or.second.first", opt(second.get_first()));
        p("or.ifedge", opt(second.get_if_edge()));
        p("or.elseedge", opt(second.get_else_edge()));
        p("or.h1.parent", opt(h1.get_parent()));
        p("or.h1.succs", list(h1.get_all_successor_edges()));
        p("or.h2.succs", list(h2.get_all_successor_edges()));
        p("or.first.succs", list(first.get_all_successor_edges()));
        p("or.second.succs", list(second.get_all_successor_edges()));
        p("or.a.preds", list(a.get_all_predecessor_edges()));
        p("or.b.preds", list(b.get_all_predecessor_edges()));
        p("or.cond", expr(second.get_headexprent().get_condition()));
        p("or.root", list(stats_of(root.0)));
    }

    reset(mt);
    {
        let (h0, h1, h2, a, b) = (bb(), bb(), bb(), bb(), bb());
        edge(FINALLYEXIT, h1, b);
        edge(FINALLYEXIT, h1, a);
        let first = if_then(h1, None);
        edge(FINALLYEXIT, h2, b);
        edge(FINALLYEXIT, h2, a);
        let second = if_then(h2, None);
        let main = if_else(h0, first.0, second.0);
        let exit = DummyExitStatement::new();
        let root = RootStatement::new(main.0, exit, mt);
        p("ternary.iftype", main.iftype().to_string());
        p(
            "ternary.changed",
            IfHelper::merge_ifs(main.0, &mut JHashSet::new()).to_string(),
        );
        p("ternary.iftype2", main.iftype().to_string());
        p("ternary.stats", list(stats_of(main.0)));
        p("ternary.ifstat", opt(main.get_ifstat()));
        p("ternary.elsestat", opt(main.get_elsestat()));
        p("ternary.ifedge", opt(main.get_if_edge()));
        p("ternary.elseedge", opt(main.get_else_edge()));
        p("ternary.h0.succs", list(h0.get_all_successor_edges()));
        p("ternary.main.succs", list(main.get_all_successor_edges()));
        p("ternary.h1.succs", list(h1.get_all_successor_edges()));
        p("ternary.h2.succs", list(h2.get_all_successor_edges()));
        p("ternary.first.succs", list(first.get_all_successor_edges()));
        p(
            "ternary.second.succs",
            list(second.get_all_successor_edges()),
        );
        p("ternary.a.preds", list(a.get_all_predecessor_edges()));
        p("ternary.b.preds", list(b.get_all_predecessor_edges()));
        p("ternary.cond", expr(main.get_headexprent().get_condition()));
        p("ternary.root", list(stats_of(root.0)));
    }

    let expected = include_str!("data/b13_oracle/expected.txt");
    for (n, (got, want)) in out.lines().zip(expected.lines()).enumerate() {
        assert_eq!(got, want, "line {}", n + 1);
    }
    assert_eq!(out.lines().count(), expected.lines().count());
}
