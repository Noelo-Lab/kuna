//! `SequenceHelper.condenseSequences`, `destroyAndFlattenStatement` and
//! `EliminateLoopsHelper`'s loop elimination on statement trees decomposed by
//! `DomHelper.processStatement`: a replay of the Vineflower 1.12.0 oracle in
//! `tests/data/b11_condense/CondenseOracle.java`.

use kuna_jvm::basic_block::BasicBlock;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dom_helper::{DomHelper, ExtPostMap};
use kuna_jvm::dom_tracer::DomTracer;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::eliminate_loops_helper::EliminateLoopsHelper;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch};
use kuna_jvm::sequence_helper::SequenceHelper;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{LastBasicType, Statement};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;

use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};

#[derive(Clone, Copy)]
enum Mode {
    Null,
    Empty,
    Const,
}

struct Graph {
    nodes: Vec<Statement>,
    first: Statement,
    exit: DummyExitStatement,
    general: GeneralStatement,
    mt: StructMethod,
}

fn graph(mt: StructMethod, n: usize, mode: Mode) -> Graph {
    reset_class_scope();
    start_method(mt);
    let mut nodes = Vec::new();
    for i in 0..n {
        let stat = match mode {
            Mode::Null => {
                let id = DecompilerContext::get_counter_container()
                    .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER);
                BasicBlockStatement::new(BasicBlock::new(id))
            }
            Mode::Empty | Mode::Const => {
                let bb = BasicBlockStatement::create();
                if let Mode::Const = mode {
                    bb.get_exprents()
                        .expect("exprents")
                        .add(ConstExprent::new_int(i as i32, false, None).0);
                }
                bb
            }
        };
        nodes.push(stat.0);
    }
    let first = nodes[0];
    let exit = DummyExitStatement::new();
    let general = GeneralStatement::new(first, nodes.clone(), None);
    Graph {
        nodes,
        first,
        exit,
        general,
        mt,
    }
}

impl Graph {
    fn link(&self, from: usize, to: i32) {
        let stat = self.nodes[from];
        let mut dest = if to < 0 {
            self.exit.0
        } else {
            self.nodes[to as usize]
        };
        let type_ = if dest == self.first {
            dest = self.general.0;
            StatEdge::TYPE_CONTINUE
        } else if to < 0 {
            StatEdge::TYPE_BREAK
        } else {
            StatEdge::TYPE_REGULAR
        };
        let closure = (type_ != StatEdge::TYPE_REGULAR).then_some(self.general.0);
        stat.add_successor(StatEdge::new_closure(type_, stat, dest, closure));
    }

    fn links(&self, edges: &[(usize, i32)]) {
        for &(from, to) in edges {
            self.link(from, to);
        }
    }

    fn link_exception(&self, from: usize, to: usize, type_: &str) {
        let stat = self.nodes[from];
        stat.add_successor(StatEdge::new_exceptions(
            stat,
            self.nodes[to],
            Some(vec![type_.to_string()]),
        ));
    }

    fn set_if(&self, nodes: &[usize]) {
        for &node in nodes {
            self.nodes[node].with_mut(|d| d.last_basic_type = LastBasicType::If);
        }
    }
}

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn opt<T: std::fmt::Display>(item: Option<T>) -> String {
    item.map_or_else(|| "null".to_string(), |x| x.to_string())
}

struct Oracle {
    out: String,
    cl: StructClass,
    root: Option<RootStatement>,
}

impl Oracle {
    fn p(&mut self, key: &str, value: impl std::fmt::Display) {
        self.out.push_str(&format!("{key} = {value}\n"));
    }

