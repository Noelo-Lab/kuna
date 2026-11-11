//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SSAUConstructorSparseEx.java`.
//!
//! A newtype over `SFormsConstructor` (see there); the overriding bodies and
//! the helpers they share take the base as `this`. `getLiveVarVersionsMap`
//! returns the node's own (shared) live map, as Java does.
#![allow(dead_code, unused_variables)]

use std::ops::{Deref, DerefMut};

use crate::field_exprent::FieldExprent;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, Shared};
use crate::s_forms_constructor::{SFormsConstructor, SFormsConstructorKind};
use crate::s_forms_fast_map_direct::{SFormsFastMapDirect, VersionSet};
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_version_node::{State, VarVersionNode};
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_graph::VarVersionsGraph;

pub struct SSAUConstructorSparseExData {
    var_assignment_map: JHashMap<VarVersionPair, VarVersionPair>,
    map_version_first_range: JHashMap<VarVersionPair, i32>,
    pub ssu_versions: VarVersionsGraph,
    map_field_vars: JHashMap<i32, i32>,
    field_var_counter: i32,
}

pub struct SSAUConstructorSparseEx(pub SFormsConstructor);

impl Deref for SSAUConstructorSparseEx {
    type Target = SFormsConstructor;
    fn deref(&self) -> &SFormsConstructor {
        &self.0
    }
}

impl DerefMut for SSAUConstructorSparseEx {
    fn deref_mut(&mut self) -> &mut SFormsConstructor {
        &mut self.0
    }
}

impl Default for SSAUConstructorSparseEx {
    fn default() -> Self {
        Self::new()
    }
}

impl SSAUConstructorSparseEx {
    pub fn new() -> SSAUConstructorSparseEx {
        SSAUConstructorSparseEx(SFormsConstructor::new(
            true,
            true,
            SFormsConstructorKind::Ssau(SSAUConstructorSparseExData {
                var_assignment_map: JHashMap::new(),
                map_version_first_range: JHashMap::new(),
                ssu_versions: VarVersionsGraph::new(),
                map_field_vars: JHashMap::new(),
                field_var_counter: -1,
            }),
        ))
    }

    fn d(this: &SFormsConstructor) -> &SSAUConstructorSparseExData {
        match &this.kind {
            SFormsConstructorKind::Ssau(d) => d,
            _ => unreachable!("not an SSAUConstructorSparseEx"),
        }
    }

    fn d_mut(this: &mut SFormsConstructor) -> &mut SSAUConstructorSparseExData {
        match &mut this.kind {
            SFormsConstructorKind::Ssau(d) => d,
            _ => unreachable!("not an SSAUConstructorSparseEx"),
        }
    }

    fn ssu_versions_of(this: &SFormsConstructor) -> VarVersionsGraph {
        Self::d(this).ssu_versions
    }

    pub fn split_variables(this: &mut SFormsConstructor, root: RootStatement, mt: StructMethod) {
        this.split_variables_base(root, mt);

        let dgraph = this.dgraph.expect("SFormsConstructor.dgraph");
        this.ssa_statements(dgraph, &mut JHashSet::new(), true, mt, 999_999);

        Self::ssu_versions_of(this).init_dominators();

        ValidationHelper::validate_var_versions_graph(
            Self::ssu_versions_of(this),
            root,
            &Self::d(this).var_assignment_map,
        );
    }

    /// Java: getNode(VarExprent)
    pub fn get_node(&self, var: VarExprent) -> Option<VarVersionNode> {
        Self::get_node_pair(&self.0, var.get_var_version_pair())
    }

    /// Java: getNode(VarVersionPair)
    fn get_node_pair(this: &SFormsConstructor, pair: VarVersionPair) -> Option<VarVersionNode> {
        Self::ssu_versions_of(this).get_node_with_key(pair)
    }

    /// Java: getNode(int, int)
    fn get_node_var_version(
        this: &SFormsConstructor,
        var: i32,
        version: i32,
    ) -> Option<VarVersionNode> {
        Self::get_node_pair(this, VarVersionPair::new(var, version))
    }

