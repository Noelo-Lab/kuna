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
        }
    }

    /// The fix whose id is `id`.
    pub fn from_id(id: &str) -> Option<KunaFix> {
        KunaFix::ALL.iter().copied().find(|f| f.id() == id)
    }

    const fn bit(self) -> u64 {
        1 << (self as u32)
    }
}

/// A set of active fixes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KunaFixes(u64);

impl KunaFixes {
    /// Every fix active: the engine's default.
    pub const DEFAULT: KunaFixes = KunaFixes(u64::MAX);
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
