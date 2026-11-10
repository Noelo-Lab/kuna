//! Port of `org/jetbrains/java/decompiler/modules/decompiler/sforms/SFormsConstructor.java`.
//!
//! The abstract class is one owned struct whose `kind` holds the subclass
//! state; `SSAConstructorSparseEx`/`SSAUConstructorSparseEx` are newtypes that
//! deref to it. Abstract and overridden methods are dispatchers that call the
//! subclass bodies in path form, `SSAUConstructorSparseEx::method(self, ...)`.
//! Var maps are `Shared`: the same map sits in a holder, a node's slot in these
//! tables and `currentCatchableMap`, and is compared by identity. The set
//! factory is an aliasing handle shared by every map and set it makes.
//!
//! kuna fix `field_read_all_paths` (spec 10.16): with `field_reads_all_paths`
//! set, a field variable stays live only while it is live on every path.
use crate::cancelation_manager::CancelationManager;
use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::exprent::{Exprent, ExprentType};
use crate::fast_sparse_set_factory::FastSparseSetFactory;
use crate::field_exprent::FieldExprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::method_descriptor::MethodDescriptor;
use crate::p3_dataflow::kuna_fix_field_read_all_paths::FieldReadAllPaths;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JavaListExt, Shared};
use crate::s_forms_fast_map_direct::{SFormsFastMapDirect, VersionSet};
use crate::ssa_constructor_sparse_ex::{SSAConstructorSparseEx, SSAConstructorSparseExData};
use crate::ssau_constructor_sparse_ex::{SSAUConstructorSparseEx, SSAUConstructorSparseExData};
use crate::statement::{Statement, StatementType};
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_map_holder::VarMapHolder;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;

thread_local! {
    /// `new FastSparseSetFactory<>(0..63)`, which every `splitVariables` builds.
    static INITIAL_FACTORY: FastSparseSetFactory<i32> = FastSparseSetFactory::new(0..64);
}

pub struct SFormsConstructor {
    pub track_field_vars: bool,
    pub track_direct_assignments: bool,
    in_var_versions: JHashMap<std::rc::Rc<str>, Shared<SFormsFastMapDirect>>,
    out_var_versions: JHashMap<std::rc::Rc<str>, Shared<SFormsFastMapDirect>>,
    out_neg_var_versions: JHashMap<std::rc::Rc<str>, Shared<SFormsFastMapDirect>>,
    extra_var_versions: JHashMap<std::rc::Rc<str>, Shared<SFormsFastMapDirect>>,
    catchable_versions: JHashMap<std::rc::Rc<str>, Shared<SFormsFastMapDirect>>,
    lastversion: JHashMap<i32, i32>,
    pub factory: Option<FastSparseSetFactory<i32>>,
    read_count: JHashMap<VarVersionPair, i32>,
    current_catchable_map: Option<Shared<SFormsFastMapDirect>>,
    pub root: Option<RootStatement>,
    mt: Option<StructMethod>,
    pub dgraph: Option<DirectGraph>,
    pub kind: SFormsConstructorKind,
    /// Fix `field_read_all_paths`: merges keep a field variable only when every
    /// predecessor has it, and a finally body that may change fields drops them
    /// on the normal exit.
    pub field_reads_all_paths: bool,
    /// Per try-finally statement id: whether its finally body may change fields.
    finally_changes_fields: JHashMap<i32, bool>,
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
            field_reads_all_paths: false,
            finally_changes_fields: JHashMap::new(),
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
        self.root = Some(root);
        self.mt = Some(mt);

        let mut flatthelper = FlattenStatementsHelper::new();
        let dgraph = flatthelper.build_direct_graph(root);
        self.dgraph = Some(dgraph);
        ValidationHelper::validate_d_graph(dgraph, root);
        ValidationHelper::validate_vars(
            dgraph,
            root,
            |var: VarExprent| var.get_version() == 0,
            "Var version is not zero",
        );

        self.factory = Some(INITIAL_FACTORY.with(FastSparseSetFactory::duplicate));

        let first_id = dgraph.first().expect("DirectGraph.first").id_key();
        let first_map = self.create_first_map();
        self.extra_var_versions.put(first_id, first_map);

        self.set_catch_maps(root.into(), dgraph, &flatthelper);

