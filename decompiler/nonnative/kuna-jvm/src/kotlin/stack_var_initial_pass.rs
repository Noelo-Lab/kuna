//! Port of `org/vineflower/kotlin/pass/StackVarInitialPass.java`.

#![allow(dead_code, unused_variables)]

use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::unported;

pub struct StackVarInitialPass;

impl Pass for StackVarInitialPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        unported!("StackVarInitialPass.run")
    }
}
