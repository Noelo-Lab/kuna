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
    /// `captured_var_clash_name`: a captured variable in an anonymous or
    /// local class or a lambda is printed with the clashing name the creating
    /// method's variable is renamed to, not the name it had before.
    CapturedVarClashName,
    /// `captured_var_through_cast`: a `new` of an anonymous or local class
    /// that passes a variable cast to a supertype of its class captures the
    /// variable.
    CapturedVarThroughCast,
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
    /// `kt_operator_precedence`: in Kotlin output, operands are
    /// parenthesised by Kotlin's operator precedence, not Java's alone.
    KtOperatorPrecedence,
    /// `kt_narrow_bitwise_operand`: in Kotlin output, a `Byte`, `Short` or
    /// `Char` operand of a bitwise or shift operator is widened to `Int`.
    KtNarrowBitwiseOperand,
    /// `kt_reference_inequality`: in Kotlin output, Java's `!=` of two
    /// references (an identity test) is written `!==`, not `!=` (`equals`).
    KtReferenceInequality,
    /// `handler_copy_shared_body`: the copies the exception deobfuscator
    /// makes of a catch-any handler's block get their own copies of the
    /// handler's further blocks, which the finally processing edits per copy.
    HandlerCopySharedBody,
    /// `kt_field_without_property`: in Kotlin output, an instance field
    /// with no Kotlin property is written as a property (`@JvmField var x:
    /// Int = 0`), not as a Java field declaration.
    KtFieldWithoutProperty,
    /// `kt_constructor_without_metadata`: in Kotlin output, a constructor
    /// with no Kotlin metadata is written `constructor(..) : super(..)`, and
    /// an anonymous class's initializer `init { .. }`, not in Java syntax.
    KtConstructorWithoutMetadata,
    /// `kt_delegation_call_arguments`: in Kotlin output, the arguments of a
    /// constructor's delegation call that kotlinc computed before a null
    /// check are written in the call, not as statements before it.
    KtDelegationCallArguments,
    /// `kt_copied_equality`: in Kotlin output, a copy of an `==` that tests
    /// equality (`Intrinsics.areEqual`) stays `==`, not `===` (identity).
    KtCopiedEquality,
    /// `kt_binary_numeric_promotion`: in Kotlin output, an operand of an
    /// arithmetic, comparison or equality operator is converted where Java's
    /// binary numeric promotion converts it and Kotlin's operator does not.
    KtBinaryNumericPromotion,
    /// `kt_negated_equality`: in Kotlin output, an `==` or `!=` that `xor
    /// true` negates (kotlinc's `a != b`) is written with the other operator.
    KtNegatedEquality,
    /// `kt_operator_line_break`: in Kotlin output, a long binary operation
    /// breaks its line after the operator, where Kotlin does not end the
    /// statement, not before it.
    KtOperatorLineBreak,
    /// `kt_unboxing_cast_type`: in Kotlin output, the cast an unwritten
    /// unboxing call unboxes is written to the Kotlin type, `x as Boolean`.
    KtUnboxingCastType,
    /// `twr_desugaring_shape`: try-with-resources is re-sugared only from
    /// javac's exact desugaring, not from code that skips a `close()`,
    /// reorders the initializer or handles the close failure otherwise.
    TwrDesugaringShape,
    /// `twr_unresolved_resource`: javac's try-with-resources desugaring is
    /// re-sugared when the resource's class hierarchy is outside the input.
    TwrUnresolvedResource,
    /// `compound_assignment_shape`: an instance field or array element is
    /// written `x op= e` exactly when the bytecode loads its receiver once,
    /// and `x = e op x` is never written `x op= e`.
    CompoundAssignmentShape,
    /// `twr_class_version`: javac 11+'s try-with-resources desugaring is
    /// recognised in class files older than Java 11 too (`--release 8`).
    TwrClassVersion,
    /// `implicit_outer_instance`: an inner class created with the outer
    /// instance an unqualified creation passes, and without javac's null check
    /// of a qualified one, is written `new Inner()`, not `Outer.this.new Inner()`.
    ImplicitOuterInstance,
    /// `vararg_array_length`: an array created by its length (not constant
    /// 0) that a varargs call passes last is written as the array, not as its
    /// (absent) elements.
    VarargArrayLength,
    /// `synthetic_null_check_shape`: a `requireNonNull`/`getClass` statement
    /// is removed as javac's null check only when its instructions are one
    /// (a check of a `dup`, or one before a constant).
    SyntheticNullCheckShape,
    /// `var_merge_declared_type`: two variables of one slot that the
    /// LocalVariableTable declares as two locals of one erased type and
    /// different generic types are not merged into one.
    VarMergeDeclaredType,
    /// `compact_record_constructor`: a canonical record constructor is written
    /// in compact form (its field stores removed) only when it ends with every
    /// component's `this.c = c`, in order (upstream 1506b2f4).
    CompactRecordConstructor,
    /// `disjoint_generic_cast`: a generic cast between two parameterizations
    /// javac finds provably distinct is written to the raw class.
    DisjointGenericCast,
    /// `thrown_type_variable`: a thrown type variable is kept: a `throw` of a
    /// checked exception the method declares only as `throws T` is cast to
    /// `T`, and a raw `new C()` receiver of a call throwing `C`'s type
    /// parameters is given `RuntimeException` for them.
    ThrownTypeVariable,
    /// `kt_java_keyword_variable_name`: in Kotlin output, a debug variable
    /// name that is a Java keyword but a Kotlin name (`new`, `default`) is
    /// kept, as the header that declares it already prints it.
    KtJavaKeywordVariableName,
    /// `kt_default_accessor_flags`: a property accessor whose flags the
    /// metadata leaves out has its property's visibility and modality.
    KtDefaultAccessorFlags,
    /// `kt_unescaped_name`: in Kotlin output, the variable, field and method
    /// names the writers shared with Java print are escaped as Kotlin names.
    KtUnescapedName,
    /// `kt_reserved_underscore_name`: a Kotlin name made of underscores alone
    /// is written in backticks.
    KtReservedUnderscoreName,
    /// `kt_java_type_syntax`: in Kotlin output, types read from a Java
    /// generic signature are written in Kotlin's syntax, not Java's.
    KtJavaTypeSyntax,
    /// `field_read_all_paths`: the Kotlin inliner moves a local's field value
    /// only to a use no path to which crosses a call, a field store or an
    /// exception edge.
    FieldReadAllPaths,
    /// `kt_inline_evaluation_order`: a Kotlin local whose value calls,
    /// allocates or reads an array element is inlined only into the next
    /// statement, ahead of anything there it could observe or be observed by.
    KtInlineEvaluationOrder,
    /// `qualified_super_call`: a superclass constructor call (or an anonymous
    /// class creation) passing an inner superclass an outer instance other
    /// than the implicit one is written qualified, `o.super(..)` (`o.new
    /// Inner(..) {..}`), without javac's null check of `o`.
    QualifiedSuperCall,
    /// `external_inherited_member_type`: member classes inherited from
    /// supertypes outside the class universe shadow imports of their simple
    /// name, and a simple name two inherited member classes share is written
    /// qualified.
    ExternalInheritedMemberType,
    /// `lambda_return_type_argument`: explicit type arguments that contradict
    /// what a lambda or method reference argument's implementation returns
    /// are not written.
    LambdaReturnTypeArgument,
    /// `type_annotation_outer_arguments`: an annotated inner class type keeps
    /// its enclosing classes' type arguments (`Outer<V>.@A Inner<?>`).
    TypeAnnotationOuterArguments,
    /// `kt_accessor_backing_field`: in Kotlin output, a property accessor's
    /// own backing field is written `field`, not `this.name` (a call of the
    /// accessor itself).
    KtAccessorBackingField,
    /// `kt_string_literal_escape`: in Kotlin output, a string literal escapes
    /// the `$` that would start a template, and a template's `Char` pieces.
    KtStringLiteralEscape,
    /// `kt_abstract_function_body`: in Kotlin output, an abstract or external
    /// function is written without a body.
    KtAbstractFunctionBody,
    /// `kt_misplaced_modality`: in Kotlin output, property accessors carry
    /// no modality and top-level properties no `final`.
    KtMisplacedModality,
    /// `kt_companion_property_initializer`: in Kotlin output, a companion
    /// object's property keeps the initializer stored in its outer class.
    KtCompanionPropertyInitializer,
    /// `kt_default_property_initializer`: in Kotlin output, a property whose
    /// field no constructor stores is written with its default value.
    KtDefaultPropertyInitializer,
    /// `kt_missed_override`: in Kotlin output, a function a bridge method
    /// calls, and a property whose getter overrides, are written `override`.
    KtMissedOverride,
    /// `kt_outer_this`: in Kotlin output, an enclosing class's instance is
    /// written `this@Outer`, not Java's `Outer.this`.
    KtOuterThis,
    /// `kt_extension_receiver`: in Kotlin output, an extension function's
    /// receiver is written `this` in its body, not its parameter's name.
    KtExtensionReceiver,
    /// `kt_nested_annotation_argument`: in Kotlin output, an annotation used
    /// as another annotation's argument is written without `@`.
    KtNestedAnnotationArgument,
    /// `kt_throws_class_name`: in Kotlin output, the `@Throws` of a `throws`
    /// clause names its classes `IOException::class`, not by internal name.
    KtThrowsClassName,
    /// `kt_throwable_cast`: in Kotlin output, a thrown value is written
    /// without kotlinc's no-op cast to `java.lang.Throwable`.
    KtThrowableCast,
    /// `kt_mapped_java_member`: in Kotlin output, calls of JDK methods that
    /// Kotlin's mapped types name otherwise (`length()`, `charAt`, `size()`,
    /// `getMessage()`, ..., an array's `length`) use Kotlin's names.
    KtMappedJavaMember,
    /// `kt_mapped_java_type`: in Kotlin output, JDK classes that Kotlin maps
    /// to built-in types (`java.lang.String`, `Throwable`, boxes) are
    /// written with the built-in's name.
    KtMappedJavaType,
    /// `kt_object_instance`: in Kotlin output, an `object`'s `INSTANCE`
    /// field is written as the object's name.
    KtObjectInstance,
    /// `kt_super_property_access`: in Kotlin output, a property read through
    /// the superclass's accessor (`invokespecial`) is written `super.name`.
    KtSuperPropertyAccess,
    /// `kt_setter_parameter_name`: in Kotlin output, a setter's body names its
    /// parameter as its header does (`value` for a `<set-?>` metadata name).
    KtSetterParameterName,
    /// `kt_java_class_literal`: in Kotlin output, a class constant written by
    /// Java's expression is `Name::class.java`, not Java's `Name.class`.
    KtJavaClassLiteral,
    /// `pattern_binding_shape`: an `instanceof` and a cast become a type
    /// pattern only where the bytecode is javac's compiled pattern: the
    /// subject read once and the binding right after the test's `ifeq`.
    PatternBindingShape,
    /// `finally_exit_versions`: the code after a `try`/`finally` sees the
    /// values the `finally` body assigned to locals.
    FinallyExitVersions,
    /// `var_merge_lost_definition`: a variable whose merge into an earlier one
    /// of its slot fails for want of a common type keeps its declaration.
    VarMergeLostDefinition,
    /// `head_definition_scope`: a declaration in a switch's or `synchronized`'s
    /// head block stays in scope after the statement for the clash renaming.
    HeadDefinitionScope,
    /// `switch_case_order`: a switch's cases are written in the order of
    /// their code, as javac and kotlinc lay the source's cases out, not by key.
    SwitchCaseOrder,
    /// `switch_table_fillers`: the keys javac fills a `tableswitch`'s gaps
    /// with are not written as labels of the default's case.
    SwitchTableFillers,
    /// `switch_synthetic_default`: the default javac adds to an exhaustive
    /// switch is left out of a switch expression and a pattern switch.
    SwitchSyntheticDefault,
    /// `vararg_spread_resolution`: the array a varargs call passes is written
    /// as its elements only when the call written so still resolves to the
    /// same method and packs the same array; a lone element the method could
    /// take as the array itself is cast to the element type.
    VarargSpreadResolution,
    /// `unseen_class_overload`: an argument of a call to a class outside the
    /// input keeps its parameter's type where another method of that name the
    /// input calls would otherwise be chosen.
    UnseenClassOverload,
    /// `double_constant_literal`: a `double` constant is written as a
    /// `double` literal, never as a shorter `float` literal or constant.
    DoubleConstantLiteral,
    /// `concat_operand_type`: a string concatenation operand is written with
    /// the type its `append`/`makeConcatWithConstants` argument converts it
    /// as (`char`, `int`, `long`, `float`, `double`).
    ConcatOperandType,
    /// `field_read_reuse`: a field value the bytecode reads once and uses
    /// twice (`dup`) is not read again at each use.
    FieldReadReuse,
    /// `overlapping_ranges`: protected ranges that overlap without nesting
    /// (kotlinc 1.3's catch range that also covers the `finally` copy before
    /// a `return`) are split, with per-path copies of the handler code, into
    /// ranges that nest.
    OverlappingRanges,
    /// `constructor_signature_alignment`: a constructor `Signature` that
    /// omits only the leading outer instance (kotlinc's pre-IR backend, whose
    /// local and anonymous class signatures keep the captured variables) is
    /// matched to the descriptor parameters it describes.
    ConstructorSignatureAlignment,
    /// `split_constructor_call`: an allocation `v = new X` and its constructor
    /// call `v.<init>(args)` that argument control flow put in different
    /// statements are joined into `v = new X(args)`.
    SplitConstructorCall,
    /// `local_final_writes`: a local written in a statement head (`if (++i > n)`)
    /// or in a loop it is declared outside of is not final (Kotlin `var`).
    LocalFinalWrites,
    /// `kt_not_null_assertion`: in Kotlin output, kotlinc's null check of a
    /// `!!` (`throwNpe`, `checkNotNull`) is written back as `x!!`.
    KtNotNullAssertion,
    /// `kt_expression_value_check`: in Kotlin output, kotlinc's check of a
    /// platform-typed value is deleted before the last stack-variable pass.
    KtExpressionValueCheck,
    /// `kt_static_initializer`: in Kotlin output, what is left of a static
    /// initializer is an `init` block of the object or companion object.
    KtStaticInitializer,
    /// `kt_object_instance_local`: a Kotlin 1.3 object's static initializer
    /// reads its instance from `INSTANCE` instead of a local.
    KtObjectInstanceLocal,
    /// `kt_property_initializer_key`: in Kotlin output, a property's initializer
    /// is found under its field's own name and descriptor.
    KtPropertyInitializerKey,
    /// `kt_synchronized_accessor`: in Kotlin output, a `synchronized` property
    /// accessor is annotated `@get:Synchronized` / `@set:Synchronized`.
    KtSynchronizedAccessor,
    /// `inline_marker_finally_copy`: a copy of a `finally` in a Kotlin inline
    /// function's body is recognised although its `InlineMarker` depth
    /// differs from the handler's.
    InlineMarkerFinallyCopy,
    /// `synchronized_break_exit`: the lock release at the target of a `break`
    /// out of a `synchronized` body is removed with the statement's others.
    SynchronizedBreakExit,
    /// `kt_unrepresentable_declared_type`: a Kotlin declaration with an
    /// initializer whose type is an anonymous class is written without a type.
    KtUnrepresentableDeclaredType,
    /// `kt_inline_lock_local`: the Kotlin inliner does not copy the value of a
    /// local a monitor locks into its uses where it may be another object.
    KtInlineLockLocal,
    /// `synchronized_lock_assignment`: only the assignment right before a
    /// `monitorenter` is taken for the compiler's lock variable.
    SynchronizedLockAssignment,
    /// `kt_facade_call`: in Kotlin output, a top-level function or property
    /// is called by its name with its import, not through its facade class.
    KtFacadeCall,
    /// `kt_extension_call`: in Kotlin output, an extension function or
    /// property is called on its receiver, not with it as an argument.
    KtExtensionCall,
    /// `kt_jvm_name_call`: in Kotlin output, a call is written with the
    /// function's Kotlin name, not its JVM name (`internal`, `@JvmName`).
    KtJvmNameCall,
    /// `kt_synthetic_accessor_call`: in Kotlin output, a call of a synthetic
    /// accessor (`access$getX$p`) is the access it makes.
    KtSyntheticAccessorCall,
    /// `kt_nullable_local`: a Kotlin local that a nullable value is stored
    /// into is declared with a nullable type.
    KtNullableLocal,
    /// `kt_field_read_reuse`: in Kotlin output, a field value the bytecode
    /// reads once and uses twice is not read again at each use.
    KtFieldReadReuse,
    /// `kt_partial_inline`: in Kotlin output, a variable that is not inlined
    /// into every use is read at each of them.
    KtPartialInline,
    /// `kt_file_static_initializer`: in Kotlin output, what is left of a file
    /// class's static initializer is written as its properties' initializers.
    KtFileStaticInitializer,
    /// `kt_nullable_cast`: in Kotlin output, the cast of a value whose
    /// Kotlin type is nullable is written `as T?`.
    KtNullableCast,    /// `enum_constant_in_values_array`: enum constants constructed inside
    /// the `$VALUES` array initializer (kotlinc 1.3) keep their constructor
    /// arguments.
    EnumConstantInValuesArray,
    /// `kt_extension_property`: in Kotlin output, an extension property is
    /// declared with its receiver and type parameters, and its accessors
    /// write the receiver `this`.
    KtExtensionProperty,
    /// `kt_interface_companion_field`: in Kotlin output, an interface's
    /// companion field is hidden like a class's.
    KtInterfaceCompanionField,
    /// `kt_generated_overload_hidden`: in Kotlin output, the overloads
    /// kotlinc generates for an `@JvmOverloads` function are not written.
    KtGeneratedOverloadHidden,
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
        KunaFix::CapturedVarClashName,
        KunaFix::CapturedVarThroughCast,
        KunaFix::KtLegacyConstructorParameterChecks,
        KunaFix::KtHiddenEmptySuperCall,
        KunaFix::KtCompoundBitwiseAssignment,
        KunaFix::KtSwitchHeadIndent,
        KunaFix::KtOperatorPrecedence,
        KunaFix::KtNarrowBitwiseOperand,
        KunaFix::KtReferenceInequality,
        KunaFix::HandlerCopySharedBody,
        KunaFix::KtFieldWithoutProperty,
        KunaFix::KtConstructorWithoutMetadata,
        KunaFix::KtDelegationCallArguments,
        KunaFix::KtCopiedEquality,
        KunaFix::KtBinaryNumericPromotion,
        KunaFix::KtNegatedEquality,
        KunaFix::KtOperatorLineBreak,
        KunaFix::KtUnboxingCastType,
        KunaFix::TwrDesugaringShape,
        KunaFix::TwrUnresolvedResource,
        KunaFix::CompoundAssignmentShape,
        KunaFix::TwrClassVersion,
        KunaFix::ImplicitOuterInstance,
        KunaFix::VarargArrayLength,
        KunaFix::SyntheticNullCheckShape,
        KunaFix::VarMergeDeclaredType,
        KunaFix::CompactRecordConstructor,
        KunaFix::DisjointGenericCast,
        KunaFix::ThrownTypeVariable,
        KunaFix::KtJavaKeywordVariableName,
        KunaFix::KtDefaultAccessorFlags,
        KunaFix::KtUnescapedName,
        KunaFix::KtReservedUnderscoreName,
        KunaFix::KtJavaTypeSyntax,
        KunaFix::FieldReadAllPaths,
        KunaFix::KtInlineEvaluationOrder,
        KunaFix::QualifiedSuperCall,
        KunaFix::ExternalInheritedMemberType,
        KunaFix::LambdaReturnTypeArgument,
        KunaFix::TypeAnnotationOuterArguments,
        KunaFix::KtAccessorBackingField,
        KunaFix::KtStringLiteralEscape,
        KunaFix::KtAbstractFunctionBody,
        KunaFix::KtMisplacedModality,
        KunaFix::KtCompanionPropertyInitializer,
        KunaFix::KtDefaultPropertyInitializer,
        KunaFix::KtMissedOverride,
        KunaFix::KtOuterThis,
        KunaFix::KtExtensionReceiver,
        KunaFix::KtNestedAnnotationArgument,
        KunaFix::KtThrowsClassName,
        KunaFix::KtThrowableCast,
        KunaFix::KtMappedJavaMember,
        KunaFix::KtMappedJavaType,
        KunaFix::KtObjectInstance,
        KunaFix::KtSuperPropertyAccess,
        KunaFix::KtSetterParameterName,
        KunaFix::KtJavaClassLiteral,
        KunaFix::PatternBindingShape,
        KunaFix::FinallyExitVersions,
        KunaFix::VarMergeLostDefinition,
        KunaFix::HeadDefinitionScope,
        KunaFix::SwitchCaseOrder,
        KunaFix::SwitchTableFillers,
        KunaFix::SwitchSyntheticDefault,
        KunaFix::VarargSpreadResolution,
        KunaFix::UnseenClassOverload,
        KunaFix::DoubleConstantLiteral,
        KunaFix::ConcatOperandType,
        KunaFix::FieldReadReuse,
        KunaFix::OverlappingRanges,
        KunaFix::ConstructorSignatureAlignment,
        KunaFix::SplitConstructorCall,
        KunaFix::LocalFinalWrites,
        KunaFix::KtNotNullAssertion,
        KunaFix::KtExpressionValueCheck,
        KunaFix::KtStaticInitializer,
        KunaFix::KtObjectInstanceLocal,
        KunaFix::KtPropertyInitializerKey,
        KunaFix::KtSynchronizedAccessor,
        KunaFix::InlineMarkerFinallyCopy,
        KunaFix::SynchronizedBreakExit,
        KunaFix::KtUnrepresentableDeclaredType,
        KunaFix::KtInlineLockLocal,
        KunaFix::SynchronizedLockAssignment,
        KunaFix::KtFacadeCall,
        KunaFix::KtExtensionCall,
        KunaFix::KtJvmNameCall,
        KunaFix::KtSyntheticAccessorCall,
        KunaFix::KtNullableLocal,
        KunaFix::KtFieldReadReuse,
        KunaFix::KtPartialInline,
        KunaFix::KtFileStaticInitializer,
        KunaFix::KtNullableCast,
        KunaFix::EnumConstantInValuesArray,
        KunaFix::KtExtensionProperty,
        KunaFix::KtInterfaceCompanionField,
        KunaFix::KtGeneratedOverloadHidden,
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
            KunaFix::CapturedVarClashName => "captured_var_clash_name",
            KunaFix::CapturedVarThroughCast => "captured_var_through_cast",
            KunaFix::KtLegacyConstructorParameterChecks => "kt_legacy_constructor_parameter_checks",
            KunaFix::KtHiddenEmptySuperCall => "kt_hidden_empty_super_call",
            KunaFix::KtCompoundBitwiseAssignment => "kt_compound_bitwise_assignment",
            KunaFix::KtSwitchHeadIndent => "kt_switch_head_indent",
            KunaFix::KtOperatorPrecedence => "kt_operator_precedence",
            KunaFix::KtNarrowBitwiseOperand => "kt_narrow_bitwise_operand",
            KunaFix::KtReferenceInequality => "kt_reference_inequality",
            KunaFix::HandlerCopySharedBody => "handler_copy_shared_body",
            KunaFix::KtFieldWithoutProperty => "kt_field_without_property",
            KunaFix::KtConstructorWithoutMetadata => "kt_constructor_without_metadata",
            KunaFix::KtDelegationCallArguments => "kt_delegation_call_arguments",
            KunaFix::KtCopiedEquality => "kt_copied_equality",
            KunaFix::KtBinaryNumericPromotion => "kt_binary_numeric_promotion",
            KunaFix::KtNegatedEquality => "kt_negated_equality",
            KunaFix::KtOperatorLineBreak => "kt_operator_line_break",
            KunaFix::KtUnboxingCastType => "kt_unboxing_cast_type",
            KunaFix::TwrDesugaringShape => "twr_desugaring_shape",
            KunaFix::TwrUnresolvedResource => "twr_unresolved_resource",
            KunaFix::CompoundAssignmentShape => "compound_assignment_shape",
            KunaFix::TwrClassVersion => "twr_class_version",
            KunaFix::ImplicitOuterInstance => "implicit_outer_instance",
            KunaFix::VarargArrayLength => "vararg_array_length",
            KunaFix::SyntheticNullCheckShape => "synthetic_null_check_shape",
            KunaFix::VarMergeDeclaredType => "var_merge_declared_type",
            KunaFix::CompactRecordConstructor => "compact_record_constructor",
            KunaFix::DisjointGenericCast => "disjoint_generic_cast",
            KunaFix::ThrownTypeVariable => "thrown_type_variable",
            KunaFix::KtJavaKeywordVariableName => "kt_java_keyword_variable_name",
            KunaFix::KtDefaultAccessorFlags => "kt_default_accessor_flags",
            KunaFix::KtUnescapedName => "kt_unescaped_name",
            KunaFix::KtReservedUnderscoreName => "kt_reserved_underscore_name",
            KunaFix::KtJavaTypeSyntax => "kt_java_type_syntax",
            KunaFix::FieldReadAllPaths => "field_read_all_paths",
            KunaFix::KtInlineEvaluationOrder => "kt_inline_evaluation_order",
            KunaFix::QualifiedSuperCall => "qualified_super_call",
            KunaFix::ExternalInheritedMemberType => "external_inherited_member_type",
            KunaFix::LambdaReturnTypeArgument => "lambda_return_type_argument",
            KunaFix::TypeAnnotationOuterArguments => "type_annotation_outer_arguments",
            KunaFix::KtAccessorBackingField => "kt_accessor_backing_field",
            KunaFix::KtStringLiteralEscape => "kt_string_literal_escape",
            KunaFix::KtAbstractFunctionBody => "kt_abstract_function_body",
            KunaFix::KtMisplacedModality => "kt_misplaced_modality",
            KunaFix::KtCompanionPropertyInitializer => "kt_companion_property_initializer",
            KunaFix::KtDefaultPropertyInitializer => "kt_default_property_initializer",
            KunaFix::KtMissedOverride => "kt_missed_override",
            KunaFix::KtOuterThis => "kt_outer_this",
            KunaFix::KtExtensionReceiver => "kt_extension_receiver",
            KunaFix::KtNestedAnnotationArgument => "kt_nested_annotation_argument",
            KunaFix::KtThrowsClassName => "kt_throws_class_name",
            KunaFix::KtThrowableCast => "kt_throwable_cast",
            KunaFix::KtMappedJavaMember => "kt_mapped_java_member",
            KunaFix::KtMappedJavaType => "kt_mapped_java_type",
            KunaFix::KtObjectInstance => "kt_object_instance",
            KunaFix::KtSuperPropertyAccess => "kt_super_property_access",
            KunaFix::KtSetterParameterName => "kt_setter_parameter_name",
            KunaFix::KtJavaClassLiteral => "kt_java_class_literal",
            KunaFix::PatternBindingShape => "pattern_binding_shape",
            KunaFix::FinallyExitVersions => "finally_exit_versions",
            KunaFix::VarMergeLostDefinition => "var_merge_lost_definition",
            KunaFix::HeadDefinitionScope => "head_definition_scope",
            KunaFix::SwitchCaseOrder => "switch_case_order",
            KunaFix::SwitchTableFillers => "switch_table_fillers",
            KunaFix::SwitchSyntheticDefault => "switch_synthetic_default",
            KunaFix::VarargSpreadResolution => "vararg_spread_resolution",
            KunaFix::UnseenClassOverload => "unseen_class_overload",
            KunaFix::DoubleConstantLiteral => "double_constant_literal",
            KunaFix::ConcatOperandType => "concat_operand_type",
            KunaFix::FieldReadReuse => "field_read_reuse",
            KunaFix::OverlappingRanges => "overlapping_ranges",
            KunaFix::ConstructorSignatureAlignment => "constructor_signature_alignment",
            KunaFix::SplitConstructorCall => "split_constructor_call",
            KunaFix::LocalFinalWrites => "local_final_writes",
            KunaFix::KtNotNullAssertion => "kt_not_null_assertion",
            KunaFix::KtExpressionValueCheck => "kt_expression_value_check",
            KunaFix::KtStaticInitializer => "kt_static_initializer",
            KunaFix::KtObjectInstanceLocal => "kt_object_instance_local",
            KunaFix::KtPropertyInitializerKey => "kt_property_initializer_key",
            KunaFix::KtSynchronizedAccessor => "kt_synchronized_accessor",
            KunaFix::InlineMarkerFinallyCopy => "inline_marker_finally_copy",
            KunaFix::SynchronizedBreakExit => "synchronized_break_exit",
            KunaFix::KtUnrepresentableDeclaredType => "kt_unrepresentable_declared_type",
            KunaFix::KtInlineLockLocal => "kt_inline_lock_local",
            KunaFix::SynchronizedLockAssignment => "synchronized_lock_assignment",
            KunaFix::KtFacadeCall => "kt_facade_call",
            KunaFix::KtExtensionCall => "kt_extension_call",
            KunaFix::KtJvmNameCall => "kt_jvm_name_call",
            KunaFix::KtSyntheticAccessorCall => "kt_synthetic_accessor_call",
            KunaFix::KtNullableLocal => "kt_nullable_local",
            KunaFix::KtFieldReadReuse => "kt_field_read_reuse",
            KunaFix::KtPartialInline => "kt_partial_inline",
            KunaFix::KtFileStaticInitializer => "kt_file_static_initializer",
            KunaFix::KtNullableCast => "kt_nullable_cast",
            KunaFix::EnumConstantInValuesArray => "enum_constant_in_values_array",
            KunaFix::KtExtensionProperty => "kt_extension_property",
            KunaFix::KtInterfaceCompanionField => "kt_interface_companion_field",
            KunaFix::KtGeneratedOverloadHidden => "kt_generated_overload_hidden",
        }
    }

    /// The fix whose id is `id`.
    pub fn from_id(id: &str) -> Option<KunaFix> {
        KunaFix::ALL.iter().copied().find(|f| f.id() == id)
    }

    const fn word_bit(self) -> (usize, u64) {
        let i = self as usize;
        (i / 64, 1 << (i % 64))
    }
}

const FIX_WORDS: usize = 4;

/// A set of active fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KunaFixes([u64; FIX_WORDS]);

const _: () = assert!(KunaFix::ALL.len() <= FIX_WORDS * 64);

impl KunaFixes {
    /// Every fix active: the engine's default.
    pub const DEFAULT: KunaFixes = KunaFixes([u64::MAX; FIX_WORDS]);
    /// No fix active: Vineflower's behaviour.
    pub const COMPAT: KunaFixes = KunaFixes([0; FIX_WORDS]);

    pub fn contains(self, fix: KunaFix) -> bool {
        let (w, b) = fix.word_bit();
        self.0[w] & b != 0
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
        let (w, b) = fix.word_bit();
        let mut words = self.0;
        words[w] &= !b;
        KunaFixes(words)
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
