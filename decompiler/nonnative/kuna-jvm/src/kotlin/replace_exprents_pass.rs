//! Port of `org/vineflower/kotlin/pass/ReplaceExprentsPass.java`.

#![allow(dead_code, unused_variables)]

use crate::exprent::Exprent;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;
use crate::unported;

pub struct ReplaceExprentsPass;

impl Pass for ReplaceExprentsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        ReplaceExprentsPass::replace(ctx.get_root().expect("NullPointerException: root").0)
    }
}

impl ReplaceExprentsPass {
    /// Java: replace(Statement)
    fn replace(stat: Statement) -> bool {
        unported!("ReplaceExprentsPass.replace")
    }

    /// Java: replace(Exprent)
    fn replace_exprent(expr: Exprent) -> bool {
        unported!("ReplaceExprentsPass.replace")
    }
}
