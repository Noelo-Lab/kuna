//! Port of `org/vineflower/kotlin/pass/ResugarKotlinMethodsPass.java`. The
//! static `MatchEngine`s are per-thread (`NONNULL_INTRINSICS.with(..)`).
//!
//! kuna fix `kt_legacy_constructor_parameter_checks`: a constructor's Kotlin
//! 1.3 parameter checks before its delegation call are dropped too
//! (`kuna_fix_kt_legacy_constructor_parameter_checks`).
//!
//! kuna fix `kt_legacy_parameter_checks`: so are the Kotlin 1.3 checks that
//! open a function's body (`kuna_fix_kt_legacy_parameter_checks`).
//!
//! kuna fix `kt_delegation_call_arguments`: first, the stack variables a
//! constructor computes its delegation call's arguments in are put back
//! into the call (`kuna_fix_kt_delegation_call_arguments`).

use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::jthrow;
use crate::kotlin::kuna_fix_kt_delegation_call_arguments;
use crate::kotlin::kuna_fix_kt_legacy_constructor_parameter_checks;
use crate::kotlin::kuna_fix_kt_legacy_parameter_checks;
use crate::kuna_fixes::{self, KunaFix};
use crate::k_function_exprent::{KFunctionExprent, KFunctionType};
use crate::match_engine::MatchEngine;
use crate::match_node::MatchValue;
use crate::pass::Pass;
use crate::pass_context::PassContext;

pub struct ResugarKotlinMethodsPass;

/// `ResugarKotlinMethodsPass.ResugarRes`.
struct ResugarRes {
    expr: Option<Exprent>,
    remove: bool,
}

impl ResugarRes {
    /// Java: ResugarRes(Exprent)
    fn new(expr: Option<Exprent>) -> ResugarRes {
        ResugarRes::new_remove(expr, false)
    }

    /// Java: ResugarRes(Exprent, boolean)
    fn new_remove(expr: Option<Exprent>, remove: bool) -> ResugarRes {
        ResugarRes { expr, remove }
    }
}

const NONNULL_INTRINSICS_DESC: [&str; 4] = [
    "exprent type:invocation invclass:kotlin/jvm/internal/Intrinsics signature:checkNotNullParameter(Ljava/lang/Object;Ljava/lang/String;)V",
    "exprent type:invocation invclass:kotlin/jvm/internal/Intrinsics signature:checkNotNullExpressionValue(Ljava/lang/Object;Ljava/lang/String;)V",
    "exprent type:invocation invclass:kotlin/jvm/internal/Intrinsics signature:checkNotNull(Ljava/lang/Object;Ljava/lang/String;)V",
    "exprent type:invocation invclass:kotlin/jvm/internal/Intrinsics signature:checkNotNull(Ljava/lang/Object;)V",
];

const EQUAL_INTRINSIC_DESC: [&str; 1] = [
    "exprent type:invocation invclass:kotlin/jvm/internal/Intrinsics name:areEqual parameter:0:$lhs$ parameter:1:$rhs$",
];

const TERNARY_NULL_CHECK_DESC: [&str; 6] = [
    "exprent type:function functype:ternary",
    " exprent position:1 ret:$x1$",
    " exprent position:2 ret:$y$",
    " exprent position:0 type:function functype:neq",
    "  exprent ret:$x$",
    "  exprent type:constant consttype:null",
];

const GET_KCLASS_DESC: [&str; 1] = [
    "exprent type:invocation invclass:kotlin/jvm/internal/Reflection name:getOrCreateKotlinClass parameter:0:$class$",
];

thread_local! {
    static NONNULL_INTRINSICS: [MatchEngine; 4] =
        NONNULL_INTRINSICS_DESC.map(|description| MatchEngine::new(&[description]));
    static EQUAL_INTRINSIC: MatchEngine = MatchEngine::new(&EQUAL_INTRINSIC_DESC);
    static TERNARY_NULL_CHECK: MatchEngine = MatchEngine::new(&TERNARY_NULL_CHECK_DESC);
    static GET_KCLASS: MatchEngine = MatchEngine::new(&GET_KCLASS_DESC);
}

/// `(Exprent) engine.getVariableValue(name)`.
fn exprent_value(engine: &MatchEngine, name: &str) -> Option<Exprent> {
    match engine.get_variable_value(name) {
        None => None,
        Some(MatchValue::Exprent(expr)) => Some(expr),
        Some(_) => jthrow!(ClassCastException; "variable {} is not an Exprent", name),
    }
}

