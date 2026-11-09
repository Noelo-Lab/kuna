//! `FlattenStatementsHelper` and the SSA / SSAU constructors on hand-built
//! statement graphs decomposed by `DomHelper::process_statement`, replayed
//! against the Vineflower 1.12.0 oracle in
//! `tests/data/b06_sforms_oracle/SFormsOracle.java`. A section that stops at a
//! callee another slice has not ported yet is skipped, and reported.

use std::cell::RefCell;

use kuna_jvm::assignment_exprent::AssignmentExprent;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::catch_all_statement::CatchAllStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::direct_edge::DirectEdge;
use kuna_jvm::direct_edge_type::DirectEdgeType;
use kuna_jvm::direct_graph::DirectGraph;
use kuna_jvm::direct_node::DirectNode;
use kuna_jvm::do_statement::{DoStatement, Type as LoopType};
use kuna_jvm::dom_helper::{DomHelper, ExtPostMap};
use kuna_jvm::dom_tracer::DomTracer;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exprent::Exprent;
use kuna_jvm::flatten_statements_helper::FlattenStatementsHelper;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch, JHashSet, JList};
use kuna_jvm::s_forms_fast_map_direct::VersionSet;
use kuna_jvm::ssa_constructor_sparse_ex::SSAConstructorSparseEx;
use kuna_jvm::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{LastBasicType, Statement};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type::VarType;
use kuna_jvm::var_version_pair::VarVersionPair;

use crate::common;

/// `public int straight(int a, int b)`: the method the oracle mocks.
fn fixture_method() -> StructMethod {
    let path = format!(
        "{}/tests/data/k2_graph_oracle/classes/K2Shapes.class",
        env!("CARGO_MANIFEST_DIR")
    );
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    StructClass::create(&mut ByteCursor::new(bytes), true)
        .get_method("straight", "(II)I")
        .expect("fixture method")
}

struct Case {
    mt: StructMethod,
    nodes: Vec<Statement>,
    first: Statement,
    exit: DummyExitStatement,
    general: GeneralStatement,
    vars: RefCell<Vec<VarExprent>>,
}

impl Case {
    fn new(mt: StructMethod, n: usize) -> Case {
        let nodes: Vec<Statement> = (0..n).map(|_| BasicBlockStatement::create().0).collect();
        let first = nodes[0];
        let exit = DummyExitStatement::new();
        let general = GeneralStatement::new(first, nodes.clone(), None);
        Case {
            mt,
            nodes,
            first,
            exit,
            general,
            vars: RefCell::new(Vec::new()),
        }
    }

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

    fn link_exception(&self, from: usize, to: usize, type_: Option<&str>) {
        let stat = self.nodes[from];
        stat.add_successor(StatEdge::new_exceptions(
            stat,
            self.nodes[to],
            type_.map(|t| vec![t.to_string()]),
        ));
    }

    fn set_if(&self, node: usize) {
        self.nodes[node].with_mut(|d| d.last_basic_type = LastBasicType::If);
    }

    fn var(&self, index: i32) -> VarExprent {
        let v = VarExprent::new(index, Some(VarType::VARTYPE_INT()), None);
        self.vars.borrow_mut().push(v);
        v
    }

    fn add(&self, block: usize, e: Exprent) {
        let st = self.nodes[block];
        if st.get_exprents().is_none() {
            st.set_exprents(JList::new());
        }
        st.get_exprents().expect("exprents").add(e);
    }

    fn write(&self, block: usize, dst: i32, value: i32) {
        let left = self.var(dst);
        let right = ConstExprent::new_int(value, false, None);
        self.add(block, AssignmentExprent::new(left.0, right.0, None).0);
    }

    fn copy(&self, block: usize, dst: i32, src: i32) {
        let e = self.assign(dst, src);
        self.add(block, e);
    }

    fn assign(&self, dst: i32, src: i32) -> Exprent {
        let left = self.var(dst);
        let right = self.var(src);
        AssignmentExprent::new(left.0, right.0, None).0
    }

