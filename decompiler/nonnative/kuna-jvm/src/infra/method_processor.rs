//! Port of `org/jetbrains/java/decompiler/main/rels/MethodProcessor.java`.
//!
//! Only the per-method schedule (`codeToJava`) is ported. The Java class also
//! runs that schedule on a watchdog thread (`run`, `lock`, `getResult`,
//! `isFinished`) to enforce `MAX_PROCESSING_METHOD`; kuna calls
//! [`MethodProcessor::code_to_java`] directly, as Vineflower itself does when
//! the limit is 0 or in unit-test mode.
//!
//! kuna option `jvm_javac_layout` (`kuna_javac_layout`): the Java schedule
//! names its method to the layout decisions, which apply only there.
//!
//! kuna fix `overlapping_ranges` (spec 10.16): ranges that overlap without
//! nesting are split before the multiple-entry check.

use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};

use crate::assert_processor::AssertProcessor;
use crate::cancelation_manager::CancelationManager;
use crate::clear_struct_helper::ClearStructHelper;
use crate::code_constants::CLINIT_NAME;
use crate::concatenation_helper::ConcatenationHelper;
use crate::control_flow_graph::ControlFlowGraph;
use crate::counter_container::CounterContainer;
use crate::dead_code_helper::DeadCodeHelper;
use crate::decompile_record::DecompileRecord;
use crate::decompiler_context::DecompilerContext;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::dom_helper::DomHelper;
use crate::eliminate_loops_helper::EliminateLoopsHelper;
use crate::exception_deobfuscator::ExceptionDeobfuscator;
use crate::exit_helper::ExitHelper;
use crate::expr_processor::ExprProcessor;
use crate::finally_processor::FinallyProcessor;
use crate::generics_processor::GenericsProcessor;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_helper::IfHelper;
use crate::if_pattern_match_processor::IfPatternMatchProcessor;
use crate::infra::kuna_reclaim::ReclaimScope;
use crate::inline_single_block_helper::InlineSingleBlockHelper;
use crate::intersection_cast_processor::IntersectionCastProcessor;
use crate::java_pass_location::JavaPassLocation;
use crate::kuna_fixes::{self, KunaFix};
use crate::label_helper::LabelHelper;
use crate::language_spec::LanguageSpec;
use crate::loop_extract_helper::LoopExtractHelper;
use crate::merge_helper::MergeHelper;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::p2_lift::kuna_fix_overlapping_ranges::OverlappingRanges;
use crate::p8_structure::kuna_javac_layout;
use crate::p_pand_mm_helper::PPandMMHelper;
use crate::pass_context::PassContext;
use crate::plugin_context::PluginContext;
use crate::root_statement::RootStatement;
use crate::secondary_functions_helper::SecondaryFunctionsHelper;
use crate::sequence_helper::SequenceHelper;
use crate::stack_vars_processor::StackVarsProcessor;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::switch_expression_helper::SwitchExpressionHelper;
use crate::switch_helper::SwitchHelper;
use crate::switch_pattern_match_processor::SwitchPatternMatchProcessor;
use crate::synchronized_helper::SynchronizedHelper;
use crate::try_helper::TryHelper;
use crate::var_definition_helper::VarDefinitionHelper;
use crate::var_processor::VarProcessor;
use crate::var_version_node::VarVersionNode;
use crate::var_versions_graph::VarVersionsGraph;

thread_local! {
    static DEBUG_CURRENTLY_DECOMPILING: Cell<Option<RootStatement>> = const { Cell::new(None) };
    static DEBUG_CURRENT_CFG: Cell<Option<ControlFlowGraph>> = const { Cell::new(None) };
    static DEBUG_CURRENT_DECOMPILE_RECORD: Cell<Option<DecompileRecord>> = const { Cell::new(None) };
}

pub struct MethodProcessor;

/// The scratch objects the passes allocate and drop on the floor (Java's GC
/// reclaims them): direct graphs and their nodes, SSA version graphs and their
/// nodes, variable-definition helpers. None of them is held by anything that
/// outlives the pass that built it, except the direct graphs
/// `MethodWrapper::get_or_build_graph` caches, which are kept with their nodes.
///
/// A scope releases every such object allocated since it was opened (or since
/// its last `checkpoint`) when it is dropped -- on return or unwind -- and at
/// each `checkpoint`, which callers place where no pass is in flight, so a long
/// method stays bounded instead of keeping every round's garbage.
pub(crate) struct ScratchScope(ScratchMarks);

