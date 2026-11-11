//! Port of `org/vineflower/kotlin/pass/ReplaceExprentsPass.java`.
//!
//! kuna fix `kt_default_args_call`: the calls are replaced knowing the method
//! they are in (`kuna_fix_kt_default_args_call::within`).

use crate::exprent::Exprent;
use crate::k_utils::KUtils;
use crate::k_var_exprent::{DeclarationType, KVarExprent};
use crate::kotlin::kuna_fix_kt_default_args_call;
use crate::kuna_fixes::{self, KunaFix};
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::rt::JList;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;

pub struct ReplaceExprentsPass;

/// One of the Java `List<Exprent>`s the pass rewrites: a body or head list,
/// or a list whose entries may be `null` (switch case values and guards).
enum ExprList {
    Plain(JList<Exprent>),
    Nullable(JList<Option<Exprent>>),
}

impl Pass for ReplaceExprentsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let root = ctx.get_root().expect("NullPointerException: root").0;
        if kuna_fixes::enabled(KunaFix::KtDefaultArgsCall) {
            kuna_fix_kt_default_args_call::within(ctx.get_method(), || {
                ReplaceExprentsPass::replace(root)
            })
        } else {
            ReplaceExprentsPass::replace(root)
        }
    }
}

impl ReplaceExprentsPass {
    /// `vars.set(i, new KVarExprent(vars.get(i)))` as an exception declaration.
    fn replace_catch_vars(vars: &JList<VarExprent>) {
        for i in 0..vars.len() {
            let expr = vars.get(i);
            let map = KVarExprent::new(expr);
            map.set_declaration_type(Some(DeclarationType::ExceptionType));
            vars.set(i, map.0);
        }
    }

    /// Java: replace(Statement)
    fn replace(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= ReplaceExprentsPass::replace(st);
        }

        let mut expr_lists: Vec<Option<ExprList>> = Vec::new();
        expr_lists.push(stat.get_exprents().map(ExprList::Plain));
        expr_lists.push(Some(ExprList::Plain(stat.get_var_definitions())));

        if stat.is_catch_all() || stat.is_catch() {
            let vars = match stat.try_as_catch_all() {
                Some(catch_all) => catch_all.get_vars(),
                None => stat.as_catch().get_vars(),
            };
            ReplaceExprentsPass::replace_catch_vars(&vars);

            if let Some(catch_all) = stat
                .try_as_catch_all()
                .filter(|catch_all| catch_all.get_monitor().is_some())
            {
                let monitor = catch_all.get_monitor().expect("monitor");
                catch_all.set_monitor(Some(KVarExprent::new(monitor).0));
            } else if let Some(catch_stat) = stat.try_as_catch() {
                expr_lists.push(Some(ExprList::Plain(catch_stat.get_resources())));
            }
        } else if let Some(do_stat) = stat.try_as_do() {
            expr_lists.push(Some(ExprList::Plain(do_stat.get_init_exprent_list())));
            expr_lists.push(Some(ExprList::Plain(do_stat.get_condition_exprent_list())));
            expr_lists.push(Some(ExprList::Plain(do_stat.get_inc_exprent_list())));
        } else if let Some(if_stat) = stat.try_as_if() {
            expr_lists.push(Some(ExprList::Plain(if_stat.get_headexprent_list())));
        } else if let Some(switch_stat) = stat.try_as_switch() {
            for values in switch_stat.get_case_values().to_vec() {
                expr_lists.push(Some(ExprList::Nullable(values)));
            }
            expr_lists.push(Some(ExprList::Nullable(switch_stat.get_case_guards())));
            expr_lists.push(Some(ExprList::Plain(switch_stat.get_headexprent_list())));
        } else if let Some(sync_stat) = stat.try_as_synchronized() {
            expr_lists.push(Some(ExprList::Plain(sync_stat.get_headexprent_list())));
        } else if stat.is_catch_all() || stat.is_catch() {
            let vars = match stat.try_as_catch_all() {
                Some(catch_all) => catch_all.get_vars(),
                None => stat.as_catch().get_vars(),
            };
            ReplaceExprentsPass::replace_catch_vars(&vars);
        }

        for exprs in expr_lists {
            match exprs {
                None => continue,
                Some(ExprList::Plain(exprs)) => {
                    let mut i = 0;
                    while i < exprs.len() {
                        let expr = exprs.get(i);
                        if let Some(map) = KUtils::replace_exprent(expr) {
                            exprs.set(i, map);
                            res = true;
                        }
                        i += 1;
                    }

                    for ex in exprs.to_vec() {
                        res |= ReplaceExprentsPass::replace_exprent(ex);
                    }
                }
                Some(ExprList::Nullable(exprs)) => {
                    let mut i = 0;
                    while i < exprs.len() {
                        let expr = exprs.get(i);
                        if let Some(map) = expr.and_then(KUtils::replace_exprent) {
                            exprs.set(i, Some(map));
                            res = true;
                        }
                        i += 1;
                    }

                    for ex in exprs.to_vec() {
                        let Some(ex) = ex else {
                            continue;
                        };

                        res |= ReplaceExprentsPass::replace_exprent(ex);
                    }
                }
            }
        }

        res
    }

    /// Java: replace(Exprent)
    fn replace_exprent(expr: Exprent) -> bool {
        let mut res = false;

        for ex in expr.get_all_exprents() {
            res |= ReplaceExprentsPass::replace_exprent(ex);
            if let Some(map) = KUtils::replace_exprent(ex) {
                expr.replace_exprent(ex, map);
                res = true;
            }
        }

        res
    }
}
