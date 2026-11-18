//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/Pass.java`.

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