    pub fn get_or_create_phantom(
        this: &mut SFormsConstructor,
        pair: VarVersionPair,
    ) -> VarVersionPair {
        let var_node = Self::get_node_pair(this, pair).expect("NullPointerException");
        match var_node.phantom_node() {
            None => {
                let phantom_node =
                    Self::create_new_node(this, var_node.var(), None, State::Phantom);
                var_node.set_phantom_node(Some(phantom_node));
                phantom_node.set_phantom_parent_node(Some(var_node));
                phantom_node.as_pair()
            }
            Some(phantom_node) => {
                ValidationHelper::validate_true(
                    phantom_node.state() == Some(State::Phantom),
                    "Expected phantom node to be PHANTOM",
                );

                phantom_node.as_pair()
            }
        }
    }

    pub fn mark_direct_assignment(
        this: &mut SFormsConstructor,
        var_version_pair: VarVersionPair,
        right_pair: VarVersionPair,
    ) {
        Self::d_mut(this)
            .var_assignment_map
            .put(var_version_pair, right_pair);
    }

    pub fn var_read_single_version(
        this: &mut SFormsConstructor,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        var_map: &Shared<SFormsFastMapDirect>,
        last_version: i32,
    ) {
        let var_index = var_exprent.get_index();
        let current_version = var_exprent.get_version();

        if current_version == 0 {
            let previous_node = Self::get_node_var_version(this, var_index, last_version)
                .expect("NullPointerException");
            let use_node = Self::create_read(this, previous_node, stat);

            var_exprent.set_version(use_node.version());
        }

        Self::update_live_map(
            this,
            VarVersionPair::new(var_index, current_version),
            var_map,
            calc_live_vars,
        );
        var_map.borrow_mut().set_current_var_exprent(var_exprent);
    }

    pub fn var_read_multiple_versions(
        this: &mut SFormsConstructor,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        var_map: &Shared<SFormsFastMapDirect>,
        versions: VersionSet,
    ) {
        let var_index = var_exprent.get_index();
        let mut current_version = var_exprent.get_version();

        if current_version == 0 {
            let node = Self::create_new_node(this, var_index, Some(stat), State::Phi);
            current_version = node.version();

            var_exprent.set_version(current_version);
        }
        Self::update_live_map(
            this,
            VarVersionPair::new(var_index, current_version),
            var_map,
            calc_live_vars,
        );

        var_map.borrow_mut().set_current_var_exprent(var_exprent);
        Self::create_or_update_phi_node(
            this,
            VarVersionPair::new(var_index, current_version),
            &versions,
            stat,
        );
    }

    fn create_or_update_phi_node(
        this: &mut SFormsConstructor,
        phivar: VarVersionPair,
        vers: &VersionSet,
        stat: Statement,
    ) {
        let mut old_nodes: JHashSet<i32> = JHashSet::new();

        let phi_node = Self::ssu_versions_of(this)
            .get_node_with_key(phivar)
            .expect("NullPointerException");
        ValidationHelper::validate_true(
            phi_node.phantom_parent_node().is_none(),
            "phi node can't be a phantom node",
        );
        let pred_count = phi_node.with(|d| d.predecessors.len());
        if pred_count == 0 {
            ValidationHelper::validate_true(
                phi_node.state() == Some(State::Phi),
                "Phi node has the wrong state?",
            );
        } else if pred_count == 1 {
            ValidationHelper::validate_true(
                phi_node.state() == Some(State::Read),
                "Trying to convert a non read node into a phi node",
            );
            phi_node.set_state(Some(State::Phi));
            phi_node.get_single_predecessor().remove_successor(phi_node);
            phi_node.with_mut(|d| d.predecessors.clear());
        } else {
            ValidationHelper::validate_true(
                phi_node.state() == Some(State::Phi),
                "Phi node has the wrong state?",
            );
            for source in phi_node.predecessors() {
                ValidationHelper::validate_true(
                    source.state() == Some(State::Read),
                    "Phi node is reading from a non READ node",
                );

                let verssrc = source.get_single_predecessor().version();
                if !vers.borrow().contains(&verssrc) {
                    source.remove_successor(phi_node);
                    phi_node.remove_predecessor(source);
                    source.set_state(Some(State::DeadRead));
                } else {
                    old_nodes.add(verssrc);
                }
            }
        }

        let vers_list: Vec<i32> = vers.borrow().iter().collect();
        for ver in vers_list {
            if old_nodes.contains(&ver) {
                continue;
            }

            let pre_node = Self::ssu_versions_of(this)
                .get_node_with_key(VarVersionPair::new(phivar.var, ver))
                .expect("NullPointerException");
            let temp_node = Self::create_read(this, pre_node, stat);
            SFormsConstructor::make_read_edge(phi_node, temp_node);
        }
    }

