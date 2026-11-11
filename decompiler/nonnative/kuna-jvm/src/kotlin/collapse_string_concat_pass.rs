//! Port of `org/vineflower/kotlin/pass/CollapseStringConcatPass.java`.

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::jthrow;
use crate::k_function_exprent::{AnyFunctionType, KFunctionExprent, KFunctionType};
use crate::kotlin_options::KotlinOptions;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;

pub struct CollapseStringConcatPass;

impl Pass for CollapseStringConcatPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        CollapseStringConcatPass::run_stat(ctx.get_root().expect("NullPointerException: root").0)
    }
}

impl CollapseStringConcatPass {
    /// Java: run(Statement)
    fn run_stat(stat: Statement) -> bool {
        if !DecompilerContext::get_option(KotlinOptions::COLLAPSE_STRING_CONCATENATION) {
            return false;
        }

        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= CollapseStringConcatPass::run_stat(st);
        }

        let mut exprs: Vec<Exprent> = Vec::new();
        if stat.is_basic_block() {
            exprs = stat
                .get_exprents()
                .expect("NullPointerException: exprents")
                .to_vec();
        } else if let Some(if_stat) = stat.try_as_if() {
            exprs = if_stat.get_headexprent_list().to_vec();
        }

        for ex in exprs {
            res |= CollapseStringConcatPass::run_exprent(ex);
        }

        res
    }

    fn is_string_concat(kex: KFunctionExprent) -> bool {
        kex.get_any_function_type() == AnyFunctionType::Java(FunctionType::StrConcat)
            || kex.get_any_function_type() == AnyFunctionType::Kotlin(KFunctionType::StrTemplate)
    }

    /// Java: run(Exprent)
    fn run_exprent(ex: Exprent) -> bool {
        let mut res = false;

        for e in ex.get_all_exprents() {
            res |= CollapseStringConcatPass::run_exprent(e);
        }

        if let Some(kex) = ex
            .try_as_k_function()
            .filter(|&kex| CollapseStringConcatPass::is_string_concat(kex))
        {
            let operands = kex.get_lst_operands().to_vec();
            let lst_operands = kex.get_lst_operands();
            for child in operands {
                if let Some(child_kex) = child
                    .try_as_k_function()
                    .filter(|&child_kex| CollapseStringConcatPass::is_string_concat(child_kex))
                {
                    let index = lst_operands.index_of(&child);
                    if index < 0 {
                        jthrow!(IndexOutOfBoundsException; "Index: {}, Size: {}", index, lst_operands.len());
                    }
                    lst_operands.add_all_at(index as usize, child_kex.get_lst_operands().to_vec());
                    lst_operands.remove_item(&child);
                }
            }
            kex.set_func_type_typed(AnyFunctionType::Kotlin(KFunctionType::StrTemplate));
            res = true;
        }

        res
    }
}
