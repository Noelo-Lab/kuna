//! Statement-graph machinery: edge maps, edge surgery, label edges, node
//! collapsing, statement replacement, orderings and the dominator helpers.
//!
//! Every statement takes its id from the decompiler context's counter
//! container, so `setup` must leave a usable one on this thread.

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::do_statement::DoStatement;
use kuna_jvm::dominator_engine::DominatorEngine;
use kuna_jvm::dominator_tree_exception_filter::DominatorTreeExceptionFilter;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch, JList};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{EdgeDirection, Statement};
use kuna_jvm::strong_connectivity_helper::StrongConnectivityHelper;
use kuna_jvm::switch_statement::SwitchStatement;

fn setup() {
    reset_class_scope();
}

fn bb() -> Statement {
    BasicBlockStatement::create().0
}

fn edge(type_: i32, source: Statement, destination: Statement) -> StatEdge {
    let e = StatEdge::new(type_, source, destination);
    source.add_successor(e);
    e
}

fn stats_of(stat: Statement) -> Vec<Statement> {
    stat.get_stats().borrow().to_vec()
}

#[test]
fn edge_maps_track_types_and_directions() {
    setup();
    let (a, b, c) = (bb(), bb(), bb());
    let reg = edge(StatEdge::TYPE_REGULAR, a, b);
    let exc = StatEdge::new_exceptions(a, c, Some(vec!["java/lang/Exception".into()]));
    a.add_successor(exc);

    assert_eq!(a.get_all_successor_edges(), vec![reg, exc]);
    assert_eq!(a.get_all_direct_successor_edges(), vec![reg]);
    assert_eq!(
        a.get_successor_edges(StatEdge::TYPE_REGULAR | StatEdge::TYPE_EXCEPTION),
        vec![reg, exc]
    );
    assert_eq!(a.get_successor_edges(StatEdge::TYPE_EXCEPTION), vec![exc]);
    assert_eq!(b.get_all_predecessor_edges(), vec![reg]);
    assert_eq!(c.get_predecessor_edges(StatEdge::TYPE_EXCEPTION), vec![exc]);
    assert_eq!(
        a.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward),
        vec![b]
    );
    assert_eq!(
        c.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Backward),
        vec![a]
    );
    assert!(a
        .get_neighbours_set(Statement::STATEDGE_ALL, EdgeDirection::Forward)
        .contains(&c));
    assert!(a.has_any_successor() && a.has_any_direct_successor());
    assert!(!a.has_successor(StatEdge::TYPE_BREAK));
    assert!(a.has_successor(StatEdge::TYPE_BREAK | StatEdge::TYPE_EXCEPTION));
    assert_eq!(a.get_first_successor(), reg);
    assert_eq!(a.get_first_direct_successor(), reg);

    let view = a.get_successor_edge_view(StatEdge::TYPE_REGULAR);
    a.remove_successor(reg);
    assert!(view.is_empty(), "the view aliases the live list");
    assert_eq!(a.get_all_successor_edges(), vec![exc]);
    assert!(b.get_all_predecessor_edges().is_empty());
    assert_eq!(a.get_first_successor(), exc);
    let caught = try_catch(|| a.get_first_direct_successor()).unwrap_err();
    assert!(caught.is("IllegalStateException"), "{}", caught.message);

    assert_eq!(
        exc.get_exceptions(),
        Some(vec!["java/lang/Exception".to_string()])
    );
    assert_eq!(
        exc.to_string(),
        format!("2: {a} -> {c} Exceptions: [java/lang/Exception]")
    );
    assert_eq!(a.to_string(), format!("{{Block}}:{}", a.id()));
}

