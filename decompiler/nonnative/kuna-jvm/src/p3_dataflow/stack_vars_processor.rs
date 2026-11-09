//! Port of `org/jetbrains/java/decompiler/modules/decompiler/StackVarsProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::catch_statement::CatchStatement;
use crate::exprent::Exprent;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::ssau_constructor_sparse_ex::SSAUConstructorSparseEx;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::struct_method::StructMethod;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_graph::VarVersionsGraph;

pub struct StackVarsProcessor;

/// `{returnExprent, changed, isReplaceable}` of `iterateChildExprent`.
type ChildRet = (Option<Exprent>, bool, bool);

const DEFAULT_OPTIONS: StackSimplifyOptions = StackSimplifyOptions {
    inline_regular_vars: false,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct StackSimplifyOptions {
    inline_regular_vars: bool,
}

impl StackSimplifyOptions {
    pub fn new() -> StackSimplifyOptions {
        StackSimplifyOptions {
            inline_regular_vars: false,
        }
    }

    pub fn inline_regular_vars(mut self) -> StackSimplifyOptions {
        self.inline_regular_vars = true;
        self
    }
}

impl StackVarsProcessor {
    pub fn simplify_stack_vars(root: RootStatement, mt: StructMethod, cl: StructClass) {
        Self::simplify_stack_vars_options(root, mt, cl, DEFAULT_OPTIONS);
    }

    /// Java: simplifyStackVars(RootStatement, StructMethod, StructClass, StackSimplifyOptions)
    pub fn simplify_stack_vars_options(
        root: RootStatement,
        mt: StructMethod,
        cl: StructClass,
        options: StackSimplifyOptions,
    ) {
        unported!("StackVarsProcessor.simplifyStackVars")
    }

    pub fn set_versions_to_null(stat: Statement) {
        match stat.get_exprents() {
            None => {
                let sub_stats = stat.get_stats().borrow().to_vec();
                for st in sub_stats {
                    Self::set_versions_to_null(st);
                }

                for exprent in stat.get_stat_exprents() {
                    Self::set_exprent_versions_to_null(exprent);
                }
            }
            Some(exprents) => {
                for exprent in exprents.iter() {
                    Self::set_exprent_versions_to_null(exprent);
                }
            }
        }
    }

    fn set_exprent_versions_to_null(exprent: Exprent) {
        let lst = exprent.get_all_exprents_rec_self(true, true);

        for expr in lst {
            if let Some(var) = expr.try_as_var() {
                var.set_version(0);
            }
        }
    }

    fn iterate_statements(
        root: RootStatement,
        ssa: &mut SSAUConstructorSparseEx,
        options: StackSimplifyOptions,
    ) -> bool {
        unported!("StackVarsProcessor.iterateStatements")
    }

    fn is_replaceable_var(
        exprent: Exprent,
        map_var_values: &JHashMap<VarVersionPair, Exprent>,
    ) -> Option<Exprent> {
        unported!("StackVarsProcessor.isReplaceableVar")
    }

    fn replace_single_var(
        parent: Exprent,
        var: VarExprent,
        dest: Exprent,
        ssau: &SSAUConstructorSparseEx,
    ) {
        unported!("StackVarsProcessor.replaceSingleVar")
    }

    #[allow(clippy::too_many_arguments)]
    fn iterate_exprent(
        lst_exprents: &JList<Exprent>,
        stat: Statement,
        lvts: &mut JHashMap<VarVersionPair, LocalVariable>,
        index: i32,
        next: Option<Exprent>,
        map_var_values: &mut JHashMap<VarVersionPair, Exprent>,
        ssau: &mut SSAUConstructorSparseEx,
        simplify_across_stack: bool,
        ret: &mut [i32; 2],
        options: StackSimplifyOptions,
    ) {
        unported!("StackVarsProcessor.iterateExprent")
    }

    fn on_deleted_var(
        stat: Statement,
        left: VarExprent,
        right: Exprent,
        lvts: &mut JHashMap<VarVersionPair, LocalVariable>,
        ssau: &SSAUConstructorSparseEx,
    ) {
        unported!("StackVarsProcessor.onDeletedVar")
    }

    fn predecessors(
        node: VarVersionNode,
        graph: VarVersionsGraph,
        assignments: &JHashMap<VarVersionPair, VarVersionPair>,
    ) -> JHashSet<Option<VarVersionNode>> {
        unported!("StackVarsProcessor.predecessors")
    }

    fn enclosing_catch(mut stat: Statement) -> Option<CatchStatement> {
        while let Some(parent) = stat.get_parent() {
            if let Some(st) = parent.try_as_catch() {
                return Some(st);
            }

            stat = parent;
        }

        None
    }

    fn simplify_across_stack_exprent(
        exprents: &JList<Exprent>,
        index: i32,
        next: Option<Exprent>,
        right: Exprent,
        left: VarExprent,
    ) -> Option<Exprent> {
        unported!("StackVarsProcessor.simplifyAcrossStackExprent")
    }

    fn set_ret(ret: &mut [i32; 2], a: i32, b: i32) {
        ret[0] = a;
        ret[1] = b;
    }

    fn are_trees_equal(left: VarExprent, tree_a: &[Exprent], tree_b: &[Exprent]) -> bool {
        unported!("StackVarsProcessor.areTreesEqual")
    }

    fn get_all_versions(exprent: Exprent) -> JHashSet<VarVersionPair> {
        unported!("StackVarsProcessor.getAllVersions")
    }

    fn iterate_child_exprent(
        exprent: Exprent,
        parent: Exprent,
        next: Option<Exprent>,
        map_var_values: &mut JHashMap<VarVersionPair, Exprent>,
        ssau: &mut SSAUConstructorSparseEx,
        ret: &mut ChildRet,
        options: StackSimplifyOptions,
    ) {
        unported!("StackVarsProcessor.iterateChildExprent")
    }

    /// Java: setRet(Object[], Object, boolean, boolean)
    fn set_ret_child(ret: &mut ChildRet, a: Option<Exprent>, b: bool, c: bool) {
        ret.0 = a;
        ret.1 = b;
        ret.2 = c;
    }

    fn get_used_versions(
        ssa: &SSAUConstructorSparseEx,
        var: VarVersionPair,
        res: &mut Vec<VarVersionNode>,
    ) -> bool {
        unported!("StackVarsProcessor.getUsedVersions")
    }

    fn is_version_to_be_replaced(
        usedvar: VarVersionPair,
        map_vars: &JHashMap<i32, Option<JHashSet<VarVersionPair>>>,
        ssau: &SSAUConstructorSparseEx,
        leftpaar: VarVersionPair,
    ) -> bool {
        unported!("StackVarsProcessor.isVersionToBeReplaced")
    }

    fn get_all_var_versions(
        leftvar: VarVersionPair,
        exprent: Exprent,
        ssau: &SSAUConstructorSparseEx,
    ) -> JHashMap<i32, Option<JHashSet<VarVersionPair>>> {
        unported!("StackVarsProcessor.getAllVarVersions")
    }

    fn set_effectively_final_vars(
        stat: Statement,
        ssau: &SSAUConstructorSparseEx,
        var_lookup_map: &mut JHashMap<VarVersionPair, VarExprent>,
    ) {
        if let Some(exprents) = stat.get_exprents() {
            if !exprents.is_empty() {
                let mut i = 0;
                while i < exprents.len() {
                    Self::set_effectively_final_vars_exprent(
                        stat,
                        exprents.get(i),
                        ssau,
                        i as i32,
                        &exprents,
                        var_lookup_map,
                    );
                    i += 1;
                }
            }
        }

        let sub_stats = stat.get_stats().borrow().to_vec();

        for st in sub_stats {
            Self::set_effectively_final_vars(st, ssau, var_lookup_map);
        }
    }

    /// Java: setEffectivelyFinalVars(Statement, Exprent, SSAUConstructorSparseEx, int, List, Map)
    fn set_effectively_final_vars_exprent(
        stat: Statement,
        exprent: Exprent,
        ssau: &SSAUConstructorSparseEx,
        index: i32,
        list: &JList<Exprent>,
        var_lookup_map: &mut JHashMap<VarVersionPair, VarExprent>,
    ) {
        unported!("StackVarsProcessor.setEffectivelyFinalVars")
    }

    fn get_roots(vvnode: VarVersionNode) -> Vec<VarVersionNode> {
        unported!("StackVarsProcessor.getRoots")
    }
}
