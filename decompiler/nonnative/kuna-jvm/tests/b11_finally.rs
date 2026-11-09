//! `FinallyProcessor.iterateGraph` on the methods of real class files, right
//! after `DomHelper.parseGraph` as `MethodProcessor.codeToJava` runs it.

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::dead_code_helper::DeadCodeHelper;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dom_helper::DomHelper;
use kuna_jvm::exception_deobfuscator::ExceptionDeobfuscator;
use kuna_jvm::finally_processor::FinallyProcessor;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::method_processor::MethodProcessor;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, try_catch};
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;

use crate::common;
use crate::k2_dom_helper::start_method;

fn read_class(path: &str) -> StructClass {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    StructClass::create(&mut ByteCursor::new(bytes), true)
}

/// `MethodProcessor.codeToJava` from `expandData` through the first `DomHelper.parseGraph`.
fn parse(cl: StructClass, mt: StructMethod) -> Option<(ControlFlowGraph, RootStatement)> {
    try_catch(|| {
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
        (graph, DomHelper::parse_graph(graph, mt, 0))
    })
    .ok()
}

/// Whether `iterateGraph`'s walk meets a catch-all it has to analyse.
fn has_fresh_catch_all(stat: Statement) -> bool {
    if let Some(parent) = stat.get_parent() {
        if parent.is_catch_all() && parent.get_first() == Some(stat) && !parent.is_copied() {
            return true;
        }
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().any(has_fresh_catch_all)
}

#[test]
fn iterate_graph_walks_real_methods() {
    let _context = common::install_default_context();
    let cfg = format!("{}/tests/data/cfg/classes", env!("CARGO_MANIFEST_DIR"));
    let mut files = vec![format!(
        "{}/tests/data/k2_graph_oracle/classes/K2Shapes.class",
        env!("CARGO_MANIFEST_DIR")
    )];
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

    let (mut parsed, mut without, mut with) = (0, 0, 0);
    let mut mismatches = Vec::new();
    for path in &files {
        let cl = read_class(path);
        DecompilerContext::set_current_class(Some(cl));
        let class = cl.qualified_name();
        let mts: Vec<StructMethod> = cl.get_methods().iter().copied().collect();
        for mt in mts {
            if !mt.contains_code() {
                continue;
            }
            reset_class_scope();
            MethodProcessor::set_debug_currently_decompiling(None);
            start_method(mt);
            let name = format!("{class}.{}{}", mt.get_name(), mt.get_descriptor());
            let Some((graph, root)) = parse(cl, mt) else {
                continue;
            };
            parsed += 1;
            let expect_finally = has_fresh_catch_all(root.0);

            let var_proc = DecompilerContext::get_var_processor();
            let mut proc = FinallyProcessor::new(mt, &mt.method_descriptor(), var_proc);
            let result = try_catch(|| proc.iterate_graph(cl, mt, root, graph));
            let reached_finally = match &result {
                Ok(false) => false,
                Err(e)
                    if e.is("UnportedError")
                        && e.message.contains("ExprProcessor.processStatement") =>
                {
                    true
                }
                other => {
                    mismatches.push(format!(
                        "{name}: {:?}",
                        other.as_ref().map_err(|e| &e.message)
                    ));
                    continue;
                }
            };
            if reached_finally {
                with += 1;
            } else {
                without += 1;
            }
            if reached_finally != expect_finally {
                mismatches.push(format!(
                    "{name}: catch-all in tree {expect_finally}, reached getFinallyInformation {reached_finally}"
                ));
            }
        }
        DecompilerContext::set_current_class(None);
    }
    reset_class_scope();

    eprintln!("iterateGraph on {parsed} parsed methods: {with} reach getFinallyInformation, {without} return false");
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    assert_eq!(parsed, 180);
    assert!(with > 0, "no method with a finally in the corpus");
}
