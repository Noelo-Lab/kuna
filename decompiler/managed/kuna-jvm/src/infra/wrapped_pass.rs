//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/WrappedPass.java`.

use std::rc::Rc;

use crate::pass::Pass;
use crate::pass_context::PassContext;

/// A pass built from a consumer; it always reports a change.
pub struct WrappedPass {
    pass: Box<dyn Fn(&mut PassContext)>,
}

impl WrappedPass {
    fn new(pass: Box<dyn Fn(&mut PassContext)>) -> WrappedPass {
        WrappedPass { pass }
    }

    pub fn of(pass: impl Fn(&mut PassContext) + 'static) -> Rc<dyn Pass> {
        Rc::new(WrappedPass::new(Box::new(pass)))
    }
}

impl Pass for WrappedPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        (self.pass)(ctx);

        true
    }
}
