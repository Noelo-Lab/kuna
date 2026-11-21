//! Port of `org/vineflower/kotlin/pass/JavaFinallyPass.java`.

use crate::cancelation_manager::CancelationManager;
use crate::dom_helper::DomHelper;
use crate::finally_processor::FinallyProcessor;
use crate::pass::Pass;
use crate::pass_context::PassContext;

pub struct JavaFinallyPass;

impl Pass for JavaFinallyPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let mut root = ctx.get_root().expect("NullPointerException: root");
        let mt = ctx.get_method();
        let cl = ctx.get_enclosing_class();
        let var_proc = ctx.get_var_proc();
        let graph = ctx.get_graph().expect("NullPointerException: graph");
        let md = ctx.get_method_descriptor();

        let mut f_proc = FinallyProcessor::new(mt, &md, var_proc);

        let mut res = false;
        let mut iteration = 0;
        while f_proc.iterate_graph(cl, mt, root, graph) {
            CancelationManager::check_canceled();
            let old_root = root;

            iteration += 1;
            root = DomHelper::parse_graph(graph, mt, iteration);
            root.add_comments(old_root);
            ctx.set_root(Some(root));

            res = true;
        }

        res
    }
}
