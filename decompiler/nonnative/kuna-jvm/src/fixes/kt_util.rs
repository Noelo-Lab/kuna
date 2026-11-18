//! What several Kotlin fix families share: kotlinc's runtime names.

pub(crate) const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
/// `Intrinsics`' parameter checks, by their Kotlin 1.4 and earlier names.
pub(crate) const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];
/// `Intrinsics`' checks of a platform-typed value, by their Kotlin 1.4 and earlier names.
pub(crate) const EXPRESSION_CHECKS: [&str; 2] = ["checkNotNullExpressionValue", "checkExpressionValueIsNotNull"];
/// The descriptor of an `Intrinsics` check that names what it checks.
pub(crate) const CHECK_DESCRIPTOR: &str = "(Ljava/lang/Object;Ljava/lang/String;)V";
/// The end of a suspend function's descriptor: its continuation, then its `Object` result.
pub(crate) const SUSPEND_DESCRIPTOR_END: &str = "Lkotlin/coroutines/Continuation;)Ljava/lang/Object;";
