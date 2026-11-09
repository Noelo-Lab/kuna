//! Port of `org/jetbrains/java/decompiler/main/rels/MethodProcessor.java`.
//!
//! Only the per-method schedule (`codeToJava`) is ported. The Java class also
//! runs that schedule on a watchdog thread (`run`, `lock`, `getResult`,
//! `isFinished`) to enforce `MAX_PROCESSING_METHOD`; kuna calls
//! [`MethodProcessor::code_to_java`] directly, as Vineflower itself does when
//! the limit is 0 or in unit-test mode.

use std::cell::Cell;

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
use crate::inline_single_block_helper::InlineSingleBlockHelper;
use crate::intersection_cast_processor::IntersectionCastProcessor;
use crate::java_pass_location::JavaPassLocation;
use crate::label_helper::LabelHelper;
use crate::language_spec::LanguageSpec;
use crate::loop_extract_helper::LoopExtractHelper;
use crate::merge_helper::MergeHelper;
use crate::method_descriptor::MethodDescriptor;
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
use crate::var_processor::VarProcessor;

thread_local! {
    static DEBUG_CURRENTLY_DECOMPILING: Cell<Option<RootStatement>> = const { Cell::new(None) };
    static DEBUG_CURRENT_CFG: Cell<Option<ControlFlowGraph>> = const { Cell::new(None) };
    static DEBUG_CURRENT_DECOMPILE_RECORD: Cell<Option<DecompileRecord>> = const { Cell::new(None) };
}

pub struct MethodProcessor;

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
    pub fn code_to_java(
        cl: StructClass,
        mt: StructMethod,
        md: &MethodDescriptor,
        var_proc: VarProcessor,
        spec: Option<&LanguageSpec>,
    ) -> RootStatement {
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

        let mut root = DomHelper::parse_graph(graph, mt, 0);

        Self::set_debug_current_decompile_record(Some(decompile_record));

        decompile_record.add("Initial", Some(root));

        Self::set_debug_currently_decompiling(Some(root));

        let mut f_proc = FinallyProcessor::new(mt, md, var_proc);
        let mut finally_processed = 0;

        while f_proc.iterate_graph(cl, mt, root, graph) {
            CancelationManager::check_canceled();
            finally_processed += 1;
            let old_root = root;
            decompile_record.add(
                &format!("ProcessFinallyOld_{finally_processed}"),
                Some(root),
            );

            root = DomHelper::parse_graph(graph, mt, finally_processed);
            root.add_comments(old_root);

            decompile_record.add(&format!("ProcessFinally_{finally_processed}"), Some(root));

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

        loop {
            CancelationManager::check_canceled();
            decompile_record.increment_main_loop();
            decompile_record.add("Start", Some(root));

            LabelHelper::clean_up_edges(root);
            decompile_record.add("CleanupEdges", Some(root));

            if root.has_loops() {
                loop {
                    CancelationManager::check_canceled();
                    decompile_record.increment_merge_loop();
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
