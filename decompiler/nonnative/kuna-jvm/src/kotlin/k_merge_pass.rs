//! Port of `org/vineflower/kotlin/pass/KMergePass.java`. `KMergePass extends
//! MergeHelper` only to reach its statics: call `MergeHelper::…` for those.

#![allow(dead_code, unused_variables)]

use crate::do_statement::DoStatement;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;
use crate::unported;

pub struct KMergePass;

impl Pass for KMergePass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        unported!("KMergePass.run")
    }
}

impl KMergePass {
    fn enhance_loops_rec(stat: Statement) -> bool {
        unported!("KMergePass.enhanceLoopsRec")
    }

    fn enhance_loop(stat: DoStatement) -> bool {
        unported!("KMergePass.enhanceLoop")
    }

    /// Hides `MergeHelper.matchForEach`.
    pub fn match_for_each(stat: DoStatement) -> bool {
        unported!("KMergePass.matchForEach")
    }
}
