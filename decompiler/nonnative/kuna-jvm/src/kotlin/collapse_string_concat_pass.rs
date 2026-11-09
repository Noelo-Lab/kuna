//! Port of `org/vineflower/kotlin/pass/CollapseStringConcatPass.java`.

#![allow(dead_code, unused_variables)]

use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_function_exprent::{AnyFunctionType, KFunctionExprent, KFunctionType};
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;
use crate::unported;

pub struct CollapseStringConcatPass;

impl Pass for CollapseStringConcatPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        CollapseStringConcatPass::run_stat(ctx.get_root().expect("NullPointerException: root").0)
    }
}

impl CollapseStringConcatPass {
    /// Java: run(Statement)
    fn run_stat(stat: Statement) -> bool {
        unported!("CollapseStringConcatPass.run")
    }

    fn is_string_concat(kex: KFunctionExprent) -> bool {
        kex.get_any_function_type() == AnyFunctionType::Java(FunctionType::StrConcat)
            || kex.get_any_function_type() == AnyFunctionType::Kotlin(KFunctionType::StrTemplate)
    }

    /// Java: run(Exprent)
    fn run_exprent(ex: Exprent) -> bool {
        unported!("CollapseStringConcatPass.run")
    }
}
