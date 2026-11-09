//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectGraph.java`.
//!
//! Node exprent lists are `JList<Exprent>`, which cannot hold the `null`
//! element Java's `iterateExprents` skips (a loop's empty init/condition/
//! increment slot); that slot is an empty list here.

use std::collections::VecDeque;

use crate::arena_object;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::exprent::Exprent;
use crate::jthrow;
use crate::rt::{JHashMap, JHashSet};
use crate::vb_style_collection::VBStyleCollection;

pub struct DirectGraphData {
    pub nodes: VBStyleCollection<DirectNode, String>,
    pub extra_nodes: Vec<DirectNode>,
    pub first: Option<DirectNode>,
    pub map_neg_if_branch: JHashMap<String, String>,
    pub map_finally_monitor_exception_path_exits: JHashMap<String, String>,
}

arena_object!(pub DirectGraph => DirectGraphData, Class);
crate::arena_identity!(DirectGraph);

/// Java: `DirectGraph.ExprentIterator`. Returns 0 to continue, 1 to cancel the
/// iteration, 2 to delete the exprent. Any `FnMut(Exprent) -> i32` is one.
pub trait ExprentIterator {
    fn process_exprent(&mut self, exprent: Exprent) -> i32;
}

impl<F: FnMut(Exprent) -> i32> ExprentIterator for F {
    fn process_exprent(&mut self, exprent: Exprent) -> i32 {
        self(exprent)
    }
}

impl DirectGraph {
    pub fn new() -> DirectGraph {
        DirectGraph::alloc(DirectGraphData {
            nodes: VBStyleCollection::new(),
            extra_nodes: Vec::new(),
            first: None,
            map_neg_if_branch: JHashMap::new(),
            map_finally_monitor_exception_path_exits: JHashMap::new(),
        })
    }

    pub fn first(self) -> Option<DirectNode> {
        self.with(|d| d.first)
    }

    pub fn set_first(self, first: Option<DirectNode>) {
        self.with_mut(|d| d.first = first)
    }

    /// The `nodes` collection's elements, in order.
    pub fn nodes(self) -> Vec<DirectNode> {
        self.with(|d| d.nodes.to_vec())
    }

    pub fn extra_nodes(self) -> Vec<DirectNode> {
        self.with(|d| d.extra_nodes.clone())
    }

    pub fn sort_reverse_post_order(self) {
        let mut res: VecDeque<DirectNode> = VecDeque::new();
        let first = self.with(|d| d.first).expect("DirectGraph.first");
        Self::add_to_reverse_post_order_list_iterative(first, &mut res);

        if res.len() != self.with(|d| d.nodes.len()) {
            let mut a = JHashSet::from_collection(self.with(|d| d.nodes.to_vec()));
            let b = JHashSet::from_collection(res.iter().copied().collect::<Vec<_>>());
            a.remove_all(&b);
            a.retain(|s| !s.statement().is_dummy_exit());

            for nd in a.to_vec() {
                self.with_mut(|d| d.extra_nodes.push(nd));
                res.push_front(nd);
            }
        }

        self.with_mut(|d| d.nodes.clear());

        for node in res {
            let id = node.id();
            self.with_mut(|d| d.nodes.add_with_key(node, id));
        }
    }

    fn add_to_reverse_post_order_list_iterative(root: DirectNode, lst: &mut VecDeque<DirectNode>) {
        let mut stack_node: Vec<DirectNode> = Vec::new();
        let mut stack_index: Vec<usize> = Vec::new();

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();

        stack_node.push(root);
        stack_index.push(0);

        while let Some(&node) = stack_node.last() {
            let mut index = stack_index.pop().expect("stackIndex");

            set_visited.add(node);

            while index < node.get_successors(DirectEdgeType::Regular).len() {
                let succ = node
                    .get_successors(DirectEdgeType::Regular)
                    .get(index)
                    .get_destination();

                if !set_visited.contains(&succ) {
                    stack_index.push(index + 1);

                    stack_node.push(succ);
                    stack_index.push(0);

                    break;
                }
                index += 1;
            }

            if index == node.get_successors(DirectEdgeType::Regular).len() {
                lst.push_front(node);

                stack_node.pop();
            }
        }
    }

    pub fn iterate_exprents(self, mut iter: impl ExprentIterator) -> bool {
        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        stack.push_back(self.with(|d| d.first).expect("DirectGraph.first"));

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();

        while let Some(node) = stack.pop_front() {
            if set_visited.contains(&node) {
                continue;
            }
            set_visited.add(node);

            let mut i = 0;
            while i < node.exprents().len() {
                let expr = node.exprents().get(i);

                let res = iter.process_exprent(expr);

                if res == 1 {
                    return false;
                }

                if res == 2 {
                    node.exprents().remove_at(i);
                } else {
                    i += 1;
                }
            }

            for suc in node.get_successors(DirectEdgeType::Regular).iter() {
                stack.push_back(suc.get_destination());
            }
        }

        true
    }

    pub fn iterate_exprents_deep(self, mut itr: impl ExprentIterator) -> bool {
        self.iterate_exprents(|exprent: Exprent| {
            for expr in exprent.get_all_exprents_rec_self(true, true).to_vec() {
                let res = itr.process_exprent(expr);
                if res == 1 {
                    return res;
                } else if res != 0 {
                    jthrow!(IllegalStateException; "Unsupported operation: {}", res);
                }
            }
            0
        })
    }
}
