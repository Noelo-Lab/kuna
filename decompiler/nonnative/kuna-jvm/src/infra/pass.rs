//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/Pass.java`.

use std::rc::Rc;

use crate::pass_context::PassContext;

/// A pass over the decompiled code; `run` returns whether it changed anything.
pub trait Pass {
    fn run(&self, ctx: &mut PassContext) -> bool;
}

impl<F: Fn(&mut PassContext) -> bool> Pass for F {
    fn run(&self, ctx: &mut PassContext) -> bool {
        self(ctx)
    }
}

/// `Pass.NO_OP`.
#[allow(non_snake_case)]
pub fn NO_OP() -> Rc<dyn Pass> {
    Rc::new(|_: &mut PassContext| false)
}
