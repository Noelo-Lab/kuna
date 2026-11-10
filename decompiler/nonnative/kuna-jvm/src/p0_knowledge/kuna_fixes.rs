//! kuna-only fixes of Vineflower defects, and the switch that turns them all off.
//!
//! The port prints what Vineflower 1.12.0 prints, failures included. A fix is
//! a deliberate departure from that: each is a [`KunaFix`], always on, named
//! by an internal id ([`KunaFix::id`]) rather than an option, and every site
//! where kuna behaves differently from the Java asks [`enabled`] and keeps the
//! Java's behaviour verbatim in the other branch. The one option is
//! `jvm_vineflower_compat` (off by default): on, it disables every fix, which
//! gives Vineflower's exact text; the Vineflower-oracle corpus runs that way.
//!
//! `Fernflower::new` takes the switch off its property list (so the option
//! map stays Vineflower's), stores the resolved set in the decompilation
//! context, and installing a context makes its set the one [`enabled`]
//! answers from on this thread.

use std::cell::Cell;

/// The option that disables every kuna-only fix.
pub const VINEFLOWER_COMPAT: &str = "jvm_vineflower_compat";

/// One kuna-only fix; its id is [`KunaFix::id`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KunaFix {
    /// `report_to_kuna`: the failure comment asks for reports to kuna's tracker.
    ReportToKuna,
    /// `lambda_wildcard_return`: a method reference on a receiver
    /// whose type argument is `?` no longer fails type inference.
    LambdaWildcardReturn,
    /// `twr_unary_condition`: a `catch (Throwable)` handler that starts
    /// with `if (!flag)` is not taken for a try-with-resources null check.
    TwrUnaryCondition,
    /// `if_denest_fallthrough`: if-else-chain denesting no longer
    /// moves the code after an if into its then-branch while the if's body
    /// still reaches that code.
    IfDenestFallthrough,
    /// `switch_yield_const_retype`: a constant a switch expression
    /// yields keeps its own type when the switch's type is one no constant has.
    SwitchYieldConstRetype,
    /// `catch_loop_back_successor`: a try/catch is not built at a
    /// level where its successor would be its own try block.
    CatchLoopBackSuccessor,
    /// `kt_synthetic_anonymous_object`: an anonymous class with only
    /// synthetic-class Kotlin metadata is written as an `object` whose
    /// supertypes come from the class file.
    KtSyntheticAnonymousObject,
    /// `kt_local_as_extension_receiver`: a `$this$` local of a
    /// method without parameters is not taken for an extension receiver.
    KtLocalAsExtensionReceiver,
    /// `kt_default_args_super_call`: a Kotlin primary constructor's
    /// supertype call through a default-arguments constructor names the
    /// constructor it stands for.
    KtDefaultArgsSuperCall,
    /// `kt_default_arg_compound_branch`: a default value computed by
    /// more than one statement is left unrecovered instead of failing the class.
    KtDefaultArgCompoundBranch,
    /// `generic_parent_without_signature`: mapping a generic type's
    /// enclosing type through a class without a `Signature` attribute no
    /// longer fails.
    GenericParentWithoutSignature,
    /// `self_referencing_lambda`: a lambda whose implementation method
    /// instantiates it again is written as a method reference.
    SelfReferencingLambda,
    /// The fix `external_class_diamond`: `new C(..)` of a class outside the
    /// input whose expected type is a parameterization of `C` keeps its diamond.
    ExternalClassDiamond,
    /// `statement_as_expression`: a statement-only exprent (an
    /// `assert`) is not written as a lambda's expression body or a `for` update.
    StatementAsExpression,
    /// `enum_body_separator`: an enum body with members but no
    /// constants starts with the `;` Java requires before its members.
    EnumBodySeparator,
    /// `anonymous_static_qualifier`: a static member of an anonymous
    /// class used inside it is written unqualified, not as `<unrepresentable>.m`.
    AnonymousStaticQualifier,
    /// `assert_dead_throw`: the `throw new AssertionError` an assert
    /// consumed through a break is removed, not left behind as dead code.
    AssertDeadThrow,
    /// `handler_copy_offsets`: the `finally` processing strips a
    /// duplicated exception handler, which has no instruction offsets.
    HandlerCopyOffsets,
    /// Fix `nested_handler_copies`: a duplicated exception handler is
    /// protected by the copies of the outer handlers that protect the range
    /// it serves.
    NestedHandlerCopies,
    /// Fix `copied_finally`: a `finally` copied by irreducible-flow
    /// splitting is a `finally` like its original.
    CopiedFinally,
    /// `type_switch_enum_label`: a pattern switch's qualified enum
    /// constant labels (`EnumDesc` dynamic constants) are resolved.
    TypeSwitchEnumLabel,
    /// `type_switch_pattern_type`: a type pattern made from a class
    /// label without a cast is typed by that class, not by `Class`.
    TypeSwitchPatternType,
    /// `type_switch_label_lookup`: each pattern-switch label is found
    /// by its own index constant, so labels after the first of a case and
    /// labels whose index equals an integer label's value are resolved.
    TypeSwitchLabelLookup,
    /// `kt_return_outside_lambda`: a Kotlin `return` is written as its bare
    /// value only in a lambda's body, not in a method without metadata or an
    /// object's function written inside a lambda.
    KtReturnOutsideLambda,
    /// `kt_failed_method_hidden`: a Kotlin method without metadata whose
    /// body fails is written with its failure comment instead of hidden.
    KtFailedMethodHidden,
    /// `kt_object_super_call`: an `object` expression's superclass is
    /// written with the arguments of the superclass constructor call.
    KtObjectSuperCall,
    /// `kt_vararg_from_metadata`: a Kotlin parameter is `vararg` exactly when
    /// its metadata gives it a vararg element type.
    KtVarargFromMetadata,
    /// `kt_default_arg_assigned_param`: a default value is read from the
    /// default-arguments branch's assignment to the parameter itself.
    KtDefaultArgAssignedParam,
    /// `kt_constructor_diamond`: a Kotlin constructor call whose type
    /// arguments are inferred is written without the Java diamond `<>`.
    KtConstructorDiamond,
    /// `kt_local_visibility_comment`: the note on a local declaration written
    /// outside its function names the function and keeps the declaration's indent.
    KtLocalVisibilityComment,
    /// `kt_default_args_ctor_descriptor`: a constructor's default-arguments
    /// constructor is looked up by the constructor's own descriptor.
    KtDefaultArgsCtorDescriptor,
    /// `var_merge_narrowing`: two variables are not merged when the
    /// merged type is a strict subtype of what either is assigned (the merge
    /// would print a cast the bytecode does not have there).
    VarMergeNarrowing,
    /// `switch_expression_poly_cast`: a switch expression whose results
    /// javac would promote together is not cast to its target type.
    SwitchExpressionPolyCast,
    /// `ternary_numeric_arms`: a reference conditional whose arms print as
    /// different numeric types keeps its first arm cast to its type.
    TernaryNumericArms,
    /// `kt_reference_anonymous_super_call`: a function reference whose
    /// class is an anonymous class is read from the super call
    /// `NestedClassProcessor` took out of its constructor.
    KtReferenceAnonymousSuperCall,
    /// `kt_property_reference`: a property-reference class is written as
    /// the reference it stands for.
    KtPropertyReference,
    /// `kt_anonymous_class_failure_reason`: an anonymous class the Kotlin
    /// writer cannot write fails with a message naming it and why.
    KtAnonymousClassFailureReason,
    /// `nonpublic_nested_import`: a non-public member class of a class
    /// outside the input and the current package is named through its
    /// enclosing class instead of imported.
    NonpublicNestedImport,
    /// `primitive_array_wildcard`: a wildcard type argument bounded by a
    /// primitive array type (`? extends byte[]`) keeps its wildcard.
    PrimitiveArrayWildcard,
    /// `override_inaccessible_method`: `@Override` is not written for a
    /// method that matches only a private supertype method or a
    /// package-private one of another package.
    OverrideInaccessibleMethod,
    /// `pattern_variable_scope`: an `instanceof` and the cast assignment
    /// after it become a pattern only when the variable is not used outside
    /// the pattern's scope.
    PatternVariableScope,
    /// `switch_case_scope`: the declarations at the top level of a case
    /// stay in scope for the later cases, so a clashing one is renamed or
    /// both cases get braces.
    SwitchCaseScope,
    /// `local_class_anonymous_subclass`: a local class is declared before
    /// the first anonymous class that extends it.
    LocalClassAnonymousSubclass,
    /// `kt_for_loop_exit_condition`: a Kotlin `for` loop that cannot be
    /// resugared is written `while (COND)`, not `while (true) { if (COND) break`.
    KtForLoopExitCondition,
    /// `kt_for_range_counter`: a Kotlin range loop is written only for a
    /// `for` that counts its initializer's variable to the bound.
    KtForRangeCounter,
    /// `kt_for_range_exclusive_bound`: a Kotlin range loop keeps the `for`
    /// loop's exclusive bound (`until`, `repeat(N)`).
    KtForRangeExclusiveBound,
    /// `kt_for_range_unmatched_bound`: a Kotlin `for` whose bound is neither
    /// a constant nor a variable is written as a `while`, not dropped.
    KtForRangeUnmatchedBound,
    /// `kt_for_loop_definitions`: a Kotlin `for` loop written as a `while`
    /// declares the variables Vineflower attached to the loop.
    KtForLoopDefinitions,
    /// `kt_for_loop_continue_update`: a Kotlin `for` loop written as a loop
    /// whose body continues it runs the update on `continue`.
    KtForLoopContinueUpdate,
    /// `var_merge_liveness`: two variables of one original slot are merged
    /// only when the earlier one's value is not read after the later one is
    /// written.
    VarMergeLiveness,
    /// `kt_partial_array_dims`: a Kotlin array creation that sizes fewer
    /// dimensions than its type (`arrayOfNulls<ByteArray>(n)`) is written.
    KtPartialArrayDims,
    /// `irreducible_with_exception_edges`: an irreducible level of the
    /// statement graph that holds exception edges is split like one without.
    IrreducibleWithExceptionEdges,
    /// `kt_semaphore_store_liveness`: Kotlin dead-variable elimination keeps
    /// the stores to a `finally` semaphore, which the finally's guard reads.
    KtSemaphoreStoreLiveness,
    /// `lambda_catch_var_rename`: a lambda body's catch variable renamed for
    /// a clash with the enclosing method is declared under its new name.
    LambdaCatchVarRename,
    /// `type_annotation_qualified_name`: an annotated class type keeps the
    /// qualifier the import collector gives its name (`Map.@A Entry`).
    TypeAnnotationQualifiedName,
    /// `type_annotation_argument_scope`: a type annotation on a generic
    /// argument is written in that argument, not in an earlier one.
    TypeAnnotationArgumentScope,
    /// `assert_throw_after_if`: an assert whose `throw new AssertionError`
    /// is the statement after an `if ($assertionsDisabled || cond)` whose
    /// body cannot complete normally is resugared.
    AssertThrowAfterIf,
    /// `sealed_private_permits`: a top-level sealed class whose permitted
    /// subclasses are its own members, one of them private, has no `permits`.
    SealedPrivatePermits,
    /// `switch_expression_assignments`: a switch statement becomes a switch
    /// expression only when each assignment made a `yield` ends its case.
    SwitchExpressionAssignments,
    /// `nested_param_rename`: a parameter whose uses are written by its
    /// local-variable-table name is declared under that name.
    NestedParamRename,
    /// `switch_expression_arm_scope`: a declaration in a switch expression's
    /// arm that clashes with a variable in scope around the expression is
    /// renamed.
    SwitchExpressionArmScope,
    /// `unchecked_type_pattern`: an `instanceof` and the unchecked cast
    /// after it are not made a type pattern, which Java rejects.
    UncheckedTypePattern,
    /// `inner_method_argument_shift`: the arguments of a call to an inner
    /// class's method are matched to its generic parameters without the
    /// one-place shift meant for the outer instance of its constructors.
    InnerMethodArgumentShift,
    /// `raw_receiver_erasure`: a call on a raw-typed receiver has the
    /// erased result type, and a for-each over a raw iterable is cast to
    /// its parameterized class.
    RawReceiverErasure,
    /// `boxed_type_variable_cast`: a cast to a type variable keeps the
    /// boxing call of its primitive operand.
    BoxedTypeVariableCast,
    /// `lambda_argument_overload`: a lambda argument that another overload's
    /// functional interface also accepts is cast to its interface.
    LambdaArgumentOverload,
    /// `static_type_variable_scope`: a class's type variables are not in
    /// scope in its static methods.
    StaticTypeVariableScope,
    /// `unchecked_generic_cast`: a declared value whose generic type the
    /// target's type arguments do not accept keeps its unchecked cast.
    UncheckedGenericCast,
    /// `kt_when_case_type`: a Kotlin `when` case constant is written with its
    /// selector's type.
    KtWhenCaseType,
    /// `kt_method_object_return_type`: a Kotlin method written without
    /// metadata that returns `Object` has its return type.
    KtMethodObjectReturnType,
    /// `kt_secondary_constructor_line`: a Kotlin secondary constructor
    /// without a body ends its line.
    KtSecondaryConstructorLine,
    /// `kt_range_operand_precedence`: a Kotlin range bound that binds less
    /// tightly than the range operator is parenthesised.
    KtRangeOperandPrecedence,
    /// `kt_lambda_singleton_returns`: a Kotlin singleton lambda with an exit
    /// not in tail position is labelled and returns with `return@label`.
    KtLambdaSingletonReturns,
    /// `kt_type_parameter_id`: a Kotlin metadata type that refers to a type
    /// parameter by id is written with its name.
    KtTypeParameterId,
    /// `kt_java_cast_syntax`: a Kotlin numeric conversion or cast to a type
    /// variable is written in Kotlin's syntax.
    KtJavaCastSyntax,
    /// `lambda_pertinence`: a type variable of a generic method that only
    /// implicitly typed lambdas and inexact method references determine is
    /// not taken for inferred from its arguments.
    LambdaPertinence,
    /// `lambda_type_variable_target`: a lambda passed for a bare type
    /// variable that javac resolves from other bounds to another type is
    /// cast to its interface.
    LambdaTypeVariableTarget,
    /// `unchecked_call_result_cast`: a call result whose declared generic
    /// type the target's type arguments do not accept keeps its unchecked
    /// cast.
    UncheckedCallResultCast,
    /// `loop_body_exit_explicit`: the edge leaving a loop's body is never
    /// taken for the fall-through at the end of the loop, so it is printed.
    LoopBodyExitExplicit,
    /// `type_variable_shadowing`: a type variable declared closer to the
    /// code keeps its bounds when an enclosing class or method declares one
    /// of the same name.
    TypeVariableShadowing,
    /// `compound_shift_count_type`: the shift count of a compound shift
    /// assignment is not given the type of the variable it shifts.
    CompoundShiftCountType,
    /// `definition_const_type`: a constant initializing a variable declared
    /// with an integral type from the local variable table is not written
    /// as a boolean.
    DefinitionConstType,
    /// `catch_tail_inline`: code after a `try` that only a catch clause
    /// reaches and that reads the clause's variable is moved into the clause.
    CatchTailInline,
    /// `diamond_argument_conflict`: a generic `new` whose argument binds a
    /// type parameter to another type variable than the expected type does
    /// is written raw, not with a diamond javac cannot infer.
    DiamondArgumentConflict,
    /// `kt_default_args_call`: a Kotlin call through a function's or a
    /// constructor's default-arguments bridge is written as the call the
    /// source makes, the defaulted arguments left out.
    KtDefaultArgsCall,
    /// `kt_generated_constructor_hidden`: the constructors kotlinc generates
    /// for a Kotlin class's default constructor arguments (no-argument and
    /// `@JvmOverloads` overloads) are not written as functions.
    KtGeneratedConstructorHidden,
    /// `kt_java_reference_cast`: a cast to a class or array type that
    /// Vineflower writes in Java's syntax in Kotlin output is written
    /// `x as T`.
    KtJavaReferenceCast,
    /// `kt_explicit_type_arguments`: a Kotlin call's explicit type arguments
    /// are written after the function's name, not in Java's syntax before it.
    KtExplicitTypeArguments,
    /// `kt_abstract_class_constructor`: an abstract Kotlin class's
    /// constructors are read from its metadata and written as constructors,
    /// not as functions named after the class.
    KtAbstractClassConstructor,
    /// `kt_constructor_body_after_first_block`: a Kotlin constructor whose
    /// code after its `super`/`this` call starts in a later block (an `if`)
    /// keeps that code, which Vineflower drops.
    KtConstructorBodyAfterFirstBlock,
    /// `kt_inline_class_constructor_accessor`: a Kotlin constructor with an
    /// inline-class parameter is read from the private constructor holding
    /// its code, not from the synthetic accessor its metadata names.
    KtInlineClassConstructorAccessor,
    /// `kt_constructor_default_signature`: a Kotlin constructor whose
    /// metadata leaves out its JVM descriptor is looked up by the descriptor
    /// its parameters give, as Kotlin's reader does.
    KtConstructorDefaultSignature,
    /// `kt_jump_to_non_loop_label`: a jump out of a labelled `if`, `switch`
    /// or unresugared `for` loop, or a `continue` of a `repeat` loop, is a
    /// `return` from the lambda Kotlin output writes the statement as, not a
    /// `break`/`continue` to a label that is not a loop's.
    KtJumpToNonLoopLabel,
    /// `kt_synthetic_object_super_call`: an anonymous class with no Class
    /// metadata whose superclass constructor takes arguments (a suspend
    /// function's continuation) is written as an `object` expression passing
    /// them.
    KtSyntheticObjectSuperCall,
    /// `kt_file_class_constructor`: the constructor kotlinc gives a Kotlin
    /// file class (a multifile class part, so that the parts can extend one
    /// another) is not written as a top-level function.
    KtFileClassConstructor,
    /// `kt_when_break`: an unlabelled `break` out of a `switch` written as a
    /// Kotlin `when` is left out where its branch ends after it, and is
    /// otherwise a `return` from a `run` lambda around the `when`.
    KtWhenBreak,
    /// `kt_java_statement_jump`: in Kotlin output, the jump after a statement
    /// that a Java statement writer writes (in a `try`, `finally` or
    /// `synchronized` block) is written in Kotlin's syntax, not Java's.
    KtJavaStatementJump,
    /// `kt_catch_block_jump`: a Kotlin `catch` block is written with the
    /// jump that follows it, which Vineflower leaves out.
    KtCatchBlockJump,
    /// `kt_supertype_type_arguments`: the superclass call in a Kotlin class
    /// header names the superclass with its type arguments.
    KtSupertypeTypeArguments,
    /// `anonymous_class_created_elsewhere`: an anonymous class that only
    /// another class creates (kotlinc's `$$inlined$` copies) is nested in
    /// that class, and its captured values are mapped from the creating method.
    AnonymousClassCreatedElsewhere,
    /// `kt_legacy_constructor_parameter_checks`: Kotlin 1.3's parameter null
    /// checks (`Intrinsics.checkParameterIsNotNull`) before a constructor's
    /// delegation call are dropped like Kotlin 1.4's, so the call is found.
    KtLegacyConstructorParameterChecks,
    /// `kt_hidden_empty_super_call`: a Kotlin constructor whose empty
    /// superclass call `hideEmptySuper` removed is written with `()` after
    /// the superclass, and a secondary one without another call taken for it.
    KtHiddenEmptySuperCall,
    /// `kt_compound_bitwise_assignment`: in Kotlin output, a compound
    /// assignment with a bitwise or shift operator, which Kotlin lacks, is
    /// written `x = x or y`.
    KtCompoundBitwiseAssignment,
    /// `kt_switch_head_indent`: the statements before a `when` (the
    /// switch's first block) are written at the `when`'s indentation.
    KtSwitchHeadIndent,
}