    fn vars(&self) -> String {
        list(
            self.vars
                .borrow()
                .iter()
                .map(|v| format!("{}_{}", v.get_index(), v.get_version())),
        )
    }
}

type Scenario = fn(StructMethod) -> Case;
type Customize = fn(&Case, RootStatement);

fn list<T: std::fmt::Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn reset(mt: StructMethod) {
    reset_class_scope();
    DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
    DecompilerContext::get_counter_container().set_counter(CounterContainer::VAR_COUNTER, 3);
}

fn process(out: &mut Vec<String>, key: &str, case: &Case) -> RootStatement {
    case.general.build_continue_set();
    case.general.build_monitor_flags();
    let root = RootStatement::new(case.general.0, case.exit, case.mt);
    let mut tracer = DomTracer::new("domhelper_0", case.mt);
    let ok = DomHelper::process_statement(root.0, root, ExtPostMap::new_linked(), &mut tracer);
    out.push(format!("{key}.ok = {ok}"));
    root
}

fn find_do(stat: Statement) -> Option<DoStatement> {
    if let Some(d) = stat.try_as_do() {
        return Some(d);
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().find_map(find_do)
}

fn find_catch_all(stat: Statement) -> Option<CatchAllStatement> {
    if let Some(c) = stat.try_as_catch_all() {
        return Some(c);
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().find_map(find_catch_all)
}

fn find_ifs(stat: Statement, out: &mut Vec<IfStatement>) {
    if let Some(ifs) = stat.try_as_if() {
        out.push(ifs);
    }
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats {
        find_ifs(st, out);
    }
}

/// Every if statement gets a head exprent reading v5.
fn tail(case: &Case, root: RootStatement) {
    let mut ifs = Vec::new();
    find_ifs(root.0, &mut ifs);
    for if_stat in ifs {
        let head = if_stat.get_headexprent_list();
        let v = case.var(5).0;
        if head.is_empty() {
            head.add(v);
        } else {
            head.set(0, v);
        }
    }
}

fn edge_ids(edges: JList<DirectEdge>, dest: bool) -> String {
    list(edges.to_vec().into_iter().map(|e| {
        if dest {
            e.get_destination().id()
        } else {
            e.get_source().id()
        }
    }))
}

fn node_id(node: Option<DirectNode>) -> String {
    node.map_or_else(|| "null".to_string(), |n| n.id())
}

fn dump_graph(out: &mut Vec<String>, key: &str, g: DirectGraph) {
    out.push(format!(
        "{key}.flat.first = {}",
        g.first().expect("first").id()
    ));
    for node in g.nodes() {
        out.push(format!(
            "{key}.flat.node = {} {} stat={} block={} tf={} ex={} succ={} esucc={} pred={} epred={}",
            node.id(),
            node.type_().name(),
            node.statement().id(),
            node.block()
                .map_or_else(|| "null".to_string(), |b| b.id().to_string()),
            node_id(node.try_finally()),
            node.exprents().len(),
            edge_ids(node.get_successors(DirectEdgeType::Regular), true),
            edge_ids(node.get_successors(DirectEdgeType::Exception), true),
            edge_ids(node.get_predecessors(DirectEdgeType::Regular), false),
            edge_ids(node.get_predecessors(DirectEdgeType::Exception), false),
        ));
    }
    out.push(format!(
        "{key}.flat.extra = {}",
        list(g.extra_nodes().into_iter().map(|n| n.id()))
    ));
    let neg: Vec<String> = g.with(|d| {
        d.map_neg_if_branch
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect()
    });
    out.push(format!("{key}.flat.neg = {{{}}}", neg.join(", ")));
}

fn pair(p: &VarVersionPair) -> String {
    format!("{}_{}", p.var, p.version)
}

fn set(s: &VersionSet) -> String {
    let items: Vec<i32> = s.borrow().iter().collect();
    list(items)
}

/// Runs `body`; an `UnportedError` blocks the section, any other throw is its `.error` line.
fn section(out: &mut Vec<String>, prefix: &str, body: impl FnOnce(&mut Vec<String>)) {
    let mut lines = Vec::new();
    match try_catch(|| {
        body(&mut lines);
        lines
    }) {
        Ok(lines) => out.extend(lines),
        Err(e) if e.is("UnportedError") => out.push(format!("BLOCKED {prefix} {}", e.message)),
        Err(e) => out.push(format!("{prefix}error = {}", e.message)),
    }
}

fn run(
    out: &mut Vec<String>,
    key: &str,
    mt: StructMethod,
    scenario: Scenario,
    customize: Customize,
    with_ssa: bool,
) {
    reset(mt);
    let case = scenario(mt);
    let root = process(out, key, &case);
    customize(&case, root);
    section(out, &format!("{key}.flat."), |out| {
        let g = FlattenStatementsHelper::new().build_direct_graph(root);
        dump_graph(out, key, g);
    });
    if !with_ssa {
        return;
    }

    section(out, &format!("{key}.ssa."), |out| {
        let mut ssa = SSAConstructorSparseEx::new();
        ssa.split_variables(root, mt);
        out.push(format!("{key}.ssa.vars = {}", case.vars()));
        let phi: Vec<String> = ssa
            .get_phi()
            .iter()
            .map(|(k, v)| format!("{}={}", pair(k), set(v)))
            .collect();
        out.push(format!("{key}.ssa.phi = {}", list(phi)));
        let lookup: Vec<String> = ssa
            .get_simple_reverse_phi_lookup()
            .iter()
            .map(|(k, v)| format!("{}={v}", pair(k)))
            .collect();
        out.push(format!("{key}.ssa.lookup = {}", list(lookup)));
        out.push(format!(
            "{key}.ssa.reads = {},{}",
            ssa.get_read_count(VarVersionPair::new(5, 1)),
            ssa.get_read_count(VarVersionPair::new(1, 1))
        ));
    });

    reset(mt);
    let case = scenario(mt);
    let root = process(out, &format!("{key}.u"), &case);
    customize(&case, root);
    section(out, &format!("{key}.ssau."), |out| {
        let mut ssau = SSAUConstructorSparseEx::new();
        ssau.split_variables(root, mt);
        out.push(format!("{key}.ssau.vars = {}", case.vars()));
        let g = ssau.get_ssu_versions();
        let all = g.nodes();
        for &node in &all {
            let preds = list(node.predecessors().iter().map(|n| pair(&n.as_pair())));
            let succs = list(node.successors().iter().map(|n| pair(&n.as_pair())));
            let mut doms = String::new();
            for &other in &all {
                let mut one = JHashSet::new();
                one.add(other);
                doms.push(match try_catch(|| g.is_dominator_set(node, &one)) {
                    Ok(true) => '1',
                    Ok(false) => '0',
                    Err(_) => 'E',
                });
            }
            let live = node
                .live()
                .map_or_else(|| "null".to_string(), |l| l.borrow().to_string());
            out.push(format!(
                "{key}.ssau.node = {} {} preds={preds} succs={succs} live={live} doms={doms}",
                pair(&node.as_pair()),
                node.state().map_or("null", |s| s.name()),
            ));
        }
        let ranges: Vec<String> = ssau
            .get_map_version_first_range()
            .iter()
            .map(|(k, v)| format!("{}={v}", pair(k)))
            .collect();
        out.push(format!("{key}.ssau.ranges = {}", list(ranges)));
        let assigns: Vec<String> = ssau
            .get_var_assignment_map()
            .iter()
            .map(|(k, v)| format!("{}={}", pair(k), pair(v)))
            .collect();
        out.push(format!("{key}.ssau.assign = {}", list(assigns)));
    });
}

fn none(_: &Case, _: RootStatement) {}

fn customize_loop(
    case: &Case,
    root: RootStatement,
    type_: LoopType,
    init: bool,
    cond: bool,
    inc: bool,
) {
    let d = find_do(root.0).expect("do statement");
    d.set_looptype(type_);
    if type_ == LoopType::ForEach {
        d.set_init_exprent(case.var(9).0);
        d.set_inc_exprent(case.var(5).0);
        return;
    }
    if init {
        d.set_init_exprent(case.assign(9, 5));
    }
    if cond {
        d.set_condition_exprent(case.var(5).0);
    }
    if inc {
        d.set_inc_exprent(case.assign(9, 5));
    }
}

fn ifelse(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.set_if(0);
    c.link(0, 1);
    c.link(0, 2);
    c.link(1, 3);
    c.link(2, 3);
    c.link(3, -1);
    c.write(0, 5, 1);
    c.write(1, 5, 2);
    c.copy(2, 6, 5);
    c.copy(3, 7, 5);
    c.copy(3, 8, 1);
    c
}

fn seq3(mt: StructMethod) -> Case {
    let c = Case::new(mt, 3);
    c.link(0, 1);
    c.link(1, 2);
    c.link(2, -1);
    c.write(0, 5, 1);
    c.copy(1, 5, 5);
    c.copy(2, 6, 5);
    c.copy(2, 2, 6);
    c
}

fn loop2(mt: StructMethod) -> Case {
    let c = Case::new(mt, 2);
    c.set_if(1);
    c.link(0, 1);
    c.link(1, 0);
    c.link(1, -1);
    c.copy(0, 5, 1);
    c.copy(1, 1, 5);
    c
}

fn ifbreak(mt: StructMethod) -> Case {
    let c = Case::new(mt, 3);
    c.set_if(0);
    c.link(0, 1);
    c.link(0, -1);
    c.link(1, 2);
    c.link(2, -1);
    c.write(0, 5, 1);
    c.write(1, 5, 2);
    c.copy(2, 6, 5);
    c
}

fn inner(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.set_if(2);
    c.link(0, 1);
    c.link(1, 2);
    c.link(2, 1);
    c.link(2, 3);
    c.link(3, -1);
    c.write(0, 5, 0);
    c.copy(1, 6, 5);
    c.copy(2, 5, 6);
    c.copy(3, 7, 5);
    c
}

fn exitloop(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.set_if(1);
    c.link(0, 1);
    c.link(1, 2);
    c.link(1, 3);
    c.link(2, 1);
    c.link(3, -1);
    c.write(0, 5, 0);
    c.copy(1, 6, 5);
    c.copy(2, 5, 6);
    c.copy(3, 7, 5);
    c
}

fn loopif(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.set_if(0);
    c.set_if(2);
    c.link(0, 1);
    c.link(0, 3);
    c.link(1, 2);
    c.link(2, 1);
    c.link(2, 3);
    c.link(3, -1);
    c.write(0, 5, 0);
    c.copy(1, 6, 5);
    c.copy(2, 5, 6);
    c.copy(3, 7, 5);
    c
}

fn trycatch(mt: StructMethod) -> Case {
    let c = Case::new(mt, 3);
    c.link(0, 2);
    c.link_exception(0, 1, Some("java/lang/Exception"));
    c.link(1, 2);
    c.link(2, -1);
    c.write(0, 5, 1);
    c.write(0, 5, 2);
    c.copy(1, 6, 3);
    c.copy(2, 7, 5);
    c
}

fn trycatch2(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.link(0, 1);
    c.link(1, 3);
    c.link_exception(1, 2, Some("java/lang/Exception"));
    c.link(2, 3);
    c.link(3, -1);
    c.write(0, 5, 1);
    c.write(1, 5, 2);
    c.write(1, 5, 3);
    c.copy(2, 6, 5);
    c.copy(2, 7, 3);
    c.copy(3, 8, 5);
    c
}

fn catchall(mt: StructMethod) -> Case {
    let c = Case::new(mt, 4);
    c.link(0, 1);
    c.link(1, 3);
    c.link_exception(1, 2, None);
    c.link(2, -1);
    c.link(3, -1);
    c.write(0, 5, 1);
    c.write(1, 5, 2);
    c.copy(2, 6, 5);
    c.copy(3, 7, 5);
    c
}

fn loop3(mt: StructMethod) -> Case {
    let c = Case::new(mt, 5);
    c.set_if(2);
    c.link(0, 1);
    c.link(1, 2);
    c.link(2, 3);
    c.link(3, 1);
    c.link(2, 4);
    c.link(4, -1);
    c.write(0, 5, 0);
    c.copy(1, 6, 5);
    c.copy(2, 7, 6);
    c.copy(3, 5, 7);
    c.copy(4, 8, 5);
    c
}

fn tryloop(mt: StructMethod) -> Case {
    let c = Case::new(mt, 5);
    c.set_if(0);
    c.set_if(3);
    c.link(0, 1);
    c.link(0, 4);
    c.link(1, 3);
    c.link_exception(1, 2, Some("java/lang/Exception"));
    c.link(2, 3);
    c.link(3, 1);
    c.link(3, 4);
    c.link(4, -1);
    c.write(0, 5, 0);
    c.copy(1, 5, 5);
    c.copy(2, 6, 5);
    c.copy(2, 7, 3);
    c.copy(3, 8, 5);
    c.copy(4, 9, 5);
    c
}

#[test]
fn flatten_and_ssa_match_the_vineflower_oracle() {
    let _context = common::install_context_with(&[(IFernflowerPreferences::TRY_LOOP_FIX, "0")]);
    let mt = fixture_method();
    let mut out = Vec::new();

    run(&mut out, "ifelse", mt, ifelse, none, true);
    run(&mut out, "ifelse_tail", mt, ifelse, tail, true);
    run(&mut out, "seq3", mt, seq3, none, true);
    run(&mut out, "loop2", mt, loop2, none, true);
    run(&mut out, "ifbreak", mt, ifbreak, none, true);
    run(&mut out, "inner", mt, inner, none, true);
    run(&mut out, "exitloop", mt, exitloop, none, true);
    run(&mut out, "loopif", mt, loopif, none, true);
    run(
        &mut out,
        "loopif_while",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::While, false, true, false),
        true,
    );
    run(
        &mut out,
        "loopif_while_empty",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::While, false, false, false),
        false,
    );
    run(
        &mut out,
        "loopif_dowhile",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::DoWhile, false, true, false),
        true,
    );
    run(
        &mut out,
        "loopif_for",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::For, true, true, true),
        true,
    );
    run(
        &mut out,
        "loopif_for_noinit",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::For, false, true, true),
        true,
    );
    run(
        &mut out,
        "loopif_foreach",
        mt,
        loopif,
        |c, r| customize_loop(c, r, LoopType::ForEach, false, false, false),
        true,
    );
    run(&mut out, "loopif_tail", mt, loopif, tail, true);
    run(&mut out, "trycatch", mt, trycatch, none, true);
    run(&mut out, "trycatch2", mt, trycatch2, none, true);
    run(&mut out, "catchall", mt, catchall, none, true);
    run(
        &mut out,
        "finally",
        mt,
        catchall,
        |_, r| find_catch_all(r.0).expect("catch-all").set_finally(true),
        true,
    );
    run(&mut out, "loop3", mt, loop3, none, true);
    run(&mut out, "tryloop", mt, tryloop, none, true);
    reset_class_scope();

    let expected: Vec<&str> = include_str!("data/b06_sforms_oracle/expected.txt")
        .lines()
        .collect();
    let mut blocked = Vec::new();
    let mut want = expected.iter().peekable();
    let mut compared = 0;
    for got in &out {
        if let Some(rest) = got.strip_prefix("BLOCKED ") {
            let (prefix, why) = rest.split_once(' ').expect("blocked line");
            blocked.push(format!("{prefix} {why}"));
            while want.peek().is_some_and(|w| w.starts_with(prefix)) {
                want.next();
            }
            continue;
        }
        let line = want
            .next()
            .unwrap_or_else(|| panic!("extra output line: {got}"));
        assert_eq!(got, line, "after {compared} matching lines");
        compared += 1;
    }
    assert!(want.next().is_none(), "missing output lines");
    eprintln!(
        "b06 oracle: {compared} lines matched, {} sections blocked: {blocked:?}",
        blocked.len()
    );
    assert!(
        blocked.iter().all(|b| !b.contains(".flat.")),
        "flattening must not be blocked: {blocked:?}"
    );
}
