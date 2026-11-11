//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/IGraphNode.java`.

pub trait IGraphNode: Sized {
    fn get_predecessors(&self) -> Vec<Self>;
}
