//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/NamedPass.java`.

use std::rc::Rc;

use crate::pass::Pass;
use crate::pass_context::PassContext;

#[derive(Clone)]
pub struct NamedPass {
    name: String,
    pass: Rc<dyn Pass>,
}

impl NamedPass {
    pub fn new(name: &str, pass: Rc<dyn Pass>) -> NamedPass {
        NamedPass {
            name: name.to_string(),
            pass,
        }
    }

    pub fn of(name: &str, pass: Rc<dyn Pass>) -> NamedPass {
        NamedPass::new(name, pass)
    }
}

impl Pass for NamedPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let res = self.pass.run(ctx);

        if res {
            ctx.get_rec().add(&self.name, ctx.get_root());
        }

        res
    }
}
