//! Port of `org/vineflower/kotlin/pass/JavaFinallyPass.java`.

#![allow(dead_code, unused_variables)]

use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::unported;

pub struct JavaFinallyPass;

impl Pass for JavaFinallyPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        unported!("JavaFinallyPass.run")
    }
}
