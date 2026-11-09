//! Dominator-based decomposition: a replay of the Vineflower 1.12.0 oracle in
//! `tests/data/k2_graph_oracle/DomOracle.java`, and `DomHelper::parse_graph` on
//! the methods of real class files.

use std::collections::BTreeMap;

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::dead_code_helper::DeadCodeHelper;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dom_helper::{DomHelper, ExtPostMap};
use kuna_jvm::dom_tracer::DomTracer;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::exception_deobfuscator::ExceptionDeobfuscator;
use kuna_jvm::fast_extended_postdominance_helper::FastExtendedPostdominanceHelper;
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::method_processor::MethodProcessor;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch};
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::{LastBasicType, Statement};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_processor::VarProcessor;

use crate::common::{self, ContextGuard};

fn read_class(path: &str) -> StructClass {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    StructClass::create(&mut ByteCursor::new(bytes), true)
}

fn fixture_dir() -> String {
    format!(
        "{}/tests/data/k2_graph_oracle/classes",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// A method of the committed fixture class.
pub(crate) fn fixture_method() -> StructMethod {
    read_class(&format!("{}/K2Shapes.class", fixture_dir()))
        .get_method("straight", "(II)I")
        .expect("fixture method")
}

/// `ClassWrapper.init`'s per-method setup: a fresh var processor and counter container.
pub(crate) fn start_method(mt: StructMethod) {
    DecompilerContext::start_method(VarProcessor::new(mt, &mt.method_descriptor()));
}

/// The oracles run with an empty option map; `TRY_LOOP_FIX` (on by default)
/// is the only option these paths read.
pub(crate) fn oracle_context() -> ContextGuard {
    common::install_context_with(&[(IFernflowerPreferences::TRY_LOOP_FIX, "0")])
}

struct Graph {
    nodes: Vec<Statement>,
    first: Statement,
    exit: DummyExitStatement,
    general: GeneralStatement,
    mt: StructMethod,
}

fn graph(mt: StructMethod, n: usize) -> Graph {
    reset_class_scope();
    start_method(mt);
    let nodes: Vec<Statement> = (0..n).map(|_| BasicBlockStatement::create().0).collect();
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
    let root = RootStatement::new(g.general.0, g.exit, g.mt);
    let mut tracer = DomTracer::new("domhelper_0", g.mt);
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
        .map(|i| format!("{}={}", opt(keys[i]), list(post.get(i))))
        .collect();
    out.push_str(&format!("{key}.post = [{}]\n", vb.join(", ")));
}

#[test]
fn decomposition_matches_the_vineflower_oracle() {
    let _context = oracle_context();
    let mt = fixture_method();
    let mut out = String::new();

    let g = graph(mt, 4);
    g.set_if(0);
    g.link(0, Some(1));
    g.link(0, Some(2));
    g.link(1, Some(3));
    g.link(2, Some(3));
    g.link(3, None);
    postdoms(&mut out, "diamond", &g);
    process(&mut out, "ifelse", &g);

    let g = graph(mt, 4);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(1, Some(3));
    g.link(3, None);
    postdoms(&mut out, "loop", &g);

    let g = graph(mt, 3);
    g.link(0, Some(2));
    g.link_exception(0, 1, "java/lang/Exception");
    g.link(1, Some(2));
    g.link(2, None);
    postdoms(&mut out, "handler", &g);

    let g = graph(mt, 3);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "seq3", &g);

    let g = graph(mt, 2);
    g.set_if(1);
    g.link(0, Some(1));
    g.link(1, Some(0));
    g.link(1, None);
    process(&mut out, "loop2", &g);

    let g = graph(mt, 3);
    g.set_if(0);
    g.link(0, Some(1));
    g.link(0, None);
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "ifbreak", &g);

    let g = graph(mt, 4);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(2, Some(3));
    g.link(3, None);
    process(&mut out, "inner_general_tail", &g);

    let g = graph(mt, 4);
    g.set_if(2);
    g.link(0, Some(1));
    g.link(1, Some(2));
    g.link(2, Some(1));
    g.link(2, Some(3));
    g.link(3, None);
    process(&mut out, "inner", &g);

    let g = graph(mt, 3);
    g.link(0, Some(2));
    g.link_exception(0, 1, "java/lang/Exception");
    g.link(1, Some(2));
    g.link(2, None);
    process(&mut out, "trycatch", &g);

    let g = graph(mt, 5);
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

/// `MethodProcessor.codeToJava` from `expandData` through `DomHelper.parseGraph`.
fn code_to_statement(cl: StructClass, mt: StructMethod) -> Result<RootStatement, String> {
    let graph = try_catch(|| {
        mt.expand_data(cl);
        let seq = mt.get_instruction_sequence().expect("expanded method");
        let graph = ControlFlowGraph::new(seq);

        DecompilerContext::get_counter_container()
            .set_counter(CounterContainer::VAR_COUNTER, mt.get_local_variables());

        DeadCodeHelper::remove_dead_blocks(graph);
        if mt.get_bytecode_version().has_jsr()
            || DecompilerContext::get_option(IFernflowerPreferences::FORCE_JSR_INLINE)
        {
            graph.inline_jsr(cl, mt);
        }
        DeadCodeHelper::connect_dummy_exit_block(graph);
        DeadCodeHelper::remove_gotos(graph);

        ExceptionDeobfuscator::remove_circular_ranges(graph);
        ExceptionDeobfuscator::restore_pop_ranges(graph);
        if DecompilerContext::get_option(IFernflowerPreferences::REMOVE_EMPTY_RANGES) {
            ExceptionDeobfuscator::remove_empty_ranges(graph);
        }
        if DecompilerContext::get_option(IFernflowerPreferences::ENSURE_SYNCHRONIZED_MONITOR) {
            DeadCodeHelper::extend_synchronized_range_to_monitorexit(graph);
        }
        if DecompilerContext::get_option(IFernflowerPreferences::INCORPORATE_RETURNS) {
            DeadCodeHelper::incorporate_value_returns(graph);
        }
        ExceptionDeobfuscator::insert_empty_exception_handler_blocks(graph);
        DeadCodeHelper::merge_basic_blocks(graph);
        if ExceptionDeobfuscator::has_obfuscated_exceptions(graph) {
            if !ExceptionDeobfuscator::handle_multiple_entry_exception_ranges(graph) {
                graph.add_comment("$VF: Could not handle exception ranges with multiple entries");
                graph.with_mut(|d| d.add_error_comment = true);
            }
            ExceptionDeobfuscator::insert_dummy_exception_handler_blocks(
                graph,
                mt.get_bytecode_version(),
            );
        }
        graph
    })
    .map_err(|e| format!("cfg: {}", e.message))?;

    try_catch(|| DomHelper::parse_graph(graph, mt, 0)).map_err(|e| {
        if e.is("UnportedError") {
            format!("unported: {}", e.message)
        } else {
            format!("parse_graph: {}", e.message)
        }
    })
}

/// The methods of these classes that Vineflower 1.12.0 itself cannot
/// decompile, with the exception it reports (`IndexOutOfBoundsException` from
/// the `IfStatement` constructor is an index panic here).
const VINEFLOWER_FAILURES: [(&str, &str); 2] = [
    (
        "CfgOddities.strandedEmptyRange()V",
        "parse_graph: RuntimeException: parsing failure!",
    ),
    (
        "CfgOddities.fallsOffTheEnd(I)V",
        "parse_graph: index out of bounds: the len is 1 but the index is 1",
    ),
];

/// `parse_graph` on every method of real classes: each one gets through
/// `processStatement` and finishes, except where Vineflower fails too.
#[test]
fn parse_graph_runs_on_real_methods() {
    let _context = common::install_default_context();
    let cfg = format!("{}/tests/data/cfg/classes", env!("CARGO_MANIFEST_DIR"));
    let mut files = vec![format!("{}/K2Shapes.class", fixture_dir())];
    for set in ["r8", "r21", "jasm", "legacy"] {
        let mut set_files: Vec<String> = std::fs::read_dir(format!("{cfg}/{set}"))
            .expect("class dir")
            .map(|e| e.expect("entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "class"))
            .map(|p| p.display().to_string())
            .collect();
        set_files.sort();
        files.extend(set_files);
    }

    let mut methods = 0;
    let mut finished = 0;
    let mut decomposed = 0;
    let mut cfg_failures = Vec::new();
    let mut blocked: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures = Vec::new();
    for path in &files {
        let cl = read_class(path);
        DecompilerContext::set_current_class(Some(cl));
        let class = cl.qualified_name();
        let mts: Vec<StructMethod> = cl.get_methods().iter().copied().collect();
        for mt in mts {
            if !mt.contains_code() {
                continue;
            }
            methods += 1;
            reset_class_scope();
            MethodProcessor::set_debug_currently_decompiling(None);
            start_method(mt);
            let name = format!("{class}.{}{}", mt.get_name(), mt.get_descriptor());
            let result = code_to_statement(cl, mt);
            if MethodProcessor::debug_currently_decompiling().is_some() {
                decomposed += 1;
            }
            match result {
                Ok(_) => finished += 1,
                Err(e) if e.starts_with("cfg: ") => cfg_failures.push(format!("{name}: {e}")),
                Err(e) if e.starts_with("unported: ") => *blocked.entry(e).or_default() += 1,
                Err(e) => failures.push((name, e)),
            }
        }
        DecompilerContext::set_current_class(None);
    }
    reset_class_scope();

    eprintln!(
        "parse_graph on {methods} methods of {} classes: {finished} finished, \
         {decomposed} through processStatement, {} stopped in the CFG layer, blocked by {blocked:?}",
        files.len(),
        cfg_failures.len()
    );
    let expected: Vec<(String, String)> = VINEFLOWER_FAILURES
        .iter()
        .map(|(m, e)| (m.to_string(), e.to_string()))
        .collect();
    assert_eq!(failures, expected, "methods that threw in parse_graph");
    assert!(
        cfg_failures.is_empty(),
        "CFG construction failed:\n{}",
        cfg_failures.join("\n")
    );
    assert_eq!(methods, 182);
    assert_eq!(decomposed + failures.len(), methods);
    assert_eq!(finished + failures.len(), methods, "blocked by {blocked:?}");
}
