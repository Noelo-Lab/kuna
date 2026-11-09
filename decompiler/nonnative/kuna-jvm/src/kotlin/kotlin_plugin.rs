//! Port of `org/vineflower/kotlin/KotlinPlugin.java`.

use std::rc::Rc;

use crate::assert_processor::AssertProcessor;
use crate::class_pass::ClassPass;
use crate::classes_processor::ClassNode;
use crate::clear_struct_helper::ClearStructHelper;
use crate::collapse_string_concat_pass::CollapseStringConcatPass;
use crate::control_flow_graph::ControlFlowGraph;
use crate::dead_code_helper::DeadCodeHelper;
use crate::dom_helper::DomHelper;
use crate::eliminate_dead_vars_pass::EliminateDeadVarsPass;
use crate::eliminate_loops_helper::EliminateLoopsHelper;
use crate::exception_deobfuscator::ExceptionDeobfuscator;
use crate::exit_helper::ExitHelper;
use crate::expr_processor::ExprProcessor;
use crate::if_helper::IfHelper;
use crate::inline_single_block_helper::InlineSingleBlockHelper;
use crate::java_finally_pass::JavaFinallyPass;
use crate::k_merge_pass::KMergePass;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::label_helper::LabelHelper;
use crate::language_spec::LanguageSpec;
use crate::loop_extract_helper::LoopExtractHelper;
use crate::looping_pass_builder::LoopingPassBuilder;
use crate::main_pass_builder::MainPassBuilder;
use crate::merge_helper::MergeHelper;
use crate::nested_class_processor::NestedClassProcessor;
use crate::nested_member_access::NestedMemberAccess;
use crate::obfuscated_exceptions_pass::ObfuscatedExceptionsPass;
use crate::p_pand_mm_helper::PPandMMHelper;
use crate::pair::Pair;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::plugin::Plugin;
use crate::plugin_options::{DefaultsConsumer, PluginOptions};
use crate::replace_exprents_pass::ReplaceExprentsPass;
use crate::replace_stats_pass::ReplaceStatsPass;
use crate::resugar_kotlin_methods_pass::ResugarKotlinMethodsPass;
use crate::root_statement::RootStatement;
use crate::secondary_functions_helper::{IdentifySecondaryOptions, SecondaryFunctionsHelper};
use crate::sequence_helper::SequenceHelper;
use crate::stack_var_initial_pass::StackVarInitialPass;
use crate::stack_vars_processor::{StackSimplifyOptions, StackVarsProcessor};
use crate::wrapped_pass::WrappedPass;

/// `ctx.getRoot()`, dereferenced.
fn root(ctx: &PassContext) -> RootStatement {
    ctx.get_root().expect("NullPointerException: ctx.getRoot()")
}