    fn create_read(
        this: &mut SFormsConstructor,
        node: VarVersionNode,
        stat: Statement,
    ) -> VarVersionNode {
        let read = Self::create_new_node(this, node.var(), Some(stat), State::Read);
        SFormsConstructor::make_read_edge(read, node);
        read
    }

    fn create_new_node(
        this: &mut SFormsConstructor,
        var: i32,
        stat: Option<Statement>,
        state: State,
    ) -> VarVersionNode {
        let version = this.get_next_free_version(var, stat);
        let node = Self::ssu_versions_of(this).create_node(VarVersionPair::new(var, version));
        node.set_state(Some(state));
        node
    }

    pub fn get_field_index(this: &mut SFormsConstructor, field: FieldExprent) -> i32 {
        let id = field.with(|d| d.id);
        if let Some(&index) = Self::d(this).map_field_vars.get(&id) {
            index
        } else {
            let d = Self::d_mut(this);
            let index = d.field_var_counter;
            d.field_var_counter -= 1;
            d.map_field_vars.put(id, index);

            Self::ssu_versions_of(this).create_node(VarVersionPair::new(index, 1));
            index
        }
    }

    pub fn get_next_free_version(
        this: &mut SFormsConstructor,
        var: i32,
        stat: Option<Statement>,
    ) -> i32 {
        let next_version = this.get_next_free_version_base(var, stat);

        if let Some(stat) = stat {
            let first_range = SFormsConstructor::get_first_protected_range(stat);

            if let Some(first_range) = first_range {
                let range_id = first_range.with(|d| d.id);
                Self::d_mut(this)
                    .map_version_first_range
                    .put(VarVersionPair::new(var, next_version), range_id);
            }
        }

        next_version
    }

    pub fn get_ssu_versions(&self) -> VarVersionsGraph {
        Self::ssu_versions_of(&self.0)
    }

    pub fn get_live_var_versions_map(
        &self,
        var_version: VarVersionPair,
    ) -> Option<Shared<SFormsFastMapDirect>> {
        let node = Self::ssu_versions_of(&self.0).get_node_with_key(var_version)?;
        Some(match node.live() {
            Some(live) => live,
            None => Shared::new(SFormsFastMapDirect::new(self.0.factory().clone())),
        })
    }

    pub fn get_map_version_first_range(&self) -> &JHashMap<VarVersionPair, i32> {
        &Self::d(&self.0).map_version_first_range
    }

    pub fn get_map_field_vars(&self) -> &JHashMap<i32, i32> {
        &Self::d(&self.0).map_field_vars
    }

    /// `getMapFieldVars()` for the callers that `put` into it.
    pub fn get_map_field_vars_mut(&mut self) -> &mut JHashMap<i32, i32> {
        &mut Self::d_mut(&mut self.0).map_field_vars
    }

    pub fn get_var_assignment_map(&self) -> &JHashMap<VarVersionPair, VarVersionPair> {
        &Self::d(&self.0).var_assignment_map
    }

    pub fn on_assignment(
        this: &mut SFormsConstructor,
        var_version_pair: VarVersionPair,
        var_map: &Shared<SFormsFastMapDirect>,
        calc_live_vars: bool,
    ) {
        Self::update_live_map(this, var_version_pair, var_map, calc_live_vars);
    }

    fn update_live_map(
        this: &mut SFormsConstructor,
        var_version_pair: VarVersionPair,
        var_map: &Shared<SFormsFastMapDirect>,
        calc_live_vars: bool,
    ) {
        if calc_live_vars {
            let node = Self::get_node_pair(this, var_version_pair).expect("NullPointerException");

            node.set_live(Some(Shared::new(var_map.borrow().get_copy())));
        }
    }

    pub fn init_version(this: &mut SFormsConstructor, var_exprent: VarExprent, stat: Statement) {
        if var_exprent.get_version() == 0 {
            let node =
                Self::create_new_node(this, var_exprent.get_index(), Some(stat), State::Write);

            var_exprent.set_version(node.version());
        }
    }

    pub fn init_parameter(
        this: &mut SFormsConstructor,
        var_index: i32,
        var_map: &Shared<SFormsFastMapDirect>,
        is_catch: bool,
    ) {
        let root: Option<Statement> = this.root.map(Into::into);
        let node = Self::create_new_node(
            this,
            var_index,
            root,
            if is_catch { State::Catch } else { State::Param },
        );

        var_map.borrow_mut().set_current_var_node(node);
    }
}