impl KunaFix {
    /// Every fix, in declaration order.
    pub const ALL: &'static [KunaFix] = &[
        KunaFix::ReportToKuna,
        KunaFix::LambdaWildcardReturn,
        KunaFix::TwrUnaryCondition,
        KunaFix::IfDenestFallthrough,
        KunaFix::SwitchYieldConstRetype,
        KunaFix::CatchLoopBackSuccessor,
        KunaFix::KtSyntheticAnonymousObject,
        KunaFix::KtLocalAsExtensionReceiver,
        KunaFix::KtDefaultArgsSuperCall,
        KunaFix::KtDefaultArgCompoundBranch,
        KunaFix::GenericParentWithoutSignature,
        KunaFix::SelfReferencingLambda,
        KunaFix::ExternalClassDiamond,
        KunaFix::StatementAsExpression,
        KunaFix::EnumBodySeparator,
        KunaFix::AnonymousStaticQualifier,
        KunaFix::AssertDeadThrow,
        KunaFix::HandlerCopyOffsets,
        KunaFix::NestedHandlerCopies,
        KunaFix::CopiedFinally,
        KunaFix::TypeSwitchEnumLabel,
        KunaFix::TypeSwitchPatternType,
        KunaFix::TypeSwitchLabelLookup,
        KunaFix::KtReturnOutsideLambda,
        KunaFix::KtFailedMethodHidden,
        KunaFix::KtObjectSuperCall,
        KunaFix::KtVarargFromMetadata,
        KunaFix::KtDefaultArgAssignedParam,
        KunaFix::KtConstructorDiamond,
        KunaFix::KtLocalVisibilityComment,
        KunaFix::KtDefaultArgsCtorDescriptor,
        KunaFix::VarMergeNarrowing,
        KunaFix::SwitchExpressionPolyCast,
        KunaFix::TernaryNumericArms,
        KunaFix::KtReferenceAnonymousSuperCall,
        KunaFix::KtPropertyReference,
        KunaFix::KtAnonymousClassFailureReason,
        KunaFix::NonpublicNestedImport,
        KunaFix::PrimitiveArrayWildcard,
        KunaFix::OverrideInaccessibleMethod,
        KunaFix::PatternVariableScope,
        KunaFix::SwitchCaseScope,
        KunaFix::LocalClassAnonymousSubclass,
        KunaFix::KtForLoopExitCondition,
        KunaFix::KtForRangeCounter,
        KunaFix::KtForRangeExclusiveBound,
        KunaFix::KtForRangeUnmatchedBound,
        KunaFix::KtForLoopDefinitions,
        KunaFix::KtForLoopContinueUpdate,
        KunaFix::VarMergeLiveness,
        KunaFix::KtPartialArrayDims,
        KunaFix::IrreducibleWithExceptionEdges,
        KunaFix::KtSemaphoreStoreLiveness,
        KunaFix::LambdaCatchVarRename,
        KunaFix::TypeAnnotationQualifiedName,
        KunaFix::TypeAnnotationArgumentScope,
        KunaFix::AssertThrowAfterIf,
        KunaFix::SealedPrivatePermits,
        KunaFix::SwitchExpressionAssignments,
        KunaFix::NestedParamRename,
        KunaFix::SwitchExpressionArmScope,
        KunaFix::UncheckedTypePattern,
        KunaFix::InnerMethodArgumentShift,
        KunaFix::RawReceiverErasure,
        KunaFix::BoxedTypeVariableCast,
        KunaFix::LambdaArgumentOverload,
        KunaFix::StaticTypeVariableScope,
        KunaFix::UncheckedGenericCast,
        KunaFix::KtWhenCaseType,
        KunaFix::KtMethodObjectReturnType,
        KunaFix::KtSecondaryConstructorLine,
        KunaFix::KtRangeOperandPrecedence,
        KunaFix::KtLambdaSingletonReturns,
        KunaFix::KtTypeParameterId,
        KunaFix::KtJavaCastSyntax,
        KunaFix::LambdaPertinence,
        KunaFix::LambdaTypeVariableTarget,
        KunaFix::UncheckedCallResultCast,
        KunaFix::LoopBodyExitExplicit,
        KunaFix::TypeVariableShadowing,
        KunaFix::CompoundShiftCountType,
        KunaFix::DefinitionConstType,
        KunaFix::CatchTailInline,
        KunaFix::DiamondArgumentConflict,
        KunaFix::KtDefaultArgsCall,
        KunaFix::KtGeneratedConstructorHidden,
        KunaFix::KtJavaReferenceCast,
        KunaFix::KtExplicitTypeArguments,
        KunaFix::KtAbstractClassConstructor,
        KunaFix::KtConstructorBodyAfterFirstBlock,
        KunaFix::KtInlineClassConstructorAccessor,
        KunaFix::KtConstructorDefaultSignature,
        KunaFix::KtJumpToNonLoopLabel,
        KunaFix::KtSyntheticObjectSuperCall,
        KunaFix::KtFileClassConstructor,
        KunaFix::KtWhenBreak,
        KunaFix::KtJavaStatementJump,
        KunaFix::KtCatchBlockJump,
        KunaFix::KtSupertypeTypeArguments,
        KunaFix::AnonymousClassCreatedElsewhere,
        KunaFix::KtLegacyConstructorParameterChecks,
        KunaFix::KtHiddenEmptySuperCall,
        KunaFix::KtCompoundBitwiseAssignment,
        KunaFix::KtSwitchHeadIndent,
    ];

    /// The fix's id, as the docs and `tests/jvm/kuna-fixes/cases.json` name it.
    pub fn id(self) -> &'static str {
        match self {
            KunaFix::ReportToKuna => "report_to_kuna",
            KunaFix::LambdaWildcardReturn => "lambda_wildcard_return",
            KunaFix::TwrUnaryCondition => "twr_unary_condition",
            KunaFix::IfDenestFallthrough => "if_denest_fallthrough",
            KunaFix::SwitchYieldConstRetype => "switch_yield_const_retype",
            KunaFix::CatchLoopBackSuccessor => "catch_loop_back_successor",
            KunaFix::KtSyntheticAnonymousObject => "kt_synthetic_anonymous_object",
            KunaFix::KtLocalAsExtensionReceiver => "kt_local_as_extension_receiver",
            KunaFix::KtDefaultArgsSuperCall => "kt_default_args_super_call",
            KunaFix::KtDefaultArgCompoundBranch => "kt_default_arg_compound_branch",
            KunaFix::GenericParentWithoutSignature => "generic_parent_without_signature",
            KunaFix::SelfReferencingLambda => "self_referencing_lambda",
            KunaFix::ExternalClassDiamond => "external_class_diamond",
            KunaFix::StatementAsExpression => "statement_as_expression",
            KunaFix::EnumBodySeparator => "enum_body_separator",
            KunaFix::AnonymousStaticQualifier => "anonymous_static_qualifier",
            KunaFix::AssertDeadThrow => "assert_dead_throw",
            KunaFix::HandlerCopyOffsets => "handler_copy_offsets",
            KunaFix::NestedHandlerCopies => "nested_handler_copies",
            KunaFix::CopiedFinally => "copied_finally",
            KunaFix::TypeSwitchEnumLabel => "type_switch_enum_label",
            KunaFix::TypeSwitchPatternType => "type_switch_pattern_type",
            KunaFix::TypeSwitchLabelLookup => "type_switch_label_lookup",
            KunaFix::KtReturnOutsideLambda => "kt_return_outside_lambda",
            KunaFix::KtFailedMethodHidden => "kt_failed_method_hidden",
            KunaFix::KtObjectSuperCall => "kt_object_super_call",
            KunaFix::KtVarargFromMetadata => "kt_vararg_from_metadata",
            KunaFix::KtDefaultArgAssignedParam => "kt_default_arg_assigned_param",
            KunaFix::KtConstructorDiamond => "kt_constructor_diamond",
            KunaFix::KtLocalVisibilityComment => "kt_local_visibility_comment",
            KunaFix::KtDefaultArgsCtorDescriptor => "kt_default_args_ctor_descriptor",
            KunaFix::VarMergeNarrowing => "var_merge_narrowing",
            KunaFix::SwitchExpressionPolyCast => "switch_expression_poly_cast",
            KunaFix::TernaryNumericArms => "ternary_numeric_arms",
            KunaFix::KtReferenceAnonymousSuperCall => "kt_reference_anonymous_super_call",
            KunaFix::KtPropertyReference => "kt_property_reference",
            KunaFix::KtAnonymousClassFailureReason => "kt_anonymous_class_failure_reason",
            KunaFix::NonpublicNestedImport => "nonpublic_nested_import",
            KunaFix::PrimitiveArrayWildcard => "primitive_array_wildcard",
            KunaFix::OverrideInaccessibleMethod => "override_inaccessible_method",
            KunaFix::PatternVariableScope => "pattern_variable_scope",
            KunaFix::SwitchCaseScope => "switch_case_scope",
            KunaFix::LocalClassAnonymousSubclass => "local_class_anonymous_subclass",
            KunaFix::KtForLoopExitCondition => "kt_for_loop_exit_condition",
            KunaFix::KtForRangeCounter => "kt_for_range_counter",
            KunaFix::KtForRangeExclusiveBound => "kt_for_range_exclusive_bound",
            KunaFix::KtForRangeUnmatchedBound => "kt_for_range_unmatched_bound",
            KunaFix::KtForLoopDefinitions => "kt_for_loop_definitions",
            KunaFix::KtForLoopContinueUpdate => "kt_for_loop_continue_update",
            KunaFix::VarMergeLiveness => "var_merge_liveness",
            KunaFix::KtPartialArrayDims => "kt_partial_array_dims",
            KunaFix::IrreducibleWithExceptionEdges => "irreducible_with_exception_edges",
            KunaFix::KtSemaphoreStoreLiveness => "kt_semaphore_store_liveness",
            KunaFix::LambdaCatchVarRename => "lambda_catch_var_rename",
            KunaFix::TypeAnnotationQualifiedName => "type_annotation_qualified_name",
            KunaFix::TypeAnnotationArgumentScope => "type_annotation_argument_scope",
            KunaFix::AssertThrowAfterIf => "assert_throw_after_if",
            KunaFix::SealedPrivatePermits => "sealed_private_permits",
            KunaFix::SwitchExpressionAssignments => "switch_expression_assignments",
            KunaFix::NestedParamRename => "nested_param_rename",
            KunaFix::SwitchExpressionArmScope => "switch_expression_arm_scope",
            KunaFix::UncheckedTypePattern => "unchecked_type_pattern",
            KunaFix::InnerMethodArgumentShift => "inner_method_argument_shift",
            KunaFix::RawReceiverErasure => "raw_receiver_erasure",
            KunaFix::BoxedTypeVariableCast => "boxed_type_variable_cast",
            KunaFix::LambdaArgumentOverload => "lambda_argument_overload",
            KunaFix::StaticTypeVariableScope => "static_type_variable_scope",
            KunaFix::UncheckedGenericCast => "unchecked_generic_cast",
            KunaFix::KtWhenCaseType => "kt_when_case_type",
            KunaFix::KtMethodObjectReturnType => "kt_method_object_return_type",
            KunaFix::KtSecondaryConstructorLine => "kt_secondary_constructor_line",
            KunaFix::KtRangeOperandPrecedence => "kt_range_operand_precedence",
            KunaFix::KtLambdaSingletonReturns => "kt_lambda_singleton_returns",
            KunaFix::KtTypeParameterId => "kt_type_parameter_id",
            KunaFix::KtJavaCastSyntax => "kt_java_cast_syntax",
            KunaFix::LambdaPertinence => "lambda_pertinence",
            KunaFix::LambdaTypeVariableTarget => "lambda_type_variable_target",
            KunaFix::UncheckedCallResultCast => "unchecked_call_result_cast",
            KunaFix::LoopBodyExitExplicit => "loop_body_exit_explicit",
            KunaFix::TypeVariableShadowing => "type_variable_shadowing",
            KunaFix::CompoundShiftCountType => "compound_shift_count_type",
            KunaFix::DefinitionConstType => "definition_const_type",
            KunaFix::CatchTailInline => "catch_tail_inline",
            KunaFix::DiamondArgumentConflict => "diamond_argument_conflict",
            KunaFix::KtDefaultArgsCall => "kt_default_args_call",
            KunaFix::KtGeneratedConstructorHidden => "kt_generated_constructor_hidden",
            KunaFix::KtJavaReferenceCast => "kt_java_reference_cast",
            KunaFix::KtExplicitTypeArguments => "kt_explicit_type_arguments",
            KunaFix::KtAbstractClassConstructor => "kt_abstract_class_constructor",
            KunaFix::KtConstructorBodyAfterFirstBlock => "kt_constructor_body_after_first_block",
            KunaFix::KtInlineClassConstructorAccessor => "kt_inline_class_constructor_accessor",
            KunaFix::KtConstructorDefaultSignature => "kt_constructor_default_signature",
            KunaFix::KtJumpToNonLoopLabel => "kt_jump_to_non_loop_label",
            KunaFix::KtSyntheticObjectSuperCall => "kt_synthetic_object_super_call",
            KunaFix::KtFileClassConstructor => "kt_file_class_constructor",
            KunaFix::KtWhenBreak => "kt_when_break",
            KunaFix::KtJavaStatementJump => "kt_java_statement_jump",
            KunaFix::KtCatchBlockJump => "kt_catch_block_jump",
            KunaFix::KtSupertypeTypeArguments => "kt_supertype_type_arguments",
            KunaFix::AnonymousClassCreatedElsewhere => "anonymous_class_created_elsewhere",
            KunaFix::KtLegacyConstructorParameterChecks => "kt_legacy_constructor_parameter_checks",
            KunaFix::KtHiddenEmptySuperCall => "kt_hidden_empty_super_call",
            KunaFix::KtCompoundBitwiseAssignment => "kt_compound_bitwise_assignment",
            KunaFix::KtSwitchHeadIndent => "kt_switch_head_indent",
        }
    }

    /// The fix whose id is `id`.
    pub fn from_id(id: &str) -> Option<KunaFix> {
        KunaFix::ALL.iter().copied().find(|f| f.id() == id)
    }

    const fn bit(self) -> u128 {
        1 << (self as u32)
    }
}

