//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionNode.java`.
//!
//! `live` is shared: `SSAUConstructorSparseEx.getLiveVarVersionsMap` hands out
//! the node's own map and `StackVarsProcessor` prunes it in place.

use crate::i_graph_node::IGraphNode;
use crate::rt::{JLinkedHashSet, JavaEq, JavaHash, Shared};
use crate::s_forms_fast_map_direct::SFormsFastMapDirect;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::validation_helper::ValidationHelper;
use crate::var_version_pair::VarVersionPair;
use crate::{arena_identity, arena_object};

pub struct VarVersionNodeData {
    pub var: i32,
    pub version: i32,
    pub successors: JLinkedHashSet<VarVersionNode>,
    pub predecessors: JLinkedHashSet<VarVersionNode>,
    pub phantom_node: Option<VarVersionNode>,
    pub phantom_parent_node: Option<VarVersionNode>,
    pub live: Option<Shared<SFormsFastMapDirect>>,
    pub lvt: Option<LocalVariable>,
    pub state: Option<State>,
}

arena_object!(pub VarVersionNode => VarVersionNodeData, Class);
arena_identity!(VarVersionNode);

impl VarVersionNode {
    pub fn new(var: i32, version: i32, lvt: Option<LocalVariable>) -> VarVersionNode {
        VarVersionNode::alloc(VarVersionNodeData {
            var,
            version,
            successors: JLinkedHashSet::new(),
            predecessors: JLinkedHashSet::new(),
            phantom_node: None,
            phantom_parent_node: None,
            live: None,
            lvt,
            state: None,
        })
    }

    pub fn var(self) -> i32 {
        self.with(|d| d.var)
    }

    pub fn version(self) -> i32 {
        self.with(|d| d.version)
    }

    /// The `successors` set's elements, in insertion order.
    pub fn successors(self) -> Vec<VarVersionNode> {
        self.with(|d| d.successors.to_vec())
    }

    /// The `predecessors` set's elements, in insertion order.
    pub fn predecessors(self) -> Vec<VarVersionNode> {
        self.with(|d| d.predecessors.to_vec())
    }

    pub fn phantom_node(self) -> Option<VarVersionNode> {
        self.with(|d| d.phantom_node)
    }

    pub fn set_phantom_node(self, node: Option<VarVersionNode>) {
        self.with_mut(|d| d.phantom_node = node)
    }

    pub fn phantom_parent_node(self) -> Option<VarVersionNode> {
        self.with(|d| d.phantom_parent_node)
    }

    pub fn set_phantom_parent_node(self, node: Option<VarVersionNode>) {
        self.with_mut(|d| d.phantom_parent_node = node)
    }

    pub fn live(self) -> Option<Shared<SFormsFastMapDirect>> {
        self.with(|d| d.live.clone())
    }

    pub fn set_live(self, live: Option<Shared<SFormsFastMapDirect>>) {
        self.with_mut(|d| d.live = live)
    }

    pub fn lvt(self) -> Option<LocalVariable> {
        self.with(|d| d.lvt.clone())
    }

    pub fn set_lvt(self, lvt: Option<LocalVariable>) {
        self.with_mut(|d| d.lvt = lvt)
    }

    pub fn state(self) -> Option<State> {
        self.with(|d| d.state)
    }

    pub fn set_state(self, state: Option<State>) {
        self.with_mut(|d| d.state = state)
    }

    pub fn get_predecessors(self) -> Vec<VarVersionNode> {
        self.predecessors()
    }

    pub fn remove_successor(self, node: VarVersionNode) {
        self.with_mut(|d| d.successors.remove(&node));
    }

    pub fn remove_predecessor(self, node: VarVersionNode) {
        self.with_mut(|d| d.predecessors.remove(&node));
    }

    pub fn has_single_predecessor(self) -> bool {
        self.with(|d| d.predecessors.len() == 1)
    }

    pub fn has_any_successors(self) -> bool {
        self.with(|d| !d.successors.is_empty())
    }

    pub fn get_single_predecessor(self) -> VarVersionNode {
        ValidationHelper::validate_true(
            self.has_single_predecessor(),
            "Expected only a single predecessor",
        );
        self.with(|d| d.predecessors.iter().next().copied())
            .expect("NoSuchElementException")
    }

    pub fn as_pair(self) -> VarVersionPair {
        self.with(|d| VarVersionPair::new(d.var, d.version))
    }
}

impl IGraphNode for VarVersionNode {
    fn get_predecessors(&self) -> Vec<VarVersionNode> {
        VarVersionNode::get_predecessors(*self)
    }
}

impl std::fmt::Display for VarVersionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (var, version) = self.with(|d| (d.var, d.version));
        write!(f, "({var}_{version})")
    }
}

/// Java: `VarVersionNode.State`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum State {
    Write,
    Read,
    Phi,
    DeadRead,
    Phantom,
    Param,
    Catch,
}

impl State {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            State::Write => "WRITE",
            State::Read => "READ",
            State::Phi => "PHI",
            State::DeadRead => "DEAD_READ",
            State::Phantom => "PHANTOM",
            State::Param => "PARAM",
            State::Catch => "CATCH",
        }
    }
}

impl JavaHash for State {
    fn java_hash(&self) -> i32 {
        self.ordinal()
    }
}

impl JavaEq for State {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
