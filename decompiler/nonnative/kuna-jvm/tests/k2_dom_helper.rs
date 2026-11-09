//! Dominator-based decomposition replayed against the Vineflower 1.12.0
//! oracle in `tests/data/k2_graph_oracle/DomOracle.java`.
//!
//! NOT registered in `tests/integration.rs` yet: `RootStatement` and
//! `DomTracer` need a `StructMethod`, and `test_method` below builds one from
//! slice K2's throwaway stub. Replace `test_method` with a real constructor
//! once the struct slices are merged, then register the module.

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::dom_helper::{DomHelper, ExtPostMap};
use kuna_jvm::dom_tracer::DomTracer;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::fast_extended_postdominance_helper::FastExtendedPostdominanceHelper;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch};
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{LastBasicType, Statement};
use kuna_jvm::struct_method::{StructMethod, StructMethodData};

fn test_method() -> StructMethod {
    StructMethod::alloc(StructMethodData {})
}

struct Graph {
    nodes: Vec<Statement>,
    first: Statement,
    exit: DummyExitStatement,
    general: GeneralStatement,
}

fn graph(n: usize) -> Graph {
    reset_class_scope();
    let nodes: Vec<Statement> = (0..n).map(|_| BasicBlockStatement::create().0).collect();
    let first = nodes[0];
    let exit = DummyExitStatement::new();
    let general = GeneralStatement::new(first, nodes.clone(), None);
    Graph {
        nodes,
        first,
        exit,
        general,
    }
}

impl Graph {
    fn link(&self, from: usize, to: Option<usize>) {
        let stat = self.nodes[from];
        let mut dest = to.map_or(self.exit.0, |t| self.nodes[t]);
        let type_ = if dest == self.first {
            dest = self.general.0;
            StatEdge::TYPE_CONTINUE
        } else if to.is_none() {
            StatEdge::TYPE_BREAK
        } else {
            StatEdge::TYPE_REGULAR
        };
        let closure = (type_ != StatEdge::TYPE_REGULAR).then_some(self.general.0);
        stat.add_successor(StatEdge::new_closure(type_, stat, dest, closure));
    }

    fn link_exception(&self, from: usize, to: usize, type_: &str) {
        let stat = self.nodes[from];
        stat.add_successor(StatEdge::new_exceptions(
            stat,
            self.nodes[to],
            Some(vec![type_.to_string()]),
        ));
    }

    fn set_if(&self, node: usize) {
        self.nodes[node].with_mut(|d| d.last_basic_type = LastBasicType::If);
    }
}

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn opt<T: std::fmt::Display>(item: Option<T>) -> String {
    item.map_or_else(|| "null".to_string(), |x| x.to_string())
}

fn dump(out: &mut String, key: &str, s: Statement, indent: &str) {
    let mut labels: Vec<String> = s
        .get_label_edges()
        .borrow()
        .to_vec()
        .iter()
        .map(|e| e.to_string())
        .collect();
    labels.sort();
    let mut line = format!(
        "{indent}{s} first={} succs={} labels={}",
        opt(s.get_first()),
        list(s.get_all_successor_edges()),
        list(labels)
    );
    if let Some(ifs) = s.try_as_if() {
        line.push_str(&format!(
            " iftype={} negated={} ifstat={} elsestat={}",
            ifs.iftype(),
            ifs.is_negated(),
            opt(ifs.get_ifstat()),
            opt(ifs.get_elsestat())
        ));
    } else if let Some(dos) = s.try_as_do() {
        line.push_str(&format!(" looptype={}", dos.get_looptype() as i32));
    } else if let Some(cs) = s.try_as_catch() {
        let exct = cs.get_exct_strings().to_vec();
        line.push_str(&format!(
            " exct={}",
            list(exct.iter().map(|e| list(e.iter())))
        ));
    } else if let Some(gs) = s.try_as_general() {
        line.push_str(&format!(" placeholder={}", gs.is_placeholder()));
    }
    out.push_str(&format!("{key} = {line}\n"));
    let child_indent = format!("{indent}  ");
    let stats = s.get_stats().borrow().to_vec();
    for st in stats {
        dump(out, key, st, &child_indent);
    }
}

