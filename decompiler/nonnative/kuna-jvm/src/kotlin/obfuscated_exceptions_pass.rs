//! Port of `org/vineflower/kotlin/pass/ObfuscatedExceptionsPass.java`.

#![allow(dead_code, unused_variables)]

use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::unported;

pub struct ObfuscatedExceptionsPass;

impl Pass for ObfuscatedExceptionsPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        unported!("ObfuscatedExceptionsPass.run")
    }
}