#[test]
fn edge_type_and_endpoint_changes_update_both_ends() {
    setup();
    let (a, b, c, d) = (bb(), bb(), bb(), bb());
    let e = edge(StatEdge::TYPE_REGULAR, a, b);

    e.change_type(StatEdge::TYPE_BREAK);
    assert_eq!(e.get_type(), StatEdge::TYPE_BREAK);
    assert!(a.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty());
    assert_eq!(a.get_successor_edges(StatEdge::TYPE_BREAK), vec![e]);
    assert_eq!(a.get_all_direct_successor_edges(), vec![e]);
    assert_eq!(b.get_predecessor_edges(StatEdge::TYPE_BREAK), vec![e]);
    assert!(b.get_predecessor_edges(StatEdge::TYPE_REGULAR).is_empty());

    let caught = try_catch(|| e.change_type(StatEdge::TYPE_EXCEPTION)).unwrap_err();
    assert_eq!(caught.message, "RuntimeException: Invalid edge type!");

    e.change_destination(c);
    assert_eq!(e.get_destination(), c);
    assert!(b.get_all_predecessor_edges().is_empty());
    assert_eq!(c.get_predecessor_edges(StatEdge::TYPE_BREAK), vec![e]);
    assert_eq!(a.get_all_successor_edges(), vec![e]);

    e.change_source(d);
    assert_eq!(e.get_source(), d);
    assert!(a.get_all_successor_edges().is_empty());
    assert_eq!(d.get_successor_edges(StatEdge::TYPE_BREAK), vec![e]);

    a.change_edge_node(EdgeDirection::Backward, e, a);
    assert_eq!(e.get_source(), a);

    d.remove_all_successors(c);
    assert!(c.get_all_predecessor_edges().is_empty());
}

#[test]
fn label_edges_follow_closures() {
    setup();
    let (a, b, x) = (bb(), bb(), bb());
    let seq = SequenceStatement::new(vec![a, b]);
    let other = SequenceStatement::new_empty();

    let e = StatEdge::new_closure(StatEdge::TYPE_BREAK, a, x, Some(seq.0));
    a.add_successor(e);
    assert!(seq.get_label_edges().borrow().contains(&e));
    assert!(seq.is_labeled());
    assert_eq!(e.to_string(), format!("4: {a} -> {x} ({})", seq.0));

    other.add_labeled_edge(e);
    assert_eq!(e.closure(), Some(other.0));
    assert!(!seq.get_label_edges().borrow().contains(&e));
    assert!(other.get_label_edges().borrow().contains(&e));

    e.change_closure(Some(seq.0));
    assert!(other.get_label_edges().borrow().is_empty());
    assert!(seq.get_label_edges().borrow().contains(&e));

    e.with_mut(|d| d.explicit = false);
    assert!(!seq.is_labeled());
    e.with_mut(|d| d.explicit = true);

    e.remove_closure();
    assert_eq!(e.closure(), None);
    assert!(!e.labeled());
    assert!(seq.get_label_edges().borrow().is_empty());

    e.change_closure(Some(seq.0));
    e.remove();
    assert!(seq.get_label_edges().borrow().is_empty());
    assert!(a.get_all_successor_edges().is_empty());
    assert!(x.get_all_predecessor_edges().is_empty());
}

#[test]
fn orderings_and_sequence_collapse() {
    setup();
    let (a, b, c) = (bb(), bb(), bb());
    let exit = bb();
    let general = GeneralStatement::new(a, vec![a, b, c], None);
    let ab = edge(StatEdge::TYPE_REGULAR, a, b);
    let bc = edge(StatEdge::TYPE_REGULAR, b, c);
    let cx = StatEdge::new_closure(StatEdge::TYPE_BREAK, c, exit, Some(general.0));
    c.add_successor(cx);
    general.set_all_parent();

    assert_eq!(stats_of(general.0), vec![a, b, c]);
    assert_eq!(general.get_reverse_post_order_list(), vec![a, b, c]);
    assert_eq!(general.get_post_reverse_post_order_list(), vec![c, b, a]);
    assert!(general.contains_statement_strict(b));
    assert!(general.contains_statement(general.0));
    assert!(!general.contains_statement_strict(exit));

    let seq = SequenceStatement::is_head2_block(a).expect("a and b form a sequence");
    assert_eq!(stats_of(seq), vec![a, b]);
    assert_eq!(seq.get_post(), Some(c));
    assert_eq!(seq.get_first(), Some(a));

    general.collapse_nodes_to_statement(seq);

    assert_eq!(stats_of(general.0), vec![c, seq]);
    assert_eq!(general.get_first(), Some(seq));
    assert_eq!(seq.get_parent(), Some(general.0));
    assert_eq!(a.get_parent(), Some(seq));
    assert_eq!(b.get_parent(), Some(seq));

    assert_eq!(bc.get_type(), StatEdge::TYPE_BREAK);
    assert_eq!(bc.closure(), Some(seq));
    assert!(seq.get_label_edges().borrow().contains(&bc));
    assert_eq!(ab.get_type(), StatEdge::TYPE_REGULAR);

    let out = seq.get_successor_edges(StatEdge::TYPE_REGULAR);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].get_destination(), c);
    assert_eq!(c.get_all_predecessor_edges(), vec![bc, out[0]]);
    assert!(seq.get_continue_set().borrow().is_empty());
    assert!(!seq.has_basic_succ_edge());
    assert_eq!(seq.get_basichead().0, a);
}

