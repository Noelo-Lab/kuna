//! Port of `org/jetbrains/java/decompiler/api/plugin/pass/ClassPass.java`.

use crate::classes_processor::ClassNode;

pub trait ClassPass {
    fn run(&self, root: ClassNode) -> bool;
}

impl<F: Fn(ClassNode) -> bool> ClassPass for F {
    fn run(&self, root: ClassNode) -> bool {
        self(root)
    }
}