/// An element of `List.of(..)`, which rejects `null`.
fn list_of_element(expr: Option<Exprent>) -> Exprent {
    expr.unwrap_or_else(|| jthrow!(NullPointerException; "List.of element is null"))
}

impl Pass for ResugarKotlinMethodsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let mut res = false;

        if kuna_fixes::enabled(KunaFix::KtDelegationCallArguments) {
            res |= kuna_fix_kt_delegation_call_arguments::inline(ctx);
        }

        if kuna_fixes::enabled(KunaFix::KtLegacyConstructorParameterChecks) {
            res |= kuna_fix_kt_legacy_constructor_parameter_checks::remove(ctx);
        }

        if kuna_fixes::enabled(KunaFix::KtLegacyParameterChecks) {
            res |= kuna_fix_kt_legacy_parameter_checks::remove(ctx);
        }

        let digraph =
            FlattenStatementsHelper::build(ctx.get_root().expect("NullPointerException: root"));

        for nd in digraph.nodes() {
            let exprs = nd.with(|d| d.exprents.clone());
            for ex in exprs.iter() {
                res |= ResugarKotlinMethodsPass::resugar_exprs(ex);
            }

            let mut i: i32 = 0;
            while (i as usize) < exprs.len() {
                let expr = exprs.get(i as usize);

                let expr_res = ResugarKotlinMethodsPass::resugar_expr(expr);
                if expr_res.remove {
                    exprs.remove_at(i as usize);
                    i -= 1;
                    res = true;
                } else if let Some(replacement) = expr_res.expr {
                    exprs.set(i as usize, replacement);
                    res = true;
                }
                i += 1;
            }
        }

        res
    }
}

impl ResugarKotlinMethodsPass {
    fn resugar_exprs(expr: Exprent) -> bool {
        let mut res = false;

        for ex in expr.get_all_exprents() {
            res |= ResugarKotlinMethodsPass::resugar_exprs(ex);

            let map = ResugarKotlinMethodsPass::resugar_expr(ex).expr;

            if let Some(map) = map {
                expr.replace_exprent(ex, map);
                res = true;
            }
        }

        res
    }

    fn resugar_expr(ex: Exprent) -> ResugarRes {
        if NONNULL_INTRINSICS.with(|engines| engines.iter().any(|engine| engine.match_(&ex))) {
            return ResugarRes::new_remove(None, true);
        }

        let equal = EQUAL_INTRINSIC.with(|engine| {
            engine.match_(&ex).then(|| {
                (
                    exprent_value(engine, "$lhs$"),
                    exprent_value(engine, "$rhs$"),
                )
            })
        });
        if let Some((lhs, rhs)) = equal {
            let operands = vec![list_of_element(lhs), list_of_element(rhs)];
            return ResugarRes::new_remove(
                Some(
                    KFunctionExprent::new_func_type(FunctionType::Eq, operands, None)
                        .0
                         .0,
                ),
                false,
            );
        }

        let ternary = TERNARY_NULL_CHECK.with(|engine| {
            engine.match_(&ex).then(|| {
                (
                    exprent_value(engine, "$x1$"),
                    engine.get_variable_value("$x$"),
                )
            })
        });
        if let Some((x1, x)) = ternary {
            let mut inner_val = x1;
            if let Some(inv) = inner_val.and_then(|v| v.try_as_invocation()) {
                if inv.is_unboxing_call() {
                    inner_val = inv.get_instance();
                }
            }

            if let Some(inner_val) = inner_val.filter(|v| v.is_var()) {
                let x_equals = match &x {
                    None => jthrow!(NullPointerException; "$x$ is null"),
                    Some(MatchValue::Exprent(x)) => x.equals(inner_val),
                    Some(_) => false,
                };
                if x_equals {
                    let (x, y) = TERNARY_NULL_CHECK.with(|engine| {
                        (exprent_value(engine, "$x$"), exprent_value(engine, "$y$"))
                    });
                    let operands = vec![list_of_element(x), list_of_element(y)];
                    return ResugarRes::new_remove(
                        Some(
                            KFunctionExprent::new_k_type(KFunctionType::IfNull, operands, None)
                                .0
                                 .0,
                        ),
                        false,
                    );
                }
            }
        }

        let kclass =
            GET_KCLASS.with(|engine| engine.match_(&ex).then(|| exprent_value(engine, "$class$")));
        if let Some(class) = kclass {
            let operands = vec![list_of_element(class)];
            return ResugarRes::new_remove(
                Some(
                    KFunctionExprent::new_k_type(KFunctionType::GetKclass, operands, None)
                        .0
                         .0,
                ),
                false,
            );
        }

        ResugarRes::new(None)
    }
}