#[test]
fn replace_statement_moves_edges_labels_and_position() {
    setup();
    let (a, b, y, x) = (bb(), bb(), bb(), bb());
    let seq = SequenceStatement::new(vec![a, b]);
    seq.set_all_parent();
    let ab = edge(StatEdge::TYPE_REGULAR, a, b);
    let labeled = StatEdge::new_closure(StatEdge::TYPE_BREAK, b, y, Some(a));
    b.add_successor(labeled);
    assert!(a.get_label_edges().borrow().contains(&labeled));

    seq.replace_statement(a, x);

    assert_eq!(stats_of(seq.0), vec![x, b]);
    assert_eq!(seq.get_first(), Some(x));
    assert_eq!(x.get_parent(), Some(seq.0));
    assert_eq!(ab.get_source(), x);
    assert_eq!(x.get_all_successor_edges(), vec![ab]);
    assert!(a.get_all_successor_edges().is_empty());
    assert_eq!(b.get_all_predecessor_edges(), vec![ab]);
    assert_eq!(labeled.closure(), Some(x));
    assert!(x.get_label_edges().borrow().contains(&labeled));
    assert!(a.get_label_edges().borrow().is_empty());

    let stranger = bb();
    let caught = try_catch(|| seq.replace_statement(stranger, a)).unwrap_err();
    assert!(caught.is("IllegalStateException"));
    assert_eq!(
        caught.message,
        format!(
            "IllegalStateException: [{}] Cannot replace {stranger} with {a} because it wasn't found in [{x}, {b}]",
            seq.0
        )
    );

    let empty = b.replace_with_empty();
    assert_eq!(stats_of(seq.0), vec![x, empty.0]);
    assert_eq!(ab.get_destination(), empty.0);
    assert!(empty.get_exprents().is_some_and(|e| e.is_empty()));
}

#[test]
fn if_statement_without_else_drops_the_fallthrough_edge() {
    setup();
    let (h, t, out) = (bb(), bb(), bb());
    let general = GeneralStatement::new(h, vec![h, t], None);
    let ht = edge(StatEdge::TYPE_REGULAR, h, t);
    let hout = StatEdge::new_closure(StatEdge::TYPE_BREAK, h, out, Some(general.0));
    h.add_successor(hout);

    let ifs = IfStatement::new(h, 1, None);
    assert_eq!(ifs.iftype(), IfStatement::IFTYPE_IF);
    assert_eq!(ifs.get_ifstat(), None);
    assert_eq!(ifs.get_if_edge(), Some(hout));
    assert_eq!(ifs.get_else_edge(), None);
    assert!(!ifs.is_negated());
    assert_eq!(ifs.get_post(), Some(t));
    assert_eq!(stats_of(ifs.0), vec![h]);
    assert_eq!(h.get_all_successor_edges(), vec![hout]);
    assert!(t.get_all_predecessor_edges().is_empty());
    assert!(ifs.has_basic_succ_edge());
    assert!(ifs.get_headexprent_list().is_empty());
    assert!(ifs.get_stat_exprents().is_empty());
    let _ = ht;

    general.collapse_nodes_to_statement(ifs.0);
    assert_eq!(stats_of(general.0), vec![t, ifs.0]);
    let out_edges = ifs.get_successor_edges(StatEdge::TYPE_REGULAR);
    assert_eq!(out_edges.len(), 1);
    assert_eq!(out_edges[0].get_destination(), t);

    let copy = ifs.get_simple_copy();
    let copy = copy.as_if();
    assert_eq!(copy.iftype(), IfStatement::IFTYPE_IF);
    assert!(copy.get_stats().borrow().is_empty());
}

