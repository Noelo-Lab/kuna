//! Port of `org/vineflower/kotlin/pass/ResugarKotlinMethodsPass.java`. The
//! static `MatchEngine`s are per-thread (`NONNULL_INTRINSICS.with(..)`).

#![allow(dead_code, unused_variables)]

use crate::exprent::Exprent;
use crate::match_engine::MatchEngine;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::unported;

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

impl Pass for ResugarKotlinMethodsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        unported!("ResugarKotlinMethodsPass.run")
    }
}

impl ResugarKotlinMethodsPass {
    fn resugar_exprs(expr: Exprent) -> bool {
        unported!("ResugarKotlinMethodsPass.resugarExprs")
    }

    fn resugar_expr(ex: Exprent) -> ResugarRes {
        unported!("ResugarKotlinMethodsPass.resugarExpr")
    }
}
