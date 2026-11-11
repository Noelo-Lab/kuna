//! Port of `org/vineflower/kotlin/pass/ReplaceStatsPass.java`.

use crate::k_catch_statement::KCatchStatement;
use crate::k_do_statement::KDoStatement;
use crate::k_if_statement::KIfStatement;
use crate::k_sequence_statement::KSequenceStatement;
use crate::k_switch_statement::KSwitchStatement;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::statement::Statement;

pub struct ReplaceStatsPass;

impl Pass for ReplaceStatsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        ReplaceStatsPass::replace(ctx.get_root().expect("NullPointerException: root").0)
    }
}

impl ReplaceStatsPass {
    /// Java: private (public here for tests).
    pub fn replace(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats();
        let mut i = 0;
        while i < stats.borrow().len() {
            let st = stats.borrow().get(i);
            res |= ReplaceStatsPass::replace(st);
            if let Some(seq) = st.try_as_sequence() {
                st.replace_with(KSequenceStatement::new(seq).0 .0);
                res = true;
            } else if let Some(do_) = st.try_as_do() {
                st.replace_with(KDoStatement::new(do_).0 .0);
                res = true;
            } else if let Some(switch) = st.try_as_switch() {
                st.replace_with(KSwitchStatement::new(switch).0 .0);
                res = true;
            } else if let Some(if_) = st.try_as_if() {
                st.replace_with(KIfStatement::new(if_).0 .0);
                res = true;
            } else if let Some(catch) = st.try_as_catch() {
                st.replace_with(KCatchStatement::new(catch).0 .0);
                res = true;
            }
            i += 1;
        }

        res
    }
}