#[test]
fn if_else_statement_and_empty_else_fix() {
    setup();
    let (h, e, i, p) = (bb(), bb(), bb(), bb());
    let he = edge(StatEdge::TYPE_REGULAR, h, e);
    let hi = edge(StatEdge::TYPE_REGULAR, h, i);
    edge(StatEdge::TYPE_REGULAR, i, p);
    let ep = edge(StatEdge::TYPE_REGULAR, e, p);

    let ifs = IfStatement::new(h, 2, Some(p));
    assert_eq!(ifs.iftype(), IfStatement::IFTYPE_IFELSE);
    assert_eq!(ifs.get_ifstat(), Some(i));
    assert_eq!(ifs.get_elsestat(), Some(e));
    assert_eq!(ifs.get_if_edge(), Some(hi));
    assert_eq!(ifs.get_else_edge(), Some(he));
    assert_eq!(ifs.get_post(), Some(p));
    assert_eq!(stats_of(ifs.0), vec![h, i, e]);
    assert!(!ifs.has_basic_succ_edge());

    ifs.fix_if_invariant_empty_else_branch();
    assert_eq!(ifs.iftype(), IfStatement::IFTYPE_IF);
    assert_eq!(ifs.get_elsestat(), None);
    assert_eq!(ifs.get_else_edge(), None);
    assert_eq!(stats_of(ifs.0), vec![h, i]);
    assert_eq!(h.get_all_successor_edges(), vec![hi]);
    assert_eq!(ep.get_source(), ifs.0);
    assert_eq!(ifs.get_all_successor_edges(), vec![ep]);
    assert!(e.get_all_successor_edges().is_empty());
}

#[test]
fn do_statement_turns_self_loop_into_continue() {
    setup();
    let a = bb();
    let general = GeneralStatement::new(a, vec![a], None);
    let back = edge(StatEdge::TYPE_REGULAR, a, a);

    let dostat = DoStatement::is_head(a).expect("self loop is a loop head");
    general.collapse_nodes_to_statement(dostat);

    assert_eq!(back.get_type(), StatEdge::TYPE_CONTINUE);
    assert_eq!(back.get_source(), a);
    assert_eq!(back.get_destination(), dostat);
    assert_eq!(back.closure(), Some(dostat));
    assert_eq!(dostat.get_all_predecessor_edges(), vec![back]);
    assert!(dostat.is_labeled());
    assert_eq!(
        a.get_neighbours(StatEdge::TYPE_CONTINUE, EdgeDirection::Forward),
        vec![dostat]
    );
    assert!(a.get_continue_set().borrow().contains(&a));
    assert!(dostat.get_continue_set().borrow().is_empty());
    assert_eq!(general.get_first(), Some(dostat));
    assert!(!dostat.has_basic_succ_edge());
    assert!(dostat.as_do().get_stat_exprents().is_empty());
}

#[test]
fn clear_temp_information_drops_exception_edges() {
    setup();
    let (a, b, h) = (bb(), bb(), bb());
    let reg = edge(StatEdge::TYPE_REGULAR, a, b);
    let exc = StatEdge::new_exceptions(a, h, None);
    a.add_successor(exc);

    a.clear_temp_information();
    assert_eq!(a.get_all_successor_edges(), vec![reg]);
    assert!(a.get_successor_edges(StatEdge::TYPE_EXCEPTION).is_empty());
    assert!(!a.is_copied());
    let caught = try_catch(|| a.get_continue_set()).unwrap_err();
    assert!(caught.exception.is_none());
}

#[test]
fn strong_connectivity_follows_the_java_visit_order() {
    setup();
    let (a, b, c) = (bb(), bb(), bb());
    let general = GeneralStatement::new(a, vec![a, b, c], None);
    edge(StatEdge::TYPE_REGULAR, a, b);
    edge(StatEdge::TYPE_REGULAR, b, a);
    edge(StatEdge::TYPE_REGULAR, b, c);

    let helper = StrongConnectivityHelper::new(general.0);
    assert_eq!(helper.get_components(), &[vec![c], vec![a, b]]);
    assert!(StrongConnectivityHelper::is_exit_component(&[c]));
    assert!(!StrongConnectivityHelper::is_exit_component(&[a, b]));
    assert_eq!(
        StrongConnectivityHelper::get_exit_reps(helper.get_components()),
        vec![c]
    );
}

