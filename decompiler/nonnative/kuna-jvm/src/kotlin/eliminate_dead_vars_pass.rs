//! Port of `org/vineflower/kotlin/pass/EliminateDeadVarsPass.java`.

#![allow(dead_code, unused_variables)]

use crate::exprent::Exprent;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::root_statement::RootStatement;
use crate::struct_method::StructMethod;
use crate::unported;

pub struct EliminateDeadVarsPass;

impl Pass for EliminateDeadVarsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        EliminateDeadVarsPass::eliminate(
            ctx.get_root().expect("NullPointerException: root"),
            ctx.get_method(),
        )
    }
}

impl EliminateDeadVarsPass {
    fn eliminate(root: RootStatement, mt: StructMethod) -> bool {
        unported!("EliminateDeadVarsPass.eliminate")
    }

    fn is_pure_to_replace(expr: Exprent) -> bool {
        unported!("EliminateDeadVarsPass.isPureToReplace")
    }
}
