//! Port of `org/vineflower/kotlin/KotlinPlugin.java`.
//! kuna fix `split_constructor_call`: the main loop's last step joins split
//! constructor calls (`JoinSplitConstructors`).
//!
//! kuna fix `kt_not_null_assertion`: the last `SimplifyStack` first folds
//! kotlinc's `!!` checks back into their expressions.
//! kuna fix `kt_expression_value_check`: it also deletes kotlinc's checks of
//! platform-typed values first, so their stack variables are inlined.
//! kuna fix `kt_object_instance_local`: and in an object's static
//! initializer, replaces the Kotlin 1.3 instance local by `INSTANCE`.
//! kuna fix `kt_enum_when_mappings`: after the main loop, where the Java
//! pipeline simplifies switches, a `when` over a `$WhenMappings` array is
//! written over the enum (`WhenMappings`).
//! kuna fix `kt_inlined_to_typed_array`: the last `SimplifyStack` first
//! writes kotlinc's inlined `toTypedArray()` as the call, and after it leaves
//! out the casts of its result (`kuna_fix_kt_inlined_to_typed_array`); the
//! call's type argument is decided once the exprents are replaced
//! (`RawGenericTypes`).
//! kuna fix `kt_inlined_stdlib_call`: the last `SimplifyStack` first writes
//! kotlinc's inlined copies of standard-library functions that take no
//! lambda as their calls (`kuna_fix_kt_inlined_stdlib_call`).
//! kuna fix `kt_narrow_constant_local`: after it, a local of constants that
//! Vineflower types `Byte` or `Short` is typed `Int` where its uses take one
//! (`kuna_fix_kt_narrow_constant_local`).
//! kuna fix `kt_suspend_state_machine`: before the graph passes, a suspend
//! function's state machine is rebuilt into its direct body
//! (`SuspendStateMachine`).
//! kuna fix `kt_suspend_call_continuation`: once the Kotlin expressions are
//! made, the suspend calls that pass the function's own continuation are
//! marked to be written without it, and the casts of their results to
//! their declared types left out (`SuspendCalls`).
//! kuna fix `kt_coroutine_boxing`: then kotlinc's coroutine boxing calls
//! become the `valueOf` boxing the writer leaves implicit (`CoroutineBoxing`).

use std::rc::Rc;

