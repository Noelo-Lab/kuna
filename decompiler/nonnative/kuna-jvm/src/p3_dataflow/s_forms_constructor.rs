//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SFormsConstructor.java`.
//!
//! The abstract class is one owned struct whose `kind` holds the subclass
//! state; `SSAConstructorSparseEx`/`SSAUConstructorSparseEx` are newtypes that
//! deref to it. Abstract and overridden methods are dispatchers that call the
//! subclass bodies in path form, `SSAUConstructorSparseEx::method(self, ...)`.
//! Var maps are `Shared`: the same map sits in a holder, a node's slot in these
//! tables and `currentCatchableMap`, and is compared by identity. The set
//! factory is an aliasing handle shared by every map and set it makes.
#![allow(dead_code, unused_variables)]

use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::exprent::Exprent;
use crate::fast_sparse_set_factory::FastSparseSetFactory;
use crate::field_exprent::FieldExprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, Shared};
use crate::s_forms_fast_map_direct::{SFormsFastMapDirect, VersionSet};
use crate::ssa_constructor_sparse_ex::{SSAConstructorSparseEx, SSAConstructorSparseExData};
use crate::ssau_constructor_sparse_ex::{SSAUConstructorSparseEx, SSAUConstructorSparseExData};
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_map_holder::VarMapHolder;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;

pub struct SFormsConstructor {
    pub track_field_vars: bool,
    pub track_direct_assignments: bool,
    in_var_versions: JHashMap<String, Shared<SFormsFastMapDirect>>,
    out_var_versions: JHashMap<String, Shared<SFormsFastMapDirect>>,
    out_neg_var_versions: JHashMap<String, Shared<SFormsFastMapDirect>>,
    extra_var_versions: JHashMap<String, Shared<SFormsFastMapDirect>>,
    catchable_versions: JHashMap<String, Shared<SFormsFastMapDirect>>,
    lastversion: JHashMap<i32, i32>,
    pub factory: Option<FastSparseSetFactory<i32>>,
    read_count: JHashMap<VarVersionPair, i32>,
    current_catchable_map: Option<Shared<SFormsFastMapDirect>>,
    pub root: Option<RootStatement>,
    mt: Option<StructMethod>,
    pub dgraph: Option<DirectGraph>,
    pub kind: SFormsConstructorKind,
}

/// The concrete subclass and its fields.
pub enum SFormsConstructorKind {
    Ssa(SSAConstructorSparseExData),
    Ssau(SSAUConstructorSparseExData),
}

impl SFormsConstructor {
    /// Java: SFormsConstructor(boolean, boolean); `kind` carries the subclass.
    pub fn new(
        track_field_vars: bool,
        track_direct_assignments: bool,
        kind: SFormsConstructorKind,
    ) -> SFormsConstructor {
        SFormsConstructor {
            track_field_vars,
            track_direct_assignments,
            in_var_versions: JHashMap::new(),
            out_var_versions: JHashMap::new(),
            out_neg_var_versions: JHashMap::new(),
            extra_var_versions: JHashMap::new(),
            catchable_versions: JHashMap::new(),
            lastversion: JHashMap::new(),
            factory: None,
            read_count: JHashMap::new(),
            current_catchable_map: None,
            root: None,
            mt: None,
            dgraph: None,
            kind,
        }
    }

    pub fn is_ssa(&self) -> bool {
        matches!(self.kind, SFormsConstructorKind::Ssa(_))
    }

    pub fn is_ssau(&self) -> bool {
        matches!(self.kind, SFormsConstructorKind::Ssau(_))
    }

    pub fn factory(&self) -> &FastSparseSetFactory<i32> {
        self.factory.as_ref().expect("SFormsConstructor.factory")
    }

