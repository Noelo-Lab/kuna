//! What several Kotlin fix families share: whether the class being written is
//! Kotlin output, kotlinc's runtime names, a walk of a statement tree's exprents
//! and the argument list of a call written after its receiver.

use crate::const_exprent::ConstValue;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::plugin_context::PluginContext;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;

/// Whether the class being written is written as Kotlin.
pub(crate) fn writing_kotlin() -> bool {
    DecompilerContext::get_current_class().is_some_and(|cl| {
        PluginContext::get_current_context()
            .get_language_spec(cl)
            .is_some()
    })
}

pub(crate) const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
/// `Intrinsics`' parameter checks, by their Kotlin 1.4 and earlier names.
pub(crate) const PARAMETER_CHECKS: [&str; 2] = ["checkNotNullParameter", "checkParameterIsNotNull"];
/// `Intrinsics`' checks of a platform-typed value, by their Kotlin 1.4 and earlier names.
pub(crate) const EXPRESSION_CHECKS: [&str; 2] = ["checkNotNullExpressionValue", "checkExpressionValueIsNotNull"];
/// The descriptor of an `Intrinsics` check that names what it checks.
pub(crate) const CHECK_DESCRIPTOR: &str = "(Ljava/lang/Object;Ljava/lang/String;)V";
/// The end of a suspend function's descriptor: its continuation, then its `Object` result.
pub(crate) const SUSPEND_DESCRIPTOR_END: &str = "Lkotlin/coroutines/Continuation;)Ljava/lang/Object;";

/// Calls `f` with each exprent of `stat`'s tree that no exprent holds: a statement's
/// variable definitions, head and block exprents, then its children's.
pub(crate) fn for_each_exprent(stat: Statement, f: &mut dyn FnMut(Exprent)) {
    let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
    lists.extend(stat.get_stat_exprents());
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    for e in lists {
        f(e);
    }
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        for_each_exprent(child, f);
    }
}

/// The value of `expr` when it is an `int` constant.
pub(crate) fn int_constant(expr: Exprent) -> Option<i32> {
    match expr.try_as_const()?.get_value()? {
        ConstValue::Int(v) => Some(v),
        _ => None,
    }
}

/// `inv`'s argument list without its first argument, which the caller wrote as
/// the receiver; the skip is undone however the write ends.
pub(crate) fn args_after_receiver(inv: InvocationExprent, indent: i32) -> TextBuffer {
    struct Restore(InvocationExprent, Option<usize>);
    impl Drop for Restore {
        fn drop(&mut self) {
            self.0.set_kuna_skip_arg(self.1);
        }
    }
    let _restore = Restore(inv, inv.d(|d| d.kuna_skip_arg));
    inv.set_kuna_skip_arg(Some(0));
    inv.append_param_list(indent)
}