/// `ctx.getGraph()`, dereferenced.
fn graph(ctx: &PassContext) -> ControlFlowGraph {
    ctx.get_graph()
        .expect("NullPointerException: ctx.getGraph()")
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KotlinPlugin;

impl KotlinPlugin {
    pub fn new() -> KotlinPlugin {
        KotlinPlugin
    }

    #[allow(non_snake_case)]
    fn INLINE_ALL_VARS() -> StackSimplifyOptions {
        StackSimplifyOptions::new().inline_regular_vars()
    }

    #[allow(non_snake_case)]
    fn FORCE_TERNARY_SIMPLIFY() -> IdentifySecondaryOptions {
        IdentifySecondaryOptions::new().force_ternary_simplification()
    }

    fn get_root_processor() -> Rc<dyn ClassPass> {
        Rc::new(|root: ClassNode| {
            NestedClassProcessor::new().process_class(root, root);

            NestedMemberAccess::new().propagate_member_access(root);

            true
        })
    }

    fn make_cfg_pass() -> Rc<dyn Pass> {
        MainPassBuilder::new()
            .add_pass(
                "RemoveCircular",
                WrappedPass::of(|ctx| ExceptionDeobfuscator::remove_circular_ranges(graph(ctx))),
            )
            .add_pass(
                "RestorePop",
                WrappedPass::of(|ctx| ExceptionDeobfuscator::restore_pop_ranges(graph(ctx))),
            )
            .add_pass(
                "RemoveEmpty",
                WrappedPass::of(|ctx| ExceptionDeobfuscator::remove_empty_ranges(graph(ctx))),
            )
            .add_pass(
                "ExtendMonitors",
                WrappedPass::of(|ctx| {
                    DeadCodeHelper::extend_synchronized_range_to_monitorexit(graph(ctx))
                }),
            )
            .add_pass(
                "ValueReturns",
                WrappedPass::of(|ctx| DeadCodeHelper::incorporate_value_returns(graph(ctx))),
            )
            .add_pass(
                "InsertEmpty",
                WrappedPass::of(|ctx| {
                    ExceptionDeobfuscator::insert_empty_exception_handler_blocks(graph(ctx))
                }),
            )
            .add_pass(
                "MergeBlocks",
                WrappedPass::of(|ctx| DeadCodeHelper::merge_basic_blocks(graph(ctx))),
            )
            .add_pass("ObfExceptions", Rc::new(ObfuscatedExceptionsPass))
            .build()
    }

    fn make_pass() -> Rc<dyn Pass> {
        MainPassBuilder::new()
            .add_pass("Finally", Rc::new(JavaFinallyPass))
            .add_pass(
                "BuildFinallySynchronized",
                Rc::new(|ctx: &mut PassContext| DomHelper::build_synchronized(root(ctx).0)),
            )
            .add_pass(
                "RemoveSynchronized",
                Rc::new(|ctx: &mut PassContext| {
                    DomHelper::remove_synchronized_handler(root(ctx).0)
                }),
            )
            .add_pass(
                "CondenseSequences",
                WrappedPass::of(|ctx| SequenceHelper::condense_sequences(root(ctx).0)),
            )
            .add_pass(
                "ClearStatements",
                WrappedPass::of(|ctx| ClearStructHelper::clear_statements(root(ctx))),
            )
            .add_pass(
                "ProcessExpr",
                WrappedPass::of(|ctx| {
                    ExprProcessor::new(&ctx.get_method_descriptor(), ctx.get_var_proc())
                        .process_statement(root(ctx), ctx.get_enclosing_class())
                }),
            )
            .add_pass(
                "CondenseSequences_1",
                WrappedPass::of(|ctx| SequenceHelper::condense_sequences(root(ctx).0)),
            )
            .add_pass("StackVars", Rc::new(StackVarInitialPass))
            .add_pass(
                "InlineIfPPMM",
                Rc::new(|ctx: &mut PassContext| PPandMMHelper::inline_pp_iand_mmi_if(root(ctx))),
            )
            .add_pass(
                "BuildAsserts",
                WrappedPass::of(|ctx| {
                    AssertProcessor::build_assertions(root(ctx));
                }),
            )
            .add_pass(
                "MainLoop",
                LoopingPassBuilder::new("Main")
                    .add_fallthrough_pass(
                        "ResetEdges",
                        WrappedPass::of(|ctx| LabelHelper::clean_up_edges(root(ctx))),
                    )
                    .add_fallthrough_pass(
                        "MergeLoop",
                        LoopingPassBuilder::new("Merge")
                            .add_looping_pass(
                                "EliminateLoops",
                                Rc::new(|ctx: &mut PassContext| {
                                    EliminateLoopsHelper::eliminate_loops(
                                        root(ctx).0,
                                        ctx.get_enclosing_class(),
                                    )
                                }),
                            )
                            .add_fallthrough_pass("EnhanceLoops", Rc::new(KMergePass))
                            .add_looping_pass(
                                "ExtractLoops",
                                Rc::new(|ctx: &mut PassContext| {
                                    LoopExtractHelper::extract_loops(root(ctx).0)
                                }),
                            )
                            .add_looping_pass(
                                "MergeAllIfs",
                                Rc::new(|ctx: &mut PassContext| IfHelper::merge_all_ifs(root(ctx))),
                            )
                            .build(),
                    )
                    .add_fallthrough_pass(
                        "SimplifyStack",
                        WrappedPass::of(|ctx| {
                            StackVarsProcessor::simplify_stack_vars(
                                root(ctx),
                                ctx.get_method(),
                                ctx.get_enclosing_class(),
                            )
                        }),
                    )
                    .add_fallthrough_pass(
                        "VarVersions",
                        WrappedPass::of(|ctx| ctx.get_var_proc().set_var_versions(root(ctx))),
                    )
                    .add_fallthrough_pass(
                        "IdentifyLabels",
                        WrappedPass::of(|ctx| LabelHelper::identify_labels(root(ctx))),
                    )
                    .add_looping_pass(
                        "IdentifySecondary",
                        Rc::new(|ctx: &mut PassContext| {
                            SecondaryFunctionsHelper::identify_secondary_functions_options(
                                root(ctx).0,
                                ctx.get_var_proc(),
                                KotlinPlugin::FORCE_TERNARY_SIMPLIFY(),
                            )
                        }),
                    )
                    .add_fallthrough_pass("EliminateDead", Rc::new(EliminateDeadVarsPass))
                    .add_looping_pass(
                        "InlineSingleBlocks",
                        Rc::new(|ctx: &mut PassContext| {
                            InlineSingleBlockHelper::inline_single_blocks(root(ctx))
                        }),
                    )
                    .add_looping_pass(
                        "MakeDoWhile",
                        Rc::new(|ctx: &mut PassContext| {
                            MergeHelper::make_do_while_loops(root(ctx))
                        }),
                    )
                    .add_looping_pass(
                        "CondenseDo",
                        Rc::new(|ctx: &mut PassContext| {
                            MergeHelper::condense_infinite_loops_with_return(root(ctx).0)
                        }),
                    )
                    .add_looping_pass(
                        "CondenseExits",
                        Rc::new(|ctx: &mut PassContext| ExitHelper::condense_exits(root(ctx))),
                    )
                    .build(),
            )
            .add_pass(
                "SimplifyStack",
                WrappedPass::of(|ctx| {
                    StackVarsProcessor::simplify_stack_vars_options(
                        root(ctx),
                        ctx.get_method(),
                        ctx.get_enclosing_class(),
                        KotlinPlugin::INLINE_ALL_VARS(),
                    )
                }),
            )
            .add_pass(
                "AdjustReturnType",
                Rc::new(|ctx: &mut PassContext| {
                    ExitHelper::adjust_return_type(root(ctx), &ctx.get_method_descriptor())
                }),
            )
            .add_pass(
                "RedundantReturns",
                Rc::new(|ctx: &mut PassContext| ExitHelper::remove_redundant_returns(root(ctx))),
            )
            .add_pass(
                "IdentifySecondary",
                Rc::new(|ctx: &mut PassContext| {
                    SecondaryFunctionsHelper::identify_secondary_functions_options(
                        root(ctx).0,
                        ctx.get_var_proc(),
                        KotlinPlugin::FORCE_TERNARY_SIMPLIFY(),
                    )
                }),
            )
            .add_pass(
                "SetVarDefinitions",
                WrappedPass::of(|ctx| ctx.get_var_proc().set_var_definitions(root(ctx))),
            )
            .add_pass("ReplaceStats", Rc::new(ReplaceStatsPass))
            .add_pass("ReplaceExprs", Rc::new(ReplaceExprentsPass))
            .add_pass("ResugarMethods", Rc::new(ResugarKotlinMethodsPass))
            .add_pass("CollapseStringConcat", Rc::new(CollapseStringConcatPass))
            .add_pass(
                "ReplaceContinue",
                Rc::new(|ctx: &mut PassContext| {
                    LabelHelper::replace_continue_with_break(root(ctx).0)
                }),
            )
            .build()
    }
}

impl Plugin for KotlinPlugin {
    fn id(&self) -> String {
        "Kotlin".to_string()
    }

    fn description(&self) -> String {
        "Detects and decompiles Kotlin class files.".to_string()
    }

    /// Not in Vineflower: the member attributes live in `kt_attrs`, which has
    /// session scope, rather than on the members themselves.
    fn close(&self) {
        KtAttrs::clear();
    }

    fn get_plugin_options(&self) -> Option<Rc<dyn PluginOptions>> {
        Some(Rc::new(|| {
            let defaults: DefaultsConsumer = Rc::new(KotlinOptions::add_defaults);
            Pair::of(KotlinOptions::OPTIONS, defaults)
        }))
    }

    fn get_language_spec(&self) -> Option<LanguageSpec> {
        Some(LanguageSpec::new(
            "kotlin",
            Rc::new(KotlinChooser::new()),
            Rc::new(DomHelper),
            Rc::new(KotlinWriter::new()),
            KotlinPlugin::make_pass(),
            KotlinPlugin::make_cfg_pass(),
            KotlinPlugin::get_root_processor(),
            "kt",
        ))
    }
}