/// A set of active fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KunaFixes(u128);

const _: () = assert!(KunaFix::ALL.len() <= u128::BITS as usize);

impl KunaFixes {
    /// Every fix active: the engine's default.
    pub const DEFAULT: KunaFixes = KunaFixes(u128::MAX);
    /// No fix active: Vineflower's behaviour.
    pub const COMPAT: KunaFixes = KunaFixes(0);

    pub fn contains(self, fix: KunaFix) -> bool {
        self.0 & fix.bit() != 0
    }

    /// The set for option pairs (later pairs win): every fix, or none while
    /// `jvm_vineflower_compat` is on.
    pub fn from_options<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> KunaFixes {
        let compat = pairs
            .into_iter()
            .filter(|(key, _)| *key == VINEFLOWER_COMPAT)
            .last()
            .is_some_and(|(_, value)| switch_value(value));
        if compat {
            KunaFixes::COMPAT
        } else {
            KunaFixes::DEFAULT
        }
    }

    /// This set without `fix`: a test hook for isolating one fix.
    #[doc(hidden)]
    pub const fn without(self, fix: KunaFix) -> KunaFixes {
        KunaFixes(self.0 & !fix.bit())
    }

    /// The active fixes, in [`KunaFix::ALL`] order.
    pub fn list(self) -> Vec<KunaFix> {
        KunaFix::ALL
            .iter()
            .copied()
            .filter(|f| self.contains(*f))
            .collect()
    }
}

