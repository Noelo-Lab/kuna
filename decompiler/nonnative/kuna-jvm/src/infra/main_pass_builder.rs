//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/MainPassBuilder.java`.

use std::rc::Rc;

use crate::named_pass::NamedPass;
use crate::pass::Pass;
use crate::pass_context::PassContext;

#[derive(Default)]
pub struct MainPassBuilder {
    passes: Vec<Rc<dyn Pass>>,
}

impl MainPassBuilder {
    pub fn new() -> MainPassBuilder {
        MainPassBuilder { passes: Vec::new() }
    }

    /// Java: addPass(String, Pass)
    pub fn add_pass(&mut self, name: &str, pass: Rc<dyn Pass>) -> &mut MainPassBuilder {
        self.passes.push(Rc::new(NamedPass::new(name, pass)));
        self
    }

    /// Java: addPass(NamedPass)
    pub fn add_pass_named(&mut self, pass: NamedPass) -> &mut MainPassBuilder {
        self.passes.push(Rc::new(pass));
        self
    }

    pub fn build(&self) -> Rc<dyn Pass> {
        Rc::new(CompiledPass::new(&self.passes))
    }
}

struct CompiledPass {
    passes: Vec<Rc<dyn Pass>>,
}

impl CompiledPass {
    fn new(passes: &[Rc<dyn Pass>]) -> CompiledPass {
        CompiledPass {
            passes: passes.to_vec(),
        }
    }
}

impl Pass for CompiledPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let mut res = false;
        for pass in &self.passes {
            res |= pass.run(ctx);
        }

        res
    }
}
