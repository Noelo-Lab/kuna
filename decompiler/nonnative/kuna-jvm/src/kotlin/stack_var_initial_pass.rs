//! Port of `org/vineflower/kotlin/pass/StackVarInitialPass.java`.

use crate::cancelation_manager::CancelationManager;
use crate::p_pand_mm_helper::PPandMMHelper;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::stack_vars_processor::StackVarsProcessor;

pub struct StackVarInitialPass;

impl Pass for StackVarInitialPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let root = ctx.get_root().expect("NullPointerException: root");
        let mt = ctx.get_method();
        let cl = ctx.get_enclosing_class();
        let var_proc = ctx.get_var_proc();

        loop {
            CancelationManager::check_canceled();
            StackVarsProcessor::simplify_stack_vars(root, mt, cl);

            var_proc.set_var_versions(root);

            if !PPandMMHelper::new(var_proc).find_p_pand_mm(root) {
                break;
            }
        }

        true
    }
}
