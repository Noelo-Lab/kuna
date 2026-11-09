//! Port of `org/vineflower/kotlin/pass/ReplaceStatsPass.java`.

#![allow(dead_code, unused_variables)]

use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;
use crate::unported;

pub struct ReplaceStatsPass;

impl Pass for ReplaceStatsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        ReplaceStatsPass::replace(ctx.get_root().expect("NullPointerException: root").0)
    }
}

impl ReplaceStatsPass {
    fn replace(stat: Statement) -> bool {
        unported!("ReplaceStatsPass.replace")
    }
}
