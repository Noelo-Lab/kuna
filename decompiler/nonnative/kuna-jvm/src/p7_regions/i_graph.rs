//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/IGraph.java`.

use crate::rt::JHashSet;

pub trait IGraph<N> {
    fn get_reverse_post_order_list(&self) -> Vec<N>;

    fn get_roots(&self) -> JHashSet<N>;
}
