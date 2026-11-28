//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/LoopingPassBuilder.java`.

use std::rc::Rc;

use crate::cancelation_manager::CancelationManager;
use crate::named_pass::NamedPass;
use crate::pass::Pass;
use crate::pass_context::PassContext;

pub struct LoopingPassBuilder {
    passes: Vec<(Rc<dyn Pass>, bool)>,
    name: String,
}

impl LoopingPassBuilder {
    pub fn new(name: &str) -> LoopingPassBuilder {
        LoopingPassBuilder {
            passes: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn add_fallthrough_pass(
        &mut self,
        name: &str,
        pass: Rc<dyn Pass>,
    ) -> &mut LoopingPassBuilder {
        self.passes
            .push((Rc::new(NamedPass::new(name, pass)), false));
        self
    }

    pub fn add_looping_pass(&mut self, name: &str, pass: Rc<dyn Pass>) -> &mut LoopingPassBuilder {
        self.passes
            .push((Rc::new(NamedPass::new(name, pass)), true));
        self
    }

    pub fn build(&self) -> Rc<dyn Pass> {
        Rc::new(CompiledPass::new(&self.name, &self.passes))
    }
}

struct CompiledPass {
    passes: Vec<(Rc<dyn Pass>, bool)>,
}

impl CompiledPass {
    fn new(_name: &str, passes: &[(Rc<dyn Pass>, bool)]) -> CompiledPass {
        CompiledPass {
            passes: passes.to_vec(),
        }
    }
}

impl Pass for CompiledPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let mut looping;
        loop {
            CancelationManager::check_canceled();
            looping = false;
            for (pass, loops) in &self.passes {
                if pass.run(ctx) && *loops {
                    looping = true;
                    break;
                }
            }
            if !looping {
                break;
            }
        }

        true
    }
}
