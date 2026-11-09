//! Port of `org/jetbrains/java/decompiler/main/rels/LambdaProcessor.java`.

#![allow(dead_code, unused_variables)]

use crate::classes_processor::ClassNode;
use crate::unported;

#[derive(Default)]
pub struct LambdaProcessor;

impl LambdaProcessor {
    const JAVAC_LAMBDA_CLASS: &'static str = "java/lang/invoke/LambdaMetafactory";
    const JAVAC_LAMBDA_METHOD: &'static str = "metafactory";
    const JAVAC_LAMBDA_ALT_METHOD: &'static str = "altMetafactory";

    pub fn new() -> LambdaProcessor {
        LambdaProcessor
    }

    /// Turns each `invokedynamic` lambda of `node` (and its nested classes) into a
    /// lambda [`ClassNode`] registered in the root class map.
    pub fn process_class(&self, node: ClassNode) {
        unported!("LambdaProcessor.processClass")
    }
}