    pub fn split_variables(&mut self, root: RootStatement, mt: StructMethod) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => self.split_variables_base(root, mt),
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::split_variables(self, root, mt)
            }
        }
    }

    /// `SFormsConstructor.splitVariables` itself (the `super` body).
    pub fn split_variables_base(&mut self, root: RootStatement, mt: StructMethod) {
        unported!("SFormsConstructor.splitVariables")
    }

    pub fn ssa_statements(
        &mut self,
        dgraph: DirectGraph,
        updated: &mut JHashSet<String>,
        calc_live_vars: bool,
        mt: StructMethod,
        iteration: i32,
    ) {
        unported!("SFormsConstructor.ssaStatements")
    }

    pub fn get_or_create_phantom(&mut self, var: VarVersionPair) -> VarVersionPair {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => {
                SSAConstructorSparseEx::get_or_create_phantom(self, var)
            }
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::get_or_create_phantom(self, var)
            }
        }
    }

    pub fn var_read(
        &mut self,
        var_maps: &VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
    ) {
        unported!("SFormsConstructor.varRead")
    }

    pub fn var_read_single_version(
        &mut self,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        varmap: &Shared<SFormsFastMapDirect>,
        last_version: i32,
    ) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => SSAConstructorSparseEx::var_read_single_version(
                self,
                stat,
                calc_live_vars,
                var_exprent,
                varmap,
                last_version,
            ),
            SFormsConstructorKind::Ssau(_) => SSAUConstructorSparseEx::var_read_single_version(
                self,
                stat,
                calc_live_vars,
                var_exprent,
                varmap,
                last_version,
            ),
        }
    }

    pub fn var_read_multiple_versions(
        &mut self,
        stat: Statement,
        calc_live_vars: bool,
        var_exprent: VarExprent,
        var_map: &Shared<SFormsFastMapDirect>,
        versions: VersionSet,
    ) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => SSAConstructorSparseEx::var_read_multiple_versions(
                self,
                stat,
                calc_live_vars,
                var_exprent,
                var_map,
                versions,
            ),
            SFormsConstructorKind::Ssau(_) => SSAUConstructorSparseEx::var_read_multiple_versions(
                self,
                stat,
                calc_live_vars,
                var_exprent,
                var_map,
                versions,
            ),
        }
    }

    pub fn mark_direct_assignment(
        &mut self,
        var_version_pair: VarVersionPair,
        right_pair: VarVersionPair,
    ) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => {
                SSAConstructorSparseEx::mark_direct_assignment(self, var_version_pair, right_pair)
            }
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::mark_direct_assignment(self, var_version_pair, right_pair)
            }
        }
    }

    fn makes_fields_dirty(expr: Exprent) -> bool {
        unported!("SFormsConstructor.makesFieldsDirty")
    }

    pub fn init_version(&mut self, var_exprent: VarExprent, stat: Statement) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => {
                SSAConstructorSparseEx::init_version(self, var_exprent, stat)
            }
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::init_version(self, var_exprent, stat)
            }
        }
    }

    pub fn update_var_exprent(
        &mut self,
        varassign: VarExprent,
        stat: Statement,
        varmap: &Shared<SFormsFastMapDirect>,
        calc_live_vars: bool,
    ) {
        unported!("SFormsConstructor.updateVarExprent")
    }

    pub fn on_assignment(
        &mut self,
        var_version_pair: VarVersionPair,
        var_map: &Shared<SFormsFastMapDirect>,
        calc_live_vars: bool,
    ) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => {
                self.on_assignment_base(var_version_pair, var_map, calc_live_vars)
            }
            SFormsConstructorKind::Ssau(_) => SSAUConstructorSparseEx::on_assignment(
                self,
                var_version_pair,
                var_map,
                calc_live_vars,
            ),
        }
    }

    /// `SFormsConstructor.onAssignment` itself: does nothing.
    pub fn on_assignment_base(
        &mut self,
        _var_version_pair: VarVersionPair,
        _var_map: &Shared<SFormsFastMapDirect>,
        _calc_live_vars: bool,
    ) {
    }

    fn merge_in_var_maps(&mut self, node: DirectNode, dgraph: DirectGraph) {
        unported!("SFormsConstructor.mergeInVarMaps")
    }

    fn get_filtered_out_map(
        &mut self,
        node: DirectNode,
        pred: DirectNode,
        dgraph: DirectGraph,
    ) -> Shared<SFormsFastMapDirect> {
        unported!("SFormsConstructor.getFilteredOutMap")
    }

    fn get_and_apply_diff(
        input: Option<&Shared<SFormsFastMapDirect>>,
        output: Option<&Shared<SFormsFastMapDirect>>,
        target: &Shared<SFormsFastMapDirect>,
    ) {
        unported!("SFormsConstructor.getAndApplyDiff")
    }

    pub fn get_first_protected_range(stat: Statement) -> Option<Statement> {
        let mut stat = stat;
        loop {
            let parent = stat.get_parent()?;

            if parent.is_catch_all() || parent.is_catch() {
                let first: Option<Statement> = parent.get_first().into();
                if first == Some(stat) {
                    return Some(parent);
                }
            } else if let Some(sync) = parent.try_as_synchronized() {
                let body: Option<Statement> = sync.get_body().into();
                if body == Some(stat) {
                    return Some(parent);
                }
            }

            stat = parent;
        }
    }

    fn set_catch_maps(
        &mut self,
        stat: Statement,
        dgraph: DirectGraph,
        flatthelper: &FlattenStatementsHelper,
    ) {
        unported!("SFormsConstructor.setCatchMaps")
    }

    fn create_first_map(&mut self) -> Shared<SFormsFastMapDirect> {
        unported!("SFormsConstructor.createFirstMap")
    }

    pub fn init_parameter(
        &mut self,
        var_index: i32,
        var_map: &Shared<SFormsFastMapDirect>,
        is_catch_var: bool,
    ) {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => {
                SSAConstructorSparseEx::init_parameter(self, var_index, var_map, is_catch_var)
            }
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::init_parameter(self, var_index, var_map, is_catch_var)
            }
        }
    }

    pub fn make_read_edge(phi_node: VarVersionNode, temp_node: VarVersionNode) {
        temp_node.with_mut(|d| d.successors.add(phi_node));
        phi_node.with_mut(|d| d.predecessors.add(temp_node));
    }

    pub fn maps_equal(
        map1: Option<&Shared<SFormsFastMapDirect>>,
        map2: Option<&Shared<SFormsFastMapDirect>>,
    ) -> bool {
        unported!("SFormsConstructor.mapsEqual")
    }

    pub fn field_read(&mut self, field: FieldExprent, varmap: &Shared<SFormsFastMapDirect>) {
        if self.track_field_vars {
            let index = self.get_field_index(field);

            varmap.borrow_mut().set_current_var(index, 1);
        }
    }

    pub fn set_current_var(&mut self, varmap: &Shared<SFormsFastMapDirect>, var: i32, vers: i32) {
        let mut set = self.factory().create_empty_set();
        set.add(vers);
        varmap.borrow_mut().put(var, Shared::new(set));
    }

    pub fn has_updated(&self, node: DirectNode, varmaps: &VarMapHolder) -> bool {
        let id = node.id();
        !Self::maps_equal(Some(&varmaps.get_if_true()), self.out_var_versions.get(&id))
            || (self.out_neg_var_versions.contains_key(&id)
                && !Self::maps_equal(
                    Some(&varmaps.get_if_false()),
                    self.out_neg_var_versions.get(&id),
                ))
    }

    pub fn get_field_index(&mut self, field: FieldExprent) -> i32 {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => SSAConstructorSparseEx::get_field_index(self, field),
            SFormsConstructorKind::Ssau(_) => SSAUConstructorSparseEx::get_field_index(self, field),
        }
    }

    pub fn get_next_free_version(&mut self, var: i32, stat: Option<Statement>) -> i32 {
        match self.kind {
            SFormsConstructorKind::Ssa(_) => self.get_next_free_version_base(var, stat),
            SFormsConstructorKind::Ssau(_) => {
                SSAUConstructorSparseEx::get_next_free_version(self, var, stat)
            }
        }
    }

    /// `SFormsConstructor.getNextFreeVersion` itself (the `super` body).
    pub fn get_next_free_version_base(&mut self, var: i32, _stat: Option<Statement>) -> i32 {
        let next = self.lastversion.get(&var).map_or(1, |v| v + 1);
        self.lastversion.put(var, next);
        next
    }

    pub fn get_read_count(&self, var: VarVersionPair) -> i32 {
        self.read_count.get(&var).copied().unwrap_or(0)
    }

    pub fn get_direct_graph(&self) -> Option<DirectGraph> {
        self.dgraph
    }
}
