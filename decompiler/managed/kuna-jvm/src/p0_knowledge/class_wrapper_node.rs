//! Port of `org/jetbrains/java/decompiler/modules/renamer/ClassWrapperNode.java`.

use crate::struct_class::StructClass;
use crate::{arena_identity, arena_object};

pub struct ClassWrapperNodeData {
    pub class_struct: StructClass,
    pub subclasses: Vec<ClassWrapperNode>,
}

arena_object!(pub ClassWrapperNode => ClassWrapperNodeData, Session);
arena_identity!(ClassWrapperNode);

impl ClassWrapperNode {
    pub fn new(cl: StructClass) -> ClassWrapperNode {
        ClassWrapperNode::alloc(ClassWrapperNodeData {
            class_struct: cl,
            subclasses: Vec::new(),
        })
    }

    pub fn add_subclass(self, node: ClassWrapperNode) {
        self.with_mut(|d| d.subclasses.push(node))
    }

    pub fn get_class_struct(self) -> StructClass {
        self.with(|d| d.class_struct)
    }

    pub fn get_subclasses(self) -> Vec<ClassWrapperNode> {
        self.with(|d| d.subclasses.clone())
    }
}
