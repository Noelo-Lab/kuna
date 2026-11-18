//! What several Kotlin fix families share: kotlinc's runtime names and
//! walks of a statement tree's exprents.

use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::statement::Statement;

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

/// Every exprent of `stat`'s tree, each after the exprents it holds; a statement's
/// children come before its head exprents, and variable definitions are left out.
pub(crate) fn all_exprents(stat: Statement) -> Vec<Exprent> {
    fn collect(stat: Statement, out: &mut Vec<Exprent>) {
        let top: Vec<Exprent> = match stat.get_exprents() {
            Some(exprents) => exprents.iter().collect(),
            None => {
                for st in stat.get_stats().borrow().iter() {
                    collect(*st, out);
                }
                stat.get_stat_exprents()
            }
        };
        for e in top {
            out.extend(e.get_all_exprents_rec(true));
            out.push(e);
        }
    }
    let mut out = Vec::new();
    collect(stat, &mut out);
    out
}

/// The value of `expr` when it is an `int` constant.
pub(crate) fn int_constant(expr: Exprent) -> Option<i32> {
    match expr.try_as_const()?.get_value()? {
        ConstValue::Int(v) => Some(v),
        _ => None,
    }
}
