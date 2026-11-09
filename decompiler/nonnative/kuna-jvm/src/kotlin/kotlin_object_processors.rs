//! Port of `org/vineflower/kotlin/util/KotlinObjectProcessors.java`.

#![allow(unused_variables)]

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::unported;

pub struct KotlinObjectProcessors;

impl KotlinObjectProcessors {
    pub fn parse_with_companion(node: ClassNode) {
        unported!("KotlinObjectProcessors.parseWithCompanion")
    }

    pub fn mark_object_instance(wrapper: ClassWrapper) {
        unported!("KotlinObjectProcessors.markObjectInstance")
    }
}
