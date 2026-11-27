//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/GenericDominatorEngine.java`.

use crate::i_graph::IGraph;
use crate::i_graph_node::IGraphNode;
use crate::jthrow;
use crate::rt::{JHashSet, JavaEq, JavaHash};
use crate::validation_helper::ValidationHelper;
use crate::vb_style_collection::VBStyleCollection;

pub struct GenericDominatorEngine<N> {
    initialized: bool,
    graph: Box<dyn IGraph<N>>,
    col_ordered_idoms: VBStyleCollection<Option<N>, N>,
    set_roots: Option<JHashSet<N>>,
}

impl<N: IGraphNode + Copy + PartialEq + JavaHash + JavaEq> GenericDominatorEngine<N> {
    pub fn new(graph: Box<dyn IGraph<N>>) -> GenericDominatorEngine<N> {
        GenericDominatorEngine {
            initialized: false,
            graph,
            col_ordered_idoms: VBStyleCollection::new(),
            set_roots: None,
        }
    }

    pub fn initialize(&mut self) {
        ValidationHelper::assert_true(
            !self.initialized,
            "Dominator Engine was already initialized",
        );
        self.calc_idoms();
        self.initialized = true;
    }

    fn order_nodes(&mut self) {
        self.set_roots = Some(self.graph.get_roots());

        for node in self.graph.get_reverse_post_order_list() {
            self.col_ordered_idoms.add_with_key(None, node);
        }
    }

    fn get_common_idom(
        node1: Option<N>,
        node2: Option<N>,
        ordered_idoms: &VBStyleCollection<Option<N>, N>,
    ) -> Option<N> {
        let (mut node1, mut node2) = match (node1, node2) {
            (None, _) => return node2,
            (_, None) => return node1,
            (Some(a), Some(b)) => (a, b),
        };

        let mut index1 = ordered_idoms.get_index_by_key(&node1);
        let mut index2 = ordered_idoms.get_index_by_key(&node2);

        while index1 != index2 {
            if index1 > index2 {
                let node_old = node1;
                let next = ordered_idoms.get_with_key(&node1).flatten();

                if next == Some(node_old) {
                    return None;
                }

                node1 = next.expect("NullPointerException");
                index1 = ordered_idoms.get_index_by_key(&node1);
            } else {
                let node_old = node2;
                let next = ordered_idoms.get_with_key(&node2).flatten();

                if next == Some(node_old) {
                    return None;
                }

                node2 = next.expect("NullPointerException");
                index2 = ordered_idoms.get_index_by_key(&node2);
            }
        }

        Some(node1)
    }

    fn calc_idoms(&mut self) {
        self.order_nodes();

        let lst_nodes: Vec<N> = self
            .col_ordered_idoms
            .get_lst_keys()
            .iter()
            .map(|k| k.expect("NullPointerException"))
            .collect();

        loop {
            let mut changed = false;

            for &node in &lst_nodes {
                let mut idom: Option<N> = None;

                if !self.set_roots.as_ref().expect("roots").contains(&node) {
                    for pred in node.get_predecessors() {
                        if self
                            .col_ordered_idoms
                            .get_with_key(&pred)
                            .flatten()
                            .is_some()
                        {
                            idom = Self::get_common_idom(idom, Some(pred), &self.col_ordered_idoms);
                            if idom.is_none() {
                                break;
                            }
                        }
                    }
                }

                let idom = idom.unwrap_or(node);

                let oldidom = self
                    .col_ordered_idoms
                    .put_with_key(Some(idom), node)
                    .flatten();
                if !oldidom.is_some_and(|old| idom.java_equals(&old)) {
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
    }

    pub fn is_dominator(&self, node: N, dom: N) -> bool {
        if !self.initialized {
            jthrow!(IllegalStateException; "GenericDominatorEngine not initialized!");
        }

        let mut node = node;
        while !node.java_equals(&dom) {
            let idom = self.col_ordered_idoms.get_with_key(&node).flatten();

            match idom {
                Some(idom) if idom == node => return false,
                None => {
                    jthrow!("Inconsistent idom sequence discovered or node not found in dom graph!")
                }
                Some(idom) => node = idom,
            }
        }

        true
    }
}
