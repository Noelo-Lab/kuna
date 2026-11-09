//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionsGraph.java`.
#![allow(dead_code)]

use std::collections::VecDeque;

use crate::generic_dominator_engine::GenericDominatorEngine;
use crate::rt::JHashSet;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::unported;
use crate::validation_helper::ValidationHelper;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;
use crate::vb_style_collection::VBStyleCollection;
use crate::{arena_identity, arena_object};

pub struct VarVersionsGraphData {
    pub nodes: VBStyleCollection<VarVersionNode, VarVersionPair>,
    engine: Option<GenericDominatorEngine<VarVersionNode>>,
}

arena_object!(pub VarVersionsGraph => VarVersionsGraphData, Class);
arena_identity!(VarVersionsGraph);

impl VarVersionsGraph {
    pub fn new() -> VarVersionsGraph {
        VarVersionsGraph::alloc(VarVersionsGraphData {
            nodes: VBStyleCollection::new(),
            engine: None,
        })
    }

    /// `nodes.getWithKey(pair)`.
    pub fn get_node_with_key(self, pair: VarVersionPair) -> Option<VarVersionNode> {
        self.with(|d| d.nodes.get_with_key(&pair))
    }

    /// The `nodes` collection's elements, in order.
    pub fn nodes(self) -> Vec<VarVersionNode> {
        self.with(|d| d.nodes.to_vec())
    }

    /// Java: createNode(VarVersionPair)
    pub fn create_node(self, ver: VarVersionPair) -> VarVersionNode {
        self.create_node_lvt(ver, None)
    }

    /// Java: createNode(VarVersionPair, LocalVariable)
    pub fn create_node_lvt(
        self,
        ver: VarVersionPair,
        lvt: Option<LocalVariable>,
    ) -> VarVersionNode {
        let node = VarVersionNode::new(ver.var, ver.version, lvt);
        self.with_mut(|d| d.nodes.add_with_key(node, ver));
        node
    }

    pub fn is_dominator_set(
        self,
        node: VarVersionNode,
        domnodes: &JHashSet<VarVersionNode>,
    ) -> bool {
        if domnodes.len() == 1 {
            let dom = *domnodes.iter().next().expect("domnodes");
            return self.with(|d| {
                d.engine
                    .as_ref()
                    .expect("VarVersionsGraph.engine")
                    .is_dominator(node, dom)
            });
        } else {
            if domnodes.contains(&node) {
                return true;
            }

            let mut seen: JHashSet<VarVersionNode> = JHashSet::new();

            let mut lst_nodes: VecDeque<VarVersionNode> = VecDeque::new();
            lst_nodes.push_back(node);

            while let Some(nd) = lst_nodes.pop_front() {
                if !seen.add(nd) {
                    continue;
                }

                let predecessors = nd.predecessors();
                if predecessors.is_empty() {
                    return false;
                }

                for pred in predecessors {
                    if !seen.contains(&pred) && !domnodes.contains(&pred) {
                        lst_nodes.push_back(pred);
                    }
                }
            }
        }

        true
    }

    pub fn init_dominators(self) {
        unported!("VarVersionsGraph.initDominators")
    }

    fn find_reachable_nodes(self, start: VarVersionNode) -> JHashSet<VarVersionNode> {
        let mut visited: JHashSet<VarVersionNode> = JHashSet::new();
        let mut stack: Vec<VarVersionNode> = vec![start];

        while let Some(node) = stack.pop() {
            if visited.add(node) {
                stack.extend(node.successors());
            }
        }

        visited
    }

    pub fn root_reachability(roots: &JHashSet<VarVersionNode>) -> JHashSet<VarVersionNode> {
        let mut visited: JHashSet<VarVersionNode> = JHashSet::new();

        let mut stack: Vec<VarVersionNode> = roots.to_vec();

        while let Some(node) = stack.pop() {
            if visited.add(node) {
                stack.extend(node.successors());
            }
        }

        visited
    }

    pub fn are_vars_analogous(self, var_base: i32, var_check: i32) -> bool {
        let mut stack: VecDeque<VarVersionNode> = VecDeque::new();
        let mut visited: JHashSet<VarVersionNode> = JHashSet::new();

        let start = self
            .get_node_with_key(VarVersionPair::new(var_base, 1))
            .expect("NullPointerException");
        stack.push_back(start);

        while let Some(node) = stack.pop_front() {
            ValidationHelper::validate_true(
                node.phantom_parent_node().is_none() && node.phantom_node().is_none(),
                "`areVarsAnalogous` should not be called after ppmm or operator assignments resugaring",
            );

            if visited.contains(&node) {
                continue;
            }

            visited.add(node);
            let Some(analog) =
                self.get_node_with_key(VarVersionPair::new(var_check, node.version()))
            else {
                return false;
            };

            if node.successors().len() != analog.successors().len() {
                return false;
            }

            for dest in node.successors() {
                stack.push_back(dest);

                let suc_analog =
                    self.get_node_with_key(VarVersionPair::new(var_check, dest.version()));

                if suc_analog.is_none() {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_reversed_post_order(roots: &[VarVersionNode]) -> Vec<VarVersionNode> {
        let mut lst: Vec<VarVersionNode> = Vec::new();
        let mut set_visited: JHashSet<VarVersionNode> = JHashSet::new();
        let mut stack_node: VecDeque<VarVersionNode> = VecDeque::new();
        let mut stack_index: VecDeque<usize> = VecDeque::new();
        let mut lst_succs: Vec<VarVersionNode> = Vec::new();

        for &root in roots {
            let mut lst_temp: Vec<VarVersionNode> = Vec::new();
            Self::add_to_reverse_post_order_list_iterative(
                root,
                &mut lst_temp,
                &mut set_visited,
                &mut stack_node,
                &mut stack_index,
                &mut lst_succs,
            );
            lst.extend(lst_temp);
        }

        lst
    }

    fn add_to_reverse_post_order_list_iterative(
        root: VarVersionNode,
        lst: &mut Vec<VarVersionNode>,
        set_visited: &mut JHashSet<VarVersionNode>,
        stack_node: &mut VecDeque<VarVersionNode>,
        stack_index: &mut VecDeque<usize>,
        lst_succs: &mut Vec<VarVersionNode>,
    ) {
        stack_node.clear();
        stack_index.clear();

        stack_node.push_back(root);
        stack_index.push_back(0);

        while let Some(&node) = stack_node.back() {
            let mut index = stack_index.pop_back().expect("stackIndex");

            set_visited.add(node);

            lst_succs.clear();
            lst_succs.extend(node.successors());
            while index < lst_succs.len() {
                let succ = lst_succs[index];

                if !set_visited.contains(&succ) {
                    stack_index.push_back(index + 1);
                    stack_node.push_back(succ);
                    stack_index.push_back(0);
                    break;
                }
                index += 1;
            }

            if index == lst_succs.len() {
                lst.insert(0, node);
                stack_node.pop_back();
            }
        }
    }
}