#[test]
fn dominators_of_a_diamond() {
    setup();
    let (a, b, c, d) = (bb(), bb(), bb(), bb());
    let general = GeneralStatement::new(a, vec![a, b, c, d], None);
    edge(StatEdge::TYPE_REGULAR, a, b);
    edge(StatEdge::TYPE_REGULAR, a, c);
    edge(StatEdge::TYPE_REGULAR, b, d);
    edge(StatEdge::TYPE_REGULAR, c, d);

    assert_eq!(general.get_reverse_post_order_list(), vec![a, c, b, d]);

    let mut engine = DominatorEngine::new(general.0);
    engine.initialize();
    assert!(engine.is_dominator(d.id(), a.id()));
    assert!(!engine.is_dominator(d.id(), b.id()));
    assert!(engine.is_dominator(b.id(), a.id()));
    assert!(engine.is_dominator(a.id(), a.id()));
    let doms = engine.all_doms_for(a.id());
    assert_eq!(doms.len(), 4);
    assert_eq!(engine.all_doms_for(b.id()).to_vec(), vec![b.id()]);
    assert_eq!(
        engine.get_ordered_idoms().get_lst_keys().to_vec(),
        vec![a.id(), c.id(), b.id(), d.id()]
    );

    let mut filter = DominatorTreeExceptionFilter::new(general.0);
    filter.initialize();
    assert!(filter.accept_statement_pair(a.id(), d.id()));
    assert!(filter.get_dom_engine().is_dominator(c.id(), a.id()));
}

#[test]
fn switch_cases_are_added_and_removed() {
    setup();
    let (h, s1, s2) = (bb(), bb(), bb());
    let sw = SwitchStatement::new_empty();
    sw.set_first(h);
    sw.get_stats().borrow_mut().add_with_key(h, h.id());

    sw.set_default_case(s1);
    let default_edge = sw.get_default_edge().expect("default edge");
    assert_eq!(default_edge.get_destination(), s1);
    assert_eq!(sw.get_default_case(), Some(s1));
    assert!(h.get_all_successor_edges().is_empty());
    assert_eq!(stats_of(sw.0), vec![h, s1]);
    assert_eq!(s1.get_parent(), Some(sw.0));

    sw.add_case(JList::from(vec![None, None]), s2);
    assert_eq!(sw.get_case_statements().to_vec(), vec![s2, s1]);
    assert_eq!(stats_of(sw.0), vec![h, s2, s1]);
    assert_eq!(h.get_all_successor_edges().len(), 2);
    assert_eq!(s2.get_all_predecessor_edges().len(), 2);
    assert_eq!(sw.get_case_values().len(), 2);
    assert_eq!(sw.get_case_edges().get(0).len(), 2);
    assert!(sw.get_case_values().get(1).get(0).is_none());

    sw.scope_case_statement(s2);
    let caught = try_catch(|| sw.scope_case_statement(h)).unwrap_err();
    assert!(caught.is("IllegalStateException"));

    sw.remove_case(s2);
    assert_eq!(sw.get_case_statements().to_vec(), vec![s1]);
    assert!(h.get_all_successor_edges().is_empty());
    assert!(s2.get_all_predecessor_edges().is_empty());
    assert_eq!(sw.get_case_edges().len(), 1);
    assert_eq!(
        stats_of(sw.0),
        vec![h, s2, s1],
        "a case added without a key is not removed by key, as in Java"
    );
}

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn opt<T: std::fmt::Display>(item: Option<T>) -> String {
    item.map_or_else(|| "null".to_string(), |x| x.to_string())
}