#[derive(Clone, Copy)]
struct ScratchMarks {
    direct_graph: u32,
    direct_node: u32,
    versions_graph: u32,
    version_node: u32,
    var_definition_helper: u32,
}

impl ScratchMarks {
    fn now() -> ScratchMarks {
        ScratchMarks {
            direct_graph: DirectGraph::arena_len(),
            direct_node: DirectNode::arena_len(),
            versions_graph: VarVersionsGraph::arena_len(),
            version_node: VarVersionNode::arena_len(),
            var_definition_helper: VarDefinitionHelper::arena_len(),
        }
    }

    fn release(self) {
        let kept_graphs = MethodWrapper::cached_graphs_since(self.direct_graph);
        let node_mark = self.direct_node;
        let mut kept_nodes = vec![false; (DirectNode::arena_len() - node_mark) as usize];
        let mut pending: Vec<DirectNode> = Vec::new();
        for graph in &kept_graphs {
            graph.with(|d| {
                pending.extend(d.nodes.iter().copied());
                pending.extend(d.extra_nodes.iter().copied());
                pending.extend(d.first);
            });
        }
        while let Some(node) = pending.pop() {
            if node.index() < node_mark {
                continue;
            }
            let seen = &mut kept_nodes[(node.index() - node_mark) as usize];
            if !*seen {
                *seen = true;
                pending.extend(node.linked_nodes());
            }
        }
        DirectGraph::release_since(self.direct_graph, |i| {
            kept_graphs.iter().any(|g| g.index() == i)
        });
        DirectNode::release_since(node_mark, |i| kept_nodes[(i - node_mark) as usize]);
        VarVersionsGraph::release_since(self.versions_graph, |_| false);
        VarVersionNode::release_since(self.version_node, |_| false);
        VarDefinitionHelper::release_since(self.var_definition_helper, |_| false);
    }
}

impl ScratchScope {
    pub(crate) fn open() -> ScratchScope {
        ScratchScope(ScratchMarks::now())
    }

    /// Release the scratch objects allocated since the last checkpoint; the
    /// caller guarantees that none of them is still in use.
    pub(crate) fn checkpoint(&mut self) {
        self.0.release();
        self.0 = ScratchMarks::now();
    }
}

impl Drop for ScratchScope {
    fn drop(&mut self) {
        self.0.release();
    }
}

impl MethodProcessor {
    /// `debugCurrentlyDecompiling.get()`.
    pub fn debug_currently_decompiling() -> Option<RootStatement> {
        DEBUG_CURRENTLY_DECOMPILING.with(Cell::get)
    }

    /// `debugCurrentlyDecompiling.set(root)`.
    pub fn set_debug_currently_decompiling(root: Option<RootStatement>) {
        DEBUG_CURRENTLY_DECOMPILING.with(|c| c.set(root));
    }

    /// `debugCurrentCFG.get()`.
    pub fn debug_current_cfg() -> Option<ControlFlowGraph> {
        DEBUG_CURRENT_CFG.with(Cell::get)
    }

    /// `debugCurrentCFG.set(graph)`.
    pub fn set_debug_current_cfg(graph: Option<ControlFlowGraph>) {
        DEBUG_CURRENT_CFG.with(|c| c.set(graph));
    }

    /// `debugCurrentDecompileRecord.get()`.
    pub fn debug_current_decompile_record() -> Option<DecompileRecord> {
        DEBUG_CURRENT_DECOMPILE_RECORD.with(Cell::get)
    }

    /// `debugCurrentDecompileRecord.set(record)`.
    pub fn set_debug_current_decompile_record(record: Option<DecompileRecord>) {
        DEBUG_CURRENT_DECOMPILE_RECORD.with(|c| c.set(record));
    }