fn process(out: &mut String, key: &str, g: &Graph) {
    g.general.build_continue_set();
    g.general.build_monitor_flags();
    let root = RootStatement::new(g.general.0, g.exit, test_method());
    let mut tracer = DomTracer::new("domhelper_0", test_method());
    match try_catch(|| {
        DomHelper::process_statement(root.0, root, ExtPostMap::new_linked(), &mut tracer)
    }) {
        Ok(ok) => out.push_str(&format!("{key}.ok = {ok}\n")),
        Err(caught) => out.push_str(&format!("{key}.error = java.lang.{}\n", caught.message)),
    }
    let comments = root.d(|d| d.comment_lines.as_ref().map(|c| list(c.to_vec())));
    out.push_str(&format!("{key}.comments = {}\n", opt(comments)));
    dump(out, key, root.0, "");
}

fn postdoms(out: &mut String, key: &str, g: &Graph) {
    let ext = FastExtendedPostdominanceHelper::new().get_extended_postdominators(g.general.0);
    let entries: Vec<String> = ext
        .iter()
        .map(|(k, v)| format!("{k}={}", list(v.to_vec())))
        .collect();
    out.push_str(&format!("{key}.ext = {{{}}}\n", entries.join(", ")));
    let post = DomHelper::calc_post_dominators(g.general.0);
    let keys = post.get_lst_keys().to_vec();
    let vb: Vec<String> = (0..keys.len())
        .map(|i| format!("{}={}", keys[i], list(post.get(i))))
        .collect();
    out.push_str(&format!("{key}.post = [{}]\n", vb.join(", ")));
}

#[test]
fn decomposition_matches_the_vineflower_oracle() {
    let mut out = String::new();

    let g = graph(4);
    g.set_if(0);
    g.link(0, Some(1));
    g.link(0, Some(2));
    g.link(1, Some(3));
    g.link(2, Some(3));
    g.link(3, None);
    postdoms(&mut out, "diamond", &g);
    process(&mut out, "ifelse", &g);

    let g = graph(4);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(1, Some(3));
    g.link(3, None);
    postdoms(&mut out, "loop", &g);

    let g = graph(3);
    g.link(0, Some(2));
    g.link_exception(0, 1, "java/lang/Exception");
    g.link(1, Some(2));
    g.link(2, None);
    postdoms(&mut out, "handler", &g);

    let g = graph(3);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "seq3", &g);

    let g = graph(2);
    g.set_if(1);
    g.link(0, Some(1));
    g.link(1, Some(0));
    g.link(1, None);
    process(&mut out, "loop2", &g);

    let g = graph(3);
    g.set_if(0);
    g.link(0, Some(1));
    g.link(0, None);
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "ifbreak", &g);

    let g = graph(4);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(2, Some(3));
    g.link(3, None);
    process(&mut out, "inner_general_tail", &g);

    let g = graph(4);
    g.set_if(2);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(2, Some(3));
    g.link(3, None);
    process(&mut out, "inner", &g);

    let g = graph(3);
    g.link(0, Some(2));
    g.link_exception(0, 1, "java/lang/Exception");
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "trycatch", &g);

    let g = graph(5);
    g.set_if(2);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(3));
    g.link(3, Some(1));
    g.link(2, Some(4));
    g.link(4, None);
    process(&mut out, "loop3", &g);

    let expected = include_str!("data/k2_graph_oracle/dom_expected.txt");
    for (n, (got, want)) in out.lines().zip(expected.lines()).enumerate() {
        assert_eq!(got, want, "line {}", n + 1);
    }
    assert_eq!(out.lines().count(), expected.lines().count());
}