/// Replays `tests/data/k2_graph_oracle/GraphOracle.java` (run against the
/// Vineflower 1.12.0 jar) and compares the dump line for line.
#[test]
fn matches_the_vineflower_oracle() {
    use kuna_jvm::catch_statement::CatchStatement;

    const REG: i32 = StatEdge::TYPE_REGULAR;
    const EXC: i32 = StatEdge::TYPE_EXCEPTION;
    const BREAK: i32 = StatEdge::TYPE_BREAK;
    let mut out = String::new();
    let mut p = |key: &str, value: String| out.push_str(&format!("{key} = {value}\n"));

    setup();
    {
        let (a, b, c) = (bb(), bb(), bb());
        let reg = edge(REG, a, b);
        let exc = StatEdge::new_exceptions(a, c, Some(vec!["java/lang/Exception".into()]));
        a.add_successor(exc);
        p("edges.all", list(a.get_all_successor_edges()));
        p("edges.direct", list(a.get_all_direct_successor_edges()));
        p("edges.regexc", list(a.get_successor_edges(REG | EXC)));
        p(
            "edges.neigh",
            list(c.get_neighbours(EXC, EdgeDirection::Backward)),
        );
        a.remove_successor(reg);
        p("edges.after", list(a.get_all_successor_edges()));
        p("edges.first", a.get_first_successor().to_string());
    }

    setup();
    {
        let (a, b, c, exit) = (bb(), bb(), bb(), bb());
        let general = GeneralStatement::new(a, vec![a, b, c], None);
        edge(REG, a, b);
        edge(REG, b, c);
        c.add_successor(StatEdge::new_closure(BREAK, c, exit, Some(general.0)));
        general.set_all_parent();
        p("seq.rpo", list(general.get_reverse_post_order_list()));
        p("seq.prpo", list(general.get_post_reverse_post_order_list()));
        let seq = SequenceStatement::is_head2_block(a).unwrap();
        p("seq.stats", list(stats_of(seq)));
        p("seq.post", opt(seq.get_post()));
        general.collapse_nodes_to_statement(seq);
        p("seq.general.stats", list(stats_of(general.0)));
        p("seq.general.first", opt(general.get_first()));
        p("seq.c.preds", list(c.get_all_predecessor_edges()));
        p("seq.labels", list(seq.get_label_edges().borrow().to_vec()));
        p("seq.succs", list(seq.get_all_successor_edges()));
    }

    setup();
    {
        let (a, b, y, x) = (bb(), bb(), bb(), bb());
        let seq = SequenceStatement::new(vec![a, b]);
        seq.set_all_parent();
        edge(REG, a, b);
        let labeled = StatEdge::new_closure(BREAK, b, y, Some(a));
        b.add_successor(labeled);
        seq.replace_statement(a, x);
        p("replace.stats", list(stats_of(seq.0)));
        p("replace.first", opt(seq.get_first()));
        p("replace.x.succs", list(x.get_all_successor_edges()));
        p("replace.labeled", labeled.to_string());
        let stranger = bb();
        let caught = try_catch(|| seq.replace_statement(stranger, a)).unwrap_err();
        p("replace.err", format!("java.lang.{}", caught.message));
        b.replace_with_empty();
        p("replace.empty.stats", list(stats_of(seq.0)));
    }

    setup();
    {
        let (h, t, o) = (bb(), bb(), bb());
        let general = GeneralStatement::new(h, vec![h, t], None);
        edge(REG, h, t);
        h.add_successor(StatEdge::new_closure(BREAK, h, o, Some(general.0)));
        let ifs = IfStatement::new(h, 1, None);
        p("if1.iftype", ifs.iftype().to_string());
        p("if1.ifedge", opt(ifs.get_if_edge()));
        p("if1.post", opt(ifs.get_post()));
        p("if1.h.succs", list(h.get_all_successor_edges()));
        general.collapse_nodes_to_statement(ifs.0);
        p("if1.general.stats", list(stats_of(general.0)));
        p("if1.succs", list(ifs.get_all_successor_edges()));
    }

    setup();
    {
        let (h, e, i, q) = (bb(), bb(), bb(), bb());
        edge(REG, h, e);
        edge(REG, h, i);
        edge(REG, i, q);
        edge(REG, e, q);
        let ifs = IfStatement::new(h, 2, Some(q));
        p("if2.iftype", ifs.iftype().to_string());
        p("if2.ifstat", opt(ifs.get_ifstat()));
        p("if2.elsestat", opt(ifs.get_elsestat()));
        p("if2.ifedge", opt(ifs.get_if_edge()));
        p("if2.elseedge", opt(ifs.get_else_edge()));
        p("if2.post", opt(ifs.get_post()));
        p("if2.stats", list(stats_of(ifs.0)));
        ifs.fix_if_invariant_empty_else_branch();
        p("if2.fix.iftype", ifs.iftype().to_string());
        p("if2.fix.stats", list(stats_of(ifs.0)));
        p("if2.fix.h.succs", list(h.get_all_successor_edges()));
        p("if2.fix.succs", list(ifs.get_all_successor_edges()));
    }

    setup();
    {
        let a = bb();
        let general = GeneralStatement::new(a, vec![a], None);
        let back = edge(REG, a, a);
        let dos = DoStatement::is_head(a).unwrap();
        general.collapse_nodes_to_statement(dos);
        p("do.back", back.to_string());
        p("do.preds", list(dos.get_all_predecessor_edges()));
        p("do.a.cont", list(a.get_continue_set().borrow().to_vec()));
        p("do.cont", list(dos.get_continue_set().borrow().to_vec()));
        p("do.labeled", dos.is_labeled().to_string());
        p("do.general.first", opt(general.get_first()));
    }

    setup();
    {
        let (a, b, c) = (bb(), bb(), bb());
        let general = GeneralStatement::new(a, vec![a, b, c], None);
        edge(REG, a, b);
        edge(REG, b, a);
        edge(REG, b, c);
        let helper = StrongConnectivityHelper::new(general.0);
        p(
            "scc",
            list(helper.get_components().iter().map(|c| list(c.iter()))),
        );
        p(
            "scc.exits",
            list(StrongConnectivityHelper::get_exit_reps(
                helper.get_components(),
            )),
        );
    }

    setup();
    {
        let (a, b, c, d) = (bb(), bb(), bb(), bb());
        let general = GeneralStatement::new(a, vec![a, b, c, d], None);
        edge(REG, a, b);
        edge(REG, a, c);
        edge(REG, b, d);
        edge(REG, c, d);
        p("dom.rpo", list(general.get_reverse_post_order_list()));
        let mut engine = DominatorEngine::new(general.0);
        engine.initialize();
        let idoms = engine.get_ordered_idoms();
        let keys = idoms.get_lst_keys().to_vec();
        p(
            "dom.idoms",
            list((0..keys.len()).map(|i| format!("{}={}", keys[i], idoms.get(i).unwrap()))),
        );
        p("dom.all.a", list(engine.all_doms_for(a.id()).to_vec()));
        p("dom.all.b", list(engine.all_doms_for(b.id()).to_vec()));
        p(
            "dom.d.by.b",
            engine.is_dominator(d.id(), b.id()).to_string(),
        );
        let mut filter = DominatorTreeExceptionFilter::new(general.0);
        filter.initialize();
        p(
            "dom.accept",
            filter.accept_statement_pair(a.id(), d.id()).to_string(),
        );
    }

    setup();
    {
        let (h, s1, s2) = (bb(), bb(), bb());
        let sw = SwitchStatement::new_empty();
        sw.set_first(h);
        sw.get_stats().borrow_mut().add_with_key(h, h.id());
        sw.set_default_case(s1);
        p("switch.default", opt(sw.get_default_edge()));
        p("switch.stats", list(stats_of(sw.0)));
        sw.add_case(JList::from(vec![None, None]), s2);
        p("switch.cases", list(sw.get_case_statements().to_vec()));
        p("switch.stats2", list(stats_of(sw.0)));
        p("switch.h.succs", list(h.get_all_successor_edges()));
        sw.remove_case(s2);
        p("switch.cases3", list(sw.get_case_statements().to_vec()));
        p("switch.stats3", list(stats_of(sw.0)));
        p("switch.h.succs3", list(h.get_all_successor_edges()));
    }

    setup();
    {
        let (head, handler, next) = (bb(), bb(), bb());
        let general = GeneralStatement::new(head, vec![head, handler, next], None);
        edge(REG, head, next);
        head.add_successor(StatEdge::new_exceptions(
            head,
            handler,
            Some(vec!["java/lang/RuntimeException".into()]),
        ));
        edge(REG, handler, next);
        let cs = CatchStatement::is_head(head).unwrap();
        p("catch.stats", list(stats_of(cs)));
        p("catch.post", opt(cs.get_post()));
        let exct = cs.as_catch().get_exct_strings().to_vec();
        p("catch.exct", list(exct.iter().map(|e| list(e.iter()))));
        p("catch.vars", cs.as_catch().get_vars().len().to_string());
        general.collapse_nodes_to_statement(cs);
        p("catch.general.stats", list(stats_of(general.0)));
        p("catch.succs", list(cs.get_all_successor_edges()));
        p("catch.next.preds", list(next.get_all_predecessor_edges()));
    }

    let expected = include_str!("data/k2_graph_oracle/expected.txt");
    for (n, (got, want)) in out.lines().zip(expected.lines()).enumerate() {
        assert_eq!(got, want, "line {}", n + 1);
    }
    assert_eq!(out.lines().count(), expected.lines().count());
}