use crate::assert_processor::AssertProcessor;
use crate::class_pass::ClassPass;
use crate::classes_processor::ClassNode;
use crate::clear_struct_helper::ClearStructHelper;
use crate::code_constants::CLINIT_NAME;
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
use crate::kotlin::{
    kuna_fix_kt_expression_value_check, kuna_fix_kt_not_null_assertion,
    kuna_fix_kt_object_instance_local,
};
use crate::kotlin::kuna_fix_kt_enum_when_mappings;
use crate::kotlin::kuna_fix_kt_inlined_to_typed_array;
use crate::kotlin::kuna_fix_kt_inlined_stdlib_call;
use crate::kotlin::kuna_fix_kt_narrow_constant_local;
use crate::kotlin::kuna_fix_kt_nullable_local;
use crate::kotlin::kuna_fix_kt_raw_generic_type;
use crate::kotlin::kuna_fix_kt_coroutine_boxing;
use crate::kotlin::kuna_fix_kt_suspend_call_continuation;
use crate::kotlin::kuna_fix_kt_suspend_state_machine;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_options::KotlinOptions;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::label_helper::LabelHelper;
use crate::language_spec::LanguageSpec;
use crate::loop_extract_helper::LoopExtractHelper;
use crate::looping_pass_builder::LoopingPassBuilder;
use crate::main_pass_builder::MainPassBuilder;
use crate::merge_helper::MergeHelper;
use crate::nested_class_processor::NestedClassProcessor;
use crate::nested_member_access::NestedMemberAccess;
use crate::obfuscated_exceptions_pass::ObfuscatedExceptionsPass;
use crate::p3_dataflow::kuna_fix_split_constructor_call::join_split_constructors;
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
                "SuspendStateMachine",
                Rc::new(|ctx: &mut PassContext| {
                    kuna_fixes::enabled(KunaFix::KtSuspendStateMachine)
                        && kuna_fix_kt_suspend_state_machine::rebuild(
                            graph(ctx),
                            ctx.get_method(),
                            ctx.get_enclosing_class(),
                        )
                }),
            )
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
                    .add_looping_pass(
                        "JoinSplitConstructors",
                        Rc::new(|ctx: &mut PassContext| {
                            kuna_fixes::enabled(KunaFix::SplitConstructorCall)
                                && join_split_constructors(root(ctx))
                        }),
                    )
                    .build(),
            )
            .add_pass(
                "WhenMappings",
                Rc::new(|ctx: &mut PassContext| {
                    kuna_fixes::enabled(KunaFix::KtEnumWhenMappings)
                        && kuna_fix_kt_enum_when_mappings::resugar(root(ctx).0)
                }),
            )
            .add_pass(
                "SimplifyStack",
                WrappedPass::of(|ctx| {
                    if kuna_fixes::enabled(KunaFix::KtInlinedToTypedArray) {
                        kuna_fix_kt_inlined_to_typed_array::resugar(root(ctx));
                    }
                    if kuna_fixes::enabled(KunaFix::KtInlinedStdlibCall) {
                        kuna_fix_kt_inlined_stdlib_call::resugar(
                            root(ctx),
                            ctx.get_enclosing_class(),
                            ctx.get_method(),
                        );
                    }
                    if kuna_fixes::enabled(KunaFix::KtNotNullAssertion) {
                        kuna_fix_kt_not_null_assertion::recover(root(ctx));
                    }
                    if kuna_fixes::enabled(KunaFix::KtExpressionValueCheck) {
                        kuna_fix_kt_expression_value_check::remove(root(ctx));
                    }
                    if kuna_fixes::enabled(KunaFix::KtObjectInstanceLocal)
                        && ctx.get_method().get_name() == CLINIT_NAME
                    {
                        kuna_fix_kt_object_instance_local::rewrite(
                            root(ctx),
                            ctx.get_enclosing_class(),
                        );
                    }
                    StackVarsProcessor::simplify_stack_vars_options(
                        root(ctx),
                        ctx.get_method(),
                        ctx.get_enclosing_class(),
                        KotlinPlugin::INLINE_ALL_VARS(),
                    );
                    if kuna_fixes::enabled(KunaFix::KtInlinedToTypedArray) {
                        kuna_fix_kt_inlined_to_typed_array::retype(root(ctx), ctx.get_var_proc());
                    }
                    if kuna_fixes::enabled(KunaFix::KtNarrowConstantLocal) {
                        kuna_fix_kt_narrow_constant_local::retype(
                            root(ctx),
                            ctx.get_method(),
                            &ctx.get_method_descriptor(),
                            ctx.get_var_proc(),
                        );
                    }
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
            .add_pass(
                "SuspendCalls",
                Rc::new(|ctx: &mut PassContext| {
                    kuna_fixes::enabled(KunaFix::KtSuspendCallContinuation)
                        && kuna_fix_kt_suspend_call_continuation::mark(root(ctx), ctx.get_method())
                }),
            )
            .add_pass("ResugarMethods", Rc::new(ResugarKotlinMethodsPass))
            .add_pass("CollapseStringConcat", Rc::new(CollapseStringConcatPass))
            .add_pass(
                "RawGenericTypes",
                WrappedPass::of(|ctx| {
                    if kuna_fixes::enabled(KunaFix::KtRawGenericType) {
                        kuna_fix_kt_raw_generic_type::infer(
                            root(ctx),
                            ctx.get_method(),
                            ctx.get_var_proc(),
                        );
                    }
                    if kuna_fixes::enabled(KunaFix::KtInlinedToTypedArray) {
                        kuna_fix_kt_inlined_to_typed_array::type_arguments(root(ctx));
                    }
                }),
            )
            .add_pass(
                "NullableLocals",
                WrappedPass::of(|ctx| {
                    if kuna_fixes::enabled(KunaFix::KtNullableLocal)
                        || kuna_fixes::enabled(KunaFix::KtNullableCast)
                    {
                        kuna_fix_kt_nullable_local::infer(root(ctx), ctx.get_method());
                    }
                }),
            )
            .add_pass(
                "CoroutineBoxing",
                Rc::new(|ctx: &mut PassContext| {
                    kuna_fixes::enabled(KunaFix::KtCoroutineBoxing)
                        && kuna_fix_kt_coroutine_boxing::rewrite(root(ctx), ctx.get_method())
                }),
            )
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