        let mut iteration = 1;
        let mut updated: JHashSet<std::rc::Rc<str>> = JHashSet::new();
        loop {
            CancelationManager::check_canceled();
            self.ssa_statements(dgraph, &mut updated, false, mt, iteration);
            iteration += 1;
            if updated.is_empty() {
                break;
            }
        }
    }

    pub fn ssa_statements(
        &mut self,
        dgraph: DirectGraph,
        updated: &mut JHashSet<std::rc::Rc<str>>,
        calc_live_vars: bool,
        _mt: StructMethod,
        _iteration: i32,
    ) {
        for node in dgraph.nodes() {
            CancelationManager::check_canceled();
            let node_id = node.id_key();
            updated.remove(&node_id);
            self.merge_in_var_maps(node, dgraph);

            let varmap = self
                .in_var_versions
                .get_cloned(&node_id)
                .expect("NullPointerException");
            let mut varmaps = VarMapHolder::of_normal_all_paths(
                &varmap,
                self.track_field_vars && self.field_reads_all_paths,
            );
            self.current_catchable_map = None;

            if node.has_successors(DirectEdgeType::Exception) {
                let catchable = Shared::new(varmap.borrow().get_copy());
                catchable.borrow_mut().remove_all_stacks();
                catchable.borrow_mut().remove_all_fields();
                self.current_catchable_map = Some(catchable.clone());
                self.catchable_versions.put(node_id.clone(), catchable);
            }

            let exprents = node.exprents();
            let foreach_var = if node.type_() == DirectNodeType::ForeachVardef {
                exprents.first().and_then(|e| e.try_as_var())
            } else {
                None
            };
            if let Some(var) = foreach_var {
                let normal = varmaps.get_normal();
                self.update_var_exprent(var, node.statement(), &normal, calc_live_vars);
            } else {
                for expr in exprents.to_vec() {
                    varmaps.to_normal();
                    expr.process_sforms(self, &mut varmaps, node.statement(), calc_live_vars);
                }
            }

            if self.has_updated(node, &varmaps) {
                self.out_var_versions
                    .put(node_id.clone(), varmaps.get_if_true());
                if dgraph.with(|d| d.map_neg_if_branch.contains_key(&node_id)) {
                    self.out_neg_var_versions
                        .put(node_id.clone(), varmaps.get_if_false());
                }

                if !dgraph.extra_nodes().j_contains(&node) {
                    for nd in node.get_successors(DirectEdgeType::Regular).to_vec() {
                        updated.add(nd.get_destination().id_key());
                    }

                    for nd in node.get_successors(DirectEdgeType::Exception).to_vec() {
                        updated.add(nd.get_destination().id_key());
                    }
                }
            }
        }
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
        let varmap = var_maps.get_normal();

        let versions = varmap.borrow().get_var_exprent(var_exprent);

        let cardinality = versions
            .as_ref()
            .map_or(0, |versions| versions.borrow().get_cardinality());
        match cardinality {
            0 => {
                self.update_var_exprent(var_exprent, stat, &varmap, calc_live_vars);
                let message = format!(
                    "Variable read before assignment: {}",
                    var_exprent.to_string()
                );
                ValidationHelper::validate_true(false, &message);
            }
            1 => {
                let versions = versions.expect("versions");
                let last_version = versions
                    .borrow()
                    .iter()
                    .next()
                    .expect("NoSuchElementException");
                self.var_read_single_version(
                    stat,
                    calc_live_vars,
                    var_exprent,
                    &varmap,
                    last_version,
                );
            }
            2 => {
                let versions = versions.expect("versions");
                self.var_read_multiple_versions(
                    stat,
                    calc_live_vars,
                    var_exprent,
                    &varmap,
                    versions,
                );
            }
            _ => {}
        }

        let pair = var_exprent.get_var_version_pair();
        let count = self.read_count.get(&pair).map_or(1, |v| v + 1);
        self.read_count.put(pair, count);
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

    #[allow(dead_code)]
    fn makes_fields_dirty(expr: Exprent) -> bool {
        match expr.get_type() {
            ExprentType::Invocation => return true,
            ExprentType::New => {
                if expr.as_new().get_new_type().type_ == CodeType::Object {
                    return true;
                }
            }
            _ => {}
        }
        false
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
        let var_index = varassign.get_index();

        self.init_version(varassign, stat);

        self.on_assignment(varassign.get_var_version_pair(), varmap, calc_live_vars);

        self.set_current_var(varmap, var_index, varassign.get_version());

        if let Some(current_catchable_map) = self.current_catchable_map.clone() {
            if var_index < VarExprent::STACK_BASE && var_index >= 0 {
                let existing = current_catchable_map.borrow().get(var_index);
                match existing {
                    Some(set) => set.borrow_mut().add(varassign.get_version()),
                    None => {
                        let mut set = self.factory().create_empty_set();
                        set.add(varassign.get_version());
                        varmap.borrow_mut().put(var_index, Shared::new(set));
                    }
                }
            }
        }
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
        let mut map_new = Shared::new(SFormsFastMapDirect::new(self.factory().clone()));
        let all_paths = self.track_field_vars && self.field_reads_all_paths;
        let mut known_preds: Vec<Shared<SFormsFastMapDirect>> = Vec::new();

        for pred in node.get_predecessors(DirectEdgeType::Regular).to_vec() {
            CancelationManager::check_canceled();
            let map_out = self.get_filtered_out_map(node, pred.get_source(), dgraph);
            if all_paths && self.has_out_map(node, pred.get_source(), dgraph) {
                known_preds.push(map_out.clone());
            }
            if map_new.borrow().is_empty() {
                map_new = Shared::new(map_out.borrow().get_copy());
            } else {
                VarMapHolder::merge_maps(&map_new, Some(&map_out));
            }
        }

        for pred in node.get_predecessors(DirectEdgeType::Exception).to_vec() {
            CancelationManager::check_canceled();
            let map_out = self.catchable_versions.get_cloned(&pred.get_source().id_key());
            if let Some(map_out) = map_out {
                if all_paths {
                    known_preds.push(map_out.clone());
                }
                if map_new.borrow().is_empty() {
                    map_new = Shared::new(map_out.borrow().get_copy());
                } else {
                    VarMapHolder::merge_maps(&map_new, Some(&map_out));
                }
            }
        }

        let node_id = node.id_key();
        if self.extra_var_versions.contains_key(&node_id) {
            let map_extra = self
                .extra_var_versions
                .get_cloned(&node_id)
                .expect("extraVarVersions");
            if map_new.borrow().is_empty() {
                map_new = Shared::new(map_extra.borrow().get_copy());
            } else {
                VarMapHolder::merge_maps(&map_new, Some(&map_extra));
            }
        }

        if all_paths {
            FieldReadAllPaths::keep_common_fields(&map_new, &known_preds);
        }

        self.in_var_versions.put(node_id, map_new);
    }

    /// Whether `pred`'s state on its edge to `node` has been computed (what
    /// `getFilteredOutMap` starts from).
    fn has_out_map(&self, node: DirectNode, pred: DirectNode, dgraph: DirectGraph) -> bool {
        let (node_id, pred_id) = (node.id_key(), pred.id_key());
        let neg_dest = dgraph.with(|d| d.map_neg_if_branch.get_cloned(&pred_id));
        if neg_dest.as_ref() == Some(&node_id) {
            self.out_neg_var_versions.contains_key(&pred_id)
        } else {
            self.out_var_versions.contains_key(&pred_id)
        }
    }

    fn get_filtered_out_map(
        &mut self,
        node: DirectNode,
        pred: DirectNode,
        dgraph: DirectGraph,
    ) -> Shared<SFormsFastMapDirect> {
        let mut map_new = Shared::new(SFormsFastMapDirect::new(self.factory().clone()));

        let (node_id, pred_id) = (node.id_key(), pred.id_key());
        let neg_dest = dgraph.with(|d| d.map_neg_if_branch.get_cloned(&pred_id));
        if neg_dest.as_ref() == Some(&node_id) {
            if let Some(out) = self.out_neg_var_versions.get(&pred_id) {
                map_new = Shared::new(out.borrow().get_copy());
            }
        } else if let Some(out) = self.out_var_versions.get(&pred_id) {
            map_new = Shared::new(out.borrow().get_copy());
        }

        let node_try_finally = node.try_finally();
        let pred_try_finally = pred.try_finally();
        if node_try_finally != pred_try_finally {
            if node_try_finally.is_some_and(|tf| {
                tf.type_() == DirectNodeType::Finally && tf.try_finally() == pred_try_finally
            }) {
            } else if pred.type_() == DirectNodeType::Finally {
            } else {
                let mut finally_node = pred_try_finally;
                while finally_node != node_try_finally {
                    ValidationHelper::not_null(finally_node.as_ref());
                    let fnode = finally_node.expect("NullPointerException");
                    if fnode.type_() == DirectNodeType::Finally {
                        if self.track_field_vars
                            && self.field_reads_all_paths
                            && self.finally_changes_fields(fnode.statement())
                        {
                            map_new.borrow_mut().remove_all_fields();
                        }
                        let input_key: std::rc::Rc<str> =
                            format!("{}_FINALLY", fnode.statement().id()).into();
                        Self::get_and_apply_diff(
                            self.in_var_versions.get(&input_key),
                            self.out_var_versions.get(&fnode.id_key()),
                            &map_new,
                        );
                    }
                    finally_node = fnode.try_finally();
                }
            }
        }

        map_new
    }

    /// Whether the finally body of the try-finally `stat` may change fields.
    fn finally_changes_fields(&mut self, stat: Statement) -> bool {
        let id = stat.id();
        if let Some(changes) = self.finally_changes_fields.get_cloned(&id) {
            return changes;
        }
        let body = stat
            .try_as_catch_all()
            .expect("finally of a catch-all")
            .get_handler();
        let changes = FieldReadAllPaths::changes_fields(body);
        self.finally_changes_fields.put(id, changes);
        changes
    }

    fn get_and_apply_diff(
        input: Option<&Shared<SFormsFastMapDirect>>,
        output: Option<&Shared<SFormsFastMapDirect>>,
        target: &Shared<SFormsFastMapDirect>,
    ) {
        let (Some(input), Some(output)) = (input, output) else {
            return;
        };

        let input_entries = input.borrow().entry_list();
        for (key, value) in input_entries {
            if key >= VarExprent::STACK_BASE {
                continue;
            }

            if value.borrow().is_empty() {
                continue;
            }

            let first = value
                .borrow()
                .iter()
                .next()
                .expect("NoSuchElementException");
            let out_set = output.borrow().get(key);
            if let Some(out_set) = out_set {
                if out_set.borrow().contains(&first) {
                    let mut check = out_set.borrow().get_copy();
                    check.complement(&value.borrow());
                    if !check.is_empty() {
                        let target_set = target.borrow().get(key).expect("NullPointerException");
                        target_set.borrow_mut().union(&check);
                    }
                } else {
                    let copy = value.borrow().get_copy();
                    target.borrow_mut().put(key, Shared::new(copy));
                }
            }
        }

        let output_entries = output.borrow().entry_list();
        for (key, value) in output_entries {
            if key >= VarExprent::STACK_BASE {
                continue;
            }

            if value.borrow().is_empty() {
                continue;
            }

            let in_set = input.borrow().get(key);
            if in_set.is_some_and(|set| !set.borrow().is_empty()) {
                continue;
            }

            let copy = value.borrow().get_copy();
            target.borrow_mut().put(key, Shared::new(copy));
        }
    }

    pub fn get_first_protected_range(stat: Statement) -> Option<Statement> {
        let mut stat = stat;
        loop {
            let parent = stat.get_parent()?;

            if parent.is_catch_all() || parent.is_catch() {
                if parent.get_first() == Some(stat) {
                    return Some(parent);
                }
            } else if let Some(sync) = parent.try_as_synchronized() {
                if sync.d(|d| d.body) == Some(stat) {
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
        if let StatementType::CatchAll | StatementType::TryCatch = stat.type_() {
            let lst_vars = match stat.try_as_catch_all() {
                Some(catch_all) => catch_all.get_vars(),
                None => stat.as_catch().get_vars(),
            };

            let mut i = 1;
            while i < stat.get_stats().borrow().size() {
                let varindex = lst_vars.get(i - 1).get_index();
                let map = Shared::new(SFormsFastMapDirect::new(self.factory().clone()));

                self.init_parameter(varindex, &map, true);

                let st = stat.get_stats().borrow().get(i);
                let id = flatthelper
                    .get_direct_node(st)
                    .expect("NullPointerException")
                    .id_key();
                self.extra_var_versions.put(id, map);

                i += 1;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            self.set_catch_maps(st, dgraph, flatthelper);
        }
    }

    fn create_first_map(&mut self) -> Shared<SFormsFastMapDirect> {
        let mt = self.mt.expect("SFormsConstructor.mt");
        let has_this = !mt.has_modifier(ACC_STATIC);

        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());

        let param_count = md.params.len() as i32 + if has_this { 1 } else { 0 };

        let var_map = Shared::new(SFormsFastMapDirect::new(self.factory().clone()));
        let mut var_index = 0;
        for i in 0..param_count {
            self.init_parameter(var_index, &var_map, false);

            if has_this {
                if i == 0 {
                    var_index += 1;
                } else {
                    var_index += md.params[(i - 1) as usize].stack_size;
                }
            } else {
                var_index += md.params[i as usize].stack_size;
            }
        }

        var_map
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
        let (map1, map2) = match (map1, map2) {
            (None, map2) => return map2.is_none(),
            (Some(_), None) => return false,
            (Some(map1), Some(map2)) => (map1, map2),
        };

        if map1.borrow().size() != map2.borrow().size() {
            return false;
        }

        let entries = map2.borrow().entry_list();
        for (key, value) in entries {
            let first = map1.borrow().get(key);
            let equal = match first {
                None => false,
                Some(first) => first.borrow().equals(&value.borrow()),
            };
            if !equal {
                return false;
            }
        }

        true
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
        let id = node.id_key();
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