    /// The per-method pass schedule: bytecode to a structured statement tree.
    /// Statements, expressions and edges the schedule allocates and drops are
    /// released when it returns or throws (see `kuna_reclaim`).
    pub fn code_to_java(
        cl: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        spec: Option<&LanguageSpec>,
    ) -> RootStatement {
        let mut reclaim = ReclaimScope::open();
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            Self::code_to_java_schedule(cl, mt, md, var_proc, spec, &mut reclaim)
        }));
        match result {
            Ok(root) => {
                reclaim.window().collect(&[root.0], var_proc);
                root
            }
            Err(payload) => {
                reclaim.window().collect(&[], var_proc);
                panic::resume_unwind(payload)
            }
        }
    }

    fn code_to_java_schedule(
        cl: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        spec: Option<&LanguageSpec>,
        reclaim: &mut ReclaimScope,
    ) -> RootStatement {
        let mut scratch = ScratchScope::open();
        CancelationManager::check_canceled();

        Self::set_debug_currently_decompiling(None);
        Self::set_debug_current_cfg(None);
        Self::set_debug_current_decompile_record(None);

        let is_initializer = CLINIT_NAME == mt.get_name();
        let plugin_context = PluginContext::get_current_context();

        mt.expand_data(cl);
        let seq = mt
            .get_instruction_sequence()
            .expect("getInstructionSequence()");
        let graph = ControlFlowGraph::new(seq);
        Self::set_debug_current_cfg(Some(graph));

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

        let decompile_record = DecompileRecord::new(mt);

        if let Some(spec) = spec {
            let mut pctx = PassContext::new(None, Some(graph), mt, cl, var_proc, decompile_record);
            spec.cfg_pass.run(&mut pctx);

            let root = spec.graph_parser.create_statement(graph, mt);

            pctx.set_root(Some(root));
            spec.pass.run(&mut pctx);

            return Self::pass_root(&pctx);
        }

        let mut pctx = PassContext::new(None, Some(graph), mt, cl, var_proc, decompile_record);

        plugin_context.run_passes(JavaPassLocation::CfgConstruction, &mut pctx);

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

        if kuna_fixes::enabled(KunaFix::OverlappingRanges) {
            OverlappingRanges::split(graph);
        }

        if ExceptionDeobfuscator::has_obfuscated_exceptions(graph) {
            if !ExceptionDeobfuscator::handle_multiple_entry_exception_ranges(graph) {
                DecompilerContext::get_logger().write_message(
                    "Found multiple entry exception ranges which could not be splitted",
                    Severity::Warn,
                );
                graph.add_comment("$VF: Could not handle exception ranges with multiple entries");
                graph.with_mut(|d| d.add_error_comment = true);
            }

            ExceptionDeobfuscator::insert_dummy_exception_handler_blocks(
                graph,
                mt.get_bytecode_version(),
            );
        }

        plugin_context.run_passes(JavaPassLocation::CfgPreStatement, &mut pctx);

        let _javac_layout = kuna_javac_layout::enter(mt, cl);
        let mut root = DomHelper::parse_graph(graph, mt, 0);

        Self::set_debug_current_decompile_record(Some(decompile_record));

        decompile_record.add("Initial", Some(root));

        Self::set_debug_currently_decompiling(Some(root));

        let mut f_proc = FinallyProcessor::new(mt, md, var_proc);
        let mut finally_processed = 0;

        while f_proc.iterate_graph(cl, mt, root, graph) {
            CancelationManager::check_canceled();
            finally_processed += 1;
            scratch.checkpoint();
            let old_root = root;
            decompile_record.add(
                &format!("ProcessFinallyOld_{finally_processed}"),
                Some(root),
            );

            root = DomHelper::parse_graph(graph, mt, finally_processed);
            root.add_comments(old_root);

            decompile_record.add(&format!("ProcessFinally_{finally_processed}"), Some(root));
            reclaim.checkpoint(&[root.0], var_proc);

            Self::set_debug_current_cfg(Some(graph));
            Self::set_debug_currently_decompiling(Some(root));
        }

        if DomHelper::build_synchronized(root.0) {
            decompile_record.add("BuildFinallySynchronized", Some(root));
        }

        if finally_processed > 0 {
            decompile_record.add("ProcessFinally_Post", Some(root));
        }

        if DomHelper::remove_synchronized_handler(root.0) {
            decompile_record.add("RemoveSynchronizedHandler", Some(root));
        }

        root.build_content_flags();

        SequenceHelper::condense_sequences(root.0);
        decompile_record.add("CondenseSequences", Some(root));

        ClearStructHelper::clear_statements(root);
        decompile_record.add("ClearStatements", Some(root));

        let proc = ExprProcessor::new(md, var_proc);
        proc.process_statement(root, cl);
        decompile_record.add("ProcessStatement", Some(root));

        SequenceHelper::condense_sequences(root.0);
        decompile_record.add("CondenseSequences_1", Some(root));

        let mut stack_vars_processed = 0;
        loop {
            CancelationManager::check_canceled();
            stack_vars_processed += 1;
            scratch.checkpoint();
            reclaim.checkpoint(&[root.0], var_proc);

            StackVarsProcessor::simplify_stack_vars(root, mt, cl);
            decompile_record.add(
                &format!("SimplifyStackVars_PPMM_{stack_vars_processed}"),
                Some(root),
            );

            var_proc.set_var_versions(root);
            decompile_record.add(
                &format!("SetVarVersions_PPMM_{stack_vars_processed}"),
                Some(root),
            );

            if !PPandMMHelper::new(var_proc).find_p_pand_mm(root) {
                break;
            }
        }

        pctx = PassContext::new(Some(root), Some(graph), mt, cl, var_proc, decompile_record);

        if PPandMMHelper::inline_pp_iand_mmi_if(root) {
            decompile_record.add("InlinePPIandMMI", Some(root));
        }

        if cl.get_version().has_indy_string_concat() {
            ConcatenationHelper::simplify_string_concat(root.0);
            decompile_record.add("SimplifyStringConcat", Some(root));
        }

        if AssertProcessor::build_assertions(root) {
            decompile_record.add("BuildAsserts", Some(root));
        }

        plugin_context.run_passes(JavaPassLocation::BeforeMain, &mut pctx);
        root = Self::pass_root(&pctx);
        Self::set_debug_currently_decompiling(Some(root));

        kuna_javac_layout::loops_built();
        if kuna_javac_layout::active() && kuna_javac_layout::place_tails(root.0) {
            SequenceHelper::condense_sequences(root.0);
            decompile_record.add("PlaceTails", Some(root));
        }
        loop {
            CancelationManager::check_canceled();
            decompile_record.increment_main_loop();
            scratch.checkpoint();
            reclaim.checkpoint(&[root.0], var_proc);
            decompile_record.add("Start", Some(root));

            LabelHelper::clean_up_edges(root);
            decompile_record.add("CleanupEdges", Some(root));

            if root.has_loops() {
                loop {
                    CancelationManager::check_canceled();
                    decompile_record.increment_merge_loop();
                    scratch.checkpoint();
                    reclaim.checkpoint(&[root.0], var_proc);
                    decompile_record.add("MergeLoopStart", Some(root));

                    if EliminateLoopsHelper::eliminate_loops(root.0, cl) {
                        decompile_record.add("EliminateLoops", Some(root));
                        continue;
                    }

                    if MergeHelper::enhance_loops(root.0) {
                        decompile_record.add("EnhanceLoops", Some(root));
                    }

                    if LoopExtractHelper::extract_loops(root.0) {
                        decompile_record.add("ExtractLoops", Some(root));
                        continue;
                    }

                    if plugin_context.run_passes(JavaPassLocation::InLoopDecomp, &mut pctx) {
                        root = Self::pass_root(&pctx);
                        Self::set_debug_currently_decompiling(Some(root));
                        continue;
                    }
                    root = Self::pass_root(&pctx);
                    Self::set_debug_currently_decompiling(Some(root));

                    if IfHelper::merge_all_ifs(root) {
                        decompile_record.add("MergeAllIfs", Some(root));
                    } else {
                        break;
                    }
                }

                decompile_record.reset_merge_loop();
                decompile_record.add("MergeLoopEnd", Some(root));
            }

            StackVarsProcessor::simplify_stack_vars(root, mt, cl);
            decompile_record.add("SimplifyStackVars", Some(root));

            var_proc.set_var_versions(root);
            decompile_record.add("SetVarVersions", Some(root));

            LabelHelper::identify_labels(root);
            decompile_record.add("IdentifyLabels", Some(root));

            if SecondaryFunctionsHelper::identify_secondary_functions(root.0, var_proc) {
                decompile_record.add("IdentifySecondary", Some(root));
                continue;
            }

            if IntersectionCastProcessor::make_intersection_casts(root) {
                decompile_record.add("intersectionCasts", Some(root));
                continue;
            }

            if DecompilerContext::get_option(IFernflowerPreferences::PATTERN_MATCHING)
                && cl.get_version().has_if_pattern_matching()
                && IfPatternMatchProcessor::match_instanceof(root)
            {
                decompile_record.add("MatchIfPatterns", Some(root));
                continue;
            }

            if root.has_switch() {
                let mut changed = false;
                if SwitchPatternMatchProcessor::has_pattern_match(root)
                    && SwitchPatternMatchProcessor::process_pattern_matching(root.0)
                {
                    decompile_record.add("ProcessSwitchPatternMatch", Some(root));
                    changed = true;
                }

                if SwitchExpressionHelper::has_switch_expressions(root)
                    && SwitchExpressionHelper::process_switch_expressions(root.0)
                {
                    decompile_record.add("ProcessSwitchExpr", Some(root));
                    changed = true;
                }

                if changed {
                    continue;
                }
            }

            if root.has_try_catch() && TryHelper::enhance_try_stats(root, cl) {
                decompile_record.add("EnhanceTry", Some(root));
                continue;
            }

            if InlineSingleBlockHelper::inline_single_blocks(root) {
                decompile_record.add("InlineSingleBlocks", Some(root));
                continue;
            }

            if root.has_loops() && MergeHelper::make_do_while_loops(root) {
                decompile_record.add("MatchDoWhile", Some(root));
                continue;
            }

            if root.has_loops() && MergeHelper::condense_infinite_loops_with_return(root.0) {
                decompile_record.add("CondenseDo", Some(root));
                continue;
            }

            if plugin_context.run_passes(JavaPassLocation::MainLoop, &mut pctx) {
                root = Self::pass_root(&pctx);
                Self::set_debug_currently_decompiling(Some(root));
                continue;
            }
            root = Self::pass_root(&pctx);
            Self::set_debug_currently_decompiling(Some(root));

            if !is_initializer && ExitHelper::condense_exits(root) {
                decompile_record.add("CondenseExits", Some(root));
                continue;
            }

            break;
        }
        decompile_record.reset_main_loop();
        decompile_record.add("MainLoopEnd", Some(root));

        if root.has_switch() && SwitchHelper::simplify_switches(root.0, mt, root) {
            SequenceHelper::condense_sequences(root.0);
            decompile_record.add("SimplifySwitches", Some(root));

            if SwitchExpressionHelper::has_switch_expressions(root)
                && SwitchExpressionHelper::process_switch_expressions(root.0)
            {
                decompile_record.add("ProcessSwitchExpr_SS", Some(root));

                StackVarsProcessor::simplify_stack_vars(root, mt, cl);
                decompile_record.add("SimplifyStackVars_SS", Some(root));

                var_proc.set_var_versions(root);
                decompile_record.add("SetVarVersions_SS", Some(root));
            }
        }

        if ExitHelper::adjust_return_type(root, md) {
            decompile_record.add("AdjustReturnType", Some(root));
        }

        if ExitHelper::remove_redundant_returns(root) {
            decompile_record.add("RedundantReturns", Some(root));
        }

        if SecondaryFunctionsHelper::identify_secondary_functions(root.0, var_proc) {
            decompile_record.add("IdentifySecondary", Some(root));
        }

        if SynchronizedHelper::clean_synchronized_var(root.0) {
            decompile_record.add("ClearSynchronized", Some(root));
        }

        if SynchronizedHelper::insert_sink(root, var_proc, root.0) {
            decompile_record.add("InsertSynchronizedAssignments", Some(root));
        }

        plugin_context.run_passes(JavaPassLocation::AfterMain, &mut pctx);
        root = Self::pass_root(&pctx);
        Self::set_debug_currently_decompiling(Some(root));

        var_proc.set_var_definitions(root);
        decompile_record.add("SetVarDefinitions", Some(root));

        if SecondaryFunctionsHelper::update_assignments(root.0) {
            decompile_record.add("UpdateAssignments", Some(root));
        }

        if root.has_switch() && LabelHelper::hide_default_switch_edges(root.0) {
            decompile_record.add("HideEmptyDefault", Some(root));
        }

        if GenericsProcessor::qualify_chains(root.0) {
            decompile_record.add("QualifyGenericChains", Some(root));
        }

        if ExprProcessor::canonicalize_casts(root) {
            decompile_record.add("CanonicalizeCasts", Some(root));
        }

        plugin_context.run_passes(JavaPassLocation::AtEnd, &mut pctx);
        root = Self::pass_root(&pctx);
        Self::set_debug_currently_decompiling(Some(root));

        if LabelHelper::replace_continue_with_break(root.0) {
            decompile_record.add("ReplaceContinues", Some(root));
        }

        ExprProcessor::mark_expr_oddities(root);

        mt.release_resources();

        root
    }

    /// `pctx.getRoot()` after a plugin location; a pass never clears it.
    fn pass_root(pctx: &PassContext) -> RootStatement {
        pctx.get_root().expect("PassContext.getRoot()")
    }
}
