//! Port of `org/vineflower/kotlin/pass/EliminateDeadVarsPass.java`.
//!
//! kuna fix `kt_semaphore_store_liveness` (`kuna_fixes`): the stores to a
//! `finally` semaphore are live (the catch-all's monitor reads them).

use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::k_types::KTypes;
use crate::kotlin::kuna_fix_kt_semaphore_store_liveness as semaphore_fix;
use crate::kuna_fixes::{self, KunaFix};
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::root_statement::RootStatement;
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::stack_vars_processor::StackVarsProcessor;
use crate::struct_method::StructMethod;
use crate::var_processor::VarProcessor;

pub struct EliminateDeadVarsPass;

impl Pass for EliminateDeadVarsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        EliminateDeadVarsPass::eliminate(
            ctx.get_root().expect("NullPointerException: root"),
            ctx.get_method(),
            ctx.get_var_proc(),
        )
    }
}

impl EliminateDeadVarsPass {
    fn eliminate(root: RootStatement, mt: StructMethod, varproc: VarProcessor) -> bool {
        let monitors = kuna_fixes::enabled(KunaFix::KtSemaphoreStoreLiveness)
            .then(|| semaphore_fix::monitor_vars(root.0));
        let mut ssau = SSAUConstructorSparseEx::new();
        ssau.split_variables(root, mt);

        let digraph = ssau
            .get_direct_graph()
            .expect("NullPointerException: digraph");
        let ssu = ssau.get_ssu_versions();

        let mut changed_any = false;
        loop {
            let mut changed = false;

            for nd in digraph.nodes() {
                let exprents = nd.exprents();

                let mut i: i32 = 0;
                while (i as usize) < exprents.len() {
                    let ex = exprents.get(i as usize);

                    if let Some(aex) = ex.try_as_assignment() {
                        let left = aex.get_left();
                        let right = aex.get_right();

                        if let Some(var) = left.try_as_var() {
                            let vvp = var.get_var_version_pair();
                            if EliminateDeadVarsPass::is_pure_to_replace(right)
                                && !ssu
                                    .get_node_with_key(vvp)
                                    .expect("NullPointerException: ssu node")
                                    .has_any_successors()
                                && !monitors.as_ref().is_some_and(|monitors| {
                                    semaphore_fix::is_monitor_store(var, monitors, varproc)
                                })
                            {
                                exprents.remove_at(i as usize);
                                changed = true;
                                changed_any = true;

                                i -= 1;
                            }
                        }
                    }
                    i += 1;
                }
            }

            if !changed {
                break;
            }
        }

        StackVarsProcessor::set_versions_to_null(root.0);

        changed_any
    }

    fn is_pure_to_replace(expr: Exprent) -> bool {
        if let Some(con) = expr.try_as_const() {
            let value = con.get_value();
            if value.is_none()
                || (matches!(value, Some(ConstValue::Int(_))) && con.get_int_value() == 0)
            {
                return true;
            }
        }

        if let Some(field) = expr.try_as_field() {
            return field.is_static() && field.get_classname() == KTypes::UNIT;
        }

        false
    }
}