/// Whether a switch value turns its option on (`0`, `off` and `false` turn it off).
pub fn switch_value(value: &str) -> bool {
    !(value == "0" || value.eq_ignore_ascii_case("off") || value.eq_ignore_ascii_case("false"))
}

/// Whether `key` is `jvm_vineflower_compat`.
pub fn is_switch(key: &str) -> bool {
    key == VINEFLOWER_COMPAT
}

/// Splits `jvm_vineflower_compat` off a custom property list: the resulting
/// set and the other pairs, in order.
pub fn take_switches(custom_properties: &[(String, String)]) -> (KunaFixes, Vec<(String, String)>) {
    let fixes = KunaFixes::from_options(
        custom_properties
            .iter()
            .filter(|(k, _)| is_switch(k))
            .map(|(k, v)| (k.as_str(), v.as_str())),
    );
    let rest = custom_properties
        .iter()
        .filter(|(k, _)| !is_switch(k))
        .cloned()
        .collect();
    (fixes, rest)
}

thread_local! {
    static ACTIVE: Cell<KunaFixes> = const { Cell::new(KunaFixes::DEFAULT) };
}

/// Whether `fix` is active for the engine instance running on this thread.
pub fn enabled(fix: KunaFix) -> bool {
    ACTIVE.with(|a| a.get().contains(fix))
}

/// The fixes [`enabled`] answers from on this thread.
pub fn active() -> KunaFixes {
    ACTIVE.with(Cell::get)
}

/// Makes `fixes` the set [`enabled`] answers from on this thread; installing
/// a decompilation context does this with the context's set.
pub fn activate(fixes: KunaFixes) {
    ACTIVE.with(|a| a.set(fixes))
}