    fn dump(&mut self, key: &str, s: Statement, indent: &str) {
        let mut labels: Vec<String> = s
            .get_label_edges()
            .borrow()
            .to_vec()
            .iter()
            .map(|e| e.to_string())
            .collect();
        labels.sort();
        let exprents = s
            .get_exprents()
            .map(|l| list(l.to_vec().iter().map(|e| e.get_id())));
        let mut line = format!(
            "{indent}{s} parent={} first={} succs={} preds={} labels={} exprents={}",
            opt(s.get_parent()),
            opt(s.get_first()),
            list(s.get_all_successor_edges()),
            list(s.get_all_predecessor_edges()),
            list(labels),
            opt(exprents)
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
        self.p(key, line);
        let child_indent = format!("{indent}  ");
        let stats = s.get_stats().borrow().to_vec();
        for st in stats {
            self.dump(key, st, &child_indent);
        }
    }

    fn root(&self) -> RootStatement {
        self.root.expect("processed root")
    }

    fn process(&mut self, key: &str, g: &Graph) -> bool {
        g.general.build_continue_set();
        g.general.build_monitor_flags();
        let root = RootStatement::new(g.general.0, g.exit, g.mt);
        let mut tracer = DomTracer::new("domhelper_0", g.mt);
        let ok = DomHelper::process_statement(root.0, root, ExtPostMap::new_linked(), &mut tracer);
        self.root = Some(root);
        self.p(&format!("{key}.ok"), ok);
        ok
    }

    fn condense(&mut self, key: &str) {
        let root = self.root();
        match try_catch(|| SequenceHelper::condense_sequences(root.0)) {
            Ok(()) => self.p(&format!("{key}.condensed"), true),
            Err(_) => self.p(&format!("{key}.error"), true),
        }
        self.dump(key, root.0, "");
    }

    /// `eliminateLoops` stops at `SimplifyExprentsHelper` while that is unported;
    /// the oracle runs exactly the part before it.
    fn eliminate(&mut self, key: &str) {
        let (root, cl) = (self.root(), self.cl);
        let ret = match try_catch(|| EliminateLoopsHelper::eliminate_loops(root.0, cl)) {
            Ok(ret) => ret,
            Err(e)
                if e.is("UnportedError")
                    && e.message
                        .contains("SimplifyExprentsHelper.simplifyStackVarsStatement") =>
            {
                true
            }
            Err(e) => panic!("{key}: eliminateLoops threw {}", e.message),
        };
        self.p(&format!("{key}.eliminated"), ret);
        if ret {
            self.dump(key, root.0, "");
        }
    }

    fn flatten(&mut self, key: &str, pick: fn(Statement) -> bool) {
        let root = self.root();
        let target = find(root.0, pick).expect("flatten target");
        self.p(&format!("{key}.target"), target);
        let bstat = SequenceHelper::destroy_and_flatten_statement(target);
        self.p(&format!("{key}.bstat"), bstat.0);
        self.dump(key, root.0, "");
    }

    fn shapes(&mut self, mt: StructMethod, prefix: &str, mode: Mode) {
        let g = graph(mt, 3, mode);
        g.links(&[(0, 1), (1, 2), (2, -1)]);
        self.run(&format!("{prefix}seq3"), &g);

        let g = graph(mt, 5, mode);
        g.links(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, -1)]);
        self.run(&format!("{prefix}seq5"), &g);

        let g = graph(mt, 4, mode);
        g.set_if(&[0]);
        g.links(&[(0, 1), (0, 2), (1, 3), (2, 3), (3, -1)]);
        self.run(&format!("{prefix}ifelse"), &g);

        let g = graph(mt, 2, mode);
        g.set_if(&[1]);
        g.links(&[(0, 1), (1, 0), (1, -1)]);
        self.run(&format!("{prefix}loop2"), &g);

        let g = graph(mt, 3, mode);
        g.set_if(&[0]);
        g.links(&[(0, 1), (0, -1), (1, 2), (2, -1)]);
        self.run(&format!("{prefix}ifbreak"), &g);

        let g = graph(mt, 4, mode);
        g.set_if(&[2]);
        g.links(&[(0, 1), (1, 2), (2, 1), (2, 3), (3, -1)]);
        self.run(&format!("{prefix}inner"), &g);

        let g = graph(mt, 3, mode);
        g.link(0, 2);
        g.link_exception(0, 1, "java/lang/Exception");
        g.links(&[(1, 2), (2, -1)]);
        self.run(&format!("{prefix}trycatch"), &g);

        let g = graph(mt, 5, mode);
        g.set_if(&[2]);
        g.links(&[(0, 1), (1, 2), (2, 3), (3, 1), (2, 4), (4, -1)]);
        self.run(&format!("{prefix}loop3"), &g);

        let g = graph(mt, 6, mode);
        g.set_if(&[1, 3]);
        g.links(&[
            (0, 1),
            (1, 2),
            (1, 4),
            (2, 3),
            (3, 1),
            (3, 5),
            (4, 5),
            (5, -1),
        ]);
        self.run(&format!("{prefix}loopexits"), &g);
    }

    fn run(&mut self, key: &str, g: &Graph) {
        if self.process(key, g) {
            self.condense(key);
        }
    }

    fn run_eliminate(&mut self, key: &str, g: &Graph) {
        if self.process(key, g) {
            self.condense(key);
            self.eliminate(key);
        }
    }
}

fn find(s: Statement, pick: fn(Statement) -> bool) -> Option<Statement> {
    if pick(s) {
        return Some(s);
    }
    let stats = s.get_stats().borrow().to_vec();
    stats.into_iter().find_map(|st| find(st, pick))
}

