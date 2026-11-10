//! kuna fix `kt_legacy_constructor_parameter_checks`: Kotlin 1.3's parameter
//! null checks before a constructor's delegation call are dropped.
//!
//! kotlinc checks each non-null reference parameter of a non-private
//! function on entry, before a constructor's `super(..)`/`this(..)` call.
//! `ResugarKotlinMethodsPass` drops the check under its Kotlin 1.4 name
//! (`Intrinsics.checkNotNullParameter`), since the parameter's non-null type
//! says it; Kotlin 1.3 and earlier call `Intrinsics.checkParameterIsNotNull`,
//! which it keeps. In a constructor the kept checks come before the
//! delegation call, so `InitializerProcessor` and `KConstructor` do not find
//! the call first: the primary constructor's header loses it and `super(..)`
//! is written as a statement of an `init` block, and a secondary
//! constructor's first check is written as its delegation call. Here the
//! checks of parameters that stand before the delegation call (with only
//! kotlinc's stores of synthetic fields among them) are dropped, as the pass
//! drops Kotlin 1.4's.

use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::pass_context::PassContext;
use crate::statements::Statements;
use crate::var_version_pair::VarVersionPair;

const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const LEGACY_CHECK: &str = "checkParameterIsNotNull";
const CHECK_DESCRIPTOR: &str = "(Ljava/lang/Object;Ljava/lang/String;)V";

/// Drops the legacy checks before the delegation call of the constructor
/// `ctx` decompiles; whether it dropped any.
pub fn remove(ctx: &PassContext) -> bool {
    let method = ctx.get_method();
    if method.get_name() != "<init>" {
        return false;
    }
    let Some(first) = ctx.get_root().and_then(|root| Statements::find_first_data(root.0)) else {
        return false;
    };
    let Some(exprents) = first.get_exprents() else {
        return false;
    };
    let parameter_slots: i32 = ctx
        .get_method_descriptor()
        .params
        .iter()
        .map(|p| p.stack_size)
        .sum();
    let this_vars = ctx.get_var_proc().get_this_vars();
    let is_this = |e: Option<Exprent>| {
        e.and_then(|e| e.try_as_var())
            .is_some_and(|v| this_vars.borrow().contains_key(&VarVersionPair::from_var(v)))
    };

    let mut checks = Vec::new();
    for (i, exprent) in exprents.to_vec().into_iter().enumerate() {
        if is_legacy_check(exprent, parameter_slots) {
            checks.push(i);
        } else if let Some(inv) = exprent.try_as_invocation() {
            if inv.get_name() != "<init>" || !is_this(inv.get_instance()) {
                return false;
            }
            for &i in checks.iter().rev() {
                exprents.remove_at(i);
            }
            return !checks.is_empty();
        } else if !exprent.try_as_assignment().is_some_and(|a| {
            a.get_right().try_as_var().is_some()
                && a.get_left().try_as_field().is_some_and(|f| is_this(f.get_instance()))
        }) {
            return false;
        }
    }
    false
}

/// `Intrinsics.checkParameterIsNotNull(p, "..")` of a parameter `p`.
fn is_legacy_check(exprent: Exprent, parameter_slots: i32) -> bool {
    let Some(inv) = exprent.try_as_invocation() else {
        return false;
    };
    if !inv.is_static()
        || inv.get_classname() != INTRINSICS
        || inv.get_name() != LEGACY_CHECK
        || inv.get_string_descriptor() != CHECK_DESCRIPTOR
    {
        return false;
    }
    let params = inv.get_lst_parameters().to_vec();
    params.len() == 2
        && params[0]
            .try_as_var()
            .is_some_and(|v| v.get_index() >= 1 && v.get_index() <= parameter_slots)
        && params[1]
            .try_as_const()
            .is_some_and(|c| matches!(c.get_value(), Some(ConstValue::String(_))))
}