fn fixture_class() -> StructClass {
    let path = format!(
        "{}/tests/data/k2_graph_oracle/classes/K2Shapes.class",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    StructClass::create(&mut ByteCursor::new(bytes), true)
}

#[test]
fn condense_matches_the_vineflower_oracle() {
    let _context = oracle_context();
    let mt = fixture_method();
    let mut o = Oracle {
        out: String::new(),
        cl: fixture_class(),
        root: None,
    };

    o.shapes(mt, "null.", Mode::Null);
    o.shapes(mt, "empty.", Mode::Empty);
    o.shapes(mt, "const.", Mode::Const);

    let g = graph(mt, 4, Mode::Null);
    g.set_if(&[2]);
    g.links(&[(0, 1), (1, 2), (2, 1), (2, 3), (3, -1)]);
    o.run_eliminate("elim.inner", &g);

    let g = graph(mt, 5, Mode::Null);
    g.set_if(&[1, 3]);
    g.links(&[(0, 1), (1, 2), (1, 4), (2, 3), (3, 1), (3, 0), (4, -1)]);
    o.run_eliminate("elim.nested", &g);

    let g = graph(mt, 4, Mode::Null);
    g.set_if(&[1, 2]);
    g.links(&[(0, 1), (1, 2), (1, 3), (2, 1), (2, 0), (3, -1)]);
    o.run_eliminate("elim.twoback", &g);

    let g = graph(mt, 4, Mode::Null);
    g.set_if(&[0, 1]);
    g.links(&[(0, 1), (0, 3), (1, 3), (1, 2), (2, 1), (3, 1)]);
    o.run_eliminate("elim.reject", &g);

    let g = graph(mt, 6, Mode::Null);
    g.set_if(&[0, 1, 2, 3, 4]);
    g.links(&[
        (0, 1),
        (0, 2),
        (1, 4),
        (1, 3),
        (2, 5),
        (2, 4),
        (3, 0),
        (3, 1),
        (4, 5),
        (4, 0),
        (5, -1),
    ]);
    o.run_eliminate("elim.found6", &g);

    let g = graph(mt, 8, Mode::Null);
    g.set_if(&[0, 1, 2, 4, 6]);
    g.links(&[
        (0, 1),
        (0, 5),
        (1, 4),
        (1, 3),
        (2, 5),
        (2, 0),
        (3, 1),
        (4, 7),
        (4, 2),
        (5, 6),
        (6, 7),
        (6, 0),
    ]);
    o.run_eliminate("elim.found8", &g);

    let g = graph(mt, 8, Mode::Null);
    g.set_if(&[0, 1, 3, 4, 5, 6]);
    g.links(&[
        (0, 1),
        (0, 3),
        (1, 6),
        (1, 5),
        (2, 7),
        (3, 0),
        (3, 4),
        (4, 6),
        (4, 0),
        (5, 7),
        (5, 1),
        (6, 2),
        (6, 3),
        (7, 2),
    ]);
    o.run_eliminate("elim.breaks8a", &g);

    let g = graph(mt, 8, Mode::Null);
    g.set_if(&[0, 1, 2, 3, 4, 5, 6]);
    g.links(&[
        (0, 2),
        (0, 5),
        (1, 4),
        (1, 6),
        (2, 7),
        (2, 0),
        (3, -1),
        (3, 1),
        (4, 0),
        (4, 5),
        (5, 3),
        (5, -1),
        (6, 2),
        (6, 7),
        (7, -1),
    ]);
    o.run_eliminate("elim.breaks8b", &g);

    let g = graph(mt, 5, Mode::Null);
    g.set_if(&[0, 1, 3]);
    g.links(&[
        (0, 1),
        (0, 4),
        (1, 4),
        (1, 3),
        (2, 0),
        (3, 1),
        (3, 0),
        (4, 2),
    ]);
    o.run_eliminate("elim.breakreject5", &g);

    let g = graph(mt, 7, Mode::Null);
    g.set_if(&[3, 4, 5]);
    g.links(&[
        (0, 2),
        (1, 2),
        (2, 3),
        (3, 4),
        (3, -1),
        (4, 2),
        (4, 6),
        (5, 6),
        (5, 1),
        (6, 5),
    ]);
    o.run_eliminate("elim.breakreject7", &g);

    let g = graph(mt, 4, Mode::Const);
    g.set_if(&[0]);
    g.links(&[(0, 1), (0, 2), (1, 3), (2, 3), (3, -1)]);
    if o.process("flat.if", &g) {
        o.flatten("flat.if", |s| s.is_if());
    }

    let g = graph(mt, 3, Mode::Const);
    g.links(&[(0, 1), (1, 2), (2, -1)]);
    if o.process("flat.block", &g) {
        o.condense("flat.block");
        o.flatten("flat.block", |s| s.is_basic_block());
    }
    reset_class_scope();

    let expected = include_str!("data/b11_condense/expected.txt");
    for (n, (got, want)) in o.out.lines().zip(expected.lines()).enumerate() {
        assert_eq!(got, want, "line {}", n + 1);
    }
    assert_eq!(o.out.lines().count(), expected.lines().count());
}
